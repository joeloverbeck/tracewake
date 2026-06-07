use std::collections::BTreeSet;

use crate::events::{EventCause, PayloadField};
use crate::ids::{ActionId, ActorId, ContainerId, EventId, ItemId, PlaceId, SchemaVersion};
use crate::time::SimTick;

pub const EPISTEMIC_RECORD_SCHEMA_V1: &str = "epistemic_record_schema_v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Confidence(u16);

impl Confidence {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(1000);

    pub fn new(parts_per_thousand: u16) -> Result<Self, ConfidenceError> {
        if parts_per_thousand <= Self::MAX.0 {
            Ok(Self(parts_per_thousand))
        } else {
            Err(ConfidenceError::OutOfRange { parts_per_thousand })
        }
    }

    pub const fn parts_per_thousand(self) -> u16 {
        self.0
    }

    pub const fn is_low(self) -> bool {
        self.0 <= 350
    }

    pub fn serialize_canonical(self) -> String {
        format!("{:04}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfidenceError {
    OutOfRange { parts_per_thousand: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    DirectSight,
    TouchOrSearch,
    SimpleSound,
    AbsenceMarker,
    ReadingPlaceholderSchemaOnly,
}

impl Channel {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Channel::DirectSight => "direct_sight",
            Channel::TouchOrSearch => "touch_or_search",
            Channel::SimpleSound => "simple_sound",
            Channel::AbsenceMarker => "absence_marker",
            Channel::ReadingPlaceholderSchemaOnly => "reading_placeholder_schema_only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TickWindow {
    pub start_tick: SimTick,
    pub end_tick: SimTick,
}

impl TickWindow {
    pub const fn at(tick: SimTick) -> Self {
        Self {
            start_tick: tick,
            end_tick: tick,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObservationSubject {
    Actor(ActorId),
    Container(ContainerId),
    Item(ItemId),
    Place(PlaceId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObservationTarget {
    Container(ContainerId),
    Item(ItemId),
    Place(PlaceId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceRef {
    Event(EventId),
    Action(ActionId),
    Cause(EventCause),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrivacyScope {
    ActorPrivate(ActorId),
    PublicPlaceholder,
    InstitutionPlaceholder(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Observation {
    pub observation_id: crate::ids::ObservationId,
    pub observer_actor_id: ActorId,
    pub channel: Channel,
    pub observed_tick: SimTick,
    pub tick_window: TickWindow,
    pub observer_place_id: PlaceId,
    pub subject: ObservationSubject,
    pub target: ObservationTarget,
    pub raw_payload: Vec<PayloadField>,
    pub confidence: Confidence,
    pub source: SourceRef,
    pub alternatives: BTreeSet<String>,
    pub schema_version: SchemaVersion,
    pub privacy_scope: PrivacyScope,
}

impl Observation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        observation_id: crate::ids::ObservationId,
        observer_actor_id: ActorId,
        channel: Channel,
        observed_tick: SimTick,
        observer_place_id: PlaceId,
        subject: ObservationSubject,
        target: ObservationTarget,
        confidence: Confidence,
        source: SourceRef,
    ) -> Self {
        Self {
            observation_id,
            observer_actor_id: observer_actor_id.clone(),
            channel,
            observed_tick,
            tick_window: TickWindow::at(observed_tick),
            observer_place_id,
            subject,
            target,
            raw_payload: Vec::new(),
            confidence,
            source,
            alternatives: BTreeSet::new(),
            schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
            privacy_scope: PrivacyScope::ActorPrivate(observer_actor_id),
        }
    }

    pub fn with_raw_payload(mut self, raw_payload: Vec<PayloadField>) -> Self {
        self.raw_payload = raw_payload;
        self
    }

    pub fn with_alternatives(mut self, alternatives: BTreeSet<String>) -> Self {
        self.alternatives = alternatives;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ObservationId;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
    }

    fn observation_id(value: &str) -> ObservationId {
        ObservationId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    #[test]
    fn channel_set_has_stable_ids() {
        assert_eq!(Channel::DirectSight.stable_id(), "direct_sight");
        assert_eq!(Channel::TouchOrSearch.stable_id(), "touch_or_search");
        assert_eq!(Channel::SimpleSound.stable_id(), "simple_sound");
        assert_eq!(Channel::AbsenceMarker.stable_id(), "absence_marker");
        assert_eq!(
            Channel::ReadingPlaceholderSchemaOnly.stable_id(),
            "reading_placeholder_schema_only"
        );
    }

    #[test]
    fn sound_observation_carries_low_confidence_and_alternatives() {
        let observation = Observation::new(
            observation_id("obs_sound_001"),
            actor_id("actor_elena"),
            Channel::SimpleSound,
            SimTick::new(4),
            place_id("hallway"),
            ObservationSubject::Place(place_id("tomas_room")),
            ObservationTarget::Place(place_id("tomas_room")),
            Confidence::new(250).unwrap(),
            SourceRef::Event(event_id("event_sound_001")),
        )
        .with_alternatives(BTreeSet::from([
            "object_shift".to_string(),
            "footstep".to_string(),
        ]));

        assert_eq!(observation.channel, Channel::SimpleSound);
        assert!(observation.confidence.is_low());
        assert_eq!(observation.confidence.serialize_canonical(), "0250");
        assert_eq!(observation.alternatives.len(), 2);
        assert_eq!(
            observation.privacy_scope,
            PrivacyScope::ActorPrivate(actor_id("actor_elena"))
        );
    }

    #[test]
    fn observation_is_not_a_belief() {
        let observation = Observation::new(
            observation_id("obs_001"),
            actor_id("actor_tomas"),
            Channel::DirectSight,
            SimTick::ZERO,
            place_id("tomas_room"),
            ObservationSubject::Place(place_id("tomas_room")),
            ObservationTarget::Place(place_id("tomas_room")),
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_001")),
        );

        assert_eq!(observation.channel, Channel::DirectSight);
        assert!(observation.alternatives.is_empty());
    }
}
