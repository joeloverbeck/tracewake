# 0005PHA3ANEEROU-008: Sleep action (duration-based)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a duration-based `sleep` action def + registry registration, with scheduled completion and fatigue recovery over time.
**Deps**: 0005PHA3ANEEROU-005, 0005PHA3ANEEROU-007

## Problem

Phase 3A sleep must be scheduled/duration-based and recover fatigue over elapsed time — never an instant meter refill (Spec 0005 §9.6, §9.4, §21; `INV-045`). Sleep is one of the required ordinary actions that must flow through the same shared pipeline used by the possessed actor (Spec §3.4, §9.1), so it is implemented as a registry action with start/completion/interruption events, not a TUI-only or debug command.

## Assumption Reassessment (2026-06-07)

1. Action defs live in `crates/tracewake-core/src/actions/defs/` (existing: movement, openclose, takeplace, wait, checkcontainer, accuseprobe, inspect) and register through `crates/tracewake-core/src/actions/registry.rs` (e.g. `register_phase1_movement_open_close`, `registry.rs:67`); `defs/mod.rs` declares them. This ticket adds `sleep.rs` following that pattern, with a Phase 3A registration fn.
2. The need model (001), sleep-lifecycle event kinds (005), and passive-delta machinery (007) provide fatigue recovery and the start/complete/interrupt events. Spec §9.6 fixes the validation (actor enabled; accessible/known sleep place or bed; at place or routable there; bed not blocked by implemented occupancy; not already body-exclusive) and the effects (intention/routine ancestry, start event, scheduled completion or interruption, fatigue recovery over duration, possible hunger rise over duration, safety adjustment only from modeled context, replay rebuild, TUI affordance + why-not).
3. Shared boundary under audit: `sleep` produces a duration action — a start event plus an expected-completion tick the scheduler resumes (Spec §9.4). The scheduled-completion mechanism is shared with work (ticket 010) and consumed by the runner (ticket 017) and reservation (ticket 011). This ticket establishes the duration-completion pattern.
4. Invariant motivating this ticket: `INV-045` — "Ordinary survival is causal; fake meter refills disconnected from world state are forbidden." Fatigue must recover as a function of elapsed sleep duration via events, with no path that sets fatigue to comfortable instantly (Spec §9.6 closing line, §21).
5. Deterministic-replay / reservation surface: sleep start/completion/interruption events and the duration recovery feed replay (ticket 006) and the body-exclusive reservation (ticket 011). This ticket adds no nondeterminism — recovery is a fixed function of elapsed ticks, completion is scheduled on a deterministic tick. Reservation conflict typing (`scheduling/reservation`) is enforced in ticket 011; here sleep only marks itself body-exclusive and reserves the bed slot modeled in ticket 003.

## Architecture Check

1. Implementing sleep as a registry action with a start event + scheduled completion (rather than a single instantaneous effect) is the only shape that makes "no instant refill" structurally true and keeps possessed and autonomous sleep on one pipeline (`INV-045`, Spec §3.4). Reusing the scheduler's deterministic ordering for completion avoids a parallel timer.
2. No backwards-compatibility shims: new action def registered through the existing registry fn pattern; duration-completion reuses scheduler ordering keys, no new scheduling engine.

## Verification Layers

1. No instant refill (`INV-045`) -> unit test: starting sleep emits a start event and schedules completion; fatigue is unchanged at start and recovers only across elapsed completion ticks — there is no synchronous "fatigue := comfortable" path.
2. Shared pipeline (Spec §3.4/§9.1) -> integration test: a `sleep` proposal from a non-human origin (Scheduler/Agent/Test) validates and applies through `actions/pipeline.rs` identically to a Human-origin proposal for equivalent state.
3. Duration determinism (`INV-018`, substrate-only) -> unit test: completion fires at the deterministic expected tick; interruption before completion yields a partial-recovery interruption event. Full replay enforcement is ticket 006 (cited).

## What to Change

### 1. Sleep action def

Add `crates/tracewake-core/src/actions/defs/sleep.rs` implementing the §9.6 validation and a duration effect: emit a sleep-start event (ticket 005), compute an expected-completion tick, reserve the bed/body slot, and on completion emit a completion event applying fatigue recovery over elapsed duration (with possible hunger rise via ticket 007's deltas). Interruption before completion emits an interruption/failure event with partial recovery.

### 2. Registry + module registration

Register `sleep` via a Phase 3A registration fn in `crates/tracewake-core/src/actions/registry.rs` and declare the module in `crates/tracewake-core/src/actions/defs/mod.rs`.

### 3. Scheduled completion hook

Use the scheduler's deterministic ordering to resume the action at the expected-completion tick (shared with ticket 010); if multiple completions land on one tick, ordering is stable (Spec §9.4).

## Files to Touch

- `crates/tracewake-core/src/actions/defs/sleep.rs` (new)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — declare `sleep`)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register Phase 3A `sleep`)
- `crates/tracewake-core/src/scheduler.rs` (modify — duration-completion resume hook, shared with work)

## Out of Scope

- Eat/food and work actions (tickets 009, 010).
- Body-exclusive reservation conflict enforcement and typing (ticket 011).
- `SleepNight`/`MorningWake` routine methods that select sleep (ticket 013).
- TUI sleep affordance rendering (ticket 022) — this ticket exposes the action; rendering is later.

## Acceptance Criteria

### Tests That Must Pass

1. Starting sleep emits a start event and schedules completion at a deterministic tick; fatigue is unchanged at start.
2. On completion, fatigue recovers as a function of elapsed duration (no instant comfortable); interruption emits a typed interruption event with partial recovery.
3. A non-human-origin `sleep` proposal validates/applies through the shared pipeline identically to a Human-origin one for equivalent state.

### Invariants

1. Fatigue never drops except across elapsed scheduled-sleep ticks via events (`INV-045`).
2. Sleep is body-exclusive and routes through the shared action pipeline, not a TUI/debug-only path (Spec §3.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/sleep.rs` (unit tests) — validation, start/completion/interruption, recovery-over-duration.
2. `crates/tracewake-core/tests/acceptance_gates.rs` (modify) — shared-pipeline parity for `sleep` across origins (extended by capstone).

### Commands

1. `cargo test -p tracewake-core actions::defs::sleep`
2. `cargo test -p tracewake-core actions`
3. Core-crate scope is correct; end-to-end sleep within `no_human_day_001`/`sleep_eat_work_001` is exercised by tickets 019/021/025.

## Outcome

Completed: 2026-06-07

Changed:
- Added the Phase 3A `sleep` action definition and registry registration.
- Sleep start now emits a duration-based `SleepStarted` event with deterministic expected completion metadata, `body_exclusive=true`, and no start-time fatigue recovery.
- Added completion/interruption builders that emit `SleepCompleted`/`SleepInterrupted` plus elapsed-duration fatigue and hunger `NeedDeltaApplied` events.
- Added deterministic duration-completion ordering-key helper for scheduled action resumes.
- Added unit and acceptance coverage for start, completion, interruption, invalid sleep place, and shared Human-vs-Scheduler pipeline routing.

Deviations:
- Bed/body reservation conflict enforcement remains deferred to ticket 011 as scoped.
- Completion scheduling is represented by deterministic start-event metadata and a shared duration ordering-key helper; the runner-level resume mechanism remains deferred to later runner/capstone tickets.

Verification:
- `cargo fmt --all --check`
- `cargo test -p tracewake-core actions::defs::sleep`
- `cargo test -p tracewake-core actions`
- `cargo test -p tracewake-core sleep_proposals_share_pipeline_across_human_and_nonhuman_origins`
