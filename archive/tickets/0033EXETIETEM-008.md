# 0033EXETIETEM-008: exec 12 deferred LOD temporal ancestry & fairness

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` (additive deferred LOD/time-acceleration temporal-ancestry and fairness proof obligations; the deferral itself is kept intact). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-010 (the consolidated budget/fairness diagnostics home + LOD temporal-ancestry diagnostics in exec `10`; the deferred-scale fairness declarations cross-reference it per spec §R-D). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T7, D-R3 (report F-07, F-10), the exec-`12` slices. Exec `12` correctly locks notices/travel/regional scale/LOD/story-sifting as deferred second proof, but carries **no** acceptance obligation the future second proof must not escape: verified 0 `temporal`/`INV-112` matches in exec `12` at `c70d119`. Without these obligations a future LOD/time-acceleration feature could promote omniscient temporal facts or starve actor classes silently. This ticket adds the deferred-work proof obligations (temporal/information ancestry, equivalence-or-declared-divergence, promotion/demotion, skipped-cognition accounting, fairness declarations) while keeping `12`'s deferral intact.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `12` (`12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`) defers notices/travel/LOD/story-sifting and carries no LOD temporal-ancestry or fairness obligation (`grep -ciE 'temporal|INV-112' docs/2-execution/12_*` → 0). The gap is real; this ticket adds proof obligations for deferred work, it does not undefer the feature.
2. Verified against spec 0033 §3.1 D-T7 and §3.2 D-R3, and ratified upstream A12 LOD temporal-ancestry + time-acceleration declaration + fairness seam (spec `0032`). LOD equivalence tolerance, regional cadence values, promotion algorithms, and fairness formulas route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `12` (deferred LOD obligations) ↔ exec `10` (consolidated budget/fairness + LOD temporal-ancestry diagnostics, ticket 010). Per spec §R-D the budget/fairness contract is stated once in exec `10`; this ticket states the deferred-scale fairness declarations and cross-references `10` (no triplication).
4. Constitutional invariant motivating this ticket, restated: `INV-110` (LOD and summary processes must preserve the firewall — summaries retain enough causal/epistemic ancestry that later promoted simulation is not contaminated by hidden truth). The obligations specialize `INV-110` for deferred temporal/time-acceleration work.
5. Deterministic-replay / fail-closed surface (substrate-only; enforcement is the future second proof, cross-referenced to the exec `10` diagnostics home): the obligations require future LOD/time-acceleration proof to declare interval, cadence, resolution, fidelity, event ancestry, information ancestry (distinct from event-time ancestry), and known-to-whom status for every summary/promoted state; require equivalence-or-declared-divergence evidence (any accelerated/regional divergence is declared, bounded, and tested against fairness and epistemic constraints); require promotion/demotion proof that actors/institutions/leads/views gain no omniscient temporal facts during LOD transitions; and require skipped/deferred-cognition accounting so acceleration cannot invisibly starve actor classes or silently script outcomes. This preserves the firewall under summarization and introduces no nondeterminism; the cross-cutting fairness contract lives once in exec `10`.

## Architecture Check

1. Exec `12` is the correct home: it already owns the deferred second-proof scope, so the LOD/time-acceleration proof obligations belong here as the gate the future work must satisfy — stated now so the deferral cannot escape them. Stating the cross-cutting fairness contract here instead of in exec `10` would triplicate it (spec §R-D).
2. No backwards-compatibility aliasing/shims: additive obligations over the deferral; no rename, no weakening, no LOD threshold/algorithm chosen, and the deferral stays intact.

## Verification Layers

1. `INV-110` LOD temporal/information ancestry → invariants alignment check: exec `12` requires every future summary/promoted state to declare interval/cadence/resolution/fidelity/event ancestry/information ancestry/known-to-whom, with no omniscient temporal fill-in on promotion/demotion.
2. Fairness / no-starvation → invariants alignment check + cross-reference: exec `12` requires equivalence-or-declared-divergence and skipped/deferred-cognition accounting, cross-referencing the consolidated exec `10` budget/fairness home (no triplication).
3. Documentation-only doctrine ticket for deferred work: the diagnostics are exec `10` (ticket 010); no replay/golden-fixture layer applies now (the feature is deferred); the layers above map each engaged obligation to a distinct surface.

## What to Change

### 1. D-T7 — LOD / time-acceleration temporal-ancestry obligations

Require future LOD/time-acceleration proof to declare interval, cadence, resolution, fidelity, event ancestry, information ancestry, and known-to-whom status for every summary or promoted state; require equivalence-or-declared-divergence evidence (accelerated/regional divergence from full-resolution processing declared, bounded, tested against fairness and epistemic constraints); require promotion/demotion proof that actors, institutions, leads, and views gain no omniscient temporal facts during LOD transitions; require skipped/deferred-cognition accounting so acceleration cannot invisibly starve actor classes or silently script outcomes. Keep `12`'s deferral intact; choose no LOD equivalence tolerance, cadence value, or promotion algorithm (§6).

### 2. D-R3 — deferred-scale fairness declarations

Require deferred-scale fairness declarations for any future LOD/time-accelerated process, cross-referencing the consolidated budget/fairness diagnostics home in exec `10` (spec §R-D). Choose no fairness formula, window, or threshold (§6).

## Files to Touch

- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` (modify)

## Out of Scope

- **LOD equivalence tolerance, regional cadence values, promotion algorithms, fairness formulas/windows/thresholds** — future scoped specs (§6).
- **The consolidated budget/fairness contract** — exec `10` (ticket 010); this ticket cross-references it (spec §R-D, no triplication).
- **Undeferring notices/travel/regional scale/LOD** — out of scope; the deferral stays intact.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T7 landing grep** — exec `12` carries the LOD temporal-ancestry obligation: `grep -niE 'temporal ancestry|information ancestry|known-to-whom|promotion' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` resolves it.
2. **D-R3 landing + consolidation grep** — deferred-scale fairness declarations present and cross-reference the exec `10` home: `grep -niE 'fairness|divergence|starv' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` resolves the declarations and a pointer to exec `10`.
3. **Deferral-intact + mechanism-token review** — manual review confirms the deferral is preserved and no LOD threshold/cadence/algorithm/fairness formula entered exec `12`; no new gate code.
4. **Invariants alignment review** — upholds `INV-110`, preserves the deferred second-proof lock (no rename/weaken); budget/fairness stated once (in `10`), referenced here.

### Invariants

1. Every future LOD summary/promoted state preserves temporal + information ancestry and creates no omniscient temporal facts on promotion/demotion (`INV-110`).
2. Deferred-scale fairness/no-starvation is a declared obligation referencing the single exec `10` home (`INV-110`, spec §R-D).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket for deferred work; verification is command-based (landing greps) plus a deferral-intact and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal ancestry|information ancestry|known-to-whom|fairness|divergence' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — confirms D-T7/D-R3 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the deferral-intact and invariants-alignment review.`

## Outcome

Completed: 2026-06-15

Implemented the exec `12` deferred LOD/time-acceleration temporal-ancestry and
fairness proof obligations in
`docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`.
The edit requires future LOD/time-acceleration proof to declare temporal,
event, and information ancestry plus known-to-whom status; to declare and bound
accelerated/regional divergence; to prove no omniscient temporal facts enter
promotion/demotion; and to account for skipped/deferred cognition so
acceleration cannot silently starve actor classes or script outcomes.

The execution-blocking owner-approval precondition in spec 0033 was satisfied
by the user's explicit request to implement the `0033EXETIETEM` ticket series.
No crate/code or fixture files were changed.

Verification:

- `grep -niE 'temporal ancestry|information ancestry|known-to-whom|promotion' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `grep -niE 'fairness|divergence|starv' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `rg -n 'locked-deferral|blocked until|does not authorize implementation|formula|window|threshold|regional cadence|LOD tolerance|promotion algorithm|new gate|10_TESTING' docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `git diff --check`

Manual review confirmed the additions uphold `INV-110`, preserve the deferred
second-proof lock without rename or weakening, correctly cross-reference exec
`10`, and introduce no LOD threshold, cadence value, promotion algorithm,
fairness formula, window, threshold, or new gate code.
