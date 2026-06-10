use std::collections::BTreeMap;

mod support;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::{
    run_pipeline, ActionRegistry, ActionScope, PipelineContext, PipelineStage, Proposal,
    ProposalOrigin, ProposalSource, ProposalSourceContext, ReasonCode, ReportStatus,
};
use tracewake_core::agent::{
    current_place_knowledge_context, NeedChangeCause, NeedKind, NeedState, RoutineExecution,
    RoutineFamily,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext,
};
use tracewake_core::controller::ControllerBindings;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1,
};
use tracewake_core::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, ControllerId,
    DecisionTraceId, EventId, FixtureId, FoodSupplyId, ItemId, PlaceId, ProposalId,
    RoutineExecutionId, RoutineTemplateId, SemanticActionId, SleepAffordanceId, ViewModelId,
    WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::no_human_day_metrics;
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::no_human::{run_no_human_day, DayWindow, NoHumanDayConfig};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, ControllerMode, DoorState, FoodSupplyState, ItemState,
    PhysicalState, PlaceState, SleepAffordanceState, WorkplaceState,
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
    state.sleep_affordances_mut().insert(
        SleepAffordanceId::new("bed_shop_front").unwrap(),
        SleepAffordanceState::new(
            SleepAffordanceId::new("bed_shop_front").unwrap(),
            shop.clone(),
            4,
            20,
            2,
        ),
    );

    let mut door = DoorState::new("door_shop_back".parse().unwrap(), shop.clone(), back);
    door.is_open = door_open;
    state.doors_mut().insert(door.door_id.clone(), door);

    let mut container =
        ContainerState::fixed_at_place(ContainerId::new("strongbox_tomas").unwrap(), shop.clone());
    container.is_open = container_open;
    container
        .contents
        .insert(ItemId::new("coin_stack_01").unwrap());
    state
        .containers_mut()
        .insert(ContainerId::new("strongbox_tomas").unwrap(), container);
    state.items_mut().insert(
        ItemId::new("coin_stack_01").unwrap(),
        ItemState::new(
            ItemId::new("coin_stack_01").unwrap(),
            Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
        ),
    );
    state.build()
}

fn agent_state() -> AgentState {
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
    state.build()
}

fn phase3a_registry() -> ActionRegistry {
    let mut registry = registry();
    registry.register_phase3a_eat();
    registry
}

#[test]
fn out_of_active_scope_action_rejects_at_phase_boundary() {
    let mut registry = ActionRegistry::new_with_active_scopes([ActionScope::Phase1]);
    registry.register_phase1_movement_open_close();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();

    let mut state = state(true, true);
    let mut agent_state = agent_state();
    let mut log = EventLog::new();
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_check_scope_rejected").unwrap(),
        ProposalOrigin::Test,
        Some(actor_id()),
        ActionId::new("check_container").unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids.push("strongbox_tomas".to_string());
    let ordering_key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(0),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        "phase-boundary-rejection",
    );

    let result = run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: &mut agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key,
        },
        &proposal,
    );

    assert_eq!(result.report.status, ReportStatus::Rejected);
    assert_eq!(
        result.report.failed_stage,
        Some(PipelineStage::PhaseBoundaryValidation)
    );
    assert_eq!(
        result.report.reason_codes,
        vec![ReasonCode::PhaseUnsupportedAction]
    );
    assert_eq!(result.appended_events.len(), 1);
    assert_eq!(
        result.appended_events[0].event_type,
        EventKind::ActionRejected
    );
}

fn capstone_registry() -> ActionRegistry {
    let mut registry = registry();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

fn agent_need(agent_state: &AgentState, need: NeedKind) -> u16 {
    agent_state.needs_by_actor()[&actor_id()][&need].value()
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
        ];
        log.append(event).unwrap();
    }
    log
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

fn has_event(log: &EventLog, kind: EventKind) -> bool {
    log.events().iter().any(|event| event.event_type == kind)
}

fn first_event_index(log: &EventLog, kind: EventKind) -> usize {
    log.events()
        .iter()
        .position(|event| event.event_type == kind)
        .unwrap_or_else(|| panic!("missing event {kind:?}"))
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
    proposal.parameters.insert(
        "sleep_affordance_id".to_string(),
        "bed_shop_front".to_string(),
    );
    let mut bindings = None;
    if is_human_origin {
        let controller_id = ControllerId::new("controller_human").unwrap();
        let content_manifest_id = ContentManifestId::new("phase3a_manifest").unwrap();
        let source_context = current_place_knowledge_context(
            state,
            None,
            &actor_id(),
            proposal.requested_tick,
            &content_manifest_id,
            0,
        );
        let source_view_model_id = ViewModelId::new("view.actor_tomas.0").unwrap();
        proposal.source_view_model_id = Some(source_view_model_id.clone());
        proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
            source_view_model_id,
            holder_known_context_id: source_context.holder_known_context_id().clone(),
            holder_known_context_hash: source_context.holder_known_context_hash().clone(),
            holder_known_context_frontier: source_context.event_frontier(),
            context_tick: proposal.requested_tick,
            actor_id: actor_id(),
            semantic_action_id: SemanticActionId::new("sleep.here").unwrap(),
            provenance_ancestry: Vec::new(),
        }));
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
    let mut world_seed = PhysicalSeed::from_state(&state(true, true));
    world_seed.food_supplies_mut().insert(
        FoodSupplyId::new("food_supply_home").unwrap(),
        FoodSupplyState::new(
            FoodSupplyId::new("food_supply_home").unwrap(),
            Location::AtPlace(PlaceId::new("shop_front").unwrap()),
            2,
            40,
        ),
    );
    let mut world = world_seed.build();
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
    let mut agent_seed = AgentSeed::from_state(&agent_state());
    agent_seed
        .needs_by_actor_mut()
        .entry(actor_id())
        .or_default()
        .insert(
            NeedKind::Fatigue,
            NeedState::initial(NeedKind::Fatigue, 820, NeedChangeCause::TickDelta),
        );
    add_routine_execution(
        &mut agent_seed,
        "routine_exec_tomas_sleep",
        "actor_tomas",
        "routine_tomas_sleep",
        RoutineFamily::SleepNight,
        0,
        4,
    );
    let mut agent_state = agent_seed.build();
    let mut registry = registry();
    registry.register_phase3a_sleep();
    let mut log = EventLog::new();
    let report = run_no_human_day(
        &mut world,
        &mut agent_state,
        &mut log,
        &registry,
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
        .any(|event| event.event_type == EventKind::SleepStarted));
    assert!(!log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::ActorWaited));
}

#[test]
fn integrated_no_human_day_capstone_emerges_from_one_autonomous_run() {
    let (mut world, mut agent_state, expected_roster) = capstone_world_and_agents();
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let registry = capstone_registry();
    let content_manifest_id = ContentManifestId::new("phase3a_capstone_manifest").unwrap();
    let mut log = capstone_seed_log(&content_manifest_id);
    let windows = vec![
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
    ];
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
    assert_eq!(report.marker_event_ids.len(), 2);
    assert!(report.ordinary_pipeline_events > expected_roster.len());

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
            has_event(&log, required),
            "capstone missing required event {required:?}"
        );
    }

    assert!(
        first_event_index(&log, EventKind::NeedDeltaApplied)
            < first_event_index(&log, EventKind::FoodConsumed)
    );
    assert!(
        first_event_index(&log, EventKind::ActorMoved)
            < first_event_index(&log, EventKind::WorkBlockStarted)
    );
    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::ActorWaited
            && event
                .payload
                .iter()
                .any(|field| field.key == "reason" && !field.value.starts_with("no_human_day:"))
    }));
    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::WorkBlockFailed
            && event
                .payload
                .iter()
                .any(|field| field.key == "blocker_kind" && field.value == "access")
    }));
    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::RoutineStepFailed
            && event
                .payload
                .iter()
                .any(|field| field.key == "reason" && field.value == "work_block_failed")
    }));
    assert!(log.events().iter().any(|event| {
        event.event_type == EventKind::IntentionContinued
            && event.payload.iter().any(|field| {
                field.key == "reason" && field.value == "ordinary_follow_on_action_committed"
            })
    }));
    for actor_id in &expected_roster {
        assert!(
            log.events().iter().any(|event| {
                event
                    .actor_id
                    .as_ref()
                    .is_some_and(|event_actor_id| event_actor_id == actor_id)
            }),
            "unpossessed actor {} produced no event",
            actor_id.as_str()
        );
    }

    let metrics = no_human_day_metrics(&log);
    assert_eq!(metrics.projection_version, "no_human_day_metrics_v1");
    assert!(metrics.events_per_day > 0);
    assert!(metrics.meals_completed > 0);
    assert!(metrics.sleep_completed > 0);
    assert!(metrics.work_blocks_completed > 0);
    assert!(metrics.work_blocks_failed > 0);
    assert_eq!(metrics.player_conditioned_event_count, 0);
    assert_eq!(metrics.player_conditioned_event_rate_per_1000, 0);

    let rendered = format!("{:?}", log.events()).to_ascii_lowercase();
    assert!(!rendered.contains("player"));
    assert!(!rendered.contains("controller"));
    assert!(!rendered.contains("food_hidden"));
    assert!(!rendered.contains("hidden_route"));
    assert!(!log
        .events()
        .iter()
        .any(|event| event.stream == EventStream::Controller));

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
