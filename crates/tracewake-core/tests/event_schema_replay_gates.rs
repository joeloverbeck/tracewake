mod support;

use std::collections::BTreeSet;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{BlockerCode, NeedChangeCause, NeedKind, NeedState, ResponsibleLayer};
use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
use tracewake_core::events::log::{EventLog, EventLogError};
use tracewake_core::events::{EventEnvelope, EventKind, EventStream, PayloadField};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, EventId, FixtureId, ItemId,
    PlaceId, SchemaVersion,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
};
use tracewake_core::time::SimTick;

fn actor_id() -> ActorId {
    ActorId::new("actor_tomas").unwrap()
}

fn manifest_id() -> ContentManifestId {
    ContentManifestId::new("phase1_schema_replay_gates_manifest").unwrap()
}

fn content_version() -> ContentVersion {
    ContentVersion::new(manifest_id().as_str()).unwrap()
}

fn ordering_key(sequence: u64, action_id: &str) -> OrderingKey {
    OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        ActionId::new(action_id).unwrap(),
        vec!["schema_replay_gate".to_string()],
        format!("schema_replay_gate:{sequence}"),
    )
}

fn event(id: &str, kind: EventKind, sequence: u64) -> EventEnvelope {
    EventEnvelope::new_v1(
        EventId::new(id).unwrap(),
        kind,
        sequence,
        sequence,
        SimTick::ZERO,
        ordering_key(sequence, kind.stable_id()),
        manifest_id(),
    )
}

fn world_state() -> PhysicalState {
    let mut seed = PhysicalSeed::default();
    let shop = PlaceId::new("shop_front").unwrap();
    let back = PlaceId::new("back_room").unwrap();

    let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
    shop_state.adjacent_place_ids.insert(back.clone());
    let mut back_state = PlaceState::new(back.clone(), "Back room");
    back_state.adjacent_place_ids.insert(shop.clone());
    seed.places_mut().insert(shop.clone(), shop_state);
    seed.places_mut().insert(back.clone(), back_state);
    seed.actors_mut()
        .insert(actor_id(), ActorBody::new(actor_id(), shop.clone()));

    let mut door = DoorState::new("door_shop_back".parse().unwrap(), shop.clone(), back);
    door.is_open = true;
    seed.doors_mut().insert(door.door_id.clone(), door);

    let mut container =
        ContainerState::fixed_at_place(ContainerId::new("strongbox_tomas").unwrap(), shop.clone());
    container.is_open = true;
    container
        .contents
        .insert(ItemId::new("coin_stack_01").unwrap());
    seed.containers_mut()
        .insert(container.container_id.clone(), container);
    seed.items_mut().insert(
        ItemId::new("coin_stack_01").unwrap(),
        ItemState::new(
            ItemId::new("coin_stack_01").unwrap(),
            Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
        ),
    );

    seed.build()
}

fn agent_state() -> AgentState {
    let mut seed = AgentSeed::default();
    seed.needs_by_actor_mut().insert(
        actor_id(),
        [
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 820, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
            ),
        ]
        .into_iter()
        .collect(),
    );
    seed.build()
}

fn checksum_context(fixture_id: &str, log: &EventLog) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new(fixture_id).unwrap(),
        content_version: content_version(),
        sim_tick: log
            .events()
            .last()
            .map(|event| event.sim_tick)
            .unwrap_or(SimTick::ZERO),
        world_stream_position_applied: log
            .events()
            .iter()
            .filter(|event| event.stream == EventStream::World)
            .count()
            .saturating_sub(1) as u64,
    }
}

fn append_to_log(log: &mut EventLog, event: EventEnvelope) -> EventEnvelope {
    log.append(event).unwrap()
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

#[test]
fn unsupported_event_schema_append_rejected() {
    let mut log = EventLog::new();
    let mut forged = event("event_unsupported_schema_append", EventKind::ActorWaited, 0);
    forged.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();

    assert_eq!(
        log.append(forged),
        Err(EventLogError::UnsupportedSchemaVersion(
            "event_schema_v999".to_string()
        ))
    );
    assert!(log.events().is_empty());
}

#[test]
fn unsupported_event_schema_replay_rejected() {
    let mut forged = event("event_unsupported_schema_replay", EventKind::ActorWaited, 0);
    forged.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();

    let mut canonical = forged.serialize_canonical();
    canonical.push(b'\n');
    let mut encoded = String::new();
    for byte in canonical {
        encoded.push_str(&format!("{byte:02x}"));
    }

    assert_eq!(
        EventLog::deserialize_canonical(encoded.as_bytes()),
        Err(EventLogError::UnsupportedSchemaVersion(
            "event_schema_v999".to_string()
        ))
    );
}

#[test]
fn stream_mismatch_replay_rejected() {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let mut log = EventLog::new();
    let mut mismatched = event("event_stream_mismatch", EventKind::ActorWaited, 0);
    mismatched.stream = EventStream::Diagnostic;
    append_to_log(&mut log, mismatched);

    let context = checksum_context("stream_mismatch_replay_rejected", &log);
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&initial_world),
    );

    assert!(
        replay
            .invariant_violations
            .iter()
            .any(|violation| violation.contains("EventKindStreamMismatch")),
        "{:?}",
        replay.invariant_violations
    );
    assert_eq!(
        replay.final_checksum,
        compute_physical_checksum(&initial_world, &context).checksum
    );
    assert!(replay.state_diff.is_empty());
}

#[test]
fn non_world_event_cannot_change_physical_checksum() {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let mut log = EventLog::new();
    let mut controller_event = event(
        "event_controller_with_world_payload",
        EventKind::ControllerAttached,
        0,
    );
    controller_event.payload = vec![
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new("from_place_id", "shop_front"),
        PayloadField::new("to_place_id", "back_room"),
        PayloadField::new("attempted_world_mutation", "true"),
    ];
    append_to_log(&mut log, controller_event);

    let context = checksum_context("non_world_event_cannot_change_physical_checksum", &log);
    let before = compute_physical_checksum(&initial_world, &context).checksum;
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&initial_world),
    );

    assert!(replay.invariant_violations.is_empty());
    assert_eq!(replay.event_count_applied, 0);
    assert_eq!(replay.final_checksum, before);
    assert!(replay.state_diff.is_empty());
}

#[test]
fn every_epistemic_event_kind_is_registered_and_replay_routed() {
    let epistemic_kinds = EventKind::all()
        .iter()
        .copied()
        .filter(|kind| kind.stream() == EventStream::Epistemic)
        .collect::<BTreeSet<_>>();
    let registered_epistemic_kinds = EventKind::registry()
        .into_iter()
        .filter(|metadata| metadata.stream == EventStream::Epistemic)
        .map(|metadata| metadata.kind)
        .collect::<BTreeSet<_>>();

    assert_eq!(epistemic_kinds, registered_epistemic_kinds);
    assert!(!epistemic_kinds.is_empty());
    for kind in epistemic_kinds {
        let metadata = kind.metadata();
        assert_eq!(metadata.stream, EventStream::Epistemic);
        assert_eq!(metadata.kind.stream(), EventStream::Epistemic);
        assert!(!metadata.physical_mutating);
        assert!(!kind.stable_id().is_empty());
    }
}

#[test]
fn unsupported_epistemic_payload_schema_replay_is_loud_and_not_applied() {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let mut log = EventLog::new();
    let mut bad_payload = event(
        "event_bad_epistemic_payload_schema",
        EventKind::BeliefUpdated,
        0,
    );
    bad_payload.payload = vec![PayloadField::new("schema_version", "event_schema_v999")];
    append_to_log(&mut log, bad_payload);

    let context = checksum_context("unsupported_epistemic_payload_schema_replay", &log);
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&initial_world),
    );

    assert!(replay.final_epistemic_projection.is_empty());
    assert_eq!(replay.event_count_applied, 0);
    assert!(replay
        .epistemic_application_errors
        .iter()
        .any(|error| error.contains("UnsupportedPayloadSchemaVersion")));
}

#[test]
fn replay_rebuild_checksum_matches_original_after_no_human_day() {
    let mut world = world_state();
    let mut agent_state = agent_state();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let mut log = EventLog::new();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();

    let report = run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry,
        manifest_id(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "morning".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );
    let context = checksum_context(
        "replay_rebuild_checksum_matches_original_after_no_human_day",
        &log,
    );
    let live_checksum = compute_physical_checksum(&world, &context).checksum;
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&world),
    );

    assert!(report.ordinary_pipeline_events > 0);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::NoHumanDayStarted));
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::NoHumanDayCompleted));
    let trace_event = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .expect("no-human day records decision trace");
    assert_eq!(
        payload_value(trace_event, "responsible_layer"),
        Some("candidate_generation")
    );
    assert_eq!(payload_value(trace_event, "blocker_code"), Some("none"));
    assert_eq!(
        payload_value(trace_event, "input_source"),
        Some("actor_known_context")
    );
    assert_eq!(
        payload_value(trace_event, "actual_source"),
        Some("actor_decision_transaction")
    );
    assert_eq!(
        payload_value(trace_event, "hidden_truth_referenced"),
        Some("false")
    );
    assert!(payload_value(trace_event, "remediation_hint").is_some());
    assert!(replay.unsupported_versions.is_empty());
    assert!(replay.invariant_violations.is_empty());
    assert!(replay.state_diff.is_empty());
    assert_eq!(replay.final_checksum, live_checksum);
    let replayed_trace = replay
        .final_agent_state
        .decision_traces()
        .values()
        .next()
        .expect("replay rebuilds decision trace record");
    assert_eq!(
        replayed_trace.typed_diagnostic.responsible_layer,
        ResponsibleLayer::CandidateGeneration
    );
    assert_eq!(
        replayed_trace.typed_diagnostic.blocker_code,
        BlockerCode::None
    );
}

#[test]
fn no_human_metrics_rebuild_from_typed_diagnostic_fields() {
    let mut log = EventLog::new();
    append_to_log(
        &mut log,
        event("event_metrics_day_started", EventKind::NoHumanDayStarted, 0),
    );
    let mut text_only = event(
        "event_metrics_text_only_trace",
        EventKind::DecisionTraceRecorded,
        1,
    );
    text_only.payload = vec![PayloadField::new(
        "trace_canonical",
        "decision_trace_v1|planner_budget_exhausted",
    )];
    append_to_log(&mut log, text_only);
    let mut typed = event(
        "event_metrics_typed_planning_stuck",
        EventKind::StuckDiagnosticRecorded,
        2,
    );
    typed.payload = vec![
        PayloadField::new(
            "responsible_layer",
            ResponsibleLayer::LocalPlanning.stable_id(),
        ),
        PayloadField::new("blocker_code", BlockerCode::LocalPlanFailed.stable_id()),
    ];
    append_to_log(&mut log, typed);
    append_to_log(
        &mut log,
        event(
            "event_metrics_day_completed",
            EventKind::NoHumanDayCompleted,
            3,
        ),
    );

    let live = no_human_day_metrics(&log);
    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let replayed = no_human_day_metrics(&replayed_log);

    assert_eq!(live.planner_failures, 1);
    assert_eq!(live.serialize_canonical(), replayed.serialize_canonical());
}

#[test]
fn legacy_decision_trace_without_typed_diagnostic_keys_rebuilds_with_defaults() {
    let mut world = world_state();
    let mut agent_state = agent_state();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let mut source_log = EventLog::new();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();

    run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut source_log,
        &registry,
        manifest_id(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "legacy_trace".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );

    let mut legacy_trace = source_log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .expect("source run records decision trace")
        .clone();
    legacy_trace.payload.retain(|field| {
        !matches!(
            field.key.as_str(),
            "responsible_layer"
                | "blocker_code"
                | "input_source"
                | "actual_source"
                | "hidden_truth_referenced"
                | "remediation_hint"
        )
    });
    let trace_canonical = legacy_trace
        .payload
        .iter_mut()
        .find(|field| field.key == "trace_canonical")
        .expect("trace canonical payload exists");
    trace_canonical.value = trace_canonical
        .value
        .split('|')
        .take(9)
        .collect::<Vec<_>>()
        .join("|");

    let mut legacy_log = EventLog::new();
    append_to_log(&mut legacy_log, legacy_trace);
    let context = checksum_context(
        "legacy_decision_trace_without_typed_diagnostic_keys_rebuilds_with_defaults",
        &legacy_log,
    );
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &legacy_log,
        &context,
        Some(&initial_world),
    );

    assert!(replay.agent_application_errors.is_empty());
    let trace = replay
        .final_agent_state
        .decision_traces()
        .values()
        .next()
        .expect("legacy trace rebuilds");
    assert_eq!(
        trace.typed_diagnostic.responsible_layer,
        ResponsibleLayer::CandidateGeneration
    );
    assert_eq!(trace.typed_diagnostic.blocker_code, BlockerCode::None);
    assert_eq!(trace.typed_diagnostic.input_source, "actor_known_context");
}
