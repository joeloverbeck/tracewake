# 0043ORDLIFCER-006: Kill movement door endpoint survivor cluster from the completed mutation campaign

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — focused anti-regression tests for `crates/tracewake-core/src/actions/defs/movement.rs`; production change only if required by the tests.
**Deps**: 0043ORDLIFCER-004

## Problem

The completed 0043 configured mutation campaign exposed four missed mutants in `build_move_event`'s connecting-door endpoint predicate. They all preserve or corrupt the same behavioral gap: a movement action must only treat a door as connecting the actor's current place to the requested destination when the two door endpoints exactly match that unordered pair. Partial endpoint matches, inverted predicates, or one-sided matches must not authorize movement.

## Assumption Reassessment (2026-06-20)

1. The final `-004` campaign completed the full configured denominator and recorded these four movement misses in `reports/0043_ord_life_cert_mutation_final_missed.txt`.
2. The live code authorizes door-backed adjacency in `crates/tracewake-core/src/actions/defs/movement.rs::build_move_event` with a two-orientation endpoint predicate over `state.doors`.
3. Cross-artifact boundary under audit: the mutation evidence in `reports/0043_ord_life_cert_mutation_triage_register.md` names these as behavior-relevant ORD-LIFE movement/reachability survivors that must be killed before the replacement capstone can pass.
4. Invariant motivation: INV-098 and the ORD-LIFE no-direct-dispatch/movement ancestry contract require movement to be physically reachable and eventful, not accepted by a malformed door predicate.
5. Enforcement surface: this ticket touches the action definition/test surface only; it must not weaken deterministic replay, actor-knowledge filtering, or proposal ancestry.

## Architecture Check

1. A focused behavioral witness around endpoint mismatch is cleaner than mutating the production predicate directly without proving the rejected cases.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. Physical reachability -> focused core test that rejects partial and inverted door endpoint matches.
2. Mutation survivor closure -> focused cargo-mutants run for `actions/defs/movement.rs` showing the four listed identities are caught or otherwise no longer generated because the same gap is covered.
3. Workspace health -> relevant targeted core tests plus final series gates later.

## What to Change

### 1. Add movement endpoint negative controls

Add or extend a core anti-regression test so a move proposal from place A to place B is rejected when the only door has one matching endpoint and one nonmatching endpoint, in both orientations needed to kill the four mutants.

### 2. Verify the survivor identities

Run a focused cargo-mutants command for `crates/tracewake-core/src/actions/defs/movement.rs` and confirm the four `build_move_event` identities from `reports/0043_ord_life_cert_mutation_final_missed.txt` no longer survive.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/src/actions/defs/movement.rs` (modify only if the focused tests expose a production defect beyond the survivor gap)

## Out of Scope

- The wait-origin survivor in `actions/defs/wait.rs` (handled by `0043ORDLIFCER-007`).
- Re-running the full configured mutation campaign or authoring the replacement acceptance artifact (handled after survivor kills).

## Acceptance Criteria

### Tests That Must Pass

1. A targeted core test rejects movement through partial and inverted door endpoint matches.
2. Focused cargo-mutants evidence for `actions/defs/movement.rs` catches the four final-run movement identities or proves they are no longer generated because the tested behavior was made explicit.
3. `cargo test --locked -p tracewake-core --test anti_regression_guards movement_door_endpoint_mismatch_rejects_partial_connections` passes for the new witness.

### Invariants

1. Movement only commits after physical reachability is established from the actor's current place to the requested destination.
2. Accepted movement events retain proposal ancestry; rejected movement commits no event.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — movement endpoint mismatch witness for the completed-run survivor cluster.

### Commands

1. `cargo test --locked -p tracewake-core --test anti_regression_guards movement_door_endpoint_mismatch_rejects_partial_connections`
2. `cargo mutants --workspace --no-shuffle -F 'crates/tracewake-core/src/actions/defs/movement\.rs'`
