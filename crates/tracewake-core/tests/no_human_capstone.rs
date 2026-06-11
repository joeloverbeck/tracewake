use std::collections::{BTreeMap, BTreeSet};

mod support;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    NeedChangeCause, NeedKind, NeedState, RoutineExecution, RoutineFamily,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventCause, EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1,
};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, DecisionTraceId, EventId, FixtureId,
    FoodSupplyId, PlaceId, ProposalId, RoutineExecutionId, RoutineTemplateId, SleepAffordanceId,
    WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, FoodSupplyState, PhysicalState, PlaceState, SleepAffordanceState,
    WorkplaceState,
};
use tracewake_core::time::SimTick;

#[test]
fn no_human_capstone_proves_typed_ancestry_and_replay() {
    let (mut world, mut agent_state, expected_roster) = capstone_world_and_agents();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let initial_actor_count = world.actors().len();
    let initial_routine_count = agent_state.routine_executions().len();
    let registry = capstone_registry();
    let content_manifest_id = ContentManifestId::new("phase3a_capstone_manifest").unwrap();
    let mut log = capstone_seed_log(&content_manifest_id);
    let windows = capstone_windows();
    let expected_window_ids = windows
        .iter()
        .map(|window| window.window_id.clone())
        .collect::<Vec<_>>();

    let report = run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry,
        content_manifest_id,
        NoHumanDayConfig {
            actor_ids: expected_roster.clone(),
            windows,
        },
    );

    assert_eq!(report.actor_decision_order, expected_roster);
    assert_eq!(report.window_ids, expected_window_ids);
    assert_eq!(report.start_tick, SimTick::ZERO);
    assert_eq!(report.final_tick, SimTick::new(32));
    assert!(report.ordinary_pipeline_events > initial_actor_count);

    assert_required_acceptance_events(&log);
    assert_no_human_proposals_have_ancestry(&log);
    assert_decision_trace_ancestry(&log, &agent_state);
    assert_intention_and_routine_ancestry(&log, &agent_state);
    assert_no_controller_or_hidden_truth_leak(&log);
    assert_phase3a_checklist_is_mapped();

    let metrics = no_human_day_metrics(&log);
    assert_eq!(metrics.projection_version, "no_human_day_metrics_v1");
    assert!(metrics.events_per_day > initial_actor_count);
    assert!(metrics.routine_event_count >= initial_routine_count);
    assert!(metrics.meals_completed > 0);
    assert!(metrics.sleep_completed > 0);
    assert!(metrics.work_blocks_completed > 0);
    assert!(metrics.work_blocks_failed > 0);
    assert_eq!(metrics.player_conditioned_event_count, 0);
    assert!(!log.events().iter().any(|event| {
        event
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "unspecified_wait")
    }));

    let context = ChecksumContext {
        fixture_id: FixtureId::new("phase3a_capstone").unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
        sim_tick: report.final_tick,
        world_stream_position_applied: log
            .events()
            .iter()
            .filter(|event| event.stream == EventStream::World)
            .count()
            .saturating_sub(1) as u64,
    };
    let live_physical_checksum = compute_physical_checksum(&world, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let rebuild = rebuild_projection(
        &initial_world,
        &initial_agent_state,
        &log,
        &context,
        Some(&world),
    );

    assert!(rebuild.unsupported_versions.is_empty());
    assert!(rebuild.unsupported_agent_versions.is_empty());
    assert!(rebuild.invariant_violations.is_empty());
    assert!(rebuild.epistemic_application_errors.is_empty());
    assert!(
        rebuild.decision_context_hash_failures.is_empty(),
        "{:?}",
        rebuild.decision_context_hash_failures
    );
    let non_marker_agent_errors = rebuild
        .agent_application_errors
        .iter()
        .filter(|error| {
            !matches!(
                error.event_kind,
                EventKind::NoHumanDayStarted | EventKind::NoHumanDayCompleted
            )
        })
        .collect::<Vec<_>>();
    assert!(
        non_marker_agent_errors.is_empty(),
        "{non_marker_agent_errors:?}"
    );
    assert!(rebuild.state_diff.is_empty());
    assert_eq!(rebuild.final_checksum, live_physical_checksum);
    assert_eq!(rebuild.final_agent_checksum, live_agent_checksum);
    assert_eq!(
        rebuild.final_agent_state.intentions(),
        agent_state.intentions()
    );
    assert_eq!(
        rebuild.final_agent_state.routine_executions(),
        agent_state.routine_executions()
    );
    assert_eq!(
        rebuild.final_agent_state.decision_traces(),
        agent_state.decision_traces()
    );
    assert_eq!(
        rebuild.final_agent_state.stuck_diagnostics(),
        agent_state.stuck_diagnostics()
    );
    assert_eq!(
        no_human_day_metrics(&EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap())
            .serialize_canonical(),
        metrics.serialize_canonical()
    );
}

fn assert_required_acceptance_events(log: &EventLog) {
    for required in [
        EventKind::NoHumanDayStarted,
        EventKind::NoHumanDayCompleted,
        EventKind::NeedDeltaApplied,
        EventKind::SleepStarted,
        EventKind::SleepCompleted,
        EventKind::FoodConsumed,
        EventKind::ActorWaited,
        EventKind::ActorMoved,
        EventKind::WorkBlockStarted,
        EventKind::WorkBlockCompleted,
        EventKind::WorkBlockFailed,
        EventKind::IntentionStarted,
        EventKind::IntentionContinued,
        EventKind::RoutineStepStarted,
        EventKind::RoutineStepCompleted,
        EventKind::RoutineStepFailed,
        EventKind::DecisionTraceRecorded,
    ] {
        assert!(
            log.events()
                .iter()
                .any(|event| event.event_type == required),
            "capstone missing required event {required:?}"
        );
    }
}

fn assert_no_human_proposals_have_ancestry(log: &EventLog) {
    for event in log.events().iter().filter(|event| {
        event.proposal_id.is_some()
            && event.process_id.is_some()
            && !matches!(
                event.event_type,
                EventKind::DecisionTraceRecorded
                    | EventKind::IntentionStarted
                    | EventKind::IntentionContinued
                    | EventKind::RoutineStepStarted
                    | EventKind::RoutineStepCompleted
                    | EventKind::RoutineStepFailed
            )
    }) {
        let proposal_id = event.proposal_id.as_ref().unwrap();
        assert!(
            event
                .causes
                .iter()
                .any(|cause| cause == &EventCause::Proposal(proposal_id.clone())),
            "ordinary no-human event {} lacks proposal ancestry",
            event.event_id.as_str()
        );
    }
}

fn assert_decision_trace_ancestry(log: &EventLog, agent_state: &AgentState) {
    let events = log.events();
    let event_ids = events
        .iter()
        .map(|event| (event.event_id.as_str().to_string(), event))
        .collect::<BTreeMap<_, _>>();
    let event_indexes = events
        .iter()
        .enumerate()
        .map(|(index, event)| (event.event_id.as_str().to_string(), index))
        .collect::<BTreeMap<_, _>>();
    let mut trace_ids_from_events = BTreeSet::new();

    for (trace_index, event) in events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.event_type == EventKind::DecisionTraceRecorded)
    {
        let proposal_id = event
            .proposal_id
            .as_ref()
            .expect("trace event carries proposal id");
        let trace_id = payload(event, "trace_id");
        let ordinary_event_id = payload(event, "ordinary_event_id");
        let ordinary_event = event_ids
            .get(ordinary_event_id)
            .unwrap_or_else(|| panic!("missing ordinary event {ordinary_event_id}"));
        let ordinary_index = event_indexes[ordinary_event_id];
        assert!(
            ordinary_index < trace_index,
            "decision trace must follow its ordinary event"
        );
        assert_eq!(
            ordinary_event.proposal_id.as_ref(),
            Some(proposal_id),
            "decision trace proposal must match ordinary event proposal"
        );
        assert!(
            event
                .causes
                .iter()
                .any(|cause| cause == &EventCause::Event(ordinary_event.event_id.clone())),
            "decision trace must be caused by the ordinary event it names"
        );
        for intervening in &events[ordinary_index + 1..trace_index] {
            assert!(
                intervening
                    .proposal_id
                    .as_ref()
                    .is_none_or(|id| id == proposal_id),
                "decision trace was inserted after unrelated proposal {}",
                intervening
                    .proposal_id
                    .as_ref()
                    .map(ProposalId::as_str)
                    .unwrap_or("none")
            );
        }

        let canonical = payload(event, "trace_canonical");
        let record =
            tracewake_core::agent::DecisionTraceRecord::deserialize_canonical(canonical.as_bytes())
                .expect("trace canonical is typed record");
        assert_eq!(record.trace_id.as_str(), trace_id);
        assert_eq!(record.actor_id.as_str(), payload(event, "actor_id"));
        assert_eq!(
            record
                .hidden_truth_audit_result
                .actor_known_only
                .to_string(),
            payload(event, "hidden_truth_audit_actor_known_only")
        );
        assert_eq!(
            record.hidden_truth_audit_result.notes,
            payload(event, "hidden_truth_audit_notes")
        );
        assert_eq!(
            agent_state
                .decision_traces()
                .get(&record.trace_id)
                .expect("live agent state stores trace"),
            &record
        );
        trace_ids_from_events.insert(record.trace_id);
    }

    assert!(!trace_ids_from_events.is_empty());
    let trace_ids_from_state = agent_state
        .decision_traces()
        .keys()
        .cloned()
        .collect::<BTreeSet<DecisionTraceId>>();
    assert_eq!(trace_ids_from_events, trace_ids_from_state);
}

fn assert_intention_and_routine_ancestry(log: &EventLog, agent_state: &AgentState) {
    let events = log.events();
    let event_ids = events
        .iter()
        .map(|event| (event.event_id.as_str().to_string(), event))
        .collect::<BTreeMap<_, _>>();
    for event in events.iter().filter(|event| {
        matches!(
            event.event_type,
            EventKind::IntentionStarted
                | EventKind::IntentionContinued
                | EventKind::RoutineStepStarted
                | EventKind::RoutineStepCompleted
                | EventKind::RoutineStepFailed
        )
    }) {
        let proposal_id = event.proposal_id.as_ref().unwrap_or_else(|| {
            panic!(
                "agent ancestry event {} {:?} carries no proposal id",
                event.event_id.as_str(),
                event.event_type
            )
        });
        let ordinary_event_id = payload_opt(event, "follow_on_event_id")
            .or_else(|| payload_opt(event, "ordinary_event_id"))
            .unwrap_or_else(|| {
                panic!(
                    "event {} missing follow-on or ordinary ancestry id",
                    event.event_id.as_str()
                )
            });
        let ordinary_event = event_ids
            .get(ordinary_event_id)
            .unwrap_or_else(|| panic!("missing ordinary ancestry event {ordinary_event_id}"));
        assert_eq!(ordinary_event.proposal_id.as_ref(), Some(proposal_id));
        assert!(
            event
                .causes
                .iter()
                .any(|cause| cause == &EventCause::Event(ordinary_event.event_id.clone())),
            "agent ancestry event must be caused by its ordinary event"
        );

        if event.event_type == EventKind::IntentionStarted {
            let trace_id = DecisionTraceId::new(payload(event, "trace_id")).unwrap();
            assert!(
                agent_state.decision_traces().contains_key(&trace_id),
                "started intention references missing decision trace {}",
                trace_id.as_str()
            );
        }
        if matches!(
            event.event_type,
            EventKind::RoutineStepStarted
                | EventKind::RoutineStepCompleted
                | EventKind::RoutineStepFailed
        ) {
            let execution_id = RoutineExecutionId::new(payload(event, "routine_execution_id"))
                .expect("routine event carries typed execution id");
            assert!(
                agent_state.routine_executions().contains_key(&execution_id),
                "routine event references missing execution {}",
                execution_id.as_str()
            );
        }
    }
}

fn assert_no_controller_or_hidden_truth_leak(log: &EventLog) {
    let rendered = format!("{:?}", log.events()).to_ascii_lowercase();
    for forbidden in ["player", "controller", "food_hidden", "hidden_route"] {
        assert!(
            !rendered.contains(forbidden),
            "capstone log leaked forbidden term {forbidden}"
        );
    }
    assert!(!log
        .events()
        .iter()
        .any(|event| event.stream == EventStream::Controller));
}

fn assert_phase3a_checklist_is_mapped() {
    const CHECKLIST: &[(&str, &str)] = &[
        (
            "transaction",
            "0008PHA3AANTCON-004/006 plus capstone proposal ancestry",
        ),
        (
            "actor_known_context",
            "0008PHA3AANTCON-001 plus capstone trace audit",
        ),
        (
            "no_hidden_truth",
            "0008PHA3AANTCON-008 plus capstone no-leak scan",
        ),
        (
            "candidate_arbitration",
            "0008PHA3AANTCON-003 plus decision trace records",
        ),
        (
            "durable_intentions",
            "0008PHA3AANTCON-005 plus intention ancestry events",
        ),
        (
            "routines",
            "0008PHA3AANTCON-006 plus routine ancestry events",
        ),
        (
            "shared_pipeline",
            "ordinary proposal causes and event proposal ids",
        ),
        (
            "validator_authority",
            "0008PHA3AANTCON-003 anti-regression gates",
        ),
        (
            "typed_diagnostics",
            "0008PHA3AANTCON-002/010 plus replay equality",
        ),
        ("debug_tui", "0008PHA3AANTCON-010 targeted TUI lanes"),
        ("adversarial_fixtures", "0008PHA3AANTCON-008/009 gates"),
        ("anti_regression", "0008PHA3AANTCON-007 gates"),
        ("replay", "capstone live/rebuild typed state equality"),
        (
            "decision_context_hash_gate",
            "capstone asserts no replay context-hash failures",
        ),
    ];
    assert_eq!(CHECKLIST.len(), 14);
    assert!(CHECKLIST
        .iter()
        .all(|(item, evidence)| !item.is_empty() && !evidence.is_empty()));
}

fn payload<'a>(event: &'a EventEnvelope, key: &str) -> &'a str {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .unwrap_or_else(|| panic!("event {} missing payload {key}", event.event_id.as_str()))
        .value
        .as_str()
}

fn payload_opt<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn capstone_registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

fn capstone_windows() -> Vec<DayWindow> {
    vec![
        DayWindow {
            window_id: "pre_dawn".to_string(),
            start_tick: SimTick::ZERO,
            end_tick: SimTick::new(4),
        },
        DayWindow {
            window_id: "morning".to_string(),
            start_tick: SimTick::new(4),
            end_tick: SimTick::new(10),
        },
        DayWindow {
            window_id: "work_window".to_string(),
            start_tick: SimTick::new(10),
            end_tick: SimTick::new(18),
        },
        DayWindow {
            window_id: "evening".to_string(),
            start_tick: SimTick::new(18),
            end_tick: SimTick::new(24),
        },
        DayWindow {
            window_id: "night".to_string(),
            start_tick: SimTick::new(24),
            end_tick: SimTick::new(32),
        },
    ]
}

fn capstone_seed_log(content_manifest_id: &ContentManifestId) -> EventLog {
    let mut log = EventLog::new();
    for (sequence, actor_id, workplace_id, place_id) in [
        (0, "actor_anna", "workplace_anna_closed", "office_anna"),
        (1, "actor_tomas", "workplace_tomas", "workshop_tomas"),
    ] {
        let actor_id = ActorId::new(actor_id).unwrap();
        let workplace_id = WorkplaceId::new(workplace_id).unwrap();
        let mut event = EventEnvelope::new_v1(
            EventId::new(format!(
                "event.capstone.role_notice.{}",
                workplace_id.as_str()
            ))
            .unwrap(),
            EventKind::RoleAssignmentNoticeRecorded,
            0,
            sequence,
            SimTick::ZERO,
            OrderingKey::new(
                SimTick::ZERO,
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Actor(actor_id.clone()),
                ProposalSequence::new(sequence),
                ActionId::new("role_assignment_notice").unwrap(),
                vec![
                    actor_id.as_str().to_string(),
                    workplace_id.as_str().to_string(),
                ],
                format!("capstone_role_notice_{sequence:04}"),
            ),
            content_manifest_id.clone(),
        );
        event.actor_id = Some(actor_id.clone());
        event.participants = vec![
            actor_id.as_str().to_string(),
            workplace_id.as_str().to_string(),
        ];
        event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("source_kind", "authored_prehistory"),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("workplace_id", workplace_id.as_str()),
            PayloadField::new("place_id", place_id),
            PayloadField::new("access_open", "true"),
        ];
        log.append(event).unwrap();
    }
    log
}

fn capstone_world_and_agents() -> (PhysicalState, AgentState, Vec<ActorId>) {
    let mut world = PhysicalSeed::default();
    for (place_id, label) in [
        ("commons", "Commons"),
        ("home_elena", "Elena home"),
        ("home_bruno", "Bruno home"),
        ("home_mara", "Mara home"),
        ("home_tomas", "Tomas home"),
        ("office_anna", "Anna office"),
        ("workshop_tomas", "Tomas workshop"),
    ] {
        world.places_mut().insert(
            PlaceId::new(place_id).unwrap(),
            PlaceState::new(PlaceId::new(place_id).unwrap(), label),
        );
    }
    for place_id in [
        "home_elena",
        "home_bruno",
        "home_mara",
        "home_tomas",
        "office_anna",
        "workshop_tomas",
    ] {
        connect_places(&mut world, "commons", place_id);
    }

    for (actor_id, place_id) in [
        ("actor_anna", "office_anna"),
        ("actor_bruno", "home_bruno"),
        ("actor_elena", "home_elena"),
        ("actor_mara", "home_mara"),
        ("actor_tomas", "home_tomas"),
    ] {
        add_actor(&mut world, actor_id, place_id);
    }
    world.sleep_affordances_mut().insert(
        SleepAffordanceId::new("bed_elena").unwrap(),
        SleepAffordanceState::new(
            SleepAffordanceId::new("bed_elena").unwrap(),
            PlaceId::new("home_elena").unwrap(),
            4,
            20,
            2,
        ),
    );

    world.food_supplies_mut().insert(
        FoodSupplyId::new("food_stew_home_bruno").unwrap(),
        FoodSupplyState::new(
            FoodSupplyId::new("food_stew_home_bruno").unwrap(),
            Location::AtPlace(PlaceId::new("home_bruno").unwrap()),
            2,
            240,
        ),
    );

    let mut closed_workplace = WorkplaceState::new(
        WorkplaceId::new("workplace_anna_closed").unwrap(),
        PlaceId::new("office_anna").unwrap(),
        4,
        8,
        4,
        900,
        900,
        "blocked_office_output",
    );
    closed_workplace
        .assigned_actor_ids
        .insert(ActorId::new("actor_anna").unwrap());
    closed_workplace.access_open = false;
    world.workplaces_mut().insert(
        WorkplaceId::new("workplace_anna_closed").unwrap(),
        closed_workplace,
    );

    let mut open_workplace = WorkplaceState::new(
        WorkplaceId::new("workplace_tomas").unwrap(),
        PlaceId::new("workshop_tomas").unwrap(),
        4,
        8,
        4,
        900,
        900,
        "tomas_work_output",
    );
    open_workplace
        .assigned_actor_ids
        .insert(ActorId::new("actor_tomas").unwrap());
    open_workplace.work_duration_ticks = 4;
    open_workplace.access_open = true;
    world
        .workplaces_mut()
        .insert(WorkplaceId::new("workplace_tomas").unwrap(), open_workplace);

    let mut agent_state = AgentSeed::default();
    for (actor_id, hunger, fatigue) in [
        ("actor_anna", 320, 260),
        ("actor_bruno", 490, 240),
        ("actor_elena", 260, 820),
        ("actor_mara", 900, 240),
        ("actor_tomas", 520, 260),
    ] {
        let actor_id = ActorId::new(actor_id).unwrap();
        agent_state.needs_by_actor_mut().insert(
            actor_id,
            BTreeMap::from([
                (
                    NeedKind::Hunger,
                    NeedState::initial(NeedKind::Hunger, hunger, NeedChangeCause::FixtureInitial),
                ),
                (
                    NeedKind::Fatigue,
                    NeedState::initial(NeedKind::Fatigue, fatigue, NeedChangeCause::FixtureInitial),
                ),
                (
                    NeedKind::Safety,
                    NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
                ),
            ]),
        );
    }
    add_routine_execution(
        &mut agent_state,
        "routine_exec_anna_work",
        "actor_anna",
        "routine_anna_work",
        RoutineFamily::WorkBlock,
        4,
        18,
    );
    add_routine_execution(
        &mut agent_state,
        "routine_exec_elena_sleep",
        "actor_elena",
        "routine_elena_sleep",
        RoutineFamily::SleepNight,
        24,
        32,
    );
    add_routine_execution(
        &mut agent_state,
        "routine_exec_tomas_go_work",
        "actor_tomas",
        "routine_tomas_go_work",
        RoutineFamily::GoToWork,
        4,
        10,
    );
    add_routine_execution(
        &mut agent_state,
        "routine_exec_tomas_work",
        "actor_tomas",
        "routine_tomas_work",
        RoutineFamily::WorkBlock,
        10,
        24,
    );

    let expected_roster = world.actors().keys().cloned().collect::<Vec<_>>();
    (world.build(), agent_state.build(), expected_roster)
}

fn connect_places(world: &mut PhysicalSeed, left: &str, right: &str) {
    let left = PlaceId::new(left).unwrap();
    let right = PlaceId::new(right).unwrap();
    world
        .places_mut()
        .get_mut(&left)
        .unwrap()
        .adjacent_place_ids
        .insert(right.clone());
    world
        .places_mut()
        .get_mut(&right)
        .unwrap()
        .adjacent_place_ids
        .insert(left);
}

fn add_actor(world: &mut PhysicalSeed, actor_id: &str, place_id: &str) {
    let actor_id = ActorId::new(actor_id).unwrap();
    let place_id = PlaceId::new(place_id).unwrap();
    world.actors_mut().insert(
        actor_id.clone(),
        ActorBody::new(actor_id.clone(), place_id.clone()),
    );
    world
        .places_mut()
        .get_mut(&place_id)
        .unwrap()
        .local_actor_ids
        .insert(actor_id);
}

fn add_routine_execution(
    agent_state: &mut AgentSeed,
    execution_id: &str,
    actor_id: &str,
    template_id: &str,
    family: RoutineFamily,
    start_tick: u64,
    end_tick: u64,
) {
    let execution_id = RoutineExecutionId::new(execution_id).unwrap();
    agent_state.routine_executions_mut().insert(
        execution_id.clone(),
        RoutineExecution::new(
            execution_id.clone(),
            ActorId::new(actor_id).unwrap(),
            RoutineTemplateId::new(template_id).unwrap(),
            family,
            SimTick::new(start_tick),
            Some(SimTick::new(start_tick).next()),
            Some(SimTick::new(end_tick)),
            Some("body".to_string()),
            DecisionTraceId::new(format!("trace_{execution_id}")).unwrap(),
        ),
    );
}
