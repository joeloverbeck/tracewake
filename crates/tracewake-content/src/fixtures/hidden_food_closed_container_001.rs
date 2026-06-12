use crate::fixtures::*;

pub fn hidden_food_closed_container_001() -> GoldenFixture {
    with_actor_mara_known_hidden_food(hidden_truth_adversarial_fixture(
        "hidden_food_closed_container_001",
        "Prove food inside a closed opaque container at the actor's place is not actor-known planner input.",
        vec![
            "actor_mara starts hungry at home_mara",
            "food_hidden_pantry is physically present in a closed opaque hidden_pantry",
            "actor_mara has an authored known_food_sources edge for food_hidden_pantry",
            "closed opaque hidden_pantry blocks direct local visibility of the food",
        ],
        vec![
            "usable food targets exclude closed-container physical truth",
            "selected proposal cannot target food_hidden_pantry without provenance",
        ],
    ))
}
