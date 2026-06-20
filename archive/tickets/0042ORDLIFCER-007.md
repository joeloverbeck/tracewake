# 0042ORDLIFCER-007: ORD-LIFE-06 — actor-known method selection, bounded local planning, planner-budget discipline, and coherent fallback

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-06 must be proven: method selection and local planning must cite actor-known provenance for every target, route, affordance, assignment, and temporal premise; planning is bounded and local (it may select among known methods and produce a short proposal sequence, but cannot search omniscient world state, fabricate provenance, or silently exceed its declared budget); and failure must yield a typed blocker plus a coherent newly traced fallback/wait/stuck/intention outcome. This ticket records the method-candidate census, planner-budget boundary witnesses, dangling-provenance negatives, and live/replay trace equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/htn.rs`, `agent/planner.rs` (`PlannerGoal`, `LocalPlanRequest`, `plan_local_actions`, `PlannedProposal`, `LocalPlan`, `LocalPlanTrace`, `LocalPlanFailure`, `DEFAULT_PLANNER_BUDGET`; `PlannerBudgetExhausted`), `agent/methods.rs`, `agent/decision.rs`, `agent/trace.rs`, `agent/transaction.rs` (`dangling_provenance_diagnostic`), `agent/actor_known.rs`, `actions/proposal.rs` (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-06 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 3/5/9 govern this ticket; §4.4 "actor-known cognition" and "meaningful progress only" bind it — a final successful action without rejected-method and budget evidence is incomplete.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-06 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section. Every supported `PlannerGoal` variant is a quantifier to enumerate against code (spec §5 "for every supported `PlannerGoal`").
4. Motivating invariants (spec §5 ORD-LIFE-06): `INV-040` (bounded local planning over known methods), `INV-036`/`INV-037`, `INV-041`, `INV-100`/`INV-102` (actor-known provenance), `INV-105`/`INV-106`. Restate before trusting the narrative: planning cites provenance for every target/route/affordance/premise, stays within `DEFAULT_PLANNER_BUDGET`, and fails to a typed blocker + traced fallback.
5. Evidence-consumer surface (audit-reads, does not modify): the method-selection/local-planning path and its proposal/replay trace. This ticket runs planner-goal and budget-boundary fixtures and records traces; it adds no omniscient search path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Setting budget to smallest boundary values (zero/one/off-by-one) and mutating decrement/termination comparisons to require deterministic success or `PlannerBudgetExhausted` (never an unbounded loop) is the decisive bounded-planning proof — stronger than asserting a budget constant exists.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-040` bounded planning + budget -> replay/golden-fixture check (`DEFAULT_PLANNER_BUDGET` and explicit test budgets are deterministic, enforce a hard bound, and appear in trace/diagnostic evidence when exhausted; boundary cases never loop unbounded).
2. `INV-100`/`INV-102` provenance citation -> codebase grep-proof + manual review (each target/route/affordance/assignment/temporal premise carries source-event citations; remove/stale/reorder/wrong-kind one source at a time → `dangling_provenance_diagnostic` or typed failure names the exact missing/invalid reference before proposal acceptance).
3. `INV-018`/`INV-105` trace replay -> replay/golden-fixture check (method-candidate census, precondition evaluations, selected/rejected methods, local-plan steps, fallback edge, blocker code, responsible layer reproduced live/replay).

## What to Change

### 1. Record per-PlannerGoal planning and budget witnesses

Enumerate every supported `PlannerGoal` into a member list; for each, run the applicable `plan_*` path with a legal actor-known context and record the `LocalPlan`, `PlannedProposal`, budget use, target, route, affordance, and source references matching the selected method. Record `DEFAULT_PLANNER_BUDGET` and explicit test budgets as deterministic hard bounds appearing in trace/diagnostic evidence on exhaustion. Exercise a primary-method failure followed by a different traced fallback or typed stuck via `method_fallback_requires_new_trace_or_stuck_001` and food-unavailable/routine-blocker fixtures. Prove a planned move/eat/sleep/work sequence stays local and proposal-based (each step reaches ordinary validation, not direct state mutation).

### 2. Record adversarial provenance and budget negatives

Record the §5 adversarial cases: remove/stale/reorder/wrong-kind one source witness at a time (`dangling_provenance_diagnostic` or typed failure names the exact missing/invalid reference before proposal acceptance); smallest budget boundary values + mutated decrement/termination comparisons (deterministic success or `PlannerBudgetExhausted`, never unbounded loop); hidden true food / hidden route edge / unobserved sleep affordance / true workplace target (planning unchanged until actor-known evidence changes); and no-applicable-method / empty-plan / unsupported-action / failed-movement-ancestry / validation-rejection (each emits a trace + resulting status).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-06 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any planning/provenance defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Trace/debug-quarantine certification (`-008`), ordinary-action validation (`-009`), and stuck-diagnostic taxonomy (`-011`) — exercised here only where planning produces them.
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test generative_lock` and `cargo test --locked -p tracewake-core --test no_human_capstone` pass; every supported `PlannerGoal` records a `LocalPlan`/`PlannedProposal` with budget use and source references, and budget boundary cases produce deterministic success or `PlannerBudgetExhausted` with no unbounded loop.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` passes; removing/staling/wrong-kinding one source witness triggers `dangling_provenance_diagnostic` or a typed failure naming the exact reference before proposal acceptance.
3. `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; `method_fallback_requires_new_trace_or_stuck_001` shows a primary-method failure yielding a different traced fallback or typed stuck.

### Invariants

1. Bounded local planning: planning never exceeds its declared budget; exhaustion is a deterministic typed outcome, not a loop.
2. Provenance-cited cognition: every target/route/affordance/assignment/premise the planner uses carries actor-known source-event citations; a missing source fails before proposal acceptance.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --locked -p tracewake-core --test golden_scenarios`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-20

Recorded ORD-LIFE-06 as a certifying pass in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with evidence rows `0042-ORD06-GOAL-CENSUS`, `0042-ORD06-BUDGET-PROVENANCE`, and `0042-ORD06-FALLBACK-NEGATIVES`.

The live supported `PlannerGoal` member list was enumerated as `ReachPlace`, `CheckContainer`, `EatKnownFood`, `StartSleep`, `StartWorkBlock`, `LeaveUnsafePlace`, and `WaitWithReason`. The report records finite fixture coverage for route/open/move, check-container, known-food, sleep, work-block, unsafe-place leave, modeled wait, budget exhaustion, hidden-truth, planner-trace, food-unavailable, routine-blocked, and method-fallback cases. No enumerated ORD-LIFE-06 member was deferred or dropped.

Verification commands run and passed:

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --locked -p tracewake-core --test golden_scenarios`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
