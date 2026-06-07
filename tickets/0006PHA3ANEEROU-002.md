# 0006PHA3ANEEROU-002: Canonical actor-known planner-input boundary

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` agent planner input construction (`agent/planner.rs`, `agent/generation.rs`, `scheduler.rs`); hidden-truth fixture enforcement
**Deps**: 0006PHA3ANEEROU-001

## Problem

Hidden-truth planning is not impossible by construction; it is caller-hygiene plus self-attestation (audit **D-04 / F-04**). `ActorKnownPlanningState` (`crates/tracewake-core/src/agent/planner.rs:17`) is the right input *type*, and the direct known-food path requires the source to appear in `known_food_sources` — but the integrated code builds that input in scheduler code from physical actor location and hard-coded strings, and decision traces self-assert `hidden_truth_audit_result.actor_known_only = true` rather than proving construction from beliefs by type. Spec §5.7 requires one canonical boundary: `ActorKnownPlanningState` built only from actor-known/actor-believed projections and visible local state, with raw physical state reachable only through explicit visible/local interfaces, and a hidden-truth audit recording the actual proof path.

## Assumption Reassessment (2026-06-07)

1. `ActorKnownPlanningState` is defined at `crates/tracewake-core/src/agent/planner.rs:17` and consumed by the planner functions at `planner.rs:66/97/171/195`. The no-human runner constructs planner input inside `crates/tracewake-core/src/scheduler.rs` (`build_agent_wait_proposal`, `scheduler.rs:410`) from physical location + hard-coded strings — confirmed the construction site is scheduler code, not a filtered builder.
2. Spec §5.7 requires a single canonical builder from actor-known projections + visible local state; spec §7.6 requires a fixture where hidden food/workplace/route exists physically, the actor lacks a modeled source, the planner cannot select it, and the trace records the absent actor-known proof. `no_hidden_truth_planning_001` fixture exists (`crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs`).
3. Shared boundary under audit: the construction edge from actor knowledge to planner input. The actor-known epistemic projection is `EpistemicProjection` (confirmed carried in `PipelineContext`, `actions/pipeline.rs:78`) plus live `AgentState` (made live by 0006PHA3ANEEROU-001). After this ticket, planner input may be produced **only** by the canonical builder; scheduler/decision code must not hand-assemble `ActorKnownPlanningState`.
4. Motivating invariants (restated): INV-002 "Belief comes before truth", INV-006 "Possession transfers no world knowledge", INV-024 "No telepathy" — actors plan only from modeled actor-known channels, never hidden ground truth.
5. Actor-knowledge-filtering surface touched: the planner-input builder is the firewall between ground-truth `PhysicalState` and actor cognition. This ticket makes the firewall structural — the builder reads only `EpistemicProjection` + `AgentState` + an explicit visible-local-affordance interface; the planner no longer accepts raw global `PhysicalState`. Hidden food/routes/closed containers/inaccessible workplaces/other actors' private knowledge cannot enter planner input absent a modeled source. The hidden-truth audit records the actual proof path, not a boolean assertion. No determinism change (construction is pure over deterministic inputs).

## Architecture Check

1. A single canonical builder makes hidden-truth leakage a compile-time/structural impossibility rather than a convention the caller must remember: if the planner cannot be handed raw `PhysicalState`, no caller can leak it. This is strictly stronger than auditing supplied inputs after the fact (the current self-attestation model).
2. No backwards-compatibility aliasing/shims: the scheduler's hand-rolled input construction is removed, not kept as a fallback. The planner's raw-`PhysicalState` access path (if any) is replaced by the narrow visible-local interface.

## Verification Layers

1. INV-024 / INV-002 (no telepathy; belief before truth) -> replay/golden-fixture check: `no_hidden_truth_planning_001` run integrated proves the planner cannot select a physically-present but actor-unknown food/workplace/route, and the trace records the absent proof.
2. INV-006 (possession transfers no knowledge) -> manual review + test: planner input for an actor is built only from that actor's `EpistemicProjection` + `AgentState`, never another actor's or a prior possessed body's state.
3. Structural firewall -> codebase grep-proof: planner functions take `ActorKnownPlanningState` from the canonical builder only; no `PhysicalState` is passed to the planner except through the explicit visible-local interface.

## What to Change

### 1. Canonical builder

Add one builder `fn build_actor_known_planning_state(actor, &EpistemicProjection, &AgentState, &VisibleLocal) -> ActorKnownPlanningState` (name per house style) in `agent/planner.rs`. It populates known routes/containers/food/workplace access strictly from actor-known projections + visible local affordances.

### 2. Restrict planner physical access

Remove any raw `PhysicalState` parameter from planner entry points; replace with a narrow visible-local-affordance interface filtered by what the actor can observe/legitimately know.

### 3. Route construction through the builder + real audit

Replace the scheduler's hand-assembled inputs with the builder. The hidden-truth audit records the actual proof source per included fact rather than a self-asserted boolean.

## Files to Touch

- `crates/tracewake-core/src/agent/planner.rs` (modify)
- `crates/tracewake-core/src/agent/generation.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)

## Out of Scope

- The autonomous decision loop that calls the builder per actor/window (0006PHA3ANEEROU-007).
- Typed HTN method conditions (0006PHA3ANEEROU-003) — this ticket supplies the actor-known state the resolver consumes.
- Live event application (0006PHA3ANEEROU-001, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. `no_hidden_truth_planning_001` run through the integrated planner: a physically-present, actor-unknown food/workplace/route is never selected, and the decision trace records the missing actor-known proof.
2. A negative test confirming the planner cannot be constructed with raw global `PhysicalState` (the type/interface forbids it).
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. `ActorKnownPlanningState` is produced only by the canonical builder; no other construction site exists.
2. Planner input for actor A derives only from A's actor-known projection + live `AgentState` + visible local affordances.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs` — make the fixture assert integrated planner rejection, not hand-constructed input absence.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` — run the hidden-truth fixture through the real planner boundary.

### Commands

1. `cargo test -p tracewake-content --test golden_fixtures_run`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
