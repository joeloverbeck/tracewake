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

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
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
