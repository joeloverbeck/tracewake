use crate::fixtures::*;

pub fn routine_no_teleport_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("routine_no_teleport_001"),
        schema_version: schema_version(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &[]),
            place_schema("remote_workshop", "Remote workshop", &[]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("work_block", "workplace_remote")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 260),
            initial_need("actor_tomas", NeedKind::Fatigue, 260),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_remote",
            "remote_workshop",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_tomas_remote_work_no_teleport",
            RoutineFamily::WorkBlock,
            vec![routine_step("start_work_block", "work_block")],
            &["actor_not_at_workplace", "route_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_remote_work_no_teleport",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "routine_no_teleport_001",
            purpose: "Prove a routine cannot start remote work without movement ancestry.",
            setup: vec![
                "actor_tomas starts at home_tomas",
                "workplace_remote is at remote_workshop",
                "no adjacency or door connects the places",
            ],
            allowed_actions: vec!["attempt work_block at workplace_remote"],
            expected_events_or_reports: vec![
                "WorkBlockFailed with access reason",
                "no ActorMoved event",
                "no WorkBlockStarted event",
            ],
            acceptance_assertions: vec![
                "missing movement ancestry prevents remote work",
                "the actor remains at home_tomas",
                "failure is typed rather than teleported success",
            ],
        },
    }
}
