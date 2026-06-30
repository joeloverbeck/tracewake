use crate::fixtures::*;

pub fn embodied_continue_hidden_workplace_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_continue_hidden_workplace_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &["hidden_workshop"]),
            concealed_place_schema("hidden_workshop", "Hidden workshop", &["home_tomas"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("move", "hidden_workshop"),
            affordance("work_block", "workplace_hidden"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 220),
            initial_need("actor_tomas", NeedKind::Fatigue, 180),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_hidden",
            "hidden_workshop",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_tomas_hidden_go_to_work",
            RoutineFamily::GoToWork,
            vec![routine_step("move_toward_place", "move")],
            &["route_blocked", "access"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_hidden_go_to_work",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_continue_hidden_workplace_001",
            purpose: "Prove embodied routine continuation cannot plan from hidden workplace truth.",
            setup: vec![
                "actor_tomas starts at home_tomas",
                "workplace_hidden exists in concealed hidden_workshop",
                "actor_tomas has a go-to-work routine but no actor-known route or workplace fact",
            ],
            allowed_actions: vec!["attempt continue_routine"],
            expected_events_or_reports: vec![
                "ContinueRoutineProposed marker",
                "typed stuck or why-not diagnostic",
                "no ActorMoved event",
                "no WorkBlockStarted event",
            ],
            acceptance_assertions: vec![
                "hidden workplace truth does not select a move target",
                "blocked continuation is typed and actor-safe",
                "actor remains at home_tomas",
            ],
        },
    }
}
