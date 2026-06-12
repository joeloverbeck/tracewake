# 0023PHA3AEMBLOC-013: Capstone — 0023 acceptance artifact, conformance rows, and EMERGE-OBS derivation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — acceptance report (new), conformance index, acceptance-checklist guard extension
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, archive/tickets/0023PHA3AEMBLOC-002.md, archive/tickets/0023PHA3AEMBLOC-003.md, archive/tickets/0023PHA3AEMBLOC-004.md, archive/tickets/0023PHA3AEMBLOC-005.md, archive/tickets/0023PHA3AEMBLOC-006.md, archive/tickets/0023PHA3AEMBLOC-007.md, archive/tickets/0023PHA3AEMBLOC-008.md, archive/tickets/0023PHA3AEMBLOC-009.md, archive/tickets/0023PHA3AEMBLOC-010.md, archive/tickets/0023PHA3AEMBLOC-011.md, archive/tickets/0023PHA3AEMBLOC-012.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Spec 0023 §7 requires an acceptance artifact recording, for the implementation
commits, the evidence map across all nineteen findings — and §6's conformance-index
rows describe *landed* corrections, so they belong with the artifact that proves the
landing (decomposition decision flagged at approval: relocated from §8 group 1 to
this capstone for honest-evidence placement — a conformance row claiming an
enforcement that has not landed yet would be the exact R-27 shape this lineage
polices). Without this ticket the batch ships corrections with no §7 evidence map,
no EMERGE-OBS re-derivation, and unrecorded scheduled-run status.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `reports/0023_ord_life_cert_scoped_acceptance.md` does not
   exist (no collision); the 0022 checklist-guard pattern
   (`acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors`) exists
   as the extension template; the `emergence_ledger` derivation surface exists in
   the crate tree; `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
   exists for the rows.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §7 (items 1–14), §6
   (conformance rows for `ORD-HARD-121`/`122`/`123`/`129`/`126`/`125`), and §8 (the
   artifact lands last, measuring the corrected surface). Re-verification of the
   seven initially agent-reported findings was completed at spec reassessment
   (2026-06-12, zero refuted), so §7 item 10 records premise-held confirmation, not
   fresh re-verification.
3. Shared contract under audit: the acceptance-evidence chain — spec §7 item list ↔
   report anchors ↔ the implementing tickets' landed surfaces — consumed by the
   checklist parity guard, which per §7 item 9 must run against this artifact
   itself.
4. Constitutional motivation restated: R-27 (acceptance evidence must be produced by
   the path under test and name the emitter) and the lineage's evidence-honesty
   contract (an unmet finding is recorded as refuted in the artifact, never silently
   dropped); explicit non-certification posture per §7 item 14.
5. This ticket touches evidence and documentation surfaces only — no production,
   epistemic, or replay change; the EMERGE-OBS derivation is measurement only (no
   thresholds), and the checklist-guard extension is fail-closed test-oracle work.
6. Change rationale (no silent retcon): the conformance rows land here rather than
   with group 1 per the flagged decomposition decision (the spec's *intent* — honest
   evidence — over its placement letter); recorded in the batch's Step 6 summary.

## Architecture Check

1. A single trailing capstone whose scope IS the spec's §7 evidence map, gated on
   every implementation ticket, is the lineage's proven closeout shape (0021/0022
   precedent): it measures the corrected surface once, runs the checklist parity
   guard against its own artifact, and keeps conformance rows truthful by
   construction. It introduces no new production logic — the new file is evidence,
   and the only code change extends the existing checklist guard to the 0023
   artifact.
2. No backwards-compatibility aliasing/shims: the 0023 artifact supersedes nothing
   (the 0022 report's §1 correction landed in -002); one artifact per pass.

## Verification Layers

1. Evidence-map completeness (R-27) -> codebase test-proof: the extended checklist
   guard maps every spec §7 item (1–14) to a required anchor in the 0023 report,
   with a firing missing-anchor synthetic.
2. Conformance-row truthfulness -> manual review + grep-proof: each §6 row cites a
   landed surface (symbol grep against the implementing ticket's diff).
3. Measurement honesty -> replay/golden-fixture check: the EMERGE-OBS ledger
   re-derivation runs over the corrected surface (post -004/-005) and the recorded
   table reproduces from the command named in the report.
4. Non-certification posture -> manual review: the explicit scoped-evidence
   statement (§7 item 14) present; scheduled-run status recorded honestly (first
   post-0022 result or still-pending, §7 item 13).

## What to Change

### 1. Acceptance artifact (spec §7 items 1–14)

Author `reports/0023_ord_life_cert_scoped_acceptance.md`: commit manifest; the
migration evidence (observation emitters, projection-derived locality, compile-time
quarantine, INV-093 negatives, repricing diff); the meta-lock repairs with firing
negatives; debug-overlay wiring; policy behavioral test output; scan-evasion
closures; in-`context()` witness; canonical-intent coherence and sleep positive;
exhaustive cause match and governance re-arm; the 0022 §1 correction record;
premise-held confirmations (item 10); risk-register and conformance diffs quoted;
EMERGE-OBS derivation; scheduled-run status; explicit non-certification statement.

### 2. Conformance-index rows (spec §6, relocated)

Add/update rows in `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
for the embodied-locality migration, the meta-lock witness/census/ratchet repairs,
the policy surface-driven lock, and the debug-overlay wiring — each citing the
landed symbol.

### 3. Checklist parity guard extension

Extend the acceptance-checklist guard to map spec 0023 §7 items to required 0023
report anchors (the 0022 pattern), with a missing-anchor synthetic; enroll under the
-001 registry.

## Files to Touch

- `reports/0023_ord_life_cert_scoped_acceptance.md` (new)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any correction itself (tickets -001…-012).
- `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move — deferred to spec
  acceptance/archival per `docs/archival-workflow.md` (cross-spec follow-up, not
  ticketed).
- Certification claims (`ORD-LIFE-CERT`, Phase 4 entry, `FIRST-PROOF-CERT`) —
  explicitly disclaimed in the artifact.

## Acceptance Criteria

### Tests That Must Pass

1. The extended checklist guard passes against the authored 0023 report and its
   missing-anchor synthetic fires
   (`cargo test -p tracewake-core --test anti_regression_guards`).
2. The EMERGE-OBS derivation command named in the report reproduces the recorded
   table byte-equivalently.
3. Every §6 conformance row's cited symbol resolves by grep against the landed tree.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. Every spec §7 item maps to a machine-checked anchor in the artifact — no
   prose-trusted evidence claims.
2. The artifact certifies scoped evidence only; the non-certification statement is
   present verbatim in posture.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — 0023 checklist parity
   guard + missing-anchor synthetic, registry-enrolled.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
