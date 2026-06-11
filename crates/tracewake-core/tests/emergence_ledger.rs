use std::collections::{BTreeMap, BTreeSet};

mod support;

use support::generative::{
    content_manifest_id, generate_case, initial_agent_state, initial_world, no_human_state_mut,
    registry, scheduled_proposals, GENERATIVE_SEEDS,
};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventEnvelope, EventKind};
use tracewake_core::scheduler::no_human::advance_no_human;
use tracewake_core::time::SimTick;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct EmergeObsLedger {
    rows: Vec<EmergeObsRow>,
    totals: EmergeObsCounters,
}

impl EmergeObsLedger {
    fn from_rows(rows: Vec<EmergeObsRow>) -> Self {
        let mut totals = EmergeObsCounters::default();
        for row in &rows {
            totals.add(&row.counters);
        }
        Self { rows, totals }
    }

    fn serialize_canonical(&self) -> String {
        let rows = self
            .rows
            .iter()
            .map(EmergeObsRow::serialize_canonical)
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "emerge_obs_v1|rows={}\n{}\ntotal|{}",
            self.rows.len(),
            rows,
            self.totals.serialize_canonical()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EmergeObsRow {
    corpus_id: String,
    start_tick: SimTick,
    end_tick: SimTick,
    counters: EmergeObsCounters,
}

impl EmergeObsRow {
    fn serialize_canonical(&self) -> String {
        format!(
            "row|corpus_id={}|start_tick={}|end_tick={}|{}",
            self.corpus_id,
            self.start_tick.value(),
            self.end_tick.value(),
            self.counters.serialize_canonical()
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct EmergeObsCounters {
    expectation_contradictions_discovered: usize,
    replans_or_fallback_selections_taken: usize,
    interruptions: usize,
    intention_switches_with_recorded_causes: usize,
    stuck_diagnostics_by_blocker_category: BTreeMap<String, usize>,
    beliefs_diverged_from_truth: usize,
    diverged_beliefs_corrected_through_modeled_channels: usize,
    distinct_event_kinds_reached: BTreeSet<&'static str>,
}

impl EmergeObsCounters {
    fn add(&mut self, other: &Self) {
        self.expectation_contradictions_discovered += other.expectation_contradictions_discovered;
        self.replans_or_fallback_selections_taken += other.replans_or_fallback_selections_taken;
        self.interruptions += other.interruptions;
        self.intention_switches_with_recorded_causes +=
            other.intention_switches_with_recorded_causes;
        self.beliefs_diverged_from_truth += other.beliefs_diverged_from_truth;
        self.diverged_beliefs_corrected_through_modeled_channels +=
            other.diverged_beliefs_corrected_through_modeled_channels;
        for (category, count) in &other.stuck_diagnostics_by_blocker_category {
            *self
                .stuck_diagnostics_by_blocker_category
                .entry(category.clone())
                .or_default() += count;
        }
        self.distinct_event_kinds_reached
            .extend(other.distinct_event_kinds_reached.iter().copied());
    }

    fn serialize_canonical(&self) -> String {
        format!(
            "expectation_contradictions={}|replans_or_fallbacks={}|interruptions={}|intention_switches_with_causes={}|stuck_by_blocker={}|belief_truth_divergences={}|modeled_channel_corrections={}|distinct_event_kinds={}",
            self.expectation_contradictions_discovered,
            self.replans_or_fallback_selections_taken,
            self.interruptions,
            self.intention_switches_with_recorded_causes,
            serialize_counts(&self.stuck_diagnostics_by_blocker_category),
            self.beliefs_diverged_from_truth,
            self.diverged_beliefs_corrected_through_modeled_channels,
            self.distinct_event_kinds_reached.len()
        )
    }
}

#[test]
fn emerge_obs_ledger_is_observer_only_and_replay_byte_identical() {
    let mut rows = Vec::new();
    for seed in GENERATIVE_SEEDS {
        let case = generate_case(*seed);
        let run = run_generated_no_human_case(&case);
        let row = derive_emerge_obs_row(format!("generative_seed_{seed:x}"), &run.log);
        let rederived = derive_emerge_obs_row(format!("generative_seed_{seed:x}"), &run.log);
        assert_eq!(row, rederived, "seed={seed:x}");

        let replayed_log = EventLog::deserialize_canonical(&run.log.serialize_canonical())
            .expect("canonical event log should deserialize");
        let replayed_row =
            derive_emerge_obs_row(format!("generative_seed_{seed:x}"), &replayed_log);
        assert_eq!(
            row.serialize_canonical(),
            replayed_row.serialize_canonical(),
            "seed={seed:x}"
        );
        rows.push(row);
    }

    let ledger = EmergeObsLedger::from_rows(rows.clone());
    let rederived_ledger = EmergeObsLedger::from_rows(rows);
    assert_eq!(
        ledger.serialize_canonical(),
        rederived_ledger.serialize_canonical()
    );
    assert_eq!(ledger.rows.len(), GENERATIVE_SEEDS.len());
    println!("{}", ledger.serialize_canonical());
}

fn run_generated_no_human_case(case: &support::generative::GeneratedCase) -> GeneratedRun {
    let mut world = initial_world(case.seed);
    let mut agent_state = initial_agent_state(case.seed);
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
    GeneratedRun { log }
}

struct GeneratedRun {
    log: EventLog,
}

fn derive_emerge_obs_row(corpus_id: String, log: &EventLog) -> EmergeObsRow {
    let events = log.events();
    let start_tick = events
        .first()
        .map(|event| event.sim_tick)
        .unwrap_or(SimTick::ZERO);
    let end_tick = events
        .last()
        .map(|event| event.sim_tick)
        .unwrap_or(SimTick::ZERO);
    let mut counters = EmergeObsCounters::default();
    for event in events {
        counters
            .distinct_event_kinds_reached
            .insert(event.event_type.stable_id());
        if matches!(
            event.event_type,
            EventKind::SleepInterrupted
                | EventKind::WorkBlockFailed
                | EventKind::IntentionInterrupted
        ) {
            counters.interruptions += 1;
        }
        match event.event_type {
            EventKind::ExpectationContradicted => {
                counters.expectation_contradictions_discovered += 1;
                counters.beliefs_diverged_from_truth += 1;
            }
            EventKind::ContinueRoutineAccepted
            | EventKind::ContinueRoutineProposed
            | EventKind::ContinueRoutineRejected
            | EventKind::RoutineStepFailed => {
                counters.replans_or_fallback_selections_taken += 1;
            }
            EventKind::IntentionStarted
            | EventKind::IntentionContinued
            | EventKind::IntentionSuspended
            | EventKind::IntentionResumed
            | EventKind::IntentionCompleted
            | EventKind::IntentionFailed
            | EventKind::IntentionAbandoned
            | EventKind::IntentionInterrupted => {
                if !event.causes.is_empty() {
                    counters.intention_switches_with_recorded_causes += 1;
                }
            }
            EventKind::StuckDiagnosticRecorded => {
                let blocker = payload_value(event, "blocker_code").unwrap_or("uncategorized");
                *counters
                    .stuck_diagnostics_by_blocker_category
                    .entry(blocker.to_string())
                    .or_default() += 1;
            }
            EventKind::BeliefUpdated => {
                counters.diverged_beliefs_corrected_through_modeled_channels += 1;
            }
            _ => {}
        }
    }
    EmergeObsRow {
        corpus_id,
        start_tick,
        end_tick,
        counters,
    }
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn serialize_counts(counts: &BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }
    counts
        .iter()
        .map(|(key, count)| format!("{key}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}
