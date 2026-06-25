use crate::events::{EventCause, EventEnvelope, EventKind, EventStream};
use crate::ids::{ActorId, ContentManifestId, EventId, ProcessId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemporalProjection {
    pub reconstructed_final_frontier: SimTick,
    pub violations: Vec<TemporalDivergence>,
    pub scheduler_authority: SchedulerReplayAuthority,
    pub scheduler_authority_violations: Vec<SchedulerAuthorityDivergence>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchedulerReplayAuthority {
    pub next_proposal_sequence: u64,
    pub no_human_process_frontier: Option<SimTick>,
    pub loaded_actor_next_decision_ticks: Vec<LoadedActorReplayAuthority>,
    pub declared_processes: Vec<DeclaredProcessReplayAuthority>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedActorReplayAuthority {
    pub actor_id: ActorId,
    pub next_decision_tick: SimTick,
    pub source_event_id: EventId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredProcessReplayAuthority {
    pub process_id: ProcessId,
    pub first_due_tick: SimTick,
    pub cadence_ticks: u64,
    pub last_effective_tick: SimTick,
    pub source_event_ids: Vec<EventId>,
    pub content_manifest_id: ContentManifestId,
    pub random_provenance: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemporalDivergence {
    MissingPayload {
        event_id: EventId,
        field: &'static str,
    },
    BadPayload {
        event_id: EventId,
        field: &'static str,
        value: String,
    },
    PriorTickMismatch {
        event_id: EventId,
        expected: SimTick,
        actual: SimTick,
    },
    ResultingTickMismatch {
        event_id: EventId,
        expected: SimTick,
        actual: SimTick,
    },
    EnvelopePayloadMismatch {
        event_id: EventId,
        envelope_tick: SimTick,
        resulting_tick: SimTick,
    },
    MissingOrWrongCause {
        event_id: EventId,
    },
    MissingOrderingAncestry {
        event_id: EventId,
    },
    DuplicateTimeAdvanced {
        event_id: EventId,
        tick: SimTick,
    },
    BackwardTimeAdvanced {
        event_id: EventId,
        prior_tick: SimTick,
        resulting_tick: SimTick,
    },
    OrdinaryEventWithoutMarker {
        event_id: EventId,
        event_tick: SimTick,
        reconstructed_frontier: SimTick,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchedulerAuthorityDivergence {
    MissingPayload {
        event_id: EventId,
        field: &'static str,
    },
    BadPayload {
        event_id: EventId,
        field: &'static str,
        value: String,
    },
    MissingProcessId {
        event_id: EventId,
    },
    ProcessPayloadMismatch {
        event_id: EventId,
        process_id: ProcessId,
    },
    DeclaredProcessAuthorityMismatch {
        event_id: EventId,
        process_id: ProcessId,
    },
    MissingActorId {
        event_id: EventId,
    },
    ActorPayloadMismatch {
        event_id: EventId,
        actor_id: ActorId,
    },
}

impl TemporalDivergence {
    pub fn event_id(&self) -> &EventId {
        match self {
            Self::MissingPayload { event_id, .. }
            | Self::BadPayload { event_id, .. }
            | Self::PriorTickMismatch { event_id, .. }
            | Self::ResultingTickMismatch { event_id, .. }
            | Self::EnvelopePayloadMismatch { event_id, .. }
            | Self::MissingOrWrongCause { event_id }
            | Self::MissingOrderingAncestry { event_id }
            | Self::DuplicateTimeAdvanced { event_id, .. }
            | Self::BackwardTimeAdvanced { event_id, .. }
            | Self::OrdinaryEventWithoutMarker { event_id, .. } => event_id,
        }
    }
}

pub fn project_temporal_frontier(
    initial_frontier: SimTick,
    events: &[EventEnvelope],
) -> TemporalProjection {
    let mut frontier = initial_frontier;
    let mut violations = Vec::new();
    let mut scheduler_authority = SchedulerReplayAuthority::default();
    let mut scheduler_authority_violations = Vec::new();

    for event in events {
        scheduler_authority.next_proposal_sequence =
            scheduler_authority.next_proposal_sequence.max(
                event
                    .ordering_key
                    .proposal_sequence
                    .value()
                    .saturating_add(1),
            );
        if event.event_type == EventKind::DeclaredWorldProcessApplied
            && payload_value(event, "process_kind") == Some("declared_world_process")
        {
            match declared_process_authority_from_event(event) {
                Ok(authority) => upsert_declared_process_authority(
                    &mut scheduler_authority.declared_processes,
                    authority,
                    event,
                    &mut scheduler_authority_violations,
                ),
                Err(violation) => scheduler_authority_violations.push(violation),
            }
        }
        if is_actor_step_authority_event(event.event_type) {
            match loaded_actor_authority_from_event(event) {
                Ok(authority) => upsert_loaded_actor_authority(
                    &mut scheduler_authority.loaded_actor_next_decision_ticks,
                    authority,
                ),
                Err(violation) => scheduler_authority_violations.push(violation),
            }
        }
        if matches!(
            event.event_type,
            EventKind::NoHumanDayCompleted | EventKind::NoHumanAdvanceCompleted
        ) {
            scheduler_authority.no_human_process_frontier = Some(event.sim_tick);
        }

        if event.event_type == EventKind::TimeAdvanced {
            match validate_time_advanced(event, frontier) {
                Ok(resulting_tick) => frontier = resulting_tick,
                Err(violation) => violations.push(violation),
            }
            continue;
        }

        if event.stream == EventStream::World && event.sim_tick > frontier {
            violations.push(TemporalDivergence::OrdinaryEventWithoutMarker {
                event_id: event.event_id.clone(),
                event_tick: event.sim_tick,
                reconstructed_frontier: frontier,
            });
        }
    }

    TemporalProjection {
        reconstructed_final_frontier: frontier,
        violations,
        scheduler_authority,
        scheduler_authority_violations,
    }
}

fn upsert_loaded_actor_authority(
    authorities: &mut Vec<LoadedActorReplayAuthority>,
    authority: LoadedActorReplayAuthority,
) {
    if let Some(existing) = authorities
        .iter_mut()
        .find(|existing| existing.actor_id == authority.actor_id)
    {
        if authority.next_decision_tick >= existing.next_decision_tick {
            *existing = authority;
        }
        return;
    }
    authorities.push(authority);
}

fn upsert_declared_process_authority(
    authorities: &mut Vec<DeclaredProcessReplayAuthority>,
    authority: DeclaredProcessReplayAuthority,
    event: &EventEnvelope,
    violations: &mut Vec<SchedulerAuthorityDivergence>,
) {
    if let Some(existing) = authorities
        .iter_mut()
        .find(|existing| existing.process_id == authority.process_id)
    {
        if existing.first_due_tick != authority.first_due_tick
            || existing.cadence_ticks != authority.cadence_ticks
            || existing.source_event_ids != authority.source_event_ids
            || existing.content_manifest_id != authority.content_manifest_id
            || existing.random_provenance != authority.random_provenance
        {
            violations.push(
                SchedulerAuthorityDivergence::DeclaredProcessAuthorityMismatch {
                    event_id: event.event_id.clone(),
                    process_id: authority.process_id,
                },
            );
            return;
        }
        existing.last_effective_tick = authority.last_effective_tick;
        return;
    }
    authorities.push(authority);
}

fn loaded_actor_authority_from_event(
    event: &EventEnvelope,
) -> Result<LoadedActorReplayAuthority, SchedulerAuthorityDivergence> {
    let actor_id =
        event
            .actor_id
            .clone()
            .ok_or_else(|| SchedulerAuthorityDivergence::MissingActorId {
                event_id: event.event_id.clone(),
            })?;
    if let Some(payload_actor_id) = payload_value(event, "actor_id") {
        let payload_actor_id = ActorId::new(payload_actor_id.to_string()).map_err(|_| {
            SchedulerAuthorityDivergence::BadPayload {
                event_id: event.event_id.clone(),
                field: "actor_id",
                value: payload_actor_id.to_string(),
            }
        })?;
        if payload_actor_id != actor_id {
            return Err(SchedulerAuthorityDivergence::ActorPayloadMismatch {
                event_id: event.event_id.clone(),
                actor_id,
            });
        }
    }
    Ok(LoadedActorReplayAuthority {
        actor_id,
        next_decision_tick: event.sim_tick.next(),
        source_event_id: event.event_id.clone(),
    })
}

fn declared_process_authority_from_event(
    event: &EventEnvelope,
) -> Result<DeclaredProcessReplayAuthority, SchedulerAuthorityDivergence> {
    let process_id =
        event
            .process_id
            .clone()
            .ok_or_else(|| SchedulerAuthorityDivergence::MissingProcessId {
                event_id: event.event_id.clone(),
            })?;
    let payload_process_id = parse_process_id(event, "process_id")?;
    if payload_process_id != process_id {
        return Err(SchedulerAuthorityDivergence::ProcessPayloadMismatch {
            event_id: event.event_id.clone(),
            process_id,
        });
    }
    let time_marker_event_id = parse_event_id(event, "time_marker_event_id")?;
    let source_event_ids = event
        .causes
        .iter()
        .filter_map(|cause| match cause {
            EventCause::Event(event_id) if event_id != &time_marker_event_id => {
                Some(event_id.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    Ok(DeclaredProcessReplayAuthority {
        process_id,
        first_due_tick: parse_authority_tick(event, "first_due_tick")?,
        cadence_ticks: parse_authority_u64(event, "cadence_ticks")?.max(1),
        last_effective_tick: parse_authority_tick(event, "effective_tick")?,
        source_event_ids,
        content_manifest_id: parse_content_manifest_id(event, "content_manifest_id")?,
        random_provenance: payload_value(event, "random_provenance").map(ToString::to_string),
    })
}

fn is_actor_step_authority_event(event_type: EventKind) -> bool {
    matches!(
        event_type,
        EventKind::DecisionTraceRecorded | EventKind::StuckDiagnosticRecorded
    )
}

fn validate_time_advanced(
    event: &EventEnvelope,
    current_frontier: SimTick,
) -> Result<SimTick, TemporalDivergence> {
    if !event
        .causes
        .iter()
        .any(|cause| matches!(cause, EventCause::Process(_)))
    {
        return Err(TemporalDivergence::MissingOrWrongCause {
            event_id: event.event_id.clone(),
        });
    }
    if payload_value(event, "ordering_ancestry") != Some("canonical_world_step_v1") {
        return Err(TemporalDivergence::MissingOrderingAncestry {
            event_id: event.event_id.clone(),
        });
    }

    let prior_tick = parse_tick(event, "prior_tick")?;
    let resulting_tick = parse_tick(event, "resulting_tick")?;

    if resulting_tick < prior_tick {
        return Err(TemporalDivergence::BackwardTimeAdvanced {
            event_id: event.event_id.clone(),
            prior_tick,
            resulting_tick,
        });
    }
    if prior_tick < current_frontier || resulting_tick <= current_frontier {
        return Err(TemporalDivergence::DuplicateTimeAdvanced {
            event_id: event.event_id.clone(),
            tick: resulting_tick,
        });
    }
    if prior_tick != current_frontier {
        return Err(TemporalDivergence::PriorTickMismatch {
            event_id: event.event_id.clone(),
            expected: current_frontier,
            actual: prior_tick,
        });
    }
    let expected_resulting = prior_tick.next();
    if resulting_tick != expected_resulting {
        return Err(TemporalDivergence::ResultingTickMismatch {
            event_id: event.event_id.clone(),
            expected: expected_resulting,
            actual: resulting_tick,
        });
    }
    if event.sim_tick != resulting_tick {
        return Err(TemporalDivergence::EnvelopePayloadMismatch {
            event_id: event.event_id.clone(),
            envelope_tick: event.sim_tick,
            resulting_tick,
        });
    }

    Ok(resulting_tick)
}

fn parse_tick(event: &EventEnvelope, field: &'static str) -> Result<SimTick, TemporalDivergence> {
    let value = payload_value(event, field).ok_or_else(|| TemporalDivergence::MissingPayload {
        event_id: event.event_id.clone(),
        field,
    })?;
    value
        .parse::<u64>()
        .map(SimTick::new)
        .map_err(|_| TemporalDivergence::BadPayload {
            event_id: event.event_id.clone(),
            field,
            value: value.to_string(),
        })
}

fn parse_authority_tick(
    event: &EventEnvelope,
    field: &'static str,
) -> Result<SimTick, SchedulerAuthorityDivergence> {
    parse_authority_u64(event, field).map(SimTick::new)
}

fn parse_authority_u64(
    event: &EventEnvelope,
    field: &'static str,
) -> Result<u64, SchedulerAuthorityDivergence> {
    let value = payload_value(event, field).ok_or_else(|| {
        SchedulerAuthorityDivergence::MissingPayload {
            event_id: event.event_id.clone(),
            field,
        }
    })?;
    value
        .parse::<u64>()
        .map_err(|_| SchedulerAuthorityDivergence::BadPayload {
            event_id: event.event_id.clone(),
            field,
            value: value.to_string(),
        })
}

fn parse_event_id(
    event: &EventEnvelope,
    field: &'static str,
) -> Result<EventId, SchedulerAuthorityDivergence> {
    let value = payload_value(event, field).ok_or_else(|| {
        SchedulerAuthorityDivergence::MissingPayload {
            event_id: event.event_id.clone(),
            field,
        }
    })?;
    EventId::new(value.to_string()).map_err(|_| SchedulerAuthorityDivergence::BadPayload {
        event_id: event.event_id.clone(),
        field,
        value: value.to_string(),
    })
}

fn parse_process_id(
    event: &EventEnvelope,
    field: &'static str,
) -> Result<ProcessId, SchedulerAuthorityDivergence> {
    let value = payload_value(event, field).ok_or_else(|| {
        SchedulerAuthorityDivergence::MissingPayload {
            event_id: event.event_id.clone(),
            field,
        }
    })?;
    ProcessId::new(value.to_string()).map_err(|_| SchedulerAuthorityDivergence::BadPayload {
        event_id: event.event_id.clone(),
        field,
        value: value.to_string(),
    })
}

fn parse_content_manifest_id(
    event: &EventEnvelope,
    field: &'static str,
) -> Result<ContentManifestId, SchedulerAuthorityDivergence> {
    let value = payload_value(event, field).ok_or_else(|| {
        SchedulerAuthorityDivergence::MissingPayload {
            event_id: event.event_id.clone(),
            field,
        }
    })?;
    ContentManifestId::new(value.to_string()).map_err(|_| {
        SchedulerAuthorityDivergence::BadPayload {
            event_id: event.event_id.clone(),
            field,
            value: value.to_string(),
        }
    })
}

fn payload_value<'a>(event: &'a EventEnvelope, field: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|payload| payload.key == field)
        .map(|payload| payload.value.as_str())
}
