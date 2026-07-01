# 0064TUIEMBPAN-002: Per-pane actor-safe binding renderers

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` presentation module `crates/tracewake-tui/src/screen/pane_bindings.rs`
**Deps**: 0064TUIEMBPAN-001

## Problem

Spec 0064 §1.1.2 requires presentation-only binding renderers that turn each region's `EmbodiedScreenModel` panes into actor-safe content lines: Header/mode bar from `viewer_actor_id`/`ViewMode` + a debug-availability marker (no hidden debug truth in embodied mode); Place from place/exits/doors/containers/items; Self from status + inventory; Actions from `semantic_actions` (menu index presentation-only, semantic id stable); Details/Why-not from the why-not pane (actor-safe only); Notebook from the notebook pane; Recent from the actor-known interval summary; Co-present actors from the enriched `VisibleActor`, rendering each row's activity disposition (spec §4.3, §4.5). §4.5 requires these renderers be shared by the buffer and text/dump paths so the two cannot diverge (0064 §1.1.3). This is the content layer over the region skeleton from 0064TUIEMBPAN-001.

## Assumption Reassessment (2026-07-02)

1. Reuse, do not recreate, the pane data and the existing disposition renderer. `crates/tracewake-tui/src/screen/text_dump.rs:188` already defines `pub(crate) fn render_activity_disposition(activity: &ObservedActivityView) -> String`; the actor pane binding must reuse it (same crate, `pub(crate)` reachable), not fork disposition formatting. `ActorsPane { local_actors: Vec<VisibleActor> }` (`model.rs`) holds the rows; `VisibleActor { observed_activity: Option<ObservedActivityView>, … }` (`crates/tracewake-core/src/view_models.rs:461`) is the landed Spec 0063 enrichment, with `VisibleActor::identity_only` (`view_models.rs:472`) as the no-apparent-activity case (not a dependency-wait fallback).
2. **Header-binding source correction (mechanical drift from spec §1.1.2):** at the screen-model layer this ticket renders, `ScreenMetadata` (`model.rs`) carries `viewer_actor_id: ActorId` and `mode: ViewMode` but has **no `debug_available` field** — that bool lives on the upstream `EmbodiedViewModel`. The screen model instead carries `debug: DebugPaneDisposition` on `EmbodiedScreenModel`. So the header binding reads `metadata.viewer_actor_id`, `metadata.mode`, and the `DebugPaneDisposition` for the actor-safe debug-*availability* marker only. Same concept as spec §1.1.2's report-derived `debug_available`, corrected to the actual screen-model surface.
3. Shared boundary under audit: the region model from 0064TUIEMBPAN-001 (`crate::screen::pane_layout`) is this ticket's input contract; the per-region line output this ticket produces is the contract that 0064TUIEMBPAN-003 (buffer render) and 0064TUIEMBPAN-004 (responsive/truncation) consume. "Shared by buffer and text/dump paths" (§4.5) means the binding renderers are the single extraction; `render_embodied_view` / `render_embodied_screen_dump` remain the existing text seams and are not forked.
4. Constitutional invariants under audit: **INV-067 — Embodied mode shows actor-known reality** and **INV-024 — No telepathy** (panes render only actor-filtered `EmbodiedScreenModel` fields, no hidden truth); **INV-068 — Debug mode is visibly non-diegetic** (header shows debug *availability*, never debug truth, in embodied mode); **INV-025 — Wrong beliefs are first-class** (why-not/notebook panes present fallible actor-known belief, not truth); **INV-008 — UI assistance is not authority** (bindings clarify, they grant no knowledge and bypass no precondition).
5. Actor-knowledge-filtering surface: this ticket is the embodied-view actor-safe projection layer, so it is exactly the surface INV-093 (leakage is high-severity) guards. The `EmbodiedScreenModel` is already actor-filtered upstream (0061/0063); these renderers must emit no field outside it and must carry no debug/non-diegetic token into any embodied pane. The actor-known-only assertion (spec §6: no debug tokens in any pane) lands here as a negative test; deterministic line ordering (spec §8) is preserved so the 0064TUIEMBPAN-005 replay/snapshot witnesses hold.

## Architecture Check

1. A single per-pane binding layer feeding both render targets is the mechanism §4.5 requires to prevent buffer/text drift: truncation, ordering, and actor-safe filtering live in one place, tested once, rather than re-implemented per backend. Producing structured lines (not backend widgets) keeps the layer `ratatui`-free and unit-testable, and lets 0064TUIEMBPAN-003 map lines→`Buffer` and the text path map lines→`String` identically.
2. No backwards-compatibility aliasing or shims: `render_activity_disposition` is reused in place; the existing `render_embodied_view` / `render_embodied_screen_dump` seams are left intact, not wrapped or duplicated.

## Verification Layers

1. INV-067 / INV-024 (actor-known only, no telepathy) -> manual review + negative test: a fixture screen renders no field absent from `EmbodiedScreenModel` and no debug/non-diegetic token in any embodied pane.
2. INV-068 (debug non-diegetic) -> unit test: the header binding emits only a debug-availability marker from `DebugPaneDisposition`, never debug-truth content, in embodied mode.
3. INV-008 / INV-069 (UI is not authority) -> codebase grep-proof: `pane_bindings.rs` reads only `crate::screen` model/layout types; no `tracewake_core` truth query or precondition bypass.
4. INV-018 (deterministic ordering, spec §8) -> unit test: repeated renders of one model yield byte-identical per-pane lines.

## What to Change

### 1. Add per-pane binding renderers

Add `crates/tracewake-tui/src/screen/pane_bindings.rs`: for each `PaneRegion` (from 0064TUIEMBPAN-001), a pure renderer `&<region panes> -> Vec<line>` producing actor-safe content:

- Header/mode bar ← `metadata.viewer_actor_id`, `metadata.mode`, and `DebugPaneDisposition` (availability marker only).
- Place/Situation ← place/exits/doors/containers/items panes.
- Self/Body/Routine ← `phase3a_status` pane and `inventory` (carried items).
- Co-present actors ← `ActorsPane.local_actors`, each row rendering identity plus, when `observed_activity` is `Some`, `render_activity_disposition`; `identity_only` rows render as no-apparent-activity.
- Actions/Affordances ← `semantic_actions` (stable semantic id shown; menu index presentation-only).
- Details/Why-not ← why-not pane (actor-safe fields only).
- Notebook/Leads ← notebook pane (source-bound, fallible leads).
- Recent actor-known changes ← actor-known-interval pane summary.
- Input-hints footer ← static pane-local hint text (no live input; presentation-only).

### 2. Register the module

In `crates/tracewake-tui/src/screen/mod.rs`, add `pub mod pane_bindings;` and re-export the binding entry point.

## Files to Touch

- `crates/tracewake-tui/src/screen/pane_bindings.rs` (new)
- `crates/tracewake-tui/src/screen/mod.rs` (modify)

## Out of Scope

- Region taxonomy and layout ordering (0064TUIEMBPAN-001).
- `ratatui` buffer rendering, snapshot harness, new dependency (0064TUIEMBPAN-003).
- Responsive collapse, truncation markers, minimum-size floor (0064TUIEMBPAN-004).
- Any core view-model change; the actor pane binds the already-landed enriched `VisibleActor` (spec §1.2).
- Theming/color (semantics must be present in text regardless — handled as a constraint in 004, not here).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui pane_bindings` — each region renderer produces the expected actor-safe lines from a fixture `EmbodiedScreenModel`, including an actor row with `Some(observed_activity)` (disposition rendered) and an `identity_only` row (no-apparent-activity).
2. Actor-known-only negative test: no debug/non-diegetic token appears in any embodied pane's rendered lines (INV-093 / spec §6).
3. Header binding emits a debug-availability marker sourced from `DebugPaneDisposition` and no debug-truth content in embodied mode.
4. `cargo test --workspace` passes.

### Invariants

1. Rendered content is a subset of actor-filtered `EmbodiedScreenModel` fields — no hidden truth reaches any pane (INV-024/INV-067).
2. Per-pane line output is deterministic for identical input (INV-018 / spec §8).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/screen/pane_bindings.rs` (unit tests) — per-region rendering correctness, actor activity disposition reuse, and the actor-known-only (no-debug-token) negative.

### Commands

1. `cargo test -p tracewake-tui pane_bindings`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`

## Outcome

Completed: 2026-07-02

Implemented the TUI-only pane binding layer in
`crates/tracewake-tui/src/screen/pane_bindings.rs` and exported it from
`crates/tracewake-tui/src/screen/mod.rs`. The binding layer renders the Spec
0064 region sequence into actor-safe line groups by sharing the existing screen
dump pane content, adding only the presentation header and input-hints footer.
Actor rows reuse the existing observed-activity disposition formatting, and
the header exposes only debug availability, not debug truth.

The new production source files `pane_layout.rs` and `pane_bindings.rs` were
added to the workspace source-classification census in
`crates/tracewake-core/tests/anti_regression_guards.rs` under the existing TUI
presentation rationale after the workspace test correctly failed closed on the
new files.

Verification run:

- `cargo fmt --all --check`
- `cargo test -p tracewake-tui pane_bindings`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

No core/content dependency, core view-model field, hidden-truth read, world
mutation path, backwards-compatibility shim, or alias path was added.
