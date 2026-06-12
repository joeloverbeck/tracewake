# 0022PHA3ABASTRI-018: Mutation baseline focused tests for candidate and decision selection

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — focused candidate, generation, and decision tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned baseline misses in
`agent/candidate.rs`, `agent/decision.rs`, and `agent/generation.rs` to this ticket.
These mutants affect stable-id vocabularies, candidate generation, actor-known notes,
goal selection, and durability scoring.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags the candidate, decision, and
   generation entries as `warrants-test:0022PHA3ABASTRI-018`.
2. The shared boundary under audit is candidate generation and decision trace
   selection.
3. INV-032 through INV-041 require inspectable symbolic decisions and stable
   diagnostic traces.

## Architecture Check

1. Focused tests on decision outputs and stable ids provide behavior evidence instead
   of accepting mutation misses in the baseline.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Symbolic cognition -> focused tests for generated candidate coverage.
2. Trace stability -> stable-id and durability-scoring assertions.

## What to Change

Add focused tests that kill the assigned candidate, decision, and generation mutants.

## Files to Touch

- `crates/tracewake-core/src/agent/candidate.rs` (modify tests)
- `crates/tracewake-core/src/agent/decision.rs` (modify tests)
- `crates/tracewake-core/src/agent/generation.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Routine and HTN parser baseline debt.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted candidate/decision/generation tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. Targeted `cargo mutants -f` runs for the three assigned source files.

### Invariants

1. Decision traces remain stable and inspectable.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. Candidate, decision, and generation module tests — focused stable-id, scoring, and
   candidate coverage assertions.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/candidate.rs --no-shuffle`
3. `cargo mutants -f crates/tracewake-core/src/agent/decision.rs --no-shuffle`
4. `cargo mutants -f crates/tracewake-core/src/agent/generation.rs --no-shuffle`
