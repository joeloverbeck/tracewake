# 0008PHA3AANTCON-001: Sealed actor-known planning context with provenance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` agent module: new sealed `ActorKnownPlanningContext` (provenance-bearing), replaces public `ActorKnownPlanningState` / `VisibleLocalPlanningState`; hidden-truth audit recomputed from provenance graph
**Deps**: None

## Problem

Spec 0008 Finding 2 (D-0008-02): the no-human builder constructs `EpistemicProjection::new(...)` and derives `VisibleLocalPlanningState` directly from `PhysicalState` (current-place adjacency, local food supplies, sleep places, assigned workplaces), then `build_actor_known_planning_state` records physical-derived facts as `Modeled` actor-known facts. The hidden-truth audit trusts those proof tags rather than a sealed provenance graph — self-attestation. `ActorKnownPlanningState` (`crates/tracewake-core/src/agent/planner.rs:62`) and `VisibleLocalPlanningState` (`planner.rs:76`) expose public fields that any caller can populate from raw physical truth.

This ticket builds the sealed, provenance-rich actor-known context (§8.2) **additively**, alongside the existing types, so no no-human behavior changes here. The scheduler/transaction switch to it lands later (0008PHA3AANTCON-004/-006). This is the substrate that makes physical-oracle planning structurally impossible.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/agent/planner.rs:62` defines `pub struct ActorKnownPlanningState` and `:76` `pub struct VisibleLocalPlanningState`; `build_actor_known_planning_state` at `planner.rs:86`; `PlannerGoal` (`:54`), `LocalPlanRequest` (`:175`), `plan_local_actions` (`:213`) consume the actor-known surface. `ActorKnownFact::proof_note` is used at `scheduler.rs:566`.
2. Spec `specs/0008_PHASE_3A_ANTI_CONTAMINATION_HARDENING_SPEC.md` §8.2 enumerates the mandatory properties (sealed fields, restricted constructors, per-fact identity/kind/value/time/actor/provenance, the six provenance variants, audit-from-graph). §4.1 binds INV-024 / INV-002.
3. Cross-artifact boundary under audit: the **actor-known fact surface** shared by `agent/planner.rs` (producer of planning state), `agent/htn.rs::select_phase3a_method` (`htn.rs:28`), and `scheduler.rs::build_actor_known_planning_state` callers. The sealed context becomes the single constructor-gated boundary.
4. INV-024 (No telepathy): information reaches actors only through modeled channels; INV-002 (Belief comes before truth): actors act from beliefs/observations, not hidden ground truth. The context must make a planner fact uncreatable from `PhysicalState` except through an explicit visible-local observation function that emits provenance.
5. Enforcement surface: the actor-known/hidden-truth firewall. The hidden-truth audit (`HiddenTruthAudit`, `agent/trace.rs:37`) must be recomputed from the provenance graph, not from boolean/string `Modeled` tags. Confirm: no provenance variant constructible inside planner context may carry `PipelineValidationTruth` or `DebugOmniscience`; building those inside planning fails to compile or fails at runtime. No epistemic-leakage path is introduced — the sealed type strictly narrows what was previously public.
6. Restructures a consumed type (`ActorKnownPlanningState`/`VisibleLocalPlanningState` → sealed `ActorKnownPlanningContext`). Consumers: `agent/planner.rs` (`plan_local_actions`, `LocalPlanRequest`), `agent/htn.rs` (`select_phase3a_method`), `scheduler.rs` (`build_agent_proposal`, `build_routine_or_need_proposal`, `build_actor_known_planning_state`, test helpers ~`scheduler.rs:2200`). Change is **breaking** by intent (the public-field constructor is removed); consumers compile against the new accessors. Scheduler call sites are rewired in 0008PHA3AANTCON-006, so this ticket keeps a thin adapter only if needed to keep the tree compiling, removed by -006 (no shim survives the flip).
7. Removes public field access on `VisibleLocalPlanningState`/`ActorKnownPlanningState`. Blast radius grep (repo-wide) at implementation: `grep -rn "VisibleLocalPlanningState\|ActorKnownPlanningState" crates/` — every match outside this ticket's Files to Touch either joins Files to Touch or is rewired by its owning ticket (scheduler sites → -006). No matches in `docs/`, `specs/`, `.claude/skills/`.

## Architecture Check

1. A sealed context with constructor-gated provenance is the only design that makes "plan from raw `PhysicalState`" a compile/runtime error rather than a convention. Public-field structs (current state) can always be repopulated from truth — that is exactly how the contamination recurs. Provenance-as-data (each fact records why the actor knows it) also gives the hidden-truth audit something real to verify.
2. No backwards-compatibility aliasing: the old public-field types are removed, not aliased. Any transitional adapter introduced to keep the tree compiling is named in Files to Touch and deleted by 0008PHA3AANTCON-006; no `pub` re-export of the old shape survives.

## Verification Layers

1. INV-024 no-telepathy → codebase grep-proof: no `ActorKnownPlanningContext` constructor accepts `&PhysicalState` except the explicit visible-local observation fn; `grep -rn "PhysicalState" crates/tracewake-core/src/agent/` shows physical truth entering only through the provenance-emitting perception fn.
2. INV-002 belief-before-truth → unit test: a fact with no observation/belief/assignment provenance cannot be added to the context (constructor rejects / type makes it unrepresentable).
3. Hidden-truth audit integrity → unit test: `HiddenTruthAudit` recomputed from a context whose facts carry mixed provenance reports the physical-only facts as non-actor-known; tampering a provenance tag changes the audit result (audit reads the graph, not a stored boolean).
4. Single-boundary proof → manual review: `PipelineValidationTruth` and `DebugOmniscience` variants are not constructible from within the planner module (enforced by module privacy / sealed enum).

## What to Change

### 1. Introduce sealed `ActorKnownPlanningContext`

New module `crates/tracewake-core/src/agent/actor_known.rs`: a sealed context whose facts each carry stable identity, semantic kind, value, time, actor, and a `Provenance` enum with the six §8.2 variants (`ObservedNow`, `RememberedBelief`, `RoutineAssignment`, `FixturePossibility`, `PipelineValidationTruth`, `DebugOmniscience`). Public access is through narrow read accessors; construction is through module-private builders plus explicit perception/epistemic assembly functions. `PipelineValidationTruth` / `DebugOmniscience` constructors are not exposed to planner code.

### 2. Visible-local perception as the only physical-truth admission path

A single `observe_visible_local(...)` function admits local perception from a constrained view and stamps each emitted fact with `ObservedNow` provenance (observation event/projection reference). Raw `PhysicalState` is not a general planner input.

### 3. Hidden-truth audit from the provenance graph

Recompute `HiddenTruthAudit` (`agent/trace.rs:37`) from the context's provenance graph instead of trusting `Modeled` tags. Remove the hard-coded `actor_known_only=true` assertion path (currently surfaced at `scheduler.rs:1027`) — that flip is owned by -006, but the audit computation moves here.

### 4. Adapt planner consumers

`plan_local_actions` / `LocalPlanRequest` (`planner.rs`) and `select_phase3a_method` (`htn.rs:28`) accept the sealed context. Keep the old `build_actor_known_planning_state` compiling via a thin internal adapter only if required; mark it for removal by -006.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — declare/export the module)
- `crates/tracewake-core/src/agent/planner.rs` (modify — remove public-field types; route through sealed context)
- `crates/tracewake-core/src/agent/htn.rs` (modify — `select_phase3a_method` takes the sealed context)
- `crates/tracewake-core/src/agent/trace.rs` (modify — `HiddenTruthAudit` computed from provenance graph)

## Out of Scope

- Wiring the scheduler / no-human runner to the sealed context (0008PHA3AANTCON-006).
- The canonical actor-decision transaction (0008PHA3AANTCON-004).
- Adversarial hidden-truth fixtures and gates (0008PHA3AANTCON-008).
- Passing a real `EpistemicProjection` into the no-human path (0008PHA3AANTCON-006).

## Acceptance Criteria

### Tests That Must Pass

1. New unit test: constructing an `ActorKnownPlanningContext` fact from raw `PhysicalState` outside `observe_visible_local` does not compile / is unrepresentable (compile-fail doctest or type-level proof).
2. New unit test: `HiddenTruthAudit` over a mixed-provenance context flags physical-only facts as non-actor-known and changes result when a provenance tag is tampered.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` green (tree compiles with consumers adapted).

### Invariants

1. No `ActorKnownPlanningContext` fact exists without provenance; planner code cannot construct `PipelineValidationTruth` / `DebugOmniscience`.
2. The audit derives from the provenance graph, never from a stored boolean/string.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/actor_known.rs` (inline `#[cfg(test)]`) — provenance construction restrictions and audit-from-graph.
2. `crates/tracewake-core/src/agent/trace.rs` tests — `HiddenTruthAudit` recomputation from provenance.

### Commands

1. `cargo test -p tracewake-core agent::actor_known`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
