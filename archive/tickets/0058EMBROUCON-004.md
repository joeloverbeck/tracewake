# 0058EMBROUCON-004: Embodied/autonomous metamorphic no-fork proof

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a core integration test (metamorphic equivalence) + a shared test helper
**Deps**: 0058EMBROUCON-001

## Problem

Source-sharing the resolver is too weak (spec §3.2 H-0058-05, §4.4 F-0058-04 — the R2 no-fork requirement). The possessed path can still fork before it calls the shared resolver by passing a different `routine_window_family`. This ticket proves *behaviorally* that embodied follow-on resolution and the autonomous baseline produce the same proposal from equivalent actor-known state.

## Assumption Reassessment (2026-06-30)

1. `resolve_routine_step_follow_on` (`crates/tracewake-core/src/agent/routine_continuation.rs:20`) is called by `ActorDecisionTransaction::run` (`crates/tracewake-core/src/agent/transaction.rs:236`). The embodied path calls `ActorDecisionTransaction::run` at `session.rs:583` with `routine_window_family: embodied_routine_window_family(...)` (590). The autonomous production path (`scheduler.rs:3136`) passes `routine_window_family: routine_window_family(...)` (the scheduler helper at `crates/tracewake-core/src/scheduler.rs:3240`), which derives from `eligible_routine_execution_for_actor(window)` (3260).
2. Spec §4.4 F-0058-04 plus the in-session `/reassess-spec` finding I1: the autonomous baseline is `scheduler.rs::routine_window_family` (window-eligible-execution keyed, with the `WorkBlock`→`GoToWork` downgrade), **not** the active intention directly. The equivalence test compares the post--001 embodied follow-on against that baseline and is the binding proof the two derivations coincide; a divergence reconcilable only by changing the autonomous derivation defers to a follow-up scheduler spec (§2.2, §9.3).
3. Cross-artifact boundary under audit: embodied follow-on resolution (`session.rs`) versus autonomous `ActorDecisionTransaction` resolution (`transaction.rs` + the `scheduler.rs::routine_window_family` baseline), compared at semantic proposal-shape level plus hidden-truth audit.
4. Invariants under audit: INV-094 (possession parity is tested), INV-108 (possession is input-only / cognition-neutral), INV-099 (truth may validate but not plan), INV-101/102 (actor-known firewall), INV-104 (no transaction-pipeline bypass).
5. Enforcement surface: the metamorphic test exercises the actor-known firewall + resolver. It audits (does not modify) the hidden-truth audit on both paths; confirm the adversarial variant (hidden workplace known only in truth) asserts no hidden-truth leakage and the test introduces no nondeterminism (fixed actor-known state over deterministic `agent_state`).

## Architecture Check

1. A behavioral metamorphic equivalence property — not a shared-symbol assertion — is the only way to prove the possessed path cannot fork before the shared resolver; INV-094 requires parity to be *tested* behaviorally. Comparing semantic proposal shape + hidden-truth audit across adversarial states catches a fork that source-sharing alone would miss.
2. No backwards-compatibility aliasing/shims: the test asserts equivalence against the real autonomous baseline; it introduces no compatibility layer.

## Verification Layers

1. INV-094/108 (possession parity, cognition-neutral) → `embodied_continue_and_autonomous_transaction_match_from_equivalent_actor_known_state` (metamorphic equivalence over fixed actor-known state).
2. INV-099/101/102 (actor-known firewall) → the adversarial hidden-workplace variant asserts no hidden-truth selection (hidden-truth audit comparison).
3. INV-104 (no bypass) → the equivalence covers selected routine execution/current step plus proposal ancestry class.

## What to Change

### 1. Shared fixed-state helper + dual invocation

Add a core test helper building one fixed actor-known state and active intention, then invoking both: the autonomous baseline `ActorDecisionTransaction::run(...)` with `routine_window_family` from `scheduler.rs::routine_window_family` over the equivalent scheduled window, and the embodied follow-on resolution path after a valid `ContinueRoutineProposed` marker.

### 2. Semantic-shape comparison

Compare the selected proposal at semantic shape level: action id, actor id, target ids, relevant parameters, selected routine execution/current step, local plan id presence, proposal ancestry class, decision-trace outcome, and hidden-truth audit.

### 3. Adversarial variants

Cover: actor at a known workplace while active intention is eat/sleep; hidden workplace known only in truth; assigned-but-inactive routine execution exists; resolved routine execution exists; another actor's execution exists.

## Files to Touch

- `crates/tracewake-core/tests/embodied_autonomous_parity.rs` (new)
- `crates/tracewake-core/tests/support/mod.rs` (modify — shared fixed-state equivalence helper, as surfaced)

## Out of Scope

- The remediations themselves (-001/-002/-003) — this ticket proves equivalence over the remediated path.
- Changing the autonomous `scheduler.rs::routine_window_family` derivation (§2.2; a proven divergence routes to a follow-up scheduler spec per §9.3).

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_continue_and_autonomous_transaction_match_from_equivalent_actor_known_state` — autonomous baseline and embodied outputs match action id, target ids, parameters, selected execution/current step, local plan id presence, proposal ancestry class, decision-trace outcome, and hidden-truth audit.
2. Adversarial variant — actor at a known workplace while the active intention is eat/sleep → both resolve the eat/sleep step, not `WorkBlock`.
3. Adversarial variant — hidden workplace known only in truth → no hidden-truth selection on either path.
4. Adversarial variants — assigned-but-inactive / resolved / other-actor executions → no divergence between paths.
5. `cargo test --workspace` passes.

### Invariants

1. Embodied and autonomous resolution produce the same proposal from equivalent actor-known state (INV-094/108).
2. Neither path selects from hidden truth (INV-099/101/102).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/embodied_autonomous_parity.rs` (new) — metamorphic equivalence + adversarial variants.
2. `crates/tracewake-core/tests/support/mod.rs` (modify) — shared fixed-state builder helper (as surfaced).

### Commands

1. `cargo test -p tracewake-core --test embodied_autonomous_parity`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-30

- Added test-support-only actor-known surface builders on `NoHumanActorKnownSurfaceBuilder` for route, food, sleep, and workplace facts. These stay behind `#[cfg(any(test, feature = "test-support"))]` and keep context construction inside the existing no-human surface boundary.
- Added shared parity helpers in `crates/tracewake-core/tests/support/mod.rs` that build a fixed active-intention state, derive the embodied family from active intention/current method, derive the autonomous baseline with scheduler-style window eligibility, and compare transaction outputs at semantic proposal shape level.
- Added `crates/tracewake-core/tests/embodied_autonomous_parity.rs` with:
  - `embodied_continue_and_autonomous_transaction_match_from_equivalent_actor_known_state`
  - actor-at-known-workplace while active intention is eat, proving no workplace shortcut
  - hidden-workplace-absent variant, proving neither path selects hidden truth
  - inactive-future, resolved, and other-actor execution decoys, proving no divergence
- The comparison checks action id, actor id, target ids, routine template/execution parameters, local-plan presence, ancestry length, decision outcome, and hidden-truth audit.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core --test embodied_autonomous_parity` passed.
- `cargo test --workspace` passed.
