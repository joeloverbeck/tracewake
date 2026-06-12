# 0022PHA3ABASTRI-021: Mutation baseline focused tests for no-human planning and scheduler routines

**Status**: COMPLETED
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

## Completion Notes (2026-06-12)

Implemented focused tests for the no-human active-intention surface, planner
sleep/work preconditions, exact scheduler witness lookups, pending completion
ticks, body-exclusive future-completion checks, stuck diagnostic boundaries, due
sleep/work start filters, window progress recording, marker idempotence, routine
failure reasons, and stuck diagnostic stable vocabulary. The scheduler advancement
loops now use finite tick ranges with debug assertions so a broken tick advance
cannot hang the test suite.

Retired the ticket-owned mutation baseline rows. The normalized baseline is now
`normalized-count=44 fnv1a64=28297e34c777adc7`.

Verification:

1. `cargo test -p tracewake-core --lib agent::no_human_surface::tests`
2. `cargo test -p tracewake-core --lib agent::planner::tests`
3. `cargo test -p tracewake-core --lib scheduler::`
4. `cargo mutants -f crates/tracewake-core/src/agent/no_human_surface.rs --no-shuffle` — `16 mutants tested in 71s: 12 caught, 4 unviable`
5. `cargo mutants -f crates/tracewake-core/src/agent/planner.rs --no-shuffle` — `22 mutants tested in 76s: 12 caught, 10 unviable`
6. `cargo mutants -f crates/tracewake-core/src/scheduler.rs --no-shuffle` — `203 mutants tested in 12m: 170 caught, 33 unviable`
7. `cargo test -p tracewake-core --test anti_regression_guards mutation_baseline_misses_are_pinned_and_ledgered`
8. `cargo fmt --all --check`
9. `cargo clippy --workspace --all-targets -- -D warnings`
10. `cargo build --workspace --all-targets --locked`
11. `cargo test --workspace`
