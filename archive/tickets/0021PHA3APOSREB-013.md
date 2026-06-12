# 0021PHA3APOSREB-013: 0021 scoped acceptance artifact and EMERGE-OBS refresh (capstone)

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — evidence/reporting only (`reports/` artifact; re-runs existing suites)
**Deps**: 0021PHA3APOSREB-001, -002, -003, -004, -005, -006, -007, -008, -009, -010, -011, `archive/tickets/0021PHA3APOSREB-012.md`; `archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (§7)

## Problem

Spec 0021 §7 requires a scoped acceptance artifact recording, for the implementation
commits, the proof outputs of every correction — including the recorded
implementer choices, re-derivation diffs, per-member dispositions, and the explicit
non-certification boundary — plus an updated read-only EMERGE-OBS ledger derivation
over the corrected surface. This capstone introduces no production logic; it
exercises and records what tickets -001 … -012 landed.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the 0020 exemplar
   (`reports/0020_ord_life_cert_scoped_acceptance.md`) establishes the artifact
   convention (manifest table of ticket commits, verification-command table,
   per-section proofs, deviations, finished-tree gate record, explicit
   non-certification boundary); the EMERGE-OBS derivation lives at
   `crates/tracewake-core/tests/emergence_ledger.rs` (observer-only, no production
   call sites — re-confirm with the `rg` exit-1 check at closure).
2. Verified against spec 0021 (reassessed 2026-06-11): §7 enumerates 17 evidence
   items; §8 orders this capstone last so the EMERGE-OBS baseline measures the
   corrected surface.
3. Cross-artifact boundary under audit: the evidence-honesty contract — every claim
   in the artifact must be produced by the path under test and re-runnable
   (R-27; two consecutive clean evidence passes are at stake).
4. Lock-durability/evidence doctrine restated: an acceptance artifact may record
   only what the code proves; recorded implementer choices (perimeter-vs-rationale,
   per-field required-vs-optional, sleep mirror-vs-deferral, per-member
   populate-vs-defer, INV-087 decision) must each appear with their grounds.
5. No enforcement surface is modified — this ticket re-runs gates and records
   results. Any failure discovered here is routed back to the owning ticket, not
   patched in the artifact.

## Architecture Check

1. A single trailing acceptance-only capstone mirrors the lineage convention
   (0020's capstone) and keeps evidence generation separate from the corrections it
   measures — the artifact cannot be written until the gates it cites pass, which
   is the gating the spec's §8 ordering encodes.
2. No backwards-compatibility aliasing/shims: not applicable — no production code.

## Verification Layers

1. Evidence honesty (R-27) -> every §7 item's claim paired with its re-runnable
   command and observed output in the artifact.
2. EMERGE-OBS observer-only invariant -> `rg "EmergeObs|emerge_obs|emergence_ledger"`
   over all three crates' `src/` returns no matches (exit 1).
3. Non-certification boundary -> explicit statement: scoped evidence toward
   `ORD-LIFE-CERT`; not full-project certification, not Phase 4 entry, not
   `FIRST-PROOF-CERT`.
4. Finished-tree gates -> the four workspace gates re-run on the finished tree and
   recorded.

## What to Change

### 1. Author `reports/0021_ord_life_cert_scoped_acceptance.md`

Per spec §7, items 1–17: rebind-after-rejection proof (1); scheduled-ratchet
live-run evidence + widened guard outputs (2); hidden-truth gate rebuild
confirmation + visibility demotion (3); per-arm census proof with the two-arm
synthetic (4); rationale split + perimeter decision record (5); real baseline triage
with closed tags, filed test-debt tickets, retired entries (6); behavioral
policy-dispatch proof + sleep-accessibility resolution (7); shared crossing-emitter
proof + corrupt-history rejection outputs (8); typed place-concealment migration +
perception guard extension (9); applier totality + deleted mutator + derived lists
(10); content-validator negatives + contract-prose reconciliation diff (11);
provenance partition proof + extended dead-surface sweep output with per-member
dispositions (12); generative deltas: two-file ban, in-block ordering, per-kind
tamper coverage, re-derived floors (13); low-group closures/deferrals with cites
including the INV-087 decision record (14); risk-register and conformance-index
diffs, quoted (15); the refreshed EMERGE-OBS ledger table (measurement only, no
thresholds) (16); the explicit non-certification statement (17).

### 2. EMERGE-OBS refresh

Re-run `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` on the
finished tree; reproduce the table in the artifact; re-confirm observer-only status.

## Files to Touch

- `reports/0021_ord_life_cert_scoped_acceptance.md` (new)

## Out of Scope

- Any production or test-logic change — failures route back to owning tickets.
- The `docs/4-specs/SPEC_LEDGER.md` row and the spec's `archive/specs/` move —
  deferred to spec acceptance/archival per the hardening-series convention
  (`docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. Every §7 item (1–17) present in the artifact with a re-runnable command and
   observed result; every recorded implementer choice carries its grounds.
2. `rg "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src` exits 1 (no production call sites).
3. The four finished-tree gates pass and are recorded:
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

### Invariants

1. The artifact claims only what re-run commands prove on the finished tree.
2. The non-certification boundary is explicit.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
2. `rg -n "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src` (expect exit 1)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Implementation Outcome (2026-06-11)

Created `reports/0021_ord_life_cert_scoped_acceptance.md` with the 12-ticket commit
manifest, all 17 spec §7 evidence items, the deferred INV-087 decision record, the
conformance/risk surfaces, the refreshed EMERGE-OBS table, and the explicit
non-certification boundary. No production code, test logic, fixture behavior, or
schema changed under this capstone.

## Verification (2026-06-11)

Passed:

1. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
2. `rg -n "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src` exited 1 as expected.
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
