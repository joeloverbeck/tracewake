use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::epistemics::Proposition;
use crate::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ActorId, ContainerId, ContentManifestId, EventId, ItemId, PlaceId};
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

pub fn build_sound_observation_event(
    state: &PhysicalState,
    source_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Option<EventEnvelope> {
    if !matches!(
        source_event.event_type,
        EventKind::ItemTakenFromPlace
            | EventKind::ItemRemovedFromContainer
            | EventKind::ItemPlacedInPlace
            | EventKind::ItemPlacedInContainer
    ) {
        return None;
    }
    let source_actor_id = source_event.actor_id.as_ref()?;
    if source_actor_id.as_str() != "actor_mara" {
        return None;
    }
    let source_actor = state.actors.get(source_actor_id)?;
    let observer_actor_id = ActorId::new("actor_elena").unwrap();
    let observer = state.actors.get(&observer_actor_id)?;
    if !places_within_sound_range(
        state,
        &source_actor.current_place_id,
        &observer.current_place_id,
    ) {
        return None;
    }

    let observation_id = format!("obs.sound.{}", source_event.event_id.as_str());
    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.observation.sound.{}",
            source_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::ObservationRecorded,
        0,
        0,
        source_event.sim_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = Some(observer_actor_id.clone());
    event.proposal_id = source_event.proposal_id.clone();
    event.participants = vec![
        observer_actor_id.to_string(),
        source_actor_id.to_string(),
        source_actor.current_place_id.to_string(),
    ];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("observation_id", observation_id),
        PayloadField::new("observer_actor_id", observer_actor_id.as_str()),
        PayloadField::new("observer_place_id", observer.current_place_id.as_str()),
        PayloadField::new("place_id", source_actor.current_place_id.as_str()),
        PayloadField::new("channel", "simple_sound"),
        PayloadField::new("observed_tick", source_event.sim_tick.value().to_string()),
        PayloadField::new("confidence", "0250"),
        PayloadField::new("source_event_id", source_event.event_id.as_str()),
        PayloadField::new(
            "alternatives",
            "house_settling,person_moving,object_shift,misheard_sound",
        ),
        PayloadField::new("intensity", "low"),
        PayloadField::new("duration", "brief"),
        PayloadField::new("material_hint", "wood_or_metal"),
    ];
    event.effects_summary = "low-confidence sound observation recorded".to_string();
    Some(event)
}

pub fn build_sound_belief_event(
    observation_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Option<EventEnvelope> {
    let observer_actor_id = payload_value(observation_event, "observer_actor_id")?;
    let place_id = PlaceId::new(payload_value(observation_event, "place_id")?).ok()?;
    let observation_id = payload_value(observation_event, "observation_id")?;
    let proposition = Proposition::SoundHeardNearPlace { place_id };
    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.belief.sound.{}",
            observation_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::BeliefUpdated,
        0,
        0,
        observation_event.sim_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = observation_event.actor_id.clone();
    event.proposal_id = observation_event.proposal_id.clone();
    event.participants = observation_event.participants.clone();
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("belief_id", format!("belief.sound.{observation_id}")),
        PayloadField::new("holder_actor_id", observer_actor_id),
        PayloadField::new("proposition", proposition.serialize_canonical()),
        PayloadField::new("stance", "plausible"),
        PayloadField::new("confidence", "0250"),
        PayloadField::new("source_event_id", observation_event.event_id.as_str()),
        PayloadField::new(
            "acquired_tick",
            observation_event.sim_tick.value().to_string(),
        ),
        PayloadField::new("channel", "simple_sound"),
    ];
    event.effects_summary = "low-confidence sound belief recorded".to_string();
    Some(event)
}

fn places_within_sound_range(state: &PhysicalState, source: &PlaceId, observer: &PlaceId) -> bool {
    if source == observer {
        return true;
    }
    state
        .places
        .get(source)
        .is_some_and(|place| place.adjacent_place_ids.contains(observer))
        || state
            .places
            .get(observer)
            .is_some_and(|place| place.adjacent_place_ids.contains(source))
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
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
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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

    fn sound_state() -> PhysicalState {
        let mut state = state(true, Location::InContainer(container_id()));
        let listener_place = PlaceId::new("street_lane").unwrap();
        state.places.insert(
            listener_place.clone(),
            PlaceState::new(listener_place.clone(), "Street"),
        );
        state
            .places
            .get_mut(&place_id())
            .unwrap()
            .adjacent_place_ids
            .insert(listener_place.clone());
        state
            .places
            .get_mut(&listener_place)
            .unwrap()
            .adjacent_place_ids
            .insert(place_id());
        let mara = ActorId::new("actor_mara").unwrap();
        let elena = ActorId::new("actor_elena").unwrap();
        state
            .actors
            .insert(mara.clone(), ActorBody::new(mara, place_id()));
        state
            .actors
            .insert(elena.clone(), ActorBody::new(elena, listener_place));
        state
    }

    fn mara_proposal(action: &str, targets: &[&str]) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new(format!("proposal_mara_{action}")).unwrap(),
            ProposalOrigin::Test,
            Some(ActorId::new("actor_mara").unwrap()),
            ActionId::new(action).unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids = targets.iter().map(|target| target.to_string()).collect();
        proposal
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

    #[test]
    fn mara_take_near_elena_builds_low_confidence_sound_without_culprit_belief() {
        let state = sound_state();
        let physical = build_take_place_event(
            &state,
            &mara_proposal("take", &["coin_stack_01", "strongbox_tomas"]),
            &ordering_key("take"),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
            true,
        )
        .unwrap();

        let observation = build_sound_observation_event(
            &state,
            &physical,
            &ordering_key("take"),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .expect("Elena hears nearby Mara action");
        let belief = build_sound_belief_event(
            &observation,
            &ordering_key("take"),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .expect("sound observation supports only uncertain sound belief");

        assert_eq!(observation.event_type, EventKind::ObservationRecorded);
        assert!(observation
            .payload
            .iter()
            .any(|field| field.key == "channel" && field.value == "simple_sound"));
        assert!(observation
            .payload
            .iter()
            .any(|field| field.key == "confidence" && field.value == "0250"));
        let alternatives = observation
            .payload
            .iter()
            .find(|field| field.key == "alternatives")
            .unwrap()
            .value
            .split(',')
            .count();
        assert!(alternatives >= 2);

        let proposition = belief
            .payload
            .iter()
            .find(|field| field.key == "proposition")
            .unwrap()
            .value
            .clone();
        assert!(proposition.starts_with("sound_heard_near_place|"));
        assert!(!proposition.contains("actor_mara"));
        assert!(!format!("{belief:?}").contains("stole"));
        assert!(!format!("{belief:?}").contains("culprit"));
    }
}
