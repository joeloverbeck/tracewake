# 0047TUIAUTWOR-008: Unified per-tick need accounting

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-core` (`scheduler.rs` coordinator accounting phase, `need_accounting.rs`, `wait.rs`); adversarial content fixtures + core tests
**Deps**: 0047TUIAUTWOR-007

## Problem

Spec 0047 §4.3 requires the coordinator to own one per-tick reconciliation across action-emitted, passive, and duration-regime effects, asserting exactly one applicable `(actor, need, tick)` classification — emitting only missing effects through the single-owner ordinary-life seam. A free actor's `wait` keeps its existing `ActorWaited` event and sealed reason; the world step recognizes that evidence rather than re-charging it. The danger is a two-pass "submit wait, then independently apply a world tick" implementation that double-charges, or treating an actor as both awake-waiting and asleep across a tick. This ticket fills the §4.9 item-5 accounting seam, preserves the existing single-charge fixtures unchanged, and adds double-charge adversarial cases.

## Assumption Reassessment (2026-06-22)

1. The accounting building blocks exist: `append_passive_need_events_before_decision` (`scheduler.rs:1468`), `passive_awake_need_deltas` (`time.rs:32`, deterministic + non-reducing per its own test), `classify_actor_tick_regimes_with_start` (`need_accounting.rs:38`, returns `TickRegimeCounts{awake_ticks, asleep_ticks, working_ticks}`), and `build_wait_events` (`wait.rs:32`) which emits `ActorWaited` plus awake need deltas (`wait.rs:141`). The coordinator must reconcile these around one per-tick ledger rather than running passive accounting and a wait charge independently.
2. The three single-charge fixtures to preserve unchanged exist in `crates/tracewake-content/src/fixtures/`: `wait_then_window_passive_charges_each_tick_once_001` ("an autonomous wait tick is not charged again by the next no-human passive window"), `sleep_spanning_window_boundary_charges_each_tick_once_001` (sleep across a window boundary gets no passive awake deltas for slept ticks), and `scheduler_cannot_rewrite_wait_reason_after_transaction_001` (scheduler does not rewrite actor-visible wait reason). These encode the contract; they are preserved unchanged (their single-charge assertions stay green).
3. Cross-artifact boundary under audit: the accounting phase reconciles outputs of the wait action (`wait.rs`), the passive-accounting seam (`time.rs`/`scheduler.rs`), and the duration-regime classification (`need_accounting.rs`) — three transport paths for "what need-delta does this `(actor, need, tick)` get". Spec §4.3 forbids two of them charging the same tick. The canonical end-state is one reconciliation in the coordinator; the proof surface is the preserved fixtures + new double-charge adversarial fixtures.
4. Constitutional invariant motivating the ticket: `INV-045` (ordinary survival is causal; fake/duplicated meter changes are forbidden — exactly one charge classification per affected need/tick) and the architecture `00` conformance row `0017 tick-charge attribution authority` (every tick-delta need charge reconciled by counted `(actor, need, tick)` occurrences).
5. Enforcement surface (fail-closed correctness + deterministic replay): the per-tick reconciliation must classify every `(actor, need, tick)` with the shared awake/asleep/working authority and emit only missing effects through the single-owner action-pipeline/ordinary-life seam. It must not implement "submit wait, then independently apply a world tick" (double-charge) or treat an actor as both awake-waiting and asleep across a tick. The accepted `ActorWaited` event and sealed reason for a free actor's one-tick wait are preserved and recognized, not replaced by an unrelated passive event. If existing structure cannot express the ordering, introduce a typed tick-accounting plan inside core — never move arithmetic into TUI code or add a "human wait already charged" boolean.

## Architecture Check

1. One coordinator-owned reconciliation across all three effect sources is the only design that holds the single-charge invariant when the same tick is simultaneously a wait-charged tick and a passive-window tick — the defect the existing `wait_then_window_passive_charges_each_tick_once_001` fixture already guards on the no-human side, now extended to the human path. A per-source independent charge (the two-pass shape) is what double-charges.
2. No backwards-compatibility aliasing/shims: no "human wait already charged" boolean flag and no TUI-side arithmetic; if ordering needs explicit representation, a typed tick-accounting plan lives in core.

## Verification Layers

1. `INV-045` single charge -> replay/golden-fixture check: the three existing single-charge fixtures stay green; new adversarial fixtures that would double-charge/double-recover under a two-pass implementation are charged exactly once.
2. `0017` tick-charge attribution -> codebase grep-proof: the coordinator classifies `(actor, need, tick)` via the shared regime authority (`classify_actor_tick_regimes_with_start`), not a regime-label set.
3. Wait-evidence preservation -> manual review: a free actor's `ActorWaited` event and sealed reason survive the world step unchanged (the `scheduler_cannot_rewrite_wait_reason_after_transaction_001` contract).

## What to Change

### 1. Coordinator accounting phase (`scheduler.rs`)

Replace the empty accounting seam (from 0047TUIAUTWOR-005) with one per-tick reconciliation: for each `(actor, need, tick)` in the step, classify the regime (awake/asleep/working) via the shared authority; recognize the free actor's already-emitted `ActorWaited` awake deltas as the charge for that tick; emit only the *missing* passive/duration-regime effects through the single-owner ordinary-life seam. Assert exactly one applicable classification per `(actor, need, tick)`.

### 2. Shared accounting seam (`need_accounting.rs`, `wait.rs`)

Expose whatever the coordinator needs to recognize an existing wait charge vs a missing passive charge (read-only over the log); keep the wait action's `ActorWaited` + sealed reason authoritative. If ordering cannot be expressed with existing structure, add a typed tick-accounting plan type in core (not TUI).

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/need_accounting.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (new — double-charge/double-recover adversarial fixtures; `fixtures/mod.rs` modify to register)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005)

## Out of Scope

- Reservation enforcement / continuation exemption (0047TUIAUTWOR-009).
- Editing the three existing single-charge fixtures (preserved unchanged — only new adversarial fixtures are added).
- TUI wiring (0047TUIAUTWOR-011) and the no-human refactor (0047TUIAUTWOR-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content` and `cargo test -p tracewake-core` — the three existing single-charge fixtures pass unchanged.
2. New adversarial fixtures (wait-tick coinciding with a passive window; sleep-then-resume across a tick) are charged/recovered exactly once — they would double under a two-pass implementation.
3. A free actor's one-tick `wait` retains its `ActorWaited` event and sealed reason after the world step (no unrelated passive event substituted).

### Invariants

1. Exactly one applicable `(actor, need, tick)` classification per tick; no `(actor, need, tick)` is both awake-waiting and asleep (`INV-045`/`0017`).
2. No "human wait already charged" boolean and no TUI-side accounting arithmetic; any ordering representation is a typed core plan.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/*` (new) — double-charge/double-recover adversarial fixtures registered in `fixtures/mod.rs`.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — per-tick reconciliation assertions + wait-evidence-preservation case.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core -p tracewake-content && cargo clippy -p tracewake-core --all-targets -- -D warnings`
3. The core+content boundary is correct: accounting is kernel-internal and exercised by content fixtures; no TUI surface changes yet.

## Outcome

Completed: 2026-06-22

Implemented the coordinator accounting phase in `crates/tracewake-core/src/scheduler.rs`. After duration lifecycle preflight, the coordinator now classifies each modeled actor's one-tick regime with `classify_actor_tick_regimes`, emits missing awake passive need deltas through the shared `build_need_delta_and_threshold_events` seam, and skips emission when applying the candidate delta would duplicate an already-charged `(actor, need, tick)` such as a prior accepted `wait`. Sleep/work ticks are not charged as awake passive ticks; duration lifecycle builders remain responsible for their elapsed effects.

Updated `crates/tracewake-core/tests/world_step_coordinator.rs` with coordinator-level adversarial coverage for missing awake passive accounting and for a prior accepted `ActorWaited` tick retaining its sealed reason without receiving a second world-step passive charge. No new content fixture files were added: the existing content single-charge fixtures remained unchanged and green under `cargo test -p tracewake-content`, while the new coordinator tests directly exercise the newly implemented world-step seam.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p tracewake-core --test world_step_coordinator` — passed.
3. `cargo test -p tracewake-core -p tracewake-content` — passed.
4. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
