use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId};
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_wait_event(
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
    ];
    event.effects_summary = "actor waited deterministic ticks".to_string();
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::events::apply::apply_event;
    use crate::ids::{ActionId, ActorId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, PhysicalState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), crate::ids::PlaceId::new("shop_front").unwrap()),
        );
        state
    }

    fn proposal() -> Proposal {
        Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("wait").unwrap(),
            SimTick::new(4),
        )
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
}
