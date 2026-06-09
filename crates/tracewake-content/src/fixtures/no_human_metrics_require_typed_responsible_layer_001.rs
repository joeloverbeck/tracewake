use crate::fixtures::*;

pub fn no_human_metrics_require_typed_responsible_layer_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_metrics_require_typed_responsible_layer_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: Vec::new(),
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_metrics_require_typed_responsible_layer_001",
            purpose: "Anchor the metric proof that planner-failure counts require typed diagnostic fields.",
            setup: vec![
                "actor_tomas has a minimal no-human day surface",
                "metric assertions construct typed and untyped diagnostic events at the core event-log seam",
            ],
            allowed_actions: vec!["metric projection over no-human diagnostic events"],
            expected_events_or_reports: vec![
                "untyped planner-looking text is not counted",
                "typed local_planning diagnostic is counted",
            ],
            acceptance_assertions: vec![
                "planner_failures derive from responsible_layer/blocker_code",
                "planner_failures do not derive from display text",
            ],
        },
    }
}
