# 0057EMBROUCON-004: Marker invariants preserved — the marker is not progress

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — guard tests in `tracewake-core` (`actions/defs/continue_routine.rs`, `tests/anti_regression_guards.rs`) pinning that the `continue_routine` marker stays non-progress and that the committed follow-on is the progress of record
**Deps**: 0057EMBROUCON-002

## Problem

Spec 0057 §4.4 and §1.2. The fix wires a follow-on commit behind `Continue routine` (0057EMBROUCON-002), and a careless implementation could be tempted to flip the marker itself to "count as progress." Doctrine forbids that: `docs/2-execution/06:181` ("A pure `continue_routine` marker is not behavioral progress") and `docs/1-architecture/04:91` (do not count "continue routine" markers as progress unless an ordinary follow-on action or explicit modeled wait/failure is committed). This ticket installs guard tests so `continue_routine` keeps `behavioral_progress=false` / `intention_mutated=false`, no code counts the marker as progress, and the progress of record is the committed follow-on — asserted by event ancestry to the marker's `intention_id` and the routine step.

## Assumption Reassessment (2026-06-30)

1. Marker definition verified: `crates/tracewake-core/src/actions/defs/continue_routine.rs:11` `build_continue_routine_event` emits `EventKind::ContinueRoutineProposed` (`:53`) with payload `intention_mutated=false` / `behavioral_progress=false` (`:72-73`) and effects-summary "continue routine marker only; behavioral progress requires next ordinary action" (`:82`). Existing tests `continue_produces_next_action_without_mutating_intention` (`:360`) and `continuation_success_requires_follow_on_ordinary_action_ancestry` (`:492`) already assert the marker-only semantics; this ticket adds the post-0057EMBROUCON-002 guard that the *committed follow-on* (not the marker) is the progress of record, by ancestry.
2. Spec assumption: `specs/0057_…_SPEC.md` §4.4 governs; the doctrine anchors are `docs/2-execution/06` (marker-not-progress) and `docs/1-architecture/04` (markers do not count as progress without a committed follow-on) — both verified present at reassessment. 0057EMBROUCON-006 sharpens those docs; this ticket guards the code property they describe.
3. **Cross-artifact boundary under audit**: the marker event's payload contract (`behavioral_progress` / `intention_mutated`) versus the follow-on's status as progress of record — the guard asserts the marker payload is unchanged while the follow-on carries the behavioral progress, linked by ancestry to the marker's `intention_id` and routine step.
4. INV-001 (causality before drama — progress arises from a committed modeled action, not a marker relabel), INV-009 (meaningful state changes require events — the follow-on, not the marker, is the eventful progress); doctrine `docs/2-execution/06` + `docs/1-architecture/04` (marker-is-not-progress). Restated so the guard pins the rule, not the ticket narrative.
5. **Deterministic-replay / payload enforcement surface**: the `ContinueRoutineProposed` event payload and the ancestry linkage. Confirm the guard asserts the marker payload (`behavioral_progress=false`, `intention_mutated=false`) is byte-stable and that the follow-on's progress is established by replay-reconstructable event ancestry to the marker's `intention_id` / routine step — no nondeterministic linkage, no actor-knowledge leakage (the assertion reads committed events only).

## Architecture Check

1. A standing guard test (rather than trusting review) is the correct mechanism because the temptation 0057EMBROUCON-002 introduces is precisely to "make the marker count" — a regression that would pass any test asserting only that progress *happened*. Pinning `behavioral_progress=false` on the marker AND requiring the follow-on to carry progress by ancestry catches that regression directly.
2. No backwards-compatibility aliasing or shims: this is test-only; it adds no production code path and wraps nothing.

## Verification Layers

1. `docs/2-execution/06` + `docs/1-architecture/04` (marker-is-not-progress) -> codebase grep-proof + unit assertion: the committed `ContinueRoutineProposed` event still carries `behavioral_progress=false` / `intention_mutated=false` after 0057EMBROUCON-002.
2. INV-009 (eventful progress) -> ancestry assertion: behavioral progress is carried by the committed follow-on ordinary event, linked by ancestry to the marker's `intention_id` and routine step — not by the marker.
3. INV-001 (no relabel-as-progress) -> anti-regression guard: no code path reports the marker as behavioral progress (the guard fails if the marker payload flips or the follow-on ancestry is absent).

## What to Change

### 1. Marker-payload guard

In `crates/tracewake-core/src/actions/defs/continue_routine.rs` tests, add/extend a guard asserting the committed marker keeps `behavioral_progress=false` and `intention_mutated=false` after the 0057EMBROUCON-002 follow-on wiring lands.

### 2. Progress-of-record-by-ancestry guard

In `crates/tracewake-core/tests/anti_regression_guards.rs`, add a guard that the behavioral progress of a `Continue routine` submission is the committed follow-on ordinary event (linked by ancestry to the marker's `intention_id` / routine step), and that the marker alone is never counted as progress.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify — marker-payload guard test)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — progress-of-record-by-ancestry guard; shares this file with 0057EMBROUCON-003, append only)

## Out of Scope

- The follow-on commit wiring (0057EMBROUCON-002, depended on).
- The typed-block / stuck eligibility behavior (0057EMBROUCON-003).
- The doctrine-doc sharpening of the marker-is-not-progress rule (0057EMBROUCON-006).
- Any change to the marker's payload or effects-summary — this ticket pins them unchanged.

## Acceptance Criteria

### Tests That Must Pass

1. Marker-payload guard: the committed `ContinueRoutineProposed` event carries `behavioral_progress=false` and `intention_mutated=false` after 0057EMBROUCON-002.
2. Progress-of-record guard: a `Continue routine` submission's behavioral progress is the committed follow-on ordinary event, established by ancestry to the marker's `intention_id` / routine step; the marker alone is never progress.
3. `cargo test --locked -p tracewake-core` — both guards green alongside the existing marker-semantics tests.

### Invariants

1. `continue_routine` keeps `behavioral_progress=false` / `intention_mutated=false`; no code counts the marker as progress.
2. The progress of record is the committed follow-on, asserted by ancestry to the marker's `intention_id` and routine step.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/continue_routine.rs` — marker-payload guard (extends the existing `continue_produces_next_action_without_mutating_intention` family).
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — progress-of-record-by-ancestry guard.

### Commands

1. `cargo test --locked -p tracewake-core continue_routine` — marker-payload guard.
2. `cargo test --locked -p tracewake-core --test anti_regression_guards` — progress-of-record-by-ancestry guard.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — four-gate suite.

## Outcome

Completed: 2026-06-30

Installed marker-invariant guards without changing production marker behavior:

- Added `continue_marker_payload_contract_stays_non_progress` in `continue_routine.rs`, asserting `ContinueRoutineProposed` keeps `intention_mutated=false`, `behavioral_progress=false`, `routes_through_shared_pipeline=true`, the expected next action id, proposal-only cause, and the marker-only effects summary.
- Added `guard_0057_continue_routine_progress_of_record_is_follow_on` to `anti_regression_guards.rs`, covered by the meta-lock registry. The guard pins the marker payload as non-progress, requires embodied continuation to commit the follow-on through `run_pipeline`, requires receipt ancestry to include the marker as marker ancestry, and keeps scheduler progress accounting special-cased on `ContinueRoutineProposed` plus explicit `behavioral_progress=true`.

Verification run:

- `cargo test --locked -p tracewake-core continue_routine`
- `cargo test --locked -p tracewake-core --test anti_regression_guards guard_0057_continue_routine_progress_of_record_is_follow_on`
- `cargo test --locked -p tracewake-core --test anti_regression_guards meta_lock_registry_covers_structural_locks_and_negatives`
- `cargo fmt --all --check`
- `cargo test --locked -p tracewake-core`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
