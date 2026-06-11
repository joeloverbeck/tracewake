use crate::fixtures::*;

pub fn hidden_food_unknown_route_001() -> GoldenFixture {
    hidden_truth_adversarial_fixture(
        "hidden_food_unknown_route_001",
        "Prove food reachable only through an unknown route does not become a planner target.",
        vec![
            "actor_mara starts at home_mara",
            "hidden_workshop contains hidden physical opportunities",
            "no authored known_food_sources edge reveals food_hidden_pantry",
            "the actor-known route view supplied to planning omits hidden_workshop",
        ],
        vec![
            "known route edges exclude hidden_workshop",
            "planner cannot choose hidden food by following physical-only route truth",
        ],
    )
}
