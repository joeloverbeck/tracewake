use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId, PlaceId};
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_move_event(
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
            "That actor cannot act.",
            "move proposal did not include an actor",
        )
    })?;
    let actor = state.actors.get(&actor_id).ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            vec![CheckedFact::new("actor_id", actor_id.as_str())],
            "That actor cannot act.",
            "move actor was not present",
        )
    })?;
    let destination = proposal.target_ids.first().ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            vec![CheckedFact::new(
                "target_count",
                proposal.target_ids.len().to_string(),
            )],
            "There is nowhere to move.",
            "move proposal did not include a destination target",
        )
    })?;
    let to_place_id = PlaceId::new(destination).map_err(|_| {
        ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            vec![CheckedFact::new("target_id", destination)],
            "That destination is invalid.",
            "move destination was not a stable place ID",
        )
    })?;
    if !state.places.contains_key(&to_place_id) {
        return Err(ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            vec![CheckedFact::new("place_id", to_place_id.as_str())],
            "That place is not reachable.",
            "move destination place was not present",
        ));
    }

    let from_place_id = actor.current_place_id.clone();
    let from_place = state.places.get(&from_place_id).ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotAtRequiredPlace,
            vec![CheckedFact::new("from_place_id", from_place_id.as_str())],
            "The actor is not at a valid place.",
            "actor current place was missing",
        )
    })?;

    let connecting_door = state.doors.values().find(|door| {
        (door.endpoint_a == from_place_id && door.endpoint_b == to_place_id)
            || (door.endpoint_b == from_place_id && door.endpoint_a == to_place_id)
    });
    let adjacent =
        from_place.adjacent_place_ids.contains(&to_place_id) || connecting_door.is_some();
    if !adjacent {
        return Err(ActionRejection::new(
            PipelineStage::LocalityReachabilityValidation,
            ReasonCode::NotAdjacent,
            vec![
                CheckedFact::new("from_place_id", from_place_id.as_str()),
                CheckedFact::new("to_place_id", to_place_id.as_str()),
            ],
            "That place is not adjacent.",
            "no adjacency edge or connecting door was present",
        ));
    }

    if let Some(door) = connecting_door {
        if !door.is_open && door.blocks_movement_when_closed {
            return Err(ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::DoorClosedBlocksMovement,
                vec![
                    CheckedFact::new("door_id", door.door_id.as_str()),
                    CheckedFact::new("door_open", door.is_open.to_string()),
                ],
                "The door is closed.",
                "closed blocking door prevented movement",
            ));
        }
        if door.is_locked {
            return Err(ActionRejection::new(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::DoorLocked,
                vec![CheckedFact::new("door_id", door.door_id.as_str())],
                "The door is locked.",
                "locked door prevented movement",
            ));
        }
    }

    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.actor_moved.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::ActorMoved,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = Some(actor_id.clone());
    event.place_id = Some(from_place_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![
        actor_id.to_string(),
        from_place_id.to_string(),
        to_place_id.to_string(),
    ];
    event.payload = vec![
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("from_place_id", from_place_id.as_str()),
        PayloadField::new("to_place_id", to_place_id.as_str()),
    ];
    event.effects_summary = "actor moved between adjacent places".to_string();
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::ids::{ActionId, ActorId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, DoorState, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn ids() -> (ActorId, PlaceId, PlaceId) {
        (
            ActorId::new("actor_tomas").unwrap(),
            PlaceId::new("shop_front").unwrap(),
            PlaceId::new("back_room").unwrap(),
        )
    }

    fn proposal() -> Proposal {
        let (actor_id, _, to_place) = ids();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_move").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id),
            ActionId::new("move").unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids.push(to_place.to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        let (actor_id, _, _) = ids();
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id),
            ProposalSequence::new(0),
            ActionId::new("move").unwrap(),
            vec!["back_room".to_string()],
            "tie",
        )
    }

    fn state(with_adjacency: bool, door_open: bool) -> PhysicalState {
        let (actor_id, from, to) = ids();
        let mut state = PhysicalState::default();
        let mut from_place = PlaceState::new(from.clone(), "Shop front");
        if with_adjacency {
            from_place.adjacent_place_ids.insert(to.clone());
        }
        state.places.insert(from.clone(), from_place);
        state
            .places
            .insert(to.clone(), PlaceState::new(to.clone(), "Back room"));
        state
            .actors
            .insert(actor_id.clone(), ActorBody::new(actor_id, from.clone()));
        if with_adjacency {
            let mut door =
                DoorState::new(crate::ids::DoorId::new("door_shop_back").unwrap(), from, to);
            door.is_open = door_open;
            state.doors.insert(door.door_id.clone(), door);
        }
        state
    }

    #[test]
    fn move_rejects_non_adjacent_destination() {
        let rejection = build_move_event(
            &state(false, false),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::NotAdjacent);
    }

    #[test]
    fn move_rejects_closed_blocking_door() {
        let rejection = build_move_event(
            &state(true, false),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::DoorClosedBlocksMovement);
    }

    #[test]
    fn move_accepts_valid_adjacent_destination() {
        let event = build_move_event(
            &state(true, true),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ActorMoved);
    }
}
