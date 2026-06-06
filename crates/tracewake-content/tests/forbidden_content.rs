use tracewake_content::validate::{validate_fixture_bytes, ValidationPhase};
use tracewake_core::actions::ActionRegistry;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry
}

#[test]
fn quest_reward_player_and_script_constructs_are_blocking_errors() {
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nquest|q1\nreward|coins\nplayer|actor_tomas\nforce_event|door_opens";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.phase == ValidationPhase::NoPlayer));
    assert!(report
        .errors
        .iter()
        .any(|error| error.phase == ValidationPhase::NoScript));
}
