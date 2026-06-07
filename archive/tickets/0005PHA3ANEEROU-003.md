# 0005PHA3ANEEROU-003: Routine template, execution, and step model

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds `RoutineTemplate`, `RoutineExecution`, `RoutineFamily`, and the bounded `RoutineStep` vocabulary to the `agent` module.
**Deps**: 0005PHA3ANEEROU-002

## Problem

Phase 3A routines are defeasible durable intentions encoded as authored HTN-style methods that propose ordinary actions — never scripts that teleport, force success, or continue silently after a failed precondition (Spec 0005 §8.5, §8.6, §3.6; `INV-035`, `INV-036`). The routine template (authored method) and routine execution (actor-specific runtime) types, plus the small concrete step vocabulary, are the contract the routine selector (ticket 013), the planner (ticket 014), the content schema (ticket 015), and events (ticket 005) reference, so they land before any routine behavior.

## Assumption Reassessment (2026-06-07)

1. The intention model this binds to lands in ticket 002 (`crates/tracewake-core/src/agent/intention.rs`); `Intention.selected_routine` references a routine handle defined here. `RoutineTemplateId`/`RoutineExecutionId` land in ticket 001's `ids.rs` extension.
2. Spec §8.5 fixes the template fields (family/kind, applicability, preconditions, steps, expected duration/band, interruption points, failure modes, fallback/replanning rules, debug labels, reservable resources, per-step action-proposal strategy) and the execution fields (actor, template, current step, step status, start/last-progress tick, expected next-progress/deadline tick, concrete action ancestry, reserved resource, fallback attempts, failure/interruption state, trace ID). Spec §8.6 fixes the minimal step vocabulary; spec §11.5 names the required families (`MorningWake`, `EatMeal`, `GoToWork`, `WorkBlock`, `ReturnHome`, `SleepNight`, `FindFood`, `ContinueCurrentIntention`, `Wait`/`IdleWithReason`).
3. Shared boundary under audit: `RoutineFamily` and `RoutineStep` are dispatch vocabularies consumed by ticket 013 (method selection authors the templates), ticket 014 (planner expands steps to actions), ticket 005 (routine-step events), and ticket 015 (content routine schema). The family set and step kinds are fixed here.
4. Invariants motivating this ticket: `INV-035` — "Routines are defeasible intentions" (can fail, be interrupted, suspended, resumed, abandoned) — and `INV-036` — "HTN methods are procedures, not story scripts" (state-dependent methods with failure, interruption, alternatives, costs, traces). The model must therefore force every template to carry at least one failure mode and declared interruption points, and forbid any `RoutineStep` variant that directly mutates location/item/need (Spec §8.5: "No routine step may directly move an actor, directly move an item, directly alter hunger/fatigue, or directly complete work").
5. Deterministic-replay surface (substrate-only): routine execution state must replay-reconstruct (Spec §15.1). This ticket introduces no nondeterminism — step ordering is the template's authored sequence, IDs are stable, and execution state derives equality from structured fields. The enforcing surface (deterministic routine-step replay) lands in ticket 006; this ticket guarantees the step vocabulary contains no direct-mutation variant a later surface would have to forbid.

## Architecture Check

1. Splitting the authored `RoutineTemplate` (possibility) from the per-actor `RoutineExecution` (runtime) keeps content as possibility-not-script (`INV-061`/`INV-036`) while giving the runtime its own failure/interruption/fallback state — a single fused struct would either bake actor state into authored content or lose the template's reusability. A closed `RoutineStep` enum whose every variant proposes an action or waits-with-reason (no `SetLocation`/`SetNeed` variant) makes the no-teleport/no-refill rule structurally unrepresentable rather than merely validated.
2. No backwards-compatibility shims: greenfield types; the per-step action-proposal strategy references existing `SemanticActionId` rather than inventing a parallel action handle.

## Verification Layers

1. Defeasible routines (`INV-035`) -> unit test: a `RoutineExecution` transitions through start→{interrupted, suspended, resumed, failed, abandoned, completed}; a template missing a failure mode is rejected by the constructor/validator hook.
2. HTN-not-script (`INV-036`) -> codebase grep-proof + manual review: `RoutineStep` has no variant that sets location/item/need directly; every variant maps to an action proposal or a typed wait.
3. Determinism (`INV-018`, substrate-only) -> unit test: routine execution state round-trips canonically and step ordering follows the authored sequence. Full step-replay enforcement deferred to ticket 006 (cited).

## What to Change

### 1. Routine family + step vocabulary

Add `crates/tracewake-core/src/agent/routine.rs` with `RoutineFamily` (the §11.5 families) and a closed `RoutineStep` enum covering the §8.6 minimal vocabulary: select-known-place, move-toward-place, open/check-known-container, consume-accessible-food, start-scheduled-sleep, complete-scheduled-sleep, start-work-block, complete-work-block, wait-until(window/resource/event), continue-current-step, fallback-to-find-food, fail-with-typed-diagnostic. No variant carries a direct state mutation; each carries the `SemanticActionId` (or typed wait reason) it proposes.

### 2. Routine template + execution

In the same module, add `RoutineTemplate` (the §8.5 authored fields) with a constructor/validation hook requiring ≥1 failure mode, ≥1 declared interruption point, and positive bounded durations; and `RoutineExecution` (the §8.5 runtime fields) with step-status transitions, fallback-attempt tracking, and reserved-resource handle. Reservations reference an opaque resource handle (bed/workstation) resolved later; this ticket only models the slot.

### 3. Module registration

Add `pub mod routine;` to `crates/tracewake-core/src/agent/mod.rs` and re-export the public surface.

## Files to Touch

- `crates/tracewake-core/src/agent/routine.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; add `pub mod routine;`)

## Out of Scope

- Decision trace and stuck diagnostic models (ticket 004).
- Authored routine method content and HTN selection behavior (ticket 013).
- Bounded local planning that expands steps to concrete actions (ticket 014).
- Reservation conflict enforcement (ticket 011).
- Content routine schema and validation (tickets 015, 016).

## Acceptance Criteria

### Tests That Must Pass

1. A `RoutineTemplate` with zero failure modes, zero interruption points, or a non-positive duration is rejected by its constructor/validation hook.
2. `RoutineExecution` step-status transitions record start/last-progress ticks and a failure/interruption reason; a fallback attempt increments the tracked count.
3. `RoutineStep` round-trips canonically and exposes the `SemanticActionId`/typed-wait it proposes for every variant.

### Invariants

1. No `RoutineStep` variant directly mutates actor location, item location, or need value (`INV-036`).
2. Every routine template carries at least one failure mode and declared interruption points; routines are defeasible (`INV-035`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/routine.rs` (unit tests) — template validation rejection, execution transitions, step round-trip and no-direct-mutation audit.

### Commands

1. `cargo test -p tracewake-core agent::routine`
2. Core-crate unit scope is correct because routine selection (013), planner (014), and content schema (015) consumers land later.
3. A grep-proof (`agent::routine` enum has no `Set*`/`Move*Direct` variant) is the correct no-teleport boundary at the type level; runtime no-teleport is exercised in ticket 020's fixture.

## Outcome

Completed: 2026-06-07

What changed:
- Added `agent::routine` with `RoutineFamily`, closed `RoutineStep` vocabulary, `RoutineStepProposal`, `RoutineTemplate`, `RoutineExecution`, and `RoutineStepStatus`.
- Added template validation requiring steps, at least one failure mode, at least one interruption point, positive bounded durations, valid duration ordering, and interruption points that reference authored steps.
- Added runtime execution helpers for starting, waiting, completing, failing, interrupting, suspending, resuming, abandoning, action ancestry, and fallback-attempt tracking.
- Added canonical `RoutineStep` serialization/deserialization and tests for proposal exposure across every step variant.

Deviations from original plan:
- `FailWithTypedDiagnostic` exposes a typed diagnostic proposal rather than a semantic action or wait reason, preserving the spec's failure vocabulary while keeping direct mutation unrepresentable.
- Resource reservations are modeled as opaque string handles only; conflict enforcement remains deferred to the reservation/action tickets.

Verification results:
- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core agent::routine` passed.
