use crate::fixtures::*;

pub fn work_block_failed_then_sleep_succeeds_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("work_block_failed_then_sleep_succeeds_001"),
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
        affordances: vec![
            affordance("move", "street"),
            affordance("work_block", "workplace_shop"),
            affordance("sleep", "street"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 100),
        ],
        homes: vec![home_schema("actor_tomas", "street")],
        sleep_places: vec![sleep_place_schema(
            "actor_tomas",
            "street",
            "sleep_place_street",
        )],
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_shop",
            "workshop",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 8)],
    };
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "work_block_failed_then_sleep_succeeds_001",
            purpose: "Adversarial fixture: a failed work duration closes its body-exclusive reservation before later sleep.",
            setup: vec![
                "actor starts at a workplace",
                "actor can be displaced to a modeled sleep place before work completion",
            ],
            allowed_actions: vec![
                "work_block starts a body-exclusive duration",
                "move displaces the actor before completion",
                "sleep can start after WorkBlockFailed closes the reservation",
            ],
            expected_events_or_reports: vec![
                "WorkBlockFailed caused by the original WorkBlockStarted event",
                "SleepStarted after the failure without ReservationConflict",
                "byte-identical log serialization roundtrip",
            ],
            acceptance_assertions: vec![
                "WorkBlockFailed is a duration-terminal event",
                "failed work reservations do not remain forever open",
            ],
        },
    }
}
