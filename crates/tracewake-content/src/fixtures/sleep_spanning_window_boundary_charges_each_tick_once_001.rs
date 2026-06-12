use crate::fixtures::*;

pub fn sleep_spanning_window_boundary_charges_each_tick_once_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("sleep_spanning_window_boundary_charges_each_tick_once_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_elena", "home_elena")],
        places: vec![place_schema("home_elena", "Elena home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("sleep", "home_elena")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_elena", NeedKind::Fatigue, 720),
            initial_need("actor_elena", NeedKind::Hunger, 300),
        ],
        homes: vec![home_schema("actor_elena", "home_elena")],
        sleep_places: vec![sleep_place_schema("actor_elena", "home_elena", "bed_elena")],
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_elena_boundary_sleep",
            RoutineFamily::SleepNight,
            vec![routine_step("start_scheduled_sleep", "sleep")],
            &["sleep_place_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_elena",
            "routine_elena_boundary_sleep",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_elena", 0, 8)],
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
            fixture_id: "sleep_spanning_window_boundary_charges_each_tick_once_001",
            purpose: "Adversarial fixture: sleep spans a no-human decision window boundary and must not receive passive awake need deltas for slept ticks.",
            setup: vec![
                "actor_elena starts at an authored sleep place",
                "a sleep routine begins before the next no-human decision window",
            ],
            allowed_actions: vec!["start scheduled sleep", "complete scheduled sleep"],
            expected_events_or_reports: vec![
                "SleepStarted before the boundary",
                "SleepCompleted at the boundary",
                "no passive awake NeedDeltaApplied event over slept ticks",
            ],
            acceptance_assertions: vec![
                "sleep ticks are charged exactly once",
                "passive elapsed_ticks excludes ticks classified as asleep",
            ],
        },
    }
}
