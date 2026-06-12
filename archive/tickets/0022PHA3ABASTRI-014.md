# 0022PHA3ABASTRI-014: 0022 scoped acceptance artifact

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — evidence report (`reports/0022_ord_life_cert_scoped_acceptance.md`, new), §7-checklist registration (`anti_regression_guards.rs`)
**Deps**: 0022PHA3ABASTRI-002, 0022PHA3ABASTRI-008, 0022PHA3ABASTRI-010, 0022PHA3ABASTRI-011, 0022PHA3ABASTRI-012, 0022PHA3ABASTRI-013

## Problem

Spec 0022 §7 requires a scoped acceptance artifact recording, for the implementation
commits, the evidence for every correction this batch lands — items 1–15, from the
real baseline triage through the meta-lock tier to the explicit non-certification
statement. The lineage's R-27/R-28 history (and `ORD-HARD-099`/`102` this pass) shows
the artifact must be the *work product's* evidence, recorded item-by-item, with
deviations explicit — and `-002`'s checklist parity guard must be registered against
this artifact itself (spec §7 item 12).

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `reports/0022_ord_life_cert_scoped_acceptance.md` does not
   exist (no collision); the sibling exemplar
   `reports/0021_ord_life_cert_scoped_acceptance.md` carries the house structure
   (manifest table, per-§7-item sections, EMERGE-OBS table, deviations, verification
   commands, non-certification statement); the EMERGE-OBS derivation reproduces via
   `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` (re-run and
   byte-compared during the 0022 audit).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §7 (items 1–15; item 12 requires
   running the checklist parity guard against this artifact) and §8 (the artifact
   lands last, measuring the corrected surface).
3. Shared contract under audit: the acceptance-evidence map — spec §7 item IDs ↔
   artifact sections ↔ enforcing symbols — machine-checked by the `-002` guard.
4. Constitutional motivation restated: INV-098 (feature acceptance is harsh: caused,
   eventful, replay-safe, regression-tested — the artifact is where those claims are
   recorded with their proofs) and the evidence-honesty contract (R-27).
5. This is the capstone: no new production logic; it exercises and records the
   pipeline the earlier tickets composed (deliverable-doubles-as-capstone does not
   apply — the verification harnesses landed with their tickets; this artifact is
   acceptance-only plus the checklist registration). Replay/epistemic surfaces are
   exercised (EMERGE-OBS re-derivation, gate-suite runs), not modified.
6. Mismatch handling restated (spec §8): a finding whose premise failed at
   implementation time is recorded here as refuted with its evidence — never silently
   dropped; the Q3 recovery-vs-fail-intent resolution from `-011` is recorded under
   item 9.
7. Change rationale (no silent retcon): N/A — new evidence artifact; the only edit to
   existing files is the checklist-guard registration.

## Architecture Check

1. A single trailing artifact gated on the full leaf set (Deps transitively cover all
   thirteen prior tickets) is the only placement where every §7 claim can be true at
   landing time — the `ORD-HARD-102` lesson (evidence recorded ahead of its proof,
   or mislabeled) is structurally prevented by the checklist guard registration.
2. No backwards-compatibility aliasing/shims; the artifact follows the 0021 house
   structure without inheriting its corrected defects (§2-mislabel class).

## Verification Layers

1. §7 completeness -> guard check: the `-002` checklist parity guard registered with
   the 0022 §7 item→anchor mapping over this artifact; missing-anchor synthetic
   fires.
2. Evidence reproducibility (exec 10) -> command re-runs: every cited test/guard
   command re-run at artifact-authoring time with outputs recorded (EMERGE-OBS table
   re-derived, byte-compared; gate suites; mutation governance checks).
3. Non-certification posture -> manual review + grep: the explicit statement (scoped
   evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4
   entry, not `FIRST-PROOF-CERT`) present per §7 item 15.
4. Deviation honesty (R-28) -> manual review: every refuted premise, Q3 resolution,
   triage owner-decisions, and the `-008` expect-allowlist rationales recorded.

## What to Change

### 1. Author `reports/0022_ord_life_cert_scoped_acceptance.md`

Following the 0021 house structure: implementation-commit manifest; one section per
spec §7 item 1–15 with the evidence (triage dispositions + filed test-debt tickets +
ratchet delta records; CI guard synthetics; render-split outputs; gate-restoration
discrimination witnesses; policy behavioral outputs; eat crossing proof + repricing
diff; scheduler conversions + allowlist rationales; census/scan synthetics;
runner-only canonical-day proof + Q3 resolution; hygiene closures; meta-lock census
output; 0021-report correction cross-reference; risk-register/conformance diffs
quoted; refreshed EMERGE-OBS derivation (measurement only, no thresholds); the
non-certification statement).

### 2. Register the checklist mapping

Extend the `-002` parity guard's data with the 0022 spec §7 item→anchor mapping for
this artifact, so the guard machine-checks it (and fires on a missing anchor).

## Files to Touch

- `reports/0022_ord_life_cert_scoped_acceptance.md` (new)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — checklist mapping
  registration; guard created by 0022PHA3ABASTRI-002)

## Out of Scope

- Any production or test-oracle change beyond the checklist registration (capstone
  discipline — the 0021 capstone's no-changes precedent holds).
- The `SPEC_LEDGER.md` row, spec Status flip, and `archive/specs/` move (spec
  acceptance/archival workflow — cross-spec follow-up).
- Certifying `ORD-LIFE-CERT`, Phase 4 entry, or `FIRST-PROOF-CERT`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — the checklist
   parity guard green over the 0022 artifact with all 15 item anchors resolving;
   missing-anchor synthetic fires.
2. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` — the
   artifact's EMERGE-OBS table matches the re-run byte-for-byte.
3. Every command quoted in the artifact's verification section re-runs green at the
   artifact's commit.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every §7 item maps to recorded, reproducible evidence (or an explicit recorded
   deviation) — machine-checked, never costume.
2. The artifact certifies nothing beyond scoped evidence toward `ORD-LIFE-CERT`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — 0022 checklist mapping
   registration (guard from `-002`).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Completion Outcome (2026-06-12)

Implemented. Added `reports/0022_ord_life_cert_scoped_acceptance.md` with the
implementation commit manifest, one report section per spec section 7 item 1-15,
explicit deviations/pending items, the refreshed `emerge_obs_v1` table, and the
non-certification boundary. The report records the scheduled-run result as pending
rather than overclaiming, records that no mutation baseline entries were retired in
tickets 001-014, and names tickets 015-023 as the focused follow-up debt.

Registered the 0022 acceptance artifact in the existing checklist parity guard with
`ACCEPTANCE_0022_REPORT`, `ACCEPTANCE_0022_CHECKLIST_ANCHORS`, and
`acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors`. The guard
uses the same real missing-anchor synthetic pattern as the 0021 artifact check.

Verification:

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
