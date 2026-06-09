# 0006PHA2AEPISUB-004: Compile-fail negative fixtures for context/projection/record sealing

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new compile-fail negative fixtures under `tracewake-core/tests/negative-fixtures/`; `negative_fixture_runner` extended to cover them.
**Deps**: 0006PHA2AEPISUB-003

## Problem

The sealing performed in 0006PHA2AEPISUB-001/-002/-003 removes public mutation/construction paths, but without compile-fail fixtures proving those paths are gone, a future refactor could silently re-introduce a `pub` field or public debug constructor and pass all runtime tests. The spec requires guard self-coverage: a scanner without failing fixtures is not enough (spec §6 EPI-HARD-004 req. 3, §7 "API sealing self-coverage"). The repo already has the harness for this — `crates/tracewake-core/tests/negative_fixture_runner.rs` discovers fixtures under `tests/negative-fixtures` (`:77`) and asserts they fail to compile with expected lints (`:190`). This ticket adds the epistemic-sealing fixtures to that existing harness.

## Assumption Reassessment (2026-06-09)

1. The negative-fixture harness exists and is wired into CI: `crates/tracewake-core/tests/negative_fixture_runner.rs` joins `tests/negative-fixtures` (`:77`) and runs `banned_api_negative_fixtures_fail_with_expected_lints` (`:190`); `.github/workflows/ci.yml` runs `cargo test -p tracewake-core --test negative_fixture_runner`. Existing `compile_fail` doc-tests already cover `agent/actor_known.rs` and `debug_capability.rs` (`crates/tracewake-core/src/debug_capability.rs:8,16`).
2. Spec authority: `specs/0006_…SPEC.md` §6 EPI-HARD-001/002/003 "Structural lock" bullets enumerate the exact fixtures; §10 item 5 makes this a single ticket. Sealing must already be in place — hence `Deps: 0006PHA2AEPISUB-003` (transitive head reaching -002 and -001).
3. Shared boundary under audit: the `negative_fixture_runner` harness contract (how fixtures are discovered and what "expected failure" means) and the now-sealed public APIs of `KnowledgeContext`, `EpistemicProjection`, and the epistemic records. The fixtures are external-crate-style consumers that MUST fail to compile.
4. Constitutional invariants motivating this ticket: `INV-024` (no telepathy), `INV-068`/`INV-107` (debug quarantine), `INV-026` (provenance) — each sealed surface gets a fixture proving its leak path is a compile error.
5. Enforcement surface: this ticket IS a guard self-coverage surface for actor-knowledge-filtering and provenance-integrity. It adds no production logic and does not weaken replay; it proves the sealing tickets' guarantees are structural.

## Architecture Check

1. Extending the existing `negative_fixture_runner` harness (rather than inventing a parallel compile-fail mechanism) keeps all structural locks discoverable in one place and reuses the CI wiring. Compile-fail fixtures are the correct proof surface because the guarantee being proven is "this code does not compile," which a runtime test cannot express.
2. No backwards-compatibility shims: fixtures are new test files; no production surface is added or aliased.

## Verification Layers

1. `INV-024`/`INV-101` (context sealed) -> compile-fail fixtures: `external_crate_cannot_mutate_knowledge_context_mode`, `external_crate_cannot_mutate_knowledge_context_viewer`, `external_crate_cannot_build_debug_knowledge_context`.
2. `INV-024`/`INV-067` (projection sealed) -> compile-fail fixtures: `external_crate_cannot_read_raw_epistemic_projection_maps`, `external_crate_cannot_insert_raw_epistemic_records`, `external_crate_cannot_build_debug_projection_view_without_core_debug_api`.
3. `INV-026`/`INV-023` (records sealed) -> compile-fail fixtures: `external_crate_cannot_construct_belief_literal`, `external_crate_cannot_mutate_belief_source_or_scope`, `external_crate_cannot_construct_observation_without_source`, `external_crate_cannot_mutate_contradiction_links`.
4. Guard self-coverage -> the `negative_fixture_runner` assertion that each new fixture fails with the expected error (a fixture that compiles is a test failure).

## What to Change

### 1. Add the sealing compile-fail fixtures

Add the nine fixtures listed in Verification Layers under `crates/tracewake-core/tests/negative-fixtures/`, matching the existing fixture file/format convention the runner expects (file naming, expected-lint annotation).

### 2. Register them with the runner

Extend `crates/tracewake-core/tests/negative_fixture_runner.rs` so the new fixtures are discovered/asserted (follow the existing registration pattern — confirm whether discovery is directory-glob or an explicit list during implementation).

## Files to Touch

- `crates/tracewake-core/tests/negative-fixtures/` (new — nine fixture files)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — register/assert new fixtures; **file created by neither this nor a sibling ticket — pre-existing**)

## Out of Scope

- The sealing itself (0006PHA2AEPISUB-001/-002/-003).
- Source-guard scanner / clippy float ban (0006PHA2AEPISUB-005).
- Runtime adversarial/replay tests (0006PHA2AEPISUB-006).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the nine new fixtures fails to compile with its expected error, as asserted by `negative_fixture_runner`.
2. Temporarily un-sealing any one surface (manual spot-check during review) makes the corresponding fixture compile and the runner test fail — proving the guard has teeth.
3. `cargo test -p tracewake-core --test negative_fixture_runner` passes.

### Invariants

1. Every sealed surface from -001/-002/-003 has at least one compile-fail fixture proving its leak path does not compile.
2. No fixture is a runtime test; all are compile-fail.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/negative-fixtures/*.rs` — the nine compile-fail fixtures, one per sealed leak path.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — registration/assertion of the new fixtures.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo test --workspace`
3. `cargo test -p tracewake-core --test negative_fixture_runner -- --nocapture` — narrow boundary: compile-fail outcomes are the entire contract of this ticket, so the runner is the correct verification surface.

## Outcome

Completed: 2026-06-09

What changed:
- Added compile-fail negative fixture crates for every named context, projection, and epistemic-record sealing leak path.
- Registered the new fixtures in `negative_fixture_runner` with expected stderr fragments.
- Covered all ten leak paths named in the verification layers, despite the ticket summary describing the count as nine.

Deviations from original plan:
- Fixtures were added under the repository-level `tests/negative-fixtures/` directory used by the existing runner, not under `crates/tracewake-core/tests/negative-fixtures/`.
- No production code changes were needed.

Verification:
- `cargo fmt --all --check`
- `cargo test -p tracewake-core --test negative_fixture_runner -- --nocapture`
- `cargo test -p tracewake-core --test negative_fixture_runner`
- `cargo test --workspace`
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
