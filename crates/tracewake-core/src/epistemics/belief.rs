use std::collections::BTreeSet;

use crate::epistemics::observation::{
    Channel, Confidence, PrivacyScope, SourceRef, EPISTEMIC_RECORD_SCHEMA_V1,
};
use crate::epistemics::proposition::Proposition;
use crate::ids::{ActorId, BeliefId, ContradictionId, ObservationId, SchemaVersion};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HolderKind {
    Actor(ActorId),
    InstitutionPlaceholder(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stance {
    BelievesTrue,
    BelievesFalse,
    ExpectsTrue,
    Plausible,
    Doubts,
    UnknownOrUnresolved,
}

impl Stance {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Stance::BelievesTrue => "believes_true",
            Stance::BelievesFalse => "believes_false",
            Stance::ExpectsTrue => "expects_true",
            Stance::Plausible => "plausible",
            Stance::Doubts => "doubts",
            Stance::UnknownOrUnresolved => "unknown_or_unresolved",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Belief {
    belief_id: BeliefId,
    holder: HolderKind,
    proposition: Proposition,
    stance: Stance,
    confidence: Confidence,
    source: SourceRef,
    channel: Option<Channel>,
    acquired_tick: SimTick,
    last_verified_tick: Option<SimTick>,
    stale_after_tick: Option<SimTick>,
    observation_ids: BTreeSet<ObservationId>,
    contradiction_ids: BTreeSet<ContradictionId>,
    schema_version: SchemaVersion,
    privacy_scope: PrivacyScope,
}

impl Belief {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        belief_id: BeliefId,
        holder: HolderKind,
        proposition: Proposition,
        stance: Stance,
        confidence: Confidence,
        source: SourceRef,
        acquired_tick: SimTick,
    ) -> Self {
        let privacy_scope = match &holder {
            HolderKind::Actor(actor_id) => PrivacyScope::ActorPrivate(actor_id.clone()),
            HolderKind::InstitutionPlaceholder(institution_id) => {
                PrivacyScope::InstitutionPlaceholder(institution_id.clone())
            }
        };

        Self {
            belief_id,
            holder,
            proposition,
            stance,
            confidence,
            source,
            channel: None,
            acquired_tick,
            last_verified_tick: None,
            stale_after_tick: None,
            observation_ids: BTreeSet::new(),
            contradiction_ids: BTreeSet::new(),
            schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
            privacy_scope,
        }
    }

    pub fn with_channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn with_last_verified_tick(mut self, last_verified_tick: Option<SimTick>) -> Self {
        self.last_verified_tick = last_verified_tick;
        self
    }

    pub fn with_observation(mut self, observation_id: ObservationId) -> Self {
        self.observation_ids.insert(observation_id);
        self
    }

    pub fn with_contradiction(mut self, contradiction_id: ContradictionId) -> Self {
        self.contradiction_ids.insert(contradiction_id);
        self
    }

    pub fn belief_id(&self) -> &BeliefId {
        &self.belief_id
    }

    pub fn holder(&self) -> &HolderKind {
        &self.holder
    }

    pub fn proposition(&self) -> &Proposition {
        &self.proposition
    }

    pub fn stance(&self) -> Stance {
        self.stance
    }

    pub fn confidence(&self) -> Confidence {
        self.confidence
    }

    pub fn source(&self) -> &SourceRef {
        &self.source
    }

    pub fn channel(&self) -> Option<Channel> {
        self.channel
    }

    pub fn acquired_tick(&self) -> SimTick {
        self.acquired_tick
    }

    pub fn last_verified_tick(&self) -> Option<SimTick> {
        self.last_verified_tick
    }

    pub fn stale_after_tick(&self) -> Option<SimTick> {
        self.stale_after_tick
    }

    pub fn observation_ids(&self) -> &BTreeSet<ObservationId> {
        &self.observation_ids
    }

    pub fn contradiction_ids(&self) -> &BTreeSet<ContradictionId> {
        &self.contradiction_ids
    }

    pub fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    pub fn privacy_scope(&self) -> &PrivacyScope {
        &self.privacy_scope
    }
}

#[derive(Clone, Debug, Default)]
pub struct BeliefDraft {
    belief_id: Option<BeliefId>,
    holder: Option<HolderKind>,
    proposition: Option<Proposition>,
    stance: Option<Stance>,
    confidence: Option<Confidence>,
    source: Option<SourceRef>,
    acquired_tick: Option<SimTick>,
}

impl BeliefDraft {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_belief_id(mut self, belief_id: BeliefId) -> Self {
        self.belief_id = Some(belief_id);
        self
    }

    pub fn with_holder(mut self, holder: HolderKind) -> Self {
        self.holder = Some(holder);
        self
    }

    pub fn with_proposition(mut self, proposition: Proposition) -> Self {
        self.proposition = Some(proposition);
        self
    }

    pub fn with_stance(mut self, stance: Stance) -> Self {
        self.stance = Some(stance);
        self
    }

    pub fn with_confidence(mut self, confidence: Confidence) -> Self {
        self.confidence = Some(confidence);
        self
    }

    pub fn with_source(mut self, source: SourceRef) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_acquired_tick(mut self, acquired_tick: SimTick) -> Self {
        self.acquired_tick = Some(acquired_tick);
        self
    }

    pub fn build(self) -> Result<Belief, BeliefBuildError> {
        Ok(Belief::new(
            self.belief_id.ok_or(BeliefBuildError::MissingBeliefId)?,
            self.holder.ok_or(BeliefBuildError::MissingHolder)?,
            self.proposition
                .ok_or(BeliefBuildError::MissingProposition)?,
            self.stance.ok_or(BeliefBuildError::MissingStance)?,
            self.confidence.ok_or(BeliefBuildError::MissingConfidence)?,
            self.source.ok_or(BeliefBuildError::MissingSource)?,
            self.acquired_tick
                .ok_or(BeliefBuildError::MissingAcquiredTick)?,
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeliefBuildError {
    MissingBeliefId,
    MissingHolder,
    MissingProposition,
    MissingStance,
    MissingConfidence,
    MissingSource,
    MissingAcquiredTick,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{ContainerId, EventId, ItemId};
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

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn expectation() -> Proposition {
        Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("coin_stack_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        }
    }

    #[test]
    fn holder_and_source_are_required_by_checked_constructor() {
        let draft = BeliefDraft::new()
            .with_belief_id(belief_id("belief_tomas_missing_coin"))
            .with_proposition(expectation())
            .with_stance(Stance::BelievesTrue)
            .with_confidence(Confidence::new(900).unwrap())
            .with_acquired_tick(SimTick::new(3));

        assert_eq!(draft.build(), Err(BeliefBuildError::MissingHolder));

        let draft = BeliefDraft::new()
            .with_belief_id(belief_id("belief_tomas_missing_coin"))
            .with_holder(HolderKind::Actor(actor_id("actor_tomas")))
            .with_proposition(expectation())
            .with_stance(Stance::BelievesTrue)
            .with_confidence(Confidence::new(900).unwrap())
            .with_acquired_tick(SimTick::new(3));

        assert_eq!(draft.build(), Err(BeliefBuildError::MissingSource));
    }

    #[test]
    fn belief_is_holder_keyed_and_source_backed() {
        let belief = Belief::new(
            belief_id("belief_tomas_missing_coin"),
            HolderKind::Actor(actor_id("actor_tomas")),
            expectation(),
            Stance::BelievesTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_obs_absence")),
            SimTick::new(3),
        )
        .with_channel(Channel::AbsenceMarker);

        assert_eq!(belief.holder(), &HolderKind::Actor(actor_id("actor_tomas")));
        assert_eq!(belief.channel(), Some(Channel::AbsenceMarker));
        assert_eq!(
            belief.privacy_scope(),
            &PrivacyScope::ActorPrivate(actor_id("actor_tomas"))
        );
        assert_eq!(
            belief.source(),
            &SourceRef::Event(event_id("event_obs_absence"))
        );
        assert_eq!(belief.schema_version().as_str(), EPISTEMIC_RECORD_SCHEMA_V1);
        assert_eq!(belief.last_verified_tick(), None);
    }

    #[test]
    fn belief_draft_builder_preserves_required_provenance() {
        let belief = BeliefDraft::new()
            .with_belief_id(belief_id("belief_tomas_missing_coin"))
            .with_holder(HolderKind::Actor(actor_id("actor_tomas")))
            .with_proposition(expectation())
            .with_stance(Stance::BelievesTrue)
            .with_confidence(Confidence::new(900).unwrap())
            .with_source(SourceRef::Event(event_id("event_obs_absence")))
            .with_acquired_tick(SimTick::new(3))
            .build()
            .unwrap();

        assert_eq!(
            belief.source(),
            &SourceRef::Event(event_id("event_obs_absence"))
        );
        assert_eq!(belief.schema_version().as_str(), EPISTEMIC_RECORD_SCHEMA_V1);
        assert_eq!(
            belief.privacy_scope(),
            &PrivacyScope::ActorPrivate(actor_id("actor_tomas"))
        );
    }

    #[test]
    fn stance_set_has_stable_ids() {
        assert_eq!(Stance::BelievesTrue.stable_id(), "believes_true");
        assert_eq!(Stance::BelievesFalse.stable_id(), "believes_false");
        assert_eq!(Stance::ExpectsTrue.stable_id(), "expects_true");
        assert_eq!(Stance::Plausible.stable_id(), "plausible");
        assert_eq!(Stance::Doubts.stable_id(), "doubts");
        assert_eq!(
            Stance::UnknownOrUnresolved.stable_id(),
            "unknown_or_unresolved"
        );
    }

    #[test]
    fn confidence_encoding_is_bounded_and_canonical() {
        assert_eq!(Confidence::new(1000).unwrap().serialize_canonical(), "1000");
        assert_eq!(
            Confidence::new(1001),
            Err(
                crate::epistemics::observation::ConfidenceError::OutOfRange {
                    parts_per_thousand: 1001
                }
            )
        );
    }
}
