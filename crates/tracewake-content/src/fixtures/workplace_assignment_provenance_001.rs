use crate::fixtures::*;

pub fn workplace_assignment_provenance_001() -> GoldenFixture {
    hidden_truth_adversarial_fixture(
        "workplace_assignment_provenance_001",
        "Prove workplace planning requires assignment or observation provenance, not physical workplace presence alone.",
        vec![
            "workplace_hidden exists in PhysicalState",
            "actor_mara is assigned only when a modeled routine-assignment fact is supplied",
            "planner assertions inspect provenance on the workplace fact",
        ],
        vec![
            "physical workplace presence alone is insufficient",
            "routine-assignment provenance is accepted when explicitly modeled",
        ],
    )
}
