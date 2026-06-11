use crate::fixtures::*;

pub fn wait_then_window_passive_charges_each_tick_once_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("wait_then_window_passive_charges_each_tick_once_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 100),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![
            day_window_schema("actor_tomas", 0, 1),
            day_window_schema("actor_tomas", 4, 5),
        ],
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
            fixture_id: "wait_then_window_passive_charges_each_tick_once_001",
            purpose: "Prove an autonomous wait tick is not charged again by the next no-human passive window.",
            setup: vec![
                "actor_tomas has no urgent need, food, workplace, or sleep surface",
                "the first no-human window produces an actor-known wait ending at tick 1",
                "the second no-human window starts at tick 4",
            ],
            allowed_actions: vec!["transaction-authored wait", "passive need accounting"],
            expected_events_or_reports: vec![
                "ActorWaited at tick 1",
                "second window passive NeedDeltaApplied elapsed_ticks=3",
                "no duplicate need-regime charges",
            ],
            acceptance_assertions: vec![
                "wait-charged tick 1 is not recharged by the second window",
                "passive accounting resumes at ticks 2, 3, and 4",
            ],
        },
    }
}
