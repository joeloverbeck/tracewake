mod support;

use std::collections::{BTreeMap, BTreeSet};

use support::generative::{
    actor_id, content_manifest_id, generate_case, initial_agent_state, initial_world,
    no_human_state_mut, registry, scheduled_proposals, windows, GENERATIVE_SEEDS,
    SLEEP_INTERRUPT_HUNGER_VALUE, WORK_FAILURE_HUNGER_VALUE,
};
use tracewake_core::agent::NeedBand;
use tracewake_core::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, ChecksumContext,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventEnvelope, EventKind, EventStream};
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

#[derive(Default)]
struct TerminalCounts {
    sleep_completed: usize,
    sleep_interrupted: usize,
    work_completed: usize,
    work_failed: usize,
    continuity_failure: usize,
}

impl TerminalCounts {
    fn record(&mut self, event: &EventEnvelope) {
        match event.event_type {
            EventKind::SleepCompleted => self.sleep_completed += 1,
            EventKind::SleepInterrupted => {
                self.sleep_interrupted += 1;
                if matches!(
                    payload_value(event, "reason"),
                    Some("actor_displaced" | "sleep_affordance_closed")
                ) {
                    self.continuity_failure += 1;
                }
            }
            EventKind::WorkBlockCompleted => self.work_completed += 1,
            EventKind::WorkBlockFailed => {
                self.work_failed += 1;
                if matches!(
                    payload_value(event, "reason"),
                    Some("actor_displaced" | "workplace_unusable")
                ) {
                    self.continuity_failure += 1;
                }
            }
            _ => {}
        }
    }
}

#[test]
fn generated_sequences_replay_and_satisfy_metamorphic_locks() {
    assert_eq!(
        NeedBand::for_value(SLEEP_INTERRUPT_HUNGER_VALUE as u16),
        NeedBand::Severe,
        "sleep-interrupt generator constant must stay in the Severe hunger band"
    );
    assert_eq!(
        NeedBand::for_value(WORK_FAILURE_HUNGER_VALUE as u16),
        NeedBand::Severe,
        "work-failure generator constant must stay in the Severe hunger band"
    );

    let mut reachability = Reachability::default();
    let mut masks = BTreeSet::new();
    let mut sequence_lengths = BTreeSet::new();
    let mut terminal_kinds = BTreeSet::new();
    let mut terminal_counts = TerminalCounts::default();
    let mut terminal_tamper_cases = 0_usize;
    let mut seed_contributors: BTreeMap<&'static str, BTreeSet<u64>> = BTreeMap::new();
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
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "actor_waited",
            has_event(&run.log, EventKind::ActorWaited),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "need_delta",
            has_event(&run.log, EventKind::NeedDeltaApplied),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "food_consumed",
            has_event(&run.log, EventKind::FoodConsumed),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "sleep_started",
            has_event(&run.log, EventKind::SleepStarted),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "work_started",
            has_event(&run.log, EventKind::WorkBlockStarted),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "sleep_completed",
            has_event(&run.log, EventKind::SleepCompleted),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "sleep_interrupted",
            has_event(&run.log, EventKind::SleepInterrupted),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "work_completed",
            has_event(&run.log, EventKind::WorkBlockCompleted),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "work_failed",
            has_event(&run.log, EventKind::WorkBlockFailed),
        );
        record_seed_contribution(
            &mut seed_contributors,
            *seed,
            "continuity_failure",
            has_continuity_failure_terminal(&run.log),
        );
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
        for event in run.log.events() {
            terminal_counts.record(event);
        }
        reachability.no_human_marker |= has_event(&run.log, EventKind::NoHumanAdvanceStarted)
            && has_event(&run.log, EventKind::NoHumanAdvanceCompleted);

        assert_live_matches_replay(&run, *seed);
        assert_payload_tamper_poisons_replay(&run, *seed);
        if duration_terminal_event_index(&run.log).is_ok() {
            assert_duration_terminal_payload_tamper_poisons_replay(&run, *seed);
            terminal_tamper_cases += 1;
        }
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
        terminal_counts.sleep_completed > 0,
        "generated corpus never emitted sleep-success terminal; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        terminal_counts.sleep_interrupted > 0,
        "generated corpus never emitted sleep-interrupt terminal; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        terminal_counts.work_completed > 0,
        "generated corpus never emitted work-success terminal; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        terminal_counts.work_failed > 0,
        "generated corpus never emitted work-fail terminal; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        terminal_tamper_cases > 0,
        "generated corpus never ran terminal-targeted tamper checks; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert!(
        terminal_counts.continuity_failure > 0,
        "generated corpus never emitted continuity-reason duration terminal; terminals={terminal_kinds:?}; {corpus_summary}"
    );
    assert_multi_seed_contributors(&seed_contributors, &corpus_summary);
    assert_derived_seed_contributors(&seed_contributors, &corpus_summary);
    assert!(
        masks.len() >= 4,
        "generated corpus mask diversity too low: {masks:?}; {corpus_summary}"
    );
    assert!(
        sequence_lengths.len() >= 2,
        "generated corpus sequence-length diversity too low: {sequence_lengths:?}; {corpus_summary}"
    );
}

#[test]
fn duration_terminal_targeted_tamper_requires_duration_terminal() {
    let empty = EventLog::new();
    assert!(
        duration_terminal_event_index(&empty).is_err(),
        "terminal-targeted tamper guard must fail loudly when no duration terminal exists"
    );
}

fn record_seed_contribution(
    contributors: &mut BTreeMap<&'static str, BTreeSet<u64>>,
    seed: u64,
    flag: &'static str,
    contributed: bool,
) {
    if contributed {
        contributors.entry(flag).or_default().insert(seed);
    }
}

fn assert_multi_seed_contributors(
    contributors: &BTreeMap<&'static str, BTreeSet<u64>>,
    corpus_summary: &str,
) {
    for flag in [
        "actor_waited",
        "need_delta",
        "food_consumed",
        "sleep_started",
        "work_started",
        "sleep_completed",
        "sleep_interrupted",
        "work_completed",
        "work_failed",
    ] {
        let seeds = contributors.get(flag).cloned().unwrap_or_default();
        assert!(
            seeds.len() >= 2,
            "generated corpus flag {flag} has too few contributing seeds: {seeds:x?}; {corpus_summary}"
        );
    }
}

fn assert_derived_seed_contributors(
    contributors: &BTreeMap<&'static str, BTreeSet<u64>>,
    corpus_summary: &str,
) {
    assert_derived_contributor_set(
        contributors,
        "sleep_interrupted",
        derived_seed_set(|case| case.mask.interrupt_sleep),
        corpus_summary,
    );
    assert_derived_contributor_set(
        contributors,
        "work_failed",
        derived_seed_set(|case| case.mask.work && !case.mask.eat && !case.mask.sleep),
        corpus_summary,
    );
    assert_derived_contributor_set(
        contributors,
        "continuity_failure",
        derived_seed_set(|case| case.mask.displace_during_work),
        corpus_summary,
    );
}

fn derived_seed_set(
    predicate: impl Fn(&support::generative::GeneratedCase) -> bool,
) -> BTreeSet<u64> {
    GENERATIVE_SEEDS
        .iter()
        .copied()
        .filter(|seed| predicate(&generate_case(*seed)))
        .collect()
}

fn assert_derived_contributor_set(
    contributors: &BTreeMap<&'static str, BTreeSet<u64>>,
    flag: &'static str,
    expected: BTreeSet<u64>,
    corpus_summary: &str,
) {
    let actual = contributors.get(flag).cloned().unwrap_or_default();
    assert_eq!(
        actual, expected,
        "generated corpus flag {flag} contributors no longer match the mask predicate: actual={actual:x?} expected={expected:x?}; {corpus_summary}"
    );

    let mut missing_one = actual.clone();
    if let Some(seed) = expected.first() {
        missing_one.remove(seed);
    }
    assert_ne!(
        missing_one, expected,
        "synthetic removal of a derived {flag} contributor must fail the contributor assertion"
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

fn assert_payload_tamper_poisons_replay(run: &GeneratedRun, seed: u64) {
    for event_index in 0..run.log.events().len() {
        let event = &run.log.events()[event_index];
        if event.payload.is_empty() {
            continue;
        }
        for payload_index in 0..event.payload.len() {
            let mut tampered_log = EventLog::new();
            for (candidate_index, mut candidate) in run.log.events().iter().cloned().enumerate() {
                if candidate_index == event_index {
                    candidate.payload[payload_index].value =
                        format!("tampered_{}", candidate.payload[payload_index].value);
                }
                tampered_log.append(candidate).unwrap();
            }

            let context = checksum_context(seed, &tampered_log, run.final_tick);
            let live_physical = compute_physical_checksum(&run.final_world, &context).checksum;
            let live_agent =
                compute_agent_state_checksum(&run.final_agent_state, &context).checksum;
            let replay = run_replay(
                &run.initial_world,
                &run.initial_agent_state,
                &tampered_log,
                &context,
                Some(&run.final_world),
                Some(live_physical),
                Some(live_agent),
            );
            if !replay.matches_expected || !replay.agent_checksum_matches {
                return;
            }
        }
    }
    panic!("seed={seed} had no payload perturbation that poisoned replay");
}

fn assert_duration_terminal_payload_tamper_poisons_replay(run: &GeneratedRun, seed: u64) {
    let event_index = duration_terminal_event_index(&run.log)
        .unwrap_or_else(|message| panic!("seed={seed} {message}"));
    let event = &run.log.events()[event_index];
    assert!(
        !event.payload.is_empty(),
        "seed={seed} duration terminal {} has no payload fields to tamper",
        event.event_type.stable_id()
    );
    for payload_index in 0..event.payload.len() {
        let tampered_log = tampered_log(&run.log, event_index, payload_index);
        assert!(
            replay_is_poisoned(run, &tampered_log, seed),
            "seed={seed} duration terminal {} payload field {} did not poison replay",
            event.event_type.stable_id(),
            event.payload[payload_index].key
        );
    }
}

fn duration_terminal_event_index(log: &EventLog) -> Result<usize, String> {
    log.events()
        .iter()
        .position(|event| {
            matches!(
                event.event_type,
                EventKind::SleepCompleted
                    | EventKind::SleepInterrupted
                    | EventKind::WorkBlockCompleted
                    | EventKind::WorkBlockFailed
            )
        })
        .ok_or_else(|| "duration-terminal tamper floor had no duration terminal event".to_string())
}

fn tampered_log(log: &EventLog, event_index: usize, payload_index: usize) -> EventLog {
    let mut tampered_log = EventLog::new();
    for (candidate_index, mut candidate) in log.events().iter().cloned().enumerate() {
        if candidate_index == event_index {
            candidate.payload[payload_index].value =
                format!("tampered_{}", candidate.payload[payload_index].value);
        }
        tampered_log.append(candidate).unwrap();
    }
    tampered_log
}

fn replay_is_poisoned(run: &GeneratedRun, tampered_log: &EventLog, seed: u64) -> bool {
    let context = checksum_context(seed, tampered_log, run.final_tick);
    let live_physical = compute_physical_checksum(&run.final_world, &context).checksum;
    let live_agent = compute_agent_state_checksum(&run.final_agent_state, &context).checksum;
    let replay = run_replay(
        &run.initial_world,
        &run.initial_agent_state,
        tampered_log,
        &context,
        Some(&run.final_world),
        Some(live_physical),
        Some(live_agent),
    );
    !replay.matches_expected || !replay.agent_checksum_matches
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

fn has_continuity_failure_terminal(log: &EventLog) -> bool {
    log.events().iter().any(|event| match event.event_type {
        EventKind::SleepInterrupted => matches!(
            payload_value(event, "reason"),
            Some("actor_displaced" | "sleep_affordance_closed")
        ),
        EventKind::WorkBlockFailed => matches!(
            payload_value(event, "reason"),
            Some("actor_displaced" | "workplace_unusable")
        ),
        _ => false,
    })
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
