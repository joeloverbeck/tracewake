use crate::fixtures::*;

pub fn no_human_advance_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_advance_001"),
        schema_version: schema_version(),
        actors: vec![actor_schema("actor_ren", "quiet_room")],
        places: vec![place_schema("quiet_room", "Quiet room", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: vec![item_carried_by("notebook_01", "actor_ren", true)],
        affordances: vec![
            affordance("inspect_place", "quiet_room"),
            affordance("inspect_entity", "notebook_01"),
        ],
        initial_beliefs: Vec::new(),
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "no_human_advance_001",
            purpose: "Prove the simulation can advance with no sacred player state.",
            setup: vec![
                "actor_ren is an ordinary actor",
                "no controller binding is required",
                "scheduler/time proof is deterministic and may be no-op apart from process markers",
            ],
            allowed_actions: vec![
                "load fixture",
                "advance fixed tick count with no controller binding",
                "replay deterministic events/reports",
            ],
            expected_events_or_reports: vec![
                "NoHumanAdvanceStarted or equivalent deterministic marker",
                "completion/report marker",
                "matching replay checksum/report",
            ],
            acceptance_assertions: vec![
                "no PlayerCharacter, player objective, or player-bound scheduler priority exists",
                "time advances only through deterministic tick/event machinery",
                "ordinary actions, if scheduled later, use the shared action pipeline",
            ],
        },
    }
}
