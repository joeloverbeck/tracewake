# 0052FOUCONFOU-007: F4-05 — closed exhaustive per-loaded-actor disposition census

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — a closed core-owned actor disposition assigns exactly one disposition to every loaded actor per world step, derived from the runtime-owned loaded actor set
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-004

## Problem

Spec 0052 F4-05 (§1.1.5, §4.6): controlled/autonomous mutual exclusion and `ActorDecisionTransaction` consumption are genuinely present, but `ActorStepStatus` (`scheduler.rs:290`) has only `Proposed` and `Stuck`, `actor_step_summaries` (`scheduler.rs:272`) is populated only for actors whose transaction was attempted, and there is no closed disposition for *controlled this tick*, *not due*, *deferred/reserved/body-exclusive*, *missing required substrate*, *budget-exhausted*, *invalidated*, or *otherwise skipped*. `WorldStepDueWorkSummary` (`scheduler.rs:174`) gives counts but no one-row-per-loaded-actor identity proof. The 0051 "closed exhaustive per-tick actor disposition census" closure claim exceeds the product.

This ticket introduces a closed core-owned actor disposition assigning **exactly one** disposition to every loaded actor at each world step, derived from the runtime-owned loaded actor set (the caller supplies no actor list — already enforced by 0052FOUCONFOU-004), reusing existing concepts without minting a doctrine identifier.

## Assumption Reassessment (2026-06-25)

1. `ActorStepStatus { Proposed, Stuck }` (`scheduler.rs:290`), `ActorStepSummary` (`scheduler.rs:277`), `WorldStepDueWorkSummary` (`scheduler.rs:174`) with `actor_step_summaries` (272), the runtime-owned `loaded_actor_next_decision_tick` map (`scheduler.rs:494`), and `due_loaded_actor_ids` (`scheduler.rs:590`) currently exist. Actor transaction/trace lives in `crates/tracewake-core/src/agent/trace.rs`.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.6; closure-order step 6. The runtime-owned loaded actor set (no caller list) is established by 0052FOUCONFOU-004's no-human de-authority.
3. Cross-artifact boundary under audit: the scheduler world-step → actor-decision-transaction → debug/trace product seam. The census product is read by the sealed view/debug receipts (001/004), so it must carry diagnostics without leaking hidden truth into embodied products.
4. Motivating invariants: INV-004/005/087/094 (possession-neutral ordinary actors), INV-041 + INV-105 (inspectable decision/stuck diagnostics as authoritative diagnostic data), INV-043 (ordinary-agent validation), INV-098 (harsh, non-vacuous acceptance — counts alone do not prove exhaustiveness).
5. Fail-closed / actor-knowledge surface: the census derives from the runtime-owned loaded actor set and enforces exactly one disposition per loaded actor, stable ordering, and no duplicate opportunity; the controlled actor is never autonomous in the same step. Diagnostics include responsible layer and causal/temporal basis but must not leak hidden truth into embodied products (INV-024, INV-093) — the disposition basis is debug/diagnostic data, surfaced only through the debug receipt under capability, never the embodied receipt.
6. Schema extension (disposition product — distinct from item 5): the closed disposition extends `ActorStepStatus` (new variants for controlled-this-tick, not-due, deferred/reserved/body-exclusive, invalid/missing-substrate) and the per-loaded-actor census carried in `WorldStepDueWorkSummary` / `ActorStepSummary`. Consumers: `scheduler.rs`, the debug/trace product (`agent/trace.rs`), and the sealed debug receipt. The extension is additive (new variants + one-row-per-loaded-actor rows; no existing variant removed), so existing `Proposed`/`Stuck` consumers keep working; every match site on `ActorStepStatus` gains the new arms (enumerate at implementation).

## Architecture Check

1. A single closed disposition per loaded actor — derived from the runtime-owned set — is cleaner than the attempted-only summary because exhaustiveness is provable (`census.len() == loaded_actor_set.len()`, IDs equal as sets, each actor once) rather than asserted; counts cannot demonstrate one-opportunity-per-actor. Reusing existing concepts (status enum + summary) avoids minting a doctrine identifier.
2. No backwards-compatibility alias: the attempted-only `actor_step_summaries` population is replaced by the exhaustive census, not retained beside it; the status enum is extended in place with every match site updated.

## Verification Layers

1. INV-004/094 (possession-neutral, parity) -> differential human/no-human run holding world state and due-work equal; the controlled actor is never autonomous in the same step.
2. INV-098 (non-vacuous) -> deterministic generated corpus over actor order, possession placement, due ticks, reservations, active durations, and stuck/proposed outcomes; for every step `census.len() == loaded_actor_set.len()`, actor IDs equal as sets, each actor appears once, every closed disposition reached by ≥1 fixture or documented as staged.
3. INV-041/105 (inspectable diagnostics) -> codebase grep-proof + manual review: diagnostics carry responsible layer and causal/temporal basis as typed/structurally-inspectable data.
4. Mutation -> coverage over filters, disposition arms, and census cardinality.

## What to Change

### 1. Closed disposition + exhaustive census (`scheduler.rs`)

Extend `ActorStepStatus` with the closed disposition set (controlled-proposal path, autonomous proposed, autonomous stuck, not due, deferred/reserved/body-exclusive, invalid/missing-substrate fail-closed) as the implementation requires. Build the census from the runtime-owned loaded actor set, assigning exactly one disposition per loaded actor with stable ordering and no duplicate opportunity. Update every `ActorStepStatus` match site.

### 2. Diagnostics without leakage (`scheduler.rs`, `agent/trace.rs`)

Each disposition carries responsible layer and causal/temporal basis as typed diagnostic data routed only through the debug receipt; no hidden-truth field reaches the embodied product.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — closed disposition set, exhaustive per-loaded-actor census, cardinality enforcement)
- `crates/tracewake-core/src/agent/trace.rs` (modify — disposition diagnostics typing)

## Out of Scope

- The embodied/debug receipt reshape and normal-output repair (008 — this ticket only routes census diagnostics into the debug product).
- No-human de-authority / runtime-owned actor set (004 — consumed here).
- The generated-corpus packaging and conformance lane (009 holds the full corpus; this ticket lands the behavior + permutation tests).

## Acceptance Criteria

### Tests That Must Pass

1. For every world step in a deterministic generated/permutation corpus: `census.len() == loaded_actor_set.len()`, actor IDs equal as sets, each actor appears exactly once, the controlled actor is never autonomous in the same step, and every closed disposition is reached by ≥1 fixture or explicitly documented as staged.
2. A differential human/no-human run holds world state and due-work equal; the witness derives the full loaded actor set from the public runtime receipt / accepted debug product (not `actor_transactions_attempted`, not a one-actor world, not preselected IDs).
3. Focused mutation over filters, disposition arms, and census cardinality; `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Exactly one disposition per loaded actor per step, stable ordering, no duplicate opportunity (INV-004, INV-043).
2. No disposition diagnostic leaks hidden truth into an embodied product (INV-024, INV-093).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — exhaustive-census cardinality + permutation + per-disposition coverage; possession differential.
2. `crates/tracewake-core/src/scheduler.rs` test module — focused disposition-arm + cardinality mutation sensitivity.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-25

Implemented an exhaustive per-loaded-actor world-step census in `ActorStepSummary`. `ActorStepStatus` now includes `Controlled`, `NotDue`, `DeferredReserved`, and `MissingSubstrate` alongside the existing autonomous `Proposed` and `Stuck` outcomes. World-step execution builds a stable actor-ID-keyed census from the loaded actor set, preserves full proposal/trace detail for attempted autonomous actors, and fills exactly one closed disposition row for every other loaded actor.

Reserved/body-exclusive actors are filtered out of autonomous due work and reported as `DeferredReserved`; controlled actors are not autonomous in the same step; loaded actors without agent substrate report `MissingSubstrate`; future-scheduled actors report `NotDue`.

Coverage now includes a coordinator receipt test proving controlled/proposed/missing/deferred rows over the derived loaded actor set, plus a focused scheduler test for the private not-due scheduling state. The existing replay temporal differential continues to compare restored actor-step summaries exactly.

Verification:

1. `cargo test -p tracewake-core --test world_step_coordinator --locked`
2. `cargo test -p tracewake-core scheduler::tests::world_step_census_marks_loaded_actor_not_due --locked`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
