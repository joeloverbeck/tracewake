#![allow(dead_code)]

pub mod acceptance_status_manifest;
pub mod generative;

use std::collections::BTreeMap;

use tracewake_core::agent::{
    ActorDecisionProposalOutcome, ActorDecisionTransaction, ActorDecisionTransactionInput,
    ActorDecisionTransactionOutcome, ActorKnownPlanningContext, DecisionOutcome,
    DecisionTraceRecord, Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState,
    NoHumanActorKnownSurfaceBuilder, RoutineExecution, RoutineFamily, RoutineStepStatus,
    StuckDiagnosticRecord,
};
use tracewake_core::ids::{
    ActorId, CandidateGoalId, ContainerId, DecisionTraceId, DoorId, EventId, FoodSupplyId,
    IntentionId, ItemId, PlaceId, RoutineExecutionId, RoutineTemplateId, SleepAffordanceId,
    StuckDiagnosticId, WorkplaceId,
};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, FoodSupplyState, ItemState, NeedModelState,
    PhysicalState, PlaceState, SleepAffordanceState, WorkplaceState,
};
use tracewake_core::time::SimTick;

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

#[derive(Clone)]
pub struct RoutineParityFixture {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub agent_state: AgentState,
    pub actor_known_context: ActorKnownPlanningContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutineParityOutcome {
    Proposed(RoutineParityProposalShape),
    Stuck {
        concrete_blocker: String,
        hidden_truth_referenced: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineParityProposalShape {
    pub action_id: String,
    pub actor_id: Option<ActorId>,
    pub target_ids: Vec<String>,
    pub routine_template_id: Option<String>,
    pub routine_execution_id: Option<String>,
    pub local_plan_id_present: bool,
    pub proposal_ancestry_len: usize,
    pub decision_outcome: DecisionOutcome,
    pub hidden_truth_actor_known_only: bool,
}

impl RoutineParityFixture {
    pub fn new(
        active_family: RoutineFamily,
        active_template_id: &str,
        active_execution_status: RoutineStepStatus,
        scheduler_executions: Vec<RoutineExecution>,
        current_place_id: PlaceId,
        surface: NoHumanActorKnownSurfaceBuilder,
    ) -> Self {
        let actor_id = ActorId::new("actor_parity").unwrap();
        let decision_tick = SimTick::new(8);
        let active_template_id = RoutineTemplateId::new(active_template_id).unwrap();
        let active_execution_id = RoutineExecutionId::new("routine_exec_active").unwrap();
        let trace_id = DecisionTraceId::new("trace_parity_active").unwrap();
        let mut executions = scheduler_executions;
        if !executions
            .iter()
            .any(|execution| execution.execution_id == active_execution_id)
        {
            let mut active_execution = RoutineExecution::new(
                active_execution_id.clone(),
                actor_id.clone(),
                active_template_id.clone(),
                active_family,
                SimTick::new(8),
                Some(SimTick::new(9)),
                Some(SimTick::new(16)),
                None,
                trace_id.clone(),
            );
            active_execution.step_status = active_execution_status;
            executions.push(active_execution);
        }
        let active_intention = Intention::adopt(
            IntentionId::new("intention_parity_active").unwrap(),
            actor_id.clone(),
            IntentionSource::RoutineDuty,
            CandidateGoalId::new("candidate_parity_active").unwrap(),
            Some(active_template_id),
            Some(active_family.stable_id().to_string()),
            8,
            SimTick::new(8),
            trace_id,
        );
        let mut seed = AgentSeed::default();
        seed.needs_by_actor_mut().insert(
            actor_id.clone(),
            BTreeMap::from([
                (
                    NeedKind::Hunger,
                    NeedState::initial(NeedKind::Hunger, 10, NeedChangeCause::FixtureInitial),
                ),
                (
                    NeedKind::Fatigue,
                    NeedState::initial(NeedKind::Fatigue, 10, NeedChangeCause::FixtureInitial),
                ),
            ]),
        );
        seed.intentions_mut().insert(
            active_intention.intention_id.clone(),
            active_intention.clone(),
        );
        seed.active_intention_by_actor_mut()
            .insert(actor_id.clone(), active_intention.intention_id.clone());
        for execution in executions {
            seed.routine_executions_mut()
                .insert(execution.execution_id.clone(), execution);
        }
        let agent_state = seed.build();
        let actor_known_context = surface.build(&agent_state).into_context();
        assert_eq!(actor_known_context.current_place_id(), &current_place_id);
        Self {
            actor_id,
            decision_tick,
            agent_state,
            actor_known_context,
        }
    }

    pub fn run_embodied(&self) -> RoutineParityOutcome {
        run_transaction(self, embodied_family_for_fixture(self))
    }

    pub fn run_autonomous(&self) -> RoutineParityOutcome {
        run_transaction(self, autonomous_family_for_fixture(self))
    }
}

pub fn parity_surface(current_place_id: PlaceId) -> NoHumanActorKnownSurfaceBuilder {
    NoHumanActorKnownSurfaceBuilder::new(
        ActorId::new("actor_parity").unwrap(),
        current_place_id,
        Some(SimTick::new(8)),
    )
}

pub fn parity_execution(
    execution_id: &str,
    actor_id: ActorId,
    template_id: &str,
    family: RoutineFamily,
    status: RoutineStepStatus,
    start_tick: SimTick,
) -> RoutineExecution {
    let mut execution = RoutineExecution::new(
        RoutineExecutionId::new(execution_id).unwrap(),
        actor_id,
        RoutineTemplateId::new(template_id).unwrap(),
        family,
        start_tick,
        Some(start_tick.advance_by(1)),
        Some(start_tick.advance_by(8)),
        None,
        DecisionTraceId::new(format!("trace_{execution_id}")).unwrap(),
    );
    execution.step_status = status;
    execution
}

pub fn test_event_id(value: &str) -> EventId {
    EventId::new(value).unwrap()
}

fn run_transaction(
    fixture: &RoutineParityFixture,
    routine_window_family: Option<RoutineFamily>,
) -> RoutineParityOutcome {
    match ActorDecisionTransaction::run(ActorDecisionTransactionInput {
        actor_id: fixture.actor_id.clone(),
        decision_tick: fixture.decision_tick,
        agent_state: &fixture.agent_state,
        actor_known_context: &fixture.actor_known_context,
        source_event_ids: None,
        source_event_kinds: None,
        routine_window_family,
        include_idle_fallback: false,
    }) {
        ActorDecisionTransactionOutcome::Proposed(proposed) => {
            RoutineParityOutcome::Proposed(shape_from_proposed(&proposed))
        }
        ActorDecisionTransactionOutcome::Stuck { diagnostic } => RoutineParityOutcome::Stuck {
            concrete_blocker: diagnostic.concrete_blocker,
            hidden_truth_referenced: diagnostic.typed_diagnostic.hidden_truth_referenced,
        },
    }
}

fn shape_from_proposed(proposed: &ActorDecisionProposalOutcome) -> RoutineParityProposalShape {
    let proposal = proposed.proposal.clone().into_proposal();
    RoutineParityProposalShape {
        action_id: proposal.action_id.as_str().to_string(),
        actor_id: proposal.actor_id.clone(),
        target_ids: proposal.target_ids.clone(),
        routine_template_id: proposal.parameters.get("routine_template_id").cloned(),
        routine_execution_id: proposal.parameters.get("routine_execution_id").cloned(),
        local_plan_id_present: proposed.decision_trace_record.local_plan_id.is_some()
            && proposal.parameters.contains_key("local_plan_id"),
        proposal_ancestry_len: proposed.selected_goal_bundle.proposal_ancestry.len(),
        decision_outcome: proposed.decision_trace.outcome,
        hidden_truth_actor_known_only: proposed
            .decision_trace
            .hidden_truth_audit_result
            .actor_known_only,
    }
}

fn embodied_family_for_fixture(fixture: &RoutineParityFixture) -> Option<RoutineFamily> {
    let active_intention_id = fixture
        .agent_state
        .active_intention_by_actor()
        .get(&fixture.actor_id)?;
    let active = fixture.agent_state.intentions().get(active_intention_id)?;
    let selected_method = active.selected_routine_method.as_ref()?;
    let family = fixture
        .agent_state
        .routine_executions()
        .values()
        .filter(|execution| execution.actor_id == fixture.actor_id)
        .filter(|execution| &execution.template_id == selected_method)
        .filter(|execution| !execution.step_status.is_resolved())
        .map(|execution| execution.family)
        .next()
        .or_else(|| routine_family_from_template_id(selected_method.as_str()))?;
    refine_work_block_family(family, &fixture.actor_known_context)
}

fn autonomous_family_for_fixture(fixture: &RoutineParityFixture) -> Option<RoutineFamily> {
    let family = fixture
        .agent_state
        .routine_executions()
        .values()
        .filter(|execution| execution.actor_id == fixture.actor_id)
        .filter(|execution| execution.start_tick <= fixture.decision_tick)
        .filter(|execution| {
            execution
                .deadline_tick
                .is_none_or(|deadline| fixture.decision_tick < deadline)
        })
        .filter(|execution| !execution.step_status.is_resolved())
        .min_by(|left, right| {
            left.start_tick
                .cmp(&right.start_tick)
                .then_with(|| left.execution_id.cmp(&right.execution_id))
        })
        .map(|execution| execution.family)?;
    refine_work_block_family(family, &fixture.actor_known_context)
}

fn refine_work_block_family(
    family: RoutineFamily,
    context: &ActorKnownPlanningContext,
) -> Option<RoutineFamily> {
    if family == RoutineFamily::WorkBlock
        && !context
            .known_workplaces()
            .values()
            .any(|place_id| place_id == context.current_place_id())
    {
        Some(RoutineFamily::GoToWork)
    } else {
        Some(family)
    }
}

fn routine_family_from_template_id(template_id: &str) -> Option<RoutineFamily> {
    if template_id.contains("go_to_work") {
        Some(RoutineFamily::GoToWork)
    } else if template_id.contains("work_block") {
        Some(RoutineFamily::WorkBlock)
    } else if template_id.contains("eat") {
        Some(RoutineFamily::EatMeal)
    } else if template_id.contains("sleep") {
        Some(RoutineFamily::SleepNight)
    } else if template_id.contains("return_home") {
        Some(RoutineFamily::ReturnHome)
    } else if template_id.contains("find_food") {
        Some(RoutineFamily::FindFood)
    } else if template_id.contains("leave_unsafe_place") {
        Some(RoutineFamily::LeaveUnsafePlace)
    } else {
        None
    }
}
