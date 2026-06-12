use crate::fixtures::*;

pub fn view_filtering_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("view_filtering_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase2AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_elena", "house_tomas"),
            actor_schema("actor_tomas", "house_tomas"),
        ],
        places: vec![place_schema("house_tomas", "Tomas house", &[])],
        doors: Vec::new(),
        containers: vec![container_schema(
            "strongbox_tomas",
            "house_tomas",
            false,
            false,
            &["coin_stack_01"],
            false,
        )],
        items: vec![item_in_container("coin_stack_01", "strongbox_tomas", true)],
        affordances: vec![
            affordance("open", "strongbox_tomas"),
            affordance("check_container", "strongbox_tomas"),
            affordance("inspect_place", "house_tomas"),
            affordance("inspect_entity", "strongbox_tomas"),
        ],
        initial_beliefs: vec![tomas_coin_expectation_seed()],
        initial_needs: vec![
            initial_need("actor_elena", NeedKind::Hunger, 100),
            initial_need("actor_elena", NeedKind::Fatigue, 100),
            initial_need("actor_elena", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 100),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
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
            fixture_id: "view_filtering_001",
            purpose: "Provide actor-scoped seed data for embodied notebook filtering.",
            setup: vec![
                "actor_tomas and actor_elena share a place",
                "only actor_tomas has the strongbox expectation seed",
                "debug views may inspect all records but embodied views must not",
            ],
            allowed_actions: vec![
                "open strongbox_tomas",
                "check strongbox_tomas",
                "bind as actor_tomas",
                "bind as actor_elena",
                "render notebook and debug epistemics",
            ],
            expected_events_or_reports: vec![
                "NotebookView for actor_tomas contains the source-backed expectation",
                "NotebookView for actor_elena omits Tomas's private expectation",
            ],
            acceptance_assertions: vec![
                "actor-private seeds do not leak through embodied views",
                "debug output remains explicitly non-diegetic",
            ],
        },
    }
}
