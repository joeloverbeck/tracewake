# 0028EXETIEDOC-007: Phase-4 institution-known provenance and freshness future-proofing

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (a Phase-4 provenance/freshness future-proofing obligation). No crate/code, no fixtures.
**Deps**: 0028EXETIEDOC-003 (this obligation inherits the `04`/`10` provenance-sufficiency + freshness fail-closed mechanics that ticket 003 establishes). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D10 (report E09). Architecture `03` (ratified by spec 0027) now states institution-known contexts are **not exempt** from provenance sufficiency, and architecture `06` requires provenance + freshness for beliefs/traces/records. Execution `11` defines Phase-4 entry, institution-known context, record surfaces, and the wrong-suspicion lock — it is pointed the right way — but its dependency/proof language does not explicitly import the shared provenance-sufficiency/freshness fail-closed mechanics, nor require live negatives for record truth-conversion, stale institutional records, dangling record provenance, or institution reactions to raw event-log truth (verified: 0 `provenance sufficien`/`fail-closed`/`freshness classifier` matches in `11` at `64a8367`). This ticket adds a Phase-4 provenance future-proofing obligation that locks the proof contract before Phase-4 institution behavior expands, without expanding Phase-4 scope.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` defines Phase-4 entry, institution-known context, record surfaces, and the wrong-suspicion lock, but `grep -inE "provenance sufficien|fail.closed|freshness classifier|truth-conversion"` returned 0 — it does not import the shared provenance-sufficiency/freshness mechanics from `04`/`10`.
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D10 and §7 R-D, and the source report `reports/execution-tier-alignment-research-report.md` §E09. Upstream contracts: architecture `03` (institution-known not exempt) + `06` (provenance/freshness), ratified by `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`. The `04`/`10` fail-closed mechanics this obligation inherits are established by sibling ticket 0028EXETIEDOC-003 (D3+D4) — hence the `Deps`.
3. Shared boundary under audit: the Phase-4 institution-known proof surface (`11`) and the actor-known provenance-sufficiency/freshness mechanics (`04`/`10`, ticket 003) it must inherit. `11` imports the mechanics for institution-known facts; it does not redefine them or design new institution mechanics.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-102` (cognition inputs — including institutional assignments, records, reports — require provenance sufficient for replay/debug; missing provenance is a rejection condition) and `INV-049` (institutions are not omniscient; they act from records, reports, testimony, procedures — not hidden truth). Foundation `07` requires institutions to be fallible social machinery, not omniscient truth channels. D10 locks the institution-known proof contract before Phase-4 expands.
5. Fail-closed-validation + no-leak surface (the enforcement surfaces are the institution-known proposal/validation path and the `04`/`10` mechanics from ticket 003; the institution machinery itself is deferred Phase-4 work): the obligation requires any institution-known fact, record-derived belief, norm-triggered procedure, wrong-suspicion inference, or artifact interpretation to prove provenance sufficiency and freshness using the **same fail-closed mechanics as `04`/`10`**, with negative fixtures for institution-reacts-to-truth-without-record, stale-record-treated-as-fresh, dangling-record-provenance, wrong-kind-provenance, and record-display-text-mistaken-for-provenance. This strengthens the no-leak firewall for institutions (INV-049/024/102); it expands no Phase-4 scope and designs no institution mechanics — proof-contract doctrine only.

## Architecture Check

1. The future-proofing obligation lives in `11` because `11` owns Phase-4 entry proof and the institution records / wrong-suspicion fixtures; importing the `04`/`10` mechanics there locks the proof contract at the right surface before Phase-4 machinery exists. It does not decide new institution mechanics (those remain deferred Phase-4 work).
2. No backwards-compatibility aliasing/shims: additive proof obligation that inherits existing `04`/`10` mechanics by reference rather than re-stating them; no existing `11` lock is weakened.

## Verification Layers

1. `INV-102`/`INV-049` institution-known provenance sufficiency → invariants alignment check + codebase grep-proof: `11` requires institution-known facts/records/norm-procedures/wrong-suspicion inferences/artifact interpretations to prove provenance sufficiency + freshness via the `04`/`10` fail-closed mechanics.
2. Negative fixtures → manual review (no-leak negative-case audit): `11` enumerates institution-reacts-to-truth-without-record, stale-record-treated-as-fresh, dangling-record-provenance, wrong-kind-provenance, and record-display-text-mistaken-for-provenance.
3. Scope containment → manual review: the obligation locks the proof contract only; it expands no Phase-4 scope and designs no institution mechanics.

## What to Change

### 1. `11` — Phase-4 provenance/freshness future-proofing obligation (D10)

Add an obligation (final wording at enactment): any institution-known fact, record-derived belief, norm-triggered procedure, wrong-suspicion inference, or artifact interpretation must prove provenance sufficiency and freshness using the **same fail-closed mechanics as `04`/`10`** (ticket 003). Add negative fixtures: institution reacts to truth without a record; stale record treated as fresh; dangling record provenance; wrong-kind provenance; record display text mistaken for provenance. Lock the proof contract before Phase-4 entry; expand no Phase-4 scope and decide no new institution mechanics.

## Files to Touch

- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (modify)

## Out of Scope

- **The `04`/`10` provenance-sufficiency + freshness mechanics themselves** — D3+D4, sibling ticket 0028EXETIEDOC-003 (this ticket inherits them by reference).
- **Expanding Phase-4 scope or designing new institution mechanics** — Phase-4 implementation remains deferred (spec §3 D10 / §7 R-D).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Future-proofing landing grep** — `11` imports the fail-closed provenance/freshness mechanics for institution-known facts: `grep -niE "provenance sufficien|fail.closed|freshness|institution-known" docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` resolves the obligation.
2. **Negative-fixtures landing grep** — `11` enumerates the five negative cases: `grep -niE "without a record|stale record|dangling|wrong-kind|display text" docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` resolves the negative set.
3. **Scope-containment review** — the obligation expands no Phase-4 scope and designs no new institution mechanics; it inherits the `04`/`10` mechanics by reference.
4. **Invariants alignment review** — D10 future-proofs institution-known provenance/freshness (INV-102/049) before Phase-4 expands.

### Invariants

1. Institution-known facts/records/procedures/inferences prove provenance sufficiency + freshness via the same fail-closed mechanics as `04`/`10` (INV-102); institutions are not omniscient truth channels (INV-049).
2. The obligation locks the proof contract only — no Phase-4 scope expansion, no new institution mechanics.

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (future-proofing + negative-fixtures landing greps) plus a scope-containment / invariants-alignment manual review against architecture 03/06. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "provenance sufficien|fail.closed|freshness|institution-known" docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — confirms the future-proofing obligation landed.
2. `grep -niE "without a record|stale record|dangling|wrong-kind|display text" docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — confirms the five negative fixtures.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the scope-containment / invariants-alignment review against architecture 03/06.`
