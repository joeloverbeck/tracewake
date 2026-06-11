use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::{Proposal, ProposalOrigin};
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::agent::{NeedBand, NeedKind, NeedState};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId};
use crate::scheduler::OrderingKey;
use crate::state::{AgentState, PhysicalState};
use crate::time::passive_awake_need_deltas;

pub fn build_wait_event(
    state: &PhysicalState,
    agent_state: &AgentState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    Ok(build_wait_events(
        state,
        agent_state,
        proposal,
        ordering_key,
        content_manifest_id,
    )?
    .remove(0))
}

pub fn build_wait_events(
    state: &PhysicalState,
    agent_state: &AgentState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<Vec<EventEnvelope>, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            Vec::new(),
            "That actor cannot wait.",
            "wait proposal did not include an actor",
        )
    })?;
    if !state.actors.contains_key(&actor_id) {
        return Err(ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            vec![CheckedFact::new("actor_id", actor_id.as_str())],
            "That actor cannot wait.",
            "wait actor was not present",
        ));
    }

    let tick_count = proposal
        .parameters
        .get("ticks")
        .map(|value| value.parse::<u64>())
        .transpose()
        .map_err(|_| {
            ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::InvalidParameter,
                vec![CheckedFact::new(
                    "ticks",
                    proposal
                        .parameters
                        .get("ticks")
                        .cloned()
                        .unwrap_or_default(),
                )],
                "That wait duration is invalid.",
                "wait ticks parameter was not an unsigned integer",
            )
        })?
        .unwrap_or(1);
    if tick_count == 0 {
        return Err(ActionRejection::new(
            PipelineStage::PhysicalPreconditionValidation,
            ReasonCode::InvalidParameter,
            vec![CheckedFact::new("ticks", "0")],
            "That wait duration is invalid.",
            "wait ticks must be greater than zero",
        ));
    }

    let reason = proposal
        .parameters
        .get("reason")
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| {
            ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                if is_autonomous_wait(proposal) {
                    ReasonCode::MissingWaitReason
                } else {
                    ReasonCode::InvalidParameter
                },
                vec![CheckedFact::new("reason", "missing")],
                "That wait needs a reason.",
                "wait proposal did not include an actor-supplied reason",
            )
        })?;
    let deltas = passive_awake_need_deltas(state.need_model(), tick_count);
    let threshold_events = build_threshold_events(
        agent_state,
        proposal,
        ordering_key,
        content_manifest_id,
        &actor_id,
        tick_count,
        deltas,
    );
    let reevaluate = !threshold_events.is_empty();

    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.actor_waited.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::ActorWaited,
        0,
        0,
        proposal.requested_tick.advance_by(tick_count),
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("ticks", tick_count.to_string()),
        PayloadField::new("reason", reason.as_str()),
        PayloadField::new("candidate_goal_reevaluation", reevaluate.to_string()),
        PayloadField::new("body_exclusive", "false"),
        PayloadField::new("interruptible", "true"),
    ];
    event.effects_summary = format!("actor waited deterministic ticks: {reason}");

    let mut events = vec![event];
    events.push(build_need_delta_event(
        proposal,
        ordering_key,
        content_manifest_id,
        &actor_id,
        tick_count,
        NeedKind::Hunger,
        deltas.hunger_delta,
    ));
    events.push(build_need_delta_event(
        proposal,
        ordering_key,
        content_manifest_id,
        &actor_id,
        tick_count,
        NeedKind::Fatigue,
        deltas.fatigue_delta,
    ));
    events.extend(threshold_events);
    Ok(events)
}

fn is_autonomous_wait(proposal: &Proposal) -> bool {
    matches!(
        proposal.origin,
        ProposalOrigin::Scheduler | ProposalOrigin::Agent
    )
}

fn build_need_delta_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    actor_id: &crate::ids::ActorId,
    tick_count: u64,
    need_kind: NeedKind,
    delta: i32,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.need_delta.{}.{}.{}",
            need_kind.stable_id(),
            actor_id.as_str(),
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::NeedDeltaApplied,
        0,
        0,
        proposal.requested_tick.advance_by(tick_count),
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
        PayloadField::new("need_kind", need_kind.stable_id()),
        PayloadField::new("delta", delta.to_string()),
        PayloadField::new("elapsed_ticks", tick_count.to_string()),
        PayloadField::new("cause_kind", "tick_delta"),
    ];
    event.effects_summary = format!(
        "{} rose by {} over {} wait ticks",
        need_kind.stable_id(),
        delta,
        tick_count
    );
    event
}

fn build_threshold_events(
    agent_state: &AgentState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    actor_id: &crate::ids::ActorId,
    tick_count: u64,
    deltas: crate::time::PassiveNeedDeltas,
) -> Vec<EventEnvelope> {
    [
        (NeedKind::Hunger, deltas.hunger_delta),
        (NeedKind::Fatigue, deltas.fatigue_delta),
    ]
    .into_iter()
    .filter_map(|(need_kind, delta)| {
        let current = need_value(agent_state, actor_id, need_kind)?;
        let next = current.saturating_add(delta.max(0) as u16).min(1000);
        let crossing = NeedState::threshold_crossing(current, next)?;
        Some(build_threshold_event(
            proposal,
            ordering_key,
            content_manifest_id,
            actor_id,
            tick_count,
            need_kind,
            current,
            next,
            crossing.from,
            crossing.to,
        ))
    })
    .collect()
}

fn need_value(
    agent_state: &AgentState,
    actor_id: &crate::ids::ActorId,
    need_kind: NeedKind,
) -> Option<u16> {
    agent_state
        .needs_by_actor
        .get(actor_id)
        .and_then(|needs| needs.get(&need_kind))
        .map(NeedState::value)
}

#[allow(clippy::too_many_arguments)]
fn build_threshold_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    actor_id: &crate::ids::ActorId,
    tick_count: u64,
    need_kind: NeedKind,
    from_value: u16,
    to_value: u16,
    from_band: NeedBand,
    to_band: NeedBand,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.need_threshold.{}.{}.{}",
            need_kind.stable_id(),
            actor_id.as_str(),
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::NeedThresholdCrossed,
        0,
        0,
        proposal.requested_tick.advance_by(tick_count),
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Proposal(proposal.proposal_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("payload_schema_version", "1"),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("need_kind", need_kind.stable_id()),
        PayloadField::new("from_value", from_value.to_string()),
        PayloadField::new("to_value", to_value.to_string()),
        PayloadField::new("from_band", from_band.stable_id()),
        PayloadField::new("to_band", to_band.stable_id()),
        PayloadField::new("candidate_goal_reevaluation", "true"),
    ];
    event.effects_summary = format!(
        "{} crossed {} to {} during wait",
        need_kind.stable_id(),
        from_band.stable_id(),
        to_band.stable_id()
    );
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::events::apply::apply_event;
    use crate::ids::{ActionId, ActorId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, AgentState, PhysicalState};
    use crate::time::SimTick;
    use crate::{agent::NeedChangeCause, agent::NeedState};
    use std::collections::BTreeMap;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), crate::ids::PlaceId::new("shop_front").unwrap()),
        );
        state
    }

    fn agent_state() -> AgentState {
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id(),
            BTreeMap::from([
                (
                    NeedKind::Hunger,
                    NeedState::initial(NeedKind::Hunger, 249, NeedChangeCause::TickDelta),
                ),
                (
                    NeedKind::Fatigue,
                    NeedState::initial(NeedKind::Fatigue, 10, NeedChangeCause::TickDelta),
                ),
            ]),
        );
        state
    }

    fn proposal() -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("wait").unwrap(),
            SimTick::new(4),
        );
        proposal
            .parameters
            .insert("reason".to_string(), "waiting for morning".to_string());
        proposal
    }

    fn reasoned_threshold_proposal() -> Proposal {
        let mut proposal = proposal();
        proposal
            .parameters
            .insert("ticks".to_string(), "1".to_string());
        proposal
            .parameters
            .insert("current_hunger".to_string(), "0".to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(4),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("wait").unwrap(),
            vec!["1_tick".to_string()],
            "tie",
        )
    }

    #[test]
    fn wait_advances_tick_and_emits_event() {
        let event = build_wait_event(
            &state(),
            &agent_state(),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ActorWaited);
        assert_eq!(event.sim_tick, SimTick::new(5));
    }

    #[test]
    fn wait_event_replays_as_world_noop_with_tick_recorded() {
        let mut replay_state = state();
        let event = build_wait_event(
            &replay_state,
            &agent_state(),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();
        let before = replay_state.clone();

        apply_event(&mut replay_state, &event).unwrap();

        assert_eq!(replay_state, before);
        assert_eq!(event.sim_tick, SimTick::new(5));
    }

    #[test]
    fn wait_carries_reason_and_passive_need_delta_events() {
        let events = build_wait_events(
            &state(),
            &agent_state(),
            &reasoned_threshold_proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::ActorWaited);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "waiting for morning"));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "hunger")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "5")
        }));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "fatigue")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "3")
        }));
    }

    #[test]
    fn wait_threshold_crossing_sets_reevaluation_flag() {
        let events = build_wait_events(
            &state(),
            &agent_state(),
            &reasoned_threshold_proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        assert!(events[0]
            .payload
            .iter()
            .any(|field| { field.key == "candidate_goal_reevaluation" && field.value == "true" }));
        let threshold = events
            .iter()
            .find(|event| event.event_type == EventKind::NeedThresholdCrossed)
            .expect("threshold crossing event emitted");
        assert!(threshold
            .payload
            .iter()
            .any(|field| field.key == "from_value" && field.value == "249"));
        assert!(threshold
            .payload
            .iter()
            .any(|field| field.key == "from_band" && field.value == "comfortable"));
        assert!(threshold
            .payload
            .iter()
            .any(|field| field.key == "to_band" && field.value == "rising"));
    }

    #[test]
    fn wait_need_effects_do_not_reduce_needs_or_claim_body_exclusivity() {
        let events = build_wait_events(
            &state(),
            &agent_state(),
            &reasoned_threshold_proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        for event in events
            .iter()
            .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
        {
            let delta = event
                .payload
                .iter()
                .find(|field| field.key == "delta")
                .unwrap()
                .value
                .parse::<i32>()
                .unwrap();
            assert!(delta >= 0);
        }
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "body_exclusive" && field.value == "false"));
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "interruptible" && field.value == "true"));
    }

    #[test]
    fn scheduler_wait_without_actor_supplied_reason_is_rejected() {
        let mut proposal = proposal();
        proposal.origin = ProposalOrigin::Scheduler;
        proposal.parameters.remove("reason");

        let rejection = build_wait_events(
            &state(),
            &agent_state(),
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::MissingWaitReason);
    }
}
