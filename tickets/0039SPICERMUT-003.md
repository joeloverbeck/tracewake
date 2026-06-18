# 0039SPICERMUT-003: Kill `content/schema.rs` SPINE survivors with boundary-cased schema-conversion witnesses

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-content` (test-only by default; a production correction in `schema.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 3 missed mutants in `crates/tracewake-content/src/schema.rs` (spec §5.2), owning SPINE-05 (schema-to-authoritative-state conversion). The cluster mutates the comparison in `FixtureSchema::to_agent_state` that selects the earliest `start_tick` when multiple routine assignments compete to become an actor's active intention: the relational operator survivors `< -> ==`, `< -> >`, and `< -> <=`. Current schema tests do not exercise below/exact/above-bound competition through the public conversion path, so each relational mutation survives.

## Assumption Reassessment (2026-06-18)

1. `FixtureSchema::to_agent_state(&self, _validation: FixtureValidationToken) -> AgentState` exists at `crates/tracewake-content/src/schema.rs:713`; the competing routine assignments carry `start_tick: SimTick` (`:420`/`:427`) and are compared at `:508`/`:515` to select the earliest active intention (verified by grep). The 3 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.2 is the implementation contract; `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` governs schema-to-state conversion and fixture validation (verified present).
3. Shared boundary under audit: the `FixtureSchema::to_agent_state` conversion seam where competing routine assignments resolve to one active intention via the earliest-`start_tick` comparison.
4. Motivating invariant: `INV-022 — Raw prose is not authoritative state` (conversion of authored fixture data into typed authoritative state must be exact and validated), with `INV-061` (authored causal machinery creates possibility space, not guaranteed arcs). The witness must observe the typed selected intention and its serialization/checksum consequence, not the comparison literal.
5. This ticket touches a fail-closed content/schema-validation surface and a deterministic serialization/checksum consequence: valid boundary cases must produce the intended typed state and a stable serialization/checksum, and invalid cases must fail with a typed schema diagnostic producing no partial authoritative state or seed events. The change is test additions only — there is **no schema shape change** (no field added or reshaped; no additive-vs-breaking analysis applies). This substrate feeds the SPINE-05 re-proof in ticket 021.

## Architecture Check

1. Below-bound, exact-bound, and above-bound competing-assignment cases through the public conversion path catch each of the three relational mutants distinctly; pairing each invalid case with a nearest valid sibling prevents a degenerate "reject everything" validator from satisfying the family.
2. No backwards-compatibility aliasing/shims: cases drive the public `to_agent_state` conversion, not a private comparison helper.

## Verification Layers

1. SPINE-05 conversion exactness (INV-022) -> schema validation: below/exact/above-bound routine-assignment competition through `to_agent_state` selects the doctrine-correct earliest intention; `< -> ==`, `< -> >`, `< -> <=` each fail a distinct case or an explicit relational assertion.
2. Deterministic consequence -> replay/golden-fixture check: the selected intention yields a stable serialization/checksum consequence; an invalid case yields a typed diagnostic and no partial authoritative state.

## What to Change

### 1. Boundary-cased conversion matrix

In `schema_conformance.rs`, identify the exact compared field and bound at the final checkout, then add below-bound, exact-bound, and above-bound competing-assignment cases through the public schema conversion path, asserting the typed selected intention.

### 2. Valid/invalid sibling pairing + relational coverage

Prove valid boundary cases produce the intended typed state and a stable serialization/checksum; prove invalid cases fail with a typed schema diagnostic and produce no partial authoritative state or seed events; pair each invalid case with a nearest valid sibling. Ensure each of `< -> ==`, `< -> >`, `< -> <=` is caught by a distinct row or an explicitly demonstrated relational assertion.

## Files to Touch

- `crates/tracewake-content/tests/schema_conformance.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Fixture-load scope arms and seed causality in `load.rs` (ticket 002).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test schema_conformance` — passes with below/exact/above-bound cases and valid/invalid sibling pairs.
2. `cargo mutants --workspace -f crates/tracewake-content/src/schema.rs --no-shuffle` — each of the 3 historical relational survivors (and any newly enumerated one) is `caught`.

### Invariants

1. Each of the three relational mutants is killed by a distinct boundary case or relational assertion; no single "all invalid fail" test satisfies the family.
2. Invalid conversions fail with a typed diagnostic and produce no partial authoritative state or seed events.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/schema_conformance.rs` — boundary-cased `to_agent_state` competition matrix with valid/invalid sibling pairs and a stable serialization/checksum consequence.

### Commands

1. `cargo test --locked -p tracewake-content --test schema_conformance`
2. `cargo mutants --workspace -f crates/tracewake-content/src/schema.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
