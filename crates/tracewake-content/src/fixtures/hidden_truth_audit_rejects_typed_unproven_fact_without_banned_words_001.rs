use crate::fixtures::*;

pub fn hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id(
            "hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001",
        ),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![place_schema("home_mara", "Mara home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Hunger, 850)],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_mara", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001",
            purpose: "Prove the hidden-truth audit rejects forbidden typed provenance classes even when display text lacks banned tokens.",
            setup: vec![
                "actor_mara has hunger pressure",
                "the decision audit is exercised by a typed input ref with clean display text and forbidden source_class",
            ],
            allowed_actions: vec!["typed audit rejection or fail-closed ordinary wait"],
            expected_events_or_reports: vec![
                "actor_known_only=false comes from source_class, not display text",
                "display text need not contain legacy banned substrings",
            ],
            acceptance_assertions: vec![
                "hidden-truth audit keys on typed provenance class",
                "substring scans are not authoritative audit evidence",
            ],
        },
    }
}
