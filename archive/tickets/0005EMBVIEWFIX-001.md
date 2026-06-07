# 0005EMBVIEWFIX-001: Separate carried inventory from place-reachable items in the embodied view

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`location.rs` `visible_locality`, `view_models.rs` `EmbodiedViewModel`, `projections.rs` `build_embodied_view_model`), `tracewake-tui` (`render.rs` embodied renderer). No event/schema/replay changes.
**Deps**: None.

## Problem

In the TUI, after an actor takes an item from a place (or a container), the item is still listed under the place's `Items:` section and the menu still offers `take.item.<id>.from.place` for it. Selecting that phantom "Take" is then correctly rejected by the validator with `Why-not: That item is not at the source.`. Concrete repro (from `cargo run -p tracewake-tui`, bound `actor_elena`, `strongbox_001`): open strongbox → take `coin_stack_01` (`Accepted`) → the next view still shows `coin_stack_01` under `Items:` and offers both `take.item.coin_stack_01.from.place` and `place.item.coin_stack_01.at.place`; taking again yields `Why-not: That item is not at the source.`

Root cause: `visible_locality` builds a single `visible_items` set that conflates **items reachable in the place** (floor + open-container contents) with **items the actor is carrying**. That conflated set is rendered as `Items:` and is also the source for `take` affordance generation, so a held item is shown as if it lay on the floor and is offered for taking.

This is a legibility/affordance defect on the embodied view-model surface. It offers an action whose precondition cannot hold (`INV-044` — affordances are conditional, not menu decoration; `INV-070` — why-not is for genuinely blocked attempts, not for actions that should never have been offered) and blurs custody/physical-control display (`INV-055`). Once the conflation is removed, held items would vanish from the view entirely, so this ticket must also add the missing possessions surface required by the embodied product surface (`INV-065`/`INV-066`, foundation doc `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`: "seeing current actor body state, needs, **possessions** …").

## Assumption Reassessment (2026-06-07)

1. `crates/tracewake-core/src/location.rs:60-120` `visible_locality` seeds `let mut visible_items = actor.carried_item_ids.clone();` (line 70), then in the item loop re-inserts carried items via the `Location::CarriedBy(actor_id) if actor_id == &actor.actor_id` arm (lines 99-101), and separately exposes a correct `carried_items` field (line 71/117). Floor items (`Location::AtPlace == current_place_id`, lines 96-97) and open/visible-when-closed container contents (lines 80-92) are also added. Confirmed: the carried-item conflation is the only defect; floor/container population is correct.
2. `crates/tracewake-core/src/projections.rs:79-104` copies `visible.visible_items` into the view model's `visible_items` (rendered as `Items:`) and passes `&visible.carried_items.into_iter().collect::<Vec<_>>()` to `semantic_actions`. `semantic_actions` (`projections.rs:294-323`) generates `take.item.<id>.from.place` per `visible_items` entry and `place.item.<id>.at.place` per `carried_items` entry — so once `visible_items` no longer contains carried items, the phantom `take` disappears and the existing `place` affordance is unaffected.
3. View-model schema extension under audit: `crates/tracewake-core/src/view_models.rs:16-33` `EmbodiedViewModel` has no carried/possessions field. This ticket adds one (additive). Consumers that construct it by struct literal and must be updated: `crates/tracewake-core/src/projections.rs:107` (production), `crates/tracewake-tui/src/render.rs:173` (test), `crates/tracewake-tui/src/input.rs:138` (test). `VisibleItem` (`view_models.rs:92`) is reused for the new field; no new item type is introduced.
4. Invariants under audit (restate before trusting the narrative): `INV-044` affordances are conditional, not menu decoration; `INV-055` ownership/custody/access/control/proof/belief are distinct (single-source item location must not be displayed as two simultaneous locations); `INV-065`/`INV-066` the TUI is a primary, possessions-legible product interface and every runnable phase has a view-model gate; `INV-067` embodied mode shows actor-known reality. The fix is view-model-only and does not touch fail-closed validation, actor-knowledge epistemic filtering, or deterministic replay.
5. This ticket extends a view-model projection struct (`EmbodiedViewModel`). The extension is a new always-populated `carried_items: Vec<VisibleItem>` field; all three struct-literal consumers (item 3) are updated in the same diff, so no stale consumer remains.
6. Adjacent contradiction classification: the inaccurate `take.item.<id>.from.place` label for container-sourced items (it ignores the real source) is a **separate bug** tracked by `0005EMBVIEWFIX-002`, not fixed here. The intentional zero-duration tick behavior (tick stays at `0` across `open`/`take`) was investigated and confirmed **doctrinal** for Phase 1/2A (Phase 1 spec "prefers immediate actions"; cost/duration pipeline slot is deliberately inert; durations are owned by Phase 3, `docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md`) — explicitly out of scope and not a defect.

## Architecture Check

1. The cleanest fix preserves the existing single-source-of-truth model (`item.location` is the only authority) and the already-present split in `VisibleLocality` (`visible_items` vs `carried_items`). We stop polluting `visible_items` with carried items rather than adding a downstream filter, so reachable-items and possessions become two honest, non-overlapping projections of the same authoritative location state. The view model then surfaces possessions through a dedicated field instead of leaking them through the place's item list. Alternatives (filtering carried out of `visible_items` only at render time, or de-duplicating affordances post-hoc) leave the conflation in the projection and would re-leak through any other `visible_items` consumer.
2. No backwards-compatibility aliasing/shims introduced. `visible_items` keeps its name and meaning (reachable, not-carried items); the new `carried_items` field is added directly with all consumers updated in the same change.

## Verification Layers

1. `INV-044` (affordances conditional, not menu decoration) -> codebase grep-proof + replay/golden-fixture check: a carried item produces no `take.item.<id>.from.place` entry; new unit/regression test asserts absence.
2. `INV-055` (custody/control distinct, single-source location) -> manual review + unit test: an item is listed under exactly one of `Items:` / possessions for any state.
3. `INV-065`/`INV-066` (TUI possessions surface; view-model gate) -> manual review + `tracewake-tui` renderer test asserting the possessions section renders carried items.
4. `INV-067` (actor-known reality; no leakage) -> invariants alignment check: change is view-model-only over already-actor-filtered state; no new truth source is read.

## What to Change

### 1. Stop conflating carried items into `visible_items` (`crates/tracewake-core/src/location.rs`)

In `visible_locality`:
- Initialize `let mut visible_items = BTreeSet::new();` instead of cloning `actor.carried_item_ids` (line 70).
- Remove the `Location::CarriedBy(actor_id) if actor_id == &actor.actor_id => { visible_items.insert(...) }` arm (lines 99-101) so carried items are no longer re-added.
- Leave floor-item and container-content population unchanged, and leave the separate `carried_items` field unchanged.

Net effect: `visible_items` = items at the current place's floor plus contents of open/visible containers, excluding anything the actor carries; `carried_items` = the actor's carried items.

### 2. Add a possessions field to the embodied view model (`crates/tracewake-core/src/view_models.rs`)

Add `pub carried_items: Vec<VisibleItem>,` to `EmbodiedViewModel`, reusing the existing `VisibleItem` type.

### 3. Populate possessions in the projection (`crates/tracewake-core/src/projections.rs`)

In `build_embodied_view_model`:
- Build a sorted `carried_items: Vec<VisibleItem>` from `visible.carried_items` by looking each id up in `state.items` (same mapping used for `visible_items`).
- Pass the carried item ids to `semantic_actions` for the `place.item.<id>.at.place` affordances (unchanged behavior; reuse the computed set rather than re-collecting).
- Set the new `carried_items` field on the returned `EmbodiedViewModel`.

### 4. Render possessions in the TUI (`crates/tracewake-tui/src/render.rs`)

In `render_embodied_view`, add an `Inventory:` section after `Items:` that lists `view.carried_items` in the same `- <id> portable=<bool>` shape. Update the two test struct literals (`render.rs:173`, `input.rs:138`) to include the new field.

## Files to Touch

- `crates/tracewake-core/src/location.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify — test struct literal only)

## Out of Scope

- Correcting the take affordance source label for container items (`take.item.<id>.from.place` vs `.from.<container>`) — that is `0005EMBVIEWFIX-002`.
- Any change to tick/duration behavior — confirmed doctrinal (Phase 3 owns durations).
- Ownership/access/proof distinctions beyond physical custody display; no new claim/belief surfaces.
- Inspect/place affordances for carried items beyond the already-existing `place.item.<id>.at.place`.

## Acceptance Criteria

### Tests That Must Pass

1. New `tracewake-core` unit test on `visible_locality`: given an actor carrying item A with floor item B at the current place, `visible_items` contains B and not A, and `carried_items` contains A and not B.
2. New `tracewake-core` regression test on `build_embodied_view_model` reproducing the bug: after an item moves to `Location::CarriedBy(actor)`, the view's `visible_items` excludes it, `carried_items` includes it, `semantic_actions` contains no `take.item.<id>.from.place` for it, and contains `place.item.<id>.at.place` for it.
3. New `tracewake-tui` renderer test: `render_embodied_view` output contains an `Inventory:` section listing a carried item, and that item does not appear under `Items:`.
4. Full pipeline: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. An item is never simultaneously presented as reachable-in-place and carried; `item.location` remains the single source of truth for both projections.
2. No `take` affordance is emitted for an item the actor carries; no `place` affordance is emitted for an item not carried.
3. The change reads only already-actor-filtered physical state — no hidden-truth source is introduced.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/location.rs` (tests module) — carried item excluded from `visible_items`, present in `carried_items`.
2. `crates/tracewake-core/src/projections.rs` (tests module) — view-model regression test for the take-then-still-takeable bug and the possessions field population.
3. `crates/tracewake-tui/src/render.rs` (tests module) — `Inventory:` section rendering and absence of carried item from `Items:`.

### Commands

1. `cargo test -p tracewake-core location:: && cargo test -p tracewake-core projections:: && cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-07

The embodied locality projection now keeps place-reachable items separate from
actor-carried items. `EmbodiedViewModel` carries a dedicated `carried_items`
inventory surface, and the TUI renderer prints it under `Inventory:` instead of
leaking possessions through `Items:`.

The implementation followed the planned view-model-only path. No event schema,
replay, action validation, or source-label behavior changed; source-accurate
container take IDs remain scoped to `0005EMBVIEWFIX-002`.

Verification:

- `cargo test -p tracewake-core location::`
- `cargo test -p tracewake-core projections::`
- `cargo test -p tracewake-tui`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
