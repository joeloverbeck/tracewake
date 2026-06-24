use crate::agent::RoutineStepStatus;
use crate::ids::{AgentProjectionVersion, ContentVersion, FixtureId};
use crate::location::Location;
use crate::state::{AgentState, PhysicalState};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChecksumStateKind {
    Physical,
    Agent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateChecksumCoverage {
    pub state_kind: ChecksumStateKind,
    pub field_name: &'static str,
    pub field_family: &'static str,
}

pub const PHYSICAL_STATE_CHECKSUM_COVERAGE: &[StateChecksumCoverage] = &[
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "actors",
        field_family: "actor",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "places",
        field_family: "place",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "doors",
        field_family: "door",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "containers",
        field_family: "container",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "items",
        field_family: "item",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "food_supplies",
        field_family: "food_supply",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "workplaces",
        field_family: "workplace",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "sleep_affordances",
        field_family: "sleep_affordance",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Physical,
        field_name: "need_model",
        field_family: "need_model",
    },
];

pub const AGENT_STATE_CHECKSUM_COVERAGE: &[StateChecksumCoverage] = &[
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "needs_by_actor",
        field_family: "need",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "need_tick_charges",
        field_family: "need_tick_charge",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "intentions",
        field_family: "intention",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "active_intention_by_actor",
        field_family: "active_intention",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "routine_executions",
        field_family: "routine_execution",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "decision_traces",
        field_family: "decision_trace",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "stuck_diagnostics",
        field_family: "stuck_diagnostic",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "need_threshold_crossings",
        field_family: "need_threshold_crossing",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "ordinary_life_episodes",
        field_family: "ordinary_life_episode",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "candidate_goal_evaluations",
        field_family: "candidate_goal_evaluation",
    },
    StateChecksumCoverage {
        state_kind: ChecksumStateKind::Agent,
        field_name: "continue_routine_arbitrations",
        field_family: "continue_routine_arbitration",
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChecksumContext {
    pub fixture_id: FixtureId,
    pub content_version: ContentVersion,
    pub sim_tick: SimTick,
    pub world_stream_position_applied: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalChecksum(String);

impl PhysicalChecksum {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_canonical_lines(lines: &[String]) -> Self {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for line in lines {
            for byte in line.as_bytes().iter().copied().chain([b'\n']) {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        Self(format!("twc1-{hash:016x}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalChecksumReport {
    pub checksum: PhysicalChecksum,
    pub canonical_input: Vec<String>,
}

impl PhysicalChecksumReport {
    pub fn recompute_from_canonical_input(&self) -> PhysicalChecksum {
        PhysicalChecksum::from_canonical_lines(&self.canonical_input)
    }
}

pub fn compute_physical_checksum(
    state: &PhysicalState,
    context: &ChecksumContext,
) -> PhysicalChecksumReport {
    let mut lines = vec![
        format!("fixture_id={}", context.fixture_id.as_str()),
        format!("content_version={}", context.content_version.as_str()),
        format!("sim_tick={}", context.sim_tick.value()),
        format!(
            "world_stream_position_applied={}",
            context.world_stream_position_applied
        ),
    ];

    for (actor_id, actor) in &state.actors {
        lines.push(format!(
            "actor|id={}|place={}|enabled={}|carried={}",
            actor_id.as_str(),
            actor.current_place_id.as_str(),
            actor.enabled,
            join_ids(actor.carried_item_ids.iter().map(|id| id.as_str()))
        ));
    }

    for (place_id, place) in &state.places {
        lines.push(format!(
            "place|id={}|label={}|adjacent={}|doors={}|containers={}|items={}|actors={}",
            place_id.as_str(),
            place.display_label,
            join_ids(place.adjacent_place_ids.iter().map(|id| id.as_str())),
            join_ids(place.connected_door_ids.iter().map(|id| id.as_str())),
            join_ids(place.local_container_ids.iter().map(|id| id.as_str())),
            join_ids(place.local_item_ids.iter().map(|id| id.as_str())),
            join_ids(place.local_actor_ids.iter().map(|id| id.as_str()))
        ));
    }

    for (door_id, door) in &state.doors {
        lines.push(format!(
            "door|id={}|a={}|b={}|open={}|locked={}|key={}|blocks_closed={}",
            door_id.as_str(),
            door.endpoint_a.as_str(),
            door.endpoint_b.as_str(),
            door.is_open,
            door.is_locked,
            door.access_key_item_id
                .as_ref()
                .map(|id| id.as_str())
                .unwrap_or(""),
            door.blocks_movement_when_closed
        ));
    }

    for (container_id, container) in &state.containers {
        lines.push(format!(
            "container|id={}|location={}|open={}|locked={}|contents={}|visible_when_closed={}",
            container_id.as_str(),
            location_key(&container.location),
            container.is_open,
            container.is_locked,
            join_ids(container.contents.iter().map(|id| id.as_str())),
            container.contents_visible_when_closed
        ));
    }

    for (item_id, item) in &state.items {
        lines.push(format!(
            "item|id={}|portable={}|carry_cost={}|location={}|value={}",
            item_id.as_str(),
            item.portable,
            item.carry_cost,
            location_key(&item.location),
            item.value_token
                .as_ref()
                .map(|value| format!("{}:{}", value.denomination, value.quantity))
                .unwrap_or_default()
        ));
    }

    for (food_supply_id, food) in &state.food_supplies {
        lines.push(format!(
            "food_supply|id={}|location={}|servings={}|hunger_reduction={}",
            food_supply_id.as_str(),
            location_key(&food.location),
            food.servings,
            food.hunger_reduction_per_serving
        ));
    }

    for (workplace_id, workplace) in &state.workplaces {
        lines.push(format!(
            "workplace|id={}|place={}|assigned={}|duration={}|fatigue_delta={}|hunger_delta={}|max_fatigue={}|max_hunger={}|access_open={}|output_tag={}",
            workplace_id.as_str(),
            workplace.place_id.as_str(),
            join_ids(workplace.assigned_actor_ids.iter().map(|id| id.as_str())),
            workplace.work_duration_ticks,
            workplace.fatigue_delta_per_tick,
            workplace.hunger_delta_per_tick,
            workplace.max_fatigue_to_start,
            workplace.max_hunger_to_start,
            workplace.access_open,
            workplace.output_tag
        ));
    }

    for (sleep_affordance_id, sleep_affordance) in &state.sleep_affordances {
        lines.push(format!(
            "sleep_affordance|id={}|place={}|access_open={}|rest_quality={}|duration_ticks={}|fatigue_recovery_per_tick={}|hunger_rise_per_tick={}",
            sleep_affordance_id.as_str(),
            sleep_affordance.place_id.as_str(),
            sleep_affordance.access_open,
            sleep_affordance.rest_quality,
            sleep_affordance.duration_ticks,
            sleep_affordance.fatigue_recovery_per_tick,
            sleep_affordance.hunger_rise_per_tick
        ));
    }

    lines.push(format!(
        "need_model|awake_hunger_delta_per_tick={}|awake_fatigue_delta_per_tick={}",
        state.need_model.awake_hunger_delta_per_tick, state.need_model.awake_fatigue_delta_per_tick
    ));

    let checksum = PhysicalChecksum::from_canonical_lines(&lines);
    PhysicalChecksumReport {
        checksum,
        canonical_input: lines,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentStateChecksum(String);

impl AgentStateChecksum {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_canonical_lines(lines: &[String]) -> Self {
        let physical = PhysicalChecksum::from_canonical_lines(lines);
        Self(format!(
            "twa1-{}",
            physical.as_str().trim_start_matches("twc1-")
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentStateChecksumReport {
    pub projection_version: AgentProjectionVersion,
    pub checksum: AgentStateChecksum,
    pub canonical_input: Vec<String>,
}

impl AgentStateChecksumReport {
    pub fn recompute_from_canonical_input(&self) -> AgentStateChecksum {
        AgentStateChecksum::from_canonical_lines(&self.canonical_input)
    }
}

pub fn compute_agent_state_checksum(
    state: &AgentState,
    context: &ChecksumContext,
) -> AgentStateChecksumReport {
    let projection_version = AgentProjectionVersion::new("agent_projection_v1").unwrap();
    let mut lines = vec![
        format!("projection_version={}", projection_version.as_str()),
        format!("fixture_id={}", context.fixture_id.as_str()),
        format!("content_version={}", context.content_version.as_str()),
        format!("sim_tick={}", context.sim_tick.value()),
        format!(
            "world_stream_position_applied={}",
            context.world_stream_position_applied
        ),
    ];

    for (actor_id, needs) in &state.needs_by_actor {
        for (kind, need) in needs {
            lines.push(format!(
                "need|actor={}|kind={}|state={}",
                actor_id.as_str(),
                kind.stable_id(),
                need.serialize_canonical()
            ));
        }
    }

    for (actor_id, need_kind, tick) in &state.need_tick_charges {
        lines.push(format!(
            "need_tick_charge|actor={}|need={}|tick={}",
            actor_id.as_str(),
            need_kind.stable_id(),
            tick
        ));
    }

    for (intention_id, intention) in &state.intentions {
        lines.push(format!(
            "intention|id={}|actor={}|source={}|goal={}|method={}|step={}|durability={}|start={}|last_progress={}|status={}|reason={}|traces={}",
            intention_id.as_str(),
            intention.actor_id.as_str(),
            intention.source.stable_id(),
            intention.selected_goal_id.as_str(),
            intention
                .selected_routine_method
                .as_ref()
                .map(|id| id.as_str())
                .unwrap_or(""),
            intention.current_step.as_deref().unwrap_or(""),
            intention.durability_level,
            intention.start_tick.value(),
            intention.last_progress_tick.value(),
            intention.status.stable_id(),
            intention.status_reason.as_deref().unwrap_or(""),
            join_ids(intention.trace_ancestry.iter().map(|id| id.as_str()))
        ));
    }

    for (actor_id, intention_id) in &state.active_intention_by_actor {
        lines.push(format!(
            "active_intention|actor={}|intention={}",
            actor_id.as_str(),
            intention_id.as_str()
        ));
    }

    for (execution_id, execution) in &state.routine_executions {
        lines.push(format!(
            "routine_execution|id={}|actor={}|template={}|family={}|step_index={}|status={}|start={}|last_progress={}|next={}|deadline={}|actions={}|resource={}|fallbacks={}|reason={}|trace={}",
            execution_id.as_str(),
            execution.actor_id.as_str(),
            execution.template_id.as_str(),
            execution.family.stable_id(),
            execution.current_step_index,
            routine_step_status_id(execution.step_status),
            execution.start_tick.value(),
            execution.last_progress_tick.value(),
            execution
                .expected_next_progress_tick
                .map(|tick| tick.value().to_string())
                .unwrap_or_default(),
            execution
                .deadline_tick
                .map(|tick| tick.value().to_string())
                .unwrap_or_default(),
            join_ids(execution.concrete_action_ancestry.iter().map(|id| id.as_str())),
            execution.reserved_resource.as_deref().unwrap_or(""),
            execution.fallback_attempts,
            execution.failure_interruption_reason.as_deref().unwrap_or(""),
            execution.trace_id.as_str()
        ));
    }

    for (trace_id, trace) in &state.decision_traces {
        lines.push(format!(
            "decision_trace|id={}|canonical={}",
            trace_id.as_str(),
            trace.serialize_canonical()
        ));
    }

    for (diagnostic_id, diagnostic) in &state.stuck_diagnostics {
        lines.push(format!(
            "stuck_diagnostic|id={}|canonical={}",
            diagnostic_id.as_str(),
            diagnostic.serialize_canonical()
        ));
    }

    for (event_id, crossing) in &state.need_threshold_crossings {
        lines.push(format!(
            "need_threshold_crossing|event={}|actor={}|need={}|from_value={}|to_value={}|from_band={}|to_band={}|payload={}",
            event_id.as_str(),
            crossing.actor_id.as_str(),
            crossing.need_kind.stable_id(),
            crossing.from_value,
            crossing.to_value,
            crossing.from_band,
            crossing.to_band,
            join_pairs(&crossing.payload_fields)
        ));
    }

    for (event_id, episode) in &state.ordinary_life_episodes {
        lines.push(format!(
            "ordinary_life_episode|event={}|kind={}|actor={}|proposal={}|causes={}|tick={}|payload={}",
            event_id.as_str(),
            episode.event_kind,
            episode
                .actor_id
                .as_ref()
                .map(crate::ids::ActorId::as_str)
                .unwrap_or("-"),
            episode
                .proposal_id
                .as_ref()
                .map(crate::ids::ProposalId::as_str)
                .unwrap_or("-"),
            join_ids(episode.caused_event_ids.iter().map(|id| id.as_str())),
            episode.sim_tick.value(),
            join_pairs(&episode.payload_fields)
        ));
    }

    for (event_id, evaluation) in &state.candidate_goal_evaluations {
        lines.push(format!(
            "candidate_goal_evaluation|event={}|kind={}|actor={}|proposal={}|causes={}|tick={}|payload={}",
            event_id.as_str(),
            evaluation.event_kind,
            evaluation
                .actor_id
                .as_ref()
                .map(crate::ids::ActorId::as_str)
                .unwrap_or("-"),
            evaluation
                .proposal_id
                .as_ref()
                .map(crate::ids::ProposalId::as_str)
                .unwrap_or("-"),
            join_ids(evaluation.caused_event_ids.iter().map(|id| id.as_str())),
            evaluation.sim_tick.value(),
            join_pairs(&evaluation.payload_fields)
        ));
    }

    for (event_id, arbitration) in &state.continue_routine_arbitrations {
        lines.push(format!(
            "continue_routine_arbitration|event={}|kind={}|actor={}|proposal={}|causes={}|tick={}|payload={}",
            event_id.as_str(),
            arbitration.event_kind,
            arbitration
                .actor_id
                .as_ref()
                .map(crate::ids::ActorId::as_str)
                .unwrap_or("-"),
            arbitration
                .proposal_id
                .as_ref()
                .map(crate::ids::ProposalId::as_str)
                .unwrap_or("-"),
            join_ids(arbitration.caused_event_ids.iter().map(|id| id.as_str())),
            arbitration.sim_tick.value(),
            join_pairs(&arbitration.payload_fields)
        ));
    }

    let checksum = AgentStateChecksum::from_canonical_lines(&lines);
    AgentStateChecksumReport {
        projection_version,
        checksum,
        canonical_input: lines,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HolderKnownContextHash(String);

impl HolderKnownContextHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_canonical_lines(lines: &[String]) -> Self {
        let physical = PhysicalChecksum::from_canonical_lines(lines);
        Self(format!(
            "hkc1-{}",
            physical.as_str().trim_start_matches("twc1-")
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HolderKnownContextHashReport {
    pub hash: HolderKnownContextHash,
    pub canonical_input: Vec<String>,
}

impl HolderKnownContextHashReport {
    pub fn recompute_from_canonical_input(&self) -> HolderKnownContextHash {
        HolderKnownContextHash::from_canonical_lines(&self.canonical_input)
    }
}

pub fn compute_holder_known_context_hash(
    canonical_inputs: impl IntoIterator<Item = String>,
) -> HolderKnownContextHashReport {
    let mut lines: Vec<String> = canonical_inputs.into_iter().collect();
    lines.sort();
    lines.dedup();
    let hash = HolderKnownContextHash::from_canonical_lines(&lines);
    HolderKnownContextHashReport {
        hash,
        canonical_input: lines,
    }
}

fn join_ids<'a>(ids: impl Iterator<Item = &'a str>) -> String {
    ids.collect::<Vec<_>>().join(",")
}

fn join_pairs(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{}:{}={}:{}", key.len(), key, value.len(), value))
        .collect::<Vec<_>>()
        .join(";")
}

fn location_key(location: &Location) -> String {
    match location {
        Location::AtPlace(id) => format!("at_place:{}", id.as_str()),
        Location::InContainer(id) => format!("in_container:{}", id.as_str()),
        Location::CarriedBy(id) => format!("carried_by:{}", id.as_str()),
    }
}

fn routine_step_status_id(status: RoutineStepStatus) -> &'static str {
    match status {
        RoutineStepStatus::NotStarted => "not_started",
        RoutineStepStatus::InProgress => "in_progress",
        RoutineStepStatus::Waiting => "waiting",
        RoutineStepStatus::Completed => "completed",
        RoutineStepStatus::Failed => "failed",
        RoutineStepStatus::Interrupted => "interrupted",
        RoutineStepStatus::Suspended => "suspended",
        RoutineStepStatus::Abandoned => "abandoned",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        BlockerCategory, DecisionOutcome, DecisionTraceRecord, HiddenTruthAudit, Intention,
        IntentionSource, NeedChangeCause, NeedKind, NeedState, RoutineExecution, RoutineFamily,
        StuckDiagnostic, StuckResultingStatus,
    };
    use crate::ids::{
        ActorId, CandidateGoalId, ContainerId, ControllerId, DecisionTraceId, DoorId, EventId,
        IntentionId, ItemId, PlaceId, RoutineExecutionId, RoutineTemplateId, StuckDiagnosticId,
        WorkplaceId,
    };
    use crate::location::Location;
    use crate::state::{
        ActorBody, ContainerState, ControllerBinding, DoorState, FoodSupplyState, ItemState,
        PhysicalState, PlaceState, WorkplaceState,
    };

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

    fn door_id(value: &str) -> DoorId {
        DoorId::new(value).unwrap()
    }

    fn workplace_id(value: &str) -> WorkplaceId {
        WorkplaceId::new(value).unwrap()
    }

    fn context() -> ChecksumContext {
        ChecksumContext {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            content_version: ContentVersion::new("content_v1").unwrap(),
            sim_tick: SimTick::new(2),
            world_stream_position_applied: 7,
        }
    }

    fn state_with_insert_order(reversed: bool) -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        let mut actor = ActorBody::new(actor_id("actor_tomas"), place_id("shop_front"));
        actor.carried_item_ids.insert(item_id("coin_stack_02"));
        actor.carried_item_ids.insert(item_id("coin_stack_01"));

        let mut place = PlaceState::new(place_id("shop_front"), "Shop front");
        place.local_actor_ids.insert(actor_id("actor_tomas"));

        let mut container =
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), place_id("shop_front"));
        container.contents.insert(item_id("coin_stack_10"));
        container.contents.insert(item_id("coin_stack_03"));

        let item_a = ItemState::new(
            item_id("coin_stack_03"),
            Location::InContainer(container_id("strongbox_tomas")),
        );
        let item_b = ItemState::new(
            item_id("coin_stack_10"),
            Location::InContainer(container_id("strongbox_tomas")),
        );

        if reversed {
            state.items.insert(item_b.item_id.clone(), item_b);
            state.items.insert(item_a.item_id.clone(), item_a);
            state
                .containers
                .insert(container.container_id.clone(), container);
            state.places.insert(place.place_id.clone(), place);
            state.actors.insert(actor.actor_id.clone(), actor);
        } else {
            state.actors.insert(actor.actor_id.clone(), actor);
            state.places.insert(place.place_id.clone(), place);
            state
                .containers
                .insert(container.container_id.clone(), container);
            state.items.insert(item_a.item_id.clone(), item_a);
            state.items.insert(item_b.item_id.clone(), item_b);
        }

        state
    }

    fn response_physical_state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        let shop = place_id("shop_front");
        let back = place_id("back_room");
        let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
        shop_state.adjacent_place_ids.insert(back.clone());
        shop_state.local_actor_ids.insert(actor_id("actor_tomas"));
        shop_state
            .local_container_ids
            .insert(container_id("strongbox_tomas"));
        shop_state.local_item_ids.insert(item_id("coin_stack_01"));
        let back_state = PlaceState::new(back.clone(), "Back room");
        state.places.insert(shop.clone(), shop_state);
        state.places.insert(back.clone(), back_state);
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(actor_id("actor_tomas"), shop.clone()),
        );
        let mut door = DoorState::new(door_id("door_shop_back"), shop.clone(), back);
        door.is_open = false;
        state.doors.insert(door.door_id.clone(), door);
        let mut container =
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), shop.clone());
        container.contents.insert(item_id("coin_stack_02"));
        state
            .containers
            .insert(container.container_id.clone(), container);
        state.items.insert(
            item_id("coin_stack_01"),
            ItemState::new(item_id("coin_stack_01"), Location::AtPlace(shop.clone())),
        );
        state.food_supplies.insert(
            crate::ids::FoodSupplyId::new("food_bread").unwrap(),
            FoodSupplyState::new(
                crate::ids::FoodSupplyId::new("food_bread").unwrap(),
                Location::AtPlace(shop.clone()),
                2,
                20,
            ),
        );
        let mut workplace = WorkplaceState::new(
            workplace_id("workshop"),
            shop,
            4,
            8,
            4,
            900,
            900,
            "work_done",
        );
        workplace.assigned_actor_ids.insert(actor_id("actor_tomas"));
        state
            .workplaces
            .insert(workplace.workplace_id.clone(), workplace);
        state
    }

    fn response_agent_state() -> AgentState {
        let mut state = AgentState::default();
        state
            .needs_by_actor
            .entry(actor_id("actor_tomas"))
            .or_default()
            .insert(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 300, NeedChangeCause::FixtureInitial),
            );
        let trace_id = DecisionTraceId::new("trace_breakfast").unwrap();
        let intention_id = IntentionId::new("intention_breakfast").unwrap();
        state.intentions.insert(
            intention_id.clone(),
            Intention::adopt(
                intention_id.clone(),
                actor_id("actor_tomas"),
                IntentionSource::FixtureRoutineAssignment,
                CandidateGoalId::new("goal_breakfast").unwrap(),
                Some(RoutineTemplateId::new("routine_breakfast").unwrap()),
                Some("eat".to_string()),
                8,
                SimTick::new(1),
                trace_id.clone(),
            ),
        );
        state
            .active_intention_by_actor
            .insert(actor_id("actor_tomas"), intention_id);
        state.routine_executions.insert(
            RoutineExecutionId::new("routine_exec_breakfast").unwrap(),
            RoutineExecution::new(
                RoutineExecutionId::new("routine_exec_breakfast").unwrap(),
                actor_id("actor_tomas"),
                RoutineTemplateId::new("routine_breakfast").unwrap(),
                RoutineFamily::EatMeal,
                SimTick::new(1),
                Some(SimTick::new(2)),
                Some(SimTick::new(8)),
                Some("body".to_string()),
                trace_id.clone(),
            ),
        );
        state.decision_traces.insert(
            trace_id.clone(),
            DecisionTraceRecord {
                trace_id: trace_id.clone(),
                actor_id: actor_id("actor_tomas"),
                window_start_tick: SimTick::new(1),
                window_end_tick: SimTick::new(2),
                outcome: DecisionOutcome::Continued,
                candidate_goal_count: 1,
                actor_known_context_hash: Some(HolderKnownContextHash::from_canonical_lines(&[])),
                actor_known_inputs: Vec::new(),
                local_plan_id: None,
                proposal_ancestry: Vec::new(),
                hidden_truth_audit_result: HiddenTruthAudit {
                    actor_known_only: true,
                    notes: "fixture".to_string(),
                },
                typed_diagnostic: crate::agent::TypedDiagnosticFields::decision_default(false),
            },
        );
        state.stuck_diagnostics.insert(
            StuckDiagnosticId::new("stuck_breakfast").unwrap(),
            StuckDiagnostic::new(
                StuckDiagnosticId::new("stuck_breakfast").unwrap(),
                actor_id("actor_tomas"),
                SimTick::new(1),
                SimTick::new(2),
                Some(NeedKind::Hunger),
                Some(CandidateGoalId::new("goal_breakfast").unwrap()),
                Some(IntentionId::new("intention_breakfast").unwrap()),
                Some(RoutineTemplateId::new("routine_breakfast").unwrap()),
                Some(RoutineExecutionId::new("routine_exec_breakfast").unwrap()),
                None,
                None,
                BlockerCategory::Resource,
                "missing food",
                "actor knows no food",
                "debug",
                "wait",
                StuckResultingStatus::Waiting,
            ),
        );
        state
    }

    fn assert_physical_checksum_changes(mut mutate: impl FnMut(&mut PhysicalState)) {
        let mut state = response_physical_state();
        let before = compute_physical_checksum(&state, &context()).checksum;
        mutate(&mut state);
        let after = compute_physical_checksum(&state, &context()).checksum;
        assert_ne!(after, before);
    }

    fn assert_agent_checksum_changes(mut mutate: impl FnMut(&mut AgentState)) {
        let mut state = response_agent_state();
        let before = compute_agent_state_checksum(&state, &context()).checksum;
        mutate(&mut state);
        let after = compute_agent_state_checksum(&state, &context()).checksum;
        assert_ne!(after, before);
    }

    #[test]
    fn same_physical_state_checksums_identically() {
        let state = state_with_insert_order(false);
        let first = compute_physical_checksum(&state, &context());
        let second = compute_physical_checksum(&state, &context());

        assert_eq!(first.checksum, second.checksum);
        assert_eq!(first.canonical_input, second.canonical_input);
    }

    #[test]
    fn canonical_input_recomputes_to_same_checksum_after_reload() {
        let report = compute_physical_checksum(&state_with_insert_order(true), &context());
        let serialized = report.canonical_input.join("\n").into_bytes();
        let reloaded_lines = String::from_utf8(serialized)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        assert_eq!(
            PhysicalChecksum::from_canonical_lines(&reloaded_lines),
            report.checksum
        );
        assert_eq!(report.recompute_from_canonical_input(), report.checksum);
    }

    #[test]
    fn insertion_order_does_not_change_checksum() {
        let first = compute_physical_checksum(&state_with_insert_order(false), &context());
        let second = compute_physical_checksum(&state_with_insert_order(true), &context());

        assert_eq!(first.checksum, second.checksum);
        assert_eq!(first.canonical_input, second.canonical_input);
    }

    #[test]
    fn physical_checksum_changes_for_each_authoritative_field_family() {
        assert_physical_checksum_changes(|state| {
            state
                .actors
                .get_mut(&actor_id("actor_tomas"))
                .unwrap()
                .current_place_id = place_id("back_room");
        });
        assert_physical_checksum_changes(|state| {
            state
                .doors
                .get_mut(&door_id("door_shop_back"))
                .unwrap()
                .is_open = true;
            state
                .containers
                .get_mut(&container_id("strongbox_tomas"))
                .unwrap()
                .is_locked = true;
        });
        assert_physical_checksum_changes(|state| {
            state
                .items
                .get_mut(&item_id("coin_stack_01"))
                .unwrap()
                .portable = false;
            state
                .food_supplies
                .get_mut(&crate::ids::FoodSupplyId::new("food_bread").unwrap())
                .unwrap()
                .servings = 1;
        });
        assert_physical_checksum_changes(|state| {
            state
                .workplaces
                .get_mut(&workplace_id("workshop"))
                .unwrap()
                .assigned_actor_ids
                .insert(actor_id("actor_mara"));
        });
    }

    #[test]
    fn agent_checksum_changes_for_each_authoritative_field_family() {
        assert_agent_checksum_changes(|state| {
            state
                .needs_by_actor
                .get_mut(&actor_id("actor_tomas"))
                .unwrap()
                .get_mut(&NeedKind::Hunger)
                .unwrap()
                .apply_delta(1, NeedChangeCause::TickDelta);
        });
        assert_agent_checksum_changes(|state| {
            state
                .intentions
                .get_mut(&IntentionId::new("intention_breakfast").unwrap())
                .unwrap()
                .record_progress(SimTick::new(3), "open_pantry");
            state
                .active_intention_by_actor
                .remove(&actor_id("actor_tomas"));
        });
        assert_agent_checksum_changes(|state| {
            state
                .routine_executions
                .get_mut(&RoutineExecutionId::new("routine_exec_breakfast").unwrap())
                .unwrap()
                .fallback_attempts = 1;
        });
        assert_agent_checksum_changes(|state| {
            state
                .decision_traces
                .get_mut(&DecisionTraceId::new("trace_breakfast").unwrap())
                .unwrap()
                .candidate_goal_count = 2;
        });
        assert_agent_checksum_changes(|state| {
            state
                .stuck_diagnostics
                .get_mut(&StuckDiagnosticId::new("stuck_breakfast").unwrap())
                .unwrap()
                .concrete_blocker = "closed pantry".to_string();
        });
        assert_agent_checksum_changes(|state| {
            state.ordinary_life_episodes.insert(
                EventId::new("event.sleep.started.test").unwrap(),
                crate::state::OrdinaryLifeEpisodeRecord {
                    event_id: EventId::new("event.sleep.started.test").unwrap(),
                    event_kind: "sleep_started".to_string(),
                    actor_id: Some(actor_id("actor_tomas")),
                    proposal_id: None,
                    caused_event_ids: vec![EventId::new("event.source.test").unwrap()],
                    sim_tick: SimTick::new(3),
                    payload_fields: vec![("duration_ticks".to_string(), "4".to_string())],
                    summary: "sleep started".to_string(),
                },
            );
        });
        assert_agent_checksum_changes(|state| {
            state.candidate_goal_evaluations.insert(
                EventId::new("event.candidate_goals.test").unwrap(),
                crate::state::CandidateGoalEvaluationRecord {
                    event_id: EventId::new("event.candidate_goals.test").unwrap(),
                    event_kind: "candidate_goals_evaluated".to_string(),
                    actor_id: Some(actor_id("actor_tomas")),
                    proposal_id: None,
                    caused_event_ids: vec![EventId::new("event.source.test").unwrap()],
                    sim_tick: SimTick::new(3),
                    payload_fields: vec![("candidate_goal_count".to_string(), "2".to_string())],
                    summary: "candidate goals evaluated".to_string(),
                },
            );
        });
        assert_agent_checksum_changes(|state| {
            state.continue_routine_arbitrations.insert(
                EventId::new("event.continue_routine.test").unwrap(),
                crate::state::ContinueRoutineArbitrationRecord {
                    event_id: EventId::new("event.continue_routine.test").unwrap(),
                    event_kind: "continue_routine_rejected".to_string(),
                    actor_id: Some(actor_id("actor_tomas")),
                    proposal_id: None,
                    caused_event_ids: vec![EventId::new("event.source.test").unwrap()],
                    sim_tick: SimTick::new(3),
                    payload_fields: vec![("reason".to_string(), "blocked".to_string())],
                    summary: "continue routine rejected".to_string(),
                },
            );
        });
    }

    #[test]
    fn controller_and_debug_metadata_are_excluded() {
        let state = state_with_insert_order(false);
        let binding_a = ControllerBinding::detached(ControllerId::new("controller_a").unwrap(), 0);
        let binding_b = ControllerBinding::detached(ControllerId::new("controller_b").unwrap(), 99);
        let debug_panel_a = "closed";
        let debug_panel_b = "item_location";

        let checksum_a = compute_physical_checksum(&state, &context()).checksum;
        let checksum_b = compute_physical_checksum(&state, &context()).checksum;

        assert_ne!(binding_a.controller_id, binding_b.controller_id);
        assert_ne!(debug_panel_a, debug_panel_b);
        assert_eq!(checksum_a, checksum_b);
    }

    #[test]
    fn materialized_agent_summary_prose_is_excluded() {
        let mut state = response_agent_state();
        state.ordinary_life_episodes.insert(
            EventId::new("event.sleep.started.summary.exclusion").unwrap(),
            crate::state::OrdinaryLifeEpisodeRecord {
                event_id: EventId::new("event.sleep.started.summary.exclusion").unwrap(),
                event_kind: "sleep_started".to_string(),
                actor_id: Some(actor_id("actor_tomas")),
                proposal_id: None,
                caused_event_ids: vec![EventId::new("event.source.summary.exclusion").unwrap()],
                sim_tick: SimTick::new(3),
                payload_fields: vec![("duration_ticks".to_string(), "4".to_string())],
                summary: "sleep started".to_string(),
            },
        );
        state.candidate_goal_evaluations.insert(
            EventId::new("event.candidate.summary.exclusion").unwrap(),
            crate::state::CandidateGoalEvaluationRecord {
                event_id: EventId::new("event.candidate.summary.exclusion").unwrap(),
                event_kind: "candidate_goals_evaluated".to_string(),
                actor_id: Some(actor_id("actor_tomas")),
                proposal_id: None,
                caused_event_ids: vec![EventId::new("event.source.summary.exclusion").unwrap()],
                sim_tick: SimTick::new(3),
                payload_fields: vec![("candidate_goal_count".to_string(), "2".to_string())],
                summary: "candidate goals evaluated".to_string(),
            },
        );
        state.continue_routine_arbitrations.insert(
            EventId::new("event.continue.summary.exclusion").unwrap(),
            crate::state::ContinueRoutineArbitrationRecord {
                event_id: EventId::new("event.continue.summary.exclusion").unwrap(),
                event_kind: "continue_routine_rejected".to_string(),
                actor_id: Some(actor_id("actor_tomas")),
                proposal_id: None,
                caused_event_ids: vec![EventId::new("event.source.summary.exclusion").unwrap()],
                sim_tick: SimTick::new(3),
                payload_fields: vec![("reason".to_string(), "blocked".to_string())],
                summary: "continue routine rejected".to_string(),
            },
        );
        let before = compute_agent_state_checksum(&state, &context());

        state
            .ordinary_life_episodes
            .get_mut(&EventId::new("event.sleep.started.summary.exclusion").unwrap())
            .unwrap()
            .summary = "different display prose".to_string();
        state
            .candidate_goal_evaluations
            .get_mut(&EventId::new("event.candidate.summary.exclusion").unwrap())
            .unwrap()
            .summary = "different candidate prose".to_string();
        state
            .continue_routine_arbitrations
            .get_mut(&EventId::new("event.continue.summary.exclusion").unwrap())
            .unwrap()
            .summary = "different arbitration prose".to_string();
        let after = compute_agent_state_checksum(&state, &context());

        assert_eq!(before.checksum, after.checksum);
        assert_eq!(before.canonical_input, after.canonical_input);
        assert!(
            before
                .canonical_input
                .iter()
                .all(|line| !line.contains("|summary=")),
            "{:?}",
            before.canonical_input
        );
    }

    #[test]
    fn holder_known_context_hash_is_order_independent_and_input_sensitive() {
        let first = compute_holder_known_context_hash([
            "viewer=actor_tomas".to_string(),
            "tick=7".to_string(),
            "source=own_direct_observation".to_string(),
        ]);
        let second = compute_holder_known_context_hash([
            "source=own_direct_observation".to_string(),
            "tick=7".to_string(),
            "viewer=actor_tomas".to_string(),
        ]);
        let changed = compute_holder_known_context_hash([
            "viewer=actor_tomas".to_string(),
            "tick=8".to_string(),
            "source=own_direct_observation".to_string(),
        ]);

        assert_eq!(first.hash, second.hash);
        assert_eq!(first.canonical_input, second.canonical_input);
        assert_eq!(first.recompute_from_canonical_input(), first.hash);
        assert_ne!(first.hash, changed.hash);
    }
}
