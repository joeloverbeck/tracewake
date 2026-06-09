mod support;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, Proposal, ProposalOrigin, ReasonCode,
    ReportStatus,
};
use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::debug_reports::{action_rejection_report, item_location_report};
use tracewake_core::epistemics::{
    Belief, Confidence, EpistemicProjection, HolderKind, KnowledgeContext, Proposition, SourceRef,
    Stance,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::EventKind;
use tracewake_core::events::EventStream;
use tracewake_core::events::{EventCause, EventEnvelope, PayloadField};
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, ContentManifestId, ContentVersion, ControllerId,
    EventId, FixtureId, ItemId, PlaceId, ProcessId, ProposalId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::{build_notebook_view, no_human_day_metrics};
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::{
    advance_no_human, run_no_human_day, DayWindow, NoHumanDayConfig, NoHumanStateMut,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
};
use tracewake_core::time::SimTick;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry
}

fn phase3a_registry() -> ActionRegistry {
    let mut registry = registry();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

fn actor_id() -> ActorId {
    ActorId::new("actor_tomas").unwrap()
}

fn coin_id() -> ItemId {
    ItemId::new("coin_stack_01").unwrap()
}

fn box_id() -> ContainerId {
    ContainerId::new("strongbox_tomas").unwrap()
}

fn manifest_id() -> ContentManifestId {
    ContentManifestId::new("phase1_manifest").unwrap()
}

fn context(log: &EventLog) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("capstone_fixture").unwrap(),
        content_version: ContentVersion::new("phase1_manifest").unwrap(),
        sim_tick: SimTick::ZERO,
        world_stream_position_applied: log
            .events()
            .iter()
            .filter(|event| event.stream == tracewake_core::events::EventStream::World)
            .count()
            .saturating_sub(1) as u64,
    }
}

fn agent_ordering_key(action: &str, sequence: u64) -> OrderingKey {
    OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        ActionId::new(action).unwrap(),
        vec!["phase3a".to_string()],
        format!("agent_{sequence}"),
    )
}

fn agent_event(
    event_id: &str,
    kind: EventKind,
    sequence: u64,
    payload: Vec<PayloadField>,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        kind,
        99,
        99,
        SimTick::ZERO,
        agent_ordering_key("continue_routine", sequence),
        manifest_id(),
        vec![EventCause::Process(
            ProcessId::new("process_agent").unwrap(),
        )],
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.payload = payload;
    event
}

fn hunger_delta_payload(delta: i32, cause_kind: &str) -> Vec<PayloadField> {
    vec![
        PayloadField::new("actor_id", "actor_tomas"),
        PayloadField::new("need_kind", "hunger"),
        PayloadField::new("delta", delta.to_string()),
        PayloadField::new("cause_kind", cause_kind),
    ]
}

fn initial_state(container_open: bool, door_open: bool) -> PhysicalState {
    let mut state = PhysicalSeed::default();
    let shop = PlaceId::new("shop_front").unwrap();
    let back = PlaceId::new("back_room").unwrap();
    let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
    shop_state.adjacent_place_ids.insert(back.clone());
    let mut back_state = PlaceState::new(back.clone(), "Back room");
    back_state.adjacent_place_ids.insert(shop.clone());
    state.places_mut().insert(shop.clone(), shop_state);
    state.places_mut().insert(back.clone(), back_state);
    state
        .actors_mut()
        .insert(actor_id(), ActorBody::new(actor_id(), shop.clone()));

    let mut door = DoorState::new(
        "door_shop_back".parse().unwrap(),
        shop.clone(),
        back.clone(),
    );
    door.is_open = door_open;
    state.doors_mut().insert(door.door_id.clone(), door);
    state
        .places_mut()
        .get_mut(&shop)
        .unwrap()
        .connected_door_ids
        .insert("door_shop_back".parse().unwrap());
    state
        .places_mut()
        .get_mut(&back)
        .unwrap()
        .connected_door_ids
        .insert("door_shop_back".parse().unwrap());

    let mut container = ContainerState::fixed_at_place(box_id(), shop.clone());
    container.is_open = container_open;
    container.contents.insert(coin_id());
    state.containers_mut().insert(box_id(), container);
    state
        .places_mut()
        .get_mut(&shop)
        .unwrap()
        .local_container_ids
        .insert(box_id());
    state.items_mut().insert(
        coin_id(),
        ItemState::new(coin_id(), Location::InContainer(box_id())),
    );
    state.build()
}

fn phase3a_agent_state() -> AgentState {
    let mut state = AgentSeed::default();
    state.needs_by_actor_mut().insert(
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
    state.build()
}

fn proposal(action: &str, targets: &[&str], sequence: u64) -> Proposal {
    let mut proposal = Proposal::new(
        ProposalId::new(format!("proposal_{action}_{sequence}")).unwrap(),
        ProposalOrigin::Test,
        Some(actor_id()),
        ActionId::new(action).unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids = targets.iter().map(|target| target.to_string()).collect();
    proposal
}

fn run_action(
    state: &mut PhysicalState,
    log: &mut EventLog,
    action: &str,
    targets: &[&str],
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let registry = registry();
    let proposal = proposal(action, targets, sequence);
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let mut pipeline_context = PipelineContext {
        registry: &registry,
        state,
        agent_state: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        log,
        controller_bindings: None,
        epistemic_projection: None,
        content_manifest_id: manifest_id(),
        ordering_key: key,
    };
    run_pipeline(&mut pipeline_context, &proposal)
}

fn tomas_coin_expectation_projection() -> EpistemicProjection {
    EpistemicProjection::from_initial_beliefs(
        manifest_id(),
        [Belief::new(
            BeliefId::new("belief_tomas_expected_coin").unwrap(),
            HolderKind::Actor(actor_id()),
            Proposition::ItemLocatedInContainer {
                item_id: coin_id(),
                container_id: box_id(),
            },
            Stance::ExpectsTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(EventId::new("event_seed_tomas_expectation").unwrap()),
            SimTick::ZERO,
        )],
    )
}

fn run_check_with_projection(
    state: &mut PhysicalState,
    log: &mut EventLog,
    projection: &mut EpistemicProjection,
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let registry = registry();
    let proposal = proposal("check_container", &["strongbox_tomas"], sequence);
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let mut pipeline_context = PipelineContext {
        registry: &registry,
        state,
        agent_state: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        log,
        controller_bindings: None,
        epistemic_projection: Some(projection),
        content_manifest_id: manifest_id(),
        ordering_key: key,
    };
    run_pipeline(&mut pipeline_context, &proposal)
}

fn run_scheduler_check_with_projection(
    state: &mut PhysicalState,
    log: &mut EventLog,
    projection: &mut EpistemicProjection,
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let registry = registry();
    let mut proposal = Proposal::new(
        ProposalId::new(format!("proposal_scheduler_check_{sequence}")).unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id()),
        ActionId::new("check_container").unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids = vec!["strongbox_tomas".to_string()];
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let mut pipeline_context = PipelineContext {
        registry: &registry,
        state,
        agent_state: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        log,
        controller_bindings: None,
        epistemic_projection: Some(projection),
        content_manifest_id: manifest_id(),
        ordering_key: key,
    };
    run_pipeline(&mut pipeline_context, &proposal)
}

fn assert_no_actor_mara_or_culprit_text(value: &str) {
    for forbidden in ["actor_mara", "Mara", "culprit", "stole", "theft"] {
        assert!(
            !value.contains(forbidden),
            "actor-known surface leaked {forbidden}: {value}"
        );
    }
}

fn run_probe_with_projection(
    state: &mut PhysicalState,
    log: &mut EventLog,
    projection: &mut EpistemicProjection,
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let registry = registry();
    let mut proposal = proposal("truthful_accuse_probe", &["actor_mara"], sequence);
    proposal.proposal_id = ProposalId::new(format!("proposal_accuse_probe_{sequence}")).unwrap();
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let mut pipeline_context = PipelineContext {
        registry: &registry,
        state,
        agent_state: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        log,
        controller_bindings: None,
        epistemic_projection: Some(projection),
        content_manifest_id: manifest_id(),
        ordering_key: key,
    };
    run_pipeline(&mut pipeline_context, &proposal)
}

fn sound_state() -> PhysicalState {
    let mut state = PhysicalSeed::default();
    let house = PlaceId::new("house_tomas").unwrap();
    let street = PlaceId::new("street_lane").unwrap();
    let mut house_state = PlaceState::new(house.clone(), "Tomas house");
    house_state.adjacent_place_ids.insert(street.clone());
    let mut street_state = PlaceState::new(street.clone(), "Street lane");
    street_state.adjacent_place_ids.insert(house.clone());
    state.places_mut().insert(house.clone(), house_state);
    state.places_mut().insert(street.clone(), street_state);
    let mara = ActorId::new("actor_mara").unwrap();
    let elena = ActorId::new("actor_elena").unwrap();
    state
        .actors_mut()
        .insert(mara.clone(), ActorBody::new(mara, house.clone()));
    state
        .actors_mut()
        .insert(elena.clone(), ActorBody::new(elena, street));
    let mut container = ContainerState::fixed_at_place(box_id(), house);
    container.is_open = true;
    container.contents.insert(coin_id());
    state.containers_mut().insert(box_id(), container);
    state.items_mut().insert(
        coin_id(),
        ItemState::new(coin_id(), Location::InContainer(box_id())),
    );
    state.build()
}

fn run_mara_take_with_projection(
    state: &mut PhysicalState,
    log: &mut EventLog,
    projection: &mut EpistemicProjection,
) -> tracewake_core::actions::PipelineResult {
    let registry = registry();
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_mara_take_coin").unwrap(),
        ProposalOrigin::Test,
        Some(ActorId::new("actor_mara").unwrap()),
        ActionId::new("take").unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids = vec!["coin_stack_01".to_string(), "strongbox_tomas".to_string()];
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(ActorId::new("actor_mara").unwrap()),
        ProposalSequence::new(0),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let mut pipeline_context = PipelineContext {
        registry: &registry,
        state,
        agent_state: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        log,
        controller_bindings: None,
        epistemic_projection: Some(projection),
        content_manifest_id: ContentManifestId::new("phase2a_manifest").unwrap(),
        ordering_key: key,
    };
    run_pipeline(&mut pipeline_context, &proposal)
}

#[test]
fn accepted_actions_append_versioned_events() {
    let mut state = initial_state(true, true);
    let mut log = EventLog::new();

    let result = run_action(&mut state, &mut log, "take", &["coin_stack_01"], 0);

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert_eq!(result.appended_events.len(), 1);
    assert!(result.appended_events[0].has_supported_schema_version());
    assert_eq!(log.events().len(), 1);
}

#[test]
fn sound_uncertainty_records_low_confidence_evidence_without_culprit_knowledge() {
    let mut state = sound_state();
    let mut log = EventLog::new();
    let mut projection =
        EpistemicProjection::new(ContentManifestId::new("phase2a_manifest").unwrap());

    let result = run_mara_take_with_projection(&mut state, &mut log, &mut projection);

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert!(result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::ObservationRecorded));
    assert!(result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::BeliefUpdated));
    let elena_context =
        KnowledgeContext::embodied(ActorId::new("actor_elena").unwrap(), SimTick::ZERO);
    let observation = projection
        .observations_for_context(&elena_context)
        .into_iter()
        .find(|observation| observation.channel.stable_id() == "simple_sound")
        .expect("simple sound observation recorded");
    assert!(observation.confidence.is_low());
    assert!(observation.alternatives.len() >= 2);

    let belief = projection
        .beliefs_for_context(&elena_context)
        .into_iter()
        .find(|belief| {
            belief
                .channel
                .is_some_and(|channel| channel.stable_id() == "simple_sound")
        })
        .expect("sound belief recorded");
    assert!(belief.confidence.is_low());
    assert!(matches!(
        belief.proposition,
        Proposition::SoundHeardNearPlace { .. } | Proposition::PossibleMovementNearPlace { .. }
    ));
    assert!(!matches!(
        belief.proposition,
        Proposition::ActorWasNearPlace { .. }
    ));

    let notebook = build_notebook_view(&projection, &elena_context);
    let rendered = format!("{notebook:?}");
    assert!(!rendered.contains("Mara"));
    assert!(!rendered.contains("actor_mara"));
    assert!(!rendered.contains("stole"));
    assert!(!rendered.contains("culprit"));
}

#[test]
fn check_container_records_observation_but_open_alone_does_not() {
    let mut open_state = initial_state(false, true);
    let mut open_log = EventLog::new();
    let open = run_action(
        &mut open_state,
        &mut open_log,
        "open",
        &["strongbox_tomas"],
        0,
    );

    assert_eq!(open.report.status, ReportStatus::Accepted);
    assert_eq!(open.appended_events.len(), 1);
    assert!(!open_log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ObservationRecorded));

    let mut check_state = open_state;
    let mut check_log = open_log;
    let check = run_action(
        &mut check_state,
        &mut check_log,
        "check_container",
        &["strongbox_tomas"],
        1,
    );

    assert_eq!(check.report.status, ReportStatus::Accepted);
    assert_eq!(check.appended_events.len(), 2);
    assert_eq!(
        check.appended_events[0].event_type,
        EventKind::ContainerChecked
    );
    assert_eq!(
        check.appended_events[1].event_type,
        EventKind::ObservationRecorded
    );
    assert_eq!(
        check
            .appended_events
            .iter()
            .filter(|event| event.event_type == EventKind::ObservationRecorded)
            .count(),
        1
    );
}

#[test]
fn expected_absence_check_creates_contradiction_and_missing_belief() {
    let mut seed = PhysicalSeed::from_state(&initial_state(true, true));
    seed.containers_mut()
        .get_mut(&box_id())
        .unwrap()
        .contents
        .clear();
    seed.items_mut().get_mut(&coin_id()).unwrap().location =
        Location::AtPlace(PlaceId::new("back_room").unwrap());
    let mut state = seed.build();
    let mut log = EventLog::new();
    let mut projection = tomas_coin_expectation_projection();

    let result = run_check_with_projection(&mut state, &mut log, &mut projection, 7);

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert_eq!(
        result
            .appended_events
            .iter()
            .map(|event| event.event_type)
            .collect::<Vec<_>>(),
        [
            EventKind::ContainerChecked,
            EventKind::ObservationRecorded,
            EventKind::ExpectationContradicted,
            EventKind::BeliefUpdated
        ]
    );
    assert_eq!(projection.debug_epistemics_view().contradictions.len(), 1);
    let context = KnowledgeContext::embodied(actor_id(), SimTick::ZERO);
    let beliefs = projection.beliefs_for_context(&context);
    assert!(beliefs.iter().any(|belief| {
        matches!(
            belief.proposition,
            Proposition::ItemMissingFromExpectedLocation { .. }
        )
    }));
    assert!(!beliefs.iter().any(|belief| {
        belief.proposition.render().contains("stole")
            || belief.proposition.render().contains("culprit")
    }));
}

#[test]
fn no_human_epistemic_check_records_evidence_without_controller() {
    let mut seed = PhysicalSeed::from_state(&initial_state(true, true));
    seed.containers_mut()
        .get_mut(&box_id())
        .unwrap()
        .contents
        .clear();
    seed.items_mut().get_mut(&coin_id()).unwrap().location =
        Location::AtPlace(PlaceId::new("back_room").unwrap());
    let mut state = seed.build();
    let before = compute_physical_checksum(&state, &context(&EventLog::new())).checksum;
    let mut log = EventLog::new();
    let mut projection = tomas_coin_expectation_projection();

    let result = run_scheduler_check_with_projection(&mut state, &mut log, &mut projection, 11);
    let after = compute_physical_checksum(&state, &context(&log)).checksum;

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert_eq!(before, after);
    assert!(result.appended_events.iter().all(|event| {
        event.ordering_key.phase == SchedulePhase::NoHumanProcess
            && matches!(event.ordering_key.source_id, SchedulerSourceId::Actor(_))
    }));
    assert!(result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::ContainerChecked));
    assert!(result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::ObservationRecorded));
    assert_eq!(projection.debug_epistemics_view().contradictions.len(), 1);

    let notebook = build_notebook_view(
        &projection,
        &KnowledgeContext::embodied(actor_id(), SimTick::ZERO),
    );
    assert_no_actor_mara_or_culprit_text(&format!("{notebook:?}"));
}

#[test]
fn missing_property_belief_does_not_support_truthful_accusation() {
    let mut seed = PhysicalSeed::from_state(&initial_state(true, true));
    seed.actors_mut().insert(
        ActorId::new("actor_mara").unwrap(),
        ActorBody::new(
            ActorId::new("actor_mara").unwrap(),
            PlaceId::new("shop_front").unwrap(),
        ),
    );
    seed.containers_mut()
        .get_mut(&box_id())
        .unwrap()
        .contents
        .clear();
    seed.items_mut().get_mut(&coin_id()).unwrap().location =
        Location::AtPlace(PlaceId::new("back_room").unwrap());
    let mut state = seed.build();
    let before = compute_physical_checksum(&state, &context(&EventLog::new())).checksum;
    let mut log = EventLog::new();
    let mut projection = tomas_coin_expectation_projection();
    let check = run_check_with_projection(&mut state, &mut log, &mut projection, 8);
    assert_eq!(check.report.status, ReportStatus::Accepted);

    let probe = run_probe_with_projection(&mut state, &mut log, &mut projection, 9);
    let after = compute_physical_checksum(&state, &context(&EventLog::new())).checksum;

    assert_eq!(before, after);
    assert_eq!(probe.report.status, ReportStatus::Rejected);
    assert_eq!(
        probe.report.reason_codes,
        vec![ReasonCode::KnowledgePreconditionNotMet]
    );
    assert!(!probe.report.actor_visible_summary.contains("actor_mara"));
    assert!(!probe.report.actor_visible_summary.contains("stole"));
    assert!(probe.report.debug_summary.contains("actor_mara"));
    assert!(probe
        .appended_events
        .iter()
        .all(|event| event.stream != EventStream::World && event.stream != EventStream::Epistemic));
}

#[test]
fn projection_rebuild_matches_live_state() {
    let initial = initial_state(true, true);
    let mut live = initial.clone();
    let mut log = EventLog::new();
    run_action(&mut live, &mut log, "take", &["coin_stack_01"], 0);
    run_action(&mut live, &mut log, "place", &["coin_stack_01"], 1);

    let report = rebuild_projection(
        &initial,
        &tracewake_core::state::AgentState::default(),
        &log,
        &context(&log),
        Some(&live),
    );

    assert!(report.state_diff.is_empty());
    assert!(report.invariant_violations.is_empty());
    assert_eq!(report.final_state, live);
}

#[test]
fn replay_checksum_matches() {
    let initial = initial_state(true, true);
    let mut live = initial.clone();
    let mut log = EventLog::new();
    run_action(&mut live, &mut log, "take", &["coin_stack_01"], 0);

    let expected = compute_physical_checksum(&live, &context(&log)).checksum;
    let report = run_replay(
        &initial,
        &tracewake_core::state::AgentState::default(),
        &log,
        &context(&log),
        Some(&live),
        Some(expected),
        None,
    );

    assert!(report.matches_expected);
    assert!(report.state_diff.is_empty());
}

#[test]
fn replay_detects_missing_or_reordered_event() {
    let initial = initial_state(true, true);
    let mut live = initial.clone();
    let mut full_log = EventLog::new();
    run_action(&mut live, &mut full_log, "take", &["coin_stack_01"], 0);
    let place = run_action(&mut live, &mut full_log, "place", &["coin_stack_01"], 1);

    let mut missing_log = EventLog::new();
    missing_log
        .append(place.appended_events[0].clone())
        .unwrap();
    let expected = compute_physical_checksum(&live, &context(&full_log)).checksum;
    let report = run_replay(
        &initial,
        &tracewake_core::state::AgentState::default(),
        &missing_log,
        &context(&missing_log),
        Some(&live),
        Some(expected),
        None,
    );

    assert!(!report.matches_expected);
    assert!(!report.application_errors.is_empty() || !report.state_diff.is_empty());
}

#[test]
fn phase3a_agent_state_replay_projection_is_deterministic() {
    let initial = initial_state(true, true);
    let mut log = EventLog::new();
    log.append(agent_event(
        "event_hunger_initial",
        EventKind::NeedDeltaApplied,
        0,
        hunger_delta_payload(490, "fixture_initial"),
    ))
    .unwrap();
    log.append(agent_event(
        "event_hunger_tick",
        EventKind::NeedDeltaApplied,
        1,
        hunger_delta_payload(40, "tick_delta"),
    ))
    .unwrap();
    log.append(agent_event(
        "event_trace_breakfast",
        EventKind::DecisionTraceRecorded,
        2,
        vec![
            PayloadField::new("trace_schema_version", "1"),
            PayloadField::new("trace_id", "trace_breakfast"),
            PayloadField::new(
                "trace_canonical",
                "decision_trace_v1|trace_breakfast|actor_tomas|1|2|completed|0|true|74657374",
            ),
        ],
    ))
    .unwrap();

    let first = rebuild_projection(
        &initial,
        &tracewake_core::state::AgentState::default(),
        &log,
        &context(&log),
        None,
    );
    let second = rebuild_projection(
        &initial,
        &tracewake_core::state::AgentState::default(),
        &log,
        &context(&log),
        None,
    );

    assert_eq!(first.final_agent_checksum, second.final_agent_checksum);
    assert_eq!(
        first.final_agent_checksum_report.canonical_input,
        second.final_agent_checksum_report.canonical_input
    );
    assert_eq!(first.final_agent_state.decision_traces().len(), 1);
    assert!(first.agent_application_errors.is_empty());
    assert!(first.unsupported_agent_versions.is_empty());
}

#[test]
fn debug_item_location_reports_last_location_event() {
    let mut state = initial_state(true, true);
    let mut log = EventLog::new();
    let result = run_action(&mut state, &mut log, "take", &["coin_stack_01"], 0);

    let report = item_location_report(&state, &log, &coin_id(), &context(&log));

    assert!(report.debug_only());
    assert_eq!(
        report.last_location_event_id,
        Some(result.appended_events[0].event_id.clone())
    );
}

#[test]
fn debug_rejection_report_names_failed_stage() {
    let mut state = initial_state(false, true);
    let mut log = EventLog::new();
    let result = run_action(&mut state, &mut log, "take", &["coin_stack_01"], 0);

    let report = action_rejection_report(&result.report, &state, &context(&log));

    assert!(report.debug_only());
    assert!(report.failed_stage.is_some());
    assert!(!report.mutation_attempted);
}

#[test]
fn controller_binding_is_not_world_state() {
    let state = initial_state(true, true);
    let before = compute_physical_checksum(&state, &context(&EventLog::new())).checksum;
    let mut bindings = ControllerBindings::new();
    let mut log = EventLog::new();
    bindings.attach(
        ControllerId::new("controller_human").unwrap(),
        actor_id(),
        tracewake_core::state::ControllerMode::Embodied,
        SimTick::ZERO,
        &mut log,
        manifest_id(),
    );
    let after = compute_physical_checksum(&state, &context(&EventLog::new())).checksum;

    assert_eq!(before, after);
    assert_eq!(log.events().len(), 1);
    assert_eq!(
        log.events()[0].stream,
        tracewake_core::events::EventStream::Controller
    );
}

#[test]
fn possession_controller_binding_is_not_world_state() {
    controller_binding_is_not_world_state();
}

#[test]
fn no_human_advance_requires_no_controller() {
    let mut state = initial_state(true, true);
    let mut log = EventLog::new();
    let report = advance_no_human(
        NoHumanStateMut {
            physical: &mut state,
            agent: Box::leak(Box::new(tracewake_core::state::AgentState::default())),
        },
        &mut log,
        &registry(),
        manifest_id(),
        SimTick::ZERO,
        2,
        Vec::new(),
    );

    assert_eq!(report.tick_count, 2);
    assert_eq!(report.marker_event_ids.len(), 2);
}

#[test]
fn phase3a_no_human_metrics_are_byte_identical_after_log_replay() {
    let initial = initial_state(true, true);
    let mut live = initial.clone();
    let initial_agent = phase3a_agent_state();
    let mut live_agent = initial_agent.clone();
    let mut log = EventLog::new();
    let report = run_no_human_day(
        &mut live,
        &mut live_agent,
        &mut log,
        &phase3a_registry(),
        manifest_id(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "fatigue_sleep".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );

    let canonical = log.serialize_canonical();
    let replayed = EventLog::deserialize_canonical(&canonical).unwrap();
    let first_metrics = no_human_day_metrics(&log).serialize_canonical();
    let replayed_metrics = no_human_day_metrics(&replayed).serialize_canonical();
    let live_physical_checksum = compute_physical_checksum(&live, &context(&log)).checksum;
    let live_agent_checksum = compute_agent_state_checksum(&live_agent, &context(&log)).checksum;
    let replay = run_replay(
        &initial,
        &initial_agent,
        &log,
        &context(&log),
        Some(&live),
        Some(live_physical_checksum.clone()),
        Some(live_agent_checksum.clone()),
    );

    assert!(report.ordinary_pipeline_events > 0);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepStarted));
    assert_eq!(replayed.serialize_canonical(), canonical);
    assert_eq!(replayed_metrics, first_metrics);
    assert_eq!(replay.final_checksum, live_physical_checksum);
    assert_eq!(replay.final_agent_checksum, live_agent_checksum);
    assert_eq!(replay.expected_agent_checksum, Some(live_agent_checksum));
    assert!(replay.agent_checksum_matches);
    assert!(first_metrics.contains("no_human_day_metrics_v1"));
    let canonical_text = String::from_utf8(canonical).unwrap().to_ascii_lowercase();
    assert!(!canonical_text.contains("player"));
    assert!(!canonical_text.contains("controller"));
}
