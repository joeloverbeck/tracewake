use crate::fixtures::*;

pub fn embodied_view_omits_unknown_sleep_affordance_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_view_omits_unknown_sleep_affordance_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("sleep", "home_tomas")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_tomas", NeedKind::Fatigue, 900)],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_view_omits_unknown_sleep_affordance_001",
            purpose: "Adversarial fixture: raw sleep affordance is omitted from embodied actions without a sealed context fact.",
            setup: vec!["an open sleep affordance exists at the actor's current place"],
            allowed_actions: vec!["sleep only when a holder-known sleep affordance fact exists"],
            expected_events_or_reports: vec!["embodied output omits sleep.here"],
            acceptance_assertions: vec![
                "raw sleep_affordances are not an embodied affordance source",
                "debug may compare truth non-diegetically",
            ],
        },
    }
}
