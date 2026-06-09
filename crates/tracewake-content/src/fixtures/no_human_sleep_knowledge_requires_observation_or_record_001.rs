use crate::fixtures::*;

pub fn no_human_sleep_knowledge_requires_observation_or_record_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_sleep_knowledge_requires_observation_or_record_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        actors: vec![actor_schema("actor_elena", "home_elena")],
        places: vec![place_schema("home_elena", "Elena home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("sleep", "home_elena")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_elena", NeedKind::Fatigue, 920)],
        homes: vec![home_schema("actor_elena", "home_elena")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_elena_sleep",
            RoutineFamily::SleepNight,
            vec![RoutineStep::FailWithTypedDiagnostic {
                diagnostic: "no_sleep_affordance: disabled sleep evidence channel".to_string(),
            }],
            &["sleep_place_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_elena",
            "routine_elena_sleep",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_elena", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_sleep_knowledge_requires_observation_or_record_001",
            purpose: "Negative fixture: no-human sleep planning requires evented sleep observation or authored record.",
            setup: vec![
                "actor_elena has fatigue pressure and an active sleep routine",
                "tests clear observation/seed evidence to disable sleep knowledge",
            ],
            allowed_actions: vec!["sleep only when observation or seed record exists"],
            expected_events_or_reports: vec![
                "disabled sleep evidence yields holder_known_context stuck diagnostic",
            ],
            acceptance_assertions: vec![
                "no sleep fact without observation or record",
                "no sleep proposal when the channel is disabled",
            ],
        },
    }
}
