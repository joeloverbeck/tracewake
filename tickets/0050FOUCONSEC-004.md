# 0050FOUCONSEC-004: Compile-fail boundary fixtures for due-work / raw-process injection

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — adds external-crate compile-fail negative fixtures and registers them in the negative-fixture runner
**Deps**: 0050FOUCONSEC-003

## Problem

Spec-0050 §4.1/§4.2 "strongest guard" + §6.2: once the world-step request is reshaped (`-003`), a compile-time boundary must prove that downstream callers *cannot* re-introduce caller-supplied due work — inject `due_actor_ids`, supply a raw `EventEnvelope` as a process event, or construct a `DueProcessInvocation` from outside core. Rust privacy is the first defense; a compile-fail negative fixture proves the boundary from outside `tracewake-core`. A source-scan source guard is only a topology alarm; the compile-fail fixture is the real boundary proof.

## Assumption Reassessment (2026-06-24)

1. The external-crate negative-fixture corpus lives at `tests/negative-fixtures/` and is driven by `crates/tracewake-core/tests/negative_fixture_runner.rs` (verified: the runner `include_str!`s fixtures and asserts banned paths/compile-fail). This ticket extends that existing pattern; it does not add a second compile-fail framework.
2. Spec-0050 §6.2 is authoritative: extend the existing external-crate negative fixtures rather than introduce `trybuild`/a new harness. `-003` has already made `due_actor_ids`/`world_process_events` private/removed and `DueProcessInvocation` constructors private (`-002`).
3. Shared boundary under audit: the `tracewake-core` public surface of `WorldStepTransactionRequest` and `DueProcessInvocation`, exercised from the external negative-fixture crate. The fixtures only attempt-and-fail; they wire no behavior.
4. `INV-103` (scheduler is not a cognition authority) and `INV-088` (declared processes) motivate the boundary: callers must not supply due-actor populations or raw process events. The fixtures prove the removed surface stays unreachable.
5. Enforcement surface: the no-direct-dispatch / authority boundary (architecture `04`, execution `05`). This is a compile-time guard over that boundary — it introduces no runtime path, no replay or actor-knowledge surface; it only asserts the boundary is unrepresentable from outside core. The runtime enforcement is in `-003`.

## Architecture Check

1. A compile-fail fixture proves unrepresentability — a stronger guarantee than a runtime test or a source scan, because it makes the misuse fail to build rather than fail at runtime. Extending the existing corpus avoids a second framework.
2. No backwards-compatibility shims: the fixtures assert the *absence* of the old public surface; they introduce no aliasing.

## Verification Layers

1. `INV-103`/`INV-088` (no caller-supplied due work) → codebase grep-proof + compile-fail check: the negative fixtures attempting to set `due_actor_ids`, pass a raw `EventEnvelope`, or build a `DueProcessInvocation` externally all fail to compile, verified by the runner.
2. Single-surface note: this is a compile-time boundary ticket; its one invariant surface (the authority boundary) maps to the compile-fail runner — no runtime layer applies.

## What to Change

### 1. Add compile-fail negative fixtures

Under `tests/negative-fixtures/`, add fixtures (as surfaced — one per banned construction) that, from outside `tracewake-core`, attempt to: (a) construct `WorldStepTransactionRequest` with a caller-supplied due-actor population; (b) supply a raw `EventEnvelope` as process work; (c) construct or invoke a `DueProcessInvocation` directly. Each must fail to compile.

### 2. Register the fixtures in the runner

Extend `crates/tracewake-core/tests/negative_fixture_runner.rs` to drive the new fixtures and assert each fails to compile (matching the existing banned-path/compile-fail assertion style).

## Files to Touch

- `tests/negative-fixtures/` (new — compile-fail fixture members, added as surfaced under the existing corpus crate)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — file created by neither; pre-existing, registers the new fixtures)

## Out of Scope

- The runtime request reshape and caller flip — `0050FOUCONSEC-003`.
- Compile-fail fixtures for the actor-step outcome boundary (`-005`), TUI perception export (`-006`), and `EventId` uniqueness (`-007`) — owned by those tickets.

## Acceptance Criteria

### Tests That Must Pass

1. Each new negative fixture fails to compile from outside `tracewake-core`, asserted by `negative_fixture_runner`.
2. `cargo test -p tracewake-core --test negative_fixture_runner` is green (the runner passes when the fixtures fail to compile as expected).

### Invariants

1. Caller-supplied due-actor populations, raw process `EventEnvelope`s, and external `DueProcessInvocation` construction are unrepresentable outside `tracewake-core` (`INV-103`/`INV-088`).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/` — new compile-fail fixtures for due-work/raw-process/invocation injection.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — registers and asserts the new fixtures.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo build --workspace --all-targets --locked`
