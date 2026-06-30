# 0057EMBROUCON-006: Doctrine amendments — embodied continue_routine commits the follow-on

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None
**Deps**: 0057EMBROUCON-002

## Problem

Spec 0057 §5 routes two doctrine amendments as *substance + home* (not ratified in the spec, no identifiers minted). With the embodied follow-on mechanism landed (0057EMBROUCON-002), the owning doctrine docs must state explicitly that an embodied `continue_routine` selection commits the routine step's follow-on ordinary action (or a typed block/wait/stuck), so possessed routine-following is behavioral progress while the bare marker remains non-progress. This sharpens — does not weaken — the existing "marker is not progress" rule. Because the doctrine documents a mechanism the code introduces (operationalizing, not establishing pre-code semantics), it lands with/after 0057EMBROUCON-002.

## Assumption Reassessment (2026-06-30)

1. Home docs verified: `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md:181` ("A pure `continue_routine` marker is not behavioral progress") and `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md:91` (do not count "continue routine" markers as behavioral progress unless an ordinary follow-on action or explicit modeled wait/failure is committed). Both are the exact sharpening sites named in spec §5.
2. Spec assumption: `specs/0057_…_SPEC.md` §5 governs; it routes substance + home and explicitly does not pre-author wording or mint identifiers. Reassessment finding M5 confirmed these are the correct anchors (and that `docs/0-foundation/12:557` is the truth-firewall gate, not the behavioral-progress anchor — so this ticket touches the execution/architecture tiers only, not foundation). §5: "No constitutional-invariant amendment is required."
3. **Cross-artifact boundary under audit**: the doctrine standard "an embodied `continue_routine` selection commits the follow-on ordinary action = behavioral progress, the human-input counterpart to the autonomous planner's step commit (same shared pipeline, same actor-known resolution)" — it governs the 0057EMBROUCON-002 mechanism and the 0057EMBROUCON-001 shared resolver. This is a doc-amendment ticket: all targets are markdown; no Rust symbol, schema, or test surface is touched.
4. INV-035 (routines are defeasible intentions producing real reach-and-act), INV-098 (feature acceptance is harsh — done only when TUI-playable *and* no-human runnable). The amendment moves the doctrine from describing only the no-human path to admitting the possessed path, while preserving the marker-is-not-progress rule. Restated so the edit pins the doctrine, not the ticket narrative.
5. **Governed enforcement surface (substrate basis — doc-only)**: the doctrine governs the behavioral-progress / actor-known follow-on commit surface (the shared pipeline + 0047 single-charge accounting) without touching code. Confirm the amendment introduces no leakage or nondeterminism path — it documents an actor-known follow-on commit through the shared pipeline (preserving INV-099/018) and does not loosen the marker-is-not-progress boundary (`docs/2-execution/06` / `docs/1-architecture/04`).

## Architecture Check

1. Documenting the embodied follow-on commit as the human-input counterpart to the autonomous step commit — one shared resolution (0057EMBROUCON-001), one pipeline — keeps doctrine and code aligned and confines the change to sharpening the existing rule rather than minting a new one, avoiding doctrine sprawl. Landing it with/after the mechanism (operationalizing-doctrine direction) keeps the docs honest: a doc describing a mechanism that did not yet exist would be dishonest.
2. No backwards-compatibility aliasing or shims: doc edits only; nothing is wrapped or aliased.

## Verification Layers

1. `docs/2-execution/06` + `docs/1-architecture/04` (marker-is-not-progress, sharpened) -> codebase grep-proof: both docs state explicitly that an embodied `continue_routine` selection commits the follow-on ordinary action (or a typed block/wait/stuck), while the bare marker stays non-progress.
2. INV-098 (harsh acceptance) -> invariants-alignment review: the amendment makes possessed routine-following TUI-playable-and-no-human, not just no-human, without weakening any existing rule.
3. Doc-only ticket -> manual review: each tier's amendment authority (ordinary owner approval for execution and architecture) signs off before the edit is applied; not applied by convention.

## What to Change

### 1. Execution-tier amendment (`docs/2-execution/06`)

State explicitly that an embodied `continue_routine` selection commits the routine step's follow-on ordinary action (or a typed block / modeled wait / typed stuck), so possessed routine-following is behavioral progress, while the bare marker remains non-progress — sharpening the existing rule at/around the §181 "marker is not behavioral progress" statement.

### 2. Architecture-tier amendment (`docs/1-architecture/04`)

Name the embodied follow-on commit as the human-input counterpart to the autonomous planner's step commit, routed through the same shared pipeline and the same actor-known resolution — sharpening the existing rule at/around the §91 "do not count markers as progress unless a follow-on is committed" statement.

## Files to Touch

- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)

## Out of Scope

- Any constitutional-invariant amendment (`docs/0-foundation/02`) — spec §5: not required; route separately only if acceptance review finds an invariant-level sentence is needed.
- The code mechanism (0057EMBROUCON-001/002) the doctrine describes.
- Minting any new gate code, risk id, or glossary term (spec §0 / §5 forbid it).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nF "follow-on" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` and `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — both docs state the embodied `continue_routine` follow-on commit explicitly.
2. The bare-marker-is-not-progress statements at `docs/2-execution/06` and `docs/1-architecture/04` remain present (sharpened, not removed).
3. Invariants-alignment review: the amendment weakens no existing rule (INV-098 / marker-is-not-progress preserved).

### Invariants

1. Doctrine states an embodied `continue_routine` selection commits the follow-on (or a typed block/wait/stuck) = behavioral progress; the bare marker remains non-progress.
2. No new identifier (gate code, risk id, glossary term) is minted; no constitutional invariant is amended.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + boundary checks) and an invariants-alignment review. Existing pipeline coverage of the mechanism lives in 0057EMBROUCON-002/005.`

### Commands

1. `grep -nE "embodied|follow-on" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — landing check for the execution-tier amendment.
2. `grep -nE "embodied|follow-on|counterpart" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — landing check for the architecture-tier amendment.
3. `grep -nF "marker is not" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — boundary check that the marker-is-not-progress rule is preserved.

## Outcome

Completed: 2026-06-30

Amended `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` to state that embodied `continue_routine` progress comes from the committed follow-on ordinary action or typed block/wait/stuck outcome, while the marker remains non-progress. Amended `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` to name embodied `continue_routine` as the human-input counterpart to the autonomous planner's step commit only when the same shared pipeline and actor-known resolution commit the follow-on or typed outcome.

Deviation from plan: none. No new gate code, risk id, glossary term, or constitutional invariant was minted.

Verification:

- `grep -nE "embodied|follow-on" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `grep -nE "embodied|follow-on|counterpart" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `grep -nF "marker is not" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `git diff --check`
