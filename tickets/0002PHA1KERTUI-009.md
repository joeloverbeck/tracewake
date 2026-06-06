# 0002PHA1KERTUI-009: move / open / close action definitions

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — registers the `move`, `open`, `close` action definitions in `tracewake-core`'s action registry; emits `ActorMoved`, `DoorOpened`, `DoorClosed`, `ContainerOpened`, `ContainerClosed`.
**Deps**: 0002PHA1KERTUI-008

## Problem

Phase 1 must support local movement through validated adjacency and door constraints, and open/close of doors and containers where physically valid (Spec 0002 §4.1 areas 7+9, §14.4). These verbs plug into the shared pipeline (ticket 008) and emit the corresponding `world` events (§11.3) via the ticket-006 applier. They are required by `door_access_001`, `container_item_move_001`, and `strongbox_001`.

## Assumption Reassessment (2026-06-06)

1. The action registry, pipeline, and `ReasonCode` enum exist from ticket 008; the world event kinds (`ActorMoved`, `DoorOpened`/`DoorClosed`, `ContainerOpened`/`ContainerClosed`) exist from ticket 005; door/container state lives in ticket 003. This ticket adds action-definition files and registers them, modifying `actions/registry.rs` (created by 008).
2. The action semantics and event mapping are `specs/0002_…_SPEC.md` §14.4 (`move` → `ActorMoved`; `open`/`close` → `Door*`/`Container*`); the relevant reason codes (`not_adjacent`, `door_closed_blocks_movement`, `door_locked`, `container_locked`, `already_open`, `already_closed`, `target_not_reachable`) are §14.5.
3. Shared boundary under audit: registry entries consuming the ticket-008 `ActionDefinition` contract and the ticket-005 event kinds; both fixed, this ticket only adds conforming definitions.
4. Invariant motivating this ticket: INV-007 (NPC-possible) and INV-044 (affordances are conditional) — these verbs are registry definitions available to any origin under physical preconditions, never player-only.
5. Deterministic-replay surface: each accepted verb emits exactly one world event applied via ticket 006; replay reconstructs door/container/actor state from those events (ticket 013). No nondeterminism introduced — preconditions read ordered state only.

## Architecture Check

1. Each verb is a small `ActionDefinition` (target-kind binding + physical precondition + event constructor) registered against the single registry, reusing the pipeline's validation stages rather than re-implementing checks. Hand-rolled per-verb validation would duplicate rules outside the pipeline (forbidden, §7.1/§20.2).
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Conditional affordance (INV-044) -> unit test: `move` is rejected (`not_adjacent`/`door_closed_blocks_movement`) when adjacency/door state forbids it, accepted when valid.
2. Eventful mutation (INV-009) -> replay/golden check: an accepted `open` emits exactly one `DoorOpened`/`ContainerOpened` world event; replay reproduces the open state.
3. Registry parity (INV-007; §14.7) -> codebase grep-proof: the verbs are registered once in the shared registry, with no human-origin-only variant.

## What to Change

### 1. move

Add `crates/tracewake-core/src/actions/defs/movement.rs`: `move` definition binding destination place (+ optional door/edge), validating adjacency and door-blocks-movement, emitting `ActorMoved`.

### 2. open / close

Add `crates/tracewake-core/src/actions/defs/openclose.rs`: `open`/`close` for doors and containers, validating reachability and current open/closed/locked state, emitting `Door*`/`Container*` events.

### 3. Registration

Register all three in `actions/registry.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/movement.rs` (new)
- `crates/tracewake-core/src/actions/defs/openclose.rs` (new)
- `crates/tracewake-core/src/actions/mod.rs` (modify — add `defs` submodule; file created by ticket 008)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register `move`/`open`/`close`; file created by ticket 008)

## Out of Scope

- take/place (ticket 010) and inspect/wait (ticket 011).
- Lock/unlock-with-key (optional per §14.4 / exec-05; not required for Phase 1 exit).

## Acceptance Criteria

### Tests That Must Pass

1. `move` to a non-adjacent place rejects with `not_adjacent`; `move` through a closed blocking door rejects with `door_closed_blocks_movement`; `move` to a valid adjacent place emits `ActorMoved`.
2. `open` on a closed reachable container emits `ContainerOpened`; `open` on an already-open target rejects with `already_open`; `open` on a locked target rejects with `*_locked`.
3. Replay of an open-then-move sequence reproduces the same door/container/actor state.

### Invariants

1. These verbs are available to any origin under identical preconditions.
2. Each accepted verb emits exactly one world event; rejections emit no world event.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/movement.rs` (unit tests) — adjacency/door rejection + accept.
2. `crates/tracewake-core/src/actions/defs/openclose.rs` (unit tests) — open/close state transitions + reason codes.

### Commands

1. `cargo test -p tracewake-core actions::defs`
2. `cargo build --workspace`
3. Unit scope is correct here; the end-to-end `door_access_001` scenario is exercised in ticket 022.
