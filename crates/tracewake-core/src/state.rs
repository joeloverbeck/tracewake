use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{
    DecisionTraceRecord, Intention, NeedKind, NeedState, RoutineExecution, StuckDiagnosticRecord,
};
use crate::ids::{
    ActorId, ContainerId, ControllerId, DecisionTraceId, DoorId, ExitId, FoodSupplyId, IntentionId,
    ItemId, PlaceId, ProposalId, RoutineExecutionId, SchemaVersion, SleepAffordanceId,
    StuckDiagnosticId, WorkplaceId,
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
    Workplace,
    SleepAffordance,
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
    Workplace(WorkplaceId),
    SleepAffordance(SleepAffordanceId),
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
            EntityId::Workplace(_) => EntityKind::Workplace,
            EntityId::SleepAffordance(_) => EntityKind::SleepAffordance,
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
            EntityId::Workplace(id) => id.as_str(),
            EntityId::SleepAffordance(id) => id.as_str(),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalState {
    pub(crate) actors: BTreeMap<ActorId, ActorBody>,
    pub(crate) places: BTreeMap<PlaceId, PlaceState>,
    pub(crate) doors: BTreeMap<DoorId, DoorState>,
    pub(crate) containers: BTreeMap<ContainerId, ContainerState>,
    pub(crate) items: BTreeMap<ItemId, ItemState>,
    pub(crate) food_supplies: BTreeMap<FoodSupplyId, FoodSupplyState>,
    pub(crate) workplaces: BTreeMap<WorkplaceId, WorkplaceState>,
    pub(crate) sleep_affordances: BTreeMap<SleepAffordanceId, SleepAffordanceState>,
    pub(crate) need_model: NeedModelState,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentState {
    pub(crate) needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
    pub(crate) need_tick_charges: BTreeSet<(ActorId, NeedKind, u64)>,
    pub(crate) intentions: BTreeMap<IntentionId, Intention>,
    pub(crate) active_intention_by_actor: BTreeMap<ActorId, IntentionId>,
    pub(crate) routine_executions: BTreeMap<RoutineExecutionId, RoutineExecution>,
    pub(crate) decision_traces: BTreeMap<DecisionTraceId, DecisionTraceRecord>,
    pub(crate) stuck_diagnostics: BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord>,
    pub(crate) need_threshold_crossings: BTreeMap<crate::ids::EventId, NeedThresholdCrossingRecord>,
    pub(crate) ordinary_life_episodes: BTreeMap<crate::ids::EventId, OrdinaryLifeEpisodeRecord>,
    pub(crate) candidate_goal_evaluations:
        BTreeMap<crate::ids::EventId, CandidateGoalEvaluationRecord>,
    pub(crate) continue_routine_arbitrations:
        BTreeMap<crate::ids::EventId, ContinueRoutineArbitrationRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NeedThresholdCrossingRecord {
    pub event_id: crate::ids::EventId,
    pub actor_id: ActorId,
    pub need_kind: NeedKind,
    pub from_value: u16,
    pub to_value: u16,
    pub from_band: String,
    pub to_band: String,
    pub payload_fields: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrdinaryLifeEpisodeRecord {
    pub event_id: crate::ids::EventId,
    pub event_kind: String,
    pub actor_id: Option<ActorId>,
    pub proposal_id: Option<ProposalId>,
    pub caused_event_ids: Vec<crate::ids::EventId>,
    pub sim_tick: crate::time::SimTick,
    pub payload_fields: Vec<(String, String)>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidateGoalEvaluationRecord {
    pub event_id: crate::ids::EventId,
    pub event_kind: String,
    pub actor_id: Option<ActorId>,
    pub proposal_id: Option<ProposalId>,
    pub caused_event_ids: Vec<crate::ids::EventId>,
    pub sim_tick: crate::time::SimTick,
    pub payload_fields: Vec<(String, String)>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContinueRoutineArbitrationRecord {
    pub event_id: crate::ids::EventId,
    pub event_kind: String,
    pub actor_id: Option<ActorId>,
    pub proposal_id: Option<ProposalId>,
    pub caused_event_ids: Vec<crate::ids::EventId>,
    pub sim_tick: crate::time::SimTick,
    pub payload_fields: Vec<(String, String)>,
    pub summary: String,
}

impl PhysicalState {
    pub fn empty(need_model: NeedModelState) -> Self {
        Self {
            actors: BTreeMap::new(),
            places: BTreeMap::new(),
            doors: BTreeMap::new(),
            containers: BTreeMap::new(),
            items: BTreeMap::new(),
            food_supplies: BTreeMap::new(),
            workplaces: BTreeMap::new(),
            sleep_affordances: BTreeMap::new(),
            need_model,
        }
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Seed construction mirrors authoritative state collections."
    )]
    pub fn from_seed_parts(
        actors: BTreeMap<ActorId, ActorBody>,
        places: BTreeMap<PlaceId, PlaceState>,
        doors: BTreeMap<DoorId, DoorState>,
        containers: BTreeMap<ContainerId, ContainerState>,
        items: BTreeMap<ItemId, ItemState>,
        food_supplies: BTreeMap<FoodSupplyId, FoodSupplyState>,
        workplaces: BTreeMap<WorkplaceId, WorkplaceState>,
        sleep_affordances: BTreeMap<SleepAffordanceId, SleepAffordanceState>,
        need_model: NeedModelState,
    ) -> Self {
        Self {
            actors,
            places,
            doors,
            containers,
            items,
            food_supplies,
            workplaces,
            sleep_affordances,
            need_model,
        }
    }

    pub fn actors(&self) -> &BTreeMap<ActorId, ActorBody> {
        &self.actors
    }

    pub fn places(&self) -> &BTreeMap<PlaceId, PlaceState> {
        &self.places
    }

    pub fn doors(&self) -> &BTreeMap<DoorId, DoorState> {
        &self.doors
    }

    pub fn containers(&self) -> &BTreeMap<ContainerId, ContainerState> {
        &self.containers
    }

    pub fn items(&self) -> &BTreeMap<ItemId, ItemState> {
        &self.items
    }

    pub fn food_supplies(&self) -> &BTreeMap<FoodSupplyId, FoodSupplyState> {
        &self.food_supplies
    }

    pub fn workplaces(&self) -> &BTreeMap<WorkplaceId, WorkplaceState> {
        &self.workplaces
    }

    pub fn sleep_affordances(&self) -> &BTreeMap<SleepAffordanceId, SleepAffordanceState> {
        &self.sleep_affordances
    }

    pub const fn need_model(&self) -> &NeedModelState {
        &self.need_model
    }

    pub fn set_need_model(&mut self, need_model: NeedModelState) {
        self.need_model = need_model;
    }
}

impl AgentState {
    pub fn from_seed_parts(
        needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
        intentions: BTreeMap<IntentionId, Intention>,
        active_intention_by_actor: BTreeMap<ActorId, IntentionId>,
        routine_executions: BTreeMap<RoutineExecutionId, RoutineExecution>,
        decision_traces: BTreeMap<DecisionTraceId, DecisionTraceRecord>,
        stuck_diagnostics: BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord>,
    ) -> Self {
        Self {
            needs_by_actor,
            need_tick_charges: BTreeSet::new(),
            intentions,
            active_intention_by_actor,
            routine_executions,
            decision_traces,
            stuck_diagnostics,
            need_threshold_crossings: BTreeMap::new(),
            ordinary_life_episodes: BTreeMap::new(),
            candidate_goal_evaluations: BTreeMap::new(),
            continue_routine_arbitrations: BTreeMap::new(),
        }
    }

    pub fn needs_by_actor(&self) -> &BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>> {
        &self.needs_by_actor
    }

    pub fn intentions(&self) -> &BTreeMap<IntentionId, Intention> {
        &self.intentions
    }

    pub fn active_intention_by_actor(&self) -> &BTreeMap<ActorId, IntentionId> {
        &self.active_intention_by_actor
    }

    pub fn routine_executions(&self) -> &BTreeMap<RoutineExecutionId, RoutineExecution> {
        &self.routine_executions
    }

    pub fn decision_traces(&self) -> &BTreeMap<DecisionTraceId, DecisionTraceRecord> {
        &self.decision_traces
    }

    pub fn stuck_diagnostics(&self) -> &BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord> {
        &self.stuck_diagnostics
    }

    pub fn need_threshold_crossings(
        &self,
    ) -> &BTreeMap<crate::ids::EventId, NeedThresholdCrossingRecord> {
        &self.need_threshold_crossings
    }

    pub fn ordinary_life_episodes(
        &self,
    ) -> &BTreeMap<crate::ids::EventId, OrdinaryLifeEpisodeRecord> {
        &self.ordinary_life_episodes
    }

    pub fn candidate_goal_evaluations(
        &self,
    ) -> &BTreeMap<crate::ids::EventId, CandidateGoalEvaluationRecord> {
        &self.candidate_goal_evaluations
    }

    pub fn continue_routine_arbitrations(
        &self,
    ) -> &BTreeMap<crate::ids::EventId, ContinueRoutineArbitrationRecord> {
        &self.continue_routine_arbitrations
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkplaceState {
    pub workplace_id: WorkplaceId,
    pub place_id: PlaceId,
    pub assigned_actor_ids: BTreeSet<ActorId>,
    pub work_duration_ticks: u64,
    pub fatigue_delta_per_tick: i32,
    pub hunger_delta_per_tick: i32,
    pub max_fatigue_to_start: i32,
    pub max_hunger_to_start: i32,
    pub access_open: bool,
    pub output_tag: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SleepAffordanceState {
    pub sleep_affordance_id: SleepAffordanceId,
    pub place_id: PlaceId,
    pub access_open: bool,
    pub rest_quality: u32,
    pub duration_ticks: u64,
    pub fatigue_recovery_per_tick: i32,
    pub hunger_rise_per_tick: i32,
}

impl SleepAffordanceState {
    pub fn new(
        sleep_affordance_id: SleepAffordanceId,
        place_id: PlaceId,
        duration_ticks: u64,
        fatigue_recovery_per_tick: i32,
        hunger_rise_per_tick: i32,
    ) -> Self {
        Self {
            sleep_affordance_id,
            place_id,
            access_open: true,
            rest_quality: 1,
            duration_ticks,
            fatigue_recovery_per_tick,
            hunger_rise_per_tick,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeedModelState {
    pub awake_hunger_delta_per_tick: i32,
    pub awake_fatigue_delta_per_tick: i32,
}

impl NeedModelState {
    pub const fn new(awake_hunger_delta_per_tick: i32, awake_fatigue_delta_per_tick: i32) -> Self {
        Self {
            awake_hunger_delta_per_tick,
            awake_fatigue_delta_per_tick,
        }
    }
}

impl WorkplaceState {
    #[allow(
        clippy::too_many_arguments,
        reason = "Workplace construction requires all authored tuning values explicitly."
    )]
    pub fn new(
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        work_duration_ticks: u64,
        fatigue_delta_per_tick: i32,
        hunger_delta_per_tick: i32,
        max_fatigue_to_start: i32,
        max_hunger_to_start: i32,
        output_tag: impl Into<String>,
    ) -> Self {
        Self {
            workplace_id,
            place_id,
            assigned_actor_ids: BTreeSet::new(),
            work_duration_ticks,
            fatigue_delta_per_tick,
            hunger_delta_per_tick,
            max_fatigue_to_start,
            max_hunger_to_start,
            access_open: true,
            output_tag: output_tag.into(),
        }
    }
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

    fn workplace_id(value: &str) -> WorkplaceId {
        WorkplaceId::new(value).unwrap()
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

    #[test]
    fn workplace_models_duration_access_and_non_economic_output() {
        let mut workplace = WorkplaceState::new(
            workplace_id("workplace_office"),
            place_id("office"),
            4,
            8,
            4,
            900,
            900,
            "service_completed_placeholder",
        );
        workplace.assigned_actor_ids.insert(actor_id("actor_tomas"));
        workplace.work_duration_ticks = 3;

        assert!(workplace.access_open);
        assert_eq!(workplace.work_duration_ticks, 3);
        assert_eq!(workplace.output_tag, "service_completed_placeholder");
        assert!(workplace
            .assigned_actor_ids
            .contains(&actor_id("actor_tomas")));
    }
}
