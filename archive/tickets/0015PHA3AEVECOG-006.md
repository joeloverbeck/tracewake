# 0015PHA3AEVECOG-006: Embodied food/sleep/route surfaces become context-backed

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — sealed `KnowledgeContext` extended with food/sleep-affordance/route-exit facts; `projections.rs` embodied helpers render context-backed facts only; guards extended; three adversarial fixtures
**Deps**: 0015PHA3AEVECOG-003

## Problem

ORD-HARD-010: the 0014 fix made workplaces context-backed but left food, sleep affordances, and routes deriving from raw state. In `crates/tracewake-core/src/projections.rs`: `actor_known_food_sources_for_context` takes the sealed `KnowledgeContext` but uses it only for a mode check, sourcing the food list from raw `state.food_supplies` filtered by physical visibility; `visible_open_sleep_affordance` takes no context at all and gates the embodied "sleep here" action on raw `access_open` truth; visible exits are built from raw `place.adjacent_place_ids` with no knowledge filter. The projection computes "what the actor knows" from authoritative truth inline, with no provenance and no seal — exactly the pattern 0014 ruled a violation for workplaces (INV-067, INV-069, INV-008, INV-099–102, INV-048 route knowledge, INV-093).

This ticket extends the sealed context packet with food, sleep-affordance, and route/exit facts populated through the same evented channels as ORD-HARD-008, and makes the embodied projection render only context-backed facts plus actor-visible action definitions. Debug projection may still compare context vs truth non-diegetically.

## Assumption Reassessment (2026-06-09)

1. Current code (verified): `actor_known_food_sources_for_context`, `visible_open_sleep_affordance`, and the exit-building path in `crates/tracewake-core/src/projections.rs` read raw `state.food_supplies` / `state.sleep_affordances()` / `adjacent_place_ids`; `KnowledgeContext` (`epistemics/knowledge_context.rs:237`) carries `actor_known_workplaces` (the 0014 fix) but no food/sleep/route equivalents. `SleepAffordanceState.access_open` is at `state.rs:353/361`. The existing guard `guard_014_embodied_projection_workplaces_are_context_backed` (`anti_regression_guards.rs:1353`) is the family to extend.
2. Specs/docs: spec 0015 §ORD-HARD-010 (required correction + structural lock); `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` embodied-affordance formula ("embodied affordance menus must be generated from the bound actor's visible/perceived/known context"); INV-067/069/008/048/093.
3. Shared boundary under audit: the sealed `KnowledgeContext` packet and the embodied projection helpers. This ticket consumes the evented food/sleep/route facts produced by the `-003` substrate (perception events for current-place visibility, memory/records for non-local knowledge), so it depends on the cutover. `anti_regression_guards.rs` is a **shared merge hub** (also 004/005/007/008; 009 rewrites).
4. INV-067 — embodied mode shows actor-known reality, hiding hidden truth. INV-069 — the TUI/view layer must not implement simulation rules or query hidden truth in embodied mode. INV-048 — route/travel knowledge is itself fallible/stale; exits offered must reflect perceived/known routes, not raw adjacency. INV-008 — UI assistance is not authority; an affordance menu must not reveal hidden truth.
5. Fail-closed / actor-knowledge / deterministic-replay surface: names the enforcement surface = the embodied projection helpers (`projections.rs`) and the `KnowledgeContext` packet. Confirm no epistemic-leakage: the embodied output omits any food/sleep/exit the context has no fact for, even when raw state contains it; debug projection may show the discrepancy non-diegetically (INV-068/107). No determinism change — projection is a pure function of the sealed context + actor-visible action defs.
6. Schema extension: `KnowledgeContext` gains `actor_known_food_sources`, `actor_known_sleep_affordances`, and `actor_known_routes/exits` (mirroring `actor_known_workplaces`). Consumers: the embodied projection helpers (`projections.rs`), `view_models.rs`, the context-hash inputs (`knowledge_context.rs` `canonical_hash_inputs`), and the TUI render that consumes the view models. Additive on the context (new fields); the projection helpers are **rewired** to read them. Versioned/hash-stable per INV-018.

## Architecture Check

1. Generating the embodied affordance menu from the sealed context (not raw state) is the doctrinal embodied-affordance formula; it closes the same leak 0014 closed for workplaces, now for the remaining three surfaces, with one consistent mechanism rather than three ad-hoc inline filters. Extending the existing `actor_known_workplaces` pattern keeps the context packet uniform.
2. No shims: the raw `state.food_supplies`/`sleep_affordances`/`adjacent_place_ids` reads are removed from the embodied helpers, not wrapped; debug comparison is a separate non-diegetic path, not a fallback.

## Verification Layers

1. INV-067/024 → replay/golden-fixture check (fixture `embodied_view_omits_unobserved_food_at_open_place_001`): raw state has food at an open place, context has no fact → embodied output omits it; debug output may show the discrepancy.
2. INV-067 → replay/golden-fixture check (fixture `embodied_view_omits_unknown_sleep_affordance_001`): unknown sleep affordance omitted from embodied "sleep here".
3. INV-048 → replay/golden-fixture check (fixture `embodied_exits_require_perceived_or_known_route_001`): exits require a perceived/known route, not raw adjacency.
4. INV-069/008 → codebase grep-proof (guard): `guard_014`-family extended so `state.food_supplies`, `state.sleep_affordances`, and `adjacent_place_ids` are banned in the embodied actor-known projection helpers.

## What to Change

### 1. Extend the sealed context packet

Add food, sleep-affordance, and route/exit fact fields to `KnowledgeContext`, populated from the evented channels (perception events for current-place visibility; memory/records for non-local), included in the canonical context-hash inputs.

### 2. Rewire embodied projection helpers

`actor_known_food_sources_for_context`, `visible_open_sleep_affordance`, and the exit builder render only context-backed facts plus actor-visible action definitions; remove the raw-state reads. Debug projection compares context vs truth non-diegetically.

### 3. Extend the guard family

Extend `guard_014_embodied_projection_*` to ban `state.food_supplies`, `state.sleep_affordances`, and `adjacent_place_ids` in the embodied helpers (all four surfaces now covered).

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify — context fields + hash inputs)
- `crates/tracewake-core/src/projections.rs` (modify — rewire embodied helpers)
- `crates/tracewake-core/src/view_models.rs` (modify — surface context-backed facts)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: also 004/005/007/008; 009 rewrites)
- `crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs` (new)
- `crates/tracewake-content/src/fixtures/embodied_view_omits_unknown_sleep_affordance_001.rs` (new)
- `crates/tracewake-content/src/fixtures/embodied_exits_require_perceived_or_known_route_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register the three fixtures)

## Out of Scope

- The evented channels themselves (`0015PHA3AEVECOG-001`/`-002`/`-003`); this ticket consumes them.
- TUI rendering changes beyond consuming the corrected view models (the view-model contract is the boundary).
- Content-authored tuning constants (`0015PHA3AEVECOG-008`).

## Acceptance Criteria

### Tests That Must Pass

1. The three adversarial fixtures pass: raw state contains the entity, context contains no fact, embodied output omits it, debug output may show the discrepancy.
2. Re-introducing a raw `state.food_supplies`/`sleep_affordances`/`adjacent_place_ids` read in an embodied helper fails the extended `guard_014` family.
3. `cargo test --workspace` green; golden checksum diffs explained.

### Invariants

1. The embodied affordance menu is a pure function of the sealed context + actor-visible action definitions; no raw-state read.
2. Debug projection remains the only path that may compare context to truth, non-diegetically.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs`, `…_unknown_sleep_affordance_001.rs`, `embodied_exits_require_perceived_or_known_route_001.rs`.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — extended `guard_014` family for the three surfaces.

### Commands

1. `cargo test -p tracewake-core projections:: && cargo test -p tracewake-content embodied_`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Extended sealed `KnowledgeContext` with actor-known food source, sleep affordance, and route facts, including deterministic sort/dedup and context-hash inputs.
- Rewired embodied projection food actions, sleep affordance actions, and visible exits to read only sealed context facts instead of raw `state.food_supplies`, `state.sleep_affordances`, or `adjacent_place_ids`.
- Added `current_place_knowledge_context` to derive sealed context facts from typed current-place perception events and used it for TUI view construction and proposal source-context validation.
- Added adversarial fixture registrations for omitted unobserved food, unknown sleep affordance, and unknown route exits.
- Extended regression guards and updated human-source tests to use the perception-backed context boundary.

Deviations from original plan:

- `crates/tracewake-core/src/view_models.rs` did not require direct edits; the existing view model shape already carried the corrected projection output.
- TUI and source-validation code needed narrow updates so human semantic action tokens use the same evented sealed context as the rendered view.

Verification:

- `cargo test -p tracewake-core projections::` passed.
- `cargo test -p tracewake-content` passed.
- `cargo test -p tracewake-core --test anti_regression_guards guard_014_embodied_projection_workplaces_are_context_backed` passed.
- `cargo test -p tracewake-core --test hidden_truth_gates embodied_affordances_exclude_hidden_food_in_closed_container` passed.
- `cargo test -p tracewake-tui --lib` passed.
- `cargo test -p tracewake-core actions::defs::continue_routine::tests::human_and_scheduler_continue_share_pipeline_result` passed.
- `cargo test -p tracewake-core --test acceptance_gates sleep_proposals_share_pipeline_across_human_and_nonhuman_origins` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
