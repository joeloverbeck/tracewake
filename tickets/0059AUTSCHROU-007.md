# 0059AUTSCHROU-007: Kill focused mutation survivors before acceptance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds or strengthens tests/guards over the 0059 scheduler routine-family, transaction, and generation seams
**Deps**: 0059AUTSCHROU-005

## Problem

0059AUTSCHROU-005 produced the required focused mutation ledger, but the focused no-config runs were survivorful: 77 mutants tested, 53 missed, 14 caught, and 10 unviable. Several missed mutants are directly in the 0059 authority seam, including scheduler routine-family binding, transaction fallback filtering, and `routine_window_goal_matches_active_intention`. 0059 cannot claim a passing focused mutation result while these survivors remain unexplained.

## Assumption Reassessment (2026-07-01)

1. The survivor ledger is recorded in `archive/tickets/0059AUTSCHROU-005.md` after 005 archives, with exact `target/mutants/0059-*-focused/mutants.out/missed.txt` paths for the run artifacts.
2. Spec `specs/0059_…_SPEC.md` §8.4 requires survivorful focused mutation to be non-pass unless every survivor is equivalent or unviable with accepted evidence.
3. Shared boundary under audit: 0059's active-intention authority over autonomous routine-family derivation and routine-window hint consumption.
4. Motivating invariant: **INV-098 — Feature acceptance is harsh**; a survivorful mutation run cannot be accepted as green by aggregate test success.
5. Enforcement surface: new or strengthened witnesses must preserve actor-known-only cognition, fail-closed diagnostics, and deterministic replay.

## Architecture Check

1. Add the missing witnesses in the behavioral/guard layer rather than changing production code only to satisfy mutation. A surviving mutant should either be killed by a meaningful witness or explicitly disposed as equivalent/unviable.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. INV-098 -> focused mutation rerun: the 0059 no-config focused runs must be non-survivorful or every survivor must have accepted equivalent/unviable disposition.
2. INV-103/INV-104 -> behavior tests/guards: scheduler routine-family cannot bypass active intention, and transaction/generation cannot admit routine-window authority without compatible active intention.

## What to Change

### 1. Add survivor-killing witnesses

Use the 0059AUTSCHROU-005 missed lists to add focused tests or strengthen existing guards for real survivors in:

- `no_human::routine_window_family`
- `no_human::eligible_routine_execution_for_actor`
- `ActorDecisionTransaction::run`
- `routine_window_goal_matches_active_intention`

### 2. Rerun and disposition mutation

Rerun the focused no-config scheduler, transaction, and generation commands from 0059AUTSCHROU-005. Record exact counts and any remaining survivor disposition.

## Files to Touch

- `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` (modify, likely)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify, if source/guard witnesses are the right fit)
- Other focused test files only if a survivor belongs outside those surfaces

## Out of Scope

- Production refactors unrelated to killing or disposing the named survivors.
- Final 0059 acceptance artifact wording; 006 consumes the post-007 mutation result.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted tests added for the real 0059 survivors.
2. The three focused no-config cargo-mutants runs from 0059AUTSCHROU-005 are rerun and recorded.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No unexplained survivor remains in the active-intention routine-family authority seam.
2. Any survivor not killed is explicitly equivalent/unviable with concrete evidence.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/scheduler_routine_derivation_authority.rs` — add behavioral witnesses for routine-family survivor classes.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — strengthen source/behavior tripwires if needed for source-shaped survivors.

### Commands

1. `cargo mutants --no-config --package tracewake-core --file crates/tracewake-core/src/scheduler.rs --re 'due_loaded_actor_ids|transact_world_one_tick|routine.*family|eligible.*routine|ActorDecisionTransactionInput' --cargo-arg --locked --output target/mutants/0059-scheduler-focused -- -p tracewake-core --test scheduler_routine_derivation_authority`
2. `cargo mutants --no-config --package tracewake-core --file crates/tracewake-core/src/agent/transaction.rs --re 'active_intention_for_actor|goal_for_routine_family|ActorDecisionTransaction::run|routine_window_family' --cargo-arg --locked --output target/mutants/0059-transaction-focused -- -p tracewake-core --test scheduler_routine_derivation_authority`
3. `cargo mutants --no-config --package tracewake-core --file crates/tracewake-core/src/agent/generation.rs --re 'routine_window_goal|RoutineDuty|generate_candidate_goals|ContinueCurrentIntention' --cargo-arg --locked --output target/mutants/0059-generation-focused -- -p tracewake-core --test scheduler_routine_derivation_authority`
