use crate::fixtures::*;

pub fn embodied_view_omits_raw_assignment_without_context_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_view_omits_raw_assignment_without_context_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
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
            fixture_id: "embodied_view_omits_raw_assignment_without_context_001",
            purpose: "Prove embodied views do not expose raw workplace assignment as actor-known work affordance.",
            setup: vec![
                "actor_tomas has a raw assigned workplace in authored state",
                "no modeled routine assignment or context-backed workplace notice is present",
            ],
            allowed_actions: vec!["embodied view may show movement but no work_block semantic action"],
            expected_events_or_reports: vec!["no embodied work affordance for workplace_tomas"],
            acceptance_assertions: vec![
                "raw workplace assignment alone is not holder-known context",
                "workplace_tomas does not appear in embodied semantic actions",
            ],
        },
    }
}
