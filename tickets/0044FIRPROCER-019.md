# 0044FIRPROCER-019: Acceptance capstone — gate/scenario/audit-point/temporal tables, packages, EMERGE-OBS, and FIRST-PROOF-CERT verdict

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — acceptance-only capstone; reconciles the prior tickets' recorded evidence into the verdict. Introduces no production logic and no new tests.
**Deps**: 0044FIRPROCER-002, 0044FIRPROCER-003, 0044FIRPROCER-004, 0044FIRPROCER-005, 0044FIRPROCER-006, 0044FIRPROCER-007, 0044FIRPROCER-008, 0044FIRPROCER-009, 0044FIRPROCER-010, 0044FIRPROCER-011, 0044FIRPROCER-012, 0044FIRPROCER-013, 0044FIRPROCER-014, 0044FIRPROCER-015, 0044FIRPROCER-016, 0044FIRPROCER-017, 0044FIRPROCER-018

## Problem

Spec §8, §11, §12, §16, and §17 require a single capstone that reconciles the recorded per-point evidence into the FIRST-PROOF-CERT verdict at one `U`. The acceptance artifact must carry four reconciled tables (FIRST-PROOF-01..17; the nine gates; the nine scenario families; the five temporal-cascade sources), each row citing evidence-item IDs and resolving only on certifying evidence; the §12.4 mandatory behavior witnesses; the §12.5 replay package and §12.6 command/mutation package; the §12.7 staged-abstraction declaration; the §12.8 observer-only `EMERGE-OBS` package; the §11 responsible-layer reconciliation; and the §16 completion checklist. The verdict (§12.9, §17) is `FIRST-PROOF-CERT passed` only if every condition holds at one `U`; otherwise the artifact names the failing point/layer and renders `FIRST-PROOF-CERT scoped remediation`. This ticket reconciles the prior tickets' sections, completes the §8 coverage matrices, and renders the aggregate verdict.

## Assumption Reassessment (2026-06-21)

1. The reconciled inputs exist as the per-point sections appended by `-002`..`-018` to `reports/0044_…_acceptance.md` (created `(new)` by `-001`), plus the mutation package (`-018`) and the consolidated temporal line (`-016`). The audited matrix/checklist surfaces (`acceptance_gates.rs`, `acceptance_artifact_wording.rs`, `emergence_ledger.rs`, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`) exist at `U` (confirmed present this session). The §8.1/§8.2 coverage matrices, §8.3 scenario-family plan, §8.4 adversarial inventory, and §8.5 temporal bundle are the reconciliation targets.
2. Spec §8 (completeness/coverage matrices), §11 (responsible-layer diagnostics), §12 (acceptance-artifact contract), §16 (completion checklist), and §17 (required outcome) govern this ticket; architecture `13` (review artifacts), `INV-111`, and `INV-098` bind it. The spec itself renders no verdict; the implementing session may render one only when every §12.9 condition holds at one `U`.
3. Cross-artifact shared boundary under audit: the aggregate tables and verdict sections of `reports/0044_…_acceptance.md`; this ticket reconciles the sibling-authored sections and appends the four reconciled tables, the §16 checklist, and the §17 verdict. It modifies no per-point section authored by `-002`..`-018`.
4. Motivating invariants/rules (spec §12.9/§17): `INV-098` (feature acceptance is harsh — only a fully caused/eventful/replay-safe/no-human/non-scripted/regression-tested result passes), `INV-111` (`EMERGE-OBS` is observer-only and non-certifying — present but no value makes a gate pass/fail). Restate before trusting the narrative: pending, historical, sampled-only, or observer-only rows cannot silently produce a pass.
5. This ticket audits/reads (does not modify) every enforcement surface the prior tickets exercised; it reconciles their evidence into verdict tables and introduces no new evidence, no production logic, and no test. The `EMERGE-OBS` artifact is labeled observer-only/retrospective/event-ancestry-backed/non-input; allowed scope wording says the result applies only to the exact unified commit and the FIRST-PROOF-CERT contract (no "latest main", "Phase 4 passed", or "second proof passed" claim).

## Architecture Check

1. A single acceptance-only capstone that reconciles the leaf set into four cross-checked tables plus the completion checklist is the only structure that catches an unfilled gate/family/source, a non-certifying evidence status, or an evidence-splice across baselines before a verdict is rendered; spec §12.9 requires every condition true at one `U`.
2. No backwards-compatibility aliasing/shims introduced; the capstone adds no production logic and exercises the evidence the prior tickets composed — it does not modify their sections or re-collect their witnesses.

## Verification Layers

1. `INV-098` harsh acceptance -> manual review + reconciliation (all FIRST-PROOF-01..17 have certifying evidence; all nine gates pass as one coherent artifact set; all nine scenario families have positive + required adversarial evidence; the five-source temporal bundle closes; workspace + every named command pass; mutation campaigns complete with no actionable floor; every failure names a responsible layer).
2. `INV-111` observer-only EMERGE-OBS -> codebase grep-proof + manual review (`EMERGE-OBS` is present, event-ancestry-backed, and observer-only; no value is used as a threshold, objective, fixture outcome, or substitute for an audit point).
3. Evidence honesty -> manual review (evidence-status and fingerprint-scope rules are honest; pending/historical/sampled/observer-only rows cannot produce a pass; no evidence splice or substantive deferral remains; scope wording is exact-commit only).

## What to Change

### 1. Reconcile the four coverage tables and §8 matrices

Complete the four reconciled tables required by §12.2: FIRST-PROOF-01..17; the nine gates; the nine scenario families; the five temporal-cascade sources. Each row cites evidence-item IDs from the sibling-authored sections; only evidence marked certifying under the live evidence-status rules determines a pass. Complete the §8.1/§8.2 audit-point-to-gate and audit-point-to-scenario matrices (every `P`/`I` cell has certifying evidence), the §8.3 scenario-family evidence plan, and the §8.4 required adversarial property inventory. Reconcile the §11 responsible-layer map so every recorded failure/negative names its canonical layer.

### 2. Assemble the §12 packages and render the §16/§17 verdict

Assemble the §12.4 mandatory behavior witnesses (all sixteen), the §12.5 replay package, the §12.6 command/mutation package, the §12.7 staged-abstraction declaration, and the §12.8 observer-only `EMERGE-OBS` package. Walk the §16 completion checklist. Render the §17 verdict: `FIRST-PROOF-CERT passed` only if every §12.9 condition holds at one `U`; otherwise name the failing point/responsible layer and render `FIRST-PROOF-CERT scoped remediation` (routing to a later separately-numbered remediation/replacement spec, not authored here). Use only exact-commit-scoped wording.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — §8 matrices, §12.2 four reconciled tables, §12.4–§12.8 packages, §16 checklist, §17 verdict; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation and authoring the remediation/replacement spec (spec §10.4/§12.9/§17: a failed floor yields `FIRST-PROOF-CERT scoped remediation` routed to a later separately-numbered spec; this capstone records the posture, it does not fix or pre-author).
- Collecting new evidence or modifying any per-point section authored by `-002`..`-018`; this ticket reconciles, it does not re-collect.
- Latest-`main` verification, Phase-4/second-proof claims, and the `docs/4-specs/SPEC_LEDGER.md` row + `archive/specs/` move (deferred to spec acceptance/archival per `docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_gates`, `--test acceptance_artifact_wording`, and `--test emergence_ledger` pass at the same `U` as every cited per-point section; the four reconciled tables resolve only on certifying evidence.
2. The §16 completion checklist is fully satisfied: FIRST-PROOF-01..17, all nine gates, all nine scenario families, and the five-source temporal bundle each have certifying evidence; mutation campaigns complete with no actionable floor; `EMERGE-OBS` is present and observer-only.
3. The rendered verdict matches the evidence: `FIRST-PROOF-CERT passed` only if every §12.9 condition holds at one `U`, else `FIRST-PROOF-CERT scoped remediation` naming the failing point/responsible layer; scope wording is exact-commit only.

### Invariants

1. No pending/historical/sampled-only/observer-only row produces a pass; no evidence splice across baselines; every failure names a responsible layer.
2. `EMERGE-OBS` is observer-only and non-certifying; the verdict makes no latest-main / Phase-4 / second-proof claim.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; verification is reconciliation of the prior tickets' recorded evidence plus the existing acceptance/ledger suites below. No production logic or new test is introduced.`

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_gates`
2. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
3. `cargo test --locked -p tracewake-core --test emergence_ledger`
4. `cargo test --workspace --locked`
