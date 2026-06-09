use crate::fixtures::*;

pub fn no_hidden_truth_planning_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_hidden_truth_planning_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![
            place_schema("home_mara", "Mara home", &["hidden_workshop"]),
            place_schema("hidden_workshop", "Hidden workshop", &["home_mara"]),
        ],
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
        affordances: vec![
            affordance("eat", "food_hidden_pantry"),
            affordance("move", "hidden_workshop"),
            affordance("work_block", "workplace_hidden"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Hunger, 880)],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_in_container(
            "food_hidden_pantry",
            "hidden_pantry",
            1,
            220,
        )],
        workplaces: vec![workplace_schema(
            "workplace_hidden",
            "hidden_workshop",
            &[],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_mara_hidden_food_guard",
            RoutineFamily::EatMeal,
            vec![routine_step("consume_accessible_food", "eat")],
            &["food_missing", "food_inaccessible"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_mara",
            "routine_mara_hidden_food_guard",
            0,
            6,
        )],
        day_windows: vec![day_window_schema("actor_mara", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_hidden_truth_planning_001",
            purpose: "Prove hidden food truth does not enter planner inputs even when physical action failure can explain it.",
            setup: vec![
                "actor_mara starts hungry at home_mara",
                "food_hidden_pantry exists in a closed opaque container at home_mara",
                "hidden_workshop and workplace_hidden exist but are not assigned to actor_mara",
                "no initial belief grants actor_mara knowledge of that food",
            ],
            allowed_actions: vec![
                "run planner with actor-known inputs only",
                "attempt eat only as an explicit test proposal",
                "record typed access failure",
            ],
            expected_events_or_reports: vec![
                "planner hidden-truth audit is actor-known-only",
                "planner does not select hidden food",
                "no-human run does not select hidden_workshop or workplace_hidden",
                "EatFailed explains physical inaccessibility",
            ],
            acceptance_assertions: vec![
                "hidden physical food does not influence the chosen plan",
                "the actor receives no belief from fixture truth",
                "action failure is typed if the hidden target is forced by test",
            ],
        },
    }
}
