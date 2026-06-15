# 0034REFTIETEM-003: reference 00 checklist — temporal-authority review block

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (additive temporal-authority review-question block; pointers, not new doctrine). No crate/code, no fixtures.
**Deps**: 0034REFTIETEM-001 (producer/consumer — the review questions reference the canonical terms the glossary ticket defines). **Execution-blocking precondition**: owner approval per spec 0034 §R-A (reference tier-3 doctrine; ordinary owner approval, not by convention).

## Problem

Spec 0034 D-T3 (report F-03), the checklist slice. The reference index/checklist enforces session-start source discipline, tier boundaries, and deferred-term handling, but asks **no** temporal-firewall review questions: it does not prompt a reviewer to confirm a temporal premise came through a modeled holder/institution-known channel, that validator/replay/scheduler time is quarantined from diegetic cognition, or that time-acceleration/debug surfaces and LOD summaries preserve the temporal firewall. Without these pointers, epoch-2 temporal drift is invisible at review time even after the glossary terms (ticket 001) and relapse memory (ticket 002) exist. This ticket adds a compact temporal-authority review block — pointers to the governing docs, not new doctrine — additively over the existing checklist.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`cda3325`): `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` carries the source-discipline gate, tier-boundary statement, compact review checklist, and deferred-term handling, and asks no temporal-firewall review questions (no `temporal`/`holder-known`/`time acceleration` review prompts). The gap is real; this ticket adds review pointers, it does not restate foundation/architecture/execution doctrine.
2. Verified against spec 0034 §3.1 D-T3 and the ratified upstream (`INV-112`/foundation `03` via `0031`; architecture `0032`; execution `0033`). The checklist is the home for the *review questions*; concrete temporal vocabulary and thresholds route to future scoped specs per spec §6.
3. Cross-artifact shared boundary under audit: this ticket (`00`) **consumes** the canonical terms produced by ticket 001 (`02` glossary) — "holder-known/institution-known source", "temporal firewall", "temporal ancestry", "time acceleration" — hence `Deps: 0034REFTIETEM-001`. A review question must cite a term the glossary defines; that coherence is the boundary verified here. The matching relapse memory is ticket 002.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-112` (time may validate, but holder-known time must plan); `INV-099` (truth may validate actions, but truth may not plan them); `INV-110` (LOD/summary processes must preserve the firewall). The review questions prompt reviewers to verify these; they create no doctrine and weaken none.
5. Governed enforcement surface (doctrine ticket, no code): the review block is review-time verification support for the truth/temporal firewall and LOD-ancestry surface (`INV-112`/`INV-099`/`INV-110`). Per the Substrate-only / doctrine-batch rule, item 5 applies: the questions ask reviewers to confirm modeled-channel provenance, validator/replay/scheduler-time quarantine, time-acceleration/debug quarantine, and temporal-ancestry preservation — so the block strengthens epistemic-leakage and nondeterminism prevention and introduces no new path. It remains pointers to the governing docs, not enforcement logic.

## Architecture Check

1. `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` is the correct single home: it already owns the compact review checklist and source-discipline gate, so a temporal-authority review block is an addition to a checklist it already maintains — not a glossary or risk-register concern. Keeping the questions here (and only referencing glossary terms from `02`) preserves per-doc 1:1 ownership and the merge-hub-free batch shape.
2. No backwards-compatibility aliasing/shims: an additive review block over the existing checklist; the source-discipline gate, tier boundary, and deferred-term handling are unchanged.

## Verification Layers

1. `INV-112`/`INV-099` temporal-firewall review → invariants alignment check + codebase grep-proof: the block asks whether every planning/procedure/view/speech/lead/LOD temporal premise has a holder-known or institution-known source and whether validator/replay/scheduler time is separated from diegetic cognition.
2. `INV-110` LOD-ancestry review → invariants alignment check: the block asks whether time acceleration and debug panels are quarantined from embodied surfaces and whether temporal ancestry is preserved across summaries, snapshots, projections, and compacted history.
3. Pointers-not-doctrine review → manual review: the block points to the governing docs and uses glossary terms; it states no concrete temporal value and does not redefine upstream doctrine.
4. Documentation-only doctrine ticket: no replay/golden-fixture or skill-dry-run layer applies; the layers above map each engaged invariant to a distinct review surface.

## What to Change

### 1. D-T3 — temporal-authority review block

Add a compact temporal-authority review block to the checklist asking: whether every planning/procedure/view/speech/lead/LOD temporal premise has a holder-known or institution-known source; whether validator/replay/scheduler time is separated from diegetic cognition; whether time acceleration and debug panels are quarantined from embodied surfaces; and whether temporal ancestry is preserved when summaries, snapshots, projections, or compacted history are used. Phrase as review pointers to the governing foundation `03`/`INV-112`, architecture, and execution homes — not new doctrine, and no concrete temporal value.

## Files to Touch

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (modify)

## Out of Scope

- **Concrete temporal values** — day-part/lateness vocabulary, calendar syntax, stale-after thresholds, time-acceleration speed (spec §6).
- **Glossary terms** — `02_GLOSSARY.md` (ticket 001); this ticket cites them.
- **Relapse-risk memory** — risk register `01` (ticket 002).
- **The source-discipline gate, tier boundary, and deferred-term handling** — unchanged; this ticket only adds the temporal review block.
- **Owner approval itself (spec §R-A)** — execution precondition, not a deliverable.
- Crate/code, fixtures; foundation/architecture/execution edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T3 landing grep** — temporal review block present: `grep -niE 'holder-known|institution-known|temporal (premise|ancestry|firewall)|time acceleration' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` resolves the review questions.
2. **Pointers-not-doctrine review** — manual review confirms the block points to the governing docs (foundation `03`/`INV-112`, architecture, execution) and states no concrete temporal value (no stale-after threshold, day-part vocabulary, or calendar syntax).
3. **Existing-checklist preservation review** — the source-discipline gate and tier-boundary statement remain present and unchanged (`grep -niE 'source discipline|subordinate|tier' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` still resolves them).

### Invariants

1. The review questions verify modeled-channel provenance and validator/replay/scheduler-time quarantine — they prompt firewall confirmation, never license a temporal-truth shortcut (`INV-112`/`INV-099`).
2. The block preserves the existing checklist's source-discipline and tier-boundary discipline and adds pointers only, not new doctrine (`INV-110` LOD-ancestry review included).

## Test Plan

### New/Modified Tests

1. `None — documentation-only reference-doctrine ticket; verification is command-based (landing grep + checklist-preservation grep) plus a pointers-not-doctrine and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'holder-known|institution-known|temporal (premise|ancestry|firewall)|time acceleration' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — confirms D-T3 landed.
2. `grep -niE 'source discipline|subordinate|tier' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — confirms the existing source-discipline gate and tier boundary are preserved.
3. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the checklist-preservation grep and the pointers-not-doctrine and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the checklist portion of spec 0034 by adding a compact `Temporal authority` review block to `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`. The block asks reviewers to confirm holder-known or institution-known sources for temporal premises, quarantine validator/replay/scheduler time from diegetic cognition and procedure, quarantine time acceleration and debug temporal panels from embodied surfaces, and preserve temporal ancestry plus information ancestry across summaries and compaction.

The execution-blocking owner-approval precondition in spec 0034 §R-A was satisfied by the user's explicit `$ticket-series implement the series tickets/0034REFTIETEM*` instruction. The new block is worded as review pointers to foundation `03` / `INV-112`, architecture, and execution proof homes; it defines no gate semantics and introduces no concrete temporal values.

Verification run:

- `grep -niE 'holder-known|institution-known|temporal (premise|ancestry|firewall)|time acceleration' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `grep -niE 'source discipline|subordinate|tier' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `rg -n 'calendar|day-part|stale-after|threshold|duration unit|tick size' docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `git diff --check`

Manual review confirmed the source-discipline gate and authority/tier boundary remain intact. The boundary grep's only match was pre-existing institutional `evidence thresholds` wording, not a concrete temporal value added by this ticket.
