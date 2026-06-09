# 0015PHA3AEVECOG-001: Seed-time knowledge events for authored starting knowledge

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `tracewake-core` `EventKind` variants (`RoleAssignmentNoticeRecorded`, `StartingBeliefRecorded`) with payloads + schema-registry metadata; `tracewake-content` load-path emission; golden-fixture checksum updates
**Deps**: None

## Problem

ORD-HARD-008's root pattern: the no-human actor-known surface reverse-engineers an actor's workplace/sleep/food knowledge from raw authoritative tables at decision time (`observe_workplace_notices_from_active_routines` reads `state.workplaces()`, `observe_sleep_notice_from_active_routine` reads `state.sleep_affordances()`), minting provenance-labeled facts (`routine_assignment_workplace_notice`, `modeled_observation:open_sleep_affordance_at_current_place`) for which **no event exists anywhere in the log**. Knowledge with no event ancestry cannot be replay-derived (INV-009, INV-018) and asserts a channel that never ran (INV-102).

This ticket builds the first half of the channel fix (the spec's required-correction item 1): materialize the knowledge an authored actor legitimately starts with — workplace assignment, home/sleep place, household food knowledge — as typed prehistory knowledge events at content load/seed time, marked as authored prehistory (INV-063). It is deliberately **additive and unwired**: the surface builder still reads raw tables until the atomic cutover in `0015PHA3AEVECOG-003`. No cognition behavior changes here; the log gains the events the cutover will consume.

## Assumption Reassessment (2026-06-09)

1. Current code mints knowledge facts from raw truth with no backing event: `observe_workplace_notices_from_active_routines` and `observe_sleep_notice_from_active_routine` in `crates/tracewake-core/src/agent/no_human_surface.rs` iterate `state.workplaces()` / `state.sleep_affordances()` and mint labeled facts (verified). The `EventKind` enum is defined in `crates/tracewake-core/src/events/envelope.rs` (variant block ~:98, string map ~:308, `EVENT_SCHEMA_REGISTRY` metadata, replay-handling classification); the application dispatch arm pattern is in `crates/tracewake-core/src/events/apply.rs:207` (`EventKind::ObservationRecorded =>`); `ObservationRecordedPayload` (envelope.rs:453) is the payload-struct exemplar to mirror.
2. Specs/docs: spec 0015 §ORD-HARD-008 "Required correction" item 1; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-063 (authored prehistory must be marked); `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` TFW provenance table (workplace/sleep knowledge must come from observation, memory, assignment notice, or record — not raw tables); `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` for the authored-actor fixture contract.
3. Shared boundary under audit: the content→core seam. Per spec §8, seed-time knowledge-event emission lives in `crates/tracewake-content/src/load.rs` calling core event constructors; core must not read content. The new `EventKind` variants and payloads live in core (`events/envelope.rs`); content emits them. This preserves dependency direction (core depends on nothing).
4. INV-063 — pre-simulation artifacts and boundary conditions must carry structured provenance marking them authored prehistory / declared boundary input; the seed knowledge events must therefore be marked authored prehistory, not runtime-caused. INV-009 — the knowledge an actor starts with is gameplay-affecting state and must be represented by events. INV-020 — new event kinds must be versioned enough that replay can reject unsupported history.
5. Deterministic-replay / actor-knowledge surface (substrate-only — enforcement is deferred to siblings): this ticket supplies the inputs that `0015PHA3AEVECOG-003`'s sealed builder and `0015PHA3AEVECOG-004`'s context-hash rebuild-from-log byte-match gate will enforce. Confirm here that (a) seed events are appended in a deterministic order keyed to the stable seed/load ordering (no wall-clock, no incidental hash-map iteration), and (b) they encode only what the authored actor legitimately knows, introducing no actor-knowledge leakage path the cutover would have to undo. No validator rejecting raw-table cognition exists yet — that is sibling 003/004.
6. Schema extension: extends the core `EventKind` event-record schema additively. Consumers of that schema — `EventKind::all()` / `EventKind::registry()` exhaustiveness tests (`events/mod.rs`), the apply dispatch (`events/apply.rs`), the deterministic checksum/replay path (`checksum.rs`), and the `EVENT_SCHEMA_REGISTRY` — each gain an arm/entry for the new kinds. The extension is **additive-only** (new variants + payloads; no existing kind changes shape), versioned per INV-020.

## Architecture Check

1. Events-as-channel is the doctrinal fix, not a call-site patch: knowledge carried by typed prehistory events is replay-derivable and provenance-bearing, where the current reverse-engineering forges a channel that never ran. Building the emission additively (unwired) lets the expensive ORD-HARD-008 cutover land as a reviewable atomic flip in 003 rather than one mega-diff.
2. No backwards-compatibility shims: the new event path is added alongside the existing raw-table reads; the raw reads are **removed** in 003, never wrapped behind a fallback.

## Verification Layers

1. INV-063 → codebase grep-proof: the new event kinds carry an authored-prehistory provenance mark; a canonical fixture actor's seed emits `RoleAssignmentNoticeRecorded` / `StartingBeliefRecorded` with that mark.
2. INV-020 → schema validation: the `EventKind` registry/exhaustiveness test (`events/mod.rs`) covers the new kinds with version metadata; `EventKind::all()` includes them.
3. INV-009 → replay/golden-fixture check: seeded knowledge appears as events in the canonical fixture's log; the golden checksum changes and the diff is explainable (new seed events only).
4. INV-018 (substrate-only) → manual review: emission order is deterministic and seed-keyed; byte-identical-replay enforcement is deferred to `0015PHA3AEVECOG-004`'s rebuild gate (named here so the determinism contract has an owner).

## What to Change

### 1. New core event kinds and payloads

Add `RoleAssignmentNoticeRecorded` and `StartingBeliefRecorded` to `EventKind` in `events/envelope.rs` (variant, display-string map, `EVENT_SCHEMA_REGISTRY` metadata, replay-handling classification), each with a typed payload struct mirroring `ObservationRecordedPayload`, and an application arm in `events/apply.rs`. Version per INV-020.

### 2. Authored-prehistory provenance mark

Mark the new event kinds as authored prehistory (INV-063) using the existing prehistory/boundary-provenance mechanism (confirm the exact marker during implementation against how other seeded/prehistory events are tagged).

### 3. Content load-path emission

In `crates/tracewake-content/src/load.rs`, emit the new knowledge events for each authored actor's legitimate starting knowledge — workplace assignment, home/sleep place, household food knowledge — derived from the authored fixture/seed data, calling the core event constructors. Do not alter the surface builder (003 owns that).

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/events/mod.rs` (modify — exhaustiveness test arms)
- `crates/tracewake-core/src/checksum.rs` (modify — if new kinds enter the checksum surface)
- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify — provenance mark / seed plumbing if required)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — checksum updates, explained)

## Out of Scope

- Building the actor-known surface from these events / removing raw-table reads (`0015PHA3AEVECOG-003`).
- The modeled perception pass for current-place visibility (`0015PHA3AEVECOG-002`).
- The hidden-truth audit fail-closed gate (`0015PHA3AEVECOG-005`).
- Source guards and negative fixtures for the channel (`0015PHA3AEVECOG-004`).
- The recorded ORD-HARD-008 minimum-cut fallback (spec §9) is acceptable only if invoked explicitly in 003; this ticket delivers the full evented-seed half regardless.

## Acceptance Criteria

### Tests That Must Pass

1. A canonical no-human fixture actor's seeded log contains `RoleAssignmentNoticeRecorded` (workplace assignment) and `StartingBeliefRecorded` (home/sleep, household food) events carrying the authored-prehistory mark.
2. `EventKind` exhaustiveness/registry test passes with the new kinds present and versioned (`cargo test -p tracewake-core events`).
3. `cargo test --workspace` green; updated golden-fixture checksums explained in the diff.

### Invariants

1. New event kinds are additive; no existing `EventKind` variant changes shape or string mapping.
2. Seed knowledge events are marked authored prehistory (INV-063) and emitted deterministically; no runtime-cause framing.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/` — a fixture (or extension of an existing no-human fixture) asserting the seeded knowledge events and their prehistory marks.
2. `crates/tracewake-core/src/events/mod.rs` — extend the `EventKind` exhaustiveness/registry test for the two new kinds.

### Commands

1. `cargo test -p tracewake-core events:: && cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Added additive core seed-channel event kinds `RoleAssignmentNoticeRecorded` and `StartingBeliefRecorded`, with versioned authored-prehistory payload structs, stable event metadata, registry coverage, and replay application routing.
- Extended `load_fixture_package` to return a deterministic `seed_event_log` alongside the canonical world and agent state.
- Emitted deterministic authored-prehistory seed events for existing initial beliefs, actor homes, actor sleep places, authored household food knowledge, and workplace assignment notices.
- Added loader coverage proving `sleep_eat_work_001` emits the role-assignment notice plus home/sleep/food starting-belief events with `source_kind = authored_prehistory`, and that repeated loads produce byte-identical seed logs.

Deviations from original plan:

- Reused the existing `InitialBeliefSeeded` event kind for already-authored `initial_beliefs` rather than introducing a duplicate belief-proposition seed path. `StartingBeliefRecorded` covers the broader Phase 3A starting knowledge facts that are not yet representable as current `Proposition` variants.
- The actor-known surface builder remains unwired by design; `0015PHA3AEVECOG-003` owns the cutover.

Verification:

- `cargo test -p tracewake-core events`
- `cargo test -p tracewake-content`
- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
