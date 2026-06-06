# 0002PHA1KERTUI-013: Projection rebuild and replay runner/report

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `replay` module (projection rebuild + replay runner + report) to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-006, 0002PHA1KERTUI-007, 0002PHA1KERTUI-012

## Problem

Replay is foundational, not a later optimization (Spec 0002 §6.4). This ticket adds projection rebuild from an accepted initial fixture plus ordered events (§11.6 — must not depend on the live mutable projection) and the replay runner/report (§11.7 — fixture id, content manifest, initial/final checksum, event count, unsupported versions, application errors, `matches_expected`, ordered `state_diff`). §11.8: Phase 1 fails if any golden scenario can only pass by comparing mutable current state, or if replay accepts checksum drift as a warning. This is the read/verify path that proves the event-sourced spine.

## Assumption Reassessment (2026-06-06)

1. No `replay` module exists; registers `pub mod replay;` in `crates/tracewake-core/src/lib.rs` (001), reusing the strict applier (006), the checksum (007), and the projection builders (012).
2. The rebuild outputs are `specs/0002_…_SPEC.md` §11.6; the replay-report fields are §11.7; the acceptance constraints are §11.8 (drift is a failure, not a warning; reject missing/reordered/unknown-version events).
3. Shared boundary under audit: the `ReplayReport` consumed by debug reports (016) and the capstone tests (022). It must reuse the ticket-006 applier (no second applier — §9.5) and the ticket-007 checksum.
4. Invariant motivating this ticket: INV-018 (deterministic replay is foundational) and INV-092 (deterministic replay is tested) — replay reconstructs significant outcomes from initial fixture + ordered events + versions, byte-identically.
5. Deterministic-replay surface: this *is* the replay surface. Rebuild starts from the accepted fixture (not live state), applies the ordered `world` stream via the ticket-006 applier, recomputes the ticket-007 checksum, and compares to expected — failing loudly on drift, missing/reordered events, or unknown versions. No nondeterministic input; `state_diff` is deterministically ordered. This ticket *is* the enforcement.

## Architecture Check

1. Rebuilding from fixture + log through the same strict applier the live path uses (rather than a replay-specific shortcut applier) guarantees replay and live runs cannot diverge by construction — the only way they differ is a real bug, which the checksum surfaces. A separate replay applier would be a second source of truth (forbidden, §9.5/§20.2).
2. Drift is a hard failure (`matches_expected = false`), never a warning (§11.8). No backwards-compatibility shims: greenfield.

## Verification Layers

1. Rebuild independence (§11.6) -> unit test: rebuild from fixture + events produces state equal to the live run, without reading the live projection.
2. Checksum match / drift detection (INV-018; §11.8) -> replay/golden check: replay of a clean log yields `matches_expected = true`; a log with a deleted or reordered item event yields `matches_expected = false` with a non-empty ordered `state_diff` (not a silent repair).
3. Unknown-version rejection (INV-020) -> unit test: a log containing an unsupported `event_schema_version` is reported under `unsupported_versions`, and replay does not "best effort" apply it.

## What to Change

### 1. Projection rebuild

Add `crates/tracewake-core/src/replay/rebuild.rs`: rebuild from accepted initial fixture + ordered `world` events via the ticket-006 applier, producing final state/projection, event count, last event id/position, content-manifest identity, final checksum, unsupported-version list, invariant violations, and a diff against expected state.

### 2. Replay runner + report

Add `crates/tracewake-core/src/replay/report.rs`: the §11.7 `ReplayReport` and the runner that computes initial/final checksum, counts world vs diagnostic events, sets `matches_expected`, and emits a deterministically-ordered `state_diff`.

### 3. Registration

Add `pub mod replay;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/replay/mod.rs` (new)
- `crates/tracewake-core/src/replay/rebuild.rs` (new)
- `crates/tracewake-core/src/replay/report.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod replay;`; file created by ticket 001)

## Out of Scope

- The debug-UI surfacing of the replay report (ticket 016 / 021 — they call this module's API).
- Fixture loading/serialization (ticket 017 supplies the accepted initial fixture).
- The seven scenario replay assertions (ticket 022).

## Acceptance Criteria

### Tests That Must Pass

1. Rebuild from fixture + ordered events equals the live run's final state and checksum.
2. Deleting or reordering one item-movement event makes replay report `matches_expected = false` with a non-empty ordered `state_diff`.
3. An unsupported `event_schema_version` appears in `unsupported_versions` and is not applied.

### Invariants

1. Rebuild reuses the single strict applier and never reads live mutable state.
2. Checksum drift is a hard failure, never a warning.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/replay/rebuild.rs` (unit tests) — rebuild-equals-live, independence from live projection.
2. `crates/tracewake-core/src/replay/report.rs` (unit tests) — drift detection, unknown-version reporting.

### Commands

1. `cargo test -p tracewake-core replay`
2. `cargo build --workspace`
3. Core-crate scope is correct: replay operates on in-crate fixtures/logs; the seven golden scenarios run in ticket 022.

## Outcome

Completed: 2026-06-06

What changed:
- Added `tracewake_core::replay` with projection rebuild and replay report APIs.
- Rebuild starts from an initial physical state and ordered world events, reusing the strict event applier.
- Replay reports initial/final checksums, world and non-world event counts, unsupported versions, application errors, match status, and ordered state diffs.
- Added drift and unsupported-version tests that fail replay rather than treating mismatches as warnings.

Deviations from original plan:
- Fixture loading remains out of scope as planned; tests construct accepted initial states directly until the content fixture tickets land.

Verification results:
- `cargo fmt` passed.
- `cargo test -p tracewake-core replay` passed: 6 matching tests.
- `cargo test -p tracewake-core` passed: 58 tests.
- `cargo build --workspace` passed.
