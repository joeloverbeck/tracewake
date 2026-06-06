# 0002PHA1KERTUI-010: take / place action definitions

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — registers the `take`, `place` action definitions; emits `ItemRemovedFromContainer`, `ItemTakenFromPlace`, `ItemPlacedInContainer`, `ItemPlacedInPlace`.
**Deps**: 0002PHA1KERTUI-008

## Problem

Phase 1 must support take/place of physical items where physically valid (Spec 0002 §4.1 area 10, §14.4). Items move between an actor's carry, a container, and a place through validated actions emitting the four item-movement world events (§11.3). These verbs are the heart of `strongbox_001`, `container_item_move_001`, and `replay_item_location_001` — proving item location derives from initial fixture plus ordered events, never a mutable-current-state accident.

## Assumption Reassessment (2026-06-06)

1. The registry/pipeline/reason-codes exist from ticket 008; the four item events exist from ticket 005; `Location` single-source holder and `ItemState` portability live in ticket 003. This ticket adds a `take`/`place` definition file and registers it, modifying `actions/registry.rs` (008).
2. Semantics and event mapping are `specs/0002_…_SPEC.md` §14.4 (`take` → `ItemRemovedFromContainer`/`ItemTakenFromPlace`; `place` → `ItemPlacedInContainer`/`ItemPlacedInPlace`); reason codes `item_not_portable`, `item_not_at_source`, `item_not_carried`, `carry_capacity_exceeded`, `destination_not_open`, `container_closed`, `target_not_reachable` are §14.5.
3. Shared boundary under audit: the single-source `Location` transition — a take/place changes exactly one holder. The applier (006) enforces old-state precondition; this ticket constructs the matching event.
4. Invariant motivating this ticket: INV-055 (ownership, custody, access, control, proof, belief are distinct) — a `take` changes physical holder (`Location`) only; it does not change `OwnershipCustody` (owner/custodian stay put), so taking is not a legal transfer.
5. Deterministic-replay surface: item location after a run must equal location rebuilt from fixture + ordered item events (the `replay_item_location_001` contract, §18.6). Each accepted take/place emits one item event applied via ticket 006; no nondeterminism. Enforcement is ticket 013/022.

## Architecture Check

1. `take`/`place` as registry definitions that produce a single `Location`-changing event reuse the single-source holder model, so item movement is one event = one holder change — making replay-derived location exact. Mutating both `OwnershipCustody` and `Location` on a take (the alternative) would conflate possession with ownership (violates INV-055).
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Single-holder transition (§12.3) -> unit test: after `take`, the item is `CarriedBy` the actor and no longer in its source; the inverse for `place`.
2. Ownership unchanged on take (INV-055) -> unit test: `take` leaves `OwnershipCustody` fields unchanged.
3. Eventful + replayable (INV-009/018) -> replay/golden check: an accepted `take`/`place` emits exactly one item event; replay reproduces the final holder.
4. Conditional affordance (INV-044) -> unit test: `take` from a closed container rejects (`container_closed`); from the wrong holder rejects (`item_not_at_source`).

## What to Change

### 1. take / place

Add `crates/tracewake-core/src/actions/defs/takeplace.rs`: `take` (item + optional source holder; validate reachability, portability, source presence, carry capacity) emitting `ItemRemovedFromContainer`/`ItemTakenFromPlace`; `place` (item + destination place/container; validate carry, destination open) emitting `ItemPlacedInContainer`/`ItemPlacedInPlace`.

### 2. Registration

Register both in `actions/registry.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/takeplace.rs` (new)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register `take`/`place`; file created by ticket 008)

## Out of Scope

- move/open/close (ticket 009) and inspect/wait (ticket 011).
- Hide-item-as-place-into-non-obvious-container (optional per §14.4; not required for Phase 1 exit — plain `place` covers it).

## Acceptance Criteria

### Tests That Must Pass

1. `take` from a closed container rejects with `container_closed` and mutates nothing; from an open container moves the item to `CarriedBy`.
2. `take` from the wrong source rejects with `item_not_at_source`.
3. `place` into a closed container rejects with `destination_not_open`; into a valid destination emits the matching `ItemPlaced*` event.
4. After `take`, `OwnershipCustody` is unchanged.

### Invariants

1. A take/place changes exactly one `Location` holder via exactly one event.
2. Physical holder change never implies an ownership/custody change.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/takeplace.rs` (unit tests) — holder transition, ownership invariance, reason codes.

### Commands

1. `cargo test -p tracewake-core actions::defs::takeplace`
2. `cargo build --workspace`
3. Unit scope is correct; `replay_item_location_001`'s replay-derived-location proof is exercised in ticket 022.
