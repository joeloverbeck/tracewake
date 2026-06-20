# 0042ORDLIFCER-002: ORD-LIFE-01 — bounded event-sourced needs, single-owner accounting, and single-charge ledgers

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-01 must be proven against the target commit: needs are bounded pressures visible through actor-known state that influence candidate families but never identify a true target, and every passive or action-emitted need delta, threshold crossing, duration tick, work completion/failure charge, wait charge, and duration terminal is emitted by the single action-pipeline/ordinary-life owning seam and covers each actor/need/tick exactly once, with all downstream consumers reading the event-backed ledger without independently charging or reconciling it. This ticket records the ORD-LIFE-01 behavior witnesses, single-charge ledger proofs, adversarial negatives, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/need.rs` (`NeedKind`, `NeedBand`, `NeedState`, `NeedThresholdCrossing`, `NeedChangeCause`, `NeedPressure`), `crates/tracewake-core/src/need_accounting.rs` (`classify_actor_tick_regimes_with_start`, `open_body_exclusive_starts`, `is_duration_terminal`, `DuplicateDurationTerminal`, `DurationInterval`), `crates/tracewake-core/src/actions/defs/need_events.rs` / `scheduler.rs` (`build_need_delta_and_threshold_events`, `NeedDeltaEventSpec`), plus `actions/pipeline.rs`, `events/{envelope,apply}.rs`, `projections.rs`, `replay/rebuild.rs` (all confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-01 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics, coverage) and §6.1 pass conditions 1/8/9 govern this ticket; §4.4 cross-cutting proof rules ("event before derived state", "single owner, single charge") bind it. Note (spec §10): `scheduler.rs` participates in passive need-event construction while doctrine assigns a single accounting owner — the artifact must establish the exact ownership/delegation boundary and prove scheduler logic is not a second charger.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-01 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-01): `INV-009` (meaningful changes are eventful with a cause), `INV-045` and `INV-091`/`INV-092` (single-owner / single-charge accounting), `INV-015` (no silent starvation/double-charge), `INV-001`/`INV-039`, `INV-018` (replay reconstructs the ledger). Restate before trusting the narrative: one owner, one charge per actor/need/tick, all derived state event-backed or replay-reconstructable.
5. Evidence-consumer surface (audit-reads, does not modify): the need-accounting owning seam and the deterministic-replay path. This ticket only runs fixtures and records the canonical ledger; it adds no production charge path, introduces no nondeterminism, and any test-only instrumentation stays observer-only and out of the charging path.

## Architecture Check

1. Proving single-charge by independently expanding event payloads into a canonical actor/need/tick ledger (rather than trusting golden bytes) is the only check that catches off-by-one, omission, double-count, and wrong-cause drift — spec §5 explicitly states stable golden bytes without semantic ledger equality do not satisfy this point.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No compatibility shim may preserve an accounting bypass.

## Verification Layers

1. `INV-009` event-before-derived-state -> replay/golden-fixture check (each need delta/threshold/terminal is event-backed; clean replay reconstructs need state, thresholds, intervals, duplicate-terminal findings, totals).
2. `INV-091`/`INV-092` single-owner/single-charge -> codebase grep-proof + manual review (canonical ledger shows one emitter/event ID per actor/need/tick; scheduler/planner/projection/replay/golden-normalization consume but do not charge).
3. `INV-015` no silent double-charge/starvation -> replay/golden-fixture check (adversarial overlapping-coverage and duplicate-terminal fixtures fail at the responsible layer with a typed negative).

## What to Change

### 1. Record positive single-charge and bounded-pressure witnesses

Run the §5 positive fixtures (`sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001`, and the integrated no-human corpus). For every actor/need/tick, expand event payloads into a canonical ledger and record emitter + event ID, duration start/terminal ancestry, tick-regime classification (awake/asleep/working/modeled-wait/interruption/completion/failure, including windows opening with an already-open body-exclusive duration), pre/post need values, threshold events, and live/replay equality. Establish and record the exact scheduler↔pipeline accounting ownership boundary (spec §10 risk).

### 2. Record adversarial negatives and responsible-layer attribution

Record the §5 adversarial cases: overlapping passive-window + action-emitted coverage for the same actor/need/tick (owning seam rejects/deduplicates before accepted state change); duplicate/conflicting duration terminal (incl. differing proposal ID, same closing cause) caught by the typed duplicate-terminal path; and a negative control proving scheduler/planner/projection/replay/golden-normalization cannot synthesize compensating deltas. Each negative names the canonical responsible layer (spec §8 vocabulary).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-01 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any accounting defect (spec §2/§8 route a substantive failed seam to a later separately-numbered ORD-LIFE-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Candidate generation / hidden-target exclusion (`-003`), intention lifecycle (`-004`), no-human metric honesty (`-010`), and the §7 mutation perimeter (`-015`).
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test generative_lock` pass; the captured single-charge ledger shows exactly one owner and one charge per actor/need/tick across all positive fixtures.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes and clean replay reconstructs identical need state, threshold crossings, duration intervals, duplicate-terminal findings, and no-human accounting totals.
3. `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; recorded adversarial negatives (overlapping coverage, duplicate terminal) fail at their typed responsible layer with expected/actual typed values.

### Invariants

1. Single owner, single charge: no scheduler/planner/projection/replay/golden-normalization path independently charges, reconciles, synthesizes, or normalizes a covered tick/window.
2. Event-before-derived-state: every need delta, threshold crossing, and duration terminal is event-backed or replay-reconstructable; semantic ledger equality, not raw golden bytes, is the certifying surface.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only and out of the charging path.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
