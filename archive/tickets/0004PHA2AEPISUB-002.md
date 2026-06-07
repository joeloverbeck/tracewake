# 0004PHA2AEPISUB-002: Observation, belief, and contradiction records

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds observation/belief/contradiction record types to the `tracewake-core` `epistemics` module.
**Deps**: 0004PHA2AEPISUB-001

## Problem

Phase 2A requires first-class, source-backed epistemic records: observations as event-backed perceptual inputs (Spec 0004 §8.4), holder-specific beliefs that record provenance (§8.5; `INV-026`), and explicit contradiction records linking a prior expectation to a contradicting observation (§8.6). Observation must never be interpretation, and no important belief may lack a holder or source. These records are the payload content carried by epistemic events (ticket 004) and stored in the projection (ticket 003), so they must exist before either.

## Assumption Reassessment (2026-06-06)

1. The record types do not exist: `grep -nE "Belief|Observation|Contradiction" crates/tracewake-core/src/state.rs` returns no epistemic structs — `state.rs` carries only physical state plus an `OwnershipCustody { expected_location: Option<Location> }` stub (`state.rs:226`). These records belong in `epistemics/`, not `state.rs` (physical state stays single-source).
2. Field sets, holder kinds, stances, and channels are fixed by Spec 0004 §8.4/§8.5/§8.6 and mirror the foundation contract in `docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` §belief-store / §observation. `Proposition` and the epistemic IDs come from ticket 001.
3. Shared boundary under audit: the `Observation`, `Belief`, and `Contradiction` field layouts are the payload contract that ticket 004 (event payloads §9.4–9.6) and ticket 012 (content seed schema §11.1) must serialize identically — the field names are fixed here.
4. Invariant motivating this ticket: `INV-026` (important beliefs need provenance — holder, claim, stance, confidence, source, channel, acquisition time, contradiction links, scope) and `INV-002` (belief comes before truth — beliefs are holder-relative, never ground truth).
5. Actor-knowledge / no-leak surface (substrate-only): these records carry `holder_id`/`observer_actor_id` and `privacy/scope`, the fields the actor-knowledge filter (ticket 003 `KnowledgeContext`, ticket 009 notebook) keys on. This ticket introduces no leakage path — records hold provenance but no cross-actor visibility grant; the enforcing filter lands in ticket 003. Construction must make holder and source non-optional for important beliefs.

## Architecture Check

1. Separate `Observation` / `Belief` / `Contradiction` types (not one merged "knowledge" struct) keep the doctrine distinction that observation is not interpretation (§8.4) and that a contradiction is a derived link, not a belief. A merged type would blur the observation→belief→contradiction provenance chain.
2. No backwards-compatibility shims: greenfield records in the new module.

## Verification Layers

1. Provenance required (`INV-026`) -> manual review + unit test: `Belief` construction requires holder and source; a builder/constructor test proves a sourceless important belief is unrepresentable or rejected.
2. Belief ≠ truth, observation ≠ belief (`INV-002`) -> unit test: an `Observation` does not implement/contain a `Belief`; recording an observation creates no belief by itself.
3. Determinism (`INV-018`) -> grep-proof: contradiction-link and scope collections use ordered/sorted containers, not `HashMap` iteration; records derive ordering by value.

## What to Change

### 1. Observation record

Add `crates/tracewake-core/src/epistemics/observation.rs` with an `Observation` struct carrying §8.4 fields (`observation_id`, `observer_actor_id`, `channel`, `observed_tick`/`tick_window`, `observer_place_id`, subject/target reference, `raw_payload`, `confidence`, source event/action/process reference, optional `alternatives`, schema version, privacy/scope) and a `Channel` enum with the §8.4 minimum channels (`direct_sight`, `touch_or_search`, `simple_sound`, `absence_marker`, `reading_placeholder_schema_only`).

### 2. Belief record

Add `crates/tracewake-core/src/epistemics/belief.rs` with a `Belief` struct (§8.5 fields), a `HolderKind` enum (`actor`, `institution_placeholder` — only `actor` active), and a `Stance` enum (`believes_true`, `believes_false`, `expects_true`, `plausible`, `doubts`, `unknown_or_unresolved`). Confidence uses the deterministic canonical encoding required by Spec §8.4 (bounded integer scale or fixed-precision decimal, not raw float `Display`).

### 3. Contradiction record

Add `crates/tracewake-core/src/epistemics/contradiction.rs` with a `Contradiction` struct (§8.6 fields) and a `ContradictionKind` enum whose only active variant is `expected_item_absent_from_container`; other kinds may exist as inert vocabulary only.

### 4. Module registration

Extend `crates/tracewake-core/src/epistemics/mod.rs` with `pub mod observation; pub mod belief; pub mod contradiction;`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/observation.rs` (new)
- `crates/tracewake-core/src/epistemics/belief.rs` (new)
- `crates/tracewake-core/src/epistemics/contradiction.rs` (new)
- `crates/tracewake-core/src/epistemics/mod.rs` (modify — file created by 0004PHA2AEPISUB-001)

## Out of Scope

- `KnowledgeContext` and projection storage (ticket 003).
- Epistemic events and payload encoding (ticket 004).
- Contradiction *detection* logic (ticket 007) — this ticket defines the record only.
- Content seed schema (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. Constructing a `Belief` without a source or holder is unrepresentable or fails a checked constructor.
2. An `Observation` with channel `simple_sound` carries `alternatives` and a low `confidence`; recording it produces no `Belief`.
3. A `Contradiction` links a prior expectation belief id and an observation id; `ContradictionKind::expected_item_absent_from_container` is the only active kind.

### Invariants

1. Beliefs are holder-keyed and source-backed; no global proposition flag exists.
2. Observation, belief, and contradiction are distinct types; observation never auto-derives a belief.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/observation.rs` (unit tests) — channel set, sound alternatives/confidence, observation ≠ belief.
2. `crates/tracewake-core/src/epistemics/belief.rs` (unit tests) — holder/source required, stance set, confidence encoding.
3. `crates/tracewake-core/src/epistemics/contradiction.rs` (unit tests) — link fields, active kind.

### Commands

1. `cargo test -p tracewake-core epistemics::`
2. `cargo build --workspace`
3. Core-crate unit scope is correct: these records have no cross-crate consumers until events (004) and content (012) land.

## Outcome

Completion date: 2026-06-07

What changed:
- Added distinct `Observation`, `Belief`, and `Contradiction` record types in `tracewake-core::epistemics`.
- Added channel, holder, stance, confidence, source, tick-window, subject/target, privacy-scope, and contradiction-kind vocabulary needed by later Phase 2A tickets.
- Added a checked `BeliefDraft::build` path proving missing holder/source records fail construction, while the concrete `Belief` type remains holder-keyed and source-backed.
- Added unit tests covering sound observation alternatives/low confidence, observation-not-belief separation, belief provenance requirements, deterministic confidence encoding, and contradiction links.

Deviations from original plan:
- None. Records remain core-only substrate types; projection storage and event application are still deferred to their tickets.

Verification results:
- `cargo test -p tracewake-core epistemics::`
- `cargo build --workspace`
- `cargo fmt --all --check`
