# 0002TUIPROOSUR-003: Actor-visible affordances derived only from the sealed context

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (affordance / semantic-action generation in `projections.rs`)
**Deps**: 0002TUIPROOSUR-001, 0002TUIPROOSUR-002

## Problem

Actor-visible action lists are generated with direct access to raw truth: Phase 3A action construction walks `state.food_supplies`, containers, `state.workplaces`, and `agent_state.active_intention_by_actor` (`crates/tracewake-core/src/projections.rs:500-590`), then relies on later validation/visual filtering. Spec 0002 §4 TUI-AC-002 requires that the actor-visible `SemanticActionEntry` list be computed from sealed-context facts plus action-definition preflight that does not reveal forbidden truth — raw-state validation may still *reject* a proposal, but it may not be the *source* of actor-visible target discovery. The risk: an action ID, label, disabled reason, or target ID for hidden food/route appears in the embodied view even though the actor has no source-bound knowledge of it (INV-100, INV-008).

## Assumption Reassessment (2026-06-08)

1. Phase 3A affordance/action generation reads raw state at `crates/tracewake-core/src/projections.rs:500-590` (food supplies, containers, workplaces, `active_intention_by_actor`). After ticket 002 the embodied builder receives the sealed packet from 001; this ticket makes the affordance sub-builder derive candidates from that packet plus a registry view filtered to known/reachable candidates (`crates/tracewake-core/src/actions/registry.rs`).
2. Spec 0002 §4 TUI-AC-002 requires affordances source-bound to sealed-context facts; its acceptance test is a hidden-food fixture proving no action/label/disabled-reason/target for hidden food appears unless the actor-known context contains a valid source-bound observation/record.
3. Cross-artifact boundary under audit: the affordance builder (`projections.rs`) ↔ the sealed context (001) and the action registry view (`actions/registry.rs`). The registry view handed to actor-visible discovery must be the known/reachable-filtered subset, not the full truth-aware registry.
4. Invariants restated: **INV-100** — no hidden world state (true item locations, true routes) may enter cognition/affordance input; **INV-008** — UI/affordance assistance may not reveal hidden truth in embodied mode; **INV-070** — disabled actions explain missing preconditions in actor-known terms only.
5. Fail-closed / no-leak surface: this IS the actor-visible target-discovery enforcement point. Confirm that after the change, hidden-food/hidden-route candidates are absent from the embodied affordance list unless source-bound, and that raw-state preflight is used only to classify availability of an already-actor-known candidate, never to surface a new one. Preflight rejection reasons remain debug-only until 005/007 type them; this ticket must not leak hidden truth into the actor-visible entry.

## Architecture Check

1. Deriving actor-visible candidates from the sealed context (then preflighting them) is cleaner and leak-proof by construction versus "generate from truth, filter visually": a target the actor cannot know is never enumerated, so there is no filtered-but-present leak path. It also localizes Phase 3A's TUI boundary without re-specifying ordinary-life mechanics.
2. No backwards-compatibility aliasing/shims: the raw-state candidate enumeration is replaced, not kept behind a flag; truth-aware preflight remains only as a rejection classifier, not a discovery source.

## Verification Layers

1. INV-100 (no hidden-truth cognition) -> replay/golden-fixture check: a hidden-food fixture yields an embodied affordance list with no hidden-food action/target id.
2. INV-008 (UI not authority) -> manual review: actor-visible discovery reads only the sealed context + filtered registry; raw state is touched only to classify availability of an already-known candidate.
3. INV-070 (why-not in actor terms) -> codebase grep-proof: disabled entries carry no hidden-target identifiers (full typing lands in 005/007; this ticket proves no hidden id leaks).

## What to Change

### 1. Affordance source = sealed context

Refactor the Phase 3A affordance/semantic-action sub-builder so candidate discovery iterates sealed-context facts (actor-known observations/records/beliefs) and a known/reachable-filtered registry view, not raw `state.food_supplies`/containers/`workplaces`/`active_intention_by_actor`.

### 2. Preflight as classifier only

Where truth-aware preflight remains, restrict it to classifying the availability of an already-actor-known candidate; it may not introduce a new actor-visible target.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)

## Out of Scope

- Typed `ActionAvailability`/reason-code schema (ticket 005) — this ticket keeps existing entry shape and only fixes the discovery source.
- Proposal context fields / validation ordering (ticket 006).
- Re-specifying ordinary-life (hunger/work/routine) mechanics (Spec 0002 §2.2 non-goal).

## Acceptance Criteria

### Tests That Must Pass

1. `hidden_food_closed_container_001` (and `hidden_food_unknown_route_001`) prove no action id, label, disabled reason, or target id for hidden food/route appears in the embodied affordance list absent a source-bound actor-known justification.
2. A positive fixture proves a source-bound, reachable target DOES surface as an affordance.
3. `cargo test -p tracewake-core` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Actor-visible target discovery is a function of the sealed context only (INV-100/INV-008).
2. Raw truth-aware preflight never adds an actor-visible candidate (INV-008).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — extend with affordance-discovery assertions over `hidden_food_*` fixtures.
2. `crates/tracewake-core/src/projections.rs` (unit tests) — affordance sub-builder consumes sealed context.

### Commands

1. `cargo test -p tracewake-core hidden_truth_gates`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
3. The hidden-truth gate test is the correct boundary because it directly exercises the leak the requirement forbids.

## Outcome

Completed: 2026-06-08

What changed:
- Added actor-visible Phase 3A candidate lists to `EmbodiedProjectionSource`.
- Changed Phase 3A affordance construction to iterate actor-known food/workplace candidates instead of enumerating all physical food/workplace records.
- Kept truth-aware validation/preflight as availability classification for already surfaced candidates.
- Added a hidden-truth integration gate proving closed-container hidden food does not appear in semantic action IDs, labels, targets, or disabled reasons, while a visible local food source still appears.

Deviations from original plan:
- The current route-knowledge model does not yet expose a separate sealed route-fact packet, so this ticket scopes the implementation to the Phase 3A food/work/routine target-discovery seam named by the ticket. Hidden-route planning remains covered by the existing typed planning gate.

Verification results:
- `cargo test -p tracewake-core --test hidden_truth_gates` passed.
- `cargo test -p tracewake-core projections::tests::view_models_embodied_phase3a_status_is_viewer_scoped_and_actor_known` passed.
- `cargo test -p tracewake-tui --test embodied_flow` passed.
- `cargo test -p tracewake-core` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
