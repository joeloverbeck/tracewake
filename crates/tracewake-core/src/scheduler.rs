use std::collections::BTreeSet;

use crate::actions::defs::need_events::{
    build_need_delta_and_threshold_events, NeedDeltaEventSpec,
};
use crate::actions::defs::sleep::{build_sleep_completion_events, build_sleep_interruption_events};
use crate::actions::defs::work::build_work_completion_events;
use crate::agent::{NeedKind, NeedState};
use crate::events::apply::{apply_agent_event, ApplyError};
use crate::events::log::{EventLog, EventLogError};
use crate::events::{
    is_duration_terminal, EventCause, EventEnvelope, EventEnvelopeBuildError, EventKind,
    PayloadField, EVENT_SCHEMA_V1,
};
use crate::ids::{ActionId, ActorId, ContentManifestId, ControllerId, EventId, IdError, ProcessId};
use crate::need_accounting::{
    classify_actor_tick_regimes, open_body_exclusive_starts, DuplicateDurationTerminal,
};
use crate::state::{AgentState, NeedModelState, PhysicalState};
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldAdvanceOrigin {
    Controller(ControllerId),
    Process(ProcessId),
    Replay,
}

impl WorldAdvanceOrigin {
    fn stable_id(&self) -> String {
        match self {
            Self::Controller(controller_id) => {
                format!("controller.{}", controller_id.as_str())
            }
            Self::Process(process_id) => format!("process.{}", process_id.as_str()),
            Self::Replay => "replay".to_string(),
        }
    }

    fn cause_process_id(&self) -> Result<ProcessId, IdError> {
        let stable_id = match self {
            Self::Controller(controller_id) => {
                format!("world_step.controller.{}", controller_id.as_str())
            }
            Self::Process(process_id) => {
                format!("world_step.process.{}", process_id.as_str())
            }
            Self::Replay => "world_step.replay".to_string(),
        };
        ProcessId::new(stable_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldAdvanceRequest {
    pub expected_tick: SimTick,
    pub origin: WorldAdvanceOrigin,
    pub content_manifest_id: ContentManifestId,
    pub authorized_sleep_interruptions: Vec<AuthorizedSleepInterruption>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthorizedSleepInterruption {
    pub start_event_id: EventId,
    pub terminal_tick: SimTick,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldStepDueWorkSummary {
    pub open_duration_candidates: usize,
    pub duration_terminals_appended: usize,
    pub actor_transactions_attempted: usize,
    pub world_processes_applied: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BodyExclusiveDurationKind {
    Sleep,
    Work,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenDurationCandidate {
    pub start_event_id: EventId,
    pub actor_id: ActorId,
    pub duration_kind: BodyExclusiveDurationKind,
    pub start_tick: SimTick,
    pub expected_completion_tick: SimTick,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldAdvanceResult {
    pub prior_tick: SimTick,
    pub resulting_tick: SimTick,
    pub appended_event_ids: Vec<EventId>,
    pub due_duration_candidates: Vec<OpenDurationCandidate>,
    pub due_work_summary: WorldStepDueWorkSummary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldAdvanceError {
    FrontierMismatch {
        expected: SimTick,
        actual: SimTick,
    },
    OpenDurationDiscovery(DuplicateDurationTerminal),
    MalformedDurationStart {
        start_event_id: EventId,
        reason: String,
    },
    DurationLifecycleBuild {
        start_event_id: EventId,
        error: ApplyError,
    },
    DurationLifecycleApply {
        event_id: EventId,
        error: ApplyError,
    },
    EventAppend(EventLogError),
    InvalidAccountingActionId(IdError),
    InvalidMarkerId(IdError),
    InvalidMarkerEnvelope(EventEnvelopeBuildError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdvanceUntilRequest {
    pub expected_tick: SimTick,
    pub origin: WorldAdvanceOrigin,
    pub content_manifest_id: ContentManifestId,
    pub possessed_actor_id: ActorId,
    pub max_ticks: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvanceUntilStopReason {
    PossessedDurationTerminal,
    ActorKnownSalientObservation,
    UserPausedBeforeNextTick,
    ControllerSafetyBound,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdvanceUntilResult {
    pub start_tick: SimTick,
    pub stop_tick: SimTick,
    pub ticks_advanced: u64,
    pub stop_reason: AdvanceUntilStopReason,
    pub appended_event_ids: Vec<EventId>,
}

pub fn sort_scheduled<T>(scheduled: &mut [Scheduled<T>]) {
    scheduled.sort_by(|left, right| left.ordering_key.cmp(&right.ordering_key));
}

pub fn build_passive_need_delta_events(
    need_model: &NeedModelState,
    actor_ids: impl IntoIterator<Item = ActorId>,
    process_id: &ProcessId,
    start_tick: SimTick,
    elapsed_ticks: u64,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    let deltas = passive_awake_need_deltas(need_model, elapsed_ticks);
    actor_ids
        .into_iter()
        .flat_map(|actor_id| {
            [
                (NeedKind::Hunger, deltas.hunger_delta),
                (NeedKind::Fatigue, deltas.fatigue_delta),
            ]
            .into_iter()
            .flat_map(move |(need_kind, delta)| {
                build_need_delta_and_threshold_events(
                    NeedDeltaEventSpec {
                        event_id: EventId::new(format!(
                            "event.passive_need_delta.{}.{}.{}.{}",
                            need_kind.stable_id(),
                            actor_id.as_str(),
                            start_tick.value(),
                            elapsed_ticks
                        ))
                        .unwrap(),
                        threshold_event_id: EventId::new(format!(
                            "event.passive_need_threshold.{}.{}.{}.{}",
                            need_kind.stable_id(),
                            actor_id.as_str(),
                            start_tick.value(),
                            elapsed_ticks
                        ))
                        .unwrap(),
                        tick: start_tick.advance_by(elapsed_ticks),
                        ordering_key: OrderingKey::new(
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
                        content_manifest_id: content_manifest_id.clone(),
                        causes: vec![EventCause::Process(process_id.clone())],
                        actor_id: actor_id.clone(),
                        proposal_id: None,
                        process_id: Some(process_id.clone()),
                        participants: vec![actor_id.to_string()],
                        need_kind,
                        delta,
                        elapsed_ticks,
                        cause_kind: "tick_delta".to_string(),
                        cause_action_id: None,
                        extra_payload: Vec::new(),
                        delta_effects_summary: format!(
                            "{} rose by {} over {} elapsed ticks",
                            need_kind.stable_id(),
                            delta,
                            elapsed_ticks
                        ),
                        threshold_effects_summary: format!(
                            "{} crossed threshold during passive ticks",
                            need_kind.stable_id()
                        ),
                    },
                    None,
                )
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

    #[cfg(test)]
    fn increment_clock_one_tick(&mut self) -> SimTick {
        self.current_tick = self.current_tick.next();
        self.current_tick
    }

    pub fn advance_world_one_tick(
        &mut self,
        state: &PhysicalState,
        agent_state: &mut AgentState,
        log: &mut EventLog,
        request: WorldAdvanceRequest,
    ) -> Result<WorldAdvanceResult, WorldAdvanceError> {
        if request.expected_tick != self.current_tick {
            return Err(WorldAdvanceError::FrontierMismatch {
                expected: request.expected_tick,
                actual: self.current_tick,
            });
        }

        let prior_tick = self.current_tick;
        let resulting_tick = prior_tick.next();
        let due_duration_candidates = discover_due_duration_candidates(log, resulting_tick)?;
        let marker = build_time_advanced_event(prior_tick, resulting_tick, &request)?;
        let lifecycle = build_duration_lifecycle_events(
            state,
            agent_state,
            log,
            &request,
            &due_duration_candidates,
            resulting_tick,
            &marker,
        )?;
        let appended_marker = log.append(marker).map_err(WorldAdvanceError::EventAppend)?;
        let mut appended_event_ids = vec![appended_marker.event_id];
        for event in lifecycle.events {
            let appended = log.append(event).map_err(WorldAdvanceError::EventAppend)?;
            apply_agent_event(agent_state, &appended).map_err(|error| {
                WorldAdvanceError::DurationLifecycleApply {
                    event_id: appended.event_id.clone(),
                    error,
                }
            })?;
            appended_event_ids.push(appended.event_id);
        }
        self.current_tick = resulting_tick;
        let open_duration_candidates = due_duration_candidates.len();

        Ok(WorldAdvanceResult {
            prior_tick,
            resulting_tick,
            appended_event_ids,
            due_duration_candidates,
            due_work_summary: WorldStepDueWorkSummary {
                open_duration_candidates,
                duration_terminals_appended: lifecycle.duration_terminals_appended,
                actor_transactions_attempted: 0,
                world_processes_applied: 0,
            },
        })
    }

    pub fn advance_until(
        &mut self,
        state: &PhysicalState,
        agent_state: &mut AgentState,
        log: &mut EventLog,
        request: AdvanceUntilRequest,
    ) -> Result<AdvanceUntilResult, WorldAdvanceError> {
        if request.expected_tick != self.current_tick {
            return Err(WorldAdvanceError::FrontierMismatch {
                expected: request.expected_tick,
                actual: self.current_tick,
            });
        }

        let start_tick = self.current_tick;
        let mut appended_event_ids = Vec::new();
        if request.max_ticks == 0 {
            return Ok(AdvanceUntilResult {
                start_tick,
                stop_tick: self.current_tick,
                ticks_advanced: 0,
                stop_reason: AdvanceUntilStopReason::UserPausedBeforeNextTick,
                appended_event_ids,
            });
        }

        for _ in 0..request.max_ticks {
            let step = self.advance_world_one_tick(
                state,
                agent_state,
                log,
                WorldAdvanceRequest {
                    expected_tick: self.current_tick,
                    origin: request.origin.clone(),
                    content_manifest_id: request.content_manifest_id.clone(),
                    authorized_sleep_interruptions: Vec::new(),
                },
            )?;
            appended_event_ids.extend(step.appended_event_ids.iter().cloned());
            if step_appended_possessed_duration_terminal(
                log,
                &step.appended_event_ids,
                &request.possessed_actor_id,
            ) {
                return Ok(AdvanceUntilResult {
                    start_tick,
                    stop_tick: self.current_tick,
                    ticks_advanced: self.current_tick.value() - start_tick.value(),
                    stop_reason: AdvanceUntilStopReason::PossessedDurationTerminal,
                    appended_event_ids,
                });
            }
            if step_appended_actor_known_salient_observation(
                log,
                &step.appended_event_ids,
                &request.possessed_actor_id,
            ) {
                return Ok(AdvanceUntilResult {
                    start_tick,
                    stop_tick: self.current_tick,
                    ticks_advanced: self.current_tick.value() - start_tick.value(),
                    stop_reason: AdvanceUntilStopReason::ActorKnownSalientObservation,
                    appended_event_ids,
                });
            }
        }

        Ok(AdvanceUntilResult {
            start_tick,
            stop_tick: self.current_tick,
            ticks_advanced: self.current_tick.value() - start_tick.value(),
            stop_reason: AdvanceUntilStopReason::ControllerSafetyBound,
            appended_event_ids,
        })
    }
}

fn step_appended_possessed_duration_terminal(
    log: &EventLog,
    appended_event_ids: &[EventId],
    possessed_actor_id: &ActorId,
) -> bool {
    appended_event_ids.iter().any(|event_id| {
        log.events().iter().any(|event| {
            &event.event_id == event_id
                && is_duration_terminal(event.event_type)
                && event.actor_id.as_ref() == Some(possessed_actor_id)
        })
    })
}

fn step_appended_actor_known_salient_observation(
    log: &EventLog,
    appended_event_ids: &[EventId],
    possessed_actor_id: &ActorId,
) -> bool {
    appended_event_ids.iter().any(|event_id| {
        log.events().iter().any(|event| {
            &event.event_id == event_id
                && event.event_type == EventKind::ObservationRecorded
                && event.actor_id.as_ref() == Some(possessed_actor_id)
        })
    })
}

struct BuiltDurationLifecycle {
    events: Vec<EventEnvelope>,
    duration_terminals_appended: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DurationLifecycleCandidate {
    SleepCompletion {
        start_event_id: EventId,
        terminal_tick: SimTick,
    },
    WorkCompletion {
        start_event_id: EventId,
        terminal_tick: SimTick,
    },
    SleepInterruption {
        start_event_id: EventId,
        terminal_tick: SimTick,
        reason: String,
    },
}

impl DurationLifecycleCandidate {
    fn start_event_id(&self) -> &EventId {
        match self {
            Self::SleepCompletion { start_event_id, .. }
            | Self::WorkCompletion { start_event_id, .. }
            | Self::SleepInterruption { start_event_id, .. } => start_event_id,
        }
    }

    const fn terminal_tick(&self) -> SimTick {
        match self {
            Self::SleepCompletion { terminal_tick, .. }
            | Self::WorkCompletion { terminal_tick, .. }
            | Self::SleepInterruption { terminal_tick, .. } => *terminal_tick,
        }
    }

    const fn ordering_priority(&self) -> u8 {
        match self {
            Self::SleepInterruption { .. } => 0,
            Self::SleepCompletion { .. } | Self::WorkCompletion { .. } => 1,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_duration_lifecycle_events(
    state: &PhysicalState,
    agent_state: &AgentState,
    log: &EventLog,
    request: &WorldAdvanceRequest,
    due_duration_candidates: &[OpenDurationCandidate],
    resulting_tick: SimTick,
    marker: &EventEnvelope,
) -> Result<BuiltDurationLifecycle, WorldAdvanceError> {
    let mut candidates = due_duration_candidates
        .iter()
        .map(|candidate| match candidate.duration_kind {
            BodyExclusiveDurationKind::Sleep => DurationLifecycleCandidate::SleepCompletion {
                start_event_id: candidate.start_event_id.clone(),
                terminal_tick: candidate.expected_completion_tick,
            },
            BodyExclusiveDurationKind::Work => DurationLifecycleCandidate::WorkCompletion {
                start_event_id: candidate.start_event_id.clone(),
                terminal_tick: candidate.expected_completion_tick,
            },
        })
        .collect::<Vec<_>>();
    candidates.extend(
        request
            .authorized_sleep_interruptions
            .iter()
            .map(
                |interruption| DurationLifecycleCandidate::SleepInterruption {
                    start_event_id: interruption.start_event_id.clone(),
                    terminal_tick: interruption.terminal_tick,
                    reason: interruption.reason.clone(),
                },
            ),
    );
    candidates.retain(|candidate| candidate.terminal_tick() <= resulting_tick);
    candidates.sort_by(|left, right| {
        left.terminal_tick()
            .cmp(&right.terminal_tick())
            .then_with(|| left.ordering_priority().cmp(&right.ordering_priority()))
            .then_with(|| left.start_event_id().cmp(right.start_event_id()))
    });

    let mut scratch_log = log.clone();
    let mut scratch_agent_state = agent_state.clone();
    scratch_log
        .append(marker.clone())
        .map_err(WorldAdvanceError::EventAppend)?;
    let process_id = request
        .origin
        .cause_process_id()
        .map_err(WorldAdvanceError::InvalidMarkerId)?;
    let mut events = Vec::new();
    let mut duration_terminals_appended = 0;
    for (sequence, candidate) in candidates.into_iter().enumerate() {
        let start_event = scratch_log
            .events()
            .iter()
            .find(|event| event.event_id == *candidate.start_event_id())
            .cloned()
            .ok_or_else(|| WorldAdvanceError::MalformedDurationStart {
                start_event_id: candidate.start_event_id().clone(),
                reason: "duration lifecycle start was not present in the event log".to_string(),
            })?;
        let ordering_key =
            duration_lifecycle_ordering_key(&candidate, &start_event, &process_id, sequence)?;
        let built_events = match &candidate {
            DurationLifecycleCandidate::SleepCompletion { terminal_tick, .. } => {
                build_sleep_completion_events(
                    state,
                    &scratch_agent_state,
                    &scratch_log,
                    &start_event,
                    &ordering_key,
                    &request.content_manifest_id,
                    *terminal_tick,
                )
            }
            DurationLifecycleCandidate::WorkCompletion { terminal_tick, .. } => {
                build_work_completion_events(
                    state,
                    &scratch_agent_state,
                    &scratch_log,
                    &start_event,
                    &ordering_key,
                    &request.content_manifest_id,
                    *terminal_tick,
                )
            }
            DurationLifecycleCandidate::SleepInterruption {
                terminal_tick,
                reason,
                ..
            } => build_sleep_interruption_events(
                &scratch_agent_state,
                &start_event,
                &scratch_log,
                &ordering_key,
                &request.content_manifest_id,
                *terminal_tick,
                reason,
            ),
        }
        .map_err(|error| WorldAdvanceError::DurationLifecycleBuild {
            start_event_id: candidate.start_event_id().clone(),
            error,
        })?;
        for event in built_events {
            let appended = scratch_log
                .append(event.clone())
                .map_err(WorldAdvanceError::EventAppend)?;
            apply_agent_event(&mut scratch_agent_state, &appended).map_err(|error| {
                WorldAdvanceError::DurationLifecycleApply {
                    event_id: appended.event_id.clone(),
                    error,
                }
            })?;
            if is_duration_terminal(appended.event_type) {
                duration_terminals_appended += 1;
            }
            events.push(event);
        }
    }
    append_missing_accounting_events(
        state,
        &mut scratch_log,
        &mut scratch_agent_state,
        request,
        resulting_tick,
        &process_id,
        &mut events,
    )?;
    Ok(BuiltDurationLifecycle {
        events,
        duration_terminals_appended,
    })
}

fn append_missing_accounting_events(
    state: &PhysicalState,
    scratch_log: &mut EventLog,
    scratch_agent_state: &mut AgentState,
    request: &WorldAdvanceRequest,
    resulting_tick: SimTick,
    process_id: &ProcessId,
    events: &mut Vec<EventEnvelope>,
) -> Result<(), WorldAdvanceError> {
    for actor_id in state.actors().keys() {
        let regime_counts = classify_actor_tick_regimes(
            scratch_log,
            actor_id,
            request.expected_tick,
            resulting_tick,
        );
        if regime_counts.awake_ticks == 0 {
            continue;
        }
        let deltas = passive_awake_need_deltas(state.need_model(), regime_counts.awake_ticks);
        append_missing_accounting_need_events(
            scratch_log,
            scratch_agent_state,
            request,
            resulting_tick,
            process_id,
            actor_id,
            NeedKind::Hunger,
            deltas.hunger_delta,
            regime_counts.awake_ticks,
            events,
        )?;
        append_missing_accounting_need_events(
            scratch_log,
            scratch_agent_state,
            request,
            resulting_tick,
            process_id,
            actor_id,
            NeedKind::Fatigue,
            deltas.fatigue_delta,
            regime_counts.awake_ticks,
            events,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn append_missing_accounting_need_events(
    scratch_log: &mut EventLog,
    scratch_agent_state: &mut AgentState,
    request: &WorldAdvanceRequest,
    resulting_tick: SimTick,
    process_id: &ProcessId,
    actor_id: &ActorId,
    need_kind: NeedKind,
    delta: i32,
    elapsed_ticks: u64,
    events: &mut Vec<EventEnvelope>,
) -> Result<(), WorldAdvanceError> {
    if delta == 0 || elapsed_ticks == 0 {
        return Ok(());
    }
    let current = scratch_agent_state
        .needs_by_actor()
        .get(actor_id)
        .and_then(|needs| needs.get(&need_kind))
        .map(NeedState::value);
    let action_id = ActionId::new("world_step_need_accounting")
        .map_err(WorldAdvanceError::InvalidAccountingActionId)?;
    let ordering_key = OrderingKey::new(
        resulting_tick,
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Process(process_id.clone()),
        ProposalSequence::new(10_000),
        action_id,
        vec![
            actor_id.as_str().to_string(),
            need_kind.stable_id().to_string(),
        ],
        format!(
            "world_step_accounting:{}:{}:{}",
            actor_id.as_str(),
            need_kind.stable_id(),
            resulting_tick.value()
        ),
    );
    let built_events = build_need_delta_and_threshold_events(
        NeedDeltaEventSpec {
            event_id: EventId::new(format!(
                "event.world_step_need_delta.{}.{}.{}",
                actor_id.as_str(),
                need_kind.stable_id(),
                resulting_tick.value()
            ))
            .map_err(WorldAdvanceError::InvalidMarkerId)?,
            threshold_event_id: EventId::new(format!(
                "event.world_step_need_threshold.{}.{}.{}",
                actor_id.as_str(),
                need_kind.stable_id(),
                resulting_tick.value()
            ))
            .map_err(WorldAdvanceError::InvalidMarkerId)?,
            tick: resulting_tick,
            ordering_key,
            content_manifest_id: request.content_manifest_id.clone(),
            causes: vec![EventCause::Process(process_id.clone())],
            actor_id: actor_id.clone(),
            proposal_id: None,
            process_id: Some(process_id.clone()),
            participants: vec![actor_id.to_string()],
            need_kind,
            delta,
            elapsed_ticks,
            cause_kind: "tick_delta".to_string(),
            cause_action_id: None,
            extra_payload: vec![PayloadField::new("accounting_phase", "world_step")],
            delta_effects_summary: format!(
                "{} changed by {} during world-step accounting",
                need_kind.stable_id(),
                delta
            ),
            threshold_effects_summary: format!(
                "{} crossed threshold during world-step accounting",
                need_kind.stable_id()
            ),
        },
        current,
    );
    let mut trial_agent_state = scratch_agent_state.clone();
    for event in &built_events {
        if let Err(error) = apply_agent_event(&mut trial_agent_state, event) {
            if matches!(error, ApplyError::DuplicateNeedTickCharge { .. }) {
                return Ok(());
            }
            return Err(WorldAdvanceError::DurationLifecycleApply {
                event_id: event.event_id.clone(),
                error,
            });
        }
    }
    for event in built_events {
        let appended = scratch_log
            .append(event.clone())
            .map_err(WorldAdvanceError::EventAppend)?;
        events.push(event);
        apply_agent_event(scratch_agent_state, &appended).map_err(|error| {
            WorldAdvanceError::DurationLifecycleApply {
                event_id: appended.event_id.clone(),
                error,
            }
        })?;
    }
    Ok(())
}

fn duration_lifecycle_ordering_key(
    candidate: &DurationLifecycleCandidate,
    start_event: &EventEnvelope,
    process_id: &ProcessId,
    sequence: usize,
) -> Result<OrderingKey, WorldAdvanceError> {
    let action_id = match candidate {
        DurationLifecycleCandidate::SleepCompletion { .. } => "sleep_completed",
        DurationLifecycleCandidate::WorkCompletion { .. } => "work_block_completed",
        DurationLifecycleCandidate::SleepInterruption { .. } => "sleep_interrupted",
    };
    let actor_target = start_event
        .actor_id
        .as_ref()
        .map(|actor_id| actor_id.as_str().to_string())
        .unwrap_or_default();
    Ok(OrderingKey::new(
        candidate.terminal_tick(),
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Process(process_id.clone()),
        ProposalSequence::new(sequence as u64),
        ActionId::new(action_id).map_err(WorldAdvanceError::InvalidMarkerId)?,
        vec![actor_target],
        format!("{}:{}", action_id, start_event.event_id.as_str()),
    ))
}

fn discover_due_duration_candidates(
    log: &EventLog,
    resulting_tick: SimTick,
) -> Result<Vec<OpenDurationCandidate>, WorldAdvanceError> {
    let mut actor_ids = BTreeSet::new();
    for event in log.events().iter().filter(|event| {
        matches!(
            event.event_type,
            EventKind::SleepStarted | EventKind::WorkBlockStarted
        )
    }) {
        if let Some(actor_id) = event.actor_id.as_ref() {
            actor_ids.insert(actor_id.clone());
        }
    }

    let mut candidates = Vec::new();
    for actor_id in actor_ids {
        for start_event_id in open_body_exclusive_starts(log, &actor_id, resulting_tick)
            .map_err(WorldAdvanceError::OpenDurationDiscovery)?
        {
            let Some(start_event) = log
                .events()
                .iter()
                .find(|event| event.event_id == start_event_id)
            else {
                return Err(WorldAdvanceError::MalformedDurationStart {
                    start_event_id,
                    reason: "open duration start was not present in the event log".to_string(),
                });
            };
            let Some(candidate) = due_duration_candidate(start_event, resulting_tick)? else {
                continue;
            };
            candidates.push(candidate);
        }
    }
    candidates.sort_by(|left, right| {
        left.expected_completion_tick
            .cmp(&right.expected_completion_tick)
            .then_with(|| left.actor_id.cmp(&right.actor_id))
            .then_with(|| left.start_event_id.cmp(&right.start_event_id))
    });
    Ok(candidates)
}

fn due_duration_candidate(
    start_event: &EventEnvelope,
    resulting_tick: SimTick,
) -> Result<Option<OpenDurationCandidate>, WorldAdvanceError> {
    let duration_kind = match start_event.event_type {
        EventKind::SleepStarted => BodyExclusiveDurationKind::Sleep,
        EventKind::WorkBlockStarted => BodyExclusiveDurationKind::Work,
        _ => return Ok(None),
    };
    let expected_completion_tick = parse_duration_start_completion_tick(start_event)?;
    if expected_completion_tick > resulting_tick {
        return Ok(None);
    }
    let Some(actor_id) = start_event.actor_id.clone() else {
        return Err(WorldAdvanceError::MalformedDurationStart {
            start_event_id: start_event.event_id.clone(),
            reason: "body-exclusive start was missing actor_id".to_string(),
        });
    };
    Ok(Some(OpenDurationCandidate {
        start_event_id: start_event.event_id.clone(),
        actor_id,
        duration_kind,
        start_tick: start_event.sim_tick,
        expected_completion_tick,
    }))
}

fn parse_duration_start_completion_tick(
    event: &EventEnvelope,
) -> Result<SimTick, WorldAdvanceError> {
    let Some(value) = payload_value(event, "expected_completion_tick") else {
        return Err(WorldAdvanceError::MalformedDurationStart {
            start_event_id: event.event_id.clone(),
            reason: "missing expected_completion_tick".to_string(),
        });
    };
    let tick = value
        .parse::<u64>()
        .map_err(|_| WorldAdvanceError::MalformedDurationStart {
            start_event_id: event.event_id.clone(),
            reason: format!("bad expected_completion_tick {value:?}"),
        })?;
    Ok(SimTick::new(tick))
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn build_time_advanced_event(
    prior_tick: SimTick,
    resulting_tick: SimTick,
    request: &WorldAdvanceRequest,
) -> Result<EventEnvelope, WorldAdvanceError> {
    let origin_id = request.origin.stable_id();
    let event_id = EventId::new(format!(
        "event.time_advanced.{}.{}.{}",
        prior_tick.value(),
        resulting_tick.value(),
        origin_id
    ))
    .map_err(WorldAdvanceError::InvalidMarkerId)?;
    let cause_process_id = request
        .origin
        .cause_process_id()
        .map_err(WorldAdvanceError::InvalidMarkerId)?;
    let ordering_key = OrderingKey::new(
        resulting_tick,
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Process(cause_process_id.clone()),
        ProposalSequence::new(0),
        ActionId::new("world_step").map_err(WorldAdvanceError::InvalidMarkerId)?,
        Vec::new(),
        format!("time_advanced:{}", origin_id),
    );
    let mut event = EventEnvelope::new_caused_v1(
        event_id,
        EventKind::TimeAdvanced,
        0,
        0,
        resulting_tick,
        ordering_key,
        request.content_manifest_id.clone(),
        vec![EventCause::Process(cause_process_id)],
    )
    .map_err(WorldAdvanceError::InvalidMarkerEnvelope)?;
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("prior_tick", prior_tick.value().to_string()),
        PayloadField::new("resulting_tick", resulting_tick.value().to_string()),
        PayloadField::new("origin", origin_id),
        PayloadField::new("ordering_ancestry", "canonical_world_step_v1"),
    ];
    event.effects_summary = format!(
        "authoritative world step advanced from tick {} to {}",
        prior_tick.value(),
        resulting_tick.value()
    );
    Ok(event)
}

pub mod no_human {
    use std::collections::{BTreeMap, BTreeSet};

    #[cfg(test)]
    use crate::actions::defs::need_events::{
        build_need_delta_and_threshold_events, NeedDeltaEventSpec,
    };
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::Proposal;
    use crate::actions::registry::ActionRegistry;
    #[cfg(test)]
    use crate::agent::NeedKind;
    use crate::agent::{
        record_current_place_perception, ActorDecisionTransaction, ActorDecisionTransactionInput,
        ActorDecisionTransactionOutcome, ActorKnownPlanningContext, BlockerCategory, BlockerCode,
        DecisionTraceRecord, Intention, IntentionSource, NoHumanActorKnownSurfaceBuilder,
        NoHumanActorKnownSurfaceRequest, ResponsibleLayer, RoutineFamily, RoutineStepStatus,
        StuckDiagnostic, StuckResultingStatus, TypedDiagnosticFields,
    };
    use crate::epistemics::EpistemicProjection;
    use crate::events::apply::{apply_agent_event, apply_epistemic_event, ApplyError};
    use crate::events::is_duration_terminal;
    use crate::events::log::EventLog;
    use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
    use crate::ids::{
        ActionId, ActorId, CandidateGoalId, ContentManifestId, EventId, IntentionId, PlaceId,
        ProcessId, RoutineExecutionId, SemanticActionId, StuckDiagnosticId,
    };
    use crate::scheduler::{
        DeterministicScheduler, OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
        WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
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

    impl DayWindow {
        /// True when `tick` falls within this window's inclusive
        /// `[start_tick, end_tick]` range.
        fn contains_tick(&self, tick: SimTick) -> bool {
            self.start_tick <= tick && tick <= self.end_tick
        }
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
        pub scheduler_errors: Vec<NoHumanSchedulerError>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum NoHumanSchedulerError {
        DuplicateDurationTerminal {
            start_event_id: EventId,
            first_terminal_event_id: EventId,
            duplicate_terminal_event_id: EventId,
        },
        CompletionBuild {
            start_event_id: EventId,
            error: ApplyError,
        },
        WorldAdvance {
            error: WorldAdvanceError,
        },
        AgentApply {
            event_id: EventId,
            error: ApplyError,
        },
    }

    impl From<crate::need_accounting::DuplicateDurationTerminal> for NoHumanSchedulerError {
        fn from(error: crate::need_accounting::DuplicateDurationTerminal) -> Self {
            Self::DuplicateDurationTerminal {
                start_event_id: error.start_event_id,
                first_terminal_event_id: error.first_terminal_event_id,
                duplicate_terminal_event_id: error.duplicate_terminal_event_id,
            }
        }
    }

    pub fn default_day_windows(start_tick: SimTick) -> Vec<DayWindow> {
        [
            ("pre_dawn", 0, 3),
            ("morning", 4, 9),
            ("work_window", 10, 17),
            ("evening", 18, 23),
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
        let start_tick = SimTick::ZERO;
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
        let mut duration_skip_by_window_actor: BTreeSet<(String, ActorId)> = BTreeSet::new();
        let mut stuck_diagnostic_event_ids = Vec::new();
        let mut scheduler_errors = Vec::new();
        for window in &config.windows {
            let completed_durations = advance_no_human_scheduler_to_tick(
                state,
                agent_state,
                log,
                &process_id,
                &mut scheduler,
                &content_manifest_id,
                window.start_tick,
                &mut scheduler_errors,
            );
            for (completed_actor_id, completion_tick) in completed_durations {
                credit_completion(
                    &mut progress_by_window_actor,
                    &config.windows,
                    &completed_actor_id,
                    completion_tick,
                );
            }
            for actor_id in &config.actor_ids {
                if !state.actors.contains_key(actor_id) {
                    continue;
                }
                if let Err(error) = crate::need_accounting::open_body_exclusive_starts(
                    log,
                    actor_id,
                    window.start_tick,
                ) {
                    scheduler_errors.push(error.into());
                    continue;
                }
                record_current_place_perception(
                    log,
                    state,
                    actor_id,
                    window.start_tick,
                    &content_manifest_id,
                );
                let has_open_duration = match actor_has_open_body_exclusive_duration(
                    log,
                    actor_id,
                    window.start_tick,
                ) {
                    Ok(has_open_duration) => has_open_duration,
                    Err(error) => {
                        scheduler_errors.push(error.into());
                        true
                    }
                };
                if has_open_duration {
                    duration_skip_by_window_actor
                        .insert((window.window_id.clone(), actor_id.clone()));
                    append_routine_stuck_diagnostics(
                        log,
                        agent_state,
                        &process_id,
                        actor_id,
                        window,
                        &content_manifest_id,
                        &mut stuck_diagnostic_event_ids,
                        &mut scheduler_errors,
                    );
                    continue;
                }
                append_routine_stuck_diagnostics(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &content_manifest_id,
                    &mut stuck_diagnostic_event_ids,
                    &mut scheduler_errors,
                );
                let Some(agent_proposal) = build_agent_proposal(
                    state,
                    agent_state,
                    log,
                    actor_id,
                    window,
                    &content_manifest_id,
                ) else {
                    continue;
                };
                let proposal = agent_proposal.proposal;
                let decision_trace_record = agent_proposal.decision_trace_record;
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
                sync_scheduler_frontier_to_appended_events(&mut scheduler, &result.appended_events);
                let progress_events = no_human_progress_event_count(&result.appended_events);
                ordinary_pipeline_events += progress_events;
                record_window_progress(
                    &mut progress_by_window_actor,
                    &window.window_id,
                    actor_id,
                    progress_events,
                );
                append_intention_lifecycle_after_proposal(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &proposal,
                    &content_manifest_id,
                    &decision_trace_record,
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
                append_decision_trace_after_proposal(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &proposal,
                    &decision_trace_record,
                    &content_manifest_id,
                    result.appended_events.first(),
                );
                append_routine_stuck_diagnostics(
                    log,
                    agent_state,
                    &process_id,
                    actor_id,
                    window,
                    &content_manifest_id,
                    &mut stuck_diagnostic_event_ids,
                    &mut scheduler_errors,
                );
            }
        }

        let completed_durations = advance_no_human_scheduler_to_tick(
            state,
            agent_state,
            log,
            &process_id,
            &mut scheduler,
            &content_manifest_id,
            final_tick,
            &mut scheduler_errors,
        );
        for (completed_actor_id, completion_tick) in completed_durations {
            credit_completion(
                &mut progress_by_window_actor,
                &config.windows,
                &completed_actor_id,
                completion_tick,
            );
        }

        for window in &config.windows {
            for actor_id in &config.actor_ids {
                if !state.actors.contains_key(actor_id) {
                    continue;
                }
                if duration_skip_by_window_actor
                    .contains(&(window.window_id.clone(), actor_id.clone()))
                {
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
                        StuckDiagnosticKind::WindowNoProgress,
                        None,
                    );
                    match apply_agent_event(agent_state, &event) {
                        Ok(_) => stuck_diagnostic_event_ids.push(event.event_id),
                        Err(error) => scheduler_errors.push(NoHumanSchedulerError::AgentApply {
                            event_id: event.event_id,
                            error,
                        }),
                    }
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
            scheduler_errors,
        }
    }

    fn record_window_progress(
        progress_by_window_actor: &mut BTreeMap<(String, ActorId), usize>,
        window_id: &str,
        actor_id: &ActorId,
        progress_events: usize,
    ) {
        if progress_events > 0 {
            progress_by_window_actor
                .insert((window_id.to_string(), actor_id.clone()), progress_events);
        }
    }

    struct BuiltAgentProposal {
        proposal: Proposal,
        decision_trace_record: DecisionTraceRecord,
    }

    fn build_agent_proposal(
        state: &PhysicalState,
        agent_state: &AgentState,
        log: &EventLog,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
    ) -> Option<BuiltAgentProposal> {
        let actor = state.actors.get(actor_id)?;
        let epistemic_projection = epistemic_projection_from_log(log, content_manifest_id);
        let actor_known_state =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &epistemic_projection,
                agent_state,
                actor_id: actor_id.clone(),
                current_place_id: actor.current_place_id.clone(),
                decision_tick: window.start_tick,
                window_id: &window.window_id,
                window_end_tick: window.end_tick,
                current_place_witness_event_id: latest_current_place_perception_event_id(
                    log,
                    actor_id,
                    window.start_tick,
                    &actor.current_place_id,
                ),
                needs_witness_event_id: latest_need_event_id(log, actor_id),
                frame_event_id: latest_frame_event_id(log),
            })
            .build(agent_state)
            .into_context();
        let source_event_ids = log
            .events()
            .iter()
            .map(|event| event.event_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let source_event_kinds = log
            .events()
            .iter()
            .map(|event| (event.event_id.clone(), event.event_type))
            .collect::<std::collections::BTreeMap<_, _>>();
        match ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id.clone(),
            decision_tick: window.start_tick,
            agent_state,
            actor_known_context: &actor_known_state,
            source_event_ids: Some(&source_event_ids),
            source_event_kinds: Some(&source_event_kinds),
            routine_window_family: routine_window_family(
                agent_state,
                actor_id,
                window,
                &actor_known_state,
            ),
            include_idle_fallback: true,
        }) {
            ActorDecisionTransactionOutcome::Proposed(proposed) => Some(BuiltAgentProposal {
                proposal: proposed.proposal.into_proposal(),
                decision_trace_record: proposed.decision_trace_record,
            }),
            ActorDecisionTransactionOutcome::Stuck { .. } => None,
        }
    }

    fn epistemic_projection_from_log(
        log: &EventLog,
        content_manifest_id: &ContentManifestId,
    ) -> EpistemicProjection {
        let mut projection = EpistemicProjection::new(content_manifest_id.clone());
        for event in log.events() {
            if apply_epistemic_event(&mut projection, event).is_err() {
                continue;
            }
        }
        projection
    }

    fn latest_frame_event_id(log: &EventLog) -> Option<EventId> {
        log.events()
            .iter()
            .rev()
            .find(|event| {
                matches!(
                    event.event_type,
                    EventKind::NoHumanDayStarted | EventKind::NoHumanAdvanceStarted
                )
            })
            .map(|event| event.event_id.clone())
    }

    fn latest_current_place_perception_event_id(
        log: &EventLog,
        actor_id: &ActorId,
        tick: SimTick,
        place_id: &PlaceId,
    ) -> Option<EventId> {
        log.events()
            .iter()
            .rev()
            .find(|event| {
                event.event_type == EventKind::ObservationRecorded
                    && event.actor_id.as_ref() == Some(actor_id)
                    && event.sim_tick == tick
                    && event.place_id.as_ref() == Some(place_id)
            })
            .map(|event| event.event_id.clone())
    }

    fn latest_need_event_id(log: &EventLog, actor_id: &ActorId) -> Option<EventId> {
        log.events()
            .iter()
            .rev()
            .find(|event| {
                event.event_type == EventKind::NeedDeltaApplied
                    && (event.actor_id.as_ref() == Some(actor_id)
                        || payload_value(event, "actor_id") == Some(actor_id.as_str()))
            })
            .map(|event| event.event_id.clone())
    }

    fn routine_window_family(
        agent_state: &AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
        actor_known_state: &ActorKnownPlanningContext,
    ) -> Option<RoutineFamily> {
        let family = eligible_routine_execution_for_actor(agent_state, actor_id, window)
            .map(|(_, execution)| execution.family)?;
        if family == RoutineFamily::WorkBlock
            && !actor_known_state
                .known_workplaces()
                .values()
                .any(|place_id| place_id == actor_known_state.current_place_id())
        {
            Some(RoutineFamily::GoToWork)
        } else {
            Some(family)
        }
    }

    fn eligible_routine_execution_for_actor<'a>(
        agent_state: &'a AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
    ) -> Option<(&'a RoutineExecutionId, &'a crate::agent::RoutineExecution)> {
        agent_state
            .routine_executions
            .iter()
            .filter(|(_, execution)| &execution.actor_id == actor_id)
            .filter(|(_, execution)| {
                execution.start_tick <= window.start_tick
                    && execution
                        .deadline_tick
                        .is_none_or(|deadline| window.start_tick < deadline)
            })
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
    }

    fn active_intention_for_actor(
        agent_state: &AgentState,
        actor_id: &ActorId,
    ) -> Option<Intention> {
        let intention_id = agent_state.active_intention_by_actor.get(actor_id)?;
        agent_state.intentions.get(intention_id).cloned()
    }

    #[allow(clippy::too_many_arguments)]
    fn advance_no_human_scheduler_to_tick(
        state: &PhysicalState,
        agent_state: &mut AgentState,
        log: &mut EventLog,
        process_id: &ProcessId,
        scheduler: &mut DeterministicScheduler,
        content_manifest_id: &ContentManifestId,
        target_tick: SimTick,
        scheduler_errors: &mut Vec<NoHumanSchedulerError>,
    ) -> Vec<(ActorId, SimTick)> {
        let mut completed_durations = Vec::new();
        while scheduler.current_tick < target_tick {
            let request = WorldAdvanceRequest {
                expected_tick: scheduler.current_tick,
                origin: WorldAdvanceOrigin::Process(process_id.clone()),
                content_manifest_id: content_manifest_id.clone(),
                authorized_sleep_interruptions: Vec::new(),
            };
            let result = match scheduler.advance_world_one_tick(state, agent_state, log, request) {
                Ok(result) => result,
                Err(error) => {
                    scheduler_errors.push(NoHumanSchedulerError::WorldAdvance { error });
                    break;
                }
            };
            completed_durations.extend(record_no_human_duration_completions(
                log,
                agent_state,
                process_id,
                content_manifest_id,
                &result.appended_event_ids,
            ));
        }
        completed_durations
    }

    fn record_no_human_duration_completions(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        content_manifest_id: &ContentManifestId,
        appended_event_ids: &[EventId],
    ) -> Vec<(ActorId, SimTick)> {
        let appended_events = appended_event_ids
            .iter()
            .filter_map(|event_id| {
                log.events()
                    .iter()
                    .find(|event| &event.event_id == event_id)
                    .cloned()
            })
            .collect::<Vec<_>>();
        let mut completed_durations = Vec::new();
        for appended in appended_events {
            if is_duration_terminal(appended.event_type) {
                if let Some(actor_id) = appended.actor_id.clone() {
                    completed_durations.push((actor_id, appended.sim_tick));
                }
                append_routine_step_completed_after_duration_completion(
                    log,
                    agent_state,
                    process_id,
                    content_manifest_id,
                    &appended,
                );
            }
        }
        completed_durations
    }

    fn sync_scheduler_frontier_to_appended_events(
        scheduler: &mut DeterministicScheduler,
        appended_events: &[EventEnvelope],
    ) {
        if let Some(latest_tick) = appended_events
            .iter()
            .map(|event| event.sim_tick)
            .max()
            .filter(|tick| *tick > scheduler.current_tick)
        {
            scheduler.current_tick = latest_tick;
        }
    }

    fn credit_completion(
        progress_by_window_actor: &mut BTreeMap<(String, ActorId), usize>,
        windows: &[DayWindow],
        completed_actor_id: &ActorId,
        completion_tick: SimTick,
    ) {
        for window in windows {
            if window.contains_tick(completion_tick) {
                progress_by_window_actor
                    .insert((window.window_id.clone(), completed_actor_id.clone()), 1);
            }
        }
    }

    fn append_routine_step_completed_after_duration_completion(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        content_manifest_id: &ContentManifestId,
        completion_event: &EventEnvelope,
    ) {
        let (family, ordinary_action_id) = match completion_event.event_type {
            EventKind::SleepCompleted => (RoutineFamily::SleepNight, "sleep"),
            EventKind::WorkBlockCompleted => (RoutineFamily::WorkBlock, "work_block"),
            _ => return,
        };
        let Some(actor_id) = completion_event.actor_id.as_ref() else {
            return;
        };
        let Some(execution_id) = agent_state
            .routine_executions
            .iter()
            .filter(|(_, execution)| &execution.actor_id == actor_id)
            .filter(|(_, execution)| execution.family == family)
            .filter(|(_, execution)| execution.start_tick <= completion_event.sim_tick)
            .filter(|(_, execution)| {
                execution.step_status == crate::agent::RoutineStepStatus::InProgress
            })
            .min_by(|(_, left), (_, right)| {
                left.start_tick
                    .cmp(&right.start_tick)
                    .then_with(|| left.execution_id.cmp(&right.execution_id))
            })
            .map(|(execution_id, _)| execution_id.clone())
        else {
            return;
        };
        append_and_apply_agent_event(
            log,
            agent_state,
            build_duration_routine_step_completed_event(
                process_id,
                &execution_id,
                actor_id,
                ordinary_action_id,
                content_manifest_id,
                completion_event,
            ),
        );
    }

    fn build_duration_routine_step_completed_event(
        process_id: &ProcessId,
        execution_id: &RoutineExecutionId,
        actor_id: &ActorId,
        ordinary_action_id: &str,
        content_manifest_id: &ContentManifestId,
        completion_event: &EventEnvelope,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.routine_step_completed.{}.{}",
                execution_id.as_str(),
                completion_event.event_id.as_str()
            ))
            .unwrap(),
            EventKind::RoutineStepCompleted,
            0,
            0,
            completion_event.sim_tick,
            OrderingKey::new(
                completion_event.sim_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("routine_step_completed").unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    execution_id.as_str().to_string(),
                ],
                format!(
                    "routine_step_completed:{}:{}",
                    actor_id.as_str(),
                    completion_event.event_id.as_str()
                ),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(completion_event.event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.proposal_id = completion_event.proposal_id.clone();
        event.participants = vec![actor_id.to_string(), execution_id.to_string()];
        event.payload = vec![
            PayloadField::new("routine_execution_id", execution_id.as_str()),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new(
                "progress_tick",
                completion_event.sim_tick.value().to_string(),
            ),
            PayloadField::new("ordinary_event_id", completion_event.event_id.as_str()),
            PayloadField::new("ordinary_action_id", ordinary_action_id),
        ];
        event.effects_summary =
            "duration routine step completed with ordinary completion ancestry".to_string();
        event
    }

    fn scheduled_completion_tick(event: &EventEnvelope) -> Option<SimTick> {
        event
            .payload
            .iter()
            .find(|field| field.key == "expected_completion_tick")
            .and_then(|field| field.value.parse::<u64>().ok())
            .map(SimTick::new)
    }

    fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
        event
            .payload
            .iter()
            .find(|field| field.key == key)
            .map(|field| field.value.as_str())
    }

    fn actor_has_open_body_exclusive_duration(
        log: &EventLog,
        actor_id: &ActorId,
        tick: SimTick,
    ) -> Result<bool, crate::need_accounting::DuplicateDurationTerminal> {
        Ok(
            crate::need_accounting::open_body_exclusive_starts(log, actor_id, tick)?
                .iter()
                .any(|event_id| {
                    log.events().iter().any(|event| {
                        &event.event_id == event_id
                            && payload_value(event, "body_exclusive") == Some("true")
                            && scheduled_completion_tick(event)
                                .is_some_and(|completion| completion > tick)
                    })
                }),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn append_routine_stuck_diagnostics(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
        stuck_diagnostic_event_ids: &mut Vec<EventId>,
        scheduler_errors: &mut Vec<NoHumanSchedulerError>,
    ) {
        let diagnostics = routine_stuck_diagnostic_kinds(agent_state, actor_id, window);
        for (kind, execution_id) in diagnostics {
            let event = append_stuck_diagnostic(
                log,
                process_id,
                actor_id,
                window,
                content_manifest_id,
                kind,
                Some(execution_id),
            );
            match apply_agent_event(agent_state, &event) {
                Ok(_) => stuck_diagnostic_event_ids.push(event.event_id),
                Err(error) => scheduler_errors.push(NoHumanSchedulerError::AgentApply {
                    event_id: event.event_id,
                    error,
                }),
            }
        }
    }

    fn routine_stuck_diagnostic_kinds(
        agent_state: &AgentState,
        actor_id: &ActorId,
        window: &DayWindow,
    ) -> Vec<(StuckDiagnosticKind, RoutineExecutionId)> {
        agent_state
            .routine_executions()
            .iter()
            .filter(|(_, execution)| &execution.actor_id == actor_id)
            .flat_map(|(execution_id, execution)| {
                let mut diagnostics = Vec::new();
                if execution
                    .expected_next_progress_tick
                    .is_some_and(|expected| {
                        expected < window.start_tick && execution.last_progress_tick < expected
                    })
                {
                    diagnostics.push((
                        StuckDiagnosticKind::PastExpectedProgressWindow,
                        execution_id.clone(),
                    ));
                }
                if execution.step_status == RoutineStepStatus::Waiting
                    && execution.fallback_attempts > 0
                    && execution.last_progress_tick < window.start_tick
                {
                    diagnostics.push((StuckDiagnosticKind::RepeatedIdleWait, execution_id.clone()));
                }
                diagnostics
            })
            .filter(|(kind, execution_id)| {
                !agent_state.stuck_diagnostics().contains_key(
                    &StuckDiagnosticId::new(format!(
                        "diagnostic_{}_{}_{}_{}",
                        kind.stable_id(),
                        actor_id.as_str(),
                        window.window_id,
                        execution_id.as_str()
                    ))
                    .unwrap(),
                )
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn append_decision_trace_after_proposal(
        log: &mut EventLog,
        agent_state: &mut AgentState,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        decision_trace_record: &DecisionTraceRecord,
        content_manifest_id: &ContentManifestId,
        ordinary_event: Option<&EventEnvelope>,
    ) {
        let Some(ordinary_event) = ordinary_event else {
            return;
        };
        append_and_apply_agent_event(
            log,
            agent_state,
            build_decision_trace_event(
                process_id,
                actor_id,
                window,
                proposal,
                decision_trace_record,
                content_manifest_id,
                ordinary_event,
            ),
        );
    }

    fn build_decision_trace_event(
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        proposal: &Proposal,
        decision_trace_record: &DecisionTraceRecord,
        content_manifest_id: &ContentManifestId,
        ordinary_event: &EventEnvelope,
    ) -> EventEnvelope {
        let trace_id = decision_trace_record.trace_id.clone();
        let trace_canonical = decision_trace_record.serialize_canonical();
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.decision_trace.{}.{}",
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap(),
            EventKind::DecisionTraceRecorded,
            0,
            0,
            window.start_tick,
            OrderingKey::new(
                window.start_tick,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(process_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("decision_trace_recorded").unwrap(),
                vec![actor_id.as_str().to_string(), trace_id.as_str().to_string()],
                format!("decision_trace:{}:{}", actor_id.as_str(), window.window_id),
            ),
            content_manifest_id.clone(),
            vec![EventCause::Event(ordinary_event.event_id.clone())],
        )
        .unwrap();
        event.actor_id = Some(actor_id.clone());
        event.process_id = Some(process_id.clone());
        event.proposal_id = Some(proposal.proposal_id.clone());
        event.participants = vec![actor_id.to_string(), trace_id.to_string()];
        event.payload = vec![
            PayloadField::new("trace_schema_version", "1"),
            PayloadField::new("trace_id", trace_id.as_str()),
            PayloadField::new("trace_canonical", trace_canonical),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("window_id", window.window_id.as_str()),
            PayloadField::new("window_start_tick", window.start_tick.value().to_string()),
            PayloadField::new("window_end_tick", window.end_tick.value().to_string()),
            PayloadField::new("action_id", proposal.action_id.as_str()),
            PayloadField::new("ordinary_event_id", ordinary_event.event_id.as_str()),
            PayloadField::new(
                "hidden_truth_audit_actor_known_only",
                decision_trace_record
                    .hidden_truth_audit_result
                    .actor_known_only
                    .to_string(),
            ),
            PayloadField::new(
                "hidden_truth_audit_notes",
                decision_trace_record
                    .hidden_truth_audit_result
                    .notes
                    .clone(),
            ),
            PayloadField::new(
                "responsible_layer",
                decision_trace_record
                    .typed_diagnostic
                    .responsible_layer
                    .stable_id(),
            ),
            PayloadField::new(
                "blocker_code",
                decision_trace_record
                    .typed_diagnostic
                    .blocker_code
                    .stable_id(),
            ),
            PayloadField::new(
                "input_source",
                decision_trace_record.typed_diagnostic.input_source.clone(),
            ),
            PayloadField::new(
                "actual_source",
                decision_trace_record.typed_diagnostic.actual_source.clone(),
            ),
            PayloadField::new(
                "hidden_truth_referenced",
                decision_trace_record
                    .typed_diagnostic
                    .hidden_truth_referenced
                    .to_string(),
            ),
            PayloadField::new(
                "remediation_hint",
                decision_trace_record
                    .typed_diagnostic
                    .remediation_hint
                    .clone(),
            ),
        ];
        event.effects_summary =
            "no-human decision trace recorded from autonomous proposal".to_string();
        event
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

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    fn build_window_passive_need_events(
        current_value: Option<u16>,
        has_active_intention: bool,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
        need_kind: NeedKind,
        delta: i32,
        elapsed_ticks: u64,
    ) -> Vec<EventEnvelope> {
        let mut events = build_need_delta_and_threshold_events(
            NeedDeltaEventSpec {
                event_id: EventId::new(format!(
                    "event.no_human_passive_need_delta.{}.{}.{}",
                    window.window_id,
                    actor_id.as_str(),
                    need_kind.stable_id()
                ))
                .unwrap(),
                threshold_event_id: EventId::new(format!(
                    "event.no_human_need_threshold.{}.{}.{}",
                    window.window_id,
                    actor_id.as_str(),
                    need_kind.stable_id()
                ))
                .unwrap(),
                tick: window.start_tick,
                ordering_key: OrderingKey::new(
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
                content_manifest_id: content_manifest_id.clone(),
                causes: vec![EventCause::Process(process_id.clone())],
                actor_id: actor_id.clone(),
                proposal_id: None,
                process_id: Some(process_id.clone()),
                participants: vec![actor_id.to_string()],
                need_kind,
                delta,
                elapsed_ticks,
                cause_kind: "tick_delta".to_string(),
                cause_action_id: None,
                extra_payload: vec![PayloadField::new("window_id", window.window_id.as_str())],
                delta_effects_summary: format!(
                    "{} changed by {} before {} decision",
                    need_kind.stable_id(),
                    delta,
                    window.window_id
                ),
                threshold_effects_summary: format!(
                    "{} crossed threshold before {} decision",
                    need_kind.stable_id(),
                    window.window_id
                ),
            },
            current_value,
        );
        for event in &mut events {
            if event.event_type != EventKind::NeedThresholdCrossed {
                continue;
            }
            let severe_need_interrupts = event
                .payload
                .iter()
                .any(|field| field.key == "to_band" && field.value == "severe")
                && has_active_intention;
            event
                .payload
                .push(PayloadField::new("window_id", window.window_id.as_str()));
            event.payload.push(PayloadField::new(
                "severe_need_interrupts_active_intention",
                severe_need_interrupts.to_string(),
            ));
            event.payload.push(PayloadField::new(
                "interruption_cause",
                if severe_need_interrupts {
                    "severe_need_pressure"
                } else {
                    "none"
                },
            ));
        }
        events
    }

    #[cfg(test)]
    fn latest_action_tick_delta_tick(
        events: &[EventEnvelope],
        actor_id: &ActorId,
    ) -> Option<SimTick> {
        events
            .iter()
            .filter(|event| {
                event.event_type == EventKind::NeedDeltaApplied
                    && event.actor_id.as_ref() == Some(actor_id)
                    && payload_value(event, "cause_kind") == Some("tick_delta")
                    && event.causes.iter().any(|cause| {
                        matches!(cause, EventCause::Proposal(_) | EventCause::Event(_))
                    })
            })
            .map(|event| event.sim_tick)
            .max()
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
        decision_trace_record: &DecisionTraceRecord,
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
                decision_trace_record,
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
        decision_trace_record: &DecisionTraceRecord,
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
        let trace_id = decision_trace_record.trace_id.clone();
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
        let Some(execution_id) = active_routine_execution_for_actor(agent_state, actor_id, window)
        else {
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
        let execution_family = execution.family;
        let execution_step_status = execution.step_status;
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
        if execution_step_status == crate::agent::RoutineStepStatus::NotStarted {
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
        if is_instant_routine_progress_event(execution_family, proposal, ordinary_event) {
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
        window: &DayWindow,
    ) -> Option<RoutineExecutionId> {
        eligible_routine_execution_for_actor(agent_state, actor_id, window)
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

    fn is_instant_routine_progress_event(
        execution_family: RoutineFamily,
        proposal: &Proposal,
        event: &EventEnvelope,
    ) -> bool {
        if execution_family == RoutineFamily::WorkBlock && proposal.action_id.as_str() == "move" {
            return false;
        }
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
        let mut scheduler_errors = Vec::new();
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
            advance_no_human_scheduler_to_tick(
                physical_state,
                agent_state,
                log,
                &process_id,
                &mut scheduler,
                &content_manifest_id,
                proposal.requested_tick,
                &mut scheduler_errors,
            );
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
            sync_scheduler_frontier_to_appended_events(&mut scheduler, &result.appended_events);
            ordinary_pipeline_events += no_human_progress_event_count(&result.appended_events);
        }

        let final_tick = start_tick.advance_by(tick_count);
        advance_no_human_scheduler_to_tick(
            physical_state,
            agent_state,
            log,
            &process_id,
            &mut scheduler,
            &content_manifest_id,
            final_tick,
            &mut scheduler_errors,
        );

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
        if event.event_type == EventKind::ActionRejected {
            return false;
        }
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
        event.causes = vec![EventCause::Process(process_id.clone())];
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum StuckDiagnosticKind {
        WindowNoProgress,
        PastExpectedProgressWindow,
        RepeatedIdleWait,
    }

    impl StuckDiagnosticKind {
        const fn stable_id(self) -> &'static str {
            match self {
                Self::WindowNoProgress => "window_no_progress",
                Self::PastExpectedProgressWindow => "past_expected_progress_window",
                Self::RepeatedIdleWait => "repeated_idle_wait",
            }
        }

        const fn concrete_blocker(self) -> &'static str {
            match self {
                Self::WindowNoProgress => "no progress recorded in no-human day window",
                Self::PastExpectedProgressWindow => "no progress past expected progress window",
                Self::RepeatedIdleWait => "repeated idle/wait without typed progress reason",
            }
        }

        const fn actual_source(self) -> &'static str {
            match self {
                Self::WindowNoProgress => "scheduler_no_progress_detection",
                Self::PastExpectedProgressWindow => {
                    "routine_expected_next_progress_stuck_detection"
                }
                Self::RepeatedIdleWait => "routine_repeated_idle_wait_stuck_detection",
            }
        }

        const fn resulting_status(self) -> StuckResultingStatus {
            match self {
                Self::WindowNoProgress => StuckResultingStatus::Idle,
                Self::PastExpectedProgressWindow | Self::RepeatedIdleWait => {
                    StuckResultingStatus::Waiting
                }
            }
        }
    }

    fn append_stuck_diagnostic(
        log: &mut EventLog,
        process_id: &ProcessId,
        actor_id: &ActorId,
        window: &DayWindow,
        content_manifest_id: &ContentManifestId,
        kind: StuckDiagnosticKind,
        routine_execution_id: Option<RoutineExecutionId>,
    ) -> EventEnvelope {
        let diagnostic_id = if let Some(execution_id) = routine_execution_id.as_ref() {
            StuckDiagnosticId::new(format!(
                "diagnostic_{}_{}_{}_{}",
                kind.stable_id(),
                actor_id.as_str(),
                window.window_id,
                execution_id.as_str()
            ))
            .unwrap()
        } else {
            StuckDiagnosticId::new(format!(
                "diagnostic_{}_{}_{}",
                kind.stable_id(),
                actor_id.as_str(),
                window.window_id
            ))
            .unwrap()
        };
        let diagnostic = StuckDiagnostic::new(
            diagnostic_id,
            actor_id.clone(),
            window.start_tick,
            window.end_tick,
            None,
            None,
            None,
            None,
            routine_execution_id,
            None,
            None,
            BlockerCategory::SchedulingReservation,
            kind.concrete_blocker(),
            format!(
                "actor {} triggered {} in {}",
                actor_id.as_str(),
                kind.stable_id(),
                window.window_id,
            ),
            "no-human day stuck detection",
            "recorded_stuck_diagnostic",
            kind.resulting_status(),
        )
        .with_typed_diagnostic(TypedDiagnosticFields {
            responsible_layer: ResponsibleLayer::Scheduler,
            blocker_code: BlockerCode::SchedulingReservation,
            input_source: "holder_known_context".to_string(),
            actual_source: kind.actual_source().to_string(),
            hidden_truth_referenced: false,
            remediation_hint: "inspect no-human ordering and proposal diagnostics".to_string(),
        });
        let canonical = diagnostic.serialize_canonical();
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(format!(
                "event.stuck_diagnostic_recorded.{}.{}.{}",
                kind.stable_id(),
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
            PayloadField::new("diagnostic_schema_version", "1"),
            PayloadField::new("diagnostic_id", diagnostic.diagnostic_id.as_str()),
            PayloadField::new("diagnostic_canonical", canonical),
            PayloadField::new(
                "responsible_layer",
                diagnostic.typed_diagnostic.responsible_layer.stable_id(),
            ),
            PayloadField::new(
                "blocker_code",
                diagnostic.typed_diagnostic.blocker_code.stable_id(),
            ),
            PayloadField::new(
                "input_source",
                diagnostic.typed_diagnostic.input_source.clone(),
            ),
            PayloadField::new(
                "actual_source",
                diagnostic.typed_diagnostic.actual_source.clone(),
            ),
            PayloadField::new(
                "hidden_truth_referenced",
                diagnostic
                    .typed_diagnostic
                    .hidden_truth_referenced
                    .to_string(),
            ),
            PayloadField::new(
                "remediation_hint",
                diagnostic.typed_diagnostic.remediation_hint.clone(),
            ),
        ];
        event.effects_summary = "no-human day stuck diagnostic recorded".to_string();
        log.append(event).expect("stuck diagnostic is versioned")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::actions::proposal::{Proposal, ProposalOrigin};
        use crate::agent::{Intention, IntentionSource};
        use crate::events::apply::{apply_agent_event, apply_event};
        use crate::events::{EventCause, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1};
        use crate::ids::{
            ActorId, CandidateGoalId, DecisionTraceId, FoodSupplyId, IntentionId, PlaceId,
            ProposalId, RoutineExecutionId, RoutineTemplateId, SemanticActionId, SleepAffordanceId,
            WorkplaceId,
        };
        use crate::location::Location;
        use crate::state::{
            ActorBody, AgentState, FoodSupplyState, PlaceState, SleepAffordanceState,
            WorkplaceState,
        };

        #[test]
        fn day_window_contains_tick_uses_inclusive_bounds() {
            let window = DayWindow {
                window_id: "window_morning".to_string(),
                start_tick: SimTick::new(10),
                end_tick: SimTick::new(20),
            };

            // A tick strictly inside the window is attributed to it.
            assert!(window.contains_tick(SimTick::new(15)));

            // Both bounds are inclusive. These boundary ticks kill the
            // `<= -> >` mutants on the start and end comparisons, which would
            // otherwise drop a completion landing exactly on a window edge.
            assert!(window.contains_tick(SimTick::new(10)));
            assert!(window.contains_tick(SimTick::new(20)));

            // Ticks just outside either bound are excluded. These kill the
            // `&& -> ||` mutant, which would credit a window for a completion
            // that satisfied only one bound.
            assert!(!window.contains_tick(SimTick::new(9)));
            assert!(!window.contains_tick(SimTick::new(21)));
        }

        #[test]
        fn default_day_windows_are_disjoint_and_cover() {
            let windows = default_day_windows(SimTick::ZERO);

            assert_eq!(
                windows
                    .iter()
                    .map(|window| {
                        (
                            window.window_id.as_str(),
                            window.start_tick.value(),
                            window.end_tick.value(),
                        )
                    })
                    .collect::<Vec<_>>(),
                vec![
                    ("pre_dawn", 0, 3),
                    ("morning", 4, 9),
                    ("work_window", 10, 17),
                    ("evening", 18, 23),
                    ("night", 24, 32),
                ]
            );

            for tick in 0..=32 {
                let containing_windows = windows
                    .iter()
                    .filter(|window| window.contains_tick(SimTick::new(tick)))
                    .map(|window| window.window_id.as_str())
                    .collect::<Vec<_>>();
                assert_eq!(
                    containing_windows.len(),
                    1,
                    "tick {tick} must belong to exactly one default day window: {containing_windows:?}"
                );
            }
        }

        fn threshold_payload<'a>(event: &'a EventEnvelope, key: &str) -> &'a str {
            event
                .payload
                .iter()
                .find(|field| field.key == key)
                .map(|field| field.value.as_str())
                .unwrap_or_else(|| panic!("missing payload field {key}"))
        }

        fn passive_threshold_event(
            current_value: u16,
            delta: i32,
            has_active_intention: bool,
        ) -> EventEnvelope {
            let process = ProcessId::new("process_passive_needs").unwrap();
            let actor = actor_id();
            let window = DayWindow {
                window_id: "window_morning".to_string(),
                start_tick: SimTick::new(4),
                end_tick: SimTick::new(8),
            };
            build_window_passive_need_events(
                Some(current_value),
                has_active_intention,
                &process,
                &actor,
                &window,
                &content_manifest_id(),
                NeedKind::Hunger,
                delta,
                1,
            )
            .into_iter()
            .find(|event| event.event_type == EventKind::NeedThresholdCrossed)
            .expect("passive need delta should cross a threshold")
        }

        #[test]
        fn severe_passive_need_interrupt_payload_requires_severe_crossing_and_active_intention() {
            let severe_with_intention = passive_threshold_event(749, 1, true);
            assert_eq!(
                threshold_payload(&severe_with_intention, "to_band"),
                "severe"
            );
            assert_eq!(
                threshold_payload(
                    &severe_with_intention,
                    "severe_need_interrupts_active_intention"
                ),
                "true"
            );
            assert_eq!(
                threshold_payload(&severe_with_intention, "interruption_cause"),
                "severe_need_pressure"
            );

            let severe_without_intention = passive_threshold_event(749, 1, false);
            assert_eq!(
                threshold_payload(
                    &severe_without_intention,
                    "severe_need_interrupts_active_intention"
                ),
                "false"
            );
            assert_eq!(
                threshold_payload(&severe_without_intention, "interruption_cause"),
                "none"
            );

            let non_severe_with_intention = passive_threshold_event(249, 1, true);
            assert_eq!(
                threshold_payload(&non_severe_with_intention, "to_band"),
                "rising"
            );
            assert_eq!(
                threshold_payload(
                    &non_severe_with_intention,
                    "severe_need_interrupts_active_intention"
                ),
                "false"
            );
            assert_eq!(
                threshold_payload(&non_severe_with_intention, "interruption_cause"),
                "none"
            );
        }

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

        fn ordinary_event(
            event_id: &str,
            event_kind: EventKind,
            actor_id: &ActorId,
            action_id: &str,
            tick: SimTick,
        ) -> EventEnvelope {
            let mut event = EventEnvelope::new_caused_v1(
                EventId::new(event_id).unwrap(),
                event_kind,
                0,
                0,
                tick,
                OrderingKey::new(
                    tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id.clone()),
                    ProposalSequence::new(0),
                    ActionId::new(action_id).unwrap(),
                    vec![actor_id.as_str().to_string()],
                    event_id,
                ),
                content_manifest_id(),
                vec![EventCause::Proposal(
                    ProposalId::new(format!("proposal_{event_id}")).unwrap(),
                )],
            )
            .unwrap();
            event.actor_id = Some(actor_id.clone());
            event.proposal_id = Some(ProposalId::new(format!("proposal_{event_id}")).unwrap());
            event.payload = vec![PayloadField::new("actor_id", actor_id.as_str())];
            event
        }

        fn in_progress_execution(
            execution_id: &str,
            actor_id: &ActorId,
            family: RoutineFamily,
            start_tick: SimTick,
            action_id: &str,
        ) -> crate::agent::RoutineExecution {
            let mut execution = crate::agent::RoutineExecution::new(
                RoutineExecutionId::new(execution_id).unwrap(),
                actor_id.clone(),
                RoutineTemplateId::new(format!("template_{execution_id}")).unwrap(),
                family,
                start_tick,
                Some(start_tick.next()),
                Some(start_tick.advance_by(6)),
                None,
                DecisionTraceId::new(format!("trace_{execution_id}")).unwrap(),
            );
            execution.start_step(
                start_tick,
                SemanticActionId::new(action_id.replace('_', ".")).unwrap(),
            );
            execution
        }

        #[test]
        fn duration_completion_appends_exact_matching_routine_step_once() {
            let process_id = ProcessId::new("process_duration_completion_witness").unwrap();
            for (event_kind, family, ordinary_action_id, suffix) in [
                (
                    EventKind::SleepCompleted,
                    RoutineFamily::SleepNight,
                    "sleep",
                    "sleep",
                ),
                (
                    EventKind::WorkBlockCompleted,
                    RoutineFamily::WorkBlock,
                    "work_block",
                    "work",
                ),
            ] {
                let actor_id = actor_id();
                let other_actor = ActorId::new(format!("actor_other_{suffix}")).unwrap();
                let matching_id =
                    RoutineExecutionId::new(format!("routine_exec_matching_{suffix}")).unwrap();
                let other_actor_id =
                    RoutineExecutionId::new(format!("routine_exec_other_actor_{suffix}")).unwrap();
                let other_family_id =
                    RoutineExecutionId::new(format!("routine_exec_other_family_{suffix}")).unwrap();
                let future_start_id =
                    RoutineExecutionId::new(format!("routine_exec_future_start_{suffix}")).unwrap();
                let completed_status_id =
                    RoutineExecutionId::new(format!("routine_exec_completed_status_{suffix}"))
                        .unwrap();
                let mut agent_state = agent_state(&actor_id);
                agent_state.routine_executions.insert(
                    matching_id.clone(),
                    in_progress_execution(
                        matching_id.as_str(),
                        &actor_id,
                        family,
                        SimTick::new(2),
                        ordinary_action_id,
                    ),
                );
                agent_state.routine_executions.insert(
                    other_actor_id.clone(),
                    in_progress_execution(
                        other_actor_id.as_str(),
                        &other_actor,
                        family,
                        SimTick::new(1),
                        ordinary_action_id,
                    ),
                );
                agent_state.routine_executions.insert(
                    other_family_id.clone(),
                    in_progress_execution(
                        other_family_id.as_str(),
                        &actor_id,
                        if family == RoutineFamily::SleepNight {
                            RoutineFamily::WorkBlock
                        } else {
                            RoutineFamily::SleepNight
                        },
                        SimTick::new(1),
                        ordinary_action_id,
                    ),
                );
                agent_state.routine_executions.insert(
                    future_start_id.clone(),
                    in_progress_execution(
                        future_start_id.as_str(),
                        &actor_id,
                        family,
                        SimTick::new(8),
                        ordinary_action_id,
                    ),
                );
                let mut completed_status = in_progress_execution(
                    completed_status_id.as_str(),
                    &actor_id,
                    family,
                    SimTick::ZERO,
                    ordinary_action_id,
                );
                completed_status.complete_step(SimTick::new(1));
                agent_state
                    .routine_executions
                    .insert(completed_status_id.clone(), completed_status);
                let mut log = EventLog::new();
                let completion = ordinary_event(
                    &format!("event_{suffix}_completed_duration_witness"),
                    event_kind,
                    &actor_id,
                    ordinary_action_id,
                    SimTick::new(5),
                );

                append_routine_step_completed_after_duration_completion(
                    &mut log,
                    &mut agent_state,
                    &process_id,
                    &content_manifest_id(),
                    &completion,
                );

                let routine_events = log
                    .events()
                    .iter()
                    .filter(|event| event.event_type == EventKind::RoutineStepCompleted)
                    .collect::<Vec<_>>();
                assert_eq!(routine_events.len(), 1, "{suffix}");
                let routine_event = routine_events[0];
                assert_eq!(routine_event.sim_tick, SimTick::new(5));
                assert_eq!(
                    routine_event.ordering_key.action_id.as_str(),
                    "routine_step_completed"
                );
                assert!(routine_event
                    .causes
                    .iter()
                    .any(|cause| cause == &EventCause::Event(completion.event_id.clone())));
                assert!(routine_event.payload.iter().any(|field| {
                    field.key == "routine_execution_id" && field.value == matching_id.as_str()
                }));
                assert!(routine_event.payload.iter().any(|field| {
                    field.key == "ordinary_action_id" && field.value == ordinary_action_id
                }));
                assert_eq!(
                    agent_state.routine_executions[&matching_id].step_status,
                    RoutineStepStatus::Completed,
                    "{suffix}"
                );
                for decoy_id in [
                    other_actor_id,
                    other_family_id,
                    future_start_id,
                    completed_status_id,
                ] {
                    assert_ne!(
                        routine_event
                            .payload
                            .iter()
                            .find(|field| field.key == "routine_execution_id")
                            .map(|field| field.value.as_str()),
                        Some(decoy_id.as_str()),
                        "{suffix}"
                    );
                }
            }
        }

        #[test]
        fn work_move_starts_but_non_move_progress_completes_routine_step() {
            let process_id = ProcessId::new("process_instant_progress_witness").unwrap();
            let actor_id = actor_id();
            let window = DayWindow {
                window_id: "work_window".to_string(),
                start_tick: SimTick::new(4),
                end_tick: SimTick::new(8),
            };

            let move_execution_id = RoutineExecutionId::new("routine_exec_work_move").unwrap();
            let mut move_agent_state = agent_state(&actor_id);
            move_agent_state.routine_executions.insert(
                move_execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    move_execution_id.clone(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_work_move").unwrap(),
                    RoutineFamily::WorkBlock,
                    SimTick::new(4),
                    Some(SimTick::new(5)),
                    Some(SimTick::new(8)),
                    None,
                    DecisionTraceId::new("trace_work_move").unwrap(),
                ),
            );
            let move_proposal = Proposal::new(
                ProposalId::new("proposal_work_move").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id.clone()),
                ActionId::new("move").unwrap(),
                SimTick::new(4),
            );
            let move_event = ordinary_event(
                "event_work_move_progress",
                EventKind::ActorMoved,
                &actor_id,
                "move",
                SimTick::new(4),
            );
            let mut move_log = EventLog::new();

            append_routine_step_events_after_proposal(
                &mut move_log,
                &mut move_agent_state,
                &process_id,
                &actor_id,
                &window,
                &move_proposal,
                &content_manifest_id(),
                Some(&move_event),
            );

            assert!(move_log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::RoutineStepStarted));
            assert!(!move_log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::RoutineStepCompleted));
            assert_eq!(
                move_agent_state.routine_executions[&move_execution_id].step_status,
                RoutineStepStatus::InProgress
            );

            let work_execution_id = RoutineExecutionId::new("routine_exec_work_wait").unwrap();
            let mut work_agent_state = agent_state(&actor_id);
            work_agent_state.routine_executions.insert(
                work_execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    work_execution_id.clone(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_work_wait").unwrap(),
                    RoutineFamily::WorkBlock,
                    SimTick::new(4),
                    Some(SimTick::new(5)),
                    Some(SimTick::new(8)),
                    None,
                    DecisionTraceId::new("trace_work_wait").unwrap(),
                ),
            );
            let work_proposal = Proposal::new(
                ProposalId::new("proposal_work_wait").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id.clone()),
                ActionId::new("work_block").unwrap(),
                SimTick::new(4),
            );
            let work_event = ordinary_event(
                "event_work_non_move_progress",
                EventKind::ActorWaited,
                &actor_id,
                "work_block",
                SimTick::new(4),
            );
            let mut work_log = EventLog::new();

            append_routine_step_events_after_proposal(
                &mut work_log,
                &mut work_agent_state,
                &process_id,
                &actor_id,
                &window,
                &work_proposal,
                &content_manifest_id(),
                Some(&work_event),
            );

            assert!(work_log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::RoutineStepStarted));
            let completed = work_log
                .events()
                .iter()
                .find(|event| event.event_type == EventKind::RoutineStepCompleted)
                .expect("non-move work-family event completes instant progress");
            assert!(completed
                .payload
                .iter()
                .any(|field| { field.key == "ordinary_action_id" && field.value == "work_block" }));
            assert_eq!(
                work_agent_state.routine_executions[&work_execution_id].step_status,
                RoutineStepStatus::Completed
            );
        }

        #[test]
        fn advance_runs_without_controller_and_marks_diagnostic_stream() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            assert_eq!(log.events().len(), 4);
            assert_eq!(
                log.events()
                    .iter()
                    .filter(|event| event.stream == EventStream::Diagnostic)
                    .count(),
                2
            );
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::TimeAdvanced));
        }

        #[test]
        fn diagnostic_markers_are_physical_noops_under_replay() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            let mut proposal = proposal;
            proposal
                .parameters
                .insert("reason".to_string(), "scheduled wait".to_string());

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
        fn advance_catches_up_intervening_ticks_before_later_proposal() {
            let actor_id = actor_id();
            let bedroom = PlaceId::new("bedroom").unwrap();
            let sleep_affordance_id = SleepAffordanceId::new("bed_tomas").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state
                .places
                .insert(bedroom.clone(), PlaceState::new(bedroom.clone(), "Bedroom"));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), bedroom.clone()),
            );
            state.sleep_affordances.insert(
                sleep_affordance_id.clone(),
                SleepAffordanceState::new(sleep_affordance_id.clone(), bedroom, 2, 80, 1),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();
            registry.register_phase3a_sleep();
            let mut agent_state = agent_state(&actor_id);

            let mut sleep = Proposal::new(
                ProposalId::new("proposal_sleep").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id.clone()),
                ActionId::new("sleep").unwrap(),
                SimTick::ZERO,
            );
            sleep.parameters.insert(
                "sleep_affordance_id".to_string(),
                sleep_affordance_id.as_str().to_string(),
            );
            sleep
                .parameters
                .insert("duration_ticks".to_string(), "2".to_string());

            let mut wait = Proposal::new(
                ProposalId::new("proposal_after_sleep_wait").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id),
                ActionId::new("wait").unwrap(),
                SimTick::new(3),
            );
            wait.parameters
                .insert("reason".to_string(), "after sleep".to_string());

            let report = advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: &mut agent_state,
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                4,
                vec![sleep, wait],
            );

            assert_eq!(report.final_tick, SimTick::new(4));
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::SleepStarted));
            let completed_index = log
                .events()
                .iter()
                .position(|event| event.event_type == EventKind::SleepCompleted)
                .expect("sleep completion at tick 2 must fire before tick 3 proposal");
            let completed = &log.events()[completed_index];
            assert_eq!(completed.sim_tick, SimTick::new(2));
            let waited_index = log
                .events()
                .iter()
                .position(|event| event.event_type == EventKind::ActorWaited)
                .expect("later wait proposal should execute after catch-up");
            let waited = &log.events()[waited_index];
            assert_eq!(waited.sim_tick, SimTick::new(4));
            assert!(
                completed_index < waited_index,
                "catch-up completions must be appended before later requested-tick proposals"
            );
        }

        #[test]
        fn no_human_day_skips_decision_for_open_body_exclusive_duration() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id(),
                ActorBody::new(actor_id(), crate::ids::PlaceId::new("bedroom").unwrap()),
            );
            let mut agent_state = agent_state(&actor_id());
            let mut log = EventLog::new();
            let mut sleep_started = EventEnvelope::new_caused_v1(
                EventId::new("event.sleep_started.proposal_sleep_open").unwrap(),
                EventKind::SleepStarted,
                0,
                0,
                SimTick::ZERO,
                OrderingKey::new(
                    SimTick::ZERO,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id()),
                    ProposalSequence::new(0),
                    ActionId::new("sleep").unwrap(),
                    vec!["actor_tomas".to_string()],
                    "sleep_open",
                ),
                content_manifest_id(),
                vec![EventCause::Proposal(
                    ProposalId::new("proposal_sleep_open").unwrap(),
                )],
            )
            .unwrap();
            sleep_started.actor_id = Some(actor_id());
            sleep_started.proposal_id = Some(ProposalId::new("proposal_sleep_open").unwrap());
            sleep_started.payload = vec![
                PayloadField::new("actor_id", actor_id().as_str()),
                PayloadField::new("expected_completion_tick", "12"),
                PayloadField::new("body_exclusive", "true"),
            ];
            log.append(sleep_started).unwrap();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id()],
                    windows: vec![DayWindow {
                        window_id: "sleeping".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(8),
                    }],
                },
            );

            assert_eq!(report.ordinary_pipeline_events, 0);
            assert!(report.stuck_diagnostic_event_ids.is_empty());
            assert!(!log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ActorWaited));
        }

        #[test]
        fn no_human_day_reports_duplicate_duration_terminal_without_panic() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id(),
                ActorBody::new(actor_id(), crate::ids::PlaceId::new("bedroom").unwrap()),
            );
            let mut agent_state = agent_state(&actor_id());
            let mut log = EventLog::new();
            let sleep_started = open_sleep_start_event();
            let start_event_id = sleep_started.event_id.clone();
            log.append(sleep_started).unwrap();
            log.append(duration_terminal_event(
                "event.sleep_completed.first",
                &start_event_id,
            ))
            .unwrap();
            log.append(duration_terminal_event(
                "event.sleep_interrupted.duplicate",
                &start_event_id,
            ))
            .unwrap();
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id()],
                    windows: vec![DayWindow {
                        window_id: "corrupt_history".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(8),
                    }],
                },
            );

            assert!(report.scheduler_errors.iter().any(|error| matches!(
                error,
                NoHumanSchedulerError::WorldAdvance {
                    error: WorldAdvanceError::OpenDurationDiscovery(error),
                } if error.start_event_id == start_event_id
            )));
        }

        #[test]
        fn advance_completes_duration_started_before_runner_invocation() {
            let actor_id = actor_id();
            let bedroom = PlaceId::new("bedroom").unwrap();
            let sleep_affordance_id = SleepAffordanceId::new("bed_tomas").unwrap();
            let state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            let mut state = state;
            state
                .places
                .insert(bedroom.clone(), PlaceState::new(bedroom.clone(), "Bedroom"));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), bedroom.clone()),
            );
            state.sleep_affordances.insert(
                sleep_affordance_id.clone(),
                SleepAffordanceState::new(sleep_affordance_id, bedroom, 2, 80, 1),
            );
            let mut agent_state = agent_state(&actor_id);
            let mut log = EventLog::new();
            let sleep_started = log.append(open_sleep_start_event()).unwrap();
            apply_agent_event(&mut agent_state, &sleep_started).unwrap();
            let registry = ActionRegistry::new();

            let report = advance_no_human(
                NoHumanStateMut {
                    physical: &mut state,
                    agent: &mut agent_state,
                },
                &mut log,
                &registry,
                content_manifest_id(),
                SimTick::ZERO,
                3,
                Vec::new(),
            );

            assert_eq!(report.final_tick, SimTick::new(3));
            assert!(log.events().iter().any(|event| {
                event.event_type == EventKind::SleepCompleted
                    && event.sim_tick == SimTick::new(2)
                    && event.causes.iter().any(|cause| {
                        matches!(
                            cause,
                            EventCause::Event(event_id)
                                if event_id.as_str()
                                    == "event.sleep_started.proposal_sleep_open"
                        )
                    })
            }));
        }

        fn open_sleep_start_event() -> EventEnvelope {
            let mut sleep_started = EventEnvelope::new_caused_v1(
                EventId::new("event.sleep_started.proposal_sleep_open").unwrap(),
                EventKind::SleepStarted,
                0,
                0,
                SimTick::ZERO,
                OrderingKey::new(
                    SimTick::ZERO,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id()),
                    ProposalSequence::new(0),
                    ActionId::new("sleep").unwrap(),
                    vec!["actor_tomas".to_string()],
                    "sleep_open",
                ),
                content_manifest_id(),
                vec![EventCause::Proposal(
                    ProposalId::new("proposal_sleep_open").unwrap(),
                )],
            )
            .unwrap();
            sleep_started.actor_id = Some(actor_id());
            sleep_started.proposal_id = Some(ProposalId::new("proposal_sleep_open").unwrap());
            sleep_started.payload = vec![
                PayloadField::new("payload_schema_version", "1"),
                PayloadField::new("actor_id", actor_id().as_str()),
                PayloadField::new("duration_ticks", "2"),
                PayloadField::new("expected_completion_tick", "2"),
                PayloadField::new("body_exclusive", "true"),
                PayloadField::new("fatigue_delta_at_start", "0"),
                PayloadField::new("sleep_affordance_id", "bed_tomas"),
                PayloadField::new("fatigue_recovery_per_tick", "80"),
                PayloadField::new("hunger_rise_per_tick", "1"),
            ];
            sleep_started
        }

        fn duration_terminal_event(event_id: &str, start_event_id: &EventId) -> EventEnvelope {
            let mut event = EventEnvelope::new_caused_v1(
                EventId::new(event_id).unwrap(),
                EventKind::SleepCompleted,
                0,
                0,
                SimTick::new(2),
                OrderingKey::new(
                    SimTick::new(2),
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id()),
                    ProposalSequence::new(0),
                    ActionId::new("sleep_completed").unwrap(),
                    vec!["actor_tomas".to_string()],
                    event_id,
                ),
                content_manifest_id(),
                vec![EventCause::Event(start_event_id.clone())],
            )
            .unwrap();
            event.actor_id = Some(actor_id());
            event.payload = vec![PayloadField::new("payload_schema_version", "1")];
            event
        }

        fn observation_event(
            event_id: &str,
            actor_id: &ActorId,
            place_id: &PlaceId,
            tick: SimTick,
        ) -> EventEnvelope {
            let mut event = EventEnvelope::new_v1(
                EventId::new(event_id).unwrap(),
                EventKind::ObservationRecorded,
                0,
                0,
                tick,
                OrderingKey::new(
                    tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id.clone()),
                    ProposalSequence::new(0),
                    ActionId::new("record_observation").unwrap(),
                    vec![actor_id.as_str().to_string(), place_id.as_str().to_string()],
                    event_id,
                ),
                content_manifest_id(),
            );
            event.actor_id = Some(actor_id.clone());
            event.place_id = Some(place_id.clone());
            event
        }

        fn need_delta_event(
            event_id: &str,
            event_actor_id: Option<ActorId>,
            payload_actor_id: Option<&ActorId>,
            cause_kind: &str,
            causes: Vec<EventCause>,
            tick: SimTick,
        ) -> EventEnvelope {
            let mut event = EventEnvelope::new_v1(
                EventId::new(event_id).unwrap(),
                EventKind::NeedDeltaApplied,
                0,
                0,
                tick,
                OrderingKey::new(
                    tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(event_actor_id.clone().unwrap_or_else(actor_id)),
                    ProposalSequence::new(0),
                    ActionId::new("need_delta").unwrap(),
                    vec![event_id.to_string()],
                    event_id,
                ),
                content_manifest_id(),
            );
            event.actor_id = event_actor_id;
            event.causes = causes;
            event.payload = vec![
                PayloadField::new("cause_kind", cause_kind),
                PayloadField::new("need_kind", "hunger"),
            ];
            if let Some(actor_id) = payload_actor_id {
                event
                    .payload
                    .push(PayloadField::new("actor_id", actor_id.as_str()));
            }
            event
        }

        #[test]
        fn no_human_log_witness_lookups_require_exact_event_shape() {
            let actor = actor_id();
            let other_actor = ActorId::new("actor_elena").unwrap();
            let kitchen = PlaceId::new("kitchen").unwrap();
            let hall = PlaceId::new("hall").unwrap();
            let mut log = EventLog::new();
            for event in [
                observation_event(
                    "event.obs.other_actor",
                    &other_actor,
                    &kitchen,
                    SimTick::new(5),
                ),
                observation_event("event.obs.other_place", &actor, &hall, SimTick::new(5)),
                observation_event("event.obs.other_tick", &actor, &kitchen, SimTick::new(4)),
                observation_event("event.obs.exact", &actor, &kitchen, SimTick::new(5)),
            ] {
                log.append(event).unwrap();
            }

            assert_eq!(
                latest_current_place_perception_event_id(&log, &actor, SimTick::new(5), &kitchen)
                    .unwrap()
                    .as_str(),
                "event.obs.exact"
            );
            assert_eq!(
                latest_current_place_perception_event_id(&log, &actor, SimTick::new(5), &hall)
                    .unwrap()
                    .as_str(),
                "event.obs.other_place"
            );
            assert!(latest_current_place_perception_event_id(
                &log,
                &other_actor,
                SimTick::new(4),
                &kitchen
            )
            .is_none());

            let mut need_log = EventLog::new();
            need_log
                .append(need_delta_event(
                    "event.need.other_actor",
                    Some(other_actor.clone()),
                    None,
                    "tick_delta",
                    vec![EventCause::Proposal(
                        ProposalId::new("proposal_other").unwrap(),
                    )],
                    SimTick::new(3),
                ))
                .unwrap();
            need_log
                .append(need_delta_event(
                    "event.need.fixture",
                    Some(actor.clone()),
                    None,
                    "fixture_initial",
                    vec![EventCause::Proposal(
                        ProposalId::new("proposal_fixture").unwrap(),
                    )],
                    SimTick::new(4),
                ))
                .unwrap();
            need_log
                .append(need_delta_event(
                    "event.need.payload_actor",
                    None,
                    Some(&actor),
                    "tick_delta",
                    vec![EventCause::Event(EventId::new("event.wait.actor").unwrap())],
                    SimTick::new(5),
                ))
                .unwrap();
            need_log
                .append(need_delta_event(
                    "event.need.action_tick",
                    Some(actor.clone()),
                    None,
                    "tick_delta",
                    vec![EventCause::Event(EventId::new("event.wait.actor").unwrap())],
                    SimTick::new(6),
                ))
                .unwrap();

            assert_eq!(
                latest_need_event_id(&need_log, &actor).unwrap().as_str(),
                "event.need.action_tick"
            );
            assert_eq!(
                latest_action_tick_delta_tick(need_log.events(), &actor),
                Some(SimTick::new(6))
            );
            assert_eq!(
                latest_action_tick_delta_tick(need_log.events(), &other_actor),
                Some(SimTick::new(3))
            );
        }

        #[test]
        fn pending_completion_and_open_duration_checks_use_future_completion_tick() {
            let actor = actor_id();
            let mut log = EventLog::new();
            let sleep_start = open_sleep_start_event();
            log.append(sleep_start.clone()).unwrap();

            assert!(actor_has_open_body_exclusive_duration(&log, &actor, SimTick::new(1)).unwrap());
            assert!(
                !actor_has_open_body_exclusive_duration(&log, &actor, SimTick::new(2)).unwrap()
            );
            assert!(!actor_has_open_body_exclusive_duration(
                &log,
                &ActorId::new("actor_elena").unwrap(),
                SimTick::new(1)
            )
            .unwrap());

            let mut non_body_exclusive = sleep_start;
            non_body_exclusive.event_id =
                EventId::new("event.sleep_started.non_body_exclusive").unwrap();
            for field in &mut non_body_exclusive.payload {
                if field.key == "body_exclusive" {
                    field.value = "false".to_string();
                }
            }
            let mut non_body_log = EventLog::new();
            non_body_log.append(non_body_exclusive).unwrap();
            assert!(!actor_has_open_body_exclusive_duration(
                &non_body_log,
                &actor,
                SimTick::new(1)
            )
            .unwrap());
        }

        #[test]
        fn no_human_day_progress_recording_is_exact() {
            let actor = actor_id();
            let mut progress = BTreeMap::new();
            record_window_progress(&mut progress, "morning", &actor, 0);
            assert!(progress.is_empty());
            record_window_progress(&mut progress, "morning", &actor, 2);
            assert_eq!(progress.get(&("morning".to_string(), actor)), Some(&2));
        }

        #[test]
        fn routine_stuck_diagnostics_use_actor_window_and_progress_boundaries() {
            let actor = actor_id();
            let other_actor = ActorId::new("actor_elena").unwrap();
            let window = DayWindow {
                window_id: "midday".to_string(),
                start_tick: SimTick::new(10),
                end_tick: SimTick::new(12),
            };
            let mut agent_state = agent_state(&actor);
            let past_due = RoutineExecutionId::new("routine_exec_past_due").unwrap();
            agent_state.routine_executions.insert(
                past_due.clone(),
                crate::agent::RoutineExecution::new(
                    past_due.clone(),
                    actor.clone(),
                    RoutineTemplateId::new("routine_past_due").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::ZERO,
                    Some(SimTick::new(8)),
                    None,
                    None,
                    DecisionTraceId::new("trace_past_due").unwrap(),
                ),
            );
            let waiting = RoutineExecutionId::new("routine_exec_waiting_boundary").unwrap();
            let mut waiting_execution = crate::agent::RoutineExecution::new(
                waiting.clone(),
                actor.clone(),
                RoutineTemplateId::new("routine_waiting_boundary").unwrap(),
                RoutineFamily::Wait,
                SimTick::ZERO,
                Some(SimTick::new(20)),
                None,
                None,
                DecisionTraceId::new("trace_waiting_boundary").unwrap(),
            );
            waiting_execution.wait(SimTick::new(9));
            waiting_execution.record_fallback_attempt();
            agent_state
                .routine_executions
                .insert(waiting.clone(), waiting_execution);
            let boundary = RoutineExecutionId::new("routine_exec_boundary").unwrap();
            let mut boundary_execution = crate::agent::RoutineExecution::new(
                boundary.clone(),
                actor.clone(),
                RoutineTemplateId::new("routine_boundary").unwrap(),
                RoutineFamily::Wait,
                SimTick::new(10),
                Some(SimTick::new(10)),
                None,
                None,
                DecisionTraceId::new("trace_boundary").unwrap(),
            );
            boundary_execution.wait(SimTick::new(10));
            boundary_execution.record_fallback_attempt();
            agent_state
                .routine_executions
                .insert(boundary, boundary_execution);
            let expected_at_start =
                RoutineExecutionId::new("routine_exec_expected_at_start").unwrap();
            agent_state.routine_executions.insert(
                expected_at_start.clone(),
                crate::agent::RoutineExecution::new(
                    expected_at_start,
                    actor.clone(),
                    RoutineTemplateId::new("routine_expected_at_start").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::ZERO,
                    Some(SimTick::new(10)),
                    None,
                    None,
                    DecisionTraceId::new("trace_expected_at_start").unwrap(),
                ),
            );
            let progressed_at_expected =
                RoutineExecutionId::new("routine_exec_progressed_at_expected").unwrap();
            let mut progressed_execution = crate::agent::RoutineExecution::new(
                progressed_at_expected.clone(),
                actor.clone(),
                RoutineTemplateId::new("routine_progressed_at_expected").unwrap(),
                RoutineFamily::Wait,
                SimTick::ZERO,
                Some(SimTick::new(8)),
                None,
                None,
                DecisionTraceId::new("trace_progressed_at_expected").unwrap(),
            );
            progressed_execution.last_progress_tick = SimTick::new(8);
            agent_state
                .routine_executions
                .insert(progressed_at_expected, progressed_execution);
            let mut no_fallback_waiting = crate::agent::RoutineExecution::new(
                RoutineExecutionId::new("routine_exec_waiting_no_fallback").unwrap(),
                actor.clone(),
                RoutineTemplateId::new("routine_waiting_no_fallback").unwrap(),
                RoutineFamily::Wait,
                SimTick::ZERO,
                Some(SimTick::new(20)),
                None,
                None,
                DecisionTraceId::new("trace_waiting_no_fallback").unwrap(),
            );
            no_fallback_waiting.wait(SimTick::new(9));
            agent_state.routine_executions.insert(
                RoutineExecutionId::new("routine_exec_waiting_no_fallback").unwrap(),
                no_fallback_waiting,
            );
            let other = RoutineExecutionId::new("routine_exec_other_actor").unwrap();
            agent_state.routine_executions.insert(
                other.clone(),
                crate::agent::RoutineExecution::new(
                    other,
                    other_actor,
                    RoutineTemplateId::new("routine_other_actor").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::ZERO,
                    Some(SimTick::new(8)),
                    None,
                    None,
                    DecisionTraceId::new("trace_other_actor").unwrap(),
                ),
            );

            let diagnostics = routine_stuck_diagnostic_kinds(&agent_state, &actor, &window);
            assert_eq!(
                diagnostics,
                vec![
                    (StuckDiagnosticKind::PastExpectedProgressWindow, past_due),
                    (StuckDiagnosticKind::RepeatedIdleWait, waiting),
                ]
            );
        }

        #[test]
        fn routine_failure_reason_marker_and_stuck_diagnostic_vocabulary_are_stable() {
            for (kind, reason) in [
                (EventKind::ActionRejected, "action_rejected"),
                (EventKind::EatFailed, "eat_failed"),
                (EventKind::WorkBlockFailed, "work_block_failed"),
                (
                    EventKind::ContinueRoutineRejected,
                    "continue_routine_rejected",
                ),
                (EventKind::ActorWaited, "ordinary_action_failed"),
            ] {
                let mut event = EventEnvelope::new_v1(
                    EventId::new(format!("event.reason.{}", kind.stable_id())).unwrap(),
                    kind,
                    0,
                    0,
                    SimTick::ZERO,
                    OrderingKey::new(
                        SimTick::ZERO,
                        SchedulePhase::NoHumanProcess,
                        SchedulerSourceId::Actor(actor_id()),
                        ProposalSequence::new(0),
                        ActionId::new("reason_probe").unwrap(),
                        Vec::new(),
                        "reason_probe",
                    ),
                    content_manifest_id(),
                );
                if kind.requires_cause() {
                    event.causes = vec![EventCause::Process(
                        ProcessId::new("process_reason").unwrap(),
                    )];
                }
                assert_eq!(routine_failure_reason(&event), reason);
            }

            for (kind, stable_id, blocker) in [
                (
                    StuckDiagnosticKind::WindowNoProgress,
                    "window_no_progress",
                    "no progress recorded in no-human day window",
                ),
                (
                    StuckDiagnosticKind::PastExpectedProgressWindow,
                    "past_expected_progress_window",
                    "no progress past expected progress window",
                ),
                (
                    StuckDiagnosticKind::RepeatedIdleWait,
                    "repeated_idle_wait",
                    "repeated idle/wait without typed progress reason",
                ),
            ] {
                assert_eq!(kind.stable_id(), stable_id);
                assert_eq!(kind.concrete_blocker(), blocker);
                assert!(!kind.actual_source().is_empty());
            }

            let process_id = ProcessId::new("process_marker_probe").unwrap();
            let mut log = EventLog::new();
            let day_completed = append_marker(
                &mut log,
                EventKind::NoHumanDayCompleted,
                &process_id,
                SimTick::new(3),
                1,
                3,
                content_manifest_id(),
            );
            assert!(day_completed.payload.iter().any(|field| {
                field.key == "metrics_projection" && field.value == "no_human_day_metrics_v1"
            }));
            let advance_completed = append_marker(
                &mut log,
                EventKind::NoHumanAdvanceCompleted,
                &process_id,
                SimTick::new(4),
                2,
                4,
                content_manifest_id(),
            );
            assert!(!advance_completed
                .payload
                .iter()
                .any(|field| field.key == "metrics_projection"));
        }

        #[test]
        fn no_human_day_counts_duration_completion_as_window_progress() {
            let actor_id = actor_id();
            let bedroom = PlaceId::new("bedroom").unwrap();
            let sleep_affordance_id = SleepAffordanceId::new("bed_tomas").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state
                .places
                .insert(bedroom.clone(), PlaceState::new(bedroom.clone(), "Bedroom"));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), bedroom.clone()),
            );
            state.sleep_affordances.insert(
                sleep_affordance_id.clone(),
                SleepAffordanceState::new(sleep_affordance_id, bedroom, 4, 80, 1),
            );
            let mut agent_state = agent_state(&actor_id);
            agent_state
                .needs_by_actor
                .get_mut(&actor_id)
                .unwrap()
                .insert(
                    NeedKind::Fatigue,
                    crate::agent::NeedState::initial(
                        NeedKind::Fatigue,
                        880,
                        crate::agent::NeedChangeCause::FixtureInitial,
                    ),
                );
            let execution_id = RoutineExecutionId::new("routine_exec_sleep_boundary").unwrap();
            agent_state.routine_executions.insert(
                execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    execution_id,
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_sleep_boundary").unwrap(),
                    RoutineFamily::SleepNight,
                    SimTick::ZERO,
                    Some(SimTick::new(4)),
                    Some(SimTick::new(8)),
                    None,
                    DecisionTraceId::new("trace_sleep_boundary").unwrap(),
                ),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_sleep();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![
                        DayWindow {
                            window_id: "sleep_start".to_string(),
                            start_tick: SimTick::ZERO,
                            end_tick: SimTick::new(4),
                        },
                        DayWindow {
                            window_id: "recovery".to_string(),
                            start_tick: SimTick::new(4),
                            end_tick: SimTick::new(8),
                        },
                    ],
                },
            );

            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::SleepStarted));
            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::SleepCompleted));
            assert!(!report.stuck_diagnostic_event_ids.iter().any(|event_id| {
                event_id
                    .as_str()
                    .contains("window_no_progress.actor_tomas.recovery")
            }));
        }

        #[test]
        fn routine_window_family_excludes_not_yet_started_execution_at_window_start() {
            let actor_id = actor_id();
            let mut agent_state = agent_state(&actor_id);
            let mid_window_execution_id =
                RoutineExecutionId::new("routine_exec_mid_window").unwrap();
            agent_state.routine_executions.insert(
                mid_window_execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    mid_window_execution_id,
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_mid_window").unwrap(),
                    RoutineFamily::EatMeal,
                    SimTick::new(6),
                    Some(SimTick::new(8)),
                    Some(SimTick::new(12)),
                    None,
                    DecisionTraceId::new("trace_mid_window").unwrap(),
                ),
            );
            let window = DayWindow {
                window_id: "morning".to_string(),
                start_tick: SimTick::new(4),
                end_tick: SimTick::new(8),
            };
            let actor_known_state = ActorKnownPlanningContext::from_observed_parts(
                actor_id.clone(),
                PlaceId::new("kitchen").unwrap(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeMap::new(),
                Vec::new(),
            );

            assert_eq!(
                routine_window_family(&agent_state, &actor_id, &window, &actor_known_state),
                None
            );

            let already_open_execution_id =
                RoutineExecutionId::new("routine_exec_already_open").unwrap();
            agent_state.routine_executions.insert(
                already_open_execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    already_open_execution_id,
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_already_open").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::new(2),
                    Some(SimTick::new(8)),
                    Some(SimTick::new(12)),
                    None,
                    DecisionTraceId::new("trace_already_open").unwrap(),
                ),
            );

            assert_eq!(
                routine_window_family(&agent_state, &actor_id, &window, &actor_known_state),
                Some(RoutineFamily::Wait)
            );
        }

        #[test]
        fn completion_credit_uses_completion_tick_for_late_processed_duration() {
            let actor_id = actor_id();
            let windows = vec![
                DayWindow {
                    window_id: "early".to_string(),
                    start_tick: SimTick::ZERO,
                    end_tick: SimTick::new(4),
                },
                DayWindow {
                    window_id: "middle".to_string(),
                    start_tick: SimTick::new(5),
                    end_tick: SimTick::new(8),
                },
                DayWindow {
                    window_id: "late_sweep".to_string(),
                    start_tick: SimTick::new(9),
                    end_tick: SimTick::new(12),
                },
            ];
            let mut progress_by_window_actor = BTreeMap::new();

            credit_completion(
                &mut progress_by_window_actor,
                &windows,
                &actor_id,
                SimTick::new(6),
            );

            assert!(
                progress_by_window_actor.contains_key(&("middle".to_string(), actor_id.clone()))
            );
            assert!(
                !progress_by_window_actor.contains_key(&("early".to_string(), actor_id.clone()))
            );
            assert!(!progress_by_window_actor.contains_key(&("late_sweep".to_string(), actor_id)));
        }

        #[test]
        fn routine_execution_selection_excludes_deadline_expired_execution() {
            let actor_id = actor_id();
            let mut agent_state = agent_state(&actor_id);
            let expired_execution_id =
                RoutineExecutionId::new("routine_exec_deadline_expired").unwrap();
            agent_state.routine_executions.insert(
                expired_execution_id,
                crate::agent::RoutineExecution::new(
                    RoutineExecutionId::new("routine_exec_deadline_expired").unwrap(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_expired").unwrap(),
                    RoutineFamily::EatMeal,
                    SimTick::ZERO,
                    Some(SimTick::new(2)),
                    Some(SimTick::new(4)),
                    None,
                    DecisionTraceId::new("trace_expired").unwrap(),
                ),
            );
            let live_execution_id = RoutineExecutionId::new("routine_exec_live").unwrap();
            agent_state.routine_executions.insert(
                live_execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    live_execution_id.clone(),
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_live").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::new(1),
                    Some(SimTick::new(5)),
                    Some(SimTick::new(9)),
                    None,
                    DecisionTraceId::new("trace_live").unwrap(),
                ),
            );
            let window = DayWindow {
                window_id: "deadline_check".to_string(),
                start_tick: SimTick::new(4),
                end_tick: SimTick::new(6),
            };
            let actor_known_state = ActorKnownPlanningContext::from_observed_parts(
                actor_id.clone(),
                PlaceId::new("kitchen").unwrap(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeMap::new(),
                Vec::new(),
            );

            assert_eq!(
                active_routine_execution_for_actor(&agent_state, &actor_id, &window),
                Some(live_execution_id)
            );
            assert_eq!(
                routine_window_family(&agent_state, &actor_id, &window, &actor_known_state),
                Some(RoutineFamily::Wait)
            );
        }

        #[test]
        fn no_human_day_detects_routine_past_expected_progress_window() {
            let actor_id = actor_id();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(
                    actor_id.clone(),
                    crate::ids::PlaceId::new("shop_front").unwrap(),
                ),
            );
            let mut agent_state = agent_state(&actor_id);
            let execution_id = RoutineExecutionId::new("routine_exec_stalled").unwrap();
            agent_state.routine_executions.insert(
                execution_id.clone(),
                crate::agent::RoutineExecution::new(
                    execution_id,
                    actor_id.clone(),
                    RoutineTemplateId::new("routine_stalled").unwrap(),
                    RoutineFamily::Wait,
                    SimTick::ZERO,
                    Some(SimTick::new(2)),
                    None,
                    None,
                    DecisionTraceId::new("trace_stalled").unwrap(),
                ),
            );
            let mut log = EventLog::new();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &ActionRegistry::new(),
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![DayWindow {
                        window_id: "stalled".to_string(),
                        start_tick: SimTick::new(3),
                        end_tick: SimTick::new(4),
                    }],
                },
            );

            assert!(report.stuck_diagnostic_event_ids.iter().any(|event_id| {
                log.events()
                    .iter()
                    .find(|event| &event.event_id == event_id)
                    .is_some_and(|event| {
                        event.payload.iter().any(|field| {
                            field.key == "actual_source"
                                && field.value == "routine_expected_next_progress_stuck_detection"
                        })
                    })
            }));
        }

        #[test]
        fn no_human_day_detects_repeated_idle_wait() {
            let actor_id = actor_id();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(
                    actor_id.clone(),
                    crate::ids::PlaceId::new("shop_front").unwrap(),
                ),
            );
            let mut agent_state = agent_state(&actor_id);
            let execution_id = RoutineExecutionId::new("routine_exec_waiting").unwrap();
            let mut execution = crate::agent::RoutineExecution::new(
                execution_id.clone(),
                actor_id.clone(),
                RoutineTemplateId::new("routine_waiting").unwrap(),
                RoutineFamily::Wait,
                SimTick::ZERO,
                Some(SimTick::new(20)),
                None,
                None,
                DecisionTraceId::new("trace_waiting").unwrap(),
            );
            execution.wait(SimTick::new(1));
            execution.record_fallback_attempt();
            agent_state
                .routine_executions
                .insert(execution_id, execution);
            let mut log = EventLog::new();

            let report = run_no_human_day(
                &mut state,
                &mut agent_state,
                &mut log,
                &ActionRegistry::new(),
                content_manifest_id(),
                NoHumanDayConfig {
                    actor_ids: vec![actor_id],
                    windows: vec![DayWindow {
                        window_id: "waiting".to_string(),
                        start_tick: SimTick::new(4),
                        end_tick: SimTick::new(5),
                    }],
                },
            );

            assert!(report.stuck_diagnostic_event_ids.iter().any(|event_id| {
                log.events()
                    .iter()
                    .find(|event| &event.event_id == event_id)
                    .is_some_and(|event| {
                        event.payload.iter().any(|field| {
                            field.key == "actual_source"
                                && field.value == "routine_repeated_idle_wait_stuck_detection"
                        })
                    })
            }));
        }

        #[test]
        fn continue_routine_marker_only_is_not_ordinary_progress() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id(),
                ActorBody::new(actor_id(), crate::ids::PlaceId::new("shop_front").unwrap()),
            );
            let mut log = EventLog::new();
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_continue_routine();
            let mut agent_state = agent_state(&actor_id());
            let intention_id = IntentionId::new("intention_workday").unwrap();
            let intention = Intention::adopt(
                intention_id.clone(),
                actor_id(),
                IntentionSource::FixtureRoutineAssignment,
                CandidateGoalId::new("goal_workday").unwrap(),
                Some(RoutineTemplateId::new("routine_workday").unwrap()),
                Some("wait".to_string()),
                5,
                SimTick::ZERO,
                DecisionTraceId::new("trace_workday").unwrap(),
            );
            agent_state
                .active_intention_by_actor
                .insert(actor_id(), intention_id.clone());
            agent_state.intentions.insert(intention_id, intention);
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
        fn no_human_proposal_comes_from_transaction_candidate_for_routine_family() {
            let actor_id = actor_id();
            let kitchen = PlaceId::new("kitchen").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            let mut log = EventLog::new();
            let window = DayWindow {
                window_id: "midday".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            };
            log.append(
                build_window_passive_need_events(
                    Some(400),
                    false,
                    &ProcessId::new("test_need_witness").unwrap(),
                    &actor_id,
                    &window,
                    &content_manifest_id(),
                    NeedKind::Hunger,
                    0,
                    0,
                )
                .remove(0),
            )
            .unwrap();
            record_current_place_perception(
                &mut log,
                &state,
                &actor_id,
                SimTick::ZERO,
                &content_manifest_id(),
            );
            let proposal = build_agent_proposal(
                &state,
                &agent_state,
                &log,
                &actor_id,
                &DayWindow {
                    window_id: "midday".to_string(),
                    start_tick: SimTick::ZERO,
                    end_tick: SimTick::new(4),
                },
                &content_manifest_id(),
            )
            .expect("transaction candidate should produce an eat proposal");

            assert_eq!(proposal.proposal.action_id.as_str(), "eat");
            assert_eq!(proposal.proposal.target_ids, ["meal_serving"]);
            assert!(proposal
                .proposal
                .parameters
                .contains_key("decision_trace_id"));
            assert!(proposal
                .proposal
                .parameters
                .contains_key("candidate_goal_id"));
            assert!(
                proposal
                    .decision_trace_record
                    .hidden_truth_audit_result
                    .actor_known_only
            );
        }

        #[test]
        fn no_human_day_records_current_place_perception_before_decision() {
            let actor_id = actor_id();
            let home = PlaceId::new("home_tomas").unwrap();
            let workshop = PlaceId::new("workshop_tomas").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), home.clone()),
            );
            let mut home_state = PlaceState::new(home.clone(), "Tomas home");
            home_state.adjacent_place_ids.insert(workshop.clone());
            state.places.insert(home.clone(), home_state);
            state
                .places
                .insert(workshop.clone(), PlaceState::new(workshop, "Workshop"));
            state.food_supplies.insert(
                FoodSupplyId::new("food_breakfast_tomas").unwrap(),
                FoodSupplyState::new(
                    FoodSupplyId::new("food_breakfast_tomas").unwrap(),
                    Location::AtPlace(home.clone()),
                    2,
                    120,
                ),
            );
            state.sleep_affordances.insert(
                SleepAffordanceId::new("bed_tomas").unwrap(),
                SleepAffordanceState::new(
                    SleepAffordanceId::new("bed_tomas").unwrap(),
                    home,
                    4,
                    20,
                    2,
                ),
            );
            let mut first_agent_state = agent_state(&actor_id);
            let mut registry = ActionRegistry::new();
            registry.register_phase1_inspect_wait();
            let mut log = EventLog::new();
            let config = NoHumanDayConfig {
                actor_ids: vec![actor_id.clone()],
                windows: vec![DayWindow {
                    window_id: "morning".to_string(),
                    start_tick: SimTick::ZERO,
                    end_tick: SimTick::new(4),
                }],
            };
            let initial_state = state.clone();

            run_no_human_day(
                &mut state,
                &mut first_agent_state,
                &mut log,
                &registry,
                content_manifest_id(),
                config.clone(),
            );
            let mut replay_state = initial_state;
            let mut replay_agent_state = agent_state(&actor_id);
            let mut replay_log = EventLog::new();
            run_no_human_day(
                &mut replay_state,
                &mut replay_agent_state,
                &mut replay_log,
                &registry,
                content_manifest_id(),
                config,
            );

            assert_eq!(log.serialize_canonical(), replay_log.serialize_canonical());
            let observations = log
                .events()
                .iter()
                .filter(|event| event.event_type == EventKind::ObservationRecorded)
                .collect::<Vec<_>>();
            assert_eq!(observations.len(), 4);
            let perceived_kinds = observations
                .iter()
                .filter_map(|event| {
                    event
                        .payload
                        .iter()
                        .find(|field| field.key == "perceived_kind")
                        .map(|field| field.value.as_str())
                })
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                perceived_kinds,
                std::collections::BTreeSet::from([
                    "current_place",
                    "visible_exit",
                    "visible_food_supply",
                    "visible_sleep_affordance"
                ])
            );
            let first_decision_index = log
                .events()
                .iter()
                .position(|event| event.event_type == EventKind::DecisionTraceRecorded)
                .expect("no-human run records a decision trace");
            assert!(log.events()[..first_decision_index]
                .iter()
                .any(|event| event.event_type == EventKind::ObservationRecorded));
            assert!(observations.iter().all(|event| {
                event.ordering_key.sim_tick == SimTick::ZERO
                    && matches!(event.ordering_key.source_id, SchedulerSourceId::Actor(_))
            }));
        }

        #[test]
        fn scheduler_cannot_rewrite_wait_reason_after_transaction() {
            let actor_id = actor_id();
            let home = PlaceId::new("home_tomas").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state
                .actors
                .insert(actor_id.clone(), ActorBody::new(actor_id.clone(), home));
            let mut agent_state = agent_state(&actor_id);
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
                        window_id: "rewrite_window".to_string(),
                        start_tick: SimTick::ZERO,
                        end_tick: SimTick::new(4),
                    }],
                },
            );

            let wait_event = log
                .events()
                .iter()
                .find(|event| event.event_type == EventKind::ActorWaited)
                .expect("no-human idle window should commit a wait event");
            let reason = wait_event
                .payload
                .iter()
                .find(|field| field.key == "reason")
                .map(|field| field.value.as_str())
                .expect("wait event carries actor-visible reason");

            assert_eq!(reason, "actor_decision_reevaluation");
            assert!(!reason.contains("no_human_day:"));
            assert!(!wait_event
                .payload
                .iter()
                .any(|field| field.value.contains("rewrite_window")));
            assert!(!wait_event.effects_summary.contains("rewrite_window"));
        }

        #[test]
        fn action_wait_delta_advances_next_passive_charge_frontier() {
            let actor_id = actor_id();
            let home = PlaceId::new("home_tomas").unwrap();
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state
                .actors
                .insert(actor_id.clone(), ActorBody::new(actor_id.clone(), home));
            let mut agent_state = agent_state(&actor_id);
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
                    windows: vec![
                        DayWindow {
                            window_id: "first_idle".to_string(),
                            start_tick: SimTick::ZERO,
                            end_tick: SimTick::new(1),
                        },
                        DayWindow {
                            window_id: "second_idle".to_string(),
                            start_tick: SimTick::new(4),
                            end_tick: SimTick::new(5),
                        },
                    ],
                },
            );

            assert!(log
                .events()
                .iter()
                .any(|event| event.event_type == EventKind::ActorWaited
                    && event.sim_tick == SimTick::new(1)));
            let second_window_passive = log
                .events()
                .iter()
                .find(|event| {
                    event.event_type == EventKind::NeedDeltaApplied
                        && event.sim_tick == SimTick::new(4)
                        && event.payload.iter().any(|field| {
                            field.key == "accounting_phase" && field.value == "world_step"
                        })
                        && event
                            .payload
                            .iter()
                            .any(|field| field.key == "need_kind" && field.value == "hunger")
                })
                .expect("coordinator emits passive hunger delta before second window");
            assert!(second_window_passive
                .payload
                .iter()
                .any(|field| { field.key == "elapsed_ticks" && field.value == "1" }));
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
            state.actors.insert(
                actor_id.clone(),
                ActorBody::new(actor_id.clone(), workshop.clone()),
            );
            state.places.insert(
                workshop.clone(),
                PlaceState::new(workshop.clone(), "Workshop"),
            );
            let mut workplace = WorkplaceState::new(
                workplace_id.clone(),
                workshop,
                4,
                8,
                4,
                900,
                900,
                "blocked_output",
            );
            workplace.assigned_actor_ids.insert(actor_id.clone());
            workplace.access_open = false;
            state.workplaces.insert(workplace_id.clone(), workplace);
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
            let mut role_notice = EventEnvelope::new_v1(
                EventId::new("event.role_notice.blocked_work").unwrap(),
                EventKind::RoleAssignmentNoticeRecorded,
                0,
                0,
                SimTick::ZERO,
                OrderingKey::new(
                    SimTick::ZERO,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id.clone()),
                    ProposalSequence::new(0),
                    ActionId::new("role_assignment_notice").unwrap(),
                    vec![
                        actor_id.as_str().to_string(),
                        workplace_id.as_str().to_string(),
                    ],
                    "blocked_work_role_notice",
                ),
                content_manifest_id(),
            );
            role_notice.actor_id = Some(actor_id.clone());
            role_notice.participants = vec![actor_id.as_str().to_string()];
            role_notice.payload = vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("source_kind", "authored_prehistory"),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id.as_str()),
                PayloadField::new("place_id", "workshop"),
                PayloadField::new("access_open", "true"),
            ];
            log.append(role_notice).unwrap();
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
                        && event.sim_tick <= SimTick::new(4)
                        && event.payload.iter().any(|field| {
                            field.key == "accounting_phase" && field.value == "world_step"
                        })
                })
                .expect("coordinator passive need delta emitted before decision");
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
                field.key == "candidate_goal_reevaluation" && field.value == "true"
            }));
        }

        #[test]
        fn no_human_day_runs_windows_in_stable_actor_order_without_controller_facts() {
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            assert_eq!(report.stuck_diagnostic_event_ids.len(), 4);
            assert_eq!(report.ordinary_pipeline_events, 0);
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
            let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
        assert_eq!(scheduler.increment_clock_one_tick(), SimTick::new(5));
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
        let need_model = NeedModelState::new(5, 3);

        let first = build_passive_need_delta_events(
            &need_model,
            actors.clone(),
            &process,
            SimTick::new(4),
            3,
            &content_manifest_id(),
        );
        let second = build_passive_need_delta_events(
            &need_model,
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
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
        assert_eq!(log.events().len(), 5);
        assert_eq!(
            log.events()
                .iter()
                .filter(|event| event.stream == EventStream::Diagnostic)
                .count(),
            2
        );
        assert!(log
            .events()
            .iter()
            .any(|event| event.event_type == EventKind::TimeAdvanced));
        assert_eq!(
            log.events()
                .iter()
                .map(|event| event.event_type)
                .collect::<Vec<_>>(),
            [
                EventKind::NoHumanAdvanceStarted,
                EventKind::TimeAdvanced,
                EventKind::TimeAdvanced,
                EventKind::TimeAdvanced,
                EventKind::NoHumanAdvanceCompleted
            ]
        );
        assert!(!format!("{:?}", log.events()).contains("PlayerCharacter"));
    }

    #[test]
    fn no_human_markers_replay_as_physical_noops() {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            action_id("wait"),
            SimTick::ZERO,
        );
        proposal
            .parameters
            .insert("reason".to_string(), "scheduled wait".to_string());

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

        let mut direct_state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
