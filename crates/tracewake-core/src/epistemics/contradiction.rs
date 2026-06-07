use crate::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use crate::epistemics::proposition::Proposition;
use crate::ids::{ActorId, BeliefId, ContradictionId, ObservationId, SchemaVersion};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContradictionKind {
    ExpectedItemAbsentFromContainer,
}

impl ContradictionKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            ContradictionKind::ExpectedItemAbsentFromContainer => {
                "expected_item_absent_from_container"
            }
        }
    }

    pub const fn is_active(self) -> bool {
        matches!(self, ContradictionKind::ExpectedItemAbsentFromContainer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contradiction {
    pub contradiction_id: ContradictionId,
    pub holder_actor_id: ActorId,
    pub kind: ContradictionKind,
    pub prior_expectation_belief_id: BeliefId,
    pub contradicting_observation_id: ObservationId,
    pub expected_proposition: Proposition,
    pub observed_proposition: Proposition,
    pub detected_tick: SimTick,
    pub schema_version: SchemaVersion,
}

impl Contradiction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contradiction_id: ContradictionId,
        holder_actor_id: ActorId,
        kind: ContradictionKind,
        prior_expectation_belief_id: BeliefId,
        contradicting_observation_id: ObservationId,
        expected_proposition: Proposition,
        observed_proposition: Proposition,
        detected_tick: SimTick,
    ) -> Self {
        Self {
            contradiction_id,
            holder_actor_id,
            kind,
            prior_expectation_belief_id,
            contradicting_observation_id,
            expected_proposition,
            observed_proposition,
            detected_tick,
            schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{ContainerId, ItemId};
    use crate::location::Location;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn belief_id(value: &str) -> BeliefId {
        BeliefId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn contradiction_id(value: &str) -> ContradictionId {
        ContradictionId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn observation_id(value: &str) -> ObservationId {
        ObservationId::new(value).unwrap()
    }

    #[test]
    fn contradiction_links_expectation_belief_and_observation() {
        let expected = Proposition::ItemLocatedInContainer {
            item_id: item_id("coin_stack_01"),
            container_id: container_id("strongbox_tomas"),
        };
        let observed = Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("coin_stack_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        };
        let contradiction = Contradiction::new(
            contradiction_id("contradiction_tomas_missing_coin"),
            actor_id("actor_tomas"),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            belief_id("belief_tomas_expected_coin"),
            observation_id("obs_tomas_checked_strongbox"),
            expected,
            observed,
            SimTick::new(5),
        );

        assert_eq!(
            contradiction.kind,
            ContradictionKind::ExpectedItemAbsentFromContainer
        );
        assert_eq!(
            contradiction.prior_expectation_belief_id,
            belief_id("belief_tomas_expected_coin")
        );
        assert_eq!(
            contradiction.contradicting_observation_id,
            observation_id("obs_tomas_checked_strongbox")
        );
    }

    #[test]
    fn expected_item_absent_from_container_is_only_active_kind() {
        assert_eq!(
            ContradictionKind::ExpectedItemAbsentFromContainer.stable_id(),
            "expected_item_absent_from_container"
        );
        assert!(ContradictionKind::ExpectedItemAbsentFromContainer.is_active());
    }
}
