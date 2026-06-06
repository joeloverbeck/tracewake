use tracewake_core::events::log::{EventLog, EventLogError};
use tracewake_core::ids::{
    ActorId, ContainerId, DoorId, FixtureId, ItemId, PlaceId, SchemaVersion,
};
use tracewake_core::location::Location;

use crate::schema::{
    ActorSchema, ContainerSchema, DoorSchema, FixtureSchema, ItemSchema, PlaceSchema,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SerializationError {
    MissingField(&'static str),
    BadLine(String),
    BadBool(String),
    Id(tracewake_core::ids::IdError),
    EventLog(EventLogError),
}

impl From<tracewake_core::ids::IdError> for SerializationError {
    fn from(value: tracewake_core::ids::IdError) -> Self {
        Self::Id(value)
    }
}

pub fn serialize_fixture(fixture: &FixtureSchema) -> Vec<u8> {
    let mut fixture = fixture.clone();
    fixture.canonicalize();
    let mut lines = vec![
        format!("fixture|{}", fixture.fixture_id.as_str()),
        format!("schema|{}", fixture.schema_version.as_str()),
    ];
    for actor in fixture.actors {
        lines.push(format!(
            "actor|{}|{}",
            actor.actor_id.as_str(),
            actor.current_place_id.as_str()
        ));
    }
    for place in fixture.places {
        lines.push(format!(
            "place|{}|{}|{}",
            place.place_id.as_str(),
            encode(&place.display_label),
            join(place.adjacent_place_ids.iter().map(|id| id.as_str()))
        ));
    }
    for door in fixture.doors {
        lines.push(format!(
            "door|{}|{}|{}|{}|{}",
            door.door_id.as_str(),
            door.endpoint_a.as_str(),
            door.endpoint_b.as_str(),
            door.is_open,
            door.is_locked
        ));
    }
    for container in fixture.containers {
        lines.push(format!(
            "container|{}|{}|{}|{}|{}|{}",
            container.container_id.as_str(),
            container.place_id.as_str(),
            container.is_open,
            container.is_locked,
            container.contents_visible_when_closed,
            join(container.contents.iter().map(|id| id.as_str()))
        ));
    }
    for item in fixture.items {
        lines.push(format!(
            "item|{}|{}|{}",
            item.item_id.as_str(),
            item.portable,
            serialize_location(&item.location)
        ));
    }
    lines.join("\n").into_bytes()
}

pub fn deserialize_fixture(bytes: &[u8]) -> Result<FixtureSchema, SerializationError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| SerializationError::BadLine("non-utf8 fixture".to_string()))?;
    let mut fixture_id = None;
    let mut schema_version = None;
    let mut actors = Vec::new();
    let mut places = Vec::new();
    let mut doors = Vec::new();
    let mut containers = Vec::new();
    let mut items = Vec::new();

    for line in text.lines() {
        let parts = line.split('|').collect::<Vec<_>>();
        match parts.as_slice() {
            ["fixture", id] => fixture_id = Some(FixtureId::new(*id)?),
            ["schema", version] => schema_version = Some(SchemaVersion::new(*version)?),
            ["actor", actor_id, place_id] => actors.push(ActorSchema {
                actor_id: ActorId::new(*actor_id)?,
                current_place_id: PlaceId::new(*place_id)?,
            }),
            ["place", place_id, label, adjacent] => places.push(PlaceSchema {
                place_id: PlaceId::new(*place_id)?,
                display_label: decode(label)?,
                adjacent_place_ids: split_ids(adjacent, |part| PlaceId::new(part))?,
            }),
            ["door", door_id, endpoint_a, endpoint_b, is_open, is_locked] => {
                doors.push(DoorSchema {
                    door_id: DoorId::new(*door_id)?,
                    endpoint_a: PlaceId::new(*endpoint_a)?,
                    endpoint_b: PlaceId::new(*endpoint_b)?,
                    is_open: parse_bool(is_open)?,
                    is_locked: parse_bool(is_locked)?,
                })
            }
            ["container", container_id, place_id, is_open, is_locked, contents_visible_when_closed, contents] => {
                containers.push(ContainerSchema {
                    container_id: ContainerId::new(*container_id)?,
                    place_id: PlaceId::new(*place_id)?,
                    is_open: parse_bool(is_open)?,
                    is_locked: parse_bool(is_locked)?,
                    contents_visible_when_closed: parse_bool(contents_visible_when_closed)?,
                    contents: split_ids(contents, |part| ItemId::new(part))?,
                })
            }
            ["item", item_id, portable, location] => items.push(ItemSchema {
                item_id: ItemId::new(*item_id)?,
                portable: parse_bool(portable)?,
                location: deserialize_location(location)?,
            }),
            _ => return Err(SerializationError::BadLine(line.to_string())),
        }
    }

    let mut fixture = FixtureSchema {
        fixture_id: fixture_id.ok_or(SerializationError::MissingField("fixture"))?,
        schema_version: schema_version.ok_or(SerializationError::MissingField("schema"))?,
        actors,
        places,
        doors,
        containers,
        items,
    };
    fixture.canonicalize();
    Ok(fixture)
}

pub fn serialize_event_log(log: &EventLog) -> Vec<u8> {
    log.serialize_canonical()
}

pub fn deserialize_event_log(bytes: &[u8]) -> Result<EventLog, SerializationError> {
    EventLog::deserialize_canonical(bytes).map_err(SerializationError::EventLog)
}

fn serialize_location(location: &Location) -> String {
    match location {
        Location::AtPlace(id) => format!("at:{}", id.as_str()),
        Location::InContainer(id) => format!("in:{}", id.as_str()),
        Location::CarriedBy(id) => format!("carried:{}", id.as_str()),
    }
}

fn deserialize_location(value: &str) -> Result<Location, SerializationError> {
    let (kind, id) = value
        .split_once(':')
        .ok_or_else(|| SerializationError::BadLine(value.to_string()))?;
    match kind {
        "at" => Ok(Location::AtPlace(PlaceId::new(id)?)),
        "in" => Ok(Location::InContainer(ContainerId::new(id)?)),
        "carried" => Ok(Location::CarriedBy(ActorId::new(id)?)),
        _ => Err(SerializationError::BadLine(value.to_string())),
    }
}

fn split_ids<T>(
    value: &str,
    parse: impl Fn(&str) -> Result<T, tracewake_core::ids::IdError>,
) -> Result<Vec<T>, SerializationError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value
            .split(',')
            .map(|part| parse(part).map_err(Into::into))
            .collect()
    }
}

fn join<'a>(values: impl Iterator<Item = &'a str>) -> String {
    values.collect::<Vec<_>>().join(",")
}

fn parse_bool(value: &str) -> Result<bool, SerializationError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(SerializationError::BadBool(value.to_string())),
    }
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode(value: &str) -> Result<String, SerializationError> {
    if !value.len().is_multiple_of(2) {
        return Err(SerializationError::BadLine(value.to_string()));
    }
    let mut bytes = Vec::new();
    for chunk in value.as_bytes().chunks_exact(2) {
        let hex = std::str::from_utf8(chunk)
            .map_err(|_| SerializationError::BadLine(value.to_string()))?;
        bytes.push(
            u8::from_str_radix(hex, 16)
                .map_err(|_| SerializationError::BadLine(value.to_string()))?,
        );
    }
    String::from_utf8(bytes).map_err(|_| SerializationError::BadLine(value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::events::log::EventLog;
    use tracewake_core::events::{EventEnvelope, EventKind};
    use tracewake_core::ids::{ActionId, ContentManifestId, EventId};
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::time::SimTick;

    fn fixture() -> FixtureSchema {
        FixtureSchema {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            actors: vec![ActorSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                current_place_id: PlaceId::new("shop_front").unwrap(),
            }],
            places: vec![PlaceSchema {
                place_id: PlaceId::new("shop_front").unwrap(),
                display_label: "Shop front".to_string(),
                adjacent_place_ids: Vec::new(),
            }],
            doors: Vec::new(),
            containers: vec![ContainerSchema {
                container_id: ContainerId::new("strongbox_tomas").unwrap(),
                place_id: PlaceId::new("shop_front").unwrap(),
                is_open: false,
                is_locked: false,
                contents: vec![ItemId::new("coin_stack_01").unwrap()],
                contents_visible_when_closed: false,
            }],
            items: vec![ItemSchema {
                item_id: ItemId::new("coin_stack_01").unwrap(),
                portable: true,
                location: Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
            }],
        }
    }

    #[test]
    fn fixture_serialization_round_trips_byte_identically() {
        let first = serialize_fixture(&fixture());
        let parsed = deserialize_fixture(&first).unwrap();
        let second = serialize_fixture(&parsed);

        assert_eq!(first, second);
    }

    #[test]
    fn event_log_serialization_round_trips_byte_identically() {
        let mut log = EventLog::new();
        let event = EventEnvelope::new_v1(
            EventId::new("event_0001").unwrap(),
            EventKind::ActorWaited,
            0,
            0,
            SimTick::ZERO,
            OrderingKey::new(
                SimTick::ZERO,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(ActorId::new("actor_tomas").unwrap()),
                ProposalSequence::new(0),
                ActionId::new("wait").unwrap(),
                vec!["1_tick".to_string()],
                "tie",
            ),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        log.append(event).unwrap();

        let first = serialize_event_log(&log);
        let second = serialize_event_log(&deserialize_event_log(&first).unwrap());

        assert_eq!(first, second);
    }
}
