# 0024PHA3ACONSCH-014: 0024 acceptance artifact and conformance-index rows (capstone)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `reports/0024_ord_life_cert_scoped_acceptance.md` (new), `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `crates/tracewake-core/tests/anti_regression_guards.rs` (artifact parity guard).
**Deps**: 0024PHA3ACONSCH-003, 0024PHA3ACONSCH-006, 0024PHA3ACONSCH-007, 0024PHA3ACONSCH-008, 0024PHA3ACONSCH-009, 0024PHA3ACONSCH-010, 0024PHA3ACONSCH-011, 0024PHA3ACONSCH-012, 0024PHA3ACONSCH-013

## Problem

Spec 0024 §7 requires a scoped acceptance artifact recording, for the
implementation commits, the evidence for every correction (items 1–14), and §6
requires conformance-index rows for the schema-version gate (`ORD-HARD-140`), the
completed embodied truth-access removal (`ORD-HARD-143`), the derived apply
perimeter (`ORD-HARD-144`), and the meta-witness completion (`ORD-HARD-141`) —
landing with the capstone, citing only landed symbols (the 0023 honest-evidence
precedent). This ticket is the capstone: it exercises the corrected surface
end-to-end and authors the evidence; it introduces no new production logic beyond
the artifact's own parity guard.

## Assumption Reassessment (2026-06-12)

1. Verified against the repo at baseline `4d62f61`: the 0023 pattern artifacts
   exist (`reports/0023_ord_life_cert_scoped_acceptance.md`; the
   `acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors` parity
   guard with its synthetic; the `emerge_obs_v1` EMERGE-OBS derivation in
   `emergence_ledger.rs`) — this ticket mirrors them for 0024.
   `reports/0024_ord_life_cert_scoped_acceptance.md` does not yet exist (no
   collision).
2. Verified against spec 0024 §6 (conformance rows + recording-destination rule)
   and §7 (artifact items 1–14, including the §7.10 premise-confirmation: all
   twenty-six findings operator-verified pre-implementation — sixteen at audit, ten
   re-verified at spec reassessment).
3. Cross-artifact boundary: the spec §7 checklist ↔ report-anchor parity contract —
   the 0024 artifact runs the parity guard against itself with a firing
   missing-anchor synthetic, exactly as 0023 did.
4. Lineage evidence-honesty rule restated: acceptance evidence cites only landed
   symbols, records pending statuses honestly (the scheduled mutation run's result
   or its still-pending status restated), carries the explicit non-certification
   statement, and records any finding premise that failed at implementation time as
   refuted rather than silently dropped.
5. Enforcement surface (evidence honesty / canonical-day proof): this ticket
   re-runs and records — it must not weaken any gate to make evidence pass; the
   EMERGE-OBS re-derivation stays measurement-only (no thresholds).

## Architecture Check

1. A single trailing capstone gated on every leaf ticket keeps the conformance
   rows and the artifact honest by construction (they cannot cite an unlanded
   symbol), per the 0023 precedent and the spec's own placement rule; distributing
   the evidence across implementation tickets would re-create the
   stale-evidence-window the lineage's deviation note on 0023 already corrected.
2. No backwards-compatibility aliasing/shims: the artifact supersedes nothing and
   amends no doctrine; it records.

## Verification Layers

1. §7 checklist parity → the 0024 artifact passes the parity guard run against
   itself, with the missing-anchor synthetic firing (test).
2. Conformance-row honesty → every symbol cited in the four new/updated rows
   resolves by grep against the landed tree (codebase grep-proof).
3. Corrected-surface measurement → `cargo test -p tracewake-core --test
   emergence_ledger` re-derivation recorded in the artifact; full four-gate run
   recorded with per-gate unmasked statuses (the spec-header measurement
   discipline).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. The acceptance artifact (spec §7 items 1–14)

`reports/0024_ord_life_cert_scoped_acceptance.md` recording, with anchors: the
schema-version gate evidence (item 1 — per spec §7's numbering: items 1–9 map the
correction groups to their firing negatives and synthetics as enumerated in §7);
the premise-confirmation (item 10, per Assumption item 2); the §6 doc diffs quoted
(item 11); the EMERGE-OBS re-derivation (item 12); the scheduled mutation run's
status, honestly (item 13); the explicit non-certification statement (item 14).

### 2. Conformance-index rows (spec §6)

Add/update rows for `ORD-HARD-140`/`141`/`143`/`144`, citing landed symbols only;
include the `ORD-HARD-161` decision text supplied by ticket -013.

### 3. Artifact parity guard

`acceptance_artifact_0024_maps_spec_section_7_items_to_report_anchors` with a
firing missing-anchor synthetic, enrolled under the live-witness rule.

## Files to Touch

- `reports/0024_ord_life_cert_scoped_acceptance.md` (new)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any production-logic change (capstone exercises; it does not modify).
- The `docs/4-specs/SPEC_LEDGER.md` row and the spec's `archive/specs/` move —
  deferred to spec acceptance/archival per `docs/archival-workflow.md` (recorded as
  a cross-spec follow-up, not ticketed).
- Certifying `ORD-LIFE-CERT`, Phase 4 entry, or `FIRST-PROOF-CERT` (the artifact
  explicitly disclaims these).

## Acceptance Criteria

### Tests That Must Pass

1. The artifact exists with all fourteen §7 items anchored; the 0024 parity guard
   passes against it and its missing-anchor synthetic fires.
2. Every conformance-row symbol resolves by grep; the rows cover
   `ORD-HARD-140`/`141`/`143`/`144` and carry the recorded `161` decision.
3. The artifact records per-gate unmasked four-gate results, the EMERGE-OBS
   derivation, the mutation-run status, the premise-confirmation, and the
   non-certification statement.
4. The four workspace gates pass on the completed batch.

### Invariants

1. Acceptance evidence cites only landed symbols; pending statuses are recorded
   honestly; no gate is weakened to make evidence pass.
2. The artifact disclaims certification beyond scoped `ORD-LIFE-CERT` evidence.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` —
   `acceptance_artifact_0024_maps_spec_section_7_items_to_report_anchors` + synthetic.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards --test emergence_ledger`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

- Added `reports/0024_ord_life_cert_scoped_acceptance.md` with all fourteen
  spec §7 anchors, the `emerge_obs_v1` derivation, honest mutation-run status,
  and the explicit non-certification statement.
- Added 0024 conformance rows for `ORD-HARD-140`, `ORD-HARD-141`,
  `ORD-HARD-143`, and `ORD-HARD-144`, and recorded the `ORD-HARD-161`
  cognition-priority decision.
- Added the 0024 acceptance-artifact parity guard and registered it in
  `META_LOCK_REGISTRY` with the firing missing-anchor synthetic.

Proof:

1. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
2. `cargo test -p tracewake-core --test anti_regression_guards --test emergence_ledger`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
