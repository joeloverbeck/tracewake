use crate::fixtures::*;

pub fn no_human_observation_facts_cite_log_events_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_observation_facts_cite_log_events_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_bruno", "home_bruno")],
        places: vec![place_schema("home_bruno", "Bruno home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_stew_home_bruno")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_bruno", NeedKind::Hunger, 920)],
        homes: vec![home_schema("actor_bruno", "home_bruno")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_stew_home_bruno",
            "home_bruno",
            2,
            240,
        )],
        workplaces: Vec::new(),
        routine_templates: vec![routine_template_schema(
            "routine_bruno_eat",
            RoutineFamily::EatMeal,
            vec![routine_step("consume_accessible_food", "eat")],
            &["no_known_food_sources"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_bruno",
            "routine_bruno_eat",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_bruno", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_observation_facts_cite_log_events_001",
            purpose: "Proof fixture: no-human observation-derived facts cite source_event_ids present in the log.",
            setup: vec![
                "actor_bruno begins hungry at a place with visible food",
                "current-place perception records the food observation before decision",
            ],
            allowed_actions: vec!["eat visible food"],
            expected_events_or_reports: vec![
                "decision actor-known inputs include source_events for observation facts",
            ],
            acceptance_assertions: vec![
                "every committed actor-known input source_event_id resolves in the log",
                "recorded actor-known context hash recomputes from persisted inputs",
            ],
        },
    }
}
