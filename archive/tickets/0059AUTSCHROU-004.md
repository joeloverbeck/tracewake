# 0059AUTSCHROU-004: Anti-regression guards + synthetic negatives

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds source/behavior anti-regression guards and synthetic negative fixtures (test-only)
**Deps**: 0059AUTSCHROU-001, 0059AUTSCHROU-002

## Problem

Even after 001/002 bind autonomous routine family to the active intention, the `ActorDecisionTransactionInput::routine_window_family` interface remains a live drift vector: a future scheduler, debug path, no-human path, or renamed helper could reintroduce a clock/window-keyed family selector that bypasses cognition. 0059 §7.4 requires standing anti-regression tripwires — at least one source guard and one behavioral guard, plus synthetic negative fixtures proving the guards are live — so the corrected binding cannot silently regress.

## Assumption Reassessment (2026-07-01)

1. The guard/synthetic-negative home is `crates/tracewake-core/tests/anti_regression_guards.rs` (verified — it already hosts `guard_*` functions and `negative_id: "synthetic_*"` entries, e.g. the 0057/0058 meta-lock guards). The behaviors the guards lock are produced by 001 (scheduler producer rebind) and 002 (transaction fail-closed), so both must land first.
2. Spec `specs/0059_…_SPEC.md` §7.4 names the guard functions (`guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention`, `guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding`, `guard_0059_synthetic_negative_census_is_live`) and the synthetic identifiers (`synthetic_0059_window_keyed_routine_family`, `synthetic_0059_eligible_execution_min_by_start`, `synthetic_0059_routine_window_family_without_active_intention`, `synthetic_0059_conflicting_routine_window_hint`, `synthetic_0059_other_actor_execution_temptation`). It also states source guards are anti-drift tripwires only; behavioral evidence remains primary (owned by 003).
3. Shared boundary under audit: the regression surface of the autonomous routine-family producer/consumer seam. The source guard inspects `scheduler.rs` for a reintroduced clock/window-keyed family selector not visibly bound to `active_intention_for_actor` / `active_intention_by_actor`; the behavioral guard drives the synthetic conflicting-window cases and asserts no routine-family candidate/action is selected ahead of the active intention.
4. Motivating invariant restated: **INV-098 — Feature acceptance is harsh** (regression-tested) together with **INV-103/INV-104** as the properties the guards protect. The guard set is the standing lock half of 0059's fix-plus-lock posture.
5. Enforcement surface audited: the guards read the same fail-closed / actor-knowledge / replay surface 001/002 establish and 003 exercises; they introduce no leakage or nondeterminism (synthetic negatives are deterministic fixtures; the source guard is a static grep-style assertion over committed source). The synthetic-negative census guard (`guard_0059_synthetic_negative_census_is_live`) proves the negatives actually fail when the forbidden bypass is present, so the guards are not vacuous.

## Architecture Check

1. Pairing a source guard (cheap, catches an obvious textual reintroduction of a clock-keyed selector) with a behavioral guard (drives the synthetic conflicting-window cases and asserts cognition wins) plus a census guard (proves the negatives are live) gives defense in depth without making the source guard load-bearing on its own — exactly the §7.4 posture that source guards are tripwires and behavior is primary.
2. No backwards-compatibility aliasing/shim (test-only additions following the established `guard_*` / `synthetic_*` convention already in `anti_regression_guards.rs`).

## Verification Layers

1. INV-103 (scheduler not cognition authority) -> codebase grep-proof (source guard): fails if `scheduler.rs` reintroduces a `start_tick`/`execution_id`-keyed routine-family selector not bound to the active-intention chain.
2. INV-104 (no direct dispatch) -> behavioral guard: fails if any synthetic conflicting-window case produces a routine-family candidate or action selection ahead of the active intention.
3. INV-098 (regression-tested) -> manual review + census guard: `guard_0059_synthetic_negative_census_is_live` confirms each `synthetic_0059_*` negative is registered and fails under the forbidden bypass (no vacuous guard).

## What to Change

### 1. Add the source and behavioral guards

In `crates/tracewake-core/tests/anti_regression_guards.rs`, add `guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention` (source guard: assert the autonomous family producer is bound to `active_intention_for_actor`/`active_intention_by_actor`), `guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding` (source guard: no `min_by(start_tick, …)`/`execution_id`-keyed family selection on the producer path), and `guard_0059_synthetic_negative_census_is_live` (behavioral/census guard).

### 2. Register the synthetic negatives

Add the five `synthetic_0059_*` negative identifiers using the established `negative_id` convention, each modeling a forbidden bypass (`window_keyed_routine_family`, `eligible_execution_min_by_start`, `routine_window_family_without_active_intention`, `conflicting_routine_window_hint`, `other_actor_execution_temptation`), and prove the behavioral guard fails when any is present.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Production changes (001/002) and the primary metamorphic behavioral suite (003) — the guards are tripwires, not the primary proof.
- Focused mutation (005) and the acceptance artifact (006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — the three `guard_0059_*` guards pass against the post-001/002 tree.
2. `cargo test -p tracewake-core --test anti_regression_guards guard_0059_synthetic_negative_census_is_live` — the census guard confirms all five `synthetic_0059_*` negatives are registered and live (fail under the forbidden bypass).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the four local gates pass.

### Invariants

1. The source guard fails on any reintroduced clock/window-keyed routine-family selector not bound to the active intention (INV-103).
2. Each of the five `synthetic_0059_*` negatives is live (census guard), so no guard is vacuous (INV-098).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — three `guard_0059_*` functions + five `synthetic_0059_*` negative identifiers; rationale: standing anti-drift lock for the 0059 producer/consumer seam.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards` — the guards file is the correct verification boundary for this deliverable (verified target: 92 existing tests list cleanly).
2. `cargo test --workspace` — full-pipeline confirmation the new guards integrate without disturbing sibling guard suites.

## Outcome

Completed: 2026-07-01

Implemented the 0059 anti-regression lock in `crates/tracewake-core/tests/anti_regression_guards.rs` by adding the three spec-named `guard_0059_*` tests, registering the guard entries in the meta-lock registry, and adding live synthetic negative checks for all five required `synthetic_0059_*` identifiers. The source guards now reject a scheduler routine-family producer that returns a family from a clock/window branch before active-intention authority or reintroduces a `min_by`/`execution_id` routine-execution selector. The census guard checks current transaction/generation sources and proves the no-active, conflicting-hint, other-actor, window-keyed, and eligible-execution synthetic bypasses are live.

Verification passed:

- `cargo test -p tracewake-core --test anti_regression_guards guard_0059`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo test -p tracewake-core --test anti_regression_guards guard_0059_synthetic_negative_census_is_live`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
