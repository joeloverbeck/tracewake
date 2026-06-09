# 0004PHA1THIHAR-007: Add unsupported-schema and non-world-leakage replay negative fixtures

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` events/replay negative fixtures (new test file)
**Deps**: None

## Problem

The event-log / schema-version / replay seam is substantially aligned (spec §6 F-003), but lacks adversarial negative fixtures proving its reject paths fail closed: forged future event schema versions, unsupported payload schema versions, stream mismatches, and non-world stream events attempting to change the physical checksum (spec §8 THIRD-AC-008). Without these, silent migration drift, replay accepting unknown data, or control/debug events mutating world state could regress undetected.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, deterministic-replay ticket. -->

1. `crates/tracewake-core/src/events/apply.rs` already enforces the reject paths: `apply_event_stream` (`:41`) dispatches by `event.stream` (`:45`), rejects unsupported schema versions (`!event.has_supported_schema_version()` at `:120`), and rejects stream mismatches (`event.stream != event.event_type.stream()` at `:126`). `crates/tracewake-core/src/replay/rebuild.rs` rebuilds projections and produces `final_checksum` via `compute_physical_checksum` (`:1`–`:21`). `events/envelope.rs` carries the schema registry and stream classification.
2. The remediation is spec §8 `THIRD-AC-008` + the §9.4 schema/replay fixtures (`unsupported_event_schema_append_rejected`, `unsupported_event_schema_replay_rejected`, `stream_mismatch_replay_rejected`, `non_world_event_cannot_change_physical_checksum`, `replay_rebuild_checksum_matches_original_after_no_human_day`), reassessed this session.
3. Shared boundary under audit: the checksum surface is conceptually adjacent to ticket 006 (field-to-registry parity), but this ticket adds replay/schema *rejection* fixtures in a new test file — no file conflict with 006's `checksum.rs`/`anti_regression_guards.rs` edits.
4. Motivating invariants (restated): `INV-009` (changes require events), `INV-011` (no current-state-only shortcut), `INV-018` (deterministic replay), `INV-020` (event schema evolution is mandatory), `INV-092` (replay is tested).
5. Fail-closed / replay surface: the enforcement surfaces are the append/apply schema validation and stream classification in `events/apply.rs` and `events/log.rs`. The fixtures prove these reject (fail-closed) and that a non-world stream event cannot mutate the physical checksum (no control/debug-event leakage into world state). The ticket adds proofs only; it weakens no reject path.

## Architecture Check

1. Adversarial negative fixtures that *attempt* a forged version, a stream mismatch, and a non-world checksum mutation, then assert rejection, prove the seam fails closed — strictly stronger than trusting the reject code by reading it. They exercise the append, apply, and replay-rebuild paths end-to-end.
2. No backwards-compatibility shims: the fixtures add no production code path; they exercise existing reject logic.

## Verification Layers

1. `INV-020` (schema evolution) -> negative-fixture: `unsupported_event_schema_append_rejected`, `unsupported_event_schema_replay_rejected`.
2. `INV-018` (replay determinism) -> replay-fixture: `stream_mismatch_replay_rejected`, `replay_rebuild_checksum_matches_original_after_no_human_day`.
3. No-leak (non-world -> world) -> negative-fixture: `non_world_event_cannot_change_physical_checksum`.

## What to Change

### 1. Schema-version rejection fixtures

Add fixtures appending and replaying envelopes with an unsupported `event_schema_version`, asserting append-time and replay-time rejection.

### 2. Stream-integrity and leakage fixtures

Add a stream-mismatch replay rejection fixture, a fixture proving a non-world stream event leaves `compute_physical_checksum` unchanged, and a positive `replay_rebuild_checksum_matches_original_after_no_human_day` equality fixture.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (new) — the unsupported-version, stream-mismatch, non-world-leakage, and replay-equality negative/positive fixtures

## Out of Scope

- Checksum field-to-registry parity (ticket 006).
- Content schema/validation (ticket 008).
- Any change to the existing reject logic in `events/apply.rs` (the fixtures exercise it, they do not modify it).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — all five fixtures pass (the four rejections reject, the equality fixture matches).
2. `cargo test --workspace` — full pipeline green.
3. `cargo build --workspace --all-targets --locked` — the new test target compiles.

### Invariants

1. An unsupported event schema version is rejected at append and at replay.
2. A non-world stream event cannot change the physical checksum.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — schema/replay/leakage negative fixtures + replay-equality; rationale: convert the "F-003 seam is strong" assessment into demonstrated fail-closed behavior.

### Commands

1. `cargo test -p tracewake-core --test event_schema_replay_gates`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Added `crates/tracewake-core/tests/event_schema_replay_gates.rs` with the five required schema/replay fixtures:
  - `unsupported_event_schema_append_rejected`
  - `unsupported_event_schema_replay_rejected`
  - `stream_mismatch_replay_rejected`
  - `non_world_event_cannot_change_physical_checksum`
  - `replay_rebuild_checksum_matches_original_after_no_human_day`
- Hardened `apply_event_stream` so replay/application rejects an envelope whose declared stream does not match its event kind before dispatching by stream. This closes the forged non-world-stream bypass exposed while implementing `stream_mismatch_replay_rejected`.

Deviations from original plan:

- The replay unsupported-schema fixture proves fail-closed replay ingestion through canonical event-log deserialization, because `EventLog` intentionally rejects unsupported schema versions before constructing a replay log.

Verification:

- `cargo test -p tracewake-core --test event_schema_replay_gates` — passed, 5 tests.
- `cargo fmt --all --check` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
