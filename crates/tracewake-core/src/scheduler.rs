use crate::agent::NeedKind;
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
use crate::ids::{ActionId, ActorId, ContentManifestId, ControllerId, EventId, ProcessId};
use crate::time::{passive_awake_need_deltas, SimTick};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchedulePhase {
    HumanCommand,
    NoHumanProcess,
    DeferredProcess,
    Replay,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchedulerSourceId {
    Actor(ActorId),
    Controller(ControllerId),
    Process(ProcessId),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProposalSequence(u64);

impl ProposalSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProposalSequenceAssigner {
    next: u64,
}

impl ProposalSequenceAssigner {
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    pub fn assign_next(&mut self) -> ProposalSequence {
        let assigned = ProposalSequence::new(self.next);
        self.next += 1;
        assigned
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderingKey {
    pub sim_tick: SimTick,
    pub phase: SchedulePhase,
    pub source_id: SchedulerSourceId,
    pub proposal_sequence: ProposalSequence,
    pub action_id: ActionId,
    pub target_ids: Vec<String>,
    pub final_tie_breaker: String,
}

impl OrderingKey {
    pub fn new(
        sim_tick: SimTick,
        phase: SchedulePhase,
        source_id: SchedulerSourceId,
        proposal_sequence: ProposalSequence,
        action_id: ActionId,
        target_ids: Vec<String>,
        final_tie_breaker: impl Into<String>,
    ) -> Self {
        Self {
            sim_tick,
            phase,
            source_id,
            proposal_sequence,
            action_id,
            target_ids,
            final_tie_breaker: final_tie_breaker.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scheduled<T> {
    pub ordering_key: OrderingKey,
    pub payload: T,
}

pub fn sort_scheduled<T>(scheduled: &mut [Scheduled<T>]) {
    scheduled.sort_by(|left, right| left.ordering_key.cmp(&right.ordering_key));
}

pub fn build_passive_need_delta_events(
    actor_ids: impl IntoIterator<Item = ActorId>,
    process_id: &ProcessId,
    start_tick: SimTick,
    elapsed_ticks: u64,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    let deltas = passive_awake_need_deltas(elapsed_ticks);
    actor_ids
        .into_iter()
        .flat_map(|actor_id| {
            [
                (NeedKind::Hunger, deltas.hunger_delta),
                (NeedKind::Fatigue, deltas.fatigue_delta),
            ]
            .into_iter()
            .map(move |(need_kind, delta)| {
                let mut event = EventEnvelope::new_caused_v1(
                    EventId::new(format!(
                        "event.passive_need_delta.{}.{}.{}.{}",
                        need_kind.stable_id(),
                        actor_id.as_str(),
                        start_tick.value(),
                        elapsed_ticks
                    ))
                    .unwrap(),
                    EventKind::NeedDeltaApplied,
                    0,
                    0,
                    start_tick.advance_by(elapsed_ticks),
                    OrderingKey::new(
                        start_tick.advance_by(elapsed_ticks),
                        SchedulePhase::NoHumanProcess,
                        SchedulerSourceId::Process(process_id.clone()),
                        ProposalSequence::new(0),
                        ActionId::new("passive_need_delta").unwrap(),
                        vec![
                            actor_id.as_str().to_string(),
                            need_kind.stable_id().to_string(),
                        ],
                        format!("{}:{}", actor_id.as_str(), need_kind.stable_id()),
                    ),
                    content_manifest_id.clone(),
                    vec![EventCause::Process(process_id.clone())],
                )
                .unwrap();
                event.actor_id = Some(actor_id.clone());
                event.process_id = Some(process_id.clone());
                event.participants = vec![actor_id.to_string()];
                event.payload = vec![
                    PayloadField::new("actor_id", actor_id.as_str()),
                    PayloadField::new("need_kind", need_kind.stable_id()),
                    PayloadField::new("delta", delta.to_string()),
                    PayloadField::new("elapsed_ticks", elapsed_ticks.to_string()),
                    PayloadField::new("cause_kind", "tick_delta"),
                ];
                event.effects_summary = format!(
                    "{} rose by {} over {} elapsed ticks",
                    need_kind.stable_id(),
                    delta,
                    elapsed_ticks
                );
                event
            })
        })
        .collect()
}

pub fn duration_completion_ordering_key(
    actor_id: &ActorId,
    action_id: &ActionId,
    completion_tick: SimTick,
    sequence: u64,
) -> OrderingKey {
    OrderingKey::new(
        completion_tick,
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(sequence),
        action_id.clone(),
        vec![actor_id.as_str().to_string()],
        format!("duration_completion:{}", actor_id.as_str()),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeterministicScheduler {
    pub current_tick: SimTick,
    proposal_sequences: ProposalSequenceAssigner,
}

impl DeterministicScheduler {
    pub const fn new(current_tick: SimTick) -> Self {
        Self {
            current_tick,
            proposal_sequences: ProposalSequenceAssigner::new(),
        }
    }

    pub fn assign_proposal_sequence(&mut self) -> ProposalSequence {
        self.proposal_sequences.assign_next()
    }

    pub fn advance_one_tick(&mut self) -> SimTick {
        self.current_tick = self.current_tick.next();
        self.current_tick
    }
}

pub mod no_human {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::agent::{
        build_actor_known_planning_state, generate_candidate_goals_from_agent_state,
        plan_local_actions, select_goal_and_trace, select_phase3a_method, ActorKnownFact,
        ActorKnownPlanningState, BlockerCategory, DecisionInput, Intention, IntentionSource,
        LiveCandidateGenerationInput, LocalPlanRequest, NeedBand, NeedKind, NeedState, PlannerGoal,
        RoutineFamily, StuckDiagnostic, StuckResultingStatus, VisibleLocalPlanningState,
    };
    use crate::epistemics::EpistemicProjection;
    use crate::events::apply::apply_agent_event;
    use crate::events::log::EventLog;
    use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
    use crate::ids::{
        ActionId, ActorId, CandidateGoalId, ContentManifestId, DecisionTraceId, EventId,
        IntentionId, ProcessId, ProposalId, RoutineExecutionId, SemanticActionId,
        StuckDiagnosticId,
    };
    use crate::location::Location;
    use crate::scheduler::{
        DeterministicScheduler, OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use crate::state::{AgentState, PhysicalState};
    use crate::time::SimTick;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NoHumanAdvanceReport {
        pub start_tick: SimTick,
        pub final_tick: SimTick,
        pub tick_count: u64,
        pub marker_event_ids: Vec<EventId>,
        pub ordinary_pipeline_events: usize,
    }

    pub struct NoHumanStateMut<'a> {
        pub physical: &'a mut PhysicalState,
        pub agent: &'a mut AgentState,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DayWindow {
        pub window_id: String,
        pub start_tick: SimTick,
        pub end_tick: SimTick,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NoHumanDayConfig {
        pub actor_ids: Vec<ActorId>,
        pub windows: Vec<DayWindow>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NoHumanDayReport {
        pub start_tick: SimTick,
        pub final_tick: SimTick,
        pub actor_decision_order: Vec<ActorId>,
        pub window_ids: Vec<String>,
        pub marker_event_ids: Vec<EventId>,
        pub ordinary_pipeline_events: usize,
        pub stuck_diagnostic_event_ids: Vec<EventId>,
    }

    pub fn default_day_windows(start_tick: SimTick) -> Vec<DayWindow> {
        [
            ("pre_dawn", 0, 4),
            ("morning", 4, 10),
            ("work_window", 10, 18),
            ("evening", 18, 24),
            ("night", 24, 32),
        ]
        .into_iter()
        .map(|(window_id, start_offset, end_offset)| DayWindow {
            window_id: window_id.to_string(),
            start_tick: start_tick.advance_by(start_offset),
            end_tick: start_tick.advance_by(end_offset),
        })
        .collect()
    }

    pub fn run_no_human_day(
        state: &mut PhysicalState,
        agent_state: &mut AgentState,
        log: &mut EventLog,
        registry: &ActionRegistry,
        content_manifest_id: ContentManifestId,
        mut config: NoHumanDayConfig,
    ) -> NoHumanDayReport {
        config.windows.sort_by(|left, right| {
            (left.start_tick, left.end_tick, &left.window_id).cmp(&(
                right.start_tick,
                right.end_tick,
                &right.window_id,
            ))
        });
        if config.actor_ids.is_empty() {
            config
                .actor_ids
                .extend(state.actors.keys().cloned().collect::<Vec<_>>());
        }
        config.actor_ids.sort();
        config.actor_ids.dedup();

        let process_id = ProcessId::new("no_human_day").unwrap();
        let start_tick = config
            .windows
            .first()
            .map(|window| window.start_tick)
            .unwrap_or(SimTick::ZERO);
        let final_tick = config
            .windows
            .last()
            .map(|window| window.end_tick)
            .unwrap_or(start_tick);
        let mut scheduler = DeterministicScheduler::new(start_tick);
        let started = append_marker(
            log,
            EventKind::NoHumanDayStarted,
            &process_id,
            start_tick,
            0,
            final_tick.value().saturating_sub(start_tick.value()),
            content_manifest_id.clone(),
        );

        let mut ordinary_pipeline_events = 0;
        let mut progress_by_window_actor: BTreeMap<(String, ActorId), usize> = BTreeMap::new();
        let mut last_decision_tick_by_actor = config
            .actor_ids
            .iter()
            .cloned()
            .map(|actor_id| (actor_id, SimTick::ZERO))
            .collect::<BTreeMap<_, _>>();
        for window in &config.windows {
            for actor_id in &config.actor_ids {
                if !state.actors.contains_key(actor_id) {
                    continue;
                }
                let previous_decision_tick = last_decision_tick_by_actor
                    .get(actor_id)
                    .copied()
                    .unwrap_or(start_tick);
                let elapsed_ticks = window
                    .start_tick
                    .value()
                    .saturating_sub(previous_decision_tick.value());
                append_passive_need_events_before_decision(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    elapsed_ticks,
                    &content_manifest_id,
                );
                last_decision_tick_by_actor.insert(actor_id.clone(), window.start_tick);
                let Some(proposal) = build_agent_proposal(
                    state,
                    agent_state,
                    actor_id,
                    window,
                    registry,
                    &content_manifest_id,
                ) else {
                    continue;
                };
                let active_before_proposal = active_intention_for_actor(agent_state, actor_id);
                let ordering_key = OrderingKey::new(
                    window.start_tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id.clone()),
                    scheduler.assign_proposal_sequence(),
                    proposal.action_id.clone(),
                    proposal.target_ids.clone(),
                    format!("{}:{}", window.window_id, actor_id.as_str()),
                );
                let mut context = PipelineContext {
                    registry,
                    state,
                    agent_state,
                    log,
                    controller_bindings: None,
                    epistemic_projection: None,
                    content_manifest_id: content_manifest_id.clone(),
                    ordering_key,
                };
                let result = run_pipeline(&mut context, &proposal);
                let progress_events = no_human_progress_event_count(&result.appended_events);
                ordinary_pipeline_events += progress_events;
                if progress_events > 0 {
                    progress_by_window_actor.insert(
                        (window.window_id.clone(), actor_id.clone()),
                        progress_events,
                    );
                }
                append_intention_lifecycle_after_proposal(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &proposal,
                    &content_manifest_id,
                    active_before_proposal.as_ref(),
                    result.appended_events.first(),
                );
                append_routine_step_events_after_proposal(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &proposal,
                    &content_manifest_id,
                    result.appended_events.first(),
                );
            }
        }

        let mut stuck_diagnostic_event_ids = Vec::new();
        for window in &config.windows {
            for actor_id in &config.actor_ids {
                if !state.actors.contains_key(actor_id) {
                    continue;
                }
                if !progress_by_window_actor
                    .contains_key(&(window.window_id.clone(), actor_id.clone()))
                {
                    let event = append_stuck_diagnostic(
                        log,
                        &process_id,
                        actor_id,
                        window,
                        &content_manifest_id,
                    );
                    stuck_diagnostic_event_ids.push(event.event_id);
                }
            }
        }

        let completed = append_marker(
            log,
            EventKind::NoHumanDayCompleted,
            &process_id,
            final_tick,
            1,
            final_tick.value().saturating_sub(start_tick.value()),
            content_manifest_id,
        );

        NoHumanDayReport {
            start_tick,
            final_tick,
            actor_decision_order: config.actor_ids,
            window_ids: config
                .windows
                .iter()
                .map(|window| window.window_id.clone())
                .collect(),
            marker_event_ids: vec![started.event_id, completed.event_id],
            ordinary_pipeline_events,
            stuck_diagnostic_event_ids,
        }
    }

    fn build_agent_proposal(
        state: &PhysicalState,
        agent_state: &AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
        content_manifest_id: &ContentManifestId,
    ) -> Option<Proposal> {
        let actor = state.actors.get(actor_id)?;
        let epistemic_projection = EpistemicProjection::new(content_manifest_id.clone());
        let visible_local = visible_local_planning_state(state, actor_id, &actor.current_place_id);
        let mut actor_known_state = build_actor_known_planning_state(
            actor_id,
            &epistemic_projection,
            agent_state,
            &visible_local,
        );
        let wait_reason_fact = ActorKnownFact::modeled(
            "modeled_wait_reason",
            format!("window_id={}:bounded_idle", window.window_id),
        );
        let reevaluation_fact = ActorKnownFact::modeled(
            "reevaluation_window_known",
            format!(
                "window_id={}:{}..{}",
                window.window_id,
                window.start_tick.value(),
                window.end_tick.value()
            ),
        );
        actor_known_state
            .actor_known_facts
            .extend([wait_reason_fact, reevaluation_fact]);
        actor_known_state.actor_known_facts.sort_by(|left, right| {
            left.stable_id
                .cmp(&right.stable_id)
                .then_with(|| left.proof_note().cmp(&right.proof_note()))
        });
        actor_known_state.proof_sources = actor_known_state
            .actor_known_facts
            .iter()
            .map(ActorKnownFact::proof_note)
            .collect();
        let actor_known_facts = actor_known_state.actor_known_facts.clone();
        if let Some(proposal) = build_routine_or_need_proposal(
            &actor_known_state,
            agent_state,
            actor_id,
            window,
            registry,
        ) {
            return Some(proposal);
        }
        registry.get(&ActionId::new("wait").unwrap())?;
        let active_intention = active_intention_for_actor(agent_state, actor_id);
        let generated = generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput {
            actor_id: actor_id.clone(),
            decision_tick: window.start_tick,
            agent_state,
            active_intention: active_intention.clone(),
            actor_known_facts: actor_known_facts.clone(),
            routine_window_goal: None,
        });
        let candidate_fallbacks = generated.candidates.clone();
        let selection = select_goal_and_trace(DecisionInput {
            actor_id: actor_id.clone(),
            decision_tick: window.start_tick,
            candidates: generated.candidates,
            active_intention,
            actor_known_inputs: generated.actor_known_inputs_used,
        })?;
        let method = select_phase3a_method(
            &selection.selected_goal,
            &actor_known_state,
            &actor_known_facts,
            window.start_tick,
        )
        .or_else(|_| {
            candidate_fallbacks
                .iter()
                .find(|candidate| candidate.goal_kind == crate::agent::GoalKind::IdleWithReason)
                .ok_or(crate::agent::MethodSelectionFailure::NoApplicableMethod {
                    family: crate::agent::RoutineFamily::Wait,
                    reason: "idle fallback candidate missing".to_string(),
                })
                .and_then(|candidate| {
                    select_phase3a_method(
                        candidate,
                        &actor_known_state,
                        &actor_known_facts,
                        window.start_tick,
                    )
                })
        })
        .ok()?;
        let wait_reason = format!("no_human_day:{}", window.window_id);
        let plan = plan_local_actions(
            &actor_known_state,
            &LocalPlanRequest {
                routine_step: method.template.steps.first().cloned().unwrap_or(
                    crate::agent::RoutineStep::WaitUntil {
                        reason: wait_reason.clone(),
                    },
                ),
                goal: PlannerGoal::WaitWithReason(wait_reason.clone()),
                budget: 1,
                actor_known_facts,
            },
        )
        .ok()?;
        let planned = plan.proposals.first()?;
        let mut proposal = Proposal::new(
            ProposalId::new(format!(
                "proposal_no_human_day_{}_{}_wait",
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            ProposalOrigin::Agent,
            Some(actor_id.clone()),
            planned.action_id.clone(),
            window.start_tick,
        );
        proposal
            .parameters
            .insert("reason".to_string(), wait_reason);
        Some(proposal)
    }

    fn build_routine_or_need_proposal(
        actor_known_state: &ActorKnownPlanningState,
        agent_state: &AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
    ) -> Option<Proposal> {
        let family = agent_state
            .routine_executions
            .values()
            .filter(|execution| &execution.actor_id == actor_id)
            .filter(|execution| execution.start_tick <= window.end_tick)
            .min_by(|left, right| left.start_tick.cmp(&right.start_tick))
            .map(|execution| execution.family);
        if let Some(family) = family {
            match family {
                RoutineFamily::SleepNight => {
                    return sleep_proposal(actor_known_state, actor_id, window, registry);
                }
                RoutineFamily::EatMeal | RoutineFamily::FindFood => {
                    return eat_proposal(actor_known_state, actor_id, window, registry);
                }
                RoutineFamily::GoToWork | RoutineFamily::WorkBlock => {
                    return work_or_move_proposal(
                        actor_known_state,
                        agent_state,
                        actor_id,
                        window,
                        registry,
                    );
                }
                RoutineFamily::MorningWake
                | RoutineFamily::ReturnHome
                | RoutineFamily::ContinueCurrentIntention
                | RoutineFamily::Wait
                | RoutineFamily::IdleWithReason => {}
            }
        }

        let needs = agent_state.needs_by_actor.get(actor_id);
        let hunger = needs
            .and_then(|needs| needs.get(&crate::agent::NeedKind::Hunger))
            .map_or(0, crate::agent::NeedState::value);
        let fatigue = needs
            .and_then(|needs| needs.get(&crate::agent::NeedKind::Fatigue))
            .map_or(0, crate::agent::NeedState::value);
        if hunger >= 500 {
            if let Some(proposal) = eat_proposal(actor_known_state, actor_id, window, registry) {
                return Some(proposal);
            }
        }
        if fatigue >= 500 {
            if let Some(proposal) = sleep_proposal(actor_known_state, actor_id, window, registry) {
                return Some(proposal);
            }
        }
        None
    }

    fn eat_proposal(
        actor_known_state: &ActorKnownPlanningState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
    ) -> Option<Proposal> {
        registry.get(&ActionId::new("eat").unwrap())?;
        let food = actor_known_state.known_food_sources.iter().next()?;
        let mut proposal =
            ordinary_proposal("eat", actor_id, window, ActionId::new("eat").unwrap());
        proposal.target_ids.push(food.clone());
        Some(proposal)
    }

    fn sleep_proposal(
        actor_known_state: &ActorKnownPlanningState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
    ) -> Option<Proposal> {
        registry.get(&ActionId::new("sleep").unwrap())?;
        let sleep_place = actor_known_state
            .known_sleep_places
            .iter()
            .next()
            .unwrap_or(&actor_known_state.current_place_id);
        let mut proposal =
            ordinary_proposal("sleep", actor_id, window, ActionId::new("sleep").unwrap());
        proposal.target_ids.push(sleep_place.as_str().to_string());
        proposal.parameters.insert(
            "sleep_place_id".to_string(),
            sleep_place.as_str().to_string(),
        );
        Some(proposal)
    }

    fn work_or_move_proposal(
        actor_known_state: &ActorKnownPlanningState,
        agent_state: &AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
    ) -> Option<Proposal> {
        let (workplace_id, workplace_place_id) =
            actor_known_state.known_workplaces.iter().next()?;
        if actor_known_state.current_place_id != *workplace_place_id {
            registry.get(&ActionId::new("move").unwrap())?;
            let destination = if actor_known_state
                .known_edges
                .get(&actor_known_state.current_place_id)
                .is_some_and(|edges| edges.contains(workplace_place_id))
            {
                workplace_place_id.as_str().to_string()
            } else {
                actor_known_state
                    .known_edges
                    .get(&actor_known_state.current_place_id)?
                    .iter()
                    .next()
                    .map(|place_id| place_id.as_str().to_string())?
            };
            let mut proposal =
                ordinary_proposal("move", actor_id, window, ActionId::new("move").unwrap());
            proposal.target_ids.push(destination);
            return Some(proposal);
        }

        registry.get(&ActionId::new("work_block").unwrap())?;
        let mut proposal = ordinary_proposal(
            "work",
            actor_id,
            window,
            ActionId::new("work_block").unwrap(),
        );
        proposal.target_ids.push(workplace_id.as_str().to_string());
        let needs = agent_state.needs_by_actor.get(actor_id);
        proposal.parameters.insert(
            "current_fatigue".to_string(),
            needs
                .and_then(|needs| needs.get(&crate::agent::NeedKind::Fatigue))
                .map_or(0, crate::agent::NeedState::value)
                .to_string(),
        );
        proposal.parameters.insert(
            "current_hunger".to_string(),
            needs
                .and_then(|needs| needs.get(&crate::agent::NeedKind::Hunger))
                .map_or(0, crate::agent::NeedState::value)
                .to_string(),
        );
        Some(proposal)
    }

    fn visible_local_planning_state(
        state: &PhysicalState,
        actor_id: &ActorId,
        current_place_id: &crate::ids::PlaceId,
    ) -> VisibleLocalPlanningState {
        let mut visible_edges = BTreeMap::new();
        if let Some(current_place) = state.places.get(current_place_id) {
            visible_edges.insert(
                current_place_id.clone(),
                current_place.adjacent_place_ids.clone(),
            );
        }
        let visible_food_sources = state
            .food_supplies
            .values()
            .filter(|food| matches!(&food.location, Location::AtPlace(place_id) if place_id == current_place_id))
            .map(|food| food.food_supply_id.as_str().to_string())
            .collect::<BTreeSet<_>>();
        let visible_workplaces = state
            .workplaces
            .iter()
            .filter(|(_, workplace)| workplace.assigned_actor_ids.contains(actor_id))
            .map(|(workplace_id, workplace)| (workplace_id.clone(), workplace.place_id.clone()))
            .collect::<BTreeMap<_, _>>();
        VisibleLocalPlanningState {
            current_place_id: current_place_id.clone(),
            visible_edges,
            visible_closed_doors: BTreeMap::new(),
            visible_containers_by_place: BTreeMap::new(),
            visible_food_sources,
            visible_sleep_places: BTreeSet::from([current_place_id.clone()]),
            visible_workplaces,
        }
    }

    fn active_intention_for_actor(
        agent_state: &AgentState,
        actor_id: &ActorId,
    ) -> Option<Intention> {
        let intention_id = agent_state.active_intention_by_actor.get(actor_id)?;
        agent_state.intentions.get(intention_id).cloned()
    }

    fn append_passive_need_events_before_decision(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        elapsed_ticks: u64,
        content_manifest_id: &ContentManifestId,
    ) {
        if elapsed_ticks == 0 {
            return;
        }
        let deltas = crate::time::passive_awake_need_deltas(elapsed_ticks);
        for (need_kind, delta) in [
            (NeedKind::Hunger, deltas.hunger_delta),
            (NeedKind::Fatigue, deltas.fatigue_delta),
        ] {
            let Some(current_value) = agent_state
                .needs_by_actor
                .get(actor_id)
                .and_then(|needs| needs.get(&need_kind))
                .map(NeedState::value)
            else {
                continue;
            };
            let next_value = (i32::from(current_value) + delta).clamp(0, 1000) as u16;
            let crossing = NeedState::threshold_crossing(current_value, next_value);
            let delta_event = append_and_apply_agent_event(
                log,
                agent_state,
                build_window_passive_need_delta_event(
                    process_id,
                    actor_id,
                    window,
                    content_manifest_id,
                    need_kind,
                    delta,
                    elapsed_ticks,
                ),
            );
            if let Some(crossing) = crossing {
                let has_active_intention =
                    agent_state.active_intention_by_actor.contains_key(actor_id);
                append_and_apply_agent_event(
                    log,
                    agent_state,
                    build_window_need_threshold_event(
                        process_id,
                        actor_id,
                        window,
                        content_manifest_id,
                        &delta_event.event_id,
                        need_kind,
                        current_value,
                        next_value,
                        crossing.from,
                        crossing.to,
                        has_active_intention,
                    ),
                );
            }
        }
    }

    fn append_and_apply_agent_event(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        event: EventEnvelope,
    ) -> EventEnvelope {
        let appended = log.append(event).expect("agent event is appendable");
        apply_agent_event(agent_state, &appended).expect("agent event applies to live state");
        appended
    }

    #[allow(clippy::too_many_arguments)]
    fn build_window_passive_need_delta_event(
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
        need_kind: NeedKind,
        delta: i32,
        elapsed_ticks: u64,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.no_human_passive_need_delta.{}.{}.{}",
                window.window_id,
                actor_id.as_str(),
                need_kind.stable_id()
            ))
            .unwrap(),
            EventKind::NeedDeltaApplied,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("passive_need_delta").unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    window.window_id.clone(),
                    need_kind.stable_id().to_string(),
                ],
                format!(
                    "before_decision:{}:{}:{}",
                    window.window_id,
                    actor_id.as_str(),
                    need_kind.stable_id()
                ),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Process(process_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.participants = vec![actor_id.to_string()];
        event.payload = vec![
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("need_kind", need_kind.stable_id()),
            PayloadField::new("delta", delta.to_string()),
            PayloadField::new("elapsed_ticks", elapsed_ticks.to_string()),
            PayloadField::new("window_id", window.window_id.as_str()),
            PayloadField::new("cause_kind", "tick_delta"),
        ];
        event.effects_summary = format!(
            "{} changed by {} before {} decision",
            need_kind.stable_id(),
            delta,
            window.window_id
        );
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn build_window_need_threshold_event(
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
        delta_event_id: &EventId,
        need_kind: NeedKind,
        from_value: u16,
        to_value: u16,
        from_band: NeedBand,
        to_band: NeedBand,
        has_active_intention: bool,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.no_human_need_threshold.{}.{}.{}",
                window.window_id,
                actor_id.as_str(),
                need_kind.stable_id()
            ))
            .unwrap(),
            EventKind::NeedThresholdCrossed,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("need_threshold_crossed").unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    window.window_id.clone(),
                    need_kind.stable_id().to_string(),
                ],
                format!(
                    "before_decision_threshold:{}:{}:{}",
                    window.window_id,
                    actor_id.as_str(),
                    need_kind.stable_id()
                ),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(delta_event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.participants = vec![actor_id.to_string()];
        let severe_need_interrupts = has_active_intention && matches!(to_band, NeedBand::Severe);
        event.payload = vec![
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("need_kind", need_kind.stable_id()),
            PayloadField::new("from_value", from_value.to_string()),
            PayloadField::new("to_value", to_value.to_string()),
            PayloadField::new("from_band", from_band.stable_id()),
            PayloadField::new("to_band", to_band.stable_id()),
            PayloadField::new("window_id", window.window_id.as_str()),
            PayloadField::new("candidate_goal_reevaluation", "true"),
            PayloadField::new(
                "severe_need_interrupts_active_intention",
                severe_need_interrupts.to_string(),
            ),
            PayloadField::new(
                "interruption_cause",
                if severe_need_interrupts {
                    "severe_need_pressure"
                } else {
                    "none"
                },
            ),
        ];
        event.effects_summary = format!(
            "{} crossed {} to {} before {} decision",
            need_kind.stable_id(),
            from_band.stable_id(),
            to_band.stable_id(),
            window.window_id
        );
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn append_intention_lifecycle_after_proposal(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        active_before_proposal: Option<&Intention>,
        ordinary_event: Option<&EventEnvelope>,
    ) {
        let Some(ordinary_event) = ordinary_event else {
            return;
        };
        if let Some(active) = active_before_proposal {
            append_and_apply_agent_event(
                log,
                agent_state,
                build_intention_continued_event(
                    process_id,
                    active,
                    window,
                    proposal,
                    content_manifest_id,
                    ordinary_event,
                ),
            );
            return;
        }
        if agent_state.active_intention_by_actor.contains_key(actor_id) {
            return;
        }
        append_and_apply_agent_event(
            log,
            agent_state,
            build_intention_started_event(
                process_id,
                actor_id,
                window,
                proposal,
                content_manifest_id,
                ordinary_event,
            ),
        );
    }

    fn build_intention_started_event(
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let action = proposal.action_id.as_str();
        let intention_id = IntentionId::new(format!(
            "intention_{}_{}_{}",
            actor_id.as_str(),
            window.window_id,
            action
        ))
        .unwrap();
        let selected_goal_id = CandidateGoalId::new(format!(
            "goal_{}_{}_{}",
            actor_id.as_str(),
            window.start_tick.value(),
            action
        ))
        .unwrap();
        let trace_id = DecisionTraceId::new(format!(
            "trace_{}_{}_{}",
            actor_id.as_str(),
            window.start_tick.value(),
            action
        ))
        .unwrap();
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.intention_started.{}.{}",
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            EventKind::IntentionStarted,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("intention_started").unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    intention_id.as_str().to_string(),
                ],
                format!(
                    "intention_started:{}:{}",
                    actor_id.as_str(),
                    window.window_id
                ),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(ordinary_event.event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.proposal_id = Some(proposal.proposal_id.clone());
        event.participants = vec![actor_id.to_string(), intention_id.to_string()];
        event.payload = vec![
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("intention_id", intention_id.as_str()),
            PayloadField::new("status", "active"),
            PayloadField::new(
                "source",
                IntentionSource::CandidateGoalSelection.stable_id(),
            ),
            PayloadField::new("selected_goal_id", selected_goal_id.as_str()),
            PayloadField::new("selected_routine_method", ""),
            PayloadField::new("current_step", action),
            PayloadField::new("durability_level", "5"),
            PayloadField::new("start_tick", window.start_tick.value().to_string()),
            PayloadField::new("trace_id", trace_id.as_str()),
            PayloadField::new("follow_on_action_id", proposal.action_id.as_str()),
            PayloadField::new("follow_on_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new("window_id", window.window_id.as_str()),
            PayloadField::new("reason", "no_human_selected_goal"),
        ];
        event.effects_summary = "no-human actor adopted a durable intention".to_string();
        event
    }

    fn build_intention_continued_event(
        process_id: &ProcessId,
        active: &Intention,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.intention_continued.{}.{}",
                active.actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            EventKind::IntentionContinued,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("intention_continued").unwrap(),
                vec![
                    active.actor_id.as_str().to_string(),
                    active.intention_id.as_str().to_string(),
                ],
                format!(
                    "intention_continued:{}:{}",
                    active.actor_id.as_str(),
                    window.window_id
                ),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(ordinary_event.event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(active.actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.proposal_id = Some(proposal.proposal_id.clone());
        event.participants = vec![active.actor_id.to_string(), active.intention_id.to_string()];
        event.payload = vec![
            PayloadField::new("actor_id", active.actor_id.as_str()),
            PayloadField::new("intention_id", active.intention_id.as_str()),
            PayloadField::new("status", "active"),
            PayloadField::new("reason", "ordinary_follow_on_action_committed"),
            PayloadField::new("progress_tick", window.start_tick.value().to_string()),
            PayloadField::new("current_step", proposal.action_id.as_str()),
            PayloadField::new("follow_on_action_id", proposal.action_id.as_str()),
            PayloadField::new("follow_on_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new("window_id", window.window_id.as_str()),
        ];
        event.effects_summary = "active intention continued through ordinary action".to_string();
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn append_routine_step_events_after_proposal(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: Option<&EventEnvelope>,
    ) {
        let Some(ordinary_event) = ordinary_event else {
            return;
        };
        let Some(execution_id) = active_routine_execution_for_actor(agent_state, actor_id) else {
            return;
        };
        let Some(execution) = agent_state.routine_executions.get(&execution_id) else {
            return;
        };
        if matches!(
            execution.step_status,
            crate::agent::RoutineStepStatus::Completed
                | crate::agent::RoutineStepStatus::Failed
                | crate::agent::RoutineStepStatus::Interrupted
                | crate::agent::RoutineStepStatus::Abandoned
        ) {
            return;
        }
        if is_routine_failure_event(ordinary_event) {
            append_and_apply_agent_event(
                log,
                agent_state,
                build_routine_step_failed_event(
                    process_id,
                    &execution_id,
                    actor_id,
                    window,
                    proposal,
                    content_manifest_id,
                    ordinary_event,
                ),
            );
            return;
        }
        if execution.step_status == crate::agent::RoutineStepStatus::NotStarted {
            append_and_apply_agent_event(
                log,
                agent_state,
                build_routine_step_started_event(
                    process_id,
                    &execution_id,
                    actor_id,
                    window,
                    proposal,
                    content_manifest_id,
                    ordinary_event,
                ),
            );
        }
        if is_instant_routine_progress_event(ordinary_event) {
            append_and_apply_agent_event(
                log,
                agent_state,
                build_routine_step_completed_event(
                    process_id,
                    &execution_id,
                    actor_id,
                    window,
                    proposal,
                    content_manifest_id,
                    ordinary_event,
                ),
            );
        }
    }

    fn active_routine_execution_for_actor(
        agent_state: &AgentState,
        actor_id: &ActorId,
    ) -> Option<RoutineExecutionId> {
        agent_state
            .routine_executions
            .iter()
            .filter(|(_, execution)| &execution.actor_id == actor_id)
            .filter(|(_, execution)| {
                !matches!(
                    execution.step_status,
                    crate::agent::RoutineStepStatus::Completed
                        | crate::agent::RoutineStepStatus::Failed
                        | crate::agent::RoutineStepStatus::Interrupted
                        | crate::agent::RoutineStepStatus::Abandoned
                )
            })
            .min_by(|(_, left), (_, right)| {
                left.start_tick
                    .cmp(&right.start_tick)
                    .then_with(|| left.execution_id.cmp(&right.execution_id))
            })
            .map(|(execution_id, _)| execution_id.clone())
    }

    fn is_routine_failure_event(event: &EventEnvelope) -> bool {
        matches!(
            event.event_type,
            EventKind::ActionRejected
                | EventKind::EatFailed
                | EventKind::WorkBlockFailed
                | EventKind::ContinueRoutineRejected
        )
    }

    fn is_instant_routine_progress_event(event: &EventEnvelope) -> bool {
        !matches!(
            event.event_type,
            EventKind::SleepStarted | EventKind::WorkBlockStarted
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build_routine_step_started_event(
        process_id: &ProcessId,
        execution_id: &RoutineExecutionId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let semantic_action_id = SemanticActionId::new(proposal.action_id.as_str()).unwrap();
        let mut event = routine_step_event(
            EventKind::RoutineStepStarted,
            process_id,
            execution_id,
            actor_id,
            window,
            proposal,
            content_manifest_id,
            ordinary_event,
            "routine_step_started",
        );
        event.payload = vec![
            PayloadField::new("routine_execution_id", execution_id.as_str()),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("action_id", semantic_action_id.as_str()),
            PayloadField::new("progress_tick", window.start_tick.value().to_string()),
            PayloadField::new("ordinary_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new("ordinary_action_id", proposal.action_id.as_str()),
        ];
        event.effects_summary = "routine step started with ordinary action ancestry".to_string();
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn build_routine_step_completed_event(
        process_id: &ProcessId,
        execution_id: &RoutineExecutionId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let mut event = routine_step_event(
            EventKind::RoutineStepCompleted,
            process_id,
            execution_id,
            actor_id,
            window,
            proposal,
            content_manifest_id,
            ordinary_event,
            "routine_step_completed",
        );
        event.payload = vec![
            PayloadField::new("routine_execution_id", execution_id.as_str()),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("progress_tick", window.start_tick.value().to_string()),
            PayloadField::new("ordinary_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new("ordinary_action_id", proposal.action_id.as_str()),
        ];
        event.effects_summary = "routine step completed with ordinary action ancestry".to_string();
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn build_routine_step_failed_event(
        process_id: &ProcessId,
        execution_id: &RoutineExecutionId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let mut event = routine_step_event(
            EventKind::RoutineStepFailed,
            process_id,
            execution_id,
            actor_id,
            window,
            proposal,
            content_manifest_id,
            ordinary_event,
            "routine_step_failed",
        );
        let reason = routine_failure_reason(ordinary_event);
        event.payload = vec![
            PayloadField::new("routine_execution_id", execution_id.as_str()),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("progress_tick", window.start_tick.value().to_string()),
            PayloadField::new("reason", reason),
            PayloadField::new("ordinary_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new("ordinary_action_id", proposal.action_id.as_str()),
            PayloadField::new("fallback_attempts", "1"),
        ];
        event.effects_summary = format!("routine step failed: {reason}");
        event
    }

    #[allow(clippy::too_many_arguments)]
    fn routine_step_event(
        kind: EventKind,
        process_id: &ProcessId,
        execution_id: &RoutineExecutionId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
        label: &str,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.{}.{}.{}",
                label,
                execution_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            kind,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new(label).unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    execution_id.as_str().to_string(),
                ],
                format!("{}:{}:{}", label, actor_id.as_str(), window.window_id),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(ordinary_event.event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.proposal_id = Some(proposal.proposal_id.clone());
        event.participants = vec![actor_id.to_string(), execution_id.to_string()];
        event
    }

    fn routine_failure_reason(event: &EventEnvelope) -> &'static str {
        match event.event_type {
            EventKind::ActionRejected => "action_rejected",
            EventKind::EatFailed => "eat_failed",
            EventKind::WorkBlockFailed => "work_block_failed",
            EventKind::ContinueRoutineRejected => "continue_routine_rejected",
            _ => "ordinary_action_failed",
        }
    }

    fn ordinary_proposal(
        label: &str,
        actor_id: &ActorId,
        window: &DayWindow,
        action_id: ActionId,
    ) -> Proposal {
        Proposal::new(
            ProposalId::new(format!(
                "proposal_no_human_day_{}_{}_{}",
                actor_id.as_str(),
                window.window_id,
                label
            ))
            .unwrap(),
            ProposalOrigin::Agent,
            Some(actor_id.clone()),
            action_id,
            window.start_tick,
        )
    }

    pub fn advance_no_human(
        state: NoHumanStateMut<'_>,
        log: &mut EventLog,
        registry: &ActionRegistry,
        content_manifest_id: ContentManifestId,
        start_tick: SimTick,
        tick_count: u64,
        scheduled_proposals: Vec<Proposal>,
    ) -> NoHumanAdvanceReport {
        let physical_state = state.physical;
        let agent_state = state.agent;
        let process_id = ProcessId::new("no_human_advance").unwrap();
        let mut scheduler = DeterministicScheduler::new(start_tick);
        let started = append_marker(
            log,
            EventKind::NoHumanAdvanceStarted,
            &process_id,
            scheduler.current_tick,
            0,
            tick_count,
            content_manifest_id.clone(),
        );

        let mut ordinary_pipeline_events = 0;
        let mut sorted = scheduled_proposals
            .into_iter()
            .map(|proposal| {
                let key = OrderingKey::new(
                    proposal.requested_tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Process(process_id.clone()),
                    scheduler.assign_proposal_sequence(),
                    proposal.action_id.clone(),
                    proposal.target_ids.clone(),
                    proposal.proposal_id.as_str().to_string(),
                );
                (key, proposal)
            })
            .collect::<Vec<_>>();
        sorted.sort_by(|left, right| left.0.cmp(&right.0));

        for (ordering_key, proposal) in sorted {
            let mut context = PipelineContext {
                registry,
                state: physical_state,
                agent_state,
                log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: content_manifest_id.clone(),
                ordering_key,
            };
            let result = run_pipeline(&mut context, &proposal);
            ordinary_pipeline_events += no_human_progress_event_count(&result.appended_events);
        }

        for _ in 0..tick_count {
            scheduler.advance_one_tick();
        }

        let completed = append_marker(
            log,
            EventKind::NoHumanAdvanceCompleted,
            &process_id,
            scheduler.current_tick,
            1,
            tick_count,
            content_manifest_id,
        );

        NoHumanAdvanceReport {
            start_tick,
            final_tick: scheduler.current_tick,
            tick_count,
            marker_event_ids: vec![started.event_id, completed.event_id],
            ordinary_pipeline_events,
        }
    }

    fn no_human_progress_event_count(events: &[EventEnvelope]) -> usize {
        events
            .iter()
            .filter(|event| is_no_human_progress_event(event))
            .count()
    }

    fn is_no_human_progress_event(event: &EventEnvelope) -> bool {
        if event.event_type != EventKind::ContinueRoutineProposed {
            return true;
        }
        event
            .payload
            .iter()
            .any(|field| field.key == "behavioral_progress" && field.value == "true")
    }

    fn append_marker(
        log: &mut EventLog,
        kind: EventKind,
        process_id: &ProcessId,
        tick: SimTick,
        sequence: u64,
        tick_count: u64,
        content_manifest_id: ContentManifestId,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_v1(
            EventId::new(format!(
                "event.{}.{}.{}",
                kind.stable_id(),
                process_id.as_str(),
                sequence
            ))
            .unwrap(),
            kind,
            0,
            0,
            tick,
            OrderingKey::new(
                tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(sequence),
                ActionId::new(kind.stable_id()).unwrap(),
                vec![tick_count.to_string()],
                "no_human_advance",
            ),
            content_manifest_id,
        );
        event.process_id = Some(process_id.clone());
        event.payload = vec![PayloadField::new("tick_count", tick_count.to_string())];
        if kind == EventKind::NoHumanDayCompleted {
            event.payload.push(PayloadField::new(
                "metrics_projection",
                "no_human_day_metrics_v1",
            ));
        }
        event.effects_summary = "no-human advance process marker".to_string();
        log.append(event).expect("no-human marker is versioned")
    }

    fn append_stuck_diagnostic(
        log: &mut EventLog,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
    ) -> EventEnvelope {
        let diagnostic = StuckDiagnostic::new(
            StuckDiagnosticId::new(format!(
                "diagnostic_no_human_day_{}_{}",
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            actor_id.clone(),
            window.start_tick,
            window.end_tick,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            BlockerCategory::SchedulingReservation,
            "no progress recorded in no-human day window",
            format!(
                "actor {} had no accepted or rejected ordinary proposal in {}",
                actor_id.as_str(),
                window.window_id
            ),
            "no-human day stuck detection",
            "recorded_stuck_diagnostic",
            StuckResultingStatus::Idle,
        );
        let canonical = diagnostic.serialize_canonical();
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.stuck_diagnostic_recorded.{}.{}",
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            EventKind::StuckDiagnosticRecorded,
            0,
            0,
            window.end_tick,
            OrderingKey::new(
                window.end_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Actor(actor_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("stuck_diagnostic_recorded").unwrap(),
                vec![actor_id.as_str().to_string(), window.window_id.clone()],
                format!("stuck:{}:{}", actor_id.as_str(), window.window_id),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Process(process_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.participants = vec![actor_id.to_string()];
        event.payload = vec![
            PayloadField::new("diagnostic_id", diagnostic.diagnostic_id.as_str()),
            PayloadField::new("diagnostic_canonical", canonical),
        ];
        event.effects_summary = "no-human day stuck diagnostic recorded".to_string();
        log.append(event).expect("stuck diagnostic is versioned")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::actions::proposal::{Proposal, ProposalOrigin};
        use crate::events::apply::apply_agent_event;
        use crate::events::apply::apply_event;
        use crate::events::{EventCause, EventStream};
        use crate::ids::{
            ActorId, CandidateGoalId, DecisionTraceId, FoodSupplyId, IntentionId, PlaceId,
            ProposalId, RoutineExecutionId, RoutineTemplateId, WorkplaceId,
        };
        use crate::state::{ActorBody, AgentState, FoodSupplyState, WorkplaceState};

        fn agent_state(actor_id: &ActorId) -> AgentState {
            let mut state = AgentState::default();
            state.needs_by_actor.insert(
                actor_id.clone(),
                [
                    (
                        crate::agent::NeedKind::Hunger,
                        crate::agent::NeedState::initial(
                            crate::agent::NeedKind::Hunger,
                            100,
                            crate::agent::NeedChangeCause::FixtureInitial,
                        ),
                    ),
                    (
                        crate::agent::NeedKind::Fatigue,
                        crate::agent::NeedState::initial(
                            crate::agent::NeedKind::Fatigue,
                            100,
                            crate::agent::NeedChangeCause::FixtureInitial,
                        ),
                    ),
                    (
                        crate::agent::NeedKind::Safety,
                        crate::agent::NeedState::initial(
                            crate::agent::NeedKind::Safety,
                            100,
                            crate::agent::NeedChangeCause::FixtureInitial,
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            );
            state
        }

        fn content_manifest_id() -> ContentManifestId {
            ContentManifestId::new("phase1_manifest").unwrap()
        }

        fn actor_id() -> ActorId {
            ActorId::new("actor_tomas").unwrap()
        }

        #[test]
        fn advance_runs_without_controller_and_marks_diagnostic_stream() {
            let mut state = PhysicalState::default();
            let mut log = EventLog::new();
            let registry = ActionRegistry::new();

            let report = advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: Box::leak(Box::new(crate::state::AgentState::default())),
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                2,
                Vec::new(),
            );

            assert_eq!(report.final_tick, SimTick::new(2));
            assert_eq!(log.events().len(), 2);
            assert!(log
                .events()
                .iter()
                .all(|event| event.stream == EventStream::Diagnostic));
        }

        #[test]
        fn diagnostic_markers_are_physical_noops_under_replay() {
            let mut state = PhysicalState::default();
            let before = state.clone();
            let mut log = EventLog::new();
            let registry = ActionRegistry::new();

            advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: Box::leak(Box::new(crate::state::AgentState::default())),
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                1,
                Vec::new(),
            );
            let mut replay = before.clone();
            for event in log.events() {
                apply_event(&mut replay, event).unwrap();
            }

            assert_eq!(replay, before);
        }

        #[test]
        fn scheduled_proposal_uses_shared_pipeline() {
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id(),
                ActorBody::new(actor_id(), crate::ids::PlaceId::new("shop_front").unwrap()),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();
            let mut agent_state = agent_state(&actor_id());
            let proposal = Proposal::new(
                ProposalId::new("proposal_wait").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id()),
                ActionId::new("wait").unwrap(),
                SimTick::ZERO,
            );

            let report = advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: &mut agent_state,
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                1,
                vec![proposal],
            );

            assert_eq!(report.ordinary_pipeline_events, 3);
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ActorWaited));
            assert_eq!(
                log.events()
                    .iter()
                    .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
                    .count(),
                2
            );
        }

        #[test]
        fn continue_routine_marker_only_is_not_ordinary_progress() {
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id(),
                ActorBody::new(actor_id(), crate::ids::PlaceId::new("shop_front").unwrap()),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_continue_routine();
            let mut agent_state = agent_state(&actor_id());
            let mut proposal = Proposal::new(
                ProposalId::new("proposal_continue_marker_only").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id()),
                ActionId::new("continue_routine").unwrap(),
                SimTick::ZERO,
            );
            proposal.parameters.insert(
                "active_intention_id".to_string(),
                "intention_workday".to_string(),
            );
            proposal
                .parameters
                .insert("next_action_id".to_string(), "wait".to_string());

            let report = advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: &mut agent_state,
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                1,
                vec![proposal],
            );

            assert_eq!(report.ordinary_pipeline_events, 0);
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ContinueRoutineProposed));
            assert!(!log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ActorWaited));
        }

        #[test]
        fn routine_dispatch_uses_family_when_template_id_has_no_magic_substring() {
            let actor_id = actor_id();
            let kitchen = PlaceId::new("kitchen").unwrap();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), kitchen.clone()),
            );
            state.food_supplies.insert(
                FoodSupplyId::new("meal_serving").unwrap(),
                FoodSupplyState {
                    food_supply_id: FoodSupplyId::new("meal_serving").unwrap(),
                    location: Location::AtPlace(kitchen.clone()),
                    servings: 1,
                    hunger_reduction_per_serving: 120,
                },
            );
            let mut agent_state = agent_state(&actor_id);
            agent_state.routine_executions.insert(
                RoutineExecutionId::new("routine_exec_midday").unwrap(),
                crate::agent::RoutineExecution::new(
                    RoutineExecutionId::new("routine_exec_midday").unwrap(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_midday").unwrap(),
                    RoutineFamily::EatMeal,
                    SimTick::ZERO,
                    Some(SimTick::new(1)),
                    Some(SimTick::new(4)),
                    None,
                    DecisionTraceId::new("trace_midday").unwrap(),
                ),
            );
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_eat();
            let actor_known_state = build_actor_known_planning_state(
                &actor_id,
                &EpistemicProjection::new(content_manifest_id()),
                &agent_state,
                &visible_local_planning_state(&state, &actor_id, &kitchen),
            );

            let proposal = build_routine_or_need_proposal(
                &actor_known_state,
                &agent_state,
                &actor_id,
                &DayWindow {
                    window_id: "midday".to_string(),
                    start_tick: SimTick::ZERO,
                    end_tick: SimTick::new(4),
                },
                &registry,
            )
            .expect("typed eat family should dispatch to eat proposal");

            assert_eq!(proposal.action_id.as_str(), "eat");
            assert_eq!(proposal.target_ids, ["meal_serving"]);
        }

        #[test]
        fn active_intention_lookup_returns_live_actor_intention() {
            let actor_id = actor_id();
            let mut agent_state = agent_state(&actor_id);
            let intention = crate::agent::Intention::adopt(
                IntentionId::new("intention_live_work").unwrap(),
                actor_id.clone(),
                crate::agent::IntentionSource::RoutineDuty,
                CandidateGoalId::new("goal_live_work").unwrap(),
                Some(RoutineTemplateId::new("routine_live_work").unwrap()),
                Some("work_block".to_string()),
                8,
                SimTick::ZERO,
                DecisionTraceId::new("trace_live_work").unwrap(),
            );
            agent_state
                .active_intention_by_actor
                .insert(actor_id.clone(), intention.intention_id.clone());
            agent_state
                .intentions
                .insert(intention.intention_id.clone(), intention.clone());

            assert_eq!(
                active_intention_for_actor(&agent_state, &actor_id),
                Some(intention)
            );
        }

        #[test]
        fn no_human_day_starts_continues_and_replays_active_intention() {
            let actor_id = actor_id();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), PlaceId::new("shop_front").unwrap()),
            );
            let mut agent_state = agent_state(&actor_id);
            let initial_agent_state = agent_state.clone();
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id.clone()],
                    windows: vec![
                        DayWindow {
                            window_id: "first".to_string(),
                            start_tick: SimTick::ZERO,
                            end_tick: SimTick::new(1),
                        },
                        DayWindow {
                            window_id: "second".to_string(),
                            start_tick: SimTick::new(1),
                            end_tick: SimTick::new(2),
                        },
                    ],
                },
            );

            let started = log
                .events()
                .iter()
                .find(|event| event.event_type == EventKind::IntentionStarted)
                .expect("first no-human action adopts intention");
            let continued = log
                .events()
                .iter()
                .find(|event| event.event_type == EventKind::IntentionContinued)
                .expect("later no-human action continues intention");
            assert!(continued
                .causes
                .iter()
                .any(|cause| matches!(cause, EventCause::Event(_))));
            assert!(continued
                .payload
                .iter()
                .any(|field| { field.key == "follow_on_action_id" && field.value == "wait" }));
            let intention_id = started
                .payload
                .iter()
                .find(|field| field.key == "intention_id")
                .map(|field| IntentionId::new(field.value.as_str()).unwrap())
                .unwrap();
            assert_eq!(
                agent_state.active_intention_by_actor.get(&actor_id),
                Some(&intention_id)
            );
            assert_eq!(
                agent_state.intentions[&intention_id]
                    .status_reason
                    .as_deref(),
                Some("ordinary_follow_on_action_committed")
            );

            let mut replay_agent_state = initial_agent_state;
            for event in log.events() {
                if matches!(
                    event.event_type,
                    EventKind::IntentionStarted | EventKind::IntentionContinued
                ) {
                    apply_agent_event(&mut replay_agent_state, event).unwrap();
                }
            }
            assert_eq!(replay_agent_state.intentions, agent_state.intentions);
            assert_eq!(
                replay_agent_state.active_intention_by_actor,
                agent_state.active_intention_by_actor
            );
        }

        #[test]
        fn no_human_day_records_routine_step_ancestry_and_replays_it() {
            let actor_id = actor_id();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), PlaceId::new("shop_front").unwrap()),
            );
            let mut agent_state = agent_state(&actor_id);
            let execution_id = RoutineExecutionId::new("routine_exec_wait").unwrap();
            agent_state.routine_executions.insert(
                execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    execution_id.clone(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_wait").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::ZERO,
                    Some(SimTick::new(1)),
                    Some(SimTick::new(2)),
                    None,
                    DecisionTraceId::new("trace_wait").unwrap(),
                ),
            );
            let initial_agent_state = agent_state.clone();
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![DayWindow {
                        window_id: "routine_wait".to_string(),
                        start_tick: SimTick::ZERO,
                        end_tick: SimTick::new(1),
                    }],
                },
            );

            assert!(log.events().iter().any(|event| event.event_type
                == EventKind::RoutineStepStarted
                && event
                    .causes
                    .iter()
                    .any(|cause| matches!(cause, EventCause::Event(_)))));
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::RoutineStepCompleted));
            let execution = &agent_state.routine_executions[&execution_id];
            assert_eq!(
                execution.step_status,
                crate::agent::RoutineStepStatus::Completed
            );
            assert_eq!(
                execution.concrete_action_ancestry,
                vec![SemanticActionId::new("wait").unwrap()]
            );

            let mut replay_agent_state = initial_agent_state;
            for event in log.events() {
                if matches!(
                    event.event_type,
                    EventKind::RoutineStepStarted | EventKind::RoutineStepCompleted
                ) {
                    apply_agent_event(&mut replay_agent_state, event).unwrap();
                }
            }
            assert_eq!(
                replay_agent_state.routine_executions,
                agent_state.routine_executions
            );
        }

        #[test]
        fn no_human_day_records_blocked_routine_failure_reason() {
            let actor_id = actor_id();
            let workshop = PlaceId::new("workshop").unwrap();
            let workplace_id = WorkplaceId::new("workplace_blocked").unwrap();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), workshop.clone()),
            );
            let mut workplace =
                WorkplaceState::new(workplace_id.clone(), workshop, "blocked_output");
            workplace.assigned_actor_ids.insert(actor_id.clone());
            workplace.access_open = false;
            state.workplaces.insert(workplace_id, workplace);
            let mut agent_state = agent_state(&actor_id);
            let execution_id = RoutineExecutionId::new("routine_exec_blocked_work").unwrap();
            agent_state.routine_executions.insert(
                execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    execution_id.clone(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_blocked_work").unwrap(),
                    RoutineFamily::WorkBlock,
                    SimTick::ZERO,
                    Some(SimTick::new(1)),
                    Some(SimTick::new(4)),
                    None,
                    DecisionTraceId::new("trace_blocked_work").unwrap(),
                ),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_work();

            run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![DayWindow {
                        window_id: "blocked_work".to_string(),
                        start_tick: SimTick::ZERO,
                        end_tick: SimTick::new(1),
                    }],
                },
            );

            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::WorkBlockFailed));
            assert!(log.events().iter().any(|event| event.event_type
                == EventKind::RoutineStepFailed
                && event
                    .payload
                    .iter()
                    .any(|field| { field.key == "reason" && field.value == "work_block_failed" })));
            let execution = &agent_state.routine_executions[&execution_id];
            assert_eq!(
                execution.step_status,
                crate::agent::RoutineStepStatus::Failed
            );
            assert_eq!(
                execution.failure_interruption_reason.as_deref(),
                Some("work_block_failed")
            );
        }

        #[test]
        fn no_human_day_applies_passive_needs_before_decision_and_replays_need_state() {
            let actor_id = actor_id();
            let kitchen = PlaceId::new("kitchen").unwrap();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), kitchen.clone()),
            );
            state.food_supplies.insert(
                FoodSupplyId::new("meal_serving").unwrap(),
                FoodSupplyState {
                    food_supply_id: FoodSupplyId::new("meal_serving").unwrap(),
                    location: Location::AtPlace(kitchen),
                    servings: 1,
                    hunger_reduction_per_serving: 120,
                },
            );
            let mut agent_state = agent_state(&actor_id);
            agent_state
                .needs_by_actor
                .get_mut(&actor_id)
                .unwrap()
                .insert(
                    NeedKind::Hunger,
                    crate::agent::NeedState::initial(
                        NeedKind::Hunger,
                        490,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                );
            let initial_agent_state = agent_state.clone();
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();
            registry.register_phase3a_eat();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id.clone()],
                    windows: vec![DayWindow {
                        window_id: "breakfast".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(8),
                    }],
                },
            );

            assert!(report.ordinary_pipeline_events > 0);
            let passive_index = log
                .events()
                .iter()
                .position(|event| {
                    event.event_type == EventKind::NeedDeltaApplied
                        && event
                            .payload
                            .iter()
                            .any(|field| field.key == "window_id" && field.value == "breakfast")
                })
                .expect("passive need delta emitted before decision");
            let consumed_index = log
                .events()
                .iter()
                .position(|event| event.event_type == EventKind::FoodConsumed)
                .expect("post-passive hunger selection consumed food");
            assert!(passive_index < consumed_index);
            assert!(log.events().iter().any(|event| {
                event.event_type == EventKind::NeedThresholdCrossed
                    && event.payload.iter().any(|field| {
                        field.key == "candidate_goal_reevaluation" && field.value == "true"
                    })
            }));
            let action_delta = log
                .events()
                .iter()
                .find(|event| {
                    event.event_type == EventKind::NeedDeltaApplied
                        && event
                            .payload
                            .iter()
                            .any(|field| field.key == "cause_action_id" && field.value == "eat")
                })
                .expect("eat emitted action-derived need delta");
            assert!(action_delta
                .causes
                .iter()
                .any(|cause| matches!(cause, EventCause::Event(_))));

            let mut replay_agent_state = initial_agent_state;
            for event in log.events() {
                if matches!(
                    event.event_type,
                    EventKind::NeedDeltaApplied | EventKind::NeedThresholdCrossed
                ) {
                    apply_agent_event(&mut replay_agent_state, event).unwrap();
                }
            }
            assert_eq!(
                replay_agent_state.needs_by_actor,
                agent_state.needs_by_actor
            );
            let replay_hunger = &replay_agent_state.needs_by_actor[&actor_id][&NeedKind::Hunger];
            let live_hunger = &agent_state.needs_by_actor[&actor_id][&NeedKind::Hunger];
            assert_eq!(replay_hunger.value(), live_hunger.value());
            assert_eq!(replay_hunger.band(), live_hunger.band());
            assert_eq!(
                replay_hunger.last_change_cause(),
                live_hunger.last_change_cause()
            );
            assert_eq!(
                replay_hunger.last_threshold_crossing(),
                live_hunger.last_threshold_crossing()
            );
        }

        #[test]
        fn severe_need_threshold_marks_active_intention_interruption_cause() {
            let actor_id = actor_id();
            let mut state = PhysicalState::default();
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), PlaceId::new("workshop").unwrap()),
            );
            let mut agent_state = agent_state(&actor_id);
            agent_state
                .needs_by_actor
                .get_mut(&actor_id)
                .unwrap()
                .insert(
                    NeedKind::Hunger,
                    crate::agent::NeedState::initial(
                        NeedKind::Hunger,
                        740,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                );
            let intention = crate::agent::Intention::adopt(
                IntentionId::new("intention_continue_work").unwrap(),
                actor_id.clone(),
                crate::agent::IntentionSource::RoutineDuty,
                CandidateGoalId::new("goal_continue_work").unwrap(),
                Some(RoutineTemplateId::new("routine_continue_work").unwrap()),
                Some("work_block".to_string()),
                8,
                SimTick::ZERO,
                DecisionTraceId::new("trace_continue_work").unwrap(),
            );
            agent_state
                .active_intention_by_actor
                .insert(actor_id.clone(), intention.intention_id.clone());
            agent_state
                .intentions
                .insert(intention.intention_id.clone(), intention);
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![DayWindow {
                        window_id: "hunger_spike".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(8),
                    }],
                },
            );

            let threshold = log
                .events()
                .iter()
                .find(|event| {
                    event.event_type == EventKind::NeedThresholdCrossed
                        && event
                            .payload
                            .iter()
                            .any(|field| field.key == "to_band" && field.value == "severe")
                })
                .expect("severe threshold crossing emitted");
            assert!(threshold.payload.iter().any(|field| {
                field.key == "severe_need_interrupts_active_intention" && field.value == "true"
            }));
            assert!(threshold.payload.iter().any(|field| {
                field.key == "interruption_cause" && field.value == "severe_need_pressure"
            }));
        }

        #[test]
        fn no_human_day_runs_windows_in_stable_actor_order_without_controller_facts() {
            let mut state = PhysicalState::default();
            state.actors.insert(
                ActorId::new("actor_tomas").unwrap(),
                ActorBody::new(
                    ActorId::new("actor_tomas").unwrap(),
                    crate::ids::PlaceId::new("shop_front").unwrap(),
                ),
            );
            state.actors.insert(
                ActorId::new("actor_mara").unwrap(),
                ActorBody::new(
                    ActorId::new("actor_mara").unwrap(),
                    crate::ids::PlaceId::new("shop_front").unwrap(),
                ),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            let report = run_no_human_day(
                &mut state,
                Box::leak(Box::new(crate::state::AgentState::default())),
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![
                        ActorId::new("actor_tomas").unwrap(),
                        ActorId::new("actor_mara").unwrap(),
                    ],
                    windows: vec![
                        DayWindow {
                            window_id: "morning".to_string(),
                            start_tick: SimTick::new(4),
                            end_tick: SimTick::new(8),
                        },
                        DayWindow {
                            window_id: "pre_dawn".to_string(),
                            start_tick: SimTick::ZERO,
                            end_tick: SimTick::new(4),
                        },
                    ],
                },
            );

            assert_eq!(
                report.actor_decision_order,
                [
                    ActorId::new("actor_mara").unwrap(),
                    ActorId::new("actor_tomas").unwrap()
                ]
            );
            assert_eq!(report.window_ids, ["pre_dawn", "morning"]);
            assert_eq!(report.start_tick, SimTick::ZERO);
            assert_eq!(report.final_tick, SimTick::new(8));
            assert_eq!(report.marker_event_ids.len(), 2);
            assert_eq!(report.stuck_diagnostic_event_ids.len(), 0);
            assert!(report.ordinary_pipeline_events > 0);
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::NoHumanDayStarted));
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::NoHumanDayCompleted));
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ActorWaited));
            let rendered = format!("{:?}", log.events());
            assert!(!rendered.contains("player"));
            assert!(!rendered.contains("controller"));
        }

        #[test]
        fn no_human_day_records_stuck_diagnostics_without_progress() {
            let mut state = PhysicalState::default();
            state.actors.insert(
                ActorId::new("actor_tomas").unwrap(),
                ActorBody::new(
                    ActorId::new("actor_tomas").unwrap(),
                    crate::ids::PlaceId::new("shop_front").unwrap(),
                ),
            );
            let mut log = EventLog::new();
            let registry = ActionRegistry::new();

            let report = run_no_human_day(
                &mut state,
                Box::leak(Box::new(crate::state::AgentState::default())),
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![ActorId::new("actor_tomas").unwrap()],
                    windows: vec![DayWindow {
                        window_id: "morning".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(8),
                    }],
                },
            );

            assert_eq!(report.ordinary_pipeline_events, 0);
            assert_eq!(report.stuck_diagnostic_event_ids.len(), 1);
            let diagnostic = log
                .events()
                .iter()
                .find(|event| event.event_type == EventKind::StuckDiagnosticRecorded)
                .expect("stuck diagnostic emitted");
            assert!(diagnostic
                .payload
                .iter()
                .any(|field| field.key == "diagnostic_canonical"
                    && field.value.starts_with("stuck_diagnostic_v1|")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::events::apply::apply_event;
    use crate::events::log::EventLog;
    use crate::events::{EventKind, EventStream};
    use crate::ids::{ContentManifestId, ProposalId};
    use crate::state::{ActorBody, AgentState, PhysicalState};

    fn action_id(value: &str) -> ActionId {
        ActionId::new(value).unwrap()
    }

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn seeded_agent_state(actor_id: ActorId) -> AgentState {
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id,
            [
                (
                    crate::agent::NeedKind::Hunger,
                    crate::agent::NeedState::initial(
                        crate::agent::NeedKind::Hunger,
                        100,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                ),
                (
                    crate::agent::NeedKind::Fatigue,
                    crate::agent::NeedState::initial(
                        crate::agent::NeedKind::Fatigue,
                        100,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                ),
                (
                    crate::agent::NeedKind::Safety,
                    crate::agent::NeedState::initial(
                        crate::agent::NeedKind::Safety,
                        100,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        state
    }

    fn process_id(value: &str) -> ProcessId {
        ProcessId::new(value).unwrap()
    }

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase1_manifest").unwrap()
    }

    fn key(
        tick: u64,
        phase: SchedulePhase,
        source_id: SchedulerSourceId,
        sequence: u64,
        action: &str,
        targets: &[&str],
        tie: &str,
    ) -> OrderingKey {
        OrderingKey::new(
            SimTick::new(tick),
            phase,
            source_id,
            ProposalSequence::new(sequence),
            action_id(action),
            targets.iter().map(|target| target.to_string()).collect(),
            tie,
        )
    }

    #[test]
    fn ordering_key_sort_is_independent_of_insertion_order() {
        let canonical = vec![
            key(
                0,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(actor_id("actor_mara")),
                0,
                "move",
                &["place_hall"],
                "a",
            ),
            key(
                0,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(actor_id("actor_tomas")),
                0,
                "open",
                &["strongbox_tomas"],
                "a",
            ),
            key(
                0,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(actor_id("actor_tomas")),
                1,
                "take",
                &["coin_stack_01", "strongbox_tomas"],
                "a",
            ),
            key(
                1,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id("ambient_tick")),
                0,
                "wait",
                &["1_tick"],
                "a",
            ),
        ];

        let mut first_order: Vec<_> = [2, 0, 3, 1]
            .into_iter()
            .map(|index| Scheduled {
                ordering_key: canonical[index].clone(),
                payload: index,
            })
            .collect();
        let mut second_order: Vec<_> = [3, 1, 2, 0]
            .into_iter()
            .map(|index| Scheduled {
                ordering_key: canonical[index].clone(),
                payload: index,
            })
            .collect();

        sort_scheduled(&mut first_order);
        sort_scheduled(&mut second_order);

        let first_keys: Vec<_> = first_order
            .iter()
            .map(|entry| entry.ordering_key.clone())
            .collect();
        let second_keys: Vec<_> = second_order
            .iter()
            .map(|entry| entry.ordering_key.clone())
            .collect();
        assert_eq!(first_keys, canonical);
        assert_eq!(second_keys, canonical);
    }

    #[test]
    fn proposal_sequence_assignment_is_monotonic() {
        let mut assigner = ProposalSequenceAssigner::new();

        assert_eq!(assigner.assign_next(), ProposalSequence::new(0));
        assert_eq!(assigner.assign_next(), ProposalSequence::new(1));
        assert_eq!(assigner.assign_next(), ProposalSequence::new(2));
    }

    #[test]
    fn scheduler_advance_changes_only_tick_and_sequence_state() {
        let mut scheduler = DeterministicScheduler::new(SimTick::new(4));

        assert_eq!(
            scheduler.assign_proposal_sequence(),
            ProposalSequence::new(0)
        );
        assert_eq!(scheduler.advance_one_tick(), SimTick::new(5));
        assert_eq!(scheduler.current_tick, SimTick::new(5));
        assert_eq!(
            scheduler.assign_proposal_sequence(),
            ProposalSequence::new(1)
        );
    }

    #[test]
    fn passive_need_delta_emission_is_deterministic_over_advancement() {
        let process = process_id("ambient_tick");
        let actors = vec![actor_id("actor_mara"), actor_id("actor_tomas")];

        let first = build_passive_need_delta_events(
            actors.clone(),
            &process,
            SimTick::new(4),
            3,
            &content_manifest_id(),
        );
        let second = build_passive_need_delta_events(
            actors,
            &process,
            SimTick::new(4),
            3,
            &content_manifest_id(),
        );

        assert_eq!(first, second);
        assert_eq!(first.len(), 4);
        assert!(first
            .iter()
            .all(|event| event.event_type == EventKind::NeedDeltaApplied));
        assert!(first.iter().any(|event| event
            .payload
            .iter()
            .any(|field| field.key == "delta" && field.value == "15")));
        assert!(first.iter().any(|event| event
            .payload
            .iter()
            .any(|field| field.key == "delta" && field.value == "9")));
    }

    #[test]
    fn scheduled_completion_at_wait_target_tick_is_not_dropped_by_wait_continuation() {
        let mut scheduled = [
            Scheduled {
                ordering_key: key(
                    5,
                    SchedulePhase::DeferredProcess,
                    SchedulerSourceId::Process(process_id("sleep_completion")),
                    0,
                    "sleep_completed",
                    &["actor_tomas"],
                    "completion",
                ),
                payload: "completion",
            },
            Scheduled {
                ordering_key: key(
                    5,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Process(process_id("wait_continue")),
                    1,
                    "wait",
                    &["actor_tomas"],
                    "wait",
                ),
                payload: "wait",
            },
        ];

        sort_scheduled(&mut scheduled);

        let payloads = scheduled
            .iter()
            .map(|entry| entry.payload)
            .collect::<Vec<_>>();
        assert_eq!(payloads.len(), 2);
        assert!(payloads.contains(&"wait"));
        assert!(payloads.contains(&"completion"));
        assert!(scheduled
            .iter()
            .all(|entry| entry.ordering_key.sim_tick == SimTick::new(5)));
    }

    #[test]
    fn no_human_advance_requires_no_controller_and_emits_diagnostic_markers() {
        let mut state = PhysicalState::default();
        let mut log = EventLog::new();
        let registry = ActionRegistry::new();

        let report = no_human::advance_no_human(
            no_human::NoHumanStateMut {
                physical: &mut state,
                agent: Box::leak(Box::new(crate::state::AgentState::default())),
            },
            &mut log,
            &registry,
            content_manifest_id(),
            SimTick::new(2),
            3,
            Vec::new(),
        );

        assert_eq!(report.start_tick, SimTick::new(2));
        assert_eq!(report.final_tick, SimTick::new(5));
        assert_eq!(report.marker_event_ids.len(), 2);
        assert_eq!(log.events().len(), 2);
        assert!(log
            .events()
            .iter()
            .all(|event| event.stream == EventStream::Diagnostic));
        assert_eq!(
            log.events()
                .iter()
                .map(|event| event.event_type)
                .collect::<Vec<_>>(),
            [
                EventKind::NoHumanAdvanceStarted,
                EventKind::NoHumanAdvanceCompleted
            ]
        );
        assert!(!format!("{:?}", log.events()).contains("PlayerCharacter"));
    }

    #[test]
    fn no_human_markers_replay_as_physical_noops() {
        let mut state = PhysicalState::default();
        let before = state.clone();
        let mut log = EventLog::new();
        let registry = ActionRegistry::new();

        no_human::advance_no_human(
            no_human::NoHumanStateMut {
                physical: &mut state,
                agent: Box::leak(Box::new(crate::state::AgentState::default())),
            },
            &mut log,
            &registry,
            content_manifest_id(),
            SimTick::ZERO,
            1,
            Vec::new(),
        );
        let mut replay = before.clone();
        for event in log.events() {
            apply_event(&mut replay, event).unwrap();
        }

        assert_eq!(replay, before);
    }

    #[test]
    fn no_human_scheduled_actions_use_shared_pipeline() {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(
                actor_id("actor_tomas"),
                crate::ids::PlaceId::new("shop_front").unwrap(),
            ),
        );
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase1_inspect_wait();
        let mut agent_state = seeded_agent_state(actor_id("actor_tomas"));
        let proposal = Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            action_id("wait"),
            SimTick::ZERO,
        );

        let report = no_human::advance_no_human(
            no_human::NoHumanStateMut {
                physical: &mut state,
                agent: &mut agent_state,
            },
            &mut log,
            &registry,
            content_manifest_id(),
            SimTick::ZERO,
            1,
            vec![proposal.clone()],
        );

        assert_eq!(report.ordinary_pipeline_events, 3);
        assert!(log
            .events()
            .iter()
            .any(|event| event.event_type == EventKind::ActorWaited));
        assert_eq!(
            log.events()
                .iter()
                .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
                .count(),
            2
        );

        let mut direct_state = PhysicalState::default();
        direct_state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(
                actor_id("actor_tomas"),
                crate::ids::PlaceId::new("shop_front").unwrap(),
            ),
        );
        let mut direct_log = EventLog::new();
        let mut direct_agent_state = seeded_agent_state(actor_id("actor_tomas"));
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut direct_state,
            agent_state: &mut direct_agent_state,
            log: &mut direct_log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: OrderingKey::new(
                SimTick::ZERO,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id("no_human_advance")),
                ProposalSequence::new(0),
                action_id("wait"),
                Vec::new(),
                "proposal_wait",
            ),
        };
        let direct = run_pipeline(&mut context, &proposal);
        assert_eq!(direct.appended_events.len(), 3);
        assert_eq!(
            direct
                .appended_events
                .iter()
                .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
                .count(),
            2
        );
    }
}
