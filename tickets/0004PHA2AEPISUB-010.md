# 0004PHA2AEPISUB-010: Debug epistemics view models

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds debug epistemics view models to `tracewake-core` (`view_models.rs`).
**Deps**: 0004PHA2AEPISUB-007

## Problem

Debug mode may reveal truth but must be visibly non-diegetic and must never satisfy actor knowledge (`INV-068`, `INV-031`). Spec 0004 §12.3 requires debug-only view models exposing ground-truth item location, all observations, all beliefs by holder, contradiction links, truth/belief mismatch, possession binding history as non-world metadata, and event/projection rebuild detail — each carrying a visible non-diegetic marker (e.g. `DEBUG NON-DIEGETIC: Epistemics`). Debug inspection must never write to actor notebooks.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/view_models.rs` has debug view models for controller binding, event log, item location, action rejection, projection rebuild, and replay report, but no debug epistemics view. `crates/tracewake-core/src/debug_reports.rs` carries structured debug provenance; the new epistemics views reuse its non-diegetic-marker convention.
2. Debug contents and the non-diegetic-marker requirement are fixed by Spec 0004 §12.3; debug reads truth from `PhysicalState` and the full `EpistemicProjection` (legitimately, in debug mode), unlike the embodied notebook.
3. Shared boundary under audit: `view_models.rs` is modified here and by ticket 009 as **parallel independent siblings** (both branch off ticket 007, no dep between them) — they will mechanically conflict on `view_models.rs`; coordinate the merge. This ticket owns only the debug epistemics surface.
4. Invariant motivating this ticket: `INV-068` (debug mode is visibly non-diegetic) and `INV-031` (human/debug notes are non-diegetic and never satisfy actor knowledge).
5. Actor-knowledge / no-leak surface: debug views deliberately show truth/mismatch, so the invariant they protect is the *separation* — debug uses a debug-mode `KnowledgeContext` (ticket 003), carries a visible non-diegetic marker, and has no write path into the embodied notebook (ticket 009). This ticket adds no embodied-leak path (it never feeds the actor-filtered view) and no nondeterminism (debug listings are ordered by stable id).

## Architecture Check

1. Separate debug epistemics view models (distinct from the embodied notebook) make the embodied/debug separation structural: the debug view is the only surface allowed to read ground truth + all-holder beliefs, and it is marked non-diegetic, so a reviewer can verify the firewall by type. Merging debug and embodied views would reintroduce the leak risk Phase 2A exists to eliminate.
2. No backwards-compatibility shims: debug epistemics views are additive alongside existing debug view models.

## Verification Layers

1. Debug visibly non-diegetic (`INV-068`) -> manual review + test: each debug epistemics view carries the non-diegetic marker; rendering (ticket 011) preserves it.
2. Debug never satisfies actor knowledge (`INV-031`) -> grep-proof + test: the debug view has no write path to the notebook; the embodied notebook (ticket 009) does not read debug views.
3. Truth/belief mismatch surfaced (`INV-068`) -> unit test: the debug mismatch view shows ground-truth item location alongside Tomas's missing-property belief and the divergence.

## What to Change

### 1. Debug epistemics view models

Add to `view_models.rs` debug view types for: `debug epistemics` (all actors — observations, beliefs by holder, contradictions, possession history as non-world metadata), `debug beliefs <actor>` (holder-focused), and `debug observations <actor>` (observer-focused), plus a truth/belief-mismatch view comparing `PhysicalState` ground truth to held beliefs. Each carries a visible non-diegetic marker constant.

### 2. Non-diegetic marker and mode

Build these views from a debug-mode `KnowledgeContext`; expose the `DEBUG NON-DIEGETIC: Epistemics` marker (or the existing debug convention from `debug_reports.rs`) so the renderer (ticket 011) cannot present them as embodied.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify — add debug epistemics view models; shared with ticket 009)

## Out of Scope

- TUI parsing/rendering of `debug epistemics`/`debug beliefs`/`debug observations` (ticket 011).
- The embodied notebook (ticket 009).
- Replay report fields (ticket 005).

## Acceptance Criteria

### Tests That Must Pass

1. `debug epistemics` view lists all observations, beliefs by holder, contradiction links, and possession history, and carries the non-diegetic marker.
2. The truth/belief-mismatch view shows ground-truth coin location and Tomas's missing-property belief side by side.
3. No debug view exposes a write method into the actor notebook.

### Invariants

1. Debug epistemics views are visibly non-diegetic and built from a debug-mode context.
2. Debug inspection never mutates actor-known projections.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/view_models.rs` (unit tests) — debug epistemics/beliefs/observations contents, non-diegetic marker, mismatch view.

### Commands

1. `cargo test -p tracewake-core view_models::`
2. `cargo test -p tracewake-core`
3. `cargo build --workspace --all-targets --locked`
