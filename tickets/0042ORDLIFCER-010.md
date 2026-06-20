# 0042ORDLIFCER-010: ORD-LIFE-09 — no-human orchestration, canonical recovery, meaningful progress, and metric honesty

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-09 must be proven: the no-human runner must advance multiple ordinary actors through the same sealed actor-known transaction, proposal, validation, event, and replay paths available to a possessed actor; start/end markers and metrics must derive from accepted events; behavioral progress is limited to committed ordinary actions, duration starts/completions/interruptions, modeled waits with typed reasons, and typed stuck/failure outcomes; and `continue_routine` alone is never progress. This ticket records the per-actor/window transaction bundles, independent progress classification, canonical recovery witness, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/scheduler.rs` (`run_no_human_day`, `advance_no_human`, `NoHumanDayConfig`, `NoHumanDayReport`, `DayWindow`, `NoHumanAdvanceReport`, `SchedulePhase`, `latest_action_tick_delta_tick`), `crates/tracewake-core/src/projections.rs` (`no_human_day_metrics`, `NoHumanDayMetrics`), `agent/no_human_surface.rs`, `agent/transaction.rs`, `agent/decision.rs`, `actions/pipeline.rs`, `events/{envelope,log}.rs`, `replay/report.rs` (confirmed in the 0042 reassessment census; `NoHumanDayMetrics` resolves in `debug_reports.rs`/`projections.rs`).
2. Spec §5 ORD-LIFE-09 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 6/8/9/10 govern this ticket; §4.4 "meaningful progress only" + "no silent liveness assumption" bind it. The canonical `no_human_day_001` outcome is `fail_only_empty_food_source` (spec §5/§11): Mara's empty home source yields a typed ordinary-life failure without consuming or targeting Tomas's food or hidden fallback food.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-09 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section. The four permitted progress categories form a closed classification the metric must match exactly.
4. Motivating invariants (spec §5 ORD-LIFE-09): `INV-111` (living-world acceptance requires observer-only emergence evidence; `EMERGE-OBS` non-gating), `INV-003` (coherent no-human run), `INV-039`–`INV-045` (ordinary-life accounting/actions), `INV-091`/`INV-092`, `INV-103`/`INV-104`. Restate before trusting the narrative: metrics derive from accepted events and count only the four permitted progress kinds; `continue_routine`/scheduler/debug markers are excluded.
5. Evidence-consumer surface (audit-reads, does not modify): the no-human transaction path and the metric projection/replay. This ticket runs the no-human corpus and independently classifies the event stream; it adds no metric-writing path, introduces no nondeterminism, and any instrumentation stays observer-only — `EMERGE-OBS` summaries remain non-gating.

## Architecture Check

1. Independently classifying the accepted event stream into the four permitted progress categories and proving `NoHumanDayMetrics` exactly matches that classification (with Goodharting controls: many waits/failures/traces without legitimate typed reason cannot raise a progress count) is the decisive metric-honesty proof — stronger than reading the metric value.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-003` no-human coherence -> replay/golden-fixture check (`no_human_day_001`, `no_human_advance_001`, `sleep_eat_work_001`, `ordinary_workday_001`: multiple actors progress without human commands through ordered day windows and ordinary transactions).
2. `INV-091`/`INV-092` metric honesty -> codebase grep-proof + manual review (independent event classification exactly matches `NoHumanDayMetrics`; `continue_routine`/scheduler-awakening/trace/debug/projection-rebuild/idle markers excluded; metrics cite event IDs + responsible layers for stuck/failure).
3. `INV-018`/`INV-111` replay + observer-only emergence -> replay/golden-fixture check (clean replay rebuilds metrics, markers, and reports; `EMERGE-OBS` summary is observer-only and never a gate/threshold/objective).

## What to Change

### 1. Record no-human progress and canonical-recovery witnesses

Run `no_human_day_001`, `no_human_advance_001`, `sleep_eat_work_001`, `ordinary_workday_001`; prove multiple actors progress without human commands through ordered day windows and ordinary transactions. For every actor/window record start/end markers, actor-known packet, candidate/rejection list, selected/rejected method, proposal ancestry, validation, ordinary action/wait/failure, need/intention changes, and replay report. Independently classify the accepted event stream into the four permitted progress categories and prove `NoHumanDayMetrics` exactly matches. Record the canonical `no_human_day_001` outcome `fail_only_empty_food_source`: Mara's empty home source yields a typed ordinary-life failure without consuming or targeting Tomas's food or hidden fallback food.

### 2. Record adversarial metric-honesty and Goodharting negatives

Record the §5 adversarial cases: windows containing only `continue_routine` / scheduler awakenings / trace records / debug records / projection-rebuild markers / repeated unreasoned idle (metrics count none as progress); changing hidden fallback food or another actor's resources without changing the focal actor's knowledge (no action/metric implies hidden recovery); removed/duplicated start/end markers, mismatched actor/window IDs, metrics computed before accepted append/replay (report fails loudly); and a fixture emitting many waits/failures/traces without legitimate typed reason (cannot pass by raising a progress count).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-09 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- The success-recovery variant / public food services / neighbor-pantry access (staged to Phase 3B per spec §11; this deferral cannot authorize hidden fallback food now).
- Production remediation of any orchestration/metric defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Stuck-diagnostic taxonomy certification (`-011`), replay-derivation lock (`-013`), and the aggregate verdict / fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test emergence_ledger` pass; multiple actors advance without human input and the `no_human_day_001` canonical outcome remains `fail_only_empty_food_source` (no hidden fallback food consumed/targeted).
2. `cargo test --locked -p tracewake-core --test golden_scenarios` passes; an independent classification of the accepted event stream into the four permitted progress categories exactly matches `NoHumanDayMetrics`, with `continue_routine`/scheduler/debug/idle markers excluded.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; clean replay rebuilds metrics/markers/reports and removed/duplicated markers or pre-append metric computation fail loudly.

### Invariants

1. Meaningful progress only: metrics count only committed ordinary actions, duration starts/completions/interruptions, modeled waits with typed reasons, and typed stuck/failure; `continue_routine` alone is never progress.
2. No hidden recovery: changing hidden fallback food / another actor's resources without changing the focal actor's knowledge produces no action or metric implying recovery; `EMERGE-OBS` stays observer-only.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test emergence_ledger`
3. `cargo test --locked -p tracewake-core --test golden_scenarios`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
