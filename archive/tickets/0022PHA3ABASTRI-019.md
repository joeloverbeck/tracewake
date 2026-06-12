# 0022PHA3ABASTRI-019: Mutation baseline focused tests for HTN, intention, methods, and routines

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — focused HTN/routine/intention tests in `tracewake-core`
**Deps**: `archive/tickets/0022PHA3ABASTRI-001.md`

## Problem

The 0022 mutation-baseline triage assigned baseline misses in `agent/htn.rs`,
`agent/intention.rs`, `agent/methods.rs`, and `agent/routine.rs` to this ticket.
These mutants affect routine-condition parsing, modeled fact resolution, routine
lifecycle transitions, method-step validation, and stable ids.

## Assumption Reassessment (2026-06-12)

1. `reports/0020_mutants_baseline_disposition.md` tags the HTN, intention, methods,
   and routine entries as `warrants-test:0022PHA3ABASTRI-019`.
2. The shared boundary under audit is routine and HTN procedure selection.
3. INV-034 through INV-037 require durable intentions and defeasible procedures rather
   than scripted outcomes.

## Architecture Check

1. Focused parser, lifecycle, and procedure tests are cleaner than treating procedure
   mutants as permanent baseline residents.
2. No backwards-compatibility aliasing/shims are introduced.

## Verification Layers

1. Procedure integrity -> routine condition parser and HTN condition tests.
2. Intention durability -> routine lifecycle transition tests.
3. Method validity -> proposal-step validation tests.

## What to Change

Add focused tests that kill the assigned HTN, intention, methods, and routine mutants.

## Files to Touch

- `crates/tracewake-core/src/agent/htn.rs` (modify tests)
- `crates/tracewake-core/src/agent/intention.rs` (modify tests)
- `crates/tracewake-core/src/agent/methods.rs` (modify tests)
- `crates/tracewake-core/src/agent/routine.rs` (modify tests)
- `reports/0020_mutants_baseline_disposition.md` (eventual tag retirement)
- `.cargo/mutants-baseline-misses.txt` (eventual entry retirement)

## Out of Scope

- Scheduler no-human routine baseline debt.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted HTN/routine/intention/method tests kill the ledger-assigned mutants.
2. `cargo test -p tracewake-core`
3. Targeted `cargo mutants -f` runs for the assigned source files.

### Invariants

1. Routines remain defeasible and state-dependent.
2. Baseline entries are removed only after the focused tests kill them.

## Test Plan

### New/Modified Tests

1. HTN, intention, methods, and routine module tests — focused parser and lifecycle
   assertions.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo mutants -f crates/tracewake-core/src/agent/htn.rs --no-shuffle`
3. `cargo mutants -f crates/tracewake-core/src/agent/intention.rs --no-shuffle`
4. `cargo mutants -f crates/tracewake-core/src/agent/methods.rs --no-shuffle`
5. `cargo mutants -f crates/tracewake-core/src/agent/routine.rs --no-shuffle`

## Outcome

Completed: 2026-06-12

- Added focused HTN condition-resolution tests for modeled food-search facts,
  search-surface availability, and known-route edge vs modeled-fact surfaces.
- Added intention stable-id coverage for every `IntentionStatus` variant and
  terminal classification.
- Added methods tests for negative empty-wait proposal validation and positive
  typed diagnostic proposal validation.
- Added routine tests for condition parser vocabulary, invalid canonical step
  shapes, parse-error rendering, and suspend/abandon lifecycle transitions.
- Retired twenty-eight HTN/intention/methods/routine mutation baseline entries
  after targeted mutation runs caught all viable assigned mutants.
- Mutation baseline normalized count/hash after retirement:
  `normalized-count=83 fnv1a64=a336ed7ea5c0ed12`.
- Verification:
  - `cargo test -p tracewake-core`
  - `cargo mutants -f crates/tracewake-core/src/agent/htn.rs --no-shuffle`
    (`26 mutants tested in 2m: 18 caught, 8 unviable`)
  - `cargo mutants -f crates/tracewake-core/src/agent/intention.rs --no-shuffle`
    (`19 mutants tested in 2m: 18 caught, 1 unviable`)
  - `cargo mutants -f crates/tracewake-core/src/agent/methods.rs --no-shuffle`
    (`18 mutants tested in 73s: 13 caught, 5 unviable`)
  - `cargo mutants -f crates/tracewake-core/src/agent/routine.rs --no-shuffle`
    (`71 mutants tested in 4m: 67 caught, 4 unviable`)
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace`
