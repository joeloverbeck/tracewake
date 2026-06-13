# 0028EXETIEDOC-004: Observation-time snapshot proof, wallhack negatives, and embodied carrier census

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (observation-time snapshot proof + wallhack negatives) plus support in `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` and `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (embodied carrier census). No crate/code, no fixtures.
**Deps**: 0028EXETIEDOC-006 (the doc-`10` carrier-census evidence references the general anti-vacuity / behavior-witness standard from D8 in `10`). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D5 (report E04). Foundation `06` requires proposal from perceived/known affordance with validation against actual affordance; architecture `10` (ratified by spec 0027) now requires embodied view models and possession preflight to consume holder-known context + permitted projection records captured at modeled observation/bind/preflight/perception boundaries — not re-read truth or freshen by live physical state. Execution `07` already proves the rendered view omits hidden truth and requires possession parity, but it proves nothing about **capture sufficiency at formation time**: that an actor-visible action/view was backed by a captured modeled-observation carrier rather than a live physical-state handle (verified: 0 `snapshot`/`carrier`/`wallhack`/`observation-time` matches in `07` at `64a8367`). This ticket adds an observation-time snapshot proof + wallhack negatives to `07` and an embodied carrier census to `10`. Per spec §6 / §7 R-E it leaves the **possession-bind perception** question (F04) neutral pending the owner decision.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` requires epistemic view models to derive from holder-known context + immediate perceptions, excludes hidden truth / debug-only facts, requires possession parity, and carries negative fixture classes for debug omniscience and view filtering — but `grep -inE "snapshot|carrier|wallhack|observation-time|captured"` returned 0, so it has no observation-time-capture proof and no wallhack negative set. The capture contract is the new architecture-`10` idea this ticket proves at the execution tier.
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D5 and §7 R-E, and the source report `reports/execution-tier-alignment-research-report.md` §E04 (incl. the F04 residual). Upstream contract: architecture `10` embodied-capture rule, ratified by `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` D5.
3. Shared boundary under audit: the possession/embodied view-model surface (`07`), the actor-known capture support (`04`), and the carrier-census review artifact (`10`). The `10` carrier census is a *specific* evidence requirement instancing the *general* behavior-witness standard owned by D8 (sibling ticket 006). `07`'s freshness parity (D4) is owned by sibling ticket 0028EXETIEDOC-003 — this ticket adds the orthogonal *capture* proof; the two are additive, non-conflicting `07` subsections.
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-099` (truth may validate, not plan — affordance enumeration must come from actor-known/perceived access, not hidden truth), `INV-024` (no telepathy), `INV-006` (possession transfers no world knowledge), `INV-108` (human possession is cognition-neutral). D5 forces actor-visible actions/views onto observation-time captured carriers so hidden truth cannot enter affordance enumeration while validators still reject on truth.
5. Actor-knowledge-filtering + no-leak surface (enforcement is the view-generation path + the `10` standard in ticket 006, not this edit): the snapshot proof requires evidence of the holder, the modeled observation/bind/preflight/perception boundary, the captured facts, their provenance/freshness, and the **absence of a live-truth handle** in the view-generation path; wallhack negatives must fail when a route/workplace/sleep affordance/container content/item location/routine opportunity is true in the world but not known/perceived by the holder. This strengthens, not weakens, the no-leak firewall (INV-024/006) and keeps validation-on-truth separate from actor-visible feedback. No code path, no leakage, no nondeterminism added — proof doctrine + a review-artifact census only.

## Architecture Check

1. The capture proof lives in `07` (the possession/view-model proof doc) with the carrier census in `10` (review artifacts), because `07` owns the embodied/possession surface and `10` owns review evidence. Proving capture *sufficiency at formation time* — not merely that the final view omits hidden facts — is the gap architecture `10` opened; co-locating it with `07`'s existing possession parity keeps the embodied-proof surface coherent.
2. No backwards-compatibility aliasing/shims: additive proof obligations. `07`'s existing view/debug-split proof is untouched; the snapshot proof and wallhack negatives are new subsections, the carrier census a new `10` review-artifact requirement.

## Verification Layers

1. `INV-099`/`INV-024`/`INV-006`/`INV-108` observation-time capture → invariants alignment check: `07` requires evidence that an actor-visible action/menu/preflight/view is backed by a captured modeled-observation carrier (holder, boundary, captured facts, provenance/freshness) with no live-truth handle in the generation path.
2. Wallhack negatives → manual review (no-leak negative-case audit): `07` enumerates the negative classes — true-but-unknown routes, workplaces, sleep affordances, container contents, item locations, routine opportunities — each failing when true in the world but not known/perceived by the holder.
3. Embodied carrier census → manual review against the `10` standard (ticket 006): `10` requires a census letting reviewers see every actor-visible datum's carrier and provenance.

## What to Change

### 1. `07` — observation-time snapshot proof (D5)

Add a subsection (final wording at enactment): for any actor-visible action, menu option, possession preflight, or embodied view, evidence must show the holder, the modeled observation/bind/preflight/perception boundary, the captured facts, their provenance/freshness, and the **absence of live-truth handles** in the view-generation path. Proves capture sufficiency at formation time, not merely that the final view omits hidden facts.

### 2. `07` — wallhack negatives (D5)

Add wallhack negative classes that must fail when the datum is true in the world but not known/perceived by the holder: true-but-unknown routes, workplaces, sleep affordances, container contents, item locations, and routine opportunities.

### 3. `10` — embodied carrier census (D5)

Add a carrier-census review-artifact requirement (referencing the D8 general standard in ticket 006): reviewers can see, for every actor-visible datum, its carrier and provenance.

### 4. F04 residual — keep `07` neutral on possession-bind perception

Record (per spec §7 R-E) that whether possession *bind* itself counts as modeled perception is an owner decision; `07` assumes neither policy. Once the owner decides, `07` proves the chosen policy with the same snapshot/parity discipline (a bounded bind-time perception snapshot with provenance/freshness, **or** no perception and no freshening). Any bind perception must be a modeled event/channel for the actor, never a human/player knowledge transfer (INV-006/INV-108).

## Files to Touch

- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify — shared with 0028EXETIEDOC-003 D4 parity, additive)
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` (modify — capture support; shared with 0028EXETIEDOC-003, additive)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — carrier census; shared hub with 0028EXETIEDOC-003/005/006; references the D8 standard in 006)

## Out of Scope

- **Deciding the possession-bind perception policy (F04)** — owner decision (spec §7 R-E); this ticket keeps `07` neutral and ready to prove either policy.
- **The general anti-vacuity / behavior-witness standard in `10`** — D8, sibling ticket 0028EXETIEDOC-006.
- **The freshness classifier in `04`/`07`** — D4, sibling ticket 0028EXETIEDOC-003 (this ticket adds the orthogonal capture proof).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Snapshot-proof landing grep** — `07` requires observation-time capture evidence: `grep -niE "snapshot|carrier|observation.time|captured.*boundary|live.truth handle" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` resolves the snapshot-proof subsection.
2. **Wallhack-negatives landing grep** — `07` enumerates the six negative classes: `grep -niE "wallhack|true-but-unknown|route|workplace|sleep affordance|container content|item location|routine opportunity" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` resolves the negative set.
3. **F04 neutrality review** — `07` records the possession-bind perception question as an owner decision and asserts neither policy (spec §7 R-E).
4. **Invariants alignment review** — D5 forces actor-visible affordances onto captured carriers (INV-099/024/006/108), keeping hidden truth out of enumeration while validators still reject on truth.

### Invariants

1. Every actor-visible action/menu/preflight/view is backed by a captured modeled-observation carrier, with no live-truth handle in the generation path (INV-099/024).
2. Possession is not a knowledge upgrade; the capture/parity discipline applies equally to no-human and possessed surfaces (INV-006/108), and the bind-perception policy stays an unresolved owner decision (F04).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (snapshot + wallhack landing greps) plus a no-leak negative-case / invariants-alignment manual review against the architecture 10 capture contract. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "snapshot|carrier|observation.time|live.truth" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — confirms the observation-time snapshot proof.
2. `grep -niE "wallhack|true-but-unknown|sleep affordance|container content|routine opportunity" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — confirms the wallhack negative classes.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the no-leak / invariants-alignment review against architecture 10.`

## Outcome

Completed: 2026-06-13

Owner approval precondition: satisfied by the user's active `$ticket-series`
goal to implement `tickets/0028EXETIEDOC*` against
`specs/0028_EXECUTION*`.

Changed:

- Added actor-visible capture support to
  `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`.
- Added observation-time snapshot proof, wallhack negatives, and possession-bind
  perception neutrality to
  `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`.
- Added the embodied carrier census requirement to
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.

Verification:

- `grep -niE "snapshot|carrier|observation.time|captured.*boundary|live.truth handle" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
  resolved the snapshot proof.
- `grep -niE "wallhack|true-but-unknown|route|workplace|sleep affordance|container content|item location|routine opportunity" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
  resolved the six wallhack negative classes.
- `grep -niE "possession-bind|owner decision|bind-time perception|no perception|no freshening" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
  resolved the F04 neutrality language.
- `grep -niE "carrier census|actor-visible datum|capture boundary|live-truth handle" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
  resolved the carrier/capture support.
- Invariants alignment review: actor-visible affordances are tied to captured
  holder-known carriers, hidden truth stays out of view generation, and
  possession remains cognition-neutral.

Deviations:

- None. The Rust gate pipeline was not run for this ticket because the accepted
  verification boundary is documentation grep plus no-leak / invariants review.
