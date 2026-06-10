use std::collections::BTreeMap;

use crate::events::log::EventLog;
use crate::events::{is_duration_terminal, EventCause, EventEnvelope, EventKind};
use crate::ids::{ActorId, EventId};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickRegime {
    Awake,
    Asleep,
    Working,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TickRegimeCounts {
    pub awake_ticks: u64,
    pub asleep_ticks: u64,
    pub working_ticks: u64,
}

pub fn classify_actor_tick_regimes(
    log: &EventLog,
    actor_id: &ActorId,
    from_exclusive: SimTick,
    to_inclusive: SimTick,
) -> TickRegimeCounts {
    classify_actor_tick_regimes_with_start(log, actor_id, from_exclusive, to_inclusive, None)
}

pub fn classify_actor_tick_regimes_with_start(
    log: &EventLog,
    actor_id: &ActorId,
    from_exclusive: SimTick,
    to_inclusive: SimTick,
    current_start: Option<&EventEnvelope>,
) -> TickRegimeCounts {
    let mut counts = TickRegimeCounts::default();
    for regime in
        actor_tick_regimes_with_start(log, actor_id, from_exclusive, to_inclusive, current_start)
    {
        match regime {
            TickRegime::Awake => counts.awake_ticks += 1,
            TickRegime::Asleep => counts.asleep_ticks += 1,
            TickRegime::Working => counts.working_ticks += 1,
        }
    }
    counts
}

pub fn actor_tick_regimes_with_start(
    log: &EventLog,
    actor_id: &ActorId,
    from_exclusive: SimTick,
    to_inclusive: SimTick,
    current_start: Option<&EventEnvelope>,
) -> Vec<TickRegime> {
    if to_inclusive.value() <= from_exclusive.value() {
        return Vec::new();
    }
    let intervals = duration_intervals(log, actor_id, current_start);
    ((from_exclusive.value() + 1)..=to_inclusive.value())
        .map(|tick| {
            intervals
                .iter()
                .find(|interval| interval.contains_tick(tick))
                .map_or(TickRegime::Awake, |interval| interval.regime)
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DurationInterval {
    start_tick: u64,
    terminal_tick: Option<u64>,
    regime: TickRegime,
}

impl DurationInterval {
    fn contains_tick(&self, tick: u64) -> bool {
        self.start_tick < tick && self.terminal_tick.is_none_or(|terminal| tick <= terminal)
    }
}

fn duration_intervals(
    log: &EventLog,
    actor_id: &ActorId,
    current_start: Option<&EventEnvelope>,
) -> Vec<DurationInterval> {
    let terminals = terminal_ticks_by_start(log);
    let mut starts = log
        .events()
        .iter()
        .filter(|event| event.actor_id.as_ref() == Some(actor_id))
        .filter_map(|event| duration_start_interval(event, &terminals))
        .collect::<Vec<_>>();
    if let Some(start) = current_start {
        if start.actor_id.as_ref() == Some(actor_id)
            && !log
                .events()
                .iter()
                .any(|event| event.event_id == start.event_id)
        {
            if let Some(interval) = duration_start_interval(start, &terminals) {
                starts.push(interval);
            }
        }
    }
    starts.sort_by(|left, right| {
        left.start_tick
            .cmp(&right.start_tick)
            .then_with(|| left.terminal_tick.cmp(&right.terminal_tick))
    });
    starts
}

fn terminal_ticks_by_start(log: &EventLog) -> BTreeMap<EventId, u64> {
    let mut terminals = BTreeMap::new();
    for event in log
        .events()
        .iter()
        .filter(|event| is_duration_terminal(event.event_type))
    {
        for cause in &event.causes {
            if let EventCause::Event(start_id) = cause {
                terminals
                    .entry(start_id.clone())
                    .and_modify(|tick: &mut u64| *tick = (*tick).min(event.sim_tick.value()))
                    .or_insert(event.sim_tick.value());
            }
        }
    }
    terminals
}

fn duration_start_interval(
    event: &EventEnvelope,
    terminals: &BTreeMap<EventId, u64>,
) -> Option<DurationInterval> {
    let regime = match event.event_type {
        EventKind::SleepStarted => TickRegime::Asleep,
        EventKind::WorkBlockStarted => TickRegime::Working,
        _ => return None,
    };
    Some(DurationInterval {
        start_tick: event.sim_tick.value(),
        terminal_tick: terminals.get(&event.event_id).copied(),
        regime,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{PayloadField, EVENT_SCHEMA_V1};
    use crate::ids::{ActionId, ContentManifestId, ProposalId};
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn ordering_key(tick: u64, action_id: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::new(tick),
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action_id).unwrap(),
            Vec::new(),
            action_id,
        )
    }

    fn start(kind: EventKind, id: &str, tick: u64) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(id).unwrap(),
            kind,
            0,
            0,
            SimTick::new(tick),
            ordering_key(tick, kind.stable_id()),
            ContentManifestId::new("phase3a_manifest").unwrap(),
            vec![EventCause::Proposal(
                ProposalId::new("proposal_duration").unwrap(),
            )],
        )
        .unwrap();
        event.actor_id = Some(actor_id());
        event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
        event
    }

    fn terminal(kind: EventKind, id: &str, start_id: &str, tick: u64) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(id).unwrap(),
            kind,
            0,
            0,
            SimTick::new(tick),
            ordering_key(tick, kind.stable_id()),
            ContentManifestId::new("phase3a_manifest").unwrap(),
            vec![EventCause::Event(EventId::new(start_id).unwrap())],
        )
        .unwrap();
        event.actor_id = Some(actor_id());
        event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
        event
    }

    #[test]
    fn classifies_sleep_across_window_boundary() {
        let mut log = EventLog::new();
        log.append(start(EventKind::SleepStarted, "event_sleep_started", 24))
            .unwrap();
        log.append(terminal(
            EventKind::SleepCompleted,
            "event_sleep_completed",
            "event_sleep_started",
            32,
        ))
        .unwrap();

        let counts =
            classify_actor_tick_regimes(&log, &actor_id(), SimTick::new(24), SimTick::new(28));

        assert_eq!(
            counts,
            TickRegimeCounts {
                awake_ticks: 0,
                asleep_ticks: 4,
                working_ticks: 0,
            }
        );
    }

    #[test]
    fn current_start_classifies_without_log_roundtrip() {
        let log = EventLog::new();
        let work_start = start(EventKind::WorkBlockStarted, "event_work_started", 10);

        let counts = classify_actor_tick_regimes_with_start(
            &log,
            &actor_id(),
            SimTick::new(10),
            SimTick::new(14),
            Some(&work_start),
        );

        assert_eq!(
            counts,
            TickRegimeCounts {
                awake_ticks: 0,
                asleep_ticks: 0,
                working_ticks: 4,
            }
        );
    }

    #[test]
    fn work_failed_is_terminal_for_later_ticks() {
        let mut log = EventLog::new();
        log.append(start(EventKind::WorkBlockStarted, "event_work_started", 10))
            .unwrap();
        log.append(terminal(
            EventKind::WorkBlockFailed,
            "event_work_failed",
            "event_work_started",
            12,
        ))
        .unwrap();

        let counts =
            classify_actor_tick_regimes(&log, &actor_id(), SimTick::new(10), SimTick::new(14));

        assert_eq!(
            counts,
            TickRegimeCounts {
                awake_ticks: 2,
                asleep_ticks: 0,
                working_ticks: 2,
            }
        );
    }
}
