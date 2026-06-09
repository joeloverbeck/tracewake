use crate::fixtures::*;

pub fn work_completion_fails_when_actor_displaced_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("work_completion_fails_when_actor_displaced_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "workshop")],
        places: vec![
            place_schema("workshop", "Workshop", &["street"]),
            place_schema("street", "Street", &["workshop"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("work_block", "workplace_shop")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 100),
        ],
        homes: vec![home_schema("actor_tomas", "street")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_shop",
            "workshop",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "work_completion_fails_when_actor_displaced_001",
            purpose: "Adversarial fixture: actor displacement prevents an open-loop scheduled work completion.",
            setup: vec!["actor starts at a modeled workplace with an adjacent displacement target"],
            allowed_actions: vec!["work completion only after continuity revalidation"],
            expected_events_or_reports: vec![
                "WorkBlockFailed with actor_displaced reason",
                "no WorkBlockCompleted when continuity breaks",
            ],
            acceptance_assertions: vec![
                "scheduled work completion checks current actor place",
                "broken continuity emits typed failure and prorated deltas",
            ],
        },
    }
}
