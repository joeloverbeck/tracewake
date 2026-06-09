# 0015PHA3AEVECOG-002: Modeled current-place perception pass emitting observation events

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — a deterministic perception pass in `tracewake-core` appending observation events (reusing the Phase 2A `ObservationRecorded` machinery) keyed to the decision-window ordering key
**Deps**: None

## Problem

ORD-HARD-008's second channel half (spec required-correction item 2): current-place perception of visible food, visible sleep affordance, and visible exits must run as a **modeled perception pass that appends observation events** before or within the decision window — not be reverse-engineered from raw tables at decision time. Today `observe_visible_food_sources_from_current_place` scans raw `state.food_supplies()` filtered by place and mints a fact, but no `ObservationRecorded` event is appended, so the "observation" leaves no trace and cannot be replayed from the log (INV-016, INV-024, INV-018).

This ticket adds the perception pass and is **additive and unwired**: it produces the observation events the `0015PHA3AEVECOG-003` cutover will consume; the surface builder still reads raw tables until then. It runs in parallel with `0015PHA3AEVECOG-001` (seed knowledge) — together they are the two evented channels the cutover unifies on.

## Assumption Reassessment (2026-06-09)

1. Current code: `observe_visible_food_sources_from_current_place` in `crates/tracewake-core/src/agent/no_human_surface.rs` scans raw `state.food_supplies()` and mints facts with no event (verified). The Phase 2A observation machinery already exists: `EventKind::ObservationRecorded` (variant `crates/tracewake-core/src/events/envelope.rs:98`, string `observation_recorded` :308, apply arm `events/apply.rs:207`) with `ObservationRecordedPayload` (envelope.rs:453), exercised by the existing fixture `expectation_contradiction_001`. The scheduler's no-human window machinery (`build_agent_proposal`, the decision-window ordering key) lives in `crates/tracewake-core/src/scheduler.rs`.
2. Specs/docs: spec 0015 §ORD-HARD-008 "Required correction" item 2; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-024 (information reaches actors only through modeled perception), INV-016 (absence becomes evidence only through a channel); `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` for the observation contract.
3. Shared boundary under audit: the scheduler→decision-window seam and the observation event surface. The perception pass must be invoked by the scheduler at a deterministic point relative to the decision window and append events through the ordinary event pipeline — not construct cognition input directly (INV-103: the scheduler stays a time/dispatch authority, not a cognition author; it triggers a modeled perception that itself emits events).
4. INV-024 — no telepathy; current-place visibility must be a modeled perception channel. INV-016 — a missing/absent item becomes evidence only through expectation, perception, instruction, or search; perception is the channel here. INV-018 — the perception pass must be deterministic and replayable.
5. Deterministic-replay / actor-knowledge surface (substrate-only — enforcement deferred to siblings): the observation events are the inputs `0015PHA3AEVECOG-003`'s sealed builder consumes and `0015PHA3AEVECOG-004`'s rebuild-from-log byte-match gate enforces. Confirm here: (a) the pass is keyed to the window ordering key so replay reproduces identical observation events in identical order; (b) it observes only what is physically perceivable at the actor's current place (no hidden-truth read), so it introduces no leakage path. The pass reads authoritative `PhysicalState` to determine *what is perceivable* — this is permitted because its **output is an event**, not a cognition fact; the firewall forbids raw-table reads feeding cognition directly (003), not perception emitting events.
6. Schema extension: reuses the existing `ObservationRecorded` event schema (no new kind unless current-place food/sleep/exit visibility needs a payload variant). If a payload field is added, it is additive-only and versioned (INV-020); name the consumers (`events/apply.rs`, `checksum.rs`, the projection that reads observations). If no schema change is needed, this item is satisfied by reuse — confirm during implementation.

## Architecture Check

1. Perception-as-events is the doctrinal channel: a logged observation is replayable and provenance-bearing, where the current inline raw scan is not. Emitting events from a scheduler-triggered modeled pass (rather than letting the scheduler mint cognition facts) keeps the scheduler a non-cognition authority (INV-103).
2. No shims: the perception pass is added alongside the raw scan; the raw `state.food_supplies()` read in the surface builder is removed in 003, not wrapped.

## Verification Layers

1. INV-024 → replay/golden-fixture check: a fixture where food/sleep/exits are present at the current place produces `ObservationRecorded` events in the log; one where they are not present produces none.
2. INV-018 → replay/golden-fixture check: re-running the fixture yields byte-identical observation events in identical order (window-keyed).
3. INV-103 → codebase grep-proof: the perception pass is invoked from the scheduler window but emits events through the pipeline; the scheduler does not itself construct the resulting facts (that path is removed in 003).
4. INV-016 (substrate-only) → manual review: absence (no food at place) yields no observation, never a negative fact; downstream "actor does not know of food" is derived from event absence, enforced by sibling 003/004.

## What to Change

### 1. Deterministic perception pass

Add a modeled perception pass (core) that, for the actor at its current place, observes visible food supplies, visible sleep affordance(s), and visible exits, appending `ObservationRecorded` events (with a payload variant if needed) keyed to the decision-window ordering key.

### 2. Scheduler invocation

Invoke the perception pass from the no-human window in `scheduler.rs` at a deterministic point before the decision transaction, so the observation events precede the decision. The scheduler triggers; the pass emits.

## Files to Touch

- `crates/tracewake-core/src/agent/` (new perception-pass module, or extend an existing observation module — confirm placement against the Phase 2A observation code)
- `crates/tracewake-core/src/scheduler.rs` (modify — invoke the pass in the window)
- `crates/tracewake-core/src/events/envelope.rs` (modify — only if an observation payload variant is required)
- `crates/tracewake-core/src/events/apply.rs` (modify — only if a payload variant is added)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — checksum updates, explained)

## Out of Scope

- Building the surface from observation events / removing raw-table reads (`0015PHA3AEVECOG-003`).
- Seed-time knowledge events (`0015PHA3AEVECOG-001`).
- Embodied projection consuming context-backed perception (`0015PHA3AEVECOG-006`).
- Source guards and negative fixtures (`0015PHA3AEVECOG-004`).

## Acceptance Criteria

### Tests That Must Pass

1. A fixture with food/sleep/exits present at the actor's current place produces the corresponding `ObservationRecorded` events in the log, keyed to the window ordering key.
2. A fixture with none present produces no observation events (absence is not a fact).
3. `cargo test --workspace` green; golden-fixture checksum diffs explained.

### Invariants

1. The perception pass emits events; it never returns cognition facts directly to the scheduler.
2. Observation events replay byte-identically in identical order.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/` — a perception fixture asserting observation-event presence/absence by current-place visibility.
2. `crates/tracewake-core/src/agent/` (perception module) — unit test for deterministic window-keyed emission.

### Commands

1. `cargo test -p tracewake-core scheduler:: && cargo test -p tracewake-core agent::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Added `agent::perception`, a deterministic current-place perception pass that appends `ObservationRecorded` events for same-place visible food supplies, open same-place sleep affordances, and visible exits.
- Invoked the perception pass from the no-human day scheduler before the actor decision transaction. The pass appends event-log observations only; it does not return cognition facts to the scheduler.
- Added unit coverage for event presence, absence-as-no-event, and byte-identical repeated appends.
- Added scheduler-level coverage proving no-human day logs perception observations before the decision trace and replay/rerun output is byte-identical.
- Preserved the existing hidden-truth guard by filtering hidden route targets out of visible-exit perception until a later route-knowledge channel can distinguish visible exits from authored hidden topology.

Deviations from original plan:

- No event schema extension was needed; the pass reuses `ObservationRecorded` with additive payload fields (`perceived_kind`, `subject_id`, `target_id`).
- No golden checksum files changed; existing fixture runs remain deterministic under the new observation events.

Verification:

- `cargo test -p tracewake-core agent::`
- `cargo test -p tracewake-core scheduler::`
- `cargo test -p tracewake-content --test golden_fixtures_run no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
