use crate::fixtures::*;

pub fn food_unavailable_replan_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("food_unavailable_replan_001"),
        schema_version: schema_version(),
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_empty_pantry_mara")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Hunger, 900)],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_empty_pantry_mara",
            "home_mara",
            0,
            180,
        )],
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_mara_eat_meal",
            RoutineFamily::EatMeal,
            vec![
                routine_step("consume_accessible_food", "eat"),
                routine_step("fallback_to_find_food", "eat"),
                wait_step("known food unavailable"),
            ],
            &["food_missing", "food_inaccessible"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_mara",
            "routine_mara_eat_meal",
            0,
            6,
        )],
        day_windows: vec![day_window_schema("actor_mara", 0, 12)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "food_unavailable_replan_001",
            purpose: "Prove the canonical hunger to missing-food to typed failure chain.",
            setup: vec![
                "actor_mara starts hungry at home_mara",
                "food_empty_pantry_mara is actor-reachable but has zero servings",
                "the fixture offers fallback wait/replan surface but no refill shortcut",
            ],
            allowed_actions: vec![
                "attempt eat food_empty_pantry_mara",
                "record typed food-unavailable failure",
                "wait with modeled reason after failure",
            ],
            expected_events_or_reports: vec![
                "EatFailed with resource blocker_kind",
                "no FoodConsumed event",
                "ActorWaited can carry the replan/failure reason",
            ],
            acceptance_assertions: vec![
                "hunger is not reduced when food is unavailable",
                "the failure is event-recorded, not silently ignored",
                "the fixture contains no refill or hidden-truth shortcut",
            ],
        },
    }
}
