# 0022PHA3ABASTRI-016: Mutation baseline focused tests for eat access and payloads

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — focused `eat.rs` tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned the remaining `actions/defs/eat.rs`
baseline misses to this ticket. These mutants affect food access checks and
food-consumption location payload evidence.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags all
   `crates/tracewake-core/src/actions/defs/eat.rs` baseline entries as
   `warrants-test:0022PHA3ABASTRI-016`.
2. The shared boundary under audit is the ordinary eat action definition and its
   access-failure evidence.
3. INV-045 and INV-055 require food access, custody, and consumption to remain causal
   and inspectable.

## Architecture Check

1. Focused tests should assert each access branch and payload source instead of
   accepting mutation misses by rationale.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Access parity -> branch tests for local, carried, and held-container food.
2. Event evidence -> payload assertions for `location_payload`.

## What to Change

Add focused tests that kill the `eat.rs` baseline mutants assigned in the ledger.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/eat.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- The eat threshold-crossing emitter fix owned by `0022PHA3ABASTRI-007`.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted eat tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. `cargo mutants -f crates/tracewake-core/src/actions/defs/eat.rs --no-shuffle`

### Invariants

1. Food access remains modeled through state and custody, not prose.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/eat.rs` — focused access and payload tests.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/actions/defs/eat.rs --no-shuffle`
