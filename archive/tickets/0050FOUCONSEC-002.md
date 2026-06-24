# 0050FOUCONSEC-002: Declared world-process registry + cadence + private `DueProcessInvocation` (additive, unwired)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a core-owned declared-process registry/cadence surface and a private `DueProcessInvocation` type (no behavior wired in yet)
**Deps**: None

## Problem

Spec-0050 §4.2 (driver F-02): the public request admits raw caller-authored process events. `WorldStepTransactionRequest` carries `world_process_events: Vec<EventEnvelope>` (`crates/tracewake-core/src/scheduler.rs:230`); inside the transaction the coordinator iterates those arbitrary envelopes, appends each to the scratch log, and applies them via `apply_event_stream` (`crates/tracewake-core/src/events/apply.rs:44`) — bypassing any declared process registry, cadence, trigger, typed invocation, or process-owned proposal. The caller decides not just *which process is due* but *which authoritative event already happened*.

This ticket lands the first half of the fix **additively and unwired**: a core-owned declared-process registry with cadence/trigger evaluation that produces a private typed `DueProcessInvocation`, created only by core. It does not yet reshape the request or delete the raw-envelope path (atomic cutover `0050FOUCONSEC-003`).

## Assumption Reassessment (2026-06-24)

1. `WorldStepTransactionRequest` exposes `pub world_process_events: Vec<EventEnvelope>` at `crates/tracewake-core/src/scheduler.rs:230`; the coordinator appends+applies those raw envelopes directly via `apply_event_stream` (`crates/tracewake-core/src/events/apply.rs:44`). Verified at `HEAD` (`8d7c119`). `DueProcessInvocation` is net-new (grep returns no existing type — a deliverable, not a rename).
2. Spec-0050 §4.2 and §9.2 are authoritative: §9.2 assigns the *registry shape* (declared processes produce ordinary proposals, or a typed process transition result) to the implementer as a recorded choice; either way cadence/trigger/source ancestry must be explicit and raw final envelopes must not cross the public request boundary.
3. Shared boundary under audit: the world-affecting causal-commitment boundary owned by the coordinator in `crates/tracewake-core/src/scheduler.rs` and the event-application boundary `apply_event_stream` (`events/apply.rs`). This ticket adds the registry/invocation surface; it does not yet route any invocation through the coordinator (that is `-003`).
4. `INV-088` (regional/world processes are declared causal processes) motivates this ticket: a declared process requires source, cadence/trigger, inputs, random model, scope, delivery channel, traces, ancestry, and replay/debug visibility. `INV-001`/`INV-009`/`INV-010` require modeled causes and cause-bearing events. The `DueProcessInvocation` names the process, trigger/cadence witness, effective tick, source event IDs, ruleset/content version, and deterministic random provenance — the declared-process contract `INV-088` requires.
5. Enforcement surface: deterministic replay (`INV-018`) and no-direct-dispatch (architecture `04` / execution `05`). The registry derivation must be a pure function of replayable state/log with stable order; the invocation must not embed a finished authoritative `EventEnvelope` (that is the F-02 defect). The downstream enforcement — routing the invocation through the shared pipeline or an owned typed transition, committed in the scratch transaction — lands in `0050FOUCONSEC-003`. No actor-knowledge surface is touched (processes are world-level).

## Architecture Check

1. A private `DueProcessInvocation` created only by core, after evaluating a declared registry, replaces caller authority over *which authoritative event already happened* with caller authority over *nothing* — the caller supplies only controlled input (`-003`). Building it additively/unwired keeps the cutover diff focused.
2. No backwards-compatibility shims: the raw-envelope `world_process_events` path is **not** preserved alongside the new surface — its deletion is the atomic cutover (`-003`). This ticket adds only the new path; it creates no dual-commitment surface.

## Verification Layers

1. `INV-088` (declared processes carry cadence/trigger/ancestry) → invariants alignment check: `DueProcessInvocation` fields enumerate source, cadence/trigger witness, effective tick, source event IDs, ruleset version, random provenance — manual review + grep-proof of the field set.
2. `INV-018` (deterministic replay) → replay/golden-fixture check: a unit test asserts the registry yields identical due-invocation sets for identical state across repeated calls and storage-order permutations.
3. `INV-001`/`INV-009`/`INV-010` (modeled causes, cause-bearing events) → schema validation: the invocation type carries no finished `EventEnvelope`; any event it ultimately produces (in `-003`) is built by an owned builder — grep-proof that the invocation struct holds no `EventEnvelope` field.

## What to Change

### 1. Add a core-owned declared-process registry + cadence/trigger evaluation

In `crates/tracewake-core/src/scheduler.rs` (or an extracted private module — implementer-recorded choice per §9.2), add a registry of declared world processes and a deterministic evaluation that, given replayable state/log and the target tick, yields the due processes. Record the registry-shape choice and rationale in the doc comment.

### 2. Add the private `DueProcessInvocation` type

Add a private (`pub(crate)` at most) typed invocation naming the process, trigger/cadence witness, effective tick, source event IDs, ruleset/content version, and deterministic random provenance if any. Its constructors are private to core. It carries **no** finished `EventEnvelope`. It is not yet routed through the coordinator.

### 3. Unit-test the registry derivation in isolation

Add an additive test in `crates/tracewake-core/tests/world_step_coordinator.rs`: a declared process with a known cadence becomes due at the right tick and is absent one tick earlier; the derivation is order-insensitive and deterministic.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)

## Out of Scope

- Reshaping `WorldStepTransactionRequest` or deleting the raw-envelope `world_process_events` apply path — atomic cutover `0050FOUCONSEC-003`.
- Routing a `DueProcessInvocation` through the coordinator/pipeline — `0050FOUCONSEC-003`.
- Actor eligibility discovery — `0050FOUCONSEC-001`.
- Compile-fail boundary fixtures — `0050FOUCONSEC-004`.

## Acceptance Criteria

### Tests That Must Pass

1. The registry derivation unit test passes: due-at-tick correctness, absent-one-tick-earlier, deterministic order.
2. Grep-proof: `DueProcessInvocation` holds no `EventEnvelope` field.
3. `cargo test -p tracewake-core --test world_step_coordinator` is green and `cargo build --workspace --all-targets --locked` compiles with the unwired registry present.

### Invariants

1. A `DueProcessInvocation` carries the declared-process provenance `INV-088` requires and no finished authoritative event (`INV-001`/`INV-009`/`INV-010`).
2. The registry derivation is a pure, deterministically-ordered function of replayable state/log (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — additive unit test of the declared-process registry derivation (cadence due/not-due, determinism, no-`EventEnvelope` field assertion).

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented the unwired declared-world-process substrate in
`crates/tracewake-core/src/scheduler.rs`. The recorded registry shape is a
private scheduler-owned cadenced process map keyed by `ProcessId`; each process
records first due tick, nonzero cadence, source event IDs, content manifest
identity, and optional deterministic random provenance. The private
`DueProcessInvocation` carries process ID, trigger witness, effective tick,
source event IDs, content identity, and random provenance, and it carries no
finished `EventEnvelope`.

Deviation: like `0050FOUCONSEC-001`, the ticket named an integration test in
`crates/tracewake-core/tests/world_step_coordinator.rs` while also requiring
the new surface to remain private/`pub(crate)` at most. The proof therefore
landed as the internal unit test
`scheduler::tests::declared_process_derivation_is_cadenced_private_and_stable`
instead of widening the API solely for test reachability. The existing
`world_step_coordinator` integration suite passed unchanged during the
implementation pass.

Verification run:

- `cargo fmt --all --check`
- `cargo test -p tracewake-core declared_process_derivation_is_cadenced_private_and_stable`
- `cargo test -p tracewake-core --test world_step_coordinator`
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
