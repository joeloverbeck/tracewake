# 0047TUIAUTWOR-012: Duration continuation + advance-until controller

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-core` (advance-until controller loop), `crates/tracewake-tui` (typed continuation command + dispatch); core + TUI tests
**Deps**: 0047TUIAUTWOR-011, 0047TUIAUTWOR-007

## Problem

Spec 0047 §1.8/§4.7 requires a typed continuation/advance-until control, separate from one-tick `wait`, that repeats canonical ticks and stops at the first deterministic stop condition: possessed duration terminal; actor-known salient observation/interruption; user pause/cancel before the next tick; or a configured deterministic safety bound / kernel error. `wait` remains a one-tick command when the actor is free; the typed distinction must survive parsing even if renderer text keeps the common case convenient. The controller is a pure loop over deterministic step results — it evaluates the stop set against typed terminals and actor-known salience, never against a hidden due queue. This ticket adds the controller (core loop) and its typed TUI command.

## Assumption Reassessment (2026-06-22)

1. The coordinator's canonical step + typed result (0047TUIAUTWOR-005) is the loop body; the duration-lifecycle phase (0047TUIAUTWOR-007) provides the typed terminals the stop set keys on, and its interruption authorization seam is what a continuation control triggers. The TUI world-step entry (0047TUIAUTWOR-011) is the human submission path the continuation command extends.
2. `UiCommand` (`crates/tracewake-tui/src/input.rs:5`) is the TUI input enum; this ticket adds a typed continuation/advance-until variant distinct from `WaitOneTick` (`input.rs:13`). Its blast radius is the parser (`input.rs`) and the dispatch (`run.rs`) — both updated here; no other consumer of `UiCommand` exists outside `tracewake-tui`.
3. Cross-artifact boundary under audit: the advance-until controller is a deterministic loop owned in core (it repeats the authoritative coordinator step), with the TUI contributing only the typed command and the user pause/cancel signal. The stop set must be evaluated against typed terminals and actor-known salience — not a hidden due queue (spec §4.7) — so the controller never reads debug/hidden state to decide when to stop.
4. Constitutional invariant motivating the ticket: `INV-112` (temporal authority — acceleration is repeated authoritative progression, and stopping is based on holder-known information) and `INV-008` (the continuation control is UI assistance that advances authoritative time through the coordinator, not a privileged shortcut).
5. Enforcement surface (deterministic stop + no-leak): the stop set is `{possessed duration terminal, actor-known salient observation/interruption, user pause/cancel before next tick, configured deterministic safety bound, kernel error}`. Salience defaults conservative and source-bearing — a hidden event never becomes salient because debug knows it (spec §9 open-question-3). A controller-bound stop (safety bound) is reported as a controller limitation, not in-world knowledge. The change reads no hidden due queue; the command-loop tests (stop-on-terminal, stop-on-salient-observation, stop-on-pause, stop-on-safety-bound) are the regression lock.

## Architecture Check

1. A separate typed continuation control — rather than overloading `wait` to sometimes mean "stay asleep" — keeps the one-tick wait and the controller-level acceleration distinct (foundation `08` amended doctrine, 0047TUIAUTWOR-001), so a free actor's `wait` is never ambiguous and the continuation never masquerades as an `ActorWaited` event (it creates no competing reservation, per 0047TUIAUTWOR-009).
2. No backwards-compatibility aliasing/shims: the controller loops the existing coordinator step; it introduces no second acceleration path and no `t`→`t+n` jump (acceleration is repeated one-tick markers).

## Verification Layers

1. `INV-112` temporal authority -> replay/golden-fixture check: advance-until repeats canonical ticks and stops at a possessed duration terminal; replay reconstructs the same stop tick and reason.
2. No-leak salience -> manual review (epistemic-leakage audit): a hidden other-actor event does not stop acceleration; only actor-known salient observation/interruption does.
3. Deterministic safety bound -> codebase grep-proof: a controller-bound stop is reported as a controller limitation (typed), distinct from an in-world stop reason.

## What to Change

### 1. Advance-until controller loop (`scheduler.rs` or a core controller module)

Add a pure loop over the coordinator's canonical step: execute a tick, evaluate the deterministic stop set (typed possessed-duration terminal; actor-known salient observation/interruption; user pause/cancel before the next tick; configured deterministic safety bound; kernel error). Default salience conservative and source-bearing. Return a typed controller result carrying the stop tick and an actor-visible stop reason plus (separately) the controller-internal stop diagnostics.

### 2. Typed continuation/advance-until TUI command (`input.rs`, `run.rs`, `app.rs`)

Add a `UiCommand` continuation/advance-until variant distinct from `WaitOneTick`; parse it (the typed distinction survives parsing even if renderer text is convenient) and dispatch it through the core controller via the `TuiApp` world-step path. The continuation triggers the coordinator's interruption authorization seam (0047TUIAUTWOR-007) when the user cancels a duration.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify — new `UiCommand` continuation variant)
- `crates/tracewake-tui/src/run.rs` (modify — dispatch)
- `crates/tracewake-tui/src/app.rs` (modify — controller invocation via world-step path)
- `crates/tracewake-tui/tests/command_loop_session.rs` (modify — command-loop stop-set tests)

## Out of Scope

- The actor-known interval-summary projection + view-model field (0047TUIAUTWOR-013) — this ticket produces the step/stop results the projection later summarizes.
- The salience *policy* tuning beyond a conservative source-bearing default (spec §9 open-question-3 leaves richer policy to implementation; this ticket lands the conservative default).
- Core duration completion/interruption mechanics (0047TUIAUTWOR-007).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` and `cargo test -p tracewake-core` — advance-until stops at a possessed duration terminal; stops at an actor-known salient observation; stops on user pause/cancel before the next tick; stops at the configured deterministic safety bound (reported as a controller limitation).
2. A hidden other-actor event does **not** stop acceleration (no leak); the stop reason for a safety-bound stop is typed as a controller limitation, not in-world knowledge.
3. The continuation command is a distinct `UiCommand` variant from `WaitOneTick` (parser test); `cargo clippy --workspace --all-targets -- -D warnings` clean.

### Invariants

1. Acceleration is repeated one-tick coordinator steps; no `t`→`t+n` jump; the stop set is evaluated against typed terminals + actor-known salience, never a hidden due queue (`INV-112`/§4.7).
2. `wait` stays one-tick; the continuation never emits `ActorWaited` for a sleeping actor and creates no competing reservation (`INV-008`/§4.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` (modify) — advance-until stop-set cases (terminal, salient observation, pause, safety-bound) + no-leak (hidden event does not stop).
2. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005) — core advance-until loop determinism + replay of stop tick/reason.

### Commands

1. `cargo test -p tracewake-tui command_loop`
2. `cargo test -p tracewake-core -p tracewake-tui && cargo clippy --workspace --all-targets -- -D warnings`
3. The core+TUI boundary is correct: the controller is a core loop driven by a TUI command, so both surfaces are exercised.
