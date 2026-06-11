use crate::fixtures::*;

pub fn knowledge_blocker_accuse_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("knowledge_blocker_accuse_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase2AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "street_lane"),
            actor_schema("actor_tomas", "house_tomas"),
        ],
        places: vec![
            place_schema("house_tomas", "Tomas house", &["street_lane"]),
            place_schema("street_lane", "Street lane", &["house_tomas"]),
        ],
        doors: vec![door_schema(
            "door_house_street",
            "house_tomas",
            "street_lane",
            true,
            false,
        )],
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
            affordance("truthful_accuse_probe", "actor_mara"),
            affordance("open", "strongbox_tomas"),
            affordance("check_container", "strongbox_tomas"),
            affordance("inspect_place", "house_tomas"),
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
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "knowledge_blocker_accuse_001",
            purpose:
                "Provide a fixture where accusation is blocked unless actor-known support exists.",
            setup: vec![
                "actor_mara is an ordinary actor, not an authored culprit",
                "actor_tomas has an expectation about the coin location",
                "no observation links actor_mara to any missing item",
            ],
            allowed_actions: vec![
                "truthful accuse probe actor_mara",
                "open strongbox_tomas",
                "check strongbox_tomas",
                "inspect local view",
            ],
            expected_events_or_reports: vec![
                "KnowledgePreconditionNotMet before actor-known support exists",
                "actor-visible why-not summary",
            ],
            acceptance_assertions: vec![
                "the fixture never encodes culprit truth",
                "accusation gating is derived from actor knowledge, not fixture flags",
            ],
        },
    }
}
