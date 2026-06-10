# 0016PHA3ANEEACC-014: 0016 scoped acceptance artifact

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — acceptance-evidence report only
**Deps**: 0016PHA3ANEEACC-013

## Problem

Spec 0016 §7: the implementation commits need a recorded acceptance artifact — `reports/0016_ord_life_cert_scoped_acceptance.md` — assembling the evidence that each finding's correction landed and its lock can fail. This is the capstone: it introduces no production logic and exercises the pipeline the earlier tickets composed (its `Deps: 0016PHA3ANEEACC-013` transitively covers the entire batch — 013 → {003, 008, 012} → all implementation tickets).

## Assumption Reassessment (2026-06-10)

1. Exemplars and inputs verified: `reports/0014_ord_life_cert_scoped_acceptance.md` and `reports/0015_ord_life_cert_scoped_acceptance.md` exist as the house-style precedents; `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is the canonical artifact template. The evidence inputs are produced by the upstream tickets: per-actor need ledgers (002, extended by 010/011 repricings), the tamper-test proof (003), the `work_block_failed_then_sleep_succeeds_001` log excerpt (001), the severe-safety flight trace (006), the embodied belief-vs-truth and band-only render comparison (007), the source-guard inventory incl. workspace + fixture censuses (012), and the `cargo mutants` baseline counts with per-miss dispositions (012).
2. Spec/docs: spec 0016 §7 items 1–8 (the artifact's required contents, including the explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`); §9 (the anticipated fifth, verification-only audit this artifact should make cheaper).
3. Cross-artifact boundary under audit: the report cites evidence from every implementation ticket in the batch — each §7 item maps to a named upstream ticket's landed surface, and every command quoted in the report must be re-runnable against the post-implementation tree.

## Architecture Check

1. A single trailing acceptance artifact mirrors the 0014/0015 precedent and the spec's own §7 enumeration — distributing the evidence across per-ticket notes would leave no one place where the scoped-certification claim (and its explicit limits) is recorded. The capstone shape (no new production logic; exercises what earlier tickets composed) keeps it reviewable as documentation.
2. No backwards-compatibility aliasing/shims: N/A — report-only; evidence is cited from live tests/fixtures, not duplicated as parallel truth.

## Verification Layers

1. Evidence completeness → manual review against spec §7: all eight items present (need ledgers before/after with every golden diff explained; tamper-gate fail+pass proof; reservation-closure log excerpt; flight trace live and under replay; embodied comparisons; guard/census inventory; mutants baseline with dispositions; non-certification statement).
2. Re-runnability → command-based check: every verification command quoted in the report resolves and passes against the implementation commits.
3. Recorded-deferral check → grep-proof: the artifact confirms `tickets/0016PHA3ANEEACC-015.md` (the ORD-HARD-021 unification deferral) exists, per the spec's acceptance criterion.

## What to Change

### 1. Author `reports/0016_ord_life_cert_scoped_acceptance.md`

Following `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and the 0015 report's voice, recording for the implementation commits:

1. Per-actor need ledgers for `no_human_day_001` and `sleep_spanning_window_boundary_charges_each_tick_once_001`, before/after ORD-HARD-014, with every golden checksum diff explained.
2. The tamper test demonstrating the context-hash re-derivation gate failing on a corrupted seed event and passing on the genuine log.
3. The `work_block_failed_then_sleep_succeeds_001` event-log excerpt.
4. The severe-safety flight trace (move proposal with ancestry) live and under replay.
5. Embodied/debug comparison for stale workplace belief vs truth; embodied render without numeric need values.
6. Source-guard inventory including the workspace census and fixture-directory census.
7. The `cargo mutants` baseline over guarded layers with caught/missed counts and dispositions for misses.
8. The explicit non-certification statement (scoped evidence toward `ORD-LIFE-CERT` only).

## Files to Touch

- `reports/0016_ord_life_cert_scoped_acceptance.md` (new)

## Out of Scope

- Any implementation or test change (upstream tickets own all surfaces; if evidence assembly reveals a defect, it routes back to the owning ticket, not into this report).
- The `docs/4-specs/SPEC_LEDGER.md` row and the spec's move to `archive/specs/` — deferred to spec acceptance/archival per `docs/archival-workflow.md`.

## Acceptance Criteria

### Tests That Must Pass

1. Every command quoted in the report re-runs green against the implementation commits (spot-verified per section).
2. The report contains all eight §7 items, the deferral-ticket existence confirmation, and the non-certification statement verbatim in intent.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` (final whole-batch gate, run at artifact time).

### Invariants

1. The artifact claims scoped evidence only — no certification of `ORD-LIFE-CERT`, the full project, Phase 4 entry, or `FIRST-PROOF-CERT`.
2. Every evidence item cites a re-runnable surface (test name, fixture, command) — no unverifiable prose claims.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `test -f reports/0016_ord_life_cert_scoped_acceptance.md && test -f tickets/0016PHA3ANEEACC-015.md`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
