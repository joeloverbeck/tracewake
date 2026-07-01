# 0061TUIEMBSCR-001: Embodied screen-model types and builder

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a new `screen` module (`EmbodiedScreenModel`, `RenderOptions`, `build_embodied_screen_model`) to `tracewake-tui`.
**Deps**: None

## Problem

The TUI Experience Overhaul (Spec 0061, Spec A / roadmap root) needs a pure, presentation-only
screen model between the existing `EmbodiedViewModel` and any future terminal drawing, so that
every later overhaul spec (`0062`–`0070`) renders through one derived seam rather than re-reading
truth. This ticket introduces the screen-model types and the deterministic, side-effect-free
builder that groups the embodied view model's fields into named panes (Spec 0061 §1.1.1–1.1.2,
§4.2). It is the foundation the plain-text dump (002), the structured `ScreenDump` (003), and the
field-disposition guard (004) all consume. It adds no dump renderer, no `ratatui`/`crossterm`
dependency, and no event loop (Spec 0061 §1.2).

## Assumption Reassessment (2026-07-01)

1. No `screen` module exists in `tracewake-tui` (`ls crates/tracewake-tui/src/screen*` → none);
   `crates/tracewake-tui/src/lib.rs` registers modules flatly (`pub mod app; … pub mod render;`)
   and this ticket adds `pub mod screen;` there. The source view model is
   `tracewake_core::view_models::EmbodiedViewModel` (`crates/tracewake-core/src/view_models.rs:19`),
   consumed by shared reference only.
2. Spec 0061 §1.1.1 enumerates the panes' source fields; the public fields are `place_label`,
   `place_id`, `visible_exits`, `visible_doors`, `visible_containers`, `visible_items`,
   `carried_items`, `local_actors`, `semantic_actions`, `phase3a_status`, `last_rejection_summary`,
   `last_rejection_why_not`, `notebook` (`view_models.rs:24-42`), plus the crate-private fields
   reached through sealed accessors — `viewer_actor_id()` (`view_models.rs:108`), `sim_tick()`
   (`view_models.rs:112`), `holder_known_context_id()`/`_hash()` (`view_models.rs:116,120`),
   `holder_known_context_frontier()`/`_source_summary()` (`view_models.rs:124,128`),
   `actor_known_interval_summary()` (`view_models.rs:132`), and `debug_available` (crate-private).
3. Shared boundary under audit: the `EmbodiedScreenModel` pane contract and the
   `build_embodied_screen_model(&EmbodiedViewModel, RenderOptions) -> EmbodiedScreenModel`
   signature that 002 (plain-text dump), 003 (structured `ScreenDump`), and 004 (disposition
   guard) all depend on. Published here.
4. Invariants motivating this ticket: **INV-069 — The TUI must not implement simulation rules**
   (the screen model is pure presentation over an actor-filtered view model; it holds no
   `PhysicalState`/session handle and performs no truth read) and **INV-099 — Truth may validate
   actions, but truth may not plan them** (the builder consumes only the sealed view model, never
   hidden world state).
5. Fail-closed / actor-knowledge surface: the enforcement surface is the actor-knowledge firewall
   already applied when core produces `EmbodiedViewModel` (INV-024 no telepathy; INV-067 embodied
   mode shows actor-known reality). `build_embodied_screen_model` takes `&EmbodiedViewModel` by
   shared reference, carries no truth handle, and derives no field from any source other than the
   view model, so it introduces no new leakage path and no nondeterministic input. The no-leak
   negative and the byte-identical determinism check land in 005; the closed field→pane disposition
   guard lands in 004. This ticket is substrate for both.

## Architecture Check

1. A single derived `EmbodiedScreenModel` built by shared reference from `&EmbodiedViewModel`
   keeps presentation structure in one place and preserves the "screen model is derived, never
   authoritative" rule (Spec 0061 §4.2): because the builder never holds a truth handle, no later
   consumer (dumps, layout, shell) can accidentally re-read world state to freshen a pane. Grouping
   fields into named panes now — rather than letting each future renderer re-group ad hoc — is what
   lets the disposition guard (004) prove closure. `RenderOptions` carries only presentation inputs
   (terminal size, focused pane, theme flags), never a truth handle, keeping the builder pure.
2. No backwards-compatibility aliasing/shims: this is the first screen-model surface; there is no
   prior version to alias. `render_embodied_view` is left untouched (a preserved parallel render,
   per Spec 0061 §1.1.5).

## Verification Layers

1. INV-069 (TUI implements no simulation rules) -> codebase grep-proof: `build_embodied_screen_model`
   takes `&EmbodiedViewModel` + `RenderOptions` only; no `PhysicalState`/session/runtime handle in
   the `screen` module (grep the module for truth-handle types).
2. INV-099 / INV-024 (no truth read, no telepathy) -> manual review + unit test: the screen model's
   fields are all derived from the view model; a unit test builds from a fixture view model and
   asserts the panes reflect only view-model content.
3. Determinism substrate (INV-018) -> unit test: `build_embodied_screen_model` on the same view
   model + options yields an equal `EmbodiedScreenModel` (full byte-identical dump determinism is
   proven over the dumps in 005).

## What to Change

### 1. Screen-model module scaffold

Add `crates/tracewake-tui/src/screen/mod.rs` re-exporting the module's public surface, and register
`pub mod screen;` in `crates/tracewake-tui/src/lib.rs`.

### 2. Types

Add `crates/tracewake-tui/src/screen/model.rs` with:
- `EmbodiedScreenModel` — a presentation-only struct grouping the view model's fields into named
  panes (place, exits, doors, containers, items, carried, local actors, semantic actions, phase3a
  status, why-not, notebook, actor-known interval summary, and the actor/tick/holder-known context
  metadata). It holds no simulation authority and no physical-state handle.
- `RenderOptions` — presentation inputs only: terminal size, focused pane, theme flags. No truth
  handle.

### 3. Builder

Add `build_embodied_screen_model(view: &EmbodiedViewModel, opts: RenderOptions) -> EmbodiedScreenModel`
— deterministic and side-effect-free, taking the view model by shared reference. Deterministic
ordering of panes, actors, actions, and leads (Spec 0061 §8).

## Files to Touch

- `crates/tracewake-tui/src/screen/mod.rs` (new)
- `crates/tracewake-tui/src/screen/model.rs` (new)
- `crates/tracewake-tui/src/lib.rs` (modify — add `pub mod screen;`)

## Out of Scope

- The plain-text `render_embodied_screen_dump` renderer (002).
- The structured `ScreenDump` value and its projection (003).
- The `embodied_screen_model_field_disposition` conformance guard (004).
- Fixed-size goldens, `ScreenDump` snapshot, debug-token-absence negative, determinism test, and
  the acceptance artifact (005).
- Any `ratatui`/`crossterm` dependency, `UiIntent` reducer, event loop, or change to any
  `tracewake-core`/`tracewake-content` type (Spec 0061 §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. `build_embodied_screen_model` builds an `EmbodiedScreenModel` from a fixture `EmbodiedViewModel`
   and every pane reflects only view-model content (no field invented).
2. Building twice from the same view model + `RenderOptions` yields equal screen models.
3. `cargo build --workspace --all-targets` compiles with the new `screen` module registered.

### Invariants

1. `build_embodied_screen_model` takes `&EmbodiedViewModel` + `RenderOptions` and holds no
   `PhysicalState`/session/runtime handle (pure derivation).
2. `RenderOptions` carries only presentation inputs — no truth handle.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/screen/model.rs` (unit tests) — build-from-fixture pane mapping and
   builder equality/determinism.

### Commands

1. `cargo test -p tracewake-tui screen`
2. `cargo build --workspace --all-targets`
3. Unit scope is correct: the screen model has no cross-crate behavior until the dumps (002/003)
   render it and the guard (004) proves field closure.

## Outcome

Completed: 2026-07-01

Implemented the `tracewake-tui` `screen` module with `EmbodiedScreenModel`,
`RenderOptions`, `TerminalSize`, `FocusedPane`, `ScreenMetadata`, and
`build_embodied_screen_model(&EmbodiedViewModel, RenderOptions)`. The builder
clones actor-filtered view-model presentation data into named panes and carries
only presentation options plus sealed view-model metadata; it accepts no
`PhysicalState`, runtime, session, or other truth handle.

Verification:

- `cargo test -p tracewake-tui screen` passed.
- `cargo build --workspace --all-targets` passed.

Deviations:

- None.
