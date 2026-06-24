# 0050FOUCONSEC-001: Core-owned loaded-actor eligibility derivation (additive, unwired)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a private core-owned due-actor eligibility derivation in `tracewake-core` (no behavior wired in yet)
**Deps**: None

## Problem

Spec-0050 §4.1 (driver F-01): production world steps do not derive loaded-world due work.
`WorldStepTransactionRequest` publicly carries `due_actor_ids: Vec<ActorId>` (`crates/tracewake-core/src/scheduler.rs:228`), the coordinator sorts/dedups and iterates the *supplied* IDs, and every production caller passes `Vec::new()` — only tests inject due actors. The world cannot, through production usage, advance an unpossessed actor.

This ticket lands the first half of the fix **additively and unwired**: a deterministic, possession-neutral, core-owned derivation of the eligible loaded actors due at a tick, built from authoritative scheduling state. It does not yet reshape the request or flip any caller (that is the atomic cutover in `0050FOUCONSEC-003`); landing it separately keeps the cutover diff reviewable and lets the derivation be unit-tested in isolation.

## Assumption Reassessment (2026-06-24)

1. `WorldStepTransactionRequest` exposes `pub due_actor_ids: Vec<ActorId>` at `crates/tracewake-core/src/scheduler.rs:228`, and the coordinator iterates the supplied IDs rather than deriving them; verified against the working tree at `HEAD` (`8d7c119`). The derivation added here is net-new (no existing symbol to reuse — grep for an existing eligibility derivation returns none; this is a deliverable, not a rename).
2. Spec-0050 §4.1 and §9.1 are authoritative: §9.1 assigns the *eligibility representation* (per-actor next-decision tick, scheduled opportunity queue, deterministic cadence from actor state, or another replayable structure) to the implementer as a recorded choice — it must be core-owned, deterministic, and possession-neutral.
3. Shared boundary under audit: the loaded-world scheduling state owned by `DeterministicScheduler` in `crates/tracewake-core/src/scheduler.rs`. The derivation reads that state only; it introduces no new public surface and is consumed by no caller until `0050FOUCONSEC-003`.
4. `INV-103` (the scheduler is not a cognition authority) motivates this ticket: the scheduler may *choose the next actor/time window* and invoke the decision transaction, but it must not construct action proposals from raw state. This derivation selects *which actors are due* only; it constructs no cognition and reads no hidden truth.
5. Enforcement surface: deterministic replay (`INV-018`, `INV-092`). This derivation is substrate feeding the wired one-tick step in `0050FOUCONSEC-003`; it must be a pure function of replayable scheduling state with a stable, sorted output order (no `HashMap` iteration order, no wall-clock). It introduces no actor-knowledge leakage path: actor eligibility is a scheduling fact, not a cognition input. The downstream enforcement (replay reconstruction, no-leak interval product) lands in `0050FOUCONSEC-003`/`-006`.

## Architecture Check

1. Building the derivation additively and unwired first means the atomic request-reshape cutover (`-003`) is a focused diff that *wires* an already-tested derivation rather than introducing and wiring it at once. The derivation is a pure read over scheduling state, so it is unit-testable without the full world-step transaction.
2. No backwards-compatibility aliasing or shims: this ticket adds a new private path only. The public `due_actor_ids` field is *not* touched here — its removal is the atomic cutover's job (`-003`), so no transitional dual surface is created.

## Verification Layers

1. `INV-103` (scheduler chooses, does not cognize) → invariants alignment check: the derivation returns `ActorId`s only, reads scheduling state only, constructs no proposal — manual review + grep-proof that it does not call proposal/decision constructors.
2. `INV-018`/`INV-092` (deterministic replay) → replay/golden-fixture check: a unit test asserts identical sorted output for identical scheduling state across repeated calls and across input-insertion-order permutations.
3. Single-surface note: this ticket adds one private derivation with two invariant surfaces (above), each mapped to its own proof; no cross-artifact mapping beyond these is applicable.

## What to Change

### 1. Add a core-owned due-actor eligibility derivation

In `crates/tracewake-core/src/scheduler.rs`, add a private function (or private submodule) that, given the authoritative scheduling state and the target tick, returns the deterministically-ordered set of eligible loaded actor IDs due at that tick. The eligibility representation is the implementer's recorded choice per spec §9.1 — record the chosen structure and its rationale in the function's doc comment. Output must be sorted/deduplicated by a stable key. The function is `pub(crate)` at most; it is not wired into `advance_world_one_tick` / the coordinator in this ticket.

### 2. Unit-test the derivation in isolation

Add an additive test in `crates/tracewake-core/tests/world_step_coordinator.rs` constructing loaded scheduling state with ≥1 due unpossessed actor and ≥1 not-yet-due actor, asserting the derivation returns exactly the due set in stable order, and asserting order-insensitivity to insertion order.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)

## Out of Scope

- Reshaping `WorldStepTransactionRequest` or removing `due_actor_ids` — that is the atomic cutover in `0050FOUCONSEC-003`.
- Wiring the derivation into the coordinator or any production caller — `0050FOUCONSEC-003`.
- World-process discovery — `0050FOUCONSEC-002`.
- Compile-fail boundary fixtures — `0050FOUCONSEC-004`.

## Acceptance Criteria

### Tests That Must Pass

1. The new derivation unit test in `world_step_coordinator.rs` passes: due-set correctness and stable order.
2. The derivation returns identical output across input-insertion-order permutations (determinism).
3. `cargo test -p tracewake-core --test world_step_coordinator` is green, and `cargo build --workspace --all-targets --locked` compiles with the unwired derivation present.

### Invariants

1. The derivation reads only replayable scheduling state and constructs no proposal/cognition (`INV-103`).
2. The derivation is a pure, deterministically-ordered function of its inputs (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — additive unit test of the eligibility derivation (due-set correctness, stable order, insertion-order invariance).

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`
