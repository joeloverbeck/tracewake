use crate::fixtures::*;

pub fn replay_item_location_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("replay_item_location_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "archive_room")],
        places: vec![place_schema("archive_room", "Archive room", &[])],
        doors: Vec::new(),
        containers: vec![container_schema(
            "evidence_box",
            "archive_room",
            true,
            false,
            &["receipt_01"],
            true,
        )],
        items: vec![item_in_container("receipt_01", "evidence_box", true)],
        affordances: vec![
            affordance("take", "receipt_01"),
            affordance("place", "receipt_01"),
            affordance("inspect_place", "archive_room"),
            affordance("inspect_entity", "receipt_01"),
            affordance("inspect_entity", "evidence_box"),
        ],
        initial_beliefs: Vec::new(),
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
            fixture_id: "replay_item_location_001",
            purpose: "Prove item location derives from initial fixture plus ordered events, not mutable current-state accident.",
            setup: vec![
                "receipt_01 has fixture-origin location in evidence_box",
                "actor_mara can move receipt_01 through ordinary take/place actions",
            ],
            allowed_actions: vec![
                "take receipt_01 from evidence_box",
                "place receipt_01 at archive_room or back into evidence_box",
                "rebuild from initial fixture plus ordered events",
                "ask item-location debug report on live and replayed state",
            ],
            expected_events_or_reports: vec![
                "ItemRemovedFromContainer",
                "ItemPlacedInPlace or ItemPlacedInContainer",
                "projection rebuild report",
                "item-location debug report",
                "matching replay checksum/report",
            ],
            acceptance_assertions: vec![
                "final item holder/location matches live run",
                "item-location report names last location-changing event",
                "deleting or reordering item movement event causes replay report failure",
            ],
        },
    }
}
