use crate::fixtures::*;

pub fn sleep_interrupted_by_severe_need_prorates_recovery_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("sleep_interrupted_by_severe_need_prorates_recovery_001"),
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
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Fatigue, 900),
            initial_need("actor_tomas", NeedKind::Hunger, 900),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: vec![sleep_place_schema("actor_tomas", "home_tomas", "bed_tomas")],
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
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
            fixture_id: "sleep_interrupted_by_severe_need_prorates_recovery_001",
            purpose: "Adversarial fixture: severe need pressure interrupts scheduled sleep instead of granting an unconditional full recovery.",
            setup: vec!["actor has an authored sleep affordance and severe hunger pressure"],
            allowed_actions: vec!["sleep may start, but completion must revalidate current needs"],
            expected_events_or_reports: vec![
                "SleepInterrupted",
                "NeedDeltaApplied with prorated fatigue recovery",
            ],
            acceptance_assertions: vec![
                "scheduled sleep completion is continuity checked",
                "interrupted sleep emits typed events and prorated deltas",
            ],
        },
    }
}
