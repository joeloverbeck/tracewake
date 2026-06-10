# 0018PHA3APROWIT-006: Proof-of-validation token on fixture materialization

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `tracewake-content` (`schema`, `validate`, `load`); one typed-error/compile-fail test
**Deps**: `archive/tickets/0018PHA3APROWIT-005.md` (shares `schema.rs`/`load.rs`; spec §8 ordering); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-041)

## Problem

`schema.rs::FixtureSchema::to_agent_state` is `pub` and calls `NeedState::initial`, which clamps out-of-band values via `need.rs::clamp_need_value` — an out-of-band authored need value materializes silently clamped. The production path is safe today (`load.rs::load_fixture_package` validates before materializing, and `validate_need_band_u16` rejects out-of-band values), but nothing structurally prevents a future caller from materializing an unvalidated fixture, reintroducing the authored-vs-materialized disagreement that 0017's ORD-HARD-030 fix closed at the validation gate (INV-022: no load-time mutation of authored facts; execution doc 08: silent migration is forbidden).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `to_agent_state` is `pub` with exactly one call site (`load.rs:96`, after `validate_state` runs in `load_fixture_package`); `NeedState::initial` clamps via `clamp_need_value`; the `accepted_world` proof-token pattern already exists in this crate (`validate.rs`/`load.rs`), giving a house precedent for validation-gated construction.
2. Spec 0018 ORD-HARD-041 (required correction + structural lock): token or `Result` — the recommended direction is the token (`accepted_world` pattern precedent); the choice is recorded here per the spec's implementer-choice framing.
3. Cross-artifact boundary under audit: the validation→materialization contract inside `tracewake-content` — authored values may reach `AgentState` only through the fail-closed validation gate.
4. INV-022 restated: prose/authored data may not silently define or mutate authoritative state; an authored seed that materializes different from what was written is a fact mutated by load.
5. Fail-closed validation surface touched: the token makes the unvalidated materialization path unconstructable (compile-time), strengthening the existing `invalid_need_band` rejection; no event stream or replay surface changes (the production path already validates), so golden checksums are untouched and no leakage path is affected.
6. Consumer blast radius of the signature change: breaking-internal (not additive) — the `to_agent_state` signature changes for one production call site (`load.rs:96`) plus any test-internal constructions surfaced at implementation, all in-workspace and updated in the same diff (local compile-atomicity; the tree must compile at every commit). No serialized schema shape changes — `FixtureSchema`'s authored fields are untouched by this ticket.

## Architecture Check

1. A proof-of-validation token (a zero-sized witness minted only by the validation pass, consumed by `to_agent_state`) follows the crate's existing `accepted_world` pattern and is cleaner than a `Result`-returning constructor: the clamp can never fire on authored seeds because unvalidated materialization no longer type-checks, rather than failing at runtime. The clamp itself stays for *runtime* need arithmetic (`apply_delta`), where saturation is the correct semantics.
2. No backwards-compatibility aliasing/shims: the tokenless `to_agent_state` signature is replaced, not kept as a deprecated alias.

## Verification Layers

1. INV-022 unconstructable bypass -> compile-fail doctest (the `DebugCapability` house pattern): calling `to_agent_state` without the validation token does not compile; or, if the token is runtime-checked, a typed-error unit test.
2. Validation-gate integrity -> existing `fixture_initial_need_out_of_band_rejected_001` rejection test stays green (`invalid_need_band` at the gate).
3. Production-path equivalence -> `cargo test -p tracewake-content --test fixtures_load` — all fixtures load and materialize identically (no behavioral change on the validated path).

## What to Change

### 1. Validation token

`validate.rs` mints a crate-controlled proof token (ZST with a private constructor) on successful `validate_state`/`validate_fixture` completion, following the `accepted_world` precedent.

### 2. Token-gated materialization

`schema.rs::to_agent_state` takes the token; `load.rs::load_fixture_package` threads it from the validation pass to the call site.

### 3. Lock test

A compile-fail doctest (or typed-error test) proving the unvalidated path is unconstructable; update any test-internal callers to go through validation.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — as surfaced by test-internal callers)

## Out of Scope

- The seed-grammar knowledge edges (ticket `0018PHA3APROWIT-005`, a dependency).
- Changing `clamp_need_value` semantics for runtime deltas — saturation remains correct for `apply_delta`.
- Any core-crate change — this is contained to `tracewake-content`.

## Acceptance Criteria

### Tests That Must Pass

1. The compile-fail doctest (or typed-error test) for tokenless materialization.
2. `cargo test -p tracewake-content --test fixtures_load` and `--test golden_fixtures_run` — validated path byte-identical.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No code path can materialize a `FixtureSchema` into `AgentState` without passing the fail-closed validation gate first.
2. Authored seed values and materialized values are identical on every constructible path; out-of-band values are rejected, never clamped, at authoring boundaries.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/schema.rs` (compile-fail doctest) — tokenless `to_agent_state` rejected.
2. `crates/tracewake-content/tests/fixtures_load.rs` — validated-path equivalence.

### Commands

1. `cargo test -p tracewake-content --doc`
2. `cargo test --workspace`
