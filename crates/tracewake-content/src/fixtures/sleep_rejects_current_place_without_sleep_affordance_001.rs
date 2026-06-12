use crate::fixtures::*;

pub fn sleep_rejects_current_place_without_sleep_affordance_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("sleep_rejects_current_place_without_sleep_affordance_001"),
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
        initial_needs: vec![initial_need("actor_elena", NeedKind::Fatigue, 880)],
        homes: vec![home_schema("actor_elena", "home_elena")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
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
            fixture_id: "sleep_rejects_current_place_without_sleep_affordance_001",
            purpose: "Prove a sleep proposal at the current place fails without an authored sleep surface.",
            setup: vec![
                "actor_elena starts tired at home_elena",
                "home_elena has no authored sleep surface",
            ],
            allowed_actions: vec!["sleep proposal is rejected with NoSleepAffordance"],
            expected_events_or_reports: vec![
                "ActionRejected reason_code=no_sleep_affordance",
                "NoSleepAffordance typed rejection",
            ],
            acceptance_assertions: vec![
                "current place is not a sleep surface",
                "sleep validation requires authored rest-surface state",
            ],
        },
    }
}
