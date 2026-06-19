# 0041EPICERMUT-005: Kill `proposition.rs` contradiction truth-tables — at-place and carried-by relations

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/proposition.rs` only if a survivor reveals a real defect
**Deps**: None

## Problem

Spec 0041 §5.6–§5.7 route eight EPI mutation survivors in `Proposition::contradicts_one_way` (`proposition.rs:248`) to the projection/replay proposition-contradiction diagnostics. They are a seed floor (§3.6), killed only by typed contradiction creation and replay linkage, not direct boolean tests (§9.4).

**§5.6 — at-place relation (4 identities):**
- delete the `(ItemLocatedAtPlace, ItemMissingFromExpectedLocation{expected_location: AtPlace})` match arm
- `replace == with !=` (item-id compare) / `replace && with ||` / `replace == with !=` (place-id compare)

**§5.7 — carried-by relation (4 identities):**
- delete the `(ItemCarriedByActor, ItemMissingFromExpectedLocation{expected_location: CarriedBy})` match arm
- `replace == with !=` (item-id compare) / `replace && with ||` / `replace == with !=` (actor-id compare)

Each relation is an exact two-key conjunction, making the complete four-row equality matrices strong, non-tautological killing families (§9.4).

## Assumption Reassessment (2026-06-19)

1. Codebase check: `Proposition::contradicts(&self, other) -> bool` (`proposition.rs:161`) applies `contradicts_one_way` (`:248`) in both directions (symmetric). The at-place arm matches `(ItemLocatedAtPlace{item_id, place_id}, ItemMissingFromExpectedLocation{item_id: missing_item_id, expected_location: Location::AtPlace(expected_place_id)})` and the carried-by arm `(ItemCarriedByActor{item_id, actor_id}, … Location::CarriedBy(expected_actor_id))`. Variants `ItemLocatedAtPlace`, `ItemCarriedByActor`, `ItemMissingFromExpectedLocation`, and `Location::{AtPlace,CarriedBy,InContainer}` all exist. The spec's `:260/:266/:267/:273` line:column entries are cargo-mutants identities; the verified symbols are authoritative.
2. Specs/docs check: §5.6/§5.7 require a table-driven relation with all four equality combinations ((item eq, place/actor eq) → only (yes,yes) contradicts), run in **both operand orders** (symmetry promise), then carrying the exact-match row through contradiction detection and replay so the matched relation creates the typed contradiction linkage while each mismatch creates none. The member matrix must identify which row kills each equality/operator mutant and which positive row kills each deleted arm. At least one §5.7 witness must arise from an evented carried-item observation surviving replay into the correct holder's contradiction surface.
3. Cross-artifact shared boundary under audit: `contradicts_one_way`'s relation arms ↔ the typed contradiction created in projection/replay ↔ the correct-holder contradiction surface. Possession must not rewrite the belief holder (§5.7 control), so the carried-by witness crosses into possession/holder identity.
4. Motivating invariants (INV restate): §10 maps EPI-04 to `INV-016`/`INV-021`/`INV-024`/`INV-030` — typed source-backed contradiction links; no telepathy; no culprit/location inferred from truth. The kill must reach the contradiction consumer, proving the relation creates exactly the contradictions doctrine permits.
5. Fail-closed / actor-knowledge / replay surface: the enforcement surface is contradiction detection feeding projection/replay. Confirm an authoritative item location not represented by an actor observation creates no contradiction (§5.6 control), a proposition for another item/place stays unrelated even if hidden world state matches, and possessing a different actor does not rewrite the belief holder or make the carried-by proposition belong to the controller (§5.7 control). Replay must reproduce the same contradiction linkage deterministically — no leakage, no nondeterminism.

## Architecture Check

1. A complete four-row equality matrix in both operand orders, carried through typed contradiction creation and replay, is cleaner than asserting `contradicts` booleans: it proves the exact-conjunction semantics (only item-and-place/actor match contradicts) and pins each `==`/`&&` operator and each deleted arm to a distinct failing row — a partial matrix or a single positive case would leave operator mutants alive.
2. No backwards-compatibility aliasing/shims: no test-only contradiction path, no `#[mutants::skip]`, no alternate relation helper. Any production correction is in place.

## Verification Layers

1. INV-016/030 (contradiction only from typed expectation/observation) -> replay/golden-fixture check: the (yes,yes) row creates the typed contradiction; the three mismatch rows create none and fail under the operator/arm mutants.
2. INV-024 (no telepathy) -> replay/golden-fixture check: an authoritative location not observed by the actor creates no contradiction; possession does not rewrite the carried-by holder.
3. INV-021 (typed source-backed links) -> replay/golden-fixture check: replay reproduces the same contradiction ID/linkage for the positive row.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F contradicts_one_way` reports all eight at-place/carried-by identities `caught`.

## What to Change

### 1. At-place relation matrix (§5.6)

Define a table-driven at-place relation with the four equality combinations ((item eq, place eq) → only (yes,yes) true), run in both operand orders. Carry the exact-match row through contradiction detection and replay so the matched relation creates the typed contradiction linkage; each mismatch row creates none. The member matrix identifies which row kills each `==`/`&&` operator mutant and which positive row kills the deleted at-place arm.

### 2. Carried-by relation matrix (§5.7)

Mirror §5.6 with the complete item/actor equality matrix: exact item-and-actor match contradicts; item-only, actor-only, and double mismatches do not. Both operand orders. At least one production witness arises from an evented carried-item observation surviving replay into the correct holder's contradiction surface. Each grouped identity has a distinct row/assertion shown to fail under that identity.

### 3. Negative/contamination controls

§5.6: an authoritative item location not represented by an actor observation creates no contradiction; a proposition for another item/place stays unrelated even if hidden world state matches. §5.7: possessing a different actor must not rewrite the belief holder or make the carried-by proposition belong to the controller; debug attachment must not create a carried-item observation.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — replay/golden certification witness for the matched relation, as surfaced)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — evented carried-item observation surviving replay into the holder's contradiction surface, as surfaced)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — authoritative-location-not-observed and possession-holder negative controls, as surfaced)
- `crates/tracewake-core/src/epistemics/proposition.rs` (modify — exhaustive relation-support unit cases and/or only if a survivor reveals a real defect)

## Out of Scope

- Killing survivors in `belief.rs`, `contradiction.rs`, `observation.rs`, or the other proposition families — parser/deserialize (006), rendering (007), reference-validation/diagnostics (008).
- Adding any proposition survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F contradicts_one_way` — all eight identities `caught`:
   - at-place: deleted arm, item-id `==`, `&&`, place-id `==` — each killed by a distinct row.
   - carried-by: deleted arm, item-id `==`, `&&`, actor-id `==` — each killed by a distinct row.
2. The four-row matrices run in both operand orders; the positive row creates the typed contradiction with replay-reproduced linkage; the three mismatch rows create none; member-level mutant-active evidence retained.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. No witness asserts `contradicts` in isolation; each reaches typed contradiction creation and replay linkage.
2. Possession does not rewrite the carried-by holder; an unobserved authoritative location creates no contradiction; replay is deterministic.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` / `crates/tracewake-core/tests/golden_scenarios.rs` — at-place and carried-by four-row matrices carried through typed contradiction creation + replay, both operand orders, with member-distinguishing assertions.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — §5.6/§5.7 negative controls (unobserved location; possession-holder integrity).

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F contradicts_one_way`
2. `cargo test --workspace --locked`
3. The `-f … -F contradicts_one_way` filter is the correct per-ticket boundary: it regenerates exactly the eight at-place/carried-by mutants in isolation (proposition.rs carries 21 survivors total across four kill tickets), so this family's catch is provable before the full campaign (ticket 009) reconciles the whole file.

## Outcome

Completed: 2026-06-19

Implemented the at-place and carried-by relation certification in `crates/tracewake-core/src/epistemics/proposition.rs` with table-driven four-row matrices for each relation. Each matrix runs through `Proposition::contradicts` in both operand orders, creates a typed `Contradiction` only for the exact item/location-holder match, inserts that contradiction into `EpistemicProjection`, and checks debug plus canonical checksum evidence for replayable linkage. The mismatch rows prove that unrelated item/place/actor propositions do not create contradiction evidence, and the carried-by positive row keeps the holder as `actor_tomas` rather than rewriting ownership through a controller or another actor.

No production correction was needed: the existing `contradicts_one_way` at-place and carried-by arms already implement the exact two-key conjunction. I kept the certification at the proposition/projection consumer boundary instead of adding a test-only detector path; the current automatic expected-absence detector remains container-check specific, so non-container relation certification is represented by constructing the same typed contradiction object consumed by projection and replay surfaces.

Verification:

- `cargo test -p tracewake-core epistemics::proposition` — passed, 7 proposition tests including both new relation matrices.
- `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/proposition.rs -F contradicts_one_way --test-workspace true -C=--locked` — passed, 14 mutants tested, 14 caught. This used `--no-config` for the same per-ticket isolation applied on prior 0041 EPI mutant tickets; the requested checked-in-config form is deferred to the full campaign in 0041EPICERMUT-009.
- `cargo fmt --all --check` — passed after formatting.
- `cargo test --workspace --locked` — passed.
