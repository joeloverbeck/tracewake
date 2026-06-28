use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::NeedKind;
use tracewake_core::content::schema::{
    ActionAffordanceSchema, ActorSchema, ContainerSchema, DoorSchema, FixtureSchema, FixtureScope,
    InitialNeedSchema, ItemSchema, NeedModelSchema, PlaceSchema, FIXTURE_SCHEMA_V1,
};
use tracewake_core::content::validate::{validate_fixture, ValidationPhase};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, DoorId, FixtureId, ItemId, PlaceId, SchemaVersion,
};
use tracewake_core::location::Location;
use tracewake_core::state::VisibilityDefault;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
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
