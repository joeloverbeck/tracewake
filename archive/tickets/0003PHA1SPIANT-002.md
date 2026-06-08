# 0003PHA1SPIANT-002: Event schema-version registry, migrator gate, and loud unknown-version failure

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`events/envelope.rs` schema-version type/registry; `events/apply.rs` + `replay/rebuild.rs` version handling; new conformance tests)
**Deps**: None

## Problem

Event-schema discipline today is a single constant `EVENT_SCHEMA_V1` plus loud rejection of unsupported versions (`events/apply.rs:179`; `UnsupportedEventSchemaVersion` at `apply.rs:96`,`:164`). There is no explicit version registry, no migrator/upcaster gate, and no test forcing a new schema version to carry a migration path or a documented no-migration proof. A future event-shape change could be added without a deliberate version bump and without replay tests for old→new handling. Spec `0003` §5.2 / SPINE-AC-002 require a total schema-version registry, a migrator gate, and loud unknown-version failure before mutation.

## Assumption Reassessment (2026-06-08)

1. `EVENT_SCHEMA_V1` is the only live schema version (`crates/tracewake-core/src/events/envelope.rs:9`; envelope creation/checking around `:487-544`). Live apply rejects unsupported versions (`crates/tracewake-core/src/events/apply.rs:179`), and replay groups unsupported schemas by stream (`crates/tracewake-core/src/replay/rebuild.rs:67-81`). A standalone `EventSchemaVersion` type does **not** yet exist — only the error variant `UnsupportedEventSchemaVersion` (`apply.rs:96`); this registry is a genuinely new deliverable, not a rename.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-002 mandates: an explicit `EventSchemaVersion` newtype/enum + registry; `EVENT_SCHEMA_V1` as the only supported live version unless a new one is introduced deliberately; adding a version requires an upcaster/migrator (or an explicit no-migration proof), old→new / new→current / unknown-future replay tests, and a fixture or synthetic stream; unknown versions fail loudly before mutation in both live apply and replay.
3. Boundary under audit: the event-envelope schema-version contract shared by `events/envelope.rs` (definition), `events/apply.rs` (live apply), and `replay/rebuild.rs` (replay). This ticket shares `events/envelope.rs` with 0003PHA1SPIANT-003 (event-kind metadata) — coordinate mechanical merges.
4. INV motivating this ticket: `INV-020` (event-schema evolution / versioned migration), reinforced by `INV-017`/`INV-018` (deterministic, byte-identical replay across versions) and `INV-009` (events are the authoritative record). Restated: schema evolution must be explicit, migrated or proven migration-free, and must fail loud rather than silently reinterpret an unknown shape.
5. Deterministic-replay surface touched: replay reads schema versions to decide application/migration (`replay/rebuild.rs`). The registry must keep replay byte-identical for `EVENT_SCHEMA_V1` streams and must reject unknown versions *before* any mutation — no partial application. The migrator, when present, is a pure deterministic function of the old event; it introduces no wall-clock/RNG input.
6. Schema extension (event-envelope version field): the change replaces an ad-hoc constant with a typed `EventSchemaVersion` registry. Consumers = `events/apply.rs`, `replay/rebuild.rs`, and every event constructor using `EVENT_SCHEMA_V1` (e.g. `actions/defs/*` via `PayloadField::new("schema_version", EVENT_SCHEMA_V1)`, `replay/rebuild.rs:342`). The extension is **additive** for the live version — `EVENT_SCHEMA_V1` keeps its current value and meaning; only its representation gains a typed home. No backward-compat shim is added (no second live version is introduced by this ticket).

## Architecture Check

1. A typed `EventSchemaVersion` + registry turns "is this version supported?" from a scattered constant comparison into one authoritative lookup, and turns "did the author remember a migrator?" from reviewer vigilance into a compile/test gate. Loud pre-mutation failure for unknown versions matches event-versioning practice (explicit upcasting; fail on uninterpretable shapes) and protects replay determinism.
2. No backwards-compatibility shim: the registry does not silently accept old/foreign shapes. Only `EVENT_SCHEMA_V1` is live; any future version is an explicit, migrator-backed addition, not an implicit compatibility window.

## Verification Layers

1. `INV-020` (schema evolution) -> conformance test: a registry-change test fails if a supported version is added without a registered migrator (or explicit no-migration proof) and its replay tests.
2. `INV-018` (deterministic replay) -> replay/golden-fixture check: a synthetic unknown-version event stream fails loudly *before* mutation in both live apply and replay; `EVENT_SCHEMA_V1` golden replays stay byte-identical.
3. `INV-009` (events authoritative) -> codebase grep-proof: all event constructors source their version from the registry/`EventSchemaVersion`, not ad-hoc literals scattered outside it.

## What to Change

### 1. Introduce `EventSchemaVersion` + registry

In `crates/tracewake-core/src/events/envelope.rs`, add an `EventSchemaVersion` newtype/enum and a registry of supported versions, with `EVENT_SCHEMA_V1` expressed through it. Provide a single supported-version check and a migrator lookup.

### 2. Migrator/upcaster gate

Define the migrator surface (a function `old_event -> current_event`, or an explicit no-migration marker) keyed by version. Adding a supported version without a registered migrator entry must fail a conformance test.

### 3. Loud unknown-version failure before mutation

Ensure `events/apply.rs` and `replay/rebuild.rs` reject unknown/unsupported versions before any state mutation, returning the typed schema-failure (extend/keep `UnsupportedEventSchemaVersion`), with a typed, version-bearing diagnostic.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/events/mod.rs` (modify — re-export the version type if needed)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — registry/migrator conformance test)

## Out of Scope

- Event-kind stream/mutation metadata totality (0003PHA1SPIANT-003).
- Introducing an actual second schema version or any real migration — this ticket builds the *gate*, exercised by a synthetic/fixture stream, not a real v2.
- Replay/checksum field coverage (0003PHA1SPIANT-004).

## Acceptance Criteria

### Tests That Must Pass

1. `unsupported_event_schema_version_replay_fails_loudly` — a synthetic unknown-version event stream fails before mutation in both live apply and replay and reports a typed schema failure.
2. `adding_event_schema_version_requires_migrator_registration` — the version registry cannot gain a supported version without a registered migrator (or explicit no-migration proof) and its replay tests.
3. `cargo test --workspace` passes with `EVENT_SCHEMA_V1` golden replays byte-identical.

### Invariants

1. Exactly one live schema version (`EVENT_SCHEMA_V1`); any addition is migrator-backed and test-gated (`INV-020`).
2. Unknown versions fail loudly before any mutation in both live apply and replay (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `unsupported_event_schema_version_replay_fails_loudly`, `adding_event_schema_version_requires_migrator_registration`.
2. `crates/tracewake-core/src/events/envelope.rs` (`#[cfg(test)]`) — registry totality / version-typing unit tests.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Added a typed `EventSchemaVersion` registry in `events/envelope.rs`, with `EVENT_SCHEMA_V1` represented as `EventSchemaVersion::V1`.
- Added explicit migration disposition metadata via `EventSchemaMigration`; the only live version is registered as `CurrentNoMigrationRequired`.
- Routed `EventEnvelope::has_supported_schema_version` through the registry instead of a direct string comparison.
- Re-exported the schema-version registry API from `events`.
- Added registry unit tests proving exactly one current version is live and unknown schema versions are unsupported.
- Added `adding_event_schema_version_requires_migrator_registration` in the anti-regression guards.
- Added `unsupported_event_schema_version_replay_fails_loudly`, proving live apply rejects unknown versions before mutation and replay reports the unsupported version without applying it.

Deviations from original plan:
- No real second schema version or migrator was introduced, per ticket scope. The gate is exercised by the current no-migration proof and a synthetic unknown-future version.

Verification:
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo test -p tracewake-core unsupported_event_schema_version_replay_fails_loudly`
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
