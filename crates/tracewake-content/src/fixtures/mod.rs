mod container_item_move_001;
mod debug_attach_001;
mod door_access_001;
mod no_human_advance_001;
mod replay_item_location_001;
mod strongbox_001;
mod view_model_local_actions_001;

use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, DoorId, FixtureId, ItemId, PlaceId, SchemaVersion,
};
use tracewake_core::location::Location;

use crate::load::SourceFile;
use crate::schema::{
    ActionAffordanceSchema, ActorSchema, ContainerSchema, DoorSchema, FixtureSchema, ItemSchema,
    PlaceSchema,
};
use crate::serialization::serialize_fixture;

pub use container_item_move_001::container_item_move_001;
pub use debug_attach_001::debug_attach_001;
pub use door_access_001::door_access_001;
pub use no_human_advance_001::no_human_advance_001;
pub use replay_item_location_001::replay_item_location_001;
pub use strongbox_001::strongbox_001;
pub use view_model_local_actions_001::view_model_local_actions_001;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenFixture {
    pub fixture: FixtureSchema,
    pub contract: FixtureContract,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureContract {
    pub fixture_id: &'static str,
    pub purpose: &'static str,
    pub setup: Vec<&'static str>,
    pub allowed_actions: Vec<&'static str>,
    pub expected_events_or_reports: Vec<&'static str>,
    pub acceptance_assertions: Vec<&'static str>,
}

impl GoldenFixture {
    pub fn source_file(&self) -> SourceFile {
        SourceFile {
            path: format!("{}.twf", self.fixture.fixture_id.as_str()),
            bytes: serialize_fixture(&self.fixture),
        }
    }
}

pub fn all() -> Vec<GoldenFixture> {
    vec![
        strongbox_001(),
        container_item_move_001(),
        door_access_001(),
        debug_attach_001(),
        no_human_advance_001(),
        replay_item_location_001(),
        view_model_local_actions_001(),
    ]
}

fn fixture_id(value: &str) -> FixtureId {
    FixtureId::new(value).unwrap()
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("schema_v1").unwrap()
}

fn actor(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn place(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn door(value: &str) -> DoorId {
    DoorId::new(value).unwrap()
}

fn container(value: &str) -> ContainerId {
    ContainerId::new(value).unwrap()
}

fn item(value: &str) -> ItemId {
    ItemId::new(value).unwrap()
}

fn action(value: &str) -> ActionId {
    ActionId::new(value).unwrap()
}

fn actor_schema(actor_id: &str, current_place_id: &str) -> ActorSchema {
    ActorSchema {
        actor_id: actor(actor_id),
        current_place_id: place(current_place_id),
    }
}

fn place_schema(place_id: &str, display_label: &str, adjacent_place_ids: &[&str]) -> PlaceSchema {
    PlaceSchema {
        place_id: place(place_id),
        display_label: display_label.to_string(),
        adjacent_place_ids: adjacent_place_ids.iter().map(|id| place(id)).collect(),
    }
}

fn door_schema(
    door_id: &str,
    endpoint_a: &str,
    endpoint_b: &str,
    is_open: bool,
    is_locked: bool,
) -> DoorSchema {
    DoorSchema {
        door_id: door(door_id),
        endpoint_a: place(endpoint_a),
        endpoint_b: place(endpoint_b),
        is_open,
        is_locked,
    }
}

fn container_schema(
    container_id: &str,
    place_id: &str,
    is_open: bool,
    is_locked: bool,
    contents: &[&str],
    contents_visible_when_closed: bool,
) -> ContainerSchema {
    ContainerSchema {
        container_id: container(container_id),
        place_id: place(place_id),
        is_open,
        is_locked,
        contents: contents.iter().map(|id| item(id)).collect(),
        contents_visible_when_closed,
    }
}

fn item_in_container(item_id: &str, container_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::InContainer(container(container_id)),
    }
}

fn item_at_place(item_id: &str, place_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::AtPlace(place(place_id)),
    }
}

fn item_carried_by(item_id: &str, actor_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::CarriedBy(actor(actor_id)),
    }
}

fn affordance(action_id: &str, target_id: &str) -> ActionAffordanceSchema {
    ActionAffordanceSchema {
        action_id: action(action_id),
        target_id: target_id.to_string(),
    }
}
