use crate::events::{EventCause, EventEnvelope, EventKind, EventStream};
use crate::ids::EventId;
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemporalProjection {
    pub reconstructed_final_frontier: SimTick,
    pub violations: Vec<TemporalDivergence>,
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

pub fn project_temporal_frontier(
    initial_frontier: SimTick,
    events: &[EventEnvelope],
) -> TemporalProjection {
    let mut frontier = initial_frontier;
    let mut violations = Vec::new();

    for event in events {
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
    }
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

fn payload_value<'a>(event: &'a EventEnvelope, field: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|payload| payload.key == field)
        .map(|payload| payload.value.as_str())
}
