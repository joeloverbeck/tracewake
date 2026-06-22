# 0047TUIAUTWOR-011: TUI one-tick world advance; route `WaitOneTick`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-tui` (`src/app.rs` typed world-step entry, `src/run.rs` dispatch, `src/input.rs` as needed); TUI tests
**Deps**: 0047TUIAUTWOR-005, 0047TUIAUTWOR-009

## Problem

Spec 0047 §4.6 requires a typed `TuiApp` world-step entry point through which `UiCommand::WaitOneTick` is routed: a free actor's `wait` remains an ordinary semantic proposal, but an accepted wait now advances the full world through the coordinator. Today `TuiApp::submit_entry` (`app.rs:246`) runs one action-pipeline transaction and adopts the last emitted event tick (`app.rs:287`), and `UiCommand::WaitOneTick` (`input.rs:13`) merely selects the ordinary `wait` action (`run.rs:72-78`) — no human path invokes an authoritative world step. This ticket adds the typed `TuiApp` world-step entry and routes `WaitOneTick` through the coordinator, while keeping the TUI a pure typed-request client (no event application, no state mutation, no terminal-builder calls, no debug-runner reuse).

## Assumption Reassessment (2026-06-22)

1. `TuiApp` exists at `crates/tracewake-tui/src/app.rs:74`; `submit_entry` (`:246`) runs `run_pipeline` and at `:287` adopts the last event's `sim_tick` — it advances the actor transaction but not the authoritative world. `UiCommand::WaitOneTick` is at `crates/tracewake-tui/src/input.rs:13`; `run.rs:72-78` maps it to the ordinary `wait` semantic action via `submit_semantic_action`. These are the seams §4.6 names.
2. The coordinator's typed world-advance request/result (0047TUIAUTWOR-005) is the contract the TUI submits to. The reservation generalization (0047TUIAUTWOR-009) ensures a human attempting an ordinary `wait` while mid-sleep is rejected with `ReservationConflict` rather than wrongly accepted — so the human path's correctness depends on both.
3. Cross-artifact boundary under audit: the TUI → core boundary (architecture `10`, amended in 0047TUIAUTWOR-002): the TUI submits a typed request and consumes a typed result; it never applies events, mutates state, calls terminal/completion builders, or reuses the debug `RunNoHumanDay` runner for gameplay. `submit_entry`'s current "adopt last event tick" shortcut is replaced by the coordinator's returned resulting tick.
4. Constitutional invariant motivating the ticket: `INV-008` (UI assistance is not authority), `INV-069` (the TUI must not implement simulation rules / mutate world state through player-only paths), and `INV-103`/`INV-104` (no direct dispatch; the TUI sends typed requests, the coordinator orders established seams).
5. Enforcement surface (kernel-authority boundary + no-direct-dispatch): the world-advance entry must route a free actor's accepted `wait` through the coordinator and adopt the coordinator's resulting tick — never advance a TUI-local clock or apply events directly. The change introduces no player-only path and no hidden-truth read (the TUI continues to render from the actor-filtered view model). The "TUI never calls `run_no_human_day`/terminal builders" guard is the regression lock.

## Architecture Check

1. A typed `TuiApp` world-step entry that submits the coordinator's request — rather than extending `submit_entry`'s last-event-tick adoption — keeps the TUI a presentation/input client and puts all world authority in core, satisfying the architecture `10` boundary. Letting the TUI adopt a tick without a coordinator step is the original defect (the world never advances).
2. No backwards-compatibility aliasing/shims: `WaitOneTick` is rerouted through the coordinator; no parallel "old wait that only advances the actor" path is retained, and the TUI gains no direct mutation or debug-runner call.

## Verification Layers

1. `INV-008`/`INV-069` UI-not-authority -> codebase grep-proof: `app.rs`/`run.rs` contain no direct event application, no terminal-builder call, and no `run_no_human_day`/`RunNoHumanDay` gameplay call on the human path.
2. `INV-103`/`INV-104` no direct dispatch -> replay/golden-fixture check: an accepted human `WaitOneTick` advances the full world by one coordinator step (a real-pipeline TUI test), adopting the coordinator's resulting tick.
3. Reservation interaction -> manual review: a human `WaitOneTick` while mid-sleep is rejected with `ReservationConflict` (depends on 0047TUIAUTWOR-009).

## What to Change

### 1. Typed `TuiApp` world-step entry (`app.rs`)

Add a typed world-step entry point that builds the coordinator's world-advance request (expected/frontier tick, human controller/process origin) and consumes the typed step result, replacing `submit_entry`'s last-event-tick adoption for world-advancing commands. The entry submits to core and never applies events or mutates state directly.

### 2. Route `UiCommand::WaitOneTick` through the entry (`run.rs`, `input.rs`)

Route `WaitOneTick` so a free actor's accepted `wait` advances the full world via the coordinator. Keep `wait` an ordinary semantic proposal (it still goes through the pipeline); the difference is that acceptance now triggers a full coordinator step. Do not overload `wait` to ever mean "stay asleep" — that is the separate continuation control (0047TUIAUTWOR-012).

## Files to Touch

- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify — as needed for the typed request, no new gameplay command yet)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — human one-tick world-advance test)

## Out of Scope

- The continuation/advance-until command and its controller (0047TUIAUTWOR-012).
- The actor-known interval projection and view-model field (0047TUIAUTWOR-013).
- Any core coordinator logic (0047TUIAUTWOR-005…009) — this ticket only wires the TUI to the existing entry.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` — an accepted human `WaitOneTick` for a free actor advances the full world one coordinator step (real parser/app/core path) and the TUI adopts the coordinator's resulting tick.
2. A human `WaitOneTick` while the possessed actor is mid-sleep is rejected with `ReservationConflict` (no world advance, no `ActorWaited`).
3. `grep -nE "run_no_human_day|RunNoHumanDay|apply_event|mutate" crates/tracewake-tui/src/app.rs crates/tracewake-tui/src/run.rs` shows no gameplay-path direct mutation / debug-runner reuse; `cargo clippy -p tracewake-tui --all-targets -- -D warnings` clean.

### Invariants

1. The TUI sends typed requests only; no event application, state mutation, terminal-builder call, or debug-runner reuse on the human path (`INV-008`/`INV-069`/`INV-103`).
2. `wait` remains a one-tick command for a free actor and never means "stay asleep" (the typed continuation distinction is preserved for 0047TUIAUTWOR-012).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` (modify) — human one-tick world-advance real-pipeline test + mid-sleep `WaitOneTick` rejection.

### Commands

1. `cargo test -p tracewake-tui embodied`
2. `cargo test -p tracewake-tui && cargo clippy -p tracewake-tui --all-targets -- -D warnings`
3. The TUI-suite boundary is correct: this ticket only wires the TUI to the coordinator; core behavior is proven by 0047TUIAUTWOR-005…009.
