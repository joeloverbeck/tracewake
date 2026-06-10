# 0017PHA3ATICLED-004: Witness-compatible provenance citations and bidirectional fact reconciliation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/no_human_surface`, `agent/actor_known`, `agent/transaction`, `agent/trace`); hidden-truth gate additions
**Deps**: `archive/tickets/0017PHA3ATICLED-003.md` (same fact-minting surface, sequential by design); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-031)

## Problem

Provenance citations are existence-checked, not witness-checked. `no_human_surface.rs::add_window_framing_facts` stamps `actor_current_place_visible` (value: the builder-supplied current place) and `agent_needs_present` (read from `agent_state.needs_by_actor()`) with the day/advance *frame marker* event as sole `source_event_ids` — an event that witnesses neither the place nor the needs, while the same-tick `record_current_place_perception` event (which does witness the place) goes uncited. `transaction.rs::dangling_provenance_diagnostic` only checks cited ids exist in the frontier. The facts↔structured reconciliation is one-directional (`actor_known.rs::audit_with` → `structured_fact_gaps()` catches structured-without-fact only; `add_role_assignment_notice` facts have no structured counterpart and bypass it; `build` hardcodes the containers/doors maps empty). And a `None` frame id silently builds a fact-less surface instead of failing closed (INV-102; foundation doc 14).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `add_window_framing_facts` early-returns on `None` frame id and cites the single frame event for all framing facts; `dangling_provenance_diagnostic` flat-maps `source_event_ids` and `find`s non-resolving ids only; `ActorKnownPlanningContext::from_observed_parts` is called from `build` with two `BTreeMap::new()` arguments (containers-by-place, closed-doors); `scheduler.rs` runs `record_current_place_perception` before `build_agent_proposal`, so a genuine same-tick witnessing event exists to cite.
2. Spec 0017 §ORD-HARD-031: every framing fact cites its genuine witnessing event; a provenance-kind compatibility table in the transaction audit fails closed on mismatch; structured fields derive from facts via a single builder; missing frame ⇒ typed `Stuck`.
3. Cross-artifact boundary under audit: the fact↔structured-context contract inside `ActorKnownPlanningContext` — six structured fields and the `facts` vec must be one source of truth, in both directions.
4. INV-102 restated: cognition inputs require provenance *sufficient for replay and debug review* — a citation to a non-witnessing event reviews to nothing; for action-relevant cognition, insufficiency is a rejection condition.
5. Fail-closed actor-knowledge surface and deterministic replay touched: the new witness-compatibility audit extends the existing fail-closed transaction path (`BlockerCode` gains `ProvenanceWitnessMismatch`); re-citing framing facts changes decision context inputs, so context hashes and golden traces shift — the from-log re-derivation gate must pass at the corrected citations; replay byte-stability re-established. No knowledge is added or leaked; only citations become honest.
6. Mismatch + correction: the spec names `structured_fact_gaps` without a host file; it is a method on the planning context in `agent/actor_known.rs` (reached via `audit_with`), not `agent/decision.rs` — mechanical drift, corrected here.

## Architecture Check

1. Deriving the six structured fields from the audited facts via one builder function makes the reconciliation bidirectional *by construction* (spec tier-1 lock) — strictly stronger than extending the audit to scan both directions, and it eliminates the hardcoded-empty-maps asymmetry. The witness-compatibility table (fact kind → admissible witnessing event kinds) lives beside the dangling check in the transaction so the two provenance audits share one fail-closed path.
2. No backwards-compatibility aliasing/shims: framing facts re-cite their true witnesses outright (no dual-citation transition); the independent structured-field population path is deleted, not deprecated.

## Verification Layers

1. INV-102 witness sufficiency -> unit test: a fact citing a real-but-unrelated event (e.g. needs-present citing the frame marker) yields typed `Stuck(ProvenanceWitnessMismatch)` before candidate selection.
2. Fail-closed empty frontier -> fixture/unit test: an empty prefix log yields the typed limitation diagnostic, never a fact-less decision surface.
3. Bidirectional reconciliation -> codebase grep-proof + unit test: structured fields are computed from facts (no independent population site); a fact with no structured counterpart can no longer exist by construction, and the existing structured-without-fact audit test still passes.
4. INV-018 deterministic replay -> the context-hash re-derivation gate passes over every golden no-human run at the corrected citations (`cargo test --workspace`).

## What to Change

### 1. Witnessing citations in `no_human_surface.rs`

`actor_current_place_visible` cites the same-tick `record_current_place_perception` event; `agent_needs_present` cites the actor's latest need event; intention-related framing facts cite the intention adoption/lifecycle event from the projection. The frame marker is no longer an acceptable witness for any fact that asserts world or agent state.

### 2. Fail-closed missing frame

A missing frame/witness event produces a typed `Stuck` (typed limitation), replacing the silent early-return.

### 3. Single derivation builder in `actor_known.rs`

Structured context fields (`known_edges`, `known_food_sources`, `known_sleep_places`, `known_workplaces`, `known_containers_by_place`, `known_closed_doors`) are computed from the facts vec by one function consumed by `from_observed_parts`/`build`; `add_role_assignment_notice` facts gain their structured counterparts through it automatically.

### 4. Witness-compatibility audit in `transaction.rs`

A fact-kind → admissible-witness-event-kind table, enforced alongside `dangling_provenance_diagnostic`; mismatch fails closed with `BlockerCode::ProvenanceWitnessMismatch` (new variant in `agent/trace.rs`).

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — context-hash/trace expectations, as surfaced)

## Out of Scope

- Staleness classification of record-derived facts (landed in `0017PHA3ATICLED-003`).
- The `SourceEventIds` field-type retype on `ActorKnownFact` (ticket `-008`; this ticket keeps the current field shape).
- Workplace believed-access facts (ticket `-006`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core provenance_witness_mismatch` — non-witnessing citation fails closed, typed, pre-proposal.
2. `cargo test -p tracewake-core` (hidden-truth gates) — empty-frontier fail-closed; structured-derivation reconciliation.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every actor-known fact's cited source events are of a kind that can witness that fact kind; the admissibility table is the single authority.
2. Structured context fields and the facts vec cannot diverge in either direction — one is derived from the other by construction.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/transaction.rs` (unit tests) — witness-mismatch fail-closed; empty-frontier typed limitation.
2. `crates/tracewake-core/src/agent/actor_known.rs` (unit tests) — derivation builder bijection (every fact kind with a structured projection appears; no independent population).
3. `crates/tracewake-core/tests/hidden_truth_gates.rs` — gate-level witness audit coverage.

### Commands

1. `cargo test -p tracewake-core provenance_witness`
2. `cargo test --workspace`

## Outcome

Completed 2026-06-10.

Added witness-kind auditing to `ActorDecisionTransaction`: transaction inputs can now carry
a log-derived source-event kind map, and mismatched actor-known citations fail closed with
`BlockerCode::ProvenanceWitnessMismatch`. The audit currently covers the no-human framing
fact kinds corrected by this ticket (`actor_current_place_visible`, `agent_needs_present`,
window framing, and active-intention procedure-state facts).

Reworked no-human framing citations so current-place visibility cites a same-tick
`ObservationRecorded` event, needs-present cites a `NeedDeltaApplied` event, and generic
window/procedure framing cites the no-human frame marker. Removed the unsafe "first event
is frame" fallback in live scheduling and replay rebuild.

Made current-place perception emit an explicit `current_place` observation even when no
other visible surface is present, giving `actor_current_place_visible` a real modeled
witness. Replay context-hash rebuilding now derives the same current-place and need witness
ids as the live scheduler.

Moved structured actor-known fields behind fact derivation in `ActorKnownPlanningContext`:
known routes, food sources, sleep places, workplaces, containers, and closed doors are now
computed from actor-known facts during construction. Added unit coverage for fact-derived
structured fields and for caller-supplied structured fields without facts being discarded by
construction.

Verification run:

1. `cargo test -p tracewake-core provenance_witness`
2. `cargo test -p tracewake-core structured_actor_known_fields_are_derived_from_facts`
3. `cargo test -p tracewake-core structured_context_without_matching_fact_is_not_constructed`
4. `cargo test -p tracewake-core current_place_perception`
5. `cargo test -p tracewake-core --test hidden_truth_gates`
6. `cargo test -p tracewake-content --test golden_fixtures_run`
7. `cargo test -p tracewake-tui --test embodied_flow`
8. `cargo fmt --all --check`
9. `cargo clippy --workspace --all-targets -- -D warnings`
10. `cargo build --workspace --all-targets --locked`
11. `cargo test --workspace`
