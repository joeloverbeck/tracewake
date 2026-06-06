# 0002PHA1KERTUI-006: Append-only event log and strict event application

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the append-only log and event-application path to `tracewake-core`'s `events` module.
**Deps**: 0002PHA1KERTUI-003, 0002PHA1KERTUI-005

## Problem

Authoritative physical state may change only through world-event application (Spec 0002 §6.3, §7.4, §11.5). This ticket adds the append-only log (§11.4 — no payload rewrites, no post-commit reordering, no silent drop of unknown versions) and the strict event applier (§11.5 — verify schema version supported, referenced entities exist with required kinds, precondition/old-state matches, apply only declared changes, update derived indexes deterministically, fail loudly on impossible application). It is the single physical-mutation path that the pipeline (008), actions (009–011), no-human advance (015), and replay (013) all route through.

## Assumption Reassessment (2026-06-06)

1. The `events` module and envelope/kind registry exist from ticket 005; the `state`/`location` records exist from ticket 003. This ticket adds `events/log.rs` and `events/apply.rs` and wires them in `events/mod.rs`.
2. The append-only constraints are `specs/0002_…_SPEC.md` §11.4; the application algorithm is §11.5; "state mutation must be routed through event application" is §7.4. An applier that "best effort" applies inconsistent events is explicitly forbidden (§11.5: "a corruption engine").
3. Shared boundary under audit: the apply function signature `(state, world_event) -> Result<(), ApplyError>` consumed by the pipeline (008) and replay (013); both must use this one path (no second applier — §9.5, §20.2).
4. Invariant motivating this ticket: INV-009 (meaningful state changes require events), INV-011 (current-state-only simulation is forbidden — state must be reproducible from fixture + ordered events), and INV-018 (deterministic replay).
5. Deterministic-replay surface: this is *the* mutation path replay re-runs. The applier is a pure function of (pre-state, event); it must reject precondition mismatch loudly (no repair), apply only declared deltas, and update indexes by ordered iteration. No nondeterministic input enters. Replay (013) re-applies the same log and must reach byte-identical state — enforced there; this ticket guarantees application is deterministic and strict.

## Architecture Check

1. A single strict applier keyed on the world event kind (with a non-`world` event being a no-op for physical state) centralizes mutation so there is exactly one place state changes — making "state is reproducible from fixture + log" structurally true. A scattered set of mutators per call site would let a UI/test path mutate behind the kernel (forbidden, §20.2).
2. No backwards-compatibility shims: greenfield; the applier is strict from the first event, no lenient fallback.

## Verification Layers

1. Event-sourced mutation only (INV-009/011; §7.4) -> codebase grep-proof: `state` fields are mutated only inside `events/apply.rs`; no other module takes `&mut` state for physical change.
2. Strict application (§11.5) -> unit test: applying an event whose precondition/old-state does not match the current state returns an error and leaves state unchanged.
3. Append-only (§11.4) -> unit test: the log exposes append + read only; no API rewrites or reorders committed events; unknown `event_schema_version` is rejected, not dropped.

## What to Change

### 1. Append-only log

Add `crates/tracewake-core/src/events/log.rs`: an ordered, append-only event log with monotonic `stream_position`/`global_order` assignment and serialization for replay fixtures and regression tests; no mutation/removal API.

### 2. Strict applier

Add `crates/tracewake-core/src/events/apply.rs`: dispatch on world event kind, validate schema version + referenced entities + old-state precondition, apply only the declared deltas to `state`/`location`, update derived indexes by deterministic (sorted/insertion-ordered) iteration, and return a typed error on any impossible application.

### 3. Wiring

Re-export `log` and `apply` from `events/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/events/log.rs` (new)
- `crates/tracewake-core/src/events/apply.rs` (new)
- `crates/tracewake-core/src/events/mod.rs` (modify — re-export; file created by ticket 005)

## Out of Scope

- Constructing events from proposals / deciding what to emit (ticket 008 and the action tickets).
- Projection rebuild and the replay report (ticket 013) — this ticket provides the apply primitive they call.
- Checksum computation (ticket 007).

## Acceptance Criteria

### Tests That Must Pass

1. Applying a valid `world` event (e.g. an `ActorMoved`/`ContainerOpened`-shaped event) changes exactly the declared state and nothing else.
2. Applying an event with a non-matching old-state precondition errors and leaves state byte-identical to before.
3. Applying an event with an unsupported `event_schema_version` errors (no silent drop).
4. A `diagnostic`/`controller`/`replay_debug` event applied to physical state is a no-op on physical state.

### Invariants

1. Physical state is mutated only inside `events/apply.rs`.
2. The committed log is append-only and order-stable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/apply.rs` (unit tests) — declared-delta-only, precondition-mismatch rejection, version rejection, non-world no-op.
2. `crates/tracewake-core/src/events/log.rs` (unit tests) — append/read ordering, serialization round-trip.

### Commands

1. `cargo test -p tracewake-core events::apply events::log`
2. `cargo build --workspace`
3. Core-crate scope is correct: application operates on in-crate `state` + `events` types; cross-crate replay is exercised in ticket 013's tests.
