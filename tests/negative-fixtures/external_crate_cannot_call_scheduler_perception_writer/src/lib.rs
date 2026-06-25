use tracewake_core::actions::ActionRegistry;
use tracewake_core::controller::ControllerBindings;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{ActorId, ContentManifestId};
use tracewake_core::scheduler::DeterministicScheduler;
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

pub fn call_scheduler_perception_writer() {
    let mut scheduler = DeterministicScheduler::new(SimTick::ZERO);
    let mut state = PhysicalState::empty(NeedModelState::new(5, 3));
    let mut agent_state = AgentState::default();
    let mut log = EventLog::new();
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let mut projection = EpistemicProjection::new(manifest_id.clone());
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let _registry = ActionRegistry::new();
    let _bindings = ControllerBindings::new();

    scheduler.record_actor_current_place_perception(
        &mut state,
        &mut agent_state,
        &mut log,
        &mut projection,
        &actor_id,
        &manifest_id,
    );
}
