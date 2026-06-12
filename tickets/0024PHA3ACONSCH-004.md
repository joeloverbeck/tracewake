# 0024PHA3ACONSCH-004: Fixture schema-version gate on the production load path

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`validate.rs`, `load.rs`, `schema.rs`) plus content tests.
**Deps**: 0024PHA3ACONSCH-001

## Problem

Spec 0024 `ORD-HARD-140` (high — the pass's product-behavior foundation violation):
the top-level fixture `schema_version` is never validated against a supported set.
`FixtureSchema.schema_version` is consumed only as a duplicate-key seed in
`validate_ids` and copied verbatim into `ContentManifest::new` by
`load_fixture_package`; a fixture declaring `schema|schema_v999` deserializes,
validates, loads, and propagates the bogus version with no diagnostic — the exact
posture INV-020 and `docs/2-execution/08`'s schema-version rule ("silent migration
is forbidden") prohibit. The per-belief epistemic gate
(`unsupported_epistemic_schema_version` in `validate_epistemic_seeds`) shows the
intended pattern; the package-level gate was simply never created.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: no membership check on
   `fixture.schema_version` anywhere in `validate_fixture_errors`; the epistemic
   contrast gate exists at `validate_epistemic_seeds`
   (`EPISTEMIC_RECORD_SCHEMA_V1`); `load_fixture_package`
   (`crates/tracewake-content/src/load.rs`) flows the unchecked version into
   `ContentManifest::new`. `FIXTURE_SCHEMA_V1` is a collision-free new const name
   (repo-wide grep at decomposition: zero matches).
2. Verified against spec 0024 §4 `ORD-HARD-140` and
   `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
   (schema-version and migration rule); INV-020 heading confirmed in
   `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
3. Cross-artifact boundary: the package-level `schema_version` contract between
   fixture bytes, `ContentManifest`, and the replay/content fingerprint — the gate
   must sit on the path every load takes, not only on a test-only entry point
   (the `ORD-HARD-150` lesson, closed separately by ticket -005).
4. INV-020 restated before trusting the narrative: event kinds and payloads must be
   versioned enough that replay can *reject unsupported history* rather than
   silently inventing repairs — extended by execution §08 to content packages:
   replay must reject mismatched content unless a named migration path exists.
5. Enforcement surface: this ticket *adds* a fail-closed, blocking validation
   (typed code `unsupported_fixture_schema_version`, phase `ParseSchema`); it
   weakens no existing gate, touches no actor-knowledge or replay-determinism
   surface, and the rejection is non-overridable (a blocker, not a warning).
6. Schema-shape statement: **no schema shape change** — the ticket adds a const and
   a validation rule; `FixtureSchema`'s fields are unchanged and every committed
   fixture already declares `schema_v1`, so no consumer updates are required
   (additive-vs-breaking analysis resolves as N/A).

## Architecture Check

1. A named supported-set const (`FIXTURE_SCHEMA_V1`) checked in
   `validate_fixture_errors` mirrors the proven epistemic-gate pattern one contract
   over, keeping both version gates structurally parallel; threading the same check
   through `load_fixture_package` (rather than only the validate entry point)
   closes the gate on the path production actually takes. A golden fixture per
   historical version (currently one) makes future version bumps an explicit,
   tested migration decision — the zero-dependency translation of the
   schema-evolution practice recorded in spec §2.
2. No backwards-compatibility aliasing/shims: no silent default, no fallback
   acceptance of unknown versions, no migration stub.

## Verification Layers

1. INV-020 / execution §08 fail-closed gate → negative test feeding
   `schema|schema_v999` through `load_fixture_package` (not only
   `validate_fixture_bytes`) asserting `LoadError::Validation` with code
   `unsupported_fixture_schema_version` (replay/golden-fixture check).
2. Gate deletion-resistance → a `CONTENT_NEGATIVE_PROOFS`-style census row so
   removing the rejection path fails the census (codebase grep-proof + test).
3. Historical-version coverage → per-version golden fixture test: each archived
   version either loads or is rejected loudly by name (schema validation).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. The gate

Add `FIXTURE_SCHEMA_V1` (in `schema.rs`, beside `SchemaVersion`); add
`validate_schema_version(fixture, errors)` to `validate_fixture_errors` rejecting
any other value with phase `ParseSchema`, code `unsupported_fixture_schema_version`.

### 2. Load-path threading

Confirm `load_fixture_package` cannot reach manifest construction with an ungated
version (the gate sits in the validation it already calls; add a load-path negative
proving it).

### 3. Golden per-version fixture + census row

Commit one golden byte fixture for `schema_v1` (inline byte literal, matching the
existing test convention) with a test asserting it loads; add the negative-proof
census row so the rejection path cannot be deleted silently.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)

## Out of Scope

- Routing the raw-line scan through the production load path (`ORD-HARD-150`) and
  ID-field shortcut scanning (`ORD-HARD-151`) — ticket -005.
- Any migration/upcast machinery (no second version exists; the gate makes a future
  version a deliberate decision).
- Epistemic per-belief schema gate (already exists; untouched).

## Acceptance Criteria

### Tests That Must Pass

1. A fixture declaring `schema|schema_v999` fails `load_fixture_package` with
   `unsupported_fixture_schema_version` at phase `ParseSchema`; the same fixture
   fails `validate_fixture_bytes` identically.
2. Every committed fixture (all currently `schema_v1`) still loads; the golden
   `schema_v1` byte fixture loads; the census row makes deleting the gate a test
   failure.
3. `cargo test -p tracewake-content`, then `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace` pass.

### Invariants

1. No load path can construct a `ContentManifest` from a fixture whose
   `schema_version` is outside the named supported set.
2. The rejection is fail-closed and blocking (a blocker, not a warning), and its
   deletion fails the negative-proof census.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (inline tests) — unsupported-version
   rejection negative; golden `schema_v1` positive.
2. `crates/tracewake-content/tests/fixtures_load.rs` — load-path negative through
   `load_fixture_package`; census-row presence.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
