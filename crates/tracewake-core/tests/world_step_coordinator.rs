use std::collections::{BTreeMap, BTreeSet};

use tracewake_core::actions::defs::wait::build_wait_events;
use tracewake_core::actions::{ActionRegistry, Proposal, ProposalOrigin};
use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use tracewake_core::events::apply::apply_agent_event;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventCause, EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1,
};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, ControllerId, EventId, FixtureId,
    PlaceId, ProposalId, SleepAffordanceId, WorkplaceId,
};
use tracewake_core::replay::rebuild_projection;
use tracewake_core::scheduler::no_human::{advance_no_human, NoHumanStateMut};
use tracewake_core::scheduler::{
    AdvanceUntilRequest, AdvanceUntilStopReason, AuthorizedSleepInterruption,
    BodyExclusiveDurationKind, DeterministicScheduler, OrderingKey, ProposalSequence,
    SchedulePhase, SchedulerSourceId, WorldAdvanceError, WorldAdvanceOrigin, WorldAdvanceRequest,
};
use tracewake_core::state::{
    ActorBody, AgentState, NeedModelState, PhysicalState, PlaceState, SleepAffordanceState,
    VisibilityDefault, WorkplaceState,
};
use tracewake_core::time::SimTick;

fn content_manifest_id() -> ContentManifestId {
    ContentManifestId::new("test_manifest").unwrap()
}

fn checksum_context(tick: SimTick, world_stream_position_applied: u64) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new("world_step_coordinator").unwrap(),
        content_version: ContentVersion::new("test_manifest").unwrap(),
        sim_tick: tick,
        world_stream_position_applied,
    }
}

fn empty_agent_state() -> AgentState {
    AgentState::from_seed_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn place_id(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn actor_id(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn world_advance_request(expected_tick: u64) -> WorldAdvanceRequest {
    WorldAdvanceRequest {
        expected_tick: SimTick::new(expected_tick),
        origin: WorldAdvanceOrigin::Controller(ControllerId::new("controller_human").unwrap()),
        content_manifest_id: content_manifest_id(),
        authorized_sleep_interruptions: Vec::new(),
    }
}

fn advance_until_request(
    actor_id: &ActorId,
    expected_tick: u64,
    max_ticks: u64,
) -> AdvanceUntilRequest {
    AdvanceUntilRequest {
        expected_tick: SimTick::new(expected_tick),
        origin: WorldAdvanceOrigin::Controller(ControllerId::new("controller_human").unwrap()),
        content_manifest_id: content_manifest_id(),
        possessed_actor_id: actor_id.clone(),
        max_ticks,
    }
}

fn ordering_key(tick: u64, action_id: &str, actor_id: &ActorId) -> OrderingKey {
    OrderingKey::new(
        SimTick::new(tick),
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(0),
        ActionId::new(action_id).unwrap(),
        vec![actor_id.as_str().to_string()],
        action_id,
    )
}

fn duration_start(
    kind: EventKind,
    id: &str,
    actor_id: &ActorId,
    start_tick: u64,
    expected_completion_tick: u64,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        0,
        0,
        SimTick::new(start_tick),
        ordering_key(start_tick, kind.stable_id(), actor_id),
        content_manifest_id(),
        vec![EventCause::Proposal(
            ProposalId::new(format!("proposal_{id}")).unwrap(),
        )],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new(
            "expected_completion_tick",
            expected_completion_tick.to_string(),
        ),
        PayloadField::new("body_exclusive", "true"),
    ];
    event
}

fn sleep_start(
    id: &str,
    actor_id: &ActorId,
    start_tick: u64,
    expected_completion_tick: u64,
    sleep_affordance_id: &SleepAffordanceId,
) -> EventEnvelope {
    let mut event = duration_start(
        EventKind::SleepStarted,
        id,
        actor_id,
        start_tick,
        expected_completion_tick,
    );
    event.payload.extend([
        PayloadField::new("sleep_affordance_id", sleep_affordance_id.as_str()),
        PayloadField::new("fatigue_recovery_per_tick", "20"),
        PayloadField::new("hunger_rise_per_tick", "2"),
    ]);
    event
}

fn work_start(
    id: &str,
    actor_id: &ActorId,
    start_tick: u64,
    expected_completion_tick: u64,
    workplace_id: &WorkplaceId,
    place_id: &PlaceId,
) -> EventEnvelope {
    let mut event = duration_start(
        EventKind::WorkBlockStarted,
        id,
        actor_id,
        start_tick,
        expected_completion_tick,
    );
    event.payload.extend([
        PayloadField::new("workplace_id", workplace_id.as_str()),
        PayloadField::new("place_id", place_id.as_str()),
        PayloadField::new("fatigue_delta_per_tick", "3"),
        PayloadField::new("hunger_delta_per_tick", "4"),
        PayloadField::new("output_tag", "stacked_firewood"),
    ]);
    event
}

fn duration_terminal(
    kind: EventKind,
    id: &str,
    actor_id: &ActorId,
    start_event_id: &str,
    tick: u64,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        0,
        0,
        SimTick::new(tick),
        ordering_key(tick, kind.stable_id(), actor_id),
        content_manifest_id(),
        vec![EventCause::Event(EventId::new(start_event_id).unwrap())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
    event
}

fn wait_events(
    physical: &PhysicalState,
    agent: &AgentState,
    actor: &ActorId,
    requested_tick: u64,
) -> Vec<EventEnvelope> {
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_wait_once").unwrap(),
        ProposalOrigin::Human,
        Some(actor.clone()),
        ActionId::new("wait").unwrap(),
        SimTick::new(requested_tick),
    );
    proposal
        .parameters
        .insert("reason".to_string(), "waiting for the fire".to_string());
    proposal
        .parameters
        .insert("ticks".to_string(), "1".to_string());
    build_wait_events(
        physical,
        agent,
        &proposal,
        &ordering_key(requested_tick, "wait", actor),
        &content_manifest_id(),
    )
    .unwrap()
}

fn physical_state_for(actor: &ActorId, place: &PlaceId) -> PhysicalState {
    physical_state_for_with_need_model(actor, place, NeedModelState::new(0, 0))
}

fn physical_state_for_with_need_model(
    actor: &ActorId,
    place: &PlaceId,
    need_model: NeedModelState,
) -> PhysicalState {
    physical_state_for_actors_with_need_model([actor.clone()], place, need_model)
}

fn physical_state_for_actors_with_need_model(
    actor_ids: impl IntoIterator<Item = ActorId>,
    place: &PlaceId,
    need_model: NeedModelState,
) -> PhysicalState {
    let sleep_affordance_id = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let workplace_id = WorkplaceId::new("workplace_woodpile").unwrap();
    let mut actors = BTreeMap::new();
    let mut local_actor_ids = BTreeSet::new();
    for actor_id in actor_ids {
        actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), place.clone()),
        );
        local_actor_ids.insert(actor_id);
    }
    let mut places = BTreeMap::new();
    places.insert(
        place.clone(),
        PlaceState {
            place_id: place.clone(),
            display_label: "Home".to_string(),
            adjacent_place_ids: BTreeSet::new(),
            connected_door_ids: BTreeSet::new(),
            local_container_ids: BTreeSet::new(),
            local_item_ids: BTreeSet::new(),
            local_actor_ids,
            visibility_default: VisibilityDefault::Visible,
        },
    );
    let mut sleep_affordances = BTreeMap::new();
    sleep_affordances.insert(
        sleep_affordance_id.clone(),
        SleepAffordanceState::new(sleep_affordance_id, place.clone(), 2, 20, 2),
    );
    let mut workplaces = BTreeMap::new();
    workplaces.insert(
        workplace_id.clone(),
        WorkplaceState::new(
            workplace_id,
            place.clone(),
            2,
            3,
            4,
            900,
            900,
            "stacked_firewood",
        ),
    );
    PhysicalState::from_seed_parts(
        actors,
        places,
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        workplaces,
        sleep_affordances,
        need_model,
    )
}

fn agent_state_for(actor: &ActorId) -> AgentState {
    agent_state_for_actors([actor.clone()])
}

fn agent_state_for_actors(actor_ids: impl IntoIterator<Item = ActorId>) -> AgentState {
    let mut needs_by_actor = BTreeMap::new();
    for actor_id in actor_ids {
        needs_by_actor.insert(actor_id, initial_needs());
    }
    AgentState::from_seed_parts(
        needs_by_actor,
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn initial_needs() -> BTreeMap<NeedKind, NeedState> {
    let mut needs = BTreeMap::new();
    needs.insert(
        NeedKind::Fatigue,
        NeedState::initial(NeedKind::Fatigue, 100, NeedChangeCause::FixtureInitial),
    );
    needs.insert(
        NeedKind::Hunger,
        NeedState::initial(NeedKind::Hunger, 100, NeedChangeCause::FixtureInitial),
    );
    needs
}

fn state_checksum_pair(
    physical: &PhysicalState,
    tick: SimTick,
    stream_position: u64,
) -> PhysicalChecksum {
    let context = checksum_context(tick, stream_position);
    compute_physical_checksum(physical, &context).checksum
}

fn need_values(agent: &AgentState) -> BTreeMap<(ActorId, NeedKind), u16> {
    let mut values = BTreeMap::new();
    for (actor_id, needs) in agent.needs_by_actor() {
        for (need_kind, need_state) in needs {
            values.insert((actor_id.clone(), *need_kind), need_state.value());
        }
    }
    values
}

fn event_kind_counts(log: &EventLog) -> BTreeMap<EventKind, usize> {
    let mut counts = BTreeMap::new();
    for event in log.events() {
        *counts.entry(event.event_type).or_insert(0) += 1;
    }
    counts
}

#[test]
fn empty_world_step_appends_time_advanced_and_rebuilds_frontier() {
    let initial_physical = PhysicalState::empty(NeedModelState::new(0, 0));
    let mut initial_agent = empty_agent_state();
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(41));

    let result = scheduler
        .advance_world_one_tick(
            &initial_physical,
            &mut initial_agent,
            &mut log,
            world_advance_request(41),
        )
        .unwrap();

    assert_eq!(result.prior_tick, SimTick::new(41));
    assert_eq!(result.resulting_tick, SimTick::new(42));
    assert_eq!(scheduler.current_tick, SimTick::new(42));
    assert_eq!(result.appended_event_ids.len(), 1);
    assert!(result.due_duration_candidates.is_empty());
    assert_eq!(result.due_work_summary.open_duration_candidates, 0);
    assert_eq!(log.events().len(), 1);

    let marker = &log.events()[0];
    assert_eq!(marker.event_type, EventKind::TimeAdvanced);
    assert_eq!(marker.stream, EventStream::World);
    assert_eq!(marker.sim_tick, SimTick::new(42));
    assert_eq!(marker.causes.len(), 1);
    assert!(marker.actor_id.is_none());
    assert_eq!(marker.payload_value("prior_tick"), Some("41"));
    assert_eq!(marker.payload_value("resulting_tick"), Some("42"));
    assert_eq!(
        marker.payload_value("origin"),
        Some("controller.controller_human")
    );
    assert_eq!(
        marker.payload_value("ordering_ancestry"),
        Some("canonical_world_step_v1")
    );

    let rebuild = rebuild_projection(
        &initial_physical,
        &empty_agent_state(),
        &log,
        &checksum_context(SimTick::new(41), 0),
        Some(&initial_physical),
    );

    assert_eq!(rebuild.reconstructed_final_frontier, SimTick::new(42));
    assert!(rebuild.temporal_violations.is_empty());
    assert_eq!(rebuild.event_count_applied, 1);
    assert_eq!(rebuild.last_event_id, Some(marker.event_id.clone()));
    assert_eq!(rebuild.last_stream_position, Some(0));
    assert!(rebuild.invariant_violations.is_empty());
    assert_eq!(rebuild.final_state, initial_physical);
    assert_eq!(rebuild.final_agent_state, empty_agent_state());
}

#[test]
fn differential_human_wait_and_no_human_wait_match_authoritative_outcome() {
    let possessed = actor_id("actor_tomas");
    let unpossessed = actor_id("actor_mara");
    let place = place_id("home_tomas");
    let sleep_affordance = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let initial_physical = physical_state_for_actors_with_need_model(
        [possessed.clone(), unpossessed.clone()],
        &place,
        NeedModelState::new(0, 0),
    );
    let initial_agent = agent_state_for_actors([possessed.clone(), unpossessed.clone()]);

    let human_physical = initial_physical.clone();
    let mut human_agent = initial_agent.clone();
    let mut human_log = EventLog::new();
    human_log
        .append(sleep_start(
            "event_sleep_started_unpossessed",
            &unpossessed,
            0,
            1,
            &sleep_affordance,
        ))
        .unwrap();
    for event in wait_events(&human_physical, &human_agent, &possessed, 0) {
        let appended = human_log.append(event).unwrap();
        apply_agent_event(&mut human_agent, &appended).unwrap();
    }
    let mut human_scheduler = DeterministicScheduler::new(SimTick::ZERO);
    let human_result = human_scheduler
        .advance_world_one_tick(
            &human_physical,
            &mut human_agent,
            &mut human_log,
            world_advance_request(0),
        )
        .unwrap();

    let mut no_human_physical = initial_physical;
    let mut no_human_agent = initial_agent;
    let mut no_human_log = EventLog::new();
    no_human_log
        .append(sleep_start(
            "event_sleep_started_unpossessed",
            &unpossessed,
            0,
            1,
            &sleep_affordance,
        ))
        .unwrap();
    let registry = ActionRegistry::new();
    let no_human_report = advance_no_human(
        NoHumanStateMut {
            physical: &mut no_human_physical,
            agent: &mut no_human_agent,
        },
        &mut no_human_log,
        &registry,
        content_manifest_id(),
        SimTick::ZERO,
        1,
        Vec::new(),
    );

    assert_eq!(human_result.resulting_tick, SimTick::new(1));
    assert_eq!(no_human_report.final_tick, SimTick::new(1));
    assert_eq!(human_result.due_duration_candidates.len(), 1);
    assert_eq!(
        human_result.due_duration_candidates[0].actor_id,
        unpossessed
    );
    assert_eq!(
        human_result.due_duration_candidates[0].duration_kind,
        BodyExclusiveDurationKind::Sleep
    );
    assert_eq!(human_result.due_work_summary.duration_terminals_appended, 1);
    assert_eq!(
        event_kind_counts(&human_log).get(&EventKind::SleepCompleted),
        Some(&1)
    );
    assert_eq!(
        event_kind_counts(&no_human_log).get(&EventKind::SleepCompleted),
        Some(&1)
    );
    assert_eq!(
        event_kind_counts(&human_log).get(&EventKind::ActorWaited),
        Some(&1)
    );
    assert_eq!(
        event_kind_counts(&no_human_log).get(&EventKind::ActorWaited),
        None
    );
    assert_eq!(human_physical, no_human_physical);
    assert_eq!(need_values(&human_agent), need_values(&no_human_agent));
    assert_eq!(
        state_checksum_pair(&human_physical, SimTick::new(1), 0),
        state_checksum_pair(&no_human_physical, SimTick::new(1), 0)
    );

    let human_time_origin = human_log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::TimeAdvanced)
        .and_then(|event| event.payload_value("origin"))
        .expect("human path records time origin");
    let no_human_time_origin = no_human_log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::TimeAdvanced)
        .and_then(|event| event.payload_value("origin"))
        .expect("no-human path records time origin");
    assert_eq!(human_time_origin, "controller.controller_human");
    assert_eq!(no_human_time_origin, "process.no_human_advance");
    assert_ne!(human_time_origin, no_human_time_origin);
    assert_eq!(no_human_report.marker_event_ids.len(), 2);
    assert_eq!(no_human_report.ordinary_pipeline_events, 0);
}

#[test]
fn awake_world_step_emits_missing_passive_need_accounting_once() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for_with_need_model(&actor, &place, NeedModelState::new(5, 7));
    let mut agent = agent_state_for(&actor);
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap();

    assert_eq!(result.resulting_tick, SimTick::new(10));
    assert_eq!(
        log.events()
            .iter()
            .filter(|event| event.event_type == EventKind::NeedDeltaApplied
                && event.payload_value("accounting_phase") == Some("world_step"))
            .count(),
        2
    );
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Hunger))
            .map(NeedState::value),
        Some(105)
    );
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Fatigue))
            .map(NeedState::value),
        Some(107)
    );
}

#[test]
fn accepted_wait_tick_charge_is_not_replaced_or_double_charged_by_world_step() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for_with_need_model(&actor, &place, NeedModelState::new(5, 7));
    let mut agent = agent_state_for(&actor);
    let mut log = EventLog::new();
    for event in wait_events(&physical, &agent, &actor, 9) {
        let appended = log.append(event).unwrap();
        apply_agent_event(&mut agent, &appended).unwrap();
    }
    let waited_event_id = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::ActorWaited)
        .map(|event| event.event_id.clone())
        .expect("wait event exists");
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap();

    assert_eq!(result.resulting_tick, SimTick::new(10));
    assert!(log.events().iter().any(|event| {
        event.event_id == waited_event_id
            && event.event_type == EventKind::ActorWaited
            && event.payload_value("reason") == Some("waiting for the fire")
    }));
    assert_eq!(
        log.events()
            .iter()
            .filter(|event| event.event_type == EventKind::NeedDeltaApplied
                && event.payload_value("accounting_phase") == Some("world_step"))
            .count(),
        0
    );
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Hunger))
            .map(NeedState::value),
        Some(105)
    );
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Fatigue))
            .map(NeedState::value),
        Some(107)
    );
}

#[test]
fn coordinator_discovers_due_open_duration_from_prior_transaction() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let sleep_affordance = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let mut log = EventLog::new();
    log.append(sleep_start(
        "event_sleep_started_prior",
        &actor,
        8,
        10,
        &sleep_affordance,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap();

    assert_eq!(result.resulting_tick, SimTick::new(10));
    assert_eq!(result.due_work_summary.open_duration_candidates, 1);
    assert_eq!(result.due_work_summary.duration_terminals_appended, 1);
    assert_eq!(result.due_duration_candidates.len(), 1);
    assert_eq!(
        result.due_duration_candidates[0].start_event_id.as_str(),
        "event_sleep_started_prior"
    );
    assert_eq!(result.due_duration_candidates[0].actor_id, actor);
    assert_eq!(
        result.due_duration_candidates[0].duration_kind,
        BodyExclusiveDurationKind::Sleep
    );
    assert_eq!(
        result.due_duration_candidates[0].expected_completion_tick,
        SimTick::new(10)
    );
    assert_eq!(log.events().len(), 5);
    assert_eq!(log.events()[1].event_type, EventKind::TimeAdvanced);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepCompleted
            && event.causes
                == vec![EventCause::Event(
                    EventId::new("event_sleep_started_prior").unwrap()
                )]));
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Fatigue))
            .map(NeedState::value),
        Some(60)
    );
}

#[test]
fn advance_until_zero_ticks_reports_user_pause_without_world_step() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_until(
            &physical,
            &mut agent,
            &mut log,
            advance_until_request(&actor, 9, 0),
        )
        .unwrap();

    assert_eq!(result.stop_tick, SimTick::new(9));
    assert_eq!(result.ticks_advanced, 0);
    assert_eq!(
        result.stop_reason,
        AdvanceUntilStopReason::UserPausedBeforeNextTick
    );
    assert!(result.appended_event_ids.is_empty());
    assert!(log.events().is_empty());
}

#[test]
fn advance_until_stops_at_controller_safety_bound() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let mut log = EventLog::new();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_until(
            &physical,
            &mut agent,
            &mut log,
            advance_until_request(&actor, 9, 2),
        )
        .unwrap();

    assert_eq!(result.stop_tick, SimTick::new(11));
    assert_eq!(result.ticks_advanced, 2);
    assert_eq!(
        result.stop_reason,
        AdvanceUntilStopReason::ControllerSafetyBound
    );
    assert_eq!(
        log.events()
            .iter()
            .filter(|event| event.event_type == EventKind::TimeAdvanced)
            .count(),
        2
    );
}

#[test]
fn advance_until_stops_at_possessed_duration_terminal() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let sleep_affordance = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let mut log = EventLog::new();
    log.append(sleep_start(
        "event_sleep_started_prior",
        &actor,
        8,
        10,
        &sleep_affordance,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_until(
            &physical,
            &mut agent,
            &mut log,
            advance_until_request(&actor, 9, 8),
        )
        .unwrap();

    assert_eq!(result.stop_tick, SimTick::new(10));
    assert_eq!(result.ticks_advanced, 1);
    assert_eq!(
        result.stop_reason,
        AdvanceUntilStopReason::PossessedDurationTerminal
    );
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepCompleted
            && event.actor_id.as_ref() == Some(&actor)));
}

#[test]
fn advance_until_ignores_hidden_other_actor_duration_terminal() {
    let actor = actor_id("actor_tomas");
    let other_actor = actor_id("actor_mara");
    let place = place_id("home_tomas");
    let physical = physical_state_for_actors_with_need_model(
        [actor.clone(), other_actor.clone()],
        &place,
        NeedModelState::new(0, 0),
    );
    let mut agent = agent_state_for_actors([actor.clone(), other_actor.clone()]);
    let sleep_affordance = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let mut log = EventLog::new();
    log.append(sleep_start(
        "event_sleep_started_other",
        &other_actor,
        8,
        10,
        &sleep_affordance,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_until(
            &physical,
            &mut agent,
            &mut log,
            advance_until_request(&actor, 9, 2),
        )
        .unwrap();

    assert_eq!(result.stop_tick, SimTick::new(11));
    assert_eq!(
        result.stop_reason,
        AdvanceUntilStopReason::ControllerSafetyBound
    );
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepCompleted
            && event.actor_id.as_ref() == Some(&other_actor)));
    assert!(!log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::SleepCompleted
            && event.actor_id.as_ref() == Some(&actor)));
}

#[test]
fn malformed_due_duration_start_rejects_without_appending_marker() {
    let actor = actor_id("actor_tomas");
    let physical = PhysicalState::empty(NeedModelState::new(0, 0));
    let mut agent = empty_agent_state();
    let mut log = EventLog::new();
    log.append(duration_start(
        EventKind::WorkBlockStarted,
        "event_work_started_tomas",
        &actor,
        6,
        10,
    ))
    .unwrap();
    log.append(duration_start(
        EventKind::SleepStarted,
        "event_sleep_started_ada",
        &actor,
        7,
        10,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap_err();

    assert!(matches!(
        result,
        WorldAdvanceError::DurationLifecycleBuild { .. }
    ));
    assert_eq!(scheduler.current_tick, SimTick::new(9));
    assert_eq!(log.events().len(), 2);
}

#[test]
fn coordinator_closes_due_work_through_existing_completion_builder() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let workplace = WorkplaceId::new("workplace_woodpile").unwrap();
    let mut log = EventLog::new();
    log.append(work_start(
        "event_work_started_prior",
        &actor,
        8,
        10,
        &workplace,
        &place,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap();

    assert_eq!(result.due_work_summary.open_duration_candidates, 1);
    assert_eq!(result.due_work_summary.duration_terminals_appended, 1);
    assert!(log
        .events()
        .iter()
        .any(|event| event.event_type == EventKind::WorkBlockCompleted
            && event.causes
                == vec![EventCause::Event(
                    EventId::new("event_work_started_prior").unwrap()
                )]));
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Fatigue))
            .map(NeedState::value),
        Some(106)
    );
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Hunger))
            .map(NeedState::value),
        Some(108)
    );
}

#[test]
fn authorized_sleep_interruption_closes_before_expected_completion_tick() {
    let actor = actor_id("actor_tomas");
    let place = place_id("home_tomas");
    let physical = physical_state_for(&actor, &place);
    let mut agent = agent_state_for(&actor);
    let sleep_affordance = SleepAffordanceId::new("sleep_mat_home").unwrap();
    let start_event_id = EventId::new("event_sleep_started_prior").unwrap();
    let mut log = EventLog::new();
    log.append(sleep_start(
        start_event_id.as_str(),
        &actor,
        8,
        12,
        &sleep_affordance,
    ))
    .unwrap();
    let mut request = world_advance_request(9);
    request
        .authorized_sleep_interruptions
        .push(AuthorizedSleepInterruption {
            start_event_id: start_event_id.clone(),
            terminal_tick: SimTick::new(10),
            reason: "controller_interrupt".to_string(),
        });
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let result = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, request)
        .unwrap();

    assert!(result.due_duration_candidates.is_empty());
    assert_eq!(result.due_work_summary.duration_terminals_appended, 1);
    let interruption = log
        .events()
        .iter()
        .find(|event| event.event_type == EventKind::SleepInterrupted)
        .expect("sleep interruption is appended");
    assert_eq!(
        interruption.causes,
        vec![EventCause::Event(start_event_id.clone())]
    );
    assert_eq!(
        interruption.payload_value("reason"),
        Some("controller_interrupt")
    );
    assert_eq!(interruption.payload_value("elapsed_ticks"), Some("2"));
    assert_eq!(
        agent
            .needs_by_actor()
            .get(&actor)
            .and_then(|needs| needs.get(&NeedKind::Fatigue))
            .map(NeedState::value),
        Some(60)
    );
}

#[test]
fn duplicate_duration_terminal_rejects_world_step_without_appending_marker() {
    let actor = actor_id("actor_tomas");
    let physical = PhysicalState::empty(NeedModelState::new(0, 0));
    let mut agent = empty_agent_state();
    let mut log = EventLog::new();
    log.append(duration_start(
        EventKind::WorkBlockStarted,
        "event_work_started_prior",
        &actor,
        8,
        10,
    ))
    .unwrap();
    log.append(duration_terminal(
        EventKind::WorkBlockCompleted,
        "event_work_completed_prior",
        &actor,
        "event_work_started_prior",
        10,
    ))
    .unwrap();
    log.append(duration_terminal(
        EventKind::WorkBlockFailed,
        "event_work_failed_duplicate",
        &actor,
        "event_work_started_prior",
        10,
    ))
    .unwrap();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(9));

    let error = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap_err();

    let WorldAdvanceError::OpenDurationDiscovery(error) = error else {
        panic!("expected open-duration discovery failure");
    };
    assert_eq!(error.start_event_id.as_str(), "event_work_started_prior");
    assert_eq!(
        error.first_terminal_event_id.as_str(),
        "event_work_completed_prior"
    );
    assert_eq!(
        error.duplicate_terminal_event_id.as_str(),
        "event_work_failed_duplicate"
    );
    assert_eq!(scheduler.current_tick, SimTick::new(9));
    assert_eq!(log.events().len(), 3);
}

#[test]
fn stale_expected_tick_rejects_without_appending_or_advancing() {
    let mut log = EventLog::new();
    let physical = PhysicalState::empty(NeedModelState::new(0, 0));
    let mut agent = empty_agent_state();
    let mut scheduler = DeterministicScheduler::new(SimTick::new(10));

    let error = scheduler
        .advance_world_one_tick(&physical, &mut agent, &mut log, world_advance_request(9))
        .unwrap_err();

    assert_eq!(
        error,
        WorldAdvanceError::FrontierMismatch {
            expected: SimTick::new(9),
            actual: SimTick::new(10),
        }
    );
    assert_eq!(scheduler.current_tick, SimTick::new(10));
    assert!(log.events().is_empty());
}

#[test]
fn time_advanced_metadata_is_world_cause_required_marker() {
    let metadata = EventKind::TimeAdvanced.metadata();

    assert_eq!(metadata.stream, EventStream::World);
    assert!(metadata.physical_mutating);
    assert!(metadata.cause_required);
    assert!(!tracewake_core::events::is_duration_terminal(
        EventKind::TimeAdvanced
    ));
}

trait PayloadLookup {
    fn payload_value(&self, key: &str) -> Option<&str>;
}

impl PayloadLookup for tracewake_core::events::EventEnvelope {
    fn payload_value(&self, key: &str) -> Option<&str> {
        self.payload
            .iter()
            .find(|field| field.key == key)
            .map(|field| field.value.as_str())
    }
}
