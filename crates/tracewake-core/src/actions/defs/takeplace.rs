use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContainerId, ContentManifestId, EventId, ItemId, PlaceId};
use crate::location::Location;
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_take_place_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    take: bool,
) -> Result<EventEnvelope, ActionRejection> {
    if take {
        build_take_event(state, proposal, ordering_key, content_manifest_id)
    } else {
        build_place_event(state, proposal, ordering_key, content_manifest_id)
    }
}

fn build_take_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let item_id = item_target(proposal)?;
    let item = state
        .items
        .get(&item_id)
        .ok_or_else(|| target_missing("item_id", item_id.as_str()))?;
    if !item.portable {
        return Err(reject(
            PipelineStage::PhysicalPreconditionValidation,
            ReasonCode::ItemNotPortable,
            "item_id",
            item_id.as_str(),
            "That item cannot be carried.",
            "item portable flag was false",
        ));
    }
    if let Some(capacity) = actor.carry_capacity {
        if actor.carried_item_ids.len() as u32 >= capacity {
            return Err(reject(
                PipelineStage::PhysicalPreconditionValidation,
                ReasonCode::CarryCapacityExceeded,
                "actor_id",
                actor_id.as_str(),
                "The actor cannot carry more.",
                "actor carry capacity would be exceeded",
            ));
        }
    }

    match &item.location {
        Location::AtPlace(place_id) if place_id == &actor.current_place_id => {
            if let Some(source) = proposal.target_ids.get(1) {
                if source != place_id.as_str() {
                    return Err(item_not_at_source(&item_id));
                }
            }
            Ok(item_event(
                proposal,
                ordering_key,
                content_manifest_id,
                EventKind::ItemTakenFromPlace,
                vec![
                    PayloadField::new("item_id", item_id.as_str()),
                    PayloadField::new("actor_id", actor_id.as_str()),
                    PayloadField::new("place_id", place_id.as_str()),
                ],
            ))
        }
        Location::InContainer(container_id) => {
            if let Some(source) = proposal.target_ids.get(1) {
                if source != container_id.as_str() {
                    return Err(item_not_at_source(&item_id));
                }
            }
            let container = state
                .containers
                .get(container_id)
                .ok_or_else(|| target_missing("container_id", container_id.as_str()))?;
            if container.location != Location::AtPlace(actor.current_place_id.clone()) {
                return Err(not_reachable("container_id", container_id.as_str()));
            }
            if !container.is_open {
                return Err(reject(
                    PipelineStage::PhysicalPreconditionValidation,
                    ReasonCode::ContainerClosed,
                    "container_id",
                    container_id.as_str(),
                    "The container is closed.",
                    "take source container was closed",
                ));
            }
            Ok(item_event(
                proposal,
                ordering_key,
                content_manifest_id,
                EventKind::ItemRemovedFromContainer,
                vec![
                    PayloadField::new("item_id", item_id.as_str()),
                    PayloadField::new("actor_id", actor_id.as_str()),
                    PayloadField::new("container_id", container_id.as_str()),
                ],
            ))
        }
        _ => Err(item_not_at_source(&item_id)),
    }
}

fn build_place_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let item_id = item_target(proposal)?;
    let item = state
        .items
        .get(&item_id)
        .ok_or_else(|| target_missing("item_id", item_id.as_str()))?;
    if item.location != Location::CarriedBy(actor_id.clone()) {
        return Err(reject(
            PipelineStage::PhysicalPreconditionValidation,
            ReasonCode::ItemNotCarried,
            "item_id",
            item_id.as_str(),
            "The actor is not carrying that item.",
            "place item was not carried by the actor",
        ));
    }
    let destination = proposal.target_ids.get(1).ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is nowhere to place it.",
            "place proposal did not include a destination",
        )
    })?;

    if let Ok(container_id) = ContainerId::new(destination) {
        if let Some(container) = state.containers.get(&container_id) {
            if container.location != Location::AtPlace(actor.current_place_id.clone()) {
                return Err(not_reachable("container_id", container_id.as_str()));
            }
            if !container.is_open {
                return Err(reject(
                    PipelineStage::PhysicalPreconditionValidation,
                    ReasonCode::DestinationNotOpen,
                    "container_id",
                    container_id.as_str(),
                    "The destination is closed.",
                    "place destination container was closed",
                ));
            }
            return Ok(item_event(
                proposal,
                ordering_key,
                content_manifest_id,
                EventKind::ItemPlacedInContainer,
                vec![
                    PayloadField::new("item_id", item_id.as_str()),
                    PayloadField::new("actor_id", actor_id.as_str()),
                    PayloadField::new("container_id", container_id.as_str()),
                ],
            ));
        }
    }

    let place_id = PlaceId::new(destination).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            "destination_id",
            destination,
            "That destination is invalid.",
            "place destination was neither a place nor a container",
        )
    })?;
    if place_id != actor.current_place_id || !state.places.contains_key(&place_id) {
        return Err(not_reachable("place_id", place_id.as_str()));
    }
    Ok(item_event(
        proposal,
        ordering_key,
        content_manifest_id,
        EventKind::ItemPlacedInPlace,
        vec![
            PayloadField::new("item_id", item_id.as_str()),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("place_id", place_id.as_str()),
        ],
    ))
}

fn item_target(proposal: &Proposal) -> Result<ItemId, ActionRejection> {
    let target = proposal.target_ids.first().ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is no item.",
            "take/place proposal did not include an item target",
        )
    })?;
    ItemId::new(target).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            "item_id",
            target,
            "That item is invalid.",
            "item target was not a stable item ID",
        )
    })
}

fn item_event(
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

fn actor_missing() -> ActionRejection {
    ActionRejection::new(
        PipelineStage::ActorLookup,
        ReasonCode::ActorNotFound,
        Vec::new(),
        "That actor cannot act.",
        "actor was missing",
    )
}

fn target_missing(key: &'static str, value: &str) -> ActionRejection {
    reject(
        PipelineStage::TargetBinding,
        ReasonCode::TargetNotFound,
        key,
        value,
        "That target is not present.",
        "target was missing from physical state",
    )
}

fn item_not_at_source(item_id: &ItemId) -> ActionRejection {
    reject(
        PipelineStage::PhysicalPreconditionValidation,
        ReasonCode::ItemNotAtSource,
        "item_id",
        item_id.as_str(),
        "That item is not at the source.",
        "item holder did not match the requested source",
    )
}

fn not_reachable(key: &'static str, value: &str) -> ActionRejection {
    reject(
        PipelineStage::LocalityReachabilityValidation,
        ReasonCode::TargetNotReachable,
        key,
        value,
        "That target is not reachable.",
        "target was not reachable from actor place",
    )
}

fn reject(
    failed_stage: PipelineStage,
    reason_code: ReasonCode,
    key: &'static str,
    value: &str,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> ActionRejection {
    ActionRejection::new(
        failed_stage,
        reason_code,
        vec![CheckedFact::new(key, value)],
        actor_visible_summary,
        debug_summary,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::events::apply::apply_event;
    use crate::ids::{ActionId, ActorId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{
        ActorBody, ContainerState, ItemState, OwnershipCustody, PhysicalState, PlaceState,
    };
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id() -> PlaceId {
        PlaceId::new("shop_front").unwrap()
    }

    fn item_id() -> ItemId {
        ItemId::new("coin_stack_01").unwrap()
    }

    fn container_id() -> ContainerId {
        ContainerId::new("strongbox_tomas").unwrap()
    }

    fn ordering_key(action: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action).unwrap(),
            vec!["coin_stack_01".to_string()],
            "tie",
        )
    }

    fn proposal(action: &str, targets: &[&str]) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new(format!("proposal_{action}")).unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new(action).unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids = targets.iter().map(|target| target.to_string()).collect();
        proposal
    }

    fn state(container_open: bool, item_location: Location) -> PhysicalState {
        let mut state = PhysicalState::default();
        state
            .places
            .insert(place_id(), PlaceState::new(place_id(), "Shop front"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id()));
        let mut container = ContainerState::fixed_at_place(container_id(), place_id());
        container.is_open = container_open;
        if item_location == Location::InContainer(container_id()) {
            container.contents.insert(item_id());
        }
        state.containers.insert(container_id(), container);
        state
            .items
            .insert(item_id(), ItemState::new(item_id(), item_location));
        state
    }

    #[test]
    fn take_from_closed_container_rejects() {
        let rejection = build_take_place_event(
            &state(false, Location::InContainer(container_id())),
            &proposal("take", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("take"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::ContainerClosed);
    }

    #[test]
    fn take_from_open_container_moves_item_to_actor() {
        let mut state = state(true, Location::InContainer(container_id()));
        let event = build_take_place_event(
            &state,
            &proposal("take", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("take"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap();

        apply_event(&mut state, &event).unwrap();
        assert_eq!(
            state.items[&item_id()].location,
            Location::CarriedBy(actor_id())
        );
        assert!(!state.containers[&container_id()]
            .contents
            .contains(&item_id()));
    }

    #[test]
    fn take_from_wrong_source_rejects() {
        let rejection = build_take_place_event(
            &state(true, Location::AtPlace(place_id())),
            &proposal("take", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("take"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::ItemNotAtSource);
    }

    #[test]
    fn place_into_closed_container_rejects() {
        let rejection = build_take_place_event(
            &state(false, Location::CarriedBy(actor_id())),
            &proposal("place", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("place"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            false,
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::DestinationNotOpen);
    }

    #[test]
    fn place_into_open_container_emits_and_applies_event() {
        let mut state = state(true, Location::CarriedBy(actor_id()));
        state
            .actors
            .get_mut(&actor_id())
            .unwrap()
            .carried_item_ids
            .insert(item_id());
        let event = build_take_place_event(
            &state,
            &proposal("place", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("place"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            false,
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ItemPlacedInContainer);
        apply_event(&mut state, &event).unwrap();
        assert_eq!(
            state.items[&item_id()].location,
            Location::InContainer(container_id())
        );
        assert!(state.containers[&container_id()]
            .contents
            .contains(&item_id()));
    }

    #[test]
    fn take_does_not_change_ownership_custody() {
        let custody = OwnershipCustody::default();
        let before = custody.clone();
        let mut state = state(true, Location::InContainer(container_id()));
        let event = build_take_place_event(
            &state,
            &proposal("take", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("take"),
            &ContentManifestId::new("phase1_manifest").unwrap(),
            true,
        )
        .unwrap();

        apply_event(&mut state, &event).unwrap();
        assert_eq!(custody, before);
    }
}
