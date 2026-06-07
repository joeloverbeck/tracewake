# 0007PHA3ASECHAR-008: continue_routine non-progress discipline

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` continue-routine action (`actions/defs/continue_routine.rs`), no-human progress metric (`scheduler.rs`, `projections.rs`)
**Deps**: 0007PHA3ASECHAR-004, 0007PHA3ASECHAR-007

## Problem

A `ContinueRoutineProposed` event with `behavioral_progress=false` is not behavioral continuation, yet it must be prevented from satisfying Phase 3A progress gates by itself (Spec 0007 D-06, §Action definitions, §Forbidden shortcuts). At the audited commit `continue_routine` emits `ContinueRoutineProposed` with `PayloadField::new("behavioral_progress", "false")` (`crates/tracewake-core/src/actions/defs/continue_routine.rs:65,84`). Continuing a routine must either produce an ordinary follow-on action in the same scheduler transaction or be explicitly treated as a non-progress diagnostic marker that never counts toward no-human progress.

## Assumption Reassessment (2026-06-07)

1. Confirmed: `continue_routine` emits `ContinueRoutineProposed` with `behavioral_progress=false` (`crates/tracewake-core/src/actions/defs/continue_routine.rs:65,84`). A guard test already asserts the marker alone is not progress: `continue_routine_marker_alone_counts_as_no_behavioral_progress` (`crates/tracewake-core/src/projections.rs:1509`) and `continue_routine_marker_alone_is_not_behavioral_progress` (`crates/tracewake-core/tests/acceptance_gates.rs:565`). The follow-on accept/reject kinds `ContinueRoutineAccepted`/`ContinueRoutineRejected` exist (`events/envelope.rs`).
2. Spec 0007 D-06 requires `continue_routine` produce an ordinary follow-on action in the same scheduler transaction OR be a non-progress diagnostic marker that never satisfies progress gates; §Forbidden shortcuts bars "`ContinueRoutineProposed` counted as routine progress without a follow-on ordinary action or typed failure/replan". §Continue-routine ancestry tests require a successful continuation transaction to include the follow-on ordinary action or typed failure/replan, with replay reconstructing the same progress.
3. Shared boundary under audit: the no-human progress metric (`ordinary_pipeline_events` / behavioral-progress counting in `scheduler.rs` `NoHumanDayReport` and the projection in `projections.rs`) between the action emitter and the progress gate. The gate must read `behavioral_progress` / the presence of a follow-on ordinary action, never credit the bare marker.
4. Motivating invariants (restated): INV-015 "Failure is eventful when consequential" — a non-progress continuation is a real diagnostic event, but it is not progress; INV-001 "Causality comes before drama" — progress must reflect a modeled ordinary action, not a marker.
5. Deterministic-replay surface touched: the no-human progress metric is replay-derived. The progress count must be computed from the event log (follow-on ordinary action or typed failure/replan ancestry), not from side-channel scheduler state, so replay reconstructs the same intention/routine progress. No actor-knowledge leakage involved.

## Architecture Check

1. Binding the continuation's progress credit to a same-transaction ordinary action (or a typed failure/replan) makes "routine continued" causal and replay-derivable, and keeps the marker as an honest non-progress diagnostic rather than a smuggled progress signal. This closes the gap the existing guard tests only partially cover (marker-alone) by also asserting the positive follow-on path.
2. No backwards-compatibility aliasing/shims: no path credits `behavioral_progress=false` as progress.

## Verification Layers

1. INV-015 (eventful non-progress) -> codebase grep-proof + unit test: `ContinueRoutineProposed` with `behavioral_progress=false` never increments the no-human progress metric.
2. Follow-on causality -> replay/golden-fixture check: a successful continuation transaction includes the follow-on ordinary action or a typed failure/replan, and replay reconstructs the same routine/intention progress.

## What to Change

### 1. Same-transaction follow-on or explicit non-progress

In the no-human loop, a continue-routine decision either submits an ordinary follow-on action in the same scheduler transaction (crediting progress via that action) or records `ContinueRoutineProposed` as a non-progress diagnostic; emit `ContinueRoutineAccepted`/`ContinueRoutineRejected` accordingly.

### 2. Progress gate ignores the bare marker

Ensure the no-human progress metric (`scheduler.rs` `NoHumanDayReport` / `projections.rs`) counts progress only from an ordinary action or typed failure/replan ancestry, never from `behavioral_progress=false`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)

## Out of Scope

- Routine step ancestry and interruption (0007PHA3ASECHAR-007, dependency).
- The integrated capstone assertion of the full chain (0007PHA3ASECHAR-012).

## Acceptance Criteria

### Tests That Must Pass

1. A unit test: `ContinueRoutineProposed` with `behavioral_progress=false` alone does not satisfy the no-human progress metric (extends the existing marker-alone guards).
2. A test: a successful routine continuation transaction includes the follow-on ordinary action or a typed failure/replan event.
3. Replay reconstructs the same intention/routine progress from the event chain.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. The bare continue-routine marker never counts as progress.
2. Progress is computed from the event log, not side-channel scheduler state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` — extend marker-alone guard with the positive follow-on case.
2. `crates/tracewake-core/tests/acceptance_gates.rs` — continuation transaction includes follow-on ordinary action / typed failure.

### Commands

1. `cargo test -p tracewake-core projections`
2. `cargo test -p tracewake-core --test acceptance_gates`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

Changed `scheduler.rs` so no-human `ordinary_pipeline_events` is derived from the pipeline result's appended event log entries through an explicit progress predicate. `ContinueRoutineProposed` only counts if its payload carries `behavioral_progress=true`; the existing `behavioral_progress=false` continuation marker no longer satisfies the no-human progress report or stuck-progress bookkeeping by itself.

Extended tests with a scheduler regression proving marker-only `continue_routine` through `advance_no_human` reports zero ordinary progress, plus a projection replay regression proving marker + follow-on routine/ordinary events reconstructs the same progress metrics from serialized event-chain replay.

Deviation from plan: `actions/defs/continue_routine.rs` already emitted `behavioral_progress=false` and already had follow-on ordinary-action ancestry coverage, so no production change was needed there. The implementation focused on the no-human progress gate and projection replay proof.

Verification:

1. `cargo test -p tracewake-core continue_routine_marker_only_is_not_ordinary_progress`
2. `cargo test -p tracewake-core continue_routine_follow_on_progress_survives_replay_from_event_chain`
3. `cargo test -p tracewake-core --test acceptance_gates continue_routine_marker_alone_is_not_behavioral_progress`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
