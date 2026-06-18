# 0038SPICEREVE-008: SPINE-07 evidence — TUI, embodied view models, transcript surface, and debug split

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-07 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-07 (spec §5) requires proof that the TUI is a client and proof surface, not a simulation authority: it renders embodied view models, accepts semantic input, submits proposals through the shared action pipeline, and shows quarantined debug overlays only under a debug capability — and never reads hidden truth into embodied views, mutates state, bypasses validation, or makes debug-only facts diegetic. This ticket gathers the embodied-vs-debug transcript snapshots, the semantic-action submission path witness, and the debug-capability quarantine evidence, and writes the SPINE-07 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-tui/src/{app,input,render,debug_panels,transcript,launch,run}.rs`, `crates/tracewake-core/src/{view_models,debug_capability,debug_reports}.rs`, `actions/{pipeline,proposal}.rs`; the tests `crates/tracewake-tui/tests/{tui_seam_conformance,adversarial_gates,command_loop_session,transcript_snapshot,tui_acceptance}.rs` and `crates/tracewake-core/tests/{hidden_truth_gates,negative_fixture_runner}.rs`.
2. Spec §5 SPINE-07 names the positive corpus (`view_model_local_actions_001`, `embodied_view_omits_raw_assignment_without_context_001`, `embodied_menu_lags_truth_change_without_perception_001`, `possession_parity_001`, `debug_attach_001`) and the adversarial corpus (`debug_omniscience_excluded_001`, plus TUI hidden-truth render / direct-mutation / direct-action-def attempts); all named fixture modules exist under `crates/tracewake-content/src/fixtures/`.
3. Shared boundary under audit: the embodied-view-model / semantic-proposal / debug-overlay channel split. The witness must record an embodied transcript snapshot and a debug transcript/overlay snapshot for the same state (proving distinct channels), the semantic-action submission path (input → `ProposalSource::TuiSemanticAction` → `run_pipeline` → appended event → view refresh from event frontier), and the debug-capability construction/quarantine evidence.
4. `INV-065` (the TUI is a primary product interface), `INV-067` (embodied mode shows actor-known reality), `INV-068` (debug mode is visibly non-diegetic), `INV-069` (the TUI must not implement simulation rules), and `INV-070` (why-not explanations) motivate this ticket: embodied views derive from holder-known contexts, debug surfaces are segregated, and TUI commands become semantic proposals through the shared pipeline.
5. This ticket audits the TUI render/input and debug-quarantine surfaces as an **evidence consumer**: it runs the TUI seam/adversarial/command-loop/transcript/acceptance suites plus `hidden_truth_gates` and `negative_fixture_runner`, then records witnesses; it weakens no firewall. Adversarial cases (debug omniscience into embodied views, TUI rendering hidden truth without holder-known provenance, TUI calling action defs or mutating state directly, debug-only tokens in an ordinary embodied transcript) must fail or omit; any leak is recorded `fail` with responsible layer and routed to remediation.

## Architecture Check

1. Auditing the seam via paired embodied/debug snapshots of the same state + the semantic-action path witness + debug-capability quarantine negatives proves the client-not-authority and non-diegetic-debug semantics, not just that the TUI renders.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-067`/`INV-069` embodied actor-known views, TUI implements no rules -> replay/golden-fixture check + manual review: embodied view-model filtering holds; TUI input becomes `ProposalSource::TuiSemanticAction` through `run_pipeline` (`command_loop_session`, `view_model_local_actions_001`).
2. `INV-068` debug non-diegetic -> manual review + codebase grep-proof: paired embodied/debug snapshots differ by channel; `debug_omniscience_excluded_001` keeps omniscience out of embodied views (`tui_seam_conformance`, `transcript_snapshot`).
3. `INV-069` no state mutation / no validator bypass from the TUI -> codebase grep-proof + negative-fixture: TUI direct-mutation / action-def-call attempts fail by structure or compile-fail (`adversarial_gates`, `negative_fixture_runner`).

## What to Change

### 1. Capture the SPINE-07 embodied/debug channel witnesses

Record an embodied transcript snapshot for a passing scenario and a debug transcript/overlay snapshot for the same state (proving different channels); record the semantic-action submission path (input → proposal → source context → pipeline → appended event → view refresh from event frontier); record debug-capability construction and quarantine evidence (the capability is not constructible by external crates or content).

### 2. Record adversarial TUI-leak and bypass rejections

For the adversarial corpus (debug omniscience into embodied views, hidden-truth render without provenance, TUI direct mutation / action-def call, debug-only tokens in ordinary embodied transcript), record the fail/omit witness and the named responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- Projection-level non-truth-writer quarantine (owned by `-004`, SPINE-03) and the no-direct-dispatch mutation closure (owned by `-009`, SPINE-08) — this ticket records the TUI channel split and one TUI direct-mutation negative.
- Remediation of any TUI leak or bypass found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` and `--test transcript_snapshot` — embodied/debug channel split and snapshots.
2. `cargo test --locked -p tracewake-tui --test command_loop_session` — TUI input becomes a semantic proposal through `run_pipeline`.
3. `cargo test --locked -p tracewake-tui --test adversarial_gates` and `--test tui_acceptance` — TUI leak/mutation/bypass attempts fail closed; embodied playability holds.
4. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test negative_fixture_runner` — hidden-truth render and debug-construction guards hold.

### Invariants

1. Embodied views derive from holder-known contexts; debug content stays out of ordinary embodied transcripts (`INV-067`/`INV-068`).
2. TUI commands become semantic proposals through the shared pipeline and never mutate state or bypass validation; the debug capability is not externally constructible (`INV-069`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing TUI seam/adversarial/command-loop/transcript/acceptance tests plus hidden-truth/negative guards, and the captured paired snapshots + semantic-action path are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-tui --test tui_seam_conformance && cargo test --locked -p tracewake-tui --test transcript_snapshot`
2. `cargo test --locked -p tracewake-tui --test command_loop_session && cargo test --locked -p tracewake-tui --test adversarial_gates`
3. `cargo test --locked -p tracewake-tui --test tui_acceptance && cargo test --locked -p tracewake-core --test hidden_truth_gates`
