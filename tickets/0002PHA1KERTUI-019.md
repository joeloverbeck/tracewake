# 0002PHA1KERTUI-019: Seven golden fixture definitions

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `fixtures` module and the seven golden fixtures to `tracewake-content`.
**Deps**: 0002PHA1KERTUI-017, 0002PHA1KERTUI-018, 0002PHA1KERTUI-009, 0002PHA1KERTUI-010, 0002PHA1KERTUI-011

## Problem

The executable Phase 1 fixture suite must include all seven fixture IDs, each with its own explicit scenario contract, expected actions, expected events/reports, and acceptance assertions (Spec 0002 §4.2, §18). This ticket adds the fixture *definitions* (entities, setup, scenario contracts) for `strongbox_001`, `container_item_move_001`, `door_access_001`, `debug_attach_001`, `no_human_advance_001`, `replay_item_location_001`, `view_model_local_actions_001`. The scenario *tests* that execute them end-to-end are the capstone (ticket 022).

## Assumption Reassessment (2026-06-06)

1. The content schema/load/validator exist from tickets 017/018; the physical action verbs exist from tickets 009–011. This adds `crates/tracewake-content/src/fixtures/` with one definition per fixture, each registered in the fixtures module (a structural consumer model — registration is the wiring).
2. The seven fixtures and their per-fixture contracts are `specs/0002_…_SPEC.md` §18.1–§18.7; the inherited `strongbox_001` baseline (entities `actor_tomas`, `actor_elena`, `strongbox_tomas`, `coin_stack_01`, `house_tomas`) is Spec 0001 §`strongbox_001` (verified at `docs/4-specs/0001_…FIXTURE_CONTRACTS.md` line 627 during this session's reassessment). Spec 0002 §18.1 "preferably `actor_elena`" matches Spec 0001's fixture roster.
3. Shared boundary under audit: fixture IDs and entity names that must stay stable across Spec 0001 and the replay/why-not examples in `docs/2-execution/05_…REPLAY.md` (which uses `actor_mara` illustratively; Spec 0002 fixtures follow Spec 0001's `strongbox_001` roster). Each fixture must pass the ticket-018 validator.
4. Invariant motivating this ticket: INV-061 (authored causal machinery is required — designers author fixtures as possibility space) and INV-062 (scenario seeds are tensions, not scripts) — fixtures define initial state and permitted affordances only, never authored outcomes.
5. No-scripting + deterministic surface: each fixture is typed initial state + permitted affordances validated by ticket 018; none carries a behavior-looking field or authored outcome chain (it would be rejected). `strongbox_001` proves physical state/replay only — no belief contradiction (§8.3, §18.1 negative assertions). Fixtures load deterministically (ticket 017). Enforcement that each fixture validates and replays is ticket 022.

## Architecture Check

1. Fixtures as validated content data (passing the ticket-018 gate) rather than imperative setup code keeps "content is possibility" structural — a fixture cannot smuggle in a script because the validator rejects behavior-looking fields. Sharing underlying content across fixtures where useful (§4.2) while keeping each fixture's own scenario contract keeps the suite small but explicit.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. No-scripting (INV-060/061) -> schema validation: each fixture passes the ticket-018 validator (no forbidden forms, no authored outcomes).
2. Spec-0001 roster fidelity (§18.1) -> codebase grep-proof: `strongbox_001` uses `actor_tomas`/`actor_elena`/`strongbox_tomas`/`coin_stack_01`/`house_tomas`, matching `docs/4-specs/0001_…FIXTURE_CONTRACTS.md`.
3. Fixture-contract completeness (§13.1 phase 13) -> schema validation: each fixture declares setup, allowed actions, expected events/reports, and acceptance assertions.

## What to Change

### 1. Fixture module

Add `crates/tracewake-content/src/fixtures/mod.rs` plus one file per fixture (`strongbox_001.rs`, `container_item_move_001.rs`, `door_access_001.rs`, `debug_attach_001.rs`, `no_human_advance_001.rs`, `replay_item_location_001.rs`, `view_model_local_actions_001.rs`), each defining entities, setup, and the §18 scenario contract.

### 2. Registration

Register all seven in `crates/tracewake-content/src/lib.rs` (declare the `fixtures` module).

## Files to Touch

- `crates/tracewake-content/src/fixtures/mod.rs` (new)
- `crates/tracewake-content/src/fixtures/strongbox_001.rs` (new)
- `crates/tracewake-content/src/fixtures/container_item_move_001.rs` (new)
- `crates/tracewake-content/src/fixtures/door_access_001.rs` (new)
- `crates/tracewake-content/src/fixtures/debug_attach_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_advance_001.rs` (new)
- `crates/tracewake-content/src/fixtures/replay_item_location_001.rs` (new)
- `crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs` (new)
- `crates/tracewake-content/src/lib.rs` (modify — declare `fixtures`; file created by ticket 017)

## Out of Scope

- The end-to-end scenario tests that run each fixture's action path and assert events/replay (ticket 022).
- The TUI's loading of a fixture (ticket 020).
- Negative-validation fixtures (live as test data in ticket 018/022).

## Acceptance Criteria

### Tests That Must Pass

1. All seven fixtures load deterministically and pass the ticket-018 content validator.
2. `strongbox_001` uses the Spec-0001 roster (`actor_tomas`, `actor_elena`, `strongbox_tomas`, `coin_stack_01`, `house_tomas`) and places `coin_stack_01` initially in `strongbox_tomas` as a physical item, not an abstract balance.
3. Each fixture declares setup, allowed actions, expected events/reports, and acceptance assertions.

### Invariants

1. No fixture carries a behavior-looking field or authored outcome chain.
2. Fixture IDs and the `strongbox_001` roster stay consistent with Spec 0001.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — all seven load + validate cleanly; `strongbox_001` roster/identity assertions.

### Commands

1. `cargo test -p tracewake-content fixtures_load`
2. `cargo build --workspace`
3. Content-crate load/validate scope is correct here; the executable scenario assertions (events, replay, why-not, debug) run in ticket 022.
