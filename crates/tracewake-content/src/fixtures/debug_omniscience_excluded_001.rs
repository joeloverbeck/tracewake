use crate::fixtures::*;

pub fn debug_omniscience_excluded_001() -> GoldenFixture {
    hidden_truth_adversarial_fixture(
        "debug_omniscience_excluded_001",
        "Prove debug-only omniscience is excluded from actor-known planning context.",
        vec![
            "hidden physical facts exist in the fixture",
            "no authored known_food_sources edge is supplied for debug-only hidden facts",
            "debug views may inspect them out of band",
            "planner context must contain no DebugOmniscience provenance",
        ],
        vec![
            "actor-known proof sources exclude debug_omniscience",
            "debug-only facts never make hidden targets applicable",
        ],
    )
}
