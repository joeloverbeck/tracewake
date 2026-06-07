# 0004PHA2AEPISUB-004: Epistemic event kinds, stream, and payloads

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `Epistemic` event stream, new event kinds, and their payloads to `tracewake-core` `events/envelope.rs`.
**Deps**: 0004PHA2AEPISUB-002, 0004PHA2AEPISUB-003

## Problem

Meaningful epistemic state changes must be eventful (`INV-009`, `INV-012`). Spec 0004 §9 requires new event kinds — `InitialBeliefSeeded`, `ObservationRecorded`, `BeliefUpdated`, `ExpectationContradicted`, `ContainerChecked` — classified into a clearly nonphysical authoritative stream (`EventStream::Epistemic` recommended), with versioned payloads (§9.4–9.6) sufficient for replay to rebuild the belief projection without consulting hidden truth. Physical application must never silently mutate beliefs.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/events/envelope.rs` currently defines `EventStream { World, Diagnostic, Controller, ReplayDebug }` (`envelope.rs:11`) and physical/controller/replay event kinds, with stream-label/parse helpers at `envelope.rs:450` and `envelope.rs:459`. No epistemic kinds or `Epistemic` stream exist; this ticket adds them.
2. Event-kind registry shape, schema-version gating, and the stream classification helper are fixed by Spec 0004 §9 and the architecture contract `docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` (versioned envelopes, schema migration). Payload content reuses the `Observation`/`Belief`/`Contradiction` records from ticket 002.
3. Shared boundary under audit: the event-kind enum and `EventStream` are consumed by the stream-label/parse functions, by application (ticket 005), and by replay (ticket 005); adding a stream variant requires a new arm at each `match`. The new kind names and stream classification are fixed here.
4. Invariant motivating this ticket: `INV-009` (meaningful state changes require events), `INV-012` (events are not only physical), and `INV-020` (event schema evolution — payloads versioned enough that replay can reject unsupported history rather than inventing repairs).
5. Deterministic-replay surface (substrate-only): event payloads are the canonical replay input. This ticket adds the kinds and `schema_version` fields and the stream classification but does NOT apply them — it introduces no nondeterminism (no wall-clock, no hash-ordering; ids and ticks come from payload). The applying/rebuilding enforcement is ticket 005. Adding the `Epistemic` variant is additive per `INV-020`; every existing `match` over `EventStream`/`EventKind` gets a new arm in this ticket.

## Architecture Check

1. A dedicated `EventStream::Epistemic` keeps the physical/epistemic application split explicit at the stream level, so `apply_event` for physical state can structurally ignore epistemic events (the §9.1 requirement that physical apply never mutates beliefs). Folding epistemic events into `World` would force a per-kind guard in the physical applier and risk silent belief mutation.
2. No backwards-compatibility shims: the new stream and kinds are additive; existing kinds and their stream mapping are unchanged.

## Verification Layers

1. Eventful epistemics (`INV-009`/`INV-012`) -> grep-proof + unit test: the five new `EventKind` variants exist and classify to `EventStream::Epistemic`; the stream-label/parse round-trip covers the new variant.
2. Schema evolution (`INV-020`) -> unit test: each epistemic payload carries `schema_version`; an unknown version is representable for ticket 005 to reject.
3. Exhaustiveness (`INV-018` ordering of streams) -> grep-proof: every `match` over `EventStream`/`EventKind` in `envelope.rs` gains the new arm; no wildcard swallows the new kinds.

## What to Change

### 1. Epistemic event stream

Add `Epistemic` to `EventStream` and extend the stream-label (`envelope.rs:450`) and parse (`envelope.rs:459`) helpers with the `"epistemic"` token.

### 2. Epistemic event kinds and payloads

Add the five `EventKind` variants and their payload structs: `ObservationRecorded` (§9.4), `BeliefUpdated` (§9.5), `ExpectationContradicted` (§9.6), `InitialBeliefSeeded` (§9.3 provenance: `authored_prehistory` source kind), and `ContainerChecked` (the proof-of-intentional-check event, §9.2). Each payload carries a `schema_version`. Classify all five to `EventStream::Epistemic` in the kind→stream mapping and add their string labels.

### 3. Registry exhaustiveness

Add the new arms to every `EventKind`/`EventStream` `match` in `envelope.rs` (label, parse, stream classification, any registry list), so the build has no non-exhaustive or wildcard gap.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify — add `EventStream::Epistemic`, five event kinds + payloads, label/parse/classification arms)

## Out of Scope

- Applying epistemic events to the projection and replay rebuild (ticket 005).
- The `check_container` action that emits `ContainerChecked`/`ObservationRecorded` (ticket 006).
- Contradiction detection that emits `ExpectationContradicted` (ticket 007).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the five new `EventKind` variants classifies to `EventStream::Epistemic`.
2. The stream label/parse round-trips `"epistemic"` ↔ `EventStream::Epistemic`.
3. An epistemic payload with an unsupported `schema_version` is constructible (so ticket 005 can reject it) and carries the record content from ticket 002.

### Invariants

1. Epistemic events are a distinct stream; physical `apply_event` structurally need not touch them.
2. Every payload is versioned; no epistemic event lacks a `schema_version`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/envelope.rs` (unit tests) — extend existing stream/kind tests with epistemic classification, label/parse round-trip, payload versioning.

### Commands

1. `cargo test -p tracewake-core events::envelope`
2. `cargo build --workspace --all-targets` (proves no non-exhaustive `match` after adding the stream variant)
3. The envelope unit scope is correct because application/replay behavior is exercised in ticket 005.
