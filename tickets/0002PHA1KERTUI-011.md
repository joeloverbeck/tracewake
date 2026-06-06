# 0002PHA1KERTUI-011: inspect (query) and wait/advance action definitions

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — registers `look`/`inspect_place`, `inspect_entity`, and `wait` action definitions; emits `ActorWaited`/`TimeAdvanced` for wait only.
**Deps**: 0002PHA1KERTUI-008, 0002PHA1KERTUI-004

## Problem

Phase 1 needs inspect actions for the current place and visible/reachable objects (§4.1 area 8) and wait/advance through the same causal scheduler/time machinery used without a human (area 11). Inspect is a view/query action that produces a view-model response and creates **no** world event and no actor knowledge (Spec 0002 §11.3, §14.4); wait advances deterministic simulation time and emits `ActorWaited`/`TimeAdvanced`.

## Assumption Reassessment (2026-06-06)

1. Registry/pipeline exist from ticket 008; `SimTick` and the scheduler advance live in ticket 004; `ActorWaited`/`TimeAdvanced` event kinds exist from ticket 005; §12.4 visibility query lives in ticket 003. This ticket adds a definition file and registers it, modifying `actions/registry.rs` (008).
2. Semantics are `specs/0002_…_SPEC.md` §14.4: `look`/`inspect_place` and `inspect_entity` produce query responses with validation reports but no world event ("inspect is a view/query action … It must not create actor knowledge", §11.3); `wait` emits `ActorWaited` and/or `TimeAdvanced`.
3. Shared boundary under audit: inspect reads the §12.4 visibility query (ticket 003) and returns a view-model payload (structures defined in ticket 012); wait drives the ticket-004 tick advance. Both go through the ticket-008 pipeline.
4. Invariant motivating this ticket: INV-024 (no telepathy) / INV-067 (embodied mode shows actor-known reality) — inspect returns only §12.4-visible facts and creates no stored knowledge, so it cannot leak hidden truth into actor state.
5. No-leak + deterministic-replay surface: inspect is read-only over the visibility filter (no state mutation, no knowledge write) — it cannot widen what the actor knows. `wait` advances time deterministically and is eventful so replay reproduces causal order (§10.4). This ticket introduces no leakage path and no nondeterminism; the firewall is the §12.4 query it calls.

## Architecture Check

1. Modeling inspect as a query-only pipeline action (validation report, no event, no knowledge write) keeps the event log free of inspection noise while still routing through the one validation path — Phase 2 can later turn inspect into an observation event without a parallel code path. Emitting an event or caching "seen" facts now would pre-empt Phase 2 epistemics and risk leakage.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. No-telepathy (INV-024/067) -> unit test: `inspect_entity` on a closed-opaque container returns the container but not its hidden contents; inspect writes no field on the actor.
2. Eventless inspect (§11.3) -> unit test: an accepted inspect appends no `world` event (log length unchanged).
3. Deterministic wait (INV-018; §10.4) -> replay/golden check: `wait` emits `ActorWaited`/`TimeAdvanced` and replay reproduces the tick/order.

## What to Change

### 1. inspect

Add `crates/tracewake-core/src/actions/defs/inspect.rs`: `look`/`inspect_place` and `inspect_entity` validating reachability/visibility and returning a view-model query response; no world event; no knowledge mutation.

### 2. wait

Add `crates/tracewake-core/src/actions/defs/wait.rs`: `wait` (tick count or default one tick) advancing simulation time via the scheduler and emitting `ActorWaited`/`TimeAdvanced`.

### 3. Registration

Register all three in `actions/registry.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/inspect.rs` (new)
- `crates/tracewake-core/src/actions/defs/wait.rs` (new)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register the three; file created by ticket 008)

## Out of Scope

- move/open/close (009), take/place (010).
- View-model data structures inspect returns (ticket 012 defines them; inspect references them).
- Any belief/observation recording (Phase 2).

## Acceptance Criteria

### Tests That Must Pass

1. `inspect_entity` on a closed-opaque container returns the container object but excludes hidden contents.
2. An accepted inspect appends no `world` event.
3. `wait` advances `SimTick` deterministically and emits `ActorWaited`/`TimeAdvanced`; replay reproduces the tick.

### Invariants

1. Inspect creates no world event and no actor-knowledge field.
2. Wait advances time only through the deterministic tick machinery.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/inspect.rs` (unit tests) — visibility filtering, no-event, no-knowledge.
2. `crates/tracewake-core/src/actions/defs/wait.rs` (unit tests) — deterministic tick advance + event emission.

### Commands

1. `cargo test -p tracewake-core actions::defs::inspect actions::defs::wait`
2. `cargo build --workspace`
3. Unit scope is correct; embodied-view leakage is additionally regression-tested end-to-end in ticket 022.
