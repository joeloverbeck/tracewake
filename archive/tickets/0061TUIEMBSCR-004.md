# 0061TUIEMBSCR-004: Field-disposition conformance guard

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds a conformance test to `crates/tracewake-tui/tests/tui_seam_conformance.rs`.
**Deps**: 0061TUIEMBSCR-001

## Problem

The overhaul's central promise is that every `EmbodiedViewModel` field has an explicit pane
disposition on `EmbodiedScreenModel` (rendered or explicitly suppressed), so later field additions
cannot be laundered through a wildcard/default arm (Spec 0061 §4.4). This ticket adds a new,
distinctly-named conformance test — `embodied_screen_model_field_disposition` — to the **existing**
`crates/tracewake-tui/tests/tui_seam_conformance.rs`, registered in its `TUI_SEAM_EVIDENCE` table
under a new requirement id. It complements — does not replace or collide with — the file's existing
guards `render_embodied_view_uses_sealed_view_model_accessors` (proves `render_embodied_view` names
every field with no rest/wildcard omission) and `closed_presentation_enum_matches_are_exhaustive_without_wildcards`.

## Assumption Reassessment (2026-07-01)

1. The conformance seam already exists: `crates/tracewake-tui/tests/tui_seam_conformance.rs` carries
   the `TUI_SEAM_EVIDENCE` registry (`:26`), `render_embodied_view_uses_sealed_view_model_accessors`
   (`:148`), and `closed_presentation_enum_matches_are_exhaustive_without_wildcards` (`:191`). This
   ticket ADDS a new test fn + a new `TUI_SEAM_EVIDENCE` row; it does not create a second
   `tui_seam_conformance` seam and does not modify the existing guards.
2. Spec 0061 §4.4 requires the guard to prove each existing `EmbodiedViewModel` field — public and
   crate-private — is assigned to a named `EmbodiedScreenModel` pane; §6 names the test
   `embodied_screen_model_field_disposition`. The `EmbodiedScreenModel` panes are introduced by 001.
3. Shared boundary under audit: the closed field→pane disposition contract between
   `tracewake_core::view_models::EmbodiedViewModel` (`crates/tracewake-core/src/view_models.rs:19`)
   and 001's `EmbodiedScreenModel`. The test is source/compile-guard style (matching the file's
   existing `StaticSourceGuard` evidence kind), guarding against wildcard/default laundering.
4. Invariant motivating this ticket: **INV-095 — TUI/view-model tests are acceptance tests** (the
   disposition guard is an acceptance artifact), reinforcing **INV-067/INV-069** (the screen model
   is actor-filtered presentation with no simulation rules).
5. Fail-closed / actor-knowledge surface: the enforcement surface is the closed-disposition guard
   itself — a static/compile guard that fails closed when any `EmbodiedViewModel` field lacks a
   named pane disposition or is reached through a wildcard/default arm. This strengthens
   epistemic-leakage prevention (a new field cannot be silently surfaced or silently dropped) and
   introduces no replay nondeterminism (it is a source guard, not a runtime path).

## Architecture Check

1. Adding the guard to the existing `tui_seam_conformance.rs` and registering it in
   `TUI_SEAM_EVIDENCE` reuses the repo's established conformance-evidence mechanism (requirement id
   → named test) rather than inventing a parallel seam — one place lists every TUI-seam acceptance
   guarantee. A closed field-enumeration guard (no wildcard/rest arm) is what forces future field
   additions to declare a disposition, which a runtime render check could not guarantee.
2. No backwards-compatibility aliasing/shims: the existing guards are preserved unchanged; this is
   an additive test + registry row.

## Verification Layers

1. INV-095 (TUI tests are acceptance tests) -> the new `TUI_SEAM_EVIDENCE` row maps its requirement
   id to `embodied_screen_model_field_disposition`, and `tui_seam_conformance_maps_tui_spine_requirements_to_named_tests`
   proves the row resolves to a real test fn.
2. Closed disposition (Spec 0061 §4.4) -> codebase grep-proof: the guard enumerates every
   `EmbodiedViewModel` field and asserts a named `EmbodiedScreenModel` pane disposition with no
   wildcard/default arm.
3. Cross-artifact (view-model ↔ screen-model) -> the test reads both
   `../../tracewake-core/src/view_models.rs` and the 001 screen-model source, mapping each field to
   its pane; no truth surface is involved.

## What to Change

### 1. Disposition conformance test

Add `embodied_screen_model_field_disposition` to `crates/tracewake-tui/tests/tui_seam_conformance.rs`:
enumerate every `EmbodiedViewModel` field and assert each maps to a named `EmbodiedScreenModel` pane
(rendered or explicitly suppressed), with a source/compile guard against a wildcard/default arm in
the builder.

### 2. Evidence registration

Add a `TUI_SEAM_EVIDENCE` row (new requirement id, `tui/view-model` layer, `StaticSourceGuard`)
whose `test_name` is `embodied_screen_model_field_disposition` and whose `acceptance_condition`
states closed field→pane disposition.

## Files to Touch

- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)

## Out of Scope

- The `EmbodiedScreenModel`/builder themselves (001) and the dumps (002/003).
- The fixed-size goldens, structured snapshot, debug-token-absence negative, and determinism tests
  (005).
- Any change to the existing `render_embodied_view_uses_sealed_view_model_accessors` or
  `closed_presentation_enum_matches_are_exhaustive_without_wildcards` guards (preserved unchanged).

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_screen_model_field_disposition` asserts a named pane disposition for every current
   `EmbodiedViewModel` field: the public fields `place_id`, `place_label`, `visible_exits`,
   `visible_doors`, `visible_containers`, `visible_items`, `carried_items`, `local_actors`,
   `semantic_actions`, `phase3a_status`, `last_rejection_summary`, `last_rejection_why_not`,
   `notebook`; and the crate-private fields `view_model_id`, `mode`, `viewer_actor_id`, `sim_tick`,
   `holder_known_context_id`, `holder_known_context_hash`, `holder_known_context_frontier`,
   `holder_known_context_source_summary`, `actor_known_interval_summary`, `debug_available`
   (23 fields total, `view_models.rs:20-42`).
2. The guard fails when a field is reached via a wildcard/default arm (source-guard assertion).
3. `tui_seam_conformance_maps_tui_spine_requirements_to_named_tests` passes with the new
   `TUI_SEAM_EVIDENCE` row (requirement id resolves to the new test fn).

### Invariants

1. Every `EmbodiedViewModel` field has an explicit, named pane disposition — no wildcard laundering.
2. The existing TUI-seam guards remain unchanged and passing.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — adds `embodied_screen_model_field_disposition`
   and its `TUI_SEAM_EVIDENCE` row.

### Commands

1. `cargo test -p tracewake-tui --test tui_seam_conformance`
2. `cargo test -p tracewake-tui --test tui_seam_conformance -- --list`
3. Test-target scope is correct: the guard is a static source check over the view-model and
   screen-model sources; no full-pipeline run is required to prove field-disposition closure.

## Outcome

Completed: 2026-07-01

Added the `embodied_screen_model_field_disposition` source guard to the
existing `crates/tracewake-tui/tests/tui_seam_conformance.rs` seam and
registered it in `TUI_SEAM_EVIDENCE` under `TUI-0061-004`. The guard
enumerates the current 23-field `EmbodiedViewModel` disposition set, verifies
each field maps to a named `EmbodiedScreenModel` pane or metadata disposition,
and rejects wildcard/default laundering tokens in the builder body.

Verification:

- `cargo test -p tracewake-tui --test tui_seam_conformance` passed.
- `cargo test -p tracewake-tui --test tui_seam_conformance -- --list` passed
  and listed `embodied_screen_model_field_disposition`.

Deviations:

- None.
