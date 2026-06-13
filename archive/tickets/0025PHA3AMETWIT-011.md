# 0025PHA3AMETWIT-011: Capstone — 0025 acceptance artifact, conformance-index rows, and artifact parity guard

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `reports/` acceptance artifact (new), conformance index (`docs/1-architecture/00`), and the artifact parity guard in core test guards. No production crate code.
**Deps**: 0025PHA3AMETWIT-002, -003, -004, -005, -006, -007, -008, -009, -010 (leaf set; -001 covered transitively)

## Problem

Spec 0025 §7 requires an acceptance artifact measuring the corrected surface —
fourteen evidence items minus the per-finding work the siblings deliver — and §6
requires conformance-index rows citing only landed symbols (the 0023/0024
precedent). Without this capstone, the 0025 package has no end-to-end evidence
record, no honest mutation-status restatement, and no non-certification statement;
and the acceptance-artifact parity-guard series
(`acceptance_artifact_0023/0024_maps_spec_section_7_items_to_report_anchors`)
would not cover the new artifact, so its anchors could drift silently.

## Assumption Reassessment (2026-06-12)

1. Verified against `9e33d7a`: the parity-guard series exists per prior artifact
   (`acceptance_artifact_0021/0023/0024_maps_spec_section_7_items_to_report_anchors`
   in `crates/tracewake-core/tests/anti_regression_guards.rs`), establishing the
   extension pattern; `reports/0024_ord_life_cert_scoped_acceptance.md` is the
   structural exemplar (fourteen anchors, `emerge_obs_v1` derivation, honest
   pending status, non-certification statement).
2. Verified against spec 0025 §7 (items 1–13) and §6 (conformance rows for the
   executable meta-witness discipline `ORD-HARD-166`, the envelope duplicate-key
   rejection `ORD-HARD-168`, the fingerprint verification seam
   `ORD-HARD-169`/`170`, the embodied carrier census `ORD-HARD-172`, plus the
   recorded `ORD-HARD-171`/`184`/`186` decisions — rows cite only landed symbols,
   landing with this capstone per the 0023/0024 precedent).
3. Shared boundary under audit: spec §7 evidence items ↔ report anchors ↔ the
   parity guard — every §7 item must map to a named report anchor, and the guard
   must fail on a missing anchor.
4. Invariant restated before trusting the narrative (INV-098 — feature acceptance
   is harsh): done means caused, eventful, replay-safe, regression-tested, and
   honestly evidenced; an acceptance artifact that overstates (or a conformance
   row citing unlanded symbols) is the evidence-dishonesty class the lineage's §7
   discipline exists to prevent.
5. Evidence/replay surface touched: the artifact records measurement only — the
   `emerge_obs_v1` derivation, gate results, and the scheduled-mutation status
   (dated green result, or "pending" restated honestly per the `ORD-HARD-190`
   rule, which this artifact's wording guard now enforces). Nothing here mutates
   simulation behavior; replay/leak surfaces are exercised, not changed.

## Architecture Check

1. A verification-only capstone (artifact + index rows + parity guard) mirrors the
   proven 0023/0024 closeout shape: evidence lands after all measured surfaces
   exist, so no row or anchor ever describes an enforcement that has not landed —
   the honest-intermediate-state rule the lineage's §6 placement follows.
   Extending the existing parity-guard series is cleaner than a bespoke check: one
   guard shape per artifact generation, census-enrolled.
2. No backwards-compatibility aliasing/shims: the artifact reports the surface as
   measured; a finding premise that failed at implementation time is recorded as
   refuted in the artifact, never silently dropped (spec §8 posture).

## Verification Layers

1. §7 completeness → the artifact carries items 1–13 (witness discipline,
   provenance kill set, envelope negatives + decisions, fingerprint table +
   golden bytes, embodied census + staleness positives, TUI gates + decisions,
   census closures, CI guard + records, agent-reported re-verification
   confirmation, doc diffs quoted, `emerge_obs_v1` derivation, mutation status,
   non-certification statement) — each mapped to a named anchor.
2. Parity guard → `acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors`
   extends the series with a firing missing-anchor synthetic; the
   `ORD-HARD-190` wording rule is enforced (certification claim + "pending"
   mutation row ⇒ fail).
3. Conformance rows → each §6 row's cited symbol grep-resolves at the capstone
   commit (codebase grep-proof; the 0023/0024 row precedent).
4. End-to-end regression → the four gates green at the capstone commit, recorded
   in the artifact with per-gate unmasked sentinels (spec header precedent).

## What to Change

### 1. Acceptance artifact

Write `reports/0025_ord_life_cert_scoped_acceptance.md` per the 0024 exemplar:
the §7 items as anchored sections, the agent-reported-premise confirmation (spec
§7 item 9 — all premises held or refuted-and-recorded), quoted §6 doc diffs, the
`emerge_obs_v1` derivation over the corrected surface, the mutation status under
the `ORD-HARD-190` rule, and the explicit non-certification statement (scoped
evidence toward `ORD-LIFE-CERT`; not full-project, not Phase 4, not
`FIRST-PROOF-CERT`).

### 2. Conformance-index rows

Add/update `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` rows
per spec §6 (Assumption item 2's list), citing only landed symbols.

### 3. Artifact parity guard

Extend the series with the 0025 guard + missing-anchor synthetic + the
`ORD-HARD-190` wording enforcement; enroll under the repaired witness routing.

## Files to Touch

- `reports/0025_ord_life_cert_scoped_acceptance.md` (new)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Any production or guard logic beyond the parity guard (verification-only
  capstone; it exercises the siblings, it does not modify them).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move — deferred
  to spec acceptance/archival per `docs/archival-workflow.md` (cross-spec
  follow-up, not ticketed).
- Certifying `ORD-LIFE-CERT`, Phase 4 entry, or `FIRST-PROOF-CERT`.

## Acceptance Criteria

### Tests That Must Pass

1. The 0025 parity guard passes against the written artifact and fails on the
   missing-anchor synthetic; the certification-claim-with-pending-mutation wording
   check fires on a synthetic violating report.
2. Every conformance row added cites a symbol that grep-resolves; the artifact
   records all four gates green with per-gate sentinels at the capstone commit.
3. `cargo test -p tracewake-core --test anti_regression_guards` and the four
   gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`)
   pass.

### Invariants

1. Every spec §7 evidence item maps to a guard-checked report anchor; evidence
   claims cite only landed, resolving symbols.
2. The artifact carries the non-certification statement and an honest mutation
   status; no certification wording can coexist with a pending mutation row.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` —
   `acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors` +
   missing-anchor synthetic + `ORD-HARD-190` wording enforcement.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

Added `reports/0025_ord_life_cert_scoped_acceptance.md` with the spec section 7
evidence map, anchored report sections, current `emerge_obs_v1` derivation,
honest scheduled-mutation pending status under the `ORD-HARD-190` rule, and an
explicit non-certification statement. Added 0025 conformance-index rows in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` for executable
meta-witness discipline, provenance-true perception taint, envelope fail-closed
decisions, manifest fingerprint honesty, embodied carrier census and observation
capture, TUI gate depth and `ControllerMode` decision, census/oracle closures,
and CI evidence-honesty closure.

Extended `crates/tracewake-core/tests/anti_regression_guards.rs` with the
`acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors` parity
guard, a missing-anchor synthetic, and the pending-mutation/certification-wording
synthetic. The meta-lock registry now enrolls the 0025 artifact guard.

Verification:

1. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. conformance-symbol grep sweep over the added row symbols
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
