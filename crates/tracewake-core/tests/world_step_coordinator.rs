use std::collections::BTreeMap;

use tracewake_core::checksum::ChecksumContext;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventCause, EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1,
};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, ControllerId, EventId, FixtureId,
    ProposalId,
};
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::{
    BodyExclusiveDurationKind, DeterministicScheduler, OrderingKey, ProposalSequence,
    SchedulePhase, SchedulerSourceId, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

fn content_manifest_id() -> ContentManifestId {
    ContentManifestId::new("test_manifest").unwrap()
}

fn checksum_context(tick: SimTick, world_stream_position_applied: u64) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("world_step_coordinator").unwrap(),
        content_version: ContentVersion::new("test_manifest").unwrap(),
        sim_tick: tick,
        world_stream_position_applied,
    }
}

fn empty_agent_state() -> AgentState {
    AgentState::from_seed_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn actor_id(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn ordering_key(tick: u64, action_id: &str, actor_id: &ActorId) -> OrderingKey {
    OrderingKey::new(
        SimTick::new(tick),
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(0),
        ActionId::new(action_id).unwrap(),
        vec![actor_id.as_str().to_string()],
        action_id,
    )
}

fn duration_start(
    kind: EventKind,
    id: &str,
    actor_id: &ActorId,
    start_tick: u64,
    expected_completion_tick: u64,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        0,
        0,
        SimTick::new(start_tick),
        ordering_key(start_tick, kind.stable_id(), actor_id),
        content_manifest_id(),
        vec![EventCause::Proposal(
            ProposalId::new(format!("proposal_{id}")).unwrap(),
        )],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new(
            "expected_completion_tick",
            expected_completion_tick.to_string(),
        ),
        PayloadField::new("body_exclusive", "true"),
    ];
    event
}

fn duration_terminal(
    kind: EventKind,
    id: &str,
    actor_id: &ActorId,
    start_event_id: &str,
    tick: u64,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        0,
        0,
        SimTick::new(tick),
        ordering_key(tick, kind.stable_id(), actor_id),
        content_manifest_id(),
        vec![EventCause::Event(EventId::new(start_event_id).unwrap())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
    event
}

#[test]
fn empty_world_step_appends_time_advanced_and_rebuilds_frontier() {
    let initial_physical = PhysicalState::empty(NeedModelState::new(0, 0));
    let initial_agent = empty_agent_state();
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(41));

    let result = scheduler
        .advance_world_one_tick(
            &mut log,
            WorldAdvanceRequest {
                expected_tick: SimTick::new(41),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: content_manifest_id(),
            },
        )
        .unwrap();

    assert_eq!(result.prior_tick, SimTick::new(41));
    assert_eq!(result.resulting_tick, SimTick::new(42));
    assert_eq!(scheduler.current_tick, SimTick::new(42));
    assert_eq!(result.appended_event_ids.len(), 1);
    assert!(result.due_duration_candidates.is_empty());
    assert_eq!(result.due_work_summary.open_duration_candidates, 0);
    assert_eq!(log.events().len(), 1);

    let marker = &log.events()[0];
    assert_eq!(marker.event_type, EventKind::TimeAdvanced);
    assert_eq!(marker.stream, EventStream::World);
    assert_eq!(marker.sim_tick, SimTick::new(42));
    assert_eq!(marker.causes.len(), 1);
    assert!(marker.actor_id.is_none());
    assert_eq!(marker.payload_value("prior_tick"), Some("41"));
    assert_eq!(marker.payload_value("resulting_tick"), Some("42"));
    assert_eq!(
        marker.payload_value("origin"),
        Some("controller.controller_human")
    );
    assert_eq!(
        marker.payload_value("ordering_ancestry"),
        Some("canonical_world_step_v1")
    );

    let rebuild = rebuild_projection(
        &initial_physical,
        &initial_agent,
        &log,
        &checksum_context(SimTick::new(42), 1),
        Some(&initial_physical),
    );

    assert_eq!(rebuild.event_count_applied, 1);
    assert_eq!(rebuild.last_event_id, Some(marker.event_id.clone()));
    assert_eq!(rebuild.last_stream_position, Some(0));
    assert!(rebuild.invariant_violations.is_empty());
    assert_eq!(rebuild.final_state, initial_physical);
    assert_eq!(rebuild.final_agent_state, initial_agent);
}

#[test]
fn coordinator_discovers_due_open_duration_from_prior_transaction() {
    let actor = actor_id("actor_tomas");
    let mut log = EventLog::new();
    log.append(duration_start(
        EventKind::SleepStarted,
        "event_sleep_started_prior",
        &actor,
        8,
        10,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(
            &mut log,
            WorldAdvanceRequest {
                expected_tick: SimTick::new(9),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: content_manifest_id(),
            },
        )
        .unwrap();

    assert_eq!(result.resulting_tick, SimTick::new(10));
    assert_eq!(result.due_work_summary.open_duration_candidates, 1);
    assert_eq!(result.due_work_summary.duration_terminals_appended, 0);
    assert_eq!(result.due_duration_candidates.len(), 1);
    assert_eq!(
        result.due_duration_candidates[0].start_event_id.as_str(),
        "event_sleep_started_prior"
    );
    assert_eq!(result.due_duration_candidates[0].actor_id, actor);
    assert_eq!(
        result.due_duration_candidates[0].duration_kind,
        BodyExclusiveDurationKind::Sleep
    );
    assert_eq!(
        result.due_duration_candidates[0].expected_completion_tick,
        SimTick::new(10)
    );
    assert_eq!(log.events().len(), 2);
    assert_eq!(log.events()[1].event_type, EventKind::TimeAdvanced);
}

#[test]
fn coordinator_discovers_due_candidates_in_deterministic_order() {
    let tomas = actor_id("actor_tomas");
    let ada = actor_id("actor_ada");
    let mut log = EventLog::new();
    log.append(duration_start(
        EventKind::WorkBlockStarted,
        "event_work_started_tomas",
        &tomas,
        6,
        10,
    ))
    .unwrap();
    log.append(duration_start(
        EventKind::SleepStarted,
        "event_sleep_started_ada",
        &ada,
        7,
        10,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(
            &mut log,
            WorldAdvanceRequest {
                expected_tick: SimTick::new(9),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: content_manifest_id(),
            },
        )
        .unwrap();

    assert_eq!(
        result
            .due_duration_candidates
            .iter()
            .map(|candidate| candidate.start_event_id.as_str())
            .collect::<Vec<_>>(),
        vec!["event_sleep_started_ada", "event_work_started_tomas"]
    );
}

#[test]
fn duplicate_duration_terminal_rejects_world_step_without_appending_marker() {
    let actor = actor_id("actor_tomas");
    let mut log = EventLog::new();
    log.append(duration_start(
        EventKind::WorkBlockStarted,
        "event_work_started_prior",
        &actor,
        8,
        10,
    ))
    .unwrap();
    log.append(duration_terminal(
        EventKind::WorkBlockCompleted,
        "event_work_completed_prior",
        &actor,
        "event_work_started_prior",
        10,
    ))
    .unwrap();
    log.append(duration_terminal(
        EventKind::WorkBlockFailed,
        "event_work_failed_duplicate",
        &actor,
        "event_work_started_prior",
        10,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let error = scheduler
        .advance_world_one_tick(
            &mut log,
            WorldAdvanceRequest {
                expected_tick: SimTick::new(9),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: content_manifest_id(),
            },
        )
        .unwrap_err();

    let WorldAdvanceError::OpenDurationDiscovery(error) = error else {
        panic!("expected open-duration discovery failure");
    };
    assert_eq!(error.start_event_id.as_str(), "event_work_started_prior");
    assert_eq!(
        error.first_terminal_event_id.as_str(),
        "event_work_completed_prior"
    );
    assert_eq!(
        error.duplicate_terminal_event_id.as_str(),
        "event_work_failed_duplicate"
    );
    assert_eq!(scheduler.current_tick, SimTick::new(9));
    assert_eq!(log.events().len(), 3);
}

#[test]
fn stale_expected_tick_rejects_without_appending_or_advancing() {
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(10));

    let error = scheduler
        .advance_world_one_tick(
            &mut log,
            WorldAdvanceRequest {
                expected_tick: SimTick::new(9),
                origin: WorldAdvanceOrigin::Controller(
                    ControllerId::new("controller_human").unwrap(),
                ),
                content_manifest_id: content_manifest_id(),
            },
        )
        .unwrap_err();

    assert_eq!(
        error,
        WorldAdvanceError::FrontierMismatch {
            expected: SimTick::new(9),
            actual: SimTick::new(10),
        }
    );
    assert_eq!(scheduler.current_tick, SimTick::new(10));
    assert!(log.events().is_empty());
}

#[test]
fn time_advanced_metadata_is_world_cause_required_marker() {
    let metadata = EventKind::TimeAdvanced.metadata();

    assert_eq!(metadata.stream, EventStream::World);
    assert!(metadata.physical_mutating);
    assert!(metadata.cause_required);
    assert!(!tracewake_core::events::is_duration_terminal(
        EventKind::TimeAdvanced
    ));
}

trait PayloadLookup {
    fn payload_value(&self, key: &str) -> Option<&str>;
}

impl PayloadLookup for tracewake_core::events::EventEnvelope {
    fn payload_value(&self, key: &str) -> Option<&str> {
        self.payload
            .iter()
            .find(|field| field.key == key)
            .map(|field| field.value.as_str())
    }
}
