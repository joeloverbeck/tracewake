# 0002PHA1KERTUI-017: Content crate — schema, deterministic load, manifest, serialization

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — populates `tracewake-content` with `schema`, `load`, `manifest`, `serialization` modules.
**Deps**: 0002PHA1KERTUI-002, 0002PHA1KERTUI-003, 0002PHA1KERTUI-005

## Problem

Fixtures and domain-pack data must load deterministically and serialize stably for golden tests and replay (Spec 0002 §9.3, §10.2, §13.4). This ticket adds the content schema (serialized fixture/domain-pack data structures that mirror core types but make invalid authoring detectable), deterministic loading with canonical ordering (§10.2 — canonical path order, lexicographic-by-stable-ID normalization, no serialized-map iteration dependence), the content manifest (package identity, fixture id, schema version, content version, canonical path order, checksum/fingerprint), and stable serialization for fixtures, event logs, validation reports, and replay reports (§13.4).

## Assumption Reassessment (2026-06-06)

1. `tracewake-content` exists as an empty crate from ticket 001 (depending on `tracewake-core`); this populates its modules. It imports core IDs (002), state records (003), and event envelopes (005) to define the serialized mirror schema.
2. The content modules and responsibilities are `specs/0002_…_SPEC.md` §9.3 (`schema`/`load`/`manifest`/`serialization`); deterministic loading order rules are §10.2; serialization requirements are §13.4. The architecture contract is `docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md`.
3. Shared boundary under audit: the serialized content schema (the authoring surface) versus the core runtime records — they mirror but the content schema must reject invalid authoring; consumers are the validator (018), fixtures (019), and the TUI loader (020).
4. Invariant motivating this ticket: INV-018 (deterministic replay) and INV-020 (event schema evolution) — the content manifest carries schema/content version so replay can check compatibility; loading is deterministic across runs/platforms.
5. Fail-closed-validation substrate + deterministic surface: this ticket builds the *inputs* to content validation (018) and the accepted `InitialWorld` replay consumes. Loading must be deterministic (canonical path order, lexicographic normalization, no hash-map iteration) and serialization round-trip-stable (no checksum change). It introduces no behavior-looking field and no nondeterminism path the validator (018) or replay (013) would have to undo; the validator enforces rejection of bad content.

## Architecture Check

1. A serialized schema distinct from the core runtime records (rather than serializing the runtime structs directly) lets authoring constraints (reject unknown fields, detect invalid shapes) live at the boundary while the kernel keeps clean types — the §9.3 "may mirror core types but should make invalid authoring detectable" intent. Canonicalize-before-resolve ordering makes load order independent of file-system or map iteration.
2. No backwards-compatibility shims: greenfield; the content crate never depends on the TUI.

## Verification Layers

1. Deterministic load (INV-018; §10.2) -> unit test: loading the same content package twice (and across shuffled input path order) yields the same canonical accepted world.
2. Serialization stability (§13.4) -> unit test: a fixture/event-log serialize→deserialize→reserialize cycle is byte-identical and preserves the canonical checksum.
3. Manifest identity (INV-020) -> unit test: the manifest carries fixture id, schema version, content version, and a content fingerprint.

## What to Change

### 1. Schema

Add `crates/tracewake-content/src/schema.rs`: serialized fixture/domain-pack structures mirroring core entity/component types, shaped so invalid authoring is detectable.

### 2. Load + manifest

Add `crates/tracewake-content/src/load.rs` (deterministic file/package loading, canonical ordering, reference resolution after canonicalization) and `crates/tracewake-content/src/manifest.rs` (package identity, fixture id, schema/content version, canonical path order, checksum/fingerprint).

### 3. Serialization

Add `crates/tracewake-content/src/serialization.rs`: stable serialization for fixtures, event logs, validation reports, replay reports.

### 4. Registration

Declare the modules in `crates/tracewake-content/src/lib.rs`.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (new)
- `crates/tracewake-content/src/load.rs` (new)
- `crates/tracewake-content/src/manifest.rs` (new)
- `crates/tracewake-content/src/serialization.rs` (new)
- `crates/tracewake-content/src/lib.rs` (modify — declare modules; file created by ticket 001)

## Out of Scope

- The content validation phases / required failures / forbidden-content rejection (ticket 018 — this ticket provides the schema it validates).
- The seven golden fixture definitions (ticket 019).

## Acceptance Criteria

### Tests That Must Pass

1. Loading the same package across shuffled input path order yields the same canonical accepted world.
2. A fixture + event-log serialize/deserialize cycle is byte-identical and checksum-stable.
3. The manifest carries fixture id, schema version, content version, and content fingerprint.

### Invariants

1. Load order is canonical and independent of file-system or map iteration order.
2. Serialization is stable enough to commit as golden regression data.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/load.rs` (unit tests) — deterministic load under shuffled input.
2. `crates/tracewake-content/src/serialization.rs` (unit tests) — round-trip stability.

### Commands

1. `cargo test -p tracewake-content schema load manifest serialization`
2. `cargo build --workspace`
3. Content-crate scope is correct: loading/serialization are exercised against core types in-crate; validation rejection is ticket 018.

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake-content` schema, load, manifest, and serialization modules.
- Added deterministic fixture canonicalization and conversion into core `PhysicalState`.
- Added content manifests with fixture/schema/content identity, canonical paths, and stable fingerprints.
- Added stable line-based fixture and event-log serialization round trips.

Deviations from original plan:
- The documented combined Cargo filter `cargo test -p tracewake-content schema load manifest serialization` is not accepted by Cargo as written, so verification used separate module filters plus a package-wide content test.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-content load` passed: 1 test.
- `cargo test -p tracewake-content manifest` passed: 1 test.
- `cargo test -p tracewake-content serialization` passed: 2 tests.
- `cargo test -p tracewake-content` passed: 4 tests.
- `cargo build --workspace` passed.
