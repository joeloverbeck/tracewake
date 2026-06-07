use std::collections::BTreeMap;

use crate::agent::{IntentionStatus, NeedChangeCause, NeedKind, RoutineStepStatus};
use crate::epistemics::{
    Belief, Channel, Confidence, Contradiction, ContradictionKind, EpistemicProjection, HolderKind,
    Observation, ObservationSubject, ObservationTarget, Proposition, SourceRef, Stance,
};
use crate::events::{EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{
    ActionId, ActorId, BeliefId, ContainerId, ContradictionId, DoorId, EventId, IntentionId,
    ItemId, ObservationId, PlaceId, RoutineExecutionId,
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
    PreconditionMismatch {
        field: &'static str,
        expected: String,
        actual: String,
    },
    EventKindStreamMismatch,
    NonAgentEvent,
    MissingNeedState {
        actor_id: ActorId,
        need_kind: NeedKind,
    },
    MissingIntention(IntentionId),
    MissingRoutineExecution(RoutineExecutionId),
    MissingCause,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EpistemicApplyError {
    UnsupportedEventSchemaVersion(String),
    UnsupportedPayloadSchemaVersion(String),
    MissingPayload(&'static str),
    BadPayload { key: &'static str, value: String },
    EventKindStreamMismatch,
    NonEpistemicEvent,
    MissingHolder,
    MissingSource,
}

pub fn apply_event(
    state: &mut PhysicalState,
    event: &EventEnvelope,
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
        EventKind::ActorWaited | EventKind::TimeAdvanced => Ok(ApplyOutcome::WorldNoOp),
        _ => Ok(ApplyOutcome::NonWorldNoOp),
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
            let observation = parse_observation_payload(&payload)?;
            projection.insert_observation(observation);
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
            apply_need_delta(state, &payload).map(|_| ApplyOutcome::Applied)
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
            let trace_id = crate::ids::DecisionTraceId::new(required(&payload, "trace_id")?)
                .map_err(|_| ApplyError::BadPayload {
                    key: "trace_id",
                    value: required(&payload, "trace_id")
                        .unwrap_or_default()
                        .to_string(),
                })?;
            state
                .decision_traces
                .insert(trace_id, required(&payload, "trace_canonical")?.to_string());
            Ok(ApplyOutcome::Applied)
        }
        EventKind::StuckDiagnosticRecorded => {
            let diagnostic_id =
                crate::ids::StuckDiagnosticId::new(required(&payload, "diagnostic_id")?).map_err(
                    |_| ApplyError::BadPayload {
                        key: "diagnostic_id",
                        value: required(&payload, "diagnostic_id")
                            .unwrap_or_default()
                            .to_string(),
                    },
                )?;
            state.stuck_diagnostics.insert(
                diagnostic_id,
                required(&payload, "diagnostic_canonical")?.to_string(),
            );
            Ok(ApplyOutcome::Applied)
        }
        EventKind::NeedThresholdCrossed
        | EventKind::CandidateGoalsEvaluated
        | EventKind::IntentionStarted
        | EventKind::SleepStarted
        | EventKind::SleepCompleted
        | EventKind::SleepInterrupted
        | EventKind::FoodConsumed
        | EventKind::FoodServiceUsed
        | EventKind::EatFailed
        | EventKind::WorkBlockStarted
        | EventKind::WorkBlockCompleted
        | EventKind::WorkBlockFailed
        | EventKind::ContinueRoutineProposed
        | EventKind::ContinueRoutineAccepted
        | EventKind::ContinueRoutineRejected
        | EventKind::NoHumanDayStarted
        | EventKind::NoHumanDayCompleted => Ok(ApplyOutcome::WorldNoOp),
        _ => Err(ApplyError::NonAgentEvent),
    }
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
    Ok(belief)
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

fn apply_need_delta(
    state: &mut AgentState,
    payload: &BTreeMap<&str, &str>,
) -> Result<(), ApplyError> {
    let actor_id = parse_actor_id(payload)?;
    let need_kind = parse_need_kind(payload)?;
    let delta = parse_i32(payload, "delta")?;
    let cause = parse_need_change_cause(payload)?;
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
    let tick = payload
        .get("progress_tick")
        .copied()
        .unwrap_or("0")
        .parse::<u64>()
        .map_err(|_| ApplyError::BadPayload {
            key: "progress_tick",
            value: payload
                .get("progress_tick")
                .copied()
                .unwrap_or("0")
                .to_string(),
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
        let mut state = PhysicalState::default();
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
            .observations_by_id
            .contains_key(&ObservationId::new("obs_tomas_checked_strongbox").unwrap()));
        assert!(projection
            .beliefs_by_id
            .contains_key(&BeliefId::new("belief_tomas_missing_coin").unwrap()));
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
        assert!(projection.beliefs_by_id.is_empty());
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
                PayloadField::new("trace_id", "trace_breakfast"),
                PayloadField::new("trace_canonical", "decision_trace_v1|minimal"),
            ],
        );
        let diagnostic_event = caused_agent_event(
            EventKind::StuckDiagnosticRecorded,
            vec![
                PayloadField::new("diagnostic_id", "diagnostic_food_missing"),
                PayloadField::new("diagnostic_canonical", "stuck_diagnostic_v1|minimal"),
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

        assert_eq!(
            state.decision_traces[&DecisionTraceId::new("trace_breakfast").unwrap()],
            "decision_trace_v1|minimal"
        );
        assert_eq!(
            state.stuck_diagnostics
                [&crate::ids::StuckDiagnosticId::new("diagnostic_food_missing").unwrap()],
            "stuck_diagnostic_v1|minimal"
        );
    }
}
