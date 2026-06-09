use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::agent::{NeedBand, NeedKind};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId, PlaceId, SleepAffordanceId};
use crate::scheduler::OrderingKey;
use crate::state::{AgentState, PhysicalState};
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

    let sleep_affordance_id = require_sleep_affordance(state, proposal, &actor.current_place_id)?;

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
        PayloadField::new("sleep_affordance_id", sleep_affordance_id.as_str()),
    ];
    if let Some(sleep_place_id) = proposal.parameters.get("sleep_place_id") {
        event
            .payload
            .push(PayloadField::new("sleep_place_id", sleep_place_id));
    }
    event.effects_summary = "sleep started; completion is duration scheduled".to_string();
    Ok(event)
}

fn require_sleep_affordance(
    state: &PhysicalState,
    proposal: &Proposal,
    actor_place_id: &PlaceId,
) -> Result<SleepAffordanceId, ActionRejection> {
    let Some(raw_sleep_affordance_id) = proposal.parameters.get("sleep_affordance_id") else {
        return Err(no_sleep_affordance_rejection(
            "missing_sleep_affordance_id",
            actor_place_id,
            "",
        ));
    };
    let sleep_affordance_id =
        SleepAffordanceId::new(raw_sleep_affordance_id.clone()).map_err(|_| {
            no_sleep_affordance_rejection(
                "invalid_sleep_affordance_id",
                actor_place_id,
                raw_sleep_affordance_id,
            )
        })?;
    let Some(sleep_affordance) = state.sleep_affordances.get(&sleep_affordance_id) else {
        return Err(no_sleep_affordance_rejection(
            "unknown_sleep_affordance_id",
            actor_place_id,
            sleep_affordance_id.as_str(),
        ));
    };
    if sleep_affordance.place_id != *actor_place_id || !sleep_affordance.access_open {
        return Err(no_sleep_affordance_rejection(
            "sleep_affordance_not_open_at_actor_place",
            actor_place_id,
            sleep_affordance_id.as_str(),
        ));
    }
    if let Some(sleep_place_id) = proposal.parameters.get("sleep_place_id") {
        if sleep_place_id != sleep_affordance.place_id.as_str() {
            return Err(ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::ActorNotAtRequiredPlace,
                vec![
                    CheckedFact::new("actor_place_id", actor_place_id.as_str()),
                    CheckedFact::new("sleep_place_id", sleep_place_id),
                    CheckedFact::new("sleep_affordance_id", sleep_affordance_id.as_str()),
                ],
                "That sleep place is not reachable from here.",
                "sleep proposal place did not match the modeled sleep affordance",
            ));
        }
    }
    Ok(sleep_affordance_id)
}

fn no_sleep_affordance_rejection(
    reason: &str,
    actor_place_id: &PlaceId,
    sleep_affordance_id: &str,
) -> ActionRejection {
    ActionRejection::new(
        PipelineStage::PhysicalPreconditionValidation,
        ReasonCode::NoSleepAffordance,
        vec![
            CheckedFact::new("actor_place_id", actor_place_id.as_str()),
            CheckedFact::new("sleep_affordance_id", sleep_affordance_id),
            CheckedFact::new("reason", reason),
        ],
        "There is no usable place to sleep here.",
        format!("sleep requires an open modeled affordance at the actor place: {reason}"),
    )
}

pub fn build_sleep_completion_events(
    state: &PhysicalState,
    agent_state: &AgentState,
    sleep_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    completion_tick: SimTick,
) -> Vec<EventEnvelope> {
    if let Some(reason) = sleep_interruption_reason(state, agent_state, sleep_started_event) {
        return build_sleep_interruption_events(
            sleep_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            reason,
        );
    }
    build_sleep_end_events(
        sleep_started_event,
        ordering_key,
        content_manifest_id,
        completion_tick,
        EventKind::SleepCompleted,
        "sleep completed",
    )
}

fn sleep_interruption_reason(
    state: &PhysicalState,
    agent_state: &AgentState,
    sleep_started_event: &EventEnvelope,
) -> Option<&'static str> {
    let Some(actor_id) = sleep_started_event.actor_id.as_ref() else {
        return Some("actor_missing");
    };
    let Some(actor) = state.actors.get(actor_id) else {
        return Some("actor_missing");
    };
    let Some(sleep_affordance_id) = payload_value(sleep_started_event, "sleep_affordance_id")
        .and_then(|value| SleepAffordanceId::new(value).ok())
    else {
        return Some("sleep_affordance_missing");
    };
    let Some(sleep_affordance) = state.sleep_affordances.get(&sleep_affordance_id) else {
        return Some("sleep_affordance_missing");
    };
    if actor.current_place_id != sleep_affordance.place_id {
        return Some("actor_displaced");
    }
    if !sleep_affordance.access_open {
        return Some("sleep_affordance_closed");
    }
    if has_severe_interrupting_need(agent_state, actor_id) {
        return Some("severe_need_pressure");
    }
    None
}

fn has_severe_interrupting_need(agent_state: &AgentState, actor_id: &crate::ids::ActorId) -> bool {
    agent_state
        .needs_by_actor
        .get(actor_id)
        .is_some_and(|needs| {
            [NeedKind::Hunger, NeedKind::Safety].iter().any(|kind| {
                needs
                    .get(kind)
                    .is_some_and(|need| need.band() == NeedBand::Severe)
            })
        })
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
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
    use std::collections::BTreeMap;

    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::agent::{NeedChangeCause, NeedState};
    use crate::ids::{ActionId, ActorId, PlaceId, ProposalId, SleepAffordanceId};
    use crate::scheduler::{
        duration_completion_ordering_key, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use crate::state::{ActorBody, SleepAffordanceState};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), PlaceId::new("tomas_room").unwrap()),
        );
        state.sleep_affordances.insert(
            SleepAffordanceId::new("bed_tomas").unwrap(),
            SleepAffordanceState::new(
                SleepAffordanceId::new("bed_tomas").unwrap(),
                PlaceId::new("tomas_room").unwrap(),
            ),
        );
        state
    }

    fn agent_state_with_needs(fatigue: i32, hunger: i32) -> AgentState {
        let mut agent_state = AgentState::default();
        let mut needs = BTreeMap::new();
        needs.insert(
            NeedKind::Fatigue,
            NeedState::initial(NeedKind::Fatigue, fatigue, NeedChangeCause::FixtureInitial),
        );
        needs.insert(
            NeedKind::Hunger,
            NeedState::initial(NeedKind::Hunger, hunger, NeedChangeCause::FixtureInitial),
        );
        agent_state.needs_by_actor.insert(actor_id(), needs);
        agent_state
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
            .parameters
            .insert("sleep_affordance_id".to_string(), "bed_tomas".to_string());
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
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "sleep_affordance_id" && field.value == "bed_tomas"));
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
            &state(),
            &AgentState::default(),
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
    fn sleep_completion_interrupts_on_severe_need_with_prorated_recovery() {
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
            SimTick::new(12),
            0,
        );
        let events = build_sleep_completion_events(
            &state(),
            &agent_state_with_needs(100, 900),
            &start,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(12),
        );

        assert_eq!(events[0].event_type, EventKind::SleepInterrupted);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "severe_need_pressure"));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "fatigue")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "-40")
        }));
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

    #[test]
    fn sleep_rejects_current_place_without_affordance() {
        let mut state = state();
        state.sleep_affordances.clear();

        let rejection = build_sleep_start_event(
            &state,
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::NoSleepAffordance);
    }

    #[test]
    fn sleep_rejects_forged_or_stale_affordance() {
        let mut proposal = proposal(ProposalOrigin::Test);
        proposal.parameters.insert(
            "sleep_affordance_id".to_string(),
            "bed_elsewhere".to_string(),
        );

        let rejection = build_sleep_start_event(
            &state(),
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();
        assert_eq!(rejection.reason_code, ReasonCode::NoSleepAffordance);

        let mut stale_state = state();
        stale_state.sleep_affordances.insert(
            SleepAffordanceId::new("bed_elsewhere").unwrap(),
            SleepAffordanceState::new(
                SleepAffordanceId::new("bed_elsewhere").unwrap(),
                PlaceId::new("market").unwrap(),
            ),
        );
        let rejection = build_sleep_start_event(
            &stale_state,
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();
        assert_eq!(rejection.reason_code, ReasonCode::NoSleepAffordance);
    }
}
