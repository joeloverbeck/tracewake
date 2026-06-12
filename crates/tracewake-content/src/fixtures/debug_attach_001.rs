use crate::fixtures::*;

pub fn debug_attach_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("debug_attach_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_iris", "north_room"),
            actor_schema("actor_jules", "south_room"),
        ],
        places: vec![
            place_schema("north_room", "North room", &["south_room"]),
            place_schema("south_room", "South room", &["north_room"]),
        ],
        doors: vec![door_schema(
            "door_north_south",
            "north_room",
            "south_room",
            true,
            false,
        )],
        containers: Vec::new(),
        items: vec![item_at_place("lantern_01", "north_room", true)],
        affordances: vec![
            affordance("move", "south_room"),
            affordance("take", "lantern_01"),
            affordance("inspect_place", "north_room"),
            affordance("inspect_place", "south_room"),
            affordance("inspect_entity", "lantern_01"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_iris", NeedKind::Hunger, 100),
            initial_need("actor_iris", NeedKind::Fatigue, 100),
            initial_need("actor_iris", NeedKind::Safety, 100),
            initial_need("actor_jules", NeedKind::Hunger, 100),
            initial_need("actor_jules", NeedKind::Fatigue, 100),
            initial_need("actor_jules", NeedKind::Safety, 100),
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
            fixture_id: "debug_attach_001",
            purpose: "Prove controller binding is non-world metadata and debug attach/switch has no player privilege.",
            setup: vec![
                "actor_iris and actor_jules are ordinary actors in deterministic places",
                "no world events are required before controller binding",
                "lantern_01 is an ordinary physical item for shared-pipeline action proof",
            ],
            allowed_actions: vec![
                "bind controller to actor_iris",
                "switch/debug attach to actor_jules",
                "build embodied and debug views",
                "take lantern_01 through ordinary action pipeline when reachable",
            ],
            expected_events_or_reports: vec![
                "ControllerAttached only in controller stream or run metadata",
                "stable physical checksum before and after binding changes",
                "debug view explicitly non-diegetic",
            ],
            acceptance_assertions: vec![
                "controller binding does not change physical state",
                "actor state does not gain knowledge or flags",
                "available ordinary actions are the same actions non-human proposals can use",
                "fixture contains no player/protagonist entity",
            ],
        },
    }
}
