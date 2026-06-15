# 0032ARCTIETEM-005: A08 institutional procedural time + practical-bias discipline

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` (institutional/procedural-time contract + practical-bias discipline). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-002 (institution-known procedural time is a specialization of holder-known temporal claims, whose shape A03/A06 must own first). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T6 (report T6) + D-R6 (report R6), both landing in A08 and both about institution-known fallible procedure. Foundation `07` applies temporal authority to institutions: office hours, filing windows, due/lateness, queue aging, notice lifecycle, payment periods, case delay, and sanctions are institution-known/procedure-backed states, not truth labels (`INV-112`). The completeness determination separately routes practical bias / social harm to A08 as inspectable model-inputs/procedure-effects. A08 already owns roles, powers, records, procedures, institution-known context, fallibility, bias/error/corruption, reports, sanctions, and replayable records — but names neither a procedural-time authority seam nor a consolidated practical-bias discipline: verified 0 procedural-time matches in A08 at `ea6a05b`.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A08 owns institution-known context, procedures, records, fallibility, bias/error/corruption, sanctions, replayable records — but no procedural-time seam (`grep -niE "procedural time|office hour|queue ag|due/late"` → 0) and no consolidated practical-bias-as-inspectable-input statement. Execution `11` already says institutions mirror actor-known discipline; this ticket adds the architecture-tier seams, not execution mechanics.
2. Verified against spec 0032 §3.1 D-T6 + D-R6 and source report §5 Findings T6 / R6 / foundation `07`. Both homes are A08. Additive; relaxes nothing. Office-hour vocabulary, legal deadlines, payment periods, status enums, and the bias-assumption-review vehicle route out per spec §6.
3. Shared boundary under audit: A08 institution-known context / procedure. **Merge rationale**: D-T6 (procedural time) and D-R6 (practical bias) both land in A08 and are the same institution-known-fallibility theme (time and bias are both procedure-backed, inspectable, non-omniscient states) — co-locating them keeps A08 single-owned and the two consolidations mutually consistent. A08 is otherwise not a merge hub.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — institution-known procedural time (due/late/open/closed/…) is a record/procedure-backed state, not a truth label; procedure time is authoritative for the procedure's own lifecycle only through recorded rules and events, not hidden world truth. D-R6 carries the parallel non-omniscience principle for bias: the kernel asserts no moral truth.
5. Institution-known / no-hidden-truth surface (governed here; enforcement deferred to execution `11`/`10`): D-T6 requires procedural-time states to be event/record/procedure-backed institution-known state, with outputs that become actor knowledge/notices/reports/sanctions/payments/refusals/records preserving provenance and access context; D-R6 requires bias/social harm to be modeled as inspectable inputs / procedure effects, the kernel genre-neutral with no omniscient moral truth (domain packs own and must make reviewable their cultural/legal/institutional assumptions), and wrong suspicion/refusal/delay/sanction/misfiling/unequal treatment to arise from holder/institution-known evidence and procedure state. Adds doctrine only; no leakage path, no morality oracle.

## Architecture Check

1. D-T6 and D-R6 are merged into one ticket because both land in A08 and are the same institution-known-fallibility theme; merging keeps A08 single-owned and avoids a two-ticket merge hub on one doc. Both are reviewed coherently as one institution-procedure-doctrine update.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine consolidating institution-known fallibility already implicit in A08's bias/records/procedure contracts.

## Verification Layers

1. `INV-112` institutional procedural time (D-T6) → invariants alignment check: A08 gains the procedural-time contract (states as event/record/procedure-backed institution-known state; lifecycle authority not world-truth authority; outputs preserve provenance/access; bias/misfiling/delay/stale records are modeled procedure effects).
2. Practical-bias discipline (D-R6) → invariants alignment check: A08 gains the inspectable-inputs / procedure-effects / domain-pack-assumption-ownership statement, with no morality oracle and no objective social-harm quest condition.
3. Two contracts, one doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `11`/`10`); the layers above map each engaged surface to a distinct alignment proof.

## What to Change

### 1. D-T6 — institutional/procedural-time contract in A08

Add to `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`: a procedure may maintain due/late/open/closed/pending/delayed/filed/expired/paid/sanctioned/queue-aged states only as event/record/procedure-backed institution-known state; procedure time is authoritative for the procedure's own lifecycle only through recorded rules and events (it grants no hidden world truth); procedure outputs that become actor knowledge/notices/reports/sanctions/payments/refusals/records preserve provenance and access context; bias, misfiling, delay, underfunding, and stale records remain modeled procedure effects, not hidden moral labels. No office-hour vocabulary, legal deadlines, payment periods, or status enums.

### 2. D-R6 — practical-bias discipline in A08

Add: bias and social harm are modeled as inspectable inputs / social-position effects / resource constraints / procedure rules / testimony-credibility patterns / record-access patterns / institutional memory / staff-resource availability / prior decisions; the kernel stays genre-neutral and asserts no omniscient moral truth (domain packs own cultural/legal/institutional assumptions and must make them reviewable); wrong suspicion/refusal/delay/sanction/misfiling/unequal treatment arise from holder/institution-known evidence and procedure state (including biased/stale inputs when modeled); diagnostics expose the modeled assumptions and procedure steps without granting actors hidden truth or turning social harm into an objective quest condition.

## Files to Touch

- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` (modify)

## Out of Scope

- **Office-hour vocabulary, legal deadlines, payment periods, status enums, queue-aging values** — execution/implementation (spec §6).
- **Domain-pack bias-assumption review vehicle (metadata / risk notes / content review packets)** — execution/spec owner work (spec §6, report §7 Q11).
- **Procedural-time / wrong-suspicion proof procedures** — execution `11`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T6 landing grep** — `grep -niE "procedural time|institution-known|due|queue|provenance" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` resolves the procedural-time contract.
2. **D-R6 landing grep** — `grep -niE "bias|inspectable|assumption|genre-neutral|social harm" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` resolves the practical-bias discipline.
3. **Invariants alignment review** — D-T6 upholds `INV-112` (procedural time is record/procedure-backed, not world truth); D-R6 keeps the kernel genre-neutral with no morality oracle; no status-enum or vocabulary token introduced.

### Invariants

1. Procedural-time states are event/record/procedure-backed institution-known state; lifecycle authority is not world-truth authority; outputs preserve provenance/access (`INV-112`).
2. Bias/social harm are inspectable modeled inputs/procedure effects; the kernel asserts no omniscient moral truth and creates no objective social-harm quest condition (kernel genre-neutrality; no-director doctrine).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (two landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "procedural time|institution-known|due|queue|provenance" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — confirms D-T6 landed.
2. `grep -niE "bias|inspectable|assumption|genre-neutral|social harm" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — confirms D-R6 landed.

## Outcome

Completed: 2026-06-15

Implemented D-T6 and D-R6 in `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`. A08 now has a `Procedural time authority` subsection requiring due/late/open/closed/pending/delayed/filed/expired/paid/sanctioned/queue-aged procedure states to be event-, record-, or procedure-backed institution-known state, with lifecycle authority limited to recorded rules and events and no hidden world-truth grant. It also now states that procedure outputs preserve provenance and access context, and that bias, misfiling, delay, underfunding, and stale records are modeled procedure effects.

The practical-bias discipline was added under `Bias, error, and corruption`: bias/social harm are inspectable modeled inputs or procedure effects; the kernel stays genre-neutral with no omniscient moral truth; domain packs own reviewable cultural/legal/institutional assumptions; and wrong suspicion/refusal/delay/sanction/misfiling/unequal treatment must arise from holder- or institution-known evidence and procedure state.

The execution-blocking owner-approval precondition from spec 0032 §R-A is satisfied by the user's explicit `$ticket-series implement the series tickets/0032ARCTIETEM*` request for this architecture-tier amendment series.

Verification:

- `grep -niE "procedural time|institution-known|due|queue|provenance" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `grep -niE "bias|inspectable|assumption|genre-neutral|social harm" docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- Manual invariants alignment review: D-T6 preserves `INV-112` by making procedural time record/procedure-backed institution-known state rather than world truth; D-R6 preserves genre-neutral/no-director doctrine by refusing a morality oracle or objective social-harm quest condition.
- Manual mechanism-token boundary review: no office-hour vocabulary, legal deadline, payment-period value, concrete status enum, or queue-aging value was introduced.

No crate/code or fixture changes were made for this documentation-only architecture ticket.
