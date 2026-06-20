# 0043ORDLIFCER-007: Kill wait autonomous-origin survivor from the completed mutation campaign

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — focused anti-regression test for `crates/tracewake-core/src/actions/defs/wait.rs`; production change only if required by the test.
**Deps**: 0043ORDLIFCER-004

## Problem

The completed 0043 configured mutation campaign exposed one missed mutant in `is_autonomous_wait`: `replace is_autonomous_wait -> bool with true`. The behavioral gap is that user-origin wait proposals must not be treated as scheduler/agent autonomous waits for candidate-goal reevaluation. Only `ProposalOrigin::Scheduler` and `ProposalOrigin::Agent` may count as autonomous.

## Assumption Reassessment (2026-06-20)

1. The final `-004` campaign completed the full configured denominator and recorded the wait survivor in `reports/0043_ord_life_cert_mutation_final_missed.txt`.
2. The live code computes `candidate_goal_reevaluation` from threshold crossing plus `is_autonomous_wait(proposal)` in `crates/tracewake-core/src/actions/defs/wait.rs`.
3. Cross-artifact boundary under audit: the mutation evidence in `reports/0043_ord_life_cert_mutation_triage_register.md` names this as a behavior-relevant ORD-LIFE wait/need-accounting survivor that must be killed before the replacement capstone can pass.
4. Invariant motivation: INV-098 and ORD-LIFE wait/need accounting require actor/user request provenance to remain distinct from autonomous scheduler/agent behavior.
5. Enforcement surface: this ticket touches the action definition/test surface only; it must not weaken deterministic replay, proposal ancestry, or single-charge need accounting.

## Architecture Check

1. A direct user-origin wait witness is cleaner than testing only accepted waits generically, because the survivor is specifically the origin-classification predicate.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. Proposal ancestry/origin -> focused core test that a user-origin wait with need threshold crossing keeps `candidate_goal_reevaluation=false`.
2. Mutation survivor closure -> focused cargo-mutants run for `actions/defs/wait.rs` showing the `is_autonomous_wait -> true` identity is caught or no longer generated because the behavior is explicit.
3. Workspace health -> relevant targeted core tests plus final series gates later.

## What to Change

### 1. Add user-origin wait negative control

Add or extend a core anti-regression test so a user-origin wait proposal that emits need events and crosses a threshold still records `candidate_goal_reevaluation=false`.

### 2. Verify the survivor identity

Run a focused cargo-mutants command for `crates/tracewake-core/src/actions/defs/wait.rs` and confirm the final-run `is_autonomous_wait -> true` identity no longer survives.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify only if the focused test exposes a production defect beyond the survivor gap)

## Out of Scope

- The movement endpoint survivor cluster in `actions/defs/movement.rs` (handled by `0043ORDLIFCER-006`).
- Re-running the full configured mutation campaign or authoring the replacement acceptance artifact (handled after survivor kills).

## Acceptance Criteria

### Tests That Must Pass

1. A targeted core test proves user-origin wait does not set candidate-goal reevaluation even when need-threshold events are emitted.
2. Focused cargo-mutants evidence for `actions/defs/wait.rs` catches the final-run wait identity or proves it is no longer generated because the tested behavior was made explicit.
3. `cargo test --locked -p tracewake-core --test anti_regression_guards user_origin_wait_keeps_candidate_goal_reevaluation_false` passes for the new witness.

### Invariants

1. User-origin wait remains distinct from scheduler/agent autonomous wait.
2. Wait events and any emitted need events retain proposal ancestry and single-charge accounting semantics.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — user-origin wait reevaluation negative control for the completed-run survivor.

### Commands

1. `cargo test --locked -p tracewake-core --test anti_regression_guards user_origin_wait_keeps_candidate_goal_reevaluation_false`
2. `cargo mutants --workspace --no-shuffle -F 'crates/tracewake-core/src/actions/defs/wait\.rs'`
