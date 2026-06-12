use crate::fixtures::*;

pub fn door_access_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("door_access_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_sena", "front_hall")],
        places: vec![
            place_schema("back_room", "Back room", &["front_hall"]),
            place_schema("front_hall", "Front hall", &["back_room"]),
        ],
        doors: vec![door_schema(
            "door_front_back",
            "front_hall",
            "back_room",
            false,
            false,
        )],
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("move", "back_room"),
            affordance("open", "door_front_back"),
            affordance("close", "door_front_back"),
            affordance("inspect_place", "front_hall"),
            affordance("inspect_entity", "door_front_back"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_sena", NeedKind::Hunger, 100),
            initial_need("actor_sena", NeedKind::Fatigue, 100),
            initial_need("actor_sena", NeedKind::Safety, 100),
        ],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: Vec::new(),
    };
    #[expect(
        clippy::disallowed_methods,
        reason = "legacy fixture blanket food-source seeding is pinned by fixtures_load census; new fixtures must author per-actor known_food_sources edges"
    )]
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "door_access_001",
            purpose:
                "Prove local movement, adjacency, door blockers, open/close events, and why-not.",
            setup: vec![
                "actor_sena starts in front_hall",
                "front_hall and back_room are connected by door_front_back",
                "door_front_back starts closed and blocks movement while closed",
            ],
            allowed_actions: vec![
                "attempt move back_room through closed door",
                "open door_front_back",
                "move back_room",
                "close door_front_back",
            ],
            expected_events_or_reports: vec![
                "ActionRejected or retained validation report for blocked movement",
                "DoorOpened",
                "ActorMoved",
                "DoorClosed",
                "matching replay checksum/report",
            ],
            acceptance_assertions: vec![
                "closed blocking door produces door_closed_blocks_movement or equivalent why-not",
                "opening the door permits ordinary movement through the shared pipeline",
                "fixture contains no player-only movement branch",
            ],
        },
    }
}
