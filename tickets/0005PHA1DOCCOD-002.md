# 0005PHA1DOCCOD-002: Phase-scoped content loading — `FixtureScope` + per-scope registry + migrate the existing fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-content` schema (`FixtureScope` field), fixture loader, content validators, all in-repo fixtures, and the content test suites.
**Deps**: 0005PHA1DOCCOD-001

## Problem

The shared fixture loader `load_fixture_package` (`crates/tracewake-content/src/load.rs:40`) — currently the *only* loader, despite the Phase 1 framing — builds an `ActionRegistry` and registers every phase before validation (`load.rs:51-59`). So even a genuine Phase 1 fixture is validated against an all-phase registry, and a Phase 1 content package cannot structurally reject a Phase 2A/3A action ID. The parity validator `validate_action_registry_parity` (`crates/tracewake-content/src/validate.rs:865-903`) accepts any affordance whose `action_id` resolves in the passed registry; `validate_routine_rules` (`validate.rs:905-965`) accepts any routine step whose semantic base maps to a registered action. This is the content half of `ALIGN-001`: a Phase 1 validation pass can be broadened by convenience registration instead of a deliberate certification boundary, producing a false `P0-CERT` proof surface.

This ticket makes content phase scope **parsed data, not inferred**: each fixture declares a `FixtureScope`; the loader selects the registry from that scope; a `Phase1` fixture rejects later-phase action IDs before runtime with typed diagnostics; later-phase fixtures load under their historical scope as boundary evidence only.

## Assumption Reassessment (2026-06-09)

1. `load_fixture_package` at `crates/tracewake-content/src/load.rs:40` registers all phases at `:51-59`. `validate_action_registry_parity` (`validate.rs:865-903`) and `validate_routine_rules` (`validate.rs:905-965`) accept any registered action. The forbidden-content test helper `registry()` repeats the all-phase setup (`crates/tracewake-content/tests/forbidden_content.rs:14-24`).
2. Host structures for the scope field, all confirmed present: `FixtureSchema` (`crates/tracewake-content/src/schema.rs:214`), `GoldenFixture` + `FixtureContract` (`crates/tracewake-content/src/fixtures/mod.rs:81`, `:87`), `ContentManifest` (`crates/tracewake-content/src/manifest.rs:4`). Spec `ALIGN-REQ-002` says "fixture/manifest schema"; implementation attaches `FixtureScope` where the loader can read it before registry selection (the `GoldenFixture`/`FixtureContract` metadata and the serialized `FixtureSchema` are the candidates — pick the one the loader consumes, and serialize it through canonicalization so it survives round-trip).
3. Shared boundary under audit: the `tracewake-core` `ActionRegistry` / `ActionScope` contract (introduced in `0005PHA1DOCCOD-001`) consumed by this loader. This ticket selects which scopes a given load registers, keyed by the fixture's declared `FixtureScope`.
4. Invariants motivating this ticket: `INV-097` (no-script compliance is tested — content must be inspected for boundary violations structurally), `INV-060`/`INV-061` (no authored outcome chains / authored causal machinery is possibility space, validated), and `INV-098` (harsh acceptance — the Phase 1 fixture validator must be able to produce a clean Phase-1-only certification artifact).
5. Fail-closed-validation surface touched: content validation runs before runtime and is blocking. The new phase-boundary rejection must be a hard error (a `ContentValidationError`), not a warning, and must reject unknown/out-of-scope action IDs by default. Confirm no epistemic leakage is introduced (scope is registry metadata, carries no hidden truth) and no replay nondeterminism (scope selection is deterministic from declared data; canonical manifest/checksum for a Phase 1 fixture is unchanged except for the new declared-scope field). The enforcement is consumed by the negative fixtures in `0005PHA1DOCCOD-003`.
6. Schema extension: add `FixtureScope` to the fixture/manifest schema. Consumers: every fixture constructed in `crates/tracewake-content/src/fixtures/` (the `GoldenFixture` builders and `fixtures::all()` at `mod.rs:112`), `load_fixture_package`, the validators, and the test suites that load fixtures. This is **breaking, not additive** — every fixture must declare a scope, so all consumers update in this diff (local compile-atomicity).
7. Migration blast radius (load-bearing): of 28 fixture files, **15 exercise later-phase actions** (`eat`/`sleep`/`work_block`/`continue_routine`/`check_container`/`truthful_accuse_probe`) and MUST be declared a later-phase `FixtureScope` so they keep loading rather than being rejected by the new Phase 1 boundary: `no_human_day_001`, `ordinary_workday_001`, `sleep_eat_work_001`, `planner_trace_001`, `routine_no_teleport_001`, `routine_blocked_diagnostic_001`, `food_unavailable_replan_001`, `expectation_contradiction_001`, `knowledge_blocker_accuse_001`, `no_hidden_truth_planning_001`, `no_human_epistemic_check_001`, `possession_parity_001`, `possession_does_not_reset_intention_001`, `sound_uncertainty_001`, `view_filtering_001`. The remaining 13 are Phase 1. **Re-derive this list at implementation time** — `grep -rln '"eat"\|"sleep"\|"work_block"\|"continue_routine"\|"check_container"\|"truthful_accuse_probe"' crates/tracewake-content/src/fixtures/` plus the inline fixture in `mod.rs` — rather than trusting this snapshot. `golden_fixtures_run.rs` iterates `fixtures::all()` (`:155`, `:1337`) and loads several later-phase fixtures by name (`ordinary_workday_001` `:187`, `sleep_eat_work_001` `:249`, `no_human_day_001` `:838`/`:1168`, etc.) — each call site must load under the fixture's declared scope, and `no_human_day_001`'s no-human-day proof must keep passing.
8. Adjacent contradiction: this ticket is deliberately Large (a single atomic diff) rather than split. Splitting the `FixtureScope` field, the per-scope loader routing, and the 15 fixture declarations would land an intermediate state where `golden_fixtures_run` regresses (a scope field with no routing leaves fixtures all-phase; routing with no declared scopes does not compile). This is a required consequence of behavior- and compile-atomicity, not future cleanup.

## Architecture Check

1. Representing phase scope as parsed per-fixture data ("parse, don't validate") makes a misscoped fixture a structural impossibility at load selection time rather than a missed runtime check — it realizes spec §11 suggestion #3 as a requirement. Routing the registry from the declared scope means a Phase 1 load physically cannot see later-phase action definitions, so rejection is by construction, with the typed diagnostic as a precise, debuggable failure.
2. No backwards-compatibility shim: the all-phase `load_fixture_package` registration block is replaced by scope-keyed registration, not wrapped. There is no fallback path that re-admits all phases for a Phase 1 fixture.

## Verification Layers

1. `INV-097` (no-script / boundary compliance tested) -> schema validation: a Phase 1 fixture with a later-phase `action_id` produces a blocking `ContentValidationError` with a typed phase-boundary phase/code, exercised by `0005PHA1DOCCOD-003`.
2. `INV-098` (clean Phase 1 certification surface) -> replay/golden-fixture check: `cargo test -p tracewake-content` — all 28 fixtures load under their declared scope; the 13 Phase 1 fixtures load under `FixtureScope::Phase1`, the 15 later-phase under historical scope.
3. Deterministic canonicalization (`INV-018`) -> replay/golden-fixture check: a Phase 1 fixture's canonical manifest/checksum is unchanged except for the new declared-scope field; `golden_fixtures_run` round-trip determinism (`:155`, `:1337`) still passes.
4. Fail-closed validation -> manual review + grep: the phase-boundary rejection is pushed as a `ContentValidationError` (blocking), not a warning, and the typed `ValidationPhase`/error code is asserted (not a display-string substring).

## What to Change

### 1. Add `FixtureScope` to the fixture/manifest schema

Define `FixtureScope { Phase1, Phase2AHistorical, Phase3AHistorical }` in the content schema and carry it on the fixture/manifest structure the loader reads before registry selection (`FixtureSchema`/`FixtureContract`/`ContentManifest` per Assumption 2). Serialize it through the existing canonicalization so it survives round-trip.

### 2. Route registry selection by declared scope in `load_fixture_package`

Replace the unconditional all-phase registration (`load.rs:51-59`) with scope-keyed registration: a `Phase1` fixture registers only `register_phase1_*`; a later-phase fixture additionally registers the corresponding historical families. A Phase 1 fixture's registry must not contain later-phase action IDs.

### 3. Make the parity + routine validators emit a typed phase-boundary diagnostic

In `validate.rs`, when an affordance `action_id` or a routine semantic base is absent from the (scope-selected) registry, ensure the error is a typed phase-boundary rejection — reuse `ValidationPhase::ActionRegistryParity` with a stable code such as `phase_unsupported_action`, or introduce `ValidationPhase::PhaseBoundary` if cleaner. Do not rely on display text or substring matching on `"phase3a"`.

### 4. Declare scope on all 28 fixtures and migrate the 15 later-phase ones

Set `FixtureScope::Phase1` on the 13 Phase 1 fixtures and the correct historical scope on the 15 later-phase fixtures (Assumption 7). Update the `GoldenFixture` builders / `fixtures::all()` in `crates/tracewake-content/src/fixtures/mod.rs` and the per-fixture files.

### 5. Update content test suites to load under declared scope

In `forbidden_content.rs` (helper `registry()`), `schema_conformance.rs`, `fixtures_load.rs`, and `golden_fixtures_run.rs`, load each fixture under its declared scope; keep every existing positive run green, including `no_human_day_001`.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/manifest.rs` (modify — only if `FixtureScope` lands on the manifest)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/src/fixtures/*.rs` (modify — the 28 per-fixture files, as surfaced by the Assumption-7 grep; Phase 1 vs historical scope per file)
- `crates/tracewake-content/src/serialization.rs` (modify — if the scope field needs canonicalization)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `crates/tracewake-content/tests/schema_conformance.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)

## Out of Scope

- The core `ActionScope` retype (`0005PHA1DOCCOD-001`, dependency).
- The dedicated adversarial Phase 1 negative fixtures asserting each later-phase family rejection (`0005PHA1DOCCOD-003`).
- The pipeline-level guard reachability/compile-fail proof (`0005PHA1DOCCOD-004`).
- The source-level guard against direct later-phase registration from the Phase 1 loader (`0005PHA1DOCCOD-005`).
- CI wiring of new gates (`0005PHA1DOCCOD-006`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test fixtures_load` — all 28 fixtures load under their declared scope.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — every existing golden/no-human run (incl. `no_human_day_001`) still passes; round-trip determinism holds.
3. `cargo test --locked -p tracewake-content --test schema_conformance` — fixture/content phase scope is represented structurally.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace` — full tree green.

### Invariants

1. A Phase 1 fixture's selected registry contains no later-phase action IDs; a later-phase action in a Phase 1 affordance or routine step is a blocking, typed `ContentValidationError` (`INV-097`).
2. A Phase 1 fixture's canonical manifest/checksum is unchanged except for the declared-scope field; identical inputs replay byte-identically (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — assert each fixture's declared `FixtureScope` and that it loads under it.
2. `crates/tracewake-content/tests/schema_conformance.rs` — assert the schema carries and canonicalizes `FixtureScope`.
3. `crates/tracewake-content/tests/golden_fixtures_run.rs` — route each named later-phase fixture through its historical scope; keep determinism/no-human assertions.

### Commands

1. `grep -rln '"eat"\|"sleep"\|"work_block"\|"continue_routine"\|"check_container"\|"truthful_accuse_probe"' crates/tracewake-content/src/fixtures/` — re-derive the later-phase fixture set before declaring scopes.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
