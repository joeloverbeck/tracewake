use tracewake_core::actions::ActionRegistry;
use tracewake_core::controller::ControllerBindings;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::ContentManifestId;
use tracewake_core::runtime::{LoadedWorldRuntime, RuntimeInitialState};
use tracewake_core::scheduler::DeterministicScheduler;
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

pub fn mutate_loaded_runtime_fields() {
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let mut runtime = LoadedWorldRuntime::from_initial_state(RuntimeInitialState {
        registry: ActionRegistry::new(),
        physical_state: PhysicalState::empty(NeedModelState::new(5, 3)),
        agent_state: AgentState::default(),
        event_log: EventLog::new(),
        epistemic_projection: EpistemicProjection::new(manifest_id.clone()),
        controller_bindings: ControllerBindings::new(),
        scheduler: DeterministicScheduler::new(SimTick::ZERO),
        content_manifest_id: manifest_id,
    });

    runtime.physical_state = PhysicalState::empty(NeedModelState::new(5, 3));
}
