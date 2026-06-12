# 0022PHA3ABASTRI-015: Mutation baseline focused tests for action pipeline decisions

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — focused action-pipeline regression tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned the remaining `actions/pipeline.rs`
baseline misses to this ticket. These mutants affect ordinary-agent validation,
controller binding, source-context validation, and semantic-action matching.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` now tags all
   `crates/tracewake-core/src/actions/pipeline.rs` baseline entries as
   `warrants-test:0022PHA3ABASTRI-015`.
2. The shared boundary under audit is the action proposal validation pipeline in
   `crates/tracewake-core/src/actions/pipeline.rs`.
3. INV-007 and INV-043 require player and autonomous actions to share ordinary-agent
   validation without controller privilege.

## Architecture Check

1. Focused behavior tests are cleaner than broad mutation-baseline acceptance because
   they assert the semantic rejection/acceptance boundary directly.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Action parity -> focused behavior tests for controller binding and actor-enabled
   decisions.
2. Source-context integrity -> focused rejection tests for source-context mismatch.
3. Semantic action integrity -> focused test proving action kind mismatch rejects.

## What to Change

Add or strengthen focused tests that kill the `actions/pipeline.rs` baseline mutants
assigned in the ledger.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Mutation debt in other source files.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted action-pipeline tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. `cargo mutants -f crates/tracewake-core/src/actions/pipeline.rs --no-shuffle`

### Invariants

1. Player and no-human action validation remain equivalent.
2. Baseline entries are removed only after the focused tests kill them.

## Outcome

Completed: 2026-06-12

- Added focused action-pipeline tests for body-exclusive detection, actor-enabled
  lookup, inactive scope rejection, controller binding, source-context staleness,
  source-context semantic matching, and inert query-only knowledge slots.
- Added a public anti-regression source-context guard for raw action-id semantic
  prefixes such as `inspect_place.shop_front`.
- Removed the seven assigned raw `actions/pipeline.rs` mutation baseline misses;
  the normalized baseline is now 137 entries with
  `fnv1a64=977cce46b241e47b`.
- Verification:
  - `cargo test -p tracewake-core`
  - `cargo mutants -f crates/tracewake-core/src/actions/pipeline.rs --no-shuffle`
    (`68 mutants tested in 3m: 49 caught, 19 unviable`)
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace`

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/pipeline.rs` — focused validation and
   controller-binding tests.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/actions/pipeline.rs --no-shuffle`
