use crate::fixtures::*;

pub fn hidden_route_edge_001() -> GoldenFixture {
    with_actor_mara_known_hidden_food(hidden_truth_adversarial_fixture(
        "hidden_route_edge_001",
        "Prove a physical route edge absent from the actor-known local view cannot drive movement planning.",
        vec![
            "home_mara is physically adjacent to hidden_workshop",
            "the actor-known route view intentionally omits that edge",
            "planner route proof must come from actor-known edges",
        ],
        vec![
            "known_edges excludes hidden_workshop when not observed",
            "movement proposals cannot target hidden_workshop from physical adjacency alone",
        ],
    ))
}
