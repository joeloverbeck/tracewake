use tracewake_core::epistemics::{Contradiction, ContradictionKind, Proposition};
use tracewake_core::ids::{
    ActorId, BeliefId, ContainerId, ContradictionId, ItemId, ObservationId,
};
use tracewake_core::time::SimTick;

pub fn mutate_contradiction_links() {
    let expected = Proposition::ItemLocatedInContainer {
        item_id: ItemId::new("coin_stack_01").unwrap(),
        container_id: ContainerId::new("strongbox_tomas").unwrap(),
    };
    let observed = Proposition::ItemMissingFromExpectedLocation {
        item_id: ItemId::new("coin_stack_01").unwrap(),
        expected_location: tracewake_core::location::Location::InContainer(
            ContainerId::new("strongbox_tomas").unwrap(),
        ),
    };
    let mut contradiction = Contradiction::new(
        ContradictionId::new("contradiction_tomas_coin").unwrap(),
        ActorId::new("actor_tomas").unwrap(),
        ContradictionKind::ExpectedItemAbsentFromContainer,
        BeliefId::new("belief_tomas_expected_coin").unwrap(),
        ObservationId::new("obs_tomas_check").unwrap(),
        expected,
        observed,
        SimTick::ZERO,
    );
    contradiction.prior_expectation_belief_id =
        BeliefId::new("belief_forged_expectation").unwrap();
}
