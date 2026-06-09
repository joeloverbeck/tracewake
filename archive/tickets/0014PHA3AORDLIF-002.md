# 0014PHA3AORDLIF-002: Sealed transaction proposal — scheduler cannot mutate after cognition

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/transaction.rs` output type reseal, `scheduler.rs` no-human path), new source guard, 1 adversarial fixture
**Deps**: 0014PHA3AORDLIF-001

## Problem

After `ActorDecisionTransaction::run` returns, `scheduler.rs::build_agent_proposal` takes the returned proposal and, when the action is `wait`, mutates it: `proposal.parameters.insert("reason", format!("no_human_day:{}", window.window_id))` (`crates/tracewake-core/src/scheduler.rs:639-642`). The actor decision transaction owns proposal construction and trace ancestry; the scheduler may order/call/enqueue and record metrics, but it must not author or rewrite actor-visible primitive proposal parameters after the actor's transaction. This injects a scheduler-authored, non-actor-known window id into an actor-visible field — the ORD-HARD-002 defect.

## Assumption Reassessment (2026-06-09)

1. The mutation is at `crates/tracewake-core/src/scheduler.rs:639-642`, inside the `ActorDecisionTransactionOutcome::Proposed(proposed)` arm of `build_agent_proposal`. The transaction constructs and returns its `Proposal` in `crates/tracewake-core/src/agent/transaction.rs:168-211` (including the actor-visible `wait` `reason` at `transaction.rs:181-187`). A second, unrelated `proposal.parameters.insert` exists at `scheduler.rs:2161` (not a post-transaction mutation) and is out of scope.
2. Spec §ORD-HARD-002 and §5.2 require transaction output proposals to be immutable to the scheduler (`SealedProposal` / `TransactionProposal` with private fields + read-only accessors), and any no-human window identity to live as scheduler/debug metadata on the no-human report or ordering key — not in actor-visible proposal parameters.
3. Shared boundary under audit: the transaction→scheduler hand-off. The contract: the transaction's `SealedProposal` is the actor-visible authority; the scheduler may attach only explicitly non-cognitive metadata through a separate wrapper, never edit the sealed proposal's `parameters` / `target_ids` / `action_id`.
4. Invariants motivating this ticket: **INV-103** (scheduler not a cognition authority), **INV-104** (routines/needs do not dispatch primitives directly — proposal authorship stays in the transaction), **INV-105** (decision traces authoritative; proposal ancestry coherent with the trace), **INV-099** (truth/scheduler may validate/order, not plan).
5. Actor-knowledge / deterministic-replay enforcement surface: the actor-visible `wait` reason is part of the committed event and the decision trace. Moving the window id to non-actor-visible metadata must keep replay byte-identical (the window id may still appear in the no-human report / ordering key, which are deterministic) and must not change the committed event's actor-visible payload semantics (INV-018). The regression test compares the trace's wait reason to the committed wait event and asserts no `no_human_day:` window id appears in actor-visible proposal parameters.
6. Schema reshape (reseal) — additive-vs-breaking: `ActorDecisionTransactionOutcome::Proposed` currently carries a mutable `Proposal`; this reseals it to a `SealedProposal` exposing read-only accessors. Consumers of the transaction outcome: `scheduler.rs::build_agent_proposal` (the only no-human consumer) and the transaction's own unit tests (`transaction.rs:451-500`). Breaking at the mutation site by design; all consumers move in this diff (local compile-atomicity).
7. Removal blast radius of the post-transaction mutation surface: grep `proposal.parameters.insert`, `proposal.target_ids.push`, `proposal.action_id =`, and actor-id mutation in `scheduler.rs` after the transaction return. Confirmed sites: `scheduler.rs:639` (this ticket removes it). The guard bans reintroduction.

## Architecture Check

1. A `SealedProposal` with private fields + accessor-only reads makes post-cognition mutation a compile error rather than a review catch — the scheduler structurally cannot rewrite an actor-visible primitive. Window identity moves to a clearly non-diegetic metadata channel, keeping the actor-visible/observer split explicit.
2. No backwards-compatibility shim: the `parameters.insert` mutation is deleted, not wrapped. The window id is relocated to non-cognitive metadata, not duplicated.

## Verification Layers

1. INV-103 / INV-104 (scheduler authors no actor-visible primitive) -> codebase grep-proof: source guard in `anti_regression_guards.rs` fails on `proposal.parameters.insert` / `proposal.target_ids.push` / `proposal.action_id =` / actor-id mutation in `scheduler.rs` after the transaction return.
2. INV-105 (coherent proposal ancestry) -> replay/golden-fixture check: `scheduler_cannot_rewrite_wait_reason_after_transaction_001` asserts the committed wait reason equals the transaction/local-plan reason and contains no `no_human_day:` window id.
3. INV-018 (deterministic replay) -> replay/golden-fixture check: the no-human capstone replay byte-matches after the reseal and window-id relocation.

## What to Change

### 1. Reseal transaction output

Introduce `SealedProposal` (private fields, read-only accessors) as the type carried by `ActorDecisionTransactionOutcome::Proposed`. The transaction builds and seals the proposal (including the actor-visible `wait` reason from the local plan, `transaction.rs:181-187`); the scheduler can only read it.

### 2. Relocate the no-human window id

Remove the `scheduler.rs:639-642` `parameters.insert`. If the no-human window identity is needed downstream, record it on the no-human report / scheduler ordering key as non-cognitive metadata, never in the sealed proposal's actor-visible parameters.

### 3. Source guard + fixture

Add the mutation-ban guard (Verification Layer 1) and the adversarial fixture (Test Plan).

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify — seal the output proposal; **also touched by 004/005, coordinate**)
- `crates/tracewake-core/src/scheduler.rs` (modify — remove post-transaction mutation; relocate window id; **shared with 0014PHA3AORDLIF-001 in `build_agent_proposal`, land 001 first**)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #2; **N-way shared hub**)
- `crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- The actor-known surface construction that precedes the transaction (ticket 0014PHA3AORDLIF-001).
- The internal method-fallback coherence of the transaction (ticket 0014PHA3AORDLIF-004).
- Typed diagnostic fields on events (ticket 0014PHA3AORDLIF-003).

## Acceptance Criteria

### Tests That Must Pass

1. `scheduler_cannot_rewrite_wait_reason_after_transaction_001` — the committed wait event's reason matches the transaction/local-plan reason; no `no_human_day:<window>` appears in actor-visible proposal parameters.
2. `cargo test -p tracewake-core --test anti_regression_guards` — mutation-ban guard passes.
3. `cargo test -p tracewake-core --test no_human_capstone` — capstone + replay byte-match still pass.

### Invariants

1. The scheduler performs no mutation of a sealed proposal's `parameters` / `target_ids` / `action_id` after the transaction returns (INV-103/104).
2. Replay reproduces the committed wait event byte-identically; window id lives only in non-actor-visible metadata (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs` — proves the wait reason is transaction-authored, not scheduler-rewritten.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning post-transaction proposal mutation in `scheduler.rs`.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`

## Outcome

Completed on 2026-06-09.

- Added `SealedProposal` as the actor decision transaction handoff type, with private proposal storage and read-only accessors.
- Removed the no-human scheduler rewrite of actor-visible wait reasons after transaction return.
- Added a scheduler regression that commits a wait event and proves the reason remains transaction-authored (`actor_decision_reevaluation`) with no `no_human_day:<window>` actor-visible payload.
- Added a source guard banning post-transaction scheduler mutation of proposal parameters, targets, action id, or actor id.
- Added and registered `scheduler_cannot_rewrite_wait_reason_after_transaction_001`.
- Updated stale acceptance expectations that previously required the forbidden scheduler-authored wait reason.

Verification run:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core scheduler_cannot_rewrite_wait_reason_after_transaction`
3. `cargo test -p tracewake-core --test anti_regression_guards`
4. `cargo test -p tracewake-content`
5. `cargo test -p tracewake-core --test no_human_capstone`
6. `cargo test -p tracewake-core --lib transaction_links_candidate_intention_method_plan_and_proposal`
7. `cargo test -p tracewake-core --test acceptance_gates integrated_no_human_day_capstone_emerges_from_one_autonomous_run`
8. `cargo test --workspace`
9. `cargo clippy --workspace --all-targets -- -D warnings`
10. `cargo build --workspace --all-targets --locked`
