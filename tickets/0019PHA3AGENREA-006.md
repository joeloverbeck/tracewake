# 0019PHA3AGENREA-006: Bidirectional witness census

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` test guard only (`tests/anti_regression_guards.rs`)
**Deps**: `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-049)

## Problem

The witness census enforces census⊆surface but not surface⊆census (`ORD-HARD-049`).
`guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms` iterates the
hand-maintained `NO_HUMAN_SURFACE_FACT_STABLE_IDS`, asserting each id appears in
`no_human_surface.rs` and has an explicit `witness_kind_allowed` arm — but nothing
extracts the stable_ids actually minted in `no_human_surface.rs` and asserts they are
all census members. A future fact minted without census registration gets no
CI-time witness-arm requirement; the runtime `_ => false` default still fails closed,
so this is a lock-durability gap, not a live leak.

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: `guard_018…` walks only the
   census constant (`NO_HUMAN_SURFACE_FACT_STABLE_IDS`); the guard body contains no
   extraction of minted stable_ids (absence re-proven by direct read of the guard
   region); all currently-minted ids were cross-checked present in the census during
   the 0019 audit, so the new direction starts green.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-049 (reassessed
   2026-06-11): the correction is the reverse-direction extraction in the
   `SCANNED_STRING_FIELDS` parity-census style; the witness table itself
   (`transaction.rs::witness_kind_allowed`, fail-closed since 0018) is untouched.
3. Cross-artifact boundary under audit: the census↔surface parity contract between the
   fact-minting surface builder (`no_human_surface.rs` — the
   `ActorKnownFact::{observed_now, remembered_belief, routine_assignment}` construction
   sites) and the census constant + witness table the guard ties together.
4. Lock-durability doctrine restated (the `ORD-HARD-035` census claim): "every
   constructed stable_id has an explicit witness arm" is only enforced when both
   directions hold — census⊆surface (no stale census entries) and surface⊆census (no
   unregistered mints); the existing guard proves only the first.

## Architecture Check

1. Extending the existing guard with a regex extraction of string-literal stable_ids
   passed to the `ActorKnownFact` constructors in `no_human_surface.rs` production code
   (the same whitespace-normalizing source-scan machinery the guards already use)
   closes the direction without new infrastructure; a parity assertion (extracted set ⊆
   census) turns an unregistered mint into a CI failure naming the missing id.
2. No backwards-compatibility aliasing/shims: one guard enforces both directions; no
   second census constant is introduced.

## Verification Layers

1. Surface⊆census -> the extended guard: every extracted minted stable_id is asserted a
   census member; a synthetic unregistered mint (exercised during development) fails
   with the id named.
2. Census⊆surface + arm coverage -> the existing guard_018 assertions remain unchanged
   and green.
3. Fail-closed runtime backstop unchanged -> existing
   `provenance_witness_notice_only_workplace_presence_fails_closed_before_proposal`
   remains green (manual confirmation that this ticket touches no production code).

## What to Change

### 1. Reverse-direction extraction in guard_018

Extract every string-literal stable_id passed to
`ActorKnownFact::observed_now` / `remembered_belief` / `routine_assignment` in
`no_human_surface.rs` production code (including ids supplied via local arrays/loops);
assert the extracted set is a subset of `NO_HUMAN_SURFACE_FACT_STABLE_IDS`. Keep the
existing forward-direction assertions as-is.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- `witness_kind_allowed` and the surface builder (`no_human_surface.rs`) — production
  code untouched.
- Any new fact kinds or witness arms.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — extended guard_018
   green; development-time synthetic unregistered mint shown red.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. A stable_id minted in `no_human_surface.rs` that is absent from
   `NO_HUMAN_SURFACE_FACT_STABLE_IDS` fails CI with the id named.
2. Both census directions (census⊆surface, surface⊆census) are enforced by guard_018.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — reverse-direction parity
   extraction added to guard_018.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`
