use std::collections::BTreeMap;

use tracewake_core::checksum::ChecksumContext;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventKind, EventStream};
use tracewake_core::ids::{ContentManifestId, ContentVersion, ControllerId, FixtureId};
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::{
    DeterministicScheduler, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
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
