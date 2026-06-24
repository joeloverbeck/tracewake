use tracewake_core::checksum::ChecksumContext;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, EventId, FixtureId, ProcessId,
};
use tracewake_core::replay::report::{ReplayDivergenceDetail, ReplayDivergenceFieldFamily};
use tracewake_core::replay::run_replay;
use tracewake_core::replay::{rebuild_projection, TemporalDivergence};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

fn context(initial_tick: u64) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("replay_temporal_frontier").unwrap(),
        content_version: ContentVersion::new("test_manifest").unwrap(),
        sim_tick: SimTick::new(initial_tick),
        world_stream_position_applied: 0,
    }
}

fn content_manifest_id() -> ContentManifestId {
    ContentManifestId::new("test_manifest").unwrap()
}

fn ordering_key(tick: u64, action_id: &str) -> OrderingKey {
    OrderingKey::new(
        SimTick::new(tick),
        SchedulePhase::DeferredProcess,
        SchedulerSourceId::Process(ProcessId::new("world_step.test").unwrap()),
        ProposalSequence::new(0),
        ActionId::new(action_id).unwrap(),
        vec![tick.to_string()],
        action_id,
    )
}

fn time_advanced(id: &str, prior_tick: u64, resulting_tick: u64) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        EventKind::TimeAdvanced,
        99,
        99,
        SimTick::new(resulting_tick),
        ordering_key(resulting_tick, "time_advanced"),
        content_manifest_id(),
        vec![EventCause::Process(
            ProcessId::new("world_step.test").unwrap(),
        )],
    )
    .unwrap();
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("prior_tick", prior_tick.to_string()),
        PayloadField::new("resulting_tick", resulting_tick.to_string()),
        PayloadField::new("origin", "process.world_step.test"),
        PayloadField::new("ordering_ancestry", "canonical_world_step_v1"),
    ];
    event
}

fn actor_waited(id: &str, tick: u64) -> EventEnvelope {
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        EventKind::ActorWaited,
        99,
        99,
        SimTick::new(tick),
        OrderingKey::new(
            SimTick::new(tick),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id.clone()),
            ProposalSequence::new(0),
            ActionId::new("wait").unwrap(),
            vec![actor_id.as_str().to_string()],
            "wait",
        ),
        content_manifest_id(),
        vec![EventCause::Process(
            ProcessId::new("world_step.test").unwrap(),
        )],
    )
    .unwrap();
    event.actor_id = Some(actor_id);
    event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
    event
}

fn rebuild(log: &EventLog, initial_tick: u64) -> tracewake_core::replay::ProjectionRebuildReport {
    rebuild_projection(
        &PhysicalState::empty(NeedModelState::new(0, 0)),
        &AgentState::default(),
        log,
        &context(initial_tick),
        None,
    )
}

#[test]
fn run_replay_temporal_violation_fails_aggregate_and_reports_typed_first_divergence() {
    let initial_state = PhysicalState::empty(NeedModelState::new(0, 0));
    let initial_agent_state = AgentState::default();
    let mut marker = time_advanced("event_time_no_ancestry_report", 0, 1);
    marker
        .payload
        .retain(|field| field.key != "ordering_ancestry");
    let mut log = EventLog::new();
    log.append(marker).unwrap();

    let report = run_replay(
        &initial_state,
        &initial_agent_state,
        &log,
        &context(0),
        Some(&initial_state),
        None,
        None,
    );

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::MissingOrderingAncestry {
            event_id: EventId::new("event_time_no_ancestry_report").unwrap(),
        }]
    );
    assert!(report.state_diff.is_empty());
    assert!(report.application_errors.is_empty());
    assert!(!report.matches_expected);
    assert_eq!(
        report.first_divergence,
        Some(ReplayDivergenceDetail {
            first_divergent_event_id: Some("event_time_no_ancestry_report".to_string()),
            field_family: ReplayDivergenceFieldFamily::Temporal,
        })
    );
}

#[test]
fn run_replay_clean_time_marker_still_matches_expected() {
    let initial_state = PhysicalState::empty(NeedModelState::new(0, 0));
    let initial_agent_state = AgentState::default();
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_clean_report", 0, 1))
        .unwrap();

    let report = run_replay(
        &initial_state,
        &initial_agent_state,
        &log,
        &context(0),
        Some(&initial_state),
        None,
        None,
    );

    assert!(report.temporal_violations.is_empty());
    assert!(report.matches_expected);
    assert_eq!(report.first_divergence, None);
}

#[test]
fn chained_time_advanced_markers_reconstruct_frontier() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_1", 0, 1)).unwrap();
    log.append(time_advanced("event_time_2", 1, 2)).unwrap();
    log.append(time_advanced("event_time_3", 2, 3)).unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(report.reconstructed_final_frontier, SimTick::new(3));
    assert!(report.temporal_violations.is_empty());
}

#[test]
fn missing_time_marker_for_future_world_event_is_typed_divergence() {
    let mut log = EventLog::new();
    log.append(actor_waited("event_wait_without_marker", 1))
        .unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::OrdinaryEventWithoutMarker {
            event_id: EventId::new("event_wait_without_marker").unwrap(),
            event_tick: SimTick::new(1),
            reconstructed_frontier: SimTick::ZERO,
        }]
    );
}

#[test]
fn duplicate_time_marker_is_typed_divergence() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_1", 0, 1)).unwrap();
    log.append(time_advanced("event_time_duplicate", 0, 1))
        .unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::DuplicateTimeAdvanced {
            event_id: EventId::new("event_time_duplicate").unwrap(),
            tick: SimTick::new(1),
        }]
    );
}

#[test]
fn duplicate_time_marker_checks_each_frontier_side_independently() {
    let mut prior_before_frontier = EventLog::new();
    prior_before_frontier
        .append(time_advanced("event_time_prior_before_frontier", 1, 3))
        .unwrap();

    let report = rebuild(&prior_before_frontier, 2);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::DuplicateTimeAdvanced {
            event_id: EventId::new("event_time_prior_before_frontier").unwrap(),
            tick: SimTick::new(3),
        }]
    );

    let mut result_at_frontier = EventLog::new();
    result_at_frontier
        .append(time_advanced("event_time_result_at_frontier", 2, 2))
        .unwrap();

    let report = rebuild(&result_at_frontier, 2);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::DuplicateTimeAdvanced {
            event_id: EventId::new("event_time_result_at_frontier").unwrap(),
            tick: SimTick::new(2),
        }]
    );
}

#[test]
fn prior_result_mismatch_is_typed_divergence() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_bad_prior", 2, 3))
        .unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::PriorTickMismatch {
            event_id: EventId::new("event_time_bad_prior").unwrap(),
            expected: SimTick::ZERO,
            actual: SimTick::new(2),
        }]
    );
}

#[test]
fn multi_tick_jump_is_typed_divergence() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_jump", 0, 2)).unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::ResultingTickMismatch {
            event_id: EventId::new("event_time_jump").unwrap(),
            expected: SimTick::new(1),
            actual: SimTick::new(2),
        }]
    );
}

#[test]
fn equal_prior_result_tick_is_not_backward_time() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_equal_tick", 5, 5))
        .unwrap();

    let report = rebuild(&log, 4);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::PriorTickMismatch {
            event_id: EventId::new("event_time_equal_tick").unwrap(),
            expected: SimTick::new(4),
            actual: SimTick::new(5),
        }]
    );
}

#[test]
fn backward_marker_is_typed_divergence() {
    let mut log = EventLog::new();
    log.append(time_advanced("event_time_backward", 2, 1))
        .unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::BackwardTimeAdvanced {
            event_id: EventId::new("event_time_backward").unwrap(),
            prior_tick: SimTick::new(2),
            resulting_tick: SimTick::new(1),
        }]
    );
}

#[test]
fn envelope_payload_disagreement_is_typed_divergence() {
    let mut marker = time_advanced("event_time_envelope_bad", 0, 1);
    marker.sim_tick = SimTick::new(2);
    let mut log = EventLog::new();
    log.append(marker).unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::EnvelopePayloadMismatch {
            event_id: EventId::new("event_time_envelope_bad").unwrap(),
            envelope_tick: SimTick::new(2),
            resulting_tick: SimTick::new(1),
        }]
    );
}

#[test]
fn missing_or_wrong_cause_is_typed_divergence() {
    let mut marker = time_advanced("event_time_no_cause", 0, 1);
    marker.causes.clear();
    let mut log = EventLog::new();
    log.append(marker).unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::MissingOrWrongCause {
            event_id: EventId::new("event_time_no_cause").unwrap(),
        }]
    );
}

#[test]
fn missing_ordering_ancestry_is_typed_divergence() {
    let mut marker = time_advanced("event_time_no_ancestry", 0, 1);
    marker
        .payload
        .retain(|field| field.key != "ordering_ancestry");
    let mut log = EventLog::new();
    log.append(marker).unwrap();

    let report = rebuild(&log, 0);

    assert_eq!(
        report.temporal_violations,
        vec![TemporalDivergence::MissingOrderingAncestry {
            event_id: EventId::new("event_time_no_ancestry").unwrap(),
        }]
    );
}
