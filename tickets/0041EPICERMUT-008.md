# 0041EPICERMUT-008: Kill `proposition.rs` reference-validation and typed-diagnostic survivors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/proposition.rs` (and the content validation boundary) only if a survivor reveals a real defect
**Deps**: None

## Problem

Spec 0041 §5.12 and §5.13 route five EPI mutation survivors in proposition reference validation (`validate_location`, `require_place`, `require_container`) and typed parse/reference-error diagnostics (the two error `Display` impls) to the content/schema-validation and diagnostics layers. They are a seed floor (§3.6), killed only through the real content/fixture loading boundary and the typed diagnostic delivered to the acceptance/review surface — not by calling the private helpers or formatting the error directly (§5.12, §5.13).

**§5.12 — reference validation (3 identities):**
- `replace validate_location -> Result<(), PropositionReferenceError> with Ok(())`
- `replace require_place -> Result<(), PropositionReferenceError> with Ok(())`
- `replace require_container -> Result<(), PropositionReferenceError> with Ok(())`

**§5.13 — typed parse/reference diagnostics (2 identities):**
- `replace <PropositionReferenceError as Display>::fmt -> fmt::Result with Ok(Default::default())`
- `replace <PropositionParseError as Display>::fmt -> fmt::Result with Ok(Default::default())`

## Assumption Reassessment (2026-06-19)

1. Codebase check: `validate_location(...) -> Result<(), PropositionReferenceError>` (`proposition.rs:364`) dispatches `AtPlace → require_place`, `InContainer → require_container`; `require_place` (`:388`) and `require_container` (`:399`) return `PropositionReferenceError::UnknownPlace`/`UnknownContainer`. `Proposition::validate_references` (`:199`) calls these helpers. `PropositionReferenceError` (`:42`, variants `UnknownActor`/`UnknownContainer`/`UnknownItem`/`UnknownPlace`) and `PropositionParseError` (`:74`, variants incl. `InvalidLocationShape`/`InvalidShape`/`InvalidId`) each have a `Display` impl (`:49`, `:81`). The content boundary is `tracewake-content/src/validate.rs` (`validate_references` `:467`, `validate_locations` `:895`, `validate_location_reference` `:861`). The spec's `:370/:392/:403/:51/:83` entries are cargo-mutants identities; the verified symbols are authoritative.
2. Specs/docs check: §5.12 requires exercising reference validation through the real content/fixture loading boundary with a matrix — a valid proposition per affected reference shape; an expected-location proposition with an unknown place; one with an unknown container; a direct place-referencing proposition with an unknown place; a container-contents proposition with an unknown container; and a valid item with an invalid secondary reference (proving the validator does not stop after the item) — each invalid fixture failing closed before the projection with the exact typed error (`UnknownPlace`/`UnknownContainer`), fixture/content source, and responsible layer, valid controls loading/replaying, and a retained case failing under each of the three mutants. §5.13 requires one real unknown-reference failure and one real malformed-canonical-proposition failure through the content/replay boundary, inspecting the typed diagnostic (responsible layer; rejected fixture/record; stable error kind; implicated reference ID or canonical-shape category; the fact that no EPI record was committed), with the structured error kind preserved independently of wording.
3. Cross-artifact shared boundary under audit: the core reference-validation helpers ↔ `Proposition::validate_references` ↔ the content `validate.rs` loading boundary ↔ the typed diagnostic on the acceptance/review surface. The witness travels content load/schema rejection, not a bare helper call; the failed fixture must leave no partial EPI record.
4. Motivating invariants (INV restate): §10 maps EPI-05/02 to `INV-024` (no telepathy), `INV-013` (typed source links / evidence is not truth), and the provenance set `INV-009`–`INV-020`/`INV-021`–`INV-031` — provenance must be sufficient and semantically matching; unknown references fail closed and are not repaired from prose/debug/default. The kill must witness fail-closed rejection + an actionable typed diagnostic, not a helper return value.
5. Fail-closed validation / actor-knowledge / replay surface: the enforcement surface is content/fixture reference validation feeding (or rejecting before) the projection, plus the typed diagnostic. Confirm unknown references are not repaired from prose labels/debug truth/a similarly named entity/a default ID, a failed fixture leaves no partial observation/belief/proposition (§5.12 controls), diagnostics dump no hidden world state/debug-only context/secrets/other-actor-private records, and error handling never falls back to accepting the malformed proposition (§5.13 controls). Replay reproduces the rejection deterministically; the default/empty `Display` would make the review diagnostic non-actionable while the structured error kind must remain asserted independently.

## Architecture Check

1. Exercising the helpers through the content/fixture loading boundary (with a matrix that includes a valid-item-but-invalid-secondary-reference case) is cleaner than calling `validate_location`/`require_*` directly: it proves the `Ok(())` mutants admit a fixture that must fail closed and that the validator does not stop after the first reference, which a private-helper call cannot. Asserting the structured error kind independently of wording prevents a snapshot launder of the `Display` mutants.
2. No backwards-compatibility aliasing/shims: no test-only validation branch, no `#[mutants::skip]`, no acceptance of a malformed proposition via fallback. A redundant helper (if found) is removed with reconciliation.

## Verification Layers

1. INV-024/013 (fail-closed references; no prose/debug repair) -> schema validation: each invalid fixture is rejected before the projection with the exact `UnknownPlace`/`UnknownContainer` typed error; the `Ok(())` mutants admit it and the witness fails.
2. INV-009–020 (provenance sufficiency) -> replay/golden-fixture check: valid controls load and replay; a failed fixture leaves no partial EPI record.
3. Typed diagnostic actionability -> manual review: the default/empty `Display` makes the review diagnostic non-actionable; the structured error kind is asserted independently of prose.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F validate_location -F require_place -F require_container -F 'PropositionReferenceError::fmt' -F 'PropositionParseError::fmt'` reports the five identities `caught`.

## What to Change

### 1. Reference-validation matrix through the content boundary (§5.12)

Build a compact content/fixture matrix: a valid proposition per affected reference shape; an expected-location proposition with an unknown place; one with an unknown container; a direct place-referencing proposition with an unknown place; a container-contents proposition with an unknown container; and a valid item with an invalid secondary reference. Each invalid fixture fails closed before the live projection with the exact typed error (`UnknownPlace`/`UnknownContainer`), fixture/content source, and responsible validation layer; valid controls load and replay; a retained case demonstrably fails under each of the three `Ok(())` mutants.

### 2. Typed diagnostic inspection (§5.13)

Cause one real unknown-reference failure and one real malformed-canonical-proposition failure through the content/replay boundary, then inspect the typed diagnostic delivered to the acceptance/review surface: responsible layer, rejected fixture/record, stable error kind, implicated reference ID or canonical-shape category, and that no EPI record was committed. Preserve the structured error kind independently of wording (exact prose may be snapshot-tested).

### 3. Negative/contamination controls

§5.12: unknown references are not repaired from prose labels, debug truth, a similarly named entity, or a default ID; a failed fixture leaves no partial observation/belief/proposition. §5.13: diagnostics dump no hidden world state, debug-only context, secrets, or unrelated actor-private records; no fallback acceptance of a malformed proposition.

## Files to Touch

- `crates/tracewake-content/tests/schema_conformance.rs` (modify — reference-validation matrix through the content loading boundary, as surfaced)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify — prose/debug-repair and partial-record negative controls, reusing forbidden-provenance/prose-born-fact conventions, as surfaced)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — typed-diagnostic inspection for the malformed-proposition rejection, as surfaced)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — acceptance-surface typed diagnostic, as surfaced)
- `crates/tracewake-core/src/epistemics/proposition.rs` (modify — only if a survivor reveals a real defect or a redundant helper must be reconciled/removed)

## Out of Scope

- Killing the other proposition families — contradiction relations (005), parser/deserialize (006), rendering (007) — or `belief.rs`/`contradiction.rs`/`observation.rs`.
- Adding any proposition survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- Repairing unknown references through any prose/debug/default fallback (§5.12, §11.4).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F validate_location -F require_place -F require_container -F 'PropositionReferenceError::fmt' -F 'PropositionParseError::fmt'` — the five identities `caught`:
   - `validate_location`/`require_place`/`require_container -> Ok(())` — each killed by an invalid fixture that must fail closed.
   - both error `Display -> Ok(default)` — killed by a typed diagnostic asserting the structured error kind independently of wording.
2. The §5.12 matrix (incl. the valid-item-but-invalid-secondary-reference case) and §5.13 unknown-reference + malformed-proposition failures are retained with member-level mutant-active evidence; valid controls load and replay; failed fixtures leave no partial EPI record.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. Unknown references fail closed before the projection with the exact typed error; no prose/debug/default repair; no partial EPI record on failure.
2. The typed diagnostic preserves the structured error kind independently of wording and dumps no hidden/other-actor-private state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/schema_conformance.rs` / `crates/tracewake-content/tests/forbidden_content.rs` — the reference-validation matrix through the content loading boundary + prose/debug-repair and partial-record controls.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` / `crates/tracewake-core/tests/acceptance_gates.rs` — typed-diagnostic inspection for unknown-reference and malformed-proposition rejections with structured-error-kind assertions.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F validate_location -F require_place -F require_container -F 'PropositionReferenceError::fmt' -F 'PropositionParseError::fmt'`
2. `cargo test --workspace --locked`
3. The `-f … -F` filter is the correct per-ticket boundary: it regenerates exactly the validation/diagnostic mutants in isolation so this family's catch is provable before the full campaign (ticket 009) reconciles the whole file.
