# 0008PHA3AANTCON-008: Hidden-truth adversarial fixtures + planner gates

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content`: new adversarial fixtures (hidden food/route/workplace/container/debug-omniscience); `tracewake-core`: gates asserting planners cannot use hidden truth
**Deps**: 0008PHA3AANTCON-006, 0008PHA3AANTCON-001

## Problem

Spec 0008 §10.2 (D-0008-10): the suite must include fixtures/tests that fail if planners can use hidden truth — hidden food in a closed/private container at the actor's place; hidden food reachable via an unknown route; a workplace present in `PhysicalState` but known only as an assignment fact with explicit provenance; a physical route edge not visible/known under the actor's context; debug-only omniscient facts that must not appear in `ActorKnownPlanningContext`. Assertions must inspect proof provenance (-001), not display strings.

## Assumption Reassessment (2026-06-08)

1. Fixture infrastructure: `crates/tracewake-content/src/fixtures/` with `mod.rs` registry (existing siblings e.g. `no_hidden_truth_planning_001.rs`, `no_human_day_001.rs`); core gates can live in a new `crates/tracewake-core/tests/` file alongside `acceptance_gates.rs`/`golden_scenarios.rs`. The sealed `ActorKnownPlanningContext` + provenance graph (-001) is the assertion surface; the transaction-driven no-human path (-006) is the behavior under test.
2. Spec §10.2 enumerates the five adversarial scenarios and mandates provenance-based assertions ("inspect proof provenance, not display strings"). §13 acceptance: "Hidden-truth adversarial fixtures fail if physical truth leaks into planning."
3. Cross-artifact boundary under audit: the **fixture (content) ↔ sealed context (core) ↔ planner-decision (core)** seam — a fixture seeds hidden physical truth with no actor observation/belief/assignment provenance; the gate asserts the planner never selects it.
4. INV-024 (No telepathy) and INV-093 (Actor-knowledge leakage is a high-severity defect): hidden food/routes/workplaces/containers must not affect no-human decisions absent a modeled channel.
5. Enforcement surface: actor-knowledge firewall. Confirm each gate asserts on provenance (a selected fact carries `ObservedNow`/`RememberedBelief`/`RoutineAssignment`, never `FixturePossibility`-only or physical-derived), not on the absence of a literal string. `DebugOmniscience` facts must be absent from any planner context. No determinism impact — fixtures are static seeds.
6. Adds new content fixtures registered in `fixtures/mod.rs` (additive); fixture schema unchanged (these use existing fields, just adversarially arranged — hidden truth without provenance). Consumers: the new core gate tests; `forbidden_content`/`fixtures_load` continue to pass (these fixtures are valid possibility-space, not scripts).

## Architecture Check

1. Provenance-based assertions are the only way to prove a no-leak property: a string-absence check ("log lacks `food_hidden`") proves nothing about why an action was chosen. Asserting the selected fact's provenance proves the actor knew it through a modeled channel.
2. No backwards-compatibility aliasing: fixtures are new possibility-space content; no production shim.

## Verification Layers

1. INV-024 no-telepathy → replay/golden-fixture check: over each adversarial fixture, the transaction's selected proposal references no fact whose provenance is physical-only/`FixturePossibility`-only.
2. INV-093 leakage-is-defect → unit test: hidden food in a closed/private container at the actor's place is never selected for `eat` absent observation/belief provenance.
3. Distinct-scenario mapping → manual review: each of the five §10.2 scenarios has its own fixture + gate; debug-omniscience facts are asserted absent from `ActorKnownPlanningContext`.

## What to Change

### 1. Adversarial fixtures

Add fixtures (names as surfaced during implementation) covering the five §10.2 scenarios, e.g. `hidden_food_closed_container_001.rs`, `hidden_food_unknown_route_001.rs`, `workplace_assignment_provenance_001.rs`, `hidden_route_edge_001.rs`, `debug_omniscience_excluded_001.rs`; register each in `fixtures/mod.rs`.

### 2. Provenance-inspecting gates

New core test asserting, per fixture, that the no-human transaction's chosen proposal (or its absence) is justified only by provenance-bearing actor-known facts; hidden physical truth and debug-omniscience never enter the planner context.

## Files to Touch

- `crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs` (new — name as surfaced)
- `crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs` (new — name as surfaced)
- `crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs` (new — name as surfaced)
- `crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs` (new — name as surfaced)
- `crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs` (new — name as surfaced)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixtures)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (new — provenance-inspecting gates)

## Out of Scope

- Content-validation anti-script rules (0008PHA3AANTCON-009).
- Static/anti-regression source guards (0008PHA3AANTCON-007).
- The capstone integrated ancestry gate (0008PHA3AANTCON-011).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the five §10.2 scenarios fails (gate red) if the planner uses the hidden fact; passes only when selection is provenance-justified.
2. `debug_omniscience_excluded_001` proves debug-only facts never appear in `ActorKnownPlanningContext`.
3. `cargo test -p tracewake-core --test hidden_truth_gates && cargo test -p tracewake-content` green.

### Invariants

1. No no-human decision references a fact lacking observation/belief/assignment provenance.
2. Assertions inspect provenance, never display strings.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — five provenance-inspecting gates.
2. `crates/tracewake-content/src/fixtures/*_001.rs` — adversarial seeds (loaded by `fixtures_load`/`golden_fixtures_run`).

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:

1. Added five registered adversarial fixtures: `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`, `workplace_assignment_provenance_001`, `hidden_route_edge_001`, and `debug_omniscience_excluded_001`.
2. Added `crates/tracewake-core/tests/hidden_truth_gates.rs` with five provenance-oriented gates covering closed-container food, unknown-route food, workplace assignment provenance, hidden route edges, and debug-omniscience exclusion.
3. Updated the content fixture registry and fixture-count/id expectations for the five new fixtures.
4. The core gates assert actor-known provenance and planner/transaction outcomes without importing content fixtures into `tracewake-core`, preserving core's zero-dependency boundary.

Deviations:

1. The five content fixtures share a common adversarial fixture factory to keep the scenario data consistent; each fixture has distinct contract metadata for its §10.2 scenario.
2. Core gates construct actor-known contexts directly rather than depending on content fixtures, so the provenance checks stay inside the core crate without adding a dev-dependency on `tracewake-content`.

Verification:

1. `cargo test -p tracewake-core --test hidden_truth_gates` passed.
2. `cargo test -p tracewake-content` passed.
3. `cargo fmt --all --check` passed.
4. `cargo clippy --workspace --all-targets -- -D warnings` passed.
5. `cargo build --workspace --all-targets --locked` passed.
6. `cargo test --workspace` passed.
