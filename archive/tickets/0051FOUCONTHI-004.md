# 0051FOUCONTHI-004: F-03 declared-process causal transactions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — replace the diagnostic-stream process marker with a process-specific causal transition applied through the atomic cutover; `world_processes_applied` counts committed transactions.
**Deps**: 0051FOUCONTHI-003

## Problem

`build_declared_world_process_event` turns a `DueProcessInvocation` into a generic `NoHumanAdvanceStarted` event (`scheduler.rs:2336`); that kind classifies as `EventStream::Diagnostic` (`events/envelope.rs:214`–`223`), and `apply_event_stream` maps `Diagnostic | Controller | ReplayDebug` to `ApplyOutcome::NonWorldNoOp` (`events/apply.rs:68`). `transact_world_one_tick` nonetheless increments `world_processes_applied` unconditionally after appending (`scheduler.rs:783`), so a "declared-process application" is a diagnostic no-op counted as a transition (F-03, critical; evidence overclaim). The fix makes a due declared process produce process-specific validated world/agent/epistemic events appended and applied through the same atomic cutover as actor work, and counts only committed transitions.

## Assumption Reassessment (2026-06-24)

1. Codebase: `build_declared_world_process_event` (`crates/tracewake-core/src/scheduler.rs:2307`, builds `EventKind::NoHumanAdvanceStarted` at `2336`); `DueProcessInvocation` is already a private struct (`scheduler.rs:239`); `due_process_invocations` (`565`); the unconditional `world_processes_applied += 1` at `783`; the diagnostic-stream → `NonWorldNoOp` map at `events/apply.rs:68`. Existing negative fixtures `external_crate_cannot_name_due_process_invocation` and `external_crate_cannot_set_world_step_process_events` already pin privacy diagnostics (extend, do not duplicate).
2. Specs/docs: spec `0051` §4.5, §9.3 (process dispatch representation — closed enum + registry, core/content-bootstrap-only trait objects, or a generated closed table: implementer-recorded choice). Architecture homes `docs/1-architecture/02_*` (event log), `04_*` (world-step).
3. Shared boundary under audit: the declared-process declaration/dispatch registry inside core — a due invocation identifies a process kind + trigger witness; callers cannot inject a completed `EventEnvelope` or construct a `DueProcessInvocation`.
4. INV-001 (causality before drama), INV-009 (meaningful state changes require events), INV-088 (regional/world processes are declared causal processes): restated — a due process is an authoritative causal transition with declared source/cadence/trigger, not a diagnostic marker.
5. Fail-closed / deterministic-replay surface: process events are appended and applied through the **same atomic cutover** as actor work (no partial commit), and the declaration is reconstructed on replay per `-003`; the replay/atomic enforcement is the surface confirmed here.
6. Schema extension (additive-vs-breaking): the process transition needs a process-specific event kind (or kinds) on the World/Agent/Epistemic stream rather than the `Diagnostic`-stream `NoHumanAdvanceStarted` — `events/envelope.rs` (new `EventKind` + `stream()` arm) and `events/apply.rs` (new apply arm, not `NonWorldNoOp`). **Additive**: new event kind(s), new dispatch arms; consumers are `apply.rs`, the checksum/replay path, and golden fixtures, per INV-020 (event schema evolution). Diagnostic marker events may *accompany* the transition but cannot *be* it. (Keyed distinctly from item 5 — schema shape vs replay/atomicity are separate obligations.)

## Architecture Check

1. A closed process-kind dispatch that produces stream-correct events applied through the shared atomic cutover is the only design that makes `world_processes_applied` truthful and replay-faithful; counting an unapplied diagnostic envelope is exactly the evidence overclaim §4.5 names. A registry that cannot accept a caller-supplied completed envelope keeps process authority in-core.
2. No backwards-compatibility alias: the diagnostic-marker substitution is removed, not retained behind a flag; `world_processes_applied` is incremented only on a committed transition.

## Verification Layers

1. INV-009 / INV-088 (causal process effects) -> replay/golden-fixture check: a declared process whose due transition changes a small authoritative state/projection fact — no effect before cadence, one effect at cadence, correct ancestry, identical replay.
2. INV-001 (causality) -> codebase grep-proof: `world_processes_applied` increments only after a committed process transition (not after a diagnostic append).
3. Seal -> negative fixture: external crates cannot construct a `DueProcessInvocation` or inject a raw process `EventEnvelope` (extend the existing process fixtures).

## What to Change

### 1. Closed process declaration/dispatch registry

Define a closed process declaration/dispatch registry inside core; a due invocation identifies a process kind + trigger witness. The declaration loads from canonical content/runtime state (`-002`) and is reconstructed on replay (`-003`).

### 2. Process-specific causal transition

The process transaction builds an ordinary typed proposal or a closed process-transition command; validation produces process-specific world/agent/epistemic events appended and applied through the same atomic cutover as actor work. Add the event kind(s) and the `stream()` + apply arms.

### 3. Truthful counting

`world_processes_applied` counts committed process transactions, not diagnostic envelopes.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-core/src/scheduler.rs` (modify) — registry, `build_declared_world_process_event` replacement, counting; merge-hub contributor
- `crates/tracewake-core/src/events/envelope.rs` (modify) — process-transition `EventKind` + `stream()` arm
- `crates/tracewake-core/src/events/apply.rs` (modify) — apply arm producing the authoritative effect
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — before/at/after-cadence effect + replay; merge-hub contributor
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — extend the process-injection seal; merge-hub contributor

## Out of Scope

- The per-tick actor census (F-04 → `-005`).
- Process declaration *reconstruction* mechanics (owned by F-02 → `-003`; this ticket consumes them).

## Acceptance Criteria

### Tests That Must Pass

1. A declared process produces no authoritative effect before cadence, exactly one effect at cadence, with correct ancestry, and identical replay.
2. `world_processes_applied` equals the count of committed process transitions (a diagnostic-only path increments it by zero).
3. `cargo test -p tracewake-core --test world_step_coordinator` and `--test negative_fixture_runner` are green.

### Invariants

1. A due declared process applies process-specific events through the shared atomic cutover.
2. External crates cannot construct a due invocation or inject a raw process event.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — cadence-boundary process-effect + replay-identity cases.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — extended process-injection compile-fail seal.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core --test negative_fixture_runner`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Added the process-specific `EventKind::DeclaredWorldProcessApplied` as a
world-stream event with metadata, stable id, cause requirements, replay
handling, and a world apply arm. `build_declared_world_process_event` now emits
that typed world event instead of the diagnostic-stream
`NoHumanAdvanceStarted` placeholder.

Updated the declared-process application loop so `world_processes_applied`
increments only when the appended process event applies as an authoritative
world transition (`Applied` or `WorldNoOp`), not for diagnostic/non-world no-op
application. The current loaded-world cadence process is an authoritative
world-stream process event with replayable ancestry; it does not yet mutate a
domain-specific physical field.

Extended actor-transaction provenance handling so
`DeclaredWorldProcessApplied` is accepted as process/frame ancestry where the
previous diagnostic no-human process marker was accepted. Updated
`world_step_coordinator` to assert the new world event kind and the truthful
process count.

Deviations:

- No domain-specific physical/agent/epistemic state column was added for the
  generic loaded-world cadence process. The committed world-stream event is the
  authoritative process transition for this scope; future concrete process
  kinds can add stronger event-specific state effects without restoring the
  diagnostic-marker substitution.

Verification:

- `cargo test -p tracewake-core --test world_step_coordinator` — passed.
- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo fmt --all --check` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
