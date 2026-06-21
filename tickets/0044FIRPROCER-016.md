# 0044FIRPROCER-016: FIRST-PROOF-16 — temporal diagnostics and consolidated five-source acceptance line

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001, 0044FIRPROCER-012, 0044FIRPROCER-013, 0044FIRPROCER-014, 0044FIRPROCER-015

## Problem

Spec §7 audit point FIRST-PROOF-16 certifies the **fifth routed temporal source** (execution `10`) and closes the complete bundle at `U`: diagnostics must distinguish validator time from holder-known premises, name the responsible layer, preserve replay ancestry, and reconcile the outputs of FIRST-PROOF-12..15 into one acceptance line. Each temporal failure/success must report canonical responsible layer, component, actor/holder, source IDs, expected/actual, accepted/rejected stage, and first divergence where applicable; diagnostics must state whether the time fact was validator-only or holder-known; the acceptance artifact must have one row per routed source (`04`, `06`, `07`, `09`, `10`); and staged abstractions/pending items cannot be counted as pass. This ticket records the FIRST-PROOF-16 temporal-diagnostic witnesses and the §8.5 consolidated five-source acceptance line (linking, not merely collecting, the five evidence items) into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/actions/report.rs`, `crates/tracewake-core/src/agent/trace.rs`, `crates/tracewake-core/src/{debug_reports.rs}`, `crates/tracewake-core/src/replay/report.rs`, the suites `acceptance_artifact_wording.rs`/`emergence_ledger.rs`, and the acceptance template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (all confirmed present this session). The §8.5 bundle inputs are produced by `-012`..`-015`.
2. Spec §7 FIRST-PROOF-16 and §8.5 (consolidated temporal evidence bundle) govern this ticket; execution `10` evidence honesty and temporal diagnostics, execution `03` temporal cascade, architecture `13`, `INV-111`, and `INV-112` bind it. The five routed sources form one causal chain whose links must be mutually consistent at `U`.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-16 section and the consolidated §8.5 acceptance line of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`). This ticket depends on `-012`..`-015` because it links their recorded source/routine/embodied/fixture evidence into one row; it appends only its own section.
4. Motivating invariants (spec §7 FIRST-PROOF-16): `INV-112` (validator time distinguished from holder-known premises), `INV-111` (no temporal metric or `EMERGE-OBS` value becomes a scheduler objective or threshold). Restate before trusting the narrative: one acceptance row links source → routine behavior → embodied rendering → fixture → diagnostic/replay without inventing temporal vocabulary.
5. This ticket audits/reads (does not modify) the diagnostic/responsible-layer, replay, view-model, and debug-quarantine enforcement surfaces. It runs fixtures and records witnesses only; a debug timestamp or observer metric cannot become an actor-visible reason, and no temporal metric or `EMERGE-OBS` value may become a scheduler objective/threshold. No nondeterminism and no new temporal vocabulary are introduced.

## Architecture Check

1. Reconciling source event → holder-known temporal premise → routine behavior → embodied output → fixture result → diagnostic → replay record in one trace is the only check that proves the five sources cohere rather than merely coexist; spec §8.5 fails the bundle if evidence items refer to different baselines/semantics or only coexist without a source-to-behavior-to-view-to-fixture-to-diagnostic trace.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let a generic "stuck"/"time mismatch" without responsible layer, or an unlinked bundle, pass.

## Verification Layers

1. `INV-112` typed temporal diagnostics -> manual review + replay/golden-fixture check (each temporal failure/success reports canonical responsible layer, component, actor/holder, source IDs, expected/actual, accepted/rejected stage, and first divergence where applicable; diagnostics state validator-only vs holder-known; live and replay diagnostics are deterministic).
2. `INV-112` bundle completeness -> manual review (the acceptance artifact has one row for each routed source `04`/`06`/`07`/`09`/`10`; the consolidated row links, not merely collects, the five source-specific evidence items; staged abstractions/pending items cannot be counted as pass).
3. `INV-111` observer-only non-targeting -> codebase grep-proof + manual review (a debug timestamp or observer metric cannot become an actor-visible reason; no temporal metric or `EMERGE-OBS` value becomes a scheduler objective/threshold).

## What to Change

### 1. Record temporal-diagnostic witnesses

Run `acceptance_artifact_wording`, `emergence_ledger`, and the §7 positive path. Record that each temporal failure/success reports canonical responsible layer, component, actor/holder, source IDs, expected/actual, accepted/rejected stage, and first divergence where applicable; that diagnostics state whether the time fact was validator-only or holder-known; that live and replay diagnostics are deterministic; and that staged abstractions/pending items are not counted as pass. Record the adversarial cases: a generic "stuck"/"time mismatch" without responsible layer is non-certifying; a debug timestamp or observer metric cannot become an actor-visible reason; a replay mismatch without first-divergence identity is non-certifying; omission of any routed source fails bundle completeness.

### 2. Record the consolidated five-source acceptance line

Build the §8.5 acceptance line with one row for each routed source `04`/`06`/`07`/`09`/`10`, drawing the evidence items recorded by `-012` (temporal-firewall paired run, source-event ancestry, fail-closed negative), `-013` (routine trace, premise source, ordinary outcome, accounting, replay), `-014` (view-model field/source citation, possession/debug relation, transcript/replay fingerprint), `-015` (fixture IDs/fingerprints, positive/adversarial trigger, event/projection/replay records), and this point's diagnostic evidence. Reconcile source event, holder-known temporal premise, routine behavior, embodied output, fixture result, diagnostic, and replay record in one trace; confirm the premise used by cognition in the capstone is the same premise rendered, exercised, diagnosed, and replayed.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-16 section + §8.5 consolidated temporal acceptance line; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any temporal-diagnostic defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Inventing any temporal unit/threshold/source category/gate code/status enum (spec §5.4); the individual temporal sources (`-012`..`-015`) and the §10 mutation perimeter (`-018`).
- The aggregate gate/scenario/audit-point verdict tables (`-019`); this ticket closes only the temporal bundle row.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` and `--test acceptance_gates` pass; each temporal diagnostic names the canonical responsible layer and states validator-only vs holder-known.
2. `cargo test --locked -p tracewake-core --test emergence_ledger`, `--test event_schema_replay_gates`, and `--test no_human_capstone` pass; live/replay diagnostics are deterministic with first-divergence identity and no `EMERGE-OBS`/temporal value is used as a threshold/objective.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-tui --test transcript_snapshot`, and `--test tui_acceptance` pass; the §8.5 acceptance line links source → routine → embodied → fixture → diagnostic/replay for all five routed sources.

### Invariants

1. No missing routed source, no ambiguous layer, no diagnostic/replay mismatch, no actor-visible debug reason, no thresholded observer evidence, no unlinked bundle.
2. The consolidated row links (not merely collects) the five evidence items at one `U`; no new temporal vocabulary is invented.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test emergence_ledger`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-core --test no_human_capstone`
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
7. `cargo test --locked -p tracewake-tui --test transcript_snapshot`
8. `cargo test --locked -p tracewake-tui --test tui_acceptance`
