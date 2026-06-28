# 0056FOUCONSEV-007: §6.1 below-foundation doctrine synchronization

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0056FOUCONSEV-003, 0056FOUCONSEV-004

## Problem

Spec §6.1. The acceptance taxonomy code (F7-03 governance posture, F7-04 closed verdict grammar) must land alongside the below-foundation doctrine it operationalizes, so the rules are executable and not merely written. No new doctrine substance is warranted — substance and home only, at ordinary-owner approval altitude. No Tier-0 constitutional amendment (the foundation already forbids forgeable validated bootstraps, ordinary-input debug authority, self-scored/laundered acceptance, and paraphrase-bypassable verdicts); the `0055` no-weakening check confirms the architecture `13` / execution `10` solo-maintainer edits are sound at their tier.

## Assumption Reassessment (2026-06-28)

1. Doc targets verified at `2720167`: `docs/2-execution/10:358-392` already ratifies `solo-maintainer-compensating-control` but does **not** yet carry the closed-verdict-grammar language; `docs/1-architecture/13:136/155` carries solo-maintainer mode but not the computed-verdict requirement; `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md:45-50` already carries the `Computed result: pass | non-pass` line (retain) but **not** the `solo-maintainer-compensating-control` field set (add); `docs/3-reference/00` (reviewer checklist) and `docs/3-reference/01` R-27/R-28/R-29 exist.
2. Spec §6.1 + §6 (no Tier-0 amendment). This is the operationalize-doctrine direction: the doctrine ticket `Deps` the mechanism tickets (003, 004) and lands with/after them, because documenting a mechanism that does not yet exist is dishonest.
3. **Shared boundary under audit**: the below-foundation acceptance doctrine (execution 10, architecture 13) and its template/reference homes; this ticket synchronizes them to the executable taxonomy that 003/004 land. No archived spec/ticket/report/acceptance/certification is edited.
4. INV-093 / INV-098 acceptance-evidence discipline — the doctrine must state that a `pass` requires the doctrine-complete computed verdict, the proven governance posture, no live survivor/pending rows, and no prose stronger than the computed state; this is the doctrine face of what 003/004 enforce in code.
5. **Governed enforcement surface (substrate basis)**: this doc-only ticket governs the fail-closed acceptance taxonomy without touching code. Item-5 on the substrate basis: name the governed surface (the manifest parser + wording guard from 003/004) and confirm the doctrine change introduces no leakage/nondeterminism path and reaffirms that source-text guards are topology alarms only and that the forced CI parser must be doctrine-complete. No schema-shape change fires the item-6 heuristic (markdown doc edits only) — the template field-set addition is documentation of 003's additive schema, recorded here, not a code schema change.

## Architecture Check

1. Synchronizing the doctrine alongside the code (rather than ahead of it) keeps the doc tier honest: the rule is executable the moment it is written. Centralizing all five doc surfaces in one ticket avoids 003/004 each half-editing the shared template `0003` (a merge hub) and keeps the doctrine edit atomic and reviewable as a doc-only diff.
2. No backwards-compatibility shim — doctrine text is amended in place; the existing computed-verdict line is retained, not duplicated (per the in-session `/reassess-spec` M1 correction).

## Verification Layers

1. INV-098 (harsh acceptance) -> grep-proof landing: execution `10` carries the closed-verdict-grammar language; architecture `13` carries the computed-verdict `pass` requirement; template `0003` carries the `solo-maintainer-compensating-control` field set (and retains the `Computed result:` line); reference `00` carries the new reviewer prompts; reference `01` R-27/R-28/R-29 status/evidence updated with no new risk ID.
2. INV-093 evidence-honesty -> invariants-alignment review that no doctrine edit weakens, mints, or redefines an invariant/gate/risk ID/glossary term (the `0055` no-weakening conditions stand).
3. Doc-only ticket — verification is grep-based landing + an invariants-alignment review, not `cargo` tests.

## What to Change

### 1. `docs/2-execution/10`

Align the fail-closed taxonomy implementation details with the settled `solo-maintainer-compensating-control` value (and its proven compensating-control field set) and the closed verdict grammar; reaffirm source-text guards are topology alarms only and the forced CI parser must be doctrine-complete.

### 2. `docs/1-architecture/13`

Keep acceptance artifacts as read models over current evidence; a `pass` requires the doctrine-complete computed verdict, the proven governance posture (independent-review, last-push-required-reviewer, or fully-proven solo-maintainer-compensating-control), no live survivor/pending rows, and no prose stronger than the computed state.

### 3. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

Add the `solo-maintainer-compensating-control` field set (the genuine gap); **retain/verify** the existing `Computed result: pass | non-pass` line (do not re-add). The status block is generated/verified from evidence, not authored as free-form certification.

### 4. `docs/3-reference/00`

Add reviewer prompts for stale negative fixtures, validated-content aliases beside sealed constructors, ordinary-input debug induction, and paraphrase-bypassable wording (navigation only).

### 5. `docs/3-reference/01`

Update existing R-27/R-28/R-29 status/evidence rows only; mint no new risk ID.

## Files to Touch

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify)
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)

## Out of Scope

- The taxonomy code itself (0056FOUCONSEV-003/004) and its mutation coverage (005).
- Post-closure conformance-row truthing at architecture `04`/`10` and execution `05`/`07` (0056FOUCONSEV-008) — that lands only after executable closure.
- Any constitutional-invariant amendment, new gate rung, new risk ID, or glossary term (spec §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proofs against the post-edit docs: execution `10` and architecture `13` carry the computed-verdict/closed-grammar requirement; template `0003` carries the `solo-maintainer-compensating-control` field set AND retains the `Computed result:` line; reference `00` carries the four reviewer prompts; reference `01` shows updated R-27/R-28/R-29 with no new R-id.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings` — repo-wide cleanliness unaffected by doc edits.

### Invariants

1. No invariant, gate code, risk ID, or glossary term is minted, weakened, or redefined; the `0055` no-weakening conditions are reaffirmed.
2. The doctrine matches the code 003/004 lands (executable, not merely written).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based grep landing checks and the invariants-alignment review named in Assumption Reassessment.`

### Commands

1. `grep -nE "solo-maintainer-compensating-control" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — field-set + value landed.
2. `grep -nE "Computed result" docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — existing verdict line retained (must still match).
3. `grep -nE "R-27|R-28|R-29" docs/3-reference/01_DESIGN_RISK_REGISTER.md` — status rows updated, no new risk ID.
