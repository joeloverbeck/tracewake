# 0061TUIEMBSCR-003: Structured `ScreenDump` value and projection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `ScreenDump` value and its projection to the `tracewake-tui` `screen` module.
**Deps**: 0061TUIEMBSCR-001

## Problem

Tests and tools need a parseable, structured form of the embodied screen — not just the plain-text
dump (002). This ticket adds a `ScreenDump` value (mode, terminal size, focused pane, per-pane
dumps, action refs, `debug_marker_present`, view-model id, holder-known context hash) and the
projection that builds it from an `EmbodiedScreenModel` (Spec 0061 §1.1.4). It is the second of
"two dumps, one source" (§4.3): it must not diverge from the plain-text dump in what it claims.
Embodied dumps carry only actor-known metadata already on the view model; no debug-only world truth
(Spec 0061 §1.1.4, §4.2). The holder-known frontier and source-summary accessors are deliberately
excluded to bound snapshot churn (Spec 0061 §1.1.4, §9.1 #6).

## Assumption Reassessment (2026-07-01)

1. `EmbodiedScreenModel` and `build_embodied_screen_model` are introduced by 001; this ticket adds a
   structured projection over that model. The actor-known metadata fields come from sealed
   accessors on `EmbodiedViewModel` carried through the screen model: `mode()`/`ViewMode`
   (`crates/tracewake-core/src/view_models.rs:104,14`), `view_model_id()` (`view_models.rs:100`),
   and `holder_known_context_hash()` (`view_models.rs:120`). `debug_marker_present` is computed
   against `tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER`
   (`crates/tracewake-core/src/debug_capability.rs:1`).
2. Spec 0061 §1.1.4 enumerates the `ScreenDump` fields; §4.3 requires no divergence from the
   plain-text dump (002); §9.1 #6 motivates excluding the holder-known frontier/source-summary.
3. Shared boundary under audit: the `ScreenDump` field schema and its projection signature, which
   the structured snapshot (005) locks and which future overhaul specs (`0062`–`0070`) parse.
   Consumes 001's `EmbodiedScreenModel`.
4. Invariants motivating this ticket: **INV-024 — No telepathy** (the dump carries only actor-known
   metadata already on the view model; no hidden world truth) and **INV-068 — Debug mode is visibly
   non-diegetic** (`debug_marker_present` is a boolean flag; no `DEBUG NON-DIEGETIC` content leaks
   into an embodied dump).
5. Fail-closed / actor-knowledge surface: the enforcement surface is embodied-output actor-knowledge
   filtering (INV-024, INV-067). The projection reads only `EmbodiedScreenModel` (derived from the
   actor-filtered view model in 001) and the sealed actor-known accessors; it carries no
   debug-only world truth and reads no `PhysicalState`, so it introduces no leakage path. The
   holder-known context hash it carries is already actor-known metadata on the view model. The
   no-leak negative and byte-identical determinism check are proven in 005.
6. Schema introduced (additive, no existing consumer): `ScreenDump` is a **new** serialized snapshot
   contract, not an extension of an existing schema. Consumers are the structured snapshot test
   (005) and future overhaul tooling (`0062`–`0070`); it breaks no existing consumer (there is
   none). It deliberately omits `holder_known_context_frontier`/`_source_summary`
   (`view_models.rs:124,128`) to bound snapshot churn (§9.1 #6), carrying only the stable
   `holder_known_context_hash`.

## Architecture Check

1. Projecting `ScreenDump` from `EmbodiedScreenModel` (the same source as the plain-text dump)
   enforces "two dumps, one source" (§4.3): both dumps read one grouped model, so per-pane content
   cannot diverge between the human-readable and machine-parseable forms. Representing debug
   presence as a `debug_marker_present` boolean — rather than embedding any marker text — keeps the
   embodied dump structurally incapable of carrying debug content.
2. No backwards-compatibility aliasing/shims: `ScreenDump` is a new v1 snapshot schema; there is no
   prior structured form to alias, and `render_embodied_view` is untouched.

## Verification Layers

1. INV-024 (no telepathy) -> unit test + manual review: the `ScreenDump` fields derive only from the
   screen model and sealed actor-known accessors; no truth handle, no debug-only field.
2. INV-068 (debug non-diegetic) -> unit test: for an embodied fixture, `debug_marker_present` is a
   flag and no per-pane dump contains a `DEBUG NON-DIEGETIC` token.
3. Schema/determinism (INV-018) -> replay/golden check (in 005): the structured snapshot for
   `ordinary_workday_001` is byte-stable across repeated projection; here a unit test asserts
   projection equality on repeat.

## What to Change

### 1. `ScreenDump` value

Add `crates/tracewake-tui/src/screen/struct_dump.rs` defining `ScreenDump` with the §1.1.4 fields:
mode, terminal size, focused pane, per-pane dumps, action refs, `debug_marker_present`, view-model
id, and holder-known context hash. Exclude the holder-known frontier and source summary (§9.1 #6).

### 2. Projection

Add the projection `build … -> ScreenDump` from `&EmbodiedScreenModel`, with deterministic ordering
of panes, actors, actions, and leads (§8), sharing pane content with the plain-text dump (§4.3).

### 3. Module export

Register `pub mod struct_dump;` and re-export `ScreenDump` + its projection in
`crates/tracewake-tui/src/screen/mod.rs`.

## Files to Touch

- `crates/tracewake-tui/src/screen/struct_dump.rs` (new)
- `crates/tracewake-tui/src/screen/mod.rs` (modify — file created by 0061TUIEMBSCR-001)

## Out of Scope

- The plain-text `render_embodied_screen_dump` (002).
- The field-disposition conformance guard (004).
- The structured snapshot golden, debug-token-absence negative, and determinism goldens (005) —
  asserted here in unit tests, locked as goldens in 005.
- Any `ratatui`/`crossterm` dependency or change to a `tracewake-core`/`tracewake-content` type
  (Spec 0061 §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. The `ScreenDump` projection on a fixture screen model populates every §1.1.4 field, and per-pane
   dumps match the plain-text dump's pane content (no divergence, §4.3).
2. For an embodied fixture, `debug_marker_present` reflects debug availability and no per-pane dump
   carries a `DEBUG NON-DIEGETIC` token.
3. The `ScreenDump` carries the holder-known context hash but not the frontier/source-summary.

### Invariants

1. `ScreenDump` carries only actor-known metadata already on the view model — no debug-only world
   truth.
2. Repeated projection of the same screen model yields an equal `ScreenDump`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/screen/struct_dump.rs` (unit tests) — field population, dump/plain-text
   non-divergence, actor-known-only content, and projection determinism.

### Commands

1. `cargo test -p tracewake-tui screen`
2. `cargo build --workspace --all-targets`

## Outcome

Completed: 2026-07-01

Implemented the structured `ScreenDump` projection over `EmbodiedScreenModel`
in `tracewake-tui::screen`. The dump carries mode, terminal size, focused pane,
per-pane dumps, semantic action refs, `debug_marker_present`, view-model id,
and holder-known context hash. Per-pane content is projected through the same
pane helper used by the plain-text dump, and the structured type deliberately
omits holder-known frontier and source-summary fields.

Verification:

- `cargo test -p tracewake-tui screen` passed.
- `cargo build --workspace --all-targets` passed.

Deviations:

- `debug_marker_present` records whether the embodied pane dump contains the
  debug marker; for the embodied fixture it is `false`, preserving debug-token
  hygiene without exposing debug-only world truth.
