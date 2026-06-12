use crate::fixtures::*;

pub fn no_human_unseen_workplace_assignment_does_not_plan_work_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_unseen_workplace_assignment_does_not_plan_work_001"),
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
        affordances: vec![
            affordance("move", "workshop_tomas"),
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_tomas", NeedKind::Fatigue, 260)],
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
    #[expect(
        clippy::disallowed_methods,
        reason = "legacy fixture blanket food-source seeding is pinned by fixtures_load census; new fixtures must author per-actor known_food_sources edges"
    )]
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_unseen_workplace_assignment_does_not_plan_work_001",
            purpose: "Prove raw workplace assignment alone is not no-human cognition.",
            setup: vec![
                "workplace_tomas assigns actor_tomas in raw state",
                "actor_tomas has no modeled routine assignment notice",
            ],
            allowed_actions: vec!["wait or record no progress without selecting work"],
            expected_events_or_reports: vec![
                "raw workplace assignment is absent from actor-known no-human surface",
                "no work proposal is selected without modeled notice provenance",
            ],
            acceptance_assertions: vec![
                "state.workplaces assignment does not create actor-known workplace facts",
                "no-human planning remains actor-known-only",
            ],
        },
    }
}
