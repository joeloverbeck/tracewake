# 0014PHA3AORDLIF-001: No-human actor-known surface builder

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (new `agent` submodule for the sealed no-human surface builder; `scheduler.rs` no-human proposal path), `tracewake-content` (3 new adversarial fixtures), new source guard in `anti_regression_guards.rs`
**Deps**: None

## Problem

In the no-human driver, `scheduler.rs::build_agent_proposal` constructs the actor's cognitive surface from authoritative `PhysicalState`. `visible_local_planning_state` (`crates/tracewake-core/src/scheduler.rs:676-709`) reads `state.places` adjacency, `state.food_supplies`, and `state.workplaces` (filtered by `assigned_actor_ids.contains(actor_id)`), and seeds known sleep places with `BTreeSet::from([current_place_id.clone()])`. `build_agent_proposal` then turns that surface into actor-known facts including `sleep_place_believed_accessible` and `actor_at_workplace` / `assigned_workplace_known` (`scheduler.rs:596-626`). This manufactures actor-known cognition directly from raw assignment and location truth, so an unpossessed actor can plan work or sleep without ever having acquired that knowledge through a modeled channel. This is the ORD-HARD-001 blocker against scoped `ORD-LIFE-CERT` evidence.

## Assumption Reassessment (2026-06-09)

1. `visible_local_planning_state` exists at `crates/tracewake-core/src/scheduler.rs:676-709` and is called only from `build_agent_proposal` at `scheduler.rs:544`; it returns `VisibleLocalPlanningState` (defined in `crates/tracewake-core/src/agent/actor_known.rs`, re-exported via `crates/tracewake-core/src/agent/mod.rs`). The `BTreeSet::from([current_place_id.clone()])` sleep-place default is at `scheduler.rs:706`; the `sleep_place_believed_accessible` / workplace fact construction is at `scheduler.rs:596-626`.
2. The spec `specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md` §ORD-HARD-001 and §5.1/§5.2/§5.5 require a provenance-bearing `NoHumanActorKnownSurfaceBuilder` producing a `SealedActorKnownSurface`, removal of the current-place sleep default, and a source guard. `ActorKnownFact` constructors (`observed_now`, `remembered_belief`) live in `crates/tracewake-core/src/agent/actor_known.rs`; `KnowledgeProvenanceKind` (the provenance enum to reuse for source classes) is at `crates/tracewake-core/src/epistemics/knowledge_context.rs:88`.
3. Shared boundary under audit: the no-human cognition surface seam between `scheduler.rs` (caller) and the `agent` actor-known surface module (builder). The contract is that the scheduler may invoke the decision transaction but may not author cognitive facts from raw truth (INV-103). The builder may read authoritative truth only to *generate modeled observations / validate visibility*, never to emit a cognitive fact directly.
4. Invariants motivating this ticket: **INV-099** (truth may validate but not plan), **INV-100** (hidden-truth cognition forbidden), **INV-101** (actor-known context is sealed; no validator-only truth), **INV-103** (the scheduler is not a cognition authority — must not construct proposals from raw state / true workplace / true route). Arch `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` forbids raw route/workplace/food/sleep tables as cognition sources.
5. Actor-knowledge / deterministic-replay enforcement surface: the actor-known facts feed `ActorDecisionTransaction::run` and the decision-trace context hash, which §5.4 requires to be replay-reconstructable from *modeled knowledge*, not raw physical truth. This change must keep the surface construction deterministic (sorted/`BTreeMap`/`BTreeSet` collections, no wall-clock, no hash-map iteration order) so identical inputs + versions reproduce the same sealed surface and context hash (INV-018). It must not introduce any path by which hidden truth reaches the transaction (INV-100).
6. Removal blast radius: `visible_local_planning_state` is private to `scheduler.rs` (only caller is `build_agent_proposal`), so its removal/replacement is local. `VisibleLocalPlanningState` itself is still consumed by `crates/tracewake-core/src/agent/planner.rs`, `crates/tracewake-core/src/agent/actor_known.rs`, and tests (`anti_regression_guards.rs`, `hidden_truth_gates.rs`, content `golden_fixtures_run.rs`); this ticket does **not** delete that type — it replaces only the no-human *construction* path that fabricates facts from truth. Confirm no other no-human caller seeds sleep/workplace facts from raw state.

## Architecture Check

1. A dedicated provenance-bearing `NoHumanActorKnownSurfaceBuilder` returning a sealed `SealedActorKnownSurface` makes "fact without provenance" unrepresentable at the type level (Rust private fields + accessor-only construction), which is stronger than auditing the scheduler by review. It keeps cognition authority out of the scheduler (INV-103) by construction rather than convention.
2. No backwards-compatibility shim: the old current-place sleep default and raw workplace/food fact seeding are removed outright, not wrapped behind a flag or fallback. The no-human path calls the new builder directly.

## Verification Layers

1. INV-103 (scheduler not a cognition authority) -> codebase grep-proof: source guard in `anti_regression_guards.rs` fails on `state.workplaces`, `state.sleep_places`, raw `state.food_supplies`, `state.places` adjacency traversal, or `BTreeSet::from([current_place_id` inside no-human cognition-surface construction except via an allowlisted observation/perception function.
2. INV-100 / INV-101 (no hidden-truth cognition; sealed context) -> replay/golden-fixture check: the 3 adversarial fixtures prove no work/sleep candidate or proposal is selected when the raw assignment/location exists but the actor-known surface lacks the corresponding provenance-bearing fact.
3. INV-018 (deterministic replay) -> replay/golden-fixture check: the no-human capstone replay still byte-matches after the builder swap (context hashes rebuild from modeled knowledge).
4. INV-099 (truth may validate, not plan) -> invariants alignment check: builder reads authoritative truth only to generate modeled observations / validate visibility, cited in the ticket and enforced by the guard's allowlist.

## What to Change

### 1. Introduce the sealed no-human actor-known surface builder

Add `SealedActorKnownSurface` (private fields, accessor-only) and `NoHumanActorKnownSurfaceBuilder` in a new `crates/tracewake-core/src/agent/no_human_surface.rs`, declared in `crates/tracewake-core/src/agent/mod.rs`. The builder consumes only: modeled observations, memories, records, role-assignment notices, home/sleep-place knowledge, visible local affordances, and explicit unknowns. It may read authoritative `PhysicalState` solely to (a) validate that something is visible from the actor's current place and (b) generate modeled observation facts — never to emit a cognitive fact directly. Each emitted `ActorKnownFact` carries a `KnowledgeProvenanceKind`-classed source.

### 2. Remove fact-fabrication from the no-human path

In `scheduler.rs::build_agent_proposal`, replace the `visible_local_planning_state` call and the `sleep_place_believed_accessible` / `actor_at_workplace` / `assigned_workplace_known` fact construction (`scheduler.rs:596-626`) with the sealed builder's output. Delete the `BTreeSet::from([current_place_id.clone()])` sleep-place default — sleep places must come from an explicit provenance-carrying sleep/rest affordance fact (the affordance itself is modeled in ticket 0014PHA3AORDLIF-006).

### 3. Source guard

Add a named guard to `anti_regression_guards.rs` per Verification Layer 1, with an explicit allowlist for the named perception/observation builder.

### 4. Adversarial fixtures

Add 3 fixtures proving the firewall (see Test Plan), registered in `crates/tracewake-content/src/fixtures/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — declare/export the new module)
- `crates/tracewake-core/src/scheduler.rs` (modify — no-human surface construction; **shared with 0014PHA3AORDLIF-002 in `build_agent_proposal`**)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #1; **N-way shared hub**)
- `crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_current_place_without_sleep_affordance_does_not_sleep_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register 3 fixtures; **shared hub**)

## Out of Scope

- Modeling the sleep/rest affordance state and content schema (ticket 0014PHA3AORDLIF-006 / -007) — this ticket only removes the current-place default and requires an actor-known sleep surface to exist.
- Sealing the transaction output proposal against scheduler mutation (ticket 0014PHA3AORDLIF-002).
- The embodied/possessed projection workplace path (ticket 0014PHA3AORDLIF-008).
- Deleting the `VisibleLocalPlanningState` type or its non-no-human consumers (`planner.rs`).

## Acceptance Criteria

### Tests That Must Pass

1. `no_human_unseen_workplace_assignment_does_not_plan_work_001` — raw `state.workplaces` assignment exists but the actor lacks an assignment notice/observation; no work candidate/proposal is selected.
2. `no_human_current_place_without_sleep_affordance_does_not_sleep_001` — fatigue is high and the actor is at an arbitrary place; no sleep proposal is selected without an actor-known sleep surface.
3. `no_human_known_workplace_requires_provenance_001` — a workplace becomes usable only after a modeled assignment notice/memory/observation is present in the actor-known packet.
4. `cargo test -p tracewake-core --test anti_regression_guards` — the new source guard passes (no banned raw-truth read in the no-human surface path).
5. `cargo test -p tracewake-core --test no_human_capstone` — the no-human capstone (including replay byte-match) still passes after the builder swap.

### Invariants

1. The scheduler authors no cognitive fact from raw `state.workplaces` / `state.food_supplies` / `state.places` adjacency / current-place sleep default (INV-103).
2. The sealed surface and its context hash are deterministically reconstructable from modeled knowledge under replay (INV-018), with no hidden-truth path to the transaction (INV-100).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs` — adversarial: raw assignment present, provenance absent → no work plan.
2. `crates/tracewake-content/src/fixtures/no_human_current_place_without_sleep_affordance_does_not_sleep_001.rs` — adversarial: high fatigue, no actor-known sleep surface → no sleep proposal.
3. `crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs` — workplace usable only with modeled provenance.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — source guard banning raw-truth reads in the no-human cognition-surface builder (allowlisted observation fn excepted).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-content` (drives the 3 new golden fixtures via the content fixture runner)
3. `cargo test --workspace` — full-pipeline confirmation that the no-human capstone, replay, and golden scenarios still pass.
