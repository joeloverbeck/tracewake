use tracewake_content::fixtures::{self, GoldenFixture};
use tracewake_content::load::{load_fixture_package, LoadError};
use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, PipelineResult, ProposalOrigin, ReportStatus,
    ValidationReport,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext, PhysicalChecksum,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::debug_reports::{
    action_rejection_report, controller_binding_report, item_location_report,
    no_human_day_debug_report, phase3a_actor_report, phase3a_needs_report, phase3a_planner_report,
    phase3a_routines_report, phase3a_stuck_report, projection_rebuild_debug_report,
    replay_debug_report,
};
use tracewake_core::epistemics::belief::HolderKind;
use tracewake_core::epistemics::observation::SourceRef;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, DebugReportId, FixtureId, ItemId,
    ProposalId, SemanticActionId,
};
use tracewake_core::projections::{
    build_debug_event_log_view, build_embodied_view_model, build_notebook_view,
    proposal_from_semantic_action_entry, EmbodiedProjectionSource, ProjectionError,
};
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{
    default_day_windows, run_no_human_day, NoHumanDayConfig, NoHumanDayReport,
};
use tracewake_core::scheduler::{
    DeterministicScheduler, OrderingKey, SchedulePhase, SchedulerSourceId,
};
use tracewake_core::state::{AgentState, PhysicalState};
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{
    DebugBeliefEntry, DebugBeliefsView, DebugContradictionEntry, DebugEpistemicsView,
    DebugHolderBeliefs, DebugObservationEntry, DebugObservationsView, EmbodiedViewModel,
    NotebookView, SemanticActionEntry,
};

use crate::debug_panels::{
    render_action_rejection_panel, render_controller_binding_panel, render_event_log_panel,
    render_item_location_panel, render_no_human_day_panel, render_phase3a_debug_panel,
    render_projection_rebuild_panel, render_replay_panel,
};
use crate::render::render_embodied_view;

#[derive(Debug)]
pub enum AppError {
    Load(LoadError),
    ActorNotFound(ActorId),
    ActorNotBound,
    Projection(ProjectionError),
    SemanticActionNotFound(String),
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
        let mut epistemic_projection =
            EpistemicProjection::new(loaded.manifest.manifest_id.clone());
        for seed in &loaded.fixture.initial_beliefs {
            epistemic_projection.insert_belief(seed.to_belief());
        }
        Ok(Self {
            registry,
            initial_state: loaded.canonical_world.clone(),
            initial_agent_state: loaded.canonical_agent_state.clone(),
            state: loaded.canonical_world,
            agent_state: loaded.canonical_agent_state,
            log: EventLog::new(),
            controller_bindings: ControllerBindings::new(),
            controller_id: ControllerId::new("controller_human").unwrap(),
            bound_actor_id: None,
            content_manifest_id: loaded.manifest.manifest_id,
            fixture_id: loaded.manifest.fixture_id,
            content_version: loaded.manifest.content_version,
            scheduler: DeterministicScheduler::new(SimTick::ZERO),
            last_rejection: None,
            epistemic_projection,
        })
    }

    pub fn bind_actor(&mut self, actor_id: ActorId) -> Result<(), AppError> {
        if !self.state.actors.contains_key(&actor_id) {
            return Err(AppError::ActorNotFound(actor_id));
        }
        self.controller_bindings.attach(
            self.controller_id.clone(),
            actor_id.clone(),
            tracewake_core::state::ControllerMode::Embodied,
            self.scheduler.current_tick,
            &mut self.log,
            self.content_manifest_id.clone(),
        );
        self.bound_actor_id = Some(actor_id);
        Ok(())
    }

    pub fn current_view(&self) -> Result<EmbodiedViewModel, AppError> {
        let actor_id = self
            .bound_actor_id
            .as_ref()
            .ok_or(AppError::ActorNotBound)?;
        let context = tracewake_core::epistemics::KnowledgeContext::embodied_at_frontier(
            actor_id.clone(),
            self.scheduler.current_tick,
            self.log.events().len() as u64,
        );
        let source = EmbodiedProjectionSource::from_sealed_context(
            &context,
            &self.state,
            Some(&self.agent_state),
        );
        let mut view = build_embodied_view_model(
            &context,
            &source,
            &self.registry,
            &self.content_manifest_id,
            self.last_rejection.as_ref(),
        )
        .map_err(AppError::from)?;
        view.notebook = Some(build_notebook_view(&self.epistemic_projection, &context));
        Ok(view)
    }

    pub fn render_current_view(&self) -> Result<String, AppError> {
        Ok(render_embodied_view(&self.current_view()?))
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
        self.submit_entry(&entry, Some(view.view_model_id))
    }

    fn submit_entry(
        &mut self,
        entry: &SemanticActionEntry,
        source_view_model_id: Option<tracewake_core::ids::ViewModelId>,
    ) -> Result<PipelineResult, AppError> {
        let actor_id = self.bound_actor_id.clone().ok_or(AppError::ActorNotBound)?;
        let sequence = self.scheduler.assign_proposal_sequence();
        let proposal = proposal_from_semantic_action_entry(
            ProposalId::new(format!("proposal_tui_{}", sequence.value())).unwrap(),
            ProposalOrigin::Human,
            Some(actor_id.clone()),
            self.scheduler.current_tick,
            entry,
            source_view_model_id,
            Some(&self.controller_id),
        );

        let ordering_key = OrderingKey::new(
            self.scheduler.current_tick,
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
        let result = run_pipeline(&mut context, &proposal);
        if result.report.status == ReportStatus::Rejected {
            self.last_rejection = Some(result.report.clone());
        } else {
            self.last_rejection = None;
            if let Some(last_event) = result.appended_events.last() {
                self.scheduler.current_tick = last_event.sim_tick;
            }
        }
        Ok(result)
    }

    pub fn event_count(&self) -> usize {
        self.log.events().len()
    }

    pub fn run_no_human_day(&mut self) -> NoHumanDayReport {
        let windows = default_day_windows(self.scheduler.current_tick);
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
        self.scheduler.current_tick = report.final_tick;
        self.last_rejection = None;
        report
    }

    pub fn physical_checksum(&self) -> PhysicalChecksum {
        compute_physical_checksum(&self.state, &self.checksum_context()).checksum
    }

    pub fn checksum_context(&self) -> ChecksumContext {
        ChecksumContext {
            fixture_id: self.fixture_id.clone(),
            content_version: self.content_version.clone(),
            sim_tick: self.scheduler.current_tick,
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
        let expected_checksum = self.physical_checksum();
        let expected_agent_checksum =
            compute_agent_state_checksum(&self.agent_state, &self.checksum_context()).checksum;
        let report = run_replay(
            &self.initial_state,
            &self.initial_agent_state,
            &self.log,
            &self.checksum_context(),
            Some(&self.state),
            Some(expected_checksum),
            Some(expected_agent_checksum),
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
        let observations = self
            .epistemic_projection
            .observations_by_id
            .values()
            .map(debug_observation_entry)
            .collect();
        let contradictions = self
            .epistemic_projection
            .contradictions_by_id
            .values()
            .map(debug_contradiction_entry)
            .collect();
        let beliefs_by_holder = self
            .epistemic_projection
            .beliefs_by_holder
            .iter()
            .map(|(holder_actor_id, belief_ids)| DebugHolderBeliefs {
                holder_actor_id: holder_actor_id.clone(),
                beliefs: belief_ids
                    .iter()
                    .filter_map(|belief_id| self.epistemic_projection.beliefs_by_id.get(belief_id))
                    .map(debug_belief_entry)
                    .collect(),
            })
            .collect();
        let checksum = self.epistemic_projection.compute_checksum().checksum;

        DebugEpistemicsView::new(
            "debug",
            observations,
            beliefs_by_holder,
            contradictions,
            Vec::new(),
            format!(
                "{} checksum={}",
                self.epistemic_projection.projection_version.as_str(),
                checksum.as_str()
            ),
        )
    }

    pub fn debug_beliefs_view(&self, actor_id: &ActorId) -> Result<DebugBeliefsView, AppError> {
        if !self.state.actors.contains_key(actor_id) {
            return Err(AppError::ActorNotFound(actor_id.clone()));
        }
        let beliefs = self
            .epistemic_projection
            .beliefs_by_id
            .values()
            .filter(|belief| belief.holder == HolderKind::Actor(actor_id.clone()))
            .map(debug_belief_entry)
            .collect();
        Ok(DebugBeliefsView::new(actor_id.clone(), beliefs))
    }

    pub fn debug_observations_view(
        &self,
        actor_id: &ActorId,
    ) -> Result<DebugObservationsView, AppError> {
        if !self.state.actors.contains_key(actor_id) {
            return Err(AppError::ActorNotFound(actor_id.clone()));
        }
        let observations = self
            .epistemic_projection
            .observations_by_id
            .values()
            .filter(|observation| observation.observer_actor_id == *actor_id)
            .map(debug_observation_entry)
            .collect();
        Ok(DebugObservationsView::new(actor_id.clone(), observations))
    }
}

fn debug_belief_entry(belief: &tracewake_core::epistemics::belief::Belief) -> DebugBeliefEntry {
    DebugBeliefEntry {
        belief_id: belief.belief_id.as_str().to_string(),
        proposition: belief.proposition.render(),
        stance: belief.stance.stable_id().to_string(),
        confidence: belief.confidence.serialize_canonical(),
        source: source_summary(&belief.source),
    }
}

fn debug_observation_entry(
    observation: &tracewake_core::epistemics::observation::Observation,
) -> DebugObservationEntry {
    DebugObservationEntry {
        observation_id: observation.observation_id.as_str().to_string(),
        observer_actor_id: observation.observer_actor_id.clone(),
        channel: observation.channel.stable_id().to_string(),
        confidence: observation.confidence.serialize_canonical(),
        source: source_summary(&observation.source),
    }
}

fn debug_contradiction_entry(
    contradiction: &tracewake_core::epistemics::contradiction::Contradiction,
) -> DebugContradictionEntry {
    DebugContradictionEntry {
        contradiction_id: contradiction.contradiction_id.as_str().to_string(),
        holder_actor_id: contradiction.holder_actor_id.clone(),
        expectation_belief_id: contradiction
            .prior_expectation_belief_id
            .as_str()
            .to_string(),
        observation_id: contradiction
            .contradicting_observation_id
            .as_str()
            .to_string(),
        summary: contradiction.kind.stable_id().to_string(),
    }
}

fn source_summary(source: &SourceRef) -> String {
    match source {
        SourceRef::Event(event_id) => format!("event:{}", event_id.as_str()),
        SourceRef::Action(action_id) => format!("action:{}", action_id.as_str()),
        SourceRef::Cause(cause) => format!("cause:{cause:?}"),
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
        let before = app.render_current_view().unwrap();
        assert!(before.contains("strongbox_tomas"));

        let result = app
            .submit_semantic_action(
                &SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
            )
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Accepted);

        let after = app.render_current_view().unwrap();
        assert!(after.contains("coin_stack_01"));
        assert!(app.event_count() >= 2);
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
        let before_events = app.event_count();

        let report = app.run_no_human_day();

        assert!(report.ordinary_pipeline_events > 0);
        assert!(app.event_count() > before_events);
        assert!(app.render_debug_no_human_day_panel().contains("events="));
        assert!(app.render_debug_no_human_day_panel().contains("canonical="));
    }
}
