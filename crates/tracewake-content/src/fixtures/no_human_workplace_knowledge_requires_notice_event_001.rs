use crate::fixtures::*;

pub fn no_human_workplace_knowledge_requires_notice_event_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_workplace_knowledge_requires_notice_event_001"),
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
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 220),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_tomas",
            "workshop_tomas",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_tomas_work_block",
            RoutineFamily::WorkBlock,
            vec![routine_step("start_work_block", "work_block")],
            &["workplace_closed", "need_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_work_block",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
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
            fixture_id: "no_human_workplace_knowledge_requires_notice_event_001",
            purpose: "Negative fixture: workplace planning requires the evented role-notice channel, not raw assignment truth.",
            setup: vec![
                "actor_tomas has raw workplace assignment truth and an active work routine",
                "tests clear the seed event log to disable the role-notice channel",
            ],
            allowed_actions: vec!["work_block only when role notice exists"],
            expected_events_or_reports: vec![
                "disabled role-notice channel yields holder_known_context stuck diagnostic",
            ],
            acceptance_assertions: vec![
                "no workplace fact without role_assignment_notice event",
                "no work proposal when the channel is disabled",
            ],
        },
    }
}
