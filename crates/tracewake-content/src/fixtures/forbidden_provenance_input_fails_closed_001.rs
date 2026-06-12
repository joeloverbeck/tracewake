use crate::fixtures::*;

pub fn forbidden_provenance_input_fails_closed_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("forbidden_provenance_input_fails_closed_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Hunger, 900)],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_mara", 0, 4)],
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
            fixture_id: "forbidden_provenance_input_fails_closed_001",
            purpose: "Negative fixture: forbidden actor-known input provenance fails closed before proposal construction.",
            setup: vec![
                "actor_mara has action-relevant need pressure",
                "core tests inject a typed forbidden provenance input with clean display text",
            ],
            allowed_actions: vec!["typed stuck diagnostic only"],
            expected_events_or_reports: vec![
                "blocker_code=hidden_truth_input",
                "hidden_truth_referenced=true",
                "no agent-origin proposal is constructed from forbidden input",
            ],
            acceptance_assertions: vec![
                "hidden-truth audit gates decision transaction",
                "pipeline rejects agent-origin dirty audit as defense in depth",
            ],
        },
    }
}
