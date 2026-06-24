use crate::actions::{run_pipeline, ActionRegistry, PipelineContext, PipelineResult, ReportStatus};
use crate::checksum::ChecksumContext;
use crate::controller::ControllerBindings;
use crate::epistemics::projection::EpistemicProjection;
use crate::events::log::EventLog;
use crate::ids::{ActorId, ContentManifestId, ControllerId};
use crate::replay::rebuild_projection;
use crate::scheduler::no_human::{
    default_day_windows, run_no_human_day, NoHumanDayConfig, NoHumanDayReport,
};
use crate::scheduler::{
    AdvanceUntilRequest, AdvanceUntilResult, DeterministicScheduler, OrderingKey, SchedulePhase,
    SchedulerSourceId, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
    WorldAdvanceResult, WorldStepTransactionRequest,
};
use crate::state::{AgentState, ControllerMode, PhysicalState};

/// Owned initial aggregates for constructing a loaded-world runtime.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeInitialState {
    pub registry: ActionRegistry,
    pub physical_state: PhysicalState,
    pub agent_state: AgentState,
    pub event_log: EventLog,
    pub epistemic_projection: EpistemicProjection,
    pub controller_bindings: ControllerBindings,
    pub scheduler: DeterministicScheduler,
    pub content_manifest_id: ContentManifestId,
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
}

/// Closed command token. Constructors stay inside `tracewake-core`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeCommand {
    kind: RuntimeCommandKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuntimeCommandKind {
    OneTickWait { origin: WorldAdvanceOrigin },
}

/// Immutable runtime receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReceipt {
    kind: RuntimeReceiptKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeReceiptKind {
    OneTickAdvanced(WorldAdvanceResult),
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

impl LoadedWorldRuntime {
    pub fn from_initial_state(initial: RuntimeInitialState) -> Self {
        Self {
            registry: initial.registry,
            physical_state: initial.physical_state,
            agent_state: initial.agent_state,
            event_log: initial.event_log,
            epistemic_projection: initial.epistemic_projection,
            controller_bindings: initial.controller_bindings,
            scheduler: initial.scheduler,
            content_manifest_id: initial.content_manifest_id,
        }
    }

    pub fn current_tick(&self) -> crate::time::SimTick {
        self.scheduler.current_tick()
    }

    pub fn from_loaded_world(
        mut initial: RuntimeInitialState,
        current_tick: crate::time::SimTick,
    ) -> Self {
        initial.scheduler = DeterministicScheduler::from_loaded_world(
            current_tick,
            &initial.physical_state,
            &initial.agent_state,
            initial.content_manifest_id.clone(),
        );
        Self::from_initial_state(initial)
    }

    pub fn event_count(&self) -> usize {
        self.event_log.events().len()
    }

    pub fn registry(&self) -> &ActionRegistry {
        &self.registry
    }

    pub fn physical_state(&self) -> &PhysicalState {
        &self.physical_state
    }

    pub fn agent_state(&self) -> &AgentState {
        &self.agent_state
    }

    pub fn event_log(&self) -> &EventLog {
        &self.event_log
    }

    pub fn epistemic_projection(&self) -> &EpistemicProjection {
        &self.epistemic_projection
    }

    pub fn controller_bindings(&self) -> &ControllerBindings {
        &self.controller_bindings
    }

    pub fn content_manifest_id(&self) -> &ContentManifestId {
        &self.content_manifest_id
    }

    pub fn bind_actor(
        &mut self,
        controller_id: ControllerId,
        actor_id: ActorId,
        mode: ControllerMode,
    ) {
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

    pub fn detach_controller(&mut self, controller_id: &ControllerId) {
        self.controller_bindings.detach(
            controller_id,
            self.scheduler.current_tick(),
            &mut self.event_log,
            self.content_manifest_id.clone(),
        );
    }

    pub fn assign_proposal_sequence(&mut self) -> crate::scheduler::ProposalSequence {
        self.scheduler.assign_proposal_sequence()
    }

    pub fn submit_controlled_proposal(
        &mut self,
        controller_id: ControllerId,
        proposal: crate::actions::Proposal,
        advance_world_after_acceptance: bool,
    ) -> Result<PipelineResult, RuntimeCommandError> {
        let expected_tick = self.scheduler.current_tick();
        let proposal_actor_id = proposal.actor_id.clone();
        let result = if advance_world_after_acceptance {
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

    pub fn advance_until(
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

    pub fn run_no_human_day(
        &mut self,
        actor_ids: Vec<ActorId>,
    ) -> Result<NoHumanDayReport, RuntimeCommandError> {
        let windows = default_day_windows(self.scheduler.current_tick());
        Ok(run_no_human_day(
            &mut self.physical_state,
            &mut self.agent_state,
            &mut self.event_log,
            &self.registry,
            self.content_manifest_id.clone(),
            NoHumanDayConfig { actor_ids, windows },
        ))
    }

    pub fn rebuild_from_owned_log(
        &mut self,
        initial_state: &PhysicalState,
        initial_agent_state: &AgentState,
        checksum_context: &ChecksumContext,
    ) -> Result<(), RuntimeCommandError> {
        let rebuild = rebuild_projection(
            initial_state,
            initial_agent_state,
            &self.event_log,
            checksum_context,
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

    pub fn refresh_actor_current_place_perception(&mut self, actor_id: &ActorId) {
        self.record_actor_current_place_perception(actor_id);
    }

    pub fn submit_command(
        &mut self,
        command: RuntimeCommand,
    ) -> Result<RuntimeReceipt, RuntimeCommandError> {
        match command.kind {
            RuntimeCommandKind::OneTickWait { origin } => self.run_one_tick_wait(origin),
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

        Ok(RuntimeReceipt {
            kind: RuntimeReceiptKind::OneTickAdvanced(result),
        })
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

impl RuntimeCommand {
    pub(crate) fn one_tick_wait(origin: WorldAdvanceOrigin) -> Self {
        Self {
            kind: RuntimeCommandKind::OneTickWait { origin },
        }
    }
}

impl RuntimeReceipt {
    pub fn kind(&self) -> &RuntimeReceiptKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ControllerId;
    use crate::state::NeedModelState;
    use crate::time::SimTick;

    fn manifest_id() -> ContentManifestId {
        ContentManifestId::new("runtime_session_test").unwrap()
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
        })
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
        }
    }
}
