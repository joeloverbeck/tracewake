# 0005PHA3ANEEROU-009: Food entity model and eat action

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a finite food-supply entity to core state and an `eat` action that consumes modeled food eventfully and reduces hunger.
**Deps**: 0005PHA3ANEEROU-005, 0005PHA3ANEEROU-007

## Problem

Phase 3A eating must consume world-modeled food (or a finite food service) and reduce hunger only through an eventful consumption — never an abstract "hunger refill" command — and must fail honestly when no accessible food exists (Spec 0005 §9.7, §16.2, §21; `INV-045`, `INV-046`). The planner may choose a believed food location; the validator reveals absence/inaccessibility by rejection/failure and observation ancestry — it must never read hidden true food locations (Spec §9.7, §3.3). This is the canonical interruption substrate Mara's food-unavailable proof (ticket 020) depends on.

## Assumption Reassessment (2026-06-07)

1. Physical entity state (actors, places, doors, containers, items) lives in `crates/tracewake-core/src/state.rs`; the existing item/container model and `check_container` (Phase 2A, `actions/defs/checkcontainer.rs`) already create observations and absence/contradiction evidence (Spec §4.1). This ticket adds a `food_supply` entity (or reuses item+servings) using the existing place/container/access machinery where possible, and the eat action reuses Phase 2A observation ancestry on failure.
2. The need model (001), food-consumption/eat-failed event kinds (005), and hunger-delta machinery (007) provide the reduction path. Spec §16.2 fixes the food model: stable ID, location, integer servings, deterministic hunger reduction per serving, access/visibility via existing place/container/action machinery, consumption decrements servings or marks consumed by event, zero servings distinguishable from hidden/unknown food.
3. Shared boundary under audit: the `food_supply` schema fields fixed here are mirrored by the content schema (ticket 015) and validated (ticket 016); the eat action is consumed by the `EatMeal`/`FindFood` routines (ticket 013) and the food-unavailable fixture (ticket 020). The serving/hunger-reduction contract is fixed here.
4. Invariants motivating this ticket: `INV-045` — "fake meter refills disconnected from world state are forbidden" (hunger falls only through real food consumption) — and `INV-046` — "V1 economy is simplified but not fake" (model food and ordinary stock/service as causal actions without macroeconomics). No prices/markets/cooking/perishability/nutrition (Spec §9.7 explicit exclusions).
5. Actor-knowledge / replay surface: the planner selecting a believed food location, with the validator revealing absence, is the actor-known boundary (`INV-024`, Spec §3.3). This ticket adds no leakage — `eat` validates against authoritative physical truth at proposal time (validator may check truth to accept/reject; the planner may not, ticket 014) and emits observation ancestry on failure (reusing Phase 2A). Consumption events feed replay (ticket 006). Zero-servings is an eventful, observable state distinct from unknown food.

## Architecture Check

1. Modeling food as a finite serving-bearing entity consumed by an eventful `eat` (rather than a hunger-refill verb) makes "no fake refill" structurally true and gives the food-unavailable interruption real causal ancestry (`INV-045`, Spec §12). Reusing the existing container/access/observation machinery (rather than a bespoke food-access path) keeps eat's access checks consistent with `check_container`/`open` and reuses Phase 2A absence evidence.
2. No backwards-compatibility shims: food-supply extends the existing entity state additively; eat is a new registry action; no parallel hunger meter.

## Verification Layers

1. Causal eating / no fake refill (`INV-045`) -> unit test: `eat` reduces hunger only by emitting a consumption event that decrements servings; with zero accessible servings, hunger is unchanged and an `eat failed` event with `resource`/`access` ancestry is emitted.
2. Actor-known boundary (`INV-024`; Spec §3.3) -> integration test: `eat` against a believed-but-absent food source fails with observation/absence ancestry; the failure path does not consult hidden true food location (validator checks authoritative truth only to reject).
3. Simplified-but-real economy (`INV-046`) -> manual review + grep-proof: food carries servings/location only — no price/market/nutrition field.

## What to Change

### 1. Food-supply entity

Extend `crates/tracewake-core/src/state.rs` with a `food_supply` entity: stable ID, location (place/container), integer servings, per-serving hunger reduction. Access/visibility reuse existing place/container rules; zero servings is a distinct observable state.

### 2. Eat action def

Add `crates/tracewake-core/src/actions/defs/eat.rs` implementing §9.7 modeling: select an actor-known/visible/believed food source (handle passed in by caller; the validator resolves authoritative access), validate location/access/permission, consume one or more servings via an eventful consumption event reducing hunger (ticket 007 deltas), decrement servings or mark consumed, and fail with a typed `resource`/`access`/`knowledge` blocker + observation ancestry when no accessible food exists.

### 3. Registry + module registration

Register `eat` via the Phase 3A registration fn in `crates/tracewake-core/src/actions/registry.rs` and declare it in `crates/tracewake-core/src/actions/defs/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/eat.rs` (new)
- `crates/tracewake-core/src/state.rs` (modify — add `food_supply` entity)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — declare `eat`)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register Phase 3A `eat`)

## Out of Scope

- Food *service* entity beyond the finite-store model (kept minimal; only add if a fixture needs it — Spec §16.2 service is optional and non-economic).
- `EatMeal`/`FindFood` routine method selection and the planner choosing the believed source (tickets 013, 014).
- Content food schema and validation (tickets 015, 016).
- The Mara food-unavailable golden fixture (ticket 020).

## Acceptance Criteria

### Tests That Must Pass

1. `eat` against an accessible food supply with servings emits a consumption event, decrements servings, and reduces hunger by the per-serving amount; servings reaching zero is observable.
2. `eat` against an absent/empty/inaccessible food source fails with a typed `resource`/`access`/`knowledge` blocker and observation/absence ancestry; hunger is unchanged.
3. A grep-proof confirms the food entity carries no price/market/nutrition field.

### Invariants

1. Hunger falls only via eventful food consumption tied to a decremented serving (`INV-045`).
2. Eat never reads hidden true food location to succeed; absence is revealed by rejection + observation ancestry (`INV-024`, Spec §3.3).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/eat.rs` (unit tests) — consume/decrement/reduce, empty/absent/inaccessible failure with ancestry.
2. `crates/tracewake-core/src/state.rs` (unit tests) — food-supply servings/zero-state modeling.

### Commands

1. `cargo test -p tracewake-core actions::defs::eat`
2. `cargo test -p tracewake-core state`
3. Core-crate scope is correct; the end-to-end food-unavailable interruption chain is exercised by tickets 020/021/025.

## Outcome

Completed: 2026-06-07

Changed:
- Added `FoodSupplyId` and finite `FoodSupplyState` to physical state with stable location, servings, and per-serving hunger reduction.
- Added `eat` as a Phase 3A registry action that emits `FoodConsumed` plus a caused hunger `NeedDeltaApplied` event for accessible servings.
- Made `FoodConsumed` a physical world event so replay/application decrements servings deterministically.
- Added `EatFailed` event paths for absent, empty, or inaccessible food sources with typed `knowledge`, `resource`, or `access` blockers and absence/observation ancestry payload fields.
- Included food supplies in physical checksums and replay diffs.

Deviations:
- Food service, planner source selection, content schemas, and food-unavailable golden fixtures remain deferred to their scoped follow-up tickets.
- Failure events are committed as eventful failed action outcomes rather than unstructured hunger changes; actor hunger is unchanged unless a consumption event is emitted.

Verification:
- `cargo fmt --all --check`
- `cargo test -p tracewake-core actions::defs::eat`
- `cargo test -p tracewake-core state`
- `cargo test -p tracewake-core actions`
- `cargo test -p tracewake-core`
- `rg -n "price|market|nutrition" crates/tracewake-core/src/state.rs crates/tracewake-core/src/actions/defs/eat.rs` returned no matches.
