# 0022PHA3ABASTRI-020: Mutation baseline focused tests for need parsing and threshold state

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — focused need-model tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned the remaining `agent/need.rs` baseline
misses to this ticket. These mutants affect need vocabulary parsing, threshold
crossing parsing/state, deserialization checks, and diagnostics.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags all
   `crates/tracewake-core/src/agent/need.rs` baseline entries as
   `warrants-test:0022PHA3ABASTRI-020`.
2. The shared boundary under audit is need-state parsing and threshold-crossing state.
3. INV-045 requires survival needs to remain causal and inspectable.

## Architecture Check

1. Focused parsing and threshold tests give direct evidence for the need model.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Need vocabulary -> parser tests for every canonical value.
2. Threshold causality -> threshold-crossing state tests.
3. Replay/schema robustness -> canonical deserialization negative tests.

## What to Change

Add focused tests that kill the `need.rs` baseline mutants assigned in the ledger.

## Files to Touch

- `crates/tracewake-core/src/agent/need.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Scheduler threshold-event baseline debt.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted need tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. `cargo mutants -f crates/tracewake-core/src/agent/need.rs --no-shuffle`

### Invariants

1. Need vocabulary and threshold state remain typed.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/need.rs` — parser, crossing, and diagnostic tests.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/need.rs --no-shuffle`
