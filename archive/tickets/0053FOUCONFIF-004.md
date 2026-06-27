# 0053FOUCONFIF-004: Sealed loaded-world bootstrap unrepresentability (atomic cutover, topology (b))

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime/state authority surface (sealed `ValidatedLoadedWorldBootstrap`, removal of public `from_loaded_state` / `from_seed_parts`); `tracewake-content` loader/schema migration; new external negative fixture
**Deps**: 0053FOUCONFIF-001

## Problem

Spec 0053 §4.1 (F5-01): `LoadedWorldBootstrap::from_loaded_state` is a public raw constructor taking authoritative aggregates, and `PhysicalState::from_seed_parts` / `AgentState::from_seed_parts` are public seed-part constructors. The validated content loader reaches the runtime through the *same* public constructor, so "validated loaded-world bootstrap" is a convention rather than a type boundary — an external crate can fabricate a bootstrap from raw parts and pass it to `from_bootstrap`. The existing negative fixture proves scheduler-injection is non-public; it does **not** prove bootstrap fabrication is impossible, so the negative perimeter is vacuous for this exact attack (§0 F5-01).

This ticket makes "validated loaded-world bootstrap" **unrepresentable outside the authority path** via the approved **topology (b)** (Q1, 2026-06-26): introduce an unforgeable `ValidatedLoadedWorldBootstrap` whose constructor is private to the runtime authority path; `from_bootstrap` accepts only that sealed product, not raw state/log/projection parts. It is an **atomic cutover** (local compile-atomicity): the public raw constructors are deleted and every in-workspace consumer migrates in one diff, leaving **no** backwards-compatible alias (§4.1, §9 step 3).

## Assumption Reassessment (2026-06-26)

1. `crates/tracewake-core/src/runtime/session.rs` exposes `pub fn LoadedWorldBootstrap::from_loaded_state(...)` (line 117) beside `pub fn LoadedWorldRuntime::from_bootstrap(LoadedWorldBootstrap, SimTick)` (line 177). `crates/tracewake-core/src/state.rs` exposes `pub fn PhysicalState::from_seed_parts` (line 223) and `pub fn AgentState::from_seed_parts` (line 285). `crates/tracewake-content/src/load.rs:60` `into_runtime_bootstrap` calls `from_loaded_state`. **Blast radius (grepped this session)**: `from_seed_parts` has a *production* cross-crate consumer at `crates/tracewake-content/src/schema.rs:686` (PhysicalState) and `:805` (AgentState), plus integration-test consumers in `crates/tracewake-core/tests/{replay_temporal_frontier,salient_stop_actor_known,generative_lock,holder_known_interval_projection,event_schema_replay_gates,world_step_coordinator}.rs` and in-crate test modules (`agent/perception.rs`, `projections.rs`, `state.rs`). Sealing therefore forces moving seed-part state assembly into core under the sealed entry and migrating those consumers — scope is wider than the spec's named trio (`expand-scope-in-place`; spec text unchanged).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §4.1 (topology options + anti-regression guard), §10.1 (Q1 topology — **(b) chosen**: non-negotiable that no external crate can fabricate a validated bootstrap; Cargo features are not security), §9 step 3 (closure order). Sibling precedent: `archive/tickets/0052FOUCONFOU-001` built `LoadedWorldBootstrap` + `from_bootstrap` additively but left `from_loaded_state` public — this ticket closes the *class*, not another instance.
3. Cross-artifact boundary under audit: the `tracewake-core` ⇄ `tracewake-content` loader/schema seam (the loader→runtime handoff and the schema's seed-part assembly) and the external-crate public surface. Core must not depend on content or tui (`docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`); the sealed bootstrap is owned by core and produced from validated content material, preserving the one-way direction. Per topology (b) the seed-part assembly that today lives in `content/schema.rs` moves behind the core-private sealed constructor (content hands core validated material; core mints the sealed bootstrap).
4. Motivating invariants: INV-018 (deterministic replay is foundational), INV-022 (raw prose is not authoritative state), INV-092 (deterministic replay is tested), with INV-002/INV-023 (belief-before-truth / records separation) — authoritative state must not be fabricable from raw parts beside the validated path. `#[non_exhaustive]` and Cargo features are not authority boundaries (§4.1); a private constructor on a sealed type is.
5. This ticket touches the deterministic-replay / truth-firewall enforcement surface: the bootstrap is the authoritative initial-state product replay reconstructs. The reseal introduces no leakage and no nondeterminism — the sealed bootstrap is a deterministic function of accepted validated configuration, carries no actor-known field, and changes no event-ordering / hash / serialization semantics; it only removes the public ability to *fabricate* one. A temporary `pub(crate)` / `#[cfg(feature = "test-support")]` constructor may exist solely to migrate core tests and MUST be removed (or remain test-support-gated, never a public production alias) before closeout (§9).
6. Schema/visibility change (additive-vs-breaking): `from_bootstrap`'s accepted input changes from `LoadedWorldBootstrap` (raw-constructible) to the sealed `ValidatedLoadedWorldBootstrap`; consumers are the content loader (`load.rs`/`schema.rs`) and any in-workspace runtime constructor. This is **breaking-internal** (a deliberate reseal, no external API stability owed; no backwards-compatible alias per §4.1).
7. Removal blast radius (grep-proof, item-7): deleting public `from_loaded_state`, `PhysicalState::from_seed_parts`, and `AgentState::from_seed_parts` — consumers enumerated in item 1. Each consumer either routes through the sealed entry (production: `content/load.rs`, `content/schema.rs`) or migrates to a `test-support`-gated constructor (the integration-test + in-crate-test set). The exact test-file set is an implementation-discovered set verified at implementation; no public raw constructor or alias survives.

## Architecture Check

1. Topology (b) — a sealed `ValidatedLoadedWorldBootstrap` with a runtime-private constructor — is cleaner than (a) folding all validation into core (largest blast radius, churns the content schema wholesale) or (c) a dedicated internal authority crate (new workspace member, heavier dependency surgery) while satisfying the same non-negotiable: no external crate can construct a validated bootstrap. The atomic cutover (delete-and-migrate in one diff) is mandated because a split that left `from_loaded_state` live behind a wrapper would re-create the exact "correct constructor beside a raw one" defect the fifth pass found.
2. No backwards-compatibility aliasing or durable shim. The only permitted transitional construct is a `pub(crate)` / `test-support`-gated constructor for test migration, removed or permanently test-gated before closeout — never a public production alias (§4.1, §9).

## Verification Layers

1. INV-018/INV-092 (deterministic replay) -> replay/golden-fixture check: a golden scenario loaded through the sealed path replays byte-identically; the sealed bootstrap reconstructs identical initial aggregates from validated configuration.
2. INV-022/INV-002/INV-023 (no fabricated authoritative state) -> external negative fixture (compile-fail): an external crate cannot construct `ValidatedLoadedWorldBootstrap`, cannot call `from_loaded_state`/`from_seed_parts` or any successor raw constructor, and cannot feed raw parts to `from_bootstrap`.
3. Kernel boundary -> dependency-direction grep-proof: core does not depend on content/tui; the sealed bootstrap is core-owned and produced from validated content material (`Cargo.toml` members + import direction).
4. Cross-artifact: the content loader still produces a runtime through the sealed path with no caller-injected scheduler actor/process registration (positive public-boundary witness).

## What to Change

### 1. Sealed `ValidatedLoadedWorldBootstrap` + resealed `from_bootstrap` (`runtime/session.rs`)

Introduce `ValidatedLoadedWorldBootstrap` with a constructor private to the runtime authority path. Change `from_bootstrap` to accept only the sealed product. Delete the public `from_loaded_state` raw constructor (no alias). Provide the core-private mint consumed only by the validated content entry point.

### 2. Seal seed-part state assembly (`state.rs`, move assembly into core)

Make `PhysicalState::from_seed_parts` and `AgentState::from_seed_parts` non-public (delete from the public surface; relocate the assembly behind the core-private sealed-bootstrap constructor). Provide a `#[cfg(feature = "test-support")]` constructor for test migration only.

### 3. Content loader/schema migration (`tracewake-content/src/load.rs`, `tracewake-content/src/schema.rs`)

Route `into_runtime_bootstrap` (and the schema's seed-part assembly at `schema.rs:686,805`) through the new sealed core entry: content hands core validated material; core mints the sealed bootstrap. Remove the content-side raw `from_seed_parts` calls.

### 4. External negative fixture (`tests/negative-fixtures/…`, new) + runner registration

Add `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` (compiled by `negative_fixture_runner.rs`) that imports `LoadedWorldBootstrap`/`ValidatedLoadedWorldBootstrap`, attempts `from_loaded_state` / `from_seed_parts` / any successor raw constructor, builds `PhysicalState`/`AgentState` from seed parts, and feeds the result to `from_bootstrap` — and must **fail to compile** for privacy/constructor reasons (not a runtime rejection), under default and all supported feature combinations, pinned to a privacy/constructor diagnostic (not a generic "cannot find function"). Register it in `negative_fixture_runner.rs`.

### 5. Test-consumer migration (implementation-discovered set)

Migrate the integration-test + in-crate-test consumers of `from_seed_parts` to the `test-support`-gated constructor: candidates `crates/tracewake-core/tests/{replay_temporal_frontier,salient_stop_actor_known,generative_lock,holder_known_interval_projection,event_schema_replay_gates,world_step_coordinator}.rs` and in-crate modules `agent/perception.rs`, `projections.rs`, `state.rs` — the exact set as surfaced by the compiler.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/` (new — `Cargo.toml` + `src/lib.rs`)
- Integration/in-crate test consumers of `from_seed_parts` (modify — implementation-discovered set; candidates enumerated in What to Change §5, under `crates/tracewake-core/tests/` and `crates/tracewake-core/src/`)

## Out of Scope

- Sealing the embodied interval/receipt products (005) and token-gating debug/no-human commands (006).
- The `food_source` survivor family (007), the standing mutation run (009), and doc-truthing (008).
- Any new property-testing dependency (§1.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — the new `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` fixture fails to compile for privacy/constructor reasons under default and all supported feature combinations.
2. A positive public-boundary witness: the real content loader produces a runtime through the sealed path with no caller-injected scheduler actor/process registration.
3. `grep -nE "pub fn from_loaded_state|pub fn from_seed_parts" crates/tracewake-core/src/runtime/session.rs crates/tracewake-core/src/state.rs` returns nothing (public raw constructors removed, no alias).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean.

### Invariants

1. No public constructor, field, or feature lets an external crate fabricate a validated bootstrap or seed-part state (INV-018/022; Cargo features are not security).
2. Deterministic replay over a golden scenario loaded through the sealed path is byte-identical (INV-018/092).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` (new) — compile-fail proof of bootstrap unforgeability.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — register the new fixture, pinned to a privacy/constructor diagnostic.
3. A `tracewake-content` positive test that the loader produces a runtime through the sealed path (no scheduler injection).

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo build -p tracewake-content --all-targets && cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-26

Implemented the atomic loaded-world bootstrap reseal by introducing
`ValidatedLoadedWorldBootstrap`, changing `LoadedWorldRuntime::from_bootstrap`
and `RuntimeReplaySeed::reconstruct_bootstrap` to use that validated product,
and removing the public `LoadedWorldBootstrap::from_loaded_state` entry. The
content loader now returns the validated bootstrap product through
`ValidatedLoadedWorldBootstrap::from_validated_content`, while the existing
positive loader/runtime test proves the real fixture path still produces a
runtime with scheduler-free caller input.

Removed the public `PhysicalState::from_seed_parts` and
`AgentState::from_seed_parts` names, migrated content schema materialization to
the validated seed-part entry, and migrated core tests to either the validated
entry or `test-support`-gated `from_test_seed_parts` helpers. Added and
registered
`external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts`, which
fails under default and `tracewake-core/test-support` feature runs because an
external crate cannot call the deleted raw seed/bootstrap constructor names.
Grep proof for
`pub fn from_loaded_state|pub fn from_seed_parts` over
`crates/tracewake-core/src/runtime/session.rs` and
`crates/tracewake-core/src/state.rs` returned no matches.

Verification:

- `cargo test -p tracewake-core --test negative_fixture_runner`
- `cargo build -p tracewake-content --all-targets`
- `cargo test -p tracewake-content`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
