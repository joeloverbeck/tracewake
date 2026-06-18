# 0038SPICEREVE-004: SPINE-03 evidence — projection rebuild and non-truth-writer quarantine

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-03 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-03 (spec §5) requires proof that projections read event-derived state and produce review/view/notebook/replay/debug material without becoming authoritative truth writers: rebuilding projections from the event log is deterministic, holder-known context construction is sealed with source and frontier, and projection output never injects actor-visible knowledge, alters state, schedules actions, or masks provenance gaps. This ticket gathers the projection fingerprints (pre/post replay), the holder-known context/provenance records, and the leak/illegal-write rejection witnesses, and writes the SPINE-03 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/projections.rs`, `epistemics/projection.rs`, `epistemics/knowledge_context.rs`, `view_models.rs`, `events/apply.rs`, `replay/report.rs`; the tests `crates/tracewake-core/tests/{spine_conformance,hidden_truth_gates,anti_regression_guards}.rs` and `crates/tracewake-tui/tests/tui_seam_conformance.rs`.
2. Spec §5 SPINE-03 names the positive corpus (`view_filtering_001`, `view_model_local_actions_001`, `possession_parity_001`, `no_human_observation_facts_cite_log_events_001`, `workplace_assignment_provenance_001`, `stale_workplace_notice_superseded_by_newer_001`) and the adversarial failures; all fixture modules exist under `crates/tracewake-content/src/fixtures/`. The artifact section comes from `-001`'s scaffold and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
3. Shared boundary under audit: the holder-known-context → scoped-projection contract. The witness must record holder-known context IDs, hashes, event frontiers, source scopes, provenance entries, and forbidden-truth audit results for representative views, and prove rebuilt projections after replay match the live-run fingerprints.
4. `INV-006` (possession transfers no world knowledge), `INV-024` (no telepathy), `INV-059` (leads/tasks are projections, not ground truth), `INV-067` (embodied mode shows actor-known reality), and `INV-107` (debug omniscience is quarantined) motivate this ticket: projections are rebuildable views and diagnostics, never simulation truth, and must not leak hidden facts to a viewer the actor-knowledge filter forbids.
5. This ticket audits the view-model projection and actor-knowledge filtering surfaces as an **evidence consumer**: it runs `hidden_truth_gates`, `spine_conformance`, `anti_regression_guards`, `tui_seam_conformance`, and the content golden runner, then records witnesses; it weakens no firewall. Adversarial cases (projection writes state/appends events, gives an actor facts absent from its holder-known context, omits provenance for actor-visible facts, silently picks a fallback when a provenance gap should reject/wait/diagnose, or routes debug content into an embodied view) must be rejected with the responsible layer named; any leak is recorded `fail` and routed to remediation, not fixed here.

## Architecture Check

1. Auditing the seam via projection-fingerprint equality (live vs rebuilt) plus holder-known provenance capture plus leak-rejection negatives proves the non-truth-writer and no-telepathy semantics, not just stable view bytes.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-024`/`INV-006` no telepathy / possession transfers no knowledge -> manual review + replay/golden check: holder-known views render without raw-truth leakage; possession parity holds (`hidden_truth_gates`, `possession_parity_001`).
2. `INV-067`/`INV-059` actor-known views, leads-as-projections -> replay/golden-fixture check: rebuilt projection fingerprints after replay equal the live run.
3. `INV-107` debug quarantine -> manual review + codebase grep-proof: debug projection content does not enter an embodied view (`tui_seam_conformance`); one negative leak fixture is rejected with the responsible layer named.

## What to Change

### 1. Capture the SPINE-03 projection and holder-known witnesses

Record projection fingerprints per positive fixture, before and after replay (proving rebuild determinism); record holder-known context IDs, hashes, event frontiers, source scopes, provenance entries, and forbidden-truth audit results for representative embodied/private views. Declare fingerprint scope per row.

### 2. Record non-truth-writer and no-leak rejection evidence

For the adversarial families (illegal state write / event append from a projection, hidden-fact injection, missing provenance for actor-visible facts, silent fallback over a provenance gap, debug-into-embodied leakage), record the rejection witness and the named responsible layer for at least one negative fixture where a projection leak is rejected.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The TUI embodied/debug-split seam (owned by `-008`, SPINE-07) — this ticket records projection-level quarantine, not the TUI render/transcript channel split.
- Remediation of any projection leak or illegal write found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — no hidden-truth leakage into holder-known projections.
2. `cargo test --locked -p tracewake-core --test spine_conformance` and `--test anti_regression_guards` — projection seams map to named evidence; guards fail closed.
3. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` — debug content excluded from embodied views.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — positive projection/provenance fixtures render expected views.

### Invariants

1. Rebuilt projection fingerprints after replay equal the live-run fingerprints (`INV-018` rebuild determinism applied to projections).
2. No projection grants an actor a fact absent from its holder-known context, and no actor-visible fact lacks provenance (`INV-024`/`INV-006`/`INV-067`); debug content stays out of embodied views (`INV-107`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing projection/holder-known/quarantine tests and the captured fingerprints + provenance records are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test spine_conformance && cargo test --locked -p tracewake-tui --test tui_seam_conformance`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
