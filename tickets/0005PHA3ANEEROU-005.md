# 0005PHA3ANEEROU-005: Phase 3A event kinds and application

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends `EventKind` with Phase 3A cognition/need/routine/action kinds and adds their application in `events/apply.rs`.
**Deps**: 0005PHA3ANEEROU-001, 0005PHA3ANEEROU-002, 0005PHA3ANEEROU-003, 0005PHA3ANEEROU-004

## Problem

Phase 3A requires that needs, intentions, routine transitions, sleep/eat/work, planner traces, and stuck diagnostics be eventful or replay-reconstructible with explicit ancestry — no state may change behind the log merely because time passed (Spec 0005 §3.1, §15.2; `INV-009`, `INV-010`, `INV-011`). The event kinds and their application are the spine every later behavior ticket (007–017) emits through and that replay (ticket 006) rebuilds, so the versioned event vocabulary and apply arms land before the behaviors.

## Assumption Reassessment (2026-06-07)

1. `EventKind` lives in `crates/tracewake-core/src/events/envelope.rs` (confirmed: `grep "enum EventKind"`), and existing kinds cover controller binding, movement, open/close, take/place, wait/time-advance, `NoHumanAdvanceStarted`/`NoHumanAdvanceCompleted`, and Phase 2A epistemic records. `crates/tracewake-core/src/events/apply.rs` is the application surface; `events/log.rs` appends versioned envelopes. This ticket extends both, following the existing versioned-envelope discipline.
2. Spec §15.2 fixes the required event/projection semantics: need delta applied, need threshold crossed, candidate goals evaluated, intention started/continued/suspended/resumed/completed/failed/abandoned/interrupted, routine step started/completed/failed, planner/decision trace recorded, sleep started/completed/interrupted, food consumed / food-service used, eat failed, work block started/completed/failed, continue-routine proposed/accepted/rejected, stuck actor diagnostic, no-human day started/completed. Exact enum names are the implementer's; semantics are fixed.
3. Shared boundary under audit: `EventKind` is the central dispatch enum; every new variant needs an arm in `events/apply.rs`, the replay rebuild (ticket 006), the checksum/projection (ticket 006), and any debug projection (ticket 023). This ticket adds the variants and the apply arms; ticket 006 owns the replay/checksum arms. The cognition data types (001–004) provide the payloads.
4. Invariants motivating this ticket: `INV-009` — "Meaningful state changes require events" — `INV-010` — "Every event needs a cause model" (explainable through preceding state, actor intention, need pressure, belief, routine, affordance, …) — and `INV-011` — "Current-state-only simulation is forbidden". Each new event must carry its cause ancestry (the intention/need/routine that produced it); passive sub-threshold need drift may be batched/reconstructible (Spec §15.2, §8.1) but threshold crossings and decision-contributing changes must be eventful.
5. Deterministic-replay surface: event ordering, schema/version fields, and canonical serialization feed replay and the agent-state checksum (ticket 006; `INV-018`). This ticket introduces no nondeterminism — new kinds reuse the existing ordering-key/version envelope, payloads derive canonical serialization from the 001–004 structured types, and no wall-clock or hash-iteration input enters the canonical form. Replay/checksum enforcement is ticket 006 (cited); this ticket guarantees the kinds are versioned and ordered.

## Architecture Check

1. Extending the single `EventKind` enum (rather than a parallel cognition event log) keeps Phase 3A inside the one append-only causal log and the existing ordering/version discipline, so replay, projections, and checksum see cognition events as first-class — a side-channel cognition log would reintroduce the current-state-only shortcut `INV-011` forbids. Carrying the cause ancestry on each variant payload satisfies `INV-010` at the type level.
2. No backwards-compatibility shims: additive enum variants with new apply arms; no existing kind is renamed or repurposed, and the envelope/version scheme is unchanged (additive per `INV-020`).

## Verification Layers

1. Meaningful changes eventful (`INV-009`/`INV-011`) -> codebase grep-proof + unit test: a need threshold crossing, intention transition, routine step transition, and stuck diagnostic each emit a typed event; no apply path mutates need/intention/routine state outside an event.
2. Cause model (`INV-010`) -> unit test: each new event payload carries its producing-cause ancestry (intention/need/routine handle); a constructor cannot build a causeless cognition event.
3. Determinism (`INV-018`, substrate-only) -> unit test: new events round-trip canonically and order by the existing ordering keys; replay/checksum enforcement is deferred to ticket 006 (cited).

## What to Change

### 1. Event kinds

Extend `EventKind` in `crates/tracewake-core/src/events/envelope.rs` with the §15.2 variants, each carrying a structured payload referencing the 001–004 types (need kind+delta+cause, threshold crossing, candidate-goal-set evaluation handle, intention transition+reason, routine step transition, decision-trace handle, sleep/eat/work lifecycle, food consumption/service use, continue-routine outcome, stuck diagnostic, no-human day markers). Reuse the existing version/schema marker bump discipline.

### 2. Event application

Add apply arms in `crates/tracewake-core/src/events/apply.rs` that mutate the agent-state slice (need values, intention status, routine execution progress) only via these events. Passive need drift is applied as a reconstructible windowed delta event (Spec §15.2 batching allowance), never an out-of-band mutation. Where agent state must be threaded, extend the authoritative state container (`crates/tracewake-core/src/state.rs`) with an agent-state slice keyed by actor.

### 3. Module wiring

Update `crates/tracewake-core/src/events/mod.rs` re-exports as needed so the new kinds are part of the public event surface.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify — add Phase 3A `EventKind` variants + payloads)
- `crates/tracewake-core/src/events/apply.rs` (modify — add apply arms mutating the agent-state slice)
- `crates/tracewake-core/src/events/mod.rs` (modify — re-export new surface if needed)
- `crates/tracewake-core/src/state.rs` (modify — add per-actor agent-state slice threaded alongside physical/epistemic state)

## Out of Scope

- Replay rebuild, checksum, and projection extension for the new kinds (ticket 006).
- The behaviors that emit these events — need effects/wait, sleep/eat/work, continue, generation, runner (tickets 007–017).
- Content schema/validation (tickets 015, 016).

## Acceptance Criteria

### Tests That Must Pass

1. Applying a need-delta event mutates the actor's `NeedState` by the clamped delta and records the cause; applying an intention-transition event advances `IntentionStatus` with its reason.
2. Each new `EventKind` variant round-trips through canonical serialization and orders by the existing ordering keys; a causeless cognition-event construction is rejected.
3. `cargo test -p tracewake-core` passes with the extended `EventKind` and apply arms (existing Phase 1/2A event tests still green — additive change).

### Invariants

1. No agent-state mutation occurs outside an applied event (`INV-009`, `INV-011`).
2. Every cognition event carries a cause ancestry; the envelope version/ordering discipline is unchanged and additive (`INV-010`, `INV-020`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/apply.rs` (unit tests) — apply arms for need/intention/routine/sleep/eat/work/diagnostic kinds.
2. `crates/tracewake-core/src/events/envelope.rs` (unit tests) — round-trip and ordering for new kinds; causeless-construction rejection.

### Commands

1. `cargo test -p tracewake-core events`
2. `cargo test -p tracewake-core` (whole-crate, to confirm additive change keeps Phase 1/2A event coverage green)
3. Core-crate scope is correct; cross-crate replay determinism is exercised by ticket 006 and the capstone (025).
