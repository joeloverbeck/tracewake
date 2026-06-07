use tracewake_content::fixtures;
use tracewake_content::fixtures::GoldenFixture;
use tracewake_content::load::load_fixture_package;
use tracewake_content::validate::{validate_fixture, validate_fixture_bytes};
use tracewake_core::actions::defs::sleep::build_sleep_completion_events;
use tracewake_core::actions::defs::work::build_work_completion_events;
use tracewake_core::actions::pipeline::{run_pipeline, PipelineContext};
use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::{EpistemicProjection, HolderKind, SourceRef};
use tracewake_core::events::apply::apply_event;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventEnvelope, EventKind};
use tracewake_core::ids::{ContentManifestId, ContentVersion};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::PhysicalState;
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

fn load(golden: GoldenFixture) -> (PhysicalState, ContentManifestId) {
    let manifest_id =
        ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str())).unwrap();
    let loaded = load_fixture_package(
        manifest_id.clone(),
        ContentVersion::new("content_v1").unwrap(),
        vec![golden.source_file()],
    )
    .unwrap();
    (loaded.canonical_world, manifest_id)
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
    log: &mut EventLog,
    manifest_id: &ContentManifestId,
    proposal: &Proposal,
    sequence: u64,
) -> Vec<EventEnvelope> {
    let registry = registry();
    let mut context = PipelineContext {
        registry: &registry,
        state,
        log,
        controller_bindings: None,
        epistemic_projection: None,
        content_manifest_id: manifest_id.clone(),
        ordering_key: ordering_key(proposal, sequence),
    };
    run_pipeline(&mut context, proposal).appended_events
}

fn append_and_apply(state: &mut PhysicalState, log: &mut EventLog, events: Vec<EventEnvelope>) {
    for event in events {
        let appended = log.append(event).unwrap();
        apply_event(state, &appended).unwrap();
    }
}

fn has_event(log: &EventLog, kind: EventKind) -> bool {
    log.events().iter().any(|event| event.event_type == kind)
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
    let (mut state, manifest_id) = load(fixtures::ordinary_workday_001());
    let mut log = EventLog::new();

    let move_to_work = proposal(
        "proposal_workday_move",
        "actor_tomas",
        "move",
        &["workshop_tomas"],
        4,
    );
    run(&mut state, &mut log, &manifest_id, &move_to_work, 0);
    let work = proposal(
        "proposal_workday_work",
        "actor_tomas",
        "work_block",
        &["workplace_tomas"],
        8,
    );
    let work_events = run(&mut state, &mut log, &manifest_id, &work, 1);
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .expect("work starts after movement ancestry")
        .clone();
    append_and_apply(
        &mut state,
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
    let actor_id = "actor_tomas".parse().unwrap();
    assert_eq!(
        state.actors[&actor_id].current_place_id.as_str(),
        "workshop_tomas"
    );
}

#[test]
fn sleep_eat_work_fixture_logs_need_effects_and_replays() {
    let (mut state, manifest_id) = load(fixtures::sleep_eat_work_001());
    let initial_state = state.clone();
    let mut log = EventLog::new();

    let mut sleep = proposal("proposal_sleep", "actor_tomas", "sleep", &["home_tomas"], 0);
    sleep
        .parameters
        .insert("duration_ticks".to_string(), "4".to_string());
    sleep
        .parameters
        .insert("sleep_place_id".to_string(), "home_tomas".to_string());
    let sleep_events = run(&mut state, &mut log, &manifest_id, &sleep, 0);
    let sleep_started = sleep_events
        .iter()
        .find(|event| event.event_type == EventKind::SleepStarted)
        .expect("sleep starts")
        .clone();
    append_and_apply(
        &mut state,
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
    run(&mut state, &mut log, &manifest_id, &eat, 2);
    let move_to_work = proposal(
        "proposal_sleep_eat_work_move",
        "actor_tomas",
        "move",
        &["workshop_tomas"],
        6,
    );
    run(&mut state, &mut log, &manifest_id, &move_to_work, 3);
    let work = proposal(
        "proposal_sleep_eat_work_work",
        "actor_tomas",
        "work_block",
        &["workplace_tomas"],
        8,
    );
    let work_events = run(&mut state, &mut log, &manifest_id, &work, 4);
    let work_started = work_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockStarted)
        .expect("work starts")
        .clone();
    append_and_apply(
        &mut state,
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
fn food_unavailable_fixture_records_typed_failure_without_refill() {
    let (mut state, manifest_id) = load(fixtures::food_unavailable_replan_001());
    let mut log = EventLog::new();

    let eat = proposal(
        "proposal_mara_empty_food",
        "actor_mara",
        "eat",
        &["food_empty_pantry_mara"],
        1,
    );
    run(&mut state, &mut log, &manifest_id, &eat, 0);

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
    let (mut state, manifest_id) = load(fixtures::routine_blocked_diagnostic_001());
    let mut log = EventLog::new();

    let work = proposal(
        "proposal_elena_blocked_work",
        "actor_elena",
        "work_block",
        &["workplace_elena"],
        1,
    );
    run(&mut state, &mut log, &manifest_id, &work, 0);

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
    let mut projection =
        EpistemicProjection::new(ContentManifestId::new("manifest_strongbox_001").unwrap());

    for seed in &golden.fixture.initial_beliefs {
        projection.insert_belief(seed.to_belief());
    }

    assert!(projection.beliefs_by_id.contains_key(
        &"belief_tomas_expects_coin_stack_01_in_strongbox_tomas"
            .parse()
            .unwrap()
    ));
    assert!(projection
        .beliefs_by_holder
        .contains_key(&"actor_tomas".parse().unwrap()));
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
fn phase2a_initial_beliefs_are_holder_and_source_backed() {
    for golden in fixtures::all()
        .into_iter()
        .filter(|golden| !golden.fixture.initial_beliefs.is_empty())
    {
        for seed in &golden.fixture.initial_beliefs {
            let belief = seed.to_belief();
            assert!(matches!(belief.holder, HolderKind::Actor(_)));
            assert!(matches!(
                belief.source,
                SourceRef::Event(_) | SourceRef::Action(_) | SourceRef::Cause(_)
            ));
            assert!(!belief.belief_id.as_str().is_empty());
            assert!(!belief.proposition.render().is_empty());
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
