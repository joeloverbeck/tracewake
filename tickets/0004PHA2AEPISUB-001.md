# 0004PHA2AEPISUB-001: Epistemic stable IDs and typed proposition model

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — creates the `epistemics` module in `tracewake-core` and extends `ids.rs` with epistemic ID newtypes.
**Deps**: None

## Problem

Phase 2A makes belief precede truth, and the spec requires typed propositions as the epistemic currency rather than stringly global flags (Spec 0004 §8.3; `INV-021`). Every later epistemic structure — observations, beliefs, contradictions, events, the content seed schema — references a `Proposition` value and the epistemic ID vocabulary, so the proposition model and its stable IDs must land first and be canonically serializable and comparable for contradiction checks. Spec §8.2 requires stable `PropositionId`, `ObservationId`, `BeliefId`, `ContradictionId`, and an `EpistemicProjectionVersion` marker following the existing stable-ID discipline.

## Assumption Reassessment (2026-06-06)

1. The `epistemics` module does not exist yet: `find crates -name '*.rs'` shows no `crates/tracewake-core/src/epistemics/`, and `crates/tracewake-core/src/lib.rs` is the module-declaration hub this ticket adds `pub mod epistemics;` to.
2. `crates/tracewake-core/src/ids.rs` already defines a `stable_id_type!` macro and newtypes (`ActorId`, `ContainerId`, `ItemId`, …) over a shared `StableId` inner type with by-value `Ord` and checked construction (`ids.rs:39`, `ids.rs:78`, `ids.rs:126`); this ticket follows that exact discipline for the new IDs rather than inventing a parallel one.
3. Shared boundary under audit: the `Proposition` enum and the epistemic ID newtypes form the vocabulary that tickets 002 (records), 004 (events), and 012 (content schema) will all import — a rename here is repo-wide, so the variant set and ID names are fixed by this ticket. Spec §8.3 fixes the minimum variant list (`ItemLocatedInContainer`, `ItemLocatedAtPlace`, `ItemCarriedByActor`, `ContainerContentsObserved`, `ItemMissingFromExpectedLocation`, `SoundHeardNearPlace`, `PossibleMovementNearPlace`, `ActorWasNearPlace`).
4. Invariant motivating this ticket: `INV-021` (typed claims/propositions are the epistemic currency) and `INV-022` (raw prose is not authoritative state) — the proposition must be a structured enum, renderable into text without prose ever being the authoritative content.
5. Deterministic-replay surface (substrate-only): proposition and ID serialization feed the canonical epistemic projection checksum and replay ordering. This ticket introduces no nondeterminism — IDs are content-stable strings with by-value `Ord`, and `Proposition` derives equality/ordering from its structured fields, never from hash iteration. The enforcing surface (deterministic epistemic rebuild) lands in ticket 005; this ticket only guarantees the inputs are canonical.

## Architecture Check

1. A structured `Proposition` enum (one variant per claim family) gives compile-time exhaustiveness for contradiction comparison and rendering, where a `String`-keyed global flag would lose typing and silently admit prose-as-truth (`INV-022`). Distinct ID newtypes over the shared `StableId` inner type give the same confusion-resistance the existing `ids.rs` newtypes provide.
2. No backwards-compatibility shims: the `epistemics` module is greenfield, and the new IDs extend the existing macro without aliasing.

## Verification Layers

1. Typed-proposition currency (`INV-021`/`INV-022`) -> invariants alignment check + manual review: `Proposition` is a non-string enum; a `render`/`Display` path exists but no constructor accepts free prose as authoritative content.
2. Determinism (`INV-018`) -> codebase grep-proof + manual review: new IDs use `stable_id_type!`; `Proposition` derives `Ord`/`Eq` from fields, with a round-trip unit test proving byte-identical canonical serialization.
3. Single-layer rationale: this is a leaf data-type ticket; the two load-bearing invariants are mapped above and have no cross-crate behavior to exercise yet.

## What to Change

### 1. Epistemic ID newtypes

Extend `crates/tracewake-core/src/ids.rs` using the existing `stable_id_type!` macro to add `PropositionId`, `ObservationId`, `BeliefId`, and `ContradictionId`, plus an `EpistemicProjectionVersion` schema marker (mirroring the existing content/schema version markers). IDs must be deterministic, non-display, whitespace-free, and orderable by value.

### 2. Typed proposition model

Add `crates/tracewake-core/src/epistemics/proposition.rs` with a `Proposition` enum carrying the §8.3 minimum variants. `expected_location` references reuse the existing `Location` type (container/place sufficient for Phase 2A). Provide: canonical serialization, structural equality, a contradiction-comparison helper (e.g. `contradicts(&self, &Self) -> bool`), a human-readable render, and a reference-validation hook against known actor/place/container/item IDs. `ActorWasNearPlace` is included as test/debug-only and must not be constructible into an actor's embodied knowledge path.

### 3. Module registration

Add `crates/tracewake-core/src/epistemics/mod.rs` declaring `pub mod proposition;` and re-exporting the public surface, and add `pub mod epistemics;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/mod.rs` (new)
- `crates/tracewake-core/src/epistemics/proposition.rs` (new)
- `crates/tracewake-core/src/ids.rs` (modify — add epistemic ID newtypes via `stable_id_type!`)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod epistemics;`)

## Out of Scope

- Observation/belief/contradiction records (ticket 002).
- Epistemic event kinds and stream (ticket 004).
- Content/fixture seed schema for propositions (ticket 012).
- Any projection or filtering logic (tickets 003, 005).

## Acceptance Criteria

### Tests That Must Pass

1. A `Proposition` value round-trips through canonical serialize/deserialize byte-identically.
2. `contradicts` returns true for `ItemLocatedInContainer(coin, box)` vs `ItemMissingFromExpectedLocation(coin, InContainer(box))` and false for unrelated propositions.
3. Each new ID rejects a display-name-shaped string (whitespace/empty) and orders by stable-string value.

### Invariants

1. `Proposition` carries no free-prose authoritative field; rendering is one-directional (structured → text).
2. ID and proposition ordering is by value, never by hash iteration or insertion order.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/proposition.rs` (unit tests) — round-trip, contradiction comparison, render, reference-validation rejection.
2. `crates/tracewake-core/src/ids.rs` (unit tests) — extend existing ID tests with the epistemic newtypes' rejection/ordering.

### Commands

1. `cargo test -p tracewake-core epistemics::proposition`
2. `cargo test -p tracewake-core ids`
3. A core-crate unit-test scope is correct here because the proposition model has no cross-crate consumers until tickets 002/004/012 land.
