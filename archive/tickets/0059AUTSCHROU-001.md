# 0059AUTSCHROU-001: Bind autonomous routine family to the active-intention chain

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — rewrites the autonomous routine-family selector in `crates/tracewake-core/src/scheduler.rs`; no new events, schemas, fixtures, or crates
**Deps**: None

## Problem

At the current baseline the no-human autonomous decision path derives an actor's routine family from a clock/window-keyed selector, not from the actor's active intention. `run_no_human_actor_decision_transaction` (`crates/tracewake-core/src/scheduler.rs:3136`) passes `routine_window_family: routine_window_family(...)` into `ActorDecisionTransactionInput`, and `routine_window_family` (`scheduler.rs:3240`) calls `eligible_routine_execution_for_actor` (`scheduler.rs:3260`), which selects a routine execution by `.min_by(start_tick, execution_id)` over window eligibility (`scheduler.rs:3284-3288`) and never consults `agent_state.active_intention_by_actor`. A scheduler/clock fact thereby chooses a routine family ahead of cognition — the violation 0059 §F-0059-01 targets. The 0058-fixed embodied path `embodied_routine_window_family` (`crates/tracewake-core/src/runtime/session.rs:1136`) already derives family through the active intention and is the reference pattern to mirror; this ticket brings the autonomous path to parity.

## Assumption Reassessment (2026-07-01)

1. The window-keyed selector is live, not hypothetical: `scheduler.rs:3240` `fn routine_window_family` and `scheduler.rs:3260` `fn eligible_routine_execution_for_actor` exist (verified present at the spec's own baseline commit `dffeefa` and at HEAD; `scheduler.rs` unchanged between). `eligible_routine_execution_for_actor` ranks by `.min_by(start_tick, execution_id)` (`scheduler.rs:3284-3288`); `active_intention_for_actor` (`scheduler.rs:3291`) reads `agent_state.active_intention_by_actor` and is the authoritative chain the selector must use instead.
2. Spec `archive/specs/0059_AUTONOMOUS_SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md` §3.1/§4 (verdict corrected to fix-plus-lock during the in-session `/reassess-spec` pass) and §F-0059-01 define the required helper contract: derive family only from the single active intention's selected routine method and unresolved current step/execution, returning a typed non-routine outcome when the chain is absent/resolved/ambiguous/malformed/foreign, and never ranking by `start_tick`, `deadline`, `execution_id`, or world truth.
3. Shared boundary under audit: the `ActorDecisionTransactionInput::routine_window_family: Option<RoutineFamily>` seam (`crates/tracewake-core/src/agent/transaction.rs:24`) between the scheduler (producer) and the decision transaction (consumer). This ticket changes only what the scheduler *produces* into that seam; how the transaction *treats* it is hardened in 0059AUTSCHROU-002.
4. Motivating invariants restated: **INV-103 — The scheduler is not a cognition authority** ("It must not construct action proposals from … routine labels, … true workplace locations") and **INV-112 — Time may validate, but holder-known time must plan** ("The scheduler and replay clock may order and validate; they must not become cognition authority"). A clock/window-keyed family selector is exactly the forbidden construction.
5. Enforcement surface touched: the autonomous cognition-input boundary (truth firewall, INV-100/INV-103). The change *removes* a hidden-clock cognition path; it introduces no new actor-knowledge input and no nondeterminism (the active-intention lookup is a deterministic `BTreeMap` read, same as the embodied path). Deterministic replay is preserved — the derivation becomes a pure function of `agent_state` with no new wall-clock or iteration-order dependency.
6. Blast radius of changing `eligible_routine_execution_for_actor`: it has **two** consumers — `routine_window_family` (`scheduler.rs:3246`, the selector being fixed) and `active_routine_execution_for_actor` (`scheduler.rs:4197`, called at `scheduler.rs:4131` for routine-execution progress/failure bookkeeping, a legitimate scheduler ordering/validation use under INV-112). The fix must NOT break `active_routine_execution_for_actor`: either leave `eligible_routine_execution_for_actor` in place for that bookkeeping consumer and stop using it for *family selection*, or split the window-eligibility lookup (bookkeeping, retained) from family derivation (rebound to the active intention). The selection-vs-bookkeeping split is the cleaner end state.
7. Adjacent contradiction classified: the embodied path (`embodied_routine_window_family`, `runtime/session.rs:1136`) and the autonomous path diverged because 0058 fixed only the embodied selector. This is a **separate, already-resolved sibling** (0058), not a defect introduced here; the embodied path is out of scope to modify and serves only as the reference pattern (its tests `embodied_routine_window_family_requires_active_intention_before_workplace_context` etc. encode the target behavior).
8. Mismatch + correction: the spec originally carried a "lock-only / no violation observed" verdict; the in-session `/reassess-spec` pass corrected §3.1/§4 to **fix-plus-lock** after proving the named selector present at the spec's own baseline. This ticket implements that corrected verdict's fix.

## Architecture Check

1. Binding routine family through the single active intention (mirroring the landed `embodied_routine_window_family`) gives the autonomous and embodied paths one authority model instead of two divergent ones, and removes the only remaining scheduler-owned cognition shortcut for routine family. Ranking executions by `start_tick`/`execution_id` is replaced by "the family the actor is already committed to," which is both doctrinally required (INV-103/112) and simpler to reason about under replay.
2. No backwards-compatibility aliasing/shim: the window-keyed family-selection path is removed outright, not wrapped behind a fallback. `eligible_routine_execution_for_actor` is retained only where it serves the non-cognition bookkeeping consumer (`active_routine_execution_for_actor`); it is not kept as a dormant family selector.

## Verification Layers

1. INV-103 (scheduler not cognition authority) -> codebase grep-proof: no `min_by`/`start_tick`/`execution_id`-keyed family selection remains on the autonomous `ActorDecisionTransactionInput::routine_window_family` producer path (`scheduler.rs`), and the family is derived via `active_intention_for_actor` / `active_intention_by_actor`.
2. INV-112 (time orders/validates, does not plan) -> replay/golden-fixture check: the no-human golden-fixture day runs (`crates/tracewake-content/tests/golden_fixtures_run.rs`) still advance deterministically with the rebound derivation.
3. INV-104 (routines/needs do not dispatch directly) -> manual review + grep-proof: the rebound helper produces a routine family only as a defeasible input to candidate generation, not a direct dispatch; the `active_routine_execution_for_actor` bookkeeping consumer remains intact (`scheduler.rs:4131`).

## What to Change

### 1. Rebind the autonomous routine-family producer to the active intention

In `crates/tracewake-core/src/scheduler.rs`, change `routine_window_family` (and its call site at `scheduler.rs:3143` inside `run_no_human_actor_decision_transaction`) so the returned `Option<RoutineFamily>` is derived from the actor's single active intention: look up `active_intention_by_actor[actor_id]`, fetch the intention record, confirm it is a routine-continuation intention with a selected method and an unresolved current step/execution, and derive the family from that bound method/step. Return `None` (a typed non-routine outcome handled in 0059AUTSCHROU-002) when the chain is absent, resolved, ambiguous, malformed, or actor-mismatched. Mirror the structure of `embodied_routine_window_family` (`runtime/session.rs:1136`), including its `WorkBlock → GoToWork` refinement when the actor is not at a known workplace, so embodied/autonomous parity holds.

### 2. Stop ranking routine executions by clock/window facts for family selection

Remove the `.min_by(start_tick, execution_id)` family-selection use of `eligible_routine_execution_for_actor` from the `routine_window_family` path. Preserve `eligible_routine_execution_for_actor`'s window-eligibility lookup only for the non-cognition bookkeeping consumer `active_routine_execution_for_actor` (`scheduler.rs:4197`, used at `scheduler.rs:4131`); do not delete that consumer or change its behavior. Prefer splitting window-eligibility bookkeeping (retained) from family derivation (rebound) so the two concerns no longer share a clock-keyed selector.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)

## Out of Scope

- The decision transaction's treatment of `ActorDecisionTransactionInput::routine_window_family` (conflict / no-active-intention fail-closed handling) — that is 0059AUTSCHROU-002.
- The embodied path `embodied_routine_window_family` (`runtime/session.rs`) — already fixed and locked by 0058.
- The severe-need-interrupt path, planner/no-human cadence, the 0047 world-tick coordinator, and scheduler-owned time-advancing follow-on routing (0059 §2.2).
- New metamorphic tests (003), anti-regression guards (004), focused mutation (005), and the acceptance artifact (006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test embodied_autonomous_parity` — embodied/autonomous parity holds with the rebound autonomous derivation.
2. `cargo test -p tracewake-content --test golden_fixtures_run` — no-human golden-fixture day runs still advance and replay deterministically.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the four local gates pass.

### Invariants

1. The autonomous `routine_window_family` producer derives family solely from `active_intention_by_actor` / the active intention record; no `start_tick`/`deadline`/`execution_id`/world-truth fact selects a routine family (INV-103, INV-112).
2. `active_routine_execution_for_actor` (`scheduler.rs:4197`) and its consumer (`scheduler.rs:4131`) retain their pre-change behavior; the window-eligibility bookkeeping use is not removed (no regression to routine progress/failure handling).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (in-module unit tests) — update/extend the existing `routine_window_family_*` unit tests to assert family now comes from the active intention and that the absent/resolved/foreign-chain cases yield `None`; rationale: lock the rebound contract at the unit level. The full A1–A10 metamorphic suite is 0059AUTSCHROU-003.

### Commands

1. `cargo test -p tracewake-core --test embodied_autonomous_parity` — narrowest behavioral boundary for the parity property this change preserves.
2. `cargo test -p tracewake-core scheduler::` — exercises the in-module scheduler unit tests including the `routine_window_family_*` cases.
3. `cargo test --workspace` — full-pipeline confirmation that no consumer of the rebound derivation regressed.

## Outcome

Completed: 2026-07-01

The autonomous no-human `routine_window_family` producer now derives routine family through the actor's active intention and selected routine method instead of the clock/window-keyed `eligible_routine_execution_for_actor` selector. Matching routine executions are still checked for unresolved and due-within-window status as validation gates, but `start_tick`, `deadline_tick`, and execution ordering no longer choose among families. If a matching execution exists but is resolved or not due, the producer returns `None` rather than resurrecting a family through template fallback. The retained `eligible_routine_execution_for_actor` path remains available for `active_routine_execution_for_actor` bookkeeping.

The scheduler unit tests now cover the active-intention requirement, resolved matching execution rejection, foreign execution ignoring, and deadline exclusion. No-human capstone, acceptance, and TUI fixtures were updated to seed authoritative active routine intentions, and the TUI post-run golden/assertion now reflects the real Commons/wait panel while preserving the debug metrics for the failed work attempt.

Verification run:
- `cargo test -p tracewake-core --test doc_invariant_references`
- `cargo test -p tracewake-core scheduler::`
- `cargo test -p tracewake-core --test embodied_autonomous_parity`
- `cargo test -p tracewake-content --test golden_fixtures_run`
- `cargo test -p tracewake-core --test acceptance_gates integrated_no_human_day_capstone_emerges_from_one_autonomous_run`
- `cargo test -p tracewake-core --test no_human_capstone`
- `cargo test -p tracewake-tui --test command_loop_session no_human_day_command_loop_renders_phase3a_behavior_rows`
- `cargo test -p tracewake-tui --test playable_capability_parity playable_capability_scenarios_match_checked_in_real_pipeline_goldens`
- `cargo test -p tracewake-tui --test tui_acceptance tui_runs_no_human_day_and_inspects_real_post_run_panels`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
