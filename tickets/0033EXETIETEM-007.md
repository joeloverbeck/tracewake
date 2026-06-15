# 0033EXETIETEM-007: exec 11 procedural-time & practical-bias Phase-4 proof

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (additive Phase-4 procedural-time evidence and practical-bias/social-harm proof over the existing Phase-4 institution entry locks). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-010 (exec `10` procedural-time + bias diagnostics; this proof cross-references the per-cause diagnostic home). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T6, D-R4 (report F-06, F-11), the exec-`11` slices. Exec `11` owns Phase-4 institution entry, institution-known context, record status, wrong-suspicion causality, and locked fixtures, but carries **no** procedural-time evidence obligation and no explicit practical-bias proof: verified 0 `procedural.time`/`temporal`/`INV-112` matches in exec `11` at `c70d119`. Without them an institution label like "late"/"closed"/"sanctioned" can be an omniscient truth label, and bias can be authored by fiat. This ticket requires every institutional temporal status to be record/procedure-backed and requires practical-bias outcomes to flow through modeled institutional mechanics — additively over the Phase-4 entry locks.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `11` (`11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`) owns the Phase-4 entry lock, institution-known context, record time/status, wrong-suspicion causes, and locked fixtures, and carries no procedural-time or practical-bias proof obligation (`grep -ciE 'procedural.time|temporal|INV-112' docs/2-execution/11_*` → 0). The gap is real; this ticket adds Phase-4 proof obligations.
2. Verified against spec 0033 §3.1 D-T6 and §3.2 D-R4, and ratified upstream A08 institutional/procedural-time + practical-bias seams (spec `0032`). Office-hour vocabulary, legal/procedural deadlines, status enums, and concrete domain-pack bias assumptions route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `11` (Phase-4 procedural-time + bias proof) ↔ exec `08` (content packs must not author omniscient procedural-time conclusions, ticket 005) ↔ exec `10` (record-error / procedure-delay / staleness / contradictory-testimony / missing-artifact / hidden-truth-leakage diagnostics, ticket 010). This ticket states the Phase-4 proof obligation; authoring validation and diagnostics live in `08`/`10`.
4. Constitutional invariants motivating this ticket, restated: `INV-112` (institution-known procedural time is modeled, record/procedure-backed — not omniscient labels); `INV-111` (observer-only emergence — social harm must not become an objective quest condition); the fallible-institution doctrine (no omniscient moral/truth shortcut). The obligations specialize these for Phase-4 institutions.
5. Actor-/institution-knowledge / fail-closed surface: the obligations require (a) Phase-4 procedural-time evidence for any institution label with time semantics (open/closed, due/late, expired/current, pending/resolved, queued/aged, notified/served, paid/unpaid, sanctioned, appealed), each backed by a modeled institution-known source (record, schedule artifact, notice, ledger entry, procedure state, testimony accepted by procedure, inspection, modeled staff action); (b) adversarial institution fixtures where true time would justify a label but records do not yet support it, and where stale/mistaken records cause plausible institutional error; (c) Phase-4 practical-bias evidence where institutional outcomes may be shaped by modeled testimony quality, unequal credibility, access barriers, underfunding, refusal, delay, misfiling, contradictory/stale/suppressed records, or staff procedure; and (d) wrong-suspicion proof showing the actor-/institution-known path that made a suspicion plausible without consulting hidden culprit truth. This strengthens epistemic-leakage prevention and the genre-neutral, no-morality-oracle posture; it introduces no determinism change.

## Architecture Check

1. Exec `11` is the correct home: it already owns Phase-4 institution entry and wrong-suspicion causality, so procedural-time evidence and practical-bias proof are specializations of contracts `11` already carries. Splitting them would divorce institutional time/bias from the institution-known gate that certifies them.
2. No backwards-compatibility aliasing/shims: additive Phase-4 proof obligations over the existing entry locks; no rename, no weakening, no calendar/legal vocabulary or status enum chosen.

## Verification Layers

1. `INV-112` procedural time → invariants alignment check: exec `11` requires each institutional temporal status to be backed by a modeled institution-known source, with adversarial fixtures for records-lag and stale/mistaken records.
2. `INV-111` / fallible-institution practical bias → invariants alignment check: exec `11` requires bias/social-harm outcomes to flow through modeled testimony/credibility/access/underfunding/refusal/delay/misfiling/records/staff procedure, with no objective social-harm quest condition.
3. Wrong-suspicion firewall → invariants alignment check: wrong-suspicion proof shows the actor-/institution-known path without hidden culprit truth.
4. Documentation-only doctrine ticket: authoring validation is exec `08` (ticket 005) and the per-cause diagnostics are exec `10` (ticket 010); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D-T6 — Phase-4 procedural-time evidence

Require Phase-4 procedural-time evidence for any institution label with time semantics (open/closed, due/late, expired/current, pending/resolved, queued/aged, notified/served, paid/unpaid, sanctioned, appealed), each backed by a modeled institution-known source (record, schedule artifact, notice, ledger entry, procedure state, testimony accepted by procedure, inspection, modeled staff action). Require adversarial institution fixtures where true time would justify a label but records do not yet support it, and where stale/mistaken records cause plausible institutional error. Cross-reference exec `08` (no authored omniscient procedural-time) and exec `10` (per-cause diagnostics). Choose no office-hour vocabulary, legal deadlines, payment periods, or status enums (§6).

### 2. D-R4 — practical-bias / social-harm proof

Require Phase-4 practical-bias evidence where institutional outcomes may be shaped by modeled testimony quality, unequal credibility, access barriers, underfunding, refusal, delay, misfiling, contradictory records, stale records, suppressed/ignored records, or staff procedure; require wrong-suspicion proof to show the actor-/institution-known path that made a suspicion plausible without consulting hidden culprit truth. Preserve the genre-neutral kernel — no morality oracle, no objective social-harm quest condition. Concrete domain-pack bias assumptions route to future scoped specs (§6); the bias-assumption authoring-validation requirement is exec `08` (ticket 005).

## Files to Touch

- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` (modify)

## Out of Scope

- **Office-hour vocabulary, legal/procedural deadlines, payment periods, status enums** — future scoped specs (§6).
- **Concrete domain-pack bias assumptions / evaluation criteria** — future scoped specs (§6); **bias-assumption authoring validation** — exec `08` (ticket 005); **per-cause diagnostics** — exec `10` (ticket 010).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code or status enum.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T6 landing grep** — exec `11` carries procedural-time evidence: `grep -niE 'procedural.time|institution.*(record|procedure).*time|due/late' docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` resolves it.
2. **D-R4 landing grep** — practical-bias proof present: `grep -niE 'practical bias|social harm|misfiling|credibility' docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` resolves it.
3. **Mechanism-token + no-oracle review** — no office-hour vocabulary, legal deadline, payment period, or status enum entered exec `11`, and no morality oracle / objective social-harm quest condition; no new gate code.
4. **Invariants alignment review** — upholds `INV-112`/`INV-111` and the fallible-institution doctrine, preserves the Phase-4 entry locks (no rename/weaken).

### Invariants

1. Every institutional temporal status is record/procedure-backed institution-known state, never an omniscient label (`INV-112`).
2. Bias/social harm flows through modeled institutional mechanics with no morality oracle or objective quest condition; wrong suspicion uses no hidden culprit truth (`INV-111`, fallible institutions).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a mechanism-token/no-oracle and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'procedural.time|due/late|practical bias|social harm|misfiling' docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — confirms D-T6/D-R4 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the no-oracle and invariants-alignment review.`
