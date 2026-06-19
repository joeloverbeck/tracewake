# 0040EPICERHOL-010: EPI-09 — embodied projection source, notebook, action availability, why-not, and stale-snapshot behavior

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-09 (§5) requires certifying that the embodied view model is built from a sealed holder-known context and a bounded embodied projection/snapshot — never from an unrestricted live `PhysicalState` handle — carries context ID/hash/frontier and actor-safe source summaries, renders only permitted entities/notebook/action-availability/why-not, preserves intentional staleness until an observation updates the actor, and keeps validation/debug detail outside the embodied channel. This ticket collects the EPI-09 witnesses and writes the EPI-09 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-09 seam files verified present at `ba9fe1c` (2026-06-19): `projections.rs` (`EmbodiedProjectionSource`, `EmbodiedTruthSnapshot`), `view_models.rs` (`EmbodiedViewModel`, `NotebookView`, `ActionAvailability`, `WhyNotView`, `ActionAvailabilityProvenanceKind`), `controller.rs`, `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, `actions/report.rs`, `crates/tracewake-tui/src/app.rs`, `render.rs`, `transcript.rs`. Negative fixture `external_crate_cannot_build_debug_projection_view_without_core_debug_api` and positive fixtures (`embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unknown_sleep_affordance_001`, `embodied_view_omits_unobserved_food_at_open_place_001`, `embodied_exits_require_perceived_or_known_route_001`, `embodied_menu_lags_truth_change_without_perception_001`, `embodied_workplace_availability_reflects_belief_not_truth_001`, `view_filtering_001`, `view_model_local_actions_001`) all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-09 section only. Section wording follows spec §5 EPI-09 and the `0003` evidence fields.
3. Shared boundary under audit: the embodied-model construction seam — `EmbodiedProjectionSource::from_sealed_context` in `projections.rs` feeding `EmbodiedViewModel`/`NotebookView`/`ActionAvailability`/`WhyNotView` in `view_models.rs`, rendered via the TUI. The evidence must prove no live `PhysicalState` handle escapes into the embodied model and that replay-rebuilt inputs reproduce the same model.
4. `INV-067` (embodied mode shows actor-known reality) and `INV-070` (why-not explanations are mandatory) motivate this audit point, with `INV-008` (UI assistance is not authority), `INV-023`…`INV-031`, `INV-065`…`INV-069`, `INV-099`…`INV-108`, `INV-112`. Restated: a truth change alone does not refresh the menu; a later admissible observation may.
5. This ticket audits the no-leak / actor-knowledge-filter and projection/replay determinism surfaces as an **evidence consumer**: it proves hidden item / closed-container contents / hidden route / raw workplace assignment / unknown sleep affordance / other-actor private belief / debug report / validator-only fact never appear in embodied structs, serialization, rendering, transcript, action labels, why-not text, ordering, counts, or timing (no leakage), and that a stale view-model/context pair or mismatched tuple is not accepted as a fresh proposal source (determinism/freshness). `ActionAvailability`/`WhyNotView` preserve the actor-safe vs debug-only split. It modifies no enforcement. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-09 evidence ticket writing only its artifact section is the accepted certification-audit shape; it adds the embodied-omission, stale-snapshot, and actor-safe-vs-debug-split witnesses without re-running the whole vocabulary.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2).

## Verification Layers

1. `INV-067` actor-known embodied view -> replay/golden-fixture check: `embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unknown_sleep_affordance_001`, `embodied_view_omits_unobserved_food_at_open_place_001`, `embodied_exits_require_perceived_or_known_route_001` omit unknown truth; `embodied_workplace_availability_reflects_belief_not_truth_001`, `view_filtering_001`, `view_model_local_actions_001` prove actor-safe availability/local filtering from the sealed context (`embodied_flow`, `acceptance_gates`).
2. `INV-028` intentional staleness -> replay/golden-fixture check: `embodied_menu_lags_truth_change_without_perception_001` proves a truth change alone does not refresh the menu; a later admissible observation may.
3. `INV-070`/`INV-008` why-not split, no-leak rendering -> manual review + replay/golden-fixture check: notebook entries show source-backed observations/beliefs/contradictions/freshness and actor-appropriate "how this could be wrong / what to check next" without hidden validation facts; hidden fields/tokens absent from structs/serialization/transcript/labels/why-not/ordering/counts/timing (`transcript_snapshot`, `tui_seam_conformance`, `adversarial_gates`).
4. live-handle / debug-API quarantine -> negative fixture + manual review: no live `PhysicalState` handle escapes into the embodied model; creating/reading a privileged debug projection view outside core's authorized API fails via `external_crate_cannot_build_debug_projection_view_without_core_debug_api`; toggling debug does not alter embodied semantic serialization/render (`negative_fixture_runner`).

## What to Change

### 1. Collect EPI-09 witnesses

Run the EPI-09 commands and retain the sealed context, embodied projection source/snapshot, semantic view-model serialization, actor-safe source/provenance table, notebook entries, action/why-not table, rendered transcript, and paired hidden-truth/debug comparisons. Prove no live state handle escapes into the embodied model and that replay-rebuilt inputs reproduce the same model. Prove a stale/mismatched context tuple is rejected as a fresh proposal source and that actor-visible error text stays truthful at the actor's epistemic level (an action may fail without naming an unobserved hidden cause).

### 2. Write the EPI-09 section of the acceptance artifact

Populate the EPI-09 section with the §9.2 ledger fields per witness (positive omission/availability, stale-snapshot, why-not split, live-handle/debug-API negatives, replay), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-09 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- The §9.4 verdict table and aggregate verdict (owned by `-015`); debug-channel capability isolation depth (owned by EPI-10 `-011`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-tui --test embodied_flow` and `--test transcript_snapshot` — embodied omission + stale-snapshot + transcript witnesses.
2. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` and `--test adversarial_gates` — no hidden field/token in structs/serialization/render/labels/why-not.
3. `cargo test --locked -p tracewake-core --test acceptance_gates` and `--test negative_fixture_runner` — actor-safe availability + debug-projection-view compile-fail negative.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — fixture witnesses + replay model reproduction.

### Invariants

1. The embodied model is built from the sealed context + bounded snapshot, carries context ID/hash/frontier, and no live `PhysicalState` handle escapes into it; replay reproduces the same model (`INV-067`/`INV-069`).
2. Hidden truth never appears in any embodied surface (struct/serialization/render/label/why-not/ordering/count/timing); a truth change alone does not refresh the menu; a stale/mismatched tuple is no fresh proposal source (`INV-028`/`INV-008`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-09 existing gates/fixtures and records the embodied serialization, actor-safe source/why-not table, rendered transcript, and hidden-field-absence scan as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-tui --test embodied_flow`
2. `cargo test --locked -p tracewake-tui --test transcript_snapshot && cargo test --locked -p tracewake-tui --test tui_seam_conformance`
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner` (debug-projection-view authorized-API boundary)
