use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContainerId, ContentManifestId, DoorId, EventId};
use crate::location::Location;
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_open_close_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    open: bool,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            Vec::new(),
            "That actor cannot act.",
            "open/close proposal did not include an actor",
        )
    })?;
    let actor = state.actors.get(&actor_id).ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::ActorLookup,
            ReasonCode::ActorNotFound,
            vec![CheckedFact::new("actor_id", actor_id.as_str())],
            "That actor cannot act.",
            "open/close actor was not present",
        )
    })?;
    let target = proposal.target_ids.first().ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            Vec::new(),
            "There is no target.",
            "open/close proposal did not include a target",
        )
    })?;

    if let Ok(door_id) = DoorId::new(target) {
        if let Some(door) = state.doors.get(&door_id) {
            if !door.connects_place(&actor.current_place_id) {
                return Err(not_reachable("door_id", door_id.as_str()));
            }
            if door.is_locked {
                return Err(locked("door_id", door_id.as_str(), ReasonCode::DoorLocked));
            }
            if door.is_open == open {
                return Err(already(open, "door_id", door_id.as_str()));
            }
            return Ok(event(
                proposal,
                ordering_key,
                content_manifest_id,
                if open {
                    EventKind::DoorOpened
                } else {
                    EventKind::DoorClosed
                },
                vec![
                    PayloadField::new("door_id", door_id.as_str()),
                    PayloadField::new("was_open", (!open).to_string()),
                    PayloadField::new("now_open", open.to_string()),
                ],
            ));
        }
    }

    let container_id = ContainerId::new(target).map_err(|_| {
        ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            vec![CheckedFact::new("target_id", target)],
            "That target cannot be opened or closed.",
            "target was neither a door nor a container ID",
        )
    })?;
    let container = state.containers.get(&container_id).ok_or_else(|| {
        ActionRejection::new(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            vec![CheckedFact::new("container_id", container_id.as_str())],
            "That container is not here.",
            "container target was not present",
        )
    })?;
    if container.location != Location::AtPlace(actor.current_place_id.clone()) {
        return Err(not_reachable("container_id", container_id.as_str()));
    }
    if container.is_locked {
        return Err(locked(
            "container_id",
            container_id.as_str(),
            ReasonCode::ContainerLocked,
        ));
    }
    if container.is_open == open {
        return Err(already(open, "container_id", container_id.as_str()));
    }

    Ok(event(
        proposal,
        ordering_key,
        content_manifest_id,
        if open {
            EventKind::ContainerOpened
        } else {
            EventKind::ContainerClosed
        },
        vec![
            PayloadField::new("container_id", container_id.as_str()),
            PayloadField::new("was_open", (!open).to_string()),
            PayloadField::new("now_open", open.to_string()),
        ],
    ))
}

fn event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    kind: EventKind,
    payload: Vec<PayloadField>,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.{}.{}",
            kind.stable_id(),
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        kind,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = proposal.actor_id.clone();
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.payload = payload;
    event.effects_summary = format!("{} applied", kind.stable_id());
    event
}

fn not_reachable(key: &'static str, value: &str) -> ActionRejection {
    ActionRejection::new(
        PipelineStage::LocalityReachabilityValidation,
        ReasonCode::TargetNotReachable,
        vec![CheckedFact::new(key, value)],
        "That target is not reachable.",
        "target was not physically reachable from actor place",
    )
}

fn locked(key: &'static str, value: &str, reason: ReasonCode) -> ActionRejection {
    ActionRejection::new(
        PipelineStage::PhysicalPreconditionValidation,
        reason,
        vec![CheckedFact::new(key, value)],
        "It is locked.",
        "locked target rejected open/close",
    )
}

fn already(open: bool, key: &'static str, value: &str) -> ActionRejection {
    ActionRejection::new(
        PipelineStage::PhysicalPreconditionValidation,
        if open {
            ReasonCode::AlreadyOpen
        } else {
            ReasonCode::AlreadyClosed
        },
        vec![CheckedFact::new(key, value)],
        if open {
            "It is already open."
        } else {
            "It is already closed."
        },
        "target was already in requested open/closed state",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::ids::{ActionId, ActorId, PlaceId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ContainerState, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id() -> PlaceId {
        PlaceId::new("shop_front").unwrap()
    }

    fn ordering_key(action: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action).unwrap(),
            vec!["strongbox_tomas".to_string()],
            "tie",
        )
    }

    fn proposal(action: &str) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new(format!("proposal_{action}")).unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new(action).unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids.push("strongbox_tomas".to_string());
        proposal
    }

    fn state(open: bool, locked: bool) -> PhysicalState {
        let mut state = PhysicalState::default();
        state
            .places
            .insert(place_id(), PlaceState::new(place_id(), "Shop front"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id()));
        let mut container = ContainerState::fixed_at_place(
            ContainerId::new("strongbox_tomas").unwrap(),
            place_id(),
        );
        container.is_open = open;
        container.is_locked = locked;
        state
            .containers
            .insert(container.container_id.clone(), container);
        state
    }

    #[test]
    fn open_closed_reachable_container_emits_event() {
        let event = build_open_close_event(
            &state(false, false),
            &proposal("open"),
            &ordering_key("open"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ContainerOpened);
    }

    #[test]
    fn open_rejects_already_open_container() {
        let rejection = build_open_close_event(
            &state(true, false),
            &proposal("open"),
            &ordering_key("open"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::AlreadyOpen);
    }

    #[test]
    fn open_rejects_locked_container() {
        let rejection = build_open_close_event(
            &state(false, true),
            &proposal("open"),
            &ordering_key("open"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::ContainerLocked);
    }
}
