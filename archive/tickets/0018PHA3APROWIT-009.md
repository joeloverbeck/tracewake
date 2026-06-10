# 0018PHA3APROWIT-009: Census durability — content negatives, fixture registry, mutants ratchet, zero-dependency census

**Status**: DONE
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — test/CI substrate (`tracewake-content` tests, `tracewake-core` lock-layer tests, `.github/workflows/ci.yml`, `clippy.toml` as surfaced); no production code changes
**Deps**: `archive/tickets/0018PHA3APROWIT-005.md`, `archive/tickets/0018PHA3APROWIT-006.md` (the content censuses cover the finished content surface; spec §8 ordering); `archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-043, §5 tiers 4/6)

## Problem

Three census/durability residuals let lock coverage drift silently. (1) The 0017-named content negatives (`fixture_workplace_zero_duration_rejected_001`, `fixture_initial_need_out_of_band_rejected_001`, `fixture_output_tag_script_marker_rejected_001`) exist as inline `validate.rs` unit tests; `negative_fixture_runner.rs`'s census covers only `tests/negative-fixtures/`, and no census ties `NumericFieldPolicy`/`StringScanPolicy` variants to proving rejections — the recurring ORD-HARD-034 inline-vs-registered shape. (2) `fixtures_load.rs` asserts `fixtures::all()` equals a hand-maintained set, so an unregistered `pub fn *_001` fixture is silently unexercised. (3) The scheduled `mutants-lock-layer` CI job checks an exact `diff -u` against `.cargo/mutants-baseline-misses.txt` (fails on line drift, unlike the PR job's normalize-and-`comm` ratchet) and gates no merge. Spec §5 adds two posture locks: clippy `disallowed_methods`/`disallowed_types` entries where banned calls have stable paths (tier 4), and an in-tree zero-dependency census (tier 6).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: the three negatives live in `validate.rs::mod tests`; `negative_fixture_runner.rs::registered_negative_fixtures_match_directory_census` enforces `tests/negative-fixtures/` only; `fixtures_load.rs` compares `fixtures::all()` to a hardcoded set; the scheduled job's check step runs `diff -u .cargo/mutants-baseline-misses.txt mutants.out/missed.txt` and runs only on `workflow_dispatch || schedule`, while the PR-time `mutants-in-diff` job (commit `544adda`) applies a normalized `comm -23` new-miss ratchet with `cargo-mutants` pinned at 27.1.0; `clippy.toml` exists with the clippy↔fixture parity census (`clippy_ban_entries_have_proving_negative_fixtures`); the workspace has zero production dependencies to enshrine.
2. Spec 0018 ORD-HARD-043 (required correction + structural lock) and §5 tier 4 (clippy policy lints, recorded with fixture parity) + tier 6 (zero-dependency census — the cargo-deny concept without the dependency; conformance row via ticket `-010`).
3. Cross-artifact boundary under audit: the lock-layer census contracts — every policy variant cites a proving rejection; every fixture constructor is registered; the mutation baseline only ratchets down; the dependency posture is CI-checked, not conventional.
4. Fail-closed validation surface touched (enforcement side only): the content-negative registry exercises `validate.rs` rejection paths and must not weaken any — it records and census-checks existing rejections; the censuses are additive test oracles. No replay, golden, or actor-knowledge surface changes; no production code changes.
5. Adjacent contradictions classified: the conformance-index rows for these censuses are ticket `0018PHA3APROWIT-010`'s docs surface (future cleanup owned by a sibling, not this ticket); accepting the exact-diff brittleness instead of ratcheting was the spec-offered alternative — this ticket implements the ratchet (recommended), recording the choice here per the spec's either-way framing.

## Architecture Check

1. A content-negative registry (policy variant → proving rejection test) mirrors the proven `CLIPPY_BAN_PROOFS` pattern — the census makes "shipped a policy without a proving negative" a CI failure, closing the inline-vs-registered gap class permanently instead of re-fixing it per pass. Deriving the positive-fixture census from the `fixtures` module source mirrors the workspace source-classification census (parse, don't hand-maintain). Aligning the scheduled mutation check with the PR job's normalized ratchet removes the false-failure mode (line drift) while keeping the real one (new misses). The zero-dep census makes the workspace's zero-dependency posture (already a stated §8 constraint in every spec of the series) machine-checked from inside the tree, with a committed allowlist for dev-dependencies.
2. No backwards-compatibility aliasing/shims: the exact-diff check is replaced by the ratchet, not kept as a parallel mode; the hand-maintained fixture set is replaced by the derived census.

## Verification Layers

1. Policy-rejection parity -> new content-negative registry census: every `NumericFieldPolicy` and `StringScanPolicy` variant cites a registered proving rejection (name → expected code + path); an unproven variant fails CI.
2. Fixture-registration completeness -> derived census: `pub fn *_001` constructors parsed from the `fixtures` module source equal `fixtures::all()` membership; an unregistered fixture fails.
3. Mutation ratchet -> the scheduled job's check uses the same strip/`comm -23` normalization as the in-diff job and fails only on new misses; `cargo-mutants` stays pinned.
4. Dependency posture -> new census test parsing the workspace `Cargo.toml`s: `tracewake-core` `[dependencies]` empty; the full workspace dependency set equals a committed allowlist.
5. Clippy policy parity -> any new `disallowed_methods`/`disallowed_types` entries carry proving negative fixtures via the existing `clippy_ban_entries_have_proving_negative_fixtures` census.

## What to Change

### 1. Content-negative registry + census

A registry in `validate.rs` (or sibling) mapping each policy variant to its proving rejection test/fixture name; a census test asserting totality; register the three existing inline negatives.

### 2. Derived positive-fixture census

`fixtures_load.rs`: replace the hardcoded expected set with a census derived from the `fixtures` module source (constructor parse), asserting parity with `all()`.

### 3. Scheduled mutation ratchet

`.github/workflows/ci.yml`: the scheduled job's baseline check normalizes (file+mutation+function, drop line:col) and `comm -23`s against the committed baseline, failing only on growth; surface failures visibly (job summary).

### 4. Zero-dependency census + clippy entries

`anti_regression_guards.rs`: the dependency-posture census (workspace `Cargo.toml` parse vs committed allowlist). Add `clippy.toml` `disallowed_methods`/`disallowed_types` entries where stable-path bans exist (as surfaced), each with its proving fixture.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `.github/workflows/ci.yml` (modify)
- `clippy.toml` (modify — as surfaced by stable-path ban candidates)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — as surfaced, if the content registry routes through it)

## Out of Scope

- The generative tier (ticket `0018PHA3APROWIT-008`).
- Conformance-index rows for the new censuses (ticket `-010`).
- Any new dependency, dev or production — the census enshrines the existing posture.
- Changing what any validation policy rejects — registration and census only.

## Acceptance Criteria

### Tests That Must Pass

1. The content-negative registry census — every policy variant proven; the three existing negatives registered.
2. The derived fixture census — parity between parsed constructors and `all()`.
3. The dependency-posture census — core `[dependencies]` empty; workspace set equals the allowlist.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No validation policy variant ships without a registered proving rejection; no fixture constructor exists unregistered.
2. The mutation baseline can only shrink without explicit human re-dispositioning; the dependency posture cannot drift silently.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (or sibling test) — content-negative registry census.
2. `crates/tracewake-content/tests/fixtures_load.rs` — derived fixture census.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — dependency-posture census; clippy parity entries.

### Commands

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test --workspace`

## Outcome

Completed 2026-06-11:

1. Added `CONTENT_NEGATIVE_PROOFS` in `validate.rs` and a census test covering every `NumericFieldPolicy` and `StringScanPolicy` variant, including the three existing inline negatives and a raw negative count proof for the unsigned count policy.
2. Replaced hand-maintained positive fixture ID sets in `fixtures_load.rs` with a source-derived constructor census for `pub fn *_001() -> GoldenFixture`, while preserving Phase 1 registry boundary checks and scope coverage assertions.
3. Updated the scheduled `mutants-lock-layer` workflow check to normalize file+mutation+function entries, drop line/column drift, ratchet with `comm -23`, and write new misses to the job summary.
4. Added `workspace_dependency_posture_matches_allowlist` in `anti_regression_guards.rs`, asserting `tracewake-core` has empty `[dependencies]` and the workspace dependency set matches the committed allowlist. No new `clippy.toml` entries were surfaced beyond the existing stable-path bans.

Verification:

1. `cargo test -p tracewake-content content_negative_registry_covers_validation_policy_variants_and_tests`
2. `cargo test -p tracewake-content --test fixtures_load`
3. `cargo test -p tracewake-core workspace_dependency_posture_matches_allowlist`
4. `cargo test -p tracewake-core --test anti_regression_guards`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `cargo build --workspace --all-targets --locked`
8. `cargo test --workspace`
