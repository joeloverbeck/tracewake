# 0003PHA1SPIANT-006: Scheduler / no-human no-direct-dispatch conformance

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (named scheduler conformance test; reviewed marker-event allowlist)
**Deps**: 0003PHA1SPIANT-001

## Problem

The scheduler today is disciplined — it produces ordering keys, no-human windows, controller/system marker events, and actor transaction/proposal invocations (`scheduler.rs:7-90`, `:294-306`, `:1770-1840`), and anti-regression tests source-scan scheduler hazards (`anti_regression_guards.rs:22-63`). But nothing structurally prevents the scheduler from becoming a "second actor brain": directly calling primitive action builders, mutating `PhysicalState`/`AgentState`, or using hidden truth as actor cognition. Spec `0003` §5.5 / SPINE-AC-006 require a named scheduler conformance test with a reviewed allowlist for marker-event constructors.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/scheduler.rs` uses typed phases/order keys (`:7-90`), deterministic sorted windows/actors (`:294-306`), and no-human event markers / proposal sequencing (`:1770-1840`). Existing source scans live in `crates/tracewake-core/tests/anti_regression_guards.rs:22-63`. Action definitions live in `crates/tracewake-core/src/actions/defs/*.rs`; the legitimate dispatch surface is the registry/pipeline (`actions/registry.rs`, `actions/pipeline.rs:158` `run_pipeline`).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-006 mandates: the scheduler may produce ordering keys, no-human windows, controller/system marker events, and actor transaction/proposal invocations; it must not directly call primitive action builders bypassing proposal validation, must not mutate authoritative state, and must not use hidden truth as cognition; current source scans are expanded into a named scheduler conformance test with a reviewed allowlist for marker-event constructors.
3. Boundary under audit: the scheduler ↔ action-pipeline ↔ authoritative-state seam. **Depends on 0003PHA1SPIANT-001**: once state mutation is sealed behind the capability, "scheduler mutates state directly" becomes a compile-time impossibility for the field path, and this ticket adds the test asserting the scheduler does not even attempt the dispatch/marker-bypass routes.
4. INV motivating this ticket: `INV-091` (no-human / autonomous operation), `INV-103`/`INV-104` (no direct dispatch; single mutation path), `INV-108` (scheduler is an orderer, not a world-authority). Restated: the scheduler schedules proposals and transaction inputs and emits marker events only — it never enacts primitive outcomes.
5. Deterministic-replay / no-leak surface touched: the test asserts the scheduler does not import/call `actions/defs/*` builders except through the registry/pipeline, does not mutate sealed state (guaranteed by 001), and does not read hidden truth as cognition. It adds no nondeterminism; it reads source structure and runtime behavior of the existing deterministic scheduler.

## Architecture Check

1. Promoting the smoke source-scan into a named conformance test with an explicit, reviewed allowlist for the few legitimate marker-event constructors makes "the scheduler stays an orderer" a durable gate rather than incidental cleanliness, and composes with 001's compile-time mutation seal (defense in depth: compile-time for field writes, test-time for dispatch/marker bypass).
2. No backwards-compatibility shim: the allowlist is explicit and minimal; there is no catch-all permitting scheduler→action-builder calls.

## Verification Layers

1. `INV-104` (no direct dispatch) -> codebase grep-proof: scheduler does not import/call `actions/defs/*` builders except via allowlisted registry/pipeline surfaces.
2. `INV-104` (single mutation path) -> compile/runtime: scheduler cannot mutate sealed `PhysicalState`/`AgentState` fields (guaranteed by 0003PHA1SPIANT-001), asserted by a runtime no-checksum-change-outside-pipeline test.
3. `INV-091`/`INV-108` (no-human orderer, not a brain) -> conformance test: scheduler only schedules proposals/transactions/marker events.

## What to Change

### 1. Named scheduler conformance test

Add `scheduler_never_direct_dispatches_primitive_action`: a source scan asserting `scheduler.rs` does not call `actions/defs/*` primitive builders except through the allowlisted registry/pipeline, with a reviewed allowlist constant for the legitimate marker-event constructors.

### 2. Runtime no-direct-mutation assertion

Add a runtime assertion that a scheduler no-human advance changes authoritative state only via the pipeline (event-log length / checksum change correlates with pipeline application), leveraging the post-001 seal.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `scheduler_never_direct_dispatches_primitive_action` + reviewed allowlist)

## Out of Scope

- The state-mutation seal itself (0003PHA1SPIANT-001).
- Pipeline append-before-apply ordering test (0003PHA1SPIANT-008).
- Any change to scheduler ordering, phases, or no-human sequencing semantics.

## Acceptance Criteria

### Tests That Must Pass

1. `scheduler_never_direct_dispatches_primitive_action` — fails if the scheduler imports/calls action-definition builders except through allowed registry/pipeline surfaces.
2. The runtime assertion that scheduler advance mutates state only via the pipeline passes.
3. `cargo test --workspace` passes.

### Invariants

1. The scheduler schedules proposals/transactions/marker events only; it never direct-dispatches primitive actions (`INV-104`, `INV-108`).
2. The scheduler cannot mutate authoritative state directly (`INV-104`; enforced structurally by 0003PHA1SPIANT-001).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `scheduler_never_direct_dispatches_primitive_action` (source scan + runtime).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Added `scheduler_never_direct_dispatches_primitive_action`, a named scheduler conformance guard.
- Added a documented scheduler marker/event allowlist for duration-completion builders, no-human process markers, and replayable agent-stream diagnostics.
- Source-scanned `scheduler.rs` against direct primitive action imports/builders and asserted ordinary scheduler proposals still route through `run_pipeline` and actor autonomy through `ActorDecisionTransaction::run`.
- Added a runtime comparison proving a no-human scheduled wait adds only diagnostic scheduler markers while its ordinary event sequence matches direct shared-pipeline execution.

Verification:
- `cargo test -p tracewake-core --test anti_regression_guards scheduler_never_direct_dispatches_primitive_action`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
