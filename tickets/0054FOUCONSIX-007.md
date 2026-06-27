# 0054FOUCONSIX-007: Below-foundation doctrine strengthening (operationalizing doctrine)

**Status**: PENDING
**Priority**: MEDIUM
**Engine Changes**: None — doctrine-doc edits only (`docs/1-architecture/13`, `docs/2-execution/10`, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, `docs/3-reference/00`, `docs/3-reference/01`); ordinary-owner approval precondition
**Effort**: Medium
**Deps**: 0054FOUCONSIX-004, 0054FOUCONSIX-005, 0054FOUCONSIX-006

## Problem

The acceptance/governance/mutation evidence-honesty rules exist in prose but are not yet operationalized as executable doctrine bound to the F6-04/F6-05/F6-06 machinery. This line warrants doctrine strengthening **below foundation** — substance and home only, at ordinary-owner approval altitude — so the rules are executable, not merely written (spec §6.1). **No Tier-0 constitutional amendment is warranted** (driver §9): the constitution already forbids forgeable bootstraps, debug-grade embodied receipts, publicly-inducible debug authority, self-scored pass, and implementer-as-acceptor governance. This spec mints no invariant, gate code, risk ID, or glossary term, and authors no ratified wording.

## Assumption Reassessment (2026-06-27)

1. `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, and `docs/3-reference/01_DESIGN_RISK_REGISTER.md` all exist at `7660051`; `01` carries `R-27`/`R-28`/`R-29` (`:333`/`:345`/`:358`). Confirmed.
2. This is the **operationalizing-doctrine** case (doctrine-with/after-code, not approved-semantics-before-code): the doctrine documents executable mechanisms tickets 004/005/006 introduce — a fail-closed acceptance manifest, a closed wording grammar, a computed-pass rule — so this ticket `Deps` those tickets and lands with/after them; a doctrine ticket documenting a mechanism that does not yet exist would be dishonest (spec §6.1 "lands with the F6-04/F6-05/F6-06 code").
3. Shared boundary under audit: the below-foundation doctrine tier governing acceptance evidence. Mints no invariant/gate/risk-ID/glossary term; `01` updates **existing** R-27/R-28/R-29 status/evidence rows only. Doc-doctrine batch: all-`(modify)` markdown; the code-centric Step 5/6 machinery (Rust spot-grep, item-6 schema heuristic, `cargo` tests) is N/A.
4. INV-098 (harsh acceptance) and the architecture/execution evidence-honesty contract motivate this; per spec §6/§11 and driver §9, **no Tier-0 amendment is warranted** — this is an ordinary-owner-altitude below-foundation strengthening, not a constitutional edit. Restated before trusting the narrative.
5. Governed enforcement surface (substrate basis): this doctrine governs the fail-closed acceptance / governance-independence / PR-blocking-mutation enforcement surfaces implemented in tickets 004/005/006. Confirm the doctrine introduces no leakage/nondeterminism path and asserts only what those mechanisms enforce — acceptance artifacts are read models over current evidence, not evidence themselves; a `pass` requires current exact-commit evidence ingestion, independent acceptance where required, no live survivor/pending rows, and no prose stronger than the computed state.

## Architecture Check

1. Landing the doctrine substance *with* the mechanism keeps the rules executable rather than aspirational, and binding the verdict grammar to computed status (template `0003`) prevents free-form certification from reintroducing the laundered-pass pattern. Editing only the existing R-27/R-28/R-29 rows (not minting a risk ID) keeps the risk register stable.
2. No backwards-compatibility aliasing/shims (doc batch): open prose verdict shapes are replaced by a closed grammar keyed to computed status, not layered beside it. Each edit carries an ordinary-owner approval precondition and must not be applied by convention.

## Verification Layers

1. Doctrine landed (INV-098) → codebase grep-proof: each doc carries the new substance at the stated home (exact-string matches for the read-model / fail-closed / closed-grammar statements).
2. No invariant minted → grep-proof that no new `INV-NNN`, gate code, risk ID, or glossary term is introduced and R-27/R-28/R-29 are status/evidence updates only.
3. Boundary (authority order) → manual review: the strengthening sits below foundation (architecture 13 / execution 10 / reference / template), contradicts no higher tier, and is recorded at ordinary-owner altitude.

## What to Change

### 1. `docs/1-architecture/13` — acceptance artifacts are read models

State that acceptance artifacts are read models over current evidence, not evidence themselves: a `pass` claim requires current exact-commit evidence ingestion, independent acceptance where required, no live survivor/pending rows, and no prose stronger than the computed state.

### 2. `docs/2-execution/10` — the fail-closed taxonomy

State that the fail-closed taxonomy computes non-pass for survivors, timeouts, pending governance, self-authored-only evidence, missing actual-artifact ingestion, and zero-approval independence gaps; source-text guards are topology alarms only; PR-blocking mutation proof is required for guarded changes.

### 3. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — closed grammar

Replace open prose verdict shapes with a closed grammar keyed to computed status; the status block must be generated/verified from evidence artifacts and the expected-finding manifest, not authored as free-form certification.

### 4. `docs/3-reference/00` — reviewer prompts (navigation only)

Add reviewer prompts for stale negative fixtures, public-constructor composition, debug-token induction, and survivor-pass taxonomy holes.

### 5. `docs/3-reference/01` — R-27/R-28/R-29 status/evidence (no new risk ID)

Update the existing R-27/R-28/R-29 status/evidence rows only; mint no new risk ID.

## Files to Touch

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- The executable F6-04/F6-05/F6-06 machinery itself (tickets 004/005/006) — this ticket documents it.
- Post-closure live-conformance doc-truthing (arch 04/10, exec 05/07) — ticket 010.
- Any constitutional (Tier-0) amendment; any new invariant, gate code, risk ID, or glossary term.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE "read model|read-model" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — the read-model-not-evidence doctrine is present.
2. `grep -nE "fail-closed|topology alarm|PR-blocking" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — the fail-closed taxonomy + PR-blocking-mutation doctrine is present.
3. `grep -nE "R-27|R-28|R-29" docs/3-reference/01_DESIGN_RISK_REGISTER.md && ! grep -nE "R-3[0-9]" docs/3-reference/01_DESIGN_RISK_REGISTER.md` — existing risk rows updated; no new risk ID minted.

### Invariants

1. No new `INV-NNN`, gate code, risk ID, or glossary term is introduced; the change is below-foundation substance + home only.
2. The closed verdict grammar in template `0003` is keyed to computed status, not free-form prose.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + boundary checks) and the executable enforcement coverage is owned by tickets 004/005/006 named in Assumption Reassessment.`

### Commands

1. `grep -nE "read model|fail-closed|closed grammar|topology alarm|R-27|R-28|R-29" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md docs/3-reference/01_DESIGN_RISK_REGISTER.md`
2. `! grep -rnE "INV-11[3-9]|R-3[0-9]" docs/` — confirm no new invariant or risk ID was minted by this edit.
3. Narrower than a `cargo` run because this is a doc-only doctrine ticket: verification is grep-based landing + an invariants-alignment manual review, not a code test.
