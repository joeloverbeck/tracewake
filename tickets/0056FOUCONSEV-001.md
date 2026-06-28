# 0056FOUCONSEV-001: Seal validated-content bootstrap construction against the live API

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` state/runtime sealing, `tracewake-content` load/schema/manifest authority topology, the `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` negative fixture, and in-workspace test-consumer migration
**Deps**: None

## Problem

Finding F7-01. The sixth-pass hardening (`0054`) sealed `PhysicalState::from_validated_seed_parts` / `AgentState::from_validated_seed_parts` to `pub(crate)`, but a public validated-content alias still forges a loaded-world bootstrap outside the validation gate. At `2720167`:

- `crates/tracewake-core/src/state.rs:281` and `:391` expose `#[doc(hidden)] pub fn PhysicalState::from_validated_content_parts(...)` / `AgentState::from_validated_content_parts(...)`, each delegating to the sealed seed-part constructor. `#[doc(hidden)]` hides Rustdoc, not item visibility — these are `pub fn` and externally callable.
- `crates/tracewake-content/src/load.rs:46` `LoadedFixture` exposes public authoritative-state fields (`canonical_world`, `canonical_agent_state`, `epistemic_projection`, `seed_event_log`) and a public `into_runtime_bootstrap(...)` (`:56`); `crates/tracewake-content/src/manifest.rs:16` `ContentManifest::new(...)` is public.
- `crates/tracewake-core/src/runtime/session.rs:127` `ValidatedLoadedWorldBootstrap::from_validated_content(...)` accepts already-materialized `PhysicalState` / `AgentState` / `EventLog` / `EpistemicProjection`, and `LoadedWorldRuntime::from_bootstrap(...)` (`:191`) is public.

An external crate can therefore fabricate authoritative aggregates, wrap them as "validated content," and reach the runtime constructor without traversing the content validation gate. The `0054` negative fixture attacks the now-private `from_validated_seed_parts` route (E0624), so it is vacuous for this live route. This re-opens the loaded-world bootstrap authority class the sixth pass closed by named instance only.

## Assumption Reassessment (2026-06-28)

1. Live API verified at `2720167`: `state.rs:223/343` `from_validated_seed_parts` is `pub(crate)` (preserve); `state.rs:281/391` `from_validated_content_parts` is `#[doc(hidden)] pub fn` (the live hole). `load.rs:49-52` `LoadedFixture` fields are `pub`; `load.rs:56` `into_runtime_bootstrap` is `pub`; `manifest.rs:16` `ContentManifest::new` is `pub`; `session.rs:127` `from_validated_content` takes raw materialized parts; `session.rs:191` `LoadedWorldRuntime::from_bootstrap` is `pub`.
2. Spec `specs/0056_FOUNDATIONAL_CONFORMANCE_SEVENTH_HARDENING_*.md` §4.1 + §10.1 + §9 govern this ticket; the cross-crate authority topology is a spec-assigned implementer-recorded choice (see item 3). Driver: `reports/0047-foundational-hardening-research-report-seventh-pass.md` F7-01.
3. **Cross-crate authority boundary under audit**: `tracewake-content` is a separate crate, so `pub(crate)` in `tracewake-core` alone cannot seal a constructor `tracewake-content` must call. The decision is the implementer's, recorded in this ticket's Architecture Check + What to Change, chosen from §10.1: (a) move content validation/materialization into `tracewake-core` so the authoritative constructors are crate-private; (b) `tracewake-content` returns a core-owned sealed `ValidatedContentArtifact` (proposed new type) minted only by a core validation entrypoint; (c) `tracewake-content` passes raw authored data/events to a core validation/materialization function that owns final aggregate construction. Requirement: unforgeability outside the authority path. **Cargo features are not security.** No backwards-compatible alias or public raw-aggregate replacement. Core must not depend on content or tui (`docs/1-architecture/01_*`).
4. INV-009 (meaningful state changes require events), INV-011 (current-state-only simulation forbidden), INV-018 (deterministic replay is foundational), INV-061 (authored causal machinery is required), INV-062 (scenario seeds are tensions not scripts), INV-063 (authored prehistory must be marked), INV-092 (deterministic replay is tested), INV-098 (feature acceptance is harsh) — the validated-content gate is what guarantees authoritative state/log/projection carry validated provenance rather than fabricated parts; a forgeable bootstrap lets unvalidated aggregates enter the runtime, defeating provenance and replay discipline.
5. **Fail-closed / replay enforcement surface**: the content validation gate (`crates/tracewake-content/src/{schema,validate,load}.rs`) and the runtime bootstrap (`runtime/session.rs`). Sealing the construction path strengthens fail-closed validation (authoritative state is constructible only through the validating loader) and does not weaken deterministic replay — the sealed constructor produces the same materialized aggregates the current loader does; only the *reachability* from outside the gate is removed. No actor-knowledge surface is touched (this is construction authority, not projection).
6. **Visibility/shape change (reseal — breaking, intra-workspace)**: `from_validated_content_parts` (×2), the `LoadedFixture` public state fields, `into_runtime_bootstrap`, `ContentManifest::new`, and `from_validated_content`'s parameter shape are narrowed/replaced. Consumers of these symbols: the production validating loader `content/src/schema.rs:686/805` (materializes canonical state — must route through the chosen sealed topology); `content/src/load.rs:590` and `tui/src/app.rs:87/88` (`into_runtime_bootstrap`); `content/src/load.rs:85` + `content/src/manifest.rs` tests (`ContentManifest::new`); and ~13 core test call sites of `from_validated_content_parts` (`tests/world_step_coordinator.rs`, `tests/replay_temporal_frontier.rs`, `tests/support/mod.rs`, `tests/salient_stop_actor_known.rs`, `tests/event_schema_replay_gates.rs`, `tests/holder_known_interval_projection.rs`). The change is **breaking** within the workspace and, per §Local compile-atomicity, the reseal plus all in-workspace consumer migrations land in this one ticket so the tree always compiles. Test consumers migrate to a test-support-gated constructor (the existing `tracewake-core` test-support feature) rather than the public alias.
7. **Removal blast radius of the public surface** (grepped repo-wide): `from_validated_content_parts` appears only under `crates/` (production `content/src/schema.rs` + core tests enumerated in item 6) — no `docs/`, `specs/` (outside `0056`), `.claude/skills/`, or sibling-spec consumers. `into_runtime_bootstrap` / `ContentManifest::new` likewise confined to `crates/`. The `#[doc(hidden)]` attribute means no Rustdoc/doc consumers exist. Per §9, a temporary internal adapter to migrate core tests may exist only transiently and must be removed before closeout; no public alias survives.

## Architecture Check

1. Compile-time unrepresentability on the *live* public symbols is stronger than the `0054` named-instance seal: by sealing `from_validated_content_parts`, privatizing the `LoadedFixture` state fields, and making `from_validated_content` accept only the sealed product, "validated loaded-world bootstrap" becomes unconstructable outside the authority path for any external crate, regardless of feature flags. This closes the authority *class*, not one named route, breaking the re-opening cycle the fourth–seventh passes diagnosed.
2. No backwards-compatibility aliasing or shims: the public alias is removed outright, not wrapped; the chosen topology (item 3) replaces the cross-crate handoff with a sealed product. The temporary core-test adapter, if used, is internal and removed before closeout (§9). Core retains zero dependency on content/tui — the sealed authority lives in `tracewake-core` (or a dedicated internal authority module within it).

## Verification Layers

1. INV-009/INV-061/INV-063 (validated provenance) -> external-crate negative fixture compile-fail: the repaired `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` (renamed/retargeted to the live route) imports the live symbols and fails with a privacy/unconstructability diagnostic, not a stale method-name error.
2. INV-011/INV-018/INV-092 (replay over validated state) -> positive content-loader integration test: the real loader path still loads content, materializes canonical state through the sealed topology, and derives due-work from core-owned scheduler discovery; deterministic-replay/golden coverage over the loaded world is unchanged.
3. INV-098 (harsh acceptance) -> `public-boundary-conformance` CI job compiling the negative fixture under default and all supported feature combinations (mapped to its own proof surface, distinct from the positive loader test).
4. Mutation sensitivity -> in-diff/standing mutation coverage of `state.rs`, `runtime/session.rs`, the content validation/materialization files, and any new sealed proof type (perimeter wiring owned by 0056FOUCONSEV-005; survivors recorded by -006).

## What to Change

### 1. Choose and record the cross-crate authority topology (§10.1)

Select one of (a)/(b)/(c) from Assumption Reassessment item 3, record the choice and the rejected alternatives with the boundary rationale here, and implement it so final authoritative-aggregate construction is owned by `tracewake-core` and unreachable from `tracewake-content` (or any external crate) except through the validating entrypoint.

### 2. Seal the live constructors and fields

- Remove public reachability of `PhysicalState::from_validated_content_parts` / `AgentState::from_validated_content_parts` — make crate-private, test-support-gated, or eliminate. No public raw-aggregate replacement, no alias.
- Make `LoadedFixture` state fields private if `LoadedFixture` remains the proof vehicle; construction only through the validating loader.
- `ValidatedLoadedWorldBootstrap::from_validated_content(...)` accepts only the sealed product, not raw `PhysicalState` / `AgentState` / `EventLog` / `EpistemicProjection`.
- `LoadedWorldRuntime::from_bootstrap(...)` may remain public only if the bootstrap type is genuinely unforgeable by external crates.

### 3. Migrate in-workspace consumers (same diff)

Route `content/src/schema.rs` materialization through the chosen sealed topology; update `content/src/load.rs` + `tui/src/app.rs` `into_runtime_bootstrap` call sites; migrate the ~13 core test call sites of `from_validated_content_parts` to the test-support-gated constructor. The tree must compile at every step (local compile-atomicity).

### 4. Repair the negative fixture to attack the live route (F7-07 part)

Rewrite `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` so it imports the **live** symbols and attempts the full attack: call `from_validated_content_parts` (×2); fabricate `ContentManifest` + `LoadedFixture { canonical_world, canonical_agent_state, epistemic_projection, seed_event_log, .. }` and `into_runtime_bootstrap(...)`; pass the bootstrap to `LoadedWorldRuntime::from_bootstrap(...)`. Rename the fixture if the seed-part framing no longer fits, and re-register it in `crates/tracewake-core/tests/negative_fixture_runner.rs` (`FIXTURES` + `ALL_FEATURE_PRODUCTION_BOUNDARY_FIXTURES`). The expected failure is a privacy/unconstructability diagnostic under default and all supported feature combinations.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/manifest.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify — only if the chosen topology relocates materialization here)
- `crates/tracewake-core/src/<sealed authority module>.rs` (new — only if topology (b)/(c) introduces a `ValidatedContentArtifact`)
- `crates/tracewake-tui/src/app.rs` (modify — `into_runtime_bootstrap` consumer at `:87`; coordinates with 0056FOUCONSEV-002)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)
- `crates/tracewake-core/tests/replay_temporal_frontier.rs` (modify)
- `crates/tracewake-core/tests/support/mod.rs` (modify)
- `crates/tracewake-core/tests/salient_stop_actor_known.rs` (modify)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — re-register the repaired fixture; coordinates with 0056FOUCONSEV-002)

## Out of Scope

- F7-02 debug-authority sealing (0056FOUCONSEV-002) — even though it shares `session.rs`/`app.rs`/`negative_fixture_runner.rs`; this ticket touches the bootstrap functions only.
- Acceptance-taxonomy work (F7-03/F7-04/F7-05/F7-06), doctrine synchronization (§6.1), and conformance-row truthing (§6.2).
- Re-sealing `from_validated_seed_parts` (already `pub(crate)` — preserved property, do not rebuild).
- The standing mutation campaign run (0056FOUCONSEV-006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — the repaired bootstrap fixture compile-fails with a privacy/unconstructability diagnostic; `registered_negative_fixtures_match_directory_census` passes.
2. The positive content-loader integration test loads content, materializes canonical state through the sealed topology, and derives due-work — proving the real path still works.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` — full tree compiles and passes after consumer migration.

### Invariants

1. No external crate can construct a `ValidatedLoadedWorldBootstrap` (or reach `LoadedWorldRuntime::from_bootstrap`) without traversing the content validation gate — proven by compile-fail, not runtime check.
2. No public alias, `#[doc(hidden)] pub fn`, or public raw-aggregate constructor for authoritative state survives; `from_validated_seed_parts` remains `pub(crate)`.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs` — retargeted to the live `from_validated_content_parts` / `LoadedFixture` / `from_validated_content` / `from_bootstrap` attack chain.
2. A positive content-loader integration test (in `crates/tracewake-content/tests/` or the existing loader test module) — real loader path through the sealed topology.

### Commands

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo test -p tracewake-content` — narrower content-loader boundary for fast iteration on the topology choice.
