use crate::fixtures::*;

pub fn strongbox_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("strongbox_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        actors: vec![
            actor_schema("actor_elena", "house_tomas"),
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
            affordance("open", "strongbox_tomas"),
            affordance("close", "strongbox_tomas"),
            affordance("take", "coin_stack_01"),
            affordance("place", "coin_stack_01"),
            affordance("move", "street_lane"),
            affordance("inspect_place", "house_tomas"),
            affordance("inspect_entity", "strongbox_tomas"),
        ],
        initial_beliefs: vec![tomas_coin_expectation_seed()],
        initial_needs: Vec::new(),
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: Vec::new(),
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "strongbox_001",
            purpose: "Physical strongbox/value-token/custody baseline narrowed to the Phase 1 physical kernel.",
            setup: vec![
                "actor_tomas and actor_elena exist as ordinary actors",
                "house_tomas contains strongbox_tomas",
                "coin_stack_01 is a physical item initially in strongbox_tomas",
                "door_house_street provides local movement topology",
            ],
            allowed_actions: vec![
                "open strongbox_tomas",
                "close strongbox_tomas",
                "take coin_stack_01",
                "place coin_stack_01",
                "move street_lane",
                "inspect house_tomas and strongbox_tomas",
            ],
            expected_events_or_reports: vec![
                "ContainerOpened",
                "ItemRemovedFromContainer",
                "ItemPlacedInPlace or ItemPlacedInContainer",
                "item-location debug report",
                "matching replay checksum/report",
            ],
            acceptance_assertions: vec![
                "fixture validates with stable physical IDs",
                "coin_stack_01 is not an abstract balance",
                "debug report identifies fixture-origin location inside strongbox_tomas",
                "only Tomas's source-backed authored-prehistory expectation is seeded; no contradiction, suspicion, report, institution, quest, reward, player, or outcome script is asserted",
            ],
        },
    }
}
