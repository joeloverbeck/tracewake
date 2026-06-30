# 0058EMBROUCON-001: Active-intention current-step authority for embodied continuation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `crates/tracewake-core/src/runtime/session.rs` (`embodied_routine_window_family`); adds core behavioral tests
**Deps**: None

## Problem

The embodied `Continue routine` follow-on selects the routine family from an actor-known workplace fact *before* it consults the actor's single authoritative active intention (spec §3.2 H-0058-03, §4.1 F-0058-01 — the OQ1 violation). `embodied_routine_window_family` returns `RoutineFamily::WorkBlock` whenever the actor's current place is a known workplace, overriding the active intention's current unresolved step. That is a routine-authority leak: an environmental fact, not the active intention, picks the routine family. The fix derives the family solely from the active intention's bound unresolved execution, with actor-known context allowed only to refine/validate (never override).

## Assumption Reassessment (2026-06-30)

1. `embodied_routine_window_family` at `crates/tracewake-core/src/runtime/session.rs:1122` — confirmed it checks `actor_known_context.known_workplaces()` against `current_place_id()` and returns `RoutineFamily::WorkBlock` (lines 1127-1133) *before* reading `agent_state.active_intention_by_actor()` (1134). Its test `embodied_routine_window_family_returns_work_block_at_known_workplace` (1784) asserts `WorkBlock` on an empty `AgentState::default()`; siblings `embodied_routine_window_family_filters_executions_by_actor` (1818) and `embodied_routine_window_family_ignores_resolved_executions` (1853) exercise the post-branch execution scan and remain load-bearing.
2. Spec §4.1 F-0058-01 and §3.2 H-0058-03 require removing the pre-intention workplace branch; the in-session `/reassess-spec` finding I1 additionally requires preserving the legitimate `WorkBlock↔GoToWork` distinction. The autonomous baseline `scheduler.rs::routine_window_family` (`crates/tracewake-core/src/scheduler.rs:3240`) is the correct mirror: it uses known-workplace knowledge only to *downgrade* `WorkBlock`→`GoToWork` when the actor is **not** at a known workplace (lines 3248-3254).
3. Cross-artifact boundary under audit: the routine-family selector shared between the embodied path (`session.rs`) and the autonomous path (`scheduler.rs`). The authority is the single active intention (`crates/tracewake-core/src/state.rs:415` `active_intention_by_actor` + `intentions`); the family helper consumes `crate::agent::ActorKnownPlanningContext` and `AgentState` only.
4. Invariants under audit: INV-035 (routines are defeasible intentions, not puppet strings), INV-104 (routines/needs may not bypass candidate generation, actor-known context, local planning), INV-101/102 (sealed/provenanced actor-known context), INV-094 (possession parity). The current workplace branch lets an environmental fact override the active intention, violating INV-035 and INV-104.
5. Enforcement surface: `embodied_routine_window_family` feeds `routine_window_family` into `ActorDecisionTransaction::run`, which adds it as an additive `routine_window_goal` candidate via `goal_for_routine_family` (`crates/tracewake-core/src/agent/transaction.rs:132`). This is an actor-known cognition surface, not a hidden-truth one — the helper reads only `ActorKnownPlanningContext` + `AgentState`. Removing the branch introduces no epistemic-leakage path (still actor-known only) and no replay nondeterminism (derivation is over deterministic `agent_state`); confirm no hidden-truth read is added.
6. Schema/shape change: no schema shape change (the N/A pole) — this modifies internal control flow in a private fn in `runtime/session.rs`. It reshapes no public enum, struct, event payload, or view-model field; `RoutineFamily` is unchanged, so there are no schema consumers to update. (The explicit N/A pole — `session.rs` is a runtime-authority surface that trips the reseal heuristic, but no reseal occurs here.)

## Architecture Check

1. Deriving the family from the active intention's bound unresolved execution — not an environmental shortcut — is the doctrine-correct authority order (INV-035/104) and makes the embodied path converge with the autonomous baseline, which is what enables the -004 metamorphic equivalence proof. Preserving the `WorkBlock↔GoToWork` refinement keeps the legitimate "am I at the workplace yet, or must I still travel?" distinction the autonomous baseline already has.
2. No backwards-compatibility aliasing/shims: the workplace branch is deleted outright (not wrapped behind a flag or fallback), and the obsoleted test is inverted, not kept green.

## Verification Layers

1. INV-035/104 (routine authority) → behavioral test `embodied_continue_uses_active_intention_current_step_not_known_workplace` (replay/behavioral check).
2. INV-101/102 (actor-known firewall) → manual epistemic-leakage review + the helper's existing actor-known audit coverage: no test path reads hidden truth.
3. INV-035/098 (marker non-progress) → event-log test `continue_routine_marker_alone_does_not_advance_routine_or_progress`.
4. INV-094 (possession parity) is proved by the cross-ticket metamorphic equivalence in -004 (this ticket makes parity *achievable* by removing the fork; the equivalence proof surface lives in -004 — not collapsed here).

## What to Change

### 1. Rewrite `embodied_routine_window_family`

Remove the early `known_workplaces()` → `WorkBlock` branch (lines 1127-1133). Derive the family via `active_intention_by_actor` → active intention record → `selected_routine_method` → bound unresolved execution (preserve the existing 1134-1145 execution-scan logic). Return `None` (typed-stuck signal) when that chain is absent or ambiguous. Apply the `WorkBlock↔GoToWork` workplace-known refinement — mirroring `scheduler.rs:3248-3254` — only *after* the active intention has selected the family, never before it.

### 2. Behavioral tests

Add `embodied_continue_uses_active_intention_current_step_not_known_workplace`, `embodied_continue_assigned_inactive_window_does_not_drive_follow_on`, and `continue_routine_marker_alone_does_not_advance_routine_or_progress`. Invert or replace `embodied_routine_window_family_returns_work_block_at_known_workplace` (it asserts the removed behavior and bakes its mutant-kill rationale into the deleted branch) rather than adapting it to keep the old branch green; preserve the two sibling tests.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify)

## Out of Scope

- The temporal gateway for time-advancing follow-ons (F-0058-02 → -002).
- Stuck-diagnostic / receipt honesty (F-0058-03 → -003).
- The embodied/autonomous metamorphic equivalence proof (F-0058-04 → -004).
- Changing the autonomous `scheduler.rs::routine_window_family` derivation — §2.2 defers it; it is only consumed and named here as the parity baseline.

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_continue_uses_active_intention_current_step_not_known_workplace` — an actor at a known workplace with an active non-work routine step resolves a follow-on matching the active step, not `WorkBlock`.
2. `embodied_continue_assigned_inactive_window_does_not_drive_follow_on` — an assigned-but-inactive tempting execution is ignored; the follow-on uses the active intention's current step or produces typed stuck.
3. `continue_routine_marker_alone_does_not_advance_routine_or_progress` — the marker alone cannot increment routine progress, mutate the intention, move the actor, start work, or satisfy scheduler progress accounting.
4. `cargo test --workspace` passes.

### Invariants

1. Routine family/current step derives solely from the single authoritative active intention; actor-known context may refine/validate but never override (INV-035/104).
2. `embodied_routine_window_family` reads only `ActorKnownPlanningContext` + `AgentState` — no hidden-truth path (INV-101/102).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (test module) — three new behavioral tests plus inverting the obsoleted `embodied_routine_window_family_returns_work_block_at_known_workplace` test; siblings preserved.

### Commands

1. `cargo test -p tracewake-core embodied_continue`
2. `cargo test --workspace`
3. `cargo test -p tracewake-core embodied_routine_window_family` — confirms the inverted test and the two preserved siblings still pass (narrow boundary for the selector change).
