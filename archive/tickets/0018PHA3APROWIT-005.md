# 0018PHA3APROWIT-005: Authored food-knowledge edges in the seed grammar

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (schema field, load path, validation policy, fixture edits); one new adversarial fixture
**Deps**: `archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-040)

## Problem

`load.rs::seed_event_log` runs a nested loop — for every actor × every `fixture.food_supplies` entry it appends a `StartingBeliefRecorded` (`household_food_source`) carrying the food supply's true location. No schema field can express an actor who does *not* know a given food source, so every fixture starts with uniformly true, uniformly shared food-location belief. The seed grammar that can only express everyone-knows-everything forecloses the partial/wrong-belief mechanics the doctrine requires (INV-002 belief before truth; INV-025 wrong beliefs are first-class). Provenance marking itself is correct (`authored_prehistory`, INV-063 holds) — the gap is expressiveness, not marking.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: the nested `for actor in &fixture.actors { for food in &fixture.food_supplies { … } }` loop in `load.rs::seed_event_log` mints `household_food_source` starting beliefs unconditionally (single `household_food_source` site in `load.rs`); `FixtureSchema` (schema.rs) has no per-actor knowledge field; 50 fixture files under `crates/tracewake-content/src/fixtures/` reference `food_supplies`, so the explicit-edges default touches a wide but mechanical surface.
2. Spec 0018 ORD-HARD-040 (required correction + structural lock) and §9 risk note: the recommended default is generating explicit edges for existing fixtures once — preserving current behavior and golden checksums — with *unknown* as the default only for newly-authored fixtures going forward.
3. Cross-artifact boundary under audit: the seed-grammar contract between `FixtureSchema` (authoring), `validate.rs` registries (policy), and `load.rs` (materialization into `StartingBeliefRecorded` events) — authored possibility, not script (`docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`).
4. INV-002/INV-025 restated: actors act from their own beliefs, not hidden ground truth; partial and wrong beliefs are required mechanics, not edge cases — the authoring grammar must be able to express them.
5. Substrate feeding the actor-knowledge firewall: starting beliefs are cognition inputs with `authored_prehistory` provenance. Selective minting must not open a leakage path — an actor without an edge must have no `household_food_source` belief and no planner route to that food (the existing `hidden_food_*` gates pattern is the enforcement surface, exercised by the new fixture). Replay determinism: with explicit edges generated for existing fixtures, seed event streams are unchanged and golden checksums stay stable (spec §9 recommendation); only fixtures that deliberately omit an edge differ — the new fixture is additive.
6. Schema extension: `FixtureSchema` gains a per-actor known-food-sources edge field (e.g. authored `known_food_sources` on the actor entry or a top-level edge list — exact shape per `docs/2-execution/08` conventions). Additive: existing fixtures gain generated explicit edges in the same change, so no consumer sees a behavioral default flip. Consumers: `load.rs` (mint per edge), `validate.rs` (the field registries force classification — the string/numeric censuses fail until the new field is registered, by design), `schema_conformance.rs`/`fixtures_load.rs` (serialization parity).

## Architecture Check

1. Authored edges (default-unknown for new fixtures) make ignorance the natural state and knowledge an authored fact with provenance — matching how workplace and sleep-place knowledge already work (notice/observation-derived, INV-024-clean), instead of food being the one omnisciently-seeded resource. Generating explicit edges for the 50 existing fixtures keeps the change behavior-neutral and checksum-stable, per spec §9.
2. No backwards-compatibility aliasing/shims: no implicit "absent field means everyone knows" fallback — an absent edge list means *no one* knows; existing fixtures carry their edges explicitly.

## Verification Layers

1. INV-025 authorable ignorance -> new fixture `seeded_food_source_unknown_to_all_actors_001`: a food supply with no edges mints no `household_food_source` belief; replay byte-match.
2. INV-024 no leakage via planner -> the same fixture asserts the planner cannot target the unknown food (the `hidden_food_*` gate pattern: no proposal references it; actor-known surface omits it).
3. Schema policy coverage -> the `validate.rs` string/numeric field censuses pass only with the new field registered and policy-classified (census failure is the lock for future field drift).
4. INV-018 stability -> all existing golden fixtures replay byte-identically (explicit edges reproduce the current belief set exactly).

## What to Change

### 1. Schema field

Add the per-actor known-food-sources edge field to `FixtureSchema` (shape per the schema conventions in `docs/2-execution/08`), registered in `CONTENT_FIELD_REGISTRY` and the scan-policy registries.

### 2. Selective minting

`load.rs::seed_event_log` mints `household_food_source` starting beliefs only for authored edges; the unconditional nested loop is removed.

### 3. Explicit edges for existing fixtures

Generate the current actor×food edge set explicitly into the 50 `food_supplies`-bearing fixtures (mechanical, behavior-preserving edit; as surfaced by the census/test runs).

### 4. Adversarial fixture

New `seeded_food_source_unknown_to_all_actors_001` fixture (registered in `fixtures/mod.rs` and the `fixtures_load.rs` census) proving no-belief + planner-unreachable + replay byte-match.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (modify — explicit-edge insertion across the 50 `food_supplies`-bearing fixtures, as surfaced)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — fixture census)

## Out of Scope

- Workplace / sleep-place knowledge seeding — already notice/observation-derived; unchanged.
- The validation-token gate on materialization (ticket `0018PHA3APROWIT-006`, which shares `schema.rs`/`load.rs` and depends on this ticket).
- Any runtime knowledge-acquisition channel change — this is authoring-grammar only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content seeded_food_source_unknown` — no belief minted; planner-unreachable; replay byte-match.
2. `cargo test -p tracewake-content --test schema_conformance` + the field-registry censuses — new field classified and canonically serialized.
3. `cargo test -p tracewake-content --test golden_fixtures_run` — existing fixtures byte-stable with explicit edges.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. A `household_food_source` starting belief exists if and only if an authored edge exists; absence of edges means absence of belief.
2. Every new `FixtureSchema` field is registered and policy-classified before CI is green (existing census lock).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs` — ignorance fixture (no-belief, planner-unreachable, replay).
2. `crates/tracewake-content/tests/fixtures_load.rs` — census update; load-path unit coverage for selective minting.

### Commands

1. `cargo test -p tracewake-content seeded_food_source_unknown`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- Added `KnownFoodSourceSchema` and the canonical `known_food_source|actor_id|food_supply_id` fixture section.
- Changed `load.rs::seed_event_log` so `household_food_source` starting beliefs are minted only from authored `known_food_sources` edges, not from the physical actor x food-supply cross product.
- Added reference and duplicate-edge validation for `known_food_sources`.
- Updated existing Rust golden fixture builders to populate their previous actor x food edge set explicitly before canonical serialization, preserving current fixture behavior and golden checks.
- Added `seeded_food_source_unknown_to_all_actors_001`, proving a physical food supply with no authored edge mints no household-food belief, exposes no actor-known food source, and is not targeted by the no-human path.
- Updated fixture registry/census and source-classification census for the new fixture.

Deviations from original plan:

- Existing fixtures preserve behavior by calling `populate_known_food_sources_for_all_actors()` in the Rust fixture builders, which serializes explicit canonical `known_food_source` edges. The loader itself has no absent-field fallback.
- No string or numeric scan-policy entry was needed for `KnownFoodSourceSchema` because both fields are typed stable IDs; the content field registry owns the schema-level metadata.

Verification:

- `cargo fmt --all --check`
- `cargo test -p tracewake-content seeded_food_source_unknown`
- `cargo test -p tracewake-content --test schema_conformance`
- `cargo test -p tracewake-content --test golden_fixtures_run`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
