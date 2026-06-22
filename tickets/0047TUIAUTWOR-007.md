# 0047TUIAUTWOR-007: Canonical duration completion/interruption phase

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-core` (`scheduler.rs` coordinator duration-lifecycle phase invoking existing terminal builders); core test
**Deps**: 0047TUIAUTWOR-006

## Problem

Spec 0047 §4.2/§4.4 requires the coordinator's duration-lifecycle phase to close due sleep/work durations (and authorized early interruptions) through the existing terminal builders, with shared keying, appending terminals through the scheduler/pipeline-owned event seam — duplicate/orphan terminals fail closed. The discovery phase (0047TUIAUTWOR-006) yields the ordered due-candidate set; this ticket consumes it and invokes the existing completion/interruption builders, filling the §4.9 item-4 seam left in the coordinator. Sleep/work terminals and authorized early interruption close through the existing builders rather than a new duration model (spec §3.1: "No new duration model is needed").

## Assumption Reassessment (2026-06-22)

1. The existing terminal builders are: `build_sleep_completion_events` (`actions/defs/sleep.rs:167`), `build_sleep_interruption_events` (`actions/defs/sleep.rs:259`), and `build_work_completion_events` (`actions/defs/work.rs:119`). All three derive elapsed effects via `classify_actor_tick_regimes_with_start` (`need_accounting.rs:38`). There is **no** `build_work_interruption_events` — work interruption is not currently modeled (resolves spec §4.9 item-12 / §9 open-question-2 "interruption case if supported": sleep has completion + interruption; work has completion + modeled failure but no interruption builder). This ticket invokes the existing builders; it adds no new duration model.
2. `is_duration_terminal` (`events/envelope.rs:363`) centralizes terminal classification (`SleepCompleted`/`SleepInterrupted`/`WorkBlockCompleted`/`WorkBlockFailed` → true). The discovery phase (0047TUIAUTWOR-006) already keys open starts by `EventId` via `open_body_exclusive_starts`; this ticket closes them with the same keying so a terminal is paired to its start.
3. Cross-artifact boundary under audit: the duration-lifecycle phase sits between discovery (0047TUIAUTWOR-006) and accounting (0047TUIAUTWOR-008) in the coordinator's frozen phase order (0047TUIAUTWOR-005). Terminals must be appended through the single scheduler/pipeline-owned event seam — not via a TUI or direct-mutation path — so the no-direct-dispatch contract holds.
4. Constitutional invariant motivating the ticket: `INV-045` (ordinary survival is causal — human-started sleep/work now reach terminals so fatigue recovers and work completes by elapsed causal ticks) and `INV-010` (every event needs a cause model — terminals carry their start's cause keying, not a synthesized record).
5. Enforcement surface (fail-closed + deterministic replay): duplicate/orphan terminals fail closed (the discovery authority's `DuplicateDurationTerminal`); the elapsed-effect derivation via `classify_actor_tick_regimes_with_start` is deterministic over the log. Appending terminals through the scheduler/pipeline-owned seam keeps the single-owner event route (`INV-103`/`INV-104`); the isolated terminal/interruption/reservation-closure test is the regression lock.

## Architecture Check

1. Invoking the existing completion/interruption builders from the coordinator — rather than re-deriving elapsed effects inline — preserves the single duration model spec §3.1 found substantial and correct, and means the human path and the no-human path (post-0047TUIAUTWOR-010) close durations through identical code. A second completion engine would risk divergent recovery/output accounting.
2. No backwards-compatibility aliasing/shims: the coordinator calls the existing builders directly; no parallel terminal-construction path is introduced. Early interruption flows through the same terminal builder with an explicit terminal tick (or a narrowly-factored lifecycle helper), yielding one shared terminal and one counted tick expansion.

## Verification Layers

1. `INV-045` causal survival -> replay/golden-fixture check: a due sleep closes via `build_sleep_completion_events` with correct recovery; a due work closes via `build_work_completion_events` with output (isolated terminal test).
2. `INV-103`/`INV-104` single event route -> codebase grep-proof: terminals are appended only through the scheduler/pipeline-owned seam in the coordinator, not a direct-mutation path.
3. Fail-closed -> schema/replay check: a duplicate/orphan terminal scenario fails closed via the shared discovery authority.

## What to Change

### 1. Coordinator duration-lifecycle phase (`scheduler.rs`)

Replace the empty lifecycle seam (from 0047TUIAUTWOR-005) with: for each due candidate from the discovery phase, in deterministic order, invoke the matching existing builder — `build_sleep_completion_events` / `build_work_completion_events` for due terminals, `build_sleep_interruption_events` for authorized sleep interruption — keyed to the open start's `EventId`. Append the resulting terminals through the scheduler/pipeline-owned event seam. Authorized early interruption supplies an explicit terminal tick so elapsed effects expand correctly.

### 2. Interruption authorization seam (`scheduler.rs`)

Accept an authorized-interruption signal (from a typed lifecycle control, wired fully in 0047TUIAUTWOR-009/012) and route it through `build_sleep_interruption_events`. Work has no interruption builder; a work duration closes via completion or modeled failure only (record this as the resolved scope of §4.9 item-12's conditional).

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005)

## Out of Scope

- The reservation predicate that rejects ordinary actions during an open duration (0047TUIAUTWOR-009) — this ticket closes durations; it does not gate new proposals.
- Unified per-tick need accounting (0047TUIAUTWOR-008).
- The advance-until continuation control that triggers interruption from the TUI (0047TUIAUTWOR-012) — this ticket provides the authorization seam, not the control.
- Adding a work-interruption builder (not modeled; out of scope for this feature).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test world_step_coordinator` — a due sleep started in a prior transaction closes through `build_sleep_completion_events` with correct fatigue recovery; a due work closes through `build_work_completion_events` with output.
2. An authorized sleep interruption at an explicit terminal tick closes through `build_sleep_interruption_events` with one shared terminal and the correct counted-tick expansion.
3. `cargo test -p tracewake-core` passes; `cargo clippy -p tracewake-core --all-targets -- -D warnings` clean.

### Invariants

1. Terminals are appended only through the scheduler/pipeline-owned event seam, keyed to the open start's `EventId` (`INV-103`/§8).
2. No new duration model or second completion engine is introduced; the existing builders are the sole terminal source (`INV-045`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — isolated terminal, authorized-interruption, and shared-keying tests over coordinator-discovered durations.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core && cargo clippy -p tracewake-core --all-targets -- -D warnings`
3. Core-suite boundary is correct: completion is a kernel-internal phase with no TUI wiring change yet.
