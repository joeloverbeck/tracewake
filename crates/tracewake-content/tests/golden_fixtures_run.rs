use std::collections::{BTreeMap, BTreeSet};

use tracewake_content::fixtures;
use tracewake_content::fixtures::GoldenFixture;
use tracewake_content::load::load_fixture_package;
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes};
use tracewake_core::actions::defs::sleep::build_sleep_completion_events;
use tracewake_core::actions::defs::work::build_work_completion_events;
use tracewake_core::actions::pipeline::{run_pipeline, PipelineContext};
use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
use tracewake_core::actions::{ActionRegistry, ReasonCode, ReportStatus};
use tracewake_core::agent::{
    build_actor_known_planning_state, generate_candidate_goals, plan_local_actions,
    select_goal_and_trace, select_method_from_templates, ActorKnownFact, CandidateGenerationInput,
    DecisionInput, DecisionTraceRecord, GoalKind, LocalPlanRequest, NeedChangeCause, NeedKind,
    NeedState, PlannerGoal, RoutineCondition, RoutineFamily, RoutineStep, RoutineTemplate,
    VisibleLocalPlanningState,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_holder_known_context_hash, compute_physical_checksum,
    ChecksumContext,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::epistemics::{EpistemicProjection, HolderKind, SourceRef};
use tracewake_core::events::apply::{apply_event, apply_event_stream, EventApplicationContext};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventEnvelope, EventKind, EventStream};
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, FixtureId, FoodSupplyId, IntentionId,
    RoutineExecutionId, RoutineTemplateId,
};
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{
    default_day_windows, run_no_human_day, NoHumanDayConfig,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{AgentState, ControllerMode, PhysicalState};
use tracewake_core::time::SimTick;

fn registry() -> ActionRegistry {
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

fn load(golden: GoldenFixture) -> (PhysicalState, AgentState, ContentManifestId) {
    let manifest_id =
        ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str())).unwrap();
    let loaded = load_fixture_package(
        manifest_id.clone(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    (
        loaded.canonical_world,
        loaded.canonical_agent_state,
        manifest_id,
    )
}

fn load_with_log(
    golden: GoldenFixture,
) -> (PhysicalState, AgentState, ContentManifestId, EventLog) {
    let manifest_id =
        ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str())).unwrap();
    let loaded = load_fixture_package(
        manifest_id.clone(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    (
        loaded.canonical_world,
        loaded.canonical_agent_state,
        manifest_id,
        loaded.seed_event_log,
    )
}

fn checksum_context(fixture_id: &str, log: &EventLog) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new(fixture_id).unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
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

fn proposal(id: &str, actor_id: &str, action_id: &str, target_ids: &[&str], tick: u64) -> Proposal {
    let mut proposal = Proposal::new(
        id.parse().unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id.parse().unwrap()),
        action_id.parse().unwrap(),
        SimTick::new(tick),
    );
    proposal.target_ids = target_ids.iter().map(|target| target.to_string()).collect();
    proposal
}

fn ordering_key(proposal: &Proposal, sequence: u64) -> OrderingKey {
    OrderingKey::new(
        proposal.requested_tick,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(proposal.actor_id.clone().unwrap()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    )
}

fn run(
    state: &mut PhysicalState,
    agent_state: &mut AgentState,
    log: &mut EventLog,
    manifest_id: &ContentManifestId,
    proposal: &Proposal,
    sequence: u64,
) -> Vec<EventEnvelope> {
    let registry = registry();
    let mut context = PipelineContext {
        registry: &registry,
        state,
        agent_state,
        log,
        controller_bindings: None,
        epistemic_projection: None,
        content_manifest_id: manifest_id.clone(),
        ordering_key: ordering_key(proposal, sequence),
    };
    run_pipeline(&mut context, proposal).appended_events
}

fn append_and_apply(
    state: &mut PhysicalState,
    agent_state: &mut AgentState,
    log: &mut EventLog,
    events: Vec<EventEnvelope>,
) {
    for event in events {
        let appended = log.append(event).unwrap();
        let mut application_context = EventApplicationContext {
            physical_state: state,
            agent_state,
            epistemic_projection: None,
        };
        apply_event_stream(&mut application_context, &appended).unwrap();
    }
}

fn has_event(log: &EventLog, kind: EventKind) -> bool {
    log.events().iter().any(|event| event.event_type == kind)
}

fn payload<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn decision_trace_records(log: &EventLog) -> Vec<DecisionTraceRecord> {
    log.events()
        .iter()
        .filter(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .map(|event| {
            DecisionTraceRecord::deserialize_canonical(
                payload(event, "trace_canonical")
                    .expect("decision trace event carries trace_canonical")
                    .as_bytes(),
            )
            .expect("decision trace canonical parses")
        })
        .collect()
}

#[test]
fn loads_fixtures_deterministically() {
    for golden in fixtures::all() {
        let first = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        let second = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();

        validate_fixture(&golden.fixture, &registry()).unwrap();
        assert_eq!(first.canonical_world, second.canonical_world);
        assert_eq!(
            first.manifest.content_fingerprint,
            second.manifest.content_fingerprint
        );
        assert_eq!(
            golden.contract.fixture_id,
            golden.fixture.fixture_id.as_str()
        );
        assert!(!golden.contract.acceptance_assertions.is_empty());
    }
}

#[test]
fn ordinary_workday_fixture_moves_before_work_completion() {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::ordinary_workday_001());
    let mut log = EventLog::new();

    let move_to_work = proposal(
        "proposal_workday_move",
        "actor_tomas",
        "move",
        &["workshop_tomas"],
        4,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &move_to_work,
        0,
    );
    let work = proposal(
        "proposal_workday_work",
        "actor_tomas",
        "work_block",
        &["workplace_tomas"],
        8,
    );
    let work_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work,
        1,
    );
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .expect("work starts after movement ancestry")
        .clone();
    append_and_apply(
        &mut state,
        &mut agent_state,
        &mut log,
        build_work_completion_events(
            &work_started,
            &ordering_key(&work, 2),
            &manifest_id,
            SimTick::new(12),
        ),
    );

    assert!(has_event(&log, EventKind::ActorMoved));
    assert!(has_event(&log, EventKind::WorkBlockStarted));
    assert!(has_event(&log, EventKind::WorkBlockCompleted));
    let actor_id: ActorId = "actor_tomas".parse().unwrap();
    assert_eq!(
        state.actors()[&actor_id].current_place_id.as_str(),
        "workshop_tomas"
    );
}

#[test]
fn sleep_eat_work_fixture_logs_need_effects_and_replays() {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::sleep_eat_work_001());
    let initial_state = state.clone();
    let mut log = EventLog::new();

    let mut sleep = proposal("proposal_sleep", "actor_tomas", "sleep", &["home_tomas"], 0);
    sleep
        .parameters
        .insert("duration_ticks".to_string(), "4".to_string());
    sleep
        .parameters
        .insert("sleep_place_id".to_string(), "home_tomas".to_string());
    sleep
        .parameters
        .insert("sleep_affordance_id".to_string(), "bed_tomas".to_string());
    let sleep_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &sleep,
        0,
    );
    let sleep_started = sleep_events
        .iter()
        .find(|event| event.event_type == EventKind::SleepStarted)
        .expect("sleep starts")
        .clone();
    append_and_apply(
        &mut state,
        &mut agent_state,
        &mut log,
        build_sleep_completion_events(
            &sleep_started,
            &ordering_key(&sleep, 1),
            &manifest_id,
            SimTick::new(4),
        ),
    );

    let eat = proposal(
        "proposal_eat_breakfast",
        "actor_tomas",
        "eat",
        &["food_breakfast_tomas"],
        5,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &eat,
        2,
    );
    let move_to_work = proposal(
        "proposal_sleep_eat_work_move",
        "actor_tomas",
        "move",
        &["workshop_tomas"],
        6,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &move_to_work,
        3,
    );
    let work = proposal(
        "proposal_sleep_eat_work_work",
        "actor_tomas",
        "work_block",
        &["workplace_tomas"],
        8,
    );
    let work_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work,
        4,
    );
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .expect("work starts")
        .clone();
    append_and_apply(
        &mut state,
        &mut agent_state,
        &mut log,
        build_work_completion_events(
            &work_started,
            &ordering_key(&work, 5),
            &manifest_id,
            SimTick::new(11),
        ),
    );

    assert!(has_event(&log, EventKind::SleepCompleted));
    assert!(has_event(&log, EventKind::FoodConsumed));
    assert!(has_event(&log, EventKind::WorkBlockCompleted));
    assert!(
        log.events()
            .iter()
            .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
            .count()
            >= 5
    );

    let replayed = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    assert_eq!(replayed, log);
    let mut replay_state = initial_state;
    for event in replayed.events() {
        apply_event(&mut replay_state, event).unwrap();
    }
    assert_eq!(replay_state, state);
}

#[test]
fn sleep_without_authored_affordance_rejects_with_typed_reason() {
    let (mut state, mut agent_state, manifest_id) =
        load(fixtures::sleep_rejects_current_place_without_sleep_affordance_001());
    let mut log = EventLog::new();
    let mut sleep = proposal(
        "proposal_sleep_no_affordance",
        "actor_elena",
        "sleep",
        &["home_elena"],
        0,
    );
    sleep
        .parameters
        .insert("duration_ticks".to_string(), "4".to_string());
    sleep
        .parameters
        .insert("sleep_place_id".to_string(), "home_elena".to_string());
    sleep
        .parameters
        .insert("sleep_affordance_id".to_string(), "bed_missing".to_string());
    let registry = registry();
    let mut context = PipelineContext {
        registry: &registry,
        state: &mut state,
        agent_state: &mut agent_state,
        log: &mut log,
        controller_bindings: None,
        epistemic_projection: None,
        content_manifest_id: manifest_id,
        ordering_key: ordering_key(&sleep, 0),
    };

    let result = run_pipeline(&mut context, &sleep);

    assert_eq!(result.report.status, ReportStatus::Rejected);
    assert_eq!(
        result.report.reason_codes,
        vec![ReasonCode::NoSleepAffordance]
    );
    assert!(result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::ActionRejected));
    assert!(!log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepStarted));
}

#[test]
fn food_unavailable_fixture_records_typed_failure_without_refill() {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::food_unavailable_replan_001());
    let mut log = EventLog::new();

    let eat = proposal(
        "proposal_mara_empty_food",
        "actor_mara",
        "eat",
        &["food_empty_pantry_mara"],
        1,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &eat,
        0,
    );

    assert!(has_event(&log, EventKind::EatFailed));
    assert!(!has_event(&log, EventKind::FoodConsumed));
    assert!(!has_event(&log, EventKind::NeedDeltaApplied));
    let failure = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::EatFailed)
        .unwrap();
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "blocker_kind" && field.value == "resource"));
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "reason" && field.value == "food source empty"));
}

#[test]
fn routine_blocked_fixture_records_access_failure_without_silent_loop() {
    let (mut state, mut agent_state, manifest_id) =
        load(fixtures::routine_blocked_diagnostic_001());
    let mut log = EventLog::new();

    let work = proposal(
        "proposal_elena_blocked_work",
        "actor_elena",
        "work_block",
        &["workplace_elena"],
        1,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work,
        0,
    );

    assert!(has_event(&log, EventKind::WorkBlockFailed));
    assert!(!has_event(&log, EventKind::WorkBlockStarted));
    let failure = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockFailed)
        .unwrap();
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "blocker_kind" && field.value == "access"));
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "reason" && field.value == "workplace access closed"));
}

#[test]
fn planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit() {
    let (_state, agent_state, manifest_id) = load(fixtures::planner_trace_001());
    let actor_id: ActorId = "actor_tomas".parse().unwrap();
    let generated = generate_candidate_goals(&CandidateGenerationInput {
        actor_id: actor_id.clone(),
        decision_tick: SimTick::new(2),
        needs: vec![NeedState::initial(
            NeedKind::Hunger,
            820,
            NeedChangeCause::FixtureInitial,
        )],
        active_intention: None,
        actor_known_facts: vec![ActorKnownFact::observed_now(
            actor_id.clone(),
            "actor_knows_food_source",
            "food_market_stew",
            "planner_trace_001:visible_food",
            None,
        )],
        routine_window_goal: Some(GoalKind::GoToWork),
    });
    let selection = select_goal_and_trace(DecisionInput {
        actor_id: actor_id.clone(),
        decision_tick: SimTick::new(2),
        candidates: generated.candidates.clone(),
        active_intention: None,
        actor_known_inputs: generated.actor_known_inputs_used.clone(),
    })
    .unwrap();
    let actor_known_state = build_actor_known_planning_state(
        &actor_id,
        &EpistemicProjection::new(manifest_id.clone()),
        &agent_state,
        &VisibleLocalPlanningState::new(
            "home_tomas".parse().unwrap(),
            BTreeMap::from([(
                "home_tomas".parse().unwrap(),
                BTreeSet::from(["market_square".parse().unwrap()]),
            )]),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::from(["food_market_stew".to_string()]),
            BTreeSet::new(),
            BTreeMap::new(),
        ),
    );
    let rejected_template = RoutineTemplate::new(
        RoutineTemplateId::new("routine_a_rejected_eat_workplace").unwrap(),
        RoutineFamily::EatMeal,
        vec![RoutineCondition::ActorKnowsWorkplace],
        vec![RoutineCondition::FoodSourceBelievedAccessible],
        vec![RoutineStep::WaitUntil {
            reason: "rejected fixture method".to_string(),
        }],
        1,
        1,
        vec![0],
        vec!["missing_workplace".to_string()],
        vec!["fallback_wait".to_string()],
        vec!["planner_trace_001".to_string()],
        None,
    )
    .unwrap();
    let selected_template = RoutineTemplate::new(
        RoutineTemplateId::new("routine_b_selected_eat_food").unwrap(),
        RoutineFamily::EatMeal,
        vec![RoutineCondition::ActorKnowsFoodSource],
        vec![RoutineCondition::FoodSourceBelievedAccessible],
        vec![RoutineStep::ConsumeAccessibleFood {
            action_id: "eat".parse().unwrap(),
        }],
        1,
        1,
        vec![0],
        vec!["food_missing".to_string()],
        vec!["fallback_wait".to_string()],
        vec!["planner_trace_001".to_string()],
        Some("food_source".to_string()),
    )
    .unwrap();
    let method = select_method_from_templates(
        &selection.selected_goal,
        &actor_known_state,
        actor_known_state.actor_known_facts(),
        SimTick::new(2),
        &[selected_template, rejected_template],
    )
    .unwrap();

    assert!(generated.candidates.len() >= 3);
    assert_eq!(selection.selected_goal.goal_kind, GoalKind::Eat);
    assert!(!selection.trace.rejected_goals.is_empty());
    assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
    assert!(method.trace.selected_method_id.is_some());
    assert!(method.trace.hidden_truth_audit_result.actor_known_only);
    assert!(!method.trace.rejected_methods.is_empty());
    assert!(method
        .trace
        .beliefs_perceptions_known_places_used
        .iter()
        .any(|source| source.contains("actor_known_state")));

    let no_source_plan = plan_local_actions(
        &actor_known_state,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_market_stew".to_string()),
            budget: 1,
            actor_known_facts: vec![ActorKnownFact::unproven(
                "actor_knows_food_source",
                "planner_trace_001 negative assertion",
            )],
        },
    )
    .unwrap();
    assert!(
        !no_source_plan
            .trace
            .hidden_truth_audit_result
            .actor_known_only
    );
    assert!(no_source_plan
        .trace
        .hidden_truth_audit_result
        .notes
        .contains("unproven:planner_trace_001 negative assertion"));
}

#[test]
fn routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry() {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::routine_no_teleport_001());
    let mut log = EventLog::new();

    let work = proposal(
        "proposal_remote_work_without_move",
        "actor_tomas",
        "work_block",
        &["workplace_remote"],
        1,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work,
        0,
    );

    assert!(has_event(&log, EventKind::WorkBlockFailed));
    assert!(!has_event(&log, EventKind::ActorMoved));
    assert!(!has_event(&log, EventKind::WorkBlockStarted));
    let actor_id = "actor_tomas".parse().unwrap();
    assert_eq!(
        state.actors()[&actor_id].current_place_id.as_str(),
        "home_tomas"
    );
    let failure = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockFailed)
        .unwrap();
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "reason" && field.value == "actor not at workplace"));
}

#[test]
fn possession_fixture_preserves_intention_needs_and_can_continue() {
    let golden = fixtures::possession_does_not_reset_intention_001();
    let manifest_id =
        ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str())).unwrap();
    let loaded = load_fixture_package(
        manifest_id.clone(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    let mut state = loaded.canonical_world;
    let mut agent_state = loaded.canonical_agent_state;
    let actor_id: ActorId = "actor_mara".parse().unwrap();
    let intention_id: IntentionId = "intention_mara_work".parse().unwrap();
    let routine_execution_id: RoutineExecutionId = "routine_exec_mara_work".parse().unwrap();
    assert_eq!(
        agent_state.active_intention_by_actor().get(&actor_id),
        Some(&intention_id)
    );
    assert!(agent_state.intentions().contains_key(&intention_id));
    assert!(agent_state
        .routine_executions()
        .contains_key(&routine_execution_id));
    let before_agent_state = agent_state.clone();
    let mut log = EventLog::new();
    let mut bindings = ControllerBindings::new();
    let controller_id = ControllerId::new("controller_human").unwrap();

    bindings.attach(
        controller_id.clone(),
        actor_id.clone(),
        ControllerMode::Embodied,
        SimTick::new(2),
        &mut log,
        manifest_id.clone(),
    );
    bindings.detach(
        &controller_id,
        SimTick::new(3),
        &mut log,
        manifest_id.clone(),
    );

    assert_eq!(agent_state, before_agent_state);
    assert!(has_event(&log, EventKind::ControllerAttached));
    assert!(has_event(&log, EventKind::ControllerDetached));

    let mut continue_proposal = proposal(
        "proposal_mara_continue_after_possession",
        "actor_mara",
        "continue_routine",
        &[],
        4,
    );
    continue_proposal.parameters.insert(
        "active_intention_id".to_string(),
        "intention_mara_work".to_string(),
    );
    continue_proposal
        .parameters
        .insert("intention_status".to_string(), "active".to_string());
    continue_proposal.parameters.insert(
        "routine_execution_id".to_string(),
        "routine_exec_mara_work".to_string(),
    );
    continue_proposal
        .parameters
        .insert("next_action_id".to_string(), "work_block".to_string());
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &continue_proposal,
        1,
    );

    assert!(has_event(&log, EventKind::ContinueRoutineProposed));
    assert_eq!(agent_state, before_agent_state);
}

#[test]
fn no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs() {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::no_hidden_truth_planning_001());
    let hidden_food_id: FoodSupplyId = "food_hidden_pantry".parse().unwrap();
    assert!(state.food_supplies().contains_key(&hidden_food_id));

    let actor_id: ActorId = "actor_mara".parse().unwrap();
    let generated = generate_candidate_goals(&CandidateGenerationInput {
        actor_id: actor_id.clone(),
        decision_tick: SimTick::new(1),
        needs: vec![NeedState::initial(
            NeedKind::Hunger,
            880,
            NeedChangeCause::FixtureInitial,
        )],
        active_intention: None,
        actor_known_facts: Vec::new(),
        routine_window_goal: None,
    });
    let selection = select_goal_and_trace(DecisionInput {
        actor_id: actor_id.clone(),
        decision_tick: SimTick::new(1),
        candidates: generated.candidates,
        active_intention: None,
        actor_known_inputs: generated.actor_known_inputs_used.clone(),
    })
    .unwrap();

    assert_eq!(selection.selected_goal.goal_kind, GoalKind::FindFood);
    assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
    assert!(!selection
        .trace
        .beliefs_perceptions_known_places_used
        .iter()
        .any(|input| input.contains("food_hidden_pantry")));

    let epistemic_projection = EpistemicProjection::new(manifest_id.clone());
    let actor_known_state = build_actor_known_planning_state(
        &actor_id,
        &epistemic_projection,
        &agent_state,
        &VisibleLocalPlanningState::new(
            "home_mara".parse().unwrap(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
        ),
    );
    assert!(!actor_known_state
        .known_food_sources()
        .contains("food_hidden_pantry"));
    assert!(actor_known_state
        .proof_sources()
        .iter()
        .any(|source| source == "agent_needs_present=remembered_belief:agent_state:needs_present"));

    let plan_failure = plan_local_actions(
        &actor_known_state,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_hidden_pantry".to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert!(
        plan_failure
            .trace
            .hidden_truth_audit_result
            .actor_known_only
    );
    assert_eq!(plan_failure.reason, "food source is not actor-known");

    let mut no_human_log = EventLog::new();
    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut no_human_log,
        &registry(),
        manifest_id.clone(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id.clone()],
            windows: vec![tracewake_core::scheduler::no_human::DayWindow {
                window_id: "hidden_truth_guard".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(8),
            }],
        },
    );
    assert!(report.ordinary_pipeline_events > 0);
    assert!(no_human_log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ActorWaited));
    for forbidden in ["food_hidden_pantry", "hidden_workshop", "workplace_hidden"] {
        assert!(!no_human_log.events().iter().any(|event| {
            event
                .ordering_key
                .target_ids
                .iter()
                .any(|target| target == forbidden)
                || event
                    .participants
                    .iter()
                    .any(|participant| participant == forbidden)
                || event
                    .payload
                    .iter()
                    .any(|field| field.value.contains(forbidden))
        }));
    }

    let mut log = EventLog::new();
    let eat = proposal(
        "proposal_forced_hidden_food",
        "actor_mara",
        "eat",
        &["food_hidden_pantry"],
        2,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &eat,
        0,
    );
    assert!(has_event(&log, EventKind::EatFailed));
    assert!(!has_event(&log, EventKind::FoodConsumed));
}

#[test]
fn no_human_day_fixture_has_roster_activity_and_metrics_envelope() {
    let golden = fixtures::no_human_day_001();
    let expected_roster = [
        "actor_anna".parse().unwrap(),
        "actor_elena".parse().unwrap(),
        "actor_mara".parse().unwrap(),
        "actor_tomas".parse().unwrap(),
    ];
    assert!(golden.contract.expected_events_or_reports.iter().any(
        |entry| entry.contains("expected_roster=actor_anna,actor_elena,actor_mara,actor_tomas")
    ));
    assert!(golden
        .contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry.contains("log_derived_metric=no_human_day_metrics_v1")));

    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id.clone(),
        NoHumanDayConfig {
            actor_ids: expected_roster.to_vec(),
            windows: default_day_windows(SimTick::ZERO),
        },
    );

    assert_eq!(report.actor_decision_order, expected_roster);
    assert!(report.ordinary_pipeline_events > 0);
    let actors_with_ordinary_actions = log
        .events()
        .iter()
        .filter(|event| {
            matches!(
                event.event_type,
                EventKind::ActorMoved
                    | EventKind::ActorWaited
                    | EventKind::FoodConsumed
                    | EventKind::EatFailed
                    | EventKind::SleepStarted
                    | EventKind::WorkBlockStarted
                    | EventKind::WorkBlockFailed
            )
        })
        .filter_map(|event| event.actor_id.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actors_with_ordinary_actions,
        expected_roster.into_iter().collect()
    );

    let eat_tomas = proposal(
        "proposal_day_tomas_eat",
        "actor_tomas",
        "eat",
        &["food_stew_home_tomas"],
        33,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &eat_tomas,
        100,
    );
    let eat_mara = proposal(
        "proposal_day_mara_empty_food",
        "actor_mara",
        "eat",
        &["food_empty_pantry_mara"],
        34,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &eat_mara,
        101,
    );

    let mut sleep_elena = proposal(
        "proposal_day_elena_sleep",
        "actor_elena",
        "sleep",
        &["home_elena"],
        35,
    );
    sleep_elena
        .parameters
        .insert("duration_ticks".to_string(), "4".to_string());
    sleep_elena
        .parameters
        .insert("sleep_place_id".to_string(), "home_elena".to_string());
    sleep_elena
        .parameters
        .insert("sleep_affordance_id".to_string(), "bed_elena".to_string());
    let sleep_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &sleep_elena,
        102,
    );
    let sleep_started = log
        .events()
        .iter()
        .find(|event| {
            event.event_type == EventKind::SleepStarted
                && event
                    .actor_id
                    .as_ref()
                    .is_some_and(|actor| actor.as_str() == "actor_elena")
        })
        .or_else(|| {
            sleep_events
                .iter()
                .find(|event| event.event_type == EventKind::SleepStarted)
        })
        .unwrap()
        .clone();
    append_and_apply(
        &mut state,
        &mut agent_state,
        &mut log,
        build_sleep_completion_events(
            &sleep_started,
            &ordering_key(&sleep_elena, 103),
            &manifest_id,
            SimTick::new(39),
        ),
    );

    let move_tomas_commons = proposal(
        "proposal_day_tomas_move_commons",
        "actor_tomas",
        "move",
        &["commons"],
        40,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &move_tomas_commons,
        104,
    );
    let move_tomas_workshop = proposal(
        "proposal_day_tomas_move_workshop",
        "actor_tomas",
        "move",
        &["workshop_tomas"],
        41,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &move_tomas_workshop,
        105,
    );
    let work_tomas = proposal(
        "proposal_day_tomas_work",
        "actor_tomas",
        "work_block",
        &["workplace_tomas"],
        42,
    );
    let work_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work_tomas,
        106,
    );
    let work_started = log
        .events()
        .iter()
        .find(|event| {
            event.event_type == EventKind::WorkBlockStarted
                && event
                    .actor_id
                    .as_ref()
                    .is_some_and(|actor| actor.as_str() == "actor_tomas")
        })
        .or_else(|| {
            work_events
                .iter()
                .find(|event| event.event_type == EventKind::WorkBlockStarted)
        })
        .unwrap()
        .clone();
    append_and_apply(
        &mut state,
        &mut agent_state,
        &mut log,
        build_work_completion_events(
            &work_started,
            &ordering_key(&work_tomas, 107),
            &manifest_id,
            SimTick::new(46),
        ),
    );

    let work_anna = proposal(
        "proposal_day_anna_blocked_work",
        "actor_anna",
        "work_block",
        &["workplace_anna_closed"],
        47,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work_anna,
        108,
    );

    let mut continue_tomas = proposal(
        "proposal_day_tomas_continue",
        "actor_tomas",
        "continue_routine",
        &[],
        48,
    );
    continue_tomas.parameters.insert(
        "active_intention_id".to_string(),
        "intention_day_tomas_work".to_string(),
    );
    continue_tomas
        .parameters
        .insert("next_action_id".to_string(), "work_block".to_string());
    continue_tomas
        .parameters
        .insert("intention_status".to_string(), "active".to_string());
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &continue_tomas,
        109,
    );

    let mut wait_anna = proposal(
        "proposal_day_anna_wait_crossing",
        "actor_anna",
        "wait",
        &[],
        49,
    );
    wait_anna
        .parameters
        .insert("ticks".to_string(), "100".to_string());
    wait_anna
        .parameters
        .insert("current_hunger".to_string(), "740".to_string());
    wait_anna.parameters.insert(
        "reason".to_string(),
        "canonical metrics crossing".to_string(),
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &wait_anna,
        110,
    );

    assert!(has_event(&log, EventKind::NoHumanDayStarted));
    assert!(has_event(&log, EventKind::NoHumanDayCompleted));
    assert!(has_event(&log, EventKind::FoodConsumed));
    assert!(has_event(&log, EventKind::EatFailed));
    assert!(has_event(&log, EventKind::SleepCompleted));
    assert!(has_event(&log, EventKind::ActorMoved));
    assert!(has_event(&log, EventKind::WorkBlockCompleted));
    assert!(has_event(&log, EventKind::WorkBlockFailed));
    assert!(has_event(&log, EventKind::ContinueRoutineProposed));
    assert!(has_event(&log, EventKind::NeedThresholdCrossed));

    let blocked = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockFailed)
        .unwrap();
    assert!(blocked
        .payload
        .iter()
        .any(|field| field.key == "blocker_kind" && field.value == "access"));
    let rendered = format!("{:?}", log.events()).to_ascii_lowercase();
    assert!(!rendered.contains("player"));
    assert!(!rendered.contains("controller"));

    let metrics = no_human_day_metrics(&log);
    assert_eq!(metrics.projection_version, "no_human_day_metrics_v1");
    assert!(metrics.events_per_day > 0);
    assert_eq!(
        metrics.routine_event_count,
        log.events()
            .iter()
            .filter(|event| matches!(
                event.event_type,
                EventKind::RoutineStepStarted
                    | EventKind::RoutineStepCompleted
                    | EventKind::RoutineStepFailed
                    | EventKind::ContinueRoutineAccepted
                    | EventKind::ContinueRoutineRejected
            ))
            .count()
    );
    assert!(metrics.meals_completed > 0);
    assert!(metrics.meals_missed > 0);
    assert!(metrics.sleep_completed > 0);
    assert!(metrics.work_blocks_completed > 0);
    assert!(metrics.work_blocks_failed > 0);
    assert!(metrics.need_threshold_crossings > 0);
    assert_eq!(metrics.player_conditioned_event_count, 0);
    assert_eq!(metrics.player_conditioned_event_rate_per_1000, 0);
}

#[test]
fn no_human_day_real_run_replays_metrics_and_trace_projection() {
    let golden = fixtures::no_human_day_001();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
    let expected_roster = state.actors().keys().cloned().collect::<Vec<_>>();

    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: expected_roster.clone(),
            windows: default_day_windows(SimTick::ZERO),
        },
    );
    let context = checksum_context("no_human_day_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let rebuild = rebuild_projection(
        &initial_state,
        &initial_agent_state,
        &log,
        &context,
        Some(&state),
    );
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &log,
        &context,
        Some(&state),
        Some(live_physical_checksum.clone()),
        Some(live_agent_checksum.clone()),
    );
    let canonical = log.serialize_canonical();
    let replayed_log = EventLog::deserialize_canonical(&canonical).unwrap();
    let real_metrics = no_human_day_metrics(&log).serialize_canonical();
    let replayed_metrics = no_human_day_metrics(&replayed_log).serialize_canonical();

    assert_eq!(report.actor_decision_order, expected_roster);
    assert!(report.ordinary_pipeline_events > 0);
    assert!(has_event(&log, EventKind::NoHumanDayStarted));
    assert!(has_event(&log, EventKind::NoHumanDayCompleted));
    assert!(has_event(&log, EventKind::SleepStarted));
    assert!(has_event(&log, EventKind::FoodConsumed) || has_event(&log, EventKind::EatFailed));
    assert!(has_event(&log, EventKind::ActorMoved));
    assert_eq!(rebuild.final_checksum, live_physical_checksum);
    assert_eq!(replayed_log.serialize_canonical(), canonical);
    assert_eq!(replayed_metrics, real_metrics);
    assert_eq!(replay.final_checksum, live_physical_checksum);
    assert_eq!(replay.final_agent_checksum, live_agent_checksum);
    assert_eq!(
        replay.expected_agent_checksum,
        Some(live_agent_checksum.clone())
    );
    assert!(replay.agent_checksum_matches);
    assert!(replay.epistemic_application_errors.is_empty());
    assert!(real_metrics.contains("no_human_day_metrics_v1"));
    assert_eq!(
        rebuild.final_agent_state.decision_traces(),
        agent_state.decision_traces()
    );
    assert_eq!(
        compute_agent_state_checksum(&agent_state, &context).checksum,
        live_agent_checksum
    );

    let missing_last =
        EventLog::deserialize_canonical(canonical.split(|byte| *byte == b'\n').next().unwrap())
            .unwrap();
    let corrupted = run_replay(
        &initial_state,
        &initial_agent_state,
        &missing_last,
        &checksum_context("no_human_day_001", &missing_last),
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );
    assert!(!corrupted.matches_expected);
    assert!(!corrupted.state_diff.is_empty() || !corrupted.application_errors.is_empty());
}

#[test]
fn no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash() {
    let golden = fixtures::no_human_observation_facts_cite_log_events_001();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let actor_id = ActorId::new("actor_bruno").unwrap();

    run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: vec![tracewake_core::scheduler::no_human::DayWindow {
                window_id: "observation_source_ids".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(8),
            }],
        },
    );

    let event_ids = log
        .events()
        .iter()
        .map(|event| event.event_id.as_str().to_string())
        .collect::<BTreeSet<_>>();
    let records = decision_trace_records(&log);
    assert!(!records.is_empty());
    for record in records {
        let recomputed = compute_holder_known_context_hash(record.actor_known_inputs.clone()).hash;
        assert_eq!(record.actor_known_context_hash, recomputed);
        assert!(!record.actor_known_inputs.is_empty());
        for input in &record.actor_known_inputs {
            let source_events = input
                .split('|')
                .find_map(|part| part.strip_prefix("source_events="))
                .expect("actor-known input records source_events");
            assert_ne!(source_events, "-");
            for event_id in source_events.split(',') {
                assert!(
                    event_ids.contains(event_id),
                    "actor-known input {input} cites missing event {event_id}"
                );
            }
        }
    }
}

#[test]
fn no_human_workplace_knowledge_requires_notice_channel() {
    let golden = fixtures::no_human_workplace_knowledge_requires_notice_event_001();
    let (mut state, mut agent_state, manifest_id, _) = load_with_log(golden);
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let mut log = EventLog::new();

    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &{
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_work();
            registry
        },
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: vec![tracewake_core::scheduler::no_human::DayWindow {
                window_id: "disabled_role_notice".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(8),
            }],
        },
    );

    assert_eq!(report.ordinary_pipeline_events, 0);
    assert!(!report.stuck_diagnostic_event_ids.is_empty());
    assert!(!has_event(&log, EventKind::WorkBlockStarted));
    assert!(log.events().iter().any(|event| event.event_type
        == EventKind::StuckDiagnosticRecorded
        && payload(event, "input_source") == Some("holder_known_context")));
}

#[test]
fn no_human_sleep_knowledge_requires_observation_or_record_channel() {
    let golden = fixtures::no_human_sleep_knowledge_requires_observation_or_record_001();
    let (mut state, mut agent_state, manifest_id, _) = load_with_log(golden);
    let actor_id = ActorId::new("actor_elena").unwrap();
    let mut log = EventLog::new();

    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &{
            let mut registry = ActionRegistry::new();
            registry.register_phase3a_sleep();
            registry
        },
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: vec![tracewake_core::scheduler::no_human::DayWindow {
                window_id: "disabled_sleep_observation".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(8),
            }],
        },
    );

    assert_eq!(report.ordinary_pipeline_events, 0);
    assert!(!report.stuck_diagnostic_event_ids.is_empty());
    assert!(!has_event(&log, EventKind::SleepStarted));
    assert!(log.events().iter().any(|event| event.event_type
        == EventKind::StuckDiagnosticRecorded
        && payload(event, "input_source") == Some("holder_known_context")));
}

#[test]
fn rejects_missing_ids_and_bad_references() {
    let report = validate_fixture_bytes(
        b"fixture|\nschema|schema_v1\nactor|actor_tomas|missing_place",
        &registry(),
    )
    .unwrap_err()
    .report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "missing_stable_id" || error.code == "bad_stable_id"));
}

#[test]
fn rejects_unsupported_action_targets() {
    let mut fixture = fixtures::strongbox_001().fixture;
    fixture
        .affordances
        .push(tracewake_content::schema::ActionAffordanceSchema {
            action_id: "open".parse().unwrap(),
            target_id: "coin_stack_01".to_string(),
        });

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "unsupported_action_target"));
}

#[test]
fn rejects_nondeterministic_ordering_hazards() {
    let mut fixture = fixtures::door_access_001().fixture;
    fixture.places[0].adjacent_place_ids =
        vec!["front_hall".parse().unwrap(), "back_room".parse().unwrap()];

    let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "non_canonical_ordering"));
}

#[test]
fn rejects_player_only_verbs() {
    let report = validate_fixture_bytes(
        b"fixture|bad_fixture\nschema|schema_v1\nplayer|actor_tomas",
        &registry(),
    )
    .unwrap_err()
    .report;

    assert!(report
        .errors
        .iter()
        .any(|error| error.code == "forbidden_form"));
}

#[test]
fn rejects_quest_and_script_content() {
    let report = validate_fixture_bytes(
        b"fixture|bad_fixture\nschema|schema_v1\nquest|q1\nreward|coins\non_open|force_event",
        &registry(),
    )
    .unwrap_err()
    .report;

    assert!(
        report
            .errors
            .iter()
            .filter(|error| error.code == "forbidden_form")
            .count()
            >= 2
    );
}

#[test]
fn llm_disabled_phase1_still_passes() {
    for golden in fixtures::all() {
        validate_fixture(&golden.fixture, &registry()).unwrap();
    }
}

#[test]
fn fixture_initial_beliefs_construct_epistemic_projection() {
    let golden = fixtures::strongbox_001();
    let projection = EpistemicProjection::from_initial_beliefs(
        ContentManifestId::new("manifest_strongbox_001").unwrap(),
        golden
            .fixture
            .initial_beliefs
            .iter()
            .map(|seed| seed.to_belief()),
    );

    assert!(projection.has_belief(
        &"belief_tomas_expects_coin_stack_01_in_strongbox_tomas"
            .parse()
            .unwrap()
    ));
    let context = KnowledgeContext::embodied("actor_tomas".parse().unwrap(), SimTick::ZERO);
    assert_eq!(projection.beliefs_for_context(&context).len(), 1);
}

#[test]
fn phase2a_golden_fixtures_have_contracts_and_validate() {
    let expected = [
        "strongbox_001",
        "expectation_contradiction_001",
        "possession_parity_001",
        "view_filtering_001",
        "knowledge_blocker_accuse_001",
        "sound_uncertainty_001",
        "no_human_epistemic_check_001",
    ];

    for fixture_id in expected {
        let golden = fixtures::all()
            .into_iter()
            .find(|golden| golden.fixture.fixture_id.as_str() == fixture_id)
            .unwrap_or_else(|| panic!("missing Phase 2A fixture {fixture_id}"));

        validate_fixture(&golden.fixture, &registry()).unwrap();
        assert_eq!(golden.contract.fixture_id, fixture_id);
        assert!(!golden.contract.setup.is_empty());
        assert!(!golden.contract.allowed_actions.is_empty());
        assert!(!golden.contract.expected_events_or_reports.is_empty());
        assert!(!golden.contract.acceptance_assertions.is_empty());
    }
}

#[test]
fn phase3a_golden_fixtures_have_contracts_and_validate() {
    let expected = [
        "ordinary_workday_001",
        "sleep_eat_work_001",
        "food_unavailable_replan_001",
        "routine_blocked_diagnostic_001",
        "planner_trace_001",
        "routine_no_teleport_001",
        "possession_does_not_reset_intention_001",
        "no_hidden_truth_planning_001",
        "no_human_day_001",
    ];

    for fixture_id in expected {
        let golden = fixtures::all()
            .into_iter()
            .find(|golden| golden.fixture.fixture_id.as_str() == fixture_id)
            .unwrap_or_else(|| panic!("missing Phase 3A fixture {fixture_id}"));

        validate_fixture(&golden.fixture, &registry()).unwrap();
        assert_eq!(golden.contract.fixture_id, fixture_id);
        assert!(!golden.contract.setup.is_empty());
        assert!(!golden.contract.allowed_actions.is_empty());
        assert!(!golden.contract.expected_events_or_reports.is_empty());
        assert!(!golden.contract.acceptance_assertions.is_empty());
    }
}

#[test]
fn phase2a_initial_beliefs_are_holder_and_source_backed() {
    for golden in fixtures::all()
        .into_iter()
        .filter(|golden| !golden.fixture.initial_beliefs.is_empty())
    {
        for seed in &golden.fixture.initial_beliefs {
            let belief = seed.to_belief();
            assert!(matches!(belief.holder(), HolderKind::Actor(_)));
            assert!(matches!(
                belief.source(),
                SourceRef::Event(_) | SourceRef::Action(_) | SourceRef::Cause(_)
            ));
            assert!(!belief.belief_id().as_str().is_empty());
            assert!(!belief.proposition().render().is_empty());
        }
    }
}

#[test]
fn phase2a_validation_rejects_shortcut_truth_fields() {
    let report = validate_fixture_bytes(
        b"fixture|bad_phase2a_fixture\nschema|schema_v1\nculprit|actor_mara\nstolen_flag|true\nnpc_knows_truth|actor_tomas\nplayer_memory|coin_stack_01",
        &registry(),
    )
    .unwrap_err()
    .report;

    for forbidden in ["culprit", "stolen_flag", "npc_knows_truth", "player_memory"] {
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.code == "forbidden_form" && error.path.contains(forbidden)),
            "missing forbidden-form validation for {forbidden}: {report:?}"
        );
    }
}
