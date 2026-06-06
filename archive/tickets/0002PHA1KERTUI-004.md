# 0002PHA1KERTUI-004: Deterministic time and scheduler primitives

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `time` and `scheduler` modules to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-002

## Problem

Phase 1 determinism is an acceptance gate, not best effort. Spec 0002 §10.3 requires an explicit ordering key for multiple proposals/scheduled actions (sim tick, scheduled phase/source class, actor/process stable ID, proposal sequence number, action definition ID, target ID tuple, final tie-breaker), and §10.4 requires discrete simulation ticks with no wall-clock authority. No outcome may depend on insertion order into an unordered collection, OS scheduling, or terminal timing. The pipeline (ticket 008) and no-human advance (ticket 015) both consume this ordering machinery.

## Assumption Reassessment (2026-06-06)

1. No `time`/`scheduler` module exists; registers `pub mod time; pub mod scheduler;` in `crates/tracewake-core/src/lib.rs` (created by 001), using ID newtypes from ticket 002.
2. The ordering-key tuple and tick model are `specs/0002_…_SPEC.md` §10.3 and §10.4; the scheduler responsibility ("minimal deterministic scheduling and no-human advance … must use stable ordering") is §9.2. `wait`/no-human advance share the same tick model (§10.4).
3. Shared boundary under audit: the `OrderingKey` type and tick type consumed by the action pipeline (008), event envelopes' `sim_tick`/`ordering_key` fields (005), and no-human advance (015). Fixed here.
4. Invariant motivating this ticket: INV-017 (randomness must be seedable and auditable) and INV-018 (deterministic replay) — the scheduler's ordering must be reproducible from recorded keys, with no wall-clock or thread-race input.
5. Deterministic-replay surface: the ordering key is the canonical tie-break that replay relies on to reproduce event order. This ticket forbids `std::time`/`Instant`/thread-scheduling input in any outcome-affecting path; ordering is a total order over the §10.3 tuple. Enforcement (replay) is ticket 013; this ticket guarantees the ordering is total and key-derived.

## Architecture Check

1. A pure ordering-key comparator plus an explicit tick counter (no event loop driven by wall-clock) keeps time as data, making replay a matter of re-applying recorded keys. An ambient real-time scheduler would make outcomes depend on host timing — forbidden by §10.2/§10.3.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Deterministic ordering (INV-018; §10.3) -> unit test: a shuffled set of proposals sorts to one canonical order by the ordering key across repeated runs.
2. No wall-clock authority (INV-017; §10.4) -> codebase grep-proof: `rg 'std::time|Instant::now|SystemTime'` returns no match in outcome-affecting scheduler/time code.
3. Single-layer rationale beyond these two: the scheduler has no view/leak surface in Phase 1; the two determinism layers are the load-bearing ones.

## What to Change

### 1. Time

Add `crates/tracewake-core/src/time.rs` with a discrete `SimTick` newtype and tick-advance helpers. No conversion from or to wall-clock for outcome logic.

### 2. Scheduler ordering

Add `crates/tracewake-core/src/scheduler.rs` with the `OrderingKey` (the §10.3 tuple), a total-order comparator, deterministic proposal-sequence assignment at intake, and a minimal deterministic advance step that later tickets (pipeline, no-human) drive.

### 3. Registration

Add `pub mod time;` and `pub mod scheduler;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/time.rs` (new)
- `crates/tracewake-core/src/scheduler.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add module declarations; file created by ticket 001)

## Out of Scope

- The full proposal/commit pipeline that uses the ordering key (ticket 008).
- The no-human advance command and its events (ticket 015).
- Randomness/RNG streams — Phase 1 avoids randomness (§10.5); none is introduced here.

## Acceptance Criteria

### Tests That Must Pass

1. Sorting a set of proposals by `OrderingKey` is identical across repeated runs and independent of insertion order.
2. Advancing time changes only `SimTick`, deterministically.
3. `rg 'Instant::now|SystemTime::now'` finds no outcome-affecting use in `time.rs`/`scheduler.rs`.

### Invariants

1. Proposal/scheduled-action order is a total order derived solely from recorded keys.
2. No outcome depends on wall-clock or thread scheduling.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (unit tests) — ordering stability under shuffled insertion.
2. `crates/tracewake-core/src/time.rs` (unit tests) — tick advance determinism.

### Commands

1. `cargo test -p tracewake-core scheduler time`
2. `cargo build --workspace`
3. Unit scope is correct: ordering is a pure function of the key tuple, fully exercisable in-crate.

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::time::SimTick` with deterministic tick construction and advance helpers.
- Added `tracewake_core::scheduler` with schedule phases, scheduler source IDs, proposal sequence assignment, ordering keys, sorted scheduled payloads, and a minimal deterministic scheduler.
- Added `ProcessId` to the stable ID vocabulary for process-origin scheduler keys.
- Registered `pub mod time;` and `pub mod scheduler;` in the core crate.

Deviations from original plan:
- The documented combined Cargo filter `cargo test -p tracewake-core scheduler time` is not accepted by Cargo as written, so verification used separate `scheduler` and `time` filters.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core scheduler` passed: 3 tests.
- `cargo test -p tracewake-core time` passed: 1 test.
- `cargo build --workspace` passed.
- `rg 'std::time|Instant::now|SystemTime::now' crates/tracewake-core/src/time.rs crates/tracewake-core/src/scheduler.rs` returned no matches.
