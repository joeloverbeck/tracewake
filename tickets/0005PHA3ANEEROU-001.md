# 0005PHA3ANEEROU-001: Agent-cognition module scaffold, stable IDs, and need model

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — creates the `agent` cognition module in `tracewake-core`, extends `ids.rs` with cognition ID newtypes, and adds the bounded `NeedKind` / `NeedState` / `NeedPressure` model.
**Deps**: None

## Problem

Phase 3A gives ordinary actors bounded needs that create pressure and candidate goals without granting truth or puppeteering action (Spec 0005 §8.1, §8.2, §3.5; `INV-039`, `INV-045`). Every later Phase 3A structure — intentions, routines, traces, events, content seeds — references the need vocabulary and a cognition ID set, so the agent-cognition module, its stable IDs, and the bounded/banded need model must land first and be canonically serializable, clamped, and replay-safe. The kernel currently has no `agent`/cognition state: `find crates -name '*.rs'` shows no `crates/tracewake-core/src/agent/`, and need pressure has no representation anywhere.

## Assumption Reassessment (2026-06-07)

1. The `agent` module does not exist yet: `crates/tracewake-core/src/` contains `actions`, `epistemics`, `events`, `replay`, plus `state.rs`/`ids.rs`/`view_models.rs`/… but no `agent`/`cognition` directory. `crates/tracewake-core/src/lib.rs` is the module-declaration hub this ticket adds `pub mod agent;` to (mirroring the existing `pub mod epistemics;`).
2. `crates/tracewake-core/src/ids.rs` defines `macro_rules! stable_id_type!` (`ids.rs:75`) and applies it to `FixtureId`/`ActorId`/`PlaceId`/`ContainerId`/`ItemId`/`ActionId`/`SemanticActionId` (`ids.rs:125-133`) over a shared `StableId` inner type with by-value `Ord` and checked construction; this ticket follows that exact discipline for the new cognition IDs rather than inventing a parallel one. Spec §8 leaves exact Rust names to the implementer but requires stable IDs for intentions, routines, candidate goals, and traces.
3. Shared boundary under audit: `NeedKind` and `NeedState` form the vocabulary tickets 002 (intentions), 005 (events), 007 (need effects), 012 (candidate-goal generation), and 015 (content schema) all import — a rename here is repo-wide, so the need set (`Hunger`, `Fatigue`, `Safety`) and ID names are fixed by this ticket (Spec §8.1 first-slice needs are exactly hunger/fatigue/safety).
4. Invariants motivating this ticket: `INV-039` — "Needs are pressures, not puppet strings" (needs influence choices without erasing belief/access/cost) — and `INV-045` — "Ordinary survival is causal" (fake meter refills disconnected from world state are forbidden). The need model must therefore be a bounded pressure value with explicit ancestry, never a free-set meter; `NeedState` exposes no public "set to comfortable" path.
5. Deterministic-replay surface (substrate-only): need values and IDs feed the agent-state checksum/projection and replay ordering that ticket 006 enforces. This ticket introduces no nondeterminism — needs are clamped integers on a fixed `0..=1000` scale with derived bands, IDs are content-stable strings with by-value `Ord`, and no floating point is used (Spec §8.1 forbids float needs). The enforcing surface (deterministic agent-state rebuild) lands in ticket 006; this ticket guarantees the inputs are canonical and clamped.

## Architecture Check

1. A dedicated `agent` module keyed by actor (parallel to `epistemics`) keeps cognition out of ad-hoc fields on `ActorBody` (Spec §6 explicitly forbids cramming cognition into `ActorBody`), preserving the physical/epistemic/cognition separation the kernel already models. A bounded integer `NeedState` with a derived `NeedBand` gives compile-time exhaustiveness for band logic where a float or stringly meter would admit underflow/overflow and silent refills (`INV-045`).
2. No backwards-compatibility shims: the `agent` module is greenfield and the new IDs extend the existing `stable_id_type!` macro without aliasing.

## Verification Layers

1. Needs-are-pressures (`INV-039`) -> invariants alignment check + manual review: `NeedState` exposes pressure/band accessors and event-/effect-driven mutators only; no public setter forces a value without an ancestry-carrying cause.
2. Causal survival / no fake refill (`INV-045`) -> codebase grep-proof + unit test: need mutation paths are limited to clamped deltas; a test proves no underflow/overflow across the `0..=1000` scale and that there is no instant-comfortable shortcut method.
3. Determinism (`INV-018`, substrate-only) -> codebase grep-proof + unit test: new IDs use `stable_id_type!`; `NeedState` derives `Ord`/`Eq` from integer fields with a round-trip test proving byte-identical canonical serialization. Enforcement of full agent-state rebuild is deferred to ticket 006 (cited).

## What to Change

### 1. Cognition ID newtypes

Extend `crates/tracewake-core/src/ids.rs` with `stable_id_type!` to add `IntentionId`, `RoutineTemplateId`, `RoutineExecutionId`, `CandidateGoalId`, `DecisionTraceId`, and `StuckDiagnosticId`, plus an `AgentProjectionVersion` schema marker mirroring the existing `EpistemicProjectionVersion`/content version markers. IDs must be deterministic, non-display, whitespace-free, and orderable by value.

### 2. Need model

Add `crates/tracewake-core/src/agent/need.rs` with:
- `NeedKind` enum: `Hunger`, `Fatigue`, `Safety` (exhaustive for Phase 3A).
- `NeedState`: a clamped value on `0..=1000` (Spec §8.1 canonical scale) with a derived `NeedBand` (`Comfortable` 0–249, `Rising` 250–499, `Urgent` 500–749, `Severe` 750–1000). Provide clamped `apply_delta`, band query, threshold-crossing detection between two values, and canonical serialization. No public method may set a need to a target value without a delta+cause.
- `NeedPressure` (Spec §8.2): a derived per-tick evaluation carrying actor, need kind, value+band, threshold crossing if any, source ancestry tag (tick delta / action / routine effect / fixture initial), interrupt-eligibility flag, and an actor-known explanation fragment plus debug-only detail. It is derived, not stored authority.

### 3. Module registration

Add `crates/tracewake-core/src/agent/mod.rs` declaring `pub mod need;` and re-exporting the public surface, and add `pub mod agent;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/agent/mod.rs` (new)
- `crates/tracewake-core/src/agent/need.rs` (new)
- `crates/tracewake-core/src/ids.rs` (modify — add cognition ID newtypes via `stable_id_type!`)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod agent;`)

## Out of Scope

- Candidate-goal and intention types (ticket 002).
- Routine template/execution and decision/stuck models (tickets 003, 004).
- Phase 3A event kinds and need effects (tickets 005, 007).
- Any need-changing behavior, generation, or action wiring (tickets 007, 012).
- Content/fixture schema for initial needs (ticket 015).

## Acceptance Criteria

### Tests That Must Pass

1. A `NeedState` value round-trips through canonical serialize/deserialize byte-identically and reports the correct `NeedBand` at each band boundary (249/250, 499/500, 749/750).
2. `apply_delta` clamps at 0 and 1000 with no underflow/overflow across a sweep of deltas, and threshold-crossing detection reports the band transition between two values.
3. Each new cognition ID rejects a display-name-shaped string (whitespace/empty) and orders by stable-string value.

### Invariants

1. `NeedState` carries no public "set comfortable"/instant-refill path; every change is a clamped delta requiring a cause tag (`INV-045`).
2. Need values are integers on a fixed bounded scale ordered by value, never floats or hash-iteration order (`INV-039`, determinism).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/need.rs` (unit tests) — band boundaries, clamp sweep, threshold crossing, round-trip, pressure derivation.
2. `crates/tracewake-core/src/ids.rs` (unit tests) — extend existing ID tests with the cognition newtypes' rejection/ordering.

### Commands

1. `cargo test -p tracewake-core agent::need`
2. `cargo test -p tracewake-core ids`
3. A core-crate unit-test scope is correct here because the need model has no cross-crate consumers until tickets 005/007/015 land.
