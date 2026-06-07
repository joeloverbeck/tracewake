# 0005PHA3ANEEROU-004: Decision-trace and stuck-diagnostic model

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds `DecisionTrace`, `BlockerCategory`, and `StuckDiagnostic` types to the `agent` module.
**Deps**: 0005PHA3ANEEROU-002, 0005PHA3ANEEROU-003

## Problem

Phase 3A must explain every salient autonomous decision and name every stuck actor aggressively — a no-human run with silent freezing, unreasoned idle loops, or no replayable explanation must fail (Spec 0005 §8.7, §8.8, §3.7; `INV-041`). Decision traces and the typed stuck-diagnostic vocabulary are the inspection contract the generation engine (012), routine selector (013), planner (014), runner (017), debug surface (023), and capstone (025) all emit/consume, so the data model lands before any emitter.

## Assumption Reassessment (2026-06-07)

1. The candidate-goal/intention model (ticket 002) and routine model (ticket 003) provide the structures a trace references (candidate goals considered, selected method, routine step). `DecisionTraceId`/`StuckDiagnosticId` land in ticket 001's `ids.rs` extension. The existing `crates/tracewake-core/src/debug_reports.rs` is the debug-projection surface ticket 023 will render these through; this ticket only defines the data.
2. Spec §8.7 fixes the trace fields (trace ID, actor, tick/window, active needs+bands, current intention before decision, candidate goals considered, selected goal+method, rejected goals/methods/actions+reasons, beliefs/perceptions/known places used, action proposal attempted, validation result, fallback considered, hidden-truth audit result, outcome). Spec §8.8 fixes the diagnostic fields and the exact blocker categories: physical, access, knowledge, resource, social/norm placeholder, scheduling/reservation, unsupported-action, planner-budget-exhausted, fixture-authoring-error.
3. Shared boundary under audit: `BlockerCategory` is a closed dispatch vocabulary consumed by ticket 005 (diagnostic events), ticket 017 (runner emission), ticket 023 (debug rendering), and ticket 025 (every category must be producible/serializable — §18.1 Diagnostics row). The category set is fixed here.
4. Invariant motivating this ticket: `INV-041` — "Agent decisions need debug traces" (debug must show beliefs used, needs/duties considered, active intention, selected method/plan, alternatives rejected, blocked preconditions, interruptions, resulting events). The trace must capture all of these as structured fields; it is a read-only record — debugging the trace must never create observations or mutate actor knowledge (Spec §8.7, §3.7).
5. Actor-knowledge / deterministic-replay surface: the trace carries a `hidden_truth_audit_result` field that records the planner did not read ground truth (Spec §3.3, §8.7), and traces must be replay-safe (Spec §15.1). This ticket introduces no leakage and no nondeterminism — the trace stores actor-known inputs plus a debug-only detail section that is never fed back into the actor-knowledge path, and IDs/fields are canonical. The no-leak enforcement (debug truth never reaches embodied view) lands in ticket 022/025; deterministic trace replay lands in ticket 006; this ticket guarantees the record shape supports both.

## Architecture Check

1. A single structured `DecisionTrace` capturing inputs→candidates→selection→rejections→outcome makes "NPC chose X" unacceptable by construction (Spec §25.5) — a free-text log would lose the per-field inspectability acceptance tests need. A closed `BlockerCategory` enum gives compile-time exhaustiveness so every stuck path is typed, not a stringly "stuck" flag. Keeping the trace a pure data record (no mutators on actor state) structurally enforces "debug visibility is allowed; debug authority is not" (`INV-008`-adjacent, Spec §3.7).
2. No backwards-compatibility shims: greenfield types reusing existing need/candidate/intention/routine references; the trace's debug-only section is a separate field, not a leak path into embodied projection.

## Verification Layers

1. Decision traces (`INV-041`) -> invariants alignment check + unit test: `DecisionTrace` exposes every §8.7 field; a constructed trace for a food-failure decision carries the candidate goals, selected method, rejected fallback, and outcome.
2. Debug-not-authority (`INV-008`; Spec §3.7) -> codebase grep-proof: `DecisionTrace`/`StuckDiagnostic` have no method that mutates `NeedState`, beliefs, or world state; they are read-only records.
3. No hidden-truth leakage (Spec §3.3) -> manual review: the trace's `hidden_truth_audit_result` records actor-known boundary compliance; the debug-only detail field is never read by actor decision paths. Full no-leak enforcement deferred to tickets 022/025 (cited).

## What to Change

### 1. Decision trace

Add `crates/tracewake-core/src/agent/trace.rs` with `DecisionTrace` carrying the §8.7 fields. It references `CandidateGoal`, `Intention`, `RoutineFamily`/`RoutineExecution`, and `NeedPressure` by value/handle. Include `DecisionOutcome` (continued, switched, waited, replanned, failed, completed) and a `hidden_truth_audit_result` (e.g. `HiddenTruthAudit { actor_known_only: bool, notes }`). Provide canonical serialization and a render path; no mutator touches actor state.

### 2. Stuck diagnostic

In the same module, add `BlockerCategory` (the nine §8.8 categories, exhaustive) and `StuckDiagnostic` carrying the §8.8 fields (actor, tick/window, active need/goal/intention, routine/method/step, attempted action/proposal, blocker category, concrete blocker, actor-known explanation, debug-only details, retry/abandon/fallback outcome, resulting status). Provide canonical serialization.

### 3. Module registration

Add `pub mod trace;` to `crates/tracewake-core/src/agent/mod.rs` and re-export the public surface.

## Files to Touch

- `crates/tracewake-core/src/agent/trace.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; add `pub mod trace;`)

## Out of Scope

- Emitting traces/diagnostics from decisions or the runner (tickets 012, 013, 014, 017).
- Trace/diagnostic event payloads and replay (tickets 005, 006).
- Debug rendering and `debug planner`/`debug stuck` commands (ticket 023).
- The no-leak/possession acceptance proofs (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. A `DecisionTrace` constructed for a food-unavailable decision exposes candidate goals considered, selected method, the rejected fallback with reason, the hidden-truth audit result, and a `DecisionOutcome`.
2. Each of the nine `BlockerCategory` variants constructs a `StuckDiagnostic` that serializes canonically and round-trips.
3. A grep-proof confirms neither type exposes a method mutating `NeedState`, beliefs, or world state.

### Invariants

1. Traces and diagnostics are read-only records; inspecting them creates no observation and mutates no actor knowledge (`INV-008`; Spec §3.7).
2. Every stuck path is one of the nine typed `BlockerCategory` values, never an untyped "stuck" flag (`INV-041`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/trace.rs` (unit tests) — trace field coverage, all nine blocker categories serialize, read-only audit.

### Commands

1. `cargo test -p tracewake-core agent::trace`
2. Core-crate unit scope is correct because trace/diagnostic emitters (012–017) and the debug renderer (023) land later; this ticket pins the record shape.
3. A grep-proof that `agent::trace` has no `&mut` actor-state mutator is the correct debug-not-authority boundary at the type level.

## Outcome

Completed: 2026-06-07

What changed:
- Added `agent::trace` with `DecisionTrace`, `DecisionOutcome`, `HiddenTruthAudit`, and `RejectedDecisionItem`.
- Added the closed nine-variant `BlockerCategory` vocabulary plus `StuckDiagnostic` and `StuckResultingStatus`.
- Added canonical serialization for decision traces and canonical serialize/deserialize round-trips for stuck diagnostics.
- Registered and re-exported the trace/diagnostic public surface from the `agent` module.

Deviations from original plan:
- Decision trace canonical serialization currently captures the stable summary fields needed for substrate replay/debug anchoring; full event/projection payload serialization remains deferred to tickets 005/006/023.
- `FailWithTypedDiagnostic` routine-step payloads inside stuck diagnostics are serialized through the routine-step canonical format rather than duplicating a second encoding.

Verification results:
- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core agent::trace` passed.
