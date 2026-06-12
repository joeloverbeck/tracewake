# 0024PHA3ACONSCH-010: Projection-policy closure — workplace accessibility gate and oracle de-hardcoding

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (`agent/no_human_surface.rs`, `epistemics/projection.rs` tests).
**Deps**: None

## Problem

Spec 0024 findings `ORD-HARD-147` (medium) and `ORD-HARD-156` (low):

- `147`: `add_workplace_knowledge` pushes `workplace_believed_accessible`
  unconditionally, while the food and sleep emitters gate the analogous fact on
  `accessibility_scope == FromAnyPlace` — so the policy table's workplace
  accessibility column is behaviorally inert, the behavioral test's workplace
  accessibility axis cannot fail for any table contents, and the row-mutation
  negative never mutates workplace accessibility.
- `156`: the policy oracle's `expected_embodied_presence` hand-derives
  `latest_current_place_record` via a hardcoded `current_place.as_str() ==
  "home_tomas"` and re-calls the production `includes_in_embodied_context`,
  masking a latest-record-derivation regression for the generic loop.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the ungated
   `workplace_believed_accessible` push in
   `agent/no_human_surface.rs::add_workplace_knowledge` versus the
   `FromAnyPlace`-gated food/sleep pushes; the hardcoded `"home_tomas"` equality in
   `epistemics/projection.rs::expected_embodied_presence`; the row-mutation
   negative (`actor_known_projection_policy_table_detects_synthetic_row_mutations`)
   mutating only `food_source.accessibility_scope`.
2. Verified against spec 0024 §4 (`ORD-HARD-147`/`156`) and 0023 §4
   `ORD-HARD-126`'s required correction (the residue this closes).
3. Cross-artifact boundary: the `actor_known_projection_kind_policy` table ↔
   no-human-surface contract — every policy column must be behaviorally live on
   every kind, with a mutation negative per axis.
4. Rule restated (R-29 self-echo): a policy assertion that compares a table against
   a surface that ignores the table cannot fail; the lock is decorative until the
   surface consumes the column.
5. Enforcement surface (actor-knowledge filtering): gating the workplace fact on
   `FromAnyPlace` is behavior-preserving today — the current table row declares
   `FromAnyPlace` for workplace (verified), so the fact remains emitted; the change
   makes the column live without repricing any fixture or golden. The oracle fix is
   test-side only. No leakage or replay surface is weakened; a future table edit
   flipping workplace accessibility now has a behavioral consequence and a firing
   negative.

## Architecture Check

1. Gating the workplace emitter to parity with food/sleep (rather than declaring
   the fact unconditional in the table) keeps one rule for all three kinds — the
   policy table stays the single source of accessibility semantics, and the
   row-mutation negative family extends uniformly. Deriving the oracle's
   latest-record flag from the real classifier output removes the
   fixture-coupled literal without re-running the production method against
   hand-computed inputs.
2. No backwards-compatibility aliasing/shims: no transitional unconditional path
   retained.

## Verification Layers

1. R-29 closure (`147`) → fourth synthetic row mutation flipping
   `workplace.accessibility_scope` to `None` produces a detected mismatch naming
   workplace accessibility (test).
2. Oracle independence (`156`) → the generic loop's expected value derives
   `latest_current_place_record` from `classified_actor_known_records_for_context`
   (real classifier output), not a place-string literal (codebase grep-proof: the
   `"home_tomas"` equality is gone from the oracle).
3. Behavior preservation → existing no-human-surface and golden tests pass
   unchanged (the workplace fact still emits under the current table).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Workplace accessibility gate (`147`)

`add_workplace_knowledge` gates `workplace_believed_accessible` on
`policy.accessibility_scope == FromAnyPlace` (parity with food/sleep).

### 2. Fourth row mutation

Extend `actor_known_projection_policy_table_detects_synthetic_row_mutations` with
the workplace-accessibility flip.

### 3. Oracle de-hardcoding (`156`)

`expected_embodied_presence` derives the latest-record flag from the real
classifier output; the `"home_tomas"` literal is removed.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify)

## Out of Scope

- Policy-table contents (no row's declared values change).
- The dedicated stale-path test (`stale_workplace_embodied_scope_mismatches`) —
  already independent; untouched.
- Closed-world context discrimination (ticket -011).

## Acceptance Criteria

### Tests That Must Pass

1. The workplace-accessibility row mutation fires a detected mismatch; the existing
   three mutations still fire.
2. The no-human surface emits `workplace_believed_accessible` under the current
   table (behavior preserved); flipping the table row in a synthetic changes
   surface output.
3. The oracle contains no fixture place literal; the generic policy loop still
   detects an embodied-scope regression (existing negatives green).
4. The four workspace gates pass.

### Invariants

1. Every policy-table column is behaviorally live on every kind, with a mutation
   negative per axis.
2. Test oracles derive expected values from production classifier outputs, not
   fixture literals re-running production methods.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (inline tests) — fourth row
   mutation; de-hardcoded oracle.
2. `crates/tracewake-core/src/agent/no_human_surface.rs` (inline tests) — gated
   workplace emission under both table poles.

### Commands

1. `cargo test -p tracewake-core --lib -- epistemics:: agent::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

- Gated `workplace_believed_accessible` on the workplace policy row's
  `FromAnyPlace` accessibility scope while preserving current table behavior.
- Added direct no-human surface coverage for workplace access under both
  accessibility poles and a workplace accessibility row-mutation negative in the
  projection policy table tests.
- Replaced the policy oracle's fixture-literal latest-record derivation with the
  classifier's own current/latest record flags.

Proof:

1. `cargo test -p tracewake-core --lib -- epistemics:: agent::`
2. `cargo fmt --all --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo test --workspace`
