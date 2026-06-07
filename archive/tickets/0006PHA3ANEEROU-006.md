# 0006PHA3ANEEROU-006: Intention and routine lifecycle, possession parity, no-teleport

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` intention/routine state (`agent/intention.rs`, `agent/routine.rs`, `state.rs`); routine-assignment instantiation; content schema (`tracewake-content`)
**Deps**: 0006PHA3ANEEROU-001

## Problem

Intention and routine types exist but the live state is not instantiated or driven (audit **D-05 / F-05**, intention/routine aspect). `Intention` has lifecycle fields (`crates/tracewake-core/src/agent/intention.rs`); `RoutineTemplate`/`RoutineExecution`/`RoutineStep` exist (`crates/tracewake-core/src/agent/routine.rs`); content can author routine templates/assignments (`crates/tracewake-content/src/schema.rs`). But the inspected `to_agent_state` path seeds initial needs and does not prove routine assignments become live active routine executions, and the no-human runner passes `active_intention: None`/`routine_window_goal: None`. Spec §5.4 requires durable, eventful intention lifecycle that survives possession; §5.5 requires routines to be embodied defeasible action chains that cannot teleport actors, skip doors, bypass reachability, invent food, or silently mark work/sleep/eat done.

## Assumption Reassessment (2026-06-07)

1. `agent/intention.rs` (`Intention` lifecycle), `agent/routine.rs` (`RoutineExecution` step/status/ancestry/fallback/trace fields), and `state.rs` (`AgentState` stores intentions/routine executions) confirmed. Content routine templates/assignments authored via `tracewake-content/src/schema.rs`; the `to_agent_state` seeding path is confirmed to seed needs but not instantiate live routine executions.
2. Spec §5.4 requires adoption/continuation/switch/interruption/suspension/completion/failure/abandonment/reactivation to be eventful or projection-derived, active intentions to survive possession attach/detach/switch, and possession to change input binding only. §5.5 requires routine assignments/templates loaded into live routine/intention state or instantiated by event at the right window, steps resolving to ordinary action proposals, and typed stuck diagnostics on failure. §7.4/7.5/7.7 fixtures (`routine_blocked_diagnostic_001`, `routine_no_teleport_001`, `possession_does_not_reset_intention_001`) exist.
3. Shared boundary under audit: content routine assignments → live `RoutineExecution`/`Intention` state (instantiated by event) → the decision loop (0006PHA3ANEEROU-007). Possession binding (`crates/tracewake-core/src/controller.rs`, TUI `app.rs`) must not mutate this state.
4. Motivating invariants (restated): INV-034 "Agents need durable intentions", INV-035 "Routines are defeasible intentions" (schedules are not teleports or puppet strings), INV-094 "Possession parity is tested" (controller binding changes input only; intentions/possessions/memory remain with actors).
5. Possession + replay surfaces touched: possession attach/detach/switch is the parity surface — this ticket guarantees it does not reset needs/intentions/routine progress/plan state/actor-known memory (INV-006/INV-094). Routine instantiation and lifecycle transitions are eventful, so they replay identically; no teleport event may set current place except ordinary validated movement/transition. No actor's state leaks to another via possession.
6. Schema change: routine assignments become live routine executions instantiated by event; this defines/constrains the content routine-assignment schema (consumed by content validation 0006PHA3ANEEROU-009) and the `AgentState` routine-execution record. Additive to `AgentState` (live executions populated where previously empty); the content assignment schema gains instantiation semantics validated in 0006PHA3ANEEROU-009.

## Architecture Check

1. Instantiating routine assignments into durable routine/intention state via events (at load or first schedule window) makes routines defeasible commitments that the decision loop reads and updates — the only way "a worker stays a worker unless interrupted" holds. Deriving lifecycle from events (rather than direct field sets) keeps it replay-consistent and debug-inspectable, and makes possession parity provable: if state is event-derived and possession emits no agent events, it cannot reset cognition.
2. No backwards-compatibility aliasing/shims: the `active_intention: None`/empty-routine construction is removed by the decision loop (0006PHA3ANEEROU-007); no parallel "seed-only" routine path remains.

## Verification Layers

1. INV-035 (no teleport) -> replay/golden-fixture check: `routine_no_teleport_001` proves an actor cannot perform remote work/sleep/eat without movement/reachability ancestry; location is unchanged after a failed remote step.
2. INV-094 / INV-006 (possession parity) -> replay/golden-fixture check: `possession_does_not_reset_intention_001` proves attach/detach/switch leaves needs/intention/routine progress unchanged and leaks no prior actor's knowledge.
3. INV-034 (durable intentions) -> manual review + test: continuation/interruption/abandonment/reactivation are eventful; `routine_blocked_diagnostic_001` yields a typed stuck diagnostic, not a silent idle loop.

## What to Change

### 1. Instantiate routine assignments

Load content routine assignments into live `RoutineExecution`/`Intention` state — at fixture load or first schedule window — through events, not direct seeding.

### 2. Eventful intention/routine lifecycle

Make adoption/continuation/switch/interruption/suspension/completion/failure/abandonment/reactivation eventful or projection-derived; routine steps resolve to ordinary action proposals; routine failure produces a typed stuck diagnostic.

### 3. Possession parity

Ensure possession attach/detach/switch (controller binding) emits no agent-cognition mutation; needs/intentions/routine progress/plan state/actor-known memory persist across binding changes.

## Files to Touch

- `crates/tracewake-core/src/agent/intention.rs` (modify)
- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs` (modify)

## Out of Scope

- The decision loop that drives routine execution per window (0006PHA3ANEEROU-007).
- `continue_routine` action semantics (0006PHA3ANEEROU-004).
- Content validation of routine assignments/templates (0006PHA3ANEEROU-009).
- Live event application mechanics (0006PHA3ANEEROU-001, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. `routine_no_teleport_001`: an actor cannot perform a remote step without movement/reachability ancestry; location unchanged after a failed remote step; no event sets current place except validated movement.
2. `possession_does_not_reset_intention_001`: attach/detach/switch leaves needs/intention/routine progress unchanged; no cross-actor knowledge leak.
3. `routine_blocked_diagnostic_001`: a blocked routine step yields a typed stuck diagnostic and step failure/interruption event, not a silent idle loop.
4. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Routine assignments become event-instantiated live executions; routine/intention lifecycle is event-derived and replay-consistent.
2. Possession changes input binding only; no agent-cognition state is reset or leaked across binding changes.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs` — assert no-teleport via integrated routine execution.
2. `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs` — assert parity across attach/detach/switch.
3. `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs` — assert typed stuck diagnostic from a real blocked step.

### Commands

1. `cargo test -p tracewake-content --test golden_fixtures_run`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Routine assignments now instantiate durable live cognition during content load:
`FixtureSchema::to_agent_state` creates deterministic active `Intention` and
`RoutineExecution` records for each routine assignment, with IDs derived from
actor and routine family and trace/goal ancestry attached. This removes the
fixture-only manual injection path previously used by the possession parity
test.

Updated `possession_does_not_reset_intention_001` coverage to assert the loaded
agent state already contains the active intention and routine execution, then
verify controller attach/detach and `continue_routine` leave cognition state
unchanged. Existing no-teleport and blocked-routine fixture tests continue to
assert ordinary action reachability/failure and typed diagnostics.

Verified with:

1. `cargo test -p tracewake-content --test golden_fixtures_run`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo fmt --all --check`
