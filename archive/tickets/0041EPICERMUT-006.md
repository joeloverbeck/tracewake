# 0041EPICERMUT-006: Kill `proposition.rs` canonical parser and location-deserialization arms

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/proposition.rs` only if a survivor reveals a real defect
**Deps**: None

## Problem

Spec 0041 §5.9–§5.10 route five EPI mutation survivors in the canonical proposition parser (`Proposition::from_str`, `proposition.rs:288`) and its location deserializer (`deserialize_location`, `proposition.rs:343`) to the projection/replay, parsing, proposal-construction, and view-model-rendering layers. They are a seed floor (§3.6), killed only by a round-trip on which the projection/proposal/view contract actually depends, not a parser-only "returns `Ok`" assertion (§5.9).

**§5.9 — canonical parser arms (3 identities):**
- delete arm `["item_carried_by_actor", item_id, actor_id]`
- delete arm `["container_contents_observed", container_id]`
- delete arm `["possible_movement_near_place", place_id]`

**§5.10 — expected-location deserialization arms (2 identities):**
- delete arm `"at_place"` in `deserialize_location`
- delete arm `"carried_by"` in `deserialize_location`

## Assumption Reassessment (2026-06-19)

1. Codebase check: `Proposition::from_str` (`proposition.rs:288`) has arms `["item_located_at_place", …]` (`:297`), `["item_carried_by_actor", item_id, actor_id]` (`:301`), `["container_contents_observed", container_id]` (`:306`), and `["possible_movement_near_place", place_id]`; `deserialize_location(value) -> Result<Location, PropositionParseError>` (`:343`) decodes `at_place`/`carried_by`/`in_container`; `PropositionParseError::InvalidLocationShape` exists (`:77`). Variants `ItemCarriedByActor`, `ContainerContentsObserved`, `PossibleMovementNearPlace`, and `Location::{AtPlace,CarriedBy,InContainer}` are present. The spec's `:301/:305/:319/:349/:351` entries are cargo-mutants identities; the verified symbols are authoritative.
2. Specs/docs check: §5.9 requires a parameterized canonical round-trip corpus over every `Proposition` variant (retained concrete cases for the three arms): construct typed via a legitimate event/fixture path → serialize to canonical bytes → deserialize through the **production** parser used by content/replay/evidence packaging → apply/rebuild through projection where the proposition participates in EPI state → compare typed semantics, source ancestry, checksum input, and the downstream action/view/contradiction consequence. Each named arm needs a concrete downstream witness: `item_carried_by_actor` → carried-item belief/expectation + possession evidence; `container_contents_observed` → expected-absence/contradiction only after the observation event; `possible_movement_near_place` → holder-known proposal/embodied surface without becoming truth. §5.10 requires round-tripping `ItemMissingFromExpectedLocation` for all three location forms, retained concrete cases for `at_place`/`carried_by`, replay reproducing the typed location + downstream contradiction linkage, and canonical reserialization matching original bytes.
3. Cross-artifact shared boundary under audit: the canonical proposition parser ↔ content/replay/evidence packaging consumers ↔ projection rebuild ↔ proposal/view consequence. The witness must travel the production parser, not a direct `from_str`/`deserialize_location` call divorced from its consumer.
4. Motivating invariants (INV restate): §10 maps these to `INV-018` (deterministic replay / canonical serialization) and `INV-021` (typed epistemic currency); `possible_movement_near_place` additionally protects `INV-024`/`INV-099`–`INV-106` (a possibility must reach the proposal/embodied surface without becoming certainty/truth). The deleted arm must break a round-trip the projection/proposal/view depends on.
5. Fail-closed / actor-knowledge / replay surface: the enforcement surface is canonical (de)serialization feeding projection/replay and proposal construction. Confirm unknown tags, wrong arity, invalid IDs, and prose-shaped inputs remain typed failures (no fallback parser or alias spelling, §5.9 control); missing `kind:id` structure, unknown location tags, empty IDs, and mismatched reference kinds fail with `PropositionParseError::InvalidLocationShape` or the precise typed ID error, and no unknown location silently becomes a default place/container/actor (§5.10 control). `possible_movement_near_place` must survive into the holder-known proposal/embodied surface without becoming truth. Replay reproduces the typed proposition deterministically — no leakage, no nondeterminism.

## Architecture Check

1. A full canonical `serialize → deserialize(production parser) → apply/rebuild → compare consequence` corpus is cleaner than a parser-only `Ok` assertion: it ties each deleted arm to the projection/proposal/view contract that actually consumes the round-trip, so deleting the arm breaks a real downstream witness rather than only a unit parse. Retaining concrete cases for the named arms (alongside any property exploration) guarantees member-level catch.
2. No backwards-compatibility aliasing/shims: no fallback parser, no alias spelling for compatibility, no `#[mutants::skip]`. Unknown inputs stay typed failures.

## Verification Layers

1. INV-018 (deterministic canonical round-trip) -> replay/golden-fixture check: each variant round-trips through the production parser and replay reproduces the typed proposition + canonical bytes.
2. INV-021/024 (typed currency; possibility-not-truth) -> replay/golden-fixture check: `possible_movement_near_place` reaches a holder-known proposal/embodied surface without becoming certainty; `container_contents_observed` enables contradiction only after the observation event.
3. Fail-closed parsing -> schema validation: unknown tags/arity/IDs and malformed locations fail with the precise typed error; no default substitution.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F from_str -F deserialize_location` reports all five identities `caught`.

## What to Change

### 1. Canonical round-trip corpus (§5.9)

Add a parameterized corpus over every `Proposition` variant with retained concrete cases for the three arms. For each case: construct via a legitimate event/fixture path; serialize to canonical bytes; deserialize through the production parser used by content/replay/evidence packaging; apply/rebuild through projection when the proposition participates in EPI state; and compare typed semantics, source ancestry, checksum input, and the downstream action/view/contradiction consequence. Provide the three named arms' concrete downstream witnesses (carried-item belief + possession; expected-absence after observation; holder-known proposal/embodied without truth).

### 2. Expected-location deserialization round-trips (§5.10)

Round-trip `ItemMissingFromExpectedLocation` through canonical bytes for `AtPlace`, `InContainer`, and `CarriedBy`, retaining separate concrete cases for the `at_place` and `carried_by` arms, then use each result in the appropriate contradiction relation. Replay reproduces the typed location + downstream contradiction linkage; canonical reserialization matches the original bytes.

### 3. Negative/contamination controls

§5.9: unknown tags, wrong arity, invalid IDs, and prose-shaped inputs remain typed failures; no fallback parser or alias spelling. §5.10: missing `kind:id`, unknown location tags, empty IDs, and mismatched reference kinds fail with `InvalidLocationShape`/precise typed ID error; no unknown location becomes a default.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — canonical round-trip + replay-reproduction corpus, as surfaced)
- `crates/tracewake-content/tests/schema_conformance.rs` (modify — typed-failure cases for unknown tags/arity/malformed locations, as surfaced)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — where serialized propositions enter fixture data, as surfaced)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify — only where the `possible_movement_near_place` proposition is already rendered)
- `crates/tracewake-core/src/epistemics/proposition.rs` (modify — exhaustive round-trip unit cases and/or only if a survivor reveals a real defect)

## Out of Scope

- Killing the other proposition families — contradiction relations (005), rendering (007), reference-validation/diagnostics (008) — or `belief.rs`/`contradiction.rs`/`observation.rs`.
- Introducing any fallback parser or alias spelling (§5.9, §11.4).
- Adding any proposition survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F from_str -F deserialize_location` — all five identities `caught`:
   - delete `item_carried_by_actor` / `container_contents_observed` / `possible_movement_near_place` arms — each killed by its concrete downstream round-trip witness.
   - delete `at_place` / `carried_by` deserialization arms — each killed by an expected-location round-trip + contradiction linkage.
2. Each named arm has a retained concrete case whose round-trip breaks under the deleted arm; replay reproduces typed proposition/location and canonical bytes; member-level evidence retained.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. The certification witness travels the production parser, not a direct `from_str`/`deserialize_location` call; `possible_movement_near_place` never becomes truth.
2. Unknown/malformed inputs stay typed failures with the precise error; no default substitution; replay is deterministic.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — canonical round-trip corpus over all variants (concrete cases for the three parser arms + two location arms) with apply/rebuild comparison.
2. `crates/tracewake-content/tests/schema_conformance.rs` / `crates/tracewake-content/tests/golden_fixtures_run.rs` — malformed/unknown typed-failure controls and fixture-data round-trips.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F from_str -F deserialize_location`
2. `cargo test --workspace --locked`
3. The `-f … -F from_str -F deserialize_location` filter is the correct per-ticket boundary: it regenerates exactly the five parser/deserialization mutants in isolation so this family's catch is provable before the full campaign (ticket 009) reconciles the whole file.

## Outcome

Completed: 2026-06-19

Implemented the parser/deserialization certification in `crates/tracewake-core/src/epistemics/proposition.rs`. The new canonical corpus round-trips every `Proposition` variant through `serialize_canonical` -> `deserialize_canonical` -> projection-consumed belief evidence, then asserts both canonical checksum input and debug belief rendering/stance. The retained concrete parser-arm witnesses cover `item_carried_by_actor`, `container_contents_observed`, and `possible_movement_near_place`; the possible-movement witness is kept as `Stance::Plausible`, so it reaches the projection surface without becoming truth.

Added expected-location round trips for `AtPlace`, `InContainer`, and `CarriedBy`, then fed the parsed missing-location proposition into typed contradiction creation and projection checksum/debug evidence. Added fail-closed controls for unknown tags, wrong arity, prose-shaped canonical input, missing `kind:id`, unknown location tags, and invalid stable IDs.

No production correction was needed: the existing parser and location deserializer already preserve the intended canonical forms. I kept the certification in the local proposition/projection consumer test module rather than spreading equivalent parser assertions into content/TUI tests, because the surviving mutants are all in `proposition.rs` and the projection checksum/debug consumer gives the required downstream evidence without adding alias or fallback paths.

Verification:

- `cargo test -p tracewake-core epistemics::proposition` — passed, 10 proposition tests including the new parser corpus and location round-trip contradiction linkage.
- `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/proposition.rs -F from_str -F deserialize_location --test-workspace true -C=--locked` — passed, 13 mutants tested: 11 caught, 2 unviable, 0 missed. This used `--no-config` for the same per-ticket isolation applied on prior 0041 EPI mutant tickets; the requested checked-in-config form is deferred to the full campaign in 0041EPICERMUT-009.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace --locked` — passed.
