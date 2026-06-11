use crate::fixtures::*;

pub fn severe_safety_without_known_exit_waits_with_knowledge_blocker_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("severe_safety_without_known_exit_waits_with_knowledge_blocker_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "sealed_room")],
        places: vec![place_schema("sealed_room", "Sealed room", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Safety, 950)],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_mara", 0, 4)],
    };
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "severe_safety_without_known_exit_waits_with_knowledge_blocker_001",
            purpose: "Prove severe safety without an actor-known exit fails as an explained knowledge blocker.",
            setup: vec![
                "actor_mara starts in sealed_room with severe safety pressure",
                "sealed_room has no actor-known outgoing edge",
            ],
            allowed_actions: vec!["typed local-planning knowledge blocker"],
            expected_events_or_reports: vec![
                "blocker_category=knowledge",
                "blocker_reason=no_actor_known_exit",
            ],
            acceptance_assertions: vec![
                "no hidden physical adjacency is consulted",
                "the no-exit branch records a typed local-planning knowledge blocker",
            ],
        },
    }
}
