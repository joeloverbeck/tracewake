use crate::fixtures::*;

pub fn sleep_eat_work_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("sleep_eat_work_001"),
        schema_version: schema_version(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &["workshop_tomas"]),
            place_schema("workshop_tomas", "Tomas workshop", &["home_tomas"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("sleep", "home_tomas"),
            affordance("eat", "food_breakfast_tomas"),
            affordance("move", "workshop_tomas"),
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 460),
            initial_need("actor_tomas", NeedKind::Fatigue, 620),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: vec![sleep_place_schema("actor_tomas", "home_tomas", "bed_tomas")],
        food_supplies: vec![food_supply_at_place(
            "food_breakfast_tomas",
            "home_tomas",
            2,
            220,
        )],
        workplaces: vec![workplace_schema(
            "workplace_tomas",
            "workshop_tomas",
            &["actor_tomas"],
            3,
            true,
        )],
        routine_templates: vec![
            routine_template_schema(
                "routine_tomas_sleep_night",
                RoutineFamily::SleepNight,
                vec![routine_step("start_scheduled_sleep", "sleep")],
                &["sleep_place_blocked"],
            ),
            routine_template_schema(
                "routine_tomas_eat_meal",
                RoutineFamily::EatMeal,
                vec![routine_step("consume_accessible_food", "eat")],
                &["food_missing", "food_inaccessible"],
            ),
            routine_template_schema(
                "routine_tomas_work_block",
                RoutineFamily::WorkBlock,
                vec![routine_step("start_work_block", "work_block")],
                &["workplace_closed", "need_blocked"],
            ),
        ],
        routine_assignments: vec![
            routine_assignment_schema("actor_tomas", "routine_tomas_sleep_night", 0, 4),
            routine_assignment_schema("actor_tomas", "routine_tomas_eat_meal", 4, 6),
            routine_assignment_schema("actor_tomas", "routine_tomas_work_block", 8, 12),
        ],
        day_windows: vec![day_window_schema("actor_tomas", 0, 24)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "sleep_eat_work_001",
            purpose: "Prove sleep, food consumption, movement, and work integrate through logged effects.",
            setup: vec![
                "actor_tomas starts at home_tomas with hunger and fatigue",
                "bed_tomas and food_breakfast_tomas are reachable at home_tomas",
                "workplace_tomas is adjacent and assigned",
            ],
            allowed_actions: vec![
                "sleep at home_tomas",
                "eat food_breakfast_tomas",
                "move to workshop_tomas",
                "start and complete work_block",
            ],
            expected_events_or_reports: vec![
                "SleepStarted and SleepCompleted",
                "FoodConsumed and hunger NeedDeltaApplied",
                "ActorMoved",
                "WorkBlockStarted and WorkBlockCompleted",
            ],
            acceptance_assertions: vec![
                "need changes are logged by sleep, food, and work events",
                "workplace access follows movement ancestry",
                "fixture replay is deterministic",
            ],
        },
    }
}
