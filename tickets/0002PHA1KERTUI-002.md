# 0002PHA1KERTUI-002: Stable ID newtypes and canonical serialization

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds the `ids` module to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-001

## Problem

Determinism is a Phase 1 acceptance gate. Spec 0002 §10.1 requires stable semantic IDs for fixtures, actors, places, exits/edges, doors, containers, items, action definitions, semantic action instances, events, validation reports, controller bindings, content manifests, and schema versions. Display names, array positions, terminal menu indices, and bare generated UUIDs are forbidden as identity. Every other Phase 1 module references these IDs, so the newtypes and their canonical serialization must land before state, events, and actions.

## Assumption Reassessment (2026-06-06)

1. The `ids` module does not exist yet; `crates/tracewake-core/src/lib.rs` was created empty by ticket 001 and this ticket registers `pub mod ids;` there.
2. The ID inventory and rules are `specs/0002_…_SPEC.md` §10.1 and §9.2 (`ids` module responsibility: "comparable and serializable canonically"). UUIDs are acceptable only if derived from stable content path + parent ID + ordinal via a documented deterministic derivation (§10.1).
3. Shared boundary under audit: the ID type vocabulary that `state`, `events`, `actions`, `content`, and `view_models` will all import — a rename here is repo-wide, so the newtype names are fixed now.
4. Invariant motivating this ticket: INV-018 (deterministic replay is foundational) — IDs participate in checksums and event ordering, so their serialized form must be canonical and platform-stable.
5. Deterministic-replay surface: the ID serialization feeds the canonical physical-state checksum (ticket 007) and event ordering keys (ticket 004). This ticket introduces no nondeterminism: IDs are content-stable strings/newtypes with `Ord` by value, never by hash or insertion order. The enforcement (checksum/replay) lands in tickets 007/013; this ticket only guarantees the inputs are canonical.

## Architecture Check

1. Distinct newtypes per entity kind (`ActorId`, `PlaceId`, `DoorId`, `ContainerId`, `ItemId`, `ActionId`, `EventId`, `ValidationReportId`, `FixtureId`, `ContentVersion`, `ControllerId`, …) over a shared inner stable-string type gives compile-time confusion-resistance (an `ItemId` cannot be passed where a `PlaceId` is expected) with one canonical serialization path. A single opaque `Id(String)` would lose that protection.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Determinism (INV-018) -> codebase grep-proof + manual review: newtypes derive `Ord`/`Eq`/`Hash` from the inner value (not a random seed), and serialization is the verbatim stable string.
2. ID-as-identity discipline (§10.1) -> schema validation (later, ticket 018) rejects display-name-as-ID; here, manual review confirms no `From<usize>`/index constructor exists.
3. Single-layer rationale: this is a leaf data-type ticket; the determinism layer is the one load-bearing invariant and is mapped above.

## What to Change

### 1. ID newtypes

Add `crates/tracewake-core/src/ids.rs` with one newtype per §10.1 entity kind wrapping a stable-string inner type, each deriving `Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash`. Provide a checked constructor that validates stable-ID syntax (non-empty, no whitespace, snake/kebab semantic form) and rejects display-name shapes.

### 2. Canonical serialization

Implement canonical serialize/deserialize (the inner stable string verbatim) and a documented deterministic-derivation helper for the UUID-from-(path,parent,ordinal) case allowed by §10.1.

### 3. Registration

Add `pub mod ids;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/ids.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod ids;`; file created by ticket 001)

## Out of Scope

- Any entity records or state that *use* these IDs (ticket 003).
- Event IDs' stream-position derivation semantics (ticket 005 defines how event IDs are minted).

## Acceptance Criteria

### Tests That Must Pass

1. Constructing an ID from a valid stable string round-trips through serialize/deserialize byte-identically.
2. Constructing an ID from a display-name-shaped string (whitespace, empty) fails.
3. Two IDs of different kinds are not interchangeable (compile-fail test or type-level assertion).

### Invariants

1. ID ordering is by stable-string value, never by hash iteration or insertion order.
2. Serialized ID form is platform- and run-stable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/ids.rs` (unit tests) — round-trip, rejection, ordering stability.

### Commands

1. `cargo test -p tracewake-core ids`
2. `cargo build --workspace`
3. A core-crate unit-test scope is correct here because IDs have no cross-crate behavior to exercise yet.
