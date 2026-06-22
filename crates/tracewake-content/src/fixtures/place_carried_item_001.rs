use crate::fixtures::*;

pub fn place_carried_item_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("place_carried_item_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase1,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_lina", "market_stall")],
        places: vec![place_schema("market_stall", "Market stall", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: vec![item_carried_by("sample_token_01", "actor_lina", true)],
        affordances: vec![
            affordance("place", "sample_token_01"),
            affordance("inspect_place", "market_stall"),
            affordance("inspect_entity", "sample_token_01"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_lina", NeedKind::Hunger, 100),
            initial_need("actor_lina", NeedKind::Fatigue, 100),
            initial_need("actor_lina", NeedKind::Safety, 100),
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
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "place_carried_item_001",
            purpose: "Focused parity fixture: a carried item presents the place semantic action at bind time.",
            setup: vec![
                "actor_lina starts in market_stall",
                "sample_token_01 starts carried by actor_lina",
                "place affordance is authored for sample_token_01",
            ],
            allowed_actions: vec!["place carried sample_token_01 at current place"],
            expected_events_or_reports: vec![
                "ItemPlacedInPlace",
                "validation report for accepted place action",
            ],
            acceptance_assertions: vec![
                "place action is visible from carried-item state",
                "fixture is additive and does not alter existing item-movement fixtures",
            ],
        },
    }
}
