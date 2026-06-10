# 0016PHA3ANEEACC-006: Severe-safety flight — LeaveUnsafePlace routine family with known-edge move

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-core` `RoutineFamily` variant + routine template; goal-mapping rewiring; two new golden fixtures
**Deps**: None

## Problem

ORD-HARD-017: severe safety pressure resolves to waiting, never flight. `agent/generation.rs` emits `GoalKind::LeaveUnsafePlace` at `GoalPriority::SevereSafety` (rank 1), but `agent/methods.rs::family_for_goal` maps it to `RoutineFamily::Wait` and `agent/transaction.rs::planner_goal_for` maps it to `PlannerGoal::WaitWithReason("actor_decision_reevaluation")`. The single highest-priority survival goal in the vocabulary plans an in-place wait — a pressure that cannot produce its own remedy is a label, not a pressure (INV-039; foundation doc 05; archived spec 0005 §8.1 "a pressure to avoid or leave obviously unsafe/inaccessible/unknown situations using actor-known information" and §11.1 "leave unsafe/blocked/unknown situation" as a required candidate goal with real behavior).

The fix: a `LeaveUnsafePlace` routine family whose step moves the actor along a *known* edge (actor-known routes only, INV-048) toward a known/believed-safer place; when no known edge exists, fall back to wait with a typed `knowledge` blocker and a stuck diagnostic — an explained inability, not a silent one.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `GoalKind::LeaveUnsafePlace` emitted at `GoalPriority::SevereSafety` (`agent/generation.rs` ~:123–133); `family_for_goal` maps it to `RoutineFamily::Wait` (`agent/methods.rs` ~:123); `planner_goal_for` maps it to `PlannerGoal::WaitWithReason("actor_decision_reevaluation")` (`agent/transaction.rs` ~:402). `RoutineFamily` (`agent/routine.rs:9–20`) has ten variants (MorningWake, EatMeal, GoToWork, WorkBlock, ReturnHome, SleepNight, FindFood, ContinueCurrentIntention, Wait, IdleWithReason) — no flight family; `GoToWork`/`ReturnHome` templates contain `move` steps and are the structural exemplars. `plan_route` (`agent/planner.rs`) reads only `state.known_edges()` of the planning context with typed `PlannerBudgetExhausted` and no truth fallback — the route discipline this family must reuse.
2. Spec/docs: spec 0016 §ORD-HARD-017 (evidence, required correction, structural lock); `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` §8.1 (:336) and §11.1 (:842); `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-039, INV-048.
3. Cross-layer boundary under audit: the goal→family→planner-goal mapping spans `generation.rs` (candidate emission, unchanged), `methods.rs` (family selection), and `transaction.rs` (planner-goal resolution) — all three must agree that severe safety plans movement, with the wait fallback reachable only through the typed no-known-edge branch.
4. INV-039 — pressures must be able to redirect behavior; a remedy-less pressure is a label. INV-048 — travel is causal: route choice from known routes, with departure/arrival events and costs through the ordinary pipeline. Restated before trusting the ticket narrative.
5. Actor-knowledge / deterministic-replay surface: the flight step consumes actor-known routes only (`known_edges` via the audited context — no truth fallback, preserving `believed_but_wrong_food_source_fails_without_truth_lookup`'s discipline for routes); both new fixtures replay byte-identically; the no-edge fallback emits a typed `knowledge` blocker + stuck diagnostic (INV-070/INV-105 typed diagnostics).
6. Schema extension: adds a `RoutineFamily` variant (and its routine template). Consumers: the family dispatch in `family_for_goal`, `phase3a_routine_templates()`, and any exhaustive `RoutineFamily` matches (grep at implementation; wildcard-free matches will fail compilation until extended — desired). Additive-only: no existing variant changes shape or meaning.

## Architecture Check

1. A dedicated routine family is the doctrine-shaped fix: routines provide defeasible method structure (INV-104), so flight belongs as a family whose template plans a `move` through the ordinary pipeline — not as a special-cased planner branch or a scheduler dispatch. Reusing `plan_route`/known-edge discipline keeps the truth firewall intact; the explicit typed fallback keeps "cannot flee" an explained state rather than a silent wait.
2. No backwards-compatibility aliasing/shims: the `LeaveUnsafePlace → Wait` mappings are replaced, not retained as fallback aliases; the wait fallback is a typed branch inside the new family, not the old mapping kept alive.

## Verification Layers

1. INV-039 (pressure produces remedy) → replay/golden-fixture check: `severe_safety_with_known_exit_produces_move_001` — severe safety + one known edge ⇒ committed `move` proposal with trace ancestry; replay byte-match.
2. INV-048 (known routes only) → fixture + unit test: the flight step plans only along context `known_edges`; no truth-adjacency fallback (negative assertion in the no-exit fixture).
3. INV-070/INV-105 (explained inability) → fixture `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`: typed `knowledge` blocker + stuck diagnostic recorded.
4. Method-selection coverage → unit test: `family_for_goal(LeaveUnsafePlace)` returns the new family (closing the gap where method-selection tests asserted only "a coherent proposal is produced").

## What to Change

### 1. `RoutineFamily::LeaveUnsafePlace` + template

New variant in `agent/routine.rs`; template in `agent/methods.rs::phase3a_routine_templates()` whose step moves along a known edge toward a known/believed-safer place (mirroring the `GoToWork`/`ReturnHome` move-step shape), with the typed no-known-edge fallback branch (wait + `knowledge` blocker + stuck diagnostic).

### 2. Mapping rewiring

`family_for_goal`: `LeaveUnsafePlace → RoutineFamily::LeaveUnsafePlace`. `planner_goal_for`: a movement planner goal toward the selected safer place (wait-with-reason only via the typed fallback).

### 3. Fixtures

`severe_safety_with_known_exit_produces_move_001` and `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, registered in `fixtures/mod.rs` and exercised by the golden runner with byte-identical replay.

## Files to Touch

- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/agent/methods.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/planner.rs` (modify — flight-step route planning, if the move step needs a dedicated planner goal)
- `crates/tracewake-content/src/fixtures/severe_safety_with_known_exit_produces_move_001.rs` (new)
- `crates/tracewake-content/src/fixtures/severe_safety_without_known_exit_waits_with_knowledge_blocker_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — run the new fixtures)

## Out of Scope

- Safety-pressure *generation* changes (`generation.rs` candidate emission is correct today).
- Cross-window stuck detection and wait-reason discipline (0016PHA3ANEEACC-010).
- Audit ordering in the transaction (`archive/tickets/0016PHA3ANEEACC-005.md` — shares `transaction.rs`; coordinate the mechanical merge).

## Acceptance Criteria

### Tests That Must Pass

1. `severe_safety_with_known_exit_produces_move_001`: committed `move` proposal with trace ancestry, live and under replay (byte-match).
2. `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`: wait with typed `knowledge` blocker and stuck diagnostic.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Severe-safety goal resolution plans movement when a known edge exists; the wait path is reachable only through the typed no-known-edge fallback.
2. Flight planning consumes actor-known routes only — no hidden-truth adjacency at any step (INV-048, INV-099).

## Test Plan

### New/Modified Tests

1. The two fixtures above — the ORD-HARD-017 structural locks.
2. `crates/tracewake-core/src/agent/methods.rs` — unit test asserting `family_for_goal(LeaveUnsafePlace)` selects the flight family and its planned `action_id` is `move` when an edge is known.

### Commands

1. `cargo test -p tracewake-core methods && cargo test -p tracewake-content golden`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
