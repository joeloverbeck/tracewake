# 0032ARCTIETEM-006: A07/A11 temporal speech expressions + lead/staleness cross-references

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — cross-reference doctrine edits to `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` and `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`. No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-002 (temporal utterances and lead labels are interpreted through the A03/A06 holder-known temporal context this cross-references). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T8 (report T8). `INV-112` names speech interpretation and leads among the surfaces that may use temporal facts only through modeled channels. A07 already requires speech acts to carry speaker/listener context and forbids LLM/prose authority; A11 already makes leads source-bound, stale-risk-bearing, and story-sifting observer-only. This is **already-owned-close**: the gap is cross-document clarity, not new doctrine — temporal expressions and stale/late labels should explicitly point at the A03/A06 temporal-claim contract.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A07 carries speaker/listener context + structured speech-act boundaries; A11 carries source-bound leads, stale-risk, and observer-only story-sifting (the `0027`-ratified carve-out). Neither cross-references temporal-claim interpretation. This ticket adds short cross-references only; it introduces no new doctrine on either surface.
2. Verified against spec 0032 §3.1 D-T8 and source report §5 Finding T8 / `INV-112` / foundation `11`,`12`. Both homes are A07 + A11, as cross-references to A03/A06 (ticket 002). Additive; relaxes nothing.
3. Shared boundary under audit: A07 (speech) and A11 (leads) ↔ A03/A06 holder-known temporal context. **A11 is also touched by sibling ticket 008** (D-T7/D-R1 embodied + play legibility); this ticket lands first (008 `Deps` on it). Coordinate the A11 merge.
4. Constitutional invariants motivating this ticket, restated: `INV-112` — temporal expressions in speech and stale/late labels on leads may use temporal facts only through modeled channels; `INV-111` — story-sifting may compute observer-only temporal summaries for review but may not create actor-known urgency or quest priority. D-T8 preserves the `0027`-ratified A11 observer-only carve-out exactly.
5. No-leak / observer-only surface (governed here; enforcement deferred to execution `07`/`10`): temporal utterances ("yesterday/late/due/before the market closed/after the bell") are structured claims interpreted via speaker+listener holder-known temporal context, provenance, and ambiguity; lead/notebook labels (stale/urgent/overdue/recently seen/old report/no longer useful) are source-bound projections over holder-known temporal claims and records; story-sifting temporal summaries stay observer-only and create no actor-known urgency. Cross-references only; no leakage path introduced and the observer-only constraint is reinforced, not relaxed.

## Architecture Check

1. Cross-references (not new doctrine) are the correct shape: A07/A11 already own their contracts; D-T8's value is pointing temporal interpretation at the A03/A06 contract so a future author does not re-derive it. A new temporal subsection on each would duplicate ticket 002.
2. No backwards-compatibility aliasing/shims: additive cross-reference text; the A11 observer-only rule is preserved verbatim in intent.

## Verification Layers

1. `INV-112` temporal speech (A07) → invariants alignment check: A07 cross-references that temporal utterances are structured claims interpreted through speaker/listener holder-known temporal context (A03/A06), provenance, and ambiguity.
2. `INV-112`/`INV-111` lead/staleness labels (A11) → invariants alignment check: A11 cross-references that stale/urgent/overdue/recently-seen labels are source-bound projections over holder-known temporal claims/records, and story-sifting temporal summaries are observer-only (no actor-known urgency / quest priority).
3. Two docs, additive cross-references: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `07`/`10`); the layers above map each engaged surface to a distinct alignment proof.

## What to Change

### 1. D-T8 — temporal-utterance cross-reference in A07

Add a short cross-reference to `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`: temporal utterances such as "yesterday," "late," "due," "before the market closed," or "after the bell" are structured claims whose interpretation depends on speaker and listener holder-known temporal context, provenance, and ambiguity (per A03/A06).

### 2. D-T8 — lead/staleness-label cross-reference in A11

Add a short cross-reference to `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`: lead/notebook labels such as stale, urgent, overdue, recently seen, old report, or no longer useful are source-bound projections over holder-known temporal claims and records; story-sifting may compute observer-only temporal summaries for review but may not create actor-known urgency or quest priority (preserving the existing observer-only correction).

## Files to Touch

- `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` (modify)
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` (modify)

## Out of Scope

- **New temporal doctrine on A07/A11** — this is a cross-reference ticket; the temporal-claim contract is ticket 002 (A03/A06).
- **Lead-staleness label vocabulary / thresholds, speech temporal-parse mechanics** — execution/implementation (spec §6).
- **A11 play-legibility / lead-usefulness consolidation** — sibling ticket 008 (D-R1).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T8 A07 landing grep** — `grep -niE "temporal|yesterday|holder-known|A03|06_" docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` resolves the temporal-utterance cross-reference.
2. **D-T8 A11 landing grep** — `grep -niE "stale|overdue|observer-only|temporal claim" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` resolves the lead-label cross-reference.
3. **Invariants alignment review** — A07/A11 cross-references uphold `INV-112` and preserve the `INV-111` observer-only story-sifting carve-out; no new diegetic-urgency path introduced.

### Invariants

1. Temporal utterances are structured claims interpreted via speaker/listener holder-known temporal context (`INV-112`).
2. Lead/staleness labels are source-bound projections; story-sifting temporal summaries stay observer-only with no actor-known urgency or quest priority (`INV-112`/`INV-111`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine cross-reference ticket; verification is command-based (two landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal|yesterday|holder-known" docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — confirms the A07 cross-reference landed.
2. `grep -niE "stale|overdue|observer-only|temporal claim" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — confirms the A11 cross-reference landed.
