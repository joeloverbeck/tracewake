# 0050FOUCONSEC-007: Fail-closed `EventId` uniqueness (append + deserialize)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — makes a duplicate `EventId` a fail-closed `EventLogError` on live append and deserialization
**Deps**: 0050FOUCONSEC-006

## Problem

Spec-0050 §4.4 (driver F-04, uniqueness half): perception `EventId` is deterministic over `(actor_id, decision_tick, perceived.kind, perceived.target_id)` — the `index` parameter is excluded (`crates/tracewake-core/src/agent/perception.rs:647`) — and `EventLog::append` (`crates/tracewake-core/src/events/log.rs:23`) checks only schema version and assigns positions; it does **not** reject an already-present `EventId`. Two events can therefore share a causal identity, making ordered replay/history ambiguous even though global positions stay monotonic.

With `-006` having stopped the TUI from re-appending the final-tick perception, this ticket makes a duplicate `EventId` a fail-closed `EventLogError` on both live append and deserialization, and — if repeated observations are semantically distinct — adds a deterministic causal sequence/parent component to the ID rather than permitting duplicates (§9.5 implementer-recorded choice).

## Assumption Reassessment (2026-06-24)

1. `EventLog::append` (`crates/tracewake-core/src/events/log.rs:23`) and `append_deserialized` (`log.rs:40`) do not reject duplicate `EventId`s; `EventLogError` (`log.rs:9`) is the existing fail-closed error type. Perception `EventId` is built at `crates/tracewake-core/src/agent/perception.rs:647` from `(actor, tick, kind, target)` excluding `index`. Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §4.4, §9.5, and §9.11 are authoritative: §9.5 assigns the repeated-observation-identity choice (deterministic occurrence/parent component vs dedup-as-same-fact) to the implementer; §9.11 records that pre-existing logs/golden fixtures/saves carrying duplicate perception IDs will now fail to deserialize, and that this is intentional (regenerate, no shim).
3. Shared boundary under audit: the `EventLog` append/deserialize boundary in `crates/tracewake-core/src/events/log.rs` and the perception `EventId` construction in `agent/perception.rs`. `-006` must land first so the TUI no longer produces the duplicate; enforcing uniqueness before that would break the live app.
4. `INV-009` (meaningful changes are events) and `INV-018` (deterministic replay) motivate this ticket: causal identities must uniquely name events for ordered replay/history to be unambiguous. A duplicate `EventId` is a determinism defect.
5. Enforcement surface: deterministic replay (`INV-018`/`INV-092`). The append/deserialize dup check is fail-closed and blocking (returns `EventLogError`, not a warning). It introduces no actor-knowledge surface. Replay over a corrected log is unambiguous; a log with a duplicate identity now fails closed rather than silently replaying ambiguously.
6. **Schema/contract shape change (event-identity contract; additive-vs-breaking)**: this changes the `EventId` *identity contract* (duplicate IDs become representable-but-rejected; optionally an occurrence/parent component is added to perception IDs). Consumers of `EventId`/`EventLog`: replay (`replay/*`), save/serialization, and every appender. The deserialize-side rejection is **breaking** for any already-serialized log containing duplicates (§9.11) — intentional, with golden/save regeneration, not a compatibility shim. The append-side rejection is additive to the error surface (new `EventLogError` variant).

## Architecture Check

1. Rejecting duplicate `EventId`s fail-closed at the single append/deserialize chokepoint is the minimal, central enforcement of the uniqueness invariant — stronger than scattered caller discipline, and it makes the F-04 duplicate (and any future one) impossible rather than merely absent. Adding a deterministic occurrence component (if repeated observations are distinct facts) preserves determinism without permitting duplicates.
2. No backwards-compatibility shims: there is no fallback that tolerates a duplicate ID; per §9.11 stale dup-bearing fixtures/saves are regenerated, not migrated behind a compatibility path.

## Verification Layers

1. `INV-009`/`INV-018` (unique causal identity; deterministic replay) → replay/golden-fixture check: an "exactly one event per `EventId` after multi-tick intervals" witness over the real `advance_until` path; a direct unit test that a duplicate append/deserialize returns `EventLogError`.
2. Single-surface note: this is an `EventLog`-identity ticket; its determinism invariant maps to the append/deserialize unit tests + the integration uniqueness witness — no actor-knowledge layer applies (perception identity is a scheduling/causal fact, not cognition input).

## What to Change

### 1. Reject duplicate `EventId` on append and deserialize

In `crates/tracewake-core/src/events/log.rs`, make `append` and `append_deserialized` return a new `EventLogError` variant when an `EventId` already exists in the log (fail-closed, blocking). Use an efficient membership structure consistent with the determinism rules (no `HashMap` iteration-order dependence in canonical forms).

### 2. Deterministic causal sequence for repeated observations (if distinct)

In `crates/tracewake-core/src/agent/perception.rs`, if repeated observations at different causal opportunities are semantically distinct, include a deterministic occurrence/parent component in the `EventId` (§9.5) so distinct occurrences get distinct IDs rather than colliding; otherwise dedup-as-same-fact. Record the chosen interpretation in the construction-site doc comment.

### 3. Witnesses

Unit tests in `log.rs` (duplicate append → `EventLogError`; duplicate deserialize → `EventLogError`) and an integration witness in `tests/world_step_coordinator.rs` asserting exactly one event per `EventId` after a multi-tick `advance_until`.

## Files to Touch

- `crates/tracewake-core/src/events/log.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)

## Out of Scope

- Core ownership of perception/interval output and TUI write removal — `0050FOUCONSEC-006` (prerequisite).
- Regenerating specific golden fixtures/saves — happens as those lanes run in `0050FOUCONSEC-010`/`-013`; §9.11 governs the policy.

## Acceptance Criteria

### Tests That Must Pass

1. Duplicate `EventId` on `append` returns the new `EventLogError` variant; duplicate on `append_deserialized` returns it too.
2. After a multi-tick `advance_until`, every `EventId` in the resulting log is unique (exactly-one witness).
3. `cargo test -p tracewake-core --test world_step_coordinator`, the `events::log` unit tests, and `cargo build --workspace --all-targets --locked` are green.

### Invariants

1. A duplicate `EventId` is unrepresentable in a committed/loaded log — rejected fail-closed on both append and deserialize (`INV-009`/`INV-018`).
2. Repeated observations either carry a deterministic distinct identity or are deduplicated as the same fact; no path yields a silent duplicate (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/log.rs` — `#[cfg(test)]` duplicate-append and duplicate-deserialize rejection tests.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` — exactly-one-event-per-`EventId` integration witness over multi-tick `advance_until`.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core events::log`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented fail-closed duplicate `EventId` rejection at `EventLog::append` and deserialize, with explicit `DuplicateEventId` unit coverage. Repeated current-place perceptions now preserve the first stable event ID and assign deterministic occurrence-suffixed IDs to later same actor/tick/kind/target observations, updating the event payload identity fields with the committed event ID. Same-tick repeated door/container observations collapse to the latest source when building embodied surfaces, preserving stale-token rejection and latest-state rendering.

Added a multi-tick `advance_until` witness asserting committed `EventId`s are unique. Verification passed:

1. `cargo test -p tracewake-core events::log`
2. `cargo test -p tracewake-core --test world_step_coordinator`
3. `cargo test -p tracewake-tui`
4. `cargo fmt --all --check`
5. `cargo build --workspace --all-targets --locked`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `git diff --check`
