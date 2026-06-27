# 0054FOUCONSIX-001: Re-sealed validated-bootstrap construction (atomic cutover)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime/state authority surface (reseal `PhysicalState::from_validated_seed_parts` / `AgentState::from_validated_seed_parts` / `ValidatedLoadedWorldBootstrap::from_validated_content`); `tracewake-content` loader/schema assembly migration; rewritten external negative fixture
**Deps**: None

## Problem

An external crate can today forge a "validated" loaded-world bootstrap from caller-authored aggregates and mint a runtime from it. `PhysicalState::from_validated_seed_parts` and `AgentState::from_validated_seed_parts` (`crates/tracewake-core/src/state.rs:223`, `:314`) are `pub fn` accepting arbitrary `BTreeMap` aggregates; `ValidatedLoadedWorldBootstrap::from_validated_content` (`crates/tracewake-core/src/runtime/session.rs:127`) is `pub fn` accepting a raw `ActionRegistry`, `PhysicalState`, `AgentState`, `EventLog`, `EpistemicProjection`, and content IDs; and `LoadedWorldRuntime::from_bootstrap` (`session.rs:191`) consumes the product. Composing these public constructors fabricates authoritative state/log/projection beside the validated content path (`tracewake-content` → `LoadedFixture::into_runtime_bootstrap` → `from_validated_content`), so the "validated" path is a convention, not a type boundary (finding F6-01).

The 0053 negative fixture `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` attacks the obsolete names `from_seed_parts` / `from_loaded_state` (`expected_stderr: "no function or associated item named \`from_seed_parts\`"` in `crates/tracewake-core/tests/negative_fixture_runner.rs:89`; the fixture src calls `PhysicalState::from_seed_parts` / `LoadedWorldBootstrap::from_loaded_state`), so the negative perimeter is **vacuous** for the live API. This re-opens the F5-01 / F4-01 authority class the fifth pass renamed but did not close.

## Assumption Reassessment (2026-06-27)

1. `crates/tracewake-core/src/state.rs:223`/`:314` expose `pub fn from_validated_seed_parts(...)` on `PhysicalState` and `AgentState` taking arbitrary aggregate maps; `from_test_seed_parts` (`:252`) is already `#[cfg(any(test, feature = "test-support"))]`-gated and delegates to the public one. Confirmed against the live tree at `7660051`.
2. `crates/tracewake-core/src/runtime/session.rs:127` exposes `pub fn ValidatedLoadedWorldBootstrap::from_validated_content(...)` (8 args: registry, physical, agent, log, projection, manifest id, fixture id, content version) wrapping a private `LoadedWorldBootstrap`; `from_bootstrap` (`:191`) is `pub fn`; the inner `from_bootstrap` on `ValidatedLoadedWorldBootstrap` (`:156`) is already private. `crates/tracewake-content/src/load.rs:56` `LoadedFixture::into_runtime_bootstrap` calls `from_validated_content` (`:60`). Confirmed.
3. Shared boundary under audit: the `tracewake-core` ↔ `tracewake-content` cross-crate construction seam. Because `tracewake-content` is a separate crate that must legitimately assemble state, `pub(crate)` alone may be insufficient — the spec assigns the authority-topology choice to the implementer (§10.1 options a/b/c). This ticket carries the **choice-and-record obligation**, not an open question: record the selected topology and rationale in this ticket's Architecture Check before implementation. Non-negotiable: no external crate can fabricate a validated bootstrap; Cargo features are not security.
4. INV-001 (causality from modeled/authored state), INV-009/INV-011 (meaningful state requires events; no current-state-only), INV-018/INV-092 (deterministic replay is foundational and tested), INV-022 (raw prose/data is not authoritative state) — the bootstrap is the entry point through which authoritative state, the event log, and the epistemic projection become the replay seed; a public raw constructor lets non-validated state enter that authority, violating INV-001/INV-022. Restated before trusting the narrative.
5. Deterministic-replay surface: the bootstrap feeds `RuntimeReplaySeed` / `reconstruct_bootstrap` (`session.rs:150`/`:162`) and `DeterministicScheduler::from_loaded_world` (`session.rs:193`). The reseal must preserve byte-identical replay: the sealed product must carry exactly the aggregates the validated content path produces, with no nondeterministic input introduced. Confirm the positive content-loader path still reconstructs an identical runtime after the cutover (replay/golden-fixture check below).
6. Schema/visibility reseal (additive-vs-breaking = **breaking**, intentional): `from_validated_seed_parts` and `from_validated_content` change from a public cross-crate constructor surface to a sealed one. Consumers of the public surface: `crates/tracewake-content/src/schema.rs:686`/`:805` (production content assembly) and 17 core test sites (`tests/support/mod.rs:85`/`:140`, `tests/world_step_coordinator.rs`, `tests/replay_temporal_frontier.rs`, `tests/holder_known_interval_projection.rs`, `tests/salient_stop_actor_known.rs`, `tests/event_schema_replay_gates.rs:1176`). All in-workspace, so this is a **local compile-atomicity** unit — every consumer migrates to the sealed assembly API (or `from_test_seed_parts`/test-support for the test sites) in the same diff or the tree will not compile. No backwards-compatible alias is left.
7. Removal blast radius of the old public surface: grep across `crates/`, `tests/`, `.claude/skills/`, `docs/`, `specs/` for `from_validated_seed_parts` / `from_validated_content` / any newly-removed public constructor confirms the only non-test production consumer is `tracewake-content/src/schema.rs`; the negative fixture `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` (and its `expected_stderr` in `negative_fixture_runner.rs:88-90`) must be rewritten to the live symbols. No doc or sibling-spec deliverable names the public constructor as a committed surface (only the archived 0053 spec, which is immutable history).

## Architecture Check

1. Sealing "validated loaded-world bootstrap" at the **type boundary** (a sealed package/witness owned by the validated-content path, or crate-private constructors with a production assembly API) makes the bad state unrepresentable rather than merely discouraged — the cycle-breaking layer the fourth/fifth passes missed by renaming instead of resealing. A convention beside a public raw constructor is not a boundary; a sealed product type is.
2. **Authority-topology decision (implementer-recorded, §10.1):** record here which of (a) crate-private/test-support-only seed-part constructors + a production assembly API that never accepts caller-built maps, (b) an unforgeable validation/assembly witness token obtainable only via the content-validation path, or (c) a content-owned sealed package type with private fields bound to schema/provenance validation, is chosen — with the boundary/dependency-direction rationale. Core must not depend on content; the sealed authority lives in `tracewake-core` (or a dedicated internal authority crate preserving the one-way direction). No backwards-compatibility aliasing/shims; a temporary internal adapter to migrate core tests, if any, is removed before closeout.

## Verification Layers

1. Unrepresentability (INV-001/INV-022) → external negative fixture compile-fail proof (codebase grep-proof + `negative_fixture_runner.rs` cargo-driven compile assertion), pinned to a privacy/constructor diagnostic on the **live** symbols.
2. Validated path preserved (INV-018) → replay/golden-fixture check: a positive content-loader integration test proves `LoadedFixture::into_runtime_bootstrap` still reaches `from_bootstrap` through the sealed validated path with no caller-injected scheduler actor/process registration.
3. Determinism (INV-092) → deterministic-replay assertion that the reconstructed runtime is byte-identical pre/post cutover for a golden fixture.
4. Boundary (arch 01) → `cargo build --workspace` proves the dependency direction holds (core does not gain a content dependency).

## What to Change

### 1. Reseal the seed-part and bootstrap constructors

Apply the recorded topology choice so that no external crate can assemble a `PhysicalState`/`AgentState` from raw aggregate maps and pass them as "validated", and `from_validated_content` accepts only a sealed product, not raw state/log/projection parts. Remove the public raw constructors with no alias.

### 2. Migrate the validated content assembly path

Move `tracewake-content/src/schema.rs` (`:686`/`:805`) and `load.rs` (`:56`/`:60`) onto the sealed assembly API so the production content path remains the single legitimate constructor of authoritative state.

### 3. Migrate in-workspace test consumers

Repoint the 17 core test sites (item-6 list) to `from_test_seed_parts` / the test-support assembly path so the workspace compiles in one diff.

### 4. Rewrite the negative fixture to attack live symbols

Rewrite `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` to import the **live** symbols (`PhysicalState::from_validated_seed_parts`, `AgentState::from_validated_seed_parts`, `EventLog::new`, `EpistemicProjection::new`, `ValidatedLoadedWorldBootstrap::from_validated_content`) and attempt to reach `LoadedWorldRuntime::from_bootstrap`; update `negative_fixture_runner.rs:88-90` `expected_stderr` to the post-reseal privacy/constructor diagnostic (not a stale "cannot find function"). It must fail to compile under default and all supported feature combinations.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `crates/tracewake-core/tests/support/mod.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)
- `crates/tracewake-core/tests/replay_temporal_frontier.rs` (modify)
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (modify)
- `crates/tracewake-core/tests/salient_stop_actor_known.rs` (modify)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- new sealed-authority module (new — only if the recorded topology introduces a dedicated type/module; otherwise the reseal lands in `state.rs`/`session.rs`)

## Out of Scope

- The one-tick wait receipt seal (F6-02, ticket 002) and debug-authority reseal (F6-03, ticket 003).
- Any acceptance-taxonomy, governance, or mutation-CI change (tickets 004–006).
- Live-conformance doc-truthing for the bootstrap row (deferred to ticket 010, post-closure).
- Adding any property-testing dependency (`proptest`/`quickcheck`) — forbidden by the spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — the rewritten `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` fails to compile with the pinned privacy/constructor diagnostic on the live symbols, under default and all supported feature combinations (`production_boundary_negative_fixtures_fail_with_test_support_feature`).
2. A positive content-loader integration test proves `LoadedFixture::into_runtime_bootstrap` → `from_validated_content` → `from_bootstrap` still constructs a runtime through the sealed path.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` — the full workspace compiles and passes after the in-workspace consumer migration.

### Invariants

1. No public symbol in any crate constructs a `ValidatedLoadedWorldBootstrap` (or its predecessor aggregates) from caller-built state/log/projection parts; the sealed product is the only route to `from_bootstrap`.
2. Deterministic replay over a golden fixture reconstructs a byte-identical runtime pre/post cutover (INV-018).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` — rewritten to attack live symbols; the proof that the authority class is closed.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — updated `expected_stderr` for the live-symbol diagnostic.
3. A positive sealed-path content-loader test (extend `crates/tracewake-content/src/load.rs` test module or a `tracewake-content` integration test) — proves the legitimate path still reaches the runtime.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/src/runtime/session.rs -f crates/tracewake-core/src/state.rs` — focused campaign over the resealed constructors for fast feedback (the standing campaign is ticket 009).
