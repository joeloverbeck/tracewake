# 0005PHA3ANEEROU-020: Doctrine-guard golden fixtures

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `planner_trace_001`, `routine_no_teleport_001`, `possession_does_not_reset_intention_001`, and `no_hidden_truth_planning_001` golden fixtures and registers them.
**Deps**: 0005PHA3ANEEROU-016, 0005PHA3ANEEROU-017

## Problem

Phase 3A must prove its hardest doctrine guarantees with dedicated fixtures: a planner trace exposes candidate goals / selected method / rejected reasons with a clean hidden-truth audit; a routine cannot teleport (validator/runtime catches missing movement ancestry); possession does not reset an active intention; and hidden food/item truth cannot influence the plan even though the action may fail and explain (Spec 0005 §17.1, §3.3, §3.6; `INV-024`, `INV-006`). These are the negative/guard proofs the capstone leans on.

## Assumption Reassessment (2026-06-07)

1. Fixtures register in `crates/tracewake-content/src/fixtures/mod.rs::all()` and run through `golden_fixtures_run.rs`; possession-parity tooling exists from Phase 2A (`possession_parity_001.rs`, the `tui_acceptance`/possession surfaces). This ticket adds four fixture files following the same structural-consumer registration as ticket 019.
2. The planner + hidden-truth audit (ticket 014), routine model + no-direct-mutation steps (ticket 003), continue/possession (ticket 011), validation no-teleport rule (ticket 016), and runner (017) are the surfaces these fixtures target. Spec §17.1 fixes each proof: `planner_trace_001` (candidate goals, selected method, rejected reasons, hidden-truth audit), `routine_no_teleport_001` (routine requires movement/action ancestry; validator/runtime catches teleport), `possession_does_not_reset_intention_001` (bind/unbind/continue preserves active intention), `no_hidden_truth_planning_001` (hidden food/item truth cannot influence the plan; action may fail and explain).
3. Shared boundary under audit: these register in the shared `fixtures/mod.rs::all()` (with tickets 019/021). `routine_no_teleport_001` is the runtime counterpart to ticket 016's static no-teleport validation; `possession_does_not_reset_intention_001` exercises ticket 011's possession-parity path; `no_hidden_truth_planning_001` exercises ticket 014's planner boundary.
4. Invariants motivating this ticket: `INV-024` — "No telepathy" (hidden truth cannot reach the planner or the embodied view) — and `INV-006` — "Possession transfers no world knowledge" (and Spec §8.4: possession must not reset/copy/complete an intention). `INV-036` (routines are not scripts) backs the no-teleport guard.
5. Actor-knowledge / determinism surface: these are the no-leak and possession-parity proofs. `no_hidden_truth_planning_001` seeds a ground truth the actor does not believe and proves the planner's chosen plan is independent of it (the action may fail). `possession_does_not_reset_intention_001` binds/unbinds and proves the intention/needs survive. They add no leakage (they prove its absence) and replay deterministically.

## Architecture Check

1. Dedicated guard fixtures (one invariant each) give the capstone clean, isolated proofs for the highest-severity invariants (no-telepathy, possession parity, no-teleport) — folding them into the full day would make a failure ambiguous. The no-teleport runtime fixture complements the static validator rule (ticket 016) so both the authoring gate and the runtime are proven.
2. No backwards-compatibility shims: additive fixtures + `mod.rs` registration; reuses Phase 2A possession tooling rather than duplicating it.

## Verification Layers

1. No telepathy (`INV-024`) -> replay/golden-fixture check: `no_hidden_truth_planning_001` proves the planner's chosen plan does not depend on a hidden food/item truth the actor does not believe; the action may fail with a typed trace.
2. Possession parity (`INV-006`; Spec §8.4) -> replay/golden-fixture check + possession test: `possession_does_not_reset_intention_001` binds/unbinds and proves the active intention, routine execution, and needs survive; `continue_routine` resumes them.
3. No teleport (`INV-036`) -> schema validation + replay/golden-fixture check: `routine_no_teleport_001` fails validation or runtime acceptance when movement/action ancestry is missing; `planner_trace_001` exposes the full candidate/selected/rejected trace with a clean hidden-truth audit.

## What to Change

### 1. Fixture files

Add under `crates/tracewake-content/src/fixtures/`: `planner_trace_001.rs`, `routine_no_teleport_001.rs`, `possession_does_not_reset_intention_001.rs`, `no_hidden_truth_planning_001.rs`, each authoring exactly the state needed for its guard proof.

### 2. Registration + golden run

Declare each in `crates/tracewake-content/src/fixtures/mod.rs` and add to `all()`; extend `crates/tracewake-content/tests/golden_fixtures_run.rs` with the per-fixture guard assertions (and a possession-parity assertion path for `possession_does_not_reset_intention_001`).

## Files to Touch

- `crates/tracewake-content/src/fixtures/planner_trace_001.rs` (new)
- `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs` (new)
- `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — declare + register the four fixtures in `all()`)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — per-fixture guard assertions)

## Out of Scope

- The action-integration fixtures (ticket 019) and canonical day (ticket 021).
- Core planner/possession/validation logic (tickets 011, 014, 016) — these fixtures exercise it.
- TUI embodied no-leak rendering (ticket 022) — a separate surface; this ticket guards the planner/state, not the view.

## Acceptance Criteria

### Tests That Must Pass

1. `no_hidden_truth_planning_001`: a hidden food/item truth the actor does not believe does not change the planner's chosen plan; the action may fail with a typed trace.
2. `possession_does_not_reset_intention_001`: binding/unbinding preserves the active intention, routine execution, and needs; `continue_routine` resumes them.
3. `routine_no_teleport_001` fails validation/runtime when movement ancestry is missing; `planner_trace_001` exposes candidate goals, selected method, rejected reasons, and a clean hidden-truth audit.

### Invariants

1. Hidden truth never influences the planner or leaks to a viewer (`INV-024`).
2. Possession transfers no knowledge and does not reset intention/needs (`INV-006`, Spec §8.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify) — per-fixture guard assertions for the four fixtures.

### Commands

1. `cargo test -p tracewake-content golden_fixtures_run`
2. `cargo test -p tracewake-content`
3. Content-crate scope is correct; the possession-parity/no-leak end-to-end gate is the capstone (025).

## Outcome

Completed on 2026-06-07.

- Added and registered `planner_trace_001`, `routine_no_teleport_001`, `possession_does_not_reset_intention_001`, and `no_hidden_truth_planning_001`.
- Extended fixture load expectations to all 21 registered fixtures.
- Added golden-run guard assertions:
  - planner trace candidate/selection/rejection coverage with actor-known-only hidden-truth audit.
  - remote work fails without movement ancestry and does not teleport.
  - controller attach/detach preserves seeded active intention, routine execution, and needs, then `continue_routine` resumes through the shared pipeline.
  - hidden physical food stays out of planner inputs; forced action fails with typed `EatFailed` rather than consuming/refilling.

Verification run:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-content --test golden_fixtures_run`
3. `cargo test -p tracewake-content --test fixtures_load`
4. `cargo test -p tracewake-content golden_fixtures_run` (passes, but this filter selects 0 tests; the explicit `--test` command above runs the assertions)
5. `cargo test -p tracewake-content`
