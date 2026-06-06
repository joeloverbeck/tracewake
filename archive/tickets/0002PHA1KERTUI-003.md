# 0002PHA1KERTUI-003: Entity/component state model and single-source location

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `state` and `location` modules to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-002

## Problem

The kernel needs an explicit, minimal authoritative state model before events can mutate it. Spec 0002 §12 defines the entity kinds (`Actor`, `Place`, `Door`, `Container`, `Item`, and inert `InstitutionPlaceholder`/`RecordPlaceholder`), the component records (`EntityHeader`, `ActorBody`, `PlaceState`, `DoorState`, `ContainerState`, `ItemState`, `OwnershipCustody`, `Location`, `ControllerBinding`), and the single-source location model (§12.3): every physical item has exactly one holder — `AtPlace`, `InContainer`, or `CarriedBy` — and may never be simultaneously in two. §12.4 defines Phase 1 visibility/reachability. This is the data substrate every action, event, projection, and checksum builds on.

## Assumption Reassessment (2026-06-06)

1. No `state`/`location` module exists; this registers `pub mod state;` and `pub mod location;` in `crates/tracewake-core/src/lib.rs` (created by 001), importing the ID newtypes from ticket 002 (`crates/tracewake-core/src/ids.rs`).
2. The component fields are `specs/0002_…_SPEC.md` §12.2 and the §12.5 ownership/custody/access distinctions; the single-source holder variants are §12.3; visibility is §12.4. The spec's §12.5 note (added during reassessment) is binding: current physical holder lives in the `Location` model, **not** as an `OwnershipCustody.possessor` field — `OwnershipCustody` carries only legal/social facts (owner, custodian, permitted access, expected-location stub). The execution component sketch `docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md` shows a bundled `possessor`; Spec 0002 §12.5 deliberately diverges to preserve single-source location.
3. Shared boundary under audit: the component-record schema imported by `events` (apply), `actions` (preconditions), `projections`/`view_models`, `checksum`, and `content` (schema mirrors these). Field names are fixed here.
4. Invariant motivating this ticket: INV-055 (ownership, custody, access, control, proof, and belief are distinct states) — the records keep these as separate fields without collapsing them into physical truth.
5. Deterministic-replay / no-leak surface: the `state` records are the checksum input (ticket 007) and the projection source (ticket 012). This ticket introduces no nondeterminism (ordered collections for `carried`/`contents`, no hash-ordered iteration) and no leakage path (§12.4 visibility is a read-time filter applied by projections, not stored on the actor). Enforcement lands in 007/012; this ticket guarantees the data model supports them.

## Architecture Check

1. Typed records with a single `Location` enum holder (rather than a broad ECS or a `possessor`-bearing ownership record) make double-location unrepresentable-by-construction and keep the diff reviewable (§12 / §20.1 "prefer typed records and explicit maps over a broad ECS"). One authoritative holder per item means there is exactly one place a move can change.
2. No backwards-compatibility shims: greenfield; no `possessor` alias is introduced (the divergence from exec-05 is deliberate and documented in §12.5).

## Verification Layers

1. Single-location invariant (§12.3) -> codebase grep-proof + unit test: `Location` is a sum type; an item's holder is one variant; a constructor/validator rejects duplicate holders.
2. Ownership/custody distinctness (INV-055) -> manual review + schema check: `OwnershipCustody` has owner/custodian/permitted-access/expected-location-stub and no `possessor`.
3. Visibility firewall (INV-024 substrate) -> manual review: §12.4 visibility is computed from place locality, with no stored "actor knows X" field that debug or controller binding could write.

## What to Change

### 1. Entity kinds and headers

Add `crates/tracewake-core/src/state.rs` with the entity-kind enum (including the two inert placeholders) and `EntityHeader` (stable ID, kind, display label, validation-restricted tags).

### 2. Component records

Add `ActorBody`, `PlaceState`, `DoorState`, `ContainerState`, `ItemState`, `OwnershipCustody`, `ControllerBinding` per §12.2, using deterministically-ordered collections for `carried` items and container `contents`.

### 3. Location model

Add `crates/tracewake-core/src/location.rs` with the `Location` holder enum (`AtPlace`/`InContainer`/`CarriedBy`), the single-source invariant check, and the §12.4 visibility/reachability query (current place, connected doors, open-container contents, visible items, carried items, co-located actors). Phase 1 keeps containers fixed at places by default; if portable containers are modeled, include the cycle check (§12.3).

### 4. Registration

Add `pub mod state;` and `pub mod location;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (new)
- `crates/tracewake-core/src/location.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod state; pub mod location;`; file created by ticket 001)

## Out of Scope

- Event-driven mutation of state (ticket 006 — state changes only via event application).
- Content/fixture schemas that mirror these records (ticket 017).
- Any belief/epistemic field (Phase 2); `OwnershipCustody` stays inert data here.

## Acceptance Criteria

### Tests That Must Pass

1. An item can be constructed `AtPlace`, `InContainer`, or `CarriedBy`, and the holder is exactly one variant.
2. A constructor/validator rejects an item asserted into two holders simultaneously.
3. The §12.4 visibility query excludes closed-opaque-container contents and items in other places.

### Invariants

1. No representable state has an item in two locations.
2. `OwnershipCustody` carries no current-physical-holder field; holder is read only from `Location`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/location.rs` (unit tests) — single-holder construction, double-location rejection, visibility filtering.
2. `crates/tracewake-core/src/state.rs` (unit tests) — record construction with ordered collections.

### Commands

1. `cargo test -p tracewake-core state location`
2. `cargo build --workspace`
3. Core-crate unit scope is correct: state records have no cross-crate behavior until events apply to them (ticket 006).

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::state` with typed entity headers, entity kinds, actor/place/door/container/item records, ownership/custody placeholders, controller binding metadata, and deterministic ordered collections.
- Added `tracewake_core::location` with the single-source `Location` holder enum, validation for exactly one asserted holder, and a read-only Phase 1 visibility query.
- Registered `pub mod state;` and `pub mod location;` in the core crate.

Deviations from original plan:
- The documented combined Cargo filter `cargo test -p tracewake-core state location` is not accepted by Cargo as written, so verification used separate `state` and `location` filters.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core state` passed: 3 tests.
- `cargo test -p tracewake-core location` passed: 3 tests.
- `cargo build --workspace` passed.
