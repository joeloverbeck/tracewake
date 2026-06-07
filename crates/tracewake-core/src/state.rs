use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{Intention, NeedKind, NeedState, RoutineExecution};
use crate::ids::{
    ActorId, ContainerId, ControllerId, DecisionTraceId, DoorId, ExitId, FoodSupplyId, IntentionId,
    ItemId, PlaceId, RoutineExecutionId, SchemaVersion, StuckDiagnosticId,
};
use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityKind {
    Actor,
    Place,
    Door,
    Container,
    Item,
    FoodSupply,
    InstitutionPlaceholder,
    RecordPlaceholder,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityId {
    Actor(ActorId),
    Place(PlaceId),
    Door(DoorId),
    Container(ContainerId),
    Item(ItemId),
    FoodSupply(FoodSupplyId),
    InstitutionPlaceholder(SchemaVersion),
    RecordPlaceholder(SchemaVersion),
}

impl EntityId {
    pub fn kind(&self) -> EntityKind {
        match self {
            EntityId::Actor(_) => EntityKind::Actor,
            EntityId::Place(_) => EntityKind::Place,
            EntityId::Door(_) => EntityKind::Door,
            EntityId::Container(_) => EntityKind::Container,
            EntityId::Item(_) => EntityKind::Item,
            EntityId::FoodSupply(_) => EntityKind::FoodSupply,
            EntityId::InstitutionPlaceholder(_) => EntityKind::InstitutionPlaceholder,
            EntityId::RecordPlaceholder(_) => EntityKind::RecordPlaceholder,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            EntityId::Actor(id) => id.as_str(),
            EntityId::Place(id) => id.as_str(),
            EntityId::Door(id) => id.as_str(),
            EntityId::Container(id) => id.as_str(),
            EntityId::Item(id) => id.as_str(),
            EntityId::FoodSupply(id) => id.as_str(),
            EntityId::InstitutionPlaceholder(id) => id.as_str(),
            EntityId::RecordPlaceholder(id) => id.as_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityHeader {
    pub id: EntityId,
    pub kind: EntityKind,
    pub display_label: String,
    pub tags: BTreeSet<String>,
}

impl EntityHeader {
    pub fn new(id: EntityId, display_label: impl Into<String>) -> Self {
        let kind = id.kind();
        Self {
            id,
            kind,
            display_label: display_label.into(),
            tags: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorBody {
    pub actor_id: ActorId,
    pub current_place_id: PlaceId,
    pub carried_item_ids: BTreeSet<ItemId>,
    pub carry_capacity: Option<u32>,
    pub enabled: bool,
}

impl ActorBody {
    pub fn new(actor_id: ActorId, current_place_id: PlaceId) -> Self {
        Self {
            actor_id,
            current_place_id,
            carried_item_ids: BTreeSet::new(),
            carry_capacity: None,
            enabled: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceState {
    pub place_id: PlaceId,
    pub display_label: String,
    pub adjacent_place_ids: BTreeSet<PlaceId>,
    pub connected_door_ids: BTreeSet<DoorId>,
    pub local_container_ids: BTreeSet<ContainerId>,
    pub local_item_ids: BTreeSet<ItemId>,
    pub local_actor_ids: BTreeSet<ActorId>,
    pub visibility_default: VisibilityDefault,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhysicalState {
    pub actors: BTreeMap<ActorId, ActorBody>,
    pub places: BTreeMap<PlaceId, PlaceState>,
    pub doors: BTreeMap<DoorId, DoorState>,
    pub containers: BTreeMap<ContainerId, ContainerState>,
    pub items: BTreeMap<ItemId, ItemState>,
    pub food_supplies: BTreeMap<FoodSupplyId, FoodSupplyState>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentState {
    pub needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
    pub intentions: BTreeMap<IntentionId, Intention>,
    pub active_intention_by_actor: BTreeMap<ActorId, IntentionId>,
    pub routine_executions: BTreeMap<RoutineExecutionId, RoutineExecution>,
    pub decision_traces: BTreeMap<DecisionTraceId, String>,
    pub stuck_diagnostics: BTreeMap<StuckDiagnosticId, String>,
}

impl PlaceState {
    pub fn new(place_id: PlaceId, display_label: impl Into<String>) -> Self {
        Self {
            place_id,
            display_label: display_label.into(),
            adjacent_place_ids: BTreeSet::new(),
            connected_door_ids: BTreeSet::new(),
            local_container_ids: BTreeSet::new(),
            local_item_ids: BTreeSet::new(),
            local_actor_ids: BTreeSet::new(),
            visibility_default: VisibilityDefault::Visible,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VisibilityDefault {
    Visible,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoorState {
    pub door_id: DoorId,
    pub endpoint_a: PlaceId,
    pub endpoint_b: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
    pub access_key_item_id: Option<ItemId>,
    pub blocks_movement_when_closed: bool,
}

impl DoorState {
    pub fn new(door_id: DoorId, endpoint_a: PlaceId, endpoint_b: PlaceId) -> Self {
        Self {
            door_id,
            endpoint_a,
            endpoint_b,
            is_open: false,
            is_locked: false,
            access_key_item_id: None,
            blocks_movement_when_closed: true,
        }
    }

    pub fn connects_place(&self, place_id: &PlaceId) -> bool {
        &self.endpoint_a == place_id || &self.endpoint_b == place_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerState {
    pub container_id: ContainerId,
    pub location: Location,
    pub is_open: bool,
    pub is_locked: bool,
    pub contents: BTreeSet<ItemId>,
    pub capacity: Option<u32>,
    pub contents_visible_when_closed: bool,
}

impl ContainerState {
    pub fn fixed_at_place(container_id: ContainerId, place_id: PlaceId) -> Self {
        Self {
            container_id,
            location: Location::AtPlace(place_id),
            is_open: false,
            is_locked: false,
            contents: BTreeSet::new(),
            capacity: None,
            contents_visible_when_closed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemState {
    pub item_id: ItemId,
    pub portable: bool,
    pub carry_cost: u32,
    pub location: Location,
    pub value_token: Option<ValueToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoodSupplyState {
    pub food_supply_id: FoodSupplyId,
    pub location: Location,
    pub servings: u32,
    pub hunger_reduction_per_serving: i32,
}

impl FoodSupplyState {
    pub fn new(
        food_supply_id: FoodSupplyId,
        location: Location,
        servings: u32,
        hunger_reduction_per_serving: i32,
    ) -> Self {
        Self {
            food_supply_id,
            location,
            servings,
            hunger_reduction_per_serving,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.servings == 0
    }
}

impl ItemState {
    pub fn new(item_id: ItemId, location: Location) -> Self {
        Self {
            item_id,
            portable: true,
            carry_cost: 1,
            location,
            value_token: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueToken {
    pub denomination: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CustodyParty {
    Actor(ActorId),
    Placeholder(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnershipCustody {
    pub legal_owner: Option<CustodyParty>,
    pub custodian: Option<CustodyParty>,
    pub permitted_access: BTreeSet<ActorId>,
    pub expected_location: Option<Location>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerBinding {
    pub controller_id: ControllerId,
    pub bound_actor_id: Option<ActorId>,
    pub mode: ControllerMode,
    pub binding_sequence: u64,
}

impl ControllerBinding {
    pub fn detached(controller_id: ControllerId, binding_sequence: u64) -> Self {
        Self {
            controller_id,
            bound_actor_id: None,
            mode: ControllerMode::Detached,
            binding_sequence,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControllerMode {
    Embodied,
    Debug,
    Detached,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExitReference {
    pub exit_id: ExitId,
    pub from_place_id: PlaceId,
    pub to_place_id: PlaceId,
    pub door_id: Option<DoorId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn food_supply_id(value: &str) -> FoodSupplyId {
        FoodSupplyId::new(value).unwrap()
    }

    #[test]
    fn records_use_ordered_collections() {
        let mut actor = ActorBody::new(actor_id("actor_tomas"), place_id("shop_front"));
        actor.carried_item_ids.insert(item_id("coin_stack_10"));
        actor.carried_item_ids.insert(item_id("coin_stack_01"));

        let ordered: Vec<_> = actor.carried_item_ids.iter().map(ItemId::as_str).collect();
        assert_eq!(ordered, ["coin_stack_01", "coin_stack_10"]);

        let mut container =
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), place_id("shop_front"));
        container.contents.insert(item_id("coin_stack_10"));
        container.contents.insert(item_id("coin_stack_01"));

        let ordered: Vec<_> = container.contents.iter().map(ItemId::as_str).collect();
        assert_eq!(ordered, ["coin_stack_01", "coin_stack_10"]);
    }

    #[test]
    fn ownership_custody_has_no_current_holder_field() {
        let mut custody = OwnershipCustody {
            legal_owner: Some(CustodyParty::Actor(actor_id("actor_tomas"))),
            custodian: Some(CustodyParty::Actor(actor_id("actor_tomas"))),
            permitted_access: BTreeSet::new(),
            expected_location: Some(Location::AtPlace(place_id("shop_front"))),
        };
        custody.permitted_access.insert(actor_id("actor_mara"));

        assert!(custody.permitted_access.contains(&actor_id("actor_mara")));
        assert_eq!(
            custody.expected_location,
            Some(Location::AtPlace(place_id("shop_front")))
        );
    }

    #[test]
    fn entity_header_kind_follows_typed_entity_id() {
        let header = EntityHeader::new(
            EntityId::Container(container_id("strongbox_tomas")),
            "Tomas's strongbox",
        );

        assert_eq!(header.kind, EntityKind::Container);
        assert_eq!(header.id.as_str(), "strongbox_tomas");
    }

    #[test]
    fn food_supply_models_finite_zero_servings_without_economy_fields() {
        let full = FoodSupplyState::new(
            food_supply_id("food_bread_loaf"),
            Location::AtPlace(place_id("kitchen")),
            2,
            100,
        );
        let empty = FoodSupplyState::new(
            food_supply_id("food_empty_bowl"),
            Location::AtPlace(place_id("kitchen")),
            0,
            100,
        );

        assert!(!full.is_empty());
        assert!(empty.is_empty());
        assert_eq!(full.hunger_reduction_per_serving, 100);
    }
}
