use crate::fixtures::*;

pub fn stale_workplace_notice_superseded_by_newer_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("stale_workplace_notice_superseded_by_newer_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "workshop_tomas")],
        places: vec![
            place_schema("home_tomas", "Tomas home", &["workshop_tomas"]),
            place_schema("workshop_tomas", "Tomas workshop", &["home_tomas"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("work_block", "workplace_tomas")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 100),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
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
    fixture.workplaces[0].role_notice_access_open = false;
    #[expect(
        clippy::disallowed_methods,
        reason = "legacy fixture blanket food-source seeding is pinned by fixtures_load census; new fixtures must author per-actor known_food_sources edges"
    )]
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "stale_workplace_notice_superseded_by_newer_001",
            purpose: "Prove embodied workplace knowledge uses the latest current-place role-notice witness.",
            setup: vec![
                "seeded role-assignment notice says workplace_tomas access is closed",
                "test appends a newer modeled role-assignment notice for the same workplace",
            ],
            allowed_actions: vec![
                "latest notice determines actor-known workplace access",
                "stale workplace notice is not re-surfaced as current embodied knowledge",
            ],
            expected_events_or_reports: vec![
                "newer role notice source event is carried by the workplace fact",
                "older role notice is not double-surfaced in the current holder-known context",
            ],
            acceptance_assertions: vec![
                "workplace freshness follows the shared current-place rule",
                "workplace availability provenance remains reviewable through source event ids",
            ],
        },
    }
}
