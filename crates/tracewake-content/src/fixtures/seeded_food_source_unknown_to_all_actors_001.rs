use crate::fixtures::*;

pub fn seeded_food_source_unknown_to_all_actors_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("seeded_food_source_unknown_to_all_actors_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: vec![container_schema(
            "hidden_pantry",
            "home_mara",
            false,
            false,
            &[],
            false,
        )],
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_hidden_pantry")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 880),
            initial_need("actor_mara", NeedKind::Fatigue, 100),
            initial_need("actor_mara", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_in_container(
            "food_hidden_pantry",
            "hidden_pantry",
            2,
            220,
        )],
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_mara_find_food",
            RoutineFamily::FindFood,
            vec![routine_step("consume_accessible_food", "eat")],
            &["no_known_food_sources"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_mara",
            "routine_mara_find_food",
            0,
            6,
        )],
        day_windows: vec![day_window_schema("actor_mara", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "seeded_food_source_unknown_to_all_actors_001",
            purpose:
                "Prove food-source seed knowledge is authored by explicit edge, not physical truth.",
            setup: vec![
                "actor_mara starts hungry at home_mara",
                "food_hidden_pantry physically exists in a closed hidden_pantry",
                "no known_food_source edge is authored",
            ],
            allowed_actions: vec![
                "run no-human decision from actor-known seed state",
                "wait when no actor-known food source exists",
            ],
            expected_events_or_reports: vec![
                "no household_food_source StartingBeliefRecorded event is seeded",
                "actor-known context exposes no food source",
                "no food consumption or eat failure targets hidden food",
            ],
            acceptance_assertions: vec![
                "absence of authored edge means absence of food belief",
                "planner cannot target hidden physical food through seed truth",
                "fixture replay is deterministic",
            ],
        },
    }
}
