# 0020PHA3ACOGSUR-005: Transitive food-omniscience helper containment

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `tracewake-content` (`src/fixtures/mod.rs`, the five wrapper-consumer fixture files, `tests/fixtures_load.rs` census); no core changes
**Deps**: `specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-062)

## Problem

The `ORD-HARD-051` allowlist census is bypassable through indirection:
`fixtures/mod.rs::hidden_truth_adversarial_fixture` internally calls
`populate_known_food_sources_for_all_actors`, and the census
(`known_food_source_helper_call_sites_from_source`) scans each fixture file for the
literal direct call only — so all five delegating fixtures attribute to the single
`mod.rs` allowlist entry, and a NEW fixture delegating to the wrapper inherits
cross-product food omniscience with no census trip and no rationale obligation
(`ORD-HARD-062`).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`: the wrapper's internal blanket call exists
   (`fixtures/mod.rs`, inside `hidden_truth_adversarial_fixture`); five consumers
   delegate (`hidden_food_closed_container_001.rs`,
   `hidden_food_unknown_route_001.rs`, `hidden_route_edge_001.rs`,
   `debug_omniscience_excluded_001.rs`, `workplace_assignment_provenance_001.rs`);
   the census is a per-file textual scan for
   `.populate_known_food_sources_for_all_actors(`; the synthetic-injection guard
   (`known_food_source_blanket_helper_call_sites_are_allowlisted`) proves only the
   direct-call direction; `LEGACY_KNOWN_FOOD_SOURCE_HELPER_CALL_SITES` is the
   allowlist.
2. Verified against spec 0020 (reassessed 2026-06-11): ORD-HARD-062 offers two
   corrections — transitively census the wrapper's callers into the allowlist, OR
   remove the internal blanket call and have each consumer author explicit
   per-actor edges. This ticket records the choice (Architecture Check 1); the
   synthetic indirect-consumer injection case lands either way.
3. Cross-artifact boundary under audit: the authored-knowledge containment contract
   spanning `schema.rs::populate_known_food_sources_for_all_actors`, the fixture
   corpus, and the `fixtures_load.rs` census — no fixture acquires blanket food
   omniscience without an allowlisted, rationale-bearing site.
4. INV-025/INV-062 posture restated (spec §2 lineage): authored ignorance must stay
   expressible and the authoring grammar's path of least resistance must not re-mint
   uniform omniscience; the census exists so reaching for the blanket helper is a
   deliberate, reviewed act.
5. Recorded implementation choice (decomposition-carried; either-or the spec leaves
   open): **primary — remove the internal blanket call** and author explicit
   per-actor `known_food_sources` edges in the five consumers. These are
   single-actor adversarial fixtures, so the explicit edge set replicates the
   cross-product exactly: identical seeded beliefs, no behavioral or checksum churn,
   and the laundering channel is deleted rather than fenced. Fallback (record if
   taken): transitively census wrapper callers into the allowlist with per-entry
   rationale — use only if implementation surfaces a consumer whose explicit edge
   set would NOT replicate current seeding (a multi-actor wrapper consumer), and
   record why.
6. Mismatch watch: if any of the five consumers seeds multiple actors (refuting the
   single-actor premise), the fallback fires for that consumer — verify per fixture
   before editing.

## Architecture Check

1. Deleting the wrapper's blanket call is preferred over fencing it: the census then
   has exactly one class of site to police (direct calls in fixture files), the
   indirection channel ceases to exist, and explicit edges make each fixture's
   knowledge posture locally legible — stronger than an allowlist entry pointing at
   shared plumbing. The synthetic indirect-consumer case still lands so a future
   wrapper reintroduction fails the suite.
2. No backwards-compatibility aliasing/shims: the wrapper keeps its adversarial
   world-building role; only the knowledge-seeding side effect moves to explicit
   per-fixture edges.

## Verification Layers

1. Census closure -> `known_food_source_blanket_helper_call_sites_are_allowlisted`
   extended with the synthetic *indirect*-consumer injection case (a fixture
   delegating to a wrapper that calls the helper must fail).
2. Seeding equivalence -> existing fixture tests and golden checksums green
   unchanged (explicit edges replicate the cross-product for single-actor
   fixtures); `partial_food_source_knowledge_seeds_only_authored_actor_edge`
   untouched.
3. Containment posture -> grep-proof: `populate_known_food_sources_for_all_actors`
   has no production call site inside `fixtures/mod.rs` wrappers after the change
   (or, fallback, every transitive caller is allowlisted with rationale).

## What to Change

### 1. Remove the wrapper's blanket call (primary)

Delete the `populate_known_food_sources_for_all_actors` call from
`hidden_truth_adversarial_fixture`; author the equivalent explicit
`known_food_sources` edges in each of the five consumer fixtures (verifying the
single-actor premise per fixture; fallback per Assumption item 5 if refuted).

### 2. Synthetic indirect-consumer case

Extend the census guard with an injection case proving a new indirect consumer
(via any wrapper that calls the helper) fails — the lock against reintroduction.

## Files to Touch

- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)

## Out of Scope

- The helper's existence and the direct-call allowlist (0019 deliverables, holding).
- Any change to seeded knowledge content — equivalence is an acceptance criterion,
  not a side effect to absorb.
- Conformance-index rows (none owed; the 0019 seed-knowledge containment row stays
  accurate and is strengthened, not contradicted).

## Acceptance Criteria

### Tests That Must Pass

1. Synthetic indirect-consumer injection case fails as designed (proven by the
   guard's synthetic-regression machinery).
2. All five edited fixtures load with byte-identical seeded beliefs and unchanged
   golden checksums (seeding equivalence).
3. Grep-proof: no production blanket-helper call remains inside wrapper functions
   (or the fallback's transitive allowlist is in place with rationales — recorded).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No fixture can acquire blanket food omniscience through indirection without
   failing the census (lock durability of ORD-HARD-051).
2. Authored knowledge posture is locally legible per fixture (INV-025/062 posture).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — indirect-consumer injection
   case; census extension or wrapper-free assertion per the recorded choice.

### Commands

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test --workspace` (full pipeline; checksum equivalence proven)
