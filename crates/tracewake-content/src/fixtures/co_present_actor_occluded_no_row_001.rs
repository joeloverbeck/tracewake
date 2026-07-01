use crate::fixtures::*;

pub fn co_present_actor_occluded_no_row_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("co_present_actor_occluded_no_row_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "front_room"),
            actor_schema("actor_tomas", "back_room"),
        ],
        places: vec![
            place_schema("back_room", "Back room", &["front_room"]),
            place_schema("front_room", "Front room", &["back_room"]),
        ],
        doors: vec![door_schema(
            "door_front_back",
            "front_room",
            "back_room",
            false,
            false,
        )],
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("inspect_place", "front_room"),
            affordance("open", "door_front_back"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 140),
            initial_need("actor_mara", NeedKind::Fatigue, 120),
            initial_need("actor_mara", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 140),
            initial_need("actor_tomas", NeedKind::Fatigue, 120),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![
            day_window_schema("actor_mara", 0, 8),
            day_window_schema("actor_tomas", 0, 8),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "co_present_actor_occluded_no_row_001",
            purpose: "Provide a closed-door non-co-present case that must not surface a visible actor row.",
            setup: vec![
                "actor_mara starts in front_room",
                "actor_tomas starts in back_room",
                "door_front_back starts closed between the rooms",
            ],
            allowed_actions: vec![
                "observe actor_mara's current place",
                "open door_front_back before any later movement test",
            ],
            expected_events_or_reports: vec![
                "actor_tomas is absent from immediate visible_actor observations",
                "opening the door is a modeled event, not an initial visibility shortcut",
            ],
            acceptance_assertions: vec![
                "separate place plus closed door does not create a co-present actor row",
                "fixture authors no initial belief about actor_tomas activity",
                "fixture canonicalizes without changing authored references",
            ],
        },
    }
}
