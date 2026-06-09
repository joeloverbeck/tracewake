# 0014PHA3AORDLIF-006: Modeled sleep/rest affordance — authoritative state + validation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`state.rs` new sleep/rest affordance state, `actions/defs/sleep.rs` validation + typed `NoSleepAffordance`, state checksum/replay parity), new source guard
**Deps**: None

## Problem

`sleep.rs` validates that the actor exists and that an optional `sleep_place_id` equals the actor's current place, then emits `SleepStarted` (`crates/tracewake-core/src/actions/defs/sleep.rs:40-47,63`); it does not prove a modeled bed/rest/sleep affordance exists at that place. Being physically present somewhere is not the same as the actor knowing and validating a legitimate sleep/rest surface. Sleep is an ordinary causal action with conditional affordances (INV-044/045), so validation must require a modeled sleep-affordance with actor-known provenance and reject absent/closed/forged/stale parameters. This is ORD-HARD-005 (kernel half).

## Assumption Reassessment (2026-06-09)

1. Sleep validation/emit lives in `build_sleep_start_event` at `crates/tracewake-core/src/actions/defs/sleep.rs:15`; the current-place check is `sleep_place_id != actor.current_place_id` at `sleep.rs:40-47`; `SleepStarted` is emitted at `sleep.rs:63`. Existing tests (duration, unreachable-place rejection) are in `sleep.rs` `mod tests` at `sleep.rs:273-446`. Authoritative `PhysicalState` is at `crates/tracewake-core/src/state.rs:122`; `WorkplaceState` (the structural analog for a place-scoped affordance) is at `state.rs:328`.
2. Spec §ORD-HARD-005 and §5.5 (conformance index "Sleep/rest knowledge" row) require: model sleep/rest affordances explicitly in authoritative state (and content schema — ticket 0014PHA3AORDLIF-007); the sleep proposal carries a sleep-affordance id / rest-surface id with actor-known provenance; validation rejects absent/closed/forged/stale surface parameters with a typed `NoSleepAffordance` (or equivalent) reason.
3. Shared boundary under audit: the sleep-affordance contract between authoritative `PhysicalState` (the affordance exists/open) and `sleep.rs` validation (the proposal references a known affordance). The content-authoring half (schema + fixtures + content validation) is ticket -007 and depends on this.
4. Invariants motivating this ticket: **INV-043** (action validation is ordinary-agent validation against state/belief/affordance/access conditions; no player branch), **INV-044** (affordances are conditional, not menu decoration — physical reach + knowledge + access + state), **INV-045** (ordinary survival is causal; no fake meter refills disconnected from world state).
5. Fail-closed validation enforcement surface: sleep validation becomes fail-closed on a missing/closed/forged/stale sleep-affordance parameter, returning a typed blocker reason `NoSleepAffordance`, not a warning. The new affordance state is part of `PhysicalState`, so it enters the state checksum / replay rebuild; the checksum-parity and `state_struct_fields` coverage in `anti_regression_guards.rs` must include the new component so determinism/replay (INV-018) holds. The validator reads authoritative affordance truth only to *validate* (INV-099), while the actor-known provenance of the affordance id is supplied by the proposal (built from the sealed actor-known surface, ticket -001).
6. Schema extension — additive-vs-breaking: adds a sleep/rest affordance component to `PhysicalState` (place-scoped, analogous to `WorkplaceState`). Consumers: `sleep.rs` validation, the state checksum / `serialize_canonical` path, replay rebuild, content schema/validation (ticket -007), and fixtures that must now provide a sleep surface. Additive new component; existing fixtures without sleep surfaces must either gain one or carry an explicit no-sleep diagnostic expectation (handled in -007). Replay of pre-existing history defaults the component (INV-020).

## Architecture Check

1. Modeling the sleep/rest affordance as authoritative place-scoped state (mirroring `WorkplaceState`) and validating the proposal's affordance id against it keeps survival causal (INV-045) and affordances conditional (INV-044) — a "present at a place" check cannot stand in for "a known, open rest surface." Validation stays in the kernel (INV-043).
2. No backwards-compatibility shim: the bare `sleep_place_id == current_place_id` check is replaced, not kept as a fallback; sleep requires the modeled affordance.

## Verification Layers

1. INV-044/045 (conditional causal affordance) -> codebase grep-proof + schema validation: a sleep/rest affordance component exists on `PhysicalState`; sleep validation references it and rejects absent/closed/forged/stale with typed `NoSleepAffordance`.
2. INV-043 (ordinary-agent validation, fail-closed) -> unit test: `sleep.rs` test proves current-place-without-sleep-affordance is rejected with the typed reason (the golden negative fixture is added in ticket -007).
3. INV-018 (deterministic replay) -> replay/golden-fixture check: state checksum parity includes the new component; the no-human capstone replay byte-matches (after -007 provisions fixtures).

## What to Change

### 1. Authoritative sleep/rest affordance state

In `crates/tracewake-core/src/state.rs`, add a place-scoped sleep/rest affordance component (id, place, open/closed state) to `PhysicalState`, analogous to `WorkplaceState`. Include it in the state checksum / `serialize_canonical` path.

### 2. Sleep validation

In `crates/tracewake-core/src/actions/defs/sleep.rs`, require the sleep proposal to carry a sleep-affordance / rest-surface id; validate it exists, is open, and is not forged/stale; reject with a typed `NoSleepAffordance` (or equivalent) blocker reason. Add a unit test for the current-place-without-affordance rejection.

### 3. Source guard + checksum coverage

Add a guard that fails if sleep-start validation checks only `current_place_id` and does not reference the sleep-affordance state / typed validator. Extend the `anti_regression_guards.rs` state-field/checksum-parity coverage to the new component.

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify — sleep/rest affordance component + checksum)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify — affordance validation + typed `NoSleepAffordance` + unit test)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard + checksum-parity coverage; **N-way shared hub**)

## Out of Scope

- Content schema, fixture provisioning, content validation, and the golden negative fixture (ticket 0014PHA3AORDLIF-007 — depends on this).
- The no-human actor-known sleep surface construction (ticket 0014PHA3AORDLIF-001).
- Eat/work affordance changes — sleep only this round.

## Acceptance Criteria

### Tests That Must Pass

1. `sleep.rs` unit test — sleep at the current place with no modeled sleep affordance is rejected with the typed `NoSleepAffordance` reason; a forged/stale affordance id is rejected.
2. `cargo test -p tracewake-core --test anti_regression_guards` — sleep-validation guard and state checksum parity pass.
3. `cargo test -p tracewake-core` — existing sleep duration/unreachable-place tests still pass; replay/checksum parity holds with the new component.

### Invariants

1. Sleep requires a modeled, open, actor-known sleep/rest affordance; presence alone is insufficient (INV-044/045).
2. Sleep validation is fail-closed with a typed blocker; the new state component is checksum/replay-parity covered (INV-043, INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/sleep.rs` (`mod tests`) — current-place-without-sleep-affordance and forged/stale rejection with typed reason.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning current-place-only sleep validation + checksum-parity coverage of the new component.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-core actions::defs::sleep`
3. `cargo test --workspace`
