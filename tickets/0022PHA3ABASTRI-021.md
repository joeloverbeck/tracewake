# 0022PHA3ABASTRI-021: Mutation baseline focused tests for no-human planning and scheduler routines

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — focused no-human, planner, and scheduler tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned baseline misses in
`agent/no_human_surface.rs`, `agent/planner.rs`, and `scheduler.rs` to this ticket.
These mutants affect active-intention projection, planner preconditions, perception
lookups, duration completion, stuck diagnostics, marker idempotence, and no-human
tick-ledger lookup.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags the no-human surface, planner,
   and scheduler entries as `warrants-test:0022PHA3ABASTRI-021`.
2. The shared boundary under audit is the no-human ordinary-life scheduler path.
3. INV-004, INV-035, and INV-041 require autonomous ordinary-life behavior with
   inspectable decision traces.

## Architecture Check

1. Focused no-human and scheduler tests are the correct proof surface for ordinary
   autonomous behavior.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. No-human autonomy -> scheduler progression and completion tests.
2. Planner preconditions -> sleep/work plan rejection and acceptance tests.
3. Diagnostics -> stuck diagnostic stable-id and blocker tests.

## What to Change

Add focused tests that kill the assigned no-human, planner, and scheduler mutants.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify tests)
- `crates/tracewake-core/src/agent/planner.rs` (modify tests)
- `crates/tracewake-core/src/scheduler.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Scheduler typed-error conversion owned by `0022PHA3ABASTRI-008`.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted no-human/planner/scheduler tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. Targeted `cargo mutants -f` runs for the assigned source files.

### Invariants

1. No-human ordinary-life behavior remains autonomous and inspectable.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. No-human, planner, and scheduler module tests — focused progression,
   precondition, and diagnostic assertions.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/no_human_surface.rs --no-shuffle`
3. `cargo mutants -f crates/tracewake-core/src/agent/planner.rs --no-shuffle`
4. `cargo mutants -f crates/tracewake-core/src/scheduler.rs --no-shuffle`
