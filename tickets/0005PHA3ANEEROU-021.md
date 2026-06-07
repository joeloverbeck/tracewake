# 0005PHA3ANEEROU-021: Canonical no_human_day_001 fixture

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the canonical `no_human_day_001` fixture (roster, homes, food, work, routes, routines, needs, day windows, expected roster + metrics envelope) and registers it.
**Deps**: 0005PHA3ANEEROU-016, 0005PHA3ANEEROU-017, 0005PHA3ANEEROU-018

## Problem

The canonical Phase 3A proof is a boring no-human ordinary day: a small roster wakes, experiences need changes, eats when possible, travels by the action pipeline, completes or fails a work block, returns/rests/sleeps, hits at least one interruption/replan and one physical blocker — with no player-identity events, no frozen unpossessed actor, no protagonist gravity, deterministic replay, and a debug-explainable day (Spec 0005 §10.4, §12; `INV-004`, `INV-024`). This is the phase's hard gate fixture.

## Assumption Reassessment (2026-06-07)

1. Fixtures register in `crates/tracewake-content/src/fixtures/mod.rs::all()` and run through `golden_fixtures_run.rs` and the core no-human runner (ticket 017); the metrics envelope comes from ticket 018. This ticket adds one fixture file following the same structural-consumer registration as tickets 019/020.
2. Spec §10.4 fixes the required content: actors `actor_tomas`, `actor_mara`, `actor_elena`, `actor_anna` (`actor_elias` optional only if cheap, no Phase 4 behavior); homes/home places; sleep places; a consumable food source; at least one missing/inaccessible/insufficient food source (the interruption proof); ≥1 workplace; a route/place graph for ordinary movement; routine assignments; initial needs; day windows; expected no-human actor roster; expected metrics envelope. Spec §10.5 fixes the actor roles (Tomas worker, Mara food-unavailable actor, Elena travel/return/sleep, Anna office/work-availability placeholder). Spec §10.4 fixes the minimum behaviors and the "no event references player identity / no actor freezes / no protagonist gravity / replay deterministic / debug explainable" guarantees.
3. Shared boundary under audit: this fixture registers in the shared `fixtures/mod.rs::all()` (with tickets 019/020) and is the one the capstone (ticket 025) runs end-to-end. Its expected-roster and expected-metrics envelopes are the acceptance anchors; the day must include the food-unavailable interruption and one physical blocker (Spec §10.4, §12).
4. Invariants motivating this ticket: `INV-004` — "The authoritative world ignores human existence" (no sacred player; no actor freezes unpossessed) — and `INV-024` — "No telepathy" (debug can explain the day without leaking hidden truth to actors). The roster must stay small (substrate proof, not population sim — Spec §10.5); the placeholders (Anna's office availability) stay inert and non-Phase-4.
5. No-human / determinism / no-leak surface: this fixture is the canonical no-human gate input. It authors only initial state + routines (possibility, `INV-061`); the day emerges from the engine, replays deterministically, and contains no player-conditioned facts. It adds no leakage and no nondeterminism; Mara's interruption is an emergent honest failure/replan, not a script.

## Architecture Check

1. A single canonical fixture authoring a small roster + ordinary infrastructure (and letting the runner produce the day from events) is the honest no-human proof — Spec §10.3/§10.4 forbid pre-scheduling actor locations, so the fixture provides possibility (homes/food/work/routes/routines/needs) and the engine produces the day. Keeping the roster tiny keeps it a substrate proof, not population simulation.
2. No backwards-compatibility shims: additive fixture + `mod.rs` registration; placeholders are inert and validated (ticket 016), not stubbed economy/institutions.

## Verification Layers

1. No-human world (`INV-004`) -> replay/golden-fixture check + grep-proof: the day runs with no bound controller; no emitted event references player/controller identity; no unpossessed actor freezes.
2. Honest interruption + blocker (§10.4/§12) -> replay/golden-fixture check: the day includes Mara's food-unavailable interruption/replan and ≥1 physical/access blocker diagnostic; need changes, meals, work, and sleep occur.
3. Deterministic + explainable (`INV-018`/`INV-024`) -> replay/hash check + manual review: replay reconstructs the same state/projections/metrics; debug can explain the day without leaking hidden truth to actors.

## What to Change

### 1. Canonical fixture

Add `crates/tracewake-content/src/fixtures/no_human_day_001.rs` authoring the §10.4 roster (Tomas/Mara/Elena/Anna, roles per §10.5), homes/sleep places, a consumable food source plus a missing/inaccessible one (Mara's interruption), ≥1 workplace, a route/place graph, routine assignments, initial needs, the §10.2 day windows, the expected no-human actor roster, and the expected metrics envelope (ticket 018).

### 2. Registration + run wiring

Declare the module in `crates/tracewake-content/src/fixtures/mod.rs` and add to `all()`; wire it into `crates/tracewake-content/tests/golden_fixtures_run.rs` for load/validation and into the core no-human run path used by the capstone.

## Files to Touch

- `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — declare + register `no_human_day_001` in `all()`)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — load/validate + minimum-behavior assertions)

## Out of Scope

- The full acceptance-gate run + replay/metrics assertions over this fixture (ticket 025) — this ticket authors and minimally validates it.
- Stretch fixtures `missing_property_setup_no_human_001` / `routine_opportunity_item_move_001` (Spec §17.2 — non-blocking stretch, deferred to Phase 3B).
- Core runner/metrics logic (tickets 017, 018).
- TUI/debug rendering (tickets 022, 023).

## Acceptance Criteria

### Tests That Must Pass

1. `no_human_day_001` loads, validates (ticket 016), and runs with no bound controller; no emitted event references player/controller identity; no unpossessed actor freezes.
2. The day includes need changes, ≥1 meal completed or explicitly missed, ≥1 sleep completed or intentionally active at end-of-day, ≥1 work block completed or failed for a modeled reason, Mara's food-unavailable interruption/replan, and ≥1 physical/access blocker diagnostic.
3. The expected-roster and expected-metrics envelopes match the run; movement is via the action pipeline (no teleport).

### Invariants

1. The day runs with no human controller and no player-conditioned facts; no protagonist gravity (`INV-004`, Spec §3.2).
2. The day is deterministic and debug-explainable without leaking hidden truth (`INV-018`, `INV-024`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify) — `no_human_day_001` load/validate + minimum-behavior assertions.

### Commands

1. `cargo test -p tracewake-content golden_fixtures_run`
2. `cargo test -p tracewake-content`
3. Content-crate load/validate scope is correct here; the end-to-end no-human run + replay/metrics gate is the capstone (025), which runs this fixture through the core runner.
