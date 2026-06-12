# 0022PHA3ABASTRI-017: Mutation baseline focused tests for actor-known context fields

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — focused actor-known context tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned the remaining `agent/actor_known.rs`
baseline misses to this ticket. These mutants affect known-container surfaces,
structured fact gaps, fact matching, ordering, and derived-field parsing.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags all
   `crates/tracewake-core/src/agent/actor_known.rs` baseline entries as
   `warrants-test:0022PHA3ABASTRI-017`.
2. The shared boundary under audit is `ActorKnownPlanningContext` and derived
   actor-known field parsing.
3. INV-002 and INV-024 require planning inputs to come from actor-known facts, not
   hidden truth.

## Architecture Check

1. Focused actor-known tests preserve the canonical context boundary more directly
   than accepting context-field mutants in the baseline.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Actor-known filtering -> focused tests for `has_fact` and known-container access.
2. Diagnostic integrity -> focused tests for structured gap reporting and parsing.

## What to Change

Add focused tests that kill the `actor_known.rs` baseline mutants assigned in the
ledger.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Planner hidden-truth adversarial restoration owned by `0022PHA3ABASTRI-005`.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted actor-known tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. `cargo mutants -f crates/tracewake-core/src/agent/actor_known.rs --no-shuffle`

### Invariants

1. Context facts remain actor-known and provenance-derived.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/actor_known.rs` — focused context tests.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/actor_known.rs --no-shuffle`
