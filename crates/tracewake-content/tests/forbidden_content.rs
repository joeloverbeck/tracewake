use std::collections::BTreeSet;

use tracewake_content::fixtures::{self, validate_fixture_contract_metadata, FixtureContract};
use tracewake_content::load::registry_for_fixture_scope;
use tracewake_content::schema::{
    content_field_by_schema_field, ActionAffordanceSchema, FixtureScope, InitialBeliefSchema,
};
use tracewake_content::serialization::serialize_fixture;
use tracewake_content::validate::{
    content_field_registry, validate_fixture, validate_fixture_bytes, ValidationPhase,
};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::RoutineStep;
use tracewake_core::epistemics::{Channel, Confidence, Proposition, SourceRef};
use tracewake_core::ids::{ActionId, BeliefId, EventId, ItemId, SchemaVersion, SemanticActionId};
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

fn phase1_registry() -> ActionRegistry {
    registry_for_fixture_scope(FixtureScope::Phase1)
}

#[test]
fn forbidden_content_phase1_rejects_each_later_phase_action_family() {
    for action_id in [
        "check_container",
        "truthful_accuse_probe",
        "sleep",
        "eat",
        "work_block",
        "continue_routine",
    ] {
        let mut fixture = fixtures::strongbox_001().fixture;
        fixture.affordances.push(ActionAffordanceSchema {
            action_id: ActionId::new(action_id).unwrap(),
            target_id: "strongbox_tomas".to_string(),
        });
        fixture.canonicalize();

        let report = validate_fixture(&fixture, &phase1_registry())
            .unwrap_err()
            .report;

        assert!(
            report.errors.iter().any(|error| {
                error.phase == ValidationPhase::ActionRegistryParity
                    && error.code == "phase_unsupported_action"
                    && error.path.contains("affordances")
            }),
            "missing typed phase-boundary rejection for {action_id}: {report:?}"
        );
    }
}

#[test]
fn forbidden_content_phase1_rejects_routine_step_smuggling_later_phase_action() {
    let mut fixture = fixtures::ordinary_workday_001().fixture;
    fixture.fixture_scope = FixtureScope::Phase1;

    let report = validate_fixture(&fixture, &phase1_registry())
        .unwrap_err()
        .report;

    assert!(
        report.errors.iter().any(|error| {
            error.phase == ValidationPhase::ActionRegistryParity
                && error.code == "phase_unsupported_action"
                && error.path.contains("routine_templates")
        }),
        "routine semantic action must fail with typed phase-boundary diagnostic: {report:?}"
    );
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
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nculprit|actor_mara\ntrue_culprit|actor_mara\nstolen_flag|true\nnpc_knows_truth|actor_elena\nknows_mara_did_it|actor_tomas\nquest_state|solved\nplayer_memory|coin\ntruth_alias|actor_mara\nnested_culprit_hint|actor_mara\nrenamed_stolen_state|true";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    for forbidden in [
        "culprit",
        "true_culprit",
        "stolen_flag",
        "npc_knows_truth",
        "knows_mara_did_it",
        "quest_state",
        "player_memory",
        "truth_alias",
        "nested_culprit_hint",
        "renamed_stolen_state",
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
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nappear_at|actor_tomas|workshop\nforce_location_at_tick|actor_tomas|10|workshop\nscripted_absence|actor_elena\nstory_beat|work_succeeds\nhunger_refill_without_food|actor_tomas\ninstant_sleep_refill|actor_tomas\nwork_always_succeeds|actor_tomas\nteleport_actor|actor_tomas|home\nmove_item_to|coin_stack_01|strongbox_tomas\nset_need|actor_tomas|hunger|0\nhidden_true_item_location|coin_stack_01|strongbox_tomas\nfinal_event|SleepStarted\nexpected_final_event|FoodConsumed\nscripted_outcome|work_done\nhidden_planner_input|food_hidden_pantry\nactor_known_hidden_input|food_hidden_pantry";
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
        "final_event",
        "expected_final_event",
        "scripted_outcome",
        "hidden_planner_input",
        "actor_known_hidden_input",
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
fn forbidden_content_marker_bearing_id_is_rejected_001() {
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nfixture_scope|phase1\nneed_model|5|3\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65||visible\nitem|appear_at_workshop|true|at:home_tomas";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript
            && error.code == "authored_shortcut_effect"
            && error.path == "items[0].item_id"
    }));
}

#[test]
fn forbidden_content_routine_template_without_typed_family_is_blocking_error() {
    let step = encode("routine_step_v1|start_work_block|work_block.workplace_shop");
    let raw = format!(
        "fixture|bad_fixture\nschema|schema_v1\nactor|actor_tomas|workshop\nplace|workshop|576f726b73686f70||visible\nroutine_template|routine_work_by_name||61737369676e65645f776f726b706c6163655f6b6e6f776e|61745f776f726b706c616365|{step}|1|2|0|616363657373|77616974|70686173653361|"
    );
    let report = validate_fixture_bytes(raw.as_bytes(), &registry())
        .unwrap_err()
        .report;

    assert!(
        report
            .errors
            .iter()
            .any(|error| error.code == "bad_line" && error.message.contains("bad routine family")),
        "missing typed family rejection: {report:?}"
    );
}

#[test]
fn forbidden_content_unknown_behavior_looking_routine_field_is_blocking_error() {
    let raw = b"fixture|bad_fixture\nschema|schema_v1\nroutine_story_beat|work_always_succeeds";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "unknown_field"));
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "authored_shortcut_effect"
    }));
}

#[test]
fn forbidden_content_outcome_chain_routine_markers_are_rejected() {
    let mut fixture = fixtures::no_hidden_truth_planning_001().fixture;
    fixture.routine_templates[0]
        .debug_labels
        .push("guaranteed_success_without_pipeline".to_string());
    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "authored_outcome_chain"
    }));
}

#[test]
fn forbidden_content_planner_intended_initial_facts_require_provenance() {
    let mut fixture = fixtures::strongbox_001().fixture;
    fixture
        .initial_beliefs
        .push(valid_seed("belief_actor_known_food_source"));
    let missing = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(missing.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "missing_actor_known_provenance"
    }));

    let mut fixture = fixtures::strongbox_001().fixture;
    let mut with_channel = valid_seed("belief_actor_known_food_source");
    with_channel.channel = Some(Channel::DirectSight);
    fixture.initial_beliefs.push(with_channel);
    fixture.canonicalize();
    validate_fixture(&fixture, &registry()).expect("explicit channel provenance is accepted");
}

#[test]
fn forbidden_content_acceptance_gaming_debug_labels_are_rejected() {
    let mut fixture = fixtures::no_hidden_truth_planning_001().fixture;
    fixture.routine_templates[0]
        .debug_labels
        .push("debug_acceptance_marker".to_string());
    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "authored_outcome_chain"
    }));
}

#[test]
fn forbidden_content_player_conditioned_ordinary_life_markers_are_rejected() {
    let mut fixture = fixtures::ordinary_workday_001().fixture;
    fixture.routine_templates[0]
        .fallback_rules
        .push("player_conditioned_success".to_string());
    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "authored_outcome_chain"
    }));
}

#[test]
fn forbidden_content_success_implying_routine_step_names_are_rejected() {
    let mut fixture = fixtures::no_hidden_truth_planning_001().fixture;
    fixture.routine_templates[0].steps = vec![RoutineStep::ContinueCurrentStep {
        action_id: SemanticActionId::new("eat.guaranteed_success").unwrap(),
    }];
    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "authored_outcome_chain"
    }));
}

#[test]
fn forbidden_content_no_human_contract_text_only_and_manual_ambiguity_are_rejected() {
    let contract = FixtureContract {
        fixture_id: "no_human_day_001",
        purpose: "bad text-only capstone proof",
        setup: vec!["roster:actor_tomas"],
        allowed_actions: vec!["run no-human day"],
        expected_events_or_reports: vec![
            "NoHumanDayStarted and NoHumanDayCompleted",
            "FoodConsumed or EatFailed",
            "SleepCompleted",
        ],
        acceptance_assertions: vec!["events happened through action units"],
    };
    let violations = validate_fixture_contract_metadata(&contract);

    assert!(violations
        .iter()
        .any(|violation| violation.code == "text_only_behavior_proof"));
    assert!(violations
        .iter()
        .any(|violation| violation.code == "manual_action_ambiguity"));
}

#[test]
fn forbidden_content_hidden_truth_source_cannot_seed_actor_known_planner_input() {
    let mut fixture = fixtures::strongbox_001().fixture;
    let mut seed = valid_seed("belief_tomas_actor_known_hidden_food");
    seed.source = SourceRef::Event(EventId::new("event_hidden_true_item_location").unwrap());
    fixture.initial_beliefs.push(seed);
    fixture.canonicalize();

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript
            && error.code == "shortcut_truth_field"
            && error.path.contains("source_id")
    }));
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

#[test]
fn content_prose_born_fact_rejected() {
    let source = fixtures::prose_born_fact_rejected_001();
    let report = validate_fixture_bytes(&source.bytes, &registry())
        .unwrap_err()
        .report;

    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "prose_born_fact"
    }));
}

#[test]
fn content_new_field_requires_typed_validation_and_canonical_serialization_metadata() {
    let schema_fields = fixture_schema_fields();
    let registrations = content_field_registry();
    let registered_fields = registrations
        .iter()
        .map(|registration| registration.schema_field)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        schema_fields, registered_fields,
        "every FixtureSchema field must have a content field registration"
    );

    let mut tags = BTreeSet::new();
    let mut diagnostic_codes = BTreeSet::new();
    for registration in registrations {
        assert!(
            content_field_by_schema_field(registration.schema_field).is_some(),
            "schema field lookup missing for {}",
            registration.schema_field
        );
        assert!(
            !registration.canonical_serialization_key.is_empty(),
            "missing canonical key for {}",
            registration.schema_field
        );
        assert!(
            !registration.diagnostic_code.is_empty(),
            "missing diagnostic code for {}",
            registration.schema_field
        );
        assert!(
            tags.insert(registration.canonical_serialization_key),
            "duplicate canonical tag {}",
            registration.canonical_serialization_key
        );
        assert!(
            diagnostic_codes.insert((registration.schema_field, registration.diagnostic_code)),
            "duplicate field diagnostic registration for {}",
            registration.schema_field
        );
    }

    let fixture_id_registration = content_field_by_schema_field("fixture_id").unwrap();
    let report = validate_fixture_bytes(b"fixture|\nschema|schema_v1", &registry())
        .unwrap_err()
        .report;
    assert!(
        report
            .errors
            .iter()
            .any(|error| error.code == fixture_id_registration.diagnostic_code),
        "registered diagnostic code was not emitted: {report:?}"
    );
}

#[test]
fn content_serialization_is_canonical_independent_of_authoring_order() {
    let mut canonical = fixtures::no_human_day_001().fixture;
    let mut shuffled = canonical.clone();
    canonical.canonicalize();
    shuffled.actors.reverse();
    shuffled.places.reverse();
    shuffled.containers.reverse();
    shuffled.items.reverse();
    shuffled.initial_beliefs.reverse();
    shuffled.initial_needs.reverse();
    shuffled.homes.reverse();
    shuffled.sleep_places.reverse();
    shuffled.food_supplies.reverse();
    shuffled.workplaces.reverse();
    shuffled.routine_templates.reverse();
    shuffled.routine_assignments.reverse();
    shuffled.day_windows.reverse();

    let canonical_bytes = serialize_fixture(&canonical);
    let shuffled_bytes = serialize_fixture(&shuffled);
    assert_eq!(shuffled_bytes, canonical_bytes);

    let registered_tags = content_field_registry()
        .iter()
        .map(|registration| registration.canonical_serialization_key)
        .collect::<BTreeSet<_>>();
    for line in std::str::from_utf8(&canonical_bytes).unwrap().lines() {
        let tag = line.split('|').next().unwrap();
        assert!(
            registered_tags.contains(tag),
            "serialized tag {tag} is not backed by the typed registry"
        );
    }
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

fn fixture_schema_fields() -> BTreeSet<&'static str> {
    let schema_source = include_str!("../src/schema.rs");
    let start = schema_source
        .find("pub struct FixtureSchema")
        .expect("FixtureSchema exists");
    let body_start = schema_source[start..]
        .find('{')
        .map(|index| start + index + 1)
        .expect("FixtureSchema has a body");
    let body_end = schema_source[body_start..]
        .find("\n}")
        .map(|index| body_start + index)
        .expect("FixtureSchema body closes");
    schema_source[body_start..body_end]
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let field = trimmed.strip_prefix("pub ")?.split_once(':')?.0;
            Some(field)
        })
        .collect()
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
