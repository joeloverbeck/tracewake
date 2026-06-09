use crate::fixtures::*;

pub fn container_item_move_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("container_item_move_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mira", "workroom")],
        places: vec![place_schema("workroom", "Workroom", &[])],
        doors: Vec::new(),
        containers: vec![
            container_schema(
                "source_chest",
                "workroom",
                false,
                false,
                &["brass_key_01"],
                false,
            ),
            container_schema("destination_crate", "workroom", true, false, &[], true),
        ],
        items: vec![item_in_container("brass_key_01", "source_chest", true)],
        affordances: vec![
            affordance("open", "source_chest"),
            affordance("close", "source_chest"),
            affordance("take", "brass_key_01"),
            affordance("place", "brass_key_01"),
            affordance("inspect_place", "workroom"),
            affordance("inspect_entity", "source_chest"),
            affordance("inspect_entity", "destination_crate"),
        ],
        initial_beliefs: Vec::new(),
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
            fixture_id: "container_item_move_001",
            purpose: "Prove open/take/place event order and projection rebuild for item movement.",
            setup: vec![
                "actor_mira starts in workroom",
                "source_chest starts closed with brass_key_01 inside",
                "destination_crate starts open in the same place",
            ],
            allowed_actions: vec![
                "inspect local view",
                "open source_chest",
                "take brass_key_01 from source_chest",
                "place brass_key_01 into destination_crate or workroom",
            ],
            expected_events_or_reports: vec![
                "ContainerOpened",
                "ItemRemovedFromContainer",
                "ItemPlacedInContainer or ItemPlacedInPlace",
                "validation reports for accepted actions",
                "matching replay checksum/report",
            ],
            acceptance_assertions: vec![
                "attempting to take brass_key_01 from a closed or wrong holder rejects without mutation",
                "projection rebuild matches live item location",
                "fixture contains no authored outcome chain",
            ],
        },
    }
}
