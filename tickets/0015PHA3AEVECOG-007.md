# 0015PHA3AEVECOG-007: Scheduled completions revalidate continuity; wire sleep/work interruption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — completion builders take current state and validate continuity; modeled interruption path wired; typed interruption events with prorated deltas; guard + two fixtures
**Deps**: None

## Problem

ORD-HARD-011: scheduled duration completions skip revalidation and sleep interruption is unreachable. `scheduler.rs::no_human::append_scheduled_completion` builds completion events from the *started-event payload* (`build_sleep_completion_events` / `build_work_completion_events`, defined in `actions/defs/sleep.rs:152` and `actions/defs/work.rs:113`) and appends them directly — neither builder receives current state, so nothing checks that the actor is still present, still in the sleeping/working condition, or that no interruption occurred. `build_sleep_interruption_events` exists (`actions/defs/sleep.rs:168`) but is invoked **only from a test** (`sleep.rs:477`, inside the `#[cfg(test)]` module) — never from production. INV-035's interruption points cannot fire, so every accepted sleep yields full scheduled recovery unconditionally (INV-043/044/045: a refill disconnected from current world state).

This ticket makes completion builders validate continuity against current state and wires a modeled interruption path for Phase 3A's interruption sources (severe need/safety pressure crossing during a duration; any event that displaces the actor). Interruptions are events (INV-015) and feed the next decision window. Independent of the ORD-HARD-008 cutover.

## Assumption Reassessment (2026-06-09)

1. Current code (verified): `append_scheduled_completion` in `crates/tracewake-core/src/scheduler.rs` calls `build_sleep_completion_events`/`build_work_completion_events`; both build from the started-event payload with no current-state parameter. `build_sleep_interruption_events` (`actions/defs/sleep.rs:168`) is referenced only at `sleep.rs:477` inside `#[cfg(test)]` (nearest `cfg(test)` at `sleep.rs:330`) — confirmed test-only. The view/status concept `SleepInterrupted` already appears in `projections.rs` and `WorkBlockFailed` in fixture `routine_no_teleport_001`; confirm whether a corresponding `EventKind` variant exists or must be added in `events/envelope.rs`.
2. Specs/docs: spec 0015 §ORD-HARD-011 (required correction + structural lock + capstone extension); INV-001 (causality), INV-035 (interruption points are part of the routine contract), INV-043/044/045 (survival is causal; no fake refills), INV-015 (failure/interruption is eventful).
3. Shared boundary under audit: the scheduler completion seam (`append_scheduled_completion`) and the duration-completion event builders. The guard allowlist rationale ("scheduler may complete previously accepted duration actions") covers *who* appends — a permitted scheduler power — but does not license completing a duration whose continuity no longer holds; this ticket adds the continuity check the allowlist does not cover.
4. INV-035 — routines are defeasible intentions with interruption points; a sleep/work duration must be interruptible by stronger modeled causes. INV-043/044/045 — recovery must connect to current world state; a full scheduled refill regardless of interruption is a fake meter refill. INV-015 — an interruption is a consequential failure and must be an event feeding the next window.
5. Fail-closed / deterministic-replay surface: names the enforcement surface = the completion builders + the scheduler completion path. Confirm: a broken continuity produces a typed `SleepInterrupted`/`WorkBlockFailed`-class event with **prorated** need deltas for elapsed ticks (not full scheduled recovery); the scheduler must not append a completion whose continuity check did not run; interrupted durations replay byte-identically. No leakage — continuity is checked against authoritative `PhysicalState`/`AgentState` to *validate* the completion (INV-099: truth may validate), not to plan cognition.
6. Schema extension: if `SleepInterrupted`/`WorkBlockFailed` are not yet `EventKind` variants, add them in `events/envelope.rs` (+apply arm, registry metadata, INV-020 version) with prorated-delta payloads. Consumers: `events/apply.rs`, `checksum.rs`, the projection reading completion/interruption status, the intention-lifecycle path. Additive-only if new; confirm against existing variants during implementation.

## Architecture Check

1. Passing current state into the completion builders and gating the scheduler append on a continuity check makes recovery causal (INV-045) and interruption reachable (INV-035) — the minimal correct fix vs. the current open-loop "append from started payload". Reusing the already-present (but unwired) `build_sleep_interruption_events` honors the existing design intent rather than inventing a parallel mechanism.
2. No shims: the open-loop completion path is replaced by the continuity-checked one; no fallback that appends a full-recovery completion when the check is skipped.

## Verification Layers

1. INV-035/015 → replay/golden-fixture check (fixture `sleep_interrupted_by_severe_need_prorates_recovery_001`): a severe need crossing during sleep produces a typed interruption event with prorated deltas, live and under replay.
2. INV-043/044/045 → replay/golden-fixture check (fixture `work_completion_fails_when_actor_displaced_001`): an actor displaced mid-work yields `WorkBlockFailed`-class output, not full completion.
3. INV-001 → codebase grep-proof (guard): completion builders reference current-state continuity checks; the scheduler completion path does not append a completion whose continuity check did not run (positive-presence assertion).
4. INV-018 → replay/golden-fixture check: interrupted durations replay byte-identically; capstone asserts at least one interrupted duration in an adversarial run.

## What to Change

### 1. Continuity-validating completion builders

`build_sleep_completion_events` / `build_work_completion_events` take current `PhysicalState`/`AgentState` and validate continuity (actor present, condition uninterrupted, affordance still usable). Broken continuity → a typed `SleepInterrupted`/`WorkBlockFailed`-class event with prorated need deltas for elapsed ticks.

### 2. Wire the modeled interruption path

Invoke `build_sleep_interruption_events` (and a work equivalent) from production: severe need/safety pressure crossing during a sleep/work duration, and any event that displaces the actor, produce interruption events that feed the next decision window.

### 3. Gate the scheduler append

`append_scheduled_completion` must not append a completion whose continuity check did not run.

### 4. Guard

Add a guard: completion builders reference current-state continuity checks (positive-presence); the scheduler completion path is continuity-gated.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — pass current state; gate the append)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify — continuity + wire interruption builder)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify — continuity + interruption equivalent)
- `crates/tracewake-core/src/events/envelope.rs` (modify — `SleepInterrupted`/`WorkBlockFailed` EventKind, if not already variants)
- `crates/tracewake-core/src/events/apply.rs` (modify — apply arms, if new kinds)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: also 004/005/006/008; 009 rewrites)
- `crates/tracewake-content/src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs` (new)
- `crates/tracewake-content/src/fixtures/work_completion_fails_when_actor_displaced_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register the two fixtures)

## Out of Scope

- The capstone's no-human run asserting an interrupted duration end-to-end (`0015PHA3AEVECOG-011`); this ticket delivers the mechanism and the two unit fixtures.
- Content-authored recovery/duration constants (`0015PHA3AEVECOG-008` — depends on this ticket).
- ORD-HARD-008 cognition channel (`0015PHA3AEVECOG-001`/`-002`/`-003`).

## Acceptance Criteria

### Tests That Must Pass

1. `sleep_interrupted_by_severe_need_prorates_recovery_001`: severe need crossing during sleep → typed interruption with prorated deltas, identical live and under replay.
2. `work_completion_fails_when_actor_displaced_001`: displacement mid-work → `WorkBlockFailed`-class, not full completion.
3. Removing the continuity check fails the completion-continuity guard.
4. `cargo test --workspace` green.

### Invariants

1. No scheduled completion is appended without a continuity check.
2. A broken continuity yields a typed interruption event with prorated deltas, never full scheduled recovery.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs`, `work_completion_fails_when_actor_displaced_001.rs`.
2. `crates/tracewake-core/src/actions/defs/sleep.rs` / `work.rs` — unit tests for continuity branching.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — completion-continuity positive-presence guard.

### Commands

1. `cargo test -p tracewake-core scheduler:: actions::defs::sleep actions::defs::work && cargo test -p tracewake-content interrupt`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
