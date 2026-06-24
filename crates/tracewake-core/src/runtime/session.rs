use crate::actions::ActionRegistry;
use crate::controller::ControllerBindings;
use crate::epistemics::projection::EpistemicProjection;
use crate::events::log::EventLog;
use crate::ids::ContentManifestId;
use crate::scheduler::{
    DeterministicScheduler, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
    WorldAdvanceResult, WorldStepTransactionRequest,
};
use crate::state::{AgentState, PhysicalState};

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
