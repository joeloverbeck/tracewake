# 0005PHA1DOCCOD-001: Replace `phase1_implemented: bool` with a typed `ActionScope` in the core action registry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` action registry (`ActionDefinition`, `ActionRegistry`), pipeline phase-boundary guard, and the core conformance/anti-regression suites.
**Deps**: None

## Problem

The Phase 1 content/action boundary is not phase-scoped. `ActionDefinition` represents the phase boundary as a public boolean `phase1_implemented` (`crates/tracewake-core/src/actions/registry.rs:24`), and both constructors set it `true` unconditionally (`registry.rs:32`, `registry.rs:40`). The later-phase registration functions (`register_phase2a_epistemics`, `register_phase3a_sleep`/`eat`/`work`/`continue_routine`, `registry.rs:111-147`) build their definitions through the same constructors, so every registered action — Phase 2A and Phase 3A included — claims to be Phase 1-implemented. The pipeline phase-boundary guard (`crates/tracewake-core/src/actions/pipeline.rs:496`, `if !definition.phase1_implemented`) is therefore unreachable for any registered action: the boolean is never `false`. This is the core of spec `ALIGN-001` — a boolean is too easy to set by convenience constructor and too hard for tests to prove complete coverage. This ticket replaces the boolean with a typed scope so the phase a given action belongs to is parsed data, not a convenience default.

## Assumption Reassessment (2026-06-09)

1. `ActionDefinition` is `{ action_id: ActionId, phase1_implemented: bool, effect: ActionEffect }` at `crates/tracewake-core/src/actions/registry.rs:22-26`; constructors `query_only` (`:29`) and `world_action` (`:38`) both hard-set `phase1_implemented: true`. Registration fns confirmed: `register_phase1_movement_open_close`/`take_place`/`inspect_wait` (`:71`,`:86`,`:97`) and `register_phase2a_epistemics`/`register_phase3a_*` (`:111`-`:147`). Phase 1 action IDs are exactly `move, open, close, take, place, look, inspect_place, inspect_entity, wait`.
2. Spec `specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md` §`ALIGN-REQ-001` requires replacing the boolean with a structural scope model (`ActionScope::{Phase1, Phase2AHistorical, Phase3AHistorical}` or a phase-scoped registry type-state) with no public constructor defaulting to `Phase1`; enforcement is "compile-time first".
3. Shared boundary under audit: the `tracewake-core` action-registry contract consumed by `tracewake-content`'s loader (which calls the `register_phase*` fns) and by the pipeline guard. This ticket changes only the in-crate (core) representation and the guard; the content-side loader routing is `0005PHA1DOCCOD-002`.
4. Invariant motivating this ticket: `INV-098` (feature acceptance is harsh — a feature is not "done" merely because code exists) and `INV-105` (decision/diagnostic data must be typed or structurally inspectable, not a display-string/flag substrate). A boolean phase flag that is structurally always-true is exactly the convenience shortcut `INV-098`'s proof discipline forbids.
5. Deterministic-replay surface touched: the action registry feeds the pipeline, whose `PhaseBoundaryValidation` stage (`pipeline.rs:46`, guard at `:496`) participates in event acceptance. Confirm the retype does not change which actions are *accepted* for any currently-exercised fixture: the guard must continue to admit every action a currently-passing pipeline run admits (the active-scope set in this ticket is the full set of registered scopes; tightening to a Phase-1-only active scope and proving rejection is `0005PHA1DOCCOD-004`). No event payload, ordering, or checksum input changes — `ActionScope` is registry-internal metadata, not serialized into the event log.
6. Schema extension/retype: `ActionDefinition.phase1_implemented: bool` → `scope: ActionScope`. This is a breaking field retype, not additive. In-workspace consumers grepped repo-wide: `phase1_implemented` appears only at `registry.rs:24/32/40` and `pipeline.rs:496` — both in `tracewake-core`. `tracewake-content` consumes the `register_phase*` functions and `ActionRegistry::get`, not the field directly, so the retype is core-local and every consumer changes in this one diff (local compile-atomicity).
7. Removal blast radius of the public `phase1_implemented` field: grep `phase1_implemented` across `crates/`, `docs/`, `specs/`, `.claude/skills/` — the only code matches are the four core sites above; spec prose references it descriptively (no code dependency). No skill or doc contract depends on the field name.

## Architecture Check

1. A typed `ActionScope` enum (or phase-scoped registry type-state) makes the illegal state — "a later-phase action mislabeled Phase 1" — unrepresentable rather than detectable-after-the-fact, which is stronger than a boolean a constructor can default. The enum keeps the registry genre-agnostic (it names phases, not domain mechanics) and leaves the kernel the sole authority over action legality.
2. No backwards-compatibility aliasing: the boolean is removed outright, not kept behind a `fn phase1_implemented() -> bool` shim. Constructors gain an explicit scope parameter (or scope-specific constructors) so no call site can omit it.

## Verification Layers

1. `INV-098` (no convenience-true phase flag) -> codebase grep-proof: `grep -rn "phase1_implemented" crates/` returns zero matches after the change.
2. `INV-105` (typed diagnostic substrate) -> codebase grep-proof: `ActionScope` enum exists in `registry.rs` and `ActionDefinition.scope` is its type; no public constructor yields `Phase1` by default (each `register_phaseN_*` sets its own scope).
3. Deterministic replay (`INV-018`) -> replay/golden-fixture check: `cargo test --workspace` — all existing golden/replay tests still pass, proving the retype changed no accepted-action set or checksum.
4. Pipeline boundary representation -> manual review + grep: `pipeline.rs:496` guard now reads `definition.scope` against an active-scope set rather than the boolean.

## What to Change

### 1. Introduce `ActionScope` and retype `ActionDefinition`

In `crates/tracewake-core/src/actions/registry.rs`: add `pub enum ActionScope { Phase1, Phase2AHistorical, Phase3AHistorical }` (derive `Clone, Copy, Debug, PartialEq, Eq`). Replace `pub phase1_implemented: bool` with `pub scope: ActionScope`. Replace the two blanket constructors so scope is explicit — either add a `scope: ActionScope` parameter to `query_only`/`world_action`, or provide scope-specific constructors — such that no constructor defaults to `Phase1`.

### 2. Set the correct scope at each registration site

`register_phase1_*` fns set `ActionScope::Phase1`; `register_phase2a_epistemics` sets `Phase2AHistorical`; `register_phase3a_*` set `Phase3AHistorical`.

### 3. Update the pipeline phase-boundary guard

In `crates/tracewake-core/src/actions/pipeline.rs:496`, change `if !definition.phase1_implemented` to test `definition.scope` against an active-scope set. In this ticket the active-scope set admits all currently-registered scopes (behavior-preserving — no fixture's accepted-action set changes); tightening it to reject out-of-scope actions and proving the rejection path is `0005PHA1DOCCOD-004`.

### 4. Update the core conformance suites

In `crates/tracewake-core/tests/spine_conformance.rs` and `crates/tracewake-core/tests/anti_regression_guards.rs`, update any assertion referencing `phase1_implemented` to assert the typed `scope`; add a positive assertion that the Phase 1 registration set carries `ActionScope::Phase1` and the later-phase sets carry their historical scopes.

## Files to Touch

- `crates/tracewake-core/src/actions/registry.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Content-side loader routing by scope and the `FixtureScope` field (`0005PHA1DOCCOD-002`).
- Tightening the pipeline active-scope set to reject out-of-phase actions + the reachability/compile-fail test (`0005PHA1DOCCOD-004`).
- Source-level guard that the Phase 1 loader cannot call later-phase registration fns (`0005PHA1DOCCOD-005`).
- Any later-phase mechanics audit (Phase 2A/3A code is boundary evidence only, per spec §10).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` — registry scope assertions pass.
2. `cargo test --locked -p tracewake-core --test anti_regression_guards` — no assertion references the removed boolean.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` — full tree compiles and passes (proves the core-local retype updated every consumer).

### Invariants

1. No registered action's scope is derivable from a constructor default; each `register_phaseN_*` site sets its scope explicitly (`INV-098`).
2. The retype is registry-internal: no event payload, ordering key, or checksum input changes; identical inputs replay byte-identically (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` — assert `ActionScope::Phase1` for the Phase 1 registration set and the historical scopes for later-phase sets.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — replace boolean-based assertions with scope-based ones.

### Commands

1. `grep -rn "phase1_implemented" crates/` — must return zero matches.
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
