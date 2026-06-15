# 0032ARCTIETEM-008: A10/A11 embodied temporal rendering, time controls, and play legibility

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (temporal rendering / time-control contract + embodied play loop) and `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` (lead-usefulness projection). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-002 (embodied temporal displays derive from the A03/A06 holder-known temporal context); 0032ARCTIETEM-006 (A11 lead-label cross-reference lands first — shared A11 surface). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T7 (report T7) + D-R1 (report R1), both on the embodied TUI/leads surface (A10/A11). Foundation `08` says time controls may advance authoritative event/replay time, but embodied views render temporal facts only when the possessed actor knows/remembers/infers/reads/hears/perceives them; debug may show omniscient time separately. The completeness determination separately routes play experience / epistemic legibility to A10/A11. A10 already owns embodied-vs-debug separation, holder-known view-model generation, observation-time capture, semantic actions, why-not split, and possession parity; A11 already owns source-bound leads and story-sifting — but neither names a temporal rendering / time-control contract nor the embodied play loop / lead-usefulness projection.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A10 owns embodied/debug split, holder-known view models, observation-time capture, semantic actions, why-not split, possession parity; A11 owns source-bound leads, stale-risk, observer-only story-sifting. Neither names temporal rendering/time controls (A10) or the embodied play loop + lead usefulness (A10/A11). Execution `07` currently keeps player-facing time acceleration staged/debug-gated; this ticket adds the architecture-tier contract, not the execution staging.
2. Verified against spec 0032 §3.1 D-T7 + D-R1 and source report §5 Findings T7 / R1 / foundation `08`. Homes: A10 (D-T7 + D-R1 play loop), A11 (D-R1 lead usefulness). Additive; relaxes nothing. UI clock format, acceleration speed, wait-command vocabulary, and summary thresholds route out per spec §6.
3. Shared boundary under audit: the A10/A11 embodied surface ↔ A03/A06 holder-known context. **Merge rationale**: D-T7 (A10 temporal rendering) and D-R1 (A10 play loop + A11 lead usefulness) are the same embodied-surface theme; merging keeps A10 single-owned. **A11 is a merge hub** with sibling ticket 006 (D-T8 lead-label cross-reference); 006 lands first (this ticket `Deps` on it). Coordinate the A11 merge.
4. Constitutional invariants motivating this ticket, restated: `INV-112` — actor-facing time displays render only holder-known/observed temporal facts; `INV-006`/`INV-108` — possession transfers no world knowledge and is cognition-neutral (so embodied time rendering is not a temporal-knowledge upgrade); `INV-024` — no telepathy (debug omniscient time is structurally non-diegetic and cannot feed embodied affordances). D-R1 adds the no-quest/no-director constraint on lead usefulness.
5. No-leak / possession surface (governed here; enforcement deferred to execution `07`/`10`): world-advancing controls are commands advancing authoritative event/replay time through the ordinary pipeline (not actor cognition); actor-facing time displays / missed-event summaries / time-to-work cues / waiting-sleeping summaries / office-closed messages / lateness labels derive from the possessed actor's holder-known context or modeled observations/records/public cues; debug/operator panels may show exact event/replay time + due queues + hidden comparisons but are structurally non-diegetic and cannot feed embodied affordances or actor-visible reasons; rejection/why-not output preserves the actor-visible/debug split. D-R1: leads can be stale/ambiguous/partial/contradictory/actionable but never objective markers, quest stages, or hidden priority; transcript evidence demonstrates the loop without parsing display prose as authority. Adds doctrine only; no leakage path, and the debug/embodied firewall is reinforced.

## Architecture Check

1. D-T7 and D-R1 are merged because both govern the embodied A10/A11 surface and share the actor-known-rendering-vs-debug firewall; merging keeps A10 single-owned and the temporal-rendering and play-loop contracts mutually consistent. D-T7's A10 contract and D-R1's A10 play loop co-locate; D-R1's A11 lead-usefulness rule is the read surface for the same loop.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine extending the existing embodied/debug split with a temporal axis and a named play loop.

## Verification Layers

1. `INV-112`/`INV-024` temporal rendering & time controls (D-T7) → invariants alignment check: A10 gains the contract separating world-advancing commands, actor-known time displays, and structurally-non-diegetic debug time, with the why-not actor/debug split.
2. `INV-006`/`INV-108` possession parity → invariants alignment check: embodied time rendering uses the same holder-known classifier as autonomous actors (possession is not a temporal-knowledge upgrade).
3. Play legibility & lead usefulness (D-R1) → invariants alignment check: A10 names the embodied play loop; A11 makes lead usefulness a source-bound projection (no objective markers / quest stages / hidden priority); transcript evidence does not parse display prose as authority.
4. Two docs, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `07`/`10`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-T7 — temporal rendering / time-control contract in A10

Add to `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`: world-advancing controls are commands advancing authoritative event/replay time through the ordinary pipeline (not actor cognition); actor-facing time displays, missed-event summaries, time-to-work cues, waiting/sleeping summaries, office-closed messages, and lateness/expectation labels derive from the possessed actor's holder-known context or modeled observations/records/public cues; debug/operator panels may show exact event/replay time, due queues, and hidden temporal comparisons but are structurally non-diegetic and cannot feed embodied affordances or actor-visible reasons; rejection/why-not output preserves the actor-visible/debug split. No UI clock format, acceleration speed, wait-command vocabulary, or summary thresholds.

### 2. D-R1 — embodied play loop in A10 + lead usefulness in A11

Add to A10: name the embodied play loop — the player forms plans from actor-known view models, attempts semantic actions, receives actor-visible failure/why-not feedback, inspects source-bound notebook/lead surfaces, and uses debug only as non-diegetic review. Add to `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`: lead usefulness is a source-bound projection concern — leads can be stale, ambiguous, partial, contradictory, or actionable, but do not become objective markers, quest stages, or hidden priority; transcript evidence demonstrates the loop without parsing display prose as authority. No drama objectives or director controls.

## Files to Touch

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` (modify)
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` (modify)

## Out of Scope

- **UI clock format, time-acceleration speed, wait-command vocabulary, missed-summary thresholds** — execution/implementation (spec §6).
- **When world-advancing controls become player-facing vs debug-only** — execution `07` staging (spec §6).
- **A11 temporal-utterance/lead-label cross-reference** — sibling ticket 006 (D-T8).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T7 landing grep** — `grep -niE "time control|temporal|missed event|non-diegetic|why-not" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` resolves the temporal-rendering contract + play loop.
2. **D-R1 A11 landing grep** — `grep -niE "lead|stale|ambiguous|marker|quest|projection" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` resolves the lead-usefulness rule.
3. **Invariants alignment review** — upholds `INV-112` (actor-known time rendering), `INV-006`/`INV-108` (possession parity), `INV-024` (debug non-diegetic); leads create no objective markers/quest stages; no UI/threshold token introduced.

### Invariants

1. Actor-facing temporal displays derive from holder-known context/modeled channels; debug time is structurally non-diegetic and cannot feed embodied affordances; possession is not a temporal-knowledge upgrade (`INV-112`/`INV-024`/`INV-006`/`INV-108`).
2. Lead usefulness is a source-bound projection; leads never become objective markers, quest stages, or hidden priority (no-director doctrine).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (two landing greps) plus an invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE "time control|temporal|missed event|non-diegetic|why-not" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — confirms D-T7 + the play loop landed.
2. `grep -niE "lead|stale|ambiguous|marker|quest|projection" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — confirms D-R1's A11 rule landed.

## Outcome

Completed: 2026-06-15

Implemented D-T7 and D-R1 in `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` and `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`. A10 now has a `Temporal rendering and embodied play loop` subsection stating that world-advancing controls advance authoritative event/replay time through the ordinary pipeline, actor-facing temporal displays and labels derive from holder-known context or modeled channels, debug/operator temporal views are structurally non-diegetic, and rejection/why-not output preserves the actor-visible/debug split. A10 also names the embodied play loop and requires transcript evidence to rely on stable IDs/source references rather than display prose as authority.

A11 now states that lead usefulness is a source-bound projection concern: leads may be stale, ambiguous, partial, contradictory, or actionable for a holder, but they do not become objective markers, quest stages, hidden priority, or director-chosen next steps.

The execution-blocking owner-approval precondition from spec 0032 §R-A is satisfied by the user's explicit `$ticket-series implement the series tickets/0032ARCTIETEM*` request for this architecture-tier amendment series.

Verification:

- `grep -niE "time control|temporal|missed event|non-diegetic|why-not" docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `grep -niE "lead|stale|ambiguous|marker|quest|projection" docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- Manual invariants alignment review: the A10 addition preserves `INV-112`, `INV-024`, `INV-006`, and `INV-108`; the A11 addition preserves no-director/no-quest doctrine.
- Manual mechanism-token boundary review: no UI clock format, time-acceleration speed, wait-command vocabulary, missed-summary threshold, drama objective, or director control was introduced.

No crate/code or fixture changes were made for this documentation-only architecture ticket.
