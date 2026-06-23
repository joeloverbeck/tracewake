use tracewake_content::fixtures::{self, GoldenFixture};
use tracewake_content::load::{load_fixture_package, LoadError};
use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, PipelineResult, ReportStatus, ValidationReport,
};
use tracewake_core::agent::{
    current_place_knowledge_context, record_current_place_perception_and_project,
};
use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::debug_reports::{
    action_rejection_report, controller_binding_report, item_location_report,
    no_human_day_debug_report, phase3a_actor_report, phase3a_needs_report, phase3a_planner_report,
    phase3a_routines_report, phase3a_stuck_report, projection_rebuild_debug_report,
    replay_debug_report,
};
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::epistemics::ActorKnownIntervalDeltaError;
use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, DebugReportId, FixtureId, ItemId,
    ProposalId, SemanticActionId,
};
use tracewake_core::projections::{
    build_debug_event_log_view, build_embodied_view_model, build_notebook_view,
    proposal_from_current_view_semantic_action, EmbodiedPreflightSource, EmbodiedProjectionSource,
    EmbodiedTruthSnapshot, IntervalStopReason, ProjectionError,
};
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{
    default_day_windows, run_no_human_day, NoHumanDayConfig, NoHumanDayReport,
};
use tracewake_core::scheduler::{
    AdvanceUntilRequest, AdvanceUntilResult, DeterministicScheduler, OrderingKey, SchedulePhase,
    SchedulerSourceId, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
    WorldStepTransactionRequest,
};
use tracewake_core::state::{AgentState, ControllerMode, PhysicalState};
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{
    DebugBeliefsView, DebugEpistemicsView, DebugObservationsView, EmbodiedViewModel, NotebookView,
    SemanticActionEntry, TypedActorKnownIntervalSummary,
};

use crate::debug_panels::{
    render_action_rejection_panel, render_controller_binding_panel, render_event_log_panel,
    render_item_location_panel, render_no_human_day_panel, render_phase3a_debug_panel,
    render_projection_rebuild_panel, render_replay_panel,
};
use crate::render::{render_debug_overlay, render_embodied_view};

#[derive(Debug)]
pub enum AppError {
    Load(LoadError),
    ActorNotFound(ActorId),
    ActorNotBound,
    DebugUnavailable,
    ActorKnownIntervalDelta(ActorKnownIntervalDeltaError),
    Projection(ProjectionError),
    SchedulerRestoreFailed,
    SemanticActionNotFound(String),
    WorldAdvance(WorldAdvanceError),
}

impl From<LoadError> for AppError {
    fn from(value: LoadError) -> Self {
        Self::Load(value)
    }
}

impl From<ProjectionError> for AppError {
    fn from(value: ProjectionError) -> Self {
        Self::Projection(value)
    }
}

pub struct TuiApp {
    registry: ActionRegistry,
    initial_state: PhysicalState,
    initial_agent_state: AgentState,
    state: PhysicalState,
    agent_state: AgentState,
    log: EventLog,
    controller_bindings: ControllerBindings,
    controller_id: ControllerId,
    bound_actor_id: Option<ActorId>,
    content_manifest_id: ContentManifestId,
    fixture_id: FixtureId,
    content_version: ContentVersion,
    scheduler: DeterministicScheduler,
    last_rejection: Option<ValidationReport>,
    epistemic_projection: EpistemicProjection,
    last_interval_summary: Option<TypedActorKnownIntervalSummary>,
}

impl TuiApp {
    pub fn load_default() -> Result<Self, AppError> {
        Self::from_golden(fixtures::strongbox_001())
    }

    pub fn from_golden(golden: GoldenFixture) -> Result<Self, AppError> {
        let loaded = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )?;
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        registry.register_phase1_take_place();
        registry.register_phase1_inspect_wait();
        registry.register_phase2a_epistemics();
        registry.register_phase3a_sleep();
        registry.register_phase3a_eat();
        registry.register_phase3a_work();
        registry.register_phase3a_continue_routine();
        let epistemic_projection = loaded.epistemic_projection;
        Ok(Self {
            registry,
            initial_state: loaded.canonical_world.clone(),
            initial_agent_state: loaded.canonical_agent_state.clone(),
            state: loaded.canonical_world,
            agent_state: loaded.canonical_agent_state,
            log: loaded.seed_event_log,
            controller_bindings: ControllerBindings::new(),
            controller_id: ControllerId::new("controller_human").unwrap(),
            bound_actor_id: None,
            content_manifest_id: loaded.manifest.manifest_id,
            fixture_id: loaded.manifest.fixture_id,
            content_version: loaded.manifest.content_version,
            scheduler: DeterministicScheduler::new(SimTick::ZERO),
            last_rejection: None,
            epistemic_projection,
            last_interval_summary: None,
        })
    }

    pub fn bind_actor(&mut self, actor_id: ActorId) -> Result<(), AppError> {
        self.bind_actor_with_mode(actor_id, ControllerMode::Embodied)
    }

    pub fn bind_debug_actor(&mut self, actor_id: ActorId) -> Result<(), AppError> {
        self.bind_actor_with_mode(actor_id, ControllerMode::Debug)
    }

    fn bind_actor_with_mode(
        &mut self,
        actor_id: ActorId,
        mode: ControllerMode,
    ) -> Result<(), AppError> {
        if !self.state.actors().contains_key(&actor_id) {
            return Err(AppError::ActorNotFound(actor_id));
        }
        self.controller_bindings.attach(
            self.controller_id.clone(),
            actor_id.clone(),
            mode,
            self.scheduler.current_tick(),
            &mut self.log,
            self.content_manifest_id.clone(),
        );
        record_current_place_perception_and_project(
            &mut self.log,
            &mut self.state,
            &mut self.agent_state,
            &mut self.epistemic_projection,
            &actor_id,
            self.scheduler.current_tick(),
            &self.content_manifest_id,
        );
        self.bound_actor_id = Some(actor_id);
        self.last_rejection = None;
        Ok(())
    }

    pub fn current_view(&self) -> Result<EmbodiedViewModel, AppError> {
        let actor_id = self
            .bound_actor_id
            .as_ref()
            .ok_or(AppError::ActorNotBound)?;
        let context = self.current_view_context(actor_id);
        let snapshot = EmbodiedTruthSnapshot::from_physical_state(&context, &self.state);
        let source = EmbodiedProjectionSource::from_sealed_context(
            &context,
            snapshot,
            Some(&self.agent_state),
        );
        let preflight =
            EmbodiedPreflightSource::new(&self.state, &self.registry, &self.content_manifest_id);
        let mut view =
            build_embodied_view_model(&context, &source, &preflight, self.last_rejection.as_ref())
                .map_err(AppError::from)?;
        view.notebook = Some(build_notebook_view(&self.epistemic_projection, &context));
        view.debug_available = self.debug_available_for(actor_id);
        view.actor_known_interval_summary = self.last_interval_summary.clone();
        Ok(view)
    }

    fn debug_available_for(&self, actor_id: &ActorId) -> bool {
        self.controller_bindings
            .binding(&self.controller_id)
            .is_some_and(|binding| {
                binding.binding.bound_actor_id.as_ref() == Some(actor_id)
                    && matches!(binding.binding.mode, ControllerMode::Debug)
            })
    }

    pub fn debug_available(&self) -> bool {
        self.bound_actor_id
            .as_ref()
            .is_some_and(|actor_id| self.debug_available_for(actor_id))
    }

    fn current_view_context(&self, actor_id: &ActorId) -> KnowledgeContext {
        current_place_knowledge_context(
            &self.state,
            Some(&self.epistemic_projection),
            actor_id,
            self.scheduler.current_tick(),
            &self.content_manifest_id,
            self.log.events().len() as u64,
        )
    }

    pub fn render_current_view(&self) -> Result<String, AppError> {
        Ok(render_embodied_view(&self.current_view()?))
    }

    pub fn render_debug_embodied_overlay(&self) -> Result<Option<String>, AppError> {
        let view = self.current_view()?;
        if view.debug_available {
            Ok(Some(render_debug_overlay(&view)))
        } else {
            Ok(None)
        }
    }

    pub fn submit_semantic_action(
        &mut self,
        semantic_action_id: &SemanticActionId,
    ) -> Result<PipelineResult, AppError> {
        let view = self.current_view()?;
        let entry = view
            .semantic_actions
            .iter()
            .find(|entry| &entry.semantic_action_id == semantic_action_id)
            .cloned()
            .ok_or_else(|| AppError::SemanticActionNotFound(semantic_action_id.to_string()))?;
        self.submit_entry(&entry, &view)
    }

    fn submit_entry(
        &mut self,
        entry: &SemanticActionEntry,
        source_view: &EmbodiedViewModel,
    ) -> Result<PipelineResult, AppError> {
        self.submit_entry_with_world_advance(
            entry,
            source_view,
            entry.semantic_action_id.as_str() == "wait.1_tick",
        )
    }

    fn submit_entry_with_world_advance(
        &mut self,
        entry: &SemanticActionEntry,
        source_view: &EmbodiedViewModel,
        advance_world_after_acceptance: bool,
    ) -> Result<PipelineResult, AppError> {
        let actor_id = self.bound_actor_id.clone().ok_or(AppError::ActorNotBound)?;
        let expected_tick = self.scheduler.current_tick();
        let sequence = self.scheduler.assign_proposal_sequence();
        let proposal = proposal_from_current_view_semantic_action(
            ProposalId::new(format!("proposal_tui_{}", sequence.value())).unwrap(),
            actor_id.clone(),
            expected_tick,
            entry,
            source_view,
            &self.controller_id,
        );

        let result = if advance_world_after_acceptance {
            let step = self
                .scheduler
                .transact_world_one_tick(
                    &mut self.state,
                    &mut self.agent_state,
                    &mut self.log,
                    &self.registry,
                    Some(&self.controller_bindings),
                    Some(&mut self.epistemic_projection),
                    WorldStepTransactionRequest {
                        advance: WorldAdvanceRequest {
                            expected_tick,
                            origin: WorldAdvanceOrigin::Controller(self.controller_id.clone()),
                            content_manifest_id: self.content_manifest_id.clone(),
                            authorized_sleep_interruptions: Vec::new(),
                        },
                        controlled_proposals: vec![proposal],
                        due_actor_ids: Vec::new(),
                        actor_known_interval_actor_id: None,
                        world_process_events: Vec::new(),
                    },
                )
                .map_err(AppError::WorldAdvance)?;
            step.controlled_pipeline_results
                .into_iter()
                .next()
                .expect("submitted proposal produces one pipeline result")
        } else {
            let ordering_key = OrderingKey::new(
                expected_tick,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Controller(self.controller_id.clone()),
                sequence,
                proposal.action_id.clone(),
                proposal.target_ids.clone(),
                proposal.proposal_id.as_str().to_string(),
            );
            let mut context = PipelineContext {
                registry: &self.registry,
                state: &mut self.state,
                agent_state: &mut self.agent_state,
                log: &mut self.log,
                controller_bindings: Some(&self.controller_bindings),
                epistemic_projection: Some(&mut self.epistemic_projection),
                content_manifest_id: self.content_manifest_id.clone(),
                ordering_key,
            };
            run_pipeline(&mut context, &proposal)
        };
        if result.report.status == ReportStatus::Rejected {
            self.last_rejection = Some(result.report.clone());
        } else {
            self.last_rejection = None;
            if let Some(actor_id) = self.bound_actor_id.clone() {
                record_current_place_perception_and_project(
                    &mut self.log,
                    &mut self.state,
                    &mut self.agent_state,
                    &mut self.epistemic_projection,
                    &actor_id,
                    self.scheduler.current_tick(),
                    &self.content_manifest_id,
                );
            }
        }
        Ok(result)
    }

    pub fn event_count(&self) -> usize {
        self.log.events().len()
    }

    pub fn advance_until(&mut self, max_ticks: u64) -> Result<AdvanceUntilResult, AppError> {
        let actor_id = self.bound_actor_id.clone().ok_or(AppError::ActorNotBound)?;
        let before_context = self.current_view_context(&actor_id);
        let result = self
            .scheduler
            .advance_until(
                &mut self.state,
                &mut self.agent_state,
                &mut self.log,
                &self.registry,
                Some(&mut self.epistemic_projection),
                AdvanceUntilRequest {
                    expected_tick: self.scheduler.current_tick(),
                    origin: WorldAdvanceOrigin::Controller(self.controller_id.clone()),
                    content_manifest_id: self.content_manifest_id.clone(),
                    possessed_actor_id: actor_id.clone(),
                    max_ticks,
                },
            )
            .map_err(AppError::WorldAdvance)?;
        record_current_place_perception_and_project(
            &mut self.log,
            &mut self.state,
            &mut self.agent_state,
            &mut self.epistemic_projection,
            &actor_id,
            self.scheduler.current_tick(),
            &self.content_manifest_id,
        );
        let after_context = self.current_view_context(&actor_id);
        self.last_interval_summary = Some(
            self.epistemic_projection
                .actor_known_interval_delta(
                    &before_context,
                    &after_context,
                    interval_stop_reason(result.stop_reason),
                )
                .map(TypedActorKnownIntervalSummary::from)
                .map_err(AppError::ActorKnownIntervalDelta)?,
        );
        self.last_rejection = None;
        Ok(result)
    }

    pub fn run_no_human_day(&mut self) -> Result<NoHumanDayReport, AppError> {
        if !self.debug_available() {
            return Err(AppError::DebugUnavailable);
        }
        let windows = default_day_windows(self.scheduler.current_tick());
        let report = run_no_human_day(
            &mut self.state,
            &mut self.agent_state,
            &mut self.log,
            &self.registry,
            self.content_manifest_id.clone(),
            NoHumanDayConfig {
                actor_ids: Vec::new(),
                windows,
            },
        );
        let rebuild = rebuild_projection(
            &self.initial_state,
            &self.initial_agent_state,
            &self.log,
            &self.checksum_context(),
            Some(&self.state),
        );
        self.scheduler = DeterministicScheduler::restore_from_rebuild_report(&rebuild)
            .ok_or(AppError::SchedulerRestoreFailed)?;
        self.state = rebuild.final_state;
        self.agent_state = rebuild.final_agent_state;
        self.epistemic_projection = rebuild.final_epistemic_projection;
        if let Some(actor_id) = self.bound_actor_id.clone() {
            record_current_place_perception_and_project(
                &mut self.log,
                &mut self.state,
                &mut self.agent_state,
                &mut self.epistemic_projection,
                &actor_id,
                self.scheduler.current_tick(),
                &self.content_manifest_id,
            );
        }
        self.last_rejection = None;
        Ok(report)
    }

    pub fn physical_checksum(&self) -> PhysicalChecksum {
        compute_physical_checksum(&self.state, &self.checksum_context()).checksum
    }

    pub fn checksum_context(&self) -> ChecksumContext {
        self.checksum_context_at(self.scheduler.current_tick())
    }

    pub fn checksum_context_at(&self, sim_tick: SimTick) -> ChecksumContext {
        ChecksumContext {
            fixture_id: self.fixture_id.clone(),
            content_version: self.content_version.clone(),
            sim_tick,
            world_stream_position_applied: self
                .log
                .events()
                .iter()
                .filter(|event| event.stream == tracewake_core::events::EventStream::World)
                .count()
                .saturating_sub(1) as u64,
        }
    }

    pub fn render_debug_event_log_panel(&self) -> String {
        render_event_log_panel(&build_debug_event_log_view(&self.log))
    }

    pub fn render_debug_controller_binding_panel(&self) -> String {
        let report = controller_binding_report(
            DebugReportId::new("debug.controller_bindings").unwrap(),
            &self.controller_bindings,
        );
        render_controller_binding_panel(&report)
    }

    pub fn render_debug_item_location_panel(&self, item_id: &ItemId) -> String {
        let report =
            item_location_report(&self.state, &self.log, item_id, &self.checksum_context());
        render_item_location_panel(&report)
    }

    pub fn render_debug_action_rejection_panel(&self) -> Option<String> {
        let report = self.last_rejection.as_ref()?;
        Some(render_action_rejection_panel(&action_rejection_report(
            report,
            &self.state,
            &self.checksum_context(),
        )))
    }

    pub fn render_debug_projection_rebuild_panel(&self) -> String {
        let report = rebuild_projection(
            &self.initial_state,
            &self.initial_agent_state,
            &self.log,
            &self.checksum_context(),
            Some(&self.state),
        );
        render_projection_rebuild_panel(&projection_rebuild_debug_report(
            DebugReportId::new("debug.projection_rebuild").unwrap(),
            report,
        ))
    }

    pub fn render_debug_replay_panel(&self) -> String {
        let replay_context = self.checksum_context_at(SimTick::ZERO);
        let report = run_replay(
            &self.initial_state,
            &self.initial_agent_state,
            &self.log,
            &replay_context,
            Some(&self.state),
            None,
            None,
        );
        render_replay_panel(&replay_debug_report(
            DebugReportId::new("debug.replay").unwrap(),
            report,
        ))
    }

    pub fn render_debug_needs_panel(&self) -> String {
        render_phase3a_debug_panel(&phase3a_needs_report(&self.agent_state))
    }

    pub fn render_debug_routines_panel(&self) -> String {
        render_phase3a_debug_panel(&phase3a_routines_report(&self.agent_state))
    }

    pub fn render_debug_planner_panel(&self, actor_id: &ActorId) -> String {
        render_phase3a_debug_panel(&phase3a_planner_report(&self.agent_state, actor_id))
    }

    pub fn render_debug_stuck_panel(&self) -> String {
        render_phase3a_debug_panel(&phase3a_stuck_report(&self.agent_state))
    }

    pub fn render_debug_no_human_day_panel(&self) -> String {
        render_no_human_day_panel(&no_human_day_debug_report(&self.log))
    }

    pub fn render_debug_actor_panel(&self, actor_id: &ActorId) -> String {
        render_phase3a_debug_panel(&phase3a_actor_report(&self.agent_state, actor_id))
    }

    pub fn notebook_view(&self) -> Result<NotebookView, AppError> {
        self.current_view()?.notebook.ok_or(AppError::ActorNotBound)
    }

    pub fn debug_epistemics_view(&self) -> DebugEpistemicsView {
        self.epistemic_projection.debug_epistemics_view()
    }

    pub fn debug_beliefs_view(&self, actor_id: &ActorId) -> Result<DebugBeliefsView, AppError> {
        if !self.state.actors().contains_key(actor_id) {
            return Err(AppError::ActorNotFound(actor_id.clone()));
        }
        Ok(self
            .epistemic_projection
            .debug_beliefs_view(actor_id.clone()))
    }

    pub fn debug_observations_view(
        &self,
        actor_id: &ActorId,
    ) -> Result<DebugObservationsView, AppError> {
        if !self.state.actors().contains_key(actor_id) {
            return Err(AppError::ActorNotFound(actor_id.clone()));
        }
        Ok(self
            .epistemic_projection
            .debug_observations_view(actor_id.clone()))
    }
}

fn interval_stop_reason(
    reason: tracewake_core::scheduler::AdvanceUntilStopReason,
) -> IntervalStopReason {
    match reason {
        tracewake_core::scheduler::AdvanceUntilStopReason::PossessedDurationTerminal => {
            IntervalStopReason::PossessedDurationTerminal
        }
        tracewake_core::scheduler::AdvanceUntilStopReason::ActorKnownSalientObservation => {
            IntervalStopReason::ActorKnownSalientObservation
        }
        tracewake_core::scheduler::AdvanceUntilStopReason::UserPausedBeforeNextTick => {
            IntervalStopReason::UserPausedBeforeNextTick
        }
        tracewake_core::scheduler::AdvanceUntilStopReason::ControllerSafetyBound => {
            IntervalStopReason::ControllerSafetyBound
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::ids::SemanticActionId;

    #[test]
    fn app_binds_renders_submits_and_rerenders() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        assert!(!app.current_view().unwrap().debug_available);
        let before = app.render_current_view().unwrap();
        assert!(before.contains("strongbox_tomas"));
        assert!(!before.contains("Debug: available=true"));
        app.controller_bindings.detach(
            &app.controller_id,
            app.scheduler.current_tick(),
            &mut app.log,
            app.content_manifest_id.clone(),
        );
        assert!(!app.current_view().unwrap().debug_available);
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();

        let result = app
            .submit_semantic_action(
                &SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
            )
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Accepted);

        let after = app.render_current_view().unwrap();
        assert!(after.contains("strongbox_tomas"));
        assert!(after.contains("coin_stack_01"));
        assert!(app.event_count() >= 2);
    }

    #[test]
    fn app_debug_overlay_renders_only_for_bound_debug_controller() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();

        let rendered = app.render_debug_embodied_overlay().unwrap().unwrap();
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Embodied Overlay"));
        assert!(rendered.contains("Knowledge context: id=hkc."));

        app.controller_bindings.detach(
            &app.controller_id,
            app.scheduler.current_tick(),
            &mut app.log,
            app.content_manifest_id.clone(),
        );

        assert_eq!(app.render_debug_embodied_overlay().unwrap(), None);
    }

    #[test]
    fn render_functions_have_production_callers_or_documented_allowlist() {
        let render_source = include_str!("render.rs");
        let debug_panels_source = include_str!("debug_panels.rs");
        let app_source = include_str!("app.rs");
        let run_source = include_str!("run.rs");
        let transcript_source = include_str!("transcript.rs");

        assert!(render_reachability_errors(
            &[render_source, debug_panels_source],
            &[
                production_source(app_source),
                production_source(run_source),
                production_source(transcript_source)
            ]
        )
        .is_empty());
    }

    #[test]
    fn render_reachability_guard_fires_on_synthetic_uncalled_renderer() {
        let debug_panels_source = format!(
            "{}\npub fn render_synthetic_uncalled_panel() -> String {{ String::new() }}\n",
            include_str!("debug_panels.rs")
        );
        let render_source = include_str!("render.rs");
        let app_source = include_str!("app.rs");
        let run_source = include_str!("run.rs");
        let transcript_source = include_str!("transcript.rs");

        let errors = render_reachability_errors(
            &[render_source, &debug_panels_source],
            &[
                production_source(app_source),
                production_source(run_source),
                production_source(transcript_source),
            ],
        );

        assert!(
            errors
                .iter()
                .any(|error| error.contains("render_synthetic_uncalled_panel")),
            "synthetic uncalled render function was not reported: {errors:?}"
        );
    }

    #[test]
    fn debug_dispatch_routes_every_arm_through_availability_gate() {
        let run_source = production_source(include_str!("run.rs"));
        let input_source = production_source(include_str!("input.rs"));

        assert!(debug_command_gating_errors(run_source, input_source).is_empty());

        let synthetic_ungated = run_source.replace(
            "    if !app.debug_available() {\n        return writeln!(writer, \"Error: debug unavailable\");\n    }\n",
            "",
        );
        let errors = debug_command_gating_errors(&synthetic_ungated, input_source);
        assert!(
            errors.iter().any(|error| error.contains("debug_available")),
            "synthetic_ungated_debug_command_arm did not trigger: {errors:?}"
        );

        let synthetic_unrouted = input_source.replace(
            "    Actor(ActorId),\n",
            "    Actor(ActorId),\n    SyntheticUnrouted,\n",
        );
        let errors = debug_command_gating_errors(run_source, &synthetic_unrouted);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("SyntheticUnrouted")),
            "synthetic_unrouted_debug_command_variant did not trigger: {errors:?}"
        );

        let synthetic_early_return = run_source.replace(
            "fn render_debug<W: Write>(\n    app: &mut TuiApp,\n    debug_command: DebugCommand,\n    writer: &mut W,\n) -> std::io::Result<()> {\n",
            "fn render_debug<W: Write>(\n    app: &mut TuiApp,\n    debug_command: DebugCommand,\n    writer: &mut W,\n) -> std::io::Result<()> {\n    return Ok(());\n",
        );
        let errors = debug_command_gating_errors(&synthetic_early_return, input_source);
        assert!(
            errors.iter().any(|error| error.contains("early return")),
            "synthetic_early_return_before_gate did not trigger: {errors:?}"
        );
    }

    #[test]
    fn rejected_action_surfaces_why_not_from_validation_report() {
        let mut app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
        app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();

        let result = app
            .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Rejected);

        let rendered = app.render_current_view().unwrap();
        assert!(rendered.contains("Why-not:"));
        assert!(rendered.contains(&result.report.actor_visible_summary));
    }

    #[test]
    fn app_runs_no_human_day_into_real_log_metrics() {
        let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let before_events = app.event_count();

        let report = app.run_no_human_day().unwrap();

        assert!(report.ordinary_pipeline_events > 0);
        assert!(app.event_count() > before_events);
        assert!(app.render_debug_no_human_day_panel().contains("events="));
        assert!(app.render_debug_no_human_day_panel().contains("canonical="));
    }

    #[test]
    fn run_no_human_day_refuses_intrinsically_without_debug_availability() {
        let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
        let before_events = app.event_count();

        let result = app.run_no_human_day();

        assert!(matches!(result, Err(AppError::DebugUnavailable)));
        assert_eq!(app.event_count(), before_events);
    }

    #[test]
    fn world_advancing_tui_methods_are_intrinsically_debug_gated_or_exempt() {
        let app_source = production_source(include_str!("app.rs"));

        let errors = world_advancing_method_gate_errors(app_source);
        assert!(errors.is_empty(), "{errors:?}");

        let synthetic = app_source.replace(
            "    pub fn event_count(&self) -> usize {\n",
            "    pub fn synthetic_advance_without_gate(&mut self) {\n        self.log.append;\n    }\n\n    pub fn event_count(&self) -> usize {\n",
        );
        let errors = world_advancing_method_gate_errors(&synthetic);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("synthetic_advance_without_gate")),
            "synthetic ungated world advancer did not trigger: {errors:?}"
        );
    }

    #[test]
    fn tui_local_guard_registry_covers_structural_guards() {
        let app_source = include_str!("app.rs");

        assert!(tui_local_guard_registry_errors(app_source).is_empty());

        let synthetic_removed = app_source.replace(
            "fn debug_dispatch_routes_every_arm_through_availability_gate()",
            "fn debug_dispatch_routes_every_arm_through_availability_gate_removed()",
        );
        let errors = tui_local_guard_registry_errors(&synthetic_removed);
        assert!(
            errors
                .iter()
                .any(|error| error
                    .contains("debug_dispatch_routes_every_arm_through_availability_gate")),
            "synthetic removed guard did not trigger: {errors:?}"
        );
    }

    #[test]
    fn controller_mode_debug_availability_decision_is_explicit() {
        let mut app = TuiApp::load_default().unwrap();
        let actor_id = ActorId::new("actor_tomas").unwrap();

        app.bind_actor(actor_id.clone()).unwrap();
        assert!(!app.current_view().unwrap().debug_available);
        assert!(!app.debug_available());

        app.bind_debug_actor(actor_id.clone()).unwrap();
        assert!(app.current_view().unwrap().debug_available);
        assert!(app.debug_available());

        app.controller_bindings.detach(
            &app.controller_id,
            app.scheduler.current_tick(),
            &mut app.log,
            app.content_manifest_id.clone(),
        );
        assert!(!app.current_view().unwrap().debug_available);

        let docs = include_str!(
            "../../../docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md"
        );
        assert!(docs.contains(
            "ControllerMode decision: debug availability requires ControllerMode::Debug"
        ));
    }

    fn render_reachability_errors(render_sources: &[&str], caller_sources: &[&str]) -> Vec<String> {
        render_sources
            .iter()
            .flat_map(|source| render_function_names(source))
            .filter(|name| !render_function_is_called(name, caller_sources))
            .filter(|name| !render_function_allowlisted(name))
            .map(|name| format!("{name} has no production caller"))
            .collect()
    }

    fn render_function_names(source: &str) -> Vec<String> {
        source
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                let rest = trimmed.strip_prefix("pub fn render_")?;
                let name_end = rest.find('(')?;
                Some(format!("render_{}", &rest[..name_end]))
            })
            .collect()
    }

    fn render_function_is_called(name: &str, caller_sources: &[&str]) -> bool {
        caller_sources
            .iter()
            .any(|source| source.matches(name).count() > 1 || source.contains(&format!("{name}(")))
    }

    fn render_function_allowlisted(name: &str) -> bool {
        match name {
            // `render_rejection` is production transcript evidence rather than an interactive
            // panel; it is exercised by `transcript.rs`, not by the command-loop app path.
            "render_rejection" => true,
            _ => false,
        }
    }

    fn debug_command_gating_errors(run_source: &str, input_source: &str) -> Vec<String> {
        let render_debug = run_source.split("fn render_debug").nth(1).unwrap_or("");
        let render_debug_body = function_body_if_present(run_source, "render_debug").unwrap_or("");
        let gate_position = render_debug.find("if !app.debug_available()");
        let match_position = render_debug.find("match debug_command");
        let mut errors = Vec::new();
        match (gate_position, match_position) {
            (Some(gate), Some(dispatch)) if gate < dispatch => {}
            _ => errors.push(
                "render_debug must check debug_available before matching DebugCommand".to_string(),
            ),
        }
        if let Some(gate) = gate_position {
            if render_debug[..gate].contains("return ") {
                errors.push("render_debug has an early return before debug_available".to_string());
            }
        }
        let variants = debug_command_variants(input_source);
        let dispatch_body = match_arm_body(render_debug_body, "match debug_command").unwrap_or("");
        for variant in variants {
            let token = format!("DebugCommand::{variant}");
            let count = dispatch_body.matches(&token).count();
            if count != 1 {
                errors.push(format!("{variant} dispatch count must be 1, got {count}"));
            }
        }
        errors
    }

    fn debug_command_variants(input_source: &str) -> Vec<String> {
        let enum_body = input_source
            .split("pub enum DebugCommand")
            .nth(1)
            .and_then(|tail| tail.split_once('{'))
            .and_then(|(_, tail)| tail.split_once('}'))
            .map(|(body, _)| body)
            .unwrap_or("");
        enum_body
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }
                let name = trimmed.split(['(', ',']).next().unwrap_or("").trim();
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }

    fn world_advancing_method_gate_errors(app_source: &str) -> Vec<String> {
        pub const EXEMPT_WORLD_ADVANCING_METHODS: &[(&str, &str)] = &[(
            "submit_semantic_action",
            "ordinary embodied player command; authorization comes from current view source context",
        ), (
            "advance_until",
            "typed gameplay time-control command; authority remains in the core coordinator loop",
        )];
        public_mut_methods(app_source)
            .into_iter()
            .filter(|(_, body)| {
                method_body_advances_world(body) && !body.contains("debug_available()")
            })
            .filter(|(name, _)| {
                !EXEMPT_WORLD_ADVANCING_METHODS
                    .iter()
                    .any(|(exempt_name, rationale)| exempt_name == name && !rationale.is_empty())
            })
            .map(|(name, _)| {
                format!("{name} advances world state without intrinsic debug_available gate")
            })
            .collect()
    }

    fn method_body_advances_world(body: &str) -> bool {
        body.contains("self.log")
            || body.contains("&mut self.log")
            || body.contains("self.state")
            || body.contains("&mut self.state")
    }

    fn public_mut_methods(source: &str) -> Vec<(String, &str)> {
        let mut methods = Vec::new();
        let mut remaining = source;
        while let Some(position) = remaining.find("pub fn ") {
            remaining = &remaining[position + "pub fn ".len()..];
            let Some((name, after_name)) = remaining.split_once('(') else {
                break;
            };
            let name = name.trim().to_string();
            let Some(body_start) = after_name.find('{') else {
                break;
            };
            let signature = &after_name[..body_start];
            let Some(body) = function_body_after_signature(after_name) else {
                break;
            };
            if signature.contains("&mut self") {
                methods.push((name, body));
            }
            let consumed = body_start + body.len();
            remaining = &after_name[consumed.min(after_name.len())..];
        }
        methods
    }

    fn tui_local_guard_registry_errors(app_source: &str) -> Vec<String> {
        const REQUIRED_TUI_GUARDS: &[&str] = &[
            "render_functions_have_production_callers_or_documented_allowlist",
            "render_reachability_guard_fires_on_synthetic_uncalled_renderer",
            "debug_dispatch_routes_every_arm_through_availability_gate",
            "run_no_human_day_refuses_intrinsically_without_debug_availability",
            "world_advancing_tui_methods_are_intrinsically_debug_gated_or_exempt",
            "tui_local_guard_registry_covers_structural_guards",
            "controller_mode_debug_availability_decision_is_explicit",
        ];
        REQUIRED_TUI_GUARDS
            .iter()
            .filter(|guard| !app_source.contains(&format!("fn {guard}(")))
            .map(|guard| format!("missing TUI local guard registry member {guard}"))
            .collect()
    }

    fn function_body_if_present<'a>(source: &'a str, name: &str) -> Option<&'a str> {
        let marker = format!("fn {name}");
        let tail = source.split(&marker).nth(1)?;
        function_body_after_signature(tail)
    }

    fn function_body_after_signature(source: &str) -> Option<&str> {
        let body_start = source.find('{')?;
        let body = &source[body_start..];
        let mut depth = 0usize;
        for (index, ch) in body.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some(&body[..=index]);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn match_arm_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
        let tail = source.split(marker).nth(1)?;
        function_body_after_signature(tail)
    }

    fn production_source(source: &str) -> &str {
        source.split("\n#[cfg(test)]").next().unwrap_or(source)
    }
}
