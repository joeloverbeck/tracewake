use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, Proposal, ProposalOrigin, ReportStatus,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ItemId, PlaceId, ProposalId,
};
use tracewake_core::location::Location;
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
    registry
}

fn actor_id() -> ActorId {
    ActorId::new("actor_tomas").unwrap()
}

fn state(container_open: bool, door_open: bool) -> PhysicalState {
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

    let mut door = DoorState::new("door_shop_back".parse().unwrap(), shop.clone(), back);
    door.is_open = door_open;
    state.doors.insert(door.door_id.clone(), door);

    let mut container =
        ContainerState::fixed_at_place(ContainerId::new("strongbox_tomas").unwrap(), shop.clone());
    container.is_open = container_open;
    container
        .contents
        .insert(ItemId::new("coin_stack_01").unwrap());
    state
        .containers
        .insert(ContainerId::new("strongbox_tomas").unwrap(), container);
    state.items.insert(
        ItemId::new("coin_stack_01").unwrap(),
        ItemState::new(
            ItemId::new("coin_stack_01").unwrap(),
            Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
        ),
    );
    state
}

fn run(
    state: &mut PhysicalState,
    log: &mut EventLog,
    origin: ProposalOrigin,
    action: &str,
    targets: &[&str],
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let mut proposal = Proposal::new(
        ProposalId::new(format!("proposal_{action}_{sequence}")).unwrap(),
        origin,
        Some(actor_id()),
        ActionId::new(action).unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids = targets.iter().map(|target| target.to_string()).collect();
    let key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(sequence),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    run_pipeline(
        &mut PipelineContext {
            registry: &registry(),
            state,
            log,
            controller_bindings: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: key,
        },
        &proposal,
    )
}

#[test]
fn human_and_nonhuman_proposals_share_validation_path() {
    let mut human_state = state(true, true);
    let mut test_state = state(true, true);
    let mut human_log = EventLog::new();
    let mut test_log = EventLog::new();

    let human = run(
        &mut human_state,
        &mut human_log,
        ProposalOrigin::Test,
        "take",
        &["coin_stack_01"],
        0,
    );
    let nonhuman = run(
        &mut test_state,
        &mut test_log,
        ProposalOrigin::Scheduler,
        "take",
        &["coin_stack_01"],
        0,
    );

    assert_eq!(human.report.status, nonhuman.report.status);
    assert_eq!(human.report.action_id, nonhuman.report.action_id);
    assert_eq!(human.report.target_ids, nonhuman.report.target_ids);
    assert_eq!(human.appended_events.len(), nonhuman.appended_events.len());
}

#[test]
fn blocked_movement_returns_structured_rejection() {
    let mut state = state(true, false);
    let mut log = EventLog::new();

    let result = run(
        &mut state,
        &mut log,
        ProposalOrigin::Test,
        "move",
        &["back_room"],
        0,
    );

    assert_eq!(result.report.status, ReportStatus::Rejected);
    assert!(result.report.failed_stage.is_some());
    assert!(!result.report.reason_codes.is_empty());
    assert!(result
        .appended_events
        .iter()
        .all(|event| event.stream != tracewake_core::events::EventStream::World));
}

#[test]
fn closed_container_take_returns_structured_rejection() {
    let mut state = state(false, true);
    let mut log = EventLog::new();

    let result = run(
        &mut state,
        &mut log,
        ProposalOrigin::Test,
        "take",
        &["coin_stack_01"],
        0,
    );

    assert_eq!(result.report.status, ReportStatus::Rejected);
    assert!(result.report.failed_stage.is_some());
    assert!(result
        .appended_events
        .iter()
        .all(|event| event.stream != tracewake_core::events::EventStream::World));
}

#[test]
fn event_append_order_is_deterministic() {
    let mut state = state(true, true);
    let mut log = EventLog::new();
    run(
        &mut state,
        &mut log,
        ProposalOrigin::Test,
        "take",
        &["coin_stack_01"],
        0,
    );
    run(
        &mut state,
        &mut log,
        ProposalOrigin::Test,
        "place",
        &["coin_stack_01"],
        1,
    );

    assert_eq!(log.events()[0].global_order, 0);
    assert_eq!(log.events()[1].global_order, 1);
    assert!(log.events()[0].event_id < log.events()[1].event_id);
}
