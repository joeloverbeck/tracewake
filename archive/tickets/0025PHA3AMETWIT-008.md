# 0025PHA3AMETWIT-008: Content-crate census closures — ID-recognizer parity and Location-embedded ID scanning

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `tracewake-content` validation (`validate.rs`) and content tests; no core/tui code.
**Deps**: None

## Problem

Spec 0025 findings `ORD-HARD-181` (low) and `ORD-HARD-182` (low). The ID-field
shortcut-scan census (`ORD-HARD-151`'s repair) recognizes ID newtypes via a
twelve-name hand list in `validate.rs::is_schema_id_field_type` (`FixtureId`,
`ActorId`, `PlaceId`, `DoorId`, `ContainerId`, `ItemId`, `ActionId`, `BeliefId`,
`SleepAffordanceId`, `FoodSupplyId`, `WorkplaceId`, `RoutineTemplateId`) — while
`schema.rs` already imports unlisted newtypes (`IntentionId`,
`RoutineExecutionId`, `CandidateGoalId`, `DecisionTraceId`): a future schema field
typed with one is neither classified as an ID nor demanded for shortcut scanning —
the same per-type-convention blindness `ORD-HARD-151` named, displaced one level
up (R-28). And IDs embedded in `Location` values are never shortcut-marker-scanned:
`validate_id_shortcut_markers` contains zero `location` references, so a fixture
authoring a marker-bearing ID solely inside a `Location` referent (e.g.
`item|coin|true|in:set_need_hunger`) reaches authored content unflagged (mitigated
only transitively: a *dangling* referent fails `bad_reference` first).

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: the twelve-name list confirmed
   verbatim in `is_schema_id_field_type` (`crates/tracewake-content/src/validate.rs`);
   the four unlisted newtypes confirmed imported by
   `crates/tracewake-content/src/schema.rs`; zero `location` references inside
   `validate_id_shortcut_markers`. All operator-verified per spec 0025 §8 (181's
   list contents and imports re-verified at reassessment).
2. Verified against spec 0025 §4 and doctrine: INV-097
   (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`),
   `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
   (content validation contract), R-28 (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`).
3. Shared boundary under audit: the schema-field → shortcut-scan census contract —
   every stable-ID-bearing position in authored fixtures must be scan-demanded,
   with the recognizer derived from the authoritative newtype population rather
   than a hand list.
4. Invariant restated before trusting the narrative (INV-097 — no-script
   compliance is tested): scenario content is inspected for hidden outcome chains
   and script markers; an ID-typed position exempt from the marker scan is an
   authoring channel the inspection misses.
5. Enforcement surface touched: fail-closed content validation. Both changes add
   scanning/rejections and relax nothing; no replay or actor-knowledge surface is
   involved.

## Architecture Check

1. Asserting the recognizer equals the set of `*Id` newtypes referenced by
   `schema.rs` (a parity census over the schema source) is cleaner than extending
   the hand list by four names: the census fails on the *next* unconsidered
   newtype, which is the actual defect class — the same derivation discipline as
   the `discover_schema_fields` census this extends. Scanning `Location`-yielded
   IDs through the existing `reject_text_by_policy` path reuses the proven
   predicate rather than minting a parallel one.
2. No backwards-compatibility aliasing/shims: the hand list either becomes the
   derived set or is parity-locked against it; no exempt legacy positions remain.

## Verification Layers

1. Recognizer parity (`ORD-HARD-181`) → census over `schema.rs`-referenced `*Id`
   newtypes vs `is_schema_id_field_type`; a synthetic unrecognized referenced
   newtype fails it.
2. Location descent (`ORD-HARD-182`) → a negative fixture with a shortcut-marker
   ID solely inside a `Location` referent fails with the typed
   `authored_shortcut_effect`/`authored_outcome_chain` code through
   `load_fixture_package`.
3. Census continuity → `schema_id_fields_are_classified_for_script_scanning` and
   its existing synthetic stay green (the `Location`-typed fields join the
   demanded set).
4. Whole-suite regression → `cargo test -p tracewake-content` and the four-gate
   workspace run.

## What to Change

### 1. Recognizer parity census (`ORD-HARD-181`)

Assert `is_schema_id_field_type`'s recognized set equals the `*Id` newtypes
referenced by `schema.rs` (derived from the schema source at test time); add the
unrecognized-newtype synthetic.

### 2. Location-embedded ID scanning (`ORD-HARD-182`)

`validate_id_shortcut_markers` scans the IDs yielded by `Location` values on
`ItemSchema.location` / `FoodSupplySchema.location` through the same
`reject_text_by_policy` predicate; teach the field census to descend
`Location`-typed fields; add the marker-in-location negative fixture.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify — marker-in-location load negative)

## Out of Scope

- Manifest/fingerprint work (`ORD-HARD-169`/`170`/`183`) — ticket
  `0025PHA3AMETWIT-004` (note: it also modifies `validate.rs` and
  `fixtures_load.rs`; coordinate the mechanical merge).
- Adding new ID newtypes or schema fields (census closure only).
- Any doctrine amendment.

## Acceptance Criteria

### Tests That Must Pass

1. The recognizer parity census passes against the current twelve + any
   schema-referenced newtypes it must absorb, and fails on the synthetic
   unrecognized newtype.
2. The marker-in-location negative fails through `load_fixture_package` with the
   typed code; existing ID-scan negatives stay green.
3. `cargo test -p tracewake-content` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. Every stable-ID-bearing position in authored fixtures — direct fields and
   `Location`-embedded referents — is shortcut-marker-scanned through one shared
   predicate.
2. The ID recognizer cannot drift from the schema's referenced newtype population
   without a census failure.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (`#[cfg(test)]`) — recognizer parity
   census + unrecognized-newtype synthetic.
2. `crates/tracewake-content/tests/fixtures_load.rs` — marker-in-location load
   negative.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

### Files Changed

- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-content/tests/fixtures_load.rs`

### Decisions

- Added `ItemSchema.location` and `FoodSupplySchema.location` to the ID scan
  registration census because `Location` embeds stable actor/place/container
  IDs.
- Reused the existing `reject_text_by_policy` path through a new
  `reject_location_ids_by_policy` helper for all `Location` variants.
- Kept the ID-newtype recognizer separate from `Location` field discovery and
  added parity against the `*Id` imports referenced by `schema.rs`.

### Acceptance Disposition

- ID-recognizer parity: completed; current schema-referenced `*Id` imports are
  recognized, and a synthetic `SyntheticFutureId` import fails the parity check.
- Location-embedded ID scanning: completed for item and food-supply locations.
- Marker-in-location negative: completed through `load_fixture_package` with
  `ValidationPhase::NoScript` and `authored_shortcut_effect` at the location ID
  path.
- Existing ID-scan negatives and content suites: passed.
- Deferred or dropped work: none.

### Verification

- `cargo test -p tracewake-content`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
