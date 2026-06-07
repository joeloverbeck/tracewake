# 0006PHA3ANEEROU-004: `continue_routine` real continuation, not a marker

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` `continue_routine` action semantics (`actions/defs/continue_routine.rs`, `agent/methods.rs`, `agent/routine.rs`); `ContinueRoutineProposed` event handling
**Deps**: 0006PHA3ANEEROU-001

## Problem

`continue_routine` is a marker, not actual routine continuation (audit **D-06 / F-06**). The action validates proposal params then emits a `ContinueRoutineProposed` event (`crates/tracewake-core/src/actions/defs/continue_routine.rs:65`) with payload fields `next_action_id` and `routes_through_shared_pipeline="true"` (`continue_routine.rs:81-82`), but it does not resolve and execute the next ordinary action through the pipeline, nor transition routine step state live. A `ContinueRoutineProposed` event can make debug output look alive while the actor has not moved, eaten, worked, or slept. Spec §5.9 requires one of two acceptable approaches, and in either, `ContinueRoutineProposed` alone must not satisfy any acceptance criterion for eating, sleeping, movement, work, or routine progress.

## Assumption Reassessment (2026-06-07)

1. `continue_routine.rs` confirmed: `parse_next_action_id` (`continue_routine.rs:118`) and `EventKind::ContinueRoutineProposed` emission with `routes_through_shared_pipeline="true"` (`continue_routine.rs:65,82`). No subsequent ordinary-action resolution exists in the action body.
2. Spec §5.9 offers (1) make `continue_routine` an internal/meta action that immediately resolves and submits the next ordinary action proposal through the shared pipeline, or (2) keep it as a diagnostic/event marker but require behavioral tests to assert the next ordinary action event ancestry before counting continuation successful. Spec §9 forbids making `ContinueRoutineProposed` count as eat/work/sleep/move.
3. Shared boundary under audit: the `continue_routine` action ↔ routine-step state ↔ the next ordinary action submitted through the shared pipeline (`run_pipeline`, made live-applying by 0006PHA3ANEEROU-001). The decision loop (0006PHA3ANEEROU-007) is the eventual caller; this ticket fixes the action/event semantics it relies on.
4. Motivating invariants (restated): INV-035 "Routines are defeasible intentions" and INV-036 "HTN methods are procedures, not story scripts" — routine continuation must be embodied ordinary action through the pipeline, not a bypass marker.
5. Replay surface touched: the events this action emits must replay into the same routine-step/action ancestry live and on rebuild (consistent with 0006PHA3ANEEROU-001's unified application). Under approach 1 the follow-on ordinary action emits its own real event ancestry; the test gate asserts that ancestry, not the marker.
6. Schema note: if approach 1 narrows or removes the `ContinueRoutineProposed` payload (or replaces it with the resolved action's own events), that is an event-payload change. Consumers: `crates/tracewake-core/src/debug_reports.rs`, the trace projection, and tests asserting continuation. Any retained marker becomes non-load-bearing (cannot satisfy behavioral acceptance) — additive in spirit, not breaking to persisted history beyond the payload's own meaning.

## Architecture Check

1. Approach 1 (meta-action that resolves + submits the next ordinary action) is recommended: it makes routine continuation produce real ordinary-action event ancestry by construction, so no test can be fooled by a marker. Approach 2 is acceptable but pushes the guarantee entirely into test discipline; the implementer may choose, but the acceptance criteria below bind either way.
2. No backwards-compatibility aliasing/shims: a residual `ContinueRoutineProposed` marker, if kept, must not be readable as routine progress anywhere; no dual "marker counts / action counts" path remains.

## Verification Layers

1. INV-035 / INV-036 (embodied defeasible routines) -> replay/golden-fixture check: a routine continuation produces subsequent ordinary action event ancestry (move/eat/work/sleep), not only `ContinueRoutineProposed`.
2. Anti-marker guarantee -> codebase grep-proof + test: no acceptance/metric path treats `ContinueRoutineProposed` alone as eat/work/sleep/move/routine-progress.
3. Routine step state -> manual review + test: routine step/status transitions live after continuation, consistent under replay.

## What to Change

### 1. Real continuation semantics

Implement §5.9 approach 1 (recommended): `continue_routine` resolves the next routine step to an ordinary action proposal and submits it through the shared pipeline (0006PHA3ANEEROU-001), transitioning routine step state from the resulting events — or implement approach 2 with the binding test discipline below.

### 2. Demote the marker

Ensure `ContinueRoutineProposed` (if retained) cannot satisfy behavioral acceptance: routine progress is counted only from the subsequent ordinary action's event ancestry.

### 3. Routine step transition

Advance routine step/status from the continuation's resulting events via the unified application path.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify)
- `crates/tracewake-core/src/agent/methods.rs` (modify)
- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/debug_reports.rs` (modify)

## Out of Scope

- The decision loop that issues continuation per window (0006PHA3ANEEROU-007).
- Routine assignment instantiation / lifecycle (0006PHA3ANEEROU-006).
- Live event application mechanics (0006PHA3ANEEROU-001, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. A routine-continuation test asserts a subsequent ordinary action event (e.g. `FoodConsumed`/`SleepCompleted`/movement/`WorkBlock*`) follows continuation; the marker alone fails the assertion.
2. A test confirms no metric/acceptance path counts `ContinueRoutineProposed` as eat/work/sleep/move/routine-progress.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Routine continuation always yields ordinary action event ancestry (or a typed failure), never a standalone success marker.
2. Routine step state transitions are event-derived and replay-consistent.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/continue_routine.rs` — unit test: continuation submits and applies the next ordinary action; marker-only does not count.
2. `crates/tracewake-core/tests/acceptance_gates.rs` — assert subsequent ordinary action ancestry after continuation.

### Commands

1. `cargo test -p tracewake-core continue_routine`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Implemented the spec-allowed marker-discipline path for `continue_routine`:
`ContinueRoutineProposed` is now explicitly marked
`behavioral_progress=false`, and no metrics/acceptance path treats it as
routine progress or ordinary movement/eat/work/sleep ancestry.

Added continuation tests that prove marker-only continuation does not move an
actor, then a follow-on ordinary `move` proposal through the shared pipeline is
the event ancestry that counts. Added projection and acceptance-gate coverage so
marker-only logs report zero routine/meal/sleep/work progress. Updated the
no-human fixture metrics assertion to derive routine progress from
event-derived routine step/accepted/rejected events only.

Verified with:

1. `cargo test -p tracewake-core continue_routine`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo fmt --all --check`
