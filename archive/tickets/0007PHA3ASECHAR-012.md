# 0007PHA3ASECHAR-012: Integrated no-human day capstone

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — capstone fixture/test only (`crates/tracewake-content/src/fixtures/no_human_day_001.rs`, `crates/tracewake-core/tests/acceptance_gates.rs`); no new production logic
**Deps**: 0007PHA3ASECHAR-004, 0007PHA3ASECHAR-005, 0007PHA3ASECHAR-006, 0007PHA3ASECHAR-007, 0007PHA3ASECHAR-008, 0007PHA3ASECHAR-009, 0007PHA3ASECHAR-010, 0007PHA3ASECHAR-011

## Problem

Phase 3A acceptance must be proven by a single integrated no-human run, not by manually forced post-run proposals (Spec 0007 D-09, §Integrated no-human day capstone, §Acceptance criteria, §Forbidden shortcuts). The capstone must run `run_no_human_day` exactly once and assert the full ordinary-life chain emerges from live needs, durable intentions, defeasible routines, actor-known planning, and shared validation — without injecting ordinary proposals after the run.

## Assumption Reassessment (2026-06-07)

1. Confirmed surfaces: `crates/tracewake-core/src/scheduler.rs:283` (`Scheduler::run_no_human_day`) and `crates/tracewake-tui/src/app.rs:247` (`App::run_no_human_day`); the capstone fixture `crates/tracewake-content/src/fixtures/no_human_day_001.rs` and the acceptance harness `crates/tracewake-core/tests/acceptance_gates.rs` exist. This ticket composes tickets 004–011 end-to-end; it adds no production logic (deliverable-doubles-as-capstone does not apply — the harness already exists).
2. Spec 0007 §Integrated no-human day capstone enumerates the required ancestry: deterministic stable order for ≥4 actors; passive need deltas before decisions; ≥1 fatigue-driven sleep with completion/recovery; ≥1 hunger-driven eat from actor-known food; ≥1 food failure/search/replan/wait-with-typed-reason when food is not actor-known; movement before remote work; a work block started/completed only after locality/access/assignment validation; a work failure/typed blocker when conditions fail; ≥1 active intention adopted and later continued/completed/failed/interrupted; routine step start/progress/failure/completion ancestry; no `Player`/controller-conditioned event; no actor silently freezing when unpossessed; no hidden food/route used by autonomous planning; replay rebuild of physical/agent/no-human-metrics/intention/routine/decision/stuck state. Binding constraint 10 forbids manually injected ordinary actions after `run_no_human_day`.
3. Cross-artifact boundary under audit: the capstone fixture (content) and the acceptance test (core) against the integrated scheduler path. The test re-enumerates expected counts from the fixture at test start rather than hardcoding.
4. Motivating invariants (restated): INV-004 "The authoritative world ignores human existence" — the chain emerges with no human controller; INV-091 "No-human tests are mandatory"; INV-098 "Feature acceptance is harsh" — done only when caused, agent-possible, eventful, epistemically bounded, replay-safe, no-human runnable, non-scripted, regression-tested.
5. Determinism + no-leak surface touched: the capstone asserts replay rebuild (live-vs-replay equality from 0007PHA3ASECHAR-010) and that no hidden food/route is used by autonomous planning (from 0007PHA3ASECHAR-004). It is acceptance-only; it introduces no new emission or projection. Counts are re-enumerated from the fixture, not hardcoded, to stay stable.

## Architecture Check

1. A single end-to-end assertion over one `run_no_human_day` is the only honest proof that the ordinary-life chain is autonomous rather than test-forced; re-enumerating counts from the fixture keeps it from going stale. It exercises tickets 004–011 without duplicating their logic.
2. No backwards-compatibility aliasing/shims: no post-run ordinary proposals are injected; the capstone reads only what the autonomous run emitted.

## Verification Layers

1. INV-004 / INV-091 (no-human autonomy) -> replay/golden-fixture check: `run_no_human_day` runs once and the required ordinary-life ancestry is present with no manually injected post-run proposals.
2. INV-024 (no hidden-truth planning) -> replay/golden-fixture check: the run uses no hidden food/route (negative assertion).
3. INV-018 (replay) -> replay/golden-fixture check: replay rebuilds physical/agent/no-human-metrics/intention/routine/decision/stuck state for the capstone run.
4. INV-098 (harsh acceptance) -> manual review: the acceptance criteria enumerate the §Acceptance-criteria bullets as mapped sub-cases.

## What to Change

### 1. Capstone fixture

Create or replace `no_human_day_001` so a single `run_no_human_day` over ≥4 deterministically-ordered actors exercises the full chain (sleep, eat-from-actor-known-food, food failure/replan, movement-before-work, validated work block, work failure, intention adopt+continue, routine step ancestry).

### 2. Capstone acceptance test

In `acceptance_gates.rs`, assert each §Acceptance-criteria bullet as a sub-case over the single run, re-enumerating expected counts from the fixture; assert no `Player`/controller-conditioned event, no silent freeze, no hidden-truth use, and replay rebuild — with no post-run ordinary-proposal injection.

## Files to Touch

- `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify)

## Out of Scope

- All production logic for needs/intentions/routines/planning/replay/TUI (0007PHA3ASECHAR-004…011) — the capstone exercises them, it does not modify them.
- Status/ledger documentation (0007PHA3ASECHAR-013).

## Acceptance Criteria

### Tests That Must Pass

1. The capstone runs `run_no_human_day` exactly once and asserts: deterministic ≥4-actor order; passive need deltas before decisions; fatigue-driven sleep + recovery; hunger-driven eat from actor-known food; food failure/replan/wait-with-typed-reason when food is not actor-known; movement before remote work; validated work block start/complete; work failure/typed blocker on failed conditions; an active intention adopted and continued/completed/failed/interrupted; routine step start/progress/failure/completion ancestry.
2. No `Player`/controller-conditioned event; no actor silently freezes when unpossessed; no hidden food/route used by autonomous planning.
3. Replay rebuilds physical/agent/no-human-metrics/intention/routine/decision/stuck state for the run.
4. No manually injected ordinary proposal after `run_no_human_day`.
5. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. The full ordinary-life chain emerges from the single autonomous run, not from post-run injection.
2. Expected counts are re-enumerated from the fixture, not hardcoded.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/no_human_day_001.rs` — the integrated capstone fixture.
2. `crates/tracewake-core/tests/acceptance_gates.rs` — the single-run capstone assertion with fixture-derived counts.

### Commands

1. `cargo test -p tracewake-core --test acceptance_gates`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo fmt --all --check`

## Outcome

Completed: 2026-06-07

Changed behavior:
- Added a single-run integrated no-human capstone in `acceptance_gates.rs` that constructs a deterministic multi-actor ordinary-day scenario and asserts passive needs before decisions, autonomous eating, no-food waiting, sleep completion, movement before work, work completion/failure, intention continuation, routine ancestry, no controller/player leakage, metrics, and replayed physical/agent/intention/routine/decision/stuck state.
- `run_no_human_day` now schedules in-run completion events for sleep and work starts whose expected completion ticks fall inside the run, using the existing sleep/work completion event builders.
- `run_no_human_day` now records deterministic decision-trace events for autonomous proposals so replay rebuilds decision-trace state from the log.
- Updated TUI no-human assertions to the new post-run need row after work completion costs are applied.

Deviations:
- The ticket expected capstone fixture/test only, but the capstone exposed that duration-based sleep/work completions and decision trace state were not yet emitted inside the autonomous run. I added narrow production scheduler hooks so the single run, not post-run manual injection, produces the required evidence.
- The core acceptance test uses a core-only capstone scenario rather than importing `tracewake-content`, preserving the workspace dependency direction.

Verification:
- `cargo test -p tracewake-core --test acceptance_gates integrated_no_human_day_capstone_emerges_from_one_autonomous_run`
- `cargo test -p tracewake-core --test acceptance_gates`
- `cargo test -p tracewake-content --test golden_fixtures_run no_human_day`
- `cargo test -p tracewake-core no_human_day`
- `cargo test -p tracewake-tui --test command_loop_session no_human_day_command_loop_renders_phase3a_behavior_rows`
- `cargo test -p tracewake-tui --test tui_acceptance tui_runs_no_human_day_and_inspects_real_post_run_panels`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
