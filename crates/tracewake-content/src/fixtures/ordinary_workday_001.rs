use crate::fixtures::*;

pub fn ordinary_workday_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("ordinary_workday_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &["workshop_tomas"]),
            place_schema("workshop_tomas", "Tomas workshop", &["home_tomas"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("move", "workshop_tomas"),
            affordance("move", "home_tomas"),
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 220),
            initial_need("actor_tomas", NeedKind::Fatigue, 180),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: vec![sleep_place_schema("actor_tomas", "home_tomas", "bed_tomas")],
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_tomas",
            "workshop_tomas",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: vec![
            routine_template_schema(
                "routine_tomas_go_to_work",
                RoutineFamily::GoToWork,
                vec![routine_step("move_toward_place", "move")],
                &["route_blocked"],
            ),
            routine_template_schema(
                "routine_tomas_work_block",
                RoutineFamily::WorkBlock,
                vec![routine_step("start_work_block", "work_block")],
                &["workplace_closed", "need_blocked"],
            ),
        ],
        routine_assignments: vec![
            routine_assignment_schema("actor_tomas", "routine_tomas_go_to_work", 4, 8),
            routine_assignment_schema("actor_tomas", "routine_tomas_work_block", 8, 14),
        ],
        day_windows: vec![day_window_schema("actor_tomas", 0, 24)],
    };
    #[expect(
        clippy::disallowed_methods,
        reason = "legacy fixture blanket food-source seeding is pinned by fixtures_load census; new fixtures must author per-actor known_food_sources edges"
    )]
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "ordinary_workday_001",
            purpose: "Prove a focused workday uses movement ancestry before modeled work.",
            setup: vec![
                "actor_tomas starts at home_tomas",
                "workplace_tomas is located at adjacent workshop_tomas",
                "work access is open and actor_tomas is assigned",
            ],
            allowed_actions: vec!["move to workshop_tomas", "start and complete work_block"],
            expected_events_or_reports: vec![
                "ActorMoved from home_tomas to workshop_tomas",
                "WorkBlockStarted at workplace_tomas",
                "WorkBlockCompleted with modeled need deltas",
            ],
            acceptance_assertions: vec![
                "workplace presence is reached by movement, not teleport",
                "work completion or failure is represented by modeled events",
                "need changes, if any, are emitted as event effects",
            ],
        },
    }
}
