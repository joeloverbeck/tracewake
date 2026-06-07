# 0007PHA3ASECHAR-007: Routine execution as defeasible procedures over ordinary actions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` routine execution ancestry and interruption (`agent/routine.rs`, `scheduler.rs`, `events/apply.rs`); routine fixtures
**Deps**: 0007PHA3ASECHAR-004, 0007PHA3ASECHAR-006

## Problem

Routines must be defeasible procedures whose step start/progress/failure/completion ancestry arises from the integrated no-human path and ties to ordinary action ancestry (Spec 0007 D-05, §Routines). The `RoutineStepStarted`/`RoutineStepCompleted`/`RoutineStepFailed` events and the `RoutineExecution` record exist, but routine progress must be tied to ordinary action ancestry (movement, sleep, eat, work, search, wait-with-reason, or typed failure) on `RoutineExecution.concrete_action_ancestry`, and routines must be interruptible by severe needs, blocked access, action failure, work-hour boundaries, safety concerns, or epistemic contradictions through typed diagnostics — not advance as bare markers.

## Assumption Reassessment (2026-06-07)

1. Confirmed: `RoutineExecution` (`crates/tracewake-core/src/agent/routine.rs:351`) carries `current_step_index`, `step_status: RoutineStepStatus`, `concrete_action_ancestry: Vec<SemanticActionId>`, `failure_interruption_reason`, `last_progress_tick`. The step events `RoutineStepStarted`/`RoutineStepCompleted`/`RoutineStepFailed` exist (`crates/tracewake-core/src/events/envelope.rs`). `RoutineStepStatus` is at `routine.rs:339`. 0007PHA3ASECHAR-002 adds the typed `family`; this ticket owns step ancestry + interruption.
2. Spec 0007 §Routines requires step start/completion/failure events emitted by the integrated no-human (and TUI) paths, routine progress tied to ordinary action ancestry, and interruption by severe needs / blocked access / action failure / work-hour boundary / safety / epistemic contradiction. D-05 requires these arise from the integrated scheduler path, not only be available to tests.
3. Shared boundary under audit: `RoutineExecution` between the no-human decision driver (which advances steps and records concrete action ancestry) and replay rebuild + debug/view-model surfaces (consumers). Routine progress must be backed by an ordinary action id on `concrete_action_ancestry` or a typed failure, never an empty advance.
4. Motivating invariants (restated): INV-035 "Routines are defeasible intentions" — a worker works only while they believe it is work time, can reach work, have access/tools, and lack stronger interruptions; schedules are not teleports; INV-036 "HTN methods are procedures, not story scripts" — routine steps fail, interrupt, and have alternatives and traces.
5. Deterministic-replay surface touched: routine step events are the replay source for `RoutineExecution`. Step transitions must be emitted deterministically and applied in `events/apply.rs` so live and replay-rebuilt routine state (step index, status, ancestry, interruption reason) match (agent-state checksum). No actor-knowledge leakage: routine locality reads actor-known/visible-local conditions (0007PHA3ASECHAR-004), not authoritative truth.

## Architecture Check

1. Tying each routine step transition to an ordinary action id (or a typed failure) makes routine progress causal and replayable — a routine cannot "advance" without a backing action — and turns interruption into inspectable typed diagnostics, satisfying defeasibility (INV-035) and the no-teleport rule. Reusing the existing step events and the `concrete_action_ancestry` field avoids new schema.
2. No backwards-compatibility aliasing/shims: routine progress is not credited from a marker; only an ordinary action or typed failure advances or fails a step.

## Verification Layers

1. INV-035 (defeasible, no teleport) -> replay/golden-fixture check: `routine_no_teleport_001` proves a routine step advances only with movement/ordinary-action ancestry, never by teleport.
2. INV-036 (interruptible procedures) -> replay/golden-fixture check: `routine_blocked_diagnostic_001` proves a blocked routine emits a typed failure/replan diagnostic and an interruption.
3. INV-018 (replay equality) -> replay/golden-fixture check: agent-state checksum reproduces routine step index, status, ancestry, and interruption reason.

## What to Change

### 1. Step ancestry from ordinary actions

In the no-human loop, on each routine step, record the ordinary action id (movement/sleep/eat/work/search/wait-with-reason) onto `RoutineExecution.concrete_action_ancestry` and emit `RoutineStepStarted`/`RoutineStepCompleted`; on no producible action, emit `RoutineStepFailed` with a typed reason.

### 2. Typed interruption

Interrupt a routine on a severe need (cause from 0007PHA3ASECHAR-005), blocked access, action failure, work-hour boundary, safety concern, or epistemic contradiction, recording `failure_interruption_reason` and emitting the routine failure/interruption diagnostic; coordinate with the intention interruption (0007PHA3ASECHAR-006).

## Files to Touch

- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs` (modify)

## Out of Scope

- `continue_routine` non-progress discipline (0007PHA3ASECHAR-008).
- Intention lifecycle events (0007PHA3ASECHAR-006) — consumed here for coordinated interruption, owned there.
- Debug/TUI rendering of routine execution state (0007PHA3ASECHAR-009).

## Acceptance Criteria

### Tests That Must Pass

1. `routine_no_teleport_001`: a routine step advances only with backing ordinary-action ancestry; no teleport.
2. `routine_blocked_diagnostic_001`: a blocked routine emits a typed failure/replan diagnostic and a typed interruption.
3. Replay reproduces routine step index, status, `concrete_action_ancestry`, and `failure_interruption_reason` (agent-state checksum equality).
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Routine progress is backed by an ordinary action id or a typed failure; no marker-only advance.
2. Routines are interruptible by stronger modeled causes with typed diagnostics.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/routine.rs` — unit tests: step advance with ancestry; step failure; interruption reason recorded.
2. `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs` — no-teleport step ancestry.
3. `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs` — blocked-routine typed diagnostic + interruption.

### Commands

1. `cargo test -p tracewake-core agent::routine`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

What changed:

- Wired `run_no_human_day` to emit and apply routine step events after ordinary no-human proposals when an actor has a live `RoutineExecution`.
- Successful instant ordinary actions now emit `RoutineStepStarted` and `RoutineStepCompleted` caused by the ordinary action event; the start event records `concrete_action_ancestry`.
- Duration starts such as sleep/work emit `RoutineStepStarted` without marker-only completion.
- Typed ordinary failures (`ActionRejected`, `EatFailed`, `WorkBlockFailed`, `ContinueRoutineRejected`) emit `RoutineStepFailed` with a stable failure reason and fallback attempt count.
- Added scheduler tests proving routine step ancestry replays to the same routine execution state and blocked work records a typed routine failure reason.

Deviations from original plan:

- No fixture schema changes were required; the existing `routine_no_teleport_001` and `routine_blocked_diagnostic_001` checks pass against the integrated no-human routine-step events.
- Broader interruption coordination remains narrow here: routine failures are recorded from typed ordinary action failures, while severe-need intention interruption remains the cause signal introduced in `0007PHA3ASECHAR-005`.

Verification results:

- `cargo test -p tracewake-core scheduler`
- `cargo test -p tracewake-core agent::routine`
- `cargo test -p tracewake-content routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry`
- `cargo test -p tracewake-content routine_blocked_fixture_records_access_failure_without_silent_loop`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
