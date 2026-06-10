use crate::fixtures::*;

pub fn view_model_local_actions_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("view_model_local_actions_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_lina", "market_stall")],
        places: vec![
            place_schema("market_stall", "Market stall", &["store_room"]),
            place_schema("store_room", "Store room", &["market_stall"]),
        ],
        doors: vec![door_schema(
            "door_market_store",
            "market_stall",
            "store_room",
            false,
            false,
        )],
        containers: vec![container_schema(
            "sample_bin",
            "market_stall",
            true,
            false,
            &["sample_token_01"],
            true,
        )],
        items: vec![item_in_container("sample_token_01", "sample_bin", true)],
        affordances: vec![
            affordance("move", "store_room"),
            affordance("open", "door_market_store"),
            affordance("close", "door_market_store"),
            affordance("take", "sample_token_01"),
            affordance("place", "sample_token_01"),
            affordance("inspect_place", "market_stall"),
            affordance("inspect_entity", "door_market_store"),
            affordance("inspect_entity", "sample_bin"),
            affordance("inspect_entity", "sample_token_01"),
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
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "view_model_local_actions_001",
            purpose: "Prove the TUI/view-model exposes stable semantic local actions without UI rule ownership.",
            setup: vec![
                "actor_lina starts in market_stall",
                "market_stall has an exit through a closed door",
                "sample_bin and sample_token_01 provide a local item interaction path",
            ],
            allowed_actions: vec![
                "bind controller to actor_lina",
                "build embodied view model",
                "select stable semantic action ID",
                "submit enabled action through shared pipeline",
                "submit disabled action and display why-not",
                "open debug panel without leaking debug truth",
            ],
            expected_events_or_reports: vec![
                "deterministic action list order",
                "DoorOpened or why-not validation report",
                "ItemRemovedFromContainer",
                "separate debug and embodied view models",
            ],
            acceptance_assertions: vec![
                "view model action IDs are stable and target-specific",
                "terminal menu order is not action identity",
                "TUI does not mutate state",
                "why-not derives from validation report",
                "transcript/snapshot is deterministic",
            ],
        },
    }
}
