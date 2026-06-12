use crate::fixtures::*;

pub fn possession_parity_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("possession_parity_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase2AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "house_tomas"),
            actor_schema("actor_tomas", "house_tomas"),
        ],
        places: vec![place_schema("house_tomas", "Tomas house", &[])],
        doors: Vec::new(),
        containers: vec![container_schema(
            "strongbox_tomas",
            "house_tomas",
            true,
            false,
            &["coin_stack_01"],
            true,
        )],
        items: vec![item_in_container("coin_stack_01", "strongbox_tomas", true)],
        affordances: vec![
            affordance("take", "coin_stack_01"),
            affordance("place", "coin_stack_01"),
            affordance("check_container", "strongbox_tomas"),
            affordance("inspect_place", "house_tomas"),
            affordance("inspect_entity", "strongbox_tomas"),
        ],
        initial_beliefs: vec![tomas_coin_expectation_seed()],
        initial_needs: Vec::new(),
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
            fixture_id: "possession_parity_001",
            purpose: "Prove Mara is an ordinary actor and possession changes use the same physical actions as any actor.",
            setup: vec![
                "actor_mara and actor_tomas are co-located ordinary actors",
                "coin_stack_01 starts as a physical item in strongbox_tomas",
                "no authored culpability or stolen flag is present",
            ],
            allowed_actions: vec![
                "take coin_stack_01",
                "place coin_stack_01",
                "check strongbox_tomas",
                "inspect local entities",
            ],
            expected_events_or_reports: vec![
                "ItemRemovedFromContainer",
                "ItemPlacedInPlace or ItemPlacedInContainer",
                "ContainerChecked",
            ],
            acceptance_assertions: vec![
                "actor_mara has no special-case theft action or knowledge flag",
                "fixture contains no culprit, stolen_flag, or npc_knows_truth field",
            ],
        },
    }
}
