# 0033EXETIETEM-003: exec 06 routine temporal premises & adaptation no-human proof

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (additive routine/social-rhythm temporal-premise obligations and positive learning/adaptation proof over the existing `NO-HUMAN` proof). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-002 (exec `05` scheduler temporal authority — D-T4 cross-references it so scheduler awakenings do not count as routine-premise evidence), 0033EXETIETEM-010 (exec `10` routine-failure diagnostics). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T3, D-T4, D-R2 (report F-03, F-04, F-09), the exec-`06` slices. Exec `06` owns ordinary-life needs, routines, and the no-human proof (`NO-HUMAN`) but treats routine temporal premises and routine adaptation as implicit: verified 0 `temporal`/`INV-112` matches in exec `06` at `c70d119`. Without these obligations a no-human run can pass while routines are selected from true schedule time rather than source-backed premises, and routine adaptation can occur via hidden truth. This ticket requires routine/social-rhythm proofs to name their temporal-premise source category, requires negative no-human examples, and requires positive adaptation proof through holder-known channels.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `06` (`06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`) owns needs-as-causal-pressure, routine templates, failure diagnostics, and no-human evidence, and carries no temporal-premise or learning-adaptation obligation (`grep -ciE 'temporal|INV-112' docs/2-execution/06_*` → 0). The gap is real; this ticket adds proof obligations.
2. Verified against spec 0033 §3.1 D-T3/D-T4 and §3.2 D-R2, and ratified upstream A05 routine temporal premises + A05/A06 learned-expectation seams (spec `0032`). The temporal-premise vocabulary and learning update rules/decay/thresholds route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `06` (routine premises + adaptation) ↔ exec `05` (scheduler awakenings, ticket 002) ↔ exec `10` (routine-failure diagnostics, ticket 010). D-T4 cross-references `05` so scheduler awakenings/elapsed-time accounting are not routine-premise evidence, and `10` so a routine failure is classified by cause. This ticket states the `06` proof obligation; the diagnostics live in `10`.
4. Constitutional invariant motivating this ticket, restated: `INV-112` — routine selection (cognition) may use temporal facts only when they reached the actor through modeled channels; the scheduler awakening an actor at a time is not such a channel. Routine templates organize method families but a template's presence is not itself an information channel.
5. Actor-knowledge / fail-closed surface: the obligations require each routine/social-rhythm proof to identify the temporal-premise **source category** it used (a modeled channel supplied the premise, not the vocabulary itself), negative no-human examples where a routine correct under true schedule time is **not** selected because the actor lacks the source-backed premise, and positive adaptation where repeated modeled experience/contradiction/interruption/notice changes later routine/method/trust selection through holder-known memory/expectation channels. This strengthens epistemic-leakage prevention; the prohibited-source negative (anti-truth-cache) counterpart is exec `04` (ticket 001) and the cause-classifying diagnostics are exec `10` (ticket 010).

## Architecture Check

1. Exec `06` is the correct home: it already owns the no-human ordinary-life proof and routine templates, so routine temporal premises and positive adaptation are specializations of contracts `06` already carries. Putting routine-premise proof in the scheduler doc would conflate "process activated at a time" with "actor selected a routine using a temporal belief."
2. No backwards-compatibility aliasing/shims: additive proof obligations over the existing `NO-HUMAN` proof; no rename, no weakening, no temporal vocabulary chosen.

## Verification Layers

1. `INV-112` routine temporal premises → invariants alignment check: exec `06` requires each routine/social-rhythm proof to name the modeled source category of its temporal premise, with negative no-human examples where true-time-only justification is insufficient.
2. `INV-112` positive adaptation → invariants alignment check: exec `06` requires repeated modeled experience/contradiction/interruption/notice to change later routine selection through holder-known memory/expectation channels.
3. Cross-reference integrity → codebase grep-proof: exec `06` points to exec `05` (scheduler awakenings excluded as premise evidence, ticket 002) and exec `10` (routine-failure cause classification, ticket 010).
4. Documentation-only doctrine ticket: replay/golden-fixture and skill-dry-run layers are exec `09`/`10` proof surfaces; the layers above map each engaged invariant and the cross-reference integrity to distinct surfaces.

## What to Change

### 1. D-T3 / D-T4 — routine/social-rhythm temporal-premise obligation

Require each routine or social-rhythm proof in exec `06` to identify the temporal-premise source category it used (the fact that a modeled channel — assignment, memory, observation, public cue, record, testimony, institutional context, or source-backed inference — supplied the premise, not the vocabulary itself), require first-playable no-human scenarios where routine behavior succeeds from modeled temporal premises and waits/fails when only ground-truth time would justify action, and require negative no-human examples where a routine correct under true schedule time is not selected because the actor lacks the source-backed premise. Cross-reference exec `05` so scheduler awakenings/elapsed-time accounting do not count as routine-premise evidence.

### 2. D-R2 — positive learning/adaptation proof

Require positive adaptation proof: repeated modeled experience, contradiction, interruption, or changed routine outcomes can affect future routine/method/trust selection through holder-known memory/expectation channels. Cross-reference exec `10` so a routine failure is classified by cause (missing knowledge, stale knowledge, budget exhaustion, blocked affordance, validation failure). Learning update rules/decay/thresholds route to a future scoped spec (§6).

## Files to Touch

- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)

## Out of Scope

- **Concrete temporal vocabulary, day-part/lateness terms** — reference/future scoped specs (§6).
- **Learning update rules, decay, trust-update semantics, thresholds** — future scoped spec (§6).
- **Scheduler temporal-authority statement** — exec `05` (ticket 002); **routine-failure diagnostics** — exec `10` (ticket 010); **prohibited-source anti-truth-cache negative** — exec `04` (ticket 001).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T3/D-T4 landing grep** — exec `06` carries the routine temporal-premise obligation: `grep -niE 'temporal premise|routine.*(premise|temporal)|no-human' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` resolves the obligation and a negative-example requirement.
2. **D-R2 landing grep** — positive adaptation proof present: `grep -niE 'adaptation|learn|expectation' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` resolves it.
3. **Cross-reference check** — exec `06` points to exec `05` (awakenings excluded) and exec `10` (failure cause classification).
4. **Mechanism-token + invariants review** — no temporal vocabulary or learning update rule entered exec `06`; upholds `INV-112`, preserves `NO-HUMAN` (no rename/weaken); no new gate code.

### Invariants

1. Routine selection uses temporal facts only via a named modeled channel; scheduler awakening is not such a channel (`INV-112`).
2. Routine adaptation flows through holder-known memory/expectation channels, never hidden truth (`INV-112`, with the prohibited-source negative owned by exec `04`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a cross-reference, mechanism-token-boundary, and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal premise|no-human|adaptation|expectation' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — confirms D-T3/D-T4/D-R2 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the cross-reference and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `06` routine temporal-premise and positive adaptation
proof obligations in
`docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`. The
edit requires routine/social-rhythm proof to identify modeled temporal-premise
source categories, requires no-human negative cases where true-time-only
justification is insufficient, and requires adaptation to flow through
holder-known memory or expectation channels.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal premise|routine.*(premise|temporal)|no-human' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `grep -niE 'adaptation|learn|expectation' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `rg -n '05_TRANSACTION|10_TESTING|temporal vocabulary|learning update rule|decay|trust-update|threshold|NO-HUMAN|new gate' docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `git diff --check`

Manual review confirmed the additions uphold `INV-112`, preserve `NO-HUMAN`
without rename or weakening, correctly cross-reference exec `05` and exec `10`,
and introduce no temporal vocabulary, learning update rule, decay,
trust-update semantics, threshold, or new gate code.
