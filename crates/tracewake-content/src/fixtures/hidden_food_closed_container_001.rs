use crate::fixtures::*;

pub fn hidden_food_closed_container_001() -> GoldenFixture {
    hidden_truth_adversarial_fixture(
        "hidden_food_closed_container_001",
        "Prove food inside a closed opaque container at the actor's place is not actor-known planner input.",
        vec![
            "actor_mara starts hungry at home_mara",
            "food_hidden_pantry is physically present in a closed opaque hidden_pantry",
            "no observation or belief reveals the hidden food",
        ],
        vec![
            "known food sources exclude closed-container physical truth",
            "selected proposal cannot target food_hidden_pantry without provenance",
        ],
    )
}
