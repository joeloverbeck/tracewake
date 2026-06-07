use crate::fixtures::*;

pub fn planner_trace_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("planner_trace_001"),
        schema_version: schema_version(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &["market_square"]),
            place_schema("market_square", "Market square", &["home_tomas"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("move", "market_square"),
            affordance("eat", "food_market_stew"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_tomas", NeedKind::Hunger, 820)],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_market_stew",
            "market_square",
            1,
            240,
        )],
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_tomas_find_food_trace",
            RoutineFamily::FindFood,
            vec![
                routine_step("fallback_to_find_food", "eat"),
                wait_step("planner trace fallback"),
            ],
            &["no_known_food_sources", "search_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_find_food_trace",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_tomas", 0, 12)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "planner_trace_001",
            purpose: "Prove planner traces expose candidates, selected method, rejected goals, and hidden-truth audit.",
            setup: vec![
                "actor_tomas has severe hunger",
                "market food exists as ordinary content",
                "the test supplies only actor-known planner inputs",
            ],
            allowed_actions: vec!["generate candidate goals", "select method", "inspect trace"],
            expected_events_or_reports: vec![
                "candidate goals include eat/find-food alternatives",
                "trace records selected goal and rejected reasons",
                "hidden-truth audit remains actor-known-only",
            ],
            acceptance_assertions: vec![
                "planner trace has candidate goals",
                "selected method is explicit",
                "hidden truth is not consulted",
            ],
        },
    }
}
