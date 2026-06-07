# 0007PHA3ASECHAR-004: Actor-known planning context replacing PhysicalState oracle

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` no-human proposal builders and actor-known planning context (`scheduler.rs`, `agent/planner.rs`); hidden-truth construction fixtures (`no_hidden_truth_planning_001`, `food_unavailable_replan_001`)
**Deps**: 0007PHA3ASECHAR-001, 0007PHA3ASECHAR-002, 0007PHA3ASECHAR-003

## Problem

Autonomous proposal builders choose food, workplaces, and movement destinations from authoritative physical truth before any actor-known planning runs, making hidden-truth planning possible by construction (Spec 0007 D-04, D-01; Binding constraints 2, 3; §Forbidden shortcuts). At the audited commit:

- `eat_proposal` selects from `state.food_supplies.values().find(..)` (`crates/tracewake-core/src/scheduler.rs` §`eat_proposal`); `work_or_move_proposal` reads `state.workplaces` and `place.adjacent_place_ids` directly; `sleep_proposal` likewise uses authoritative place state.
- `build_agent_proposal` constructs an empty `EpistemicProjection::new(content_manifest_id)` and an all-empty `VisibleLocalPlanningState { visible_edges: {}, visible_food_sources: {}, .. }` (`scheduler.rs:446-458`), then declares actor-known planning over it while the proposal builders read authoritative `PhysicalState` directly.
- `build_agent_proposal` passes `active_intention: None` (`scheduler.rs:463,472`) even when the actor has an active intention in `AgentState`.

The forbidden list names exactly these: "selecting food directly from `PhysicalState.food_supplies`", "selecting workplaces directly from `PhysicalState.workplaces`", "selecting movement destinations from raw `PlaceState.adjacent_place_ids`", "constructing an empty `EpistemicProjection` and then declaring hidden-truth safety by convention", and "passing `active_intention: None` when the actor has an active intention".

## Assumption Reassessment (2026-06-07)

1. Confirmed: `eat_proposal` reads `state.food_supplies` and `work_or_move_proposal` reads `state.workplaces` / `adjacent_place_ids` directly (`crates/tracewake-core/src/scheduler.rs`, §`eat_proposal`/`work_or_move_proposal`); the empty `EpistemicProjection::new(..)` + empty `VisibleLocalPlanningState` at `scheduler.rs:446-458`; `active_intention: None` at `scheduler.rs:463,472`. The actor-known builder `build_actor_known_planning_state` and `ActorKnownPlanningState`/`VisibleLocalPlanningState` live at `crates/tracewake-core/src/agent/planner.rs:19,30`.
2. Spec 0007 D-04 requires autonomous planning be "structurally unable to inspect hidden physical truth as a decision oracle"; Binding constraint 3 keeps validator-only truth out of goal/food/route/workplace selection; the §Hidden-truth construction tests require that hidden food/route/workplace/container present in a fixture are NOT selected by the autonomous generator.
3. Shared boundary under audit: the `ActorKnownPlanningState` / `VisibleLocalPlanningState` contract (`agent/planner.rs`) between the actor-visible sources (epistemic projection + actor-known facts, producers) and the proposal builders (consumers). The proposal builders must select only from this context; `PhysicalState` may be read by the action validator at submission time, never by the autonomous selector.
4. Motivating invariants (restated): INV-024 "No telepathy" and INV-006 "Possession transfers no world knowledge" — autonomous selection uses only modeled actor-known/visible-local inputs; INV-008 "UI assistance is not authority" and INV-043 "Action validation is ordinary-agent validation" — the validator (not the selector) reads authoritative truth, and it does not branch for autonomy.
5. Actor-knowledge-firewall surface touched (central): this ticket is the structural no-leak enforcement for autonomous planning. The proposal builders are rewritten to consume a real `ActorKnownPlanningState` built from the actor's epistemic projection and the typed actor-known facts (0007PHA3ASECHAR-001); a hidden food/route/workplace with no actor-known proof is unreachable by selection. A forced human/test proposal targeting hidden truth still reaches the validator and is rejected there (the validator's authoritative read is lawful). Determinism: selection consumes canonical actor-known data in stable order; no `PhysicalState` iteration leaks nondeterministic ordering into the choice.

## Architecture Check

1. Routing all autonomous food/route/workplace/sleep selection through one `ActorKnownPlanningState` makes hidden-truth planning impossible by construction rather than by reviewer vigilance, and concentrates the actor-known/visible-local read in one inspectable place. The validator retains its lawful authoritative read, preserving the "selector is bounded, validator is authoritative" split (INV-043).
2. No backwards-compatibility aliasing/shims: the direct `PhysicalState` reads in the proposal builders and the empty-projection-by-convention are removed; no oracle fallback remains.

## Verification Layers

1. INV-024 / INV-006 (no hidden-truth oracle) -> codebase grep-proof: `eat_proposal`/`sleep_proposal`/`work_or_move_proposal` no longer read `state.food_supplies` / `state.workplaces` / `adjacent_place_ids`; they consume `ActorKnownPlanningState`.
2. No-leak by construction -> replay/golden-fixture check: `no_hidden_truth_planning_001` (hidden food at the actor's place, hidden route edge, hidden workplace) proves the autonomous generator selects none of them.
3. INV-043 (validator still authoritative) -> negative test: a forced proposal targeting hidden food is rejected at validation, while the autonomous generator never proposes it.
4. `active_intention` wiring -> unit test: `build_agent_proposal` passes the actor's real active intention, not `None`, when one exists in `AgentState`.

## What to Change

### 1. Build a real actor-known planning context

In `build_agent_proposal`, construct `ActorKnownPlanningState` / `VisibleLocalPlanningState` from the actor's epistemic projection and the typed actor-known facts (0007PHA3ASECHAR-001) — populated visible edges, visible food sources, visible containers — instead of `EpistemicProjection::new(..)` plus all-empty visible state.

### 2. Select only from actor-known inputs

Rewrite `eat_proposal`, `sleep_proposal`, and `work_or_move_proposal` to choose food, rest place, workplace, and movement destination from the `ActorKnownPlanningState`, not from authoritative `PhysicalState`. When no actor-known option exists, produce a food-search / replan / wait-with-typed-reason / typed-blocker proposal.

### 3. Pass the real active intention

Replace `active_intention: None` (`scheduler.rs:463,472`) with the actor's active intention read from `AgentState`.

### 4. Hidden-truth construction fixtures

Strengthen / add `no_hidden_truth_planning_001` (hidden food at current place, hidden route edge to a workplace, hidden workplace assignment, hidden container) and `food_unavailable_replan_001` so the autonomous generator demonstrably avoids hidden facts while the validator still rejects forced proposals that target them.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/agent/planner.rs` (modify)
- `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs` (modify)

## Out of Scope

- Live need-delta emission and threshold re-evaluation (0007PHA3ASECHAR-005).
- Intention lifecycle event emission (0007PHA3ASECHAR-006) and routine step ancestry (0007PHA3ASECHAR-007) — this ticket wires the active intention into selection but does not own its lifecycle events.
- The integrated capstone assertion (0007PHA3ASECHAR-012).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: `grep -nE 'food_supplies|workplaces|adjacent_place_ids' crates/tracewake-core/src/scheduler.rs` shows no read inside `eat_proposal`/`sleep_proposal`/`work_or_move_proposal` (only the validator path, if any, may read authoritative state).
2. `no_hidden_truth_planning_001`: with hidden food at the actor's place and a hidden route edge, the no-human run selects neither; the autonomous decision instead yields a typed food-search/replan/wait.
3. A forced test proposal targeting the hidden food is rejected at validation.
4. `build_agent_proposal` passes the actor's real active intention (unit test asserts non-`None` when present).
5. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Autonomous selection reads only `ActorKnownPlanningState`; no `PhysicalState` oracle remains in the proposal builders.
2. Hidden physical truth present in a fixture is never selected by the autonomous generator; the validator still rejects forced hidden-truth proposals.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` — unit tests: actor-known selection; food-search fallback when no actor-known food; real active intention passed.
2. `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs` — hidden food/route/workplace/container not selected; validator rejects forced proposals.
3. `crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs` — hunger with no actor-known food drives replan/wait-with-reason.

### Commands

1. `cargo test -p tracewake-core scheduler`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

What changed:

- Extended `ActorKnownPlanningState` / `VisibleLocalPlanningState` with actor-known sleep places and workplaces, with modeled proof facts for visible sleep/workplace context.
- Reworked no-human proposal building so `build_agent_proposal` builds a populated visible-local context, passes that through actor-known planning, and supplies the actor's real active intention to candidate generation and decision selection.
- Rewrote `eat_proposal`, `sleep_proposal`, and `work_or_move_proposal` to select from `ActorKnownPlanningState` instead of authoritative physical maps.
- Strengthened `no_hidden_truth_planning_001` with a closed opaque hidden pantry, hidden food, a hidden workshop edge, and an unassigned hidden workplace; the no-human run now asserts that none of those hidden targets are selected.
- Kept the forced hidden-food proposal path in the fixture test to prove validation still rejects hidden/inaccessible truth when a caller explicitly submits it.

Deviations from original plan:

- The scheduler still has one visible-local context builder that translates currently visible physical state into `VisibleLocalPlanningState`. That is the intended boundary: selectors consume actor-known data only, while the context builder and validator remain the physical-state read sites.
- `food_unavailable_replan_001` did not need a fixture change for this acceptance slice; the strengthened `no_hidden_truth_planning_001` plus workspace tests cover the hidden food/route/workplace/container construction.

Verification results:

- `rg -n "food_supplies|workplaces|adjacent_place_ids" crates/tracewake-core/src/scheduler.rs` shows no physical-map reads inside `eat_proposal`, `sleep_proposal`, or `work_or_move_proposal`; remaining hits are the actor-known workplace field name, the visible-local context builder, and test setup.
- `cargo test -p tracewake-core scheduler`
- `cargo test -p tracewake-content no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`
- `cargo test -p tracewake-content`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
