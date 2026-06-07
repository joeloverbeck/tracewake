# 0005PHA3ANEEROU-018: No-human day metrics artifact

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds a deterministic, replayable no-human day metrics/observability artifact derived from events/projections.
**Deps**: 0005PHA3ANEEROU-017

## Problem

Phase 3A must produce a no-human day metrics/debug artifact derived from event/projection state — not from unlogged side effects — covering events/day, routine events, meals completed/missed, sleep completed/interrupted, work blocks completed/failed, need threshold crossings, routine interruptions, planner failures, stuck actors, run duration, replay failures, TUI action coverage, and player-conditioned event rate (Spec 0005 §19). For the canonical day, zero routine events / need crossings / meals / progress is failure — so the metrics are also an acceptance signal.

## Assumption Reassessment (2026-06-07)

1. The no-human runner (ticket 017) emits the day-start/summary markers and drives the events these metrics count; the agent-state projection (ticket 006) and `crates/tracewake-core/src/projections.rs` / `debug_reports.rs` are the derivation surfaces. Phase 2A precedent: debug reports are deterministic projections over the log, not runtime counters.
2. Spec §19 fixes the required metrics list and the rule: metrics must be derived from event/projection state, not unlogged side effects; a zero value is acceptable only where a fixture intentionally proves absence; for the canonical no-human day, zero routine events / need crossings / meals / progress is failure.
3. Shared boundary under audit: the metrics artifact is consumed by the debug surface (ticket 023, `debug no-human-day`), the canonical fixture's expected-metrics envelope (ticket 021), and the capstone (ticket 025). Its field set and derivation (event-derived) are fixed here.
4. Invariant motivating this ticket: `INV-013` — "Meaningful events leave traces" (and the §19 rule that metrics derive from events, not side effects). Counting from the event log/projection keeps metrics replay-stable; a runtime counter would diverge from replay.
5. Deterministic-replay surface: metrics are a pure function of the event log/projection, so identical logs yield identical metrics (`INV-018`). This ticket adds no nondeterminism and no leakage — the artifact aggregates already-logged events and carries no hidden truth (the player-conditioned-event-rate metric is a count over emitted events proving it is zero for no-human runs).

## Architecture Check

1. Deriving metrics from the event log/agent-state projection (rather than incrementing runtime counters during the run) makes the metrics replay-identical and impossible to desync from the actual events (`INV-013`, §19) — a side-effect counter would silently drift under replay. Reusing the debug-report projection style keeps observability non-authoritative.
2. No backwards-compatibility shims: a new derived artifact over existing projections; no runtime mutable counter state.

## Verification Layers

1. Event-derived metrics (`INV-013`; §19) -> unit test: every metric equals a re-count over the event log/projection; no metric reads unlogged runtime state.
2. Replay stability (`INV-018`) -> replay/golden check: replaying a no-human day yields identical metrics to the original run.
3. Acceptance signal (§19) -> unit test: for a populated no-human day, routine-events/meals/need-crossings are non-zero; a zero there fails (distinct from a fixture intentionally proving absence).

## What to Change

### 1. Metrics artifact

Add a deterministic metrics struct/projection (in `crates/tracewake-core/src/projections.rs` or `debug_reports.rs`) computing the §19 metrics from the event log/agent-state projection over a no-human day: events/day, routine events, meals completed/missed, sleep completed/interrupted, work completed/failed, need threshold crossings, routine interruptions, planner failures, stuck actors, run duration, replay failures, TUI action coverage, player-conditioned event rate.

### 2. Wiring to the day summary

Make the runner's no-human day-summary marker (ticket 017) reference/emit the metrics envelope so it is replay-reconstructible and consumable by the debug surface and capstone.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify — no-human day metrics projection)
- `crates/tracewake-core/src/debug_reports.rs` (modify — expose the metrics artifact for debug rendering, if placed here)

## Out of Scope

- The runner itself (ticket 017).
- Debug rendering of the metrics (`debug no-human-day`, ticket 023).
- The canonical fixture's expected-metrics envelope (ticket 021).
- The acceptance gate asserting the full metrics (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. Each metric equals an independent re-count over the event log/projection; no metric reads unlogged runtime state.
2. Replaying a no-human day yields byte-identical metrics.
3. For a populated no-human day, routine events / meals / need crossings are non-zero; the player-conditioned-event-rate metric is zero.

### Invariants

1. Metrics are derived from events/projections, never side effects (`INV-013`, §19).
2. Metrics are replay-stable (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (unit tests) — per-metric re-count equality and replay stability.

### Commands

1. `cargo test -p tracewake-core projections`
2. Core-crate scope is correct; the canonical-day metrics envelope assertion is tickets 021/025.
3. A re-count-equality test is the correct boundary because it proves derivation-from-events without needing the full day fixture.

## Outcome

Completed on 2026-06-07.

- Added `NoHumanDayMetrics` and `no_human_day_metrics` in `crates/tracewake-core/src/projections.rs`, deriving the Phase 3A metrics from `EventLog` events only.
- Added canonical serialization for replay-stable byte comparisons, including raw player-conditioned event count and a deterministic per-1000 rate.
- Made the no-human day completion marker reference `no_human_day_metrics_v1` for downstream debug/capstone consumers.
- Added projection tests for independent recount equality, canonical replay stability, and populated no-human activity with zero player-conditioned rate.

Verification run:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core projections`
3. `cargo test -p tracewake-core`
