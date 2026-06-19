use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::ids::{ActorId, ContainerId, IdError, ItemId, PlaceId};
use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Proposition {
    ItemLocatedInContainer {
        item_id: ItemId,
        container_id: ContainerId,
    },
    ItemLocatedAtPlace {
        item_id: ItemId,
        place_id: PlaceId,
    },
    ItemCarriedByActor {
        item_id: ItemId,
        actor_id: ActorId,
    },
    ContainerContentsObserved {
        container_id: ContainerId,
    },
    ItemMissingFromExpectedLocation {
        item_id: ItemId,
        expected_location: Location,
    },
    SoundHeardNearPlace {
        place_id: PlaceId,
    },
    PossibleMovementNearPlace {
        place_id: PlaceId,
    },
    ActorWasNearPlace {
        actor_id: ActorId,
        place_id: PlaceId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropositionReferenceError {
    UnknownActor(ActorId),
    UnknownContainer(ContainerId),
    UnknownItem(ItemId),
    UnknownPlace(PlaceId),
}

impl fmt::Display for PropositionReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropositionReferenceError::UnknownActor(actor_id) => {
                write!(f, "unknown actor referenced by proposition: {actor_id}")
            }
            PropositionReferenceError::UnknownContainer(container_id) => {
                write!(
                    f,
                    "unknown container referenced by proposition: {container_id}"
                )
            }
            PropositionReferenceError::UnknownItem(item_id) => {
                write!(f, "unknown item referenced by proposition: {item_id}")
            }
            PropositionReferenceError::UnknownPlace(place_id) => {
                write!(f, "unknown place referenced by proposition: {place_id}")
            }
        }
    }
}

impl std::error::Error for PropositionReferenceError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropositionParseError {
    InvalidUtf8,
    InvalidShape,
    InvalidLocationShape,
    InvalidId(IdError),
}

impl fmt::Display for PropositionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropositionParseError::InvalidUtf8 => {
                write!(f, "canonical proposition bytes must be UTF-8")
            }
            PropositionParseError::InvalidShape => write!(f, "invalid canonical proposition shape"),
            PropositionParseError::InvalidLocationShape => {
                write!(f, "invalid canonical proposition location shape")
            }
            PropositionParseError::InvalidId(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for PropositionParseError {}

impl From<IdError> for PropositionParseError {
    fn from(value: IdError) -> Self {
        Self::InvalidId(value)
    }
}

impl Proposition {
    pub fn serialize_canonical(&self) -> String {
        match self {
            Proposition::ItemLocatedInContainer {
                item_id,
                container_id,
            } => format!(
                "item_located_in_container|{}|{}",
                item_id.serialize_canonical(),
                container_id.serialize_canonical()
            ),
            Proposition::ItemLocatedAtPlace { item_id, place_id } => format!(
                "item_located_at_place|{}|{}",
                item_id.serialize_canonical(),
                place_id.serialize_canonical()
            ),
            Proposition::ItemCarriedByActor { item_id, actor_id } => format!(
                "item_carried_by_actor|{}|{}",
                item_id.serialize_canonical(),
                actor_id.serialize_canonical()
            ),
            Proposition::ContainerContentsObserved { container_id } => format!(
                "container_contents_observed|{}",
                container_id.serialize_canonical()
            ),
            Proposition::ItemMissingFromExpectedLocation {
                item_id,
                expected_location,
            } => format!(
                "item_missing_from_expected_location|{}|{}",
                item_id.serialize_canonical(),
                serialize_location(expected_location)
            ),
            Proposition::SoundHeardNearPlace { place_id } => {
                format!("sound_heard_near_place|{}", place_id.serialize_canonical())
            }
            Proposition::PossibleMovementNearPlace { place_id } => format!(
                "possible_movement_near_place|{}",
                place_id.serialize_canonical()
            ),
            Proposition::ActorWasNearPlace { actor_id, place_id } => format!(
                "actor_was_near_place|{}|{}",
                actor_id.serialize_canonical(),
                place_id.serialize_canonical()
            ),
        }
    }

    pub fn serialize_canonical_bytes(&self) -> Vec<u8> {
        self.serialize_canonical().into_bytes()
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, PropositionParseError> {
        let value = std::str::from_utf8(value).map_err(|_| PropositionParseError::InvalidUtf8)?;
        Self::from_str(value)
    }

    pub fn contradicts(&self, other: &Self) -> bool {
        self.contradicts_one_way(other) || other.contradicts_one_way(self)
    }

    pub fn render(&self) -> String {
        match self {
            Proposition::ItemLocatedInContainer {
                item_id,
                container_id,
            } => format!("{item_id} is in {container_id}"),
            Proposition::ItemLocatedAtPlace { item_id, place_id } => {
                format!("{item_id} is at {place_id}")
            }
            Proposition::ItemCarriedByActor { item_id, actor_id } => {
                format!("{item_id} is carried by {actor_id}")
            }
            Proposition::ContainerContentsObserved { container_id } => {
                format!("contents of {container_id} were observed")
            }
            Proposition::ItemMissingFromExpectedLocation {
                item_id,
                expected_location,
            } => format!(
                "{item_id} is missing from expected location {}",
                render_location(expected_location)
            ),
            Proposition::SoundHeardNearPlace { place_id } => {
                format!("sound was heard near {place_id}")
            }
            Proposition::PossibleMovementNearPlace { place_id } => {
                format!("movement may have occurred near {place_id}")
            }
            Proposition::ActorWasNearPlace { actor_id, place_id } => {
                format!("{actor_id} was near {place_id}")
            }
        }
    }

    pub fn validate_references(
        &self,
        known_actors: &BTreeSet<ActorId>,
        known_places: &BTreeSet<PlaceId>,
        known_containers: &BTreeSet<ContainerId>,
        known_items: &BTreeSet<ItemId>,
    ) -> Result<(), PropositionReferenceError> {
        match self {
            Proposition::ItemLocatedInContainer {
                item_id,
                container_id,
            } => {
                require_item(item_id, known_items)?;
                require_container(container_id, known_containers)
            }
            Proposition::ItemLocatedAtPlace { item_id, place_id } => {
                require_item(item_id, known_items)?;
                require_place(place_id, known_places)
            }
            Proposition::ItemCarriedByActor { item_id, actor_id } => {
                require_item(item_id, known_items)?;
                require_actor(actor_id, known_actors)
            }
            Proposition::ContainerContentsObserved { container_id } => {
                require_container(container_id, known_containers)
            }
            Proposition::ItemMissingFromExpectedLocation {
                item_id,
                expected_location,
            } => {
                require_item(item_id, known_items)?;
                validate_location(
                    expected_location,
                    known_actors,
                    known_places,
                    known_containers,
                )
            }
            Proposition::SoundHeardNearPlace { place_id }
            | Proposition::PossibleMovementNearPlace { place_id } => {
                require_place(place_id, known_places)
            }
            Proposition::ActorWasNearPlace { actor_id, place_id } => {
                require_actor(actor_id, known_actors)?;
                require_place(place_id, known_places)
            }
        }
    }

    fn contradicts_one_way(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Proposition::ItemLocatedInContainer {
                    item_id,
                    container_id,
                },
                Proposition::ItemMissingFromExpectedLocation {
                    item_id: missing_item_id,
                    expected_location: Location::InContainer(expected_container_id),
                },
            ) => item_id == missing_item_id && container_id == expected_container_id,
            (
                Proposition::ItemLocatedAtPlace { item_id, place_id },
                Proposition::ItemMissingFromExpectedLocation {
                    item_id: missing_item_id,
                    expected_location: Location::AtPlace(expected_place_id),
                },
            ) => item_id == missing_item_id && place_id == expected_place_id,
            (
                Proposition::ItemCarriedByActor { item_id, actor_id },
                Proposition::ItemMissingFromExpectedLocation {
                    item_id: missing_item_id,
                    expected_location: Location::CarriedBy(expected_actor_id),
                },
            ) => item_id == missing_item_id && actor_id == expected_actor_id,
            _ => false,
        }
    }
}

impl fmt::Display for Proposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render())
    }
}

impl FromStr for Proposition {
    type Err = PropositionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = value.split('|').collect();
        let proposition = match parts.as_slice() {
            ["item_located_in_container", item_id, container_id] => {
                Proposition::ItemLocatedInContainer {
                    item_id: ItemId::new(*item_id)?,
                    container_id: ContainerId::new(*container_id)?,
                }
            }
            ["item_located_at_place", item_id, place_id] => Proposition::ItemLocatedAtPlace {
                item_id: ItemId::new(*item_id)?,
                place_id: PlaceId::new(*place_id)?,
            },
            ["item_carried_by_actor", item_id, actor_id] => Proposition::ItemCarriedByActor {
                item_id: ItemId::new(*item_id)?,
                actor_id: ActorId::new(*actor_id)?,
            },
            ["container_contents_observed", container_id] => {
                Proposition::ContainerContentsObserved {
                    container_id: ContainerId::new(*container_id)?,
                }
            }
            ["item_missing_from_expected_location", item_id, expected_location] => {
                Proposition::ItemMissingFromExpectedLocation {
                    item_id: ItemId::new(*item_id)?,
                    expected_location: deserialize_location(expected_location)?,
                }
            }
            ["sound_heard_near_place", place_id] => Proposition::SoundHeardNearPlace {
                place_id: PlaceId::new(*place_id)?,
            },
            ["possible_movement_near_place", place_id] => Proposition::PossibleMovementNearPlace {
                place_id: PlaceId::new(*place_id)?,
            },
            ["actor_was_near_place", actor_id, place_id] => Proposition::ActorWasNearPlace {
                actor_id: ActorId::new(*actor_id)?,
                place_id: PlaceId::new(*place_id)?,
            },
            _ => return Err(PropositionParseError::InvalidShape),
        };

        Ok(proposition)
    }
}

fn serialize_location(location: &Location) -> String {
    match location {
        Location::AtPlace(place_id) => format!("at_place:{}", place_id.serialize_canonical()),
        Location::InContainer(container_id) => {
            format!("in_container:{}", container_id.serialize_canonical())
        }
        Location::CarriedBy(actor_id) => format!("carried_by:{}", actor_id.serialize_canonical()),
    }
}

fn deserialize_location(value: &str) -> Result<Location, PropositionParseError> {
    let Some((kind, id)) = value.split_once(':') else {
        return Err(PropositionParseError::InvalidLocationShape);
    };

    match kind {
        "at_place" => Ok(Location::AtPlace(PlaceId::new(id)?)),
        "in_container" => Ok(Location::InContainer(ContainerId::new(id)?)),
        "carried_by" => Ok(Location::CarriedBy(ActorId::new(id)?)),
        _ => Err(PropositionParseError::InvalidLocationShape),
    }
}

fn render_location(location: &Location) -> String {
    match location {
        Location::AtPlace(place_id) => format!("place {place_id}"),
        Location::InContainer(container_id) => format!("container {container_id}"),
        Location::CarriedBy(actor_id) => format!("actor {actor_id}"),
    }
}

fn validate_location(
    location: &Location,
    known_actors: &BTreeSet<ActorId>,
    known_places: &BTreeSet<PlaceId>,
    known_containers: &BTreeSet<ContainerId>,
) -> Result<(), PropositionReferenceError> {
    match location {
        Location::AtPlace(place_id) => require_place(place_id, known_places),
        Location::InContainer(container_id) => require_container(container_id, known_containers),
        Location::CarriedBy(actor_id) => require_actor(actor_id, known_actors),
    }
}

fn require_actor(
    actor_id: &ActorId,
    known_actors: &BTreeSet<ActorId>,
) -> Result<(), PropositionReferenceError> {
    if known_actors.contains(actor_id) {
        Ok(())
    } else {
        Err(PropositionReferenceError::UnknownActor(actor_id.clone()))
    }
}

fn require_place(
    place_id: &PlaceId,
    known_places: &BTreeSet<PlaceId>,
) -> Result<(), PropositionReferenceError> {
    if known_places.contains(place_id) {
        Ok(())
    } else {
        Err(PropositionReferenceError::UnknownPlace(place_id.clone()))
    }
}

fn require_container(
    container_id: &ContainerId,
    known_containers: &BTreeSet<ContainerId>,
) -> Result<(), PropositionReferenceError> {
    if known_containers.contains(container_id) {
        Ok(())
    } else {
        Err(PropositionReferenceError::UnknownContainer(
            container_id.clone(),
        ))
    }
}

fn require_item(
    item_id: &ItemId,
    known_items: &BTreeSet<ItemId>,
) -> Result<(), PropositionReferenceError> {
    if known_items.contains(item_id) {
        Ok(())
    } else {
        Err(PropositionReferenceError::UnknownItem(item_id.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epistemics::{
        Belief, Channel, Confidence, Contradiction, ContradictionKind, EpistemicProjection,
        HolderKind, SourceRef, Stance,
    };
    use crate::ids::{BeliefId, ContentManifestId, ContradictionId, EventId, ObservationId};

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn belief_id(value: &str) -> BeliefId {
        BeliefId::new(value).unwrap()
    }

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
    }

    fn contradiction_id(value: &str) -> ContradictionId {
        ContradictionId::new(value).unwrap()
    }

    fn observation_id(value: &str) -> ObservationId {
        ObservationId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn typed_relation_contradiction(
        contradiction: &str,
        expected: Proposition,
        observed: Proposition,
    ) -> Option<Contradiction> {
        if !expected.contradicts(&observed) || !observed.contradicts(&expected) {
            return None;
        }

        Some(Contradiction::new(
            contradiction_id(contradiction),
            actor_id("actor_tomas"),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            belief_id("belief_tomas_expected_location"),
            observation_id("obs_tomas_checked_location"),
            expected,
            observed,
            crate::time::SimTick::new(9),
        ))
    }

    fn assert_replayable_typed_relation_contradiction(
        contradiction: Contradiction,
        expected_summary: &str,
    ) {
        let expected_canonical = contradiction.expected_proposition().serialize_canonical();
        let observed_canonical = contradiction.observed_proposition().serialize_canonical();
        let mut projection = EpistemicProjection::new(
            ContentManifestId::new("proposition_relation_matrix").unwrap(),
        );
        projection.insert_contradiction(contradiction);
        let replayed_checksum = projection.compute_checksum().checksum;
        let debug = projection.debug_epistemics_view();

        assert_eq!(debug.contradictions.len(), 1);
        assert_eq!(
            debug.contradictions[0].holder_actor_id,
            actor_id("actor_tomas")
        );
        assert_eq!(
            debug.contradictions[0].expectation_belief_id,
            "belief_tomas_expected_location"
        );
        assert_eq!(
            debug.contradictions[0].observation_id,
            "obs_tomas_checked_location"
        );
        assert_eq!(debug.contradictions[0].summary, expected_summary);
        assert!(projection
            .compute_checksum()
            .canonical_input
            .iter()
            .any(
                |entry| entry.contains(&expected_canonical) && entry.contains(&observed_canonical)
            ));
        assert_eq!(projection.compute_checksum().checksum, replayed_checksum);
    }

    fn at_place_expected(item: &str, place: &str) -> Proposition {
        Proposition::ItemLocatedAtPlace {
            item_id: item_id(item),
            place_id: place_id(place),
        }
    }

    fn carried_by_expected(item: &str, actor: &str) -> Proposition {
        Proposition::ItemCarriedByActor {
            item_id: item_id(item),
            actor_id: actor_id(actor),
        }
    }

    fn missing_from(item: &str, location: Location) -> Proposition {
        Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id(item),
            expected_location: location,
        }
    }

    fn assert_projection_consumes_round_tripped_proposition(
        label: &str,
        proposition: Proposition,
        stance: Stance,
    ) -> Proposition {
        let canonical = proposition.serialize_canonical();
        let parsed = Proposition::deserialize_canonical(canonical.as_bytes()).unwrap();
        assert_eq!(
            parsed, proposition,
            "round-trip typed proposition for {label}"
        );
        assert_eq!(
            parsed.serialize_canonical(),
            canonical,
            "canonical for {label}"
        );

        let mut projection =
            EpistemicProjection::new(ContentManifestId::new("proposition_parser_corpus").unwrap());
        let belief = Belief::new(
            belief_id(&format!("belief_parser_{label}")),
            HolderKind::Actor(actor_id("actor_tomas")),
            parsed.clone(),
            stance,
            Confidence::new(875).unwrap(),
            SourceRef::Event(event_id("event_parser_corpus")),
            crate::time::SimTick::new(11),
        )
        .with_channel(Channel::ReadingPlaceholderSchemaOnly);
        projection.insert_belief(belief);
        let checksum = projection.compute_checksum();
        let debug = projection.debug_epistemics_view();

        assert!(checksum
            .canonical_input
            .iter()
            .any(|entry| entry.contains(&canonical)));
        assert!(debug.beliefs_by_holder.iter().any(|holder| {
            holder.holder_actor_id == actor_id("actor_tomas")
                && holder.beliefs.iter().any(|entry| {
                    entry.belief_id == format!("belief_parser_{label}")
                        && entry.proposition == proposition.render()
                        && entry.stance == stance.stable_id()
                })
        }));

        parsed
    }

    #[test]
    fn proposition_round_trips_through_canonical_bytes() {
        let proposition = Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("coin_stack_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        };
        let bytes = proposition.serialize_canonical_bytes();

        assert_eq!(
            String::from_utf8(bytes.clone()).unwrap(),
            "item_missing_from_expected_location|coin_stack_01|in_container:strongbox_tomas"
        );
        assert_eq!(
            Proposition::deserialize_canonical(&bytes).unwrap(),
            proposition
        );
        assert_eq!(
            Proposition::deserialize_canonical(
                &Proposition::deserialize_canonical(&bytes)
                    .unwrap()
                    .serialize_canonical_bytes()
            )
            .unwrap()
            .serialize_canonical_bytes(),
            bytes
        );
    }

    #[test]
    fn canonical_parser_round_trips_every_variant_through_projection_consumers() {
        let cases = [
            (
                "item_in_container",
                Proposition::ItemLocatedInContainer {
                    item_id: item_id("coin_stack_01"),
                    container_id: container_id("strongbox_tomas"),
                },
                Stance::BelievesTrue,
            ),
            (
                "item_at_place",
                Proposition::ItemLocatedAtPlace {
                    item_id: item_id("coin_stack_01"),
                    place_id: place_id("back_room"),
                },
                Stance::BelievesTrue,
            ),
            (
                "item_carried_by_actor",
                Proposition::ItemCarriedByActor {
                    item_id: item_id("coin_stack_01"),
                    actor_id: actor_id("actor_tomas"),
                },
                Stance::ExpectsTrue,
            ),
            (
                "container_contents_observed",
                Proposition::ContainerContentsObserved {
                    container_id: container_id("strongbox_tomas"),
                },
                Stance::BelievesTrue,
            ),
            (
                "sound_heard_near_place",
                Proposition::SoundHeardNearPlace {
                    place_id: place_id("back_room"),
                },
                Stance::Plausible,
            ),
            (
                "possible_movement_near_place",
                Proposition::PossibleMovementNearPlace {
                    place_id: place_id("back_room"),
                },
                Stance::Plausible,
            ),
            (
                "actor_was_near_place",
                Proposition::ActorWasNearPlace {
                    actor_id: actor_id("actor_tomas"),
                    place_id: place_id("back_room"),
                },
                Stance::BelievesTrue,
            ),
        ];

        for (label, proposition, stance) in cases {
            assert_projection_consumes_round_tripped_proposition(label, proposition, stance);
        }
    }

    #[test]
    fn expected_location_parser_round_trips_into_typed_contradiction_linkage() {
        let cases = [
            (
                "at_place",
                at_place_expected("coin_stack_01", "back_room"),
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                "coin_stack_01 is at back_room -> coin_stack_01 is missing from expected location place back_room",
            ),
            (
                "in_container",
                Proposition::ItemLocatedInContainer {
                    item_id: item_id("coin_stack_01"),
                    container_id: container_id("strongbox_tomas"),
                },
                missing_from(
                    "coin_stack_01",
                    Location::InContainer(container_id("strongbox_tomas")),
                ),
                "coin_stack_01 is in strongbox_tomas -> coin_stack_01 is missing from expected location container strongbox_tomas",
            ),
            (
                "carried_by",
                carried_by_expected("coin_stack_01", "actor_tomas"),
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                "coin_stack_01 is carried by actor_tomas -> coin_stack_01 is missing from expected location actor actor_tomas",
            ),
        ];

        for (label, expected, observed, summary) in cases {
            let parsed_observed = assert_projection_consumes_round_tripped_proposition(
                label,
                observed,
                Stance::BelievesTrue,
            );
            let contradiction = typed_relation_contradiction(
                &format!("contradiction_location_parser_{label}"),
                expected,
                parsed_observed,
            )
            .expect("round-tripped location should create typed contradiction");
            assert_replayable_typed_relation_contradiction(contradiction, summary);
        }
    }

    #[test]
    fn malformed_canonical_propositions_fail_closed_without_defaults() {
        let invalid_shape = [
            b"unknown_tag|coin_stack_01".as_slice(),
            b"item_carried_by_actor|coin_stack_01".as_slice(),
            b"container_contents_observed|strongbox_tomas|extra".as_slice(),
            b"possible_movement_near_place".as_slice(),
            b"coin_stack_01 is carried by actor_tomas".as_slice(),
        ];
        for canonical in invalid_shape {
            assert_eq!(
                Proposition::deserialize_canonical(canonical),
                Err(PropositionParseError::InvalidShape)
            );
        }

        assert_eq!(
            Proposition::deserialize_canonical(
                b"item_missing_from_expected_location|coin_stack_01|back_room"
            ),
            Err(PropositionParseError::InvalidLocationShape)
        );
        assert_eq!(
            Proposition::deserialize_canonical(
                b"item_missing_from_expected_location|coin_stack_01|near_place:back_room"
            ),
            Err(PropositionParseError::InvalidLocationShape)
        );
        assert_eq!(
            Proposition::deserialize_canonical(b"item_carried_by_actor|coin_stack_01|actor:tomas"),
            Err(PropositionParseError::InvalidId(
                IdError::InvalidCharacter { found: ':' }
            ))
        );
    }

    #[test]
    fn display_formats_all_variants_as_structured_explanations() {
        let cases = [
            (
                Proposition::ItemLocatedInContainer {
                    item_id: item_id("coin_stack_01"),
                    container_id: container_id("strongbox_tomas"),
                },
                "coin_stack_01 is in strongbox_tomas",
            ),
            (
                Proposition::ItemLocatedAtPlace {
                    item_id: item_id("coin_stack_01"),
                    place_id: place_id("back_room"),
                },
                "coin_stack_01 is at back_room",
            ),
            (
                Proposition::ItemCarriedByActor {
                    item_id: item_id("coin_stack_01"),
                    actor_id: actor_id("actor_tomas"),
                },
                "coin_stack_01 is carried by actor_tomas",
            ),
            (
                Proposition::ContainerContentsObserved {
                    container_id: container_id("strongbox_tomas"),
                },
                "contents of strongbox_tomas were observed",
            ),
            (
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                "coin_stack_01 is missing from expected location place back_room",
            ),
            (
                missing_from(
                    "coin_stack_01",
                    Location::InContainer(container_id("strongbox_tomas")),
                ),
                "coin_stack_01 is missing from expected location container strongbox_tomas",
            ),
            (
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                "coin_stack_01 is missing from expected location actor actor_tomas",
            ),
            (
                Proposition::SoundHeardNearPlace {
                    place_id: place_id("back_room"),
                },
                "sound was heard near back_room",
            ),
            (
                Proposition::PossibleMovementNearPlace {
                    place_id: place_id("back_room"),
                },
                "movement may have occurred near back_room",
            ),
            (
                Proposition::ActorWasNearPlace {
                    actor_id: actor_id("actor_tomas"),
                    place_id: place_id("back_room"),
                },
                "actor_tomas was near back_room",
            ),
        ];

        for (proposition, rendered) in cases {
            assert_eq!(format!("{proposition}"), rendered);
            assert_eq!(proposition.render(), rendered);
            assert_eq!(
                Proposition::deserialize_canonical(proposition.serialize_canonical().as_bytes())
                    .unwrap(),
                proposition
            );
        }
    }

    #[test]
    fn contradiction_detects_missing_expected_location() {
        let located = Proposition::ItemLocatedInContainer {
            item_id: item_id("coin_stack_01"),
            container_id: container_id("strongbox_tomas"),
        };
        let missing = Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("coin_stack_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        };
        let unrelated = Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("silver_ring_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        };

        assert!(located.contradicts(&missing));
        assert!(missing.contradicts(&located));
        assert!(!located.contradicts(&unrelated));
    }

    #[test]
    fn at_place_missing_relation_creates_only_exact_typed_contradiction() {
        let rows = [
            (
                "item_and_place_match",
                at_place_expected("coin_stack_01", "back_room"),
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                true,
            ),
            (
                "item_differs",
                at_place_expected("silver_ring_01", "back_room"),
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                false,
            ),
            (
                "place_differs",
                at_place_expected("coin_stack_01", "shop_front"),
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                false,
            ),
            (
                "both_differ",
                at_place_expected("silver_ring_01", "shop_front"),
                missing_from("coin_stack_01", Location::AtPlace(place_id("back_room"))),
                false,
            ),
        ];

        for (label, expected, observed, should_contradict) in rows {
            let detection = typed_relation_contradiction(
                &format!("contradiction_at_place_{label}"),
                expected,
                observed,
            );
            assert_eq!(
                detection.is_some(),
                should_contradict,
                "at-place row {label}"
            );
            if let Some(contradiction) = detection {
                assert_replayable_typed_relation_contradiction(
                    contradiction,
                    "coin_stack_01 is at back_room -> coin_stack_01 is missing from expected location place back_room",
                );
            }
        }
    }

    #[test]
    fn carried_by_missing_relation_creates_only_exact_holder_contradiction() {
        let rows = [
            (
                "item_and_actor_match",
                carried_by_expected("coin_stack_01", "actor_tomas"),
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                true,
            ),
            (
                "item_differs",
                carried_by_expected("silver_ring_01", "actor_tomas"),
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                false,
            ),
            (
                "actor_differs",
                carried_by_expected("coin_stack_01", "actor_mara"),
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                false,
            ),
            (
                "both_differ",
                carried_by_expected("silver_ring_01", "actor_mara"),
                missing_from(
                    "coin_stack_01",
                    Location::CarriedBy(actor_id("actor_tomas")),
                ),
                false,
            ),
        ];

        for (label, expected, observed, should_contradict) in rows {
            let detection = typed_relation_contradiction(
                &format!("contradiction_carried_by_{label}"),
                expected,
                observed,
            );
            assert_eq!(
                detection.is_some(),
                should_contradict,
                "carried-by row {label}"
            );
            if let Some(contradiction) = detection {
                assert_eq!(contradiction.holder_actor_id(), &actor_id("actor_tomas"));
                assert_replayable_typed_relation_contradiction(
                    contradiction,
                    "coin_stack_01 is carried by actor_tomas -> coin_stack_01 is missing from expected location actor actor_tomas",
                );
            }
        }
    }

    #[test]
    fn render_is_derived_from_structured_fields() {
        let proposition = Proposition::SoundHeardNearPlace {
            place_id: place_id("tomas_room"),
        };

        assert_eq!(proposition.render(), "sound was heard near tomas_room");
    }

    #[test]
    fn reference_validation_rejects_unknown_ids() {
        let known_actors = BTreeSet::from([actor_id("actor_tomas")]);
        let known_places = BTreeSet::from([place_id("tomas_room")]);
        let known_containers = BTreeSet::from([container_id("strongbox_tomas")]);
        let known_items = BTreeSet::from([item_id("coin_stack_01")]);

        let valid = Proposition::ItemLocatedInContainer {
            item_id: item_id("coin_stack_01"),
            container_id: container_id("strongbox_tomas"),
        };
        assert!(valid
            .validate_references(
                &known_actors,
                &known_places,
                &known_containers,
                &known_items
            )
            .is_ok());

        let invalid = Proposition::ActorWasNearPlace {
            actor_id: actor_id("actor_mara"),
            place_id: place_id("tomas_room"),
        };
        assert_eq!(
            invalid.validate_references(
                &known_actors,
                &known_places,
                &known_containers,
                &known_items
            ),
            Err(PropositionReferenceError::UnknownActor(actor_id(
                "actor_mara"
            )))
        );
    }

    #[test]
    fn actor_was_near_place_is_structured_but_not_special_cased_as_knowledge() {
        let proposition = Proposition::ActorWasNearPlace {
            actor_id: actor_id("actor_mara"),
            place_id: place_id("tomas_room"),
        };

        assert_eq!(
            proposition.serialize_canonical(),
            "actor_was_near_place|actor_mara|tomas_room"
        );
    }
}
