use std::collections::{BTreeMap, BTreeSet};

use tracewake_content::fixtures;
use tracewake_content::fixtures::GoldenFixture;
use tracewake_content::load::load_fixture_package;
use tracewake_content::serialization::{
    deserialize_event_log, deserialize_fixture, serialize_event_log, serialize_fixture,
};
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes};
use tracewake_core::actions::defs::sleep::build_sleep_completion_events;
use tracewake_core::actions::defs::work::build_work_completion_events;
use tracewake_core::actions::pipeline::{run_pipeline, PipelineContext};
use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
use tracewake_core::actions::{ActionRegistry, ReasonCode, ReportStatus};
use tracewake_core::agent::{
    generate_candidate_goals, plan_local_actions, record_current_place_perception_and_project,
    select_goal_and_trace, select_method_from_templates, ActorDecisionTransaction,
    ActorDecisionTransactionInput, ActorDecisionTransactionOutcome, ActorKnownFact,
    BlockerCategory, CandidateGenerationInput, DecisionInput, DecisionTraceRecord, GoalKind,
    LocalPlanRequest, NeedChangeCause, NeedKind, NeedState, NoHumanActorKnownSurfaceBuilder,
    NoHumanActorKnownSurfaceRequest, PlannerGoal, ResponsibleLayer, RoutineCondition,
    RoutineFamily, RoutineStep, RoutineTemplate, SourceEventIds,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_holder_known_context_hash, compute_physical_checksum,
    ChecksumContext,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::epistemics::{
    Channel, Confidence, EpistemicProjection, HolderKind, PrivacyScope, Proposition, SourceRef,
    Stance,
};
use tracewake_core::events::apply::{apply_event, apply_event_stream, EventApplicationContext};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventEnvelope, EventKind, EventStream, InitialBeliefSourceKind, PayloadField,
};
use tracewake_core::ids::{
    ActorId, BeliefId, ContentManifestId, ContentVersion, ControllerId, EventId, FixtureId,
    FoodSupplyId, IntentionId, PlaceId, RoutineExecutionId, RoutineTemplateId, SchemaVersion,
};
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::{rebuild_decision_context_hashes, rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{
    default_day_windows, run_no_human_day, DayWindow, NoHumanDayConfig,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{AgentState, ControllerMode, PhysicalState};
use tracewake_core::time::SimTick;

const FROZEN_FIXTURE_FINGERPRINTS: &[(&str, &str)] = &[
    (
        "aged_food_record_surfaces_as_remembered_belief_not_observation_001",
        "twf1-5594c16c721cde82",
    ),
    ("container_item_move_001", "twf1-bc90d56dcf16b9d3"),
    ("debug_attach_001", "twf1-7818100e3c7b9fd1"),
    ("debug_omniscience_excluded_001", "twf1-87f6032bf6fbdfba"),
    ("door_access_001", "twf1-eb38689018fcec34"),
    (
        "embodied_exits_require_perceived_or_known_route_001",
        "twf1-2fd542c243164d0b",
    ),
    (
        "embodied_menu_lags_truth_change_without_perception_001",
        "twf1-645af09b6ae93a2c",
    ),
    (
        "embodied_view_omits_raw_assignment_without_context_001",
        "twf1-3513f06b88932a09",
    ),
    (
        "embodied_view_omits_unknown_sleep_affordance_001",
        "twf1-f8871d3f1313e7a6",
    ),
    (
        "embodied_view_omits_unobserved_food_at_open_place_001",
        "twf1-3f2c6b594ec18991",
    ),
    (
        "embodied_workplace_availability_reflects_belief_not_truth_001",
        "twf1-ca46cff100748efa",
    ),
    (
        "embodied_workplace_believed_open_truth_closed_commit_fails_001",
        "twf1-bb58577539f8967c",
    ),
    ("expectation_contradiction_001", "twf1-f3c4a068a9286fb2"),
    ("food_unavailable_replan_001", "twf1-1fcc4fb4b016d64f"),
    (
        "forbidden_provenance_input_fails_closed_001",
        "twf1-6487b387a0259f60",
    ),
    ("hidden_food_closed_container_001", "twf1-c08091b0fdf1f677"),
    ("hidden_food_unknown_route_001", "twf1-bc29e64a91f1bfc8"),
    ("hidden_route_edge_001", "twf1-935cce05feb5969c"),
    (
        "hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001",
        "twf1-543763eed4a84dc6",
    ),
    ("knowledge_blocker_accuse_001", "twf1-df413503a3b3eb4b"),
    (
        "method_fallback_requires_new_trace_or_stuck_001",
        "twf1-8b1b9fa49c3dc51e",
    ),
    ("no_hidden_truth_planning_001", "twf1-cd857919c1f8d27d"),
    ("no_human_advance_001", "twf1-9f2afd3fa905f659"),
    (
        "no_human_current_place_without_sleep_affordance_does_not_sleep_001",
        "twf1-5a92aa16a492fada",
    ),
    ("no_human_day_001", "twf1-9b8d796c5201bb66"),
    ("no_human_epistemic_check_001", "twf1-82d3a78edaaa1eef"),
    (
        "no_human_known_workplace_requires_provenance_001",
        "twf1-7e9256e156e8a091",
    ),
    (
        "no_human_metrics_require_typed_responsible_layer_001",
        "twf1-5bda1b2ce1329896",
    ),
    (
        "no_human_observation_facts_cite_log_events_001",
        "twf1-d08ba3324093602b",
    ),
    (
        "no_human_sleep_knowledge_requires_observation_or_record_001",
        "twf1-d6b80f53c7cee79a",
    ),
    (
        "no_human_unseen_workplace_assignment_does_not_plan_work_001",
        "twf1-bff52880b0b738ac",
    ),
    (
        "no_human_workplace_knowledge_requires_notice_event_001",
        "twf1-8e18a768d762d556",
    ),
    ("ordinary_workday_001", "twf1-9759d1e7c395df11"),
    ("partial_food_source_knowledge_001", "twf1-db3392dae3200877"),
    ("planner_trace_001", "twf1-f8a39fef166adb45"),
    (
        "possession_does_not_reset_intention_001",
        "twf1-94b19dd69e937a66",
    ),
    ("possession_parity_001", "twf1-3acf525712657954"),
    ("replay_item_location_001", "twf1-5f2d9b53aa902800"),
    ("routine_blocked_diagnostic_001", "twf1-83b07670e067fdad"),
    ("routine_no_teleport_001", "twf1-82ec55a58e8a4cf0"),
    (
        "scheduler_cannot_rewrite_wait_reason_after_transaction_001",
        "twf1-06d98abee9bc534e",
    ),
    (
        "seeded_food_source_unknown_to_all_actors_001",
        "twf1-335eca617f0ebd12",
    ),
    (
        "severe_safety_with_known_exit_produces_move_001",
        "twf1-410c96361e4bf722",
    ),
    (
        "severe_safety_without_known_exit_waits_with_knowledge_blocker_001",
        "twf1-1d1f53cd0a6baef7",
    ),
    ("sleep_eat_work_001", "twf1-f77b963ad0e7de01"),
    (
        "sleep_interrupted_by_severe_need_prorates_recovery_001",
        "twf1-9dc4012085bdf089",
    ),
    (
        "sleep_rejects_current_place_without_sleep_affordance_001",
        "twf1-827318a9f91a9c1c",
    ),
    (
        "sleep_spanning_window_boundary_charges_each_tick_once_001",
        "twf1-78762745528d5dd2",
    ),
    ("sound_uncertainty_001", "twf1-a8bcbc95e6ae13e3"),
    (
        "stale_workplace_notice_superseded_by_newer_001",
        "twf1-ecb18402c595cb98",
    ),
    ("strongbox_001", "twf1-c89a51158996ae87"),
    ("view_filtering_001", "twf1-02b96aa6cd71dec0"),
    ("view_model_local_actions_001", "twf1-d1a3bea039a51232"),
    (
        "wait_then_window_passive_charges_each_tick_once_001",
        "twf1-fdfca760dcde678d",
    ),
    (
        "work_block_failed_then_sleep_succeeds_001",
        "twf1-0b7637da136627bc",
    ),
    (
        "work_completion_fails_when_actor_displaced_001",
        "twf1-59dea7ebbb271d5f",
    ),
    (
        "workplace_assignment_provenance_001",
        "twf1-13bb06631b702540",
    ),
];

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

fn frozen_fixture_fingerprint_errors(expected: &BTreeMap<&str, &str>) -> Vec<String> {
    let mut errors = Vec::new();
    let mut actual = BTreeMap::new();
    for golden in fixtures::all() {
        let fixture_id = golden.fixture.fixture_id.as_str().to_string();
        let loaded = load_fixture_package(
            ContentManifestId::new(format!("manifest_{fixture_id}")).unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        actual.insert(fixture_id, loaded.manifest.content_fingerprint);
    }

    for fixture_id in expected.keys() {
        if !actual.contains_key(*fixture_id) {
            errors.push(format!(
                "frozen fingerprint table has unknown fixture {fixture_id}"
            ));
        }
    }
    for (fixture_id, fingerprint) in &actual {
        match expected.get(fixture_id.as_str()) {
            Some(expected_fingerprint) if expected_fingerprint == fingerprint => {}
            Some(expected_fingerprint) => errors.push(format!(
                "{fixture_id} fingerprint changed: expected {expected_fingerprint}, actual {fingerprint}"
            )),
            None => errors.push(format!(
                "{fixture_id} missing frozen fingerprint {fingerprint}"
            )),
        }
    }
    errors
}

#[test]
fn fixture_fingerprints_match_frozen_goldens() {
    let expected = FROZEN_FIXTURE_FINGERPRINTS
        .iter()
        .copied()
        .collect::<BTreeMap<_, _>>();
    let errors = frozen_fixture_fingerprint_errors(&expected);
    assert!(
        errors.is_empty(),
        "fixture fingerprint goldens diverged:\n{}",
        errors.join("\n")
    );

    let mut mismatched = expected.clone();
    let first_fixture_id = expected
        .keys()
        .next()
        .copied()
        .expect("frozen fingerprint table is non-empty");
    mismatched.insert(first_fixture_id, "twf1-0000000000000000");
    assert!(
        frozen_fixture_fingerprint_errors(&mismatched)
            .iter()
            .any(|error| error.contains(first_fixture_id)),
        "synthetic mismatched manifest fingerprint must fail the frozen seam"
    );
}

#[test]
fn serialization_round_trips_spine_variant_matrix() {
    let mut fixture = fixtures::strongbox_001().fixture;
    fixture.fixture_scope = tracewake_content::schema::FixtureScope::Phase3AHistorical;

    for (index, (channel, stance, privacy_scope, last_verified_tick)) in [
        (
            Some(Channel::TouchOrSearch),
            Stance::BelievesTrue,
            PrivacyScope::ActorPrivate(ActorId::new("actor_tomas").unwrap()),
            None,
        ),
        (
            Some(Channel::AbsenceMarker),
            Stance::BelievesFalse,
            PrivacyScope::PublicPlaceholder,
            Some(SimTick::new(17)),
        ),
        (
            Some(Channel::ReadingPlaceholderSchemaOnly),
            Stance::Doubts,
            PrivacyScope::InstitutionPlaceholder("village_notice_board".to_string()),
            Some(SimTick::new(42)),
        ),
        (
            Some(Channel::DirectSight),
            Stance::UnknownOrUnresolved,
            PrivacyScope::ActorPrivate(ActorId::new("actor_tomas").unwrap()),
            Some(SimTick::new(99)),
        ),
        (
            None,
            Stance::ExpectsTrue,
            PrivacyScope::PublicPlaceholder,
            None,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        fixture
            .initial_beliefs
            .push(tracewake_content::schema::InitialBeliefSchema {
                belief_id: BeliefId::new(format!("belief_serialization_variant_{index}")).unwrap(),
                holder_actor_id: ActorId::new("actor_tomas").unwrap(),
                proposition: Proposition::ItemLocatedAtPlace {
                    item_id: "coin_stack_01".parse().unwrap(),
                    place_id: "shop_front".parse().unwrap(),
                },
                stance,
                confidence: Confidence::new(700 + index as u16).unwrap(),
                source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
                source: SourceRef::Event(
                    EventId::new(format!("prehistory_serialization_variant_{index}")).unwrap(),
                ),
                channel,
                acquired_tick: SimTick::new(index as u64),
                last_verified_tick,
                privacy_scope,
                schema_version: SchemaVersion::new("epistemic_record_v1").unwrap(),
            });
    }

    let routine_families = [
        (RoutineFamily::MorningWake, vec![]),
        (RoutineFamily::EatMeal, vec![7]),
        (RoutineFamily::GoToWork, vec![2, 5, 9]),
        (RoutineFamily::WorkBlock, vec![3]),
        (RoutineFamily::ReturnHome, vec![4]),
        (RoutineFamily::SleepNight, vec![6]),
        (RoutineFamily::FindFood, vec![8]),
        (RoutineFamily::LeaveUnsafePlace, vec![10]),
        (RoutineFamily::ContinueCurrentIntention, vec![11]),
        (RoutineFamily::Wait, vec![12]),
        (RoutineFamily::IdleWithReason, vec![13]),
    ];
    for (index, (family, interruption_points)) in routine_families.iter().enumerate() {
        fixture
            .routine_templates
            .push(tracewake_content::schema::RoutineTemplateSchema {
                template_id: RoutineTemplateId::new(format!("routine_serialization_{index}"))
                    .unwrap(),
                family: *family,
                applicability_conditions: vec![RoutineCondition::ActorKnowsHome],
                preconditions: Vec::new(),
                steps: vec![RoutineStep::WaitUntil {
                    reason: format!("serialization_{index}"),
                }],
                min_duration_ticks: 1,
                max_duration_ticks: 3,
                interruption_points: interruption_points.clone(),
                failure_modes: vec![format!("failure_{index}")],
                fallback_rules: vec![format!("fallback_{index}")],
                debug_labels: vec![format!("debug_{index}")],
                reservable_resource: if index % 2 == 0 {
                    Some(format!("resource_{index}"))
                } else {
                    None
                },
            });
    }
    fixture.canonicalize();

    let bytes = serialize_fixture(&fixture);
    let parsed = deserialize_fixture(&bytes).unwrap();
    let bytes_after = serialize_fixture(&parsed);

    assert_eq!(bytes, bytes_after);
    assert_eq!(
        parsed
            .initial_beliefs
            .iter()
            .map(|belief| (
                belief.channel,
                belief.stance,
                belief.privacy_scope.clone(),
                belief.last_verified_tick
            ))
            .collect::<Vec<_>>(),
        fixture
            .initial_beliefs
            .iter()
            .map(|belief| (
                belief.channel,
                belief.stance,
                belief.privacy_scope.clone(),
                belief.last_verified_tick
            ))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        parsed
            .routine_templates
            .iter()
            .map(|template| (template.family, template.interruption_points.clone()))
            .collect::<Vec<_>>(),
        fixture
            .routine_templates
            .iter()
            .map(|template| (template.family, template.interruption_points.clone()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn serialized_event_log_replays_to_identical_state() {
    let (initial_state, initial_agent_state, final_state, final_agent_state, log) =
        sleep_eat_work_replay_fixture();
    assert!(!log.events().is_empty());

    let bytes = serialize_event_log(&log);
    assert!(
        !bytes.is_empty(),
        "nonempty event logs must serialize to nonempty canonical bytes"
    );
    let round_tripped_log = deserialize_event_log(&bytes).unwrap();
    assert_eq!(round_tripped_log.events().len(), log.events().len());
    assert_eq!(serialize_event_log(&round_tripped_log), bytes);

    let context = checksum_context("sleep_eat_work_001", &round_tripped_log);
    let physical_checksum = compute_physical_checksum(&final_state, &context).checksum;
    let agent_checksum = compute_agent_state_checksum(&final_agent_state, &context).checksum;
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &round_tripped_log,
        &context,
        Some(&final_state),
        Some(physical_checksum.clone()),
        Some(agent_checksum.clone()),
    );

    assert_eq!(replay.final_checksum, physical_checksum);
    assert_eq!(replay.final_agent_checksum, agent_checksum);
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

fn actor_known_context_from_projection(
    projection: &EpistemicProjection,
    agent_state: &AgentState,
    actor_id: ActorId,
    current_place_id: PlaceId,
    decision_tick: SimTick,
) -> tracewake_core::agent::ActorKnownPlanningState {
    let frame_event_id = EventId::new(format!(
        "event_test_frame_{}_{}",
        actor_id.as_str(),
        decision_tick.value()
    ))
    .unwrap();
    NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
        projection,
        agent_state,
        actor_id,
        current_place_id,
        decision_tick,
        window_id: "test_window",
        window_end_tick: SimTick::new(decision_tick.value() + 4),
        current_place_witness_event_id: Some(frame_event_id.clone()),
        needs_witness_event_id: Some(frame_event_id.clone()),
        frame_event_id: Some(frame_event_id),
    })
    .build(agent_state)
    .into_context()
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

fn has_no_human_event(log: &EventLog, kind: EventKind) -> bool {
    log.events().iter().any(|event| {
        event.event_type == kind && event.ordering_key.phase == SchedulePhase::NoHumanProcess
    })
}

fn has_no_human_event_for_actor(log: &EventLog, kind: EventKind, actor_id: &str) -> bool {
    log.events().iter().any(|event| {
        event.event_type == kind
            && event.ordering_key.phase == SchedulePhase::NoHumanProcess
            && event
                .actor_id
                .as_ref()
                .is_some_and(|event_actor_id| event_actor_id.as_str() == actor_id)
    })
}

fn canonical_mara_recovery_resolution(golden: &GoldenFixture) -> Option<&str> {
    golden
        .contract
        .expected_events_or_reports
        .iter()
        .find_map(|entry| entry.strip_prefix("canonical_mara_recovery_resolution="))
}

fn mara_recovery_resolution_errors(resolution: &str, log: &EventLog) -> Vec<String> {
    let mut errors = Vec::new();
    match resolution {
        "fail_only_empty_food_source" => {
            if !has_no_human_event_for_actor(log, EventKind::EatFailed, "actor_mara") {
                errors.push("fail-only Mara recovery lacks no-human EatFailed".to_string());
            }
            if has_no_human_event_for_actor(log, EventKind::FoodConsumed, "actor_mara") {
                errors.push("fail-only Mara recovery consumed food".to_string());
            }
            if log.events().iter().any(|event| {
                event.event_type == EventKind::ActorMoved
                    && event.ordering_key.phase == SchedulePhase::NoHumanProcess
                    && event
                        .actor_id
                        .as_ref()
                        .is_some_and(|actor_id| actor_id.as_str() == "actor_mara")
                    && payload(event, "to_place_id") == Some("home_tomas")
            }) {
                errors.push("fail-only Mara recovery moved toward Tomas food".to_string());
            }
            if log.events().iter().any(|event| {
                event.ordering_key.phase == SchedulePhase::NoHumanProcess
                    && event
                        .actor_id
                        .as_ref()
                        .is_some_and(|actor_id| actor_id.as_str() == "actor_mara")
                    && payload(event, "food_supply_id") == Some("food_stew_home_tomas")
            }) {
                errors.push("fail-only Mara recovery targeted Tomas food".to_string());
            }
        }
        other => errors.push(format!(
            "unsupported canonical_mara_recovery_resolution token {other}"
        )),
    }
    errors
}

fn payload<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn assert_no_duplicate_need_regime_charges(log: &EventLog) {
    let mut start_kind_by_event = BTreeMap::new();
    for event in log.events() {
        if matches!(
            event.event_type,
            EventKind::SleepStarted | EventKind::WorkBlockStarted
        ) {
            start_kind_by_event.insert(event.event_id.clone(), event.event_type);
        }
    }
    let mut charged = BTreeMap::<(String, String, u64), u64>::new();
    for event in log
        .events()
        .iter()
        .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
    {
        let Some(actor_id) = event.actor_id.as_ref() else {
            continue;
        };
        let Some(need_kind) = payload(event, "need_kind") else {
            continue;
        };
        let Some(cause_kind) = payload(event, "cause_kind") else {
            continue;
        };
        match cause_kind {
            "tick_delta" => {
                let elapsed_ticks = payload(event, "elapsed_ticks")
                    .and_then(|value| value.parse::<u64>().ok())
                    .expect("tick_delta need charge carries elapsed_ticks");
                assert!(elapsed_ticks > 0, "tick_delta charge interval is positive");
                for tick in event
                    .sim_tick
                    .value()
                    .saturating_sub(elapsed_ticks)
                    .saturating_add(1)..=event.sim_tick.value()
                {
                    charged
                        .entry((actor_id.to_string(), need_kind.to_string(), tick))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            "action_effect" => {
                let regime = event.causes.iter().find_map(|cause| match cause {
                    tracewake_core::events::EventCause::Event(start_id) => {
                        match start_kind_by_event.get(start_id) {
                            Some(EventKind::SleepStarted) => Some("asleep"),
                            Some(EventKind::WorkBlockStarted) => Some("working"),
                            _ => None,
                        }
                    }
                    _ => None,
                });
                let Some(_regime) = regime else {
                    continue;
                };
                let elapsed_ticks = payload(event, "elapsed_ticks")
                    .and_then(|value| value.parse::<u64>().ok())
                    .expect("action_effect need charge carries elapsed_ticks");
                assert!(
                    elapsed_ticks > 0,
                    "action_effect duration charge interval is positive"
                );
                for tick in event
                    .sim_tick
                    .value()
                    .saturating_sub(elapsed_ticks)
                    .saturating_add(1)..=event.sim_tick.value()
                {
                    charged
                        .entry((actor_id.to_string(), need_kind.to_string(), tick))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            _ => {}
        }
    }
    let duplicates = charged
        .iter()
        .filter(|(_, count)| **count > 1)
        .collect::<Vec<_>>();
    assert!(
        duplicates.is_empty(),
        "duplicate need charges: {duplicates:?}"
    );
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

fn tamper_first_decision_trace_source_event(
    log: &EventLog,
    replacement_event_id: &str,
) -> EventLog {
    let mut tampered = EventLog::new();
    let mut replaced = false;
    for event in log.events() {
        let mut event = event.clone();
        if !replaced && event.event_type == EventKind::DecisionTraceRecorded {
            let trace_canonical = event
                .payload
                .iter_mut()
                .find(|field| field.key == "trace_canonical")
                .expect("decision trace event carries trace_canonical");
            let mut record =
                DecisionTraceRecord::deserialize_canonical(trace_canonical.value.as_bytes())
                    .expect("decision trace canonical parses before tamper");
            let first_input = record
                .actor_known_inputs
                .first_mut()
                .expect("decision trace records actor-known inputs");
            *first_input = replace_first_source_event(first_input, replacement_event_id);
            record.actor_known_context_hash =
                Some(compute_holder_known_context_hash(record.actor_known_inputs.clone()).hash);
            trace_canonical.value = record.serialize_canonical();
            replaced = true;
        }
        tampered.append(event).unwrap();
    }
    assert!(replaced, "fixture contains decision trace event");
    tampered
}

fn replace_first_source_event(input: &str, replacement_event_id: &str) -> String {
    let mut parts = input
        .split('|')
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    for part in &mut parts {
        let Some(source_events) = part.strip_prefix("source_events=") else {
            continue;
        };
        let suffix = source_events
            .split_once(',')
            .map(|(_, rest)| format!(",{rest}"))
            .unwrap_or_default();
        *part = format!("source_events={replacement_event_id}{suffix}");
        return parts.join("|");
    }
    panic!("actor-known input records source_events");
}

fn tamper_first_continue_routine_kind(log: &EventLog, replacement_kind: EventKind) -> EventLog {
    let mut tampered = EventLog::new();
    let mut replaced = false;
    for event in log.events() {
        let mut event = event.clone();
        if !replaced && event.event_type == EventKind::ContinueRoutineProposed {
            event.event_type = replacement_kind;
            replaced = true;
        }
        tampered.append(event).unwrap();
    }
    assert!(
        replaced,
        "fixture contains continue-routine arbitration event"
    );
    tampered
}

fn tamper_first_continue_routine_reason(log: &EventLog, replacement_reason: &str) -> EventLog {
    let mut tampered = EventLog::new();
    let mut replaced = false;
    for event in log.events() {
        let mut event = event.clone();
        if !replaced && event.event_type == EventKind::ContinueRoutineProposed {
            if let Some(reason) = event.payload.iter_mut().find(|field| field.key == "reason") {
                reason.value = replacement_reason.to_string();
            } else {
                event
                    .payload
                    .push(PayloadField::new("reason", replacement_reason));
            }
            replaced = true;
        }
        tampered.append(event).unwrap();
    }
    assert!(
        replaced,
        "fixture contains continue-routine arbitration event"
    );
    tampered
}

fn tamper_first_payload_field(
    log: &EventLog,
    event_kind: EventKind,
    key: &str,
    replacement_value: &str,
) -> EventLog {
    let mut tampered = EventLog::new();
    let mut replaced = false;
    for event in log.events() {
        let mut event = event.clone();
        if !replaced && event.event_type == event_kind {
            let field = event
                .payload
                .iter_mut()
                .find(|field| field.key == key)
                .unwrap_or_else(|| panic!("{} carries {key}", event_kind.stable_id()));
            field.value = replacement_value.to_string();
            replaced = true;
        }
        tampered.append(event).unwrap();
    }
    assert!(
        replaced,
        "fixture contains {} payload field {key}",
        event_kind.stable_id()
    );
    tampered
}

fn possession_continue_routine_replay_fixture() -> (
    PhysicalState,
    AgentState,
    PhysicalState,
    AgentState,
    EventLog,
) {
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
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
    let mut log = EventLog::new();
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
    (initial_state, initial_agent_state, state, agent_state, log)
}

fn sleep_eat_work_replay_fixture() -> (
    PhysicalState,
    AgentState,
    PhysicalState,
    AgentState,
    EventLog,
) {
    let (mut state, mut agent_state, manifest_id) = load(fixtures::sleep_eat_work_001());
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
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
    let completion_events = build_sleep_completion_events(
        &state,
        &agent_state,
        &log,
        &sleep_started,
        &ordering_key(&sleep, 1),
        &manifest_id,
        SimTick::new(4),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
    let completion_events = build_work_completion_events(
        &state,
        &agent_state,
        &log,
        &work_started,
        &ordering_key(&work, 5),
        &manifest_id,
        SimTick::new(11),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

    (initial_state, initial_agent_state, state, agent_state, log)
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
    let completion_events = build_work_completion_events(
        &state,
        &agent_state,
        &log,
        &work_started,
        &ordering_key(&work, 2),
        &manifest_id,
        SimTick::new(12),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
    let completion_events = build_sleep_completion_events(
        &state,
        &agent_state,
        &log,
        &sleep_started,
        &ordering_key(&sleep, 1),
        &manifest_id,
        SimTick::new(4),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
    let completion_events = build_work_completion_events(
        &state,
        &agent_state,
        &log,
        &work_started,
        &ordering_key(&work, 5),
        &manifest_id,
        SimTick::new(11),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
fn work_block_failed_then_sleep_succeeds_fixture_closes_reservation() {
    let (mut state, mut agent_state, manifest_id) =
        load(fixtures::work_block_failed_then_sleep_succeeds_001());
    let mut log = EventLog::new();

    let work = proposal(
        "proposal_failed_then_sleep_work",
        "actor_tomas",
        "work_block",
        &["workplace_shop"],
        0,
    );
    let work_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &work,
        0,
    );
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .expect("work starts")
        .clone();

    let move_to_street = proposal(
        "proposal_failed_then_sleep_move",
        "actor_tomas",
        "move",
        &["street"],
        1,
    );
    run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &move_to_street,
        1,
    );

    let completion_events = build_work_completion_events(
        &state,
        &agent_state,
        &log,
        &work_started,
        &ordering_key(&work, 2),
        &manifest_id,
        SimTick::new(4),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);
    assert!(has_event(&log, EventKind::WorkBlockFailed));

    let mut sleep = proposal(
        "proposal_failed_then_sleep_sleep",
        "actor_tomas",
        "sleep",
        &["sleep_place_street"],
        5,
    );
    sleep
        .parameters
        .insert("duration_ticks".to_string(), "2".to_string());
    sleep
        .parameters
        .insert("sleep_place_id".to_string(), "street".to_string());
    sleep.parameters.insert(
        "sleep_affordance_id".to_string(),
        "sleep_place_street".to_string(),
    );
    let sleep_events = run(
        &mut state,
        &mut agent_state,
        &mut log,
        &manifest_id,
        &sleep,
        3,
    );

    assert!(sleep_events
        .iter()
        .any(|event| event.event_type == EventKind::SleepStarted));
    assert!(!sleep_events.iter().any(|event| {
        event.event_type == EventKind::ActionRejected
            && payload(event, "reason_codes").is_some_and(|value| {
                value
                    .split(',')
                    .any(|reason| reason == ReasonCode::ReservationConflict.stable_id())
            })
    }));
    let replayed = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    assert_eq!(replayed, log);
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
    let golden = fixtures::planner_trace_001();
    let manifest_id =
        ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str())).unwrap();
    let loaded = load_fixture_package(
        manifest_id.clone(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    let agent_state = loaded.canonical_agent_state;
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
            SourceEventIds::checked(vec![EventId::new("event_planner_trace_food").unwrap()])
                .unwrap(),
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
    let actor_known_state = actor_known_context_from_projection(
        &loaded.epistemic_projection,
        &agent_state,
        actor_id.clone(),
        "home_tomas".parse().unwrap(),
        SimTick::new(2),
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
fn severe_safety_with_known_exit_produces_move_and_replays() {
    let golden = fixtures::severe_safety_with_known_exit_produces_move_001();
    let (initial_state, initial_agent_state, manifest_id, mut log) = load_with_log(golden);
    let mut state = initial_state.clone();
    let mut agent_state = initial_agent_state.clone();
    let actor_id = ActorId::new("actor_mara").unwrap();

    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id.clone()],
            windows: vec![DayWindow {
                window_id: "safety_window".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );

    assert_eq!(report.actor_decision_order, vec![actor_id]);
    let moved = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::ActorMoved)
        .expect("severe safety should commit a move");
    assert_eq!(payload(moved, "to_place_id"), Some("safety_corridor"));
    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::DecisionTraceRecorded
            && payload(event, "trace_canonical")
                .is_some_and(|canonical| canonical.contains("leave_unsafe_place"))
    }));

    let context = checksum_context("severe_safety_with_known_exit_produces_move_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &log,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );
    assert!(replay.matches_expected, "{replay:?}");
    assert!(replay.agent_checksum_matches, "{replay:?}");
    assert!(replay.application_errors.is_empty(), "{replay:?}");
    assert!(replay.agent_application_errors.is_empty(), "{replay:?}");
    assert!(
        replay.decision_context_hash_failures.is_empty(),
        "{replay:?}"
    );
}

#[test]
fn severe_safety_without_known_exit_is_local_knowledge_blocker() {
    let golden = fixtures::severe_safety_without_known_exit_waits_with_knowledge_blocker_001();
    let (state, agent_state, _manifest_id) = load(golden);
    let actor_id = ActorId::new("actor_mara").unwrap();
    let current_place = state
        .actors()
        .get(&actor_id)
        .expect("fixture actor exists")
        .current_place_id
        .clone();
    let empty_projection = EpistemicProjection::new(_manifest_id.clone());
    let actor_known_context = actor_known_context_from_projection(
        &empty_projection,
        &agent_state,
        actor_id.clone(),
        current_place,
        SimTick::ZERO,
    );

    let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
        actor_id,
        decision_tick: SimTick::ZERO,
        agent_state: &agent_state,
        actor_known_context: &actor_known_context,
        source_event_ids: None,
        source_event_kinds: None,
        routine_window_family: None,
        include_idle_fallback: true,
    });

    let ActorDecisionTransactionOutcome::Stuck { diagnostic } = outcome else {
        panic!("expected no-exit severe safety to fail closed");
    };
    assert_eq!(diagnostic.blocker_category, BlockerCategory::Knowledge);
    assert_eq!(
        diagnostic.concrete_blocker,
        "no actor-known exit from unsafe place"
    );
    assert_eq!(
        diagnostic.typed_diagnostic.responsible_layer,
        ResponsibleLayer::LocalPlanning
    );
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
    assert_eq!(agent_state.intentions(), before_agent_state.intentions());
    assert_eq!(
        agent_state.routine_executions(),
        before_agent_state.routine_executions()
    );
    assert_eq!(
        agent_state.needs_by_actor(),
        before_agent_state.needs_by_actor()
    );
    assert_eq!(agent_state.continue_routine_arbitrations().len(), 1);
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
    let actor_known_state = actor_known_context_from_projection(
        &epistemic_projection,
        &agent_state,
        actor_id.clone(),
        "home_mara".parse().unwrap(),
        SimTick::new(1),
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
fn partial_food_source_knowledge_seeds_only_authored_actor_edge() {
    let golden = fixtures::partial_food_source_knowledge_001();
    let (state, agent_state, _manifest_id, log) = load_with_log(golden);
    let mara: ActorId = "actor_mara".parse().unwrap();
    let tomas: ActorId = "actor_tomas".parse().unwrap();

    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::StartingBeliefRecorded
            && payload(event, "actor_id") == Some("actor_mara")
            && payload(event, "belief_kind") == Some("household_food_source")
            && payload(event, "subject_id") == Some("food_shared_pantry")
    }));
    assert!(!log.events().iter().any(|event| {
        event.event_type == EventKind::StartingBeliefRecorded
            && payload(event, "actor_id") == Some("actor_tomas")
            && payload(event, "belief_kind") == Some("household_food_source")
            && payload(event, "subject_id") == Some("food_shared_pantry")
    }));

    let rebuild = rebuild_projection(
        &state,
        &AgentState::default(),
        &log,
        &checksum_context("partial_food_source_knowledge_001", &log),
        Some(&state),
    );
    assert!(rebuild.invariant_violations.is_empty());
    let epistemic_projection = rebuild.final_epistemic_projection;

    let mara_surface =
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection: &epistemic_projection,
            agent_state: &agent_state,
            actor_id: mara,
            current_place_id: "home_mara".parse().unwrap(),
            decision_tick: SimTick::new(1),
            window_id: "partial_food_mara",
            window_end_tick: SimTick::new(8),
            current_place_witness_event_id: None,
            needs_witness_event_id: None,
            frame_event_id: None,
        })
        .build(&agent_state);
    let tomas_surface =
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection: &epistemic_projection,
            agent_state: &agent_state,
            actor_id: tomas,
            current_place_id: "home_mara".parse().unwrap(),
            decision_tick: SimTick::new(1),
            window_id: "partial_food_tomas",
            window_end_tick: SimTick::new(8),
            current_place_witness_event_id: None,
            needs_witness_event_id: None,
            frame_event_id: None,
        })
        .build(&agent_state);
    let mara_known = mara_surface.context();
    let tomas_known = tomas_surface.context();

    assert!(mara_known
        .known_food_sources()
        .contains("food_shared_pantry"));
    assert!(!tomas_known
        .known_food_sources()
        .contains("food_shared_pantry"));

    let mara_plan = plan_local_actions(
        mara_known,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_shared_pantry".to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap();
    assert_eq!(mara_plan.proposals[0].action_id.as_str(), "eat");
    assert_eq!(
        mara_plan.proposals[0].target_ids,
        vec!["food_shared_pantry"]
    );

    let tomas_plan_failure = plan_local_actions(
        tomas_known,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_shared_pantry".to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(tomas_plan_failure.reason, "food source is not actor-known");
    assert!(
        tomas_plan_failure
            .trace
            .hidden_truth_audit_result
            .actor_known_only
    );
}

#[test]
fn no_human_day_fixture_has_roster_activity_and_metrics_envelope() {
    let golden = fixtures::no_human_day_001();
    let canonical_mara_resolution = canonical_mara_recovery_resolution(&golden)
        .expect("no_human_day_001 records canonical_mara_recovery_resolution")
        .to_string();
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
    assert!(has_no_human_event(&log, EventKind::WorkBlockCompleted));
    assert!(has_no_human_event(&log, EventKind::SleepCompleted));
    assert!(has_no_human_event_for_actor(
        &log,
        EventKind::EatFailed,
        "actor_mara"
    ));
    let canonical_mara_errors = mara_recovery_resolution_errors(&canonical_mara_resolution, &log);
    assert!(
        canonical_mara_errors.is_empty(),
        "canonical Mara recovery intent diverged from runner behavior: {canonical_mara_errors:#?}"
    );
    let mut flipped = fixtures::no_human_day_001();
    flipped.contract.expected_events_or_reports = flipped
        .contract
        .expected_events_or_reports
        .iter()
        .map(|entry| {
            if entry.starts_with("canonical_mara_recovery_resolution=") {
                "canonical_mara_recovery_resolution=consume_tomas_food"
            } else {
                *entry
            }
        })
        .collect();
    let flipped_resolution = canonical_mara_recovery_resolution(&flipped)
        .expect("synthetic flip keeps canonical resolution token");
    assert!(
        mara_recovery_resolution_errors(flipped_resolution, &log)
            .iter()
            .any(|error| error.contains("unsupported canonical_mara_recovery_resolution")),
        "synthetic canonical_mara_recovery_resolution flip must fail behavior binding"
    );
    let mut tampered_log = log.clone();
    let mut injected_mara_consumption = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::FoodConsumed)
        .expect("canonical day includes a no-human consumed-food event to clone")
        .clone();
    injected_mara_consumption.event_id =
        EventId::new("event_synthetic_mara_autonomous_food_consumed").unwrap();
    injected_mara_consumption.actor_id = Some(ActorId::new("actor_mara").unwrap());
    injected_mara_consumption.participants = vec!["actor_mara".to_string()];
    injected_mara_consumption.ordering_key = OrderingKey::new(
        injected_mara_consumption.sim_tick,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(ActorId::new("actor_mara").unwrap()),
        ProposalSequence::new(999),
        "eat".parse().unwrap(),
        vec!["food_stew_home_tomas".to_string()],
        "synthetic_mara_autonomous_food_consumed".to_string(),
    );
    tampered_log.append(injected_mara_consumption).unwrap();
    assert!(
        mara_recovery_resolution_errors(&canonical_mara_resolution, &tampered_log)
            .iter()
            .any(|error| error.contains("fail-only Mara recovery consumed food")),
        "synthetic_mara_autonomous_food_consumed_fails_resolution must fail the consumed-food arm"
    );

    // Payload-shape coverage below is intentionally hand-driven; runner-only
    // canonical evidence is asserted above before these manual proposals.
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
    let sleep_started = sleep_events
        .iter()
        .find(|event| event.event_type == EventKind::SleepStarted)
        .or_else(|| {
            log.events().iter().find(|event| {
                event.event_type == EventKind::SleepStarted
                    && event
                        .actor_id
                        .as_ref()
                        .is_some_and(|actor| actor.as_str() == "actor_elena")
            })
        })
        .unwrap()
        .clone();
    let completion_events = build_sleep_completion_events(
        &state,
        &agent_state,
        &log,
        &sleep_started,
        &ordering_key(&sleep_elena, 103),
        &manifest_id,
        SimTick::new(39),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .or_else(|| {
            log.events().iter().find(|event| {
                event.event_type == EventKind::WorkBlockStarted
                    && event
                        .actor_id
                        .as_ref()
                        .is_some_and(|actor| actor.as_str() == "actor_tomas")
            })
        })
        .unwrap()
        .clone();
    let completion_events = build_work_completion_events(
        &state,
        &agent_state,
        &log,
        &work_started,
        &ordering_key(&work_tomas, 107),
        &manifest_id,
        SimTick::new(46),
    )
    .unwrap();
    append_and_apply(&mut state, &mut agent_state, &mut log, completion_events);

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
    assert!(has_event(&log, EventKind::ActionRejected));
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
fn sleep_spanning_window_boundary_charges_each_tick_once() {
    let golden = fixtures::sleep_spanning_window_boundary_charges_each_tick_once_001();
    let actor_id = "actor_elena".parse().unwrap();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let report = run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: default_day_windows(SimTick::ZERO),
        },
    );

    assert!(report.ordinary_pipeline_events > 0);
    assert!(has_event(&log, EventKind::SleepStarted));
    assert!(has_event(&log, EventKind::SleepCompleted));
    assert_no_duplicate_need_regime_charges(&log);
    assert!(!log.events().iter().any(|event| {
        event.event_type == EventKind::NeedDeltaApplied
            && payload(event, "cause_kind") == Some("tick_delta")
            && payload(event, "window_id") == Some("morning")
            && payload(event, "elapsed_ticks") == Some("4")
    }));
}

#[test]
fn no_human_need_ledger_has_no_duplicate_regime_charges() {
    let golden = fixtures::no_human_day_001();
    let actor_ids = [
        "actor_anna".parse().unwrap(),
        "actor_elena".parse().unwrap(),
        "actor_mara".parse().unwrap(),
        "actor_tomas".parse().unwrap(),
    ];
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: actor_ids.to_vec(),
            windows: default_day_windows(SimTick::ZERO),
        },
    );

    assert_no_duplicate_need_regime_charges(&log);
}

#[test]
fn wait_then_window_passive_charges_each_tick_once() {
    let golden = fixtures::wait_then_window_passive_charges_each_tick_once_001();
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);

    run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: vec![
                DayWindow {
                    window_id: "first_idle".to_string(),
                    start_tick: SimTick::ZERO,
                    end_tick: SimTick::new(1),
                },
                DayWindow {
                    window_id: "second_idle".to_string(),
                    start_tick: SimTick::new(4),
                    end_tick: SimTick::new(5),
                },
            ],
        },
    );

    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::ActorWaited && event.sim_tick == SimTick::new(1)
    }));
    let second_window_passive = log
        .events()
        .iter()
        .find(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && payload(event, "window_id") == Some("second_idle")
                && payload(event, "need_kind") == Some("hunger")
        })
        .expect("second window passive hunger delta exists");
    assert_eq!(payload(second_window_passive, "elapsed_ticks"), Some("3"));
    assert_no_duplicate_need_regime_charges(&log);
}

#[test]
fn aged_food_record_surfaces_as_remembered_belief() {
    let golden = fixtures::aged_food_record_surfaces_as_remembered_belief_not_observation_001();
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let current_place_id = PlaceId::new("home_tomas").unwrap();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let mut epistemic_projection = EpistemicProjection::new(manifest_id.clone());

    let perception_events = record_current_place_perception_and_project(
        &mut log,
        &mut state,
        &mut agent_state,
        &mut epistemic_projection,
        &actor_id,
        SimTick::new(4),
        &manifest_id,
    );
    assert!(perception_events.iter().any(|event| {
        event.event_type == EventKind::ObservationRecorded
            && payload(event, "target_id") == Some("food_stew_home_tomas")
    }));

    let surface =
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection: &epistemic_projection,
            agent_state: &agent_state,
            actor_id,
            current_place_id,
            decision_tick: SimTick::new(9),
            window_id: "later_window",
            window_end_tick: SimTick::new(12),
            current_place_witness_event_id: perception_events
                .first()
                .map(|event| event.event_id.clone()),
            needs_witness_event_id: None,
            frame_event_id: None,
        })
        .build(&agent_state);

    assert!(surface
        .context()
        .known_food_sources()
        .contains("food_stew_home_tomas"));
    let food_fact = surface
        .context()
        .actor_known_facts()
        .iter()
        .find(|fact| fact.stable_id() == "actor_knows_food_source")
        .expect("food source fact remains available as memory");
    assert_eq!(food_fact.semantic_kind(), "remembered_belief");
    assert_eq!(food_fact.tick(), Some(SimTick::new(4)));
    assert!(food_fact
        .proof_note()
        .contains("remembered_belief:evented_perception:visible_food_supply"));
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
    assert!(
        rebuild.decision_context_hash_failures.is_empty(),
        "{:?}",
        rebuild.decision_context_hash_failures
    );
    assert!(
        replay.decision_context_hash_failures.is_empty(),
        "{:?}",
        replay.decision_context_hash_failures
    );
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
fn continue_routine_tamper_kind_flip_poisons_replay() {
    let (initial_state, initial_agent_state, state, agent_state, log) =
        possession_continue_routine_replay_fixture();
    let context = checksum_context("possession_does_not_reset_intention_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let tampered = tamper_first_continue_routine_kind(&log, EventKind::ContinueRoutineRejected);
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &tampered,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );

    assert!(!replay.matches_expected);
    assert!(!replay.agent_checksum_matches);
}

#[test]
fn continue_routine_tamper_reason_rewrite_poisons_replay() {
    let (initial_state, initial_agent_state, state, agent_state, log) =
        possession_continue_routine_replay_fixture();
    let context = checksum_context("possession_does_not_reset_intention_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let tampered = tamper_first_continue_routine_reason(&log, "tampered_continue_routine_reason");
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &tampered,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );

    assert!(!replay.matches_expected);
    assert!(!replay.agent_checksum_matches);
}

#[test]
fn episode_tamper_output_tag_poisons_replay() {
    let (initial_state, initial_agent_state, state, agent_state, log) =
        sleep_eat_work_replay_fixture();
    let context = checksum_context("sleep_eat_work_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let tampered = tamper_first_payload_field(
        &log,
        EventKind::WorkBlockStarted,
        "output_tag",
        "tampered_output_tag",
    );
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &tampered,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );

    assert!(!replay.matches_expected);
    assert!(!replay.agent_checksum_matches);
}

#[test]
fn episode_tamper_proration_poisons_replay() {
    let golden = fixtures::sleep_interrupted_by_severe_need_prorates_recovery_001();
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
    run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids: vec![actor_id],
            windows: default_day_windows(SimTick::ZERO),
        },
    );
    assert!(has_event(&log, EventKind::SleepInterrupted));
    let context = checksum_context(
        "sleep_interrupted_by_severe_need_prorates_recovery_001",
        &log,
    );
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let tampered =
        tamper_first_payload_field(&log, EventKind::SleepInterrupted, "fatigue_delta", "999");
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &tampered,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );

    assert!(!replay.matches_expected);
    assert!(!replay.agent_checksum_matches);
}

#[test]
fn no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash() {
    let golden = fixtures::no_human_observation_facts_cite_log_events_001();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
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
    let rebuilt_failures =
        rebuild_decision_context_hashes(&initial_state, &initial_agent_state, &log);
    assert!(rebuilt_failures.is_empty(), "{rebuilt_failures:?}");
    for record in records {
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
fn no_human_decision_context_hash_gate_fails_when_source_evidence_tampered() {
    let golden = fixtures::no_human_day_001();
    let (mut state, mut agent_state, manifest_id, mut log) = load_with_log(golden);
    let initial_state = state.clone();
    let initial_agent_state = agent_state.clone();
    let actor_ids = state.actors().keys().cloned().collect::<Vec<_>>();

    run_no_human_day(
        &mut state,
        &mut agent_state,
        &mut log,
        &registry(),
        manifest_id,
        NoHumanDayConfig {
            actor_ids,
            windows: default_day_windows(SimTick::ZERO),
        },
    );

    let context = checksum_context("no_human_day_001", &log);
    let live_physical_checksum = compute_physical_checksum(&state, &context).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&agent_state, &context).checksum;
    let tampered =
        tamper_first_decision_trace_source_event(&log, "event.tampered.actor_known_source");
    let replay = run_replay(
        &initial_state,
        &initial_agent_state,
        &tampered,
        &context,
        Some(&state),
        Some(live_physical_checksum),
        Some(live_agent_checksum),
    );

    assert!(!replay.matches_expected);
    assert!(!replay.decision_context_hash_failures.is_empty());
    assert!(
        replay
            .decision_context_hash_failures
            .iter()
            .any(|failure| failure.issue.contains("decision_context_hash_mismatch")),
        "{:?}",
        replay.decision_context_hash_failures
    );
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
        "embodied_menu_lags_truth_change_without_perception_001",
        "embodied_workplace_availability_reflects_belief_not_truth_001",
        "sleep_eat_work_001",
        "food_unavailable_replan_001",
        "routine_blocked_diagnostic_001",
        "planner_trace_001",
        "routine_no_teleport_001",
        "severe_safety_with_known_exit_produces_move_001",
        "severe_safety_without_known_exit_waits_with_knowledge_blocker_001",
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
