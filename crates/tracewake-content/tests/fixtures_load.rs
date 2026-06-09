use std::collections::BTreeSet;

use tracewake_content::fixtures::{self, validate_fixture_contract_metadata};
use tracewake_content::load::{load_fixture_package, registry_for_fixture_scope};
use tracewake_content::schema::{
    ActorSchema, DayWindowSchema, FixtureSchema, FixtureScope, FoodSupplySchema, HomeSchema,
    InitialBeliefSchema, InitialNeedSchema, NeedModelSchema, PlaceSchema, RoutineAssignmentSchema,
    RoutineTemplateSchema, SleepPlaceSchema, WorkplaceSchema,
};
use tracewake_content::serialization::{deserialize_fixture, serialize_fixture};
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{Confidence, Proposition, SourceRef};
use tracewake_core::events::EventKind;
use tracewake_core::ids::ActionId;
use tracewake_core::ids::{
    ActorId, BeliefId, ContainerId, ContentManifestId, ContentVersion, EventId, FixtureId,
    FoodSupplyId, PlaceId, RoutineTemplateId, SchemaVersion, SemanticActionId, SleepAffordanceId,
    WorkplaceId,
};
use tracewake_core::location::Location;
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

fn phase3a_fixture() -> FixtureSchema {
    FixtureSchema {
        fixture_id: FixtureId::new("phase3a_schema_001").unwrap(),
        schema_version: SchemaVersion::new("schema_v1").unwrap(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: NeedModelSchema {
            awake_hunger_delta_per_tick: 5,
            awake_fatigue_delta_per_tick: 3,
        },
        actors: vec![ActorSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            current_place_id: PlaceId::new("home_tomas").unwrap(),
        }],
        places: vec![
            PlaceSchema {
                place_id: PlaceId::new("workshop").unwrap(),
                display_label: "Workshop".to_string(),
                adjacent_place_ids: vec![PlaceId::new("home_tomas").unwrap()],
            },
            PlaceSchema {
                place_id: PlaceId::new("home_tomas").unwrap(),
                display_label: "Tomas home".to_string(),
                adjacent_place_ids: vec![PlaceId::new("workshop").unwrap()],
            },
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            InitialNeedSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                kind: NeedKind::Fatigue,
                value: 120,
            },
            InitialNeedSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                kind: NeedKind::Hunger,
                value: 350,
            },
        ],
        homes: vec![HomeSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            place_id: PlaceId::new("home_tomas").unwrap(),
        }],
        sleep_places: vec![SleepPlaceSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            place_id: PlaceId::new("home_tomas").unwrap(),
            sleep_place_id: SleepAffordanceId::new("bed_tomas").unwrap(),
            access_open: true,
            duration_ticks: 4,
            fatigue_recovery_per_tick: 20,
            hunger_rise_per_tick: 2,
        }],
        food_supplies: vec![FoodSupplySchema {
            food_supply_id: FoodSupplyId::new("food_soup_pot").unwrap(),
            location: Location::AtPlace(PlaceId::new("home_tomas").unwrap()),
            servings: 3,
            hunger_reduction_per_serving: 180,
        }],
        workplaces: vec![WorkplaceSchema {
            workplace_id: WorkplaceId::new("workplace_shop").unwrap(),
            place_id: PlaceId::new("workshop").unwrap(),
            assigned_actor_ids: vec![ActorId::new("actor_tomas").unwrap()],
            work_duration_ticks: 4,
            fatigue_delta_per_tick: 8,
            hunger_delta_per_tick: 4,
            max_fatigue_to_start: 800,
            max_hunger_to_start: 850,
            access_open: true,
            output_tag: "service_completed_placeholder".to_string(),
        }],
        routine_templates: vec![RoutineTemplateSchema {
            template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
            family: RoutineFamily::WorkBlock,
            applicability_conditions: vec![RoutineCondition::AssignedWorkplaceKnown],
            preconditions: vec![RoutineCondition::AtWorkplace],
            steps: vec![RoutineStep::StartWorkBlock {
                action_id: SemanticActionId::new("work_block.workplace_shop").unwrap(),
            }],
            min_duration_ticks: 4,
            max_duration_ticks: 6,
            interruption_points: vec![0],
            failure_modes: vec!["access".to_string()],
            fallback_rules: vec!["wait".to_string()],
            debug_labels: vec!["phase3a_schema_sample".to_string()],
            reservable_resource: Some("body".to_string()),
        }],
        routine_assignments: vec![RoutineAssignmentSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
            start_tick: SimTick::new(10),
            end_tick: SimTick::new(20),
        }],
        day_windows: vec![DayWindowSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            start_tick: SimTick::new(0),
            end_tick: SimTick::new(100),
        }],
    }
}

#[test]
fn all_fixtures_load_deterministically_and_validate() {
    let registry = registry();
    let all = fixtures::all();
    assert_eq!(all.len(), 45);

    let ids = all
        .iter()
        .map(|fixture| fixture.fixture.fixture_id.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ids,
        BTreeSet::from([
            "container_item_move_001".to_string(),
            "debug_attach_001".to_string(),
            "debug_omniscience_excluded_001".to_string(),
            "door_access_001".to_string(),
            "embodied_exits_require_perceived_or_known_route_001".to_string(),
            "embodied_view_omits_raw_assignment_without_context_001".to_string(),
            "embodied_view_omits_unknown_sleep_affordance_001".to_string(),
            "embodied_view_omits_unobserved_food_at_open_place_001".to_string(),
            "expectation_contradiction_001".to_string(),
            "food_unavailable_replan_001".to_string(),
            "forbidden_provenance_input_fails_closed_001".to_string(),
            "hidden_food_closed_container_001".to_string(),
            "hidden_food_unknown_route_001".to_string(),
            "hidden_route_edge_001".to_string(),
            "hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001".to_string(),
            "knowledge_blocker_accuse_001".to_string(),
            "method_fallback_requires_new_trace_or_stuck_001".to_string(),
            "no_human_advance_001".to_string(),
            "no_human_current_place_without_sleep_affordance_does_not_sleep_001".to_string(),
            "no_human_day_001".to_string(),
            "no_human_epistemic_check_001".to_string(),
            "no_human_known_workplace_requires_provenance_001".to_string(),
            "no_human_metrics_require_typed_responsible_layer_001".to_string(),
            "no_human_observation_facts_cite_log_events_001".to_string(),
            "no_human_sleep_knowledge_requires_observation_or_record_001".to_string(),
            "no_human_unseen_workplace_assignment_does_not_plan_work_001".to_string(),
            "no_human_workplace_knowledge_requires_notice_event_001".to_string(),
            "no_hidden_truth_planning_001".to_string(),
            "ordinary_workday_001".to_string(),
            "planner_trace_001".to_string(),
            "possession_parity_001".to_string(),
            "possession_does_not_reset_intention_001".to_string(),
            "replay_item_location_001".to_string(),
            "routine_blocked_diagnostic_001".to_string(),
            "routine_no_teleport_001".to_string(),
            "scheduler_cannot_rewrite_wait_reason_after_transaction_001".to_string(),
            "sleep_eat_work_001".to_string(),
            "sleep_interrupted_by_severe_need_prorates_recovery_001".to_string(),
            "sleep_rejects_current_place_without_sleep_affordance_001".to_string(),
            "sound_uncertainty_001".to_string(),
            "strongbox_001".to_string(),
            "view_filtering_001".to_string(),
            "view_model_local_actions_001".to_string(),
            "work_completion_fails_when_actor_displaced_001".to_string(),
            "workplace_assignment_provenance_001".to_string(),
        ])
    );

    for golden in all {
        validate_fixture(&golden.fixture, &registry).unwrap();
        let first = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        let second = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        assert_eq!(first.canonical_world, second.canonical_world);
        assert_eq!(
            first.manifest.content_fingerprint,
            second.manifest.content_fingerprint
        );
    }
}

#[test]
fn phase3a_load_emits_authored_prehistory_seed_events() {
    let golden = fixtures::sleep_eat_work_001();
    let first = load_fixture_package(
        ContentManifestId::new("manifest_sleep_eat_work_001").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    let second = load_fixture_package(
        ContentManifestId::new("manifest_sleep_eat_work_001").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();

    assert_eq!(first.seed_event_log, second.seed_event_log);

    let events = first.seed_event_log.events();
    let role_notice = events
        .iter()
        .find(|event| event.event_type == EventKind::RoleAssignmentNoticeRecorded)
        .expect("workplace assignment notice is seeded");
    assert!(role_notice
        .payload
        .iter()
        .any(|field| { field.key == "source_kind" && field.value == "authored_prehistory" }));
    assert!(role_notice
        .payload
        .iter()
        .any(|field| field.key == "workplace_id" && field.value == "workplace_tomas"));

    let starting_belief_kinds = events
        .iter()
        .filter(|event| event.event_type == EventKind::StartingBeliefRecorded)
        .filter_map(|event| {
            event
                .payload
                .iter()
                .find(|field| field.key == "belief_kind")
                .map(|field| field.value.as_str())
        })
        .collect::<BTreeSet<_>>();
    assert!(starting_belief_kinds.contains("home_place"));
    assert!(starting_belief_kinds.contains("sleep_place"));
    assert!(starting_belief_kinds.contains("household_food_source"));
    assert!(events.iter().all(|event| event
        .payload
        .iter()
        .any(|field| field.key == "schema_version"
            && field.value == tracewake_core::events::EVENT_SCHEMA_V1)));
}

#[test]
fn fixtures_declare_scope_and_phase1_registry_excludes_later_actions() {
    let phase1 = registry_for_fixture_scope(FixtureScope::Phase1);
    for action_id in [
        "move",
        "open",
        "close",
        "take",
        "place",
        "look",
        "inspect_place",
        "inspect_entity",
        "wait",
    ] {
        assert!(
            phase1.get(&ActionId::new(action_id).unwrap()).is_some(),
            "Phase 1 registry must expose {action_id}"
        );
    }
    for action_id in [
        "check_container",
        "truthful_accuse_probe",
        "sleep",
        "eat",
        "work_block",
        "continue_routine",
    ] {
        assert!(
            phase1.get(&ActionId::new(action_id).unwrap()).is_none(),
            "Phase 1 registry must not expose later-phase action {action_id}"
        );
    }

    let ids_for_scope = |scope| {
        fixtures::all()
            .into_iter()
            .filter(|golden| golden.fixture.fixture_scope == scope)
            .map(|golden| golden.fixture.fixture_id.as_str().to_string())
            .collect::<BTreeSet<_>>()
    };

    assert_eq!(
        ids_for_scope(FixtureScope::Phase1),
        BTreeSet::from([
            "container_item_move_001".to_string(),
            "debug_attach_001".to_string(),
            "door_access_001".to_string(),
            "no_human_advance_001".to_string(),
            "replay_item_location_001".to_string(),
            "strongbox_001".to_string(),
            "view_model_local_actions_001".to_string(),
        ])
    );
    assert_eq!(
        ids_for_scope(FixtureScope::Phase2AHistorical),
        BTreeSet::from([
            "expectation_contradiction_001".to_string(),
            "knowledge_blocker_accuse_001".to_string(),
            "no_human_epistemic_check_001".to_string(),
            "possession_parity_001".to_string(),
            "sound_uncertainty_001".to_string(),
            "view_filtering_001".to_string(),
        ])
    );
    assert_eq!(
        ids_for_scope(FixtureScope::Phase3AHistorical),
        BTreeSet::from([
            "debug_omniscience_excluded_001".to_string(),
            "embodied_exits_require_perceived_or_known_route_001".to_string(),
            "embodied_view_omits_raw_assignment_without_context_001".to_string(),
            "embodied_view_omits_unknown_sleep_affordance_001".to_string(),
            "embodied_view_omits_unobserved_food_at_open_place_001".to_string(),
            "food_unavailable_replan_001".to_string(),
            "forbidden_provenance_input_fails_closed_001".to_string(),
            "hidden_food_closed_container_001".to_string(),
            "hidden_food_unknown_route_001".to_string(),
            "hidden_route_edge_001".to_string(),
            "hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001".to_string(),
            "method_fallback_requires_new_trace_or_stuck_001".to_string(),
            "no_hidden_truth_planning_001".to_string(),
            "no_human_current_place_without_sleep_affordance_does_not_sleep_001".to_string(),
            "no_human_day_001".to_string(),
            "no_human_known_workplace_requires_provenance_001".to_string(),
            "no_human_metrics_require_typed_responsible_layer_001".to_string(),
            "no_human_observation_facts_cite_log_events_001".to_string(),
            "no_human_sleep_knowledge_requires_observation_or_record_001".to_string(),
            "no_human_unseen_workplace_assignment_does_not_plan_work_001".to_string(),
            "no_human_workplace_knowledge_requires_notice_event_001".to_string(),
            "ordinary_workday_001".to_string(),
            "planner_trace_001".to_string(),
            "possession_does_not_reset_intention_001".to_string(),
            "routine_blocked_diagnostic_001".to_string(),
            "routine_no_teleport_001".to_string(),
            "scheduler_cannot_rewrite_wait_reason_after_transaction_001".to_string(),
            "sleep_eat_work_001".to_string(),
            "sleep_interrupted_by_severe_need_prorates_recovery_001".to_string(),
            "sleep_rejects_current_place_without_sleep_affordance_001".to_string(),
            "work_completion_fails_when_actor_displaced_001".to_string(),
            "workplace_assignment_provenance_001".to_string(),
        ])
    );
}

#[test]
fn fixtures_load_phase3a_fixture_into_core_shapes_with_canonical_ordering() {
    let fixture = phase3a_fixture();
    let bytes = serialize_fixture(&fixture);
    let loaded = load_fixture_package(
        ContentManifestId::new("manifest_phase3a_schema").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![tracewake_content::load::SourceFile {
            path: "phase3a_schema_001.twf".to_string(),
            bytes,
        }],
    )
    .unwrap();

    assert!(loaded
        .canonical_world
        .food_supplies()
        .contains_key(&FoodSupplyId::new("food_soup_pot").unwrap()));
    assert!(loaded
        .canonical_world
        .workplaces()
        .contains_key(&WorkplaceId::new("workplace_shop").unwrap()));
    assert_eq!(
        loaded.canonical_agent_state.needs_by_actor()[&ActorId::new("actor_tomas").unwrap()]
            [&NeedKind::Hunger]
            .value(),
        350
    );
    assert_eq!(loaded.manifest.actor_roster, ["actor_tomas".to_string()]);
    assert_eq!(
        loaded.manifest.no_human_day_windows,
        ["actor_tomas:0-100".to_string()]
    );
    assert_eq!(loaded.fixture.places[0].place_id.as_str(), "home_tomas");
    assert_eq!(loaded.fixture.routine_templates[0].steps.len(), 1);

    let round_tripped = deserialize_fixture(&serialize_fixture(&loaded.fixture)).unwrap();
    assert_eq!(loaded.fixture, round_tripped);
}

#[test]
fn fixtures_load_phase3a_duplicate_and_dangling_references_are_rejected() {
    let mut fixture = phase3a_fixture();
    fixture.food_supplies.push(FoodSupplySchema {
        food_supply_id: FoodSupplyId::new("food_soup_pot").unwrap(),
        location: Location::AtPlace(PlaceId::new("missing_place").unwrap()),
        servings: 1,
        hunger_reduction_per_serving: 100,
    });
    fixture.routine_assignments.push(RoutineAssignmentSchema {
        actor_id: ActorId::new("actor_tomas").unwrap(),
        template_id: RoutineTemplateId::new("routine_missing").unwrap(),
        start_tick: SimTick::new(21),
        end_tick: SimTick::new(30),
    });

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
    let codes = report
        .errors
        .iter()
        .map(|error| error.code)
        .collect::<BTreeSet<_>>();
    assert!(codes.contains("duplicate_id"));
    assert!(codes.contains("bad_reference"));
}

#[test]
fn fixtures_load_phase3a_unknown_fields_are_rejected_by_default() {
    let raw = b"fixture|phase3a_unknown\nschema|schema_v1\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65|\nunknown_phase3a_section|actor_tomas|workshop";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "unknown_field"));
}

#[test]
fn fixtures_load_phase3a_missing_need_model_is_rejected() {
    let bytes = serialize_fixture(&phase3a_fixture());
    let text = String::from_utf8(bytes).unwrap();
    let without_need_model = text
        .lines()
        .filter(|line| !line.starts_with("need_model|"))
        .collect::<Vec<_>>()
        .join("\n");
    let report = validate_fixture_bytes(without_need_model.as_bytes(), &registry())
        .unwrap_err()
        .report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "missing_field" && error.path == "fixture.need_model"));
}

#[test]
fn fixtures_load_phase3a_missing_sleep_tuning_fields_are_rejected() {
    let bytes = serialize_fixture(&phase3a_fixture());
    let text = String::from_utf8(bytes).unwrap();
    let old_shape = text
        .lines()
        .map(|line| {
            if line.starts_with("sleep_place|") {
                line.split('|').take(5).collect::<Vec<_>>().join("|")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let report = validate_fixture_bytes(old_shape.as_bytes(), &registry())
        .unwrap_err()
        .report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "bad_line" && error.message.contains("sleep_place")));
}

#[test]
fn strongbox_fixture_preserves_spec_0001_roster_and_physical_coin() {
    let golden = fixtures::strongbox_001();
    let actor_ids = golden
        .fixture
        .actors
        .iter()
        .map(|actor| actor.actor_id.as_str())
        .collect::<BTreeSet<_>>();
    let place_ids = golden
        .fixture
        .places
        .iter()
        .map(|place| place.place_id.as_str())
        .collect::<BTreeSet<_>>();

    assert!(actor_ids.contains("actor_tomas"));
    assert!(actor_ids.contains("actor_elena"));
    assert!(place_ids.contains("house_tomas"));
    assert!(golden
        .fixture
        .containers
        .iter()
        .any(|container| container.container_id.as_str() == "strongbox_tomas"));
    assert!(golden.fixture.items.iter().any(|item| {
        item.item_id.as_str() == "coin_stack_01"
            && item.location == Location::InContainer(ContainerId::new("strongbox_tomas").unwrap())
    }));
    let seed = golden
        .fixture
        .initial_beliefs
        .iter()
        .find(|belief| {
            belief.belief_id.as_str() == "belief_tomas_expects_coin_stack_01_in_strongbox_tomas"
        })
        .expect("Tomas expectation seed exists");
    assert_eq!(seed.source_kind.stable_id(), "authored_prehistory");
    match &seed.source {
        SourceRef::Event(event_id) => assert_eq!(
            event_id.as_str(),
            "prehistory_tomas_checked_strongbox_before_start"
        ),
        other => panic!("authored prehistory seed should be event-backed, got {other:?}"),
    }
}

#[test]
fn phase2a_fixtures_are_registered_and_script_free() {
    let all = fixtures::all();
    let phase2a_ids = [
        "strongbox_001",
        "expectation_contradiction_001",
        "possession_parity_001",
        "view_filtering_001",
        "knowledge_blocker_accuse_001",
        "sound_uncertainty_001",
        "no_human_epistemic_check_001",
    ];
    for fixture_id in phase2a_ids {
        let golden = all
            .iter()
            .find(|golden| golden.fixture.fixture_id.as_str() == fixture_id)
            .expect("Phase 2A fixture registered");
        validate_fixture(&golden.fixture, &registry()).unwrap();
        let serialized = String::from_utf8(golden.source_file().bytes).unwrap();
        for forbidden in ["culprit", "stolen_flag", "npc_knows_truth", "quest_state"] {
            assert!(
                !serialized.contains(forbidden),
                "{fixture_id} contains forbidden shortcut {forbidden}"
            );
        }
    }
}

#[test]
fn possession_parity_contains_mara_as_ordinary_actor() {
    let golden = fixtures::possession_parity_001();

    assert!(golden
        .fixture
        .actors
        .iter()
        .any(|actor| actor.actor_id.as_str() == "actor_mara"));
    assert!(golden
        .contract
        .acceptance_assertions
        .iter()
        .any(|assertion| assertion.contains("no culprit")));
}

#[test]
fn every_fixture_declares_contract_actions_reports_and_assertions() {
    for golden in fixtures::all() {
        assert_eq!(
            golden.contract.fixture_id,
            golden.fixture.fixture_id.as_str()
        );
        assert!(!golden.contract.setup.is_empty());
        assert!(!golden.contract.allowed_actions.is_empty());
        assert!(!golden.contract.expected_events_or_reports.is_empty());
        assert!(!golden.contract.acceptance_assertions.is_empty());
    }
}

#[test]
fn no_human_day_fixture_declares_validated_acceptance_contract() {
    let golden = fixtures::no_human_day_001();

    assert_eq!(golden.contract.fixture_id, "no_human_day_001");
    assert_eq!(
        validate_fixture_contract_metadata(&golden.contract),
        Vec::new()
    );
    assert!(golden
        .contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry == &"autonomous_no_human_event=NoHumanDayStarted"));
    assert!(golden
        .contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry == &"autonomous_no_human_event=NoHumanDayCompleted"));
    assert!(golden
        .contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry == &"log_derived_metric=no_human_day_metrics_v1"));
    for required in [
        "player",
        "roster actor",
        "not manually forced",
        "movement",
        "food",
        "workplace",
        "metrics",
    ] {
        assert!(
            golden
                .contract
                .acceptance_assertions
                .iter()
                .any(|assertion| assertion.contains(required)),
            "missing no-human contract assertion containing {required}"
        );
    }
}

#[test]
fn valid_epistemic_seed_validates_and_round_trips_canonically() {
    let mut golden = fixtures::strongbox_001();
    golden
        .fixture
        .initial_beliefs
        .push(InitialBeliefSchema::new_expectation(
            BeliefId::new("belief_tomas_expected_coin").unwrap(),
            "actor_tomas".parse().unwrap(),
            Proposition::ItemLocatedInContainer {
                item_id: "coin_stack_01".parse().unwrap(),
                container_id: "strongbox_tomas".parse().unwrap(),
            },
            Confidence::new(900).unwrap(),
            SourceRef::Event(EventId::new("event_authored_prehistory_tomas_coin").unwrap()),
            SimTick::ZERO,
        ));
    golden.fixture.canonicalize();

    validate_fixture(&golden.fixture, &registry()).unwrap();
    let first = serialize_fixture(&golden.fixture);
    let parsed = deserialize_fixture(&first).unwrap();
    let second = serialize_fixture(&parsed);

    assert_eq!(first, second);
    let seed = parsed
        .initial_beliefs
        .iter()
        .find(|belief| belief.belief_id.as_str() == "belief_tomas_expected_coin")
        .expect("added seed round-trips");
    assert_eq!(seed.schema_version.as_str(), EPISTEMIC_RECORD_SCHEMA_V1);
    assert!(String::from_utf8(first)
        .unwrap()
        .contains("initial_belief|belief_tomas_expected_coin|actor_tomas"));
}
