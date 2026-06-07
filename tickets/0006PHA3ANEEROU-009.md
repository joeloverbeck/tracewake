# 0006PHA3ANEEROU-009: Content and routine-template validation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` schema + validation (`schema.rs`, `validate.rs`); forbidden-content tests
**Deps**: 0006PHA3ANEEROU-003, 0006PHA3ANEEROU-006

## Problem

Content fixtures encode ambitious Phase 3A contracts, but validation does not enforce that content stays possibility-space rather than script, nor that routine templates/assignments are well-formed against the new typed model. Spec §6 requires content validation: routine template step action IDs exist in the action registry; routine failure modes are typed/constrained to known values; routine templates cannot contain direct place/state set operations; routine assignments reference valid actors/templates/windows; homes/sleep places/food supplies/workplaces/day windows reference valid entities; food/work/sleep fixtures cannot assert final events as scripted outcomes; no fixture encodes hidden planner inputs as actor-known unless supported by initial belief/provenance; no fixture contains player/protagonist/quest/culprit flags; no-human day fixtures declare acceptance contracts that tests actually enforce.

## Assumption Reassessment (2026-06-07)

1. Content schema/validation confirmed at `crates/tracewake-content/src/schema.rs` and `crates/tracewake-content/src/validate.rs`; `crates/tracewake-content/tests/forbidden_content.rs` is the no-scripting guard test. Typed routine conditions land in 0006PHA3ANEEROU-003; routine-assignment instantiation semantics land in 0006PHA3ANEEROU-006 — this ticket validates both.
2. Spec §6 enumerates the validation rules above; §9 forbids encoding hidden truth as actor-known without provenance and forbids player/quest/culprit flags. §7 requires no-human day fixtures to declare acceptance contracts tests enforce (the enforcement lands in feature tickets + the capstone 0006PHA3ANEEROU-010).
3. Shared boundary under audit: the content schema/validation contract for routine templates, routine assignments, and Phase 3A fixtures — the gate between authored content and the engine. Consumers of the typed condition schema (0006PHA3ANEEROU-003) and the routine-assignment schema (0006PHA3ANEEROU-006) are validated here.
4. Motivating invariants (restated): INV-060 "No authored outcome chains", INV-061 "Authored causal machinery is required" (content creates possibility space, not guaranteed arcs), INV-097 "No-script compliance is tested", INV-036 "HTN methods are procedures, not story scripts".
5. Fail-closed validation surface touched: validation must be deterministic and blocking, reject unknown fields by default, refuse behavior-looking fields (direct place/state set operations, scripted final-event outcomes), and reject fixtures encoding hidden planner inputs as actor-known absent provenance — closing the no-leak gap at content-load time. Failing validation blocks the fixture (hard fail), not a warning. No epistemic-leakage or determinism path is introduced.
6. Schema validation: this ticket validates (not extends) the routine-template / routine-assignment / fixture schemas defined by 0006PHA3ANEEROU-003 and 0006PHA3ANEEROU-006. Consumers: all Phase 3A content fixtures (`crates/tracewake-content/src/fixtures/*`). Additive validation rules (new blocking checks); no existing valid fixture field is removed.

## Architecture Check

1. Enforcing the §6 rules at content-load time makes "content is possibility, not script" a structural guarantee rather than a reviewer's vigilance: a routine template with a direct place-set op, a scripted final event, or a hidden-input-as-actor-known field fails validation and cannot load. Validating action IDs against the registry catches dangling routine steps before they reach the planner.
2. No backwards-compatibility aliasing/shims: validation is fail-closed (blocking), with no override path for behavior-looking fields; unknown fields are rejected by default.

## Verification Layers

1. INV-060 / INV-097 (no authored outcome chains; no-script tested) -> schema validation: a fixture asserting a scripted final event or containing a player/quest/culprit flag fails validation (negative test in `forbidden_content.rs`).
2. INV-036 (well-formed methods) -> schema validation: a routine template step with an action ID absent from the registry, or a direct place/state set op, fails validation.
3. No-leak at load -> schema validation: a fixture encoding a hidden planner input as actor-known without belief/provenance fails validation.

## What to Change

### 1. Routine template/assignment validation

`validate.rs`: step action IDs exist in the action registry; failure modes typed/known; no direct place/state set ops; assignments reference valid actors/templates/windows; homes/sleep/food/workplace/day-window references resolve.

### 2. No-script + no-leak content checks

Reject fixtures asserting final events as scripted outcomes, encoding hidden planner inputs as actor-known without provenance, or containing player/protagonist/quest/culprit flags. Reject unknown fields by default.

### 3. No-human day fixture acceptance-contract declaration

Require no-human day fixtures to declare acceptance contracts in a validated form (enforcement of the contracts themselves lands in feature tickets + 0006PHA3ANEEROU-010).

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)

## Out of Scope

- The typed condition representation (0006PHA3ANEEROU-003) and routine-assignment instantiation (0006PHA3ANEEROU-006) — dependencies this ticket validates.
- Behavioral enforcement of fixture acceptance contracts at runtime (feature tickets + 0006PHA3ANEEROU-010).

## Acceptance Criteria

### Tests That Must Pass

1. A routine template with a step action ID not in the registry, or a direct place/state set op, fails validation (negative test).
2. A fixture asserting a scripted final event, a hidden-input-as-actor-known field, or a player/quest/culprit flag fails validation; unknown fields are rejected.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Content validation is deterministic, fail-closed, and blocking; behavior-looking and hidden-truth content fields cannot load.
2. Routine templates/assignments and Phase 3A fixtures validate against the typed model.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — negative tests for scripted outcomes, hidden-input-as-actor-known, player/quest flags.
2. `crates/tracewake-content/tests/fixtures_load.rs` — routine-template/assignment well-formedness + registry action-ID checks.

### Commands

1. `cargo test -p tracewake-content --test forbidden_content`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
