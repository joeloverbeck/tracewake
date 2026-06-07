# 0005PHA3ANEEROU-014: Bounded local planning and hidden-truth audit

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the bounded GOAP/STRIPS-style local planner for concrete action sequencing within a selected routine step, with a hidden-truth audit.
**Deps**: 0005PHA3ANEEROU-013

## Problem

Bounded local planning is allowed only for concrete action sequencing inside a selected goal/method (route home→work via known places, open a known door before moving, check a known pantry before eating) — with a bounded budget, deterministic expansion order, no hidden-truth reads, no global omniscient scan, and typed failures (Spec 0005 §11.4, §3.3; `INV-037`, `INV-024`). This is the means-level planner that turns routine steps needing routing into concrete `move`/`open`/`check_container` proposals, and the surface the no-hidden-truth-planning proof (ticket 020) targets.

## Assumption Reassessment (2026-06-07)

1. The routine methods/selector (ticket 013) invoke this planner for steps needing routing/checking; the ordinary actions (008–011) and Phase 1 `move`/`open`/`check_container` are the proposable primitives; the decision-trace model (ticket 004) records planner inputs/candidates/selection. Phase 2A knowledge context (`epistemics/knowledge_context.rs`) and the place/exit graph (`location.rs`) supply actor-known adjacency.
2. Spec §11.4 fixes the constraints: bounded budget; deterministic expansion order; no hidden-truth reads; no global omniscient route/resource scan unless the actor has authored public knowledge; every failure reports `planner-budget-exhausted`/`knowledge`/`physical`/`access`/`resource` or another typed blocker (ticket 004 categories); debug trace records inputs, candidates, selected plan, rejected steps. The planner may propose `move`/`open`/`check_container`/`eat`/`sleep`/`work_block`/`wait`/`continue_routine` and must not directly mutate state.
3. Shared boundary under audit: the planner expands a routine step into a concrete proposal sequence, all validated through the shared pipeline (tickets 008–011); it is the GOAP/STRIPS layer nested under HTN methods (`INV-037`). Its budget and expansion order are fixed here and feed determinism (ticket 006) and the planner-trace fixture (ticket 020, `planner_trace_001`).
4. Invariants motivating this ticket: `INV-037` — "Bounded local planning is for concrete means" (GOAP/STRIPS-style planning sequences concrete actions; must be bounded and nested under explicit intentions/methods) — and `INV-024` — "No telepathy" (the planner searches a tiny actor-known space and can fail; it may not read ground-truth container contents/hidden locations). The action validator may check authoritative truth to accept/reject a proposal, but the planner may not inspect that truth as a decision oracle (Spec §3.3, §21 "action validator result is used as a planner oracle before proposal" is forbidden).
5. Actor-knowledge / deterministic-replay surface: this is the primary no-hidden-truth surface. The planner reads only actor-known adjacency/beliefs; it emits a `hidden_truth_audit_result` on its trace (ticket 004) proving actor-known-only inputs. It is deterministic (fixed expansion order, bounded budget, no RNG) and adds no leakage. The no-hidden-truth-planning proof is ticket 020; replay is ticket 006 (both cited).

## Architecture Check

1. A bounded planner nested under HTN methods (rather than a free global search or a giant state machine) gives goal/action decoupling with fail-able, inspectable, deterministic sequencing — the GOAP lesson minus omniscience/cinematics (Spec §25.3). Forbidding any read of authoritative truth in the planner (the validator owns truth checks at proposal time) keeps the actor subjective by construction and makes the validator-as-oracle shortcut structurally impossible (`INV-024`, Spec §21).
2. No backwards-compatibility shims: the planner proposes existing registry actions through the shared pipeline; it introduces no new mutation path and no global scan.

## Verification Layers

1. Bounded means planning (`INV-037`) -> unit test: the planner sequences `open`→`move` toward a known place within budget; exceeding the budget yields a `planner-budget-exhausted` diagnostic with the candidates tried.
2. No telepathy / no oracle (`INV-024`; Spec §3.3/§21) -> integration test + grep-proof: the planner's inputs are actor-known adjacency/beliefs only; a grep-proof confirms it never calls the action validator or reads authoritative container contents before proposing; a believed-but-wrong route fails with a typed blocker.
3. Determinism (`INV-018`, substrate-only) -> unit test: identical actor-known state yields identical expansion order and selected plan; the trace records inputs/candidates/rejected steps. Replay is ticket 006; no-hidden-truth fixture is ticket 020 (cited).

## What to Change

### 1. Bounded local planner

Add `crates/tracewake-core/src/agent/planner.rs` implementing a bounded, deterministic-expansion-order local planner over the actor-known place/exit graph and beliefs: given a routine step needing routing/checking, produce a concrete proposal sequence (`move`/`open`/`check_container`/`eat`/`sleep`/`work_block`/`wait`/`continue_routine`) within a fixed budget. It reads only actor-known facts and never the action validator or authoritative truth.

### 2. Hidden-truth audit + typed failures

On every plan attempt, populate the `hidden_truth_audit_result` on the `DecisionTrace` (ticket 004) and, on failure, emit the typed blocker (`planner-budget-exhausted`/`knowledge`/`physical`/`access`/`resource`). Record inputs, candidates, selected plan, and rejected steps for debug.

### 3. Module registration

Declare `pub mod planner;` in `crates/tracewake-core/src/agent/mod.rs` and re-export the entry point used by tickets 011 (continue) and 013 (methods).

## Files to Touch

- `crates/tracewake-core/src/agent/planner.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; declare `pub mod planner;`)

## Out of Scope

- Routine method authoring/selection (ticket 013) — this ticket is the within-step sequencer it calls.
- The no-human runner (ticket 017).
- The `planner_trace_001` / `no_hidden_truth_planning_001` golden fixtures (ticket 020).
- Any global multi-actor route optimization or omniscient scan (forbidden — Spec §11.4).

## Acceptance Criteria

### Tests That Must Pass

1. The planner sequences a concrete `open`→`move` plan toward a known place within budget; budget exhaustion yields a `planner-budget-exhausted` diagnostic listing candidates tried.
2. A believed-but-wrong route/food location fails with a typed `knowledge`/`access`/`resource` blocker; the planner never reads authoritative container contents or the validator before proposing (grep-proof).
3. Identical actor-known state yields identical expansion order and selected plan; the trace records inputs/candidates/rejected steps and the hidden-truth audit.

### Invariants

1. Local planning is bounded, deterministic, nested under a method, and proposes (never mutates) (`INV-037`).
2. The planner reads only actor-known facts; truth checks belong to the validator, not the planner (`INV-024`, Spec §3.3).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/planner.rs` (unit tests) — bounded sequencing, budget exhaustion, typed failures, deterministic expansion, hidden-truth audit, no-oracle grep-proof.

### Commands

1. `cargo test -p tracewake-core agent::planner`
2. Core-crate scope is correct; the no-hidden-truth-planning acceptance proof is ticket 020 and the capstone (025).
3. A grep-proof that `agent::planner` does not call the action validator or read authoritative container contents is the correct no-oracle boundary at the source level.
