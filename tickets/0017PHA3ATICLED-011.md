# 0017PHA3ATICLED-011: Scoped acceptance artifact for ORD-LIFE-CERT evidence

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — verification-only capstone; one new report document
**Deps**: `archive/tickets/0017PHA3ATICLED-010.md` (transitively covers `-001`…`-009`; the corrected 0016 report is itself an evidence item); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (§7)

## Problem

Spec 0017 §7 requires a scoped acceptance artifact recording, for the implementation commits, the evidence that each finding's correction and lock actually landed and can fail — the lineage's defense against exactly the hollow-delivery pattern ORD-HARD-029/034 exposed in the 0016 cycle. Without the artifact, the batch's golden-checksum reprice and provenance-class shifts are unreviewable diffs.

## Assumption Reassessment (2026-06-10)

1. Verified against current repo state: the artifact convention and naming follow `reports/0014_ord_life_cert_scoped_acceptance.md` / `0015_…` / `0016_…` (all present); the mutants baseline precedent is `reports/0016_ord_hard_025_mutants_baseline.md`. Every §7 evidence item maps to a producing ticket in this batch: need ledgers (`-002`), provenance-class and witness proofs (`-003`/`-004`), tamper-flips + capstone gate (`-005`), embodied divergence pair (`-006`), numeric census + rejections (`-007`), mutants pin/diff-gate + miss-set (`-008`), generative corpus summary (`-009`), corrected 0016 report (`-010`).
2. Spec 0017 §7 enumerates the nine required sections, including the explicit non-certification statement (scoped evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`).
3. Cross-artifact boundary under audit: the acceptance artifact ↔ ticket-evidence contract — every claim in the artifact must cite a runnable command, a named test, or a committed data file produced by the named ticket; no claim may rest on prose alone (the ORD-HARD-034 lesson).
4. INV-105 restated: acceptance evidence is authoritative diagnostic data — this artifact records only what was run and observed at the implementation commits, with commands reproducible by a reviewer.
5. Adjacent items classified as future work outside this batch: the `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move happen at spec acceptance/archival (hardening-series convention; cross-spec follow-up), and the spec's recommended sixth verification-only audit is a future brainstorm, not a ticket.

## Architecture Check

1. A single trailing verification-only capstone (no new production logic; the report IS the deliverable) matches the spec lineage's acceptance pattern and the decomposition contract for capstone tickets — it exercises every prior ticket end-to-end rather than modifying them. The hybrid structure (CI-runnable assertions + implementer runbook for evidence gathering) is required because ledger diffs and the mutants baseline are run-and-record steps, not test assertions.
2. No backwards-compatibility aliasing/shims: not applicable — verification-only.

## Verification Layers

1. Evidence completeness -> manual review against spec §7: all nine sections present, each citing its producing ticket and a reproducible command or committed data file.
2. Gate evidence -> replay/golden-fixture checks re-run at the final tree: the context-hash gate, tamper-flips, occurrence-count reconciliation, and generative corpus all green in one `cargo test --workspace` run recorded in the artifact.
3. Honesty of negative evidence -> each "the gate can fail" claim (tamper tests, seeded guard violation, provenance fail-closed cases) cites the named failing-mode test, not prose.
4. Single-layer justification: this ticket adds no production surface; its verification IS the layer mapping above — no further per-invariant decomposition applies.

## What to Change

### 1. Runbook (implementer-checklist; the artifact records outcomes)

1. At the post-`-010` tree, run the full gate set and record versions/commit.
2. Gather per-actor need ledgers for `no_human_day_001` and `wait_then_window_passive_charges_each_tick_once_001`, before/after `-002`, explaining every golden checksum diff.
3. Record the provenance-class proof (aged record as `remembered_belief` with original tick, live and replayed) and both fail-closed cases (`provenance_class_mismatch`, `provenance_witness_mismatch`).
4. Record both tamper-flip runs poisoning replay and the capstone run with the context-hash assertion active.
5. Record the embodied divergence pair renders (belief-closed/truth-open; belief-open/truth-closed with the commit consequence).
6. Record the numeric-policy census output and the named rejection tests.
7. Record the generative corpus summary: seed list, sequence count, reachability counters, zero differential divergences, metamorphic relations passing.
8. Run the pinned mutants job against the finished tree; record the diff against `.cargo/mutants-baseline-misses.txt` with dispositions for any change.
9. Include the corrected-0016-report citation and the explicit non-certification statement.

### 2. The artifact

Write `reports/0017_ord_life_cert_scoped_acceptance.md` with the nine §7 sections in spec order.

## Files to Touch

- `reports/0017_ord_life_cert_scoped_acceptance.md` (new)

## Out of Scope

- Any production, test, fixture, or CI modification (a failure discovered here routes back to the owning ticket — this ticket does not fix surfaces).
- `docs/4-specs/SPEC_LEDGER.md` row and `archive/specs/` archival (spec-acceptance follow-up).
- Certification claims beyond the scoped non-certification statement.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f reports/0017_ord_life_cert_scoped_acceptance.md` and the artifact contains all nine spec §7 sections, each citing a reproducible command, named test, or committed data file.
2. `grep -q "not Phase 4 entry" reports/0017_ord_life_cert_scoped_acceptance.md` — the non-certification statement is verbatim-present in scope.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — recorded green at the artifact's stated commit.

### Invariants

1. Every artifact claim is reproducible: command, named test, or committed data file — no prose-only evidence (the ORD-HARD-034 lesson applied to this cycle's own artifact).
2. The artifact certifies nothing beyond scoped `ORD-LIFE-CERT` evidence.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.` (The capstone exercises tests landed by `-001`…`-009`; it adds none.)

### Commands

1. `cargo test --workspace`
2. `grep -c "##" reports/0017_ord_life_cert_scoped_acceptance.md` — section census against spec §7's nine items.
