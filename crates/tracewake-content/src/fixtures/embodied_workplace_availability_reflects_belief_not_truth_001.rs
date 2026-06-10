use crate::fixtures::*;

pub fn embodied_workplace_availability_reflects_belief_not_truth_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_workplace_availability_reflects_belief_not_truth_001"),
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
        affordances: vec![affordance("work_block", "workplace_tomas")],
        initial_beliefs: Vec::new(),
        initial_needs: Vec::new(),
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_tomas",
            "workshop_tomas",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_workplace_availability_reflects_belief_not_truth_001",
            purpose: "Prove embodied workplace availability follows holder-known workplace belief rather than open workplace truth.",
            setup: vec![
                "truth contains an open assigned workplace for actor_tomas",
                "no context-backed workplace notice or belief is present",
            ],
            allowed_actions: vec!["embodied view may show route knowledge but no work_block semantic action"],
            expected_events_or_reports: vec![
                "truth_open_workplace_absent_from_embodied_actions_without_belief",
            ],
            acceptance_assertions: vec![
                "truth-open workplace state is insufficient for embodied availability",
                "workplace_tomas does not appear in embodied semantic actions without context fact",
            ],
        },
    }
}
