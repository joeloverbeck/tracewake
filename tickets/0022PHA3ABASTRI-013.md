# 0022PHA3ABASTRI-013: Conformance-index rows for landed 0022 enforcement

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — documentation only (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`)
**Deps**: 0022PHA3ABASTRI-001, 0022PHA3ABASTRI-003, 0022PHA3ABASTRI-004, 0022PHA3ABASTRI-005, 0022PHA3ABASTRI-006, 0022PHA3ABASTRI-007, 0022PHA3ABASTRI-009

## Problem

Spec 0022 §6 requires conformance-index updates that describe enforcement landed by
this batch: the disposition-tag governance row (once the real triage lands), the
scheduled-ratchet row (concurrency isolation), the per-kind policy dispatch row (once
the behavioral test is real), the shared-emitter row (eat.rs perimeter), and new rows
for the embodied debug-render split, the planner-gate adversarial restoration, and
the meta-lock tier. Spec §8 places documentation corrections with group 1, but rows
describing enforcement must not land before the enforcement exists — this ticket is
the flagged decomposition decision applying the honest-placement rule: it lands after
its enforcing tickets (the risk-register half of §6 landed with `-001`).

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
   exists and carries the 0021 rows the spec names (disposition-tag governance,
   scheduled ratchet, per-kind policy dispatch, shared emitter, plus the "0021 INV-087
   bind-time perception decision" row — all diff-verified during the 0022 audit's
   evidence-honesty slice).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §6 (the row list) and §8 (the
   group-1 placement directive this decomposition diverges from, flagged and approved
   at Step 4).
3. Shared contract under audit: the conformance index as the architecture tier's
   claim surface — every row must describe enforcement that exists at the commit the
   row lands in (the dishonest-intermediate-state rule; R-27).
4. Constitutional motivation restated: the lineage's evidence-honesty contract —
   docs/2-execution/10's review-artifact accuracy applied to the conformance index.
5. No code surface touched: documentation-only; each row's claim is verified against
   the landed symbol (grep-confirmed) before the row is written — no enforcement
   claim precedes its proof.
6. Change rationale (no silent retcon): rows update because their described
   enforcement changed in tickets `-001`–`-009`; each row cites the enforcing test or
   guard by name.

## Architecture Check

1. A single trailing docs ticket whose Deps are exactly the enforcing tickets keeps
   every row true at landing time — per-ticket doc edits would either stagger the
   index (churn) or front-run enforcement (dishonesty). The risk-register split into
   `-001` is correct because R-29's pattern description is true immediately.
2. No backwards-compatibility aliasing/shims: rows are updated in place; superseded
   claims are replaced, not annotated around.

## Verification Layers

1. Row-claim truth (R-27) -> codebase grep-proof: each updated/new row's named
   symbol (guard, test, derivation) exists at HEAD when the row lands — one grep per
   row, recorded in implementation notes.
2. Coverage of §6's list -> manual review: all seven row actions (triage governance,
   scheduled ratchet/concurrency, policy dispatch, shared emitter, debug-render
   split, planner-gate restoration, meta-lock tier) present.
3. No stale 0021 claims -> grep-proof: rows superseded by 0022 enforcement no longer
   describe the 0021-era shape.

## What to Change

### 1. Update existing rows

Disposition-tag governance (post-`-001` triage + new guard rules); scheduled-ratchet
row (post-`-003` concurrency isolation + fail-closed config checks); per-kind policy
dispatch (post-`-006` behavioral table-driven lock); shared-emitter row (post-`-007`
derived perimeter including eat.rs).

### 2. Add new rows

Embodied debug-render split (`-009`); planner hidden-truth gate adversarial
restoration with discrimination witness (`-005`); the meta-lock tier — bijection
census, nonzero-witness rule, two-sided ratchets (`-004`).

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- The risk-register edits (landed with `0022PHA3ABASTRI-001`).
- The `SPEC_LEDGER.md` row and `archive/specs/` move (deferred to spec
  acceptance/archival per the hardening-spec convention — cross-spec follow-up, not
  ticketed).
- The acceptance artifact (`0022PHA3ABASTRI-014`).

## Acceptance Criteria

### Tests That Must Pass

1. Each row's named enforcing symbol grep-resolves at HEAD (one command per row,
   recorded).
2. All seven §6 row actions present in the index; no row describes unlanded
   enforcement.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
   (docs-only — gates prove no accidental code touch).

### Invariants

1. Every conformance row's claim is true at the commit it lands in.
2. The index names enforcing symbols (grep-stable), not prose summaries alone.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing
   pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "bijection\|nonzero-witness\|two-sided\|debug-render\|adversarial" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
2. Per-row symbol greps against `crates/tracewake-core/tests/` (recorded in
   implementation notes).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
