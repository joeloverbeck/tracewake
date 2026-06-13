# 0025PHA3AMETWIT-004: Manifest fingerprint honesty — raw-bytes payload, frozen golden fingerprints, per-scope golden bytes

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`manifest.rs`, `load.rs`, `validate.rs`) and content test suites; no core/tui code.
**Deps**: None

## Problem

Spec 0025 findings `ORD-HARD-169` (medium), `ORD-HARD-170` (medium),
`ORD-HARD-183` (low). The content fingerprint is decorative: `ContentManifest::new`
/ `stable_fingerprint` compute it on every load, but no consumer reads it (core and
tui read only `manifest_id`/`fixture_id`/`content_version`), no `twf1-` golden is
frozen anywhere, and the only assertions are a prefix check and load-twice
self-consistency — the fingerprint can drift to any value undetected. Its payload
is also wrong: `load_fixture_package` fingerprints
`canonical_bytes = serialize_fixture(&fixture)` (the re-serialized parsed struct)
plus secondary-file *path strings* — raw-byte drift the codec normalizes and
secondary-file *content* are invisible to it (the research-corroborated CAS
pitfall: fingerprint raw bytes, never the parsed struct). And the golden-bytes pin
(`ORD-HARD-165`'s repair) covers one Phase-1-shaped fixture with a tautological
trailing assert (`perturbed[position] = b'I'; assert_ne!(perturbed, EXPECTED)` —
cannot fail).

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: `content_fingerprint` consumers
   are confined to `tracewake-content` (`load.rs`, `manifest.rs`, two test files;
   zero hits in core/tui); `canonical_bytes = serialize_fixture(&fixture)` in
   `load_fixture_package`; no frozen `twf1-` literal repo-wide; the tautological
   perturbation assert in `validate.rs`. All operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 and doctrine: INV-018/019 (replay reconstruction
   inputs; ancestry survival), `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
   ("Fixture manifests declare compatible doctrine/gate expectations"); replay's
   data-version input is currently satisfied by `content_version` string equality
   only.
3. Shared boundary under audit: the `ContentManifest` fingerprint contract — what
   the fingerprint covers (raw loaded bytes, primary + secondary) and where it is
   verified (frozen goldens + the replay-acceptance seam in content tests).
4. Invariant restated before trusting the narrative (INV-018 — deterministic
   replay is foundational): replay reconstructs from seed, authored state, ordered
   events, and ruleset/data versions; an integrity surface that vouches for data
   identity must actually be checked, or it is a hollow evidence signal (R-29).
5. Replay-evidence surface touched: this ticket adds fingerprint verification at
   the content-crate replay-acceptance seam and freezes goldens. The fingerprint
   remains a pure FNV over ordered bytes (deterministic); no nondeterministic
   input enters it; no actor-knowledge surface is touched.
6. Schema/contract extension: the fingerprint *payload* is redefined (re-serialized
   struct + path strings → raw primary bytes + per-secondary `(path, bytes)`).
   Consumers of the old payload: none authoritative (verified — tests only assert
   prefix/self-consistency), so the redefinition breaks no recorded value; the
   newly frozen golden table is the first recorded consumer and lands in the same
   diff.

## Architecture Check

1. Fingerprinting the raw validated bytes (primary + secondaries) with frozen
   per-fixture goldens is cleaner than wiring the struct-payload fingerprint into
   replay: the CAS pattern makes byte-level drift visible by construction, and the
   frozen table turns every codec/content change into an explicit reviewable
   constant diff — the same discipline as the golden serialization bytes
   (`ORD-HARD-165`) it extends. Crate direction is preserved: verification lives in
   `tracewake-content` tests (content depends on core; core never reads content).
2. No backwards-compatibility aliasing/shims: one fingerprint definition; the old
   struct-payload derivation is replaced, not kept alongside.

## Verification Layers

1. Fingerprint consumption (`ORD-HARD-169`) → frozen `fixture_id → twf1-…` golden
   table in `golden_fixtures_run.rs`, plus the replay-acceptance seam comparing the
   recomputed fingerprint of each loaded fixture against the frozen value.
2. Payload coverage (`ORD-HARD-170`) → secondary-file mutation negative (mutate a
   secondary file's bytes; fingerprint must reprice) and a raw-token negative
   (raw-byte difference with identical parsed struct must reprice).
3. Golden-bytes coverage (`ORD-HARD-183`) → committed per-scope golden bytes
   (Phase-2A and Phase-3A representatives with belief/routine/workplace rows);
   tautological perturbation assert deleted (grep-proof).
4. Determinism (INV-018) → the fingerprint remains pure FNV over ordered bytes;
   `insertion_order_does_not_change_checksum`-style ordering tests stay green.
5. Whole-suite regression → four-gate workspace run.

## What to Change

### 1. Raw-bytes fingerprint payload (`ORD-HARD-170`)

`load_fixture_package` fingerprints the raw validated `primary.bytes` plus every
secondary file's `(path, bytes)` pair. The struct-level canonical bytes remain a
separate serialization-honesty check, not the fingerprint payload.

### 2. Frozen golden fingerprints + verification seam (`ORD-HARD-169`)

A frozen `fixture_id → twf1-…` table in
`crates/tracewake-content/tests/golden_fixtures_run.rs`, updated only by explicit
edit; the content-crate replay-acceptance tests assert each loaded fixture's
recomputed fingerprint matches the frozen value, and a synthetic mismatched
manifest fails the seam.

### 3. Per-scope golden serialization bytes; delete the tautology (`ORD-HARD-183`)

Add frozen golden-bytes pins for a Phase-2A and a Phase-3A representative fixture
in `validate.rs`'s golden test family; delete the `assert_ne!(perturbed, EXPECTED)`
tautology.

## Files to Touch

- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/manifest.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — secondary-file repricing negative)

## Out of Scope

- Kernel envelope integrity (`ORD-HARD-168`/`171`/`184`) — ticket
  `0025PHA3AMETWIT-003`.
- Threading fingerprint verification into `tracewake-core` (core cannot depend on
  content; the verification seam lives in content tests by crate direction).
- Any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. Every committed fixture's recomputed raw-bytes fingerprint matches its frozen
   `twf1-` golden; a synthetic mismatch fails the seam.
2. Mutating a secondary file's bytes reprices the fingerprint (negative fires); a
   raw-byte difference with identical parsed struct reprices; per-scope golden
   bytes (Phase-2A + Phase-3A) are committed; the tautological perturbation assert
   is gone (grep-proof).
3. `cargo test -p tracewake-content` and the four gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. The content fingerprint covers exactly the raw loaded bytes (primary +
   secondaries) and is verified against frozen goldens on the replay-acceptance
   path — never a self-consistency check alone.
2. Codec or fixture changes reprice frozen constants via explicit reviewable diffs;
   no wildcard or recomputed-expectation widening.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — frozen fingerprint
   table + verification seam + mismatch negative.
2. `crates/tracewake-content/tests/fixtures_load.rs` — secondary-file mutation
   repricing negative.
3. `crates/tracewake-content/src/validate.rs` (`#[cfg(test)]`) — per-scope golden
   bytes; tautology deleted.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
