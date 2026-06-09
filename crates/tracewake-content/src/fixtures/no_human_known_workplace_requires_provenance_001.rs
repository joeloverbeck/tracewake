use crate::fixtures::*;

pub fn no_human_known_workplace_requires_provenance_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_known_workplace_requires_provenance_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
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
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_tomas", NeedKind::Hunger, 220)],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
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
            routine_assignment_schema("actor_tomas", "routine_tomas_go_to_work", 0, 4),
            routine_assignment_schema("actor_tomas", "routine_tomas_work_block", 4, 10),
        ],
        day_windows: vec![day_window_schema("actor_tomas", 0, 12)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_known_workplace_requires_provenance_001",
            purpose: "Prove workplace cognition is unlocked by modeled routine-assignment notice.",
            setup: vec![
                "actor_tomas has explicit go-to-work and work-block routine assignments",
                "workplace_tomas is reachable and assigned",
            ],
            allowed_actions: vec!["move to workshop_tomas", "start work_block after arrival"],
            expected_events_or_reports: vec![
                "role_assignment_notice provenance appears in actor-known proof sources",
                "workplace becomes usable only through modeled routine assignment notice",
            ],
            acceptance_assertions: vec![
                "workplace knowledge has provenance",
                "raw assignment without notice remains covered by the paired negative fixture",
            ],
        },
    }
}
