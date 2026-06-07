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
    use std::collections::BTreeMap;

    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::agent::{
        generate_candidate_goals, plan_local_actions, select_goal_and_trace, select_phase3a_method,
        ActorKnownPlanningState, BlockerCategory, CandidateGenerationInput, DecisionInput,
        LocalPlanRequest, PlannerGoal, StuckDiagnostic, StuckResultingStatus,
    };
    use crate::events::log::EventLog;
    use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
    use crate::ids::{
        ActionId, ActorId, ContentManifestId, EventId, ProcessId, ProposalId, StuckDiagnosticId,
    };
    use crate::scheduler::{
        DeterministicScheduler, OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use crate::state::PhysicalState;
    use crate::time::SimTick;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NoHumanAdvanceReport {
        pub start_tick: SimTick,
        pub final_tick: SimTick,
        pub tick_count: u64,
        pub marker_event_ids: Vec<EventId>,
        pub ordinary_pipeline_events: usize,
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

        let mut scheduled = Vec::new();
        for window in &config.windows {
            for actor_id in &config.actor_ids {
                if !state.actors.contains_key(actor_id) {
                    continue;
                }
                let Some(proposal) = build_agent_wait_proposal(state, actor_id, window, registry)
                else {
                    continue;
                };
                let ordering_key = OrderingKey::new(
                    window.start_tick,
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id.clone()),
                    scheduler.assign_proposal_sequence(),
                    proposal.action_id.clone(),
                    proposal.target_ids.clone(),
                    format!("{}:{}", window.window_id, actor_id.as_str()),
                );
                scheduled.push((window.clone(), actor_id.clone(), ordering_key, proposal));
            }
        }
        scheduled.sort_by(|left, right| left.2.cmp(&right.2));

        let mut ordinary_pipeline_events = 0;
        let mut progress_by_window_actor: BTreeMap<(String, ActorId), usize> = BTreeMap::new();
        for (window, actor_id, ordering_key, proposal) in scheduled {
            let before = log.events().len();
            let mut context = PipelineContext {
                registry,
                state,
                log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: content_manifest_id.clone(),
                ordering_key,
            };
            run_pipeline(&mut context, &proposal);
            let appended = log.events().len().saturating_sub(before);
            ordinary_pipeline_events += appended;
            if appended > 0 {
                progress_by_window_actor.insert((window.window_id, actor_id), appended);
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

    fn build_agent_wait_proposal(
        state: &PhysicalState,
        actor_id: &ActorId,
        window: &DayWindow,
        registry: &ActionRegistry,
    ) -> Option<Proposal> {
        registry.get(&ActionId::new("wait").unwrap())?;
        let actor = state.actors.get(actor_id)?;
        let actor_known_inputs = vec![
            "reason_available".to_string(),
            "reevaluation_scheduled".to_string(),
            format!("day_window:{}", window.window_id),
        ];
        let generated = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id.clone(),
            decision_tick: window.start_tick,
            needs: Vec::new(),
            active_intention: None,
            actor_known_inputs: actor_known_inputs.clone(),
            routine_window_goal: None,
        });
        let selection = select_goal_and_trace(DecisionInput {
            actor_id: actor_id.clone(),
            decision_tick: window.start_tick,
            candidates: generated.candidates,
            active_intention: None,
            actor_known_inputs: generated.actor_known_inputs_used,
        })?;
        let method = select_phase3a_method(
            &selection.selected_goal,
            &actor_known_inputs,
            window.start_tick,
        )
        .ok()?;
        let wait_reason = format!("no_human_day:{}", window.window_id);
        let plan = plan_local_actions(
            &ActorKnownPlanningState {
                actor_id: actor_id.clone(),
                current_place_id: actor.current_place_id.clone(),
                known_edges: BTreeMap::new(),
                known_closed_doors: BTreeMap::new(),
                known_containers_by_place: BTreeMap::new(),
                known_food_sources: Default::default(),
            },
            &LocalPlanRequest {
                routine_step: method.template.steps.first().cloned().unwrap_or(
                    crate::agent::RoutineStep::WaitUntil {
                        reason: wait_reason.clone(),
                    },
                ),
                goal: PlannerGoal::WaitWithReason(wait_reason.clone()),
                budget: 1,
                actor_known_inputs,
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

    pub fn advance_no_human(
        state: &mut PhysicalState,
        log: &mut EventLog,
        registry: &ActionRegistry,
        content_manifest_id: ContentManifestId,
        start_tick: SimTick,
        tick_count: u64,
        scheduled_proposals: Vec<Proposal>,
    ) -> NoHumanAdvanceReport {
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
            let before = log.events().len();
            let mut context = PipelineContext {
                registry,
                state,
                log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: content_manifest_id.clone(),
                ordering_key,
            };
            run_pipeline(&mut context, &proposal);
            ordinary_pipeline_events += log.events().len().saturating_sub(before);
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
        use crate::events::apply::apply_event;
        use crate::events::EventStream;
        use crate::ids::{ActorId, ProposalId};
        use crate::state::ActorBody;

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
                &mut state,
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
                &mut state,
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
            let proposal = Proposal::new(
                ProposalId::new("proposal_wait").unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id()),
                ActionId::new("wait").unwrap(),
                SimTick::ZERO,
            );

            let report = advance_no_human(
                &mut state,
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
    use crate::state::{ActorBody, PhysicalState};

    fn action_id(value: &str) -> ActionId {
        ActionId::new(value).unwrap()
    }

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
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
            &mut state,
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
            &mut state,
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
        let proposal = Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            action_id("wait"),
            SimTick::ZERO,
        );

        let report = no_human::advance_no_human(
            &mut state,
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
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut direct_state,
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
