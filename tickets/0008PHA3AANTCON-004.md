# 0008PHA3AANTCON-004: Canonical actor-decision transaction module

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core`: new actor-decision transaction module (the single cognition path), built unwired
**Deps**: 0008PHA3AANTCON-001, 0008PHA3AANTCON-002, 0008PHA3AANTCON-005

## Problem

Spec 0008 Finding 1 (D-0008-01) §8.1: there is no single canonical actor-decision transaction. `build_agent_proposal` (`scheduler.rs:525`) constructs planning state then calls `build_routine_or_need_proposal` (`:654`) which can produce a proposal by routine family / need threshold **before** the candidate→intention→HTN→local-planning chain runs (the chain at `scheduler.rs:586-650` is only a wait-only fallback). The trace can therefore be a narrative attached to an already-chosen action rather than its causal source.

This ticket introduces the canonical transaction (§8.1) as a new module that owns ordinary-life cognition end-to-end and is **not yet wired into the scheduler** (the atomic flip is 0008PHA3AANTCON-006). It consumes the sealed actor-known context (-001), emits typed records (-002), and drives the hardened arbitration chain (-005).

## Assumption Reassessment (2026-06-08)

1. Existing chain symbols confirmed: `generate_candidate_goals_from_agent_state` (`agent/generation.rs`, with `LiveCandidateGenerationInput` at `generation.rs:27`), `select_goal_and_trace`/`DecisionInput`/`DecisionSelection` (`agent/decision.rs:23/:9/:18`), `select_phase3a_method` (`agent/htn.rs:28`), `plan_local_actions`/`LocalPlanRequest`/`PlannerGoal` (`agent/planner.rs:213/:175/:54`), `active_intention_for_actor` (`scheduler.rs:841`). The scheduler fallback at `scheduler.rs:586-650` already composes these into a wait-only proposal — the transaction generalizes that composition into the authoritative path for all ordinary actions.
2. Spec §8.1 fixes the transaction shape (trigger → due-completions → sealed context → needs/intention → candidates → intention lifecycle → method → bounded local plan → proposal with ancestry → pipeline → typed records → live/replay invariants). §8.3 fixes the candidate/BDI/HTN integration the transaction orchestrates. Naming is flexible; shape is mandatory.
3. Cross-artifact boundary under audit: the **transaction ↔ {sealed context (-001), typed records (-002), arbitration chain (-005), shared pipeline}** seam. The transaction is the only orchestrator; it must not duplicate `select_goal_and_trace` — it calls it.
4. INV-032 (V1 agents are symbolic and inspectable) and INV-037 (Bounded local planning is for concrete means, nested under explicit intentions/methods): the transaction sequences belief→candidate→intention→method→bounded plan→proposal, with a decision trace recording each stage.
5. Enforcement surface: actor-knowledge firewall + deterministic replay. **Substrate-only**: the transaction produces the inputs (sealed-context reads, typed decision/stuck records) that 0008PHA3AANTCON-006 wires into the no-human run and 0008PHA3AANTCON-007 guards against bypass. Confirm the transaction admits planner facts only via the sealed context's provenance-emitting accessors (no raw `PhysicalState` read), and that the records it emits route through -002's deterministic typed-event path — introducing no leakage or nondeterminism the later wiring would have to undo.

## Architecture Check

1. One transaction with a fixed stage order is the only way to guarantee the trace is the causal source of the action: candidates/intention/method/plan all run before the proposal exists, and the proposal carries their ancestry IDs. A scheduler that owns proposal policy (current state) can always re-introduce a shortcut; a transaction the scheduler merely triggers cannot.
2. No backwards-compatibility aliasing: the transaction does not wrap `build_routine_or_need_proposal`; that function is deleted by -006. The transaction is the replacement path, not a layer over the old one.

## Verification Layers

1. INV-032 inspectable cognition → unit test: a transaction run over a fixture produces a `DecisionTraceRecord` whose `candidate_ids`/`selected_candidate_id`/`selected_method_id`/`local_plan_id`/`proposal_id` link in order (ancestry present before the proposal).
2. INV-037 bounded nesting → unit test: the local plan is invoked only under a selected method/intention; an empty candidate set yields a typed failure/stuck record, not a silent proposal.
3. Single-path proof → codebase grep-proof: the transaction module calls `select_goal_and_trace` exactly once per decision and contains no routine-family→primitive dispatch (`grep -n "RoutineFamily" <transaction module>` shows no proposal-construction arm).
4. Substrate determinism → manual review: records emitted via -002's typed-event API; planner facts read via -001's sealed accessors only.

## What to Change

### 1. New transaction module

New `crates/tracewake-core/src/agent/transaction.rs`: an `ActorDecisionTransaction` (name flexible) with the §8.1 input set and staged `run(...)` returning a typed outcome (proposal + decision trace, or typed failure/stuck record). Stages call the existing chain symbols (assumption 1) in the mandated order, threading ancestry IDs.

### 2. Ancestry-linked proposal origin

The proposal the transaction emits carries enough origin/ancestry to link decision trace, candidate, intention, method, local plan, and (after pipeline) event IDs (§8.4.5). No live-state echo parameters (consistent with -003).

### 3. Typed outcome records

On success/failure/replan/stuck the transaction returns the typed `DecisionTraceRecord` / `StuckDiagnosticRecord` (-002) for the caller to commit as events. The transaction does not itself own the event log; it produces the records and the validated proposal.

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — declare/export the module)

## Out of Scope

- Wiring `run_no_human_day` to the transaction and deleting the bypasses (0008PHA3AANTCON-006).
- Chronological due-completion processing inside the run loop (0008PHA3AANTCON-006 owns the scheduler loop; the transaction exposes the stage hook).
- Anti-regression guards (0008PHA3AANTCON-007).

## Acceptance Criteria

### Tests That Must Pass

1. Transaction unit test: over a routine-window fixture, produces a proposal with a linked `DecisionTraceRecord` (ancestry IDs resolve in stage order).
2. Empty-candidate test: a trigger with no applicable candidate yields a typed `StuckDiagnosticRecord`, not a silent/forged proposal.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` green (module compiles, unwired).

### Invariants

1. The transaction is the single goal-selection orchestrator; it calls `select_goal_and_trace`, never a parallel selector.
2. No proposal is emitted without prior candidate/intention/method/plan ancestry.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/transaction.rs` (inline `#[cfg(test)]`) — stage-order ancestry and empty-candidate stuck path.

### Commands

1. `cargo test -p tracewake-core agent::transaction`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
