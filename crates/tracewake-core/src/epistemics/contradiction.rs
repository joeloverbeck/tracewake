use std::collections::BTreeSet;

use crate::epistemics::belief::{Belief, HolderKind, Stance};
use crate::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use crate::epistemics::observation::{Channel, Confidence, SourceRef};
use crate::epistemics::proposition::Proposition;
use crate::ids::{
    ActorId, BeliefId, ContainerId, ContradictionId, EventId, ItemId, ObservationId, SchemaVersion,
};
use crate::location::Location;
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
    contradiction_id: ContradictionId,
    holder_actor_id: ActorId,
    kind: ContradictionKind,
    prior_expectation_belief_id: BeliefId,
    contradicting_observation_id: ObservationId,
    expected_proposition: Proposition,
    observed_proposition: Proposition,
    detected_tick: SimTick,
    schema_version: SchemaVersion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpectedAbsenceDetection {
    pub contradiction: Contradiction,
    pub missing_belief: Belief,
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

    pub fn contradiction_id(&self) -> &ContradictionId {
        &self.contradiction_id
    }

    pub fn holder_actor_id(&self) -> &ActorId {
        &self.holder_actor_id
    }

    pub fn kind(&self) -> ContradictionKind {
        self.kind
    }

    pub fn prior_expectation_belief_id(&self) -> &BeliefId {
        &self.prior_expectation_belief_id
    }

    pub fn contradicting_observation_id(&self) -> &ObservationId {
        &self.contradicting_observation_id
    }

    pub fn expected_proposition(&self) -> &Proposition {
        &self.expected_proposition
    }

    pub fn observed_proposition(&self) -> &Proposition {
        &self.observed_proposition
    }

    pub fn detected_tick(&self) -> SimTick {
        self.detected_tick
    }

    pub fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }
}

#[allow(clippy::too_many_arguments)]
pub fn detect_expected_absences(
    holder_actor_id: &ActorId,
    checked_container_id: &ContainerId,
    observed_item_ids: &BTreeSet<ItemId>,
    expectation_beliefs: &[&Belief],
    observation_id: &ObservationId,
    source_event_id: &EventId,
    detected_tick: SimTick,
    confidence: Confidence,
) -> Vec<ExpectedAbsenceDetection> {
    let mut detections = Vec::new();
    for belief in expectation_beliefs {
        if belief.holder() != &HolderKind::Actor(holder_actor_id.clone())
            || belief.stance() != Stance::ExpectsTrue
        {
            continue;
        }
        let Proposition::ItemLocatedInContainer {
            item_id,
            container_id,
        } = belief.proposition()
        else {
            continue;
        };
        if container_id != checked_container_id || observed_item_ids.contains(item_id) {
            continue;
        }

        let observed_proposition = Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id.clone(),
            expected_location: Location::InContainer(checked_container_id.clone()),
        };
        let contradiction_id = ContradictionId::new(format!(
            "contradiction.{}.{}",
            observation_id.as_str(),
            item_id.as_str()
        ))
        .unwrap();
        let missing_belief_id = BeliefId::new(format!(
            "belief.missing.{}.{}",
            observation_id.as_str(),
            item_id.as_str()
        ))
        .unwrap();
        let contradiction = Contradiction::new(
            contradiction_id.clone(),
            holder_actor_id.clone(),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            belief.belief_id().clone(),
            observation_id.clone(),
            belief.proposition().clone(),
            observed_proposition.clone(),
            detected_tick,
        );
        let missing_belief = Belief::new(
            missing_belief_id,
            HolderKind::Actor(holder_actor_id.clone()),
            observed_proposition,
            Stance::BelievesTrue,
            confidence,
            SourceRef::Event(source_event_id.clone()),
            detected_tick,
        )
        .with_channel(Channel::AbsenceMarker)
        .with_observation(observation_id.clone())
        .with_contradiction(contradiction_id);
        detections.push(ExpectedAbsenceDetection {
            contradiction,
            missing_belief,
        });
    }
    detections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epistemics::observation::Confidence;
    use crate::ids::{ContainerId, ItemId};

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

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
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
            contradiction.kind(),
            ContradictionKind::ExpectedItemAbsentFromContainer
        );
        assert_eq!(
            contradiction.prior_expectation_belief_id(),
            &belief_id("belief_tomas_expected_coin")
        );
        assert_eq!(
            contradiction.contradicting_observation_id(),
            &observation_id("obs_tomas_checked_strongbox")
        );
        assert_eq!(
            contradiction.schema_version().as_str(),
            EPISTEMIC_RECORD_SCHEMA_V1
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

    fn expectation_belief(
        belief_id_value: &str,
        actor: &str,
        item: &str,
        container: &str,
    ) -> Belief {
        Belief::new(
            belief_id(belief_id_value),
            HolderKind::Actor(actor_id(actor)),
            Proposition::ItemLocatedInContainer {
                item_id: item_id(item),
                container_id: container_id(container),
            },
            Stance::ExpectsTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_seed")),
            SimTick::ZERO,
        )
    }

    #[test]
    fn detection_derives_missing_belief_for_absent_expected_item() {
        let belief = expectation_belief(
            "belief_tomas_expected_coin",
            "actor_tomas",
            "coin_stack_01",
            "strongbox_tomas",
        );
        let detections = detect_expected_absences(
            &actor_id("actor_tomas"),
            &container_id("strongbox_tomas"),
            &BTreeSet::new(),
            &[&belief],
            &observation_id("obs_tomas_checked_strongbox"),
            &event_id("event_observation"),
            SimTick::new(4),
            Confidence::new(950).unwrap(),
        );

        assert_eq!(detections.len(), 1);
        assert_eq!(
            detections[0].contradiction.prior_expectation_belief_id(),
            &belief_id("belief_tomas_expected_coin")
        );
        assert_eq!(
            detections[0].missing_belief.holder(),
            &HolderKind::Actor(actor_id("actor_tomas"))
        );
        assert_eq!(detections[0].missing_belief.stance(), Stance::BelievesTrue);
        assert!(matches!(
            detections[0].missing_belief.proposition(),
            Proposition::ItemMissingFromExpectedLocation { .. }
        ));
    }

    #[test]
    fn detection_requires_relevant_expectation_and_absence() {
        let other_container = expectation_belief(
            "belief_other_place",
            "actor_tomas",
            "coin_stack_01",
            "other_box",
        );
        let present_item = expectation_belief(
            "belief_present_item",
            "actor_tomas",
            "coin_stack_02",
            "strongbox_tomas",
        );
        let no_expectations = detect_expected_absences(
            &actor_id("actor_tomas"),
            &container_id("strongbox_tomas"),
            &BTreeSet::new(),
            &[],
            &observation_id("obs_tomas_checked_strongbox"),
            &event_id("event_observation"),
            SimTick::new(4),
            Confidence::new(950).unwrap(),
        );
        let different_location = detect_expected_absences(
            &actor_id("actor_tomas"),
            &container_id("strongbox_tomas"),
            &BTreeSet::new(),
            &[&other_container],
            &observation_id("obs_tomas_checked_strongbox"),
            &event_id("event_observation"),
            SimTick::new(4),
            Confidence::new(950).unwrap(),
        );
        let item_present = detect_expected_absences(
            &actor_id("actor_tomas"),
            &container_id("strongbox_tomas"),
            &BTreeSet::from([item_id("coin_stack_02")]),
            &[&present_item],
            &observation_id("obs_tomas_checked_strongbox"),
            &event_id("event_observation"),
            SimTick::new(4),
            Confidence::new(950).unwrap(),
        );

        assert!(no_expectations.is_empty());
        assert!(different_location.is_empty());
        assert!(item_present.is_empty());
    }
}
