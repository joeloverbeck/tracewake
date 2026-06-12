use crate::fixtures::*;

pub fn aged_food_record_surfaces_as_remembered_belief_not_observation_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id(
            "aged_food_record_surfaces_as_remembered_belief_not_observation_001",
        ),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("eat", "food_stew_home_tomas")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 650),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_stew_home_tomas",
            "home_tomas",
            2,
            240,
        )],
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 4, 12)],
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
            fixture_id: "aged_food_record_surfaces_as_remembered_belief_not_observation_001",
            purpose:
                "Prove aged projected food knowledge is remembered belief rather than observed_now.",
            setup: vec![
                "actor_tomas observes food_stew_home_tomas at home_tomas at tick 4",
                "the no-human actor-known surface is built later at tick 9",
                "food remains actor-known through modeled projection ancestry",
            ],
            allowed_actions: vec![
                "record current-place perception",
                "build no-human actor-known surface",
            ],
            expected_events_or_reports: vec![
                "ObservationRecorded at tick 4",
                "actor_knows_food_source remembered_belief tick=4",
            ],
            acceptance_assertions: vec![
                "aged food knowledge remains available",
                "aged food knowledge is not stamped observed_now at the decision tick",
            ],
        },
    }
}
