use std::collections::BTreeMap;

use crate::agent::{
    Intention, IntentionSource, IntentionStatus, NeedChangeCause, NeedKind, RoutineStepStatus,
};
use crate::epistemics::{
    Belief, Channel, Confidence, Contradiction, ContradictionKind, EpistemicProjection, HolderKind,
    Observation, ObservationSubject, ObservationTarget, Proposition, SourceRef, Stance,
};
use crate::events::mutation::{AgentMutationCapability, WorldMutationCapability};
use crate::events::{
    is_duration_terminal, EventCause, EventEnvelope, EventKind, EventStream, PayloadField,
    EVENT_SCHEMA_V1,
};
use crate::ids::{
    ActionId, ActorId, BeliefId, ContainerId, ContradictionId, DoorId, EventId, FoodSupplyId,
    IntentionId, ItemId, ObservationId, PlaceId, RoutineExecutionId,
};
use crate::location::Location;
use crate::state::{AgentState, PhysicalState};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyOutcome {
    Applied,
    NonWorldNoOp,
    WorldNoOp,
}

#[derive(Debug)]
pub struct EventApplicationContext<'a> {
    pub physical_state: &'a mut PhysicalState,
    pub agent_state: &'a mut AgentState,
    pub epistemic_projection: Option<&'a mut EpistemicProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventApplicationError {
    World(ApplyError),
    Agent(ApplyError),
    Epistemic(EpistemicApplyError),
}

pub fn apply_event_stream(
    context: &mut EventApplicationContext<'_>,
    event: &EventEnvelope,
) -> Result<ApplyOutcome, EventApplicationError> {
    if event.stream != event.event_type.stream() {
        return Err(EventApplicationError::World(
            ApplyError::EventKindStreamMismatch,
        ));
    }

    match event.stream {
        EventStream::World => {
            apply_event(context.physical_state, event).map_err(EventApplicationError::World)
        }
        EventStream::Agent => {
            apply_agent_event(context.agent_state, event).map_err(EventApplicationError::Agent)
        }
        EventStream::Epistemic => {
            if let Some(projection) = context.epistemic_projection.as_deref_mut() {
                apply_epistemic_event(projection, event).map_err(EventApplicationError::Epistemic)
            } else {
                Ok(ApplyOutcome::NonWorldNoOp)
            }
        }
        EventStream::Diagnostic | EventStream::Controller | EventStream::ReplayDebug => {
            Ok(ApplyOutcome::NonWorldNoOp)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyError {
    UnsupportedSchemaVersion(String),
    MissingPayload(&'static str),
    BadPayload {
        key: &'static str,
        value: String,
    },
    MissingActor(ActorId),
    MissingPlace(PlaceId),
    MissingDoor(DoorId),
    MissingContainer(ContainerId),
    MissingItem(ItemId),
    MissingFoodSupply(FoodSupplyId),
    PreconditionMismatch {
        field: &'static str,
        expected: String,
        actual: String,
    },
    EventKindStreamMismatch,
    UnhandledWorldEventKind(EventKind),
    NonAgentEvent,
    MissingNeedState {
        actor_id: ActorId,
        need_kind: NeedKind,
    },
    DuplicateNeedTickCharge {
        actor_id: ActorId,
        need_kind: NeedKind,
        tick: u64,
    },
    MalformedElapsedTicks(String),
    MissingIntention(IntentionId),
    MissingRoutineExecution(RoutineExecutionId),
    MissingCause,
    DuplicateDurationTerminal {
        start_event_id: EventId,
    },
}

pub const AGENT_WORLD_NOOP_ALLOWLIST: &[EventKind] = &[
    EventKind::FoodConsumed,
    EventKind::NoHumanDayStarted,
    EventKind::NoHumanDayCompleted,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EpistemicApplyError {
    UnsupportedEventSchemaVersion(String),
    UnsupportedPayloadSchemaVersion(String),
    MissingPayload(&'static str),
    BadPayload { key: &'static str, value: String },
    EventKindStreamMismatch,
    NonEpistemicEvent,
    MissingCause,
    MissingHolder,
    MissingSource,
}

pub fn apply_event(
    state: &mut PhysicalState,
    event: &EventEnvelope,
) -> Result<ApplyOutcome, ApplyError> {
    let capability = WorldMutationCapability::mint();
    apply_event_with_capability(state, event, capability)
}

fn apply_event_with_capability(
    state: &mut PhysicalState,
    event: &EventEnvelope,
    _capability: WorldMutationCapability,
) -> Result<ApplyOutcome, ApplyError> {
    if !event.has_supported_schema_version() {
        return Err(ApplyError::UnsupportedSchemaVersion(
            event.event_schema_version.as_str().to_string(),
        ));
    }

    if event.stream != event.event_type.stream() {
        return Err(ApplyError::EventKindStreamMismatch);
    }

    if event.stream != EventStream::World {
        return Ok(ApplyOutcome::NonWorldNoOp);
    }

    if event.event_type.requires_cause() && event.causes.is_empty() {
        return Err(ApplyError::MissingCause);
    }

    let payload = payload_map(&event.payload);
    match event.event_type {
        EventKind::ActorMoved => apply_actor_moved(state, &payload).map(|_| ApplyOutcome::Applied),
        EventKind::DoorOpened => {
            apply_door_open_state(state, &payload, false, true).map(|_| ApplyOutcome::Applied)
        }
        EventKind::DoorClosed => {
            apply_door_open_state(state, &payload, true, false).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ContainerOpened => {
            apply_container_open_state(state, &payload, false, true).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ContainerClosed => {
            apply_container_open_state(state, &payload, true, false).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ItemTakenFromPlace => {
            apply_item_taken_from_place(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ItemRemovedFromContainer => {
            apply_item_removed_from_container(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ItemPlacedInPlace => {
            apply_item_placed_in_place(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ItemPlacedInContainer => {
            apply_item_placed_in_container(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::FoodConsumed => {
            apply_food_consumed(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::ActorWaited | EventKind::TimeAdvanced => Ok(ApplyOutcome::WorldNoOp),
        other => Err(ApplyError::UnhandledWorldEventKind(other)),
    }
}

pub fn apply_epistemic_event(
    projection: &mut EpistemicProjection,
    event: &EventEnvelope,
) -> Result<ApplyOutcome, EpistemicApplyError> {
    if !event.has_supported_schema_version() {
        return Err(EpistemicApplyError::UnsupportedEventSchemaVersion(
            event.event_schema_version.as_str().to_string(),
        ));
    }

    if event.stream != event.event_type.stream() {
        return Err(EpistemicApplyError::EventKindStreamMismatch);
    }

    if event.stream != EventStream::Epistemic {
        return Ok(ApplyOutcome::NonWorldNoOp);
    }

    if event.event_type.requires_cause() && event.causes.is_empty() {
        return Err(EpistemicApplyError::MissingCause);
    }

    let payload = payload_map(&event.payload);
    let payload_schema_version = required_epistemic(&payload, "schema_version")?;
    if payload_schema_version != EVENT_SCHEMA_V1 {
        return Err(EpistemicApplyError::UnsupportedPayloadSchemaVersion(
            payload_schema_version.to_string(),
        ));
    }

    match event.event_type {
        EventKind::InitialBeliefSeeded | EventKind::BeliefUpdated => {
            let belief = parse_belief_payload(&payload)?;
            projection.insert_belief(belief);
            Ok(ApplyOutcome::Applied)
        }
        EventKind::ObservationRecorded => {
            let observation =
                parse_observation_payload(&payload)?.with_raw_payload(event.payload.clone());
            projection.insert_observation(observation);
            Ok(ApplyOutcome::Applied)
        }
        EventKind::RoleAssignmentNoticeRecorded => {
            let actor_id = parse_actor_id_epistemic(&payload, "actor_id")?;
            let workplace_id = required_epistemic(&payload, "workplace_id").and_then(|value| {
                crate::ids::WorkplaceId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
                    key: "workplace_id",
                    value: value.to_string(),
                })
            })?;
            let place_id = parse_place_id_epistemic(&payload, "place_id")?;
            let access_open = match required_epistemic(&payload, "access_open")? {
                "true" => true,
                "false" => false,
                value => {
                    return Err(EpistemicApplyError::BadPayload {
                        key: "access_open",
                        value: value.to_string(),
                    })
                }
            };
            projection.insert_role_assignment_notice(
                actor_id,
                workplace_id,
                place_id,
                access_open,
                event.event_id.clone(),
                event.sim_tick,
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::StartingBeliefRecorded => {
            let actor_id = parse_actor_id_epistemic(&payload, "actor_id")?;
            let belief_kind = required_epistemic(&payload, "belief_kind")?;
            let subject_id = required_epistemic(&payload, "subject_id")?;
            let value = required_epistemic(&payload, "value")?;
            projection.insert_starting_belief(
                actor_id,
                belief_kind,
                subject_id,
                value,
                event.event_id.clone(),
                event.sim_tick,
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::ExpectationContradicted => {
            let contradiction = parse_contradiction_payload(&payload)?;
            projection.insert_contradiction(contradiction);
            Ok(ApplyOutcome::Applied)
        }
        EventKind::ContainerChecked => Ok(ApplyOutcome::WorldNoOp),
        _ => Err(EpistemicApplyError::NonEpistemicEvent),
    }
}

pub fn apply_agent_event(
    state: &mut AgentState,
    event: &EventEnvelope,
) -> Result<ApplyOutcome, ApplyError> {
    let capability = AgentMutationCapability::mint();
    apply_agent_event_with_capability(state, event, capability)
}

fn apply_agent_event_with_capability(
    state: &mut AgentState,
    event: &EventEnvelope,
    _capability: AgentMutationCapability,
) -> Result<ApplyOutcome, ApplyError> {
    if !event.has_supported_schema_version() {
        return Err(ApplyError::UnsupportedSchemaVersion(
            event.event_schema_version.as_str().to_string(),
        ));
    }

    if event.stream != event.event_type.stream() {
        return Err(ApplyError::EventKindStreamMismatch);
    }

    if event.stream != EventStream::Agent {
        return Ok(ApplyOutcome::NonWorldNoOp);
    }

    if event.event_type.requires_cause() && event.causes.is_empty() {
        return Err(ApplyError::MissingCause);
    }

    let payload = payload_map(&event.payload);
    match event.event_type {
        EventKind::NeedDeltaApplied => {
            apply_need_delta(state, event, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::IntentionStarted => {
            apply_intention_started(state, &payload).map(|_| ApplyOutcome::Applied)
        }
        EventKind::IntentionSuspended
        | EventKind::IntentionResumed
        | EventKind::IntentionCompleted
        | EventKind::IntentionFailed
        | EventKind::IntentionAbandoned
        | EventKind::IntentionInterrupted
        | EventKind::IntentionContinued => {
            apply_intention_transition(state, event.event_type, &payload)
                .map(|_| ApplyOutcome::Applied)
        }
        EventKind::RoutineStepStarted
        | EventKind::RoutineStepCompleted
        | EventKind::RoutineStepFailed => {
            apply_routine_step_transition(state, event.event_type, &payload)
                .map(|_| ApplyOutcome::Applied)
        }
        EventKind::DecisionTraceRecorded => {
            require_payload_version(&payload, "trace_schema_version", "1")?;
            let trace_id = crate::ids::DecisionTraceId::new(required(&payload, "trace_id")?)
                .map_err(|_| ApplyError::BadPayload {
                    key: "trace_id",
                    value: required(&payload, "trace_id")
                        .unwrap_or_default()
                        .to_string(),
                })?;
            let record = crate::agent::DecisionTraceRecord::deserialize_canonical(
                required(&payload, "trace_canonical")?.as_bytes(),
            )
            .map_err(|_| ApplyError::BadPayload {
                key: "trace_canonical",
                value: required(&payload, "trace_canonical")
                    .unwrap_or_default()
                    .to_string(),
            })?;
            if record.trace_id != trace_id {
                return Err(ApplyError::BadPayload {
                    key: "trace_id",
                    value: trace_id.to_string(),
                });
            }
            validate_typed_diagnostic_payload(&payload, &record.typed_diagnostic)?;
            state.decision_traces.insert(trace_id, record);
            Ok(ApplyOutcome::Applied)
        }
        EventKind::StuckDiagnosticRecorded => {
            require_payload_version(&payload, "diagnostic_schema_version", "1")?;
            let diagnostic_id =
                crate::ids::StuckDiagnosticId::new(required(&payload, "diagnostic_id")?).map_err(
                    |_| ApplyError::BadPayload {
                        key: "diagnostic_id",
                        value: required(&payload, "diagnostic_id")
                            .unwrap_or_default()
                            .to_string(),
                    },
                )?;
            let record = crate::agent::StuckDiagnostic::deserialize_canonical(
                required(&payload, "diagnostic_canonical")?.as_bytes(),
            )
            .map_err(|_| ApplyError::BadPayload {
                key: "diagnostic_canonical",
                value: required(&payload, "diagnostic_canonical")
                    .unwrap_or_default()
                    .to_string(),
            })?;
            if record.diagnostic_id != diagnostic_id {
                return Err(ApplyError::BadPayload {
                    key: "diagnostic_id",
                    value: diagnostic_id.to_string(),
                });
            }
            validate_typed_diagnostic_payload(&payload, &record.typed_diagnostic)?;
            state.stuck_diagnostics.insert(diagnostic_id, record);
            Ok(ApplyOutcome::Applied)
        }
        EventKind::NeedThresholdCrossed => {
            require_payload_version(&payload, "payload_schema_version", "1")?;
            let actor_id = ActorId::new(required(&payload, "actor_id")?).map_err(|_| {
                ApplyError::BadPayload {
                    key: "actor_id",
                    value: required(&payload, "actor_id")
                        .unwrap_or_default()
                        .to_string(),
                }
            })?;
            let need_kind = match required(&payload, "need_kind")? {
                "hunger" => NeedKind::Hunger,
                "fatigue" => NeedKind::Fatigue,
                "safety" => NeedKind::Safety,
                value => {
                    return Err(ApplyError::BadPayload {
                        key: "need_kind",
                        value: value.to_string(),
                    });
                }
            };
            let from_value = required(&payload, "from_value")?
                .parse::<u16>()
                .map_err(|_| ApplyError::BadPayload {
                    key: "from_value",
                    value: required(&payload, "from_value")
                        .unwrap_or_default()
                        .to_string(),
                })?;
            let to_value = required(&payload, "to_value")?
                .parse::<u16>()
                .map_err(|_| ApplyError::BadPayload {
                    key: "to_value",
                    value: required(&payload, "to_value")
                        .unwrap_or_default()
                        .to_string(),
                })?;
            state.need_threshold_crossings.insert(
                event.event_id.clone(),
                crate::state::NeedThresholdCrossingRecord {
                    event_id: event.event_id.clone(),
                    actor_id,
                    need_kind,
                    from_value,
                    to_value,
                    from_band: required(&payload, "from_band")?.to_string(),
                    to_band: required(&payload, "to_band")?.to_string(),
                    payload_fields: payload_fields(event),
                },
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::SleepStarted
        | EventKind::SleepCompleted
        | EventKind::SleepInterrupted
        | EventKind::FoodServiceUsed
        | EventKind::EatFailed
        | EventKind::WorkBlockStarted
        | EventKind::WorkBlockCompleted
        | EventKind::WorkBlockFailed => {
            require_payload_version(&payload, "payload_schema_version", "1")?;
            reject_duplicate_duration_terminal(state, event)?;
            state.ordinary_life_episodes.insert(
                event.event_id.clone(),
                crate::state::OrdinaryLifeEpisodeRecord {
                    event_id: event.event_id.clone(),
                    event_kind: event.event_type.stable_id().to_string(),
                    actor_id: event.actor_id.clone(),
                    proposal_id: event.proposal_id.clone(),
                    caused_event_ids: event
                        .causes
                        .iter()
                        .filter_map(|cause| match cause {
                            EventCause::Event(event_id) => Some(event_id.clone()),
                            _ => None,
                        })
                        .collect(),
                    sim_tick: event.sim_tick,
                    payload_fields: payload_fields(event),
                    summary: event.effects_summary.clone(),
                },
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::CandidateGoalsEvaluated => {
            require_payload_version(&payload, "payload_schema_version", "1")?;
            state.candidate_goal_evaluations.insert(
                event.event_id.clone(),
                crate::state::CandidateGoalEvaluationRecord {
                    event_id: event.event_id.clone(),
                    event_kind: event.event_type.stable_id().to_string(),
                    actor_id: event.actor_id.clone(),
                    proposal_id: event.proposal_id.clone(),
                    caused_event_ids: caused_event_ids(event),
                    sim_tick: event.sim_tick,
                    payload_fields: payload_fields(event),
                    summary: event.effects_summary.clone(),
                },
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::ContinueRoutineProposed
        | EventKind::ContinueRoutineAccepted
        | EventKind::ContinueRoutineRejected => {
            require_payload_version(&payload, "payload_schema_version", "1")?;
            state.continue_routine_arbitrations.insert(
                event.event_id.clone(),
                crate::state::ContinueRoutineArbitrationRecord {
                    event_id: event.event_id.clone(),
                    event_kind: event.event_type.stable_id().to_string(),
                    actor_id: event.actor_id.clone(),
                    proposal_id: event.proposal_id.clone(),
                    caused_event_ids: caused_event_ids(event),
                    sim_tick: event.sim_tick,
                    payload_fields: payload_fields(event),
                    summary: event.effects_summary.clone(),
                },
            );
            Ok(ApplyOutcome::Applied)
        }
        kind if AGENT_WORLD_NOOP_ALLOWLIST.contains(&kind) => Ok(ApplyOutcome::WorldNoOp),
        _ => Err(ApplyError::NonAgentEvent),
    }
}

fn caused_event_ids(event: &EventEnvelope) -> Vec<EventId> {
    event
        .causes
        .iter()
        .filter_map(|cause| match cause {
            EventCause::Event(event_id) => Some(event_id.clone()),
            _ => None,
        })
        .collect()
}

fn payload_fields(event: &EventEnvelope) -> Vec<(String, String)> {
    let mut fields = event
        .payload
        .iter()
        .map(|field| (field.key.clone(), field.value.clone()))
        .collect::<Vec<_>>();
    fields.sort();
    fields
}

fn reject_duplicate_duration_terminal(
    state: &AgentState,
    event: &EventEnvelope,
) -> Result<(), ApplyError> {
    if !is_duration_terminal(event.event_type) {
        return Ok(());
    }
    for start_event_id in event.causes.iter().filter_map(|cause| match cause {
        EventCause::Event(event_id) => Some(event_id),
        _ => None,
    }) {
        let already_terminated = state.ordinary_life_episodes.values().any(|episode| {
            matches!(
                episode.event_kind.as_str(),
                "sleep_completed"
                    | "sleep_interrupted"
                    | "work_block_completed"
                    | "work_block_failed"
            ) && episode
                .caused_event_ids
                .iter()
                .any(|id| id == start_event_id)
        });
        if already_terminated {
            return Err(ApplyError::DuplicateDurationTerminal {
                start_event_id: start_event_id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_typed_diagnostic_payload(
    payload: &std::collections::BTreeMap<&str, &str>,
    typed: &crate::agent::TypedDiagnosticFields,
) -> Result<(), ApplyError> {
    if let Some(value) = payload.get("responsible_layer") {
        let parsed =
            crate::agent::ResponsibleLayer::parse(value).map_err(|_| ApplyError::BadPayload {
                key: "responsible_layer",
                value: (*value).to_string(),
            })?;
        if parsed != typed.responsible_layer {
            return Err(ApplyError::BadPayload {
                key: "responsible_layer",
                value: (*value).to_string(),
            });
        }
    }
    if let Some(value) = payload.get("blocker_code") {
        let parsed =
            crate::agent::BlockerCode::parse(value).map_err(|_| ApplyError::BadPayload {
                key: "blocker_code",
                value: (*value).to_string(),
            })?;
        if parsed != typed.blocker_code {
            return Err(ApplyError::BadPayload {
                key: "blocker_code",
                value: (*value).to_string(),
            });
        }
    }
    if let Some(value) = payload.get("hidden_truth_referenced") {
        let parsed = match *value {
            "true" => true,
            "false" => false,
            _ => {
                return Err(ApplyError::BadPayload {
                    key: "hidden_truth_referenced",
                    value: (*value).to_string(),
                });
            }
        };
        if parsed != typed.hidden_truth_referenced {
            return Err(ApplyError::BadPayload {
                key: "hidden_truth_referenced",
                value: (*value).to_string(),
            });
        }
    }
    for (key, expected) in [
        ("input_source", typed.input_source.as_str()),
        ("actual_source", typed.actual_source.as_str()),
        ("remediation_hint", typed.remediation_hint.as_str()),
    ] {
        if let Some(value) = payload.get(key) {
            if *value != expected {
                return Err(ApplyError::BadPayload {
                    key,
                    value: (*value).to_string(),
                });
            }
        }
    }
    Ok(())
}

fn payload_map(payload: &[PayloadField]) -> BTreeMap<&str, &str> {
    payload
        .iter()
        .map(|field| (field.key.as_str(), field.value.as_str()))
        .collect()
}

fn required<'a>(
    payload: &'a BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<&'a str, ApplyError> {
    payload
        .get(key)
        .copied()
        .ok_or(ApplyError::MissingPayload(key))
}

fn require_payload_version(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
    expected: &'static str,
) -> Result<(), ApplyError> {
    let actual = required(payload, key)?;
    if actual == expected {
        Ok(())
    } else {
        Err(ApplyError::BadPayload {
            key,
            value: actual.to_string(),
        })
    }
}

fn required_epistemic<'a>(
    payload: &'a BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<&'a str, EpistemicApplyError> {
    payload
        .get(key)
        .copied()
        .ok_or(EpistemicApplyError::MissingPayload(key))
}

fn parse_actor_id_epistemic(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<ActorId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    ActorId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_belief_id(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<BeliefId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    BeliefId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_observation_id(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<ObservationId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    ObservationId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_contradiction_id(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<ContradictionId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    ContradictionId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_place_id_epistemic(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<PlaceId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    PlaceId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_event_id_epistemic(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<EventId, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    EventId::new(value).map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_tick(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<SimTick, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    let parsed = value
        .parse::<u64>()
        .map_err(|_| EpistemicApplyError::BadPayload {
            key,
            value: value.to_string(),
        })?;
    Ok(SimTick::new(parsed))
}

fn parse_optional_tick(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<Option<SimTick>, EpistemicApplyError> {
    payload
        .get(key)
        .map(|_| parse_tick(payload, key))
        .transpose()
}

fn parse_confidence(payload: &BTreeMap<&str, &str>) -> Result<Confidence, EpistemicApplyError> {
    let value = required_epistemic(payload, "confidence")?;
    let parsed = value
        .parse::<u16>()
        .map_err(|_| EpistemicApplyError::BadPayload {
            key: "confidence",
            value: value.to_string(),
        })?;
    Confidence::new(parsed).map_err(|_| EpistemicApplyError::BadPayload {
        key: "confidence",
        value: value.to_string(),
    })
}

fn parse_stance(payload: &BTreeMap<&str, &str>) -> Result<Stance, EpistemicApplyError> {
    let value = required_epistemic(payload, "stance")?;
    match value {
        "believes_true" => Ok(Stance::BelievesTrue),
        "believes_false" => Ok(Stance::BelievesFalse),
        "expects_true" => Ok(Stance::ExpectsTrue),
        "plausible" => Ok(Stance::Plausible),
        "doubts" => Ok(Stance::Doubts),
        "unknown_or_unresolved" => Ok(Stance::UnknownOrUnresolved),
        _ => Err(EpistemicApplyError::BadPayload {
            key: "stance",
            value: value.to_string(),
        }),
    }
}

fn parse_channel(payload: &BTreeMap<&str, &str>) -> Result<Channel, EpistemicApplyError> {
    let value = required_epistemic(payload, "channel")?;
    match value {
        "direct_sight" => Ok(Channel::DirectSight),
        "touch_or_search" => Ok(Channel::TouchOrSearch),
        "simple_sound" => Ok(Channel::SimpleSound),
        "absence_marker" => Ok(Channel::AbsenceMarker),
        "reading_placeholder_schema_only" => Ok(Channel::ReadingPlaceholderSchemaOnly),
        _ => Err(EpistemicApplyError::BadPayload {
            key: "channel",
            value: value.to_string(),
        }),
    }
}

fn parse_proposition(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<Proposition, EpistemicApplyError> {
    let value = required_epistemic(payload, key)?;
    value.parse().map_err(|_| EpistemicApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_belief_payload(payload: &BTreeMap<&str, &str>) -> Result<Belief, EpistemicApplyError> {
    let holder_actor_id = parse_actor_id_epistemic(payload, "holder_actor_id")
        .map_err(|_| EpistemicApplyError::MissingHolder)?;
    let source_event_id = parse_event_id_epistemic(payload, "source_event_id")
        .map_err(|_| EpistemicApplyError::MissingSource)?;
    let mut belief = Belief::new(
        parse_belief_id(payload, "belief_id")?,
        HolderKind::Actor(holder_actor_id),
        parse_proposition(payload, "proposition")?,
        parse_stance(payload)?,
        parse_confidence(payload)?,
        SourceRef::Event(source_event_id),
        parse_tick(payload, "acquired_tick")?,
    );
    if payload.contains_key("channel") {
        belief = belief.with_channel(parse_channel(payload)?);
    }
    belief = belief.with_last_verified_tick(parse_optional_tick(payload, "last_verified_tick")?);
    belief = belief.with_stale_after_tick(parse_optional_tick(payload, "stale_after_tick")?);
    if let Some(observation_ids) = payload.get("observation_ids") {
        for observation_id in parse_observation_ids(observation_ids)? {
            belief = belief.with_observation(observation_id);
        }
    }
    if let Some(contradiction_ids) = payload.get("contradiction_ids") {
        for contradiction_id in parse_contradiction_ids(contradiction_ids)? {
            belief = belief.with_contradiction(contradiction_id);
        }
    }
    Ok(belief)
}

fn parse_observation_ids(value: &str) -> Result<Vec<ObservationId>, EpistemicApplyError> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(|part| {
            ObservationId::new(part).map_err(|_| EpistemicApplyError::BadPayload {
                key: "observation_ids",
                value: part.to_string(),
            })
        })
        .collect()
}

fn parse_contradiction_ids(value: &str) -> Result<Vec<ContradictionId>, EpistemicApplyError> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(|part| {
            ContradictionId::new(part).map_err(|_| EpistemicApplyError::BadPayload {
                key: "contradiction_ids",
                value: part.to_string(),
            })
        })
        .collect()
}

fn parse_observation_payload(
    payload: &BTreeMap<&str, &str>,
) -> Result<Observation, EpistemicApplyError> {
    let observer_actor_id = parse_actor_id_epistemic(payload, "observer_actor_id")?;
    let observer_place_id = parse_place_id_epistemic(payload, "observer_place_id")?;
    let source_event_id = parse_event_id_epistemic(payload, "source_event_id")
        .map_err(|_| EpistemicApplyError::MissingSource)?;
    let channel = parse_channel(payload)?;
    let (subject, target) = if let Some(container_id) = payload.get("container_id") {
        let container_id =
            ContainerId::new(*container_id).map_err(|_| EpistemicApplyError::BadPayload {
                key: "container_id",
                value: (*container_id).to_string(),
            })?;
        (
            ObservationSubject::Container(container_id.clone()),
            ObservationTarget::Container(container_id),
        )
    } else {
        let place_id = parse_place_id_epistemic(payload, "place_id")?;
        (
            ObservationSubject::Place(place_id.clone()),
            ObservationTarget::Place(place_id),
        )
    };
    let mut observation = Observation::new(
        parse_observation_id(payload, "observation_id")?,
        observer_actor_id,
        channel,
        parse_tick(payload, "observed_tick")?,
        observer_place_id,
        subject,
        target,
        parse_confidence(payload)?,
        SourceRef::Event(source_event_id),
    );
    if let Some(alternatives) = payload.get("alternatives") {
        observation = observation.with_alternatives(parse_alternatives(alternatives));
    }
    Ok(observation)
}

fn parse_alternatives(value: &str) -> std::collections::BTreeSet<String> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_contradiction_payload(
    payload: &BTreeMap<&str, &str>,
) -> Result<Contradiction, EpistemicApplyError> {
    Ok(Contradiction::new(
        parse_contradiction_id(payload, "contradiction_id")?,
        parse_actor_id_epistemic(payload, "holder_actor_id")?,
        ContradictionKind::ExpectedItemAbsentFromContainer,
        parse_belief_id(payload, "prior_expectation_belief_id")?,
        parse_observation_id(payload, "contradicting_observation_id")?,
        parse_proposition(payload, "expected_proposition")?,
        parse_proposition(payload, "observed_proposition")?,
        parse_tick(payload, "detected_tick")?,
    ))
}

fn parse_actor_id(payload: &BTreeMap<&str, &str>) -> Result<ActorId, ApplyError> {
    let value = required(payload, "actor_id")?;
    ActorId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "actor_id",
        value: value.to_string(),
    })
}

fn parse_place_id(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<PlaceId, ApplyError> {
    let value = required(payload, key)?;
    PlaceId::new(value).map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_door_id(payload: &BTreeMap<&str, &str>) -> Result<DoorId, ApplyError> {
    let value = required(payload, "door_id")?;
    DoorId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "door_id",
        value: value.to_string(),
    })
}

fn parse_container_id(payload: &BTreeMap<&str, &str>) -> Result<ContainerId, ApplyError> {
    let value = required(payload, "container_id")?;
    ContainerId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "container_id",
        value: value.to_string(),
    })
}

fn parse_item_id(payload: &BTreeMap<&str, &str>) -> Result<ItemId, ApplyError> {
    let value = required(payload, "item_id")?;
    ItemId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "item_id",
        value: value.to_string(),
    })
}

fn parse_food_supply_id(payload: &BTreeMap<&str, &str>) -> Result<FoodSupplyId, ApplyError> {
    let value = required(payload, "food_supply_id")?;
    FoodSupplyId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "food_supply_id",
        value: value.to_string(),
    })
}

fn parse_intention_id(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<IntentionId, ApplyError> {
    let value = required(payload, key)?;
    IntentionId::new(value).map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_routine_execution_id(
    payload: &BTreeMap<&str, &str>,
) -> Result<RoutineExecutionId, ApplyError> {
    let value = required(payload, "routine_execution_id")?;
    RoutineExecutionId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "routine_execution_id",
        value: value.to_string(),
    })
}

fn parse_need_kind(payload: &BTreeMap<&str, &str>) -> Result<NeedKind, ApplyError> {
    let value = required(payload, "need_kind")?;
    match value {
        "hunger" => Ok(NeedKind::Hunger),
        "fatigue" => Ok(NeedKind::Fatigue),
        "safety" => Ok(NeedKind::Safety),
        _ => Err(ApplyError::BadPayload {
            key: "need_kind",
            value: value.to_string(),
        }),
    }
}

fn parse_i32(payload: &BTreeMap<&str, &str>, key: &'static str) -> Result<i32, ApplyError> {
    let value = required(payload, key)?;
    value.parse::<i32>().map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_u32(payload: &BTreeMap<&str, &str>, key: &'static str) -> Result<u32, ApplyError> {
    let value = required(payload, key)?;
    value.parse::<u32>().map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_action_id_for_agent(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<ActionId, ApplyError> {
    let value = required(payload, key)?;
    ActionId::new(value).map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_need_change_cause(payload: &BTreeMap<&str, &str>) -> Result<NeedChangeCause, ApplyError> {
    let cause_kind = required(payload, "cause_kind")?;
    match cause_kind {
        "fixture_initial" => Ok(NeedChangeCause::FixtureInitial),
        "tick_delta" => Ok(NeedChangeCause::TickDelta),
        "action_effect" => Ok(NeedChangeCause::ActionEffect(parse_action_id_for_agent(
            payload,
            "cause_action_id",
        )?)),
        "routine_effect" => Ok(NeedChangeCause::RoutineEffect(parse_routine_execution_id(
            payload,
        )?)),
        _ => Err(ApplyError::BadPayload {
            key: "cause_kind",
            value: cause_kind.to_string(),
        }),
    }
}

fn parse_intention_status(payload: &BTreeMap<&str, &str>) -> Result<IntentionStatus, ApplyError> {
    let value = required(payload, "status")?;
    match value {
        "active" => Ok(IntentionStatus::Active),
        "suspended" => Ok(IntentionStatus::Suspended),
        "completed" => Ok(IntentionStatus::Completed),
        "failed" => Ok(IntentionStatus::Failed),
        "abandoned" => Ok(IntentionStatus::Abandoned),
        "interrupted" => Ok(IntentionStatus::Interrupted),
        _ => Err(ApplyError::BadPayload {
            key: "status",
            value: value.to_string(),
        }),
    }
}

fn parse_intention_source(payload: &BTreeMap<&str, &str>) -> Result<IntentionSource, ApplyError> {
    let value = required(payload, "source")?;
    match value {
        "need_pressure" => Ok(IntentionSource::NeedPressure),
        "routine_duty" => Ok(IntentionSource::RoutineDuty),
        "candidate_goal_selection" => Ok(IntentionSource::CandidateGoalSelection),
        "safety_interruption" => Ok(IntentionSource::SafetyInterruption),
        "fixture_routine_assignment" => Ok(IntentionSource::FixtureRoutineAssignment),
        "fallback" => Ok(IntentionSource::Fallback),
        _ => Err(ApplyError::BadPayload {
            key: "source",
            value: value.to_string(),
        }),
    }
}

fn parse_candidate_goal_id(
    payload: &BTreeMap<&str, &str>,
) -> Result<crate::ids::CandidateGoalId, ApplyError> {
    let value = required(payload, "selected_goal_id")?;
    crate::ids::CandidateGoalId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "selected_goal_id",
        value: value.to_string(),
    })
}

fn parse_decision_trace_id(
    payload: &BTreeMap<&str, &str>,
) -> Result<crate::ids::DecisionTraceId, ApplyError> {
    let value = required(payload, "trace_id")?;
    crate::ids::DecisionTraceId::new(value).map_err(|_| ApplyError::BadPayload {
        key: "trace_id",
        value: value.to_string(),
    })
}

fn parse_optional_routine_template_id(
    payload: &BTreeMap<&str, &str>,
) -> Result<Option<crate::ids::RoutineTemplateId>, ApplyError> {
    let Some(value) = payload.get("selected_routine_method").copied() else {
        return Ok(None);
    };
    if value.is_empty() {
        return Ok(None);
    }
    crate::ids::RoutineTemplateId::new(value)
        .map(Some)
        .map_err(|_| ApplyError::BadPayload {
            key: "selected_routine_method",
            value: value.to_string(),
        })
}

fn parse_u8(payload: &BTreeMap<&str, &str>, key: &'static str) -> Result<u8, ApplyError> {
    let value = required(payload, key)?;
    value.parse::<u8>().map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn parse_u64_agent(payload: &BTreeMap<&str, &str>, key: &'static str) -> Result<u64, ApplyError> {
    let value = required(payload, key)?;
    value.parse::<u64>().map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn apply_need_delta(
    state: &mut AgentState,
    event: &EventEnvelope,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let actor_id = parse_actor_id(payload)?;
    let need_kind = parse_need_kind(payload)?;
    let delta = parse_i32(payload, "delta")?;
    let cause = parse_need_change_cause(payload)?;
    assert_single_tick_delta_charge(state, event, payload, &actor_id, need_kind)?;
    if matches!(cause, NeedChangeCause::FixtureInitial) {
        let needs = state.needs_by_actor.entry(actor_id.clone()).or_default();
        needs
            .entry(need_kind)
            .or_insert_with(|| crate::agent::NeedState::initial(need_kind, 0, cause.clone()));
    }

    let need_state = state
        .needs_by_actor
        .get_mut(&actor_id)
        .and_then(|needs| needs.get_mut(&need_kind))
        .ok_or_else(|| ApplyError::MissingNeedState {
            actor_id: actor_id.clone(),
            need_kind,
        })?;

    need_state.apply_delta(delta, cause);
    Ok(())
}

fn assert_single_tick_delta_charge(
    state: &mut AgentState,
    event: &EventEnvelope,
    payload: &BTreeMap<&str, &str>,
    actor_id: &ActorId,
    need_kind: NeedKind,
) -> Result<(), ApplyError> {
    let Some(elapsed_tick_value) = payload.get("elapsed_ticks") else {
        return Ok(());
    };
    let elapsed_ticks = elapsed_tick_value
        .parse::<u64>()
        .map_err(|_| ApplyError::MalformedElapsedTicks((*elapsed_tick_value).to_string()))?;
    if elapsed_ticks == 0 {
        return Ok(());
    }
    let Some(cause_kind) = payload.get("cause_kind") else {
        return Ok(());
    };
    if !matches!(*cause_kind, "tick_delta" | "action_effect") {
        return Ok(());
    }
    let first_tick = event
        .sim_tick
        .value()
        .saturating_sub(elapsed_ticks)
        .saturating_add(1);
    for tick in first_tick..=event.sim_tick.value() {
        let inserted = state
            .need_tick_charges
            .insert((actor_id.clone(), need_kind, tick));
        if !inserted {
            return Err(ApplyError::DuplicateNeedTickCharge {
                actor_id: actor_id.clone(),
                need_kind,
                tick,
            });
        }
    }
    Ok(())
}

fn apply_intention_started(
    state: &mut AgentState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let intention_id = parse_intention_id(payload, "intention_id")?;
    let actor_id = parse_actor_id(payload)?;
    let status = parse_intention_status(payload)?;
    if status != IntentionStatus::Active {
        return Err(ApplyError::PreconditionMismatch {
            field: "status",
            expected: IntentionStatus::Active.stable_id().to_string(),
            actual: status.stable_id().to_string(),
        });
    }
    let intention = Intention::adopt(
        intention_id.clone(),
        actor_id.clone(),
        parse_intention_source(payload)?,
        parse_candidate_goal_id(payload)?,
        parse_optional_routine_template_id(payload)?,
        payload
            .get("current_step")
            .map(|value| (*value).to_string()),
        parse_u8(payload, "durability_level")?,
        SimTick::new(parse_u64_agent(payload, "start_tick")?),
        parse_decision_trace_id(payload)?,
    );
    state
        .active_intention_by_actor
        .insert(actor_id, intention_id.clone());
    state.intentions.insert(intention_id, intention);
    Ok(())
}

fn apply_intention_transition(
    state: &mut AgentState,
    event_type: EventKind,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let intention_id = parse_intention_id(payload, "intention_id")?;
    let status = parse_intention_status(payload)?;
    let reason = required(payload, "reason")?.to_string();
    let intention = state
        .intentions
        .get_mut(&intention_id)
        .ok_or_else(|| ApplyError::MissingIntention(intention_id.clone()))?;

    let expected_status = match event_type {
        EventKind::IntentionContinued | EventKind::IntentionResumed => IntentionStatus::Active,
        EventKind::IntentionSuspended => IntentionStatus::Suspended,
        EventKind::IntentionCompleted => IntentionStatus::Completed,
        EventKind::IntentionFailed => IntentionStatus::Failed,
        EventKind::IntentionAbandoned => IntentionStatus::Abandoned,
        EventKind::IntentionInterrupted => IntentionStatus::Interrupted,
        _ => status,
    };
    if status != expected_status {
        return Err(ApplyError::PreconditionMismatch {
            field: "status",
            expected: expected_status.stable_id().to_string(),
            actual: status.stable_id().to_string(),
        });
    }

    match event_type {
        EventKind::IntentionContinued => {
            intention.status = IntentionStatus::Active;
            intention.status_reason = Some(reason);
            if let Some(progress_tick) = payload.get("progress_tick") {
                let tick = progress_tick
                    .parse::<u64>()
                    .map_err(|_| ApplyError::BadPayload {
                        key: "progress_tick",
                        value: (*progress_tick).to_string(),
                    })?;
                let step = required(payload, "current_step")?;
                intention.record_progress(SimTick::new(tick), step);
            }
        }
        EventKind::IntentionResumed => {
            intention
                .reactivate(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        EventKind::IntentionSuspended => {
            intention
                .suspend(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        EventKind::IntentionCompleted => {
            intention
                .complete(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        EventKind::IntentionFailed => {
            intention
                .fail(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        EventKind::IntentionAbandoned => {
            intention
                .abandon(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        EventKind::IntentionInterrupted => {
            intention
                .interrupt(reason)
                .map_err(|err| ApplyError::BadPayload {
                    key: "status",
                    value: format!("{err:?}"),
                })?
        }
        _ => {}
    }
    Ok(())
}

fn apply_routine_step_transition(
    state: &mut AgentState,
    event_type: EventKind,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let routine_execution_id = parse_routine_execution_id(payload)?;
    let tick_value = required(payload, "progress_tick")?;
    let tick = tick_value
        .parse::<u64>()
        .map_err(|_| ApplyError::BadPayload {
            key: "progress_tick",
            value: tick_value.to_string(),
        })?;
    let execution = state
        .routine_executions
        .get_mut(&routine_execution_id)
        .ok_or_else(|| ApplyError::MissingRoutineExecution(routine_execution_id.clone()))?;

    match event_type {
        EventKind::RoutineStepStarted => {
            let action_id = crate::ids::SemanticActionId::new(required(payload, "action_id")?)
                .map_err(|_| ApplyError::BadPayload {
                    key: "action_id",
                    value: required(payload, "action_id")
                        .unwrap_or_default()
                        .to_string(),
                })?;
            execution.start_step(SimTick::new(tick), action_id);
        }
        EventKind::RoutineStepCompleted => execution.complete_step(SimTick::new(tick)),
        EventKind::RoutineStepFailed => {
            execution.fail(SimTick::new(tick), required(payload, "reason")?)
        }
        _ => {}
    }
    if let Some(fallback_attempts) = payload.get("fallback_attempts") {
        let attempts = fallback_attempts
            .parse::<u32>()
            .map_err(|_| ApplyError::BadPayload {
                key: "fallback_attempts",
                value: (*fallback_attempts).to_string(),
            })?;
        execution.fallback_attempts = attempts;
    }
    if event_type == EventKind::RoutineStepStarted {
        execution.step_status = RoutineStepStatus::InProgress;
    }
    Ok(())
}

fn expect_bool(
    payload: &BTreeMap<&str, &str>,
    key: &'static str,
    expected: bool,
) -> Result<(), ApplyError> {
    let value = required(payload, key)?;
    let actual = match value {
        "true" => true,
        "false" => false,
        _ => {
            return Err(ApplyError::BadPayload {
                key,
                value: value.to_string(),
            })
        }
    };
    if actual == expected {
        Ok(())
    } else {
        Err(ApplyError::PreconditionMismatch {
            field: key,
            expected: expected.to_string(),
            actual: actual.to_string(),
        })
    }
}

fn apply_actor_moved(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let actor_id = parse_actor_id(payload)?;
    let from_place_id = parse_place_id(payload, "from_place_id")?;
    let to_place_id = parse_place_id(payload, "to_place_id")?;

    if !state.places.contains_key(&from_place_id) {
        return Err(ApplyError::MissingPlace(from_place_id));
    }
    if !state.places.contains_key(&to_place_id) {
        return Err(ApplyError::MissingPlace(to_place_id));
    }

    let actor = state
        .actors
        .get_mut(&actor_id)
        .ok_or_else(|| ApplyError::MissingActor(actor_id.clone()))?;
    if actor.current_place_id != from_place_id {
        return Err(ApplyError::PreconditionMismatch {
            field: "actor.current_place_id",
            expected: from_place_id.to_string(),
            actual: actor.current_place_id.to_string(),
        });
    }

    actor.current_place_id = to_place_id.clone();
    if let Some(from_place) = state.places.get_mut(&from_place_id) {
        from_place.local_actor_ids.remove(&actor_id);
    }
    if let Some(to_place) = state.places.get_mut(&to_place_id) {
        to_place.local_actor_ids.insert(actor_id);
    }
    Ok(())
}

fn apply_door_open_state(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
    expected_old: bool,
    new_value: bool,
) -> Result<(), ApplyError> {
    let door_id = parse_door_id(payload)?;
    expect_bool(payload, "was_open", expected_old)?;
    expect_bool(payload, "now_open", new_value)?;
    let door = state
        .doors
        .get_mut(&door_id)
        .ok_or_else(|| ApplyError::MissingDoor(door_id.clone()))?;
    if door.is_open != expected_old {
        return Err(ApplyError::PreconditionMismatch {
            field: "door.is_open",
            expected: expected_old.to_string(),
            actual: door.is_open.to_string(),
        });
    }
    door.is_open = new_value;
    Ok(())
}

fn apply_container_open_state(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
    expected_old: bool,
    new_value: bool,
) -> Result<(), ApplyError> {
    let container_id = parse_container_id(payload)?;
    expect_bool(payload, "was_open", expected_old)?;
    expect_bool(payload, "now_open", new_value)?;
    let container = state
        .containers
        .get_mut(&container_id)
        .ok_or_else(|| ApplyError::MissingContainer(container_id.clone()))?;
    if container.is_open != expected_old {
        return Err(ApplyError::PreconditionMismatch {
            field: "container.is_open",
            expected: expected_old.to_string(),
            actual: container.is_open.to_string(),
        });
    }
    container.is_open = new_value;
    Ok(())
}

fn apply_item_taken_from_place(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let item_id = parse_item_id(payload)?;
    let actor_id = parse_actor_id(payload)?;
    let place_id = parse_place_id(payload, "place_id")?;

    require_item_location(state, &item_id, Location::AtPlace(place_id.clone()))?;
    require_actor(state, &actor_id)?;
    let item = state
        .items
        .get_mut(&item_id)
        .ok_or_else(|| ApplyError::MissingItem(item_id.clone()))?;
    item.location = Location::CarriedBy(actor_id.clone());
    state
        .actors
        .get_mut(&actor_id)
        .expect("actor checked")
        .carried_item_ids
        .insert(item_id.clone());
    if let Some(place) = state.places.get_mut(&place_id) {
        place.local_item_ids.remove(&item_id);
    }
    Ok(())
}

fn apply_item_removed_from_container(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let item_id = parse_item_id(payload)?;
    let actor_id = parse_actor_id(payload)?;
    let container_id = parse_container_id(payload)?;

    require_item_location(state, &item_id, Location::InContainer(container_id.clone()))?;
    require_actor(state, &actor_id)?;
    let container = state
        .containers
        .get_mut(&container_id)
        .ok_or_else(|| ApplyError::MissingContainer(container_id.clone()))?;
    if !container.contents.remove(&item_id) {
        return Err(ApplyError::PreconditionMismatch {
            field: "container.contents",
            expected: item_id.to_string(),
            actual: "missing".to_string(),
        });
    }
    state
        .items
        .get_mut(&item_id)
        .expect("item location checked")
        .location = Location::CarriedBy(actor_id.clone());
    state
        .actors
        .get_mut(&actor_id)
        .expect("actor checked")
        .carried_item_ids
        .insert(item_id);
    Ok(())
}

fn apply_item_placed_in_place(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let item_id = parse_item_id(payload)?;
    let actor_id = parse_actor_id(payload)?;
    let place_id = parse_place_id(payload, "place_id")?;

    require_item_location(state, &item_id, Location::CarriedBy(actor_id.clone()))?;
    if !state.places.contains_key(&place_id) {
        return Err(ApplyError::MissingPlace(place_id));
    }
    state
        .items
        .get_mut(&item_id)
        .expect("item location checked")
        .location = Location::AtPlace(place_id.clone());
    if let Some(actor) = state.actors.get_mut(&actor_id) {
        actor.carried_item_ids.remove(&item_id);
    }
    state
        .places
        .get_mut(&place_id)
        .expect("place checked")
        .local_item_ids
        .insert(item_id);
    Ok(())
}

fn apply_item_placed_in_container(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let item_id = parse_item_id(payload)?;
    let actor_id = parse_actor_id(payload)?;
    let container_id = parse_container_id(payload)?;

    require_item_location(state, &item_id, Location::CarriedBy(actor_id.clone()))?;
    if !state.containers.contains_key(&container_id) {
        return Err(ApplyError::MissingContainer(container_id));
    }
    state
        .items
        .get_mut(&item_id)
        .expect("item location checked")
        .location = Location::InContainer(container_id.clone());
    if let Some(actor) = state.actors.get_mut(&actor_id) {
        actor.carried_item_ids.remove(&item_id);
    }
    state
        .containers
        .get_mut(&container_id)
        .expect("container checked")
        .contents
        .insert(item_id);
    Ok(())
}

fn apply_food_consumed(
    state: &mut PhysicalState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let food_supply_id = parse_food_supply_id(payload)?;
    let servings_consumed = parse_u32(payload, "servings_consumed")?;
    let servings_before = parse_u32(payload, "servings_before")?;
    let servings_after = parse_u32(payload, "servings_after")?;
    if servings_before.saturating_sub(servings_consumed) != servings_after {
        return Err(ApplyError::PreconditionMismatch {
            field: "food_supply.servings_after",
            expected: servings_before
                .saturating_sub(servings_consumed)
                .to_string(),
            actual: servings_after.to_string(),
        });
    }

    let food = state
        .food_supplies
        .get_mut(&food_supply_id)
        .ok_or_else(|| ApplyError::MissingFoodSupply(food_supply_id.clone()))?;
    if food.servings != servings_before {
        return Err(ApplyError::PreconditionMismatch {
            field: "food_supply.servings",
            expected: servings_before.to_string(),
            actual: food.servings.to_string(),
        });
    }
    food.servings = servings_after;
    Ok(())
}

fn require_actor(state: &PhysicalState, actor_id: &ActorId) -> Result<(), ApplyError> {
    if state.actors.contains_key(actor_id) {
        Ok(())
    } else {
        Err(ApplyError::MissingActor(actor_id.clone()))
    }
}

fn require_item_location(
    state: &PhysicalState,
    item_id: &ItemId,
    expected: Location,
) -> Result<(), ApplyError> {
    let item = state
        .items
        .get(item_id)
        .ok_or_else(|| ApplyError::MissingItem(item_id.clone()))?;
    if item.location == expected {
        Ok(())
    } else {
        Err(ApplyError::PreconditionMismatch {
            field: "item.location",
            expected: format!("{expected:?}"),
            actual: format!("{:?}", item.location),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState, RoutineExecution,
        RoutineFamily,
    };
    use crate::events::{EventCause, EventKind};
    use crate::ids::{
        ActionId, CandidateGoalId, ContentManifestId, DecisionTraceId, EventId, IntentionId,
        ProcessId, RoutineExecutionId, RoutineTemplateId, SchemaVersion, SemanticActionId,
    };
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ContainerState, PlaceState};
    use crate::time::SimTick;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn base_state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state.places.insert(
            place_id("shop_front"),
            PlaceState::new(place_id("shop_front"), "Shop front"),
        );
        state.places.insert(
            place_id("back_room"),
            PlaceState::new(place_id("back_room"), "Back room"),
        );
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(actor_id("actor_tomas"), place_id("shop_front")),
        );
        state
            .places
            .get_mut(&place_id("shop_front"))
            .unwrap()
            .local_actor_ids
            .insert(actor_id("actor_tomas"));
        state.containers.insert(
            container_id("strongbox_tomas"),
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), place_id("shop_front")),
        );
        state
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id("actor_tomas")),
            ProposalSequence::new(0),
            ActionId::new("move").unwrap(),
            vec!["back_room".to_string()],
            "tie",
        )
    }

    fn event(kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_v1(
            EventId::new("event_0001").unwrap(),
            kind,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        event.payload = payload;
        if kind.requires_cause() {
            event.causes = vec![EventCause::Process(
                ProcessId::new("process_apply_test").unwrap(),
            )];
        }
        event
    }

    fn caused_agent_event(kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new("event_agent_0001").unwrap(),
            kind,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            ContentManifestId::new("phase3a_manifest").unwrap(),
            vec![EventCause::Process(
                ProcessId::new("process_agent").unwrap(),
            )],
        )
        .unwrap();
        event.payload = payload;
        event
    }

    fn agent_state() -> AgentState {
        let actor_id = actor_id("actor_tomas");
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id.clone(),
            [(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 490, NeedChangeCause::FixtureInitial),
            )]
            .into_iter()
            .collect(),
        );

        let intention = Intention::adopt(
            IntentionId::new("intention_breakfast").unwrap(),
            actor_id.clone(),
            IntentionSource::CandidateGoalSelection,
            CandidateGoalId::new("goal_breakfast").unwrap(),
            Some(RoutineTemplateId::new("routine_eat_meal").unwrap()),
            Some("check_known_container".to_string()),
            5,
            SimTick::ZERO,
            DecisionTraceId::new("trace_breakfast").unwrap(),
        );
        state
            .active_intention_by_actor
            .insert(actor_id.clone(), intention.intention_id.clone());
        state
            .intentions
            .insert(intention.intention_id.clone(), intention);

        state.routine_executions.insert(
            RoutineExecutionId::new("routine_exec_breakfast").unwrap(),
            RoutineExecution::new(
                RoutineExecutionId::new("routine_exec_breakfast").unwrap(),
                actor_id,
                RoutineTemplateId::new("routine_eat_meal").unwrap(),
                RoutineFamily::EatMeal,
                SimTick::ZERO,
                Some(SimTick::new(1)),
                Some(SimTick::new(5)),
                None,
                DecisionTraceId::new("trace_breakfast").unwrap(),
            ),
        );
        state
    }

    fn proposition() -> String {
        Proposition::ItemMissingFromExpectedLocation {
            item_id: ItemId::new("coin_stack_01").unwrap(),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        }
        .serialize_canonical()
    }

    fn belief_payload() -> Vec<PayloadField> {
        vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("belief_id", "belief_tomas_missing_coin"),
            PayloadField::new("holder_actor_id", "actor_tomas"),
            PayloadField::new("proposition", proposition()),
            PayloadField::new("stance", "believes_true"),
            PayloadField::new("confidence", "900"),
            PayloadField::new("source_event_id", "event_obs_absence"),
            PayloadField::new("acquired_tick", "3"),
            PayloadField::new("channel", "absence_marker"),
        ]
    }

    fn observation_payload() -> Vec<PayloadField> {
        vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("observation_id", "obs_tomas_checked_strongbox"),
            PayloadField::new("observer_actor_id", "actor_tomas"),
            PayloadField::new("channel", "absence_marker"),
            PayloadField::new("observed_tick", "3"),
            PayloadField::new("observer_place_id", "shop_front"),
            PayloadField::new("container_id", "strongbox_tomas"),
            PayloadField::new("confidence", "950"),
            PayloadField::new("source_event_id", "event_container_checked"),
        ]
    }

    #[test]
    fn valid_world_event_changes_declared_state_only() {
        let mut state = base_state();
        let before_containers = state.containers.clone();
        let before_items = state.items.clone();
        let moved = event(
            EventKind::ActorMoved,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("from_place_id", "shop_front"),
                PayloadField::new("to_place_id", "back_room"),
            ],
        );

        assert_eq!(apply_event(&mut state, &moved), Ok(ApplyOutcome::Applied));

        assert_eq!(
            state.actors[&actor_id("actor_tomas")].current_place_id,
            place_id("back_room")
        );
        assert_eq!(state.containers, before_containers);
        assert_eq!(state.items, before_items);
    }

    #[test]
    fn precondition_mismatch_errors_and_leaves_state_unchanged() {
        let mut state = base_state();
        let before = state.clone();
        let moved = event(
            EventKind::ActorMoved,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("from_place_id", "back_room"),
                PayloadField::new("to_place_id", "shop_front"),
            ],
        );

        assert!(matches!(
            apply_event(&mut state, &moved),
            Err(ApplyError::PreconditionMismatch { .. })
        ));
        assert_eq!(state, before);
    }

    #[test]
    fn stream_mismatch_rejects_before_mutation() {
        let mut state = base_state();
        let before = state.clone();
        let mut moved = event(
            EventKind::ActorMoved,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("from_place_id", "shop_front"),
                PayloadField::new("to_place_id", "back_room"),
            ],
        );
        moved.stream = EventStream::Diagnostic;

        assert_eq!(
            apply_event(&mut state, &moved),
            Err(ApplyError::EventKindStreamMismatch)
        );
        assert_eq!(state, before);
    }

    #[test]
    fn unsupported_schema_version_errors() {
        let mut state = base_state();
        let before = state.clone();
        let mut moved = event(EventKind::ActorMoved, vec![]);
        moved.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();

        assert_eq!(
            apply_event(&mut state, &moved),
            Err(ApplyError::UnsupportedSchemaVersion(
                "event_schema_v999".to_string()
            ))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn non_world_event_is_physical_noop() {
        let mut state = base_state();
        let before = state.clone();
        let diagnostic = event(EventKind::ActionRejected, vec![]);

        assert_eq!(
            apply_event(&mut state, &diagnostic),
            Ok(ApplyOutcome::NonWorldNoOp)
        );
        assert_eq!(state, before);
    }

    #[test]
    fn epistemic_event_is_physical_noop() {
        let mut state = base_state();
        let before = state.clone();
        let epistemic = event(EventKind::BeliefUpdated, belief_payload());

        assert_eq!(
            apply_event(&mut state, &epistemic),
            Ok(ApplyOutcome::NonWorldNoOp)
        );
        assert_eq!(state, before);
    }

    #[test]
    fn epistemic_application_updates_projection() {
        let mut projection =
            EpistemicProjection::new(ContentManifestId::new("phase2a_manifest").unwrap());
        let observation_event = event(EventKind::ObservationRecorded, observation_payload());
        let belief_event = event(EventKind::BeliefUpdated, belief_payload());

        assert_eq!(
            apply_epistemic_event(&mut projection, &observation_event),
            Ok(ApplyOutcome::Applied)
        );
        assert_eq!(
            apply_epistemic_event(&mut projection, &belief_event),
            Ok(ApplyOutcome::Applied)
        );

        assert!(projection
            .observation(&ObservationId::new("obs_tomas_checked_strongbox").unwrap())
            .is_some());
        assert!(projection.has_belief(&BeliefId::new("belief_tomas_missing_coin").unwrap()));
    }

    #[test]
    fn unsupported_epistemic_payload_version_errors() {
        let mut projection =
            EpistemicProjection::new(ContentManifestId::new("phase2a_manifest").unwrap());
        let mut payload = belief_payload();
        payload[0] = PayloadField::new("schema_version", "event_schema_v999");
        let belief_event = event(EventKind::BeliefUpdated, payload);

        assert_eq!(
            apply_epistemic_event(&mut projection, &belief_event),
            Err(EpistemicApplyError::UnsupportedPayloadSchemaVersion(
                "event_schema_v999".to_string()
            ))
        );
        assert!(projection.is_empty());
    }

    #[test]
    fn agent_need_delta_event_mutates_need_state_with_cause() {
        let mut state = agent_state();
        let event = caused_agent_event(
            EventKind::NeedDeltaApplied,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("delta", "40"),
                PayloadField::new("cause_kind", "tick_delta"),
            ],
        );

        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );

        let hunger = &state.needs_by_actor[&actor_id("actor_tomas")][&NeedKind::Hunger];
        assert_eq!(hunger.value(), 530);
        assert_eq!(hunger.last_change_cause(), &NeedChangeCause::TickDelta);
    }

    #[test]
    fn single_tick_delta_charge_rejects_overlapping_action_effect_duration() {
        let mut state = agent_state();
        let mut event = caused_agent_event(
            EventKind::NeedDeltaApplied,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("delta", "40"),
                PayloadField::new("elapsed_ticks", "2"),
                PayloadField::new("cause_kind", "action_effect"),
                PayloadField::new("cause_action_id", "sleep"),
            ],
        );
        event.sim_tick = SimTick::new(5);

        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );
        assert!(matches!(
            apply_agent_event(&mut state, &event),
            Err(ApplyError::DuplicateNeedTickCharge {
                actor_id: duplicate_actor_id,
                need_kind: NeedKind::Hunger,
                tick: 4,
            }) if duplicate_actor_id == actor_id("actor_tomas")
        ));
    }

    #[test]
    fn malformed_elapsed_ticks_rejects_without_mutating_need_state() {
        let mut state = agent_state();
        let before = state.clone();
        let event = caused_agent_event(
            EventKind::NeedDeltaApplied,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("delta", "40"),
                PayloadField::new("elapsed_ticks", "two"),
                PayloadField::new("cause_kind", "action_effect"),
                PayloadField::new("cause_action_id", "sleep"),
            ],
        );

        assert_eq!(
            apply_agent_event(&mut state, &event),
            Err(ApplyError::MalformedElapsedTicks("two".to_string()))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn agent_intention_transition_event_records_status_and_reason() {
        let mut state = agent_state();
        let event = caused_agent_event(
            EventKind::IntentionSuspended,
            vec![
                PayloadField::new("intention_id", "intention_breakfast"),
                PayloadField::new("status", "suspended"),
                PayloadField::new("reason", "known food source blocked"),
            ],
        );

        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );

        let intention = &state.intentions[&IntentionId::new("intention_breakfast").unwrap()];
        assert_eq!(intention.status, IntentionStatus::Suspended);
        assert_eq!(
            intention.status_reason.as_deref(),
            Some("known food source blocked")
        );
    }

    #[test]
    fn agent_routine_step_event_records_progress_and_fallbacks() {
        let mut state = agent_state();
        let event = caused_agent_event(
            EventKind::RoutineStepStarted,
            vec![
                PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
                PayloadField::new("progress_tick", "3"),
                PayloadField::new("action_id", "check_container.pantry"),
                PayloadField::new("fallback_attempts", "1"),
            ],
        );

        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );

        let execution =
            &state.routine_executions[&RoutineExecutionId::new("routine_exec_breakfast").unwrap()];
        assert_eq!(execution.step_status, RoutineStepStatus::InProgress);
        assert_eq!(execution.last_progress_tick, SimTick::new(3));
        assert_eq!(execution.fallback_attempts, 1);
        assert_eq!(
            execution.concrete_action_ancestry,
            vec![SemanticActionId::new("check_container.pantry").unwrap()]
        );
    }

    #[test]
    fn agent_trace_and_stuck_diagnostic_events_record_canonical_payloads() {
        let mut state = agent_state();
        let trace_event = caused_agent_event(
            EventKind::DecisionTraceRecorded,
            vec![
                PayloadField::new("trace_schema_version", "1"),
                PayloadField::new("trace_id", "trace_breakfast"),
                PayloadField::new(
                    "trace_canonical",
                    "decision_trace_v1|trace_breakfast|actor_tomas|1|2|completed|0|true|74657374",
                ),
            ],
        );
        let diagnostic_event = caused_agent_event(
            EventKind::StuckDiagnosticRecorded,
            vec![
                PayloadField::new("diagnostic_schema_version", "1"),
                PayloadField::new("diagnostic_id", "stuck_food_missing"),
                PayloadField::new(
                    "diagnostic_canonical",
                    "stuck_diagnostic_v1|stuck_food_missing|actor_tomas|1|2|-|-|-|-|-|-|-|resource|666f6f64|6163746f725f6b6e6f776e|6465627567|7265747279|replanning",
                ),
            ],
        );

        assert_eq!(
            apply_agent_event(&mut state, &trace_event),
            Ok(ApplyOutcome::Applied)
        );
        assert_eq!(
            apply_agent_event(&mut state, &diagnostic_event),
            Ok(ApplyOutcome::Applied)
        );

        let trace = &state.decision_traces[&DecisionTraceId::new("trace_breakfast").unwrap()];
        assert_eq!(
            trace.typed_diagnostic.responsible_layer,
            crate::agent::ResponsibleLayer::CandidateGeneration
        );
        assert_eq!(
            trace.typed_diagnostic.blocker_code,
            crate::agent::BlockerCode::None
        );
        assert!(trace
            .serialize_canonical()
            .starts_with("decision_trace_v1|trace_breakfast|actor_tomas|1|2|completed"));
        let diagnostic = &state.stuck_diagnostics
            [&crate::ids::StuckDiagnosticId::new("stuck_food_missing").unwrap()];
        assert_eq!(
            diagnostic.typed_diagnostic.blocker_code,
            crate::agent::BlockerCode::Resource
        );
        assert!(diagnostic.serialize_canonical().starts_with(
            "stuck_diagnostic_v1|stuck_food_missing|actor_tomas|1|2|-|-|-|-|-|-|-|resource"
        ));
    }

    #[test]
    fn agent_trace_event_rejects_unsupported_payload_version() {
        let mut state = agent_state();
        let trace_event = caused_agent_event(
            EventKind::DecisionTraceRecorded,
            vec![
                PayloadField::new("trace_schema_version", "2"),
                PayloadField::new("trace_id", "trace_breakfast"),
                PayloadField::new(
                    "trace_canonical",
                    "decision_trace_v1|trace_breakfast|actor_tomas|1|2|completed|0|true|74657374",
                ),
            ],
        );

        assert!(matches!(
            apply_agent_event(&mut state, &trace_event),
            Err(ApplyError::BadPayload {
                key: "trace_schema_version",
                ..
            })
        ));
    }

    #[test]
    fn threshold_and_episode_events_materialize_agent_projection_state() {
        let mut state = AgentState::default();
        let mut threshold = caused_agent_event(
            EventKind::NeedThresholdCrossed,
            vec![
                PayloadField::new("payload_schema_version", "1"),
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("from_value", "249"),
                PayloadField::new("to_value", "254"),
                PayloadField::new("from_band", "comfortable"),
                PayloadField::new("to_band", "rising"),
            ],
        );
        threshold.event_id = EventId::new("event.threshold.hunger.actor_tomas").unwrap();
        threshold.actor_id = Some(actor_id("actor_tomas"));
        threshold.sim_tick = SimTick::new(7);

        let mut sleep = caused_agent_event(EventKind::SleepStarted, Vec::new());
        sleep.event_id = EventId::new("event.sleep.started.actor_tomas").unwrap();
        sleep.actor_id = Some(actor_id("actor_tomas"));
        sleep.proposal_id = Some(crate::ids::ProposalId::new("proposal_sleep").unwrap());
        sleep.sim_tick = SimTick::new(8);
        sleep.payload = vec![
            PayloadField::new("payload_schema_version", "1"),
            PayloadField::new("sleep_place_id", "home_tomas"),
        ];
        sleep.effects_summary = "sleep episode started".to_string();

        assert_eq!(
            apply_agent_event(&mut state, &threshold),
            Ok(ApplyOutcome::Applied)
        );
        assert_eq!(
            apply_agent_event(&mut state, &sleep),
            Ok(ApplyOutcome::Applied)
        );

        assert_eq!(state.need_threshold_crossings.len(), 1);
        assert_eq!(
            state.need_threshold_crossings[&threshold.event_id].to_band,
            "rising"
        );
        assert!(state.need_threshold_crossings[&threshold.event_id]
            .payload_fields
            .contains(&("payload_schema_version".to_string(), "1".to_string())));
        assert_eq!(state.ordinary_life_episodes.len(), 1);
        assert_eq!(
            state.ordinary_life_episodes[&sleep.event_id].event_kind,
            "sleep_started"
        );
        assert_eq!(
            state.ordinary_life_episodes[&sleep.event_id].payload_fields,
            vec![
                ("payload_schema_version".to_string(), "1".to_string()),
                ("sleep_place_id".to_string(), "home_tomas".to_string())
            ]
        );
    }

    #[test]
    fn duplicate_duration_terminal_rejected_live() {
        let mut state = AgentState::default();
        let mut start = caused_agent_event(EventKind::WorkBlockStarted, Vec::new());
        start.event_id = EventId::new("event.work.started.actor_tomas").unwrap();
        start.actor_id = Some(actor_id("actor_tomas"));
        start.payload = vec![PayloadField::new("payload_schema_version", "1")];

        let mut completed = caused_agent_event(EventKind::WorkBlockCompleted, Vec::new());
        completed.event_id = EventId::new("event.work.completed.actor_tomas").unwrap();
        completed.actor_id = Some(actor_id("actor_tomas"));
        completed.causes = vec![EventCause::Event(start.event_id.clone())];
        completed.payload = vec![PayloadField::new("payload_schema_version", "1")];

        let mut failed = caused_agent_event(EventKind::WorkBlockFailed, Vec::new());
        failed.event_id = EventId::new("event.work.failed.actor_tomas").unwrap();
        failed.actor_id = Some(actor_id("actor_tomas"));
        failed.causes = vec![EventCause::Event(start.event_id.clone())];
        failed.payload = vec![PayloadField::new("payload_schema_version", "1")];

        assert_eq!(
            apply_agent_event(&mut state, &start),
            Ok(ApplyOutcome::Applied)
        );
        assert_eq!(
            apply_agent_event(&mut state, &completed),
            Ok(ApplyOutcome::Applied)
        );
        assert_eq!(
            apply_agent_event(&mut state, &failed),
            Err(ApplyError::DuplicateDurationTerminal {
                start_event_id: start.event_id
            })
        );
    }
}
