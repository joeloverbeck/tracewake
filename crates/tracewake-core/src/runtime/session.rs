use crate::actions::pipeline::PipelineStage;
use crate::actions::{
    run_pipeline, ActionRegistry, PipelineContext, PipelineResult, ReasonCode, ReportStatus,
    ValidationReport,
};
use crate::agent::{
    current_place_knowledge_context, ActorDecisionTransaction, ActorDecisionTransactionInput,
    ActorDecisionTransactionOutcome, BlockerCategory, NoHumanActorKnownSurfaceBuilder,
    NoHumanActorKnownSurfaceRequest, RoutineFamily, StuckDiagnosticRecord, StuckResultingStatus,
};
use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::controller::ControllerBindings;
use crate::debug_capability::DebugSessionAuthority;
use crate::debug_reports::{
    action_rejection_report, controller_binding_report, item_location_report,
    no_human_day_debug_report, phase3a_actor_report, phase3a_needs_report, phase3a_planner_report,
    phase3a_routines_report, phase3a_stuck_report, projection_rebuild_debug_report,
    replay_debug_report, ActionRejectionDebugReport, ControllerBindingDebugReport,
    ItemLocationDebugReport, NoHumanDayDebugReport, Phase3ADebugReport,
    ProjectionRebuildDebugReport, ReplayDebugReport,
};
use crate::epistemics::projection::EpistemicProjection;
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind};
use crate::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, ControllerId, DebugReportId, EventId,
    FixtureId, ItemId, PlaceId, ProcessId, ProposalId, StuckDiagnosticId, ValidationReportId,
};
use crate::projections::{
    build_debug_event_log_view, build_embodied_view_model, build_notebook_view,
    proposal_from_current_view_semantic_action, EmbodiedPreflightSource, EmbodiedProjectionSource,
    EmbodiedTruthSnapshot, ProjectionError,
};
use crate::replay::{rebuild_projection, run_replay};
use crate::scheduler::no_human::{
    append_embodied_routine_stuck_diagnostics, default_day_windows, run_no_human_day,
    NoHumanDayConfig, NoHumanDayReport,
};
use crate::scheduler::{
    append_and_apply_actor_artifact, build_actor_decision_trace_event, build_actor_intention_event,
    build_actor_stuck_diagnostic_event, AdvanceUntilRequest, AdvanceUntilResult,
    DeterministicScheduler, OrderingKey, SchedulePhase, SchedulerSourceId, WorldAdvanceError,
    WorldAdvanceOrigin, WorldAdvanceRequest, WorldStepTransactionRequest,
};
use crate::state::{AgentState, ControllerMode, PhysicalState};
use crate::time::SimTick;
use crate::view_models::{
    DebugBeliefsView, DebugEpistemicsView, DebugObservationsView, EmbodiedViewModel,
    TypedActorKnownIntervalSummary,
};

use super::command::{RuntimeCommand, RuntimeCommandKind};
use super::receipt::{
    DebugRuntimeReceipt, EmbodiedRuntimeReceipt, RuntimeActionReceipt, RuntimeReceipt,
};

/// Owned initial aggregates for crate-internal runtime construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RuntimeInitialState {
    registry: ActionRegistry,
    physical_state: PhysicalState,
    agent_state: AgentState,
    event_log: EventLog,
    epistemic_projection: EpistemicProjection,
    controller_bindings: ControllerBindings,
    scheduler: DeterministicScheduler,
    content_manifest_id: ContentManifestId,
    fixture_id: FixtureId,
    content_version: ContentVersion,
}

/// Scheduler-free loaded-world bootstrap product.
///
/// The scheduler is derived by `LoadedWorldRuntime::from_bootstrap`; clients
/// cannot supply one through this additive production handoff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedWorldBootstrap {
    registry: ActionRegistry,
    physical_state: PhysicalState,
    agent_state: AgentState,
    event_log: EventLog,
    epistemic_projection: EpistemicProjection,
    controller_bindings: ControllerBindings,
    content_manifest_id: ContentManifestId,
    fixture_id: FixtureId,
    content_version: ContentVersion,
}

/// Core runtime bootstrap accepted by `LoadedWorldRuntime::from_bootstrap`.
///
/// The wrapper makes the loaded-world runtime handoff explicit: callers cannot
/// pass raw aggregate state directly to the runtime constructor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedLoadedWorldBootstrap {
    bootstrap: LoadedWorldBootstrap,
}

/// Opaque replay seed for reconstructing the accepted initial aggregates
/// without retaining mutable runtime state in a client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReplaySeed {
    bootstrap: ValidatedLoadedWorldBootstrap,
}

/// Core-owned loaded-world runtime/session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedWorldRuntime {
    registry: ActionRegistry,
    physical_state: PhysicalState,
    agent_state: AgentState,
    event_log: EventLog,
    epistemic_projection: EpistemicProjection,
    controller_bindings: ControllerBindings,
    scheduler: DeterministicScheduler,
    content_manifest_id: ContentManifestId,
    fixture_id: FixtureId,
    content_version: ContentVersion,
    initial_physical_state: PhysicalState,
    initial_agent_state: AgentState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeCommandError {
    WorldAdvance(WorldAdvanceError),
    SchedulerRestoreFailed,
}

impl From<WorldAdvanceError> for RuntimeCommandError {
    fn from(value: WorldAdvanceError) -> Self {
        Self::WorldAdvance(value)
    }
}

impl ValidatedLoadedWorldBootstrap {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_validated_content(
        registry: ActionRegistry,
        physical_state: PhysicalState,
        agent_state: AgentState,
        event_log: EventLog,
        epistemic_projection: EpistemicProjection,
        content_manifest_id: ContentManifestId,
        fixture_id: FixtureId,
        content_version: ContentVersion,
    ) -> Self {
        Self::from_bootstrap(LoadedWorldBootstrap {
            registry,
            physical_state,
            agent_state,
            event_log,
            epistemic_projection,
            controller_bindings: ControllerBindings::new(),
            content_manifest_id,
            fixture_id,
            content_version,
        })
    }

    #[cfg(feature = "test-support")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_test_content(
        registry: ActionRegistry,
        physical_state: PhysicalState,
        agent_state: AgentState,
        event_log: EventLog,
        epistemic_projection: EpistemicProjection,
        content_manifest_id: ContentManifestId,
        fixture_id: FixtureId,
        content_version: ContentVersion,
    ) -> Self {
        Self::from_validated_content(
            registry,
            physical_state,
            agent_state,
            event_log,
            epistemic_projection,
            content_manifest_id,
            fixture_id,
            content_version,
        )
    }

    pub fn replay_seed(&self) -> RuntimeReplaySeed {
        RuntimeReplaySeed {
            bootstrap: self.clone(),
        }
    }

    fn from_bootstrap(bootstrap: LoadedWorldBootstrap) -> Self {
        Self { bootstrap }
    }
}

impl RuntimeReplaySeed {
    pub fn reconstruct_bootstrap(&self) -> ValidatedLoadedWorldBootstrap {
        self.bootstrap.clone()
    }
}

impl LoadedWorldRuntime {
    pub(crate) fn from_initial_state(initial: RuntimeInitialState) -> Self {
        let initial_physical_state = initial.physical_state.clone();
        let initial_agent_state = initial.agent_state.clone();
        Self {
            registry: initial.registry,
            physical_state: initial.physical_state,
            agent_state: initial.agent_state,
            event_log: initial.event_log,
            epistemic_projection: initial.epistemic_projection,
            controller_bindings: initial.controller_bindings,
            scheduler: initial.scheduler,
            content_manifest_id: initial.content_manifest_id,
            fixture_id: initial.fixture_id,
            content_version: initial.content_version,
            initial_physical_state,
            initial_agent_state,
        }
    }

    pub fn current_tick(&self) -> crate::time::SimTick {
        self.scheduler.current_tick()
    }

    pub fn from_bootstrap(bootstrap: ValidatedLoadedWorldBootstrap, current_tick: SimTick) -> Self {
        let bootstrap = bootstrap.bootstrap;
        let scheduler = DeterministicScheduler::from_loaded_world(
            current_tick,
            &bootstrap.physical_state,
            &bootstrap.agent_state,
            bootstrap.content_manifest_id.clone(),
        );
        Self::from_initial_state(RuntimeInitialState {
            registry: bootstrap.registry,
            physical_state: bootstrap.physical_state,
            agent_state: bootstrap.agent_state,
            event_log: bootstrap.event_log,
            epistemic_projection: bootstrap.epistemic_projection,
            controller_bindings: bootstrap.controller_bindings,
            scheduler,
            content_manifest_id: bootstrap.content_manifest_id,
            fixture_id: bootstrap.fixture_id,
            content_version: bootstrap.content_version,
        })
    }

    pub fn event_count(&self) -> usize {
        self.event_log.events().len()
    }

    pub fn actor_exists(&self, actor_id: &ActorId) -> bool {
        self.physical_state.actors().contains_key(actor_id)
    }

    pub fn controller_debug_available_for(
        &self,
        controller_id: &ControllerId,
        actor_id: &ActorId,
    ) -> bool {
        self.controller_bindings
            .binding(controller_id)
            .is_some_and(|binding| {
                binding.binding.bound_actor_id.as_ref() == Some(actor_id)
                    && matches!(binding.binding.mode, ControllerMode::Debug)
            })
    }

    pub fn debug_session_authority_for(
        &self,
        authority: &DebugSessionAuthority,
        controller_id: &ControllerId,
        actor_id: &ActorId,
    ) -> Option<DebugSessionAuthority> {
        (authority.debug_only() && self.controller_debug_available_for(controller_id, actor_id))
            .then(|| authority.clone())
    }

    pub fn embodied_view_model(
        &self,
        controller_id: &ControllerId,
        actor_id: &ActorId,
        last_rejection: Option<&ValidationReport>,
        actor_known_interval_summary: Option<TypedActorKnownIntervalSummary>,
    ) -> Result<EmbodiedViewModel, ProjectionError> {
        let context = self.current_view_context(actor_id);
        let snapshot = EmbodiedTruthSnapshot::from_physical_state(&context, &self.physical_state);
        let source = EmbodiedProjectionSource::from_sealed_context(
            &context,
            snapshot,
            Some(&self.agent_state),
        );
        // Same event-log open-duration authority the reservation-conflict check uses,
        // so the surface disables ordinary actions exactly when submit would reject
        // them. A duplicate-terminal error is rejected upstream; treat it as no open
        // duration here rather than failing the view.
        let actor_in_body_exclusive_duration = crate::need_accounting::open_body_exclusive_starts(
            &self.event_log,
            actor_id,
            context.current_tick(),
        )
        .map(|starts| !starts.is_empty())
        .unwrap_or(false);
        let preflight = EmbodiedPreflightSource::with_body_exclusive_duration(
            &self.physical_state,
            &self.registry,
            &self.content_manifest_id,
            actor_in_body_exclusive_duration,
        );
        let mut view = build_embodied_view_model(&context, &source, &preflight, last_rejection)?;
        view.set_notebook(Some(build_notebook_view(
            &self.epistemic_projection,
            &context,
        )));
        view.set_actor_known_interval_summary(actor_known_interval_summary);
        view.set_debug_available(self.controller_debug_available_for(controller_id, actor_id));
        Ok(view)
    }

    pub fn physical_checksum(&self) -> PhysicalChecksum {
        compute_physical_checksum(&self.physical_state, &self.checksum_context()).checksum
    }

    pub fn checksum_context(&self) -> ChecksumContext {
        self.checksum_context_at(self.scheduler.current_tick())
    }

    pub fn checksum_context_at(&self, sim_tick: SimTick) -> ChecksumContext {
        self.checksum_context_for_log(sim_tick, &self.event_log)
    }

    fn checksum_context_for_log(&self, sim_tick: SimTick, log: &EventLog) -> ChecksumContext {
        ChecksumContext {
            fixture_id: self.fixture_id.clone(),
            content_version: self.content_version.clone(),
            sim_tick,
            world_stream_position_applied: Self::world_stream_position_applied_for_log(log),
        }
    }

    pub fn debug_event_log_view(&self) -> crate::view_models::DebugEventLogView {
        build_debug_event_log_view(&self.event_log)
    }

    pub fn controller_binding_debug_report(
        &self,
        report_id: DebugReportId,
    ) -> ControllerBindingDebugReport {
        controller_binding_report(report_id, &self.controller_bindings)
    }

    pub fn item_location_debug_report(
        &self,
        _report_id: DebugReportId,
        item_id: &ItemId,
    ) -> ItemLocationDebugReport {
        item_location_report(
            &self.physical_state,
            &self.event_log,
            item_id,
            &self.checksum_context(),
        )
    }

    pub fn action_rejection_debug_report(
        &self,
        report: &ValidationReport,
    ) -> ActionRejectionDebugReport {
        action_rejection_report(report, &self.physical_state, &self.checksum_context())
    }

    pub fn projection_rebuild_debug_report(
        &self,
        report_id: DebugReportId,
    ) -> ProjectionRebuildDebugReport {
        projection_rebuild_debug_report(
            report_id,
            rebuild_projection(
                &self.initial_physical_state,
                &self.initial_agent_state,
                &self.event_log,
                &self.checksum_context(),
                Some(&self.physical_state),
            ),
        )
    }

    pub fn replay_debug_report(&self, report_id: DebugReportId) -> ReplayDebugReport {
        replay_debug_report(
            report_id,
            run_replay(
                &self.initial_physical_state,
                &self.initial_agent_state,
                &self.event_log,
                &self.checksum_context_at(SimTick::ZERO),
                Some(&self.physical_state),
                None,
                None,
            ),
        )
    }

    pub fn phase3a_needs_debug_report(&self) -> Phase3ADebugReport {
        phase3a_needs_report(&self.agent_state)
    }

    pub fn phase3a_routines_debug_report(&self) -> Phase3ADebugReport {
        phase3a_routines_report(&self.agent_state)
    }

    pub fn phase3a_planner_debug_report(&self, actor_id: &ActorId) -> Phase3ADebugReport {
        phase3a_planner_report(&self.agent_state, actor_id)
    }

    pub fn phase3a_stuck_debug_report(&self) -> Phase3ADebugReport {
        phase3a_stuck_report(&self.agent_state)
    }

    pub fn no_human_day_debug_report(&self) -> NoHumanDayDebugReport {
        no_human_day_debug_report(&self.event_log)
    }

    pub fn phase3a_actor_debug_report(&self, actor_id: &ActorId) -> Phase3ADebugReport {
        phase3a_actor_report(&self.agent_state, actor_id)
    }

    pub fn debug_epistemics_view(&self) -> DebugEpistemicsView {
        self.epistemic_projection.debug_epistemics_view()
    }

    pub fn debug_beliefs_view(&self, actor_id: &ActorId) -> Option<DebugBeliefsView> {
        self.actor_exists(actor_id).then(|| {
            self.epistemic_projection
                .debug_beliefs_view(actor_id.clone())
        })
    }

    pub fn debug_observations_view(&self, actor_id: &ActorId) -> Option<DebugObservationsView> {
        self.actor_exists(actor_id).then(|| {
            self.epistemic_projection
                .debug_observations_view(actor_id.clone())
        })
    }

    fn bind_actor(&mut self, controller_id: ControllerId, actor_id: ActorId, mode: ControllerMode) {
        self.controller_bindings.attach(
            controller_id,
            actor_id.clone(),
            mode,
            self.scheduler.current_tick(),
            &mut self.event_log,
            self.content_manifest_id.clone(),
        );
        self.record_actor_current_place_perception(&actor_id);
    }

    fn detach_controller(&mut self, controller_id: &ControllerId) {
        self.controller_bindings.detach(
            controller_id,
            self.scheduler.current_tick(),
            &mut self.event_log,
            self.content_manifest_id.clone(),
        );
    }

    fn assign_proposal_sequence(&mut self) -> crate::scheduler::ProposalSequence {
        self.scheduler.assign_proposal_sequence()
    }

    fn run_semantic_proposal(
        &mut self,
        controller_id: ControllerId,
        proposal: crate::actions::Proposal,
        uses_world_step: bool,
    ) -> Result<PipelineResult, RuntimeCommandError> {
        let expected_tick = self.scheduler.current_tick();
        let proposal_actor_id = proposal.actor_id.clone();
        let is_continue_routine = proposal.action_id.as_str() == "continue_routine";
        let result = if uses_world_step {
            let step = self.scheduler.transact_world_one_tick(
                &mut self.physical_state,
                &mut self.agent_state,
                &mut self.event_log,
                &self.registry,
                Some(&self.controller_bindings),
                Some(&mut self.epistemic_projection),
                WorldStepTransactionRequest {
                    advance: WorldAdvanceRequest {
                        expected_tick,
                        origin: WorldAdvanceOrigin::Controller(controller_id.clone()),
                        content_manifest_id: self.content_manifest_id.clone(),
                        authorized_sleep_interruptions: Vec::new(),
                    },
                    controlled_proposals: vec![proposal],
                    actor_known_interval_actor_id: None,
                },
            )?;
            step.controlled_pipeline_results
                .into_iter()
                .next()
                .expect("submitted proposal produces one pipeline result")
        } else {
            let ordering_key = OrderingKey::new(
                expected_tick,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Controller(controller_id.clone()),
                self.scheduler.assign_proposal_sequence(),
                proposal.action_id.clone(),
                proposal.target_ids.clone(),
                proposal.proposal_id.as_str().to_string(),
            );
            let mut context = PipelineContext {
                registry: &self.registry,
                state: &mut self.physical_state,
                agent_state: &mut self.agent_state,
                log: &mut self.event_log,
                controller_bindings: Some(&self.controller_bindings),
                epistemic_projection: Some(&mut self.epistemic_projection),
                content_manifest_id: self.content_manifest_id.clone(),
                ordering_key,
            };
            run_pipeline(&mut context, &proposal)
        };
        if result.report.status != ReportStatus::Rejected {
            if let Some(actor_id) = proposal_actor_id.as_ref() {
                self.record_actor_current_place_perception(actor_id);
            }
        }
        if !uses_world_step && result.report.status != ReportStatus::Rejected && is_continue_routine
        {
            if let Some(actor_id) = proposal_actor_id.as_ref() {
                if let Some(follow_on) =
                    self.run_embodied_continue_routine_follow_on(&controller_id, actor_id, &result)?
                {
                    return Ok(follow_on);
                }
            }
        }
        Ok(result)
    }

    fn run_embodied_continue_routine_follow_on(
        &mut self,
        _controller_id: &ControllerId,
        actor_id: &ActorId,
        marker_result: &PipelineResult,
    ) -> Result<Option<PipelineResult>, RuntimeCommandError> {
        let Some(actor) = self.physical_state.actors().get(actor_id) else {
            return Ok(None);
        };
        let decision_tick = self.scheduler.current_tick();
        let current_place_id = actor.current_place_id.clone();
        let actor_known_context =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &self.epistemic_projection,
                agent_state: &self.agent_state,
                actor_id: actor_id.clone(),
                current_place_id: current_place_id.clone(),
                decision_tick,
                window_id: "embodied_continue_routine",
                window_end_tick: decision_tick,
                current_place_witness_event_id: latest_current_place_perception_event_id(
                    &self.event_log,
                    actor_id,
                    decision_tick,
                    &current_place_id,
                ),
                needs_witness_event_id: latest_need_event_id(&self.event_log, actor_id),
                frame_event_id: latest_frame_event_id(&self.event_log),
            })
            .build(&self.agent_state)
            .into_context();
        let source_event_ids = self
            .event_log
            .events()
            .iter()
            .map(|event| event.event_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let source_event_kinds = self
            .event_log
            .events()
            .iter()
            .map(|event| (event.event_id.clone(), event.event_type))
            .collect::<std::collections::BTreeMap<_, _>>();
        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id.clone(),
            decision_tick,
            agent_state: &self.agent_state,
            actor_known_context: &actor_known_context,
            source_event_ids: Some(&source_event_ids),
            source_event_kinds: Some(&source_event_kinds),
            routine_window_family: embodied_routine_window_family(
                &self.agent_state,
                actor_id,
                &actor_known_context,
            ),
            include_idle_fallback: false,
        });
        let prior_scheduler_stuck_events = append_embodied_routine_stuck_diagnostics(
            &mut self.event_log,
            &mut self.agent_state,
            actor_id,
            decision_tick,
            &self.content_manifest_id,
        );
        let proposed = match outcome {
            ActorDecisionTransactionOutcome::Proposed(proposed) => proposed,
            ActorDecisionTransactionOutcome::Stuck { diagnostic } => {
                return self.run_embodied_continue_routine_stuck_outcome(
                    actor_id,
                    decision_tick,
                    marker_result,
                    &prior_scheduler_stuck_events,
                    &diagnostic,
                );
            }
        };
        let decision_trace_record = proposed.decision_trace_record.clone();
        let lifecycle_effects = proposed.lifecycle_effects.clone();
        let follow_on_proposal = proposed.proposal.into_proposal();
        if follow_on_proposal.action_id.as_str() == "continue_routine" {
            let diagnostic = embodied_recursive_continue_routine_diagnostic(
                actor_id,
                decision_tick,
                &follow_on_proposal,
            );
            return self.run_embodied_continue_routine_stuck_outcome(
                actor_id,
                decision_tick,
                marker_result,
                &prior_scheduler_stuck_events,
                &diagnostic,
            );
        }
        if embodied_follow_on_advances_time(&follow_on_proposal) {
            let diagnostic = embodied_time_advancing_follow_on_diagnostic(
                actor_id,
                decision_tick,
                &follow_on_proposal,
            );
            return self.run_embodied_continue_routine_stuck_outcome(
                actor_id,
                decision_tick,
                marker_result,
                &prior_scheduler_stuck_events,
                &diagnostic,
            );
        }
        let ordering_key = OrderingKey::new(
            decision_tick,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id.clone()),
            self.scheduler.assign_proposal_sequence(),
            follow_on_proposal.action_id.clone(),
            follow_on_proposal.target_ids.clone(),
            format!("embodied_continue_routine:{}", actor_id.as_str()),
        );
        let mut context = PipelineContext {
            registry: &self.registry,
            state: &mut self.physical_state,
            agent_state: &mut self.agent_state,
            log: &mut self.event_log,
            controller_bindings: Some(&self.controller_bindings),
            epistemic_projection: Some(&mut self.epistemic_projection),
            content_manifest_id: self.content_manifest_id.clone(),
            ordering_key,
        };
        let mut follow_on_result = run_pipeline(&mut context, &follow_on_proposal);
        if follow_on_result.report.status == ReportStatus::Rejected {
            let mut receipt_prefix = marker_result.appended_events.clone();
            receipt_prefix.extend(prior_scheduler_stuck_events);
            follow_on_result
                .appended_events
                .splice(0..0, receipt_prefix);
            return Ok(Some(follow_on_result));
        }
        let ordinary_event = first_appended_event(&follow_on_result).cloned();
        let process_id = ProcessId::new(format!(
            "process.embodied_continue_routine.{}.{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .map_err(WorldAdvanceError::InvalidMarkerId)?;
        let frame_event_id = latest_frame_event_id(&self.event_log).or_else(|| {
            marker_result
                .appended_events
                .first()
                .map(|event| event.event_id.clone())
                .or_else(|| {
                    self.event_log
                        .events()
                        .last()
                        .map(|event| event.event_id.clone())
                })
        });
        for effect in &lifecycle_effects {
            let event = build_actor_intention_event(
                actor_id,
                decision_tick,
                &process_id,
                &follow_on_proposal,
                &decision_trace_record,
                effect,
                &self.content_manifest_id,
                ordinary_event.as_ref(),
                frame_event_id.as_ref(),
            )?;
            let event_id = append_and_apply_actor_artifact(
                &mut self.event_log,
                &mut self.agent_state,
                actor_id,
                event,
            )?;
            if let Some(appended) = self
                .event_log
                .events()
                .iter()
                .find(|event| event.event_id == event_id)
            {
                follow_on_result.appended_events.push(appended.clone());
            }
        }
        let trace_event = build_actor_decision_trace_event(
            actor_id,
            decision_tick,
            &process_id,
            &follow_on_proposal,
            &decision_trace_record,
            &self.content_manifest_id,
            ordinary_event.as_ref(),
            frame_event_id.as_ref(),
        )?;
        let trace_event_id = append_and_apply_actor_artifact(
            &mut self.event_log,
            &mut self.agent_state,
            actor_id,
            trace_event,
        )?;
        if let Some(appended) = self
            .event_log
            .events()
            .iter()
            .find(|event| event.event_id == trace_event_id)
        {
            follow_on_result.appended_events.push(appended.clone());
        }
        let mut receipt_prefix = marker_result.appended_events.clone();
        receipt_prefix.extend(prior_scheduler_stuck_events);
        follow_on_result
            .appended_events
            .splice(0..0, receipt_prefix);
        self.record_actor_current_place_perception(actor_id);
        Ok(Some(follow_on_result))
    }

    fn run_embodied_continue_routine_stuck_outcome(
        &mut self,
        actor_id: &ActorId,
        decision_tick: SimTick,
        marker_result: &PipelineResult,
        prior_scheduler_stuck_events: &[EventEnvelope],
        diagnostic: &StuckDiagnosticRecord,
    ) -> Result<Option<PipelineResult>, RuntimeCommandError> {
        let process_id = ProcessId::new(format!(
            "process.embodied_continue_routine_stuck.{}.{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .map_err(WorldAdvanceError::InvalidMarkerId)?;
        let frame_event_id = latest_frame_event_id(&self.event_log);
        let event = build_actor_stuck_diagnostic_event(
            actor_id,
            decision_tick,
            &process_id,
            diagnostic,
            &self.content_manifest_id,
            frame_event_id.as_ref(),
        )?;
        let event_id = event.event_id.clone();
        if !self.event_log.contains_event_id(&event_id) {
            append_and_apply_actor_artifact(
                &mut self.event_log,
                &mut self.agent_state,
                actor_id,
                event,
            )?;
        }
        let appended = self
            .event_log
            .events()
            .iter()
            .find(|event| event.event_id == event_id)
            .cloned()
            .into_iter()
            .collect::<Vec<_>>();
        let mut appended_events = marker_result.appended_events.clone();
        appended_events.extend(prior_scheduler_stuck_events.iter().cloned());
        appended_events.extend(appended);
        self.record_actor_current_place_perception(actor_id);
        Ok(Some(PipelineResult {
            report: embodied_continue_routine_stuck_report(
                actor_id,
                decision_tick,
                diagnostic,
                &appended_events,
            ),
            appended_events,
        }))
    }

    fn advance_until(
        &mut self,
        controller_id: ControllerId,
        possessed_actor_id: ActorId,
        max_ticks: u64,
    ) -> Result<AdvanceUntilResult, RuntimeCommandError> {
        Ok(self.scheduler.advance_until(
            &mut self.physical_state,
            &mut self.agent_state,
            &mut self.event_log,
            &self.registry,
            Some(&mut self.epistemic_projection),
            AdvanceUntilRequest {
                expected_tick: self.scheduler.current_tick(),
                origin: WorldAdvanceOrigin::Controller(controller_id),
                content_manifest_id: self.content_manifest_id.clone(),
                possessed_actor_id,
                max_ticks,
            },
        )?)
    }

    fn run_no_human_day(&mut self) -> Result<NoHumanDayReport, RuntimeCommandError> {
        let windows = default_day_windows(self.scheduler.current_tick());
        let actor_ids = self.physical_state.actors().keys().cloned().collect();
        let mut physical_state = self.physical_state.clone();
        let mut agent_state = self.agent_state.clone();
        let mut event_log = self.event_log.clone();
        let report = run_no_human_day(
            &mut physical_state,
            &mut agent_state,
            &mut event_log,
            &self.registry,
            self.content_manifest_id.clone(),
            NoHumanDayConfig { actor_ids, windows },
        );
        let checksum_context =
            self.checksum_context_for_log(self.scheduler.current_tick(), &event_log);
        let rebuild = rebuild_projection(
            &self.initial_physical_state,
            &self.initial_agent_state,
            &event_log,
            &checksum_context,
            Some(&physical_state),
        );
        let scheduler = DeterministicScheduler::restore_from_rebuild_report(
            &rebuild,
            self.content_manifest_id.clone(),
        )
        .ok_or(RuntimeCommandError::SchedulerRestoreFailed)?;
        physical_state = rebuild.final_state;
        agent_state = rebuild.final_agent_state;
        let mut epistemic_projection = rebuild.final_epistemic_projection;
        let bound_actor_ids = self
            .controller_bindings
            .debug_bindings()
            .iter()
            .filter_map(|binding| binding.binding.bound_actor_id.clone())
            .collect::<Vec<_>>();
        for actor_id in bound_actor_ids {
            scheduler.record_actor_current_place_perception(
                &mut physical_state,
                &mut agent_state,
                &mut event_log,
                &mut epistemic_projection,
                &actor_id,
                &self.content_manifest_id,
            );
        }
        self.physical_state = physical_state;
        self.agent_state = agent_state;
        self.event_log = event_log;
        self.epistemic_projection = epistemic_projection;
        self.scheduler = scheduler;
        Ok(report)
    }

    fn rebuild_from_owned_log(&mut self) -> Result<(), RuntimeCommandError> {
        let checksum_context = self.checksum_context();
        let rebuild = rebuild_projection(
            &self.initial_physical_state,
            &self.initial_agent_state,
            &self.event_log,
            &checksum_context,
            Some(&self.physical_state),
        );
        self.scheduler = DeterministicScheduler::restore_from_rebuild_report(
            &rebuild,
            self.content_manifest_id.clone(),
        )
        .ok_or(RuntimeCommandError::SchedulerRestoreFailed)?;
        self.physical_state = rebuild.final_state;
        self.agent_state = rebuild.final_agent_state;
        self.epistemic_projection = rebuild.final_epistemic_projection;
        Ok(())
    }

    pub fn submit_command(
        &mut self,
        command: RuntimeCommand,
    ) -> Result<RuntimeReceipt, RuntimeCommandError> {
        match command.kind {
            RuntimeCommandKind::OneTickWait { origin } => self.run_one_tick_wait(origin),
            RuntimeCommandKind::SubmitSemanticAction {
                controller_id,
                actor_id,
                entry,
                source_view,
            } => {
                let proposal_sequence = self.assign_proposal_sequence();
                let proposal = proposal_from_current_view_semantic_action(
                    ProposalId::new(format!("proposal_runtime_{}", proposal_sequence.value()))
                        .expect("runtime proposal ids are generated from numeric sequences"),
                    actor_id,
                    self.scheduler.current_tick(),
                    &entry,
                    source_view.as_ref(),
                    &controller_id,
                );
                let uses_world_step = entry.action_id.as_str() == "wait";
                let result =
                    self.run_semantic_proposal(controller_id, proposal, uses_world_step)?;
                Ok(RuntimeReceipt::action_submitted(
                    RuntimeActionReceipt::from(result),
                ))
            }
            RuntimeCommandKind::ContinueUntil {
                controller_id,
                possessed_actor_id,
                max_ticks,
            } => {
                let result = self.advance_until(controller_id, possessed_actor_id, max_ticks)?;
                Ok(RuntimeReceipt::continued(result))
            }
            RuntimeCommandKind::BindController {
                controller_id,
                actor_id,
                mode,
            } => {
                self.bind_actor(controller_id, actor_id, mode);
                Ok(RuntimeReceipt::embodied(EmbodiedRuntimeReceipt::new(
                    "Controller binding changed.",
                    vec!["runtime:controller_binding".to_string()],
                )))
            }
            RuntimeCommandKind::BindDebugController {
                controller_id,
                actor_id,
                ..
            } => {
                self.bind_actor(controller_id, actor_id, ControllerMode::Debug);
                Ok(RuntimeReceipt::embodied(EmbodiedRuntimeReceipt::new(
                    "Debug controller binding changed.",
                    vec!["runtime:debug_controller_binding".to_string()],
                )))
            }
            RuntimeCommandKind::DetachController { controller_id } => {
                self.detach_controller(&controller_id);
                Ok(RuntimeReceipt::embodied(EmbodiedRuntimeReceipt::new(
                    "Controller binding removed.",
                    vec!["runtime:controller_binding".to_string()],
                )))
            }
            RuntimeCommandKind::RunNoHumanDay { .. } => {
                let result = self.run_no_human_day()?;
                Ok(RuntimeReceipt::no_human_day(result))
            }
            RuntimeCommandKind::RebuildFromReplaySeed => {
                self.rebuild_from_owned_log()?;
                Ok(RuntimeReceipt::embodied(EmbodiedRuntimeReceipt::new(
                    "Replay seed rebuild requested.",
                    vec!["runtime:replay_seed".to_string()],
                )))
            }
            RuntimeCommandKind::EmbodiedView { actor_id } => {
                Ok(RuntimeReceipt::embodied(EmbodiedRuntimeReceipt::new(
                    format!("Embodied view requested for {}.", actor_id.as_str()),
                    vec!["runtime:embodied_view".to_string()],
                )))
            }
            RuntimeCommandKind::DebugView { _authority } => {
                Ok(RuntimeReceipt::debug(DebugRuntimeReceipt::new(
                    &_authority,
                    self.scheduler.current_tick(),
                    self.scheduler.current_tick(),
                    Vec::new(),
                    Some("debug_view".to_string()),
                )))
            }
        }
    }

    pub fn wait_one_tick(
        &mut self,
        origin: WorldAdvanceOrigin,
    ) -> Result<RuntimeReceipt, RuntimeCommandError> {
        self.submit_command(RuntimeCommand::one_tick_wait(origin))
    }

    pub fn wait_one_tick_debug(
        &mut self,
        authority: &DebugSessionAuthority,
        origin: WorldAdvanceOrigin,
    ) -> Result<DebugRuntimeReceipt, RuntimeCommandError> {
        let result = self.run_world_one_tick(origin)?;
        Ok(DebugRuntimeReceipt::from_world_advance_result(
            authority,
            result,
            Some("one_tick_wait".to_string()),
        ))
    }

    fn run_one_tick_wait(
        &mut self,
        origin: WorldAdvanceOrigin,
    ) -> Result<RuntimeReceipt, RuntimeCommandError> {
        let result = self.run_world_one_tick(origin)?;

        Ok(RuntimeReceipt::one_tick_advanced(result))
    }

    fn run_world_one_tick(
        &mut self,
        origin: WorldAdvanceOrigin,
    ) -> Result<crate::scheduler::WorldAdvanceResult, RuntimeCommandError> {
        let result = self.scheduler.transact_world_one_tick(
            &mut self.physical_state,
            &mut self.agent_state,
            &mut self.event_log,
            &self.registry,
            Some(&self.controller_bindings),
            Some(&mut self.epistemic_projection),
            WorldStepTransactionRequest {
                advance: WorldAdvanceRequest {
                    expected_tick: self.scheduler.current_tick(),
                    origin,
                    content_manifest_id: self.content_manifest_id.clone(),
                    authorized_sleep_interruptions: Vec::new(),
                },
                controlled_proposals: Vec::new(),
                actor_known_interval_actor_id: None,
            },
        )?;

        Ok(result)
    }

    fn current_view_context(&self, actor_id: &ActorId) -> crate::epistemics::KnowledgeContext {
        current_place_knowledge_context(
            &self.physical_state,
            Some(&self.epistemic_projection),
            actor_id,
            self.scheduler.current_tick(),
            &self.content_manifest_id,
            self.event_log.events().len() as u64,
        )
    }

    fn world_stream_position_applied_for_log(log: &EventLog) -> u64 {
        log.events()
            .iter()
            .filter(|event| event.stream == crate::events::EventStream::World)
            .count()
            .saturating_sub(1) as u64
    }

    fn record_actor_current_place_perception(&mut self, actor_id: &ActorId) {
        self.scheduler.record_actor_current_place_perception(
            &mut self.physical_state,
            &mut self.agent_state,
            &mut self.event_log,
            &mut self.epistemic_projection,
            actor_id,
            &self.content_manifest_id,
        );
    }
}

fn first_appended_event(result: &PipelineResult) -> Option<&EventEnvelope> {
    result.appended_events.first()
}

fn latest_frame_event_id(log: &EventLog) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            matches!(
                event.event_type,
                EventKind::NoHumanDayStarted
                    | EventKind::NoHumanAdvanceStarted
                    | EventKind::DeclaredWorldProcessApplied
            )
        })
        .map(|event| event.event_id.clone())
}

fn latest_current_place_perception_event_id(
    log: &EventLog,
    actor_id: &ActorId,
    tick: SimTick,
    place_id: &PlaceId,
) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            event.event_type == EventKind::ObservationRecorded
                && event.actor_id.as_ref() == Some(actor_id)
                && event.sim_tick == tick
                && event.place_id.as_ref() == Some(place_id)
        })
        .map(|event| event.event_id.clone())
}

fn latest_need_event_id(log: &EventLog, actor_id: &ActorId) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && (event.actor_id.as_ref() == Some(actor_id)
                    || event
                        .payload
                        .iter()
                        .any(|field| field.key == "actor_id" && field.value == actor_id.as_str()))
        })
        .map(|event| event.event_id.clone())
}

fn embodied_routine_window_family(
    agent_state: &AgentState,
    actor_id: &ActorId,
    actor_known_context: &crate::agent::ActorKnownPlanningContext,
) -> Option<RoutineFamily> {
    let active_intention_id = agent_state.active_intention_by_actor().get(actor_id)?;
    let active = agent_state.intentions().get(active_intention_id)?;
    let selected_method = active.selected_routine_method.as_ref()?;
    let family = agent_state
        .routine_executions()
        .values()
        .filter(|execution| &execution.actor_id == actor_id)
        .filter(|execution| &execution.template_id == selected_method)
        .filter(|execution| !execution.step_status.is_resolved())
        .map(|execution| execution.family)
        .next()
        .or_else(|| routine_family_from_template_id(selected_method.as_str()))?;
    if family == RoutineFamily::WorkBlock
        && !actor_known_context
            .known_workplaces()
            .values()
            .any(|place_id| place_id == actor_known_context.current_place_id())
    {
        Some(RoutineFamily::GoToWork)
    } else {
        Some(family)
    }
}

fn routine_family_from_template_id(template_id: &str) -> Option<RoutineFamily> {
    if template_id.contains("go_to_work") {
        Some(RoutineFamily::GoToWork)
    } else if template_id.contains("work_block") {
        Some(RoutineFamily::WorkBlock)
    } else if template_id.contains("eat") {
        Some(RoutineFamily::EatMeal)
    } else if template_id.contains("sleep") {
        Some(RoutineFamily::SleepNight)
    } else if template_id.contains("return_home") {
        Some(RoutineFamily::ReturnHome)
    } else if template_id.contains("find_food") {
        Some(RoutineFamily::FindFood)
    } else if template_id.contains("leave_unsafe_place") {
        Some(RoutineFamily::LeaveUnsafePlace)
    } else {
        None
    }
}

fn embodied_continue_routine_stuck_report(
    actor_id: &ActorId,
    decision_tick: SimTick,
    diagnostic: &StuckDiagnosticRecord,
    appended_events: &[EventEnvelope],
) -> ValidationReport {
    ValidationReport {
        validation_report_id: ValidationReportId::new(format!(
            "report.embodied_continue_routine_stuck.{}.{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .expect("embodied continue routine stuck report ids are generated from typed ids"),
        proposal_id: ProposalId::new(format!(
            "proposal.embodied_continue_routine_stuck.{}.{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .expect("embodied continue routine stuck proposal ids are generated from typed ids"),
        actor_id: Some(actor_id.clone()),
        action_id: ActionId::new("continue_routine")
            .expect("continue_routine is a stable action id"),
        target_ids: vec![diagnostic.diagnostic_id.as_str().to_string()],
        status: ReportStatus::Rejected,
        failed_stage: Some(PipelineStage::MutationPlanConstruction),
        reason_codes: vec![ReasonCode::RoutineStepBlocked],
        checked_facts: Vec::new(),
        actor_visible_facts: Vec::new(),
        debug_only_facts: Vec::new(),
        actor_visible_summary: diagnostic.actor_known_explanation.clone(),
        debug_summary: format!(
            "embodied continue_routine stuck: {}",
            diagnostic.concrete_blocker
        ),
        would_mutate: true,
        event_ids: appended_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        checksum_before: None,
        checksum_after: None,
    }
}

fn embodied_recursive_continue_routine_diagnostic(
    actor_id: &ActorId,
    decision_tick: SimTick,
    proposal: &crate::actions::Proposal,
) -> StuckDiagnosticRecord {
    StuckDiagnosticRecord::new(
        StuckDiagnosticId::new(format!(
            "stuck_embodied_continue_routine_recursive_{}_{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .expect("embodied recursive continue diagnostic ids are generated from typed ids"),
        actor_id.clone(),
        decision_tick,
        decision_tick.advance_by(1),
        None,
        None,
        None,
        proposal
            .parameters
            .get("routine_template_id")
            .and_then(|template_id| crate::ids::RoutineTemplateId::new(template_id.clone()).ok()),
        proposal
            .parameters
            .get("routine_execution_id")
            .and_then(|execution_id| {
                crate::ids::RoutineExecutionId::new(execution_id.clone()).ok()
            }),
        None,
        None,
        BlockerCategory::PlannerBudgetExhausted,
        "recursive continue_routine follow-on",
        "routine continuation could not select an ordinary follow-on action",
        "embodied continue_routine resolved to continue_routine again",
        "typed_stuck_diagnostic",
        StuckResultingStatus::Failed,
    )
}

fn embodied_follow_on_advances_time(proposal: &crate::actions::Proposal) -> bool {
    proposal.action_id.as_str() == "wait"
}

fn embodied_time_advancing_follow_on_diagnostic(
    actor_id: &ActorId,
    decision_tick: SimTick,
    proposal: &crate::actions::Proposal,
) -> StuckDiagnosticRecord {
    StuckDiagnosticRecord::new(
        StuckDiagnosticId::new(format!(
            "stuck_embodied_continue_routine_time_advancing_{}_{}",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .expect("embodied time-advancing continue diagnostic ids are generated from typed ids"),
        actor_id.clone(),
        decision_tick,
        decision_tick.advance_by(1),
        None,
        None,
        None,
        proposal
            .parameters
            .get("routine_template_id")
            .and_then(|template_id| crate::ids::RoutineTemplateId::new(template_id.clone()).ok()),
        proposal
            .parameters
            .get("routine_execution_id")
            .and_then(|execution_id| {
                crate::ids::RoutineExecutionId::new(execution_id.clone()).ok()
            }),
        None,
        None,
        BlockerCategory::SchedulingReservation,
        "time-advancing follow-on requires scheduler authority",
        "routine continuation cannot safely commit a time-advancing follow-on yet",
        "embodied continue_routine resolved to wait; scheduler-owned routing is deferred",
        "typed_stuck_diagnostic",
        StuckResultingStatus::Failed,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;
    use crate::agent::{NeedChangeCause, NeedKind, NeedState};
    use crate::ids::{ControllerId, PlaceId};
    use crate::runtime::RuntimeReceiptKind;
    use crate::state::{ActorBody, NeedModelState, PlaceState, VisibilityDefault};
    use crate::time::SimTick;

    fn manifest_id() -> ContentManifestId {
        ContentManifestId::new("runtime_session_test").unwrap()
    }

    fn fixture_id() -> FixtureId {
        FixtureId::new("runtime_session_test").unwrap()
    }

    fn content_version() -> ContentVersion {
        ContentVersion::new("content_v1").unwrap()
    }

    fn empty_runtime() -> LoadedWorldRuntime {
        LoadedWorldRuntime::from_initial_state(RuntimeInitialState {
            registry: ActionRegistry::new(),
            physical_state: PhysicalState::empty(NeedModelState::new(5, 3)),
            agent_state: AgentState::default(),
            event_log: EventLog::new(),
            epistemic_projection: EpistemicProjection::new(manifest_id()),
            controller_bindings: ControllerBindings::new(),
            scheduler: DeterministicScheduler::new(SimTick::ZERO),
            content_manifest_id: manifest_id(),
            fixture_id: fixture_id(),
            content_version: content_version(),
        })
    }

    fn loaded_runtime() -> LoadedWorldRuntime {
        let actor_id = ActorId::new("actor_runtime").unwrap();
        let place_id = PlaceId::new("runtime_room").unwrap();
        let mut actors = BTreeMap::new();
        actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), place_id.clone()),
        );
        let mut local_actor_ids = BTreeSet::new();
        local_actor_ids.insert(actor_id.clone());
        let mut places = BTreeMap::new();
        places.insert(
            place_id.clone(),
            PlaceState {
                place_id,
                display_label: "Runtime room".to_string(),
                adjacent_place_ids: BTreeSet::new(),
                connected_door_ids: BTreeSet::new(),
                local_container_ids: BTreeSet::new(),
                local_item_ids: BTreeSet::new(),
                local_actor_ids,
                visibility_default: VisibilityDefault::Visible,
            },
        );
        let physical_state = PhysicalState::from_test_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            NeedModelState::new(5, 3),
        );
        let agent_state = AgentState::from_test_seed_parts(
            BTreeMap::from([(
                actor_id,
                BTreeMap::from([
                    (
                        NeedKind::Hunger,
                        NeedState::initial(NeedKind::Hunger, 10, NeedChangeCause::FixtureInitial),
                    ),
                    (
                        NeedKind::Fatigue,
                        NeedState::initial(NeedKind::Fatigue, 10, NeedChangeCause::FixtureInitial),
                    ),
                ]),
            )]),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
        );
        let scheduler = DeterministicScheduler::from_loaded_world(
            SimTick::ZERO,
            &physical_state,
            &agent_state,
            manifest_id(),
        );
        LoadedWorldRuntime::from_initial_state(RuntimeInitialState {
            registry: ActionRegistry::new(),
            physical_state,
            agent_state,
            event_log: EventLog::new(),
            epistemic_projection: EpistemicProjection::new(manifest_id()),
            controller_bindings: ControllerBindings::new(),
            scheduler,
            content_manifest_id: manifest_id(),
            fixture_id: fixture_id(),
            content_version: content_version(),
        })
    }

    fn empty_bootstrap() -> ValidatedLoadedWorldBootstrap {
        ValidatedLoadedWorldBootstrap::from_validated_content(
            ActionRegistry::new(),
            PhysicalState::empty(NeedModelState::new(5, 3)),
            AgentState::default(),
            EventLog::new(),
            EpistemicProjection::new(manifest_id()),
            manifest_id(),
            fixture_id(),
            content_version(),
        )
    }

    #[test]
    fn assign_proposal_sequence_advances_monotonically_from_runtime() {
        use crate::scheduler::ProposalSequence;

        let mut runtime = empty_runtime();

        let first = runtime.assign_proposal_sequence();
        let second = runtime.assign_proposal_sequence();

        // The first assignment legitimately equals the default (sequence 0); a
        // `-> Default::default()` mutation would also return 0 on the second call,
        // so assert the second advances to 1 and is not the default.
        assert_eq!(first, ProposalSequence::new(0));
        assert_eq!(second, ProposalSequence::new(1));
        assert_ne!(second, ProposalSequence::default());
    }

    #[test]
    fn one_tick_wait_advances_world_through_owned_runtime() {
        let mut runtime = empty_runtime();

        let receipt = runtime
            .wait_one_tick(WorldAdvanceOrigin::Controller(
                ControllerId::new("controller_human").unwrap(),
            ))
            .unwrap();

        assert_eq!(runtime.current_tick(), SimTick::new(1));
        assert!(runtime.event_count() > 0);
        match receipt.kind() {
            RuntimeReceiptKind::OneTickAdvanced(receipt) => {
                assert!(receipt.advanced());
                assert!(receipt.appended_event_count() > 0);
                assert!(receipt.actor_known_interval_summary().is_none());
            }
            _ => panic!("expected one-tick receipt"),
        }
    }

    #[test]
    fn actor_exists_reports_known_and_absent_actors() {
        let runtime = loaded_runtime();
        let known_actor_id = ActorId::new("actor_runtime").unwrap();
        let absent_actor_id = ActorId::new("actor_absent").unwrap();

        assert!(runtime.actor_exists(&known_actor_id));
        assert!(!runtime.actor_exists(&absent_actor_id));
    }

    #[test]
    fn embodied_stuck_outcome_is_idempotent_for_same_tick_diagnostic() {
        let mut runtime = loaded_runtime();
        let actor_id = ActorId::new("actor_runtime").unwrap();
        let decision_tick = SimTick::ZERO;
        let marker_result = PipelineResult {
            report: ValidationReport {
                validation_report_id: ValidationReportId::new("report_marker").unwrap(),
                proposal_id: ProposalId::new("proposal_marker").unwrap(),
                actor_id: Some(actor_id.clone()),
                action_id: ActionId::new("continue_routine").unwrap(),
                target_ids: Vec::new(),
                status: ReportStatus::Accepted,
                failed_stage: None,
                reason_codes: Vec::new(),
                checked_facts: Vec::new(),
                actor_visible_facts: Vec::new(),
                debug_only_facts: Vec::new(),
                actor_visible_summary: "marker accepted".to_string(),
                debug_summary: "marker accepted".to_string(),
                would_mutate: false,
                event_ids: Vec::new(),
                checksum_before: None,
                checksum_after: None,
            },
            appended_events: Vec::new(),
        };
        let wait_proposal = crate::actions::Proposal::new(
            ProposalId::new("proposal_wait_follow_on").unwrap(),
            crate::actions::ProposalOrigin::Test,
            Some(actor_id.clone()),
            ActionId::new("wait").unwrap(),
            decision_tick,
        );
        let diagnostic =
            embodied_time_advancing_follow_on_diagnostic(&actor_id, decision_tick, &wait_proposal);

        let first = runtime
            .run_embodied_continue_routine_stuck_outcome(
                &actor_id,
                decision_tick,
                &marker_result,
                &[],
                &diagnostic,
            )
            .unwrap()
            .expect("stuck outcome returns a receipt");
        let second = runtime
            .run_embodied_continue_routine_stuck_outcome(
                &actor_id,
                decision_tick,
                &marker_result,
                &[],
                &diagnostic,
            )
            .unwrap()
            .expect("repeat stuck outcome returns a receipt");

        assert_eq!(first.report.status, ReportStatus::Rejected);
        assert_eq!(second.report.status, ReportStatus::Rejected);
        assert!(second
            .report
            .reason_codes
            .contains(&ReasonCode::RoutineStepBlocked));
        assert_eq!(
            second
                .appended_events
                .iter()
                .filter(|event| event.event_type == EventKind::StuckDiagnosticRecorded)
                .count(),
            1,
            "repeat receipt should include the already-recorded diagnostic"
        );
        assert_eq!(
            runtime
                .event_log
                .events()
                .iter()
                .filter(|event| event.event_type == EventKind::StuckDiagnosticRecorded)
                .count(),
            1,
            "same tick diagnostic is recorded once in the event log"
        );
    }

    #[test]
    fn replay_seed_command_rebuilds_scheduler_from_owned_log() {
        let mut runtime = loaded_runtime();
        runtime
            .wait_one_tick(WorldAdvanceOrigin::Controller(
                ControllerId::new("controller_human").unwrap(),
            ))
            .unwrap();
        assert_eq!(runtime.current_tick(), SimTick::new(1));

        runtime.scheduler = DeterministicScheduler::new(SimTick::ZERO);
        assert_eq!(runtime.current_tick(), SimTick::ZERO);

        let receipt = runtime
            .submit_command(RuntimeCommand::rebuild_from_replay_seed())
            .unwrap();

        assert_eq!(runtime.current_tick(), SimTick::new(1));
        assert!(matches!(receipt.kind(), RuntimeReceiptKind::Embodied(_)));
    }

    #[test]
    fn checksum_context_uses_last_applied_world_stream_position() {
        let mut runtime = empty_runtime();
        assert_eq!(runtime.checksum_context().world_stream_position_applied, 0);

        for _ in 0..3 {
            runtime
                .wait_one_tick(WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ))
                .unwrap();
        }

        assert_eq!(runtime.event_count(), 3);
        assert_eq!(
            runtime.checksum_context().world_stream_position_applied,
            runtime.event_count() as u64 - 1
        );
    }

    #[test]
    fn closed_family_one_tick_wait_preserves_existing_effect() {
        let mut runtime = empty_runtime();

        let receipt = runtime
            .submit_command(RuntimeCommand::one_tick_wait(
                WorldAdvanceOrigin::Controller(ControllerId::new("controller_human").unwrap()),
            ))
            .unwrap();

        assert_eq!(runtime.current_tick(), SimTick::new(1));
        assert!(runtime.event_count() > 0);
        assert!(matches!(
            receipt.kind(),
            RuntimeReceiptKind::OneTickAdvanced(result)
                if result.advanced() && result.appended_event_count() > 0
        ));
    }

    #[test]
    fn debug_one_tick_receipt_retains_privileged_scheduler_details() {
        let mut runtime = loaded_runtime();
        let authority = DebugSessionAuthority::mint();

        let receipt = runtime
            .wait_one_tick_debug(
                &authority,
                WorldAdvanceOrigin::Controller(ControllerId::new("controller_human").unwrap()),
            )
            .unwrap();

        assert!(receipt.debug_only());
        assert_eq!(receipt.prior_tick(), SimTick::ZERO);
        assert_eq!(receipt.resulting_tick(), SimTick::new(1));
        assert!(!receipt.event_ids().is_empty());
        let due_work = receipt
            .due_work_summary()
            .expect("debug receipt carries due-work summary");
        assert!(due_work.actor_transactions_attempted > 0);
        assert!(!receipt.actor_step_summaries().is_empty());
    }

    #[test]
    fn debug_session_authority_requires_supplied_operator_authority() {
        let mut runtime = loaded_runtime();
        let controller_id = ControllerId::new("controller_operator_debug").unwrap();
        let actor_id = ActorId::new("actor_runtime").unwrap();

        runtime
            .submit_command(RuntimeCommand::bind_controller(
                controller_id.clone(),
                actor_id.clone(),
            ))
            .unwrap();
        let authority = DebugSessionAuthority::mint();
        assert!(
            runtime
                .debug_session_authority_for(&authority, &controller_id, &actor_id)
                .is_none(),
            "ordinary embodied binding must not validate debug authority"
        );

        runtime
            .submit_command(RuntimeCommand::bind_debug_controller(
                authority.clone(),
                controller_id.clone(),
                actor_id.clone(),
            ))
            .unwrap();
        let validated = runtime
            .debug_session_authority_for(&authority, &controller_id, &actor_id)
            .expect("debug binding with held operator authority validates authority");
        assert!(validated.debug_only());
    }

    #[test]
    fn replay_seed_reconstructs_byte_identical_bootstrap() {
        let bootstrap = empty_bootstrap();
        let seed = bootstrap.replay_seed();

        assert_eq!(seed.reconstruct_bootstrap(), bootstrap);
    }

    #[test]
    fn bootstrap_derives_scheduler_without_client_injection() {
        let runtime = LoadedWorldRuntime::from_bootstrap(empty_bootstrap(), SimTick::ZERO);

        assert_eq!(runtime.current_tick(), SimTick::ZERO);
    }

    use crate::agent::{
        ActorKnownFact, ActorKnownPlanningContext, Intention, IntentionSource, RoutineExecution,
        RoutineStepStatus, SourceEventIds,
    };
    use crate::events::PayloadField;
    use crate::ids::{
        CandidateGoalId, DecisionTraceId, IntentionId, RoutineExecutionId, RoutineTemplateId,
    };
    use crate::scheduler::ProposalSequence;

    fn perception_test_event(
        event_id: &str,
        kind: EventKind,
        actor_id: Option<&ActorId>,
        tick: SimTick,
        place_id: Option<&PlaceId>,
        payload: Vec<PayloadField>,
    ) -> EventEnvelope {
        let ordering_actor = actor_id
            .cloned()
            .unwrap_or_else(|| ActorId::new("actor_perception_ordering").unwrap());
        let mut event = EventEnvelope::new_v1(
            EventId::new(event_id).unwrap(),
            kind,
            0,
            0,
            tick,
            OrderingKey::new(
                tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Actor(ordering_actor),
                ProposalSequence::new(0),
                ActionId::new("noop").unwrap(),
                Vec::new(),
                format!("test_event:{event_id}"),
            ),
            manifest_id(),
        );
        event.actor_id = actor_id.cloned();
        event.place_id = place_id.cloned();
        event.payload = payload;
        event
    }

    #[test]
    fn latest_current_place_perception_event_id_requires_all_four_dimensions() {
        let actor = ActorId::new("actor_perception").unwrap();
        let other_actor = ActorId::new("actor_other").unwrap();
        let place = PlaceId::new("place_perception").unwrap();
        let other_place = PlaceId::new("place_other").unwrap();
        let tick = SimTick::new(5);
        let other_tick = SimTick::new(4);

        // No event matches all four of {type, actor, tick, place}. Each decoy
        // misses on a different dimension, so broadening any single `&&` to `||`
        // would return a decoy instead of the required `None`.
        let mut log = EventLog::new();
        log.append(perception_test_event(
            "event.perception.observation_wrong_dims",
            EventKind::ObservationRecorded,
            Some(&other_actor),
            other_tick,
            Some(&other_place),
            Vec::new(),
        ))
        .unwrap();
        log.append(perception_test_event(
            "event.perception.right_dims_wrong_type",
            EventKind::NeedDeltaApplied,
            Some(&other_actor),
            tick,
            Some(&place),
            Vec::new(),
        ))
        .unwrap();

        assert_eq!(
            latest_current_place_perception_event_id(&log, &actor, tick, &place),
            None
        );

        // A fully matching observation is selected.
        log.append(perception_test_event(
            "event.perception.match",
            EventKind::ObservationRecorded,
            Some(&actor),
            tick,
            Some(&place),
            Vec::new(),
        ))
        .unwrap();
        assert_eq!(
            latest_current_place_perception_event_id(&log, &actor, tick, &place),
            Some(EventId::new("event.perception.match").unwrap())
        );
    }

    #[test]
    fn latest_need_event_id_matches_direct_actor_then_payload() {
        let actor = ActorId::new("actor_need").unwrap();

        // Direct actor_id match with no payload: a `==`->`!=` on the actor check
        // or a `||`->`&&` across the actor/payload branch would drop this match.
        let mut direct_log = EventLog::new();
        direct_log
            .append(perception_test_event(
                "event.need.direct",
                EventKind::NeedDeltaApplied,
                Some(&actor),
                SimTick::new(3),
                None,
                Vec::new(),
            ))
            .unwrap();
        assert_eq!(
            latest_need_event_id(&direct_log, &actor),
            Some(EventId::new("event.need.direct").unwrap())
        );

        // Payload-only match (actor_id absent on the envelope, present in the
        // payload): a `==`->`!=` on either the payload key or value comparison
        // would drop it.
        let mut payload_log = EventLog::new();
        payload_log
            .append(perception_test_event(
                "event.need.payload",
                EventKind::NeedDeltaApplied,
                None,
                SimTick::new(3),
                None,
                vec![PayloadField::new("actor_id", actor.as_str())],
            ))
            .unwrap();
        assert_eq!(
            latest_need_event_id(&payload_log, &actor),
            Some(EventId::new("event.need.payload").unwrap())
        );

        // A payload field whose value does not match must not qualify; a
        // `&&`->`||` across the payload key/value comparison would wrongly
        // accept it.
        let mut mismatch_log = EventLog::new();
        mismatch_log
            .append(perception_test_event(
                "event.need.mismatch",
                EventKind::NeedDeltaApplied,
                None,
                SimTick::new(3),
                None,
                vec![PayloadField::new("actor_id", "actor_someone_else")],
            ))
            .unwrap();
        assert_eq!(latest_need_event_id(&mismatch_log, &actor), None);
    }

    fn window_execution(
        execution_id: &str,
        actor: &ActorId,
        template: &RoutineTemplateId,
        family: RoutineFamily,
        status: RoutineStepStatus,
    ) -> RoutineExecution {
        let mut execution = RoutineExecution::new(
            RoutineExecutionId::new(execution_id).unwrap(),
            actor.clone(),
            template.clone(),
            family,
            SimTick::new(0),
            None,
            None,
            None,
            DecisionTraceId::new(format!("trace_{execution_id}")).unwrap(),
        );
        execution.step_status = status;
        execution
    }

    fn window_agent_state(
        actor: &ActorId,
        template: &RoutineTemplateId,
        executions: Vec<RoutineExecution>,
    ) -> AgentState {
        let mut state = AgentState::default();
        let intention_id = IntentionId::new("intention_window_test").unwrap();
        state.intentions.insert(
            intention_id.clone(),
            Intention::adopt(
                intention_id.clone(),
                actor.clone(),
                IntentionSource::RoutineDuty,
                CandidateGoalId::new("candidate_window_test").unwrap(),
                Some(template.clone()),
                None,
                1,
                SimTick::new(0),
                DecisionTraceId::new("trace_window_intention").unwrap(),
            ),
        );
        state
            .active_intention_by_actor
            .insert(actor.clone(), intention_id);
        for execution in executions {
            state
                .routine_executions
                .insert(execution.execution_id.clone(), execution);
        }
        state
    }

    fn window_context(actor: &ActorId) -> ActorKnownPlanningContext {
        // Empty facts derive no known workplaces, so a selected WorkBlock
        // routine is refined to GoToWork after active-intention selection.
        ActorKnownPlanningContext::from_observed_parts(
            actor.clone(),
            PlaceId::new("place_window_test").unwrap(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            Vec::new(),
        )
    }

    #[test]
    fn embodied_continue_uses_active_intention_current_step_not_known_workplace() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let template = RoutineTemplateId::new("routine_eat_meal").unwrap();
        let state = window_agent_state(
            &actor,
            &template,
            vec![window_execution(
                "routine_exec_active_eat",
                &actor,
                &template,
                RoutineFamily::EatMeal,
                RoutineStepStatus::InProgress,
            )],
        );
        // A known workplace at the actor's current place is only actor-known
        // context. It may not upgrade the active eat step into WorkBlock.
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor.clone(),
            PlaceId::new("place_window_test").unwrap(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![ActorKnownFact::observed_now(
                actor.clone(),
                "actor_knows_workplace",
                "workplace_window@place_window_test",
                "test:resolver",
                Some(SimTick::new(0)),
                SourceEventIds::checked(vec![EventId::new("event_window_workplace").unwrap()])
                    .unwrap(),
            )],
        );

        assert_eq!(
            embodied_routine_window_family(&state, &actor, &context),
            Some(RoutineFamily::EatMeal)
        );
    }

    #[test]
    fn embodied_routine_window_family_refines_work_block_to_go_to_work_when_not_at_known_workplace()
    {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let template = RoutineTemplateId::new("routine_work_block").unwrap();
        let state = window_agent_state(
            &actor,
            &template,
            vec![window_execution(
                "routine_exec_active_work",
                &actor,
                &template,
                RoutineFamily::WorkBlock,
                RoutineStepStatus::InProgress,
            )],
        );
        let context = window_context(&actor);

        assert_eq!(
            embodied_routine_window_family(&state, &actor, &context),
            Some(RoutineFamily::GoToWork)
        );
    }

    #[test]
    fn embodied_continue_assigned_inactive_window_does_not_drive_follow_on() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let active_template = RoutineTemplateId::new("routine_eat_meal").unwrap();
        let inactive_template = RoutineTemplateId::new("routine_work_block").unwrap();
        let state = window_agent_state(
            &actor,
            &active_template,
            vec![
                window_execution(
                    "routine_exec_active_eat",
                    &actor,
                    &active_template,
                    RoutineFamily::EatMeal,
                    RoutineStepStatus::InProgress,
                ),
                window_execution(
                    "routine_exec_inactive_work",
                    &actor,
                    &inactive_template,
                    RoutineFamily::WorkBlock,
                    RoutineStepStatus::InProgress,
                ),
            ],
        );
        let context = window_context(&actor);

        assert_eq!(
            embodied_routine_window_family(&state, &actor, &context),
            Some(RoutineFamily::EatMeal)
        );
    }

    #[test]
    fn embodied_routine_window_family_requires_active_intention_before_workplace_context() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor.clone(),
            PlaceId::new("place_window_test").unwrap(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![ActorKnownFact::observed_now(
                actor.clone(),
                "actor_knows_workplace",
                "workplace_window@place_window_test",
                "test:resolver",
                Some(SimTick::new(0)),
                SourceEventIds::checked(vec![EventId::new("event_window_workplace").unwrap()])
                    .unwrap(),
            )],
        );

        assert_eq!(
            embodied_routine_window_family(&AgentState::default(), &actor, &context),
            None
        );
    }

    #[test]
    fn embodied_routine_window_family_filters_executions_by_actor() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let other = ActorId::new("actor_window_other").unwrap();
        let template = RoutineTemplateId::new("routine_go_to_work").unwrap();
        let state = window_agent_state(
            &actor,
            &template,
            vec![
                window_execution(
                    "routine_exec_primary",
                    &actor,
                    &template,
                    RoutineFamily::EatMeal,
                    RoutineStepStatus::InProgress,
                ),
                window_execution(
                    "routine_exec_other",
                    &other,
                    &template,
                    RoutineFamily::SleepNight,
                    RoutineStepStatus::InProgress,
                ),
            ],
        );
        let context = window_context(&actor);

        // Only the requested actor's execution may drive the family; a `==`->`!=`
        // actor filter would surface the other actor's `SleepNight` execution.
        assert_eq!(
            embodied_routine_window_family(&state, &actor, &context),
            Some(RoutineFamily::EatMeal)
        );
    }

    #[test]
    fn embodied_routine_window_family_ignores_resolved_executions() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let template = RoutineTemplateId::new("routine_go_to_work").unwrap();
        let state = window_agent_state(
            &actor,
            &template,
            vec![
                window_execution(
                    "routine_exec_resolved",
                    &actor,
                    &template,
                    RoutineFamily::SleepNight,
                    RoutineStepStatus::Completed,
                ),
                window_execution(
                    "routine_exec_active",
                    &actor,
                    &template,
                    RoutineFamily::EatMeal,
                    RoutineStepStatus::InProgress,
                ),
            ],
        );
        let context = window_context(&actor);

        // Only the unresolved execution may drive the family; deleting the `!`
        // on the resolved filter would surface the resolved `SleepNight` one.
        assert_eq!(
            embodied_routine_window_family(&state, &actor, &context),
            Some(RoutineFamily::EatMeal)
        );
    }

    #[test]
    fn embodied_continue_wait_follow_on_is_not_direct_pipelined() {
        let actor = ActorId::new("actor_window_primary").unwrap();
        let mut proposal = crate::actions::Proposal::new(
            crate::ids::ProposalId::new("proposal_embodied_wait_follow_on").unwrap(),
            crate::actions::ProposalOrigin::Agent,
            Some(actor.clone()),
            ActionId::new("wait").unwrap(),
            SimTick::new(7),
        );
        proposal.parameters.insert(
            "routine_template_id".to_string(),
            "routine_wait_idle".to_string(),
        );
        proposal.parameters.insert(
            "routine_execution_id".to_string(),
            "routine_exec_wait_idle".to_string(),
        );

        assert!(embodied_follow_on_advances_time(&proposal));
        let diagnostic =
            embodied_time_advancing_follow_on_diagnostic(&actor, SimTick::new(7), &proposal);

        assert_eq!(
            diagnostic.concrete_blocker,
            "time-advancing follow-on requires scheduler authority"
        );
        assert_eq!(
            diagnostic.actor_known_explanation,
            "routine continuation cannot safely commit a time-advancing follow-on yet"
        );
        assert_eq!(
            diagnostic.debug_only_details,
            "embodied continue_routine resolved to wait; scheduler-owned routing is deferred"
        );
        assert_eq!(diagnostic.resulting_status, StuckResultingStatus::Failed);
    }
}
