use tracewake_content::fixtures::{self, GoldenFixture};
use tracewake_content::load::{load_fixture_package, LoadError};
use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, PipelineResult, Proposal, ProposalOrigin,
    ReportStatus, ValidationReport,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, ProposalId, SemanticActionId,
};
use tracewake_core::projections::{build_embodied_view_model, ProjectionError};
use tracewake_core::scheduler::{
    DeterministicScheduler, OrderingKey, SchedulePhase, SchedulerSourceId,
};
use tracewake_core::state::PhysicalState;
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{EmbodiedViewModel, SemanticActionEntry};

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
    state: PhysicalState,
    log: EventLog,
    controller_bindings: ControllerBindings,
    controller_id: ControllerId,
    bound_actor_id: Option<ActorId>,
    content_manifest_id: ContentManifestId,
    scheduler: DeterministicScheduler,
    last_rejection: Option<ValidationReport>,
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
        Ok(Self {
            registry,
            state: loaded.canonical_world,
            log: EventLog::new(),
            controller_bindings: ControllerBindings::new(),
            controller_id: ControllerId::new("controller_human").unwrap(),
            bound_actor_id: None,
            content_manifest_id: loaded.manifest.manifest_id,
            scheduler: DeterministicScheduler::new(SimTick::ZERO),
            last_rejection: None,
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
        build_embodied_view_model(
            &self.state,
            actor_id,
            self.scheduler.current_tick,
            self.last_rejection.as_ref(),
        )
        .map_err(Into::into)
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
        let mut proposal = Proposal::new(
            ProposalId::new(format!("proposal_tui_{}", sequence.value())).unwrap(),
            ProposalOrigin::Human,
            Some(actor_id.clone()),
            entry.action_id.clone(),
            self.scheduler.current_tick,
        );
        proposal.target_ids = entry.target_ids.clone();
        proposal.source_view_model_id = source_view_model_id;
        proposal
            .parameters
            .insert("controller_id".to_string(), self.controller_id.to_string());
        if entry.semantic_action_id.as_str() == "wait.1_tick" {
            proposal
                .parameters
                .insert("ticks".to_string(), "1".to_string());
        }

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
            log: &mut self.log,
            controller_bindings: Some(&self.controller_bindings),
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
}
