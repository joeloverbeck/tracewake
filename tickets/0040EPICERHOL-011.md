# 0040EPICERHOL-011: EPI-10 — debug capability isolation, report/view separation, TUI quarantine, and no feedback path

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-10 (§5) requires certifying that debug access is a privileged, non-diegetic observer channel: core alone may mint the unforgeable debug capability; debug contexts/reports/views require that capability or another equally strict core-owned authority path; debug surfaces carry a visible non-diegetic marker and stay structurally separate from embodied models; and rendering or using debug data creates no simulation facts, proposals, events, actor feedback, beliefs, observations, or projection records. This ticket collects the EPI-10 witnesses and writes the EPI-10 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-10 seam files verified present at `ba9fe1c` (2026-06-19): `debug_capability.rs` (`DebugCapability::mint()` is `pub(crate) const`), `debug_reports.rs`, `view_models.rs` (`DebugViewModel`, `Debug*View` families), `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, `agent/trace.rs`, `controller.rs`, `crates/tracewake-tui/src/debug_panels.rs` (`DEBUG_MARKER = "DEBUG NON-DIEGETIC"`), `render.rs` (`DEBUG_TOKENS`), `app.rs`. Negative fixtures (`external_crate_cannot_build_debug_knowledge_context`, `…_build_debug_projection_view_without_core_debug_api`, `…_construct_debug_report`) and positive fixtures (`debug_attach_001`, `debug_omniscience_excluded_001`, `prose_born_fact_rejected_001`) all verified present; `adversarial_gates`, `tui_seam_conformance` test binaries verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-10 section only. Section wording follows spec §5 EPI-10 and the `0003` evidence fields.
3. Shared boundary under audit: the debug-capability isolation seam — `DebugCapability` minting restricted to core, the capability-sealed debug report/view types, the `debug_only()` contract, and the `DEBUG NON-DIEGETIC` marker / `DEBUG_TOKENS` embodied-render suppression in the TUI. The evidence must prove a UI color/label alone is not quarantine proof — only the paired no-feedback/no-state-change proof closes the claim.
4. `INV-107` (debug omniscience is quarantined) and `INV-068` (debug mode is visibly non-diegetic) motivate this audit point, with `INV-031` (human/debug notes are non-diegetic), `INV-065`…`INV-070`, `INV-099`/`INV-100`, `INV-108`. Restated: debug output must not create actor/institution knowledge, records, rumors, notices, speech acts, or future plans unless an explicit debug/test event is marked non-ordinary.
5. This ticket audits the debug-quarantine / no-leak surfaces as an **evidence consumer**: it proves an external crate cannot mint/construct the capability/context/report/projection-map (unforgeability), that a distinctive injected debug token appears only in authorized debug output and nowhere in embodied artifacts/feedback/event payloads/beliefs/observations/context hash (no leakage), and that enabling/disabling/rendering debug leaves authoritative state checksum, event log, epistemic projection checksum, actor-known context, selected proposal, and embodied output equal (non-interference). It modifies no enforcement. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-10 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the capability-unforgeability, debug-token-absence, and no-feedback/no-state-change witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2); the dependency graph must preserve core-owned capability/report types with no reverse dependency or TUI-owned minting path.

## Verification Layers

1. `INV-107` capability unforgeability -> negative fixture: the `debug_capability.rs` compile-fail doctests plus `external_crate_cannot_build_debug_knowledge_context`/`…_build_debug_projection_view_without_core_debug_api`/`…_construct_debug_report` prove an external crate cannot mint/construct the capability/context/report/projection map (`negative_fixture_runner`, cross-ref `-012`).
2. `INV-068`/`INV-031` non-diegetic marker + token absence -> manual review + replay/golden-fixture check: authorized debug construction returns true from `debug_only()` and renders `DEBUG NON-DIEGETIC`; an injected distinctive debug token appears in authorized debug output and nowhere in embodied structs/serialization/transcript/labels/why-not/feedback/event payloads/beliefs/observations/context hash (`adversarial_gates`, `tui_seam_conformance`).
3. debug non-interference -> replay/golden-fixture check: enabling/disabling/opening/closing/rendering debug panels around the same run leaves authoritative state checksum, event log, epistemic projection checksum, actor-known context, selected proposal, and embodied output equal; replay debug-disabled and debug-enabled inputs independently yield the same authoritative/epistemic result (`hidden_truth_gates`, `event_schema_replay_gates`).
4. `INV-008`/`INV-100` no debug feedback path -> manual review + negative fixture: pasting/routing debug prose into a command/LLM/content surface creates no typed fact (`prose_born_fact_rejected_001`); a debug validation reason does not become the next actor-safe reason absent an independent modeled event.

## What to Change

### 1. Collect EPI-10 witnesses

Run the EPI-10 commands and package capability-construction boundary evidence, compile-fail transcripts, debug report/view identities, the non-diegetic marker, distinct-channel render snapshots, a debug-token-absence scan over embodied artifacts, before/after state/event/projection/context checksums, proposal equality, and dependency-graph/source-guard evidence. Replay the debug-disabled and debug-enabled event inputs independently and prove each rebuild yields the same authoritative/epistemic result and actor-visible relation. `debug_attach_001` and `debug_omniscience_excluded_001` show an operator-only diagnostic view while embodied cognition stays unchanged.

### 2. Write the EPI-10 section of the acceptance artifact

Populate the EPI-10 section with the §9.2 ledger fields per witness (capability negatives, marker/token-absence, non-interference, no-feedback-path), each row carrying exactly one §9.2 status. A UI color/label alone is explicitly not quarantine proof.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-10 section only)

## Out of Scope

- Other audit points (EPI-10 cross-references the §6.1 compile-fail corpus owned by `-012` for the debug capability/report/context/projection negatives).
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — capability/context/report compile-fail fixtures remain negative.
2. `cargo test --locked -p tracewake-tui --test adversarial_gates` and `--test tui_seam_conformance` — non-diegetic marker present only in debug channel; injected debug token absent from embodied artifacts.
3. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test event_schema_replay_gates` — debug enable/disable leaves state/event/projection/context/proposal/embodied equal; independent replay agreement.
4. `cargo test --locked -p tracewake-tui --test transcript_snapshot` and `--test tui_acceptance`, plus `cargo test --locked -p tracewake-content --test golden_fixtures_run` — distinct-channel render + no-feedback-path witnesses.

### Invariants

1. Only core mints the unforgeable `DebugCapability`; an external crate cannot mint/construct the capability/context/report/projection map; debug surfaces carry `DEBUG NON-DIEGETIC` and stay structurally separate (`INV-107`/`INV-068`).
2. Enabling/rendering debug changes no simulation event, state checksum, projection checksum, actor-known context, proposal, or embodied output; an injected debug token never reaches an embodied/feedback/event/belief/observation surface; debug prose creates no typed fact (`INV-107`/`INV-100`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-10 existing gates/fixtures and records the capability-boundary transcripts, debug-token-absence scan, and before/after non-interference checksums as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-tui --test adversarial_gates`
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner && cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` (debug-token embodied-suppression boundary)
