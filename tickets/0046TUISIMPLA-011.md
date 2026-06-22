# 0046TUISIMPLA-011: Acceptance-template parity evidence block + reference-tier amendments (0003, ref 00/01)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — acceptance-artifact template (`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`) and reference-tier docs (`docs/3-reference/00_*`, `01_*`); no code. `docs/3-reference/02_GLOSSARY.md` is route-forward only (see Assumption 2). Execution precondition: ordinary reference/specs-tier owner approval.
**Deps**: 0046TUISIMPLA-004, 0046TUISIMPLA-006, 0046TUISIMPLA-007

## Problem

Spec 0046 §5 `PAR-DOC-006`/`PAR-DOC-007`. The acceptance-artifact template needs a parity evidence
block so a feature's parity proof is captured uniformly, and the reference tier needs the review-checklist
question + the `R-15` risk substance updated to name both relapse modes. Without these, the standing
obligation (ticket 010) has no template slot and the risk register understates the failure surface.

## Assumption Reassessment (2026-06-22)

1. Verified the target docs exist at baseline `1145e109`:
   `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (sections include "Per-requirement acceptance
   evidence", "Evidence item ledger"), `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`,
   `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (`R-15 — TUI-First Playability Erosion` at `:206`),
   `docs/3-reference/02_GLOSSARY.md`. The coverage-report shape it references lands in ticket 004; the
   capability matrix in 006/007 (Deps).
2. Verified against spec 0046 §5: `PAR-DOC-006` adds a parity evidence block to 0003 (target
   implementation commit + fixture/content fingerprints; capability entries in scope; the generated
   coverage report; typed causal + actor-known witnesses; rendered golden paths/digests; anti-leak +
   debug-quarantine evidence; replay/no-human disposition; compiler/source-conformance evidence; exact
   commands + verdicts; must not reduce to screenshots/display strings). `PAR-DOC-007` — ref `00`
   checklist gains "For every capability added or changed, where is its declared surface disposition and
   real-pipeline actor-filtered witness?"; update the **substance** of existing `R-15` to name both
   relapse modes (projection blindness and renderer blindness) and the expected controls; **mint no new
   risk ID**. The glossary term (ref `02`) is route-forward: add it only after the owner ratifies the
   final mechanism name (§9.7) — so ref 02 is NOT edited in this ticket; the choice-and-record obligation
   (term options + that 02 is the recording destination) is carried here for the owner.
3. Shared boundary under audit: the 0003 template is consumed by every future acceptance artifact
   (including this spec's own, ticket 012); ref 00/01 are the review-checklist + risk authorities. This
   is a cross-artifact doctrine ticket.
4. Invariant restated (`PAR-DOC` motivation): `INV-095` TUI/view-model tests are acceptance tests;
   `INV-093` actor-knowledge leakage is a high-severity defect (the R-15 relapse modes are leakage/
   blindness shapes). No invariant amended; no new risk ID minted (spec §1.2).
5. Enforcement surface governed (doctrine, substrate basis): the 0003 parity block governs what
   acceptance evidence must include (anti-leak + debug-quarantine + replay/no-human), and R-15 governs
   the relapse-risk controls. Confirm the block requires actor-filtered witnesses and forbids reducing
   the package to display strings — strengthening the no-leak posture, introducing no leakage path.

## Architecture Check

1. Extending the existing 0003 template (rather than a bespoke parity artifact) keeps acceptance
   evidence uniform and reviewable, and updating R-15's substance (rather than minting a new risk ID)
   matches the spec's "no new risk identifier" constraint while naming both relapse modes the contract
   defends against. Deferring the glossary term until owner ratification avoids premature naming (§9.7).
2. No backwards-compatibility aliasing/shims: template + reference prose only; no parallel artifact
   format or duplicate risk entry.

## Verification Layers

1. `PAR-DOC-006` → grep-proof on 0003: the parity evidence block enumerates commit/fingerprints,
   capability entries, coverage report, typed + actor-known witnesses, rendered golden paths/digests,
   anti-leak + debug-quarantine evidence, replay/no-human disposition, compiler/source-conformance
   evidence, exact commands + verdicts, and the no-screenshots clause.
2. `PAR-DOC-007` → grep-proof on ref 00: the per-capability surface-disposition + real-pipeline witness
   checklist question is present. Grep-proof on ref 01: R-15 names both relapse modes (projection
   blindness + renderer blindness) and controls; no new `R-NN` ID added.
3. Invariants-alignment review: ref 02 unchanged (glossary term deferred); no new risk ID.

## What to Change

### 1. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (`PAR-DOC-006`)

Add a parity evidence block per Assumption 2 (commit + fingerprints, capability entries, coverage
report, typed/actor-known witnesses, rendered golden paths/digests, anti-leak + debug-quarantine
evidence, replay/no-human disposition, compiler/source-conformance evidence, exact commands + verdicts;
must not reduce to screenshots/display strings).

### 2. `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (`PAR-DOC-007`)

Add the checklist question: "For every capability added or changed, where is its declared surface
disposition and real-pipeline actor-filtered witness?"

### 3. `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (`PAR-DOC-007`)

Update the substance of `R-15 — TUI-First Playability Erosion` to name both relapse modes — projection
blindness (the projection omits an actor-known fact) and renderer blindness (the view model carries it
but the renderer drops it) — and the expected controls (Hop-1 goldens + Hop-2 compile guards). Mint no
new risk ID.

### 4. Glossary term — deferred (route-forward, `PAR-DOC-007` / §9.7)

Do NOT add a `02_GLOSSARY.md` term now. Record (here, for the owner) that the final mechanism name
("playable-capability parity contract" is a working label) is an owner decision; once ratified, the term
is added to `02_GLOSSARY.md` in a follow-up.

## Files to Touch

- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- `docs/3-reference/02_GLOSSARY.md` — deferred until the owner ratifies the mechanism name (§9.7);
  route-forward follow-up, not this ticket.
- Architecture/execution-tier amendments (009/010).
- Any new risk ID or constitutional-invariant amendment (spec §1.2 forbids).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: 0003 carries the parity evidence block with every enumerated element + the
   no-screenshots clause; ref 00 carries the per-capability checklist question; R-15 in ref 01 names
   both relapse modes + controls.
2. Grep-proof: no new `R-NN` risk ID added; `02_GLOSSARY.md` is unchanged (term deferred).

### Invariants

1. Acceptance evidence for any feature with parity impact must include actor-filtered witnesses and may
   not reduce to display strings.
2. R-15 substance is updated in place; no new risk ID, no invariant amendment.

## Test Plan

### New/Modified Tests

1. `None — documentation-only doctrine amendment; verification is grep-based landing + invariants-
   alignment review. Pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "parity|surface disposition|actor-filtered witness|projection blindness|renderer blindness" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md docs/3-reference/01_DESIGN_RISK_REGISTER.md`
2. `grep -cE "^### R-[0-9]+" docs/3-reference/01_DESIGN_RISK_REGISTER.md` (confirm risk-ID count unchanged; no new ID)
