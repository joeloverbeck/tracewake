use std::collections::BTreeMap;

use tracewake_core::agent::{current_place_knowledge_context, SourceEventIds};
use tracewake_core::epistemics::{
    ActorKnownIntervalDeltaError, ActorKnownWorkplaceFact, AllowedKnowledgeSource,
    EpistemicProjection, KnowledgeContext, KnowledgeProvenanceEntry, KnowledgeProvenanceKind,
};
use tracewake_core::events::apply::{apply_epistemic_event, ApplyOutcome};
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, EventId, PlaceId, ProcessId, ProposalId,
    WorkplaceId,
};
use tracewake_core::projections::{IntervalNoticeKind, IntervalStopReason};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{ActorBody, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

fn actor_id(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn place_id() -> PlaceId {
    PlaceId::new("workshop_floor").unwrap()
}

fn workplace_id() -> WorkplaceId {
    WorkplaceId::new("workplace_loom").unwrap()
}

fn container_id(value: &str) -> ContainerId {
    ContainerId::new(value).unwrap()
}

fn manifest_id() -> ContentManifestId {
    ContentManifestId::new("test_manifest").unwrap()
}

fn process_id() -> ProcessId {
    ProcessId::new("process.interval_projection_test").unwrap()
}

fn ordering_key(tick: SimTick) -> OrderingKey {
    OrderingKey::new(
        tick,
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Process(process_id()),
        ProposalSequence::new(0),
        ActionId::new("role_assignment_notice").unwrap(),
        vec![workplace_id().as_str().to_string()],
        "interval_projection_test",
    )
}

fn role_notice_event(event_id: &str, actor_id: &ActorId, tick: SimTick) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        EventKind::RoleAssignmentNoticeRecorded,
        99,
        99,
        tick,
        ordering_key(tick),
        manifest_id(),
        vec![EventCause::Process(process_id())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(ProposalId::new("proposal_role_notice").unwrap());
    event.participants = vec![
        actor_id.as_str().to_string(),
        workplace_id().as_str().to_string(),
    ];
    event.place_id = Some(place_id());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", workplace_id().as_str()),
        PayloadField::new("place_id", place_id().as_str()),
        PayloadField::new("access_open", "true"),
    ];
    event
}

fn visible_container_event(
    event_id: &str,
    observation_id: &str,
    source_event_id: &str,
    actor_id: &ActorId,
    container_id: &str,
    is_open: bool,
) -> EventEnvelope {
    let tick = SimTick::new(4);
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        EventKind::ObservationRecorded,
        99,
        99,
        tick,
        ordering_key(tick),
        manifest_id(),
        vec![EventCause::Process(process_id())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.participants = vec![actor_id.as_str().to_string(), container_id.to_string()];
    event.place_id = Some(place_id());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("observation_id", observation_id),
        PayloadField::new("observer_actor_id", actor_id.as_str()),
        PayloadField::new("channel", "direct_sight"),
        PayloadField::new("observed_tick", tick.value().to_string()),
        PayloadField::new("observer_place_id", place_id().as_str()),
        PayloadField::new("place_id", place_id().as_str()),
        PayloadField::new("confidence", "900"),
        PayloadField::new("source_event_id", source_event_id),
        PayloadField::new("perceived_kind", "visible_container"),
        PayloadField::new("target_id", container_id),
        PayloadField::new("is_open", is_open.to_string()),
        PayloadField::new("is_locked", "false"),
    ];
    event
}

fn projection_with_notice(event_id: &str, actor_id: &ActorId) -> EpistemicProjection {
    let mut projection = EpistemicProjection::new(manifest_id());
    let event = role_notice_event(event_id, actor_id, SimTick::new(3));
    assert_eq!(
        apply_epistemic_event(&mut projection, &event),
        Ok(ApplyOutcome::Applied)
    );
    projection
}

fn embodied_context_from_projection(
    projection: &EpistemicProjection,
    actor_id: ActorId,
    tick: SimTick,
    frontier: u64,
) -> KnowledgeContext {
    let state = PhysicalState::from_validated_seed_parts(
        BTreeMap::from([(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), place_id()),
        )]),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        NeedModelState::new(5, 3),
    );
    current_place_knowledge_context(
        &state,
        Some(projection),
        &actor_id,
        tick,
        &manifest_id(),
        frontier,
    )
}

fn workplace_fact(source_event_id: &str) -> ActorKnownWorkplaceFact {
    ActorKnownWorkplaceFact::new(
        workplace_id(),
        place_id(),
        true,
        format!("role_assignment_notice:{source_event_id}"),
        SourceEventIds::checked(vec![EventId::new(source_event_id).unwrap()]).unwrap(),
        SimTick::new(3),
    )
}

fn context_with_source(
    actor_id: ActorId,
    tick: SimTick,
    frontier: u64,
    source_event_id: Option<&str>,
    kind: KnowledgeProvenanceKind,
) -> KnowledgeContext {
    let (workplaces, provenance) = if let Some(source_event_id) = source_event_id {
        (
            vec![workplace_fact(source_event_id)],
            vec![KnowledgeProvenanceEntry::actor_known_source(
                kind,
                AllowedKnowledgeSource::OwnSourceBackedBelief,
                source_event_id,
            )],
        )
    } else {
        (Vec::new(), Vec::new())
    };
    KnowledgeContext::embodied_at_frontier_with_all_facts_observations_and_provenance(
        actor_id,
        tick,
        frontier,
        workplaces,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        provenance,
    )
}

#[test]
fn applied_observations_replace_same_subject_without_erasing_sibling_subjects() {
    let actor = actor_id("actor_tomas");
    let mut projection = EpistemicProjection::new(manifest_id());
    for event in [
        visible_container_event(
            "event_observed_strongbox_closed",
            "observation_strongbox_closed",
            "event_visible_container_window",
            &actor,
            "strongbox_tomas",
            false,
        ),
        visible_container_event(
            "event_observed_strongbox_open",
            "observation_strongbox_open",
            "event_visible_container_window",
            &actor,
            "strongbox_tomas",
            true,
        ),
        visible_container_event(
            "event_observed_crate_closed",
            "observation_crate_closed",
            "event_visible_container_window",
            &actor,
            "crate_tomas",
            false,
        ),
    ] {
        assert_eq!(
            apply_epistemic_event(&mut projection, &event),
            Ok(ApplyOutcome::Applied)
        );
    }

    let context = embodied_context_from_projection(&projection, actor, SimTick::new(4), 3);
    assert_eq!(context.actor_known_containers().len(), 2);
    let strongbox = context
        .actor_known_containers()
        .iter()
        .find(|fact| fact.container_id() == &container_id("strongbox_tomas"))
        .expect("same-source replacement keeps the newest strongbox fact");
    assert!(strongbox.is_open());
    assert!(!strongbox.is_locked());
    assert!(context
        .actor_known_containers()
        .iter()
        .any(|fact| fact.container_id() == &container_id("crate_tomas")));
}

#[test]
fn holder_known_interval_delta_emits_only_verified_new_sources() {
    let actor = actor_id("actor_tomas");
    let source_event_id = "event_role_notice_interval";
    let projection = projection_with_notice(source_event_id, &actor);
    let before = context_with_source(
        actor.clone(),
        SimTick::new(2),
        10,
        None,
        KnowledgeProvenanceKind::Record,
    );
    let after = context_with_source(
        actor.clone(),
        SimTick::new(3),
        11,
        Some(source_event_id),
        KnowledgeProvenanceKind::Record,
    );

    let delta = projection
        .actor_known_interval_delta(&before, &after, IntervalStopReason::ControllerSafetyBound)
        .unwrap();

    assert_eq!(delta.viewer_actor_id(), &actor);
    assert_eq!(delta.start_tick(), SimTick::new(2));
    assert_eq!(delta.stop_tick(), SimTick::new(3));
    assert_eq!(delta.start_frontier(), 10);
    assert_eq!(delta.stop_frontier(), 11);
    assert_eq!(
        delta.stop_reason(),
        IntervalStopReason::ControllerSafetyBound
    );
    assert!(!delta.no_new_actor_known_information());
    assert_eq!(delta.notices().len(), 1);
    assert_eq!(delta.notices()[0].notice_kind(), IntervalNoticeKind::Record);
    assert_eq!(
        delta.notices()[0].source_event_id(),
        &EventId::new(source_event_id).unwrap()
    );
}

#[test]
fn hidden_world_difference_without_holder_known_source_is_observationally_equal() {
    let actor = actor_id("actor_tomas");
    let before = context_with_source(
        actor.clone(),
        SimTick::new(2),
        10,
        None,
        KnowledgeProvenanceKind::Record,
    );
    let after = context_with_source(
        actor,
        SimTick::new(3),
        11,
        None,
        KnowledgeProvenanceKind::Record,
    );
    let hidden_a = EpistemicProjection::new(manifest_id());
    let hidden_b = projection_with_notice("event_other_actor_notice", &actor_id("actor_mara"));

    let delta_a = hidden_a
        .actor_known_interval_delta(&before, &after, IntervalStopReason::ControllerSafetyBound)
        .unwrap();
    let delta_b = hidden_b
        .actor_known_interval_delta(&before, &after, IntervalStopReason::ControllerSafetyBound)
        .unwrap();

    assert_eq!(delta_a, delta_b);
    assert!(delta_a.no_new_actor_known_information());
}

#[test]
fn holder_known_interval_delta_fails_closed_for_unresolved_or_wrong_kind_sources() {
    let actor = actor_id("actor_tomas");
    let projection = EpistemicProjection::new(manifest_id());
    let before = context_with_source(
        actor.clone(),
        SimTick::new(2),
        10,
        None,
        KnowledgeProvenanceKind::Record,
    );
    let after = context_with_source(
        actor.clone(),
        SimTick::new(3),
        11,
        Some("event_missing_projection_source"),
        KnowledgeProvenanceKind::Record,
    );

    assert_eq!(
        projection.actor_known_interval_delta(
            &before,
            &after,
            IntervalStopReason::ControllerSafetyBound
        ),
        Err(ActorKnownIntervalDeltaError::UnresolvedSource {
            source_key: "event_missing_projection_source".to_string(),
        })
    );

    let projection = projection_with_notice("event_memory_only_source", &actor);
    let wrong_kind_after = context_with_source(
        actor,
        SimTick::new(3),
        11,
        Some("event_memory_only_source"),
        KnowledgeProvenanceKind::Memory,
    );

    assert_eq!(
        projection.actor_known_interval_delta(
            &before,
            &wrong_kind_after,
            IntervalStopReason::ControllerSafetyBound
        ),
        Err(ActorKnownIntervalDeltaError::UnsupportedProvenanceKind {
            source_key: "event_memory_only_source".to_string(),
            kind: "memory",
        })
    );
}

#[test]
fn holder_known_interval_delta_rejects_other_holder_or_debug_contexts() {
    let actor = actor_id("actor_tomas");
    let projection = EpistemicProjection::new(manifest_id());
    let before = context_with_source(
        actor,
        SimTick::new(2),
        10,
        None,
        KnowledgeProvenanceKind::Record,
    );
    let other_after = context_with_source(
        actor_id("actor_mara"),
        SimTick::new(3),
        11,
        None,
        KnowledgeProvenanceKind::Record,
    );

    assert_eq!(
        projection.actor_known_interval_delta(
            &before,
            &other_after,
            IntervalStopReason::ControllerSafetyBound
        ),
        Err(ActorKnownIntervalDeltaError::ContextHolderMismatch)
    );
}
