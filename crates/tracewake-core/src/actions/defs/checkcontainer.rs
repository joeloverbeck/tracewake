use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ContainerId, ContentManifestId, EventId};
use crate::location::Location;
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_check_container_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let container_id = container_target(proposal)?;
    let container = state
        .containers
        .get(&container_id)
        .ok_or_else(|| target_missing("container_id", container_id.as_str()))?;

    if container.location != Location::AtPlace(actor.current_place_id.clone()) {
        return Err(reject(
            PipelineStage::LocalityReachabilityValidation,
            ReasonCode::TargetNotReachable,
            "container_id",
            container_id.as_str(),
            "That container is not reachable.",
            "check target container was not at actor place",
        ));
    }
    if container.is_locked {
        return Err(reject(
            PipelineStage::PhysicalPreconditionValidation,
            ReasonCode::ContainerLocked,
            "container_id",
            container_id.as_str(),
            "The container is locked.",
            "check target container was locked",
        ));
    }
    if !container.is_open && !container.contents_visible_when_closed {
        return Err(reject(
            PipelineStage::PhysicalPreconditionValidation,
            ReasonCode::ContainerClosed,
            "container_id",
            container_id.as_str(),
            "The container is closed.",
            "check target container was closed and opaque",
        ));
    }

    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.container_checked.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::ContainerChecked,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = Some(actor_id.clone());
    event.place_id = Some(actor.current_place_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("container_id", container_id.as_str()),
        PayloadField::new("source_action_id", proposal.action_id.as_str()),
        PayloadField::new("observed_tick", proposal.requested_tick.value().to_string()),
    ];
    event.effects_summary = "container intentionally checked".to_string();
    Ok(event)
}

pub fn build_observation_recorded_event(
    check_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> EventEnvelope {
    let actor_id = check_event
        .actor_id
        .as_ref()
        .expect("check event carries actor_id");
    let place_id = check_event
        .place_id
        .as_ref()
        .expect("check event carries place_id");
    let container_id = payload_value(check_event, "container_id")
        .expect("check event carries container_id payload");
    let observed_tick = payload_value(check_event, "observed_tick")
        .expect("check event carries observed_tick payload");

    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.observation_recorded.{}",
            check_event
                .proposal_id
                .as_ref()
                .expect("check event carries proposal_id")
                .as_str()
        ))
        .unwrap(),
        EventKind::ObservationRecorded,
        0,
        0,
        check_event.sim_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
    );
    event.actor_id = Some(actor_id.clone());
    event.place_id = Some(place_id.clone());
    event.proposal_id = check_event.proposal_id.clone();
    event.causes = vec![crate::events::EventCause::Event(
        check_event.event_id.clone(),
    )];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new(
            "observation_id",
            format!(
                "obs.container_contents.{}",
                check_event
                    .proposal_id
                    .as_ref()
                    .expect("check event carries proposal_id")
                    .as_str()
            ),
        ),
        PayloadField::new("observer_actor_id", actor_id.as_str()),
        PayloadField::new("channel", "touch_or_search"),
        PayloadField::new("observed_tick", observed_tick),
        PayloadField::new("observer_place_id", place_id.as_str()),
        PayloadField::new("container_id", container_id),
        PayloadField::new("confidence", "1000"),
        PayloadField::new("source_event_id", check_event.event_id.as_str()),
    ];
    event.effects_summary = "container contents observation recorded".to_string();
    event
}

fn container_target(proposal: &Proposal) -> Result<ContainerId, ActionRejection> {
    let target = proposal.target_ids.first().ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is no container to check.",
            "check_container proposal did not include a container target",
        )
    })?;
    ContainerId::new(target).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            "container_id",
            target,
            "That container is invalid.",
            "check target was not a stable container ID",
        )
    })
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
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
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::ids::{ActionId, ActorId, PlaceId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ContainerState, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn container_id() -> ContainerId {
        ContainerId::new("strongbox_tomas").unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn state(open: bool, locked: bool) -> PhysicalState {
        let mut state = PhysicalState::default();
        state.places.insert(
            place_id("shop_front"),
            PlaceState::new(place_id("shop_front"), "Shop front"),
        );
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), place_id("shop_front")),
        );
        let mut container = ContainerState::fixed_at_place(container_id(), place_id("shop_front"));
        container.is_open = open;
        container.is_locked = locked;
        state.containers.insert(container_id(), container);
        state
    }

    fn proposal(origin: ProposalOrigin) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_check").unwrap(),
            origin,
            Some(actor_id()),
            ActionId::new("check_container").unwrap(),
            SimTick::new(3),
        );
        proposal.target_ids.push("strongbox_tomas".to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(3),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("check_container").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "tie",
        )
    }

    #[test]
    fn open_container_check_emits_container_checked_event() {
        let event = build_check_container_event(
            &state(true, false),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ContainerChecked);
        assert_eq!(event.actor_id, Some(actor_id()));
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "container_id" && field.value == "strongbox_tomas"));
    }

    #[test]
    fn closed_opaque_container_check_is_rejected() {
        let rejection = build_check_container_event(
            &state(false, false),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::ContainerClosed);
    }

    #[test]
    fn locked_container_check_is_rejected() {
        let rejection = build_check_container_event(
            &state(true, true),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::ContainerLocked);
    }

    #[test]
    fn observation_event_is_caused_by_check_event() {
        let check = build_check_container_event(
            &state(true, false),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        )
        .unwrap();
        let observation = build_observation_recorded_event(
            &check,
            &ordering_key(),
            &ContentManifestId::new("phase2a_manifest").unwrap(),
        );

        assert_eq!(observation.event_type, EventKind::ObservationRecorded);
        assert_eq!(
            observation.causes,
            vec![crate::events::EventCause::Event(check.event_id.clone())]
        );
    }
}
