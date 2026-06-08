# 0003PHA1SPIANT-011: Typed, provenance-bearing, actor/debug-separated diagnostics discipline

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (`actions/report.rs` actor/debug field separation if needed; diagnostic-discipline source-scan tests)
**Deps**: None

## Problem

Diagnostics are typed today — `ReasonCode::stable_id` (`actions/report.rs:49`), typed `CheckedFact`/`CheckedFactSource` (`:169`), and reports carry actor-visible and debug-only fields. Execution doctrine forbids display-string test authority (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:135-144`). But nothing guards against a future diagnostic losing its `ReasonCode`/`ValidationPhase`/`CheckedFact`/provenance, a test asserting on a display label instead of a typed reason, or debug-only facts leaking into actor-visible why-not. Spec `0003` §5.* / SPINE-AC-011 require enum/newtype-backed discipline plus tests that fail if a diagnostic loses typing or actor/debug separation.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/actions/report.rs` defines `ReasonCode` (`:10`) with `stable_id()` (`:49`), `CheckedFact` (`:169`), and reports carrying actor-visible / debug-only fields. Execution doctrine `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:135-144` forbids display-string test authority.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-011 mandates: tests assert reason-code enum/stable IDs and validation phases, not actor-facing sentences; actor-visible summaries may be snapshot-tested for UI stability but are not the sole semantic proof; debug-only facts are structurally separated from actor-visible facts in report structs; a test fails if a diagnostic loses `ReasonCode`/`ValidationPhase`/`CheckedFact`/provenance/source or actor/debug separation.
3. Boundary under audit: the typed-diagnostic contract in `actions/report.rs`, and the test discipline that proves diagnostics by type rather than display string. Shares `actions/report.rs` with 0003PHA1SPIANT-007 (source-context reasons) — coordinate mechanical merges.
4. INV motivating this ticket: `INV-070` (typed why-not / no display-string authority), `INV-105`/`INV-106` (typed provenance-bearing diagnostics), `INV-107` (debug separation; debug facts are non-diegetic). Restated: diagnostics are typed and provenance-bearing, and debug-only reasons never leak into the actor-visible why-not.
5. No-leak / typed-validation surface touched: the actor-visible vs debug-only report field split is a leakage boundary — a debug reason surfacing in the actor-visible why-not would leak non-diegetic information. The discipline test must assert the structural separation holds and that semantic assertions key on `ReasonCode`/`ValidationPhase`, never substrings. No determinism change.

## Architecture Check

1. A source-scan/test-discipline gate ("semantic assertions must reference `ReasonCode`/`stable_id`/`ValidationPhase`, never a forbidden actor-facing substring where a typed reason exists") plus a struct-level actor/debug separation assertion converts the typed-diagnostic doctrine into an enforced property, preventing the slow erosion where a convenient string assertion replaces a typed one.
2. No backwards-compatibility shim: no untyped diagnostic fallback is added; actor-visible strings remain presentation only, never semantic authority.

## Verification Layers

1. `INV-070` (typed why-not) -> codebase grep-proof: a discipline test fails on a semantic assertion that matches an actor-facing display substring where a typed `ReasonCode` exists.
2. `INV-105`/`INV-106` (typed provenance) -> compile/type check: diagnostics retain `ReasonCode`/`ValidationPhase`/`CheckedFact`/provenance fields.
3. `INV-107` (debug separation) -> manual review + test: debug-only facts are structurally separated from actor-visible facts and do not appear in actor-visible why-not.

## What to Change

### 1. Diagnostic-discipline source-scan test

Add `diagnostics_never_assert_display_label_as_authority`: scan test sources for forbidden substring/display-label semantic assertions where a typed reason exists.

### 2. Actor/debug separation assertion

Confirm (and lock with a test) that report structs structurally separate debug-only facts from actor-visible facts; add the separation if a field is currently shared.

### 3. Typed-field retention test

Add a test asserting a diagnostic that loses `ReasonCode`/`ValidationPhase`/`CheckedFact`/provenance/source or the actor/debug split fails.

## Files to Touch

- `crates/tracewake-core/src/actions/report.rs` (modify — actor/debug separation if needed; shared with 0003PHA1SPIANT-007)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `diagnostics_never_assert_display_label_as_authority` + typed-field retention)

## Out of Scope

- Source-context reason coverage (0003PHA1SPIANT-007).
- Content-validation diagnostics (0003PHA1SPIANT-010).
- Adding new reason codes or diagnostic fields beyond what separation requires.

## Acceptance Criteria

### Tests That Must Pass

1. `diagnostics_never_assert_display_label_as_authority` — fails on a substring/display-label semantic assertion where a typed reason exists.
2. A typed-field retention test fails if a diagnostic loses `ReasonCode`/`ValidationPhase`/`CheckedFact`/provenance/source or actor/debug separation.
3. `cargo test --workspace` passes.

### Invariants

1. Diagnostics are typed and provenance-bearing; display strings are never semantic authority (`INV-070`, `INV-105`).
2. Debug-only facts are structurally separated from actor-visible facts (`INV-107`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `diagnostics_never_assert_display_label_as_authority`, typed-field retention.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Added `diagnostics_never_assert_display_label_as_authority` to scan test sources for forbidden display-summary-as-authority patterns where typed `ReasonCode`/stable IDs should be used.
- Added `validation_report_keeps_typed_provenance_and_actor_debug_split` to lock `ValidationReport` fields for `PipelineStage`, `ReasonCode`, `CheckedFact`, `CheckedFactSource`, and separate actor-visible/debug-only fact vectors.
- Reused the existing report API; no runtime diagnostic shape change was needed.

Deviations from original plan:
- None. The existing actor/debug split was structurally present, so the ticket landed as guard coverage.

Verification:
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
