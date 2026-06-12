# 0023PHA3AEMBLOC-003: Embodied-sweep cite-only deferral content witness

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — test-oracle sweep (`anti_regression_guards.rs`)
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, `archive/specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

The embodied dead-surface sweep's cite-only deferral path matches on file path alone
(spec 0023 `ORD-HARD-124`, medium): `embodied_field_has_registered_producer`
short-circuits on `entry.producer_snippet.is_empty() || source.contains(...)`, and the
`debug_only_diagnostics` deferral entry in `EMBODIED_SURFACE_FIELD_PRODUCERS` carries
an empty snippet — so the entry is satisfied whenever `view_models.rs` is merely
present in `producer_sources`, which it always is. A debug surface could be orphaned
in production while the sweep stays green: the `ORD-HARD-104` "nonzero-witness applied
to per-entry matches" requirement was never realized for the cite-only path.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the `entry.producer_snippet.is_empty() ||` short-circuit in
   `embodied_field_has_registered_producer` and the empty-snippet
   `debug_only_diagnostics` entry in `EMBODIED_SURFACE_FIELD_PRODUCERS`
   (`crates/tracewake-core/tests/anti_regression_guards.rs`); the
   `synthetic_constant_literal_embodied_surface_producer` negative covers the constant
   shape only — no negative exercises an orphaned empty-snippet deferral.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-124`
   (operator-verified evidence; the §3 validated-list confirms the deferral *shape* is
   the intended `ORD-HARD-104` outcome — the defect is the missing content witness,
   not the deferral itself).
3. Shared contract under audit: the embodied dead-surface sweep —
   `EMBODIED_SURFACE_FIELD_PRODUCERS` ↔ `view_models.rs` field census ↔ producer
   sources — consumed by every embodied view-model field's reachability proof. Note:
   ticket -006 wires `render_debug_overlay` into production, which may let the
   `debug_only_diagnostics` deferral graduate to a real producer/consumer entry; this
   ticket's witness must hold either way (the witness validates whatever entry shape
   remains).
4. Constitutional motivation restated: lock durability (R-29) and INV-093's direction
   (a debug-classified surface losing its reachability proof silently is the
   leakage-test gap class) — a deferral entry must still prove the field it defers is
   referenced where it claims to be.
5. This ticket touches a fail-closed test-oracle surface only: the repaired predicate
   tightens the sweep (cite-only entries must still content-match); no epistemic,
   replay, or product surface is weakened.
6. Change rationale (no silent retcon): the cite-only path changes because spec
   0023's evidence proves it is satisfied unconditionally for the entry it exists to
   police; mandated by `ORD-HARD-124`'s required correction.

## Architecture Check

1. Requiring `source.contains(field_name)` even when `producer_snippet` is empty (or
   an explicit deferral record with its own consumer-existence proof) keeps the
   cite-only convenience while restoring the content witness — cleaner than banning
   empty snippets outright, which would force declaration-aliasing snippets back in
   (the exact `ORD-HARD-104` defect the empty snippet replaced).
2. No backwards-compatibility aliasing/shims: the unconditional short-circuit is
   replaced, not wrapped; the deferral entry keeps one honest shape.

## Verification Layers

1. Per-entry content witness (R-29) -> codebase test-proof: a synthetic deferral
   entry whose field is absent from its cited source fails the sweep — routed through
   the production predicate, enrolled under -001's live-witness and bijection rules.
2. Real-entry integrity -> codebase grep-proof: the `debug_only_diagnostics` entry
   passes via a genuine field-name match in `view_models.rs`, not via the empty-snippet
   short-circuit.

## What to Change

### 1. Content-matching cite-only path

In `embodied_field_has_registered_producer`, require the field name to be present in
the cited source even when `producer_snippet` is empty (or introduce an explicit
deferral record carrying a consumer-existence proof). Update the
`debug_only_diagnostics` entry to the repaired shape.

### 2. Orphaned-deferral negative

Add the synthetic negative (deferral entry whose field is absent from the cited
source must produce a "no producer" error) and enroll the lock + negative in the
-001-repaired registry with a live witness count.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Wiring `render_debug_overlay` into production and the derived debug-token negative
  (`ORD-HARD-125`/`135` — ticket -006).
- The constant-literal producer predicate (already landed in 0022; verified holding).

## Acceptance Criteria

### Tests That Must Pass

1. The orphaned-deferral synthetic fails the sweep; the repaired
   `debug_only_diagnostics` entry passes via genuine content match
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. `cargo test --workspace` green; fmt + clippy clean.

### Invariants

1. No `EMBODIED_SURFACE_FIELD_PRODUCERS` entry can be satisfied by path presence
   alone — every entry proves field-level content in its cited source.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — content-matching
   cite-only predicate + orphaned-deferral negative + registry enrollment.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented the embodied-sweep cite-only deferral content witness in
`crates/tracewake-core/tests/anti_regression_guards.rs`.

- Repaired `embodied_field_has_registered_producer` so an empty
  `producer_snippet` no longer passes by path presence alone; it must find the
  declared field name in the cited source.
- Added `synthetic_orphaned_deferral_embodied_surface_producer`, proving an
  `ActionAvailability.debug_only_diagnostics` deferral whose cited source omits the
  field fails the sweep.
- Enrolled the embodied sweep registry negative under the orphaned-deferral
  synthetic while retaining the existing constant-literal producer negative inside
  the same test.

Deviations: none.

Verification:

- `cargo test -p tracewake-core --test anti_regression_guards` — passed
- `cargo fmt --all --check` — passed after applying `cargo fmt --all`
- `cargo clippy --workspace --all-targets -- -D warnings` — passed
- `cargo build --workspace --all-targets --locked` — passed
- `cargo test --workspace` — passed
