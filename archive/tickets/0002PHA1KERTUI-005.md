# 0002PHA1KERTUI-005: Event envelope v1, streams, and event-kind registry

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `events` module (envelope, streams, kind registry) to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-002

## Problem

Meaningful physical changes must be versioned events from the first event (Spec 0002 §6.3, §11.2). This ticket defines the event-envelope v1 schema (all §11.2 fields), the four event streams (`world`, `diagnostic`, `controller`, `replay_debug`, §11.1 — only `world` reconstructs physical state), and the Phase 1 event-kind registry (§11.3). It is the type substrate for the append-only log and event application (ticket 006), the pipeline's event construction (ticket 008), and replay (ticket 013).

## Assumption Reassessment (2026-06-06)

1. No `events` module exists; registers `pub mod events;` in `crates/tracewake-core/src/lib.rs` (001), importing IDs (002), `SimTick`/`OrderingKey` (004) for the `sim_tick`/`ordering_key` envelope fields.
2. The envelope field set is `specs/0002_…_SPEC.md` §11.2; the stream taxonomy is §11.1; the required event kinds are §11.3. Spec 0002 §11.3 is a consistent superset of `docs/2-execution/05_…REPLAY.md`'s kind list — it adds `ItemTakenFromPlace`, `ItemPlacedInPlace`, `ActorWaited`/`TimeAdvanced`, `ControllerDetached`, `NoHumanAdvanceCompleted` (the container-only exec-05 list plus the wait/time and place-variant completions). All §11.3 kinds are in scope.
3. Shared boundary under audit: the envelope schema and event-kind enum consumed by `events/apply` (006), the pipeline (008), projections/replay (012/013), debug reports (016), and content serialization (017). Fixed here.
4. Invariant motivating this ticket: INV-020 (event schema evolution is mandatory) — every envelope carries `event_schema_version` from the first event so replay can reject unsupported history rather than silently inventing repairs; and INV-009 (meaningful state changes require events).
5. Deterministic-replay surface: envelopes carry `stream_position`, `global_order`, `sim_tick`, `ordering_key`, and (for world events) `checksum_after` — the fields replay re-derives. This ticket defines the schema only (no application); it guarantees the fields needed for byte-identical replay exist and serialize canonically (ordered, no hash-map iteration). Enforcement is ticket 013.

## Architecture Check

1. One versioned envelope type with an explicit `stream` tag and a closed event-kind enum (rather than free-form event structs) means replay can reject unknown `event_schema_version`/kind deterministically and the four streams stay separable (only `world` is authoritative). A stringly-typed event would lose the closed-set guarantee §11.2 requires ("no display strings as type authority").
2. No backwards-compatibility shims: this is the v1 envelope; there is no prior version to alias.

## Verification Layers

1. Schema versioning (INV-020) -> unit test: every constructed envelope has a non-empty `event_schema_version`; an unknown version is representable and flagged (consumed by replay in 013).
2. Stream separation (§11.1) -> unit test: only `world`-stream kinds are marked physical-mutating; `diagnostic`/`controller`/`replay_debug` are non-mutating.
3. Canonical serialization (INV-018) -> manual review: envelope serialization uses ordered fields and ordered `participants`, no hash-map iteration.

## What to Change

### 1. Envelope

Add `crates/tracewake-core/src/events/envelope.rs` (or `events/mod.rs`) with the v1 envelope carrying every §11.2 field (`event_id`, `event_type`, `event_schema_version`, `stream`, `stream_position`, `global_order`, `sim_tick`, `ordering_key`, `actor_id?`, `process_id?`, `participants`, `place_id?`, `causes`, `proposal_id?`, `validation_report_id?`, `random_draws`, `payload`, `effects_summary`, `content_manifest_id`, `checksum_after?`).

### 2. Streams + kind registry

Add the `Stream` enum (§11.1) and the event-kind enum/registry covering every §11.3 kind, each tagged with its stream and whether it mutates physical world state.

### 3. Registration

Add `pub mod events;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/events/mod.rs` (new)
- `crates/tracewake-core/src/events/envelope.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod events;`; file created by ticket 001)

## Out of Scope

- The append-only log and strict event application (ticket 006).
- Event construction from accepted proposals (ticket 008).
- Per-kind payload mutation semantics beyond the typed payload shape (defined where each action emits its event: tickets 009–011, 014, 015).

## Acceptance Criteria

### Tests That Must Pass

1. Every event-kind variant maps to exactly one stream, and only `world` kinds are physical-mutating.
2. An envelope serializes/deserializes round-trip with a stable `event_schema_version`.
3. The kind registry contains every §11.3 kind (count assertion re-enumerated from the registry, not hardcoded).

### Invariants

1. Every envelope is versioned from construction.
2. Non-`world` streams are never marked physical-mutating.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/mod.rs` (unit tests) — stream tagging, version presence, round-trip, kind-coverage.

### Commands

1. `cargo test -p tracewake-core events`
2. `cargo build --workspace`
3. Unit scope is correct: the schema has no cross-crate behavior until application (006) and replay (013).

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::events` with a versioned v1 event envelope carrying the Phase 1 envelope field set.
- Added the four event streams and a closed Phase 1 event-kind registry with stream and physical-mutation metadata.
- Added canonical dependency-free byte serialization/deserialization for the envelope schema surface.
- Registered `pub mod events;` in the core crate.

Deviations from original plan:
- Kept the implementation to the envelope, stream taxonomy, and event-kind registry. The append-only log remains with ticket 006, matching this ticket's explicit out-of-scope note.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core events` passed: 4 tests.
- `cargo build --workspace` passed.
