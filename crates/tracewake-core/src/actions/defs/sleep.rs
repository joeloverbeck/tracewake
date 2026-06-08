use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId};
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;
use crate::time::SimTick;

pub const DEFAULT_SLEEP_DURATION_TICKS: u64 = 4;
pub const FATIGUE_RECOVERY_PER_SLEEP_TICK: i32 = 20;
pub const HUNGER_RISE_PER_SLEEP_TICK: i32 = 2;

pub fn build_sleep_start_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            Vec::new(),
            "That actor cannot sleep.",
            "sleep proposal did not include an actor",
        )
    })?;
    let actor = state.actors.get(&actor_id).ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            vec![CheckedFact::new("actor_id", actor_id.as_str())],
            "That actor cannot sleep.",
            "sleep actor was not present",
        )
    })?;

    if let Some(sleep_place_id) = proposal.parameters.get("sleep_place_id") {
        if sleep_place_id != actor.current_place_id.as_str() {
            return Err(ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::ActorNotAtRequiredPlace,
                vec![
                    CheckedFact::new("actor_place_id", actor.current_place_id.as_str()),
                    CheckedFact::new("sleep_place_id", sleep_place_id),
                ],
                "That sleep place is not reachable from here.",
                "sleep requires the actor to be at the modeled sleep place in this slice",
            ));
        }
    }

    let duration_ticks = parse_duration_ticks(proposal)?;
    let expected_completion_tick = proposal.requested_tick.advance_by(duration_ticks);
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.sleep_started.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::SleepStarted,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Proposal(proposal.proposal_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("duration_ticks", duration_ticks.to_string()),
        PayloadField::new(
            "expected_completion_tick",
            expected_completion_tick.value().to_string(),
        ),
        PayloadField::new("body_exclusive", "true"),
        PayloadField::new("fatigue_delta_at_start", "0"),
    ];
    if let Some(sleep_place_id) = proposal.parameters.get("sleep_place_id") {
        event
            .payload
            .push(PayloadField::new("sleep_place_id", sleep_place_id));
    }
    event.effects_summary = "sleep started; completion is duration scheduled".to_string();
    Ok(event)
}

pub fn build_sleep_completion_events(
    sleep_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    completion_tick: SimTick,
) -> Vec<EventEnvelope> {
    build_sleep_end_events(
        sleep_started_event,
        ordering_key,
        content_manifest_id,
        completion_tick,
        EventKind::SleepCompleted,
        "sleep completed",
    )
}

pub fn build_sleep_interruption_events(
    sleep_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    interruption_tick: SimTick,
    reason: &str,
) -> Vec<EventEnvelope> {
    build_sleep_end_events(
        sleep_started_event,
        ordering_key,
        content_manifest_id,
        interruption_tick,
        EventKind::SleepInterrupted,
        reason,
    )
}

fn build_sleep_end_events(
    sleep_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    end_tick: SimTick,
    kind: EventKind,
    reason: &str,
) -> Vec<EventEnvelope> {
    let actor_id = sleep_started_event
        .actor_id
        .clone()
        .expect("sleep start event carries actor");
    let elapsed_ticks = end_tick
        .value()
        .saturating_sub(sleep_started_event.sim_tick.value());
    let fatigue_delta = -(elapsed_ticks as i32).saturating_mul(FATIGUE_RECOVERY_PER_SLEEP_TICK);
    let hunger_delta = (elapsed_ticks as i32).saturating_mul(HUNGER_RISE_PER_SLEEP_TICK);
    let mut lifecycle = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.{}.{}",
            kind.stable_id(),
            sleep_started_event.event_id.as_str()
        ))
        .unwrap(),
        kind,
        0,
        0,
        end_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(sleep_started_event.event_id.clone())],
    )
    .unwrap();
    lifecycle.actor_id = Some(actor_id.clone());
    lifecycle.proposal_id = sleep_started_event.proposal_id.clone();
    lifecycle.participants = vec![actor_id.to_string()];
    lifecycle.payload = vec![
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("elapsed_ticks", elapsed_ticks.to_string()),
        PayloadField::new("reason", reason),
        PayloadField::new("fatigue_delta", fatigue_delta.to_string()),
        PayloadField::new("hunger_delta", hunger_delta.to_string()),
    ];
    lifecycle.effects_summary = format!("{} after {} ticks", kind.stable_id(), elapsed_ticks);

    let mut fatigue = need_delta_event(
        sleep_started_event,
        ordering_key,
        content_manifest_id,
        end_tick,
        "fatigue",
        fatigue_delta,
    );
    fatigue.effects_summary = format!(
        "fatigue recovered by {} over {} sleep ticks",
        fatigue_delta.abs(),
        elapsed_ticks
    );
    let hunger = need_delta_event(
        sleep_started_event,
        ordering_key,
        content_manifest_id,
        end_tick,
        "hunger",
        hunger_delta,
    );
    vec![lifecycle, fatigue, hunger]
}

fn need_delta_event(
    sleep_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    tick: SimTick,
    need_kind: &str,
    delta: i32,
) -> EventEnvelope {
    let actor_id = sleep_started_event
        .actor_id
        .clone()
        .expect("sleep start event carries actor");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.sleep_need_delta.{}.{}",
            need_kind,
            sleep_started_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::NeedDeltaApplied,
        0,
        0,
        tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(sleep_started_event.event_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("need_kind", need_kind),
        PayloadField::new("delta", delta.to_string()),
        PayloadField::new("cause_kind", "action_effect"),
        PayloadField::new("cause_action_id", "sleep"),
    ];
    event.effects_summary = format!("{need_kind} delta from elapsed sleep");
    event
}

fn parse_duration_ticks(proposal: &Proposal) -> Result<u64, ActionRejection> {
    let duration_ticks = proposal
        .parameters
        .get("duration_ticks")
        .map(|value| value.parse::<u64>())
        .transpose()
        .map_err(|_| {
            ActionRejection::new(
                PipelineStage::CostDurationCheck,
                ReasonCode::InvalidParameter,
                vec![CheckedFact::new(
                    "duration_ticks",
                    proposal
                        .parameters
                        .get("duration_ticks")
                        .cloned()
                        .unwrap_or_default(),
                )],
                "That sleep duration is invalid.",
                "sleep duration_ticks parameter was not an unsigned integer",
            )
        })?
        .unwrap_or(DEFAULT_SLEEP_DURATION_TICKS);
    if duration_ticks == 0 {
        return Err(ActionRejection::new(
            PipelineStage::CostDurationCheck,
            ReasonCode::InvalidParameter,
            vec![CheckedFact::new("duration_ticks", "0")],
            "That sleep duration is invalid.",
            "sleep duration must be greater than zero",
        ));
    }
    Ok(duration_ticks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::ids::{ActionId, ActorId, PlaceId, ProposalId};
    use crate::scheduler::{
        duration_completion_ordering_key, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use crate::state::ActorBody;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), PlaceId::new("tomas_room").unwrap()),
        );
        state
    }

    fn proposal(origin: ProposalOrigin) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_sleep").unwrap(),
            origin,
            Some(actor_id()),
            ActionId::new("sleep").unwrap(),
            SimTick::new(10),
        );
        proposal
            .parameters
            .insert("duration_ticks".to_string(), "3".to_string());
        proposal
            .parameters
            .insert("sleep_place_id".to_string(), "tomas_room".to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(10),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("sleep").unwrap(),
            Vec::new(),
            "sleep",
        )
    }

    #[test]
    fn sleep_start_is_duration_based_and_does_not_recover_fatigue() {
        let event = build_sleep_start_event(
            &state(),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::SleepStarted);
        assert_eq!(event.sim_tick, SimTick::new(10));
        assert!(event
            .payload
            .iter()
            .any(|field| { field.key == "expected_completion_tick" && field.value == "13" }));
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "fatigue_delta_at_start" && field.value == "0"));
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "body_exclusive" && field.value == "true"));
    }

    #[test]
    fn sleep_completion_recovers_fatigue_by_elapsed_duration() {
        let start = build_sleep_start_event(
            &state(),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();
        let completion_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("sleep").unwrap(),
            SimTick::new(13),
            0,
        );
        let events = build_sleep_completion_events(
            &start,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(13),
        );

        assert_eq!(events[0].event_type, EventKind::SleepCompleted);
        let fatigue = events
            .iter()
            .find(|event| {
                event.event_type == EventKind::NeedDeltaApplied
                    && event
                        .payload
                        .iter()
                        .any(|field| field.key == "need_kind" && field.value == "fatigue")
            })
            .unwrap();
        assert!(fatigue
            .payload
            .iter()
            .any(|field| field.key == "delta" && field.value == "-60"));
        assert!(!fatigue.effects_summary.contains("comfortable"));
    }

    #[test]
    fn sleep_interruption_records_partial_recovery() {
        let start = build_sleep_start_event(
            &state(),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();
        let interruption_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("sleep").unwrap(),
            SimTick::new(11),
            0,
        );
        let events = build_sleep_interruption_events(
            &start,
            &interruption_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(11),
            "interrupted by salient event",
        );

        assert_eq!(events[0].event_type, EventKind::SleepInterrupted);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "interrupted by salient event"));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "fatigue")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "-20")
        }));
    }

    #[test]
    fn sleep_rejects_unreachable_sleep_place() {
        let mut proposal = proposal(ProposalOrigin::Test);
        proposal
            .parameters
            .insert("sleep_place_id".to_string(), "market".to_string());

        assert!(build_sleep_start_event(
            &state(),
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .is_err());
    }
}
