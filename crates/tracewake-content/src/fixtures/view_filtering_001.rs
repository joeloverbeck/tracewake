use crate::fixtures::*;

pub fn view_filtering_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("view_filtering_001"),
        schema_version: schema_version(),
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
            affordance("check_container", "strongbox_tomas"),
            affordance("inspect_place", "house_tomas"),
            affordance("inspect_entity", "strongbox_tomas"),
        ],
        initial_beliefs: vec![tomas_coin_expectation_seed()],
    };
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
