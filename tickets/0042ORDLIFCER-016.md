# 0042ORDLIFCER-016: Acceptance capstone — per-seam verdict tables, replay/provenance & mutation packages, staged abstraction, EMERGE-OBS, and aggregate ORD-LIFE-CERT verdict

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — consolidates the audit-point / generated-evidence / mutation evidence into the §9.3 verdict tables, assembles the §9.5–§9.7 replay-provenance and mutation packages, the §9.8 staged-abstraction declaration, and the §9.10 EMERGE-OBS slot, and renders the §9.11 aggregate verdict; introduces no production logic.
**Deps**: 0042ORDLIFCER-001, 0042ORDLIFCER-002, 0042ORDLIFCER-003, 0042ORDLIFCER-004, 0042ORDLIFCER-005, 0042ORDLIFCER-006, 0042ORDLIFCER-007, 0042ORDLIFCER-008, 0042ORDLIFCER-009, 0042ORDLIFCER-010, 0042ORDLIFCER-011, 0042ORDLIFCER-012, 0042ORDLIFCER-013, 0042ORDLIFCER-014, 0042ORDLIFCER-015

## Problem

Spec §9 requires a single acceptance artifact that, beyond the per-gate evidence, renders the consolidated verdict: per-seam verdict tables for ORD-LIFE-01…12, the ten live pass conditions, and the seven mandatory fixture families (§9.3); the replay/provenance package (§9.6) and mutation package (§9.7); the staged-abstraction declaration (§9.8); the observer-only EMERGE-OBS slot (§9.10); and the aggregate verdict rule (§9.11) that may render `ORD-LIFE-CERT passed` only when every certifying condition holds, else `ORD-LIFE-CERT scoped remediation`. This capstone consolidates the prior tickets' evidence and renders that verdict; it adds no new production logic.

## Assumption Reassessment (2026-06-20)

1. The consolidated artifact `reports/0042_…_acceptance.md` is created by `0042ORDLIFCER-001` and populated section-by-section by `-002`…`-015`; the mutation triage register `reports/0042_ord_life_cert_mutation_triage_register.md` is created by `-015`; the acceptance template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` supplies the §9.4 evidence-item ledger fields (all verified at `98dc042`).
2. Spec §9 (acceptance-artifact contract: §9.1–§9.11) and §10 (preliminary static survey — informational, non-certifying) govern this ticket; §6.1 maps the ten pass conditions and §6.2 the seven mandatory fixture families that the verdict tables must mirror. Spec §11 lists the tolerated deferrals (SPINE/EPI consumed, temporal cascade and success-recovery variant routed downstream) the verdict must respect.
3. Cross-artifact shared boundary under audit: the §9.3–§9.11 capstone sections of `reports/0042_…_acceptance.md`; this ticket appends only those sections and reads (does not re-run) the per-gate / harness / mutation evidence its Deps produced.
4. Motivating invariants (spec §9.11 / §10): `INV-111` (living-world acceptance requires observer-only emergence evidence; `EMERGE-OBS` is never a gate/threshold/objective), `INV-018` (deterministic replay underpins the §9.6 package), the aggregate-rule discipline that no row may pass from historical/pending/sampled-only/observer-only/static-only/snapshot-only/aggregate-score evidence. Restate before trusting the narrative: the verdict is computed only from certifying evidence; a substantive failed point or mutation floor forces `scoped remediation`.
5. Evidence-consumer surface (audit-reads/consolidates, modifies no enforcement surface): the replay/provenance, actor-knowledge, and validation evidence assembled across all gates. This capstone introduces no production code, no leakage path, and no nondeterminism; it confirms each verdict row derives only from certifying evidence and that the §9.10 EMERGE-OBS member stays `status = observer-only` and non-gating.

## Architecture Check

1. A single acceptance-only capstone that consolidates verdicts and renders the aggregate result — introducing no new production logic — keeps the verdict computation auditable in one place and prevents any per-gate ticket from prematurely claiming the gate; it matches the EPI-CERT capstone shape (`archive/tickets/0040EPICERHOL-015`).
2. No backwards-compatibility aliasing/shims introduced; this assembles tables and renders a verdict over existing evidence. No production logic changes.

## Verification Layers

1. Aggregate verdict rule (§9.11) -> manual review (all twelve audit points + ten pass conditions + seven fixture families pass from certifying evidence; clean baseline + every required named command pass; configured mutation perimeter complete with no actionable/pending/untriaged/timed-out floor; predecessor citations scoped and Phase 4 blocked — else `ORD-LIFE-CERT scoped remediation`).
2. Evidence-honesty (§9.4) -> manual review + codebase grep-proof (every cited item carries evidence status / fingerprint scope / behavior witness / replay-provenance / sampling / pending-historical / certification-use; no row passes from historical/pending/sampled-only/observer-only/static-only/snapshot-only/aggregate-score evidence).
3. `INV-111` EMERGE-OBS quarantine -> manual review (the §9.10 EMERGE-OBS member is `status = observer-only` and never a phase gate, threshold, scheduler objective, scenario goal, mutation substitute, or code-quality score).

## What to Change

### 1. Assemble the §9.3 verdict tables and §9.5–§9.8 packages

In the acceptance artifact, build the three §9.3 verdict tables (one row per ORD-LIFE-01…12; one per the ten live pass conditions; one per the seven mandatory fixture families), each naming responsible layer, certifying evidence item IDs, negative controls, mutation entries, and a result computed only from certifying evidence. Assemble the §9.5 ordinary-life behavior-witness package, the §9.6 replay/provenance package (exact log bytes + parsed semantic scope, replay-handling census, applied event range, content/ruleset/seed identity, live/replay fingerprints, actor-known source-event table, proposal context parity, per-need/tick ledger, metric + stuck-diagnostic derivation, decision-context hash rebuild), and the §9.7 mutation package (version, config fingerprint, file/mutant census, baseline, in-diff run, full configured run, caught/missed/timeout counts, survivor register, equivalence proofs, final floor). Instantiate the §9.8 staged-abstraction declaration (what is proven now / abstracted / must-not-fake / must-not-block / overclaim-prevention / not-implemented-vs-abstracted-vs-broken-vs-overclaimed diagnostics).

### 2. Render the §9.10 EMERGE-OBS slot and the §9.11 aggregate verdict

Add the §9.10 EMERGE-OBS member as `status = observer-only` (summarizing emergent patterns / actor variety / causal surprises) explicitly marked non-gating. Apply the §9.11 aggregate verdict rule and render `ORD-LIFE-CERT passed` only if all eight conditions hold; otherwise render `ORD-LIFE-CERT scoped remediation`, naming every failed row/layer/evidence-gap/survivor and routing to a later separately-numbered remediation/replacement spec. State the final wording limited to the exact tested commit and evidence package; respect the §11 deferrals (SPINE/EPI consumed, temporal cascade + success-recovery routed downstream); record the §10 static survey as informational/non-certifying.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — §9.3–§9.11 capstone sections; file created by 0042ORDLIFCER-001)

## Out of Scope

- Re-running any per-gate / harness / mutation evidence (owned by `-002`…`-015`; this capstone consolidates and renders only).
- Any production/engine change or remediation (a non-pass routes to a later separately-numbered remediation/replacement spec; this document renders the verdict, it does not fix).
- The `docs/4-specs/SPEC_LEDGER.md` row and the `archive/specs/` move (deferred to spec acceptance/archival per `docs/archival-workflow.md` — a Step 6 cross-spec follow-up, not ticketed here).

## Acceptance Criteria

### Tests That Must Pass

1. The artifact contains the three §9.3 verdict tables (ORD-LIFE-01…12; ten pass conditions; seven fixture families), each row naming responsible layer, certifying evidence item IDs, negative controls, and mutation entries, with a result computed only from certifying evidence.
2. The §9.6 replay/provenance and §9.7 mutation packages are assembled and cross-reference `reports/0042_ord_life_cert_mutation_triage_register.md`; the §9.8 staged-abstraction and §9.10 observer-only EMERGE-OBS sections are present; `cargo test --workspace --locked` is green on the tested tree.
3. The §9.11 aggregate verdict is rendered strictly per the eight-condition rule — `ORD-LIFE-CERT passed` only if all hold, else `ORD-LIFE-CERT scoped remediation` naming every gap and routing remediation — limited to the exact tested commit and evidence package.

### Invariants

1. No certifying row passes from historical/pending/sampled-only/observer-only/static-only/snapshot-only/aggregate-score evidence; a substantive failed point or mutation floor forces `scoped remediation`.
2. Phase 4 stays blocked behind this verdict; predecessor (SPINE/EPI) citations remain scoped; the EMERGE-OBS member stays observer-only and non-gating.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; no production logic or test is added. Verification is the assembled verdict tables/packages plus a green workspace suite and the wording/doc guards exercised by ticket -013.`

### Commands

1. `cargo test --workspace --locked`
