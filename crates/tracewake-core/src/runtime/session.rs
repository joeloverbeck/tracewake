use crate::actions::{
    run_pipeline, ActionRegistry, PipelineContext, PipelineResult, ReportStatus, ValidationReport,
};
use crate::agent::current_place_knowledge_context;
use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::controller::ControllerBindings;
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
use crate::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, DebugReportId, FixtureId, ItemId,
    ProposalId,
};
use crate::projections::{
    build_debug_event_log_view, build_embodied_view_model, build_notebook_view,
    proposal_from_current_view_semantic_action, EmbodiedPreflightSource, EmbodiedProjectionSource,
    EmbodiedTruthSnapshot, ProjectionError,
};
use crate::replay::{rebuild_projection, run_replay};
use crate::scheduler::no_human::{
    default_day_windows, run_no_human_day, NoHumanDayConfig, NoHumanDayReport,
};
use crate::scheduler::{
    AdvanceUntilRequest, AdvanceUntilResult, DeterministicScheduler, OrderingKey, SchedulePhase,
    SchedulerSourceId, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
    WorldStepTransactionRequest,
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

/// Opaque replay seed for reconstructing the accepted initial aggregates
/// without retaining mutable runtime state in a client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReplaySeed {
    bootstrap: LoadedWorldBootstrap,
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

impl LoadedWorldBootstrap {
    #[allow(clippy::too_many_arguments)]
    pub fn from_loaded_state(
        registry: ActionRegistry,
        physical_state: PhysicalState,
        agent_state: AgentState,
        event_log: EventLog,
        epistemic_projection: EpistemicProjection,
        content_manifest_id: ContentManifestId,
        fixture_id: FixtureId,
        content_version: ContentVersion,
    ) -> Self {
        Self {
            registry,
            physical_state,
            agent_state,
            event_log,
            epistemic_projection,
            controller_bindings: ControllerBindings::new(),
            content_manifest_id,
            fixture_id,
            content_version,
        }
    }

    pub fn replay_seed(&self) -> RuntimeReplaySeed {
        RuntimeReplaySeed {
            bootstrap: self.clone(),
        }
    }
}

impl RuntimeReplaySeed {
    pub fn reconstruct_bootstrap(&self) -> LoadedWorldBootstrap {
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

    pub fn from_bootstrap(bootstrap: LoadedWorldBootstrap, current_tick: SimTick) -> Self {
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
        let preflight = EmbodiedPreflightSource::new(
            &self.physical_state,
            &self.registry,
            &self.content_manifest_id,
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
                        origin: WorldAdvanceOrigin::Controller(controller_id),
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
                SchedulerSourceId::Controller(controller_id),
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
        Ok(result)
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
            RuntimeCommandKind::DebugView { .. } => {
                Ok(RuntimeReceipt::debug(DebugRuntimeReceipt::new(
                    crate::debug_capability::DebugCapability::mint(),
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

    fn run_one_tick_wait(
        &mut self,
        origin: WorldAdvanceOrigin,
    ) -> Result<RuntimeReceipt, RuntimeCommandError> {
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

        Ok(RuntimeReceipt::one_tick_advanced(result))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ControllerId;
    use crate::runtime::RuntimeReceiptKind;
    use crate::state::NeedModelState;
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

    fn empty_bootstrap() -> LoadedWorldBootstrap {
        LoadedWorldBootstrap::from_loaded_state(
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
            RuntimeReceiptKind::OneTickAdvanced(result) => {
                assert_eq!(result.prior_tick, SimTick::ZERO);
                assert_eq!(result.resulting_tick, SimTick::new(1));
                assert!(!result.appended_event_ids.is_empty());
            }
            _ => panic!("expected one-tick receipt"),
        }
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
                if result.prior_tick == SimTick::ZERO && result.resulting_tick == SimTick::new(1)
        ));
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
}
