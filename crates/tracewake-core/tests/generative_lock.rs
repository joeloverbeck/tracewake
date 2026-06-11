mod support;

use std::collections::BTreeSet;

use support::generative::{
    actor_id, content_manifest_id, generate_case, initial_agent_state, initial_world,
    no_human_state_mut, registry, scheduled_proposals, windows, GENERATIVE_SEEDS,
};
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventKind, EventStream};
use tracewake_core::ids::{ContentVersion, FixtureId};
use tracewake_core::replay::{rebuild_projection, run_replay};
use tracewake_core::scheduler::no_human::advance_no_human;
use tracewake_core::time::SimTick;

#[derive(Default)]
struct Reachability {
    actor_waited: bool,
    need_delta: bool,
    food_consumed: bool,
    sleep_block: bool,
    work_block: bool,
    duration_terminal: bool,
    interruption: bool,
    no_human_marker: bool,
    replay_round_trip: bool,
    prefix_replay: bool,
}

#[test]
fn generated_sequences_replay_and_satisfy_metamorphic_locks() {
    let mut reachability = Reachability::default();
    let mut masks = BTreeSet::new();
    let mut sequence_lengths = BTreeSet::new();
    let mut terminal_kinds = BTreeSet::new();
    for seed in GENERATIVE_SEEDS {
        let case = generate_case(*seed);
        masks.insert(case.mask.stable_id());
        sequence_lengths.insert(case.sequence.len());
        let run = run_case(&case);
        reachability.actor_waited |= has_event(&run.log, EventKind::ActorWaited);
        reachability.need_delta |= has_event(&run.log, EventKind::NeedDeltaApplied);
        reachability.food_consumed |= has_event(&run.log, EventKind::FoodConsumed);
        reachability.sleep_block |= has_event(&run.log, EventKind::SleepStarted);
        reachability.work_block |= has_event(&run.log, EventKind::WorkBlockStarted);
        reachability.duration_terminal |= run.log.events().iter().any(|event| {
            matches!(
                event.event_type,
                EventKind::SleepCompleted
                    | EventKind::SleepInterrupted
                    | EventKind::WorkBlockCompleted
                    | EventKind::WorkBlockFailed
            )
        });
        reachability.interruption |= has_event(&run.log, EventKind::SleepInterrupted);
        terminal_kinds.extend(
            run.log
                .events()
                .iter()
                .filter_map(|event| match event.event_type {
                    EventKind::SleepCompleted
                    | EventKind::SleepInterrupted
                    | EventKind::WorkBlockCompleted
                    | EventKind::WorkBlockFailed => Some(event.event_type.stable_id()),
                    _ => None,
                }),
        );
        reachability.no_human_marker |= has_event(&run.log, EventKind::NoHumanAdvanceStarted)
            && has_event(&run.log, EventKind::NoHumanAdvanceCompleted);

        assert_live_matches_replay(&run, *seed);
        assert_serialization_round_trip(&run.log, *seed);
        assert_prefix_replay_matches_full(&run, *seed);
        assert_marker_append_does_not_change_physical_checksum(&run, *seed);
        assert_single_need_charge_per_actor_tick_kind(&run.log, *seed);

        reachability.replay_round_trip = true;
        reachability.prefix_replay = true;
    }

    let corpus_summary =
        format!("seeds={GENERATIVE_SEEDS:x?} masks={masks:?} lengths={sequence_lengths:?}");
    assert!(
        reachability.actor_waited,
        "generated corpus never waited; {corpus_summary}"
    );
    assert!(
        reachability.need_delta,
        "generated corpus never charged needs; {corpus_summary}"
    );
    assert!(
        reachability.food_consumed,
        "generated corpus never consumed food; {corpus_summary}"
    );
    assert!(
        reachability.sleep_block,
        "generated corpus never started sleep; {corpus_summary}"
    );
    assert!(
        reachability.work_block,
        "generated corpus never started work; {corpus_summary}"
    );
    assert!(
        reachability.duration_terminal,
        "generated corpus never emitted duration terminals; terminals={terminal_kinds:?} {corpus_summary}"
    );
    assert!(
        reachability.interruption,
        "generated corpus never emitted interruption terminal; terminals={terminal_kinds:?} {corpus_summary}"
    );
    assert!(
        reachability.no_human_marker,
        "generated corpus never emitted no-human markers"
    );
    assert!(
        reachability.replay_round_trip,
        "generated corpus never exercised replay"
    );
    assert!(
        reachability.prefix_replay,
        "generated corpus never exercised prefix replay"
    );
    assert!(
        terminal_kinds.len() >= 2,
        "generated corpus terminal diversity too low: {terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        masks.len() >= 4,
        "generated corpus mask diversity too low: {masks:?}; {corpus_summary}"
    );
    assert!(
        sequence_lengths.len() >= 2,
        "generated corpus sequence-length diversity too low: {sequence_lengths:?}; {corpus_summary}"
    );
}

struct GeneratedRun {
    initial_world: tracewake_core::state::PhysicalState,
    initial_agent_state: tracewake_core::state::AgentState,
    final_world: tracewake_core::state::PhysicalState,
    final_agent_state: tracewake_core::state::AgentState,
    log: EventLog,
    final_tick: SimTick,
}

fn run_case(case: &support::generative::GeneratedCase) -> GeneratedRun {
    let mut world = initial_world(case.seed);
    let mut agent_state = initial_agent_state(case.seed);
    let initial_world = world.clone();
    let initial_agent_state = agent_state.clone();
    let mut log = EventLog::new();
    let registry = registry();
    let report = advance_no_human(
        no_human_state_mut(&mut world, &mut agent_state),
        &mut log,
        &registry,
        content_manifest_id(case.seed),
        case.start_tick,
        case.end_tick
            .value()
            .saturating_sub(case.start_tick.value()),
        scheduled_proposals(case),
    );
    assert_eq!(report.final_tick, case.end_tick, "seed={}", case.seed);
    assert_eq!(
        windows(case).len(),
        case.sequence.len(),
        "seed={}",
        case.seed
    );
    GeneratedRun {
        initial_world,
        initial_agent_state,
        final_world: world,
        final_agent_state: agent_state,
        log,
        final_tick: report.final_tick,
    }
}

fn checksum_context(seed: u64, log: &EventLog, tick: SimTick) -> ChecksumContext {
    ChecksumContext {
        fixture_id: FixtureId::new(format!("generative_lock_{seed:x}")).unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
        sim_tick: tick,
        world_stream_position_applied: log
            .events()
            .iter()
            .filter(|event| event.stream == EventStream::World)
            .count()
            .saturating_sub(1) as u64,
    }
}

fn assert_live_matches_replay(run: &GeneratedRun, seed: u64) {
    let context = checksum_context(seed, &run.log, run.final_tick);
    let live_physical = compute_physical_checksum(&run.final_world, &context).checksum;
    let live_agent = compute_agent_state_checksum(&run.final_agent_state, &context).checksum;
    let replay = run_replay(
        &run.initial_world,
        &run.initial_agent_state,
        &run.log,
        &context,
        Some(&run.final_world),
        Some(live_physical.clone()),
        Some(live_agent.clone()),
    );
    assert!(replay.matches_expected, "seed={seed} replay={replay:?}");
    assert!(
        replay.agent_checksum_matches,
        "seed={seed} replay={replay:?}"
    );
    assert_eq!(replay.final_checksum, live_physical, "seed={seed}");
    assert_eq!(replay.final_agent_checksum, live_agent, "seed={seed}");
}

fn assert_serialization_round_trip(log: &EventLog, seed: u64) {
    let encoded = log.serialize_canonical();
    let decoded = EventLog::deserialize_canonical(&encoded).unwrap();
    assert_eq!(decoded, *log, "seed={seed}");
}

fn assert_prefix_replay_matches_full(run: &GeneratedRun, seed: u64) {
    let midpoint = run.log.events().len() / 2;
    let mut prefix = EventLog::new();
    for event in run.log.events().iter().take(midpoint).cloned() {
        prefix.append(event).unwrap();
    }
    let prefix_context = checksum_context(seed, &prefix, run.final_tick);
    let prefix_rebuild = rebuild_projection(
        &run.initial_world,
        &run.initial_agent_state,
        &prefix,
        &prefix_context,
        None,
    );
    assert!(
        prefix_rebuild.invariant_violations.is_empty(),
        "seed={seed} prefix={prefix_rebuild:?}"
    );

    let full_context = checksum_context(seed, &run.log, run.final_tick);
    let full_rebuild = rebuild_projection(
        &run.initial_world,
        &run.initial_agent_state,
        &run.log,
        &full_context,
        Some(&run.final_world),
    );
    assert!(
        full_rebuild.invariant_violations.is_empty(),
        "seed={seed} full={full_rebuild:?}"
    );
    assert_eq!(
        full_rebuild.final_checksum,
        compute_physical_checksum(&run.final_world, &full_context).checksum,
        "seed={seed}"
    );
}

fn assert_marker_append_does_not_change_physical_checksum(run: &GeneratedRun, seed: u64) {
    let before_context = checksum_context(seed, &run.log, run.final_tick);
    let before_physical = compute_physical_checksum(&run.final_world, &before_context).checksum;
    let before_agent =
        compute_agent_state_checksum(&run.final_agent_state, &before_context).checksum;
    let mut marker_log = run.log.clone();
    let marker = run
        .log
        .events()
        .iter()
        .rev()
        .find(|event| event.event_type == EventKind::NoHumanAdvanceCompleted)
        .cloned()
        .expect("generated run emits completion marker");
    marker_log.append(marker).unwrap();
    let after_context = checksum_context(seed, &marker_log, run.final_tick);
    let after_physical = compute_physical_checksum(&run.final_world, &after_context).checksum;
    let after_agent = compute_agent_state_checksum(&run.final_agent_state, &after_context).checksum;
    assert_eq!(before_physical, after_physical, "seed={seed}");
    assert_eq!(before_agent, after_agent, "seed={seed}");
}

fn assert_single_need_charge_per_actor_tick_kind(log: &EventLog, seed: u64) {
    let mut seen = BTreeSet::new();
    for event in log
        .events()
        .iter()
        .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
    {
        let actor = event
            .actor_id
            .as_ref()
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| actor_id().as_str().to_string());
        let need_kind = payload_value(event, "need_kind").unwrap_or("unknown");
        let key = (actor, event.sim_tick.value(), need_kind.to_string());
        assert!(
            seen.insert(key.clone()),
            "seed={seed} duplicate need charge key={key:?}"
        );
    }
}

fn has_event(log: &EventLog, kind: EventKind) -> bool {
    log.events().iter().any(|event| event.event_type == kind)
}

fn payload_value<'a>(
    event: &'a tracewake_core::events::EventEnvelope,
    key: &str,
) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}
