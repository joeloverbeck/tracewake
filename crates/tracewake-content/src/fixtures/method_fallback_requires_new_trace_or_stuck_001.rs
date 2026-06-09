use crate::fixtures::*;

pub fn method_fallback_requires_new_trace_or_stuck_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("method_fallback_requires_new_trace_or_stuck_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 850),
            initial_need("actor_mara", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_mara", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "method_fallback_requires_new_trace_or_stuck_001",
            purpose: "Prove method fallback cannot mix one candidate's decision trace with another candidate's proposal.",
            setup: vec![
                "actor_mara has pressure that can generate a high-priority candidate without sufficient actor-known method inputs",
                "fallback behavior must rerun selection coherently or end with typed method_selection stuck",
            ],
            allowed_actions: vec!["coherent fallback wait or typed stuck diagnostic"],
            expected_events_or_reports: vec![
                "proposal decision_trace_id and candidate_goal_id come from one SelectedGoalBundle",
                "terminal stuck diagnostics use responsible_layer=method_selection when no candidate can yield a method",
            ],
            acceptance_assertions: vec![
                "no post-trace fallback scan can silently swap candidate ids",
                "fallback candidate rejection is preserved as typed method_selection evidence",
            ],
        },
    }
}
