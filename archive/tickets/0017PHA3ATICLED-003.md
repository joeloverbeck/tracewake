# 0017PHA3ATICLED-003: Projection freshness rule and honest provenance classes

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`epistemics/projection`, `agent/no_human_surface`, `agent/perception`, `agent/transaction`); one new content fixture; one new source guard; golden trace/checksum updates
**Deps**: `archive/tickets/0017PHA3ATICLED-002.md` (rides the same golden re-baseline window; spec §8 ordering); `archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-027)

## Problem

The unified no-human actor-known surface has no staleness rule and records false provenance classes. `epistemics/projection.rs::actor_known_records_for_context` returns every record for the viewer actor — the `KnowledgeContext` argument is consulted only for `viewer_actor_id()`, with no tick or place comparison — and `no_human_surface.rs::consume_projection_record` mints `ActorKnownFact::observed_now(..., self.decision_tick, ...)` for every record. A record projected at tick 0 surfaces at tick 95 as an observation made at tick 95. Meanwhile the embodied path (`perception.rs::current_place_knowledge_context`) filters to the latest current-place perception window — the two consumers of the "unified" projection apply divergent freshness semantics, and the no-human decision trace asserts an observation class and time that are false (INV-026, INV-102; the unfinished half of ORD-HARD-021 / ticket 0016PHA3ANEEACC-015).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `actor_known_records_for_context` flat-maps `actor_known_records_by_actor` with no tick/place filter; `consume_projection_record` stamps `observed_now` with `self.decision_tick` for Route/FoodSource (and sibling) records; `perception.rs::current_place_knowledge_context` filters to the latest perception window (proven by `current_place_knowledge_context_uses_latest_projection_window_not_live_truth`). `ActorKnownFact` already has both `observed_now` and `remembered_belief` constructors taking an acquisition tick — no new provenance class is needed, only honest selection and the original tick.
2. Spec 0017 §ORD-HARD-027: one freshness rule on the projection, both consumers use it; `observed_now` only for currently-perceived records; `remembered_belief` carries the ORIGINAL source observation tick. INV-028 read precisely: stale knowledge stays available to planning — as memory, not as perpetually-current observation.
3. Cross-artifact boundary under audit: the holder-known projection contract (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — one cognition source implies one freshness semantics) between `EpistemicProjection` and its two consumers (`no_human_surface`, `perception`).
4. INV-102/INV-026 restated: cognition inputs require provenance sufficient for replay and debug review; provenance records acquisition/observation time honestly. A fact stamped `observed_now` at a tick when no observation occurred is false provenance in the authoritative trace.
5. Fail-closed actor-knowledge surface and deterministic replay both touched: (a) the new provenance-class audit in `ActorDecisionTransaction::run` fails closed (typed `Stuck`, `blocker_code = provenance_class_mismatch` — extending `agent/trace.rs::BlockerCode`); (b) the freshness rule must be a deterministic function of (log prefix, decision tick, place) so `replay/rebuild.rs::rebuild_decision_context_hashes` re-derives identical contexts — decision traces and golden checksums shift where stale records downgrade, and replay byte-stability is re-established at the corrected classes. Knowledge availability is not reduced (no leakage direction change); only its class label becomes honest.
6. Adjacent contradiction classified: window-framing facts citing a non-witnessing frame event (spec ORD-HARD-031) live in the same fact-minting file but are a separate ticket (`0017PHA3ATICLED-004`, which depends on this one); this ticket changes record-derived facts only, not framing facts.

## Architecture Check

1. Putting the freshness rule on the projection accessor (one classification function taking the context's tick/place, returning records tagged currently-perceived vs remembered) is cleaner than two consumer-side filters: the divergence that caused this finding becomes structurally impossible, and `perception.rs` sheds its bespoke filter in favor of the shared rule. Behavior preference (observation over memory, if any selection logic weights it) shifts honestly with the class label — that shift is the correct behavior per spec §9.
2. No backwards-compatibility aliasing/shims: the unfiltered accessor signature is replaced (consumers must pass the context tick/place); no parallel "legacy" accessor remains.

## Verification Layers

1. INV-026 honest acquisition time -> new fixture `aged_food_record_surfaces_as_remembered_belief_not_observation_001`: food observed at tick N, actor leaves, decision at tick N+k ⇒ consumed fact is `remembered_belief` with acquisition tick N; replay byte-match.
2. INV-102 fail-closed class audit -> unit test: an `observed_now` fact whose cited source events all precede the actor's latest perception event for that place yields typed `Stuck(provenance_class_mismatch)` before proposal construction.
3. One freshness semantics -> codebase grep-proof + source guard: `perception.rs` and `no_human_surface.rs` both consume the shared projection rule; new guard bans direct `ActorKnownFact::observed_now` construction in `no_human_surface.rs` outside the currently-perceived branch (runtime audit as backstop).
4. INV-018 deterministic replay -> the context-hash gate (`rebuild_decision_context_hashes`) passes over every golden no-human run at the corrected classes; `cargo test --workspace`.

## What to Change

### 1. Freshness classification on the projection

Replace `actor_known_records_for_context` with an accessor that classifies each record against the context's decision tick and current place: *currently-perceived* (source observation within the actor's latest perception window for the relevant place) vs *remembered* (older, or other-place visibility-class sources). One rule, owned by `epistemics/projection.rs`.

### 2. Class-honest fact minting in `no_human_surface.rs`

`consume_projection_record` mints `observed_now` only for currently-perceived records; remembered records mint `remembered_belief` carrying the original source observation tick (from the record), not `self.decision_tick`.

### 3. Shared rule in `perception.rs`

`current_place_knowledge_context` consumes the same projection classification instead of its bespoke latest-window filter (assert existing staleness tests still pass — the rule generalizes, not weakens, that filter).

### 4. Provenance-class audit in the transaction

`ActorDecisionTransaction::run` (alongside the existing dangling-provenance check) fails closed with `BlockerCode::ProvenanceClassMismatch` when an `observed_now` fact's cited sources all precede the latest perception event for its place.

### 5. Fixture, guard, golden updates

New fixture (registered in `fixtures/mod.rs`); `anti_regression_guards.rs` gains the `observed_now`-construction guard; update golden trace/checksum expectations surfaced by the class corrections.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify — `BlockerCode` variant)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — trace/checksum expectations, as surfaced)

## Out of Scope

- Window-framing fact citations and facts↔structured reconciliation (ticket `0017PHA3ATICLED-004`).
- Workplace believed-access facts (ticket `-006`); replay payload materialization (ticket `-005`).
- Any reduction of what an actor knows — stale knowledge remains plannable as memory (INV-028).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content aged_food_record_surfaces_as_remembered_belief` — aged record downgrades with original tick; replay byte-match.
2. `cargo test -p tracewake-core provenance_class_mismatch` — fail-closed typed Stuck on misclassified observation.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. One freshness rule, owned by the projection, consumed by both the no-human surface and embodied perception; no consumer-side reimplementation.
2. Every `observed_now` fact in a decision trace is backed by a perception-window-current source event; everything else is `remembered_belief` with its true acquisition tick.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs` — multi-tick staleness fixture.
2. `crates/tracewake-core/src/agent/transaction.rs` (unit test) — `provenance_class_mismatch` fail-closed path.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — `observed_now`-construction guard.
4. `crates/tracewake-core/src/agent/perception.rs` — existing staleness test retargeted at the shared rule.

### Commands

1. `cargo test -p tracewake-core provenance_class`
2. `cargo test --workspace`

## Outcome

Completed 2026-06-10.

Replaced the unfiltered actor-known projection accessor with
`classified_actor_known_records_for_context`, which classifies projection records by current
place and source tick. Projection records now carry `source_tick`, visible food records
retain their observed place, and both `no_human_surface.rs` and
`perception.rs` consume the shared projection classification rather than owning separate
staleness logic.

The no-human surface now mints `observed_now` only through the freshness helper when a
record is currently perceived at the decision tick; aged projection records become
`remembered_belief` with the original source tick. Added the transaction backstop
`BlockerCode::ProvenanceClassMismatch` for stale facts mislabeled `observed_now`.

Added the adversarial fixture
`aged_food_record_surfaces_as_remembered_belief_not_observation_001`, a content test for
aged food knowledge, unit coverage for the no-human surface and transaction audit, fixture
census/source-guard updates, and a guard that projection-record consumption cannot directly
construct `ActorKnownFact::observed_now` outside the freshness helper.

Implementation note: the embodied current-place context still uses the latest
current-place projection window for affordance facts, but the shared projection
classification distinguishes that "latest known" record from a fact currently observed at
the decision tick. No golden checksum constants needed rebaselining; the workspace replay
and context-hash gates stayed stable.

Verification run:

1. `cargo test -p tracewake-content aged_food_record_surfaces_as_remembered_belief`
2. `cargo test -p tracewake-core aged_food_record_surfaces_as_remembered_belief_not_observation`
3. `cargo test -p tracewake-core provenance_class_mismatch`
4. `cargo test -p tracewake-core current_place_knowledge_context_uses_latest_projection_window_not_live_truth`
5. `cargo test -p tracewake-content --test fixtures_load`
6. `cargo test -p tracewake-content --test golden_fixtures_run`
7. `cargo test -p tracewake-core guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth`
8. `cargo test -p tracewake-core workspace_source_classification_census_matches_production_tree`
9. `cargo test -p tracewake-core every_blocker_category_serializes_and_round_trips`
10. `cargo fmt --all --check`
11. `cargo clippy --workspace --all-targets -- -D warnings`
12. `cargo build --workspace --all-targets --locked`
13. `cargo test --workspace`
