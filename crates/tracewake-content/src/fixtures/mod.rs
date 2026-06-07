mod container_item_move_001;
mod debug_attach_001;
mod door_access_001;
mod expectation_contradiction_001;
mod knowledge_blocker_accuse_001;
mod no_human_advance_001;
mod no_human_epistemic_check_001;
mod possession_parity_001;
mod replay_item_location_001;
mod sound_uncertainty_001;
mod strongbox_001;
mod view_filtering_001;
mod view_model_local_actions_001;

use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{
    Channel, Confidence, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, DoorId, EventId, FixtureId, ItemId, PlaceId,
    SchemaVersion,
};
use tracewake_core::location::Location;
use tracewake_core::time::SimTick;

use crate::load::SourceFile;
use crate::schema::{
    ActionAffordanceSchema, ActorSchema, ContainerSchema, DoorSchema, FixtureSchema,
    InitialBeliefSchema, ItemSchema, PlaceSchema,
};
use crate::serialization::serialize_fixture;

pub use container_item_move_001::container_item_move_001;
pub use debug_attach_001::debug_attach_001;
pub use door_access_001::door_access_001;
pub use expectation_contradiction_001::expectation_contradiction_001;
pub use knowledge_blocker_accuse_001::knowledge_blocker_accuse_001;
pub use no_human_advance_001::no_human_advance_001;
pub use no_human_epistemic_check_001::no_human_epistemic_check_001;
pub use possession_parity_001::possession_parity_001;
pub use replay_item_location_001::replay_item_location_001;
pub use sound_uncertainty_001::sound_uncertainty_001;
pub use strongbox_001::strongbox_001;
pub use view_filtering_001::view_filtering_001;
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
        expectation_contradiction_001(),
        possession_parity_001(),
        view_filtering_001(),
        knowledge_blocker_accuse_001(),
        sound_uncertainty_001(),
        no_human_epistemic_check_001(),
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

fn tomas_coin_expectation_seed() -> InitialBeliefSchema {
    expectation_seed(
        "belief_tomas_expects_coin_stack_01_in_strongbox_tomas",
        "actor_tomas",
        "coin_stack_01",
        "strongbox_tomas",
        "prehistory_tomas_checked_strongbox_before_start",
    )
}

fn expectation_seed(
    belief_id: &str,
    holder_actor_id: &str,
    item_id: &str,
    container_id: &str,
    source_id: &str,
) -> InitialBeliefSchema {
    InitialBeliefSchema::new_expectation(
        BeliefId::new(belief_id).unwrap(),
        actor(holder_actor_id),
        Proposition::ItemLocatedInContainer {
            item_id: item(item_id),
            container_id: container(container_id),
        },
        Confidence::new(900).unwrap(),
        SourceRef::Event(EventId::new(source_id).unwrap()),
        SimTick::ZERO,
    )
}

fn sound_lead_seed(
    belief_id: &str,
    holder_actor_id: &str,
    place_id: &str,
    source_id: &str,
) -> InitialBeliefSchema {
    InitialBeliefSchema {
        belief_id: BeliefId::new(belief_id).unwrap(),
        holder_actor_id: actor(holder_actor_id),
        proposition: Proposition::SoundHeardNearPlace {
            place_id: place(place_id),
        },
        stance: Stance::Plausible,
        confidence: Confidence::new(250).unwrap(),
        source_kind: tracewake_core::events::InitialBeliefSourceKind::AuthoredPrehistory,
        source: SourceRef::Event(EventId::new(source_id).unwrap()),
        channel: Some(Channel::SimpleSound),
        acquired_tick: SimTick::ZERO,
        last_verified_tick: None,
        privacy_scope: PrivacyScope::ActorPrivate(actor(holder_actor_id)),
        schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
    }
}
