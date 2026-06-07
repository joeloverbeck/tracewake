use tracewake_core::actions::{
    run_pipeline, ActionRegistry, PipelineContext, Proposal, ProposalOrigin, ReportStatus,
};
use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
use tracewake_core::checksum::{compute_agent_state_checksum, ChecksumContext};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventKind, EventStream};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, ControllerId, FixtureId,
    FoodSupplyId, ItemId, PlaceId, ProposalId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, ControllerMode, DoorState, FoodSupplyState, ItemState,
    PhysicalState, PlaceState,
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

fn agent_state() -> AgentState {
    let mut state = AgentState::default();
    state.needs_by_actor.insert(
        actor_id(),
        [
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
            ),
        ]
        .into_iter()
        .collect(),
    );
    state
}

fn phase3a_registry() -> ActionRegistry {
    let mut registry = registry();
    registry.register_phase3a_eat();
    registry
}

fn agent_need(agent_state: &AgentState, need: NeedKind) -> u16 {
    agent_state.needs_by_actor[&actor_id()][&need].value()
}

fn agent_checksum_context(log: &EventLog) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("ticket_0006_agent_live_pipeline").unwrap(),
        content_version: ContentVersion::new("phase3a_manifest").unwrap(),
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

fn run(
    state: &mut PhysicalState,
    log: &mut EventLog,
    origin: ProposalOrigin,
    action: &str,
    targets: &[&str],
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let mut agent_state = agent_state();
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
            agent_state: &mut agent_state,
            log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: key,
        },
        &proposal,
    )
}

fn run_sleep(
    state: &mut PhysicalState,
    log: &mut EventLog,
    origin: ProposalOrigin,
    sequence: u64,
) -> tracewake_core::actions::PipelineResult {
    let mut agent_state = agent_state();
    let mut registry = registry();
    registry.register_phase3a_sleep();
    let is_human_origin = origin == ProposalOrigin::Human;
    let mut proposal = Proposal::new(
        ProposalId::new(format!("proposal_sleep_{sequence}")).unwrap(),
        origin,
        Some(actor_id()),
        ActionId::new("sleep").unwrap(),
        SimTick::ZERO,
    );
    proposal
        .parameters
        .insert("duration_ticks".to_string(), "3".to_string());
    let mut bindings = None;
    if is_human_origin {
        let controller_id = ControllerId::new("controller_human").unwrap();
        proposal
            .parameters
            .insert("controller_id".to_string(), controller_id.to_string());
        let mut human_bindings = ControllerBindings::new();
        let mut binding_log = EventLog::new();
        human_bindings.attach(
            controller_id,
            actor_id(),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut binding_log,
            ContentManifestId::new("phase3a_manifest").unwrap(),
        );
        bindings = Some(human_bindings);
    }
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
            registry: &registry,
            state,
            agent_state: &mut agent_state,
            log,
            controller_bindings: bindings.as_ref(),
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
            ordering_key: key,
        },
        &proposal,
    )
}

#[test]
fn phase3a_agent_events_apply_live_and_replay_to_same_agent_checksum() {
    let mut world = state(true, true);
    world.food_supplies.insert(
        FoodSupplyId::new("food_supply_home").unwrap(),
        FoodSupplyState::new(
            FoodSupplyId::new("food_supply_home").unwrap(),
            Location::AtPlace(PlaceId::new("shop_front").unwrap()),
            2,
            40,
        ),
    );
    let initial_agent_state = agent_state();
    let initial_world = world.clone();
    let mut live_agent_state = initial_agent_state.clone();
    let mut log = EventLog::new();
    let registry = phase3a_registry();

    let mut eat = Proposal::new(
        ProposalId::new("proposal_eat_live_agent").unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id()),
        ActionId::new("eat").unwrap(),
        SimTick::ZERO,
    );
    eat.target_ids.push("food_supply_home".to_string());
    let eat_key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(0),
        eat.action_id.clone(),
        eat.target_ids.clone(),
        eat.proposal_id.as_str().to_string(),
    );
    let hunger_before_eat = agent_need(&live_agent_state, NeedKind::Hunger);
    let eat_result = run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut world,
            agent_state: &mut live_agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
            ordering_key: eat_key,
        },
        &eat,
    );
    assert_eq!(eat_result.report.status, ReportStatus::Accepted);
    assert!(eat_result
        .appended_events
        .iter()
        .any(|event| event.event_type == EventKind::NeedDeltaApplied));
    assert!(agent_need(&live_agent_state, NeedKind::Hunger) < hunger_before_eat);

    let mut wait = Proposal::new(
        ProposalId::new("proposal_wait_live_agent").unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id()),
        ActionId::new("wait").unwrap(),
        SimTick::new(1),
    );
    wait.parameters.insert("ticks".to_string(), "3".to_string());
    wait.parameters.insert(
        "current_hunger".to_string(),
        agent_need(&live_agent_state, NeedKind::Hunger).to_string(),
    );
    wait.parameters.insert(
        "current_fatigue".to_string(),
        agent_need(&live_agent_state, NeedKind::Fatigue).to_string(),
    );
    let wait_key = OrderingKey::new(
        SimTick::new(1),
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(1),
        wait.action_id.clone(),
        wait.target_ids.clone(),
        wait.proposal_id.as_str().to_string(),
    );
    let fatigue_before_wait = agent_need(&live_agent_state, NeedKind::Fatigue);
    let wait_result = run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut world,
            agent_state: &mut live_agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
            ordering_key: wait_key,
        },
        &wait,
    );
    assert_eq!(wait_result.report.status, ReportStatus::Accepted);
    assert!(agent_need(&live_agent_state, NeedKind::Fatigue) > fatigue_before_wait);

    let context = agent_checksum_context(&log);
    let live_checksum = compute_agent_state_checksum(&live_agent_state, &context).checksum;
    let replay = rebuild_projection(&initial_world, &initial_agent_state, &log, &context, None);
    assert!(replay.agent_application_errors.is_empty());
    assert_eq!(replay.final_agent_checksum, live_checksum);
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
fn sleep_proposals_share_pipeline_across_human_and_nonhuman_origins() {
    let mut human_state = state(true, true);
    let mut scheduler_state = state(true, true);
    let mut human_log = EventLog::new();
    let mut scheduler_log = EventLog::new();

    let human = run_sleep(&mut human_state, &mut human_log, ProposalOrigin::Human, 0);
    let scheduler = run_sleep(
        &mut scheduler_state,
        &mut scheduler_log,
        ProposalOrigin::Scheduler,
        0,
    );

    assert_eq!(human.report.status, scheduler.report.status);
    assert_eq!(human.report.action_id, scheduler.report.action_id);
    assert_eq!(human.appended_events.len(), scheduler.appended_events.len());
    assert_eq!(human.appended_events[0].event_type, EventKind::SleepStarted);
    assert_eq!(
        scheduler.appended_events[0].event_type,
        EventKind::SleepStarted
    );
}

#[test]
fn no_human_day_runner_smoke_uses_no_controller_and_pipeline_events() {
    let mut world = state(true, true);
    let mut agent_state = agent_state();
    let mut log = EventLog::new();
    let report = run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry(),
        ContentManifestId::new("phase3a_manifest").unwrap(),
        NoHumanDayConfig {
            actor_ids: vec![actor_id()],
            windows: vec![DayWindow {
                window_id: "morning".to_string(),
                start_tick: SimTick::ZERO,
                end_tick: SimTick::new(4),
            }],
        },
    );

    assert_eq!(report.stuck_diagnostic_event_ids.len(), 0);
    assert!(report.ordinary_pipeline_events > 0);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::NoHumanDayStarted));
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::NoHumanDayCompleted));
    let rendered = format!("{:?}", log.events());
    assert!(!rendered.contains("controller"));
    assert!(!rendered.contains("player"));
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ActorWaited));
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

#[test]
fn phase2a_epistemic_event_kinds_are_nonphysical_and_versioned() {
    for kind in [
        EventKind::InitialBeliefSeeded,
        EventKind::ObservationRecorded,
        EventKind::BeliefUpdated,
        EventKind::ExpectationContradicted,
        EventKind::ContainerChecked,
    ] {
        assert_eq!(kind.stream(), EventStream::Epistemic);
        assert!(!kind.physical_mutating());
        assert!(!kind.stable_id().is_empty());
        assert!(EventKind::registry()
            .iter()
            .any(|metadata| metadata.kind == kind
                && metadata.stream == EventStream::Epistemic
                && !metadata.physical_mutating));
    }
}

#[test]
fn phase3a_event_kinds_are_streamed_versioned_and_replay_visible() {
    for kind in [
        EventKind::NeedDeltaApplied,
        EventKind::NeedThresholdCrossed,
        EventKind::IntentionStarted,
        EventKind::IntentionInterrupted,
        EventKind::RoutineStepStarted,
        EventKind::RoutineStepCompleted,
        EventKind::RoutineStepFailed,
        EventKind::SleepStarted,
        EventKind::SleepCompleted,
        EventKind::FoodConsumed,
        EventKind::EatFailed,
        EventKind::WorkBlockStarted,
        EventKind::WorkBlockCompleted,
        EventKind::WorkBlockFailed,
        EventKind::DecisionTraceRecorded,
        EventKind::StuckDiagnosticRecorded,
        EventKind::NoHumanDayStarted,
        EventKind::NoHumanDayCompleted,
        EventKind::ContinueRoutineProposed,
    ] {
        assert!(!kind.stable_id().is_empty());
        assert!(EventKind::registry()
            .iter()
            .any(|metadata| metadata.kind == kind && metadata.stream == kind.stream()));
        assert_ne!(kind.stream(), EventStream::Controller);
    }
}

#[test]
fn continue_routine_marker_alone_is_not_behavioral_progress() {
    let mut registry = registry();
    registry.register_phase3a_continue_routine();
    let mut state = state(true, true);
    let mut agent_state = agent_state();
    let mut log = EventLog::new();
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_continue_marker_only").unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id()),
        ActionId::new("continue_routine").unwrap(),
        SimTick::new(5),
    );
    proposal.parameters.insert(
        "active_intention_id".to_string(),
        "intention_workday".to_string(),
    );
    proposal
        .parameters
        .insert("next_action_id".to_string(), "move".to_string());

    let result = run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: &mut agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("acceptance_manifest").unwrap(),
            ordering_key: OrderingKey::new(
                SimTick::new(5),
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Actor(actor_id()),
                ProposalSequence::new(0),
                ActionId::new("continue_routine").unwrap(),
                Vec::new(),
                "continue_marker_only",
            ),
        },
        &proposal,
    );

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ContinueRoutineProposed));
    assert!(!log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ActorMoved));
    let metrics = no_human_day_metrics(&log);
    assert_eq!(metrics.routine_event_count, 0);
    assert_eq!(metrics.meals_completed, 0);
    assert_eq!(metrics.sleep_completed, 0);
    assert_eq!(metrics.work_blocks_completed, 0);
}
