use std::collections::BTreeSet;

use tracewake_content::fixtures::{
    self, validate_fixture_contract_metadata, validate_golden_fixture_contract_metadata,
    FixtureContract,
};
use tracewake_content::load::{load_fixture_package, registry_for_fixture_scope, LoadError};
use tracewake_content::schema::{
    ActorSchema, DayWindowSchema, FixtureSchema, FixtureScope, FoodSupplySchema, HomeSchema,
    InitialBeliefSchema, InitialNeedSchema, NeedModelSchema, PlaceSchema, RoutineAssignmentSchema,
    RoutineTemplateSchema, SleepPlaceSchema, WorkplaceSchema, FIXTURE_SCHEMA_V1,
};
use tracewake_content::serialization::{deserialize_fixture, serialize_fixture};
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes, ValidationPhase};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    current_place_knowledge_context, record_current_place_perception_and_project,
};
use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{Confidence, Proposition, SourceRef};
use tracewake_core::events::apply::apply_epistemic_event;
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::ActionId;
use tracewake_core::ids::{
    ActorId, BeliefId, ContainerId, ContentManifestId, ContentVersion, EventId, FixtureId,
    FoodSupplyId, PlaceId, ProcessId, RoutineTemplateId, SchemaVersion, SemanticActionId,
    SleepAffordanceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::VisibilityDefault;
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

#[allow(
    clippy::disallowed_methods,
    reason = "fixture census test scans source constructors; this is test substrate, not simulation outcome code"
)]
fn positive_fixture_constructor_ids_from_source() -> BTreeSet<String> {
    let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/fixtures");
    let mut ids = BTreeSet::new();
    for entry in std::fs::read_dir(fixtures_dir).expect("fixtures directory is readable") {
        let path = entry.expect("fixture directory entry is readable").path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs")
            || path.file_name().and_then(|file_name| file_name.to_str()) == Some("mod.rs")
        {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("fixture source is readable");
        for line in source.lines() {
            let Some(signature) = line.trim_start().strip_prefix("pub fn ") else {
                continue;
            };
            let Some((name, remainder)) = signature.split_once('(') else {
                continue;
            };
            if name.ends_with("_001") && remainder.contains("-> GoldenFixture") {
                ids.insert(name.to_string());
            }
        }
    }
    ids
}

const LEGACY_KNOWN_FOOD_SOURCE_HELPER_CALL_SITES: &[&str] = &[
    "src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs",
    "src/fixtures/container_item_move_001.rs",
    "src/fixtures/debug_attach_001.rs",
    "src/fixtures/door_access_001.rs",
    "src/fixtures/embodied_exits_require_perceived_or_known_route_001.rs",
    "src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs",
    "src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs",
    "src/fixtures/embodied_view_omits_unknown_sleep_affordance_001.rs",
    "src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs",
    "src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs",
    "src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs",
    "src/fixtures/expectation_contradiction_001.rs",
    "src/fixtures/food_unavailable_replan_001.rs",
    "src/fixtures/forbidden_provenance_input_fails_closed_001.rs",
    "src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs",
    "src/fixtures/knowledge_blocker_accuse_001.rs",
    "src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs",
    "src/fixtures/no_hidden_truth_planning_001.rs",
    "src/fixtures/no_human_advance_001.rs",
    "src/fixtures/no_human_current_place_without_sleep_affordance_does_not_sleep_001.rs",
    "src/fixtures/no_human_day_001.rs",
    "src/fixtures/no_human_epistemic_check_001.rs",
    "src/fixtures/no_human_known_workplace_requires_provenance_001.rs",
    "src/fixtures/no_human_metrics_require_typed_responsible_layer_001.rs",
    "src/fixtures/no_human_observation_facts_cite_log_events_001.rs",
    "src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs",
    "src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs",
    "src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs",
    "src/fixtures/ordinary_workday_001.rs",
    "src/fixtures/planner_trace_001.rs",
    "src/fixtures/possession_does_not_reset_intention_001.rs",
    "src/fixtures/possession_parity_001.rs",
    "src/fixtures/replay_item_location_001.rs",
    "src/fixtures/routine_blocked_diagnostic_001.rs",
    "src/fixtures/routine_no_teleport_001.rs",
    "src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs",
    "src/fixtures/severe_safety_with_known_exit_produces_move_001.rs",
    "src/fixtures/severe_safety_without_known_exit_waits_with_knowledge_blocker_001.rs",
    "src/fixtures/sleep_eat_work_001.rs",
    "src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs",
    "src/fixtures/sleep_rejects_current_place_without_sleep_affordance_001.rs",
    "src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs",
    "src/fixtures/sound_uncertainty_001.rs",
    "src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs",
    "src/fixtures/strongbox_001.rs",
    "src/fixtures/view_filtering_001.rs",
    "src/fixtures/view_model_local_actions_001.rs",
    "src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs",
    "src/fixtures/work_block_failed_then_sleep_succeeds_001.rs",
    "src/fixtures/work_completion_fails_when_actor_displaced_001.rs",
];

#[allow(
    clippy::disallowed_methods,
    reason = "fixture helper census scans source constructors; this is test substrate, not simulation outcome code"
)]
fn known_food_source_helper_call_sites_from_source() -> BTreeSet<String> {
    let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/fixtures");
    let mut sources = Vec::new();
    for entry in std::fs::read_dir(fixtures_dir).expect("fixtures directory is readable") {
        let path = entry.expect("fixture directory entry is readable").path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("fixture source is readable");
        let file_name = path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .expect("fixture source has file name");
        sources.push((format!("src/fixtures/{file_name}"), source));
    }
    known_food_source_helper_call_sites_from_sources(&sources)
}

fn known_food_source_helper_call_sites_from_sources(
    sources: &[(String, String)],
) -> BTreeSet<String> {
    const HELPER_CALL: &str = ".populate_known_food_sources_for_all_actors(";
    let mut call_sites = BTreeSet::new();
    let mut helper_wrappers = BTreeSet::new();
    for (path, source) in sources {
        if source.contains(HELPER_CALL) {
            call_sites.insert(path.clone());
        }
        for function_name in function_names_before_helper_calls(source, HELPER_CALL) {
            if !function_name.ends_with("_001") {
                helper_wrappers.insert(function_name);
            }
        }
    }

    loop {
        let mut changed = false;
        let current_wrappers = helper_wrappers.iter().cloned().collect::<Vec<_>>();
        for (_, source) in sources {
            for wrapper in &current_wrappers {
                let wrapper_call = format!("{wrapper}(");
                for function_name in function_names_before_helper_calls(source, &wrapper_call) {
                    if !function_name.ends_with("_001") && helper_wrappers.insert(function_name) {
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    for (path, source) in sources {
        for wrapper in &helper_wrappers {
            let wrapper_call = format!("{wrapper}(");
            if function_names_before_helper_calls(source, &wrapper_call)
                .into_iter()
                .any(|function_name| function_name.ends_with("_001"))
            {
                call_sites.insert(path.clone());
            }
        }
    }
    call_sites
}

fn function_names_before_helper_calls(source: &str, helper_call: &str) -> BTreeSet<String> {
    let mut function_names = BTreeSet::new();
    let mut current_function = None;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(signature) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("pub fn "))
        {
            let Some((name, _)) = signature.split_once('(') else {
                current_function = None;
                continue;
            };
            current_function = Some(name.trim().to_string());
            continue;
        }
        if line.contains(helper_call) {
            if let Some(function_name) = &current_function {
                function_names.insert(function_name.clone());
            }
        }
    }
    function_names
}

fn known_food_source_helper_census_errors(call_sites: &BTreeSet<String>) -> Vec<String> {
    let allowed = LEGACY_KNOWN_FOOD_SOURCE_HELPER_CALL_SITES
        .iter()
        .map(|path| path.to_string())
        .collect::<BTreeSet<_>>();
    let mut errors = Vec::new();
    for unexpected in call_sites.difference(&allowed) {
        errors.push(format!(
            "unallowlisted blanket known-food helper call site {unexpected}; new fixtures must author per-actor known_food_sources edges or join the allowlist with rationale"
        ));
    }
    for missing in allowed.difference(call_sites) {
        errors.push(format!(
            "legacy known-food helper allowlist entry is stale: {missing}"
        ));
    }
    errors
}

fn phase3a_fixture() -> FixtureSchema {
    FixtureSchema {
        fixture_id: FixtureId::new("phase3a_schema_001").unwrap(),
        schema_version: SchemaVersion::new(FIXTURE_SCHEMA_V1).unwrap(),
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
                visibility_default: VisibilityDefault::Visible,
            },
            PlaceSchema {
                place_id: PlaceId::new("home_tomas").unwrap(),
                display_label: "Tomas home".to_string(),
                adjacent_place_ids: vec![PlaceId::new("workshop").unwrap()],
                visibility_default: VisibilityDefault::Visible,
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
            InitialNeedSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                kind: NeedKind::Safety,
                value: 100,
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
        known_food_sources: Vec::new(),
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
            role_notice_access_open: true,
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
fn fixture_validation_rejects_duplicate_routine_assignment_family() {
    let mut fixture = phase3a_fixture();
    fixture.routine_assignments.push(RoutineAssignmentSchema {
        actor_id: ActorId::new("actor_tomas").unwrap(),
        template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
        start_tick: SimTick::new(21),
        end_tick: SimTick::new(30),
    });
    fixture.canonicalize();

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "duplicate_actor_routine_family"));
}

#[test]
fn fixture_validation_rejects_ambiguous_actor_assignment_suffix_collision() {
    let mut fixture = phase3a_fixture();
    fixture.actors.push(ActorSchema {
        actor_id: ActorId::new("tomas").unwrap(),
        current_place_id: PlaceId::new("home_tomas").unwrap(),
    });
    fixture.routine_assignments.push(RoutineAssignmentSchema {
        actor_id: ActorId::new("tomas").unwrap(),
        template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
        start_tick: SimTick::new(21),
        end_tick: SimTick::new(30),
    });
    fixture.canonicalize();

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "ambiguous_actor_assignment_suffix"));
    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "routine_assignment_id_collision"));
}

#[test]
fn fixture_validation_rejects_tied_active_intention_derivation() {
    let mut fixture = phase3a_fixture();
    fixture.routine_templates.push(RoutineTemplateSchema {
        template_id: RoutineTemplateId::new("routine_sleep_shift").unwrap(),
        family: RoutineFamily::SleepNight,
        applicability_conditions: Vec::new(),
        preconditions: Vec::new(),
        steps: vec![RoutineStep::StartScheduledSleep {
            action_id: SemanticActionId::new("sleep.bed_tomas").unwrap(),
        }],
        min_duration_ticks: 4,
        max_duration_ticks: 6,
        interruption_points: vec![0],
        failure_modes: vec!["sleep_place_blocked".to_string()],
        fallback_rules: vec!["wait".to_string()],
        debug_labels: vec!["phase3a_schema_sample".to_string()],
        reservable_resource: Some("body".to_string()),
    });
    fixture.routine_assignments.push(RoutineAssignmentSchema {
        actor_id: ActorId::new("actor_tomas").unwrap(),
        template_id: RoutineTemplateId::new("routine_sleep_shift").unwrap(),
        start_tick: SimTick::new(10),
        end_tick: SimTick::new(30),
    });
    fixture.canonicalize();

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "ambiguous_active_routine_assignment"));
}

#[test]
fn source_ref_cause_uses_stable_typed_canonical_encoding() {
    let mut fixture = phase3a_fixture();
    fixture.initial_beliefs.push(InitialBeliefSchema {
        belief_id: BeliefId::new("belief_cause_source_probe").unwrap(),
        holder_actor_id: ActorId::new("actor_tomas").unwrap(),
        proposition: Proposition::ItemLocatedAtPlace {
            item_id: "coin_stack_01".parse().unwrap(),
            place_id: "home_tomas".parse().unwrap(),
        },
        stance: tracewake_core::epistemics::Stance::BelievesTrue,
        confidence: Confidence::new(900).unwrap(),
        source_kind: tracewake_core::events::InitialBeliefSourceKind::AuthoredPrehistory,
        source: SourceRef::Cause(EventCause::Process(
            ProcessId::new("process_fixture_cause").unwrap(),
        )),
        channel: None,
        acquired_tick: SimTick::ZERO,
        last_verified_tick: None,
        privacy_scope: tracewake_core::epistemics::PrivacyScope::ActorPrivate(
            ActorId::new("actor_tomas").unwrap(),
        ),
        schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
    });
    fixture.canonicalize();

    let bytes = serialize_fixture(&fixture);
    let rendered = String::from_utf8(bytes).unwrap();

    assert!(rendered.contains("process:process_fixture_cause"));
    assert!(!rendered.contains("Process("));
}

#[test]
fn all_fixtures_load_deterministically_and_validate() {
    let registry = registry();
    let all = fixtures::all();

    let ids = all
        .iter()
        .map(|fixture| fixture.fixture.fixture_id.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ids,
        positive_fixture_constructor_ids_from_source(),
        "every positive fixture constructor must be registered in fixtures::all()"
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
fn all_fixtures_author_explicit_need_seeds_for_every_actor() {
    for golden in fixtures::all() {
        let seeded = golden
            .fixture
            .initial_needs
            .iter()
            .map(|need| (need.actor_id.clone(), need.kind))
            .collect::<BTreeSet<_>>();
        for actor in &golden.fixture.actors {
            for kind in [NeedKind::Hunger, NeedKind::Fatigue, NeedKind::Safety] {
                assert!(
                    seeded.contains(&(actor.actor_id.clone(), kind)),
                    "fixture {} missing {:?} seed for {}",
                    golden.fixture.fixture_id.as_str(),
                    kind,
                    actor.actor_id.as_str()
                );
            }
        }
    }
}

#[test]
fn known_food_source_blanket_helper_call_sites_are_allowlisted() {
    let call_sites = known_food_source_helper_call_sites_from_source();
    let errors = known_food_source_helper_census_errors(&call_sites);
    assert!(
        errors.is_empty(),
        "blanket food-source helper call-site census diverged: {errors:#?}"
    );

    let mut synthetic = call_sites.clone();
    synthetic.insert("src/fixtures/synthetic_new_fixture_001.rs".to_string());
    let synthetic_errors = known_food_source_helper_census_errors(&synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("synthetic_new_fixture_001.rs")),
        "synthetic new blanket helper call site must fail with the file named"
    );

    let synthetic_indirect = known_food_source_helper_call_sites_from_sources(&[
        (
            "src/fixtures/mod.rs".to_string(),
            r#"
            fn hidden_truth_adversarial_fixture() -> GoldenFixture {
                fixture.populate_known_food_sources_for_all_actors();
                golden
            }

            fn depth_two_helper() -> GoldenFixture {
                hidden_truth_adversarial_fixture()
            }
            "#
            .to_string(),
        ),
        (
            "src/fixtures/synthetic_indirect_consumer_001.rs".to_string(),
            r#"
            pub fn synthetic_indirect_consumer_001() -> GoldenFixture {
                depth_two_helper()
            }
            "#
            .to_string(),
        ),
    ]);
    let synthetic_indirect_errors = known_food_source_helper_census_errors(&synthetic_indirect);
    assert!(
        synthetic_indirect_errors
            .iter()
            .any(|error| error.contains("synthetic_indirect_consumer_001.rs")),
        "synthetic indirect blanket helper consumer must fail with the file named"
    );
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

    let phase1_ids = ids_for_scope(FixtureScope::Phase1);
    let phase2_ids = ids_for_scope(FixtureScope::Phase2AHistorical);
    let phase3_ids = ids_for_scope(FixtureScope::Phase3AHistorical);
    assert!(
        !phase1_ids.is_empty(),
        "Phase 1 fixtures must remain covered"
    );
    assert!(
        !phase2_ids.is_empty(),
        "Phase 2A historical fixtures must remain covered"
    );
    assert!(
        !phase3_ids.is_empty(),
        "Phase 3A historical fixtures must remain covered"
    );

    let mut registered_ids = BTreeSet::new();
    registered_ids.extend(phase1_ids);
    registered_ids.extend(phase2_ids);
    registered_ids.extend(phase3_ids);
    assert_eq!(
        registered_ids,
        positive_fixture_constructor_ids_from_source(),
        "every positive fixture constructor must have exactly one declared fixture scope"
    );
}

#[test]
fn stale_workplace_notice_superseded_by_newer_001() {
    let golden = fixtures::stale_workplace_notice_superseded_by_newer_001();
    let mut loaded = load_fixture_package(
        ContentManifestId::new("manifest_stale_workplace_notice").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let decision_tick = SimTick::new(3);

    record_current_place_perception_and_project(
        &mut loaded.seed_event_log,
        &mut loaded.canonical_world,
        &mut loaded.canonical_agent_state,
        &mut loaded.epistemic_projection,
        &actor_id,
        decision_tick,
        &loaded.manifest.manifest_id,
    );

    let newer_notice_id = EventId::new("event_role_notice_newer_open_workplace_tomas").unwrap();
    let mut newer_notice = EventEnvelope::new_v1(
        newer_notice_id.clone(),
        EventKind::RoleAssignmentNoticeRecorded,
        loaded.seed_event_log.events().len() as u64,
        loaded.seed_event_log.events().len() as u64,
        decision_tick,
        OrderingKey::new(
            decision_tick,
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Actor(actor_id.clone()),
            ProposalSequence::new(99),
            ActionId::new("role_assignment_notice").unwrap(),
            vec!["workplace_tomas".to_string()],
            "newer_role_notice",
        ),
        loaded.manifest.manifest_id.clone(),
    );
    newer_notice.actor_id = Some(actor_id.clone());
    newer_notice.place_id = Some(PlaceId::new("workshop_tomas").unwrap());
    newer_notice.process_id = Some(ProcessId::new("role_assignment_notice").unwrap());
    newer_notice.participants = vec![
        actor_id.as_str().to_string(),
        "workplace_tomas".to_string(),
        "workshop_tomas".to_string(),
    ];
    newer_notice.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("source_kind", "modeled_role_update"),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", "workplace_tomas"),
        PayloadField::new("place_id", "workshop_tomas"),
        PayloadField::new("access_open", "true"),
    ];
    loaded.seed_event_log.append(newer_notice.clone()).unwrap();
    apply_epistemic_event(&mut loaded.epistemic_projection, &newer_notice).unwrap();

    let context = current_place_knowledge_context(
        &loaded.canonical_world,
        Some(&loaded.epistemic_projection),
        &actor_id,
        SimTick::new(4),
        &loaded.manifest.manifest_id,
        loaded.seed_event_log.events().len() as u64,
    );

    assert_eq!(context.actor_known_workplaces().len(), 1);
    let workplace = &context.actor_known_workplaces()[0];
    assert_eq!(workplace.workplace_id().as_str(), "workplace_tomas");
    assert!(workplace.believed_access_open());
    assert_eq!(workplace.acquired_tick(), decision_tick);
    assert_eq!(workplace.source_event_ids().as_slice(), &[newer_notice_id]);
}

#[test]
fn seeded_food_source_unknown_to_all_actors_omits_seed_belief_and_actor_known_food() {
    let golden = fixtures::seeded_food_source_unknown_to_all_actors_001();
    let loaded = load_fixture_package(
        ContentManifestId::new("manifest_seeded_food_source_unknown").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();

    assert!(loaded.fixture.known_food_sources.is_empty());
    assert!(!loaded.seed_event_log.events().iter().any(|event| {
        event.event_type == EventKind::StartingBeliefRecorded
            && event
                .payload
                .iter()
                .any(|field| field.key == "belief_kind" && field.value == "household_food_source")
    }));

    let context = current_place_knowledge_context(
        &loaded.canonical_world,
        Some(&loaded.epistemic_projection),
        &ActorId::new("actor_mara").unwrap(),
        SimTick::ZERO,
        &loaded.manifest.manifest_id,
        loaded.seed_event_log.events().len() as u64,
    );

    assert!(
        context.actor_known_food_sources().is_empty(),
        "{:?}",
        context.actor_known_food_sources()
    );

    let mut world = loaded.canonical_world.clone();
    let mut agent_state = loaded.canonical_agent_state.clone();
    let mut log = loaded.seed_event_log.clone();
    let report = run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry_for_fixture_scope(loaded.fixture.fixture_scope),
        loaded.manifest.manifest_id.clone(),
        NoHumanDayConfig {
            actor_ids: vec![ActorId::new("actor_mara").unwrap()],
            windows: vec![DayWindow {
                window_id: "unknown_food_window".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(8),
            }],
        },
    );

    assert!(report.ordinary_pipeline_events > 0);
    assert!(!log.events().iter().any(|event| {
        matches!(
            event.event_type,
            EventKind::FoodConsumed | EventKind::EatFailed
        )
    }));
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
    let raw = b"fixture|phase3a_unknown\nschema|schema_v1\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65||visible\nunknown_phase3a_section|actor_tomas|workshop";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "unknown_field"));
}

#[test]
fn fixtures_load_raw_line_prose_born_fact_rejected_001() {
    let raw = b"fixture|load_prose_born_fact_001\nschema|schema_v1\nfixture_scope|phase1\nneed_model|5|3\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65||visible\nnotes|the culprit is actor_mara";

    let error = load_fixture_package(
        ContentManifestId::new("manifest_load_prose_born_fact").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![tracewake_content::load::SourceFile {
            path: "fixtures/load_prose_born_fact.twf".to_string(),
            bytes: raw.to_vec(),
        }],
    )
    .unwrap_err();

    let LoadError::Validation(failure) = error else {
        panic!("prose-born raw line must fail validation, got {error:?}");
    };
    assert!(failure.report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript && error.code == "prose_born_fact"
    }));
}

#[test]
fn fixtures_load_forbidden_top_level_key_rejected_001() {
    let raw = b"fixture|load_forbidden_key_001\nschema|schema_v1\nfixture_scope|phase1\nneed_model|5|3\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65||visible\nappear_at|actor_tomas|workshop";

    let error = load_fixture_package(
        ContentManifestId::new("manifest_load_forbidden_key").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![tracewake_content::load::SourceFile {
            path: "fixtures/load_forbidden_key.twf".to_string(),
            bytes: raw.to_vec(),
        }],
    )
    .unwrap_err();

    let LoadError::Validation(failure) = error else {
        panic!("forbidden top-level raw key must fail validation, got {error:?}");
    };
    assert!(failure.report.errors.iter().any(|error| {
        error.phase == ValidationPhase::NoScript
            && error.code == "forbidden_form"
            && error.path.contains("appear_at")
    }));
}

#[test]
fn fixtures_load_unsupported_schema_version_rejected_001() {
    let mut fixture = phase3a_fixture();
    fixture.schema_version = SchemaVersion::new("schema_v999").unwrap();
    let bytes = serialize_fixture(&fixture);

    let validation_report = validate_fixture_bytes(&bytes, &registry())
        .unwrap_err()
        .report;
    assert!(validation_report.errors.iter().any(|error| {
        error.phase == ValidationPhase::ParseSchema
            && error.code == "unsupported_fixture_schema_version"
            && error.path == "fixture.schema_version"
    }));

    let error = load_fixture_package(
        ContentManifestId::new("manifest_bad_schema_version").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![tracewake_content::load::SourceFile {
            path: "fixtures/bad_schema_version.twf".to_string(),
            bytes,
        }],
    )
    .unwrap_err();

    let LoadError::Validation(failure) = error else {
        panic!("unsupported fixture schema version must fail validation, got {error:?}");
    };
    assert!(failure.report.errors.iter().any(|error| {
        error.phase == ValidationPhase::ParseSchema
            && error.code == "unsupported_fixture_schema_version"
            && error.path == "fixture.schema_version"
    }));
}

#[test]
fn fixtures_load_schema_v1_golden_bytes_load() {
    const GOLDEN_SCHEMA_V1_BYTES: &[u8] = b"fixture|schema_v1_golden_load_001\nschema|schema_v1\nfixture_scope|phase1\nneed_model|5|3\nactor|actor_tomas|shop_front\nplace|shop_front|53686f702066726f6e74||visible\ninitial_need|actor_tomas|hunger|100\ninitial_need|actor_tomas|fatigue|100\ninitial_need|actor_tomas|safety|100";

    let loaded = load_fixture_package(
        ContentManifestId::new("manifest_schema_v1_golden_load").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        vec![tracewake_content::load::SourceFile {
            path: "fixtures/schema_v1_golden_load.twf".to_string(),
            bytes: GOLDEN_SCHEMA_V1_BYTES.to_vec(),
        }],
    )
    .unwrap();

    assert_eq!(loaded.fixture.schema_version.as_str(), FIXTURE_SCHEMA_V1);
    assert_eq!(loaded.manifest.schema_version.as_str(), FIXTURE_SCHEMA_V1);
}

#[test]
fn fixtures_load_place_visibility_default_is_required() {
    let raw = b"fixture|missing_place_visibility\nschema|schema_v1\nactor|actor_tomas|home_tomas\nplace|home_tomas|486f6d65|";
    let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "bad_line" && error.message.contains("place|")));
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
fn hidden_truth_fixture_contracts_match_authored_known_food_edges() {
    for golden in [
        fixtures::hidden_food_closed_container_001(),
        fixtures::hidden_food_unknown_route_001(),
        fixtures::hidden_route_edge_001(),
        fixtures::debug_omniscience_excluded_001(),
        fixtures::workplace_assignment_provenance_001(),
    ] {
        assert_eq!(
            validate_golden_fixture_contract_metadata(&golden),
            Vec::new(),
            "{} contract should match authored known_food_sources",
            golden.contract.fixture_id
        );
    }

    assert_eq!(
        fixtures::hidden_food_closed_container_001()
            .fixture
            .known_food_sources
            .len(),
        1,
        "closed-container fixture keeps the load-bearing known-food edge"
    );
    for golden in [
        fixtures::hidden_food_unknown_route_001(),
        fixtures::hidden_route_edge_001(),
        fixtures::debug_omniscience_excluded_001(),
        fixtures::workplace_assignment_provenance_001(),
    ] {
        assert!(
            golden.fixture.known_food_sources.is_empty(),
            "{} should not carry unrelated known-food seed edges",
            golden.contract.fixture_id
        );
    }
}

#[test]
fn known_food_contract_denial_is_rejected() {
    let mut golden = fixtures::hidden_food_closed_container_001();
    golden.contract = FixtureContract {
        fixture_id: "hidden_food_closed_container_001",
        purpose: "synthetic denial",
        setup: vec!["no observation or belief reveals the hidden food"],
        allowed_actions: vec!["synthetic"],
        expected_events_or_reports: vec!["synthetic"],
        acceptance_assertions: vec!["synthetic"],
    };

    let violations = validate_golden_fixture_contract_metadata(&golden);

    assert!(violations
        .iter()
        .any(|violation| violation.code == "known_food_contract_denial"));
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
