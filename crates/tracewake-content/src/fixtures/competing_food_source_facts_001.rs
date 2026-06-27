use crate::fixtures::*;
use crate::schema::KnownFoodSourceSchema;
use tracewake_core::ids::FoodSupplyId;

pub fn competing_food_source_facts_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("competing_food_source_facts_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_empty_bowl_tomas")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 880),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_empty_bowl_tomas",
            "home_tomas",
            0,
            220,
        )],
        known_food_sources: vec![KnownFoodSourceSchema {
            actor_id: actor("actor_tomas"),
            food_supply_id: FoodSupplyId::new("food_empty_bowl_tomas").unwrap(),
        }],
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "competing_food_source_facts_001",
            purpose: "Prove source-bearing observed food-source facts supersede source-less seed knowledge at the public actor-visible boundary.",
            setup: vec![
                "actor_tomas has an authored known_food_sources edge actor_tomas -> food_empty_bowl_tomas",
                "food_empty_bowl_tomas is visible at home_tomas with zero servings",
                "the authored seed provides source-less known-food identity while current perception provides serving knowledge",
            ],
            allowed_actions: vec![
                "seed a household_food_source belief for food_empty_bowl_tomas",
                "build embodied view through the public TUI/runtime boundary",
                "disable eat from holder-known observed zero servings",
            ],
            expected_events_or_reports: vec![
                "actor_tomas receives household_food_source StartingBeliefRecorded",
                "embodied semantic action eat.food.food_empty_bowl_tomas is disabled",
                "disabled action cites holder-known context provenance",
            ],
            acceptance_assertions: vec![
                "source-bearing serving knowledge supersedes source-less food knowledge",
                "the source-less seed fact does not keep the eat action enabled",
                "no raw truth or hidden provenance is rendered to the embodied viewer",
            ],
        },
    }
}
