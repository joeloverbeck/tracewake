use std::collections::{BTreeMap, BTreeSet};

use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::current_place_knowledge_context;
use tracewake_core::checksum::ChecksumContext;
use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::EventStream;
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, FixtureId, PlaceId,
};
use tracewake_core::projections::{IntervalNoticeKind, IntervalSalience, IntervalStopReason};
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::{
    AdvanceUntilRequest, AdvanceUntilStopReason, DeterministicScheduler, WorldAdvanceOrigin,
    WorldAdvanceRequest, WorldStepTransactionRequest,
};
use tracewake_core::state::{
    ActorBody, AgentState, NeedModelState, PhysicalState, PlaceState, VisibilityDefault,
};
use tracewake_core::time::SimTick;

fn actor_id(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn place_id(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn manifest_id() -> ContentManifestId {
    ContentManifestId::new("salient_stop_actor_known").unwrap()
}

fn physical_state(actor_ids: impl IntoIterator<Item = ActorId>) -> PhysicalState {
    physical_state_with_remote_actor(actor_ids, None)
}

fn physical_state_with_remote_actor(
    actor_ids: impl IntoIterator<Item = ActorId>,
    remote_actor_id: Option<&ActorId>,
) -> PhysicalState {
    let kitchen = place_id("kitchen");
    let pantry = place_id("pantry");
    let mut actors = BTreeMap::new();
    let mut kitchen_actor_ids = BTreeSet::new();
    let mut pantry_actor_ids = BTreeSet::new();
    for actor_id in actor_ids {
        let actor_place = if remote_actor_id == Some(&actor_id) {
            pantry.clone()
        } else {
            kitchen.clone()
        };
        actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), actor_place.clone()),
        );
        if actor_place == pantry {
            pantry_actor_ids.insert(actor_id);
        } else {
            kitchen_actor_ids.insert(actor_id);
        }
    }
    let mut places = BTreeMap::new();
    places.insert(
        kitchen.clone(),
        PlaceState {
            place_id: kitchen,
            display_label: "Kitchen".to_string(),
            adjacent_place_ids: BTreeSet::from([pantry.clone()]),
            connected_door_ids: BTreeSet::new(),
            local_container_ids: BTreeSet::new(),
            local_item_ids: BTreeSet::new(),
            local_actor_ids: kitchen_actor_ids,
            visibility_default: VisibilityDefault::Visible,
        },
    );
    places.insert(
        pantry.clone(),
        PlaceState {
            place_id: pantry,
            display_label: "Pantry".to_string(),
            adjacent_place_ids: BTreeSet::new(),
            connected_door_ids: BTreeSet::new(),
            local_container_ids: BTreeSet::new(),
            local_item_ids: BTreeSet::new(),
            local_actor_ids: pantry_actor_ids,
            visibility_default: VisibilityDefault::Visible,
        },
    );
    PhysicalState::from_validated_content_parts(
        actors,
        places,
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        NeedModelState::new(0, 0),
    )
}

fn agent_state() -> AgentState {
    AgentState::from_validated_content_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn world_advance_request(expected_tick: u64) -> WorldAdvanceRequest {
    WorldAdvanceRequest {
        expected_tick: SimTick::new(expected_tick),
        origin: WorldAdvanceOrigin::Controller(ControllerId::new("controller_human").unwrap()),
        content_manifest_id: manifest_id(),
        authorized_sleep_interruptions: Vec::new(),
    }
}

fn checksum_context(tick: SimTick, log: &EventLog) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("salient_stop_actor_known").unwrap(),
        content_version: ContentVersion::new(manifest_id().as_str()).unwrap(),
        sim_tick: tick,
        world_stream_position_applied: log
            .events()
            .iter()
            .filter(|event| event.stream == EventStream::World)
            .count()
            .saturating_sub(1) as u64,
    }
}

#[test]
fn typed_salience_policy_distinguishes_quiet_novel_hidden_and_replay_cases() {
    let possessed = actor_id("actor_tomas");
    let other = actor_id("actor_mara");

    let mut quiet_physical = physical_state([possessed.clone(), other.clone()]);
    let mut quiet_agent = agent_state();
    let mut quiet_log = EventLog::new();
    let mut quiet_projection = EpistemicProjection::new(manifest_id());
    let mut quiet_scheduler = DeterministicScheduler::new(SimTick::ZERO);
    quiet_scheduler
        .transact_world_one_tick(
            &mut quiet_physical,
            &mut quiet_agent,
            &mut quiet_log,
            &ActionRegistry::new(),
            None,
            Some(&mut quiet_projection),
            WorldStepTransactionRequest {
                advance: world_advance_request(0),
                controlled_proposals: Vec::new(),
                actor_known_interval_actor_id: Some(possessed.clone()),
            },
        )
        .unwrap();

    let quiet_result = quiet_scheduler
        .advance_until(
            &mut quiet_physical,
            &mut quiet_agent,
            &mut quiet_log,
            &ActionRegistry::new(),
            Some(&mut quiet_projection),
            AdvanceUntilRequest {
                expected_tick: SimTick::new(1),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: manifest_id(),
                possessed_actor_id: possessed.clone(),
                max_ticks: 3,
            },
        )
        .unwrap();
    assert_eq!(
        quiet_result.stop_reason,
        AdvanceUntilStopReason::ControllerSafetyBound
    );
    let quiet_delta = quiet_result
        .actor_known_interval_delta
        .as_ref()
        .expect("quiet interval still carries typed interval evidence");
    assert_eq!(quiet_delta.salience(), IntervalSalience::None);
    assert!(
        quiet_delta
            .notices()
            .iter()
            .any(|notice| notice.notice_kind() == IntervalNoticeKind::Observation),
        "quiet case must include routine perception notices, got {:#?}",
        quiet_delta.notices()
    );

    let initial_physical = physical_state([possessed.clone(), other.clone()]);
    let initial_agent = agent_state();
    let mut physical = initial_physical.clone();
    let mut agent = initial_agent.clone();
    let mut log = EventLog::new();
    let mut projection = EpistemicProjection::new(manifest_id());
    let mut scheduler = DeterministicScheduler::new(SimTick::ZERO);

    let advance_result = scheduler
        .advance_until(
            &mut physical,
            &mut agent,
            &mut log,
            &ActionRegistry::new(),
            Some(&mut projection),
            AdvanceUntilRequest {
                expected_tick: SimTick::ZERO,
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: manifest_id(),
                possessed_actor_id: possessed.clone(),
                max_ticks: 3,
            },
        )
        .unwrap();
    assert_eq!(
        advance_result.stop_reason,
        AdvanceUntilStopReason::ActorKnownSalientObservation
    );
    assert_eq!(advance_result.stop_tick, SimTick::new(1));
    let delta = advance_result
        .actor_known_interval_delta
        .as_ref()
        .expect("salient stop carries typed actor-known interval evidence");
    assert_eq!(
        delta.stop_reason(),
        IntervalStopReason::ActorKnownSalientObservation
    );
    assert_eq!(delta.salience(), IntervalSalience::NovelActorKnownFact);
    assert!(
        delta.notices().iter().any(|notice| notice.notice_kind()
            == IntervalNoticeKind::Observation
            && notice.source_event_id().as_str() != "event_hidden_observation_other_actor"),
        "expected possessed-actor perception notice, got {:#?}",
        delta.notices()
    );
    assert!(delta.notices().iter().all(|notice| {
        notice.source_event_id().as_str() != "event_hidden_observation_other_actor"
    }));

    let mut hidden_physical =
        physical_state_with_remote_actor([possessed.clone(), other.clone()], Some(&other));
    let mut hidden_agent = agent_state();
    let mut hidden_log = EventLog::new();
    let mut hidden_projection = EpistemicProjection::new(manifest_id());
    let mut hidden_scheduler = DeterministicScheduler::new(SimTick::ZERO);
    hidden_scheduler
        .transact_world_one_tick(
            &mut hidden_physical,
            &mut hidden_agent,
            &mut hidden_log,
            &ActionRegistry::new(),
            None,
            Some(&mut hidden_projection),
            WorldStepTransactionRequest {
                advance: world_advance_request(0),
                controlled_proposals: Vec::new(),
                actor_known_interval_actor_id: Some(possessed.clone()),
            },
        )
        .unwrap();
    let hidden_result = hidden_scheduler
        .advance_until(
            &mut hidden_physical,
            &mut hidden_agent,
            &mut hidden_log,
            &ActionRegistry::new(),
            Some(&mut hidden_projection),
            AdvanceUntilRequest {
                expected_tick: SimTick::new(1),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: manifest_id(),
                possessed_actor_id: possessed.clone(),
                max_ticks: 3,
            },
        )
        .unwrap();
    assert_eq!(
        hidden_result.stop_reason,
        AdvanceUntilStopReason::ControllerSafetyBound
    );
    assert_eq!(
        hidden_result
            .actor_known_interval_delta
            .as_ref()
            .unwrap()
            .salience(),
        IntervalSalience::None
    );

    let rebuild = rebuild_projection(
        &initial_physical,
        &initial_agent,
        &log,
        &checksum_context(SimTick::ZERO, &log),
        Some(&physical),
    );
    assert!(
        rebuild.temporal_violations.is_empty(),
        "{:#?}",
        rebuild.temporal_violations
    );
    assert!(rebuild.epistemic_application_errors.is_empty());
    let before = current_place_knowledge_context(
        &initial_physical,
        None,
        &possessed,
        SimTick::ZERO,
        &manifest_id(),
        1,
    );
    let after = current_place_knowledge_context(
        &rebuild.final_state,
        Some(&rebuild.final_epistemic_projection),
        &possessed,
        rebuild.reconstructed_final_frontier,
        &manifest_id(),
        log.events().len() as u64,
    );
    let replay_delta = rebuild
        .final_epistemic_projection
        .actor_known_interval_delta(
            &before,
            &after,
            IntervalStopReason::ActorKnownSalientObservation,
        )
        .unwrap();
    assert_eq!(replay_delta.stop_tick(), scheduler.current_tick());
    assert_eq!(replay_delta.salience(), delta.salience());
    assert_eq!(replay_delta.notices(), delta.notices());
    assert!(replay_delta.notices().iter().all(|notice| {
        notice.source_event_id().as_str() != "event_hidden_observation_other_actor"
    }));
}
