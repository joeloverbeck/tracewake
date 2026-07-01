# 0061TUIEMBSCR-002: Plain-text screen dump renderer

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds `render_embodied_screen_dump` to the `tracewake-tui` `screen` module.
**Deps**: 0061TUIEMBSCR-001

## Problem

An agent reading the TUI needs a stable, framework-free textual form of the embodied screen — the
`SCREEN … / PANE …` layout the research report describes (Spec 0061 §1.1.3, report §6.2). This
ticket adds `render_embodied_screen_dump(&EmbodiedScreenModel) -> String`, the plain-text projection
of the screen model built by 001. It is one of "two dumps, one source" (Spec 0061 §4.3): it and the
structured `ScreenDump` (003) are two projections of the same `EmbodiedScreenModel` and must not
diverge in what they claim. The existing debug-token hygiene extends to this dump (Spec 0061
§1.1.6): no `DEBUG NON-DIEGETIC` marker may appear in embodied output.

## Assumption Reassessment (2026-07-01)

1. `EmbodiedScreenModel` and `build_embodied_screen_model` are introduced by 001 in
   `crates/tracewake-tui/src/screen/`; this ticket adds a renderer over that model and does not
   re-derive panes from `EmbodiedViewModel`. Debug-token hygiene reuses the existing marker
   `tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER`
   (`crates/tracewake-core/src/debug_capability.rs:1`) and/or the `render.rs` `DEBUG_TOKENS` const
   (`crates/tracewake-tui/src/render.rs:5`).
2. Spec 0061 §1.1.3 requires the `SCREEN … / PANE …` textual form; §4.3 requires it not diverge
   from the structured dump (003); §8 requires deterministic ordering of panes, actors, actions,
   and leads.
3. Shared boundary under audit: the `render_embodied_screen_dump(&EmbodiedScreenModel) -> String`
   signature and the pane text layout, which the fixed-size goldens (005) lock. Consumes 001's
   `EmbodiedScreenModel`; produces the primary golden text form.
4. Invariant motivating this ticket: **INV-068 — Debug mode is visibly non-diegetic** (the embodied
   dump must never carry a `DEBUG NON-DIEGETIC` marker; debug output is a separate surface).
5. Fail-closed / actor-knowledge surface: the enforcement surface is embodied-output actor-knowledge
   filtering (INV-067 embodied mode shows actor-known reality; INV-024 no telepathy). The dump
   renders only fields present on `EmbodiedScreenModel` (itself derived from the actor-filtered view
   model in 001) and reads no truth, so it introduces no leakage path; the debug-token-absence
   negative and the byte-identical determinism check are proven in 005.

## Architecture Check

1. Rendering the plain-text form from `EmbodiedScreenModel` (not from `EmbodiedViewModel` directly)
   is what makes the "two dumps, one source" guarantee (§4.3) enforceable: both this dump and the
   structured `ScreenDump` (003) read the same grouped model, so a pane cannot appear in one and
   silently differ in the other. A dump built directly from the view model would re-implement pane
   grouping and could drift from the structured dump.
2. No backwards-compatibility aliasing/shims: `render_embodied_view` (`render.rs`) is left as the
   preserved parallel render (Spec 0061 §1.1.5); this dump is added alongside it, not wrapped around
   it.

## Verification Layers

1. INV-068 (debug non-diegetic) -> codebase grep-proof + unit test: the rendered dump contains no
   `DEBUG NON-DIEGETIC` token for an embodied fixture (negative asserted here and locked in 005).
2. INV-024/INV-067 (actor-known only) -> manual review: the dump reads only `EmbodiedScreenModel`
   panes; no truth handle, no hidden field.
3. Determinism (INV-018) -> unit test: repeated `render_embodied_screen_dump` on the same screen
   model is byte-identical (full golden coverage in 005).

## What to Change

### 1. Plain-text dump renderer

Add `crates/tracewake-tui/src/screen/text_dump.rs` with
`render_embodied_screen_dump(screen: &EmbodiedScreenModel) -> String` producing the `SCREEN … /
PANE …` textual form with deterministic pane/actor/action/lead ordering (Spec 0061 §1.1.3, §8).

### 2. Module export

Register `pub mod text_dump;` and re-export `render_embodied_screen_dump` in
`crates/tracewake-tui/src/screen/mod.rs`.

## Files to Touch

- `crates/tracewake-tui/src/screen/text_dump.rs` (new)
- `crates/tracewake-tui/src/screen/mod.rs` (modify — file created by 0061TUIEMBSCR-001)

## Out of Scope

- The structured `ScreenDump` value and its projection (003).
- The field-disposition conformance guard (004).
- Fixed-size goldens and the debug-token-absence / determinism regression tests (005) — this ticket
  asserts the properties in unit tests; the golden locks live in 005.
- Any `ratatui`/`crossterm` dependency or change to `render_embodied_view` (Spec 0061 §1.1.5, §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. `render_embodied_screen_dump` on a fixture screen model emits a `SCREEN … / PANE …` form whose
   panes match the screen model's grouped content.
2. The rendered embodied dump contains no `DEBUG NON-DIEGETIC` token.
3. Repeated rendering of the same screen model is byte-identical.

### Invariants

1. The dump reads only `EmbodiedScreenModel` — no truth handle, no hidden field.
2. No embodied dump carries a debug non-diegetic marker.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/screen/text_dump.rs` (unit tests) — pane rendering, debug-token
   absence, and render determinism.

### Commands

1. `cargo test -p tracewake-tui screen`
2. `cargo build --workspace --all-targets`

## Outcome

Completed: 2026-07-01

Implemented `render_embodied_screen_dump(&EmbodiedScreenModel) -> String` in
the `tracewake-tui` `screen` module. The renderer emits a stable
`SCREEN`/`META`/`PANE` textual form from `EmbodiedScreenModel` only, keeps a
fixed pane order, and exposes shared pane-dump helpers for the structured dump
ticket so the two dump forms can use one grouped screen-model source.

Verification:

- `cargo test -p tracewake-tui screen` passed.
- `cargo build --workspace --all-targets` passed.

Deviations:

- None.
