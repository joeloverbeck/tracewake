# 0003PHA1SPIANT-003: Total, mechanically-tested event-kind stream/mutation metadata

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`events/envelope.rs` event-kind metadata centralization; conformance + adversarial tests in `events/apply.rs` / core tests)
**Deps**: None

## Problem

`EventKind` metadata (stream mapping, `physical_mutating` flag) exists (`events/envelope.rs:21-148`, `:151-226`) and apply routes non-world streams away from physical mutation (`events/apply.rs:40-58`). But nothing mechanically guarantees totality: a future `EventKind` variant could be added without metadata, a physical-mutating kind could be mapped to a non-`World` stream, or a controller/diagnostic/debug event with a physical-looking payload could slip through. Spec `0003` §5.2 / SPINE-AC-003 require a single authoritative metadata registry and conformance tests proving totality plus an adversarial non-world-event checksum-invariance test.

## Assumption Reassessment (2026-06-08)

1. `EventKind` is defined at `crates/tracewake-core/src/events/envelope.rs:22`, with `EventKind::all()` at `:80` and `physical_mutating(self)` at `:211`; `EventStream` at `:12`; metadata struct at `:338`. Apply routes by stream and treats non-world streams as non-mutating (`crates/tracewake-core/src/events/apply.rs:40-58`).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-003 mandates: one authoritative metadata registry; every `EventKind` carries stream + schema + physical-mutation flag + replay handling; conformance tests asserting `EventKind::all()` covers every variant, every physical-mutating kind maps to `EventStream::World` (barring an explicit exception), controller/diagnostic/replay-debug/debug-only streams cannot change the `PhysicalState` checksum, and stream mismatch is rejected before mutation; plus an adversarial non-world event with a physical-looking payload proving the physical checksum is unchanged.
3. Boundary under audit: the `EventKind` → (stream, physical_mutating, replay-handling) metadata contract in `events/envelope.rs`, consumed by `events/apply.rs`. Shares `events/envelope.rs` with 0003PHA1SPIANT-002 (schema-version registry) — coordinate mechanical merges.
4. INV motivating this ticket: `INV-008` (kernel authority over legality/mutation), `INV-068`/`INV-069` (debug/non-diegetic surfaces are not world authority), `INV-107` (debug holds no mutation power), with `INV-009`/`INV-011` (only world events mutate world state). Restated: stream/mutation routing must be total and mechanically enforced so no event class can mutate physical state outside the world stream.
5. Determinism / no-leak surface touched: the adversarial test asserts a non-world event with a physical-looking payload leaves the `PhysicalState` checksum (`checksum.rs:47-152`) byte-identical — i.e. routing cannot be bypassed by payload shape. Apply must reject stream mismatch *before* mutation. No nondeterminism is introduced; the test reads existing canonical checksums.
6. Schema extension (event-kind metadata): centralizing/expanding the per-kind metadata registry. Consumers = `events/apply.rs` routing and any replay handling keyed on `EventKind`. Extension is **additive** — existing kinds keep their stream/flag; the registry gains a totality guarantee and (if missing) a `replay-handling` field. No consumer behavior changes for current kinds.

## Architecture Check

1. A single authoritative registry + `EventKind::all()`-driven conformance test converts "did the author wire up the new kind everywhere?" into a build failure, and the adversarial checksum test proves the routing is payload-shape-proof rather than convention-proof. This is strictly stronger than the current correct-by-inspection routing.
2. No backwards-compatibility shim: no second metadata source or fallback default for un-mapped kinds — an un-mapped kind is a hard test failure, not a silently-defaulted entry.

## Verification Layers

1. `INV-008` (kernel authority over mutation routing) -> conformance test: `EventKind::all()` covers every variant and each has complete metadata.
2. `INV-107`/`INV-068` (debug/non-world streams hold no mutation power) -> adversarial replay/checksum check: a non-world event with a physical-looking payload leaves the `PhysicalState` checksum unchanged.
3. `INV-011` (no hidden mutation) -> codebase grep-proof + runtime: stream mismatch is rejected before mutation; physical-mutating kinds map to `EventStream::World`.

## What to Change

### 1. Centralize and complete event-kind metadata

In `crates/tracewake-core/src/events/envelope.rs`, ensure a single registry maps each `EventKind` to stream, schema, `physical_mutating`, and replay-handling. Add the replay-handling field if not already present.

### 2. Totality + routing conformance tests

Add tests asserting: `EventKind::all()` covers every enum variant; every physical-mutating kind maps to `EventStream::World` (or an explicitly-listed exception); stream mismatch is rejected before mutation.

### 3. Adversarial non-world-mutation test

Add a test constructing a controller/diagnostic/replay-debug event carrying a physical-looking payload and assert the `PhysicalState` checksum is unchanged after application.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — stream-mismatch-before-mutation guard + tests)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `event_kind_metadata_is_total`)

## Out of Scope

- Schema-version registry/migrator (0003PHA1SPIANT-002).
- Checksum field-coverage totality (0003PHA1SPIANT-004) — this ticket asserts checksum *invariance* under non-world events, not field coverage.
- Adding or renaming any event kind.

## Acceptance Criteria

### Tests That Must Pass

1. `event_kind_metadata_is_total` — every `EventKind` variant has complete stream/mutation/schema/replay metadata; fails if a variant lacks an entry.
2. `non_world_stream_cannot_change_physical_checksum` — a controller/diagnostic/replay-debug event with a physical-looking payload leaves the `PhysicalState` checksum unchanged.
3. `cargo test --workspace` passes.

### Invariants

1. Stream/mutation metadata is total over `EventKind` and mechanically enforced (`INV-008`).
2. Only `EventStream::World` events change physical state; non-world streams cannot (`INV-011`, `INV-107`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `event_kind_metadata_is_total`, `non_world_stream_cannot_change_physical_checksum`.
2. `crates/tracewake-core/src/events/apply.rs` (`#[cfg(test)]`) — stream-mismatch-before-mutation unit test.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`
