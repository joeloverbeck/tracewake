# 0062TUIDETUI-001: `UiIntent` enum and presentation-state model

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new `tracewake-tui` module `intent` (`UiIntent` enum + presentation-state struct); registers `pub mod intent;` in `crates/tracewake-tui/src/lib.rs`. No core types, no event/schema/view-model changes.
**Deps**: None.

## Problem

Spec 0062 (§1.1) replaces `tracewake-tui`'s ad-hoc text-command dispatch with a deterministic intent layer. The foundational types must land first: a closed `UiIntent` enum (§1.1 item 1) and the minimal TUI-only presentation-state model (§1.1 item 6) that the reducer mutates and the headless runner renders. Every later ticket (reducer, parsers, runner, conformance) depends on these types. This ticket introduces the types only — no dispatch behavior.

## Assumption Reassessment (2026-07-01)

1. **Landed pane-identity type (code).** `FocusedPane` is a 12-variant enum at `crates/tracewake-tui/src/screen/model.rs:67` (`Place, Exits, Doors, Containers, Items, Inventory, Actors, Actions, Status, WhyNot, Notebook, ActorKnownInterval`), currently carried as a render option (`ScreenDump.focused_pane`, `screen/struct_dump.rs:11`). Per spec 0062 §1.1 item 6, `UiIntent::FocusPane` reuses this type (the spec's earlier `PaneId` name was corrected to `FocusedPane` during the 2026-07-01 reassessment). Do NOT define a new pane-identity type.
2. **Module placement (spec + code).** The `intent` module is TUI-only presentation logic — it belongs under `crates/tracewake-tui/src/`, registered in `crates/tracewake-tui/src/lib.rs` (which currently declares `app, debug_panels, input, launch, render, run, screen, transcript`). No core (`tracewake-core`) type is added — consistent with spec §8 ("No new core types") and §1.2.
3. **Cross-artifact boundary under audit.** The presentation-state model is the shared contract between this spec's reducer (0062TUIDETUI-002) and Spec 0065's fullscreen shell, which "holds and drives this same model" (spec §2 Contract-to-0065). This ticket defines the struct; 0065 later instantiates it. Confirm the struct exposes: focused pane (`FocusedPane`), a per-pane selection index, scroll offsets, and a help-overlay flag (spec §1.1 item 6) — no more, so 0065 adds no new presentation-state type.
4. **Motivating invariant (INV-069).** `INV-069` ("The TUI must not implement simulation rules") governs: `UiIntent` and the presentation-state model carry only presentation/command intent and presentation state — no world authority, no hidden-truth query, no validator bypass. The enum is a closed set of typed intents, not a rules surface.
5. **Substrate for deferred enforcement surfaces.** These types feed two enforcement surfaces implemented by sibling tickets: the reducer's no-direct-dispatch discipline (0062TUIDETUI-002, `INV-104`) and the runner's no-leak/deterministic dumps (0062TUIDETUI-005, `INV-018`/`INV-024`). This ticket introduces no leakage or nondeterminism path: the presentation-state struct holds only actor-facing presentation data (pane focus, selection, scroll, help), no hidden-truth field, and its values are set only by later typed-intent reduction. No fail-closed-validation, actor-knowledge-filtering, or replay surface is modified here.

## Architecture Check

1. Defining the enum and presentation-state struct as a standalone types-only ticket keeps the reducer, parsers, and runner reviewable as independent diffs against a stable type contract, and gives Spec 0065 a settled struct to consume — cleaner than co-landing types with the reducer, which would couple type review to dispatch logic. Reusing the landed `FocusedPane` avoids a duplicate pane-identity type.
2. No backwards-compatibility aliasing or shims; `intent` is a new module.

## Verification Layers

1. INV-069 (TUI implements no rules) -> codebase grep-proof: `UiIntent`/presentation-state expose no world-mutation or hidden-truth surface; the module imports no core authority type beyond the id/`FocusedPane` types it references.
2. INV-108 (possession cognition-neutral) -> manual review: the presentation-state model is input/viewpoint state only; it grants no planning or knowledge.
3. Type contract -> codebase grep-proof: `FocusedPane` reused from `screen/model.rs` (no new pane-identity type); `pub mod intent;` present in `lib.rs`.

## What to Change

### 1. Add the `intent` module with the `UiIntent` enum

Create `crates/tracewake-tui/src/intent/mod.rs` defining `pub enum UiIntent` — a closed set of presentation/command intents: `FocusNext`, `FocusPane(FocusedPane)`, `MoveSelection(Direction)`, `ActivateSelection`, `SubmitSemanticAction(SemanticActionId)`, `SubmitDebugCommand(DebugCommand)`, `WaitOneTick`, `ContinueUntil { max_ticks: u64 }`, `ToggleHelp`, `FocusNotebook`, `Quit`. Define the supporting `Direction` enum (e.g. `Up`/`Down`). Reference `FocusedPane` (`crate::screen::model::FocusedPane`), `DebugCommand` (`crate::input::DebugCommand`), and `SemanticActionId` (`tracewake_core::ids::SemanticActionId`). No dispatch logic in this ticket.

### 2. Add the presentation-state model

Create `crates/tracewake-tui/src/intent/state.rs` defining a presentation-state struct (e.g. `PresentationState`) holding: `focused_pane: FocusedPane`, a per-pane selection index, scroll offsets, and a `help_open: bool`. Provide a `Default`/constructor for the headless runner and Spec 0065's shell to instantiate. Presentation-only — no world/core surface.

### 3. Register the module

Add `pub mod intent;` to `crates/tracewake-tui/src/lib.rs`.

## Files to Touch

- `crates/tracewake-tui/src/intent/mod.rs` (new)
- `crates/tracewake-tui/src/intent/state.rs` (new)
- `crates/tracewake-tui/src/lib.rs` (modify)

## Out of Scope

- The intent reducer and any dispatch behavior (0062TUIDETUI-002).
- Key-script / semantic-script parsers (003 / 004).
- Headless runner and transcript emission (005).
- Any `crossterm` event loop (Spec 0065) and any new core type (spec §8).
- Wiring the presentation-state model into a fullscreen shell (Spec 0065).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p tracewake-tui --locked` compiles with the new `intent` module.
2. A unit test constructs a `UiIntent` for each variant and a default `PresentationState`, asserting the module is usable by dependents (no behavior asserted here).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`.

### Invariants

1. `UiIntent` and `PresentationState` expose no world-mutation, validator-bypass, or hidden-truth surface (INV-069).
2. `FocusPane` reuses the landed `FocusedPane`; no duplicate pane-identity type is introduced.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/intent/mod.rs` (unit test module) — construct each `UiIntent` variant and a default `PresentationState`; smoke-level, proves the type contract compiles and is constructible.

### Commands

1. `cargo build -p tracewake-tui --locked`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-01

Implemented the TUI-only `intent` module with the closed `UiIntent` enum,
`Direction`, and the minimal `PresentationState` contract for focused pane,
per-pane selection index, per-pane scroll offset, and help overlay state. The
module reuses the existing `FocusedPane`, `DebugCommand`, and
`SemanticActionId` types; it adds no core type, reducer behavior, event
application path, validator bypass, or hidden-truth surface.

Because the ticket adds new production source files, the workspace source
classification guard was updated for `crates/tracewake-tui/src/intent/mod.rs`
and `crates/tracewake-tui/src/intent/state.rs` with the existing TUI
presentation-boundary rationale.

Verification:

- `cargo build -p tracewake-tui --locked`
- `cargo test -p tracewake-tui intent`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p tracewake-tui`
