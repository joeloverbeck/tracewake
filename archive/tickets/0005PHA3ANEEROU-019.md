# 0005PHA3ANEEROU-019: Action-integration golden fixtures

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `ordinary_workday_001`, `sleep_eat_work_001`, `food_unavailable_replan_001`, and `routine_blocked_diagnostic_001` golden fixtures and registers them.
**Deps**: 0005PHA3ANEEROU-016, 0005PHA3ANEEROU-017

## Problem

Phase 3A requires focused golden fixtures proving the action families and the canonical interruption/blocker chains independently of the full day: a focused workday, the sleep/eat/work integration, the food-unavailable replan/fail chain, and a physical/access blocker diagnostic with no silent loop (Spec 0005 §17.1, §12; `INV-045`). These exercise tickets 007–017 end-to-end at fixture scope and seed the capstone's golden assertions.

## Assumption Reassessment (2026-06-07)

1. Golden fixtures live in `crates/tracewake-content/src/fixtures/` as one Rust file per fixture, declared in `fixtures/mod.rs` (`mod <name>;`) and surfaced through `all()` (`fixtures/mod.rs:1-13,72`); the golden runner is `crates/tracewake-content/tests/golden_fixtures_run.rs`. This ticket adds four fixture files and registers each in `mod.rs::all()`, following the existing structural-consumer model (registration is the wiring).
2. The Phase 3A schema (ticket 015), validation (ticket 016), runner (ticket 017), and the action/cognition machinery (007–014) must exist for these fixtures to load, validate, and run. Spec §17.1 fixes each fixture's required proof: `ordinary_workday_001` (one actor travels to work and completes or fails a work block for modeled reasons), `sleep_eat_work_001` (sleep/eat/work/wait/needs/durations/replay), `food_unavailable_replan_001` (hunger→EatMeal→missing food→replan/fail typed trace — the canonical interruption, Spec §12), `routine_blocked_diagnostic_001` (route/workplace/bed blocked→diagnostic, no silent loop).
3. Shared boundary under audit: all four fixtures register in `fixtures/mod.rs::all()` (shared file with tickets 020/021) and run through `golden_fixtures_run.rs`; their movement/action ancestry must be real (no teleport). Mara's food-unavailable chain (Spec §12 negative variant `food_unavailable_replan_001`) is the canonical interruption proof the capstone asserts.
4. Invariant motivating this ticket: `INV-045` — "Ordinary survival is causal" — and the §12 interruption requirement (food unavailable → replan or fail with typed diagnostic). The fixtures must prove the engine fails honestly: no hunger refill without food, no teleport to a workplace, no silent idle loop.
5. Deterministic-replay / no-scripting surface: each fixture is authored possibility (`INV-061`), validated by ticket 016 (no teleport/refill fields), and must replay deterministically. The fixtures add no leakage and no nondeterminism — they author initial state + routine assignments only; outcomes emerge from the engine. `food_unavailable_replan_001` proves a *negative* outcome (honest failure), not a scripted one.

## Architecture Check

1. Authoring focused fixtures (one concern each) rather than only the full day gives small, debuggable reviewable diffs that pin each action family and the interruption/blocker chains independently — a single mega-fixture would couple unrelated failures. Registration through the existing `all()` structural consumer keeps fixture wiring uniform.
2. No backwards-compatibility shims: additive fixture files + `mod.rs` registration; no change to existing fixtures.

## Verification Layers

1. Causal survival (`INV-045`) -> replay/golden-fixture check: `sleep_eat_work_001` shows sleep/eat/work changing needs only via duration/consumption events; `ordinary_workday_001` shows workplace presence via movement ancestry.
2. Honest interruption (§12) -> replay/golden-fixture check: `food_unavailable_replan_001` runs hunger→EatMeal→missing food→replan-or-fail with a typed trace (no refill, no teleport).
3. No silent loop (§8.8) -> replay/golden-fixture check: `routine_blocked_diagnostic_001` produces a typed physical/access blocker diagnostic rather than a silent idle loop.

## What to Change

### 1. Fixture files

Add under `crates/tracewake-content/src/fixtures/`: `ordinary_workday_001.rs`, `sleep_eat_work_001.rs`, `food_unavailable_replan_001.rs`, `routine_blocked_diagnostic_001.rs`, each authoring the actors/places/food/work/routine-assignment/needs/day-windows required for its proof (Spec §17.1), with real movement/action ancestry and no teleport/refill fields.

### 2. Registration + golden run

Declare each new module in `crates/tracewake-content/src/fixtures/mod.rs` and add it to `all()`; extend `crates/tracewake-content/tests/golden_fixtures_run.rs` with the per-fixture proof assertions.

## Files to Touch

- `crates/tracewake-content/src/fixtures/ordinary_workday_001.rs` (new)
- `crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs` (new)
- `crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs` (new)
- `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — declare + register the four fixtures in `all()`)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — per-fixture proof assertions)

## Out of Scope

- The doctrine-guard fixtures (planner_trace/no_teleport/possession_no_reset/no_hidden_truth, ticket 020).
- The canonical `no_human_day_001` fixture (ticket 021).
- Core action/runner logic (tickets 007–017) — these fixtures exercise it.
- TUI/debug surfaces (tickets 022, 023).

## Acceptance Criteria

### Tests That Must Pass

1. `ordinary_workday_001` shows workplace presence via movement ancestry and completes or fails a work block for a modeled reason; `sleep_eat_work_001` shows sleep/eat/work changing needs via duration/consumption events and replays deterministically.
2. `food_unavailable_replan_001` runs the hunger→EatMeal→missing-food→replan-or-fail chain with a typed trace, no hunger refill, no teleport.
3. `routine_blocked_diagnostic_001` produces a typed physical/access blocker diagnostic with no silent idle loop; all four fixtures load, validate (ticket 016), and run.

### Invariants

1. Need changes occur only via duration/consumption events; no teleport or refill (`INV-045`).
2. The food-unavailable chain proves honest failure, not a scripted outcome (`INV-060`/§12).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify) — per-fixture proof assertions for the four fixtures.
2. `crates/tracewake-content/tests/fixtures_load.rs` (modify) — the four fixtures load and validate.

### Commands

1. `cargo test -p tracewake-content golden_fixtures_run`
2. `cargo test -p tracewake-content`
3. Content-crate scope is correct; the canonical day and full acceptance are tickets 021/025.

## Outcome

Completed on 2026-06-07.

- Added and registered `ordinary_workday_001`, `sleep_eat_work_001`, `food_unavailable_replan_001`, and `routine_blocked_diagnostic_001`.
- Added Phase 3A fixture helper constructors for needs, homes, sleep places, food supplies, workplaces, routine templates/assignments, and day windows.
- Extended `fixtures_load` to assert all 17 registered fixtures, including the four Phase 3A action-integration fixtures.
- Extended `golden_fixtures_run` with shared action-pipeline helpers and focused fixture assertions:
  - `ordinary_workday_001` moves to the workplace before work start/completion.
  - `sleep_eat_work_001` logs sleep/eat/work need effects and replays deterministically.
  - `food_unavailable_replan_001` records typed `EatFailed` without refill/consumption.
  - `routine_blocked_diagnostic_001` records typed `WorkBlockFailed` for closed access without silent success.

Verification run:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-content --test golden_fixtures_run`
3. `cargo test -p tracewake-content --test fixtures_load`
4. `cargo test -p tracewake-content golden_fixtures_run` (passes, but this filter selects 0 tests; the explicit `--test` command above runs the assertions)
5. `cargo test -p tracewake-content`
