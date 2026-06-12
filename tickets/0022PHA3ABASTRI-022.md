# 0022PHA3ABASTRI-022: Mutation baseline focused tests for trace vocabulary and deserialization

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — focused trace parsing tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned the remaining `agent/trace.rs` baseline
misses to this ticket. These mutants affect decision outcome, responsible-layer,
blocker-code, stuck-status, need decoding, diagnostic parsing, and canonical
deserialization.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags all
   `crates/tracewake-core/src/agent/trace.rs` baseline entries as
   `warrants-test:0022PHA3ABASTRI-022`.
2. The shared boundary under audit is typed decision and stuck-diagnostic trace
   serialization.
3. INV-041 requires inspectable decision traces; INV-022 forbids prose from becoming
   authoritative state.

## Architecture Check

1. Focused parser and round-trip tests prove the trace vocabulary directly.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Diagnostic vocabulary -> parser tests for every canonical trace token.
2. Replay robustness -> canonical deserialization negative tests.
3. Prose boundary -> typed stuck-diagnostic parse tests.

## What to Change

Add focused tests that kill the `trace.rs` baseline mutants assigned in the ledger.

## Files to Touch

- `crates/tracewake-core/src/agent/trace.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Content typed-diagnostic vocabulary cleanup owned by `0022PHA3ABASTRI-012`.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted trace tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. `cargo mutants -f crates/tracewake-core/src/agent/trace.rs --no-shuffle`

### Invariants

1. Trace diagnostics remain typed and parseable.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/trace.rs` — parser, round-trip, and diagnostic
   tests.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/trace.rs --no-shuffle`
