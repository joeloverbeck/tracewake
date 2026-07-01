# 0064TUIEMBPAN-001: Embodied pane taxonomy and stable layout regions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` presentation module `crates/tracewake-tui/src/screen/pane_layout.rs`
**Deps**: None

## Problem

Today's embodied output is a single vertical list with no stable panel positions (spec 0064 §0 Driver). Spec 0064 §1.1.1 / §4.1–4.2 require a stable, at-a-glance region composition over the existing `EmbodiedScreenModel`: a header/mode bar, Place/Situation, Self/Body/Routine, Co-present actors, Actions/Affordances, Details/Why-not, Notebook/Leads, Recent actor-known changes, and an input-hints footer — with fixed positions so a player builds peripheral memory, and urgent facts (needs, why-not, actions) given priority regions while background info stays peripheral.

This ticket introduces the presentation-only **layout region model**: a stable mapping from the already-existing per-pane data (spec 0061's `EmbodiedScreenModel` sub-panes) into the coarser visual regions the spec names, with deterministic region ordering. It does not render to a terminal buffer (that is 0064TUIEMBPAN-003) and does not fill per-pane content lines (that is 0064TUIEMBPAN-002).

## Assumption Reassessment (2026-07-02)

1. The pane **data model already exists** and this ticket must reuse, not recreate it: `crates/tracewake-tui/src/screen/model.rs` defines `EmbodiedScreenModel` with sub-panes `place: PlacePane`, `exits: ExitsPane`, `doors: DoorsPane`, `containers: ContainersPane`, `items: ItemsPane`, `inventory: InventoryPane`, `actors: ActorsPane`, `actions: ActionsPane`, `phase3a_status: Phase3AStatusPane`, `why_not: WhyNotPane`, `notebook: NotebookPane`, `actor_known_interval: ActorKnownIntervalPane`, `debug: DebugPaneDisposition`, plus `metadata: ScreenMetadata` and `options: RenderOptions`. A pane-focus enum `FocusedPane` (variants `Place, Exits, Doors, Containers, Items, Inventory, Actors, Actions, Status, WhyNot, Notebook, ActorKnownInterval`) already exists at `model.rs:67`. This ticket adds a region-grouping layer above these; it introduces no new pane data struct and no core view-model change (spec 0064 §1.2).
2. Spec 0064 §1.1.1 (in `specs/0064_TUI_EMBODIED_PANE_LAYOUT_AND_AT_A_GLANCE_PANELS_SPEC.md`) names the region set; §4.1 requires priority regions for urgent facts and peripheral placement for background info; §8 requires deterministic ordering of panes. The visual region set is coarser than `FocusedPane` (e.g. Place/Situation groups Place+Exits+Doors+Containers+Items; Self/Body/Routine maps to `Status`; Recent maps to `ActorKnownInterval`) and adds two regions with no `FocusedPane` member: the header/mode bar and the input-hints footer.
3. Shared boundary under audit: the `screen` module public surface (`crates/tracewake-tui/src/screen/mod.rs`), which re-exports `EmbodiedScreenModel`, `FocusedPane`, `RenderOptions`, `TerminalSize`. This ticket adds a `pane_layout` module and re-export; sibling tickets 002 (bindings), 003 (buffer render), 004 (responsive) consume its region model, so the layout API is the contract they depend on.
4. Constitutional invariant under audit: **INV-069 — The TUI must not implement simulation rules.** The region model only arranges already-projected, actor-filtered panes; it reads no hidden truth, maintains no quest state, and decides no world legality. **INV-018 — Deterministic replay is foundational** governs §8's deterministic pane ordering: region order and intra-region pane order must be a pure, stable function of the model, not incidental map iteration.
5. Deterministic-ordering surface: region assignment and ordering must be deterministic (INV-018 / spec §8). This module holds no actor-knowledge filter — the `EmbodiedScreenModel` it consumes is already actor-filtered upstream (0061); the layout re-reads no truth and re-filters nothing, so it introduces no epistemic-leakage path (INV-024). It is substrate for the actor-known-only assertion enforced in 0064TUIEMBPAN-002 and the deterministic-replay witness in 0064TUIEMBPAN-005.

## Architecture Check

1. A region-grouping layer above the existing per-pane data keeps the stable-position layout concern separate from both the per-pane content extraction (002) and the terminal backend (003), so the layout can be unit-tested and snapshotted without a `ratatui` dependency and reused unchanged by both the buffer and text render paths. Encoding regions as data (a region enum + ordered pane membership) rather than as inline render code keeps priority ordering inspectable and testable per §4.1.
2. No backwards-compatibility aliasing or shims: `FocusedPane` and the pane sub-structs are reused as-is; the new module adds a layer, it does not fork or duplicate the pane taxonomy.

## Verification Layers

1. INV-069 (TUI implements no rules) -> codebase grep-proof: `pane_layout.rs` imports only `crate::screen::model` types and `std`; no `tracewake_core` truth-query or world-mutation symbol.
2. INV-018 (deterministic pane ordering, spec §8) -> unit test: the same `EmbodiedScreenModel` yields byte-identical region ordering across repeated calls; ordering is derived from a fixed region sequence, not map iteration.
3. Region-coverage invariant -> unit test: every `FocusedPane` variant and every `EmbodiedScreenModel` sub-pane maps into exactly one region (exhaustiveness), so no pane is silently dropped from the layout.

## What to Change

### 1. Add the layout region model

Add `crates/tracewake-tui/src/screen/pane_layout.rs`:

- A `PaneRegion` enum for the spec §1.1.1 regions: `HeaderModeBar`, `PlaceSituation`, `SelfBodyRoutine`, `CoPresentActors`, `ActionsAffordances`, `DetailsWhyNot`, `NotebookLeads`, `RecentChanges`, `InputHintsFooter`.
- A pure function returning the deterministic, priority-ordered region sequence, and a mapping from each `PaneRegion` to the ordered set of `EmbodiedScreenModel` sub-panes / `FocusedPane` members it composes (Place/Situation ← Place, Exits, Doors, Containers, Items; Self/Body/Routine ← Status; Co-present ← Actors; Actions ← Actions; Details/Why-not ← WhyNot; Notebook ← Notebook; Recent ← ActorKnownInterval; header/footer are region-only).
- A borrow over `&EmbodiedScreenModel` that yields, per region, references to the sub-panes it contains, in deterministic order — no cloning of pane payloads, no content formatting (that is 002).

### 2. Register the module

In `crates/tracewake-tui/src/screen/mod.rs`, add `pub mod pane_layout;` and re-export the region enum and the layout entry function.

## Files to Touch

- `crates/tracewake-tui/src/screen/pane_layout.rs` (new)
- `crates/tracewake-tui/src/screen/mod.rs` (modify)

## Out of Scope

- Per-pane content/line extraction and actor-safe formatting (0064TUIEMBPAN-002).
- `ratatui` buffer rendering and any new dependency (0064TUIEMBPAN-003).
- Responsive collapse, truncation markers, minimum-size floor (0064TUIEMBPAN-004).
- Any change to core view models or the pane data structs (spec 0064 §1.2 — no new core field).
- Live input, focus movement, or a `crossterm` event loop (Specs 0065/0062).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui pane_layout` — region ordering is deterministic and priority-ordered (urgent regions Details/Why-not, Actions, Self needs precede peripheral regions).
2. Exhaustiveness test: every `FocusedPane` variant and every `EmbodiedScreenModel` sub-pane is claimed by exactly one region (no unmapped pane, no double-mapping).
3. `cargo build -p tracewake-tui --all-targets --locked` — the new module compiles and is exported.

### Invariants

1. The layout layer reads only actor-filtered `EmbodiedScreenModel` data and adds no truth query or world mutation (INV-069).
2. Region and intra-region ordering is a pure function of the model — identical inputs give identical ordering (INV-018 / spec §8).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/screen/pane_layout.rs` (unit tests in `#[cfg(test)]`) — region ordering determinism, priority ordering, and pane→region exhaustiveness.

### Commands

1. `cargo test -p tracewake-tui pane_layout`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`

## Outcome

Completed: 2026-07-02

Implemented the TUI-only pane layout region model in
`crates/tracewake-tui/src/screen/pane_layout.rs` and exported it from
`crates/tracewake-tui/src/screen/mod.rs`. The new model provides the closed
Spec 0064 region taxonomy, a deterministic priority-ordered region sequence,
borrowed references to the existing `EmbodiedScreenModel` sub-panes, and a
one-to-one `FocusedPane` to region mapping. Header and footer remain
region-only as planned.

Verification run:

- `cargo fmt --all --check` initially reported formatting drift in the new file.
- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p tracewake-tui pane_layout`
- `cargo build -p tracewake-tui --all-targets --locked`

No core/content dependencies or view-model fields were added. No
backwards-compatibility shims or alias paths were introduced. The broader
workspace test/clippy gates remain for later tickets and final Spec 0064
closeout.
