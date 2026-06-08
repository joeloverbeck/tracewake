# 0003PHA1SPIANT-009: Prove actor-known and debug-capability unforgeability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (public-API / compile-fail tests around `agent/actor_known.rs` and `debug_capability.rs`; adversarial debug-no-leak test)
**Deps**: None

## Problem

Actor-known facts and planning context have private fields and crate-private construction (`agent/actor_known.rs:185` `ActorKnownPlanningContext`, with crate-private construction), provenance gates actor-known status, and `DebugCapability` is private with a crate-private `mint` (`debug_capability.rs:21`). Hidden-truth tests already reject debug/unproven facts (`hidden_truth_gates.rs:99-309`). But there is no explicit public-API / compile-fail proof that external crates cannot construct a `DebugCapability` or an `ActorKnownPlanningContext` from raw truth, and no `From<PhysicalState>`-style privileged constructor guard. Spec `0003` §5.6 / SPINE-AC-009 require structural unforgeability with public-API / compile-fail tests.

## Assumption Reassessment (2026-06-08)

1. `ActorKnownPlanningContext` is at `crates/tracewake-core/src/agent/actor_known.rs:185` with private fields and crate-private construction (`:197-223`), provenance gating (`:142-162`, `:276-298`). `DebugCapability` is at `crates/tracewake-core/src/debug_capability.rs:16` with a private field and `pub(crate) const fn mint()` at `:21`. Hidden-truth gates are at `crates/tracewake-core/tests/hidden_truth_gates.rs:99-309`. Actor decision transactions require `&ActorKnownPlanningContext` (`agent/transaction.rs`).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-009 mandates: keep `ActorKnownPlanningContext` construction private/crate-private (external crates receive it only from sanctioned projection/view/model builders); keep `DebugCapability::mint` crate-private or narrower; add no `From<PhysicalState>`-style privileged actor-known constructor; add compile-fail / public-API tests proving external crates cannot construct debug capability or actor-known planning context from truth; a debug-panel test proving debug output does not change available actions, source context, or actor-known checksums.
3. Cross-artifact boundary under audit: the actor-known cognition firewall (`agent/actor_known.rs`, `agent/transaction.rs`) and the debug-truth capability (`debug_capability.rs`), and what the TUI/debug layer (a separate crate) can and cannot construct. This is the no-telepathy boundary between truth and actor-known cognition.
4. INV motivating this ticket: `INV-099`–`INV-102`/`INV-106` (actor-known cognition is provenance-gated; truth validates but does not plan), `INV-107` (debug is non-diegetic and holds no authority), `INV-108` (no bypass), reinforced by `INV-024` (no telepathy). Restated: hidden/validation/unproven truth must never enter actor planning, affordances, or embodied rendering, and the capability/constructor privacy is what makes that structural.
5. Actor-knowledge / no-leak surface touched directly: this is the truth-firewall enforcement surface. The compile-fail / public-API tests prove the firewall cannot be bypassed by an out-of-crate constructor; the debug-panel test proves debug rendering does not perturb actor-known checksums or the available-action set (no leakage from the debug surface into cognition). No determinism change; the tests read existing canonical checksums and action lists.

## Architecture Check

1. Public-API / compile-fail tests turn "external crates can't forge these" from a code-reading claim into an enforced property: a future `pub fn` exposing `DebugCapability` construction or an actor-known context from truth would make the test fail to compile or fail to pass. This is the object-capability discipline applied to the truth firewall.
2. No backwards-compatibility shim: no privileged `From<PhysicalState>` / public constructor is added for convenience; the only construction paths remain the sanctioned crate-private builders.

## Verification Layers

1. `INV-107` (debug holds no authority) -> compile/public-API check: external crates cannot construct `DebugCapability` (`mint` stays crate-private).
2. `INV-099`–`INV-102` (actor-known provenance) -> compile/public-API check: external crates cannot construct `ActorKnownPlanningContext` from raw truth; no privileged `From<PhysicalState>` exists.
3. `INV-024`/`INV-106` (no telepathy; debug non-leaking) -> adversarial test: rendering/opening a debug panel leaves the available-action set, source context, and actor-known checksums unchanged.

## What to Change

### 1. Public-API / compile-fail unforgeability tests

Add `actor_known_context_unforgeable_from_truth`: public-API / compile-fail tests (doc tests or a minimal harness inside the crate) proving external crates/tests cannot construct `ActorKnownPlanningContext` from raw truth/debug facts, and cannot construct/mint `DebugCapability`.

### 2. Guard against privileged constructors

Confirm (and lock with a test) that no `From<PhysicalState>`-style or public constructor exists for actor-known planning; keep `mint` crate-private or narrower.

### 3. Debug-no-leak adversarial test

Add `debug_panel_does_not_change_embodied_affordances`: assert that rendering/opening a debug panel leaves the semantic action list, proposal source context, and actor-known checksums unchanged.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (modify — public-API/compile-fail test scaffolding, no constructor relaxation)
- `crates/tracewake-core/src/debug_capability.rs` (modify — compile-fail doc test that external construction fails)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — `actor_known_context_unforgeable_from_truth`)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — `debug_panel_does_not_change_embodied_affordances`)

## Out of Scope

- Proposal source-context sealing (0003PHA1SPIANT-007).
- TUI proof-seam regression suite (0003PHA1SPIANT-012) — the debug-no-affordance test here is the core-level assertion; broader TUI re-verification is owned there.
- Any relaxation of existing privacy (this ticket only proves and locks current privacy).

## Acceptance Criteria

### Tests That Must Pass

1. `actor_known_context_unforgeable_from_truth` — external construction of `ActorKnownPlanningContext` from truth and of `DebugCapability` does not compile / is not reachable.
2. `debug_panel_does_not_change_embodied_affordances` — debug rendering leaves action list, source context, and actor-known checksums unchanged.
3. `cargo test --workspace` passes.

### Invariants

1. Hidden/validation/unproven truth never enters actor planning or rendering (`INV-099`–`INV-102`, `INV-024`).
2. `DebugCapability` and actor-known planning context are unforgeable from outside their authority layer (`INV-107`, `INV-108`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — `actor_known_context_unforgeable_from_truth`.
2. `crates/tracewake-tui/tests/adversarial_gates.rs` — `debug_panel_does_not_change_embodied_affordances`.
3. `crates/tracewake-core/src/debug_capability.rs` (doc test) — external `mint` does not compile.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo test -p tracewake-tui --test adversarial_gates`
3. `cargo test --workspace`
