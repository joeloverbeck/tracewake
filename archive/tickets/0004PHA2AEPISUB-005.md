# 0004PHA2AEPISUB-005: Epistemic projection application and deterministic rebuild

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds explicit epistemic event application and replay rebuild to `tracewake-core` (`events/apply.rs`, `replay/`).
**Deps**: 0004PHA2AEPISUB-003, 0004PHA2AEPISUB-004

## Problem

Replay must rebuild epistemic projections deterministically and reject unsupported versions rather than silently repairing them (Spec 0004 §13; `INV-018`, `INV-020`). The physical applier must stay pure — it must not mutate beliefs — so epistemic application is a separate, explicit, deterministic path that consumes the event kinds from ticket 004 and fills the projection from ticket 003. Replay output must be byte-identical across repeated runs.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/events/apply.rs` applies physical world events to `PhysicalState` and treats non-world events as no-ops, rejecting unsupported event schema versions. `crates/tracewake-core/src/replay/rebuild.rs` and `replay/report.rs` build physical projections and the replay report; neither touches epistemic state yet.
2. The projection store and `KnowledgeContext` come from ticket 003; the epistemic event kinds and `schema_version` fields from ticket 004. Spec §13.2 fixes the rebuild inputs (initial seed, ordered log, supported schema registry, projection version, manifest id) and §13.3 the rejection set.
3. Shared boundary under audit: the replay report shape is consumed by the debug replay surface (ticket 010/011) and the capstone determinism check (ticket 016); this ticket extends it with epistemic projection version and event-position failure detail (§13.3) additively.
4. Invariant motivating this ticket: `INV-018` (deterministic replay is foundational), `INV-020` (schema evolution — reject unsupported), and `INV-019` (snapshots/compaction may not erase live ancestry — the epistemic projection rebuilds from the log, not a lossy cache).
5. Deterministic-replay / no-leak surface: this is the enforcing ticket for epistemic determinism. Application iterates the ordered log and writes to ordered projection indexes; no wall-clock, no `HashMap` iteration, no hidden-truth read (an epistemic event's belief content comes only from its payload + initial seed, never from `PhysicalState` ground truth). Repeated rebuild must produce identical ordering and an identical projection checksum/report. Unsupported epistemic event/proposition/projection versions and missing source/holder references reject loudly per §13.3.

## Architecture Check

1. A dedicated `apply_epistemic_event` path (separate from physical `apply_event`) makes the §9.1 split structural: physical state can never be mutated by an epistemic event and vice-versa, so a belief can diverge from physical truth without corrupting single-source location. A merged applier would require per-kind guards and risk silent cross-mutation.
2. No backwards-compatibility shims: the physical applier and existing replay report fields are unchanged; epistemic application and report fields are additive.

## Verification Layers

1. Deterministic replay (`INV-018`) -> replay/golden-fixture check + unit test: rebuilding the same log twice yields identical projection ordering and an identical epistemic projection checksum.
2. Reject-unsupported (`INV-020`) -> unit test: an event with an unknown epistemic schema version, an unknown proposition variant, or a missing source/holder reference causes a loud replay failure naming event position and version (§13.3); nothing is silently repaired.
3. No hidden-truth leak (`INV-024`) -> manual review: epistemic application reads only event payloads + initial seed, never `PhysicalState`; physical apply stays pure (grep-proof that `apply.rs` epistemic arm does not touch `PhysicalState`).

## What to Change

### 1. Epistemic event application

Extend `crates/tracewake-core/src/events/apply.rs` with an explicit epistemic application path that, for each `EventStream::Epistemic` event, updates the `EpistemicProjection` (insert observation, insert/update belief, link contradiction, seed initial belief) deterministically. The physical `apply_event` continues to treat epistemic events as physical no-ops.

### 2. Deterministic rebuild and report

Extend `crates/tracewake-core/src/replay/rebuild.rs` to rebuild the epistemic projection from initial seed + ordered log, and `crates/tracewake-core/src/replay/report.rs` to record epistemic projection version, content-manifest id, and the §13.3 failure detail (event position, content version, projection version, unsupported-kind/missing-reference reason).

### 3. Version and reference rejection

Reject unknown epistemic event kinds, unsupported epistemic/proposition/projection schema versions, and missing source/holder references during rebuild, with no silent repair.

## Files to Touch

- `crates/tracewake-core/src/events/apply.rs` (modify — add epistemic application path; physical apply unchanged)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — rebuild epistemic projection)
- `crates/tracewake-core/src/replay/report.rs` (modify — epistemic projection version + failure detail)

## Out of Scope

- The actions that emit epistemic events (tickets 006, 007).
- Notebook/debug view models over the projection (tickets 009, 010).
- Content-manifest seed loading (ticket 012/013) — this ticket consumes seeds the loader provides.

## Acceptance Criteria

### Tests That Must Pass

1. Replaying a fixed epistemic event log twice produces byte-identical projection ordering and checksum.
2. An event with an unsupported epistemic schema version fails replay loudly, naming event position and version; no repaired belief store is produced.
3. Physical `apply_event` over an epistemic event leaves `PhysicalState` unchanged (and vice-versa).

### Invariants

1. The epistemic projection is rebuildable solely from initial seed + ordered log + supported schema versions.
2. Physical and epistemic application are separate paths; neither mutates the other's state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/replay/rebuild.rs` (unit tests) — deterministic epistemic rebuild, double-rebuild equality.
2. `crates/tracewake-core/src/events/apply.rs` (unit tests) — epistemic application updates projection; physical apply purity.
3. `crates/tracewake-core/tests/golden_scenarios.rs` (extend) — an epistemic replay determinism case (full assertions land with fixtures in ticket 013/016).

### Commands

1. `cargo test -p tracewake-core replay:: events::apply`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. `cargo build --workspace --all-targets --locked`

## Outcome

Completion date: 2026-06-07

What changed:
- Added explicit `apply_epistemic_event` handling for the Phase 2A epistemic event stream while keeping physical `apply_event` a non-world no-op for epistemic events.
- Added deterministic epistemic projection checksum generation over ordered projection records.
- Extended replay rebuild/report output with rebuilt epistemic projection, checksum, projection version, unsupported epistemic version details, and epistemic application errors.
- Added tests for deterministic double rebuild, unsupported epistemic event-schema rejection with event position/version, epistemic application into projection, unsupported payload-schema rejection, and physical purity for epistemic events.

Deviations from original plan:
- The ticket's combined cargo test filter is not valid cargo syntax, so `replay::` and `events::apply` were run as separate filters.
- `events/mod.rs` was touched to re-export `EVENT_SCHEMA_V1`, which the explicit payload parser needs.

Verification results:
- `cargo test -p tracewake-core replay::`
- `cargo test -p tracewake-core events::apply`
- `cargo test -p tracewake-core --test golden_scenarios`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
