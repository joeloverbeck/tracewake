use crate::fixtures::*;

pub fn embodied_menu_lags_truth_change_without_perception_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_menu_lags_truth_change_without_perception_001"),
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
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: Vec::new(),
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_breakfast_tomas",
            "home_tomas",
            2,
            220,
        )],
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_menu_lags_truth_change_without_perception_001",
            purpose: "Anchor the Phase 3A staleness regression where embodied menu facts come from retained perception rather than live truth.",
            setup: vec![
                "actor_tomas starts at home_tomas with visible food_breakfast_tomas",
                "runtime test mutates food truth before recording a later perception event",
            ],
            allowed_actions: vec!["eat action is projection-backed after modeled perception"],
            expected_events_or_reports: vec![
                "embodied facts persist across truth mutation without perception",
                "latest same-place perception window refreshes embodied facts",
            ],
            acceptance_assertions: vec![
                "live food truth alone does not add or remove embodied menu facts",
                "new perception event updates the projection-backed embodied context",
            ],
        },
    }
}
