# 0005EMBVIEWFIX-002: Label take affordances by actual item source (place vs container)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (`view_models.rs` `VisibleItem`, `projections.rs` `visible item` mapping + `semantic_actions`). No event/schema/replay changes; no change to action behavior or `target_ids`.
**Deps**: `0005EMBVIEWFIX-001` (both edit the `visible_items` → `semantic_actions` path in `projections.rs`; sequence after to avoid a colliding diff).

## Problem

The embodied action menu labels every take affordance `take.item.<id>.from.place`, regardless of where the item actually is. When an item is inside an open container, the generated semantic action ID still says `.from.place` even though the item's location is `Location::InContainer(<container>)`. The take itself succeeds — `build_take_event` dispatches on the item's real location and emits `ItemRemovedFromContainer` — but the stable semantic action ID, which is the action's *identity* (`README.md`: "The numbered menu position is never an action identity. Numeric input resolves through the current view to the stable semantic action ID"), is inaccurate and misleading.

The intended honest form already exists in a test: `crates/tracewake-core/src/ids.rs:228` round-trips `take.item.coin_stack_01.from.strongbox_tomas` — i.e. `take.item.<id>.from.<container_id>` for a container-sourced take. The generator (`projections.rs:296`) never produces that form. This is an action-menu legibility defect (`INV-044` affordances are conditional and honest; `INV-070` mode-aware legible action surface).

## Assumption Reassessment (2026-06-07)

1. `crates/tracewake-core/src/projections.rs:294-303` `semantic_actions` hard-codes `format!("take.item.{}.from.place", item.item_id.as_str())` for every `VisibleItem`. `crates/tracewake-core/src/view_models.rs:92` `VisibleItem` currently carries only `item_id` and `portable`, so the generator has no way to know the item's source. Confirmed: source must be threaded into `VisibleItem` (or into the affordance step) to label correctly.
2. `crates/tracewake-core/src/ids.rs:226-238` already encodes the intended container-sourced ID shape `take.item.<id>.from.<container_id>` (literal `take.item.coin_stack_01.from.strongbox_tomas`). The floor-sourced shape stays `take.item.<id>.from.place`. This ticket aligns the generator with that existing convention.
3. `crates/tracewake-core/src/actions/defs/takeplace.rs` `build_take_event` resolves the source from the item's actual `Location` (place vs container) and emits `ItemTakenFromPlace` or `ItemRemovedFromContainer`; the proposal's `target_ids` is `[item_id]` only (`crates/tracewake-tui/src/app.rs` proposal construction). Confirmed: the semantic ID string is a label/identity; correcting it does not change `target_ids`, dispatch, or the committed event. Behavior is identical; only the ID/label becomes honest.
4. Invariant under audit: `INV-044` (affordances are conditional, not menu decoration) and `INV-070` (legible, mode-aware action surface). No fail-closed validation, actor-knowledge filtering, or deterministic-replay surface is touched — the change is view-model labeling only.
5. Schema extension under audit: `VisibleItem` (view-model projection) gains a source descriptor (e.g. an enum over place / container id). Additive; the sole struct-literal construction site is `crates/tracewake-core/src/projections.rs:83`. The TUI renderer reads `VisibleItem` for the `Items:` list (`crates/tracewake-tui/src/render.rs:58-65`) and is unaffected by an added field it does not print.
6. Adjacent-contradiction classification: the carried-vs-place conflation and the missing possessions surface are handled by `0005EMBVIEWFIX-001` (required predecessor), not here. No new contradiction is introduced.

## Architecture Check

1. Putting the item's source on `VisibleItem` keeps the affordance generator a pure function of the view model: the projection that decides what is visible (and from where) already knows each item's location, so it records the source once and the menu builder reads it. This is cleaner than re-deriving location inside `semantic_actions` (which would require passing raw `containers`/`items` maps back into a presentation step) and matches the already-authored ID convention in `ids.rs`.
2. No backwards-compatibility aliasing/shims. The `.from.place` form is retained only for genuinely floor-sourced items; container-sourced items get the correct `.from.<container_id>` form. No dual-ID or alias path is created.

## Verification Layers

1. `INV-044` (affordances conditional/honest) -> codebase grep-proof + unit test: a container-sourced visible item yields `take.item.<id>.from.<container_id>`; a floor-sourced item yields `take.item.<id>.from.place`.
2. `INV-070` (legible action surface) -> manual review: the menu ID names the real source the take will act on.

## What to Change

### 1. Record item source on `VisibleItem` (`crates/tracewake-core/src/view_models.rs`)

Add a source descriptor to `VisibleItem` — an enum such as `ItemSource { Place, Container(ContainerId) }` (carried items are not `VisibleItem`s in the place list after `0005EMBVIEWFIX-001`, so only place/container sources apply). Keep `item_id` and `portable`.

### 2. Populate the source in the projection (`crates/tracewake-core/src/projections.rs`)

When mapping `visible.visible_items` into `VisibleItem` (around line 79-88), resolve each item's `Location` from `state.items`: `Location::AtPlace` → `ItemSource::Place`; `Location::InContainer(c)` → `ItemSource::Container(c)`. (These are the only two sources that reach the place's reachable-items list.)

### 3. Generate the source-accurate take ID (`crates/tracewake-core/src/projections.rs`)

In `semantic_actions`, build the take ID from the item's source: `take.item.<id>.from.place` for `ItemSource::Place`, `take.item.<id>.from.<container_id>` for `ItemSource::Container`. Leave `action_id` (`take`), `target_ids` (`[item_id]`), the human label, and portability gating unchanged.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)

## Out of Scope

- Changing `target_ids`, the committed take events, or `build_take_event` dispatch — behavior is unchanged.
- Rendering the source in the `Items:` list text (renderer display is unchanged unless trivially required to compile).
- The carried/possessions split and the phantom-affordance fix — `0005EMBVIEWFIX-001`.

## Acceptance Criteria

### Tests That Must Pass

1. New `tracewake-core` unit test: a visible item inside an open container yields a `take.item.<id>.from.<container_id>` affordance (matching the `ids.rs` convention) and not `.from.place`.
2. New `tracewake-core` unit test: a floor item at the current place yields `take.item.<id>.from.place`.
3. Existing `crates/tracewake-core/src/ids.rs` round-trip test for `take.item.coin_stack_01.from.strongbox_tomas` continues to pass (format consistency).
4. Full pipeline: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. A take affordance's semantic ID names the actual source the take will resolve against (`place` vs the holding container).
2. The committed take event and `target_ids` are unchanged by the relabeling; only the action identity string differs.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (tests module) — container-sourced and floor-sourced take ID generation.

### Commands

1. `cargo test -p tracewake-core projections:: && cargo test -p tracewake-core ids::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
