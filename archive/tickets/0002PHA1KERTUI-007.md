# 0002PHA1KERTUI-007: Canonical physical-state checksum

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds the `checksum` module to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-003, 0002PHA1KERTUI-006

## Problem

Replay proves correctness by comparing a deterministic physical-state checksum (Spec 0002 §10.6). The checksum input must be canonical and must include fixture ID + content version, sim tick/order position, entity IDs and kinds, actor locations and carried items, place-graph state, door/container open/closed/locked state and ordered container contents, item holder/location, ownership/custody/access placeholder fields where they affect validation/debug, and event-stream position applied — while excluding non-world runtime metadata (terminal layout, active debug panel, current controller binding). Replay (013), debug reports (016), and the rejection reports (008) all consume this.

## Assumption Reassessment (2026-06-06)

1. No `checksum` module exists; registers `pub mod checksum;` in `crates/tracewake-core/src/lib.rs` (001), reading `state`/`location` (003) and the applied event-stream position from the log (006).
2. The required checksum inputs and the exclusion list are `specs/0002_…_SPEC.md` §10.6; the canonicalization discipline (no hash-map iteration order) is §10.2/§20.1.
3. Shared boundary under audit: the checksum/`PhysicalChecksum` type embedded in the replay report (013, §11.7), the item-location/rejection debug reports (016, §17), and event envelopes' `checksum_after` (005). Fixed here.
4. Invariant motivating this ticket: INV-018 (deterministic replay is foundational) — identical physical state + versions must yield a byte-identical checksum, and the checksum is the replay match criterion.
5. Deterministic-replay surface: the checksum is the canonical fingerprint replay compares. It must be a pure function of canonicalized physical state (sorted entity IDs, ordered container contents/carried lists), exclude runtime metadata (controller binding, debug panel), and never read wall-clock or hash iteration order. Enforcement that two runs match is ticket 013; this ticket guarantees the function is canonical and metadata-free.

## Architecture Check

1. A single canonical-serialization-then-hash function over an explicitly enumerated, sorted field set (rather than hashing the in-memory struct layout) guarantees platform- and run-stability and makes the exclusion of runtime metadata explicit and reviewable. Hashing struct memory or a `HashMap` iteration would be nondeterministic — forbidden by §10.2.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Determinism (INV-018; §10.6) -> unit test: the same physical state checksums identically across repeated computation and across a serialize/reload cycle.
2. Metadata exclusion (§10.6) -> unit test: changing controller binding / active debug panel does not change the physical checksum.
3. Canonical ordering (§10.2) -> manual review + codebase grep-proof: the checksum input is built by sorting entity IDs and using ordered contents, with no `HashMap`/`HashSet` iteration.

## What to Change

### 1. Checksum

Add `crates/tracewake-core/src/checksum.rs` computing a canonical physical-state checksum/report over the §10.6 field set: canonicalize (sort entities by stable ID, order container contents and carried items), serialize to a stable byte form, hash. Expose both the raw checksum and a structured report carrying the included fields for debug.

### 2. Registration

Add `pub mod checksum;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/checksum.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod checksum;`; file created by ticket 001)

## Out of Scope

- The replay runner/report that compares checksums (ticket 013).
- A separate run-metadata checksum (§10.6 allows one but Phase 1 does not require it; not introduced here).

## Acceptance Criteria

### Tests That Must Pass

1. Two independent computations over the same physical state produce an equal checksum.
2. A serialize→reload→recompute cycle produces the same checksum.
3. Mutating only controller binding or debug-panel state leaves the physical checksum unchanged.

### Invariants

1. The checksum is a pure function of canonicalized physical state + content version + stream position.
2. No runtime/presentation metadata enters the physical checksum.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/checksum.rs` (unit tests) — determinism, reload stability, metadata exclusion.

### Commands

1. `cargo test -p tracewake-core checksum`
2. `cargo build --workspace`
3. Unit scope is correct: the checksum is a pure function over in-crate state; cross-run equality under replay is exercised in ticket 013.

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::checksum` with `ChecksumContext`, `PhysicalChecksum`, and `PhysicalChecksumReport`.
- Implemented canonical physical-state line serialization over fixture/content version, tick, applied world-stream position, actors, places, doors, containers, and items.
- Implemented a stable dependency-free checksum over the canonical input.
- Registered `pub mod checksum;` in the core crate.

Deviations from original plan:
- The reload test recomputes from the canonical serialized checksum input rather than a full fixture loader, which remains out of scope until the content/replay tickets.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core checksum` passed: 4 tests.
- `cargo build --workspace` passed.
