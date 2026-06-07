# 0007PHA3ASECHAR-011: Typed Phase 3A content schema & validation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` schema and validation (`schema.rs`, `validate.rs`, `manifest.rs`); routine/fixture contracts
**Deps**: 0007PHA3ASECHAR-001, 0007PHA3ASECHAR-002

## Problem

Phase 3A content must use typed schema fields and validation must fail closed against substring-only routine semantics and text-only behavioral proof (Spec 0007 §Content and validation, D-03). Routine templates, routine assignments, work windows, and known food/rest/work facts must use typed schema fields; validation must forbid routine-template names whose semantic meaning is only discoverable by substring, reject fixture contracts that assert capstone behavior by expected text only, and distinguish autonomous no-human events from manually forced action-unit events in fixture contracts.

## Assumption Reassessment (2026-06-07)

1. Confirmed surfaces: `crates/tracewake-content/src/schema.rs`, `crates/tracewake-content/src/validate.rs`, `crates/tracewake-content/src/manifest.rs`, and the fixture set under `crates/tracewake-content/src/fixtures/`. The typed routine `family` (0007PHA3ASECHAR-002) and the typed conditions (0007PHA3ASECHAR-001) are the engine surfaces this validation keys on. `forbidden_content.rs` (`crates/tracewake-content/tests/forbidden_content.rs`) is the existing fail-closed test harness.
2. Spec 0007 §Content and validation requires typed schema fields for routine templates/assignments/work windows/known facts; validation forbidding substring-only routine-template semantics; rejection of fixture contracts asserting capstone behavior by expected text only; and fixture contracts distinguishing autonomous no-human events from manually forced action-unit events. D-03 requires typed conditions/facts replace string shortcuts.
3. Shared boundary under audit: the content↔engine routine/condition schema. With 0007PHA3ASECHAR-001 (typed conditions) and 0007PHA3ASECHAR-002 (typed `family`) defining the engine representation, this ticket enforces them at content-load time and rejects behavior-looking or substring-only content.
4. Motivating invariants (restated): no-scripting / content-is-possibility (foundation 09; INV-061 "Authored causal machinery is required" — content creates possibility space, not guaranteed arcs); INV-060 "No authored outcome chains" — fixtures must not assert outcomes by authored text; validation discipline (fail-closed, blocking, rejects unknown/behavior-looking fields).
5. Fail-closed validation surface touched: the new checks must be deterministic and blocking, reject unknown fields by default, refuse behavior-looking fields, and name what failing means (block). A fixture asserting capstone behavior by `expected_events_or_reports` text only is rejected as a blocker, not a warning. No determinism or actor-knowledge change in the validator itself; it reads authored content.
6. Schema change: routine templates/assignments/work windows/known-fact contracts move to typed fields (typed `family`, typed conditions, typed known-fact kinds). Consumers: the fixtures under `crates/tracewake-content/src/fixtures/*`, the golden-fixture run harness (`tests/golden_fixtures_run.rs`), and the engine loaders. The change is **breaking** for any fixture still using string-coded routine semantics — intentional per D-03; such fixtures are updated in their owning tickets (002/004/007) and re-validated here.

## Architecture Check

1. Enforcing typed routine/condition/known-fact fields at content-load time makes the no-scripting and substring-free guarantees structural — a substring-only or behavior-looking routine cannot load — rather than relying on engine-side defensiveness. It extends the existing fail-closed `forbidden_content` harness rather than inventing a parallel validator.
2. No backwards-compatibility aliasing/shims: no string-coded routine-semantics path is accepted; unknown and behavior-looking fields are rejected, not coerced.

## Verification Layers

1. No-scripting / INV-060 (no authored outcome chains) -> schema validation: a fixture asserting capstone behavior by expected text only is rejected (blocking).
2. Substring-free routine semantics -> schema validation: a routine template whose family is discoverable only by substring (no typed `family`) is rejected at load.
3. Fail-closed discipline -> test (`forbidden_content.rs`): unknown and behavior-looking fields on routine/known-fact contracts are rejected with a named blocking failure.

## What to Change

### 1. Typed Phase 3A content schema

Extend `schema.rs` so routine templates/assignments, work windows, and known food/rest/work facts use typed fields (typed `family`, typed conditions, typed known-fact kinds), rejecting unknown fields by default.

### 2. Fail-closed Phase 3A validation

Add validation rules in `validate.rs` that block: routine-template names whose semantics are only substring-discoverable, fixture contracts asserting capstone behavior by expected text only, and fixture contracts that do not distinguish autonomous no-human events from manually forced action-unit events. Wire any new manifest entries in `manifest.rs`.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/manifest.rs` (modify)

## Out of Scope

- The engine-side typed conditions/family (0007PHA3ASECHAR-001/002, dependencies) — this ticket validates them, it does not define them.
- Updating individual fixtures' behavior (owned by 002/004/007) — this ticket re-validates them.
- The capstone fixture authoring (0007PHA3ASECHAR-012).

## Acceptance Criteria

### Tests That Must Pass

1. A validation test: a routine template lacking a typed `family` (substring-only semantics) is rejected at load with a blocking failure.
2. A validation test (`forbidden_content.rs`): a fixture asserting capstone behavior by `expected_events_or_reports` text only is rejected; an unknown/behavior-looking routine field is rejected.
3. A validation test: a fixture contract that does not distinguish autonomous no-human events from manually forced action-unit events is rejected.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Phase 3A content uses typed fields; unknown and behavior-looking fields are rejected by default.
2. Validation is deterministic, blocking, and names what failing means.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — substring-only family, text-only behavioral proof, unknown/behavior-looking field rejections.
2. `crates/tracewake-content/src/validate.rs` — unit tests: typed-field acceptance, blocking failures.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

Changed behavior:
- Added no-human fixture contract metadata validation that rejects text-only behavior proof and contracts that do not exclude manually forced action-unit events.
- Converted the canonical Phase 3A no-human day contract to structured `autonomous_no_human_event=` and `log_derived_metric=` rows.
- Extended raw content validation so unknown routine-like sections containing behavior/script shortcuts are blocked as no-script authored shortcut effects.
- Added fail-closed tests for missing typed routine `family`, behavior-looking routine fields, text-only capstone contract proof, and ambiguous no-human/manual action-unit proof.

Deviations:
- No schema struct migration was needed; routine templates, assignments, day windows, and Phase 3A facts were already typed from earlier tickets. This ticket enforced those shapes through load/contract validation.

Verification:
- `cargo test -p tracewake-content --test forbidden_content`
- `cargo test -p tracewake-content --test fixtures_load`
- `cargo test -p tracewake-content --test golden_fixtures_run no_human_day_fixture_has_roster_activity_and_metrics_envelope`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
