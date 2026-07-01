use crate::fixtures::*;

pub fn co_present_actor_activity_not_apparent_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("co_present_actor_activity_not_apparent_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "market_square"),
            actor_schema("actor_tomas", "market_square"),
        ],
        places: vec![place_schema("market_square", "Market square", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("inspect_place", "market_square")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 140),
            initial_need("actor_mara", NeedKind::Fatigue, 120),
            initial_need("actor_mara", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 140),
            initial_need("actor_tomas", NeedKind::Fatigue, 120),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![
            day_window_schema("actor_mara", 0, 8),
            day_window_schema("actor_tomas", 0, 8),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "co_present_actor_activity_not_apparent_001",
            purpose: "Provide a co-present actor case where identity is visible but no activity cue is modeled.",
            setup: vec![
                "actor_mara and actor_tomas start in market_square",
                "actor_tomas has no routine, workplace, sleep place, or food activity cue",
            ],
            allowed_actions: vec!["observe co-present actor_tomas from actor_mara"],
            expected_events_or_reports: vec![
                "visible_actor observation can cite actor_tomas as co-present",
                "activity classification remains activity_not_apparent",
            ],
            acceptance_assertions: vec![
                "fixture authors no initial belief about actor_tomas activity",
                "co-presence alone does not infer work, sleep, or eating activity",
                "fixture canonicalizes without changing authored references",
            ],
        },
    }
}
