#![allow(dead_code)]

pub mod acceptance_status_manifest;
pub mod generative;

use std::collections::BTreeMap;

use tracewake_core::agent::{
    DecisionTraceRecord, Intention, NeedKind, NeedState, RoutineExecution, StuckDiagnosticRecord,
};
use tracewake_core::ids::{
    ActorId, ContainerId, DecisionTraceId, DoorId, FoodSupplyId, IntentionId, ItemId, PlaceId,
    RoutineExecutionId, SleepAffordanceId, StuckDiagnosticId, WorkplaceId,
};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, FoodSupplyState, ItemState, NeedModelState,
    PhysicalState, PlaceState, SleepAffordanceState, WorkplaceState,
};

#[derive(Default)]
pub struct PhysicalSeed {
    actors: BTreeMap<ActorId, ActorBody>,
    places: BTreeMap<PlaceId, PlaceState>,
    doors: BTreeMap<DoorId, DoorState>,
    containers: BTreeMap<ContainerId, ContainerState>,
    items: BTreeMap<ItemId, ItemState>,
    food_supplies: BTreeMap<FoodSupplyId, FoodSupplyState>,
    workplaces: BTreeMap<WorkplaceId, WorkplaceState>,
    sleep_affordances: BTreeMap<SleepAffordanceId, SleepAffordanceState>,
}

impl PhysicalSeed {
    pub fn from_state(state: &PhysicalState) -> Self {
        Self {
            actors: state.actors().clone(),
            places: state.places().clone(),
            doors: state.doors().clone(),
            containers: state.containers().clone(),
            items: state.items().clone(),
            food_supplies: state.food_supplies().clone(),
            workplaces: state.workplaces().clone(),
            sleep_affordances: state.sleep_affordances().clone(),
        }
    }

    pub fn actors_mut(&mut self) -> &mut BTreeMap<ActorId, ActorBody> {
        &mut self.actors
    }

    pub fn actors(&self) -> &BTreeMap<ActorId, ActorBody> {
        &self.actors
    }

    pub fn places_mut(&mut self) -> &mut BTreeMap<PlaceId, PlaceState> {
        &mut self.places
    }

    pub fn doors_mut(&mut self) -> &mut BTreeMap<DoorId, DoorState> {
        &mut self.doors
    }

    pub fn containers_mut(&mut self) -> &mut BTreeMap<ContainerId, ContainerState> {
        &mut self.containers
    }

    pub fn items_mut(&mut self) -> &mut BTreeMap<ItemId, ItemState> {
        &mut self.items
    }

    pub fn food_supplies_mut(&mut self) -> &mut BTreeMap<FoodSupplyId, FoodSupplyState> {
        &mut self.food_supplies
    }

    pub fn workplaces_mut(&mut self) -> &mut BTreeMap<WorkplaceId, WorkplaceState> {
        &mut self.workplaces
    }

    pub fn sleep_affordances_mut(
        &mut self,
    ) -> &mut BTreeMap<SleepAffordanceId, SleepAffordanceState> {
        &mut self.sleep_affordances
    }

    pub fn build(self) -> PhysicalState {
        PhysicalState::from_test_seed_parts(
            self.actors,
            self.places,
            self.doors,
            self.containers,
            self.items,
            self.food_supplies,
            self.workplaces,
            self.sleep_affordances,
            NeedModelState::new(5, 3),
        )
    }
}

#[derive(Default)]
pub struct AgentSeed {
    needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
    intentions: BTreeMap<IntentionId, Intention>,
    active_intention_by_actor: BTreeMap<ActorId, IntentionId>,
    routine_executions: BTreeMap<RoutineExecutionId, RoutineExecution>,
    decision_traces: BTreeMap<DecisionTraceId, DecisionTraceRecord>,
    stuck_diagnostics: BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord>,
}

impl AgentSeed {
    pub fn from_state(state: &AgentState) -> Self {
        Self {
            needs_by_actor: state.needs_by_actor().clone(),
            intentions: state.intentions().clone(),
            active_intention_by_actor: state.active_intention_by_actor().clone(),
            routine_executions: state.routine_executions().clone(),
            decision_traces: state.decision_traces().clone(),
            stuck_diagnostics: state.stuck_diagnostics().clone(),
        }
    }

    pub fn needs_by_actor_mut(&mut self) -> &mut BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>> {
        &mut self.needs_by_actor
    }

    pub fn routine_executions_mut(
        &mut self,
    ) -> &mut BTreeMap<RoutineExecutionId, RoutineExecution> {
        &mut self.routine_executions
    }

    pub fn intentions_mut(&mut self) -> &mut BTreeMap<IntentionId, Intention> {
        &mut self.intentions
    }

    pub fn active_intention_by_actor_mut(&mut self) -> &mut BTreeMap<ActorId, IntentionId> {
        &mut self.active_intention_by_actor
    }

    pub fn build(self) -> AgentState {
        AgentState::from_test_seed_parts(
            self.needs_by_actor,
            self.intentions,
            self.active_intention_by_actor,
            self.routine_executions,
            self.decision_traces,
            self.stuck_diagnostics,
        )
    }
}
