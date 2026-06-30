# 0057EMBROUCON-001: Shared actor-known routine-step resolver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` `agent` module: a new shared routine-step→ordinary-action resolver consumed by both the autonomous decision transaction and (in 0057EMBROUCON-002) the embodied submission path
**Deps**: None

## Problem

In embodied (possessed) play the TUI presents `Continue routine`, but submitting it produces no behavioral progress: the marker (`ContinueRoutineProposed`) commits and nothing commits the routine step's follow-on ordinary action (move toward workplace, then `work_block`). The autonomous no-human path *does* make that progress because `ActorDecisionTransaction::run` (`crates/tracewake-core/src/agent/transaction.rs:90`) resolves the active intention's current routine step into a concrete `Proposal` (action id + actor-known target ids) and commits it. That resolution is currently locked inside the autonomous decision transaction.

Per spec 0057 §4.1, the embodied path must commit the **same** follow-on the autonomous planner would — using an actor-known resolution **identical** in admissibility to the autonomous one, not a TUI-local shortcut and not a parallel copy (§9 R2). This ticket factors that routine-step resolution into a callable resolver so 0057EMBROUCON-002 can invoke it for an embodied submission against a named intention's `current_step`, while the autonomous path keeps using the very same code.

## Assumption Reassessment (2026-06-30)

1. Autonomous resolution surface verified: `crates/tracewake-core/src/agent/transaction.rs:90` `ActorDecisionTransaction::run(...) -> ActorDecisionTransactionOutcome`, returning `Proposed(Box<ActorDecisionProposalOutcome>)` (carrying a `SealedProposal`, `transaction.rs:37/57`) or `Stuck { diagnostic: Box<StuckDiagnosticRecord> }` (`transaction.rs:29-33`). The routine step itself is `RoutineStep::MoveTowardPlace { action_id }` / `WaitUntil { reason }` (`crates/tracewake-core/src/agent/routine.rs:44/52`), surfaced through `RoutineStep::proposed() -> RoutineStepProposal` (`routine.rs:59`); `crates/tracewake-core/src/agent/planner.rs:483` builds `RoutineStep::MoveTowardPlace` under `ActorKnownPlanningContext` (`planner.rs:7/25`). The step→`Proposal`(+target) resolution exists but is not independently callable.
2. Spec assumption: `specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md` §4.1 governs this ticket; §9 R2 ("resolver divergence") mandates a unit test pinning identical output for a fixed actor-known state. §1.2 forbids changing the autonomous planner's *behavior* — this is a pure extraction (same resolution, now shared), not a planner rewrite.
3. **Cross-artifact boundary under audit**: the shared resolver is the new boundary between the autonomous decision transaction (`agent/transaction.rs`) and the embodied submission path (0057EMBROUCON-002, which consumes it from `runtime/session.rs`). Its signature — `(sealed actor-known planning context, the named active intention, its `current_step`) -> typed result` — is the contract both callers must share; named here before implementation so 002 binds to a stable surface.
4. INV-099 (truth may validate actions but may not plan them), INV-101 (action proposal generation must consume a sealed actor-known context, never validator-only truth), INV-104 (routines/needs/intentions may not bypass candidate generation, actor-known context, proposal construction, shared validation, and event commitment). The resolver must resolve the follow-on *only* from actor-known context, returning a `Proposal` that still flows through shared validation — never a truth-driven or directly-dispatched action.
5. **Actor-knowledge / determinism enforcement surface**: the resolver consumes the same sealed actor-known planning context (`ActorKnownPlanningContext`, `agent/planner.rs`) the autonomous path already uses — not `PhysicalState` ground truth. Confirm: (a) no hidden-truth field selects the move destination or workplace (INV-099/100 firewall); (b) for a fixed actor-known context + content version the resolver is a pure function (same inputs → byte-identical `Proposal`), preserving deterministic replay (INV-018). The resolver *selects*; truth still *validates* downstream in the pipeline.

## Architecture Check

1. A single resolver shared by both the autonomous transaction and the embodied path makes possession parity (INV-094) structural rather than test-asserted, and confines the truth firewall (INV-099) to one auditable surface — strictly better than a TUI-local shortcut (which would put planning in presentation, violating INV-008/069) or a copied resolution (which would drift, the §9 R2 hazard). The autonomous path is refactored to *call* the extracted resolver, so "same resolution" is guaranteed by construction, not by review.
2. No backwards-compatibility aliasing or shims: `ActorDecisionTransaction::run`'s step-resolution is moved into the shared resolver and called from there — the old inline path is removed, not wrapped behind a deprecated alias. `tracewake-core` stays dependency-free.

## Verification Layers

1. INV-099 / INV-101 (actor-known only) -> unit test (`agent::routine_continuation` tests): for a fixed sealed actor-known context the resolver's resolved target ids are all present in that context; a context with a hidden-only target yields a typed knowledge blocker, not a truth-driven target.
2. INV-094 / §9 R2 (no divergence) -> identical-output unit test: the autonomous decision transaction and the shared resolver produce the same `Proposal` (action id + target ids + parameters) for one fixed actor-known state.
3. INV-104 (no direct dispatch) -> codebase grep-proof: the resolver returns a `Proposal` (not an applied event); commitment remains the pipeline's job (asserted structurally — the resolver has no `EventLog`/apply access).

## What to Change

### 1. Extract the shared routine-step resolver

Add `crates/tracewake-core/src/agent/routine_continuation.rs` exposing a resolver that, given the sealed actor-known planning context, a named active intention, and its `current_step`, returns a typed result: a resolved ordinary `Proposal` (action id + actor-known target ids), a typed blocker (precondition / route / workplace / need / terminal — reusing the existing `StuckDiagnosticRecord` / blocker vocabulary), or a modeled-wait directive (mirroring `RoutineStepProposal::Wait`). Declare the module in `crates/tracewake-core/src/agent/mod.rs`.

### 2. Route the autonomous path through the resolver

Refactor `ActorDecisionTransaction::run` (`agent/transaction.rs`) so its routine-step→`Proposal` resolution delegates to the new resolver instead of resolving inline — the autonomous `Proposed`/`Stuck` outcomes are now produced from the shared resolver's typed result. No change to goal selection, planner budget, or autonomous cadence.

### 3. Identical-resolution unit proof

Add the §9 R2 unit test pinning that autonomous and shared-resolver resolution are byte-identical for a fixed actor-known state, plus the actor-known-only firewall test from Verification Layer 1.

## Files to Touch

- `crates/tracewake-core/src/agent/routine_continuation.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — declare the module / re-export the resolver)
- `crates/tracewake-core/src/agent/transaction.rs` (modify — delegate step resolution to the shared resolver; no behavioral change)

## Out of Scope

- The embodied follow-on commit + receipt surfacing (0057EMBROUCON-002 consumes this resolver).
- Any change to autonomous planner behavior, scheduler cadence, or `advance_until` (spec §1.2).
- Stuck-eligibility wiring for repeated embodied continuations (0057EMBROUCON-003).
- The `continue_routine` marker definition (unchanged; 0057EMBROUCON-004 guards it).

## Acceptance Criteria

### Tests That Must Pass

1. Identical-resolution unit test: for a fixed actor-known state, the autonomous decision transaction and the shared resolver yield the same `Proposal` (action id, target ids, parameters) — proves no divergence (§9 R2).
2. Actor-known-firewall unit test: a target known only in hidden truth (absent from the sealed actor-known context) yields a typed knowledge blocker from the resolver, never a resolved move to that target (INV-099).
3. `cargo test --locked -p tracewake-core` — full core suite green, including the unchanged autonomous golden/no-human resolution paths (proves the extraction is behavior-preserving).

### Invariants

1. The follow-on action and its target are derived solely from the sealed actor-known planning context; no `PhysicalState` ground-truth field selects them.
2. The autonomous decision transaction and the embodied path resolve through one shared function — no parallel copy exists.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/routine_continuation.rs` (unit tests in-module) — identical-resolution proof + actor-known-only firewall proof.
2. `crates/tracewake-core/src/agent/transaction.rs` — existing autonomous-resolution tests must remain green unchanged (regression guard that the extraction preserved behavior).

### Commands

1. `cargo test --locked -p tracewake-core routine_continuation` — the new resolver unit proofs.
2. `cargo test --locked -p tracewake-core` — full core suite, including unchanged autonomous resolution.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked` — four-gate hygiene.

## Outcome

Completed: 2026-06-30

Implemented the shared actor-known routine-step follow-on resolver in
`crates/tracewake-core/src/agent/routine_continuation.rs` and exported it from
`agent/mod.rs`. `ActorDecisionTransaction::run` now delegates routine-step
proposal construction to that resolver, while retaining the existing
transaction-level stuck-diagnostic mapping and sealed proposal handoff shape.

The resolver owns the previously inline actor-known target selection,
local-plan request construction, sleep-affordance parameter shaping, selected
goal bundle construction, and sealed proposal parameter population. The
autonomous path and future embodied path now share one callable resolution
surface rather than a transaction-local copy.

Added resolver unit coverage proving:

- the shared resolver produces byte-identical proposal action, targets,
  parameters, local-plan first proposal, and selected-goal bundle to the real
  autonomous `ActorDecisionTransaction::run` for a fixed actor-known state;
- actor-known go-to-work resolution yields the expected `move` target when the
  workplace and route are actor-known;
- a hidden-only route fails closed with a `BlockerCategory::Knowledge` local-plan
  failure instead of resolving a truth-driven target.

Updated the anti-regression guard source census and source scans so the guarded
layer recognizes `routine_continuation.rs` and continues checking the moved
sleep-affordance and selected-goal-bundle proposal construction invariants in
their new home.

Verification run:

- `cargo test --locked -p tracewake-core routine_continuation` — passed.
- `cargo test --locked -p tracewake-core` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.

Deviations: none. The ticket's actor-known firewall proof was implemented as a
missing actor-known route knowledge blocker for a known workplace, which is the
current resolver/planner failure surface for preventing truth-driven movement to
the hidden route target.
