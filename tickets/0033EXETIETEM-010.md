# 0033EXETIETEM-010: exec 10 consolidated diagnostics & review artifacts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (the consolidated home for all new temporal / completeness / staged-abstraction diagnostic and review-artifact obligations, including the single budget/fairness contract). No crate/code, no fixtures.
**Deps**: None (ticket-internal; this is the consolidated diagnostics home that sibling tickets 002/004/007/008 cross-reference, so it should land early). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T2, D-T3, D-T5, D-T7, D-R1, D-R2, D-R3, D-R4, D-R5, D-S1 (report F-02/03/05/07/08/09/10/11/12/13), the exec-`10` slices. Exec `10` owns test families, diagnostic standards, responsible-layer labels, `EMERGE-OBS` observer-only evidence, anti-vacuity, and review templates — the tier's diagnostics home — but carries **no** temporal, budget/fairness, LOD-ancestry, learning, quantity-lineage, bias, or staged-abstraction diagnostic fields: verified 0 `temporal`/`INV-112` matches in exec `10` at `c70d119`. Per spec §R-D the budget/fairness contract must be stated **once** here (not triplicated across temporal/scheduler/observability), and per §R-F the new review summaries must stay observer-only. This ticket consolidates all the diagnostic/review-artifact additions into exec `10`, additively over the existing diagnostic posture.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `10` (`10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) owns the test families, diagnostic standards, responsible-layer labels, `EMERGE-OBS`, anti-vacuity, and review templates, and carries no temporal/budget/LOD/learning/quantity/bias/staged diagnostic fields (`grep -ciE 'temporal|INV-112' docs/2-execution/10_*` → 0). The gap is real; this ticket adds diagnostic-field obligations over the existing posture.
2. Verified against spec 0033 §3.1/§3.2/§3.3 (the ten exec-`10` slices) and ratified upstream A02/A13 temporal observability, A13 budget/fairness + compiler-discipline evidence shape (spec `0032`). Concrete diagnostic field names, fairness formulas, and thresholds route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `10` is the consolidated diagnostics home cross-referenced by exec `05` (ticket 002), `07` (ticket 004), `11` (ticket 007), `12` (ticket 008), and the Block-T gates. Per spec §R-D the budget/fairness contract is stated once here; the sibling tickets point to it. This ticket owns the canonical diagnostics; it must not be triplicated.
4. Constitutional invariants motivating this ticket, restated: `INV-105` (actor decision traces / diagnostics are authoritative typed data, not display strings); `INV-111` (living-world acceptance requires observer-only emergence evidence — the new temporal/LOD/fairness/staged review summaries stay observer-only, non-certifying, never simulation inputs or actor-known urgency); `INV-087` (human focus is not player privilege — fairness evidence proves no human-proximity/possessed-actor bias). The obligations specialize these for the new payload classes.
5. Observability / fail-closed / deterministic-replay surface (this IS the diagnostics doc, and the `EMERGE-OBS` observer-only carve-out lives here): the obligations require typed, responsible-layer-attributed diagnostic fields for temporal divergence, validator-vs-holder review, temporal rendering, LOD temporal-ancestry/fairness, quantity/custody lineage, learning, the consolidated budget/fairness contract, bias source, compiler-discipline review evidence, and staged-abstraction review — all observer-only and non-certifying (`INV-111`/`EMERGE-OBS` preserved per spec §R-F: counters/summaries must not become simulation objectives or actor-known urgency). This strengthens typed diagnostics and observer-only discipline and introduces no determinism change.

## Architecture Check

1. Exec `10` is the correct single home: it already owns the tier's diagnostics, responsible-layer labels, and the `EMERGE-OBS` observer-only carve-out, so consolidating the new diagnostic fields here (rather than scattering budget/fairness across `04`/`05`/`12`) directly serves spec §R-D ("stated once") and §R-F (observer-only preserved). One Large but thematically unified diff ("all new diagnostic/review-artifact requirements for the tier") is the cleanest expression of the consolidated-home intent.
2. No backwards-compatibility aliasing/shims: additive diagnostic-field obligations over the existing diagnostic posture; no rename, no weakening, no concrete field name / fairness formula / threshold chosen.

## Verification Layers

1. `INV-105` typed temporal/completeness diagnostics → invariants alignment check: exec `10` requires temporal-divergence, validator-vs-holder, rendering, LOD-ancestry, quantity-lineage, learning, and bias-source diagnostics as typed, responsible-layer-attributed fields, not display strings.
2. `INV-105`/`INV-087` consolidated budget/fairness (D-R3, §R-D) → invariants alignment check: exec `10` is the single home for budget/fairness diagnostics (starvation risk, repeated deferral, actor-/region-class imbalance, time-acceleration effects, replay determinism, no human-proximity/possessed-actor bias); `05`/`12`/Block-T cross-reference it.
3. `INV-111` observer-only / staged-abstraction review (D-S1, §R-F) → invariants alignment check: the new review summaries and staged-abstraction review fields stay observer-only, non-certifying, never simulation inputs or actor-known urgency; the `EMERGE-OBS` carve-out is preserved.
4. Documentation-only doctrine ticket: the gates these diagnostics observe are exec `04`/`05`/`06`/`07`/`08`/`11`/`12` (sibling tickets) and the fixtures are exec `09` (ticket 006); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. Temporal diagnostics (D-T2, D-T3, D-T5, D-T7)

Add, as typed responsible-layer-attributed diagnostic/review fields: temporal-divergence diagnostics labelling whether leakage came from candidate generation, sealed-context assembly, scheduler dispatch, action validation, projection/view rendering, fixture authoring, or review-artifact construction (D-T2); review artifacts distinguishing "validator time used to validate" from "holder-known temporal premise used to plan," with both positive-acceptance and fail-closed paths (D-T3); temporal-rendering diagnostics distinguishing embodied, possession, debug, transcript, and observer-only surfaces (D-T5); and LOD temporal-ancestry and fairness review diagnostics (D-T7).

### 2. Completeness diagnostics (D-R1, D-R2, D-R4, D-R5)

Add: diagnostics identifying representation error, custody-lineage error, procedure-visibility error, and replay divergence for quantity/economy (D-R1); diagnostics distinguishing learned expectation, remembered event, direct observation, testimony, record-derived belief, and prohibited truth-cache update (D-R2); diagnostics identifying the modeled source of a biased/harmful outcome and distinguishing it from hidden-truth leakage or arbitrary author fiat (D-R4); and review evidence that static validation ran, negative fixtures failed for the intended reason, and diagnostics identified the responsible layer (D-R5).

### 3. Consolidated budget/fairness contract (D-R3 — §R-D, stated once here)

Add exec `10` as the single consolidated home for budget/fairness diagnostics: starvation risk, repeated deferral, actor-class/region-class imbalance, time-acceleration effects, and replay determinism, with no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing. Exec `05`/`12` and the Block-T tickets cross-reference this; do not triplicate. Choose no fairness formula, window, or threshold (§6).

### 4. Staged-abstraction review fields (D-S1)

Add staged-abstraction review fields: proof currently provided; behavior intentionally abstracted; falsehoods the stage must not fake; future feature or tier it must not block; evidence that prevents overclaiming; diagnostics that would fail if the abstraction leaks into certification. Keep these observer-only and non-certifying.

All additions are observer-only and non-certifying (spec §R-F); the `EMERGE-OBS` carve-out is preserved. Choose no concrete diagnostic field names, fairness formulas, or thresholds (§6); mint no new observation-obligation or gate code.

## Files to Touch

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- **Concrete diagnostic field names, fairness formulas, window sizes, thresholds** — future scoped specs (§6).
- **The gates these diagnostics observe** — exec `04`/`05`/`06`/`07`/`08`/`11`/`12` (sibling tickets); **the fixtures** — exec `09` (ticket 006).
- **Minting a new observation-obligation code or gate code** — forbidden (spec §R-G); diagnostics thread into the existing posture.
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Temporal-diagnostics landing grep** — `grep -niE 'temporal[- ]divergence|responsible.layer|validator time|temporal.*render|LOD.*ancestry' docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves D-T2/D-T3/D-T5/D-T7.
2. **Completeness + budget/fairness landing grep** — `grep -niE 'custody|truth[- ]cache|budget|fairness|starv|staged' docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves D-R1/D-R2/D-R3/D-R4/D-R5/D-S1.
3. **Consolidation review (§R-D)** — manual review confirms the budget/fairness contract is stated once here and only cross-referenced from `05`/`12`/Block-T, not triplicated.
4. **Observer-only review (§R-F / `INV-111`)** — manual review confirms the new review summaries and staged-abstraction fields stay observer-only, non-certifying, and never simulation inputs/actor-known urgency; `EMERGE-OBS` carve-out preserved.
5. **Mechanism-token + no-new-code review** — no diagnostic field name, fairness formula, or threshold entered exec `10`; no new observation-obligation or gate code.

### Invariants

1. All new diagnostics are typed, responsible-layer-attributed fields, not display strings (`INV-105`).
2. The budget/fairness contract is stated once (`INV-105`, §R-D); the new review summaries stay observer-only and non-certifying (`INV-111`/`EMERGE-OBS`, §R-F).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus consolidation, observer-only, and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal[- ]divergence|validator time|custody|truth[- ]cache|budget|fairness|staged' docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — confirms the ten exec-`10` slices landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the consolidation, observer-only, and invariants-alignment review.`
