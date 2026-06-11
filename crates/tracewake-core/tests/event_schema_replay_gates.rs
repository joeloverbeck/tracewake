mod support;

use std::collections::BTreeSet;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    BlockerCode, Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState,
    ResponsibleLayer, RoutineExecution, RoutineFamily,
};
use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::events::apply::{
    apply_agent_event, apply_epistemic_event, ApplyError, EpistemicApplyError,
};
use tracewake_core::events::log::{EventLog, EventLogError};
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, EventStream, PayloadField};
use tracewake_core::ids::{
    ActionId, ActorId, CandidateGoalId, ContainerId, ContentManifestId, ContentVersion,
    DecisionTraceId, EventId, FixtureId, IntentionId, ItemId, PlaceId, ProcessId,
    RoutineExecutionId, RoutineTemplateId, SchemaVersion, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::{rebuild_projection, run_replay};
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

fn caused_event(
    id: &str,
    kind: EventKind,
    sequence: u64,
    causes: Vec<EventCause>,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        sequence,
        sequence,
        SimTick::ZERO,
        ordering_key(sequence, kind.stable_id()),
        manifest_id(),
        causes,
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.payload = vec![PayloadField::new("payload_schema_version", "1")];
    event
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

fn agent_state_with_active_intention_and_routine() -> AgentState {
    let base = agent_state();
    let mut seed = AgentSeed::from_state(&base);
    let intention_id = IntentionId::new("intention_breakfast").unwrap();
    let routine_execution_id = RoutineExecutionId::new("routine_exec_breakfast").unwrap();
    let routine_template_id = RoutineTemplateId::new("routine_eat_meal").unwrap();
    seed.active_intention_by_actor_mut()
        .insert(actor_id(), intention_id.clone());
    seed.intentions_mut().insert(
        intention_id.clone(),
        Intention::adopt(
            intention_id,
            actor_id(),
            IntentionSource::CandidateGoalSelection,
            CandidateGoalId::new("goal_breakfast").unwrap(),
            Some(routine_template_id.clone()),
            Some("check_known_container".to_string()),
            5,
            SimTick::ZERO,
            DecisionTraceId::new("trace_breakfast").unwrap(),
        ),
    );
    seed.routine_executions_mut().insert(
        routine_execution_id.clone(),
        RoutineExecution::new(
            routine_execution_id,
            actor_id(),
            routine_template_id,
            RoutineFamily::EatMeal,
            SimTick::ZERO,
            Some(SimTick::new(1)),
            Some(SimTick::new(5)),
            None,
            DecisionTraceId::new("trace_breakfast").unwrap(),
        ),
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

fn assert_rebuild_agent_error(
    fixture_id: &str,
    initial_agent_state: &AgentState,
    log: &EventLog,
    expected_issue: &str,
) {
    let initial_world = world_state();
    let context = checksum_context(fixture_id, log);
    let replay = rebuild_projection(
        &initial_world,
        initial_agent_state,
        log,
        &context,
        Some(&initial_world),
    );
    assert!(
        replay
            .agent_application_errors
            .iter()
            .any(|failure| failure.issue.contains(expected_issue)),
        "{:?}",
        replay.agent_application_errors
    );
}

fn assert_rebuild_epistemic_error(log: &EventLog, expected_issue: &str) {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let context = checksum_context("epistemic_corrupt_history_rejects", log);
    let replay = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        log,
        &context,
        Some(&initial_world),
    );
    assert!(
        replay
            .epistemic_application_errors
            .iter()
            .any(|failure| failure.contains(expected_issue)),
        "{:?}",
        replay.epistemic_application_errors
    );
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn assert_forged_agent_payload_version_rejected(
    event_kind: EventKind,
    schema_key: &'static str,
    event_id: &str,
) {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let mut forged_live = caused_event(
        event_id,
        event_kind,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    forged_live.payload = vec![PayloadField::new(schema_key, "2")];

    let mut live_agent = initial_agent_state.clone();
    assert!(
        matches!(
            apply_agent_event(&mut live_agent, &forged_live),
            Err(ApplyError::BadPayload { key, value }) if key == schema_key && value == "2"
        ),
        "{event_kind:?} must reject forged {schema_key}"
    );

    let mut replay_log = EventLog::new();
    append_to_log(&mut replay_log, forged_live);
    let context = checksum_context(event_id, &replay_log);
    let replay = run_replay(
        &initial_world,
        &initial_agent_state,
        &replay_log,
        &context,
        Some(&initial_world),
        None,
        None,
    );

    assert!(!replay.matches_expected);
    assert!(
        replay.agent_application_errors.iter().any(|failure| {
            failure.issue.contains("BadPayload")
                && failure.issue.contains(schema_key)
                && failure.issue.contains("\"2\"")
        }),
        "{:?}",
        replay.agent_application_errors
    );
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
fn duplicate_need_tick_charge_rejects_live_and_replay_001() {
    let initial_agent_state = agent_state();
    let mut live_agent = initial_agent_state.clone();
    let mut first = caused_event(
        "event_need_delta_duplicate_first",
        EventKind::NeedDeltaApplied,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    first.sim_tick = SimTick::new(5);
    first.payload = vec![
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new("need_kind", "hunger"),
        PayloadField::new("delta", "10"),
        PayloadField::new("elapsed_ticks", "2"),
        PayloadField::new("cause_kind", "action_effect"),
        PayloadField::new("cause_action_id", "wait"),
    ];
    let mut duplicate = first.clone();
    duplicate.event_id = EventId::new("event_need_delta_duplicate_second").unwrap();

    assert_eq!(
        apply_agent_event(&mut live_agent, &first),
        Ok(tracewake_core::events::apply::ApplyOutcome::Applied)
    );
    assert!(matches!(
        apply_agent_event(&mut live_agent, &duplicate),
        Err(ApplyError::DuplicateNeedTickCharge {
            need_kind: NeedKind::Hunger,
            tick: 4,
            ..
        })
    ));

    let mut log = EventLog::new();
    append_to_log(&mut log, first);
    append_to_log(&mut log, duplicate);
    assert_rebuild_agent_error(
        "duplicate_need_tick_charge_rejects_live_and_replay_001",
        &initial_agent_state,
        &log,
        "DuplicateNeedTickCharge",
    );
}

#[test]
fn malformed_elapsed_ticks_rejects_live_and_replay_001() {
    let initial_agent_state = agent_state();
    let mut forged = caused_event(
        "event_need_delta_malformed_elapsed_ticks",
        EventKind::NeedDeltaApplied,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    forged.payload = vec![
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new("need_kind", "hunger"),
        PayloadField::new("delta", "10"),
        PayloadField::new("elapsed_ticks", "two"),
        PayloadField::new("cause_kind", "action_effect"),
        PayloadField::new("cause_action_id", "wait"),
    ];

    let mut live_agent = initial_agent_state.clone();
    assert_eq!(
        apply_agent_event(&mut live_agent, &forged),
        Err(ApplyError::MalformedElapsedTicks("two".to_string()))
    );

    let mut log = EventLog::new();
    append_to_log(&mut log, forged);
    assert_rebuild_agent_error(
        "malformed_elapsed_ticks_rejects_live_and_replay_001",
        &initial_agent_state,
        &log,
        "MalformedElapsedTicks",
    );
}

#[test]
fn missing_intention_continued_current_step_rejects_live_and_replay_001() {
    let initial_agent_state = agent_state_with_active_intention_and_routine();
    let mut forged = caused_event(
        "event_intention_continued_missing_current_step",
        EventKind::IntentionContinued,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    forged.payload = vec![
        PayloadField::new("intention_id", "intention_breakfast"),
        PayloadField::new("status", "active"),
        PayloadField::new("reason", "continue routine"),
        PayloadField::new("progress_tick", "3"),
    ];

    let mut live_agent = initial_agent_state.clone();
    assert_eq!(
        apply_agent_event(&mut live_agent, &forged),
        Err(ApplyError::MissingPayload("current_step"))
    );

    let mut log = EventLog::new();
    append_to_log(&mut log, forged);
    assert_rebuild_agent_error(
        "missing_intention_continued_current_step_rejects_live_and_replay_001",
        &initial_agent_state,
        &log,
        "MissingPayload(\"current_step\")",
    );
}

#[test]
fn missing_routine_step_progress_tick_rejects_live_and_replay_001() {
    let initial_agent_state = agent_state_with_active_intention_and_routine();
    let mut forged = caused_event(
        "event_routine_step_started_missing_progress_tick",
        EventKind::RoutineStepStarted,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    forged.payload = vec![
        PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
        PayloadField::new("action_id", "check_container.pantry"),
    ];

    let mut live_agent = initial_agent_state.clone();
    assert_eq!(
        apply_agent_event(&mut live_agent, &forged),
        Err(ApplyError::MissingPayload("progress_tick"))
    );

    let mut log = EventLog::new();
    append_to_log(&mut log, forged);
    assert_rebuild_agent_error(
        "missing_routine_step_progress_tick_rejects_live_and_replay_001",
        &initial_agent_state,
        &log,
        "MissingPayload(\"progress_tick\")",
    );
}

#[test]
fn missing_role_assignment_access_open_rejects_live_and_replay_001() {
    let mut forged = caused_event(
        "event_role_notice_missing_access_open",
        EventKind::RoleAssignmentNoticeRecorded,
        0,
        vec![EventCause::Process("process_no_human_day".parse().unwrap())],
    );
    forged.payload = vec![
        PayloadField::new("schema_version", "event_schema_v1"),
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new(
            "workplace_id",
            WorkplaceId::new("workshop_tomas").unwrap().as_str(),
        ),
        PayloadField::new("place_id", PlaceId::new("shop_front").unwrap().as_str()),
    ];

    let mut projection = EpistemicProjection::new(manifest_id());
    assert_eq!(
        apply_epistemic_event(&mut projection, &forged),
        Err(EpistemicApplyError::MissingPayload("access_open"))
    );

    let mut log = EventLog::new();
    append_to_log(&mut log, forged);
    assert_rebuild_epistemic_error(&log, "MissingPayload(\"access_open\")");
}

#[test]
fn duplicate_duration_terminal_poisons_rebuild_001() {
    let initial_world = world_state();
    let initial_agent_state = agent_state();
    let mut log = EventLog::new();
    let start = append_to_log(
        &mut log,
        caused_event(
            "event_work_started_duplicate_terminal_gate",
            EventKind::WorkBlockStarted,
            0,
            vec![EventCause::Process("process_no_human_day".parse().unwrap())],
        ),
    );
    append_to_log(
        &mut log,
        caused_event(
            "event_work_completed_duplicate_terminal_gate",
            EventKind::WorkBlockCompleted,
            1,
            vec![EventCause::Event(start.event_id.clone())],
        ),
    );
    append_to_log(
        &mut log,
        caused_event(
            "event_work_failed_duplicate_terminal_gate",
            EventKind::WorkBlockFailed,
            2,
            vec![EventCause::Event(start.event_id.clone())],
        ),
    );

    let context = checksum_context("duplicate_duration_terminal_poisons_rebuild_001", &log);
    let rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&initial_world),
    );
    let replay = run_replay(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&initial_world),
        None,
        None,
    );

    assert!(
        rebuild
            .invariant_violations
            .iter()
            .any(|violation| violation.contains("duplicate_duration_terminal")),
        "{:?}",
        rebuild.invariant_violations
    );
    assert!(!replay.matches_expected);
}

#[test]
fn forged_payload_schema_version_rejected_for_every_materialized_agent_kind_001() {
    for (index, event_kind) in [
        EventKind::NeedThresholdCrossed,
        EventKind::SleepStarted,
        EventKind::SleepCompleted,
        EventKind::SleepInterrupted,
        EventKind::FoodServiceUsed,
        EventKind::EatFailed,
        EventKind::WorkBlockStarted,
        EventKind::WorkBlockCompleted,
        EventKind::WorkBlockFailed,
        EventKind::CandidateGoalsEvaluated,
        EventKind::ContinueRoutineProposed,
        EventKind::ContinueRoutineAccepted,
        EventKind::ContinueRoutineRejected,
    ]
    .into_iter()
    .enumerate()
    {
        assert_forged_agent_payload_version_rejected(
            event_kind,
            "payload_schema_version",
            &format!("event_forged_payload_schema_{index}"),
        );
    }
}

#[test]
fn forged_trace_and_diagnostic_schema_versions_are_rejected_for_materialized_agent_replay_001() {
    assert_forged_agent_payload_version_rejected(
        EventKind::DecisionTraceRecorded,
        "trace_schema_version",
        "event_forged_trace_schema",
    );
    assert_forged_agent_payload_version_rejected(
        EventKind::StuckDiagnosticRecorded,
        "diagnostic_schema_version",
        "event_forged_diagnostic_schema",
    );
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
    let mut bad_payload = caused_event(
        "event_bad_epistemic_payload_schema",
        EventKind::BeliefUpdated,
        0,
        vec![EventCause::Process(
            ProcessId::new("process_bad_payload").unwrap(),
        )],
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
        .take(11)
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
