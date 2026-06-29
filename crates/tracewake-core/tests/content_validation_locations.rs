use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::routine::RoutineDiagnosticKind;
use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
use tracewake_core::content::schema::{
    ActionAffordanceSchema, ActorSchema, ContainerSchema, DoorSchema, FixtureSchema, FixtureScope,
    InitialNeedSchema, ItemSchema, NeedModelSchema, PlaceSchema, RoutineTemplateSchema,
    SleepPlaceSchema, WorkplaceSchema, FIXTURE_SCHEMA_V1,
};
use tracewake_core::content::validate::{validate_fixture, ValidationPhase};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, DoorId, FixtureId, ItemId, PlaceId, RoutineTemplateId,
    SchemaVersion, SemanticActionId, SleepAffordanceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::state::VisibilityDefault;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();
    registry
}

fn fixture() -> FixtureSchema {
    FixtureSchema {
        fixture_id: FixtureId::new("location_validation_fixture").unwrap(),
        schema_version: SchemaVersion::new(FIXTURE_SCHEMA_V1).unwrap(),
        fixture_scope: FixtureScope::Phase1,
        need_model: NeedModelSchema {
            awake_hunger_delta_per_tick: 5,
            awake_fatigue_delta_per_tick: 3,
        },
        actors: vec![ActorSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            current_place_id: PlaceId::new("shop_front").unwrap(),
        }],
        places: vec![
            PlaceSchema {
                place_id: PlaceId::new("back_room").unwrap(),
                display_label: "Back room".to_string(),
                adjacent_place_ids: vec![PlaceId::new("shop_front").unwrap()],
                visibility_default: VisibilityDefault::Visible,
            },
            PlaceSchema {
                place_id: PlaceId::new("shop_front").unwrap(),
                display_label: "Shop front".to_string(),
                adjacent_place_ids: vec![PlaceId::new("back_room").unwrap()],
                visibility_default: VisibilityDefault::Visible,
            },
        ],
        doors: vec![DoorSchema {
            door_id: DoorId::new("door_shop_back").unwrap(),
            endpoint_a: PlaceId::new("shop_front").unwrap(),
            endpoint_b: PlaceId::new("back_room").unwrap(),
            is_open: false,
            is_locked: false,
        }],
        containers: vec![ContainerSchema {
            container_id: ContainerId::new("strongbox_tomas").unwrap(),
            place_id: PlaceId::new("shop_front").unwrap(),
            is_open: false,
            is_locked: false,
            contents: vec![ItemId::new("coin_stack_01").unwrap()],
            contents_visible_when_closed: false,
        }],
        items: vec![ItemSchema {
            item_id: ItemId::new("coin_stack_01").unwrap(),
            portable: true,
            location: Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
        }],
        affordances: vec![
            ActionAffordanceSchema {
                action_id: ActionId::new("open").unwrap(),
                target_id: "strongbox_tomas".to_string(),
            },
            ActionAffordanceSchema {
                action_id: ActionId::new("move").unwrap(),
                target_id: "back_room".to_string(),
            },
        ],
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
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: Vec::new(),
    }
}

#[test]
fn container_contents_and_item_locations_must_agree() {
    let mut container_lists_item_at_place = fixture();
    container_lists_item_at_place.items[0].location =
        Location::AtPlace(PlaceId::new("shop_front").unwrap());
    let report = validate_fixture(&container_lists_item_at_place, &registry())
        .unwrap_err()
        .report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::Location
            && error.path == "containers[0].contents"
            && error.code == "container_item_mismatch"
    }));

    let mut item_claims_container_that_omits_it = fixture();
    item_claims_container_that_omits_it.containers[0]
        .contents
        .clear();
    let report = validate_fixture(&item_claims_container_that_omits_it, &registry())
        .unwrap_err()
        .report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::Location
            && error.path == "items[0].location"
            && error.code == "container_item_mismatch"
    }));
}

#[test]
fn authored_state_invariants_are_required_for_valid_fixtures() {
    let mut fixture = fixture();
    fixture
        .initial_needs
        .retain(|need| need.kind != NeedKind::Safety);
    fixture.doors[0].is_locked = true;
    fixture.doors[0].is_open = true;
    fixture.containers[0].is_locked = true;
    fixture.containers[0].is_open = true;

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "initial_needs.actor_tomas.safety"
            && error.code == "missing_actor_need_seed"
    }));
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "doors[0]"
            && error.code == "locked_open_state"
    }));
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "containers[0]"
            && error.code == "locked_open_state"
    }));
}

#[test]
fn locked_closed_and_unlocked_open_states_remain_valid() {
    let mut locked_closed = fixture();
    locked_closed.doors[0].is_locked = true;
    locked_closed.doors[0].is_open = false;
    locked_closed.containers[0].is_locked = true;
    locked_closed.containers[0].is_open = false;
    validate_fixture(&locked_closed, &registry())
        .expect("locked but closed doors and containers remain valid authored state");

    let mut unlocked_open = fixture();
    unlocked_open.doors[0].is_locked = false;
    unlocked_open.doors[0].is_open = true;
    unlocked_open.containers[0].is_locked = false;
    unlocked_open.containers[0].is_open = true;
    validate_fixture(&unlocked_open, &registry())
        .expect("open but unlocked doors and containers remain valid authored state");
}

#[test]
fn authored_duration_fields_must_be_positive() {
    let mut fixture = fixture();
    fixture.sleep_places.push(SleepPlaceSchema {
        actor_id: ActorId::new("actor_tomas").unwrap(),
        place_id: PlaceId::new("shop_front").unwrap(),
        sleep_place_id: SleepAffordanceId::new("sleep_shop_front").unwrap(),
        access_open: true,
        duration_ticks: 0,
        fatigue_recovery_per_tick: 1,
        hunger_rise_per_tick: 0,
    });
    fixture.workplaces.push(WorkplaceSchema {
        workplace_id: WorkplaceId::new("workplace_shop").unwrap(),
        place_id: PlaceId::new("shop_front").unwrap(),
        assigned_actor_ids: vec![ActorId::new("actor_tomas").unwrap()],
        work_duration_ticks: 0,
        fatigue_delta_per_tick: 1,
        hunger_delta_per_tick: 1,
        max_fatigue_to_start: 100,
        max_hunger_to_start: 100,
        access_open: true,
        role_notice_access_open: true,
        output_tag: "shelf_stocked".to_string(),
    });

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "sleep_places[0].duration_ticks"
            && error.code == "invalid_duration"
    }));
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "workplaces[0].work_duration_ticks"
            && error.code == "invalid_duration"
    }));
}

#[test]
fn phase3a_sleep_routines_require_surface_or_typed_diagnostic() {
    let mut fixture = fixture();
    fixture.fixture_scope = FixtureScope::Phase3AHistorical;
    validate_fixture(&fixture, &registry())
        .expect("Phase 3A fixtures without sleep routines do not need a sleep surface");

    fixture.routine_templates.push(RoutineTemplateSchema {
        template_id: RoutineTemplateId::new("routine_sleep_night").unwrap(),
        family: RoutineFamily::SleepNight,
        applicability_conditions: vec![RoutineCondition::ActorKnowsSleepPlace],
        preconditions: vec![RoutineCondition::SleepPlaceBelievedAccessible],
        steps: vec![RoutineStep::StartScheduledSleep {
            action_id: SemanticActionId::new("sleep.sleep_shop_front").unwrap(),
        }],
        min_duration_ticks: 4,
        max_duration_ticks: 6,
        interruption_points: vec![0],
        failure_modes: vec!["access".to_string()],
        fallback_rules: vec!["wait".to_string()],
        debug_labels: vec!["phase3a_sleep_surface_contract".to_string()],
        reservable_resource: Some("body".to_string()),
    });

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "sleep_places"
            && error.code == "missing_sleep_surface"
    }));

    fixture.routine_templates[0].steps = vec![RoutineStep::FailWithTypedDiagnostic {
        diagnostic: RoutineDiagnosticKind::NoSleepAffordance,
    }];
    validate_fixture(&fixture, &registry())
        .expect("typed no-sleep diagnostic satisfies absent sleep surface contract");

    fixture.routine_templates[0].failure_modes = vec!["scripted_failure".to_string()];
    fixture.routine_templates[0].fallback_rules = vec!["scripted_recovery".to_string()];
    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "routine_templates[0].failure_modes[0]"
            && error.code == "unknown_failure_mode"
    }));
    assert!(report.errors.iter().any(|error| {
        error.phase == ValidationPhase::State
            && error.path == "routine_templates[0].fallback_rules[0]"
            && error.code == "unknown_fallback_rule"
    }));
}
