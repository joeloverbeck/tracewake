use crate::fixtures::*;

pub fn no_human_current_place_without_sleep_affordance_does_not_sleep_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id(
            "no_human_current_place_without_sleep_affordance_does_not_sleep_001",
        ),
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
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_elena", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_current_place_without_sleep_affordance_does_not_sleep_001",
            purpose: "Prove current place is not implicit sleep-place knowledge.",
            setup: vec![
                "actor_elena starts tired at home_elena",
                "no modeled sleep-place knowledge or sleep routine notice exists",
            ],
            allowed_actions: vec!["wait or record no progress without selecting sleep"],
            expected_events_or_reports: vec![
                "current place is absent from known_sleep_places by default",
                "no sleep proposal is selected without actor-known sleep provenance",
            ],
            acceptance_assertions: vec![
                "current-place sleep default is not restored",
                "sleep planning requires actor-known sleep surface provenance",
            ],
        },
    }
}
