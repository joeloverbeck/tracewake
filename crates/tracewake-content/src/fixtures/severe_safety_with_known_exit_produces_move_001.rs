use crate::fixtures::*;

pub fn severe_safety_with_known_exit_produces_move_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("severe_safety_with_known_exit_produces_move_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "unsafe_room")],
        places: vec![
            place_schema("unsafe_room", "Unsafe room", &["safety_corridor"]),
            place_schema("safety_corridor", "Safety corridor", &["unsafe_room"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("move", "safety_corridor")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Safety, 950),
            initial_need("actor_mara", NeedKind::Hunger, 100),
            initial_need("actor_mara", NeedKind::Fatigue, 100),
        ],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_mara", 0, 4)],
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
            fixture_id: "severe_safety_with_known_exit_produces_move_001",
            purpose: "Prove severe safety pressure produces autonomous movement along an actor-known exit.",
            setup: vec![
                "actor_mara starts in unsafe_room with severe safety pressure",
                "unsafe_room has one visible authored edge to safety_corridor",
            ],
            allowed_actions: vec!["autonomous move to safety_corridor"],
            expected_events_or_reports: vec![
                "autonomous_no_human_event=ActorMoved",
                "decision_trace_family=leave_unsafe_place",
            ],
            acceptance_assertions: vec![
                "severe safety selects movement before lower-priority pressure",
                "movement target comes from actor-known local edges",
                "live and replay checksums match",
            ],
        },
    }
}
