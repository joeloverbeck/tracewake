use crate::fixtures::*;

pub fn expectation_contradiction_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("expectation_contradiction_001"),
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
            &[],
            false,
        )],
        items: vec![item_carried_by("coin_stack_01", "actor_mara", true)],
        affordances: vec![
            affordance("open", "strongbox_tomas"),
            affordance("check_container", "strongbox_tomas"),
            affordance("truthful_accuse_probe", "actor_mara"),
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
            fixture_id: "expectation_contradiction_001",
            purpose: "Seed Tomas's expectation against a physical state where the expected coin is absent from the strongbox.",
            setup: vec![
                "actor_tomas starts at house_tomas with strongbox_tomas",
                "coin_stack_01 is physically carried by ordinary actor_mara",
                "Tomas has only a source-backed authored-prehistory expectation",
            ],
            allowed_actions: vec![
                "open strongbox_tomas",
                "check strongbox_tomas",
                "inspect house_tomas and strongbox_tomas",
                "probe whether knowledge would support accusing actor_mara",
            ],
            expected_events_or_reports: vec![
                "ContainerChecked",
                "ObservationRecorded",
                "ExpectationContradicted",
                "BeliefUpdated for missing expected item",
            ],
            acceptance_assertions: vec![
                "absence can contradict Tomas's expectation without naming a culprit",
                "fixture contains no culprit, stolen flag, quest, or player memory shortcut",
            ],
        },
    }
}
