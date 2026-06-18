#[derive(Clone, Copy)]
struct ContentSchemaEvidence {
    requirement: &'static str,
    layer: &'static str,
    test_name: &'static str,
}

const CONTENT_SCHEMA_EVIDENCE: &[ContentSchemaEvidence] = &[
    ContentSchemaEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/validation",
        test_name: "content_prose_born_fact_rejected",
    },
    ContentSchemaEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/schema",
        test_name:
            "content_new_field_requires_typed_validation_and_canonical_serialization_metadata",
    },
];

const FORBIDDEN_CONTENT_RS: &str = include_str!("forbidden_content.rs");

#[test]
fn schema_conformance_maps_content_spine_requirements_to_named_tests() {
    for evidence in CONTENT_SCHEMA_EVIDENCE {
        assert_eq!(
            evidence.requirement, "SPINE-AC-010",
            "content schema conformance owns only the content/schema and content/validation slice"
        );
        assert!(
            ["content/schema", "content/validation"].contains(&evidence.layer),
            "unknown content conformance layer {}",
            evidence.layer
        );
        let needle = format!("fn {}(", evidence.test_name);
        assert!(
            FORBIDDEN_CONTENT_RS.contains(&needle),
            "missing {} evidence for {} in {}",
            evidence.test_name,
            evidence.requirement,
            evidence.layer
        );
    }

    assert!(CONTENT_SCHEMA_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "content/schema"));
    assert!(CONTENT_SCHEMA_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "content/validation"));
}

#[test]
fn fixture_scope_is_registered_and_canonicalized() {
    use tracewake_content::fixtures;
    use tracewake_content::schema::{
        canonical_key_for_schema_field, content_field_by_schema_field, FixtureScope,
        ForbiddenConstructPolicy, ValidationPhase,
    };
    use tracewake_content::serialization::{deserialize_fixture, serialize_fixture};

    let registration = content_field_by_schema_field("fixture_scope")
        .expect("fixture_scope must be in the content field registry");
    assert_eq!(registration.canonical_serialization_key, "fixture_scope");
    assert_eq!(registration.validation_phase, ValidationPhase::ParseSchema);
    assert_eq!(
        registration.forbidden_construct_policy,
        ForbiddenConstructPolicy::TypedAffordance
    );
    assert_eq!(
        canonical_key_for_schema_field("fixture_scope"),
        "fixture_scope"
    );

    let golden = fixtures::sleep_eat_work_001();
    assert_eq!(
        golden.fixture.fixture_scope,
        FixtureScope::Phase3AHistorical
    );
    let serialized = serialize_fixture(&golden.fixture);
    let text = String::from_utf8(serialized.clone()).unwrap();
    assert!(
        text.lines()
            .any(|line| line == "fixture_scope|phase3a_historical"),
        "serialized fixture must carry fixture_scope as canonical data"
    );
    let round_tripped = deserialize_fixture(&serialized).unwrap();
    assert_eq!(round_tripped.fixture_scope, FixtureScope::Phase3AHistorical);
}

#[test]
fn routine_assignment_start_tick_boundaries_select_authoritative_active_intention() {
    use tracewake_content::schema::{
        ActorSchema, DayWindowSchema, FixtureSchema, FixtureScope, InitialNeedSchema,
        NeedModelSchema, PlaceSchema, RoutineAssignmentSchema, RoutineTemplateSchema,
        SleepPlaceSchema, FIXTURE_SCHEMA_V1,
    };
    use tracewake_content::serialization::serialize_fixture;
    use tracewake_content::validate::validate_fixture;
    use tracewake_core::actions::ActionRegistry;
    use tracewake_core::agent::{NeedKind, RoutineFamily, RoutineStep, RoutineStepStatus};
    use tracewake_core::checksum::{compute_agent_state_checksum, ChecksumContext};
    use tracewake_core::ids::{
        ActorId, ContentVersion, FixtureId, IntentionId, PlaceId, RoutineTemplateId, SchemaVersion,
        SemanticActionId, SleepAffordanceId,
    };
    use tracewake_core::time::SimTick;

    fn registry() -> ActionRegistry {
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_sleep();
        registry.register_phase3a_work();
        registry
    }

    fn template(
        template_id: &str,
        family: RoutineFamily,
        step: RoutineStep,
        failure_mode: &str,
    ) -> RoutineTemplateSchema {
        RoutineTemplateSchema {
            template_id: RoutineTemplateId::new(template_id).unwrap(),
            family,
            applicability_conditions: Vec::new(),
            preconditions: Vec::new(),
            steps: vec![step],
            min_duration_ticks: 1,
            max_duration_ticks: 8,
            interruption_points: vec![0],
            failure_modes: vec![failure_mode.to_string()],
            fallback_rules: vec!["wait".to_string()],
            debug_labels: vec!["schema_boundary_probe".to_string()],
            reservable_resource: Some("body".to_string()),
        }
    }

    fn assignment(template_id: &str, start_tick: u64, end_tick: u64) -> RoutineAssignmentSchema {
        RoutineAssignmentSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            template_id: RoutineTemplateId::new(template_id).unwrap(),
            start_tick: SimTick::new(start_tick),
            end_tick: SimTick::new(end_tick),
        }
    }

    fn fixture(
        fixture_id: &str,
        work_template_id: &str,
        work_start_tick: u64,
        sleep_template_id: &str,
        sleep_start_tick: u64,
    ) -> FixtureSchema {
        let mut fixture = FixtureSchema {
            fixture_id: FixtureId::new(fixture_id).unwrap(),
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
            places: vec![PlaceSchema {
                place_id: PlaceId::new("home_tomas").unwrap(),
                display_label: "Home".to_string(),
                adjacent_place_ids: Vec::new(),
                visibility_default: tracewake_core::state::VisibilityDefault::Visible,
            }],
            doors: Vec::new(),
            containers: Vec::new(),
            items: Vec::new(),
            affordances: Vec::new(),
            initial_beliefs: Vec::new(),
            initial_needs: vec![
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Hunger,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Fatigue,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Safety,
                    value: 100,
                },
            ],
            homes: Vec::new(),
            sleep_places: vec![SleepPlaceSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                place_id: PlaceId::new("home_tomas").unwrap(),
                sleep_place_id: SleepAffordanceId::new("bed_tomas").unwrap(),
                access_open: true,
                duration_ticks: 4,
                fatigue_recovery_per_tick: 20,
                hunger_rise_per_tick: 2,
            }],
            food_supplies: Vec::new(),
            known_food_sources: Vec::new(),
            workplaces: Vec::new(),
            routine_templates: vec![
                template(
                    work_template_id,
                    RoutineFamily::WorkBlock,
                    RoutineStep::StartWorkBlock {
                        action_id: SemanticActionId::new("work_block.schema_probe").unwrap(),
                    },
                    "access",
                ),
                template(
                    sleep_template_id,
                    RoutineFamily::SleepNight,
                    RoutineStep::StartScheduledSleep {
                        action_id: SemanticActionId::new("sleep.bed_tomas").unwrap(),
                    },
                    "sleep_place_blocked",
                ),
            ],
            routine_assignments: vec![
                assignment(work_template_id, work_start_tick, work_start_tick + 4),
                assignment(sleep_template_id, sleep_start_tick, sleep_start_tick + 4),
            ],
            day_windows: vec![DayWindowSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                start_tick: SimTick::new(0),
                end_tick: SimTick::new(100),
            }],
        };
        fixture.canonicalize();
        fixture
    }

    fn accepted_active_intention_id(fixture: &FixtureSchema) -> IntentionId {
        let accepted = validate_fixture(fixture, &registry()).unwrap();
        let agent_state = accepted.fixture.to_agent_state(accepted.validation_token);
        let actor_id = ActorId::new("actor_tomas").unwrap();
        let active_intention_id = agent_state.active_intention_by_actor()[&actor_id].clone();
        let active = &agent_state.intentions()[&active_intention_id];
        assert_eq!(active.start_tick, active.last_progress_tick);
        assert_eq!(active.durability_level, 8);

        let execution_id = if active_intention_id.as_str().ends_with("_sleep") {
            "routine_exec_tomas_sleep"
        } else {
            "routine_exec_tomas_work"
        };
        let execution = &agent_state.routine_executions()[&execution_id.parse().unwrap()];
        assert_eq!(execution.start_tick, active.start_tick);
        assert_eq!(
            execution.expected_next_progress_tick,
            Some(active.start_tick.next())
        );
        assert_eq!(execution.step_status, RoutineStepStatus::NotStarted);

        let checksum = compute_agent_state_checksum(
            &agent_state,
            &ChecksumContext {
                fixture_id: fixture.fixture_id.clone(),
                content_version: ContentVersion::new("content_v1").unwrap(),
                sim_tick: SimTick::ZERO,
                world_stream_position_applied: 0,
            },
        );
        assert_eq!(checksum.checksum, checksum.recompute_from_canonical_input());
        assert!(checksum.canonical_input.iter().any(|line| {
            line == &format!(
                "active_intention|actor=actor_tomas|intention={}",
                active_intention_id.as_str()
            )
        }));
        active_intention_id
    }

    let below_bound = fixture(
        "schema_start_tick_below_bound_probe",
        "routine_a_work",
        10,
        "routine_z_sleep",
        9,
    );
    assert_eq!(
        accepted_active_intention_id(&below_bound).as_str(),
        "intention_tomas_sleep",
        "later-ordered assignment below the current bound must replace the active candidate"
    );

    let above_bound = fixture(
        "schema_start_tick_above_bound_probe",
        "routine_z_work",
        10,
        "routine_a_sleep",
        9,
    );
    assert_eq!(
        accepted_active_intention_id(&above_bound).as_str(),
        "intention_tomas_sleep",
        "later assignment above the current bound must not replace the active candidate"
    );

    assert_ne!(
        serialize_fixture(&below_bound),
        serialize_fixture(&above_bound),
        "the two valid boundary siblings must have distinct canonical fixture inputs"
    );

    let tied = fixture(
        "schema_start_tick_exact_bound_rejected",
        "routine_a_work",
        10,
        "routine_z_sleep",
        10,
    );
    let tied_report = validate_fixture(&tied, &registry()).unwrap_err().report;
    assert!(tied_report
        .errors
        .iter()
        .any(|error| error.code == "ambiguous_active_routine_assignment"));

    let token = validate_fixture(&below_bound, &registry())
        .unwrap()
        .validation_token;
    let tied_agent_state = tied.to_agent_state(token);
    assert_eq!(
        tied_agent_state
            .active_intention_by_actor()
            .get(&ActorId::new("actor_tomas").unwrap())
            .unwrap()
            .as_str(),
        "intention_tomas_work",
        "the exact-bound relational guard must not replace on equality"
    );
}
