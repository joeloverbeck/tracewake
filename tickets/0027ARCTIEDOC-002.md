# 0027ARCTIEDOC-002: Scope architecture 11 story-sifting "may not create evidence" to diegetic-only

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` (scope one bullet + the acceptance-implications wording). No crate/code, no fixtures.
**Deps**: 0027ARCTIEDOC-001 (D2's allowed-evidence carve-out cross-references D1's one-way observer-only contract authored in A13). **Execution-blocking precondition**: owner approval per spec 0027 §R-A (architecture tier-1 doctrine; not by convention).

## Problem

D2 (report A11-E2). A11's story-sifting section lists "create evidence" as a flat "may not" bullet (verified: `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md:95` — `- create evidence.`). Before `INV-111` this was a sound anti-quest guard; after `INV-111` the unqualified wording can be misread as forbidding the very observer-only acceptance evidence the constitution now requires (the A13 record from 0027ARCTIEDOC-001). The fix scopes the rule so it forbids diegetic / in-world evidence creation while explicitly allowing the observer-only retrospective record — preserving the questless / no-director doctrine, not weakening it.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`fdfd0b9`): `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` line 95 carries the bare bullet `- create evidence.` inside the story-sifting "may not" list; A11's core rule (incidents/leads/reports/notices/rumors/story summaries are world artifacts or observer projections; they do not spawn truth, repair pacing, select culprits, guarantee rewards, or wait for the player) is intact and is preserved. "create evidence" appears in no other architecture doc (`grep -rniE "create evidence" docs/1-architecture` → only A11:95), so this is a single-site scoping.
2. Verified against spec 0027 §3 D2 + §7 R-B and source report §4.2 (A11-E2). The allowed-evidence carve-out references the observer-only one-way contract authored in A13 by sibling ticket 0027ARCTIEDOC-001 (D1) → declared in Deps. The questless-incident doctrine (INV-058/059/060) is not relaxed.
3. Shared boundary under audit: the A11 story-sifting authority boundary (what a sifter may vs. may not produce) against the A13 observer-only acceptance-artifact contract (0027ARCTIEDOC-001). The correction must split "diegetic evidence creation" (forbidden) from "observer-only review evidence" (allowed under A13's one-way contract) without reopening the no-director hole.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-111` (acceptance requires observer-only emergence evidence) is why the unqualified wording is now too broad; `INV-060` (no authored outcome chains) and `INV-059` (leads/tasks are source-bound projections, not ground-truth objectives) are the no-director guarantees the scoping must preserve. The edit is a clarifying scope correction; no invariant is weakened (spec §4, R-B).
5. No-leak / no-director surface (governed here, enforced by the A13 contract and deferred execution surfaces): the carve-out must allow ONLY observer-only retrospective evidence that is reproducible from event/projection input, carries event-log ancestry, and is structurally quarantined from cognition/schedulers/validators (INV-024 / INV-006 / foundation 09); it must keep forbidding any sifter-produced diegetic / holder-known / institution-known / in-world evidence, clue, proof, record, sanction, reward, or action reason. R-B is the relapse risk (over-narrowing reopens the no-director hole; under-narrowing leaves the INV-111 contradiction). This ticket adds doctrine only; no code path, leakage, or nondeterminism.

## Architecture Check

1. Scoping the existing bullet (rather than deleting it or adding a separate exception elsewhere) keeps the forbidden-evidence list intact and adds the narrow observer-only carve-out at the exact site the over-broad wording lives — the minimal, locally-reviewable correction. Deleting "create evidence" entirely would lose the diegetic-evidence prohibition; adding the carve-out in A13 only would leave A11's flat wording still misreadable.
2. No backwards-compatibility aliasing/shims: a prose scope correction; no mechanism, no compatibility layer.

## Verification Layers

1. `INV-111` observer-only evidence allowed (D2) → invariants alignment check: A11 now permits the observer-only retrospective record (under A13's one-way contract) while forbidding diegetic evidence.
2. `INV-060`/`INV-059` no-director preserved → manual review (no-director audit): the forbidden-evidence list (diegetic/holder/institution/in-world evidence, clues, proof, records, sanctions, rewards, action reasons by a sifter) remains intact; only observer-only review evidence is carved out.
3. Single-doc prose correction: no replay/fixture/skill-dry-run layer applies; the two layers above each map an engaged invariant to a distinct proof surface.

## What to Change

### 1. D2 — scope the A11 story-sifting "create evidence" rule

Amend the story-sifting "may not" list (and the acceptance-implications wording) in `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` to distinguish:

- **forbidden** (unchanged force): diegetic / holder-known / institution-known / in-world evidence, clues, proof, records, sanctions, rewards, or action reasons produced by a sifter;
- **allowed**: observer-only retrospective acceptance/review evidence under A13's one-way contract (0027ARCTIEDOC-001) — reproducible from event/projection input, carrying event-log ancestry, structurally quarantined from cognition/schedulers/validators.

Author the final wording at enactment; preserve the questless / no-director doctrine. The practical change scopes "create evidence", it does not weaken the incident doctrine.

## Files to Touch

- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` (modify)

## Out of Scope

- **The A13 observer-only emergence-evidence record itself** — sibling ticket 0027ARCTIEDOC-001 (D1).
- **`EMERGE-OBS` mechanism / thresholds / ratchets** — execution `10`.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits, any other A11 rule.

## Acceptance Criteria

### Tests That Must Pass

1. **Landing grep** — A11's story-sifting wording now distinguishes forbidden diegetic evidence from allowed observer-only review evidence: `grep -niE "observer-only|diegetic|create evidence" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` shows the scoped wording.
2. **No-director preservation review** — the forbidden-evidence list still bars sifter-produced diegetic/holder/institution/in-world evidence, clues, proof, records, sanctions, rewards, action reasons.
3. **Invariants alignment review** — the carve-out admits only observer-only ancestry-bound review evidence; INV-058/059/060 unweakened.

### Invariants

1. A11 forbids diegetic/in-world evidence creation by a sifter and allows only observer-only retrospective review evidence under A13's one-way contract — the no-director doctrine holds (INV-060/059).
2. The allowed observer-only evidence remains structurally quarantined from cognition/schedulers/validators (INV-024/INV-111).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus a no-director / invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "observer-only|diegetic|create evidence" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — confirms the scoped wording landed.
2. `git diff -- docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — review that the scope correction preserves the forbidden list.
3. `Documentation-only: the Rust pipeline is unaffected and is not the verification boundary for an architecture-doc edit; the boundary is the landing grep plus the no-director review.`
