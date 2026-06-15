# 0033EXETIETEM-009: exec 02 first-proof acceptance temporal cross-reference

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (a narrow first-proof acceptance cross-reference; no broad rewrite). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-001 (exec `04`), 0033EXETIETEM-003 (exec `06`), 0033EXETIETEM-004 (exec `07`), 0033EXETIETEM-006 (exec `09`), 0033EXETIETEM-010 (exec `10`) — the cross-reference points to the temporal evidence those tickets land, so it must resolve to landed content. **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T8 (report §4 coverage register for exec `02`; added to the spec by the in-session `/reassess-spec` pass as finding A1). Exec `02` defines first-proof scope and the acceptance stance, but says nothing about the temporal-firewall evidence now folded into first-playable acceptance. Verified `02` is not referenced by the spec's other deliverables (`grep -n '02_FIRST_PROOF' ...`); without a narrow cross-reference, a reviewer reading the first-proof acceptance contract would not know acceptance now includes temporal evidence. This ticket adds only that narrow cross-reference — no broad rewrite — keeping `02`'s first-proof scope and acceptance stance intact.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `02` (`02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`) defines first-proof scope/acceptance and carries no temporal cross-reference. The report's §4 coverage register (line 165) recommends "no broad rewrite. Add only narrow cross-reference that first-playable acceptance now includes temporal-firewall evidence through `04`, routine temporal proof through `06`, embodied temporal rendering through `07`, fixtures through `09`, and diagnostics through `10`." This ticket implements exactly that.
2. Verified against spec 0033 §3.1 D-T8 (the deliverable added by the in-session reassess A1 fix) and §6 (no broad rewrite). The cross-reference is the entire scope; concrete temporal values route to reference/spec per §6.
3. Shared boundary under audit: exec `02` (first-proof acceptance contract) ↔ exec `04`/`06`/`07`/`09`/`10` (the temporal evidence homes, tickets 001/003/004/006/010). This ticket's cross-reference points into those homes; it owns no temporal proof content of its own.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — the first-proof acceptance contract should make first-playable temporal-firewall evidence discoverable from the acceptance stance, so acceptance is not read as temporally silent. The pointer records that acceptance now includes temporal evidence; it does not restate the evidence.
5. Acceptance-surface (first-proof) cross-reference: the obligation is a pointer, not new enforcement — it states that first-playable acceptance now includes temporal-firewall evidence through `04`, routine temporal proof through `06`, embodied temporal rendering through `07`, fixture families through `09`, and diagnostics through `10`. It adds no validation, weakens no acceptance gate, and introduces no determinism change; the enforcement lives in the pointed-to homes (this ticket `Deps` on them so the pointer resolves to landed content).

## Architecture Check

1. Exec `02` is the correct home for a first-proof acceptance cross-reference, and a narrow *pointer* (not a rewrite) is the correct shape: the report explicitly says "no broad rewrite." The pointer routes reviewers from the acceptance stance to the temporal evidence homes the sibling tickets author.
2. No backwards-compatibility aliasing/shims: additive cross-reference; no behavior change, no duplication of the homes' obligations, no rewrite of the first-proof scope.

## Verification Layers

1. `INV-112` first-proof acceptance discoverability (D-T8) → invariants alignment check: exec `02` gains a narrow cross-reference stating first-playable acceptance includes temporal-firewall evidence through `04`/`06`/`07`/`09`/`10`.
2. Pointer-resolves check → codebase grep-proof: each home the cross-reference names carries the temporal obligation its sibling ticket authored (the sibling landing greps), so the pointer is not dangling.
3. Documentation-only narrow cross-reference: no replay/golden-fixture or skill-dry-run layer applies; the layers above map the engaged invariant and the pointer-integrity check to distinct surfaces.

## What to Change

### 1. D-T8 — narrow first-proof acceptance cross-reference

Add to exec `02` a narrow cross-reference (no broad rewrite) recording that first-playable acceptance now includes temporal-firewall evidence through `04`, routine temporal proof through `06`, embodied temporal rendering through `07`, fixture families through `09`, and diagnostics through `10`. Keep `02`'s first-proof scope and acceptance stance otherwise intact.

## Files to Touch

- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` (modify)

## Out of Scope

- **Any broad rewrite of `02`'s first-proof scope or acceptance stance** — explicitly forbidden (report §4; spec D-T8); the change is a narrow cross-reference only.
- **The temporal evidence content itself** — the pointed-to homes exec `04`/`06`/`07`/`09`/`10` (tickets 001/003/004/006/010).
- **Concrete temporal values** — reference/future scoped specs (§6).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T8 landing grep** — exec `02` carries the temporal acceptance cross-reference: `grep -niE 'temporal.*(evidence|firewall)|temporal' docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` resolves the cross-reference.
2. **Pointer-integrity review** — each home named (`04`/`06`/`07`/`09`/`10`) carries its sibling ticket's landed temporal obligation; no pointer dangles.
3. **No-rewrite review** — manual review confirms only a narrow cross-reference was added and `02`'s first-proof scope/acceptance stance is otherwise intact; no new gate code.

### Invariants

1. First-proof acceptance is discoverable as including temporal-firewall evidence via a pointer to the real homes, not a restatement (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only narrow cross-reference ticket; verification is command-based (landing grep) plus a pointer-integrity and no-rewrite manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal' docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — confirms D-T8 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the pointer-integrity and no-rewrite review.`

## Outcome

Completed: 2026-06-15

Implemented the narrow first-proof temporal acceptance cross-reference in
`docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`.
The edit points first-playable acceptance to the landed temporal evidence homes
in exec `04`, `06`, `07`, `09`, and `10` without rewriting the first-proof
scope or duplicating those homes' obligations.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal.*(evidence|firewall)|temporal' docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `grep -niE 'temporal[- ]firewall|temporal premise|temporal claim' docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `grep -niE 'temporal premise|adaptation|expectation' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `grep -niE 'temporal (label|render)|time control' docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `grep -niE 'temporal.*fixture|fixture famil' docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `grep -niE 'temporal[- ]divergence|validator time|temporal.*render|LOD.*ancestry' docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `git diff --check`

Manual review confirmed every pointed-to home carries its sibling ticket's
landed temporal obligation, the `02` edit is only a narrow cross-reference, and
no new gate code was minted.
