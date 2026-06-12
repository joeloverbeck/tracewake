use crate::agent::{NeedBand, NeedKind, NeedState};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
use crate::ids::{ActorId, ContentManifestId, EventId, ProcessId, ProposalId};
use crate::scheduler::OrderingKey;
use crate::time::SimTick;

#[derive(Clone, Debug)]
pub(crate) struct NeedDeltaEventSpec {
    pub event_id: EventId,
    pub threshold_event_id: EventId,
    pub tick: SimTick,
    pub ordering_key: OrderingKey,
    pub content_manifest_id: ContentManifestId,
    pub causes: Vec<EventCause>,
    pub actor_id: ActorId,
    pub proposal_id: Option<ProposalId>,
    pub process_id: Option<ProcessId>,
    pub participants: Vec<String>,
    pub need_kind: NeedKind,
    pub delta: i32,
    pub elapsed_ticks: u64,
    pub cause_kind: String,
    pub cause_action_id: Option<String>,
    pub extra_payload: Vec<PayloadField>,
    pub delta_effects_summary: String,
    pub threshold_effects_summary: String,
}

pub(crate) fn build_need_delta_and_threshold_events(
    spec: NeedDeltaEventSpec,
    current_value: Option<u16>,
) -> Vec<EventEnvelope> {
    let next_value = current_value.map(|current| apply_need_delta(current, spec.delta));
    let threshold_event = current_value.zip(next_value).and_then(|(current, next)| {
        NeedState::threshold_crossing(current, next)
            .map(|crossing| build_threshold_event(&spec, current, next, crossing.from, crossing.to))
    });
    let mut events = vec![build_need_delta_event(&spec)];
    if let Some(threshold_event) = threshold_event {
        events.push(threshold_event);
    }
    events
}

pub(crate) fn apply_need_delta(current: u16, delta: i32) -> u16 {
    let next = i32::from(current).saturating_add(delta).clamp(0, 1000);
    next as u16
}

fn build_need_delta_event(spec: &NeedDeltaEventSpec) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        spec.event_id.clone(),
        EventKind::NeedDeltaApplied,
        0,
        0,
        spec.tick,
        spec.ordering_key.clone(),
        spec.content_manifest_id.clone(),
        spec.causes.clone(),
    )
    .unwrap();
    event.actor_id = Some(spec.actor_id.clone());
    event.proposal_id = spec.proposal_id.clone();
    event.process_id = spec.process_id.clone();
    event.participants = spec.participants.clone();
    event.payload = vec![
        PayloadField::new("actor_id", spec.actor_id.as_str()),
        PayloadField::new("need_kind", spec.need_kind.stable_id()),
        PayloadField::new("delta", spec.delta.to_string()),
        PayloadField::new("elapsed_ticks", spec.elapsed_ticks.to_string()),
        PayloadField::new("cause_kind", spec.cause_kind.as_str()),
    ];
    if let Some(cause_action_id) = &spec.cause_action_id {
        event.payload.push(PayloadField::new(
            "cause_action_id",
            cause_action_id.as_str(),
        ));
    }
    event.payload.extend(spec.extra_payload.clone());
    event.effects_summary = spec.delta_effects_summary.clone();
    event
}

fn build_threshold_event(
    spec: &NeedDeltaEventSpec,
    from_value: u16,
    to_value: u16,
    from_band: NeedBand,
    to_band: NeedBand,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        spec.threshold_event_id.clone(),
        EventKind::NeedThresholdCrossed,
        0,
        0,
        spec.tick,
        spec.ordering_key.clone(),
        spec.content_manifest_id.clone(),
        spec.causes
            .iter()
            .cloned()
            .chain([EventCause::Event(spec.event_id.clone())])
            .collect(),
    )
    .unwrap();
    event.actor_id = Some(spec.actor_id.clone());
    event.proposal_id = spec.proposal_id.clone();
    event.process_id = spec.process_id.clone();
    event.participants = spec.participants.clone();
    event.payload = vec![
        PayloadField::new("payload_schema_version", "1"),
        PayloadField::new("actor_id", spec.actor_id.as_str()),
        PayloadField::new("need_kind", spec.need_kind.stable_id()),
        PayloadField::new("from_value", from_value.to_string()),
        PayloadField::new("to_value", to_value.to_string()),
        PayloadField::new("from_band", from_band.stable_id()),
        PayloadField::new("to_band", to_band.stable_id()),
        PayloadField::new("candidate_goal_reevaluation", "true"),
    ];
    event.effects_summary = spec.threshold_effects_summary.clone();
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ActionId;
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};

    fn spec(delta: i32) -> NeedDeltaEventSpec {
        let actor_id = ActorId::new("actor_tomas").unwrap();
        NeedDeltaEventSpec {
            event_id: EventId::new("event.need_delta.test").unwrap(),
            threshold_event_id: EventId::new("event.need_threshold.test").unwrap(),
            tick: SimTick::new(4),
            ordering_key: OrderingKey::new(
                SimTick::new(4),
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Actor(actor_id.clone()),
                ProposalSequence::new(0),
                ActionId::new("need_delta").unwrap(),
                vec![actor_id.as_str().to_string()],
                "need_delta:test".to_string(),
            ),
            content_manifest_id: ContentManifestId::new("manifest_test").unwrap(),
            causes: vec![EventCause::Process(ProcessId::new("process_test").unwrap())],
            actor_id: actor_id.clone(),
            proposal_id: None,
            process_id: None,
            participants: vec![actor_id.as_str().to_string()],
            need_kind: NeedKind::Hunger,
            delta,
            elapsed_ticks: 1,
            cause_kind: "tick_delta".to_string(),
            cause_action_id: None,
            extra_payload: Vec::new(),
            delta_effects_summary: "delta".to_string(),
            threshold_effects_summary: "threshold".to_string(),
        }
    }

    #[test]
    fn shared_emitter_pairs_upward_and_downward_band_crossings() {
        let upward = build_need_delta_and_threshold_events(spec(1), Some(249));
        assert_eq!(upward.len(), 2);
        assert_eq!(upward[0].event_type, EventKind::NeedDeltaApplied);
        assert_eq!(upward[1].event_type, EventKind::NeedThresholdCrossed);

        let downward = build_need_delta_and_threshold_events(spec(-251), Some(750));
        assert_eq!(downward.len(), 2);
        assert_eq!(downward[0].event_type, EventKind::NeedDeltaApplied);
        assert_eq!(downward[1].event_type, EventKind::NeedThresholdCrossed);
    }
}
