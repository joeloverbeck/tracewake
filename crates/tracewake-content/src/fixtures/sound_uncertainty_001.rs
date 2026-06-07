use crate::fixtures::*;

pub fn sound_uncertainty_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("sound_uncertainty_001"),
        schema_version: schema_version(),
        actors: vec![
            actor_schema("actor_elena", "street_lane"),
            actor_schema("actor_mara", "house_tomas"),
            actor_schema("actor_tomas", "house_tomas"),
        ],
        places: vec![
            place_schema("house_tomas", "Tomas house", &["street_lane"]),
            place_schema("street_lane", "Street lane", &["house_tomas"]),
        ],
        doors: vec![door_schema(
            "door_house_street",
            "house_tomas",
            "street_lane",
            true,
            false,
        )],
        containers: vec![container_schema(
            "strongbox_tomas",
            "house_tomas",
            false,
            false,
            &["coin_stack_01"],
            false,
        )],
        items: vec![item_in_container("coin_stack_01", "strongbox_tomas", true)],
        affordances: vec![
            affordance("check_container", "strongbox_tomas"),
            affordance("inspect_place", "street_lane"),
            affordance("inspect_place", "house_tomas"),
        ],
        initial_beliefs: vec![
            tomas_coin_expectation_seed(),
            sound_lead_seed(
                "belief_elena_heard_sound_near_house_tomas",
                "actor_elena",
                "house_tomas",
                "prehistory_elena_heard_uncertain_sound",
            ),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "sound_uncertainty_001",
            purpose: "Seed a low-confidence sound lead without turning it into objective truth.",
            setup: vec![
                "actor_elena has a private plausible sound lead near house_tomas",
                "actor_tomas has the ordinary strongbox expectation",
                "the physical world carries no culprit or scripted sound outcome",
            ],
            allowed_actions: vec![
                "inspect local places",
                "check strongbox_tomas",
                "render actor-scoped notebooks",
            ],
            expected_events_or_reports: vec![
                "low-confidence sound belief appears only for actor_elena",
                "runtime sound observation generation remains outside this fixture",
            ],
            acceptance_assertions: vec![
                "sound uncertainty is represented as a plausible private belief",
                "no npc_knows_truth, culprit, or branch field exists",
            ],
        },
    }
}
