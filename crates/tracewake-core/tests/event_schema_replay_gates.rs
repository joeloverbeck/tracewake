mod support;

use std::collections::BTreeSet;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    BlockerCategory, BlockerCode, DecisionTraceRecord, Intention, IntentionSource, IntentionStatus,
    NeedChangeCause, NeedKind, NeedState, ResponsibleLayer, RoutineExecution, RoutineFamily,
    RoutineStepStatus, StuckDiagnostic, StuckResultingStatus, TypedDiagnosticFields,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_holder_known_context_hash, compute_physical_checksum,
    ChecksumContext,
};
use tracewake_core::epistemics::{Channel, EpistemicProjection, KnowledgeContext, Stance};
use tracewake_core::events::apply::{
    apply_agent_event, apply_epistemic_event, apply_event, ApplyError, ApplyOutcome,
    EpistemicApplyError, AGENT_WORLD_NOOP_ALLOWLIST,
};
use tracewake_core::events::log::{EventLog, EventLogError};
use tracewake_core::events::{
    EventCause, EventEnvelope, EventEnvelopeParseError, EventKind, EventStream, PayloadField,
    RandomDrawRef,
};
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, CandidateGoalId, ContainerId, ContentManifestId, ContentVersion,
    ContradictionId, ControllerId, DecisionTraceId, EventId, FixtureId, IntentionId, ItemId,
    ObservationId, PlaceId, ProcessId, RoutineExecutionId, RoutineTemplateId, SchemaVersion,
    SemanticActionId, StuckDiagnosticId, ValidationReportId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::report::ReplayDivergenceFieldFamily;
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
};
use tracewake_core::time::SimTick;

const APPLY_RS: &str = include_str!("../src/events/apply.rs");

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
    seed.items_mut().insert(
        ItemId::new("loose_key_01").unwrap(),
        ItemState::new(
            ItemId::new("loose_key_01").unwrap(),
            Location::AtPlace(shop.clone()),
        ),
    );
    seed.places_mut()
        .get_mut(&shop)
        .expect("shop seed exists")
        .local_item_ids
        .insert(ItemId::new("loose_key_01").unwrap());

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
    checksum_context_at(
        fixture_id,
        log,
        log.events()
            .last()
            .map(|event| event.sim_tick)
            .unwrap_or(SimTick::ZERO),
    )
}

fn checksum_context_at(fixture_id: &str, log: &EventLog, sim_tick: SimTick) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new(fixture_id).unwrap(),
        content_version: content_version(),
        sim_tick,
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

fn process_cause() -> Vec<EventCause> {
    vec![EventCause::Process("process_apply_matrix".parse().unwrap())]
}

fn event_cause(event_id: &str) -> Vec<EventCause> {
    vec![EventCause::Event(EventId::new(event_id).unwrap())]
}

fn agent_event_with_payload(
    event_id: &str,
    kind: EventKind,
    sequence: u64,
    payload: Vec<PayloadField>,
) -> EventEnvelope {
    let mut event = caused_event(event_id, kind, sequence, process_cause());
    event.payload = payload;
    event
}

fn world_event_with_payload(
    event_id: &str,
    kind: EventKind,
    sequence: u64,
    payload: Vec<PayloadField>,
) -> EventEnvelope {
    let mut event = caused_event(event_id, kind, sequence, process_cause());
    event.payload = payload;
    event
}

fn epistemic_event_with_payload(
    event_id: &str,
    kind: EventKind,
    sequence: u64,
    payload: Vec<PayloadField>,
) -> EventEnvelope {
    let mut event = caused_event(event_id, kind, sequence, process_cause());
    event.payload = payload;
    event
}

fn belief_payload(
    belief_id: &str,
    stance: &str,
    channel: Option<&str>,
    confidence: u16,
    source_event_id: &str,
) -> Vec<PayloadField> {
    let mut payload = vec![
        PayloadField::new("schema_version", "event_schema_v1"),
        PayloadField::new("belief_id", belief_id),
        PayloadField::new("holder_actor_id", actor_id().as_str()),
        PayloadField::new(
            "proposition",
            "item_located_at_place|loose_key_01|shop_front",
        ),
        PayloadField::new("stance", stance),
        PayloadField::new("confidence", confidence.to_string()),
        PayloadField::new("source_event_id", source_event_id),
        PayloadField::new("acquired_tick", "11"),
    ];
    if let Some(channel) = channel {
        payload.push(PayloadField::new("channel", channel));
    }
    payload
}

fn observation_payload(
    observation_id: &str,
    channel: &str,
    confidence: u16,
    source_event_id: &str,
) -> Vec<PayloadField> {
    vec![
        PayloadField::new("schema_version", "event_schema_v1"),
        PayloadField::new("observation_id", observation_id),
        PayloadField::new("observer_actor_id", actor_id().as_str()),
        PayloadField::new("channel", channel),
        PayloadField::new("observed_tick", "12"),
        PayloadField::new("observer_place_id", "shop_front"),
        PayloadField::new("place_id", "shop_front"),
        PayloadField::new("confidence", confidence.to_string()),
        PayloadField::new("source_event_id", source_event_id),
        PayloadField::new("alternatives", "coin_stack_01,loose_key_01"),
    ]
}

fn contradiction_payload(
    contradiction_id: &str,
    belief_id: &str,
    observation_id: &str,
) -> Vec<PayloadField> {
    vec![
        PayloadField::new("schema_version", "event_schema_v1"),
        PayloadField::new("contradiction_id", contradiction_id),
        PayloadField::new("holder_actor_id", actor_id().as_str()),
        PayloadField::new("prior_expectation_belief_id", belief_id),
        PayloadField::new("contradicting_observation_id", observation_id),
        PayloadField::new(
            "expected_proposition",
            "item_located_in_container|loose_key_01|strongbox_tomas",
        ),
        PayloadField::new(
            "observed_proposition",
            "item_missing_from_expected_location|loose_key_01|in_container:strongbox_tomas",
        ),
        PayloadField::new("detected_tick", "13"),
    ]
}

fn intention_started_payload(
    intention_id: &str,
    source: &str,
    routine_template: Option<&str>,
    durability: u8,
    start_tick: u64,
) -> Vec<PayloadField> {
    let mut payload = vec![
        PayloadField::new("intention_id", intention_id),
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new("status", "active"),
        PayloadField::new("source", source),
        PayloadField::new("selected_goal_id", format!("goal_{intention_id}")),
        PayloadField::new("current_step", "inspect_visible_key"),
        PayloadField::new("durability_level", durability.to_string()),
        PayloadField::new("start_tick", start_tick.to_string()),
        PayloadField::new("trace_id", format!("trace_{intention_id}")),
    ];
    if let Some(routine_template) = routine_template {
        payload.push(PayloadField::new(
            "selected_routine_method",
            routine_template,
        ));
    }
    payload
}

fn transition_payload(intention_id: &str, status: &str, reason: &str) -> Vec<PayloadField> {
    vec![
        PayloadField::new("intention_id", intention_id),
        PayloadField::new("status", status),
        PayloadField::new("reason", reason),
    ]
}

fn active_intention<'a>(state: &'a AgentState, intention_id: &str) -> &'a Intention {
    state
        .intentions()
        .get(&IntentionId::new(intention_id).unwrap())
        .expect("intention exists")
}

fn routine_execution<'a>(state: &'a AgentState, execution_id: &str) -> &'a RoutineExecution {
    state
        .routine_executions()
        .get(&RoutineExecutionId::new(execution_id).unwrap())
        .expect("routine execution exists")
}

fn event_ids(values: &[&str]) -> Vec<EventId> {
    values
        .iter()
        .map(|value| EventId::new(*value).unwrap())
        .collect()
}

fn envelope_hex(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn encoded_vec(values: &[String]) -> String {
    values
        .iter()
        .map(|value| envelope_hex(value))
        .collect::<Vec<_>>()
        .join(";")
}

fn random_draw_wire(scope: &str, draw_id: &str, value: &str) -> String {
    [scope, draw_id, value]
        .into_iter()
        .map(envelope_hex)
        .collect::<Vec<_>>()
        .join(":")
}

fn append_ordered(events: Vec<EventEnvelope>) -> EventLog {
    let mut log = EventLog::new();
    for event in events {
        append_to_log(&mut log, event);
    }
    log
}

fn unrelated_observation_event(
    event_id: &str,
    observer_actor_id: &str,
    place_id: &str,
    tick: SimTick,
) -> EventEnvelope {
    let mut event = epistemic_event_with_payload(
        event_id,
        EventKind::ObservationRecorded,
        tick.value(),
        vec![
            PayloadField::new("schema_version", "event_schema_v1"),
            PayloadField::new("observation_id", format!("obs_{event_id}")),
            PayloadField::new("observer_actor_id", observer_actor_id),
            PayloadField::new("channel", "direct_sight"),
            PayloadField::new("observed_tick", tick.value().to_string()),
            PayloadField::new("observer_place_id", place_id),
            PayloadField::new("place_id", place_id),
            PayloadField::new("confidence", "777"),
            PayloadField::new("source_event_id", event_id),
        ],
    );
    event.actor_id = Some(ActorId::new(observer_actor_id).unwrap());
    event.place_id = Some(PlaceId::new(place_id).unwrap());
    event.sim_tick = tick;
    event
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
        Some("decision_outcome:continued")
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
fn replay_report_match_mismatch_pair_exposes_semantic_fingerprints() {
    let mut world = world_state();
    let mut agent_state = agent_state();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let mut log = EventLog::new();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();

    run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry,
        manifest_id(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "report_pair".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );

    let final_context = checksum_context("replay_report_match_mismatch_pair", &log);
    let replay_context =
        checksum_context_at("replay_report_match_mismatch_pair", &log, SimTick::ZERO);
    let expected_checksum = compute_physical_checksum(&world, &final_context).checksum;
    let expected_agent_checksum =
        compute_agent_state_checksum(&agent_state, &final_context).checksum;
    let expected_diagnostic_event_count = log
        .events()
        .iter()
        .filter(|event| event.stream != EventStream::World)
        .count();
    assert!(expected_diagnostic_event_count > 0);
    assert_ne!(
        expected_diagnostic_event_count,
        log.events().len() - expected_diagnostic_event_count
    );
    let matching = run_replay(
        &initial_world,
        &initial_agent_state,
        &log,
        &replay_context,
        Some(&world),
        Some(expected_checksum.clone()),
        Some(expected_agent_checksum.clone()),
    );

    assert!(matching.matches_expected);
    assert!(matching.agent_checksum_matches);
    assert_eq!(
        matching.diagnostic_event_count,
        expected_diagnostic_event_count
    );
    assert_eq!(matching.expected_checksum, Some(expected_checksum.clone()));
    assert_eq!(matching.final_checksum, expected_checksum);
    assert_eq!(
        matching.expected_agent_checksum,
        Some(expected_agent_checksum.clone())
    );
    assert_eq!(matching.final_agent_checksum, expected_agent_checksum);
    assert!(matching.state_diff.is_empty());
    assert_eq!(matching.first_divergence, None);

    let mut corrupted_items = world.items().clone();
    let unexpected_item_id = ItemId::new("unexpected_replay_report_item").unwrap();
    corrupted_items.insert(
        unexpected_item_id.clone(),
        ItemState::new(
            unexpected_item_id,
            Location::AtPlace(PlaceId::new("shop_front").unwrap()),
        ),
    );
    let corrupted_expected_world = PhysicalState::from_seed_parts(
        world.actors().clone(),
        world.places().clone(),
        world.doors().clone(),
        world.containers().clone(),
        corrupted_items,
        world.food_supplies().clone(),
        world.workplaces().clone(),
        world.sleep_affordances().clone(),
        *world.need_model(),
    );
    let corrupted_expected_checksum =
        compute_physical_checksum(&corrupted_expected_world, &final_context).checksum;
    let mismatching = run_replay(
        &initial_world,
        &initial_agent_state,
        &log,
        &replay_context,
        Some(&corrupted_expected_world),
        Some(corrupted_expected_checksum.clone()),
        Some(matching.final_agent_checksum.clone()),
    );

    assert!(!mismatching.matches_expected);
    assert!(mismatching.agent_checksum_matches);
    assert_eq!(
        mismatching.diagnostic_event_count,
        expected_diagnostic_event_count
    );
    assert_eq!(
        mismatching.expected_checksum,
        Some(corrupted_expected_checksum.clone())
    );
    assert_ne!(mismatching.final_checksum, corrupted_expected_checksum);
    assert_eq!(
        mismatching.expected_agent_checksum,
        Some(matching.final_agent_checksum)
    );
    assert!(!mismatching.state_diff.is_empty());
    let first_divergence = mismatching
        .first_divergence
        .expect("corrupt expected world reports first divergence");
    assert!(first_divergence.first_divergent_event_id.is_some());
    assert_eq!(
        first_divergence.field_family,
        ReplayDivergenceFieldFamily::Item
    );
}

#[test]
fn checksum_identity_distinguishes_location_routine_status_and_replay_fingerprints() {
    let context = checksum_context("checksum_identity_witness", &EventLog::new());
    let base_world = world_state();
    let base_physical_report = compute_physical_checksum(&base_world, &context);

    let equivalent_world = PhysicalSeed::from_state(&base_world).build();
    let equivalent_physical_report = compute_physical_checksum(&equivalent_world, &context);
    assert_eq!(
        base_physical_report.checksum,
        equivalent_physical_report.checksum
    );
    assert_eq!(
        base_physical_report.canonical_input,
        equivalent_physical_report.canonical_input
    );

    let mut relocated_seed = PhysicalSeed::from_state(&base_world);
    relocated_seed
        .items_mut()
        .get_mut(&ItemId::new("loose_key_01").unwrap())
        .expect("loose key exists")
        .location = Location::InContainer(ContainerId::new("strongbox_tomas").unwrap());
    let relocated_world = relocated_seed.build();
    let relocated_physical_report = compute_physical_checksum(&relocated_world, &context);
    assert_ne!(
        base_physical_report.checksum,
        relocated_physical_report.checksum
    );
    assert!(
        base_physical_report
            .canonical_input
            .iter()
            .any(|line| line.contains("item|id=loose_key_01") && line.contains("at_place:")),
        "{:?}",
        base_physical_report.canonical_input
    );
    assert!(
        relocated_physical_report
            .canonical_input
            .iter()
            .any(|line| line.contains("item|id=loose_key_01") && line.contains("in_container:")),
        "{:?}",
        relocated_physical_report.canonical_input
    );

    let base_agent = agent_state_with_active_intention_and_routine();
    let mut in_progress_seed = AgentSeed::from_state(&base_agent);
    in_progress_seed
        .routine_executions_mut()
        .get_mut(&RoutineExecutionId::new("routine_exec_breakfast").unwrap())
        .expect("routine exists")
        .step_status = RoutineStepStatus::InProgress;
    let in_progress_agent = in_progress_seed.build();
    let mut completed_seed = AgentSeed::from_state(&base_agent);
    completed_seed
        .routine_executions_mut()
        .get_mut(&RoutineExecutionId::new("routine_exec_breakfast").unwrap())
        .expect("routine exists")
        .step_status = RoutineStepStatus::Completed;
    let completed_agent = completed_seed.build();
    let in_progress_agent_report = compute_agent_state_checksum(&in_progress_agent, &context);
    let completed_agent_report = compute_agent_state_checksum(&completed_agent, &context);
    assert_ne!(
        in_progress_agent_report.checksum,
        completed_agent_report.checksum
    );
    assert!(
        in_progress_agent_report
            .canonical_input
            .iter()
            .any(|line| line.contains("routine_execution|") && line.contains("status=in_progress")),
        "{:?}",
        in_progress_agent_report.canonical_input
    );
    assert!(
        completed_agent_report
            .canonical_input
            .iter()
            .any(|line| line.contains("routine_execution|") && line.contains("status=completed")),
        "{:?}",
        completed_agent_report.canonical_input
    );

    let replay = run_replay(
        &base_world,
        &in_progress_agent,
        &EventLog::new(),
        &context,
        Some(&base_world),
        Some(base_physical_report.checksum.clone()),
        Some(in_progress_agent_report.checksum.clone()),
    );
    assert!(replay.matches_expected);
    assert_eq!(
        replay
            .expected_agent_checksum
            .as_ref()
            .map(|checksum| checksum.as_str()),
        Some(replay.final_agent_checksum.as_str())
    );
    assert!(replay.final_agent_checksum.as_str().starts_with("twa1-"));

    let corrupted_replay = run_replay(
        &base_world,
        &in_progress_agent,
        &EventLog::new(),
        &context,
        Some(&relocated_world),
        Some(relocated_physical_report.checksum),
        Some(in_progress_agent_report.checksum),
    );
    assert!(!corrupted_replay.matches_expected);
    assert_ne!(
        corrupted_replay.expected_checksum,
        Some(corrupted_replay.final_checksum)
    );
    assert!(!corrupted_replay.state_diff.is_empty());
    assert_eq!(
        corrupted_replay
            .first_divergence
            .expect("relocated expected state reports first divergence")
            .field_family,
        ReplayDivergenceFieldFamily::Item
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

#[test]
fn deterministic_rebuild_context_hash_uses_causal_and_latest_witnesses() {
    let mut world = world_state();
    let mut agent_state = agent_state();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let mut log = EventLog::new();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();

    run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry,
        manifest_id(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "deterministic_rebuild".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );
    let context = checksum_context("deterministic_rebuild_context_hash", &log);
    let first = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&world),
    );
    let second = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap(),
        &context,
        Some(&world),
    );
    assert_eq!(first, second);
    assert!(first.decision_context_hash_failures.is_empty());
    assert!(first.invariant_violations.is_empty());
    assert!(first.state_diff.is_empty());

    let trace_index = log
        .events()
        .iter()
        .position(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .expect("no-human day records a decision trace");
    let trace = &log.events()[trace_index];
    let ordinary_event_id = payload_value(trace, "ordinary_event_id")
        .expect("trace records ordinary event id")
        .to_string();
    let recorded_trace = DecisionTraceRecord::deserialize_canonical(
        payload_value(trace, "trace_canonical")
            .expect("trace canonical exists")
            .as_bytes(),
    )
    .expect("trace canonical parses");
    assert!(
        recorded_trace.actor_known_context_hash.is_some(),
        "fixture must prove non-empty decision-context hash rebuilding"
    );

    let mut cause_only_events = log.events().to_vec();
    let cause_only_trace = &mut cause_only_events[trace_index];
    cause_only_trace
        .payload
        .retain(|field| field.key != "ordinary_event_id");
    cause_only_trace.causes = vec![EventCause::Event(EventId::new(&ordinary_event_id).unwrap())];
    let cause_only_log = append_ordered(cause_only_events);
    let cause_only_rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &cause_only_log,
        &checksum_context("deterministic_rebuild_cause_only", &cause_only_log),
        Some(&world),
    );
    assert!(
        cause_only_rebuild.decision_context_hash_failures.is_empty(),
        "{:?}",
        cause_only_rebuild.decision_context_hash_failures
    );

    let mut missing_cause_events = cause_only_log.events().to_vec();
    missing_cause_events[trace_index].causes.clear();
    let missing_cause_log = append_ordered(missing_cause_events);
    let missing_cause_rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &missing_cause_log,
        &checksum_context("deterministic_rebuild_missing_cause", &missing_cause_log),
        Some(&world),
    );
    assert!(missing_cause_rebuild
        .decision_context_hash_failures
        .iter()
        .any(|failure| failure.issue == "missing_ordinary_event_id"));

    let mut unrelated_events = log.events().to_vec();
    let ordinary_index = unrelated_events
        .iter()
        .position(|event| event.event_id.as_str() == ordinary_event_id)
        .expect("ordinary event exists");
    let decision_tick = recorded_trace.window_start_tick;
    unrelated_events.insert(
        ordinary_index,
        unrelated_observation_event(
            "event_unrelated_wrong_actor_perception",
            "actor_unrelated",
            "shop_front",
            decision_tick,
        ),
    );
    unrelated_events.insert(
        ordinary_index + 1,
        unrelated_observation_event(
            "event_unrelated_wrong_place_perception",
            actor_id().as_str(),
            "back_room",
            decision_tick,
        ),
    );
    unrelated_events.insert(
        ordinary_index + 2,
        unrelated_observation_event(
            "event_unrelated_wrong_tick_perception",
            actor_id().as_str(),
            "shop_front",
            SimTick::new(decision_tick.value() + 99),
        ),
    );
    let mut unrelated_need = agent_event_with_payload(
        "event_unrelated_need_wrong_actor",
        EventKind::NeedDeltaApplied,
        decision_tick.value() + 100,
        vec![
            PayloadField::new("actor_id", "actor_unrelated"),
            PayloadField::new("need_kind", "hunger"),
            PayloadField::new("delta", "1"),
            PayloadField::new("cause_kind", "fixture_initial"),
        ],
    );
    unrelated_need.actor_id = Some(ActorId::new("actor_unrelated").unwrap());
    unrelated_events.insert(ordinary_index + 3, unrelated_need);
    let unrelated_log = append_ordered(unrelated_events);
    let unrelated_rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &unrelated_log,
        &checksum_context("deterministic_rebuild_unrelated", &unrelated_log),
        Some(&world),
    );
    assert!(
        unrelated_rebuild.decision_context_hash_failures.is_empty(),
        "{:?}",
        unrelated_rebuild.decision_context_hash_failures
    );

    let mut payload_only_need_events = log.events().to_vec();
    let ordinary_index = payload_only_need_events
        .iter()
        .position(|event| event.event_id.as_str() == ordinary_event_id)
        .expect("ordinary event exists");
    let trace_index = payload_only_need_events
        .iter()
        .position(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .expect("trace event exists");
    let mut payload_only_need = agent_event_with_payload(
        "event_relevant_payload_only_need",
        EventKind::NeedDeltaApplied,
        decision_tick.value() + 101,
        vec![
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("need_kind", "hunger"),
            PayloadField::new("delta", "0"),
            PayloadField::new("cause_kind", "fixture_initial"),
        ],
    );
    payload_only_need.actor_id = None;
    payload_only_need_events.insert(ordinary_index, payload_only_need);

    let shifted_trace_index = if trace_index >= ordinary_index {
        trace_index + 1
    } else {
        trace_index
    };
    let trace_canonical = payload_only_need_events[shifted_trace_index]
        .payload
        .iter_mut()
        .find(|field| field.key == "trace_canonical")
        .expect("trace canonical exists");
    let mut expected_trace =
        DecisionTraceRecord::deserialize_canonical(trace_canonical.value.as_bytes())
            .expect("trace canonical parses");
    expected_trace.actor_known_inputs.push(
        "agent_needs_present=remembered_belief:agent_state:needs_present|source_class=memory|explicit_unknown=false|source_events=event_relevant_payload_only_need"
            .to_string(),
    );
    expected_trace.actor_known_context_hash =
        Some(compute_holder_known_context_hash(expected_trace.actor_known_inputs.clone()).hash);
    trace_canonical.value = expected_trace.serialize_canonical();

    let payload_only_need_log = append_ordered(payload_only_need_events);
    let payload_only_need_rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &payload_only_need_log,
        &checksum_context(
            "deterministic_rebuild_payload_only_need",
            &payload_only_need_log,
        ),
        Some(&world),
    );
    assert!(
        payload_only_need_rebuild
            .decision_context_hash_failures
            .is_empty(),
        "{:?}",
        payload_only_need_rebuild.decision_context_hash_failures
    );
}

#[test]
fn envelope_identity_round_trips_stream_phase_source_cause_and_random_draws() {
    let controller_id = ControllerId::new("controller_replay_audit").unwrap();
    let validation_report_id = ValidationReportId::new("validation_report_replay_audit").unwrap();
    let random_draw = RandomDrawRef::new("rng:phase3a", "draw:987", "1844674407370955161");

    let mut controller = EventEnvelope::new_v1(
        EventId::new("event_controller_envelope_identity").unwrap(),
        EventKind::ControllerAttached,
        99,
        99,
        SimTick::new(30),
        OrderingKey::new(
            SimTick::new(30),
            SchedulePhase::DeferredProcess,
            SchedulerSourceId::Controller(controller_id.clone()),
            ProposalSequence::new(17),
            ActionId::new("debug.bind").unwrap(),
            vec![
                "actor_tomas".to_string(),
                "controller_replay_audit".to_string(),
            ],
            "controller:deferred:17",
        ),
        manifest_id(),
    );
    controller.actor_id = Some(actor_id());
    controller.participants = vec![
        actor_id().as_str().to_string(),
        controller_id.as_str().to_string(),
    ];
    controller.random_draws = vec![random_draw.clone()];

    let mut diagnostic = EventEnvelope::new_v1(
        EventId::new("event_validation_cause_envelope_identity").unwrap(),
        EventKind::ActionRejected,
        99,
        99,
        SimTick::new(31),
        OrderingKey::new(
            SimTick::new(31),
            SchedulePhase::Replay,
            SchedulerSourceId::Controller(controller_id.clone()),
            ProposalSequence::new(18),
            ActionId::new("open").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "controller:replay:18",
        ),
        manifest_id(),
    );
    diagnostic.causes = vec![EventCause::ValidationReport(validation_report_id.clone())];
    diagnostic.validation_report_id = Some(validation_report_id.clone());
    diagnostic.effects_summary = "validation report rejected action".to_string();

    let replay_debug = EventEnvelope::new_v1(
        EventId::new("event_replay_debug_envelope_identity").unwrap(),
        EventKind::ReplayProjectionRebuilt,
        99,
        99,
        SimTick::new(32),
        OrderingKey::new(
            SimTick::new(32),
            SchedulePhase::Replay,
            SchedulerSourceId::Process(ProcessId::new("process_replay_audit").unwrap()),
            ProposalSequence::new(19),
            ActionId::new("replay.projection").unwrap(),
            vec!["world".to_string()],
            "replay:debug:19",
        ),
        manifest_id(),
    );

    let mut log = EventLog::new();
    let controller = append_to_log(&mut log, controller);
    let diagnostic = append_to_log(&mut log, diagnostic);
    let replay_debug = append_to_log(&mut log, replay_debug);
    let canonical = log.serialize_canonical();
    let first_round_trip = EventLog::deserialize_canonical(&canonical).unwrap();
    let duplicate_round_trip =
        EventLog::deserialize_canonical(&first_round_trip.serialize_canonical()).unwrap();

    assert_eq!(first_round_trip.serialize_canonical(), canonical);
    assert_eq!(duplicate_round_trip.serialize_canonical(), canonical);
    assert_eq!(
        first_round_trip.events(),
        &[controller, diagnostic, replay_debug]
    );

    let round_tripped_controller = &first_round_trip.events()[0];
    assert_eq!(round_tripped_controller.stream, EventStream::Controller);
    assert_eq!(round_tripped_controller.stream_position, 0);
    assert_eq!(
        round_tripped_controller.ordering_key.phase,
        SchedulePhase::DeferredProcess
    );
    assert_eq!(
        round_tripped_controller.ordering_key.source_id,
        SchedulerSourceId::Controller(controller_id.clone())
    );
    assert_eq!(round_tripped_controller.random_draws, vec![random_draw]);

    let round_tripped_diagnostic = &first_round_trip.events()[1];
    assert_eq!(
        round_tripped_diagnostic.ordering_key.phase,
        SchedulePhase::Replay
    );
    assert_eq!(
        round_tripped_diagnostic.ordering_key.source_id,
        SchedulerSourceId::Controller(controller_id)
    );
    assert_eq!(
        round_tripped_diagnostic.causes,
        vec![EventCause::ValidationReport(validation_report_id.clone())]
    );
    assert_eq!(
        round_tripped_diagnostic.validation_report_id.as_ref(),
        Some(&validation_report_id)
    );

    let round_tripped_replay_debug = &first_round_trip.events()[2];
    assert_eq!(round_tripped_replay_debug.stream, EventStream::ReplayDebug);
    assert_eq!(round_tripped_replay_debug.stream_position, 0);
    assert_eq!(
        round_tripped_replay_debug.ordering_key.phase,
        SchedulePhase::Replay
    );
}

#[test]
fn random_draw_deserialization_rejects_malformed_and_bad_hex_records() {
    let mut envelope = event(
        "event_random_draw_corruption_matrix",
        EventKind::ActorWaited,
        0,
    );
    envelope.random_draws = vec![RandomDrawRef::new(
        "rng:corruption",
        "draw:nonzero",
        "987654321",
    )];
    let canonical = String::from_utf8(envelope.serialize_canonical()).unwrap();
    let good_random_draws = encoded_vec(&[random_draw_wire(
        "rng:corruption",
        "draw:nonzero",
        "987654321",
    )]);
    assert!(
        canonical.contains(&format!("random_draws={good_random_draws}")),
        "{canonical}"
    );

    let malformed_tuple = encoded_vec(&[format!(
        "{}:{}",
        envelope_hex("rng:corruption"),
        envelope_hex("draw:nonzero")
    )]);
    let malformed = canonical.replace(
        &format!("random_draws={good_random_draws}"),
        &format!("random_draws={malformed_tuple}"),
    );
    assert_eq!(
        EventEnvelope::deserialize_canonical(malformed.as_bytes()),
        Err(EventEnvelopeParseError::InvalidTuple)
    );

    let bad_inner_hex = encoded_vec(&[format!(
        "zz:{}:{}",
        envelope_hex("draw:nonzero"),
        envelope_hex("987654321")
    )]);
    let bad_hex = canonical.replace(
        &format!("random_draws={good_random_draws}"),
        &format!("random_draws={bad_inner_hex}"),
    );
    assert_eq!(
        EventEnvelope::deserialize_canonical(bad_hex.as_bytes()),
        Err(EventEnvelopeParseError::BadHex)
    );
}

#[test]
fn epistemic_apply_matrix_preserves_fields_and_rejects_unknown_tokens() {
    let context = KnowledgeContext::embodied(actor_id(), SimTick::new(20));
    let mut projection = EpistemicProjection::new(manifest_id());
    let mut log = EventLog::new();
    for (index, (stance, channel)) in [
        ("believes_false", Some("reading_placeholder_schema_only")),
        ("expects_true", Some("touch_or_search")),
        ("doubts", Some("absence_marker")),
        ("unknown_or_unresolved", None),
    ]
    .into_iter()
    .enumerate()
    {
        let source_event_id = format!("event_epistemic_source_{index}");
        let belief_id = format!("belief_apply_matrix_{index}");
        let event = epistemic_event_with_payload(
            &format!("event_belief_apply_matrix_{index}"),
            EventKind::BeliefUpdated,
            index as u64,
            belief_payload(
                &belief_id,
                stance,
                channel,
                731 + index as u16,
                &source_event_id,
            ),
        );
        append_to_log(&mut log, event.clone());
        assert_eq!(
            apply_epistemic_event(&mut projection, &event),
            Ok(ApplyOutcome::Applied)
        );
        let belief = projection
            .beliefs_for_context(&context)
            .into_iter()
            .find(|belief| belief.belief_id() == &BeliefId::new(&belief_id).unwrap())
            .expect("belief applied through projection");
        assert_eq!(
            belief.source(),
            &tracewake_core::epistemics::SourceRef::Event(EventId::new(&source_event_id).unwrap())
        );
        assert_eq!(
            belief.confidence().serialize_canonical(),
            format!("0{}", 731 + index as u16)
        );
        match stance {
            "believes_false" => assert_eq!(belief.stance(), Stance::BelievesFalse),
            "expects_true" => assert_eq!(belief.stance(), Stance::ExpectsTrue),
            "doubts" => assert_eq!(belief.stance(), Stance::Doubts),
            "unknown_or_unresolved" => assert_eq!(belief.stance(), Stance::UnknownOrUnresolved),
            _ => unreachable!(),
        }
        match channel {
            Some("reading_placeholder_schema_only") => {
                assert_eq!(
                    belief.channel(),
                    Some(Channel::ReadingPlaceholderSchemaOnly)
                );
            }
            Some("touch_or_search") => assert_eq!(belief.channel(), Some(Channel::TouchOrSearch)),
            Some("absence_marker") => assert_eq!(belief.channel(), Some(Channel::AbsenceMarker)),
            None => assert_eq!(belief.channel(), None),
            _ => unreachable!(),
        }
    }

    let bad_stance = epistemic_event_with_payload(
        "event_belief_bad_stance_apply_matrix",
        EventKind::BeliefUpdated,
        10,
        belief_payload(
            "belief_bad_stance_apply_matrix",
            "not_a_stance",
            None,
            800,
            "event_epistemic_source_bad_stance",
        ),
    );
    assert!(matches!(
        apply_epistemic_event(&mut projection, &bad_stance),
        Err(EpistemicApplyError::BadPayload { key: "stance", .. })
    ));

    let bad_channel = epistemic_event_with_payload(
        "event_observation_bad_channel_apply_matrix",
        EventKind::ObservationRecorded,
        11,
        observation_payload(
            "observation_bad_channel_apply_matrix",
            "not_a_channel",
            812,
            "event_epistemic_source_bad_channel",
        ),
    );
    assert!(matches!(
        apply_epistemic_event(&mut projection, &bad_channel),
        Err(EpistemicApplyError::BadPayload { key: "channel", .. })
    ));

    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let context = checksum_context("epistemic_apply_matrix_preserves_fields", &replayed_log);
    let replay = run_replay(
        &world_state(),
        &agent_state(),
        &replayed_log,
        &context,
        Some(&world_state()),
        None,
        None,
    );
    assert!(replay.epistemic_application_errors.is_empty());
    assert_ne!(
        replay.final_epistemic_checksum,
        EpistemicProjection::new(manifest_id())
            .compute_checksum()
            .checksum
    );
}

#[test]
fn starting_observation_and_contradiction_events_survive_replay_with_sources() {
    let mut projection = EpistemicProjection::new(manifest_id());
    let mut log = EventLog::new();
    let starting = epistemic_event_with_payload(
        "event_starting_food_belief_apply_matrix",
        EventKind::StartingBeliefRecorded,
        0,
        vec![
            PayloadField::new("schema_version", "event_schema_v1"),
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("belief_kind", "household_food_source"),
            PayloadField::new("subject_id", "food_visible_shelf"),
            PayloadField::new("value", "place:shop_front"),
        ],
    );
    let observation = epistemic_event_with_payload(
        "event_observation_apply_matrix",
        EventKind::ObservationRecorded,
        1,
        observation_payload(
            "observation_apply_matrix",
            "reading_placeholder_schema_only",
            877,
            "event_starting_food_belief_apply_matrix",
        ),
    );
    let belief = epistemic_event_with_payload(
        "event_expectation_apply_matrix",
        EventKind::BeliefUpdated,
        2,
        belief_payload(
            "belief_expectation_apply_matrix",
            "expects_true",
            Some("touch_or_search"),
            866,
            "event_observation_apply_matrix",
        ),
    );
    let contradiction = epistemic_event_with_payload(
        "event_contradiction_apply_matrix",
        EventKind::ExpectationContradicted,
        3,
        contradiction_payload(
            "contradiction_apply_matrix",
            "belief_expectation_apply_matrix",
            "observation_apply_matrix",
        ),
    );

    for event in [
        starting.clone(),
        observation.clone(),
        belief.clone(),
        contradiction.clone(),
    ] {
        append_to_log(&mut log, event.clone());
        assert_eq!(
            apply_epistemic_event(&mut projection, &event),
            Ok(ApplyOutcome::Applied)
        );
    }

    let context = KnowledgeContext::embodied(actor_id(), SimTick::new(20));
    let observations = projection.observations_for_context(&context);
    assert!(observations.iter().any(|observation| {
        observation.observation_id() == &ObservationId::new("observation_apply_matrix").unwrap()
            && observation.source()
                == &tracewake_core::epistemics::SourceRef::Event(
                    EventId::new("event_starting_food_belief_apply_matrix").unwrap(),
                )
            && observation.channel() == Channel::ReadingPlaceholderSchemaOnly
            && observation.alternatives().contains("loose_key_01")
    }));
    let contradictions = projection.contradictions_for_context(&context);
    assert!(contradictions.iter().any(|contradiction| {
        contradiction.contradiction_id()
            == &ContradictionId::new("contradiction_apply_matrix").unwrap()
            && contradiction.prior_expectation_belief_id()
                == &BeliefId::new("belief_expectation_apply_matrix").unwrap()
            && contradiction.contradicting_observation_id()
                == &ObservationId::new("observation_apply_matrix").unwrap()
    }));

    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let context = checksum_context("starting_observation_contradiction_sources", &replayed_log);
    let replay = run_replay(
        &world_state(),
        &agent_state(),
        &replayed_log,
        &context,
        Some(&world_state()),
        None,
        None,
    );
    assert!(replay.epistemic_application_errors.is_empty());
    assert_ne!(
        replay.final_epistemic_checksum,
        EpistemicProjection::new(manifest_id())
            .compute_checksum()
            .checksum
    );
}

#[test]
fn belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay() {
    let mut projection = EpistemicProjection::new(manifest_id());
    let mut log = EventLog::new();
    let observation = epistemic_event_with_payload(
        "event_belief_link_observation",
        EventKind::ObservationRecorded,
        0,
        observation_payload(
            "observation_belief_link",
            "touch_or_search",
            905,
            "event_search_container",
        ),
    );
    let contradiction = epistemic_event_with_payload(
        "event_belief_link_contradiction",
        EventKind::ExpectationContradicted,
        1,
        contradiction_payload(
            "contradiction_belief_link",
            "belief_frontier_and_links",
            "observation_belief_link",
        ),
    );
    let mut belief_payload = belief_payload(
        "belief_frontier_and_links",
        "expects_true",
        Some("touch_or_search"),
        940,
        "event_belief_link_observation",
    );
    belief_payload.push(PayloadField::new("last_verified_tick", "12"));
    belief_payload.push(PayloadField::new("stale_after_tick", "17"));
    belief_payload.push(PayloadField::new(
        "observation_ids",
        "observation_belief_link",
    ));
    belief_payload.push(PayloadField::new(
        "contradiction_ids",
        "contradiction_belief_link",
    ));
    let belief = epistemic_event_with_payload(
        "event_belief_frontier_and_links",
        EventKind::BeliefUpdated,
        2,
        belief_payload,
    );

    for event in [observation.clone(), contradiction.clone(), belief.clone()] {
        append_to_log(&mut log, event.clone());
        assert_eq!(
            apply_epistemic_event(&mut projection, &event),
            Ok(ApplyOutcome::Applied)
        );
    }

    let embodied_before_frontier = KnowledgeContext::embodied(actor_id(), SimTick::new(16));
    let belief_before = projection
        .beliefs_for_context(&embodied_before_frontier)
        .into_iter()
        .find(|belief| belief.belief_id() == &BeliefId::new("belief_frontier_and_links").unwrap())
        .expect("belief visible to holder before frontier");
    assert_eq!(belief_before.stale_after_tick(), Some(SimTick::new(17)));
    assert!(embodied_before_frontier.current_tick() < belief_before.stale_after_tick().unwrap());

    let embodied_at_frontier = KnowledgeContext::embodied(actor_id(), SimTick::new(17));
    let belief_at = projection
        .beliefs_for_context(&embodied_at_frontier)
        .into_iter()
        .find(|belief| belief.belief_id() == &BeliefId::new("belief_frontier_and_links").unwrap())
        .expect("belief visible to holder at frontier");
    assert_eq!(belief_at.stale_after_tick(), Some(SimTick::new(17)));

    let embodied_after_frontier = KnowledgeContext::embodied(actor_id(), SimTick::new(18));
    let belief_after = projection
        .beliefs_for_context(&embodied_after_frontier)
        .into_iter()
        .find(|belief| belief.belief_id() == &BeliefId::new("belief_frontier_and_links").unwrap())
        .expect("belief visible to holder after frontier");
    assert_eq!(belief_after.stale_after_tick(), Some(SimTick::new(17)));
    assert!(belief_after.stale_after_tick().unwrap() < embodied_after_frontier.current_tick());

    let observation_id = ObservationId::new("observation_belief_link").unwrap();
    let contradiction_id = ContradictionId::new("contradiction_belief_link").unwrap();
    assert!(belief_after.observation_ids().contains(&observation_id));
    assert!(belief_after.contradiction_ids().contains(&contradiction_id));

    let checksum = projection.compute_checksum();
    let belief_line = checksum
        .canonical_input
        .iter()
        .find(|line| line.starts_with("belief|id=belief_frontier_and_links|"))
        .expect("belief contributes canonical checksum evidence");
    assert!(belief_line.contains("|stale_after=17|"));
    assert!(belief_line.contains("|observations=observation_belief_link|"));
    assert!(belief_line.contains("|contradictions=contradiction_belief_link"));

    let debug_view = projection.debug_beliefs_view(actor_id());
    let debug_belief = debug_view
        .beliefs
        .iter()
        .find(|belief| belief.belief_id == "belief_frontier_and_links")
        .expect("debug belief view exposes witness chain");
    assert_eq!(debug_belief.stale_after_tick, Some(17));
    assert_eq!(debug_belief.observation_ids, ["observation_belief_link"]);
    assert_eq!(
        debug_belief.contradiction_ids,
        ["contradiction_belief_link"]
    );

    let other_actor_context =
        KnowledgeContext::embodied(ActorId::new("actor_elena").unwrap(), SimTick::new(18));
    assert!(projection
        .beliefs_for_context(&other_actor_context)
        .into_iter()
        .all(|belief| belief.belief_id() != &BeliefId::new("belief_frontier_and_links").unwrap()));

    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let mut replayed_projection = EpistemicProjection::new(manifest_id());
    for event in replayed_log.events() {
        apply_epistemic_event(&mut replayed_projection, event).unwrap();
    }
    assert_eq!(
        replayed_projection.compute_checksum().canonical_input,
        checksum.canonical_input
    );
    assert_eq!(
        replayed_projection.compute_checksum().checksum,
        checksum.checksum
    );
}

#[test]
fn observation_confidence_debug_evidence_crosses_low_boundary_and_replays() {
    let mut projection = EpistemicProjection::new(manifest_id());
    let mut log = EventLog::new();
    for (sequence, (observation_id, confidence)) in [
        ("observation_confidence_low_250", 250_u16),
        ("observation_confidence_low_boundary_350", 350),
        ("observation_confidence_standard_boundary_351", 351),
        ("observation_confidence_standard_875", 875),
    ]
    .into_iter()
    .enumerate()
    {
        let event = epistemic_event_with_payload(
            &format!("event_{observation_id}"),
            EventKind::ObservationRecorded,
            sequence as u64,
            observation_payload(
                observation_id,
                "direct_sight",
                confidence,
                "event_confidence_source",
            ),
        );
        append_to_log(&mut log, event.clone());
        assert_eq!(
            apply_epistemic_event(&mut projection, &event),
            Ok(ApplyOutcome::Applied)
        );
    }

    let debug_view = projection.debug_observations_view(actor_id());
    let debug_entries = debug_view
        .observations
        .iter()
        .map(|entry| {
            (
                entry.observation_id.as_str(),
                entry.confidence_parts_per_thousand,
                entry.confidence_class.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        debug_entries,
        [
            ("observation_confidence_low_250", 250, "low"),
            ("observation_confidence_low_boundary_350", 350, "low"),
            ("observation_confidence_standard_875", 875, "standard",),
            (
                "observation_confidence_standard_boundary_351",
                351,
                "standard",
            ),
        ]
    );

    let invalid = epistemic_event_with_payload(
        "event_observation_confidence_invalid",
        EventKind::ObservationRecorded,
        99,
        observation_payload(
            "observation_confidence_invalid",
            "direct_sight",
            1001,
            "event_confidence_source",
        ),
    );
    assert!(matches!(
        apply_epistemic_event(&mut projection, &invalid),
        Err(EpistemicApplyError::BadPayload {
            key: "confidence",
            ..
        })
    ));

    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let mut replayed_projection = EpistemicProjection::new(manifest_id());
    for event in replayed_log.events() {
        apply_epistemic_event(&mut replayed_projection, event).unwrap();
    }
    let replayed_debug_view = replayed_projection.debug_observations_view(actor_id());
    let replayed_debug_entries = replayed_debug_view
        .observations
        .iter()
        .map(|entry| {
            (
                entry.observation_id.as_str(),
                entry.confidence_parts_per_thousand,
                entry.confidence_class.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(replayed_debug_entries, debug_entries);
    assert_eq!(
        replayed_projection.compute_checksum().checksum,
        projection.compute_checksum().checksum
    );
}

#[test]
fn agent_apply_matrix_observes_parser_arms_transitions_and_causality() {
    let mut state = agent_state_with_active_intention_and_routine();
    let need = agent_event_with_payload(
        "event_need_delta_safety_routine_effect_matrix",
        EventKind::NeedDeltaApplied,
        0,
        vec![
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("need_kind", "safety"),
            PayloadField::new("delta", "37"),
            PayloadField::new("elapsed_ticks", "3"),
            PayloadField::new("cause_kind", "routine_effect"),
            PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
        ],
    );
    assert_eq!(
        apply_agent_event(&mut state, &need),
        Ok(ApplyOutcome::Applied)
    );
    let safety = state
        .needs_by_actor()
        .get(&actor_id())
        .and_then(|needs| needs.get(&NeedKind::Safety))
        .expect("safety need exists");
    assert_eq!(safety.value(), 137);
    assert!(matches!(
        safety.last_change_cause(),
        NeedChangeCause::RoutineEffect(id) if id.as_str() == "routine_exec_breakfast"
    ));

    for (index, (source, routine_template)) in [
        ("need_pressure", Some("routine_need_pressure")),
        ("routine_duty", Some("routine_duty_template")),
        ("safety_interruption", None),
        (
            "fixture_routine_assignment",
            Some("routine_fixture_template"),
        ),
        ("fallback", None),
    ]
    .into_iter()
    .enumerate()
    {
        let intention_id = format!("intention_source_matrix_{index}");
        let event = agent_event_with_payload(
            &format!("event_intention_source_matrix_{index}"),
            EventKind::IntentionStarted,
            index as u64 + 1,
            intention_started_payload(
                &intention_id,
                source,
                routine_template,
                5 + index as u8,
                20 + index as u64,
            ),
        );
        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );
        let intention = active_intention(&state, &intention_id);
        assert_eq!(intention.status, IntentionStatus::Active);
        assert_eq!(intention.durability_level, 5 + index as u8);
        assert_eq!(intention.start_tick, SimTick::new(20 + index as u64));
        match (source, intention.source) {
            ("need_pressure", IntentionSource::NeedPressure)
            | ("routine_duty", IntentionSource::RoutineDuty)
            | ("safety_interruption", IntentionSource::SafetyInterruption)
            | ("fixture_routine_assignment", IntentionSource::FixtureRoutineAssignment)
            | ("fallback", IntentionSource::Fallback) => {}
            _ => panic!("source {source} parsed as {:?}", intention.source),
        }
        assert_eq!(
            intention
                .selected_routine_method
                .as_ref()
                .map(|id| id.as_str()),
            routine_template
        );
    }

    let bad_source = agent_event_with_payload(
        "event_intention_bad_source_matrix",
        EventKind::IntentionStarted,
        8,
        intention_started_payload("intention_bad_source_matrix", "not_a_source", None, 6, 30),
    );
    assert!(matches!(
        apply_agent_event(&mut state, &bad_source),
        Err(ApplyError::BadPayload { key: "source", .. })
    ));

    for (index, (kind, status, expected)) in [
        (
            EventKind::IntentionSuspended,
            "suspended",
            IntentionStatus::Suspended,
        ),
        (
            EventKind::IntentionCompleted,
            "completed",
            IntentionStatus::Completed,
        ),
        (
            EventKind::IntentionFailed,
            "failed",
            IntentionStatus::Failed,
        ),
        (
            EventKind::IntentionAbandoned,
            "abandoned",
            IntentionStatus::Abandoned,
        ),
        (
            EventKind::IntentionInterrupted,
            "interrupted",
            IntentionStatus::Interrupted,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let mut transition_state = agent_state_with_active_intention_and_routine();
        let event = agent_event_with_payload(
            &format!("event_intention_transition_matrix_{index}"),
            kind,
            20 + index as u64,
            transition_payload("intention_breakfast", status, "matrix transition"),
        );
        assert_eq!(
            apply_agent_event(&mut transition_state, &event),
            Ok(ApplyOutcome::Applied)
        );
        let intention = active_intention(&transition_state, "intention_breakfast");
        assert_eq!(intention.status, expected);
        assert_eq!(
            intention.status_reason.as_deref(),
            Some("matrix transition")
        );
    }

    let mut resumed_state = agent_state_with_active_intention_and_routine();
    let suspended = agent_event_with_payload(
        "event_intention_suspend_before_resume_matrix",
        EventKind::IntentionSuspended,
        40,
        transition_payload("intention_breakfast", "suspended", "pause before resume"),
    );
    let resumed = agent_event_with_payload(
        "event_intention_resume_matrix",
        EventKind::IntentionResumed,
        41,
        transition_payload("intention_breakfast", "active", "resume matrix"),
    );
    assert_eq!(
        apply_agent_event(&mut resumed_state, &suspended),
        Ok(ApplyOutcome::Applied)
    );
    assert_eq!(
        apply_agent_event(&mut resumed_state, &resumed),
        Ok(ApplyOutcome::Applied)
    );
    assert_eq!(
        active_intention(&resumed_state, "intention_breakfast").status,
        IntentionStatus::Active
    );

    let continued = agent_event_with_payload(
        "event_intention_continue_matrix",
        EventKind::IntentionContinued,
        42,
        vec![
            PayloadField::new("intention_id", "intention_breakfast"),
            PayloadField::new("status", "active"),
            PayloadField::new("reason", "continue matrix"),
            PayloadField::new("progress_tick", "33"),
            PayloadField::new("current_step", "inspect_visible_key"),
        ],
    );
    assert_eq!(
        apply_agent_event(&mut state, &continued),
        Ok(ApplyOutcome::Applied)
    );
    let intention = active_intention(&state, "intention_breakfast");
    assert_eq!(intention.last_progress_tick, SimTick::new(33));
    assert_eq!(
        intention.current_step.as_deref(),
        Some("inspect_visible_key")
    );

    for (kind, payload, status, attempts) in [
        (
            EventKind::RoutineStepStarted,
            vec![
                PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
                PayloadField::new("progress_tick", "44"),
                PayloadField::new("action_id", "check_container.pantry"),
                PayloadField::new("fallback_attempts", "2"),
            ],
            tracewake_core::agent::RoutineStepStatus::InProgress,
            2,
        ),
        (
            EventKind::RoutineStepCompleted,
            vec![
                PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
                PayloadField::new("progress_tick", "45"),
                PayloadField::new("fallback_attempts", "3"),
            ],
            tracewake_core::agent::RoutineStepStatus::Completed,
            3,
        ),
        (
            EventKind::RoutineStepFailed,
            vec![
                PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
                PayloadField::new("progress_tick", "46"),
                PayloadField::new("reason", "blocked matrix"),
                PayloadField::new("fallback_attempts", "4"),
            ],
            tracewake_core::agent::RoutineStepStatus::Failed,
            4,
        ),
    ] {
        let event = agent_event_with_payload(
            &format!("event_routine_step_matrix_{}", kind.stable_id()),
            kind,
            50,
            payload,
        );
        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::Applied)
        );
        let routine = routine_execution(&state, "routine_exec_breakfast");
        assert_eq!(routine.step_status, status);
        assert_eq!(routine.fallback_attempts, attempts);
    }

    let start = agent_event_with_payload(
        "event_candidate_cause_source_matrix",
        EventKind::CandidateGoalsEvaluated,
        60,
        vec![PayloadField::new("payload_schema_version", "1")],
    );
    let mut caused = caused_event(
        "event_continue_arbitration_cause_matrix",
        EventKind::ContinueRoutineAccepted,
        61,
        event_cause("event_candidate_cause_source_matrix"),
    );
    caused.payload = vec![PayloadField::new("payload_schema_version", "1")];
    assert_eq!(
        apply_agent_event(&mut state, &start),
        Ok(ApplyOutcome::Applied)
    );
    assert_eq!(
        apply_agent_event(&mut state, &caused),
        Ok(ApplyOutcome::Applied)
    );
    assert_eq!(
        state
            .continue_routine_arbitrations()
            .get(&EventId::new("event_continue_arbitration_cause_matrix").unwrap())
            .expect("continue record exists")
            .caused_event_ids,
        event_ids(&["event_candidate_cause_source_matrix"])
    );
}

#[test]
fn typed_diagnostic_hidden_truth_true_is_validated_and_replayed() {
    let mut state = agent_state();
    let typed = TypedDiagnosticFields {
        responsible_layer: ResponsibleLayer::ActionValidation,
        blocker_code: BlockerCode::HiddenTruthInput,
        input_source: "actor_known_context".to_string(),
        actual_source: "hidden_truth_probe".to_string(),
        hidden_truth_referenced: true,
        remediation_hint: "reject hidden truth probe".to_string(),
    };
    let diagnostic = StuckDiagnostic::new(
        StuckDiagnosticId::new("diagnostic_hidden_truth_true_matrix").unwrap(),
        actor_id(),
        SimTick::new(7),
        SimTick::new(9),
        Some(NeedKind::Safety),
        None,
        None,
        None,
        None,
        None,
        Some(SemanticActionId::new("inspect.hidden_truth").unwrap()),
        BlockerCategory::Knowledge,
        "hidden truth attempted",
        "actor did not know this fact",
        "debug-only hidden truth input",
        "rejected",
        StuckResultingStatus::Waiting,
    )
    .with_typed_diagnostic(typed.clone());
    let event = agent_event_with_payload(
        "event_stuck_diagnostic_hidden_truth_true_matrix",
        EventKind::StuckDiagnosticRecorded,
        0,
        vec![
            PayloadField::new("diagnostic_schema_version", "1"),
            PayloadField::new("diagnostic_id", "diagnostic_hidden_truth_true_matrix"),
            PayloadField::new("diagnostic_canonical", diagnostic.serialize_canonical()),
            PayloadField::new("responsible_layer", typed.responsible_layer.stable_id()),
            PayloadField::new("blocker_code", typed.blocker_code.stable_id()),
            PayloadField::new("input_source", &typed.input_source),
            PayloadField::new("actual_source", &typed.actual_source),
            PayloadField::new("hidden_truth_referenced", "true"),
            PayloadField::new("remediation_hint", &typed.remediation_hint),
        ],
    );
    assert_eq!(
        apply_agent_event(&mut state, &event),
        Ok(ApplyOutcome::Applied)
    );
    let record = state
        .stuck_diagnostics()
        .get(&StuckDiagnosticId::new("diagnostic_hidden_truth_true_matrix").unwrap())
        .expect("diagnostic applied");
    assert_eq!(record.typed_diagnostic, typed);

    let mut forged = event.clone();
    forged.payload.iter_mut().for_each(|field| {
        if field.key == "hidden_truth_referenced" {
            field.value = "maybe".to_string();
        }
    });
    assert!(matches!(
        apply_agent_event(&mut agent_state(), &forged),
        Err(ApplyError::BadPayload {
            key: "hidden_truth_referenced",
            ..
        })
    ));
}

#[test]
fn agent_noop_allowlist_remains_narrow_and_behavioral() {
    assert_eq!(
        AGENT_WORLD_NOOP_ALLOWLIST,
        &[
            EventKind::FoodConsumed,
            EventKind::NoHumanDayStarted,
            EventKind::NoHumanDayCompleted,
        ]
    );
    assert!(
        APPLY_RS.contains(
            "kind if AGENT_WORLD_NOOP_ALLOWLIST.contains(&kind) => Ok(ApplyOutcome::WorldNoOp)"
        ),
        "agent no-op fallback must stay tied to the explicit allowlist"
    );

    let mut state = agent_state();
    for (index, kind) in [EventKind::NoHumanDayStarted, EventKind::NoHumanDayCompleted]
        .into_iter()
        .enumerate()
    {
        let event = agent_event_with_payload(
            &format!("event_agent_noop_allowlist_matrix_{index}"),
            kind,
            index as u64,
            Vec::new(),
        );
        assert_eq!(
            apply_agent_event(&mut state, &event),
            Ok(ApplyOutcome::WorldNoOp)
        );
    }
    assert_eq!(state, agent_state());
}

#[test]
fn intention_transition_wrong_status_rejects_before_state_change() {
    for (kind, wrong_status) in [
        (EventKind::IntentionContinued, "suspended"),
        (EventKind::IntentionSuspended, "active"),
        (EventKind::IntentionCompleted, "active"),
        (EventKind::IntentionFailed, "active"),
        (EventKind::IntentionAbandoned, "active"),
        (EventKind::IntentionInterrupted, "active"),
    ] {
        let mut state = agent_state_with_active_intention_and_routine();
        let before = state.clone();
        let event = agent_event_with_payload(
            &format!("event_wrong_status_{}", kind.stable_id()),
            kind,
            70,
            transition_payload("intention_breakfast", wrong_status, "wrong status"),
        );
        assert!(matches!(
            apply_agent_event(&mut state, &event),
            Err(ApplyError::PreconditionMismatch {
                field: "status",
                ..
            })
        ));
        assert_eq!(state, before);
    }

    let mut resumed_state = agent_state_with_active_intention_and_routine();
    let suspend = agent_event_with_payload(
        "event_wrong_status_resume_setup",
        EventKind::IntentionSuspended,
        80,
        transition_payload("intention_breakfast", "suspended", "setup suspended"),
    );
    assert_eq!(
        apply_agent_event(&mut resumed_state, &suspend),
        Ok(ApplyOutcome::Applied)
    );
    let before = resumed_state.clone();
    let wrong_resume = agent_event_with_payload(
        "event_wrong_status_resume_matrix",
        EventKind::IntentionResumed,
        81,
        transition_payload("intention_breakfast", "suspended", "wrong resume status"),
    );
    assert!(matches!(
        apply_agent_event(&mut resumed_state, &wrong_resume),
        Err(ApplyError::PreconditionMismatch {
            field: "status",
            ..
        })
    ));
    assert_eq!(resumed_state, before);
}

#[test]
fn world_item_apply_matrix_observes_state_and_precondition_failures() {
    let mut world = world_state();
    let take = world_event_with_payload(
        "event_take_loose_key_matrix",
        EventKind::ItemTakenFromPlace,
        0,
        vec![
            PayloadField::new("item_id", "loose_key_01"),
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("place_id", "shop_front"),
        ],
    );
    assert_eq!(apply_event(&mut world, &take), Ok(ApplyOutcome::Applied));
    assert_eq!(
        world
            .items()
            .get(&ItemId::new("loose_key_01").unwrap())
            .unwrap()
            .location,
        Location::CarriedBy(actor_id())
    );
    assert!(world
        .actors()
        .get(&actor_id())
        .unwrap()
        .carried_item_ids
        .contains(&ItemId::new("loose_key_01").unwrap()));
    assert!(!world
        .places()
        .get(&PlaceId::new("shop_front").unwrap())
        .unwrap()
        .local_item_ids
        .contains(&ItemId::new("loose_key_01").unwrap()));

    let place = world_event_with_payload(
        "event_place_loose_key_matrix",
        EventKind::ItemPlacedInPlace,
        1,
        vec![
            PayloadField::new("item_id", "loose_key_01"),
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("place_id", "back_room"),
        ],
    );
    assert_eq!(apply_event(&mut world, &place), Ok(ApplyOutcome::Applied));
    assert_eq!(
        world
            .items()
            .get(&ItemId::new("loose_key_01").unwrap())
            .unwrap()
            .location,
        Location::AtPlace(PlaceId::new("back_room").unwrap())
    );
    assert!(!world
        .actors()
        .get(&actor_id())
        .unwrap()
        .carried_item_ids
        .contains(&ItemId::new("loose_key_01").unwrap()));
    assert!(world
        .places()
        .get(&PlaceId::new("back_room").unwrap())
        .unwrap()
        .local_item_ids
        .contains(&ItemId::new("loose_key_01").unwrap()));

    let before = world.clone();
    let bad_take = world_event_with_payload(
        "event_take_missing_actor_matrix",
        EventKind::ItemTakenFromPlace,
        2,
        vec![
            PayloadField::new("item_id", "loose_key_01"),
            PayloadField::new("actor_id", "actor_missing"),
            PayloadField::new("place_id", "back_room"),
        ],
    );
    assert!(matches!(
        apply_event(&mut world, &bad_take),
        Err(ApplyError::MissingActor(actor)) if actor.as_str() == "actor_missing"
    ));
    assert_eq!(world, before);

    let bad_place = world_event_with_payload(
        "event_place_wrong_location_matrix",
        EventKind::ItemPlacedInPlace,
        3,
        vec![
            PayloadField::new("item_id", "loose_key_01"),
            PayloadField::new("actor_id", actor_id().as_str()),
            PayloadField::new("place_id", "shop_front"),
        ],
    );
    assert!(matches!(
        apply_event(&mut world, &bad_place),
        Err(ApplyError::PreconditionMismatch {
            field: "item.location",
            ..
        })
    ));
    assert_eq!(world, before);
}

#[test]
fn world_boolean_preconditions_reject_payload_lies_before_state_change() {
    let mut world = world_state();
    let bad_door_close = world_event_with_payload(
        "event_bad_door_bool_matrix",
        EventKind::DoorClosed,
        0,
        vec![
            PayloadField::new("door_id", "door_shop_back"),
            PayloadField::new("was_open", "false"),
            PayloadField::new("now_open", "true"),
        ],
    );
    let before = world.clone();
    assert!(matches!(
        apply_event(&mut world, &bad_door_close),
        Err(ApplyError::PreconditionMismatch {
            field: "was_open",
            ..
        })
    ));
    assert_eq!(world, before);

    let bad_container_close = world_event_with_payload(
        "event_bad_container_bool_matrix",
        EventKind::ContainerClosed,
        1,
        vec![
            PayloadField::new("container_id", "strongbox_tomas"),
            PayloadField::new("was_open", "false"),
            PayloadField::new("now_open", "true"),
        ],
    );
    let before = world.clone();
    assert!(matches!(
        apply_event(&mut world, &bad_container_close),
        Err(ApplyError::PreconditionMismatch {
            field: "was_open",
            ..
        })
    ));
    assert_eq!(world, before);
}
