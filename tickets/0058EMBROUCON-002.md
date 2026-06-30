# 0058EMBROUCON-002: Temporal gateway for embodied follow-ons

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `crates/tracewake-core/src/runtime/session.rs` (`run_embodied_continue_routine_follow_on` gateway); adds core behavioral tests
**Deps**: 0058EMBROUCON-001

## Problem

`run_embodied_continue_routine_follow_on` commits the follow-on via a direct `run_pipeline` call at the marker tick with no guard against time-advancing follow-ons (spec §3.2 H-0058-04, §4.2 F-0058-02 — the R1 harsh-acceptance failure). A `wait` follow-on emits `ActorWaited` and need deltas at `requested_tick.advance_by(ticks)` — an unguarded tick/need-accounting escape from scheduler temporal authority. This ticket adds a typed gateway: `continue_routine` follow-ons stay recursive typed stuck; same-tick non-duration ordinary follow-ons commit through the shared pipeline; time-advancing follow-ons are **rejected as typed stuck** (the approved decision — scheduler-owned wait routing is deferred per §2.2).

## Assumption Reassessment (2026-06-30)

1. `run_embodied_continue_routine_follow_on` at `crates/tracewake-core/src/runtime/session.rs:540` — the `OrderingKey` is built at `decision_tick = self.scheduler.current_tick()` (line 549) under `SchedulePhase::HumanCommand` (635) and the follow-on commits via `run_pipeline` (652) with no temporal guard; the recursive `continue_routine` guard exists (619-632). `wait.rs` emits `EventKind::ActorWaited` (`crates/tracewake-core/src/actions/defs/wait.rs:144`) and `NeedDeltaApplied` (204) at `proposal.requested_tick.advance_by(tick_count)`.
2. Spec §4.2 F-0058-02 and §3.2 H-0058-04; the approved either-or resolution is to **reject** a time-advancing follow-on as typed stuck (defer scheduler-owned wait routing). `DeterministicScheduler::transact_world_one_tick` (`crates/tracewake-core/src/scheduler.rs:790`) has a controlled-`wait` branch at line 839 — named here as the deferred routing target, **not** used.
3. Cross-artifact boundary under audit: the embodied follow-on commit path (`session.rs`) versus scheduler temporal authority (`scheduler.rs::transact_world_one_tick`). The gateway keeps time advancement scheduler-owned by failing closed.
4. Invariants under audit: INV-018/092 (deterministic replay is foundational / tested), INV-098 (feature acceptance is harsh), INV-112 (time may validate, but holder-known time must plan; the scheduler/replay clock may order and validate but must not become cognition authority). An unguarded direct-pipeline `wait` charges needs outside scheduler temporal ownership.
5. Enforcement surface: the new typed gateway before `run_pipeline` in `run_embodied_continue_routine_follow_on`. It is a fail-closed temporal/replay surface — a time-advancing follow-on must not commit need deltas at a future tick via the direct pipeline. Confirm the typed-stuck rejection reuses the existing embodied stuck-outcome machinery (`run_embodied_continue_routine_stuck_outcome`, session.rs:607), is eventful/replayable/actor-known, and introduces no replay nondeterminism or epistemic leak (the stuck reason is actor-known).
6. Schema/shape change: no schema shape change (the N/A pole) — internal control flow added to a private fn in `runtime/session.rs`; the typed-stuck path reuses existing `StuckDiagnosticRecord` / `ValidationReport` shapes unchanged and adds no public field. (Explicit N/A pole — `session.rs` trips the reseal heuristic but no reseal occurs.)

## Architecture Check

1. A typed gateway that fails closed on time-advancing follow-ons is the minimal harsh-acceptance fix (INV-098) that preserves scheduler temporal authority (INV-112) without rewriting scheduler cadence (§2.2). Classifying the follow-on by its temporal semantics at the commit boundary is cleaner than letting `run_pipeline` silently advance time and charge needs.
2. No backwards-compatibility aliasing/shims: the gateway rejects time-advancing follow-ons outright; it does not wrap the old direct-pipeline path behind a fallback. Scheduler-owned routing is a named future spec, not a hidden alias.

## Verification Layers

1. INV-018/112 (temporal authority) → behavioral test `embodied_continue_wait_follow_on_is_not_direct_pipelined` (fails if `ActorWaited` appears at `decision_tick + 1` without scheduler frontier advancement).
2. INV-098 (harsh acceptance) → the typed-stuck rejection assertion + the cross-ticket source guard `guard_0058_embodied_continue_time_advancing_follow_on_is_gated` in -006.
3. INV-015/105 (typed diagnostic) → the typed-stuck path emits a typed `StuckDiagnosticRecorded` with an actor-known explanation (manual review + the stuck-path assertion).

## What to Change

### 1. Typed gateway before `run_pipeline`

In `run_embodied_continue_routine_follow_on`, after the recursive-`continue_routine` handling and before the `run_pipeline` call (652), classify the follow-on proposal. Same-tick non-duration ordinary follow-ons commit through the shared pipeline at the marker tick (current behavior, retained). Tick-advancing follow-ons (`wait`, or any action whose builder advances the tick / charges passive needs) are rejected as typed stuck via `run_embodied_continue_routine_stuck_outcome`, with an actor-visible reason that continuation cannot safely commit a time-advancing follow-on yet.

Detection of "tick-advancing" is an implementer-recorded choice — preferred: an action-registry duration/advances-tick classification when available; otherwise an explicit `action_id == "wait"` check with a forward-looking note for future duration actions. Record the chosen mechanism in the implementing commit.

### 2. R1 documentation in code + tests

State in code comments and tests: the marker shares the current tick and is never progress; a non-time-advancing follow-on shares the marker tick; a time-advancing follow-on fails closed as typed stuck (scheduler-owned routing deferred).

### 3. Tests

Add `embodied_continue_wait_follow_on_is_not_direct_pipelined` (force the resolver to propose `wait`; assert a typed-stuck outcome, not a direct `ActorWaited` at `decision_tick + 1`). `embodied_continue_time_advancing_follow_on_charges_needs_once` is N/A under the typed-stuck decision — record the N/A (it would be required only if scheduler-owned wait routing were implemented).

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify)

## Out of Scope

- Scheduler-owned wait routing through `transact_world_one_tick` (deferred per §2.2 — a future scheduler spec).
- The single-charge / replay test `embodied_continue_time_advancing_follow_on_charges_needs_once` (N/A under the typed-stuck decision).
- Active-intention current-step authority (F-0058-01 → -001).

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_continue_wait_follow_on_is_not_direct_pipelined` — the resolver proposing `wait` yields a typed-stuck diagnostic, not a direct `ActorWaited` at `decision_tick + 1`; the test fails if `ActorWaited` is committed without scheduler frontier advancement.
2. `cargo test --workspace` passes.

### Invariants

1. No embodied follow-on advances time outside scheduler authority; time-advancing follow-ons fail closed as typed stuck (INV-112/098).
2. The typed-stuck outcome is eventful, replayable, and actor-known (INV-015/105).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (test module) — `embodied_continue_wait_follow_on_is_not_direct_pipelined`; `embodied_continue_time_advancing_follow_on_charges_needs_once` recorded as N/A (typed-stuck path).

### Commands

1. `cargo test -p tracewake-core embodied_continue_wait`
2. `cargo test --workspace`
