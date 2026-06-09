# 0005PHA1DOCCOD-003: Adversarial Phase 1 negative fixtures rejecting each later-phase action family

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` negative/forbidden-content test coverage (and optionally a package-level negative fixture).
**Deps**: 0005PHA1DOCCOD-002

## Problem

Once content loading is phase-scoped (`0005PHA1DOCCOD-002`), the boundary needs adversarial proof: a Phase 1 content package that attempts each later-phase action family must fail at content validation, before runtime, with a typed phase-boundary diagnostic — not pass silently because the action exists in an all-phase registry, and not fail only later at the pipeline. The current `forbidden_content.rs` negative tests prove no-script/quest/reward/player/truth-field rejection but do not prove Phase 1 rejection of later-phase action IDs (`crates/tracewake-content/tests/forbidden_content.rs` helper registers all phases at `:14-24`). This ticket adds the missing adversarial coverage for `ALIGN-REQ-003`.

## Assumption Reassessment (2026-06-09)

1. `crates/tracewake-content/tests/forbidden_content.rs` exists; its `registry()` helper (`:14-24`) currently registers all phases and its tests assert blocking `ContentValidationError`s for quest/reward/player/script constructs. After `0005PHA1DOCCOD-002`, fixtures are loaded under a declared `FixtureScope` and a Phase 1 load rejects later-phase action IDs.
2. Spec `ALIGN-REQ-003` requires at least one Phase 1 negative fixture per later-phase action family: `check_container`, `truthful_accuse_probe` (Phase 2A); `sleep`, `eat`, `work_block`, `continue_routine` (Phase 3A). It prefers a compact generated table test over six copy-paste fixtures, provided each family is verified individually, and the assertion must inspect the typed `ValidationPhase`/error code, not a display-string substring.
3. Shared boundary under audit: the content-validation phase-boundary contract established in `0005PHA1DOCCOD-002` (the typed `ValidationPhase`/code emitted when a Phase 1 fixture names a later-phase action). This ticket consumes that contract; it does not define it.
4. Invariants motivating this ticket: `INV-097` (no-script compliance is tested — adversarial, not friendly-only, gates), `INV-098` (harsh acceptance — the Phase 1 certification surface must include adversarial negatives), and `INV-105` (typed diagnostics — assertions bind to the typed phase/code, not display text).
5. Fail-closed-validation surface: this is the adversarial gate proving the `0005PHA1DOCCOD-002` rejection is blocking. Confirm each negative case fails at content validation (pre-runtime), the failure is a hard error, and no later-phase action ID slips through to pipeline acceptance. No epistemic-leakage or replay surface is touched (negative fixtures never reach the event log).

## Architecture Check

1. A table test that enumerates the six action families against a `FixtureScope::Phase1` load gives per-family coverage in one reviewable, low-duplication diff and stays honest about which families are proven. Asserting the typed `ValidationPhase`/error code (not a `"phase"` substring) prevents the gate from passing on an unrelated error and keeps the diagnostic substrate authoritative.
2. No backwards-compatibility shim: these are new negative tests; nothing is aliased or wrapped.

## Verification Layers

1. `INV-097` (adversarial boundary tested) -> schema validation: each of the six later-phase action IDs, placed in a `FixtureScope::Phase1` fixture affordance, yields a blocking `ContentValidationError`.
2. `INV-105` (typed diagnostic) -> codebase grep-proof: the test asserts the typed `ValidationPhase` variant and stable error code, with no `contains("phase")`-style assertion.
3. Routine-smuggling path -> schema validation: a Phase 1 routine step whose semantic base maps to a later-phase action fails with the same typed boundary code (covers `validate_routine_rules`, not only affordance parity).

## What to Change

### 1. Add a Phase 1 later-phase-rejection table test

In `crates/tracewake-content/tests/forbidden_content.rs`, add a test that, for each of `check_container`, `truthful_accuse_probe`, `sleep`, `eat`, `work_block`, `continue_routine`, builds a minimal `FixtureScope::Phase1` fixture containing that action in an affordance, loads it, and asserts a blocking `ContentValidationError` with the typed phase-boundary `ValidationPhase`/code. Verify each family individually (table-driven).

### 2. Add the routine-smuggling negative case

Add at least one case where a `FixtureScope::Phase1` routine step's semantic base maps to a later-phase action, asserting the same typed boundary code — proving the boundary holds through `validate_routine_rules`, not only `validate_action_registry_parity`.

### 3. (Optional) package-level negative fixture

If the repo's `negative_fixture_runner` consumes package-level negative fixtures, add `tests/negative-fixtures/phase1_rejects_later_phase_actions/` rather than inlining — only if that runner is the idiomatic home; otherwise keep the table test in `forbidden_content.rs`.

## Files to Touch

- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `tests/negative-fixtures/phase1_rejects_later_phase_actions/` (new — only if `negative_fixture_runner` is used for package-level negatives)

## Out of Scope

- The phase-scoped loader/validator mechanism itself (`0005PHA1DOCCOD-002`, dependency).
- The pipeline-level guard reachability test (`0005PHA1DOCCOD-004`) — this ticket proves rejection at *content validation*, before the pipeline.
- The source-level loader-registration guard (`0005PHA1DOCCOD-005`).
- CI wiring (`0005PHA1DOCCOD-006`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test forbidden_content` — the new per-family Phase 1 rejection cases and the routine-smuggling case pass.
2. `cargo test --workspace` — full suite green.

### Invariants

1. Each later-phase action family, placed in a `FixtureScope::Phase1` fixture, fails at content validation (pre-runtime) with a typed phase-boundary diagnostic (`INV-097`, `INV-098`).
2. No assertion relies on a display-string substring; all bind to the typed `ValidationPhase`/error code (`INV-105`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — table test over the six later-phase action families + a routine-smuggling case; typed-diagnostic assertions.

### Commands

1. `cargo test --locked -p tracewake-content --test forbidden_content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Added a Phase 1 scoped content-validation table test covering every later-phase action family: `check_container`, `truthful_accuse_probe`, `sleep`, `eat`, `work_block`, and `continue_routine`.
- Added a routine-smuggling negative test that forces an ordinary-workday routine fixture through `FixtureScope::Phase1` and asserts the routine step fails at content validation.
- Both new tests assert typed `ValidationPhase::ActionRegistryParity` plus stable code `phase_unsupported_action`; they do not assert display-text substrings.

Deviations from original plan:

- Kept the adversarial cases inline in `crates/tracewake-content/tests/forbidden_content.rs`; no package-level negative fixture was added because the existing forbidden-content suite is the idiomatic content validation home and exercises the pre-runtime validator directly.

Verification results:

- `cargo test --locked -p tracewake-content --test forbidden_content` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
