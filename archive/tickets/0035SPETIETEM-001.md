# 0035SPETIETEM-001: specs 0003 template — staged-abstraction declaration area

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — template/doctrine edit to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (additive staged-abstraction declaration area over the existing epoch-1 evidence-honesty fields). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0035 §7 R-A (specs tier-4 self-amendment to the `0003` template; ordinary owner approval, not by convention).

## Problem

Spec 0035 D-S1 (report Finding 1; the `0033` D-S1 / execution F-13 hand-off). The `0003` acceptance-artifact template owns the epoch-1 evidence-honesty fields (evidence status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical, certification use) plus `EMERGE-OBS` observer-only handling, but carries **no** staged-abstraction declaration area: verified 0 matches for `proves now` / `must not fake` / `staged.abstraction` / `deliberately abstract` in `0003`. Execution `10` ("Temporal, Completeness, Fairness, and Staged-Abstraction Diagnostics") requires staged-abstraction review artifacts to state six declaration categories and marks them observer-only / non-certifying / mint-no-gate-code; `0033` D-S1 (F-13) routes the concrete acceptance-artifact *template* addition to the specs tier. Without the declaration area, a staged acceptance artifact can bury an abstraction and falsely certify omitted future behavior by implication. This ticket authors the declaration area additively, never renaming or weakening an existing epoch-1 field or `EMERGE-OBS` handling.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`HEAD` `7be2290`): `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` owns the epoch-1 evidence-honesty fields (the evidence-item ledger: evidence status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical handling, certification use) plus `EMERGE-OBS` observer-only handling, and carries no staged-abstraction declaration area (`grep -niE 'proves now|must not fake|staged.abstraction|deliberately abstract' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` → 0). The gap is real; this ticket adds the declaration area, it does not restate execution doctrine.
2. Verified against spec 0035 §3.1 D-S1 and the ratified upstream: execution `10` "Temporal, Completeness, Fairness, and Staged-Abstraction Diagnostics" lists the six categories and marks them observer-only / non-certifying / no-new-gate-code (the `0033` Block-S enactment); reference `0034` (enacted) added the staged-abstraction / false-certification glossary terms and the additive evidence-honesty risk note that this area's subordination clause points to. The specs-tier `0003` edit **is** the concrete enactment that `0033`/execution-F-13 routed forward, so authoring the template wording here is in-scope — what is forbidden is minting a new gate/obligation code or restating the gate as if the template defined it (spec §3.1, §6).
3. Cross-artifact shared boundary under audit: the `0003` template (specs tier-4) is the **consumer** of execution `10`'s staged-abstraction requirement (the gate definition) and reference `0034`'s staged-abstraction terms (the vocabulary). This ticket adds the declaration area that operationalizes the execution requirement and cites the reference terms; it must not redefine or re-mint the gate. Boundary verified: the template declares, it does not define doctrine.
4. Constitutional invariant motivating this ticket, restated before trusting the narrative: `INV-111` (living-world acceptance requires observer-only emergence evidence — "That evidence must be retrospective, explainable through event-log ancestry, and unable to feed simulation behavior, author outcomes, or set dramatic objectives"). The staged-abstraction declaration area is observer-only and certifies nothing by itself; it operationalizes this for the acceptance template and weakens no invariant.
5. Governed enforcement surface (doctrine/template ticket, no code): the declaration area feeds the acceptance-evidence-honesty / false-certification surface that execution `10` governs. Per the Substrate-only rule, item 5 applies: the area names what an artifact "must not fake" while abstracting and states the declaration is observer-only / non-certifying / certifies-nothing-by-itself, so it introduces no path by which a staged abstraction could be counted as certifying evidence, leak ground/debug/fixture truth into a pass claim, or override the template's existing "compute the requirement result only from certifying evidence" rule. The enforcing surface is execution `10` (already ratified); this ticket adds the template-level declaration that surface requires.

## Architecture Check

1. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is the correct single home: execution `10` and `0033` D-S1/F-13 name the acceptance-artifact template by path as the route target, and `0003` already owns the evidence-honesty / certification-use material the declaration area sits beside. Authoring it here — not a new file, not a new gate — preserves the single-template discipline and the operationalize-don't-define boundary the specs tier is bound to.
2. No backwards-compatibility aliasing/shims: additive declaration area over the existing epoch-1 fields; no rename, weakening, or redefinition of the evidence-status, fingerprint-scope, certification-use, or `EMERGE-OBS` material.

## Verification Layers

1. `INV-111` observer-only / non-certifying → invariants alignment check + manual review: the declaration area states it certifies nothing by itself and the fields are observer-only / non-certifying unless a future scoped spec promotes a specific check, so no field becomes a certifying pass/fail gate, scheduler objective, or scenario goal.
2. Execution-`10` fidelity → codebase grep-proof + manual review: the six categories (proves-now, deliberately-abstracts, must-not-fake, must-not-block, overclaim-prevention, failure-diagnostics) are present and track execution `10`'s list; no new gate/obligation code, status enum, or fingerprint scope is minted.
3. Mechanism-token boundary → manual review: the authored area names no concrete temporal value, schema, threshold, or fixture name; the four-way failure-diagnostic distinction ("not implemented yet" / "intentionally abstracted" / "implemented but broken" / "overclaimed") is review *shape*, not a new status vocabulary.
4. Documentation-only template ticket: no replay/golden-fixture or skill-dry-run layer applies (those are code/fixture surfaces); the layers above map each engaged invariant/contract to a distinct review surface.

## What to Change

### 1. Add a staged-abstraction declaration area to `0003`

Add a section (sited near the evidence-item ledger / certification-use material, whose job is the same — preventing false certification) requiring a future acceptance artifact that stages a bounded abstraction to declare the substance of:

- (a) what the artifact **proves now**;
- (b) what it **deliberately abstracts**;
- (c) what the implementation/proof **must not fake** while abstracting;
- (d) what future feature or tier the abstraction **must not block**;
- (e) what evidence **prevents overclaiming** from the current artifact;
- (f) what **failure diagnostics** distinguish "not implemented yet," "intentionally abstracted," "implemented but broken," and "overclaimed."

State explicitly that the declaration **certifies nothing by itself**, that these fields are **observer-only and non-certifying** unless a future scoped spec promotes a specific check through the normal authority chain, and that the area remains **subordinate to execution `10` and the reference staged-abstraction terms/risk (`0034`)**. Mint no new gate code, obligation code, status enum, or fingerprint scope; do not duplicate or restate execution `10` as if the template defined the gate; do not re-open or rename any epoch-1 evidence-honesty field or `EMERGE-OBS` handling.

(Owner-decision per spec §7: the exact placement — a standalone section vs. folded beside the certification-use wording — and the final field phrasing are authored at implementation; record the decision in the diff.)

## Files to Touch

- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify)

## Out of Scope

- **New gate codes, obligation codes, status enums, fingerprint scopes** — minting any is forbidden (spec §6); the area declares, it does not define a gate.
- **Concrete temporal values, schemas, thresholds, fixtures, depth** — the §3.2 backlog / future scoped specs (spec §6).
- **Edits to `README.md`, `SPEC_LEDGER.md`, or `0001`** — boundary-awareness no-change verdicts (spec §6 / V2).
- **The `SPEC_LEDGER.md` row for spec 0035** — lands at acceptance/closeout per the staged-spec convention (spec §8), not this ticket.
- **Re-opening epoch-1 `0003` content** — the evidence-status / fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical / certification-use fields and `EMERGE-OBS` handling are baseline and untouched.
- **Owner approval itself (spec §7 R-A)** — execution precondition, not a deliverable.
- Crate/code, fixtures; foundation/architecture/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Declaration-area landing grep** — `grep -niE 'proves now|deliberately abstract|must not fake|must not block|overclaim|failure diagnostic' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the six categories.
2. **Non-certification clause grep** — `grep -niE 'certifies nothing|observer-only|non-certifying|subordinate' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` resolves the subordination / non-certification statement.
3. **Mechanism-token / new-gate boundary review** — manual review confirms the added area mints no new gate code, obligation code, status enum, or fingerprint scope, and names no concrete temporal value, schema, threshold, or fixture name.
4. **Epoch-1 preservation review** — `grep -niE 'Evidence status|Fingerprint scope|Certification use|EMERGE-OBS' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` still resolves and the baseline field definitions are unchanged (no rename/weaken).

### Invariants

1. The staged-abstraction declaration is observer-only and certifies nothing by itself; no field becomes a certifying pass/fail gate, scheduler objective, or scenario goal (`INV-111`).
2. The template operationalizes execution `10` without redefining it — no new gate/obligation code, status enum, or fingerprint scope is minted, and the area stays subordinate to execution `10` and reference `0034` (specs-tier authority boundary, `docs/4-specs/README.md`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only specs-tier template ticket; verification is command-based (landing greps) plus a mechanism-token/new-gate boundary, epoch-1-preservation, and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'proves now|deliberately abstract|must not fake|must not block|overclaim|failure diagnostic' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — confirms the six declaration categories landed.
2. `grep -niE 'certifies nothing|observer-only|non-certifying|subordinate' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — confirms the non-certification / subordination clause.
3. `git diff --check` — whitespace/conflict-marker hygiene on the doc edit; the Rust pipeline is unaffected, so the verification boundary is the landing greps plus the boundary / preservation / alignment review.

## Outcome

Completed: 2026-06-16

The owner-approval precondition from spec 0035 §7 R-A was satisfied by the
current user request to implement ticket `0035SPETIETEM-001`.

Changed `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` by adding a compact
`Staged-abstraction declaration` area directly after the evidence-item ledger
and `EMERGE-OBS` / certification-use cluster. The new area requires future
acceptance artifacts using bounded staged abstractions to state what they prove
now, deliberately abstract, must not fake, must not block, what evidence
prevents overclaiming, and what failure diagnostics distinguish not implemented,
intentionally abstracted, implemented-but-broken, and overclaimed cases.

The addition explicitly says the declaration certifies nothing by itself, is
observer-only and non-certifying unless a future scoped spec promotes a specific
check through the normal authority chain, and remains subordinate to execution
`10` plus the reference staged-abstraction terms and risk memory enacted by
archived spec `0034`. Manual review found no new gate code, obligation code,
status enum, fingerprint scope, concrete temporal value, schema, threshold, or
fixture name. The existing epoch-1 evidence-status, fingerprint-scope,
certification-use, and `EMERGE-OBS` template material was preserved unchanged.

Verification run:

- `grep -niE 'proves now|deliberately abstract|must not fake|must not block|overclaim|failure diagnostic' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `grep -niE 'certifies nothing|observer-only|non-certifying|subordinate' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `grep -niE 'Evidence status|Fingerprint scope|Certification use|EMERGE-OBS' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `git diff --check`

No ticket acceptance checks were skipped.
