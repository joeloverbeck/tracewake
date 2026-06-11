# 0019PHA3AGENREA-007: Seed-knowledge helper allowlist census and partial-knowledge fixture

**Status**: PENDING
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (new canonical fixture + registration; allowlist census in tests); conformance-index row
**Deps**: `tickets/0019PHA3AGENREA-001.md` (new fixture's golden expectations computed once, under the post-repricing checksum scheme); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-051)

## Problem

The omniscience-restoring fixture helper is default-on across the authored corpus
(`ORD-HARD-051`). `schema.rs::FixtureSchema::populate_known_food_sources_for_all_actors`
builds the full actor × food-supply cross product as `known_food_sources` edges and is
called by 51 fixture files — every food-bearing fixture except the hidden-food
negative. The helper's existence is sanctioned (0018 §9 recommended explicit edges for
existing fixtures to keep checksums stable), but it is an unguarded one-liner any
future fixture can reach for, and the canonical corpus contains no partial-knowledge
exemplar — authored ignorance is *expressible* (`ORD-HARD-040`) but the grammar's path
of least resistance re-mints uniform omniscience (INV-025/INV-062 in spirit).

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: the helper exists in
   `crates/tracewake-content/src/schema.rs` with 51 fixture-file call sites;
   `load.rs::seed_event_log` mints `household_food_source` beliefs only from authored
   `known_food_sources` edges; `seeded_food_source_unknown_to_all_actors_001` is the
   only fixture exercising ignorance, and it is all-or-nothing (no actor knows), not
   partial. The positive-fixture census (`positive_fixture_constructor_ids_from_source`
   parity with `fixtures::all()`) will automatically force registration of any new
   fixture.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-051 (reassessed
   2026-06-11): the correction is (a) a call-site allowlist census — existing golden
   fixtures enumerated; a new fixture using the helper must join the allowlist with a
   rationale or author explicit per-actor edges — and (b) at least one canonical
   fixture with genuinely partial food knowledge, so partial-belief seeding has a
   living exemplar in the golden corpus.
3. Cross-artifact boundary under audit: the seed-knowledge authoring contract between
   fixture constructors (`fixtures/*.rs`), the schema helper (`schema.rs`), and the
   loader's edge-driven belief minting (`load.rs::seed_event_log`) — per-actor
   knowledge is an authored decision, not a default.
4. INV-025 restated: wrong and partial beliefs are first-class state — required
   mechanics, not edge cases; INV-062: seeds are tensions, not scripts — a seed grammar
   whose easiest path is everyone-knows-everything underexpresses the possibility space
   the doctrine requires. Provenance marking itself (INV-063) already holds and is
   untouched.
5. Content-validation surface touched (fail-closed preserved): the new fixture passes
   the existing fail-closed validation gates (field policies, union scanner, epistemic
   seed checks) unchanged; the allowlist census is a test-layer addition; no validation
   rule is weakened and no schema field changes.

## Architecture Check

1. An allowlist census (call sites of `populate_known_food_sources_for_all_actors`
   pinned to the enumerated legacy golden set, in the source-scan census style) makes
   reaching for blanket omniscience a visible, rationale-bearing decision without
   churning 51 existing fixtures' checksums — exactly the trade 0018 §9 recommended.
   A canonical partial-knowledge fixture makes the intended authoring pattern
   (explicit per-actor edges) the documented exemplar rather than a negative-test
   curiosity.
2. No backwards-compatibility aliasing/shims: the helper is pinned, not wrapped in a
   deprecated alias; new fixtures author edges explicitly.

## Verification Layers

1. Helper containment -> new allowlist census in the content tests: the set of files
   calling `populate_known_food_sources_for_all_actors` equals the committed allowlist;
   a synthetic new call site fails with the file named.
2. Partial-knowledge exemplar -> the new fixture's assertions: an actor with an
   authored edge receives the `household_food_source` starting belief and can plan
   toward the source; an actor without the edge receives no belief and cannot target it
   (the `hidden_food_*` gate pattern, applied per-actor).
3. Registration integrity -> existing `positive_fixture_constructor_ids_from_source`
   parity census (forces `fixtures::all()` registration automatically).
4. INV-018 -> golden suite green with the new fixture's expectations computed once
   under the post--001 checksum scheme.

## What to Change

### 1. Call-site allowlist census

In `crates/tracewake-content/tests/fixtures_load.rs`: scan `src/fixtures/*.rs` (and
`src/schema.rs` for the definition site) for calls to
`populate_known_food_sources_for_all_actors`; assert the call-site set equals a
committed allowlist of the existing golden fixtures, with a comment stating the
join-with-rationale-or-author-edges rule for new fixtures.

### 2. Canonical partial-knowledge fixture

New `crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs`:
two-plus actors, one food source known to a strict subset via explicit
`known_food_sources` edges (no helper call); assertions per Verification Layer 2;
register in `fixtures/mod.rs::all()`; golden expectations computed once.

### 3. Conformance-index row

Add the seed-knowledge helper-allowlist row to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (coordinate with the
-003/-005 row edits — shared file).

## Files to Touch

- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — registration)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — new fixture expectations, as surfaced)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Removing or renaming the helper (sanctioned for the legacy goldens; checksum
  stability per 0018 §9).
- Editing any of the 51 existing fixtures.
- Schema changes (`known_food_sources` landed in 0018; sufficient as-is).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content --test fixtures_load` — allowlist census green;
   registration parity green with the new fixture.
2. `cargo test -p tracewake-content --test golden_fixtures_run` — partial-knowledge
   assertions green (edge-bearing actor believes and plans; edge-less actor neither).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. A new call site of `populate_known_food_sources_for_all_actors` fails CI unless
   deliberately allowlisted with rationale.
2. The golden corpus carries at least one fixture where food-source knowledge is
   genuinely partial across actors.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — call-site allowlist census.
2. `crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs` — the
   exemplar fixture with per-actor belief/planning assertions.

### Commands

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test --workspace`
