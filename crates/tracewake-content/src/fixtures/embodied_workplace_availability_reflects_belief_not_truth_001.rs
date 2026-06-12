use crate::fixtures::*;

pub fn embodied_workplace_availability_reflects_belief_not_truth_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_workplace_availability_reflects_belief_not_truth_001"),
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
            fixture_id: "embodied_workplace_availability_reflects_belief_not_truth_001",
            purpose: "Prove embodied workplace availability follows holder-known workplace belief rather than open workplace truth.",
            setup: vec![
                "truth contains an open assigned workplace for actor_tomas",
                "role-assignment notice says workplace_tomas access is closed",
            ],
            allowed_actions: vec!["embodied view shows work_block disabled from holder-known closed-access belief"],
            expected_events_or_reports: vec![
                "truth_open_workplace_disabled_by_believed_closed_access",
            ],
            acceptance_assertions: vec![
                "truth-open workplace state is insufficient to enable embodied availability",
                "workplace_tomas appears disabled from the actor-known closed-access fact",
            ],
        },
    }
}
