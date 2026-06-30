# 0058EMBROUCON-005: TUI parity rows and de-authority lock

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds `spec0058.*` parity rows + scenario measurement; possible anti-leak fixture
**Deps**: 0058EMBROUCON-001, 0058EMBROUCON-002

## Problem

Spec §4.5 F-0058-05 / §6.2 — extend the parity row class with active-intention and temporal-authority adversaries, proving *consequence* (typed causal + actor-known + rendered + replay + anti-leak), not just affordance presence, and keep `app.rs` at forwarding altitude.

## Assumption Reassessment (2026-06-30)

1. Parity row `spec0057.routine.embodied_continue_workday` exists at `crates/tracewake-tui/tests/parity/census_actions.rs:69` with `OwnershipScope::FuturePack` (70) and the `CapabilityClass` / `EvidenceFlag` imports (4). `app.rs::submit_entry` forwards the selected semantic action to the runtime command (spec H-0058-08, aligned). The anti-leak fixture `embodied_continue_hidden_workplace_001.rs` is registered in `crates/tracewake-content/src/fixtures/mod.rs`.
2. Spec §6.2 requires two `spec0058.*` rows — `spec0058.routine.embodied_continue_active_intention_current_step` and `spec0058.routine.embodied_continue_temporal_authority` — the latter encoding the **typed-stuck branch** (matching -002's approved decision, since scheduler-owned wait routing is deferred). Each row needs `CapabilityClass::ActorObservableConsequence`, `SurfaceDisposition::Embodied`, typed causal witness, actor-knowledge witness, rendered witness, replay evidence, and an anti-leak fixture.
3. Cross-artifact boundary under audit: the TUI parity census (`census_actions.rs` / `scenario.rs` / `runner.rs`) ↔ the runtime command surface; `app.rs` must stay a forwarder (no routine-family selection, target repair, or hidden-truth checks).
4. Invariants under audit: INV-065/066/069 (TUI surfaces actor-known state, no hidden leakage), INV-095 (TUI/view-model tests are acceptance tests), INV-108 (possession is input-only).
5. Enforcement surface: the parity rows + scenario measurement assert the actor-known witness (target/family came from the active intention + actor-known context, not hidden truth or assigned inactive windows) and an anti-leak fixture when hidden truth/workplace/adversarial inactive routine is present. Confirm no rendered/replay path leaks hidden truth and `app.rs` gains no simulation authority (INV-069/095/108); replay evidence is required per row.

## Architecture Check

1. Measuring typed causal consequence + actor-known + rendered + replay + anti-leak per row proves the capability is actually reachable and de-authoritied, which an affordance-presence row cannot. Encoding the temporal row as the typed-stuck branch keeps the parity surface honest about -002's deferred wait routing.
2. No backwards-compatibility aliasing/shims: `app.rs` stays a forwarder; no simulation rules, target repair, or routine-family selection are added to TUI code.

## Verification Layers

1. INV-095 (TUI acceptance tests) → the two `spec0058` parity rows run under `cargo test --workspace`.
2. INV-065/066/069 (no leak) → anti-leak fixture evidence per row (hidden workplace / adversarial inactive routine).
3. INV-108 (possession input-only) → `app.rs` forwarding altitude preserved (the source-scan guard lands in -006; here, the rendered + actor-known witness proves consequence without authority).

## What to Change

### 1. Two `spec0058.*` parity rows

Add them under `OwnershipScope::FuturePack { namespace: "spec0058_embodied_routine_continuation_foundational_alignment" }` in `census_actions.rs`, each with `CapabilityClass::ActorObservableConsequence`, `SurfaceDisposition::Embodied`, and typed causal + actor-known + rendered + replay + anti-leak evidence.

### 2. Scenario measurement

Add scenario measurement (`scenario.rs` / `embodied_flow.rs`) for the active-intention and temporal-authority adversaries; the temporal row encodes the typed-stuck branch.

### 3. Anti-leak fixture; keep `app.rs` forwarding-only

Reuse `embodied_continue_hidden_workplace_001` or add a `spec0058` anti-leak fixture (registered in `fixtures/mod.rs` if new). Touch `app.rs` only if needed to keep forwarding clean — add no simulation logic.

## Files to Touch

- `crates/tracewake-tui/tests/parity/census_actions.rs` (modify)
- `crates/tracewake-tui/tests/parity/scenario.rs` (modify)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify — only if needed to keep forwarding clean)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — only if a new `spec0058` anti-leak fixture is added)
- `crates/tracewake-content/src/fixtures/embodied_continue_active_intention_spec0058_001.rs` (new — only if a new anti-leak fixture is added, as surfaced)

## Out of Scope

- Adding simulation rules / routine-family selection / hidden-truth checks to `app.rs`.
- The behavioral remediations (-001/-002/-003) and the metamorphic core test (-004).

## Acceptance Criteria

### Tests That Must Pass

1. `spec0058.routine.embodied_continue_active_intention_current_step` — the row asserts target/family from the active intention + actor-known context, not hidden truth or assigned inactive windows; typed causal + rendered + replay + anti-leak evidence present.
2. `spec0058.routine.embodied_continue_temporal_authority` — the row encodes the typed-stuck branch with typed causal + actor-known + rendered + replay evidence.
3. `cargo test --workspace` passes with the parity rows included.

### Invariants

1. Each row proves actor-observable consequence through the TUI surface, not affordance presence (INV-095).
2. No rendered/replay path leaks hidden truth; `app.rs` gains no simulation authority (INV-069/108).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/parity/census_actions.rs` (modify) — two `spec0058` rows.
2. `crates/tracewake-tui/tests/parity/scenario.rs` + `crates/tracewake-tui/tests/embodied_flow.rs` (modify) — scenario measurement.
3. `crates/tracewake-content/src/fixtures/` (new fixture + `mod.rs` registration, as surfaced).

### Commands

1. `cargo test -p tracewake-tui spec0058`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-30

- Added two `spec0058.*` parity rows under `spec0058_embodied_routine_continuation_foundational_alignment`:
  - `spec0058.routine.embodied_continue_active_intention_current_step`
  - `spec0058.routine.embodied_continue_temporal_authority`
- Both rows are `ActorObservableConsequence` / `Embodied`, require replay and no-human evidence, and declare `embodied_continue_hidden_workplace_001` as anti-leak coverage.
- Added scenario operations for active-intention continuation and temporal-authority typed-stuck measurement. The active-intention row measures `ContinueRoutineProposed` + `ActorMoved` to `workshop_tomas` with no `WorkBlockStarted`; the temporal row measures rejected `continue_routine` with `RoutineStepBlocked`, `StuckDiagnosticRecorded`, and no `ActorWaited`.
- Added runner load-bearing checks for both spec0058 rows and a targeted `spec0058_routine_parity_rows_measure_real_scenarios` test so `cargo test -p tracewake-tui spec0058` exercises the real scenario evidence.
- `crates/tracewake-tui/src/app.rs` was not touched; no TUI-side simulation authority, target repair, routine-family selection, or hidden-truth checks were added.

Verification:

- `cargo test -p tracewake-tui spec0058` passed.
- `cargo fmt --all --check` passed.
- `cargo test --workspace` passed.
