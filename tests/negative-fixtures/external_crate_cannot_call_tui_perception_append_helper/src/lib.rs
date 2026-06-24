use tracewake_core::agent::record_current_place_perception_and_project;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{ActorId, ContentManifestId};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

pub fn call_tui_perception_append_helper() {
    let mut log = EventLog::new();
    let mut state = PhysicalState::empty(NeedModelState::new(5, 3));
    let mut agent_state = AgentState::default();
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let mut projection = EpistemicProjection::new(manifest_id.clone());
    let actor_id = ActorId::new("actor_tomas").unwrap();

    record_current_place_perception_and_project(
        &mut log,
        &mut state,
        &mut agent_state,
        &mut projection,
        &actor_id,
        SimTick::ZERO,
        &manifest_id,
    );
}
