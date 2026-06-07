use tracewake_content::fixtures;
use tracewake_content::schema::InitialBeliefSchema;
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes, ValidationPhase};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::{Confidence, Proposition, SourceRef};
use tracewake_core::ids::{BeliefId, EventId, ItemId, SchemaVersion};
use tracewake_core::time::SimTick;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

#[test]
fn forbidden_content_quest_reward_player_and_script_constructs_are_blocking_errors() {
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

#[test]
fn forbidden_content_shortcut_truth_fields_are_blocking_errors() {
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nculprit|actor_mara\ntrue_culprit|actor_mara\nstolen_flag|true\nnpc_knows_truth|actor_elena\nknows_mara_did_it|actor_tomas\nquest_state|solved\nplayer_memory|coin";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    for forbidden in [
        "culprit",
        "true_culprit",
        "stolen_flag",
        "npc_knows_truth",
        "knows_mara_did_it",
        "quest_state",
        "player_memory",
    ] {
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.phase == ValidationPhase::NoScript
                    && error.path.contains(forbidden)),
            "missing forbidden error for {forbidden}: {report:?}"
        );
    }
}

#[test]
fn forbidden_content_phase3a_teleport_and_refill_shortcuts_are_blocking_errors() {
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nappear_at|actor_tomas|workshop\nforce_location_at_tick|actor_tomas|10|workshop\nscripted_absence|actor_elena\nstory_beat|work_succeeds\nhunger_refill_without_food|actor_tomas\ninstant_sleep_refill|actor_tomas\nwork_always_succeeds|actor_tomas\nteleport_actor|actor_tomas|home\nmove_item_to|coin_stack_01|strongbox_tomas\nset_need|actor_tomas|hunger|0\nhidden_true_item_location|coin_stack_01|strongbox_tomas";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    for forbidden in [
        "appear_at",
        "force_location_at_tick",
        "scripted_absence",
        "story_beat",
        "hunger_refill_without_food",
        "instant_sleep_refill",
        "work_always_succeeds",
        "teleport_actor",
        "move_item_to",
        "set_need",
        "hidden_true_item_location",
    ] {
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.phase == ValidationPhase::NoScript
                    && error.path.contains(forbidden)),
            "missing forbidden error for {forbidden}: {report:?}"
        );
    }
}

#[test]
fn forbidden_content_malformed_epistemic_seed_fields_are_rejected() {
    let proposition = encode(
        &Proposition::ItemLocatedInContainer {
            item_id: "coin_stack_01".parse().unwrap(),
            container_id: "strongbox_tomas".parse().unwrap(),
        }
        .serialize_canonical(),
    );
    let missing_holder = format!(
        "fixture|bad_fixture\nschema|schema_v1\ninitial_belief|belief_tomas_expected_coin||{proposition}|expects_true|0900|authored_prehistory|prehistory_seed||0||actor:actor_tomas|epistemic_record_schema_v1"
    );
    let missing_source = format!(
        "fixture|bad_fixture\nschema|schema_v1\ninitial_belief|belief_tomas_expected_coin|actor_tomas|{proposition}|expects_true|0900|authored_prehistory|||0||actor:actor_tomas|epistemic_record_schema_v1"
    );
    let raw_prose = encode("Tomas thinks the coins are probably gone");
    let raw_prose_seed = format!(
        "fixture|bad_fixture\nschema|schema_v1\ninitial_belief|belief_tomas_expected_coin|actor_tomas|{raw_prose}|expects_true|0900|authored_prehistory|prehistory_seed||0||actor:actor_tomas|epistemic_record_schema_v1"
    );

    for raw in [missing_holder, missing_source, raw_prose_seed] {
        assert!(validate_fixture_bytes(raw.as_bytes(), &registry()).is_err());
    }
}

#[test]
fn forbidden_content_epistemic_seed_reference_duplicate_and_version_failures_are_blocking() {
    let mut fixture = fixtures::strongbox_001().fixture;
    fixture
        .initial_beliefs
        .push(valid_seed("belief_tomas_expected_coin"));
    fixture
        .initial_beliefs
        .push(valid_seed("belief_tomas_expected_coin"));
    let duplicate = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(duplicate
        .errors
        .iter()
        .any(|error| error.code == "duplicate_id"));

    let mut fixture = fixtures::strongbox_001().fixture;
    let mut bad_reference = valid_seed("belief_tomas_expected_missing_item");
    bad_reference.proposition = Proposition::ItemLocatedInContainer {
        item_id: ItemId::new("missing_coin").unwrap(),
        container_id: "strongbox_tomas".parse().unwrap(),
    };
    fixture.initial_beliefs.push(bad_reference);
    let reference = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(reference
        .errors
        .iter()
        .any(|error| error.code == "bad_reference"));

    let mut fixture = fixtures::strongbox_001().fixture;
    let mut bad_version = valid_seed("belief_tomas_expected_coin");
    bad_version.schema_version = SchemaVersion::new("epistemic_record_schema_v999").unwrap();
    fixture.initial_beliefs.push(bad_version);
    let version = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(version
        .errors
        .iter()
        .any(|error| error.code == "unsupported_epistemic_schema_version"));
}

fn valid_seed(id: &str) -> InitialBeliefSchema {
    InitialBeliefSchema::new_expectation(
        BeliefId::new(id).unwrap(),
        "actor_tomas".parse().unwrap(),
        Proposition::ItemLocatedInContainer {
            item_id: "coin_stack_01".parse().unwrap(),
            container_id: "strongbox_tomas".parse().unwrap(),
        },
        Confidence::new(900).unwrap(),
        SourceRef::Event(EventId::new("event_authored_prehistory_tomas_coin").unwrap()),
        SimTick::ZERO,
    )
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
