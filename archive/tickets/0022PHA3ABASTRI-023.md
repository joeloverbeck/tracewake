# 0022PHA3ABASTRI-023: Mutation baseline focused tests for transaction, projection, and perception surfaces

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — focused transaction/projection/perception tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned baseline misses in `agent/transaction.rs`,
`projections.rs`, and `agent/perception.rs` to this ticket. These mutants affect
witness-kind compatibility, stuck diagnostics, active-intention lookup, embodied
semantic actions, no-human metrics, diagnostic projections, player-conditioned fact
scans, planner-failure classification, and typed visibility.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags the transaction, projection,
   and perception entries as `warrants-test:0022PHA3ABASTRI-023`.
2. The shared boundary under audit is transaction witness compatibility plus
   actor-facing projection/perception.
3. INV-002, INV-008, INV-024, and INV-068 require holder-known projection and
   embodied visibility to remain actor-filtered and debug-safe.

## Architecture Check

1. Focused tests across transaction and projection surfaces prove the behavior that
   the baseline currently accepts as test debt.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Transaction provenance -> witness-kind and stuck-diagnostic tests.
2. Projection filtering -> semantic-action, metric, and player-conditioned scan tests.
3. Perception visibility -> typed visibility tests.

## What to Change

Add focused tests that kill the assigned transaction, projection, and perception
mutants.

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify tests)
- `crates/tracewake-core/src/projections.rs` (modify tests)
- `crates/tracewake-core/src/agent/perception.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Embodied debug-render split owned by `0022PHA3ABASTRI-009`.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted transaction/projection/perception tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. Targeted `cargo mutants -f` runs for the assigned source files.

### Invariants

1. Holder-known projection remains actor-filtered and typed.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. Transaction, projection, and perception module tests — focused witness,
   projection, and visibility assertions.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/transaction.rs --no-shuffle`
3. `cargo mutants -f crates/tracewake-core/src/projections.rs --no-shuffle`
4. `cargo mutants -f crates/tracewake-core/src/agent/perception.rs --no-shuffle`

## Outcome

Completed: 2026-06-12

Implemented focused transaction, projection, and perception tests for the final
baseline rows. Transaction coverage now proves actor-specific active-intention
lookup, no-human/need witness compatibility, stuck-diagnostic blocker mapping, and
a fail-closed method-selection retry guard. Projection coverage now proves known
sleep affordance projection, disabled empty-food semantic actions, typed planner
failure metrics, replay-failure event filtering, player-conditioned scans, and
semantic-action proposal parameter branches. Perception coverage now directly proves
typed visible-exit filtering.

Retired the final mutation baseline rows. The normalized baseline is now
`normalized-count=0 fnv1a64=cbf29ce484222325`.

Verification:

1. `cargo test -p tracewake-core --lib agent::transaction::tests`
2. `cargo test -p tracewake-core --lib projections::tests`
3. `cargo test -p tracewake-core --lib agent::perception::tests`
4. `cargo mutants -f crates/tracewake-core/src/agent/transaction.rs --no-shuffle` — `70 mutants tested in 4m: 56 caught, 14 unviable`
5. `cargo mutants -f crates/tracewake-core/src/projections.rs --no-shuffle` — `106 mutants tested in 6m: 85 caught, 21 unviable`
6. `cargo mutants -f crates/tracewake-core/src/agent/perception.rs --no-shuffle` — `21 mutants tested in 2m: 16 caught, 5 unviable`
7. `cargo test -p tracewake-core --test anti_regression_guards mutation_baseline_misses_are_pinned_and_ledgered`
8. `cargo test -p tracewake-core --test anti_regression_guards`
9. `cargo test -p tracewake-core`
10. `cargo fmt --all --check`
11. `cargo clippy --workspace --all-targets -- -D warnings`
12. `cargo build --workspace --all-targets --locked`
13. `cargo test --workspace`
