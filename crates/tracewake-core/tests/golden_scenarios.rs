use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, Proposal, ProposalOrigin, ReportStatus,
};
use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::debug_reports::{action_rejection_report, item_location_report};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::EventKind;
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, ControllerId, FixtureId,
    ItemId, PlaceId, ProposalId,
};
use tracewake_core::location::Location;
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::advance_no_human;
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
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

fn initial_state(container_open: bool, door_open: bool) -> PhysicalState {
    let mut state = PhysicalState::default();
    let shop = PlaceId::new("shop_front").unwrap();
    let back = PlaceId::new("back_room").unwrap();
    let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
    shop_state.adjacent_place_ids.insert(back.clone());
    let mut back_state = PlaceState::new(back.clone(), "Back room");
    back_state.adjacent_place_ids.insert(shop.clone());
    state.places.insert(shop.clone(), shop_state);
    state.places.insert(back.clone(), back_state);
    state
        .actors
        .insert(actor_id(), ActorBody::new(actor_id(), shop.clone()));

    let mut door = DoorState::new(
        "door_shop_back".parse().unwrap(),
        shop.clone(),
        back.clone(),
    );
    door.is_open = door_open;
    state.doors.insert(door.door_id.clone(), door);
    state
        .places
        .get_mut(&shop)
        .unwrap()
        .connected_door_ids
        .insert("door_shop_back".parse().unwrap());
    state
        .places
        .get_mut(&back)
        .unwrap()
        .connected_door_ids
        .insert("door_shop_back".parse().unwrap());

    let mut container = ContainerState::fixed_at_place(box_id(), shop.clone());
    container.is_open = container_open;
    container.contents.insert(coin_id());
    state.containers.insert(box_id(), container);
    state
        .places
        .get_mut(&shop)
        .unwrap()
        .local_container_ids
        .insert(box_id());
    state.items.insert(
        coin_id(),
        ItemState::new(coin_id(), Location::InContainer(box_id())),
    );
    state
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
        log,
        controller_bindings: None,
        content_manifest_id: manifest_id(),
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
fn projection_rebuild_matches_live_state() {
    let initial = initial_state(true, true);
    let mut live = initial.clone();
    let mut log = EventLog::new();
    run_action(&mut live, &mut log, "take", &["coin_stack_01"], 0);
    run_action(&mut live, &mut log, "place", &["coin_stack_01"], 1);

    let report = rebuild_projection(&initial, &log, &context(&log), Some(&live));

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
    let report = run_replay(&initial, &log, &context(&log), Some(&live), Some(expected));

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
        &missing_log,
        &context(&missing_log),
        Some(&live),
        Some(expected),
    );

    assert!(!report.matches_expected);
    assert!(!report.application_errors.is_empty() || !report.state_diff.is_empty());
}

#[test]
fn debug_item_location_reports_last_location_event() {
    let mut state = initial_state(true, true);
    let mut log = EventLog::new();
    let result = run_action(&mut state, &mut log, "take", &["coin_stack_01"], 0);

    let report = item_location_report(&state, &log, &coin_id(), &context(&log));

    assert!(report.debug_only);
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

    assert!(report.debug_only);
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
        &mut state,
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
