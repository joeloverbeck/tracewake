# 0014PHA3AORDLIF-004: Sealed goal bundle — no silent method fallback (coherent re-run)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/transaction.rs` selection/fallback flow + `SelectedGoalBundle`), new source guard, 1 adversarial fixture
**Deps**: 0014PHA3AORDLIF-002, 0014PHA3AORDLIF-003

## Problem

In `ActorDecisionTransaction::run`, when method selection fails for the selected (top-priority) candidate, the code scans lower-priority candidates and picks the first with a workable method *without re-running the decision-trace/lifecycle sequence* (`crates/tracewake-core/src/agent/transaction.rs:86-124`). Proposal construction then inserts `decision_trace_id` from the original `selection.trace` (`transaction.rs:198-200`) while inserting `candidate_goal_id` from `method_goal` — which may be the fallback (`transaction.rs:202-204`). The committed proposal therefore cites a trace id for one goal and a candidate id for another: behavior is plausible but the trace is false. This is ORD-HARD-003.

**Resolution of the spec's two options (Q2, delegated to doctrine):** the spec offers (1) fail-closed-on-first-miss or (2) coherent re-run. The doc pack selects **option 2**: arch `docs/1-architecture/05_…NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md:119` mandates fail-closed only when *no* valid candidate/method/plan exists; `05:109` ("a selected method emits trace evidence; rejected methods emit typed rejection reasons") and exec `docs/2-execution/06_…NO_HUMAN_PROOF.md:104` ("selected **and rejected** candidates/methods visible") expect alternatives to be tried and recorded; the reference `docs/3-reference/01_DESIGN_RISK_REGISTER.md:229-230` flags "failed actions produce no fallback" as a failure mode and requires "fallback actions." So a fallback candidate is tried as a **fresh, coherent transaction emitting its own decision trace** (with the rejected top candidate carrying a typed rejection reason), bounded by planner budget; only when **no** candidate yields a valid method/plan does it fail closed with a typed stuck diagnostic naming `method_selection`.

## Assumption Reassessment (2026-06-09)

1. The silent fallback scan is `candidate_fallbacks.iter().filter(...).find_map(|candidate| select_phase3a_method(...).ok()...)` at `crates/tracewake-core/src/agent/transaction.rs:94-108`; the desync is `decision_trace_id` from `selection.trace.trace_id` (`transaction.rs:198-200`) vs `candidate_goal_id` from `method_goal` (`transaction.rs:202-204`). `select_phase3a_method` is imported at `transaction.rs:4`. The stuck path uses `stuck_diagnostic(...)` (`transaction.rs:110-119`).
2. Spec §ORD-HARD-003 requires a `SelectedGoalBundle` carrying `candidate_goal_id`, `decision_trace_id`, `intention_transition_id`, `selected_method_id`, `local_plan_id`, and proposal ancestry, with proposal construction accepting only that bundle; and a source guard banning the fallback scan after trace selection without a new trace. Doctrine (above) selects the re-run variant; fail-closed is its terminal branch.
3. Shared boundary under audit: the intra-transaction bundle binding selected candidate → lifecycle transition → method → local plan → proposal → decision trace into one coherent unit. The contract: every committed proposal's `decision_trace_id` and `candidate_goal_id` come from the **same** bundle.
4. Invariants motivating this ticket: **INV-105** (decision traces authoritative only as a coherent bundle), **INV-034** (durable intentions not silently overwritten), **INV-036** (HTN methods have typed alternatives/rejection, not silent swaps), and arch `05:119` (fail closed only when nothing valid). Bounded re-selection respects **INV-037/INV-038** (planning is bounded/budgeted).
5. Deterministic-replay / trace-coherence enforcement surface: each re-run emits a distinct decision trace; replay must reconstruct the same sequence of traces (selected + typed-rejected) deterministically. The stuck terminal names `responsible_layer = method_selection` using the typed `ResponsibleLayer` enum from ticket -003. No hidden truth enters fallback selection (candidates come from actor-known context only). Must stay within planner budget so it does not become a per-tick replan storm (risk register `01:229`).
6. Removal blast radius: the silent `find_map` fallback scan at `transaction.rs:94-108` is removed and replaced by coherent re-selection; grep `transaction.rs` confirms this is the only post-trace candidate-swap site. The guard bans its reintroduction.

## Architecture Check

1. A `SelectedGoalBundle` that proposal construction accepts as its *only* input makes a mixed trace/candidate proposal unrepresentable — the trace id and candidate id are read from one object, so they cannot disagree. Re-run rather than fail-closed-on-first-miss preserves agent competence (INV-040) while keeping every trace coherent.
2. No backwards-compatibility shim: the silent fallback scan is deleted, not retained behind a flag. Re-selection produces new traces through the same transaction path; no parallel "quick fallback" path survives.

## Verification Layers

1. INV-105 (coherent bundle) -> codebase grep-proof: source guard in `anti_regression_guards.rs` bans `candidate_fallbacks.iter()...find_map(...select_phase3a_method...)` (post-trace candidate swap without a new trace) in `transaction.rs`.
2. INV-105 / INV-034 (no mixed trace; competent re-run) -> replay/golden-fixture check: `method_fallback_requires_new_trace_or_stuck_001` — top candidate has no method, a lower candidate does; the test proves either a new coherent trace for the fallback goal OR a typed stuck diagnostic naming `method_selection`, never a mixed trace/proposal.
3. INV-037/038 (bounded) -> manual review + unit test: re-selection is bounded by candidate count / planner budget; no unbounded loop.

## What to Change

### 1. SelectedGoalBundle

In `crates/tracewake-core/src/agent/transaction.rs`, introduce `SelectedGoalBundle { candidate_goal_id, decision_trace_id, intention_transition_id, selected_method_id, local_plan_id, proposal_ancestry }`. Proposal construction (the `transaction.rs:168-211` block) accepts only the bundle; `decision_trace_id` and `candidate_goal_id` are read from it.

### 2. Coherent re-run on method failure

Replace the `find_map` silent scan: on selected-method failure, emit a typed rejection for that candidate and re-enter candidate-selection/lifecycle/method/local-planning for the next candidate, producing a new `SelectedGoalBundle` (and decision trace) for the fallback goal. Bound the re-selection by candidate count / planner budget.

### 3. Fail-closed terminal

When no candidate yields a valid method/plan, return a typed stuck diagnostic with `responsible_layer = method_selection` (typed field from ticket -003) and no proposal.

### 4. Source guard + fixture

Add the fallback-scan-ban guard and the adversarial fixture.

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify — `SelectedGoalBundle`, re-run flow; **shared with 002/003/005, land 002 & 003 first**)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #3; **N-way shared hub**)
- `crates/tracewake-content/src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- Adding the typed `responsible_layer` field itself (ticket 0014PHA3AORDLIF-003 — this ticket consumes it).
- Sealing the output proposal against scheduler mutation (ticket 0014PHA3AORDLIF-002 — this ticket consumes `SealedProposal`).
- The actor-known input provenance refs (ticket 0014PHA3AORDLIF-005).

## Acceptance Criteria

### Tests That Must Pass

1. `method_fallback_requires_new_trace_or_stuck_001` — highest-priority candidate has no method, a lower-priority candidate does; the committed proposal's `decision_trace_id` and `candidate_goal_id` belong to the same bundle (a new trace for the fallback goal), OR a typed stuck diagnostic names `method_selection`; never a mixed trace/proposal.
2. `cargo test -p tracewake-core --test anti_regression_guards` — fallback-scan-ban guard passes.
3. `cargo test -p tracewake-core` — re-selection is bounded; transaction unit tests pass.

### Invariants

1. Every committed proposal's trace id and candidate id come from one `SelectedGoalBundle` (INV-105).
2. Fallback is a coherent re-run (or a typed stuck terminal), never a silent post-trace swap (INV-034/036); bounded by planner budget (INV-037/038).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs` — proves coherent re-run or typed stuck, never mixed.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning the post-trace fallback scan.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`
