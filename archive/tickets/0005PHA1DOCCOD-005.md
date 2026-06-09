# 0005PHA1DOCCOD-005: Source-level regression guard — Phase 1 loader cannot register later-phase action families

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` anti-regression / negative-fixture guard coverage (defense-in-depth source guard).
**Deps**: 0005PHA1DOCCOD-001, 0005PHA1DOCCOD-002

## Problem

The structural fix (`0005PHA1DOCCOD-001` typed scope, `0005PHA1DOCCOD-002` scope-keyed loader) prevents a Phase 1 load from seeing later-phase actions. `ALIGN-REQ-006` asks for a subordinate source-level regression guard as defense in depth: a self-tested guard that fails if a Phase 1 fixture loader directly calls `register_phase2a_*`/`register_phase3a_*`, or if a generic action constructor hard-codes Phase 1 scope for a non-Phase1 registration. This catches a future maintainer re-introducing the original `ALIGN-001` shortcut even if the type-state is locally weakened. The guard must be negative-fixture-backed (mutation-tested) so it cannot become ceremonial, and it must be explicitly secondary to the compile-time scoping.

## Assumption Reassessment (2026-06-09)

1. `crates/tracewake-core/tests/anti_regression_guards.rs` and `crates/tracewake-core/tests/negative_fixture_runner.rs` exist and run in the CI lock-layer job (`.github/workflows/ci.yml:81`,`:84`). The later-phase registration fns are `register_phase2a_epistemics`, `register_phase3a_sleep`/`eat`/`work`/`continue_routine` (`crates/tracewake-core/src/actions/registry.rs:111-147`). The content loader after `0005PHA1DOCCOD-002` registers later-phase families only under a later-phase `FixtureScope`.
2. Spec `ALIGN-REQ-006` requires the guard to be source/test-level, negative-fixture-backed (self-coverage proving the guard fails when the rule is violated), and explicitly subordinate to compile-time phase typing — "It must not be the only enforcement mechanism."
3. Shared boundary under audit: the loader→registry call surface — specifically that the Phase 1 load path does not call later-phase `register_*` fns. The guard codifies the end state that `0005PHA1DOCCOD-002` establishes, which is why this depends on both `-001` and `-002`.
4. Invariants motivating this ticket: `INV-097` (no-script/boundary compliance is tested), `INV-098` (acceptance is harsh — guard must be self-proving), `INV-105` (the guard's verdict is structurally inspectable).
5. Fail-closed surface: the guard is a blocking test, not a warning. Its self-coverage (a deliberate violation that the guard must catch) is what keeps a grep-style scanner from silently passing. No epistemic-leakage or replay surface is touched — this is a static/source structural check.
6. Adjacent contradiction classification: if the type-state from `0005PHA1DOCCOD-001`/`-002` already makes a Phase-1-loader call to `register_phase2a_*` a compile error, this guard is redundant-but-required defense in depth per `ALIGN-REQ-006`; it is not future cleanup. Document that subordination explicitly in the guard rather than dropping it.

## Architecture Check

1. A self-tested source guard (with a negative fixture proving it fires on violation) backstops the compile-time boundary against a future weakening, at low cost, without becoming a primary mechanism. Mutation-backing (a deliberate rule violation the guard must catch) prevents the guard rotting into a no-op grep.
2. No backwards-compatibility shim: the guard is additive test coverage; it introduces no production indirection and no fallback path that re-admits later-phase registration in a Phase 1 context.

## Verification Layers

1. `INV-097`/`INV-098` (boundary regression guarded) -> codebase grep-proof / source guard: a test asserts the Phase 1 loader path contains no call to `register_phase2a_*`/`register_phase3a_*` and that no generic constructor hard-codes Phase 1 scope for a later-phase registration.
2. Guard self-coverage -> negative-fixture / mutation check: a deliberate violation (a fixture or tiny test-crate snippet that registers a later-phase family into a Phase 1 loader) is demonstrated to make the guard fail, proving the guard is not ceremonial.
3. Subordination -> manual review: the guard's own comments/docs state it is secondary to the compile-time `ActionScope`/`FixtureScope` typing.

## What to Change

### 1. Add the source-level regression guard

In `crates/tracewake-core/tests/anti_regression_guards.rs`, add a guard that fails if the Phase 1 fixture-load path directly calls `register_phase2a_*`/`register_phase3a_*`, and if a generic action constructor hard-codes `ActionScope::Phase1` for a non-Phase1 registration. Implement as the repo's existing guard style (source scan or structural assertion consistent with the file's current guards).

### 2. Back the guard with self-coverage

Add a negative fixture / mutation case (in `crates/tracewake-core/tests/negative_fixture_runner.rs` or alongside the guard) demonstrating that a deliberate violation is caught by the guard, so the guard cannot silently pass.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/...` (new — only if the repo keeps source-scanner negative fixtures there and the runner consumes them)

## Out of Scope

- The compile-time `ActionScope` typing (`0005PHA1DOCCOD-001`) and content scope routing (`0005PHA1DOCCOD-002`) — this guard is subordinate to them.
- Content-validation and pipeline rejection coverage (`0005PHA1DOCCOD-003`/`-004`).
- CI wiring (`0005PHA1DOCCOD-006`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test anti_regression_guards` — the loader-registration guard passes.
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — the guard self-coverage case passes (guard fires on the deliberate violation).
3. `cargo test --workspace` — full suite green.

### Invariants

1. The Phase 1 loader path contains no direct later-phase registration call; the guard proves this and is itself proven to fail on violation (`INV-097`, `INV-098`).
2. The guard is documented as subordinate to the compile-time phase typing (`INV-105`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — Phase 1 loader-registration guard.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — guard self-coverage (mutation/negative case).

### Commands

1. `cargo test --locked -p tracewake-core --test anti_regression_guards && cargo test --locked -p tracewake-core --test negative_fixture_runner`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed on 2026-06-09.

Changes:

1. Added a source-level Phase 1 loader guard in `crates/tracewake-core/tests/anti_regression_guards.rs` that scans the content loader and fails if `load_fixture_package` or the `FixtureScope::Phase1` arm directly calls later-phase registration functions.
2. Kept the generic-constructor guard explicit by asserting `ActionDefinition` constructors do not hard-code `ActionScope::Phase1`.
3. Added mutation self-coverage in both `anti_regression_guards.rs` and `negative_fixture_runner.rs`, feeding the scanner a deliberately bad Phase 1 loader snippet and asserting the guard fires.

Deviations:

1. The self-coverage uses an in-memory mutated loader fixture rather than a new standalone Cargo negative fixture, matching the repo's existing source-scan guard style while still proving the guard fails on the targeted mutation.
2. The guard is explicitly documented in its assertion as secondary to `ActionScope`/`FixtureScope` typing, not as the primary enforcement mechanism.

Verification:

1. `cargo test --locked -p tracewake-core --test anti_regression_guards`
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
