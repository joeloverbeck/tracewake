# 0002PHA1KERTUI-012: Projection builders and embodied/debug view models

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `projections` and `view_models` modules to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-003, 0002PHA1KERTUI-006

## Problem

The TUI and tests consume kernel-produced view models; they must never query hidden truth or infer rules (Spec 0002 §7.6, §15.3–§15.5). This ticket adds the derived projection builders and the embodied/debug view-model data structures: the embodied local view model (§15.3 — viewer actor, place, visible exits/doors/containers/items, local actors, semantic actions, last-rejection summary) filtered by §12.4 visibility, the semantic-action entries (§15.4 — stable IDs, not menu indices), and the debug view-model structures (§15.5). These feed the TUI shell (020), projection rebuild (013), and the inspect action (011).

## Assumption Reassessment (2026-06-06)

1. No `projections`/`view_models` module exists; registers both in `crates/tracewake-core/src/lib.rs` (001), reading `state`/`location` visibility (003) and the event log (006).
2. The embodied view-model fields are `specs/0002_…_SPEC.md` §15.3; semantic-action entry fields are §15.4; debug view models are §15.5; the §12.4 visibility rules bound what the embodied model may contain. Display labels are explicitly "not authority" (§15.3/§15.4).
3. Shared boundary under audit: the `EmbodiedViewModel`/`SemanticActionEntry`/debug view-model types consumed by the TUI render/input (020), the inspect action (011), and transcript tests (021). Fixed here.
4. Invariant motivating this ticket: INV-024 (no telepathy) and INV-067 (embodied mode shows actor-known reality) — the embodied projection is the firewall surface; it must exclude closed-opaque-container contents, items in other places, debug truth, event-log truth, and controller binding as a world fact (§12.4).
5. No-leak surface: the embodied view model is built solely from the §12.4 visibility query (ticket 003); it has no access to the full event log's hidden facts. Debug view models are a separate type, never merged into the embodied one. This ticket guarantees the embodied projection cannot carry hidden truth; the negative regression is ticket 022's debug-non-leakage test.

## Architecture Check

1. Two distinct view-model types (embodied vs debug) built by separate projection functions make leakage a type-level concern: an embodied projection function never receives debug truth, so it cannot leak it. A single view model with a "debug" flag would risk a code path that conditionally reveals truth.
2. Deterministically-ordered fields (visible exits/doors/containers/items, semantic actions) keep the view model and any transcript stable. No backwards-compatibility shims: greenfield.

## Verification Layers

1. No-telepathy (INV-024/067) -> unit test: the embodied projection of an actor with a closed-opaque container in the room excludes the container contents and any item in another place.
2. Stable semantic IDs (§15.4) -> unit test: `SemanticActionEntry.semantic_action_id` is a stable target-specific ID, not an array index, and ordering is deterministic.
3. Embodied/debug separation (§15.5; INV-068) -> codebase grep-proof: the embodied view-model builder takes no debug/truth input parameter.

## What to Change

### 1. Projections

Add `crates/tracewake-core/src/projections.rs`: derived physical/local/debug projection builders over authoritative state + event history, building the embodied projection through the §12.4 visibility query.

### 2. View models

Add `crates/tracewake-core/src/view_models.rs`: the `EmbodiedViewModel` (§15.3 fields), `SemanticActionEntry` (§15.4 fields), and debug view-model structures (§15.5), all with deterministic ordering. No terminal rendering here.

### 3. Registration

Add `pub mod projections;` and `pub mod view_models;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (new)
- `crates/tracewake-core/src/view_models.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add module declarations; file created by ticket 001)

## Out of Scope

- Projection rebuild from fixture + ordered events (ticket 013 — this ticket provides the live projection builders it reuses).
- Terminal rendering of view models (ticket 020).
- Structured debug provenance reports (ticket 016 — distinct from the debug view-model data structures here).

## Acceptance Criteria

### Tests That Must Pass

1. The embodied projection excludes closed-opaque-container contents, other-place items, debug truth, and controller binding.
2. Semantic-action entries carry stable target-specific IDs in deterministic order; no menu index is used as identity.
3. The embodied view-model builder has no parameter through which debug truth could enter.

### Invariants

1. The embodied view model contains only §12.4-visible facts.
2. Embodied and debug view models are distinct types built by distinct functions.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (unit tests) — visibility filtering, deterministic ordering.
2. `crates/tracewake-core/src/view_models.rs` (unit tests) — stable semantic IDs, embodied/debug separation.

### Commands

1. `cargo test -p tracewake-core projections view_models`
2. `cargo build --workspace`
3. Unit scope is correct; the full no-leak regression across an embodied↔debug round trip is exercised in ticket 022.

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::view_models` with embodied view-model, semantic-action, and separate debug view-model structures.
- Added `tracewake_core::projections` with an embodied view builder using the Phase 1 visibility query and a separate debug event-log view builder.
- Registered `pub mod projections;` and `pub mod view_models;` in the core crate.

Deviations from original plan:
- The documented combined Cargo filter `cargo test -p tracewake-core projections view_models` is not accepted by Cargo as written, so verification used separate filters.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core projections` passed: 2 tests.
- `cargo test -p tracewake-core view_models` passed: 2 tests.
- `cargo build --workspace` passed.
- Grep confirmed the embodied view-model builder is distinct from debug builders and has no debug/truth input parameter.
