use crate::fixtures::*;

pub fn embodied_exits_require_perceived_or_known_route_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_exits_require_perceived_or_known_route_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "shop_front")],
        places: vec![
            place_schema("shop_front", "Shop front", &["back_room"]),
            place_schema("back_room", "Back room", &["shop_front"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: Vec::new(),
        homes: vec![home_schema("actor_tomas", "shop_front")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_exits_require_perceived_or_known_route_001",
            purpose: "Adversarial fixture: raw adjacency is omitted from embodied exits without a sealed route fact.",
            setup: vec!["raw place adjacency connects shop_front to back_room"],
            allowed_actions: vec!["move only when a holder-known route fact exists"],
            expected_events_or_reports: vec!["embodied output omits move.to.back_room"],
            acceptance_assertions: vec![
                "raw adjacent_place_ids are not an embodied exit source",
                "debug may compare truth non-diegetically",
            ],
        },
    }
}
