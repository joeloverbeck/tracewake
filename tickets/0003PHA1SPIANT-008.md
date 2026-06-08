# 0003PHA1SPIANT-008: Mandatory pipeline append-before-apply + no-direct-apply source scan

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (pipeline conformance test; no-direct-`apply_event` source scan)
**Deps**: 0003PHA1SPIANT-001

## Problem

Accepted world actions flow proposal → validation → event construction → append → apply → projection (`actions/pipeline.rs` `run_pipeline` at `:158`; append to `EventLog` at `events/log.rs:4`; stream applier `apply_event_stream` at `events/apply.rs:40`). But nothing structurally forbids an accepted mutation without an appended event, an event appended after mutation, dry-run mutation leakage, or a stray `apply_event` call from a non-event/non-replay site. Spec `0003` §5.5 / SPINE-AC-008 require a conformance test that the event log changes before authoritative state for accepted runtime proposals, plus a source scan banning direct `apply_event` outside event/replay/tests.

## Assumption Reassessment (2026-06-08)

1. `run_pipeline` is at `crates/tracewake-core/src/actions/pipeline.rs:158`; `EventLog` at `crates/tracewake-core/src/events/log.rs:4`; `apply_event` at `crates/tracewake-core/src/events/apply.rs:106` and `apply_event_stream` at `:40`. The pipeline validates source context, constructs events, appends to the log, then applies.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-008 mandates: accepted world actions follow proposal → validation → event-envelope construction → append → apply → projection/report; validation/dry-run may inspect state or apply hypothetically only on clones / non-authoritative temporaries; no TUI/scheduler/loader/debug/test path exposes an accepted outcome without an appended event (except fixture initial-state construction); a conformance test asserts event-log length changes before authoritative-state checksum for accepted proposals; a negative source scan fails on direct `apply_event` outside event/replay/tests unless allowlisted.
3. Boundary under audit: the `run_pipeline` → `EventLog::append` → `apply_event_stream` ordering contract. **Depends on 0003PHA1SPIANT-001**: the mutation-capability seal makes "mutate without going through apply" a compile-time impossibility for the field path; this ticket adds the ordering/append-before-apply assertion and the stray-`apply_event` scan on top.
4. INV motivating this ticket: `INV-009` (accepted changes are events), `INV-011` (no hidden mutation), `INV-104` (single mutation path), `INV-017`/`INV-018` (the appended event is the replay record). Restated: every accepted world outcome is an appended event applied in order; nothing mutates authoritative state without first appending.
5. Deterministic-replay / mutation surface touched: append-before-apply is what guarantees the replay log is the complete authoritative record. The conformance test asserts log growth precedes checksum change; dry-run paths must operate on clones / non-authoritative temporaries so a rejected/hypothetical proposal leaves both the log and the checksum unchanged. No nondeterminism introduced.

## Architecture Check

1. A conformance test keyed on observable ordering (event-log length changes before authoritative checksum) plus a source scan for stray `apply_event` calls converts the pipeline's intended ordering from "true by construction today" into a regression gate, composing with 001's compile-time seal (the seal stops field writes; this stops out-of-order or event-less application).
2. No backwards-compatibility shim: no alternate accepted-mutation path is retained; the only direct `apply_event` callers are event/replay/tests, explicitly allowlisted.

## Verification Layers

1. `INV-009`/`INV-011` (events authoritative; no hidden mutation) -> runtime conformance: for an accepted proposal, event-log length changes before the authoritative-state checksum.
2. `INV-104` (single mutation path) -> codebase grep-proof: a source scan fails on direct `apply_event` calls outside `events`/`replay`/tests unless allowlisted.
3. `INV-018` (replay record complete) -> replay/golden-fixture check: a rejected/dry-run proposal leaves both the event log and the checksum unchanged.

## What to Change

### 1. Append-before-apply conformance test

Add `accepted_action_appends_before_authoritative_apply`: drive an accepted proposal through `run_pipeline` and assert the event-log length increments before the authoritative-state checksum changes; assert a rejected proposal changes neither.

### 2. No-direct-apply source scan

Add a source-scan test that fails on direct `apply_event`/`apply_event_stream` calls outside `events`, `replay`, and test modules, with an explicit allowlist.

### 3. Dry-run isolation check

Assert validation/dry-run code paths operate on clones / non-authoritative temporaries (no leakage into authoritative state or the log).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `accepted_action_appends_before_authoritative_apply` + no-direct-apply scan + dry-run isolation)

## Out of Scope

- The state-mutation seal (0003PHA1SPIANT-001).
- Scheduler no-direct-dispatch conformance (0003PHA1SPIANT-006).
- Source-context validation specifics (0003PHA1SPIANT-007).

## Acceptance Criteria

### Tests That Must Pass

1. `accepted_action_appends_before_authoritative_apply` — accepted proposal grows the log before changing the checksum; rejected proposal changes neither.
2. The no-direct-`apply_event` source scan fails on a stray call outside event/replay/tests.
3. `cargo test --workspace` passes.

### Invariants

1. Every accepted world outcome is an appended event applied in order (`INV-009`, `INV-104`).
2. Dry-run/validation never leaks into authoritative state or the log (`INV-011`, `INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `accepted_action_appends_before_authoritative_apply`, no-direct-apply scan, dry-run isolation.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`
