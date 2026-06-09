use crate::fixtures::*;

pub fn routine_blocked_diagnostic_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("routine_blocked_diagnostic_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_elena", "workshop_elena")],
        places: vec![place_schema("workshop_elena", "Elena workshop", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("work_block", "workplace_elena")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_elena", NeedKind::Hunger, 260),
            initial_need("actor_elena", NeedKind::Fatigue, 260),
        ],
        homes: vec![home_schema("actor_elena", "workshop_elena")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_elena",
            "workshop_elena",
            &["actor_elena"],
            4,
            false,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_elena_blocked_work",
            RoutineFamily::WorkBlock,
            vec![
                routine_step("start_work_block", "work_block"),
                wait_step("blocked routine diagnostic"),
            ],
            &["workplace_closed", "access"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_elena",
            "routine_elena_blocked_work",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_elena", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "routine_blocked_diagnostic_001",
            purpose: "Prove a blocked ordinary routine records a typed access diagnostic instead of looping silently.",
            setup: vec![
                "actor_elena starts at the assigned workplace place",
                "workplace_elena exists but access_open is false",
                "routine includes explicit fallback/diagnostic language",
            ],
            allowed_actions: vec![
                "attempt work_block at workplace_elena",
                "record access-blocked WorkBlockFailed",
                "wait only with a modeled blocked-routine reason",
            ],
            expected_events_or_reports: vec![
                "WorkBlockFailed with access blocker",
                "no WorkBlockStarted",
                "failure payload names workplace access closed",
            ],
            acceptance_assertions: vec![
                "blocked routine produces a typed event",
                "the actor does not silently idle without a reason",
                "closed access is modeled as fixture state, not a script",
            ],
        },
    }
}
