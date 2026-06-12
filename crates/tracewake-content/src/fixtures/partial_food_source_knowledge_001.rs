use crate::fixtures::*;
use crate::schema::KnownFoodSourceSchema;
use tracewake_core::ids::FoodSupplyId;

pub fn partial_food_source_knowledge_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("partial_food_source_knowledge_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "home_mara"),
            actor_schema("actor_tomas", "home_mara"),
        ],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_shared_pantry")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 880),
            initial_need("actor_tomas", NeedKind::Hunger, 880),
            initial_need("actor_mara", NeedKind::Fatigue, 100),
            initial_need("actor_mara", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![
            home_schema("actor_mara", "home_mara"),
            home_schema("actor_tomas", "home_mara"),
        ],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_shared_pantry",
            "home_mara",
            2,
            220,
        )],
        known_food_sources: vec![KnownFoodSourceSchema {
            actor_id: actor("actor_mara"),
            food_supply_id: FoodSupplyId::new("food_shared_pantry").unwrap(),
        }],
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_partial_food_find",
            RoutineFamily::FindFood,
            vec![routine_step("consume_accessible_food", "eat")],
            &["no_known_food_sources"],
        )],
        routine_assignments: vec![
            routine_assignment_schema("actor_mara", "routine_partial_food_find", 0, 6),
            routine_assignment_schema("actor_tomas", "routine_partial_food_find", 0, 6),
        ],
        day_windows: vec![
            day_window_schema("actor_mara", 0, 8),
            day_window_schema("actor_tomas", 0, 8),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "partial_food_source_knowledge_001",
            purpose: "Prove authored food-source seed knowledge can be partial across actors.",
            setup: vec![
                "actor_mara and actor_tomas start hungry at home_mara",
                "food_shared_pantry physically exists at home_mara",
                "only actor_mara has an explicit known_food_sources edge",
            ],
            allowed_actions: vec![
                "seed a household_food_source belief for the edged actor",
                "withhold that belief from the actor without an edge",
                "plan only from actor-known food-source state",
            ],
            expected_events_or_reports: vec![
                "actor_mara receives household_food_source StartingBeliefRecorded",
                "actor_tomas receives no household_food_source StartingBeliefRecorded",
                "actor_tomas cannot target food_shared_pantry through seed truth",
            ],
            acceptance_assertions: vec![
                "partial food knowledge is authored by explicit per-actor edge",
                "physical food truth does not imply every actor knows it",
                "fixture replay is deterministic",
            ],
        },
    }
}
