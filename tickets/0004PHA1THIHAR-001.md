# 0004PHA1THIHAR-001: Seal seed-mutation API behind a seed-build capability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`state.rs` authoritative-state mutation surface; optional new sealed seed builder/capability); migration of 5 `tracewake-core` integration test files
**Deps**: None

## Problem

`PhysicalState` and `AgentState` expose public `pub fn seed_*_mut(&mut self) -> &mut BTreeMap<…>` accessors (`crates/tracewake-core/src/state.rs:191`–`:289`). Because the types are public, any current or future workspace crate — or a test helper — can obtain a live `&mut` handle to authoritative world/agent state *after* seed construction and mutate it without event append/apply, replay accounting, proposal validation, or checksum provenance. That is a direct-mutation bypass of the one-way causal spine (spec §6 F-001, §8 THIRD-AC-001). The fields themselves are already `pub(crate)` (`state.rs:123`–`:134`); only the public mutable accessors keep the bypass open.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-7 selected for an invariant-motivated, replay-surface, visibility-reshape, removal-blast-radius ticket. -->

1. The 13 mutators are `pub fn` at `crates/tracewake-core/src/state.rs:191`–`:289` (`seed_actors_mut`, `seed_places_mut`, `seed_doors_mut`, `seed_containers_mut`, `seed_items_mut`, `seed_food_supplies_mut`, `seed_workplaces_mut`, `seed_needs_by_actor_mut`, `seed_intentions_mut`, `seed_active_intention_by_actor_mut`, `seed_routine_executions_mut`, `seed_decision_traces_mut`, `seed_stuck_diagnostics_mut`); backing maps are `pub(crate)` at `:123`–`:134`. The canonical seed-construction entry `from_seed_parts` already exists at `state.rs:143` (`PhysicalState`) and `:221` (`AgentState`).
2. The remediation is spec §8 `THIRD-AC-001` (reassessed this session); §6 F-001's assessment confirms the only live call sites are `tracewake-core`'s own integration tests, not content or TUI.
3. Shared boundary under audit: the authoritative-state *mutation* boundary between the core kernel and its consumers. `tracewake-content` already builds seed state via `PhysicalState::from_seed_parts` / `AgentState::from_seed_parts` (`crates/tracewake-content/src/schema.rs:398`, `:494`), never the mutators — so the content seam needs no change.
4. Motivating invariants (restated): `INV-009` — meaningful state changes require events; `INV-011` — current-state-only simulation is forbidden; `INV-101` — actor-known context is sealed (proposal generation must consume sealed context, not mutate state directly); `INV-104` — routines and needs do not dispatch primitive actions directly. A public post-seed `&mut` map handle is exactly the "direct mutation" these forbid.
5. Deterministic-replay surface: post-seed authoritative mutation outside the event path desynchronizes replay (`INV-018`). The legitimate post-seed mutation enforcement surface is `crates/tracewake-core/src/events/apply.rs` via `WorldMutationCapability` / `AgentMutationCapability` (`events/mutation.rs`). This ticket *removes* a bypass; it introduces no new leakage or nondeterminism path — seed-time construction from fully-formed ordered `BTreeMap`s is replay-neutral.
6. Visibility reshape (additive-vs-breaking): turning the `pub` mutators into a sealed/private surface is a **breaking** visibility change. Consumers = 5 `tracewake-core` integration test files (`golden_scenarios.rs`, `acceptance_gates.rs`, `no_human_capstone.rs`, `hidden_truth_gates.rs`, `anti_regression_guards.rs`), all migrated in this same diff (local compile-atomicity — the tree will not compile with a partial migration).
7. Removal blast radius (`grep -rn 'seed_[a-z_]*_mut' crates/`): matches are confined to `state.rs` (definitions) and the 5 core test files above. Zero matches in `tracewake-content`, `tracewake-tui`, `docs/`, `specs/`, or `.claude/skills/`. No doc-governed contract names these methods.

## Architecture Check

1. Sealing post-seed mutation — either by removing the `seed_*_mut` accessors and routing all construction through the existing `from_seed_parts`, or by adding a non-public `SeedBuildCapability` + `PhysicalSeedBuilder` / `AgentSeedBuilder` minted only inside `tracewake-core` — makes post-seed external mutation *type-impossible*, mirroring the existing `WorldMutationCapability` / `AgentMutationCapability` pattern (`events/mutation.rs`). This is an **atomic cutover**: per the spec's anti-contamination thesis the bypass is removed, not wrapped — a wrapper or deprecated alias leaving the old `&mut` path reachable would defeat the structural lock, so the whole reseal + consumer migration lands as one reviewable diff (Split is unsafe here, not merely inconvenient).
2. No backwards-compatibility aliasing or shims: the `seed_*_mut` accessors are deleted outright, not deprecated or re-exported under a new name.

## Verification Layers

1. `INV-009` / `INV-104` (no direct post-seed mutation) -> codebase grep-proof: `grep -rnE 'pub fn seed_[a-z_]+_mut' crates/tracewake-core/src/state.rs` returns nothing.
2. `INV-018` (deterministic replay preserved) -> replay/golden-fixture check: `golden_scenarios.rs` and `no_human_capstone.rs` replay-equality assertions still pass after migration to `from_seed_parts`.
3. Visibility seal (capability boundary) -> codebase grep-proof: no external crate references a seed mutator; `grep -rn 'seed_[a-z_]*_mut' crates/tracewake-content crates/tracewake-tui` returns nothing.

## What to Change

### 1. Seal authoritative-state mutation

Remove the public `seed_*_mut` accessors from `PhysicalState` and `AgentState`. Route seed-time construction through the existing `from_seed_parts(...)` (which takes fully-constructed ordered maps), or introduce a non-public `SeedBuildCapability` + `PhysicalSeedBuilder` / `AgentSeedBuilder` minted only by core fixture/load conversion functions. Backing maps stay `pub(crate)`; post-seed mutation remains reachable only through `WorldMutationCapability` / `AgentMutationCapability` in `events/apply.rs`.

### 2. Migrate the 5 core test files

Rewrite each `seed_*_mut().insert(...)` / `.get_mut(...)` call site to assemble ordered maps and pass them through `from_seed_parts` (or the new seed builder). `anti_regression_guards.rs` is a shared hub (also touched by tickets 003, 004, 006) — coordinate the mechanical merge; confirm whether its `seed_*_mut` reference is a live call site or a banned-token string before editing.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify) — remove `seed_*_mut`; route construction through `from_seed_parts`/sealed builder (may add a private builder type in this file)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — shared hub w/ 003, 004, 006)

## Out of Scope

- Compile-time / clippy negative fixtures proving the bypass is gone (ticket 003 — depends on this seal and the runner harness in 002).
- The event-apply mutation capability itself (`events/apply.rs`, `events/mutation.rs`) — already correct; not modified.
- Checksum field-to-registry parity (ticket 006) — independent `state.rs` concern.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build --workspace --all-targets --locked` — the tree compiles after the seal and the in-diff test migration (no partial/uncompilable intermediate).
2. `cargo test --workspace` — migrated `golden_scenarios`, `acceptance_gates`, `no_human_capstone`, `hidden_truth_gates` cases pass, including replay-equality assertions.
3. `! grep -rqE 'pub fn seed_[a-z_]+_mut' crates/tracewake-core/src/state.rs` — no public seed mutators remain.

### Invariants

1. No authoritative-state mutation path exists outside event application once seeding is complete.
2. Initial state is reachable only through `from_seed_parts` / the sealed seed builder, fed fully-constructed ordered maps.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/golden_scenarios.rs` — migrate seed construction to `from_seed_parts`/builder; rationale: highest-density mutator consumer, exercises replay equality.
2. `crates/tracewake-core/tests/{acceptance_gates,no_human_capstone,hidden_truth_gates,anti_regression_guards}.rs` — migrate remaining seed-construction call sites so the workspace compiles and the no-human/acceptance suites stay green.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
