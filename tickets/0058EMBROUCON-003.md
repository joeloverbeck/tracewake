# 0058EMBROUCON-003: Stuck-diagnostic and receipt honesty

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — modifies `crates/tracewake-core/src/runtime/session.rs` (receipt/stuck assembly); test support in `scheduler.rs`; adds core behavioral tests
**Deps**: 0058EMBROUCON-002

## Problem

The embodied success and stuck paths need stronger behavioral locks (spec §4.3 F-0058-03, §3.2 H-0058-07). A successful continuation must not create a current stuck diagnostic for the actor merely because the prior scheduler-owned diagnostic scan ran; a genuine stuck must emit exactly one typed current diagnostic. Receipt assembly must let a reader tell whether a diagnostic predates the success, arose from the current rejection/stuck, or is absent. Separately (R3, §9.1), composing the marker + follow-on receipt must not change `advance_until` stop reasons or `ticks_advanced` arithmetic.

## Assumption Reassessment (2026-06-30)

1. `append_embodied_routine_stuck_diagnostics` is defined in `crates/tracewake-core/src/scheduler.rs` and called at `crates/tracewake-core/src/runtime/session.rs:597`; `run_embodied_continue_routine_stuck_outcome` at session.rs:607; `embodied_continue_routine_stuck_report` at session.rs:1169 builds a typed `ValidationReport` (`ReasonCode::RoutineStepBlocked`). The success path splices `marker_result.appended_events` + `embodied_stuck_events` into the receipt (654-658). `StuckDiagnosticRecorded` events are typed.
2. Spec §4.3 F-0058-03, §3.2 H-0058-07, and §9.1 R3 (stop-reason lock). `advance_until` returns `AdvanceUntilResult` / `AdvanceUntilStopReason` (`crates/tracewake-core/src/runtime/receipt.rs`; `advance_until` at session.rs:792); the embodied follow-on path is separate and must not mutate stop reasons.
3. Cross-artifact boundary under audit: the receipt-assembly seam (`session.rs`) between the prior scheduler-owned diagnostic scan (`append_embodied_routine_stuck_diagnostics`, `scheduler.rs`) and the current continuation stuck outcome. Reader-distinguishability of the two is the contract under audit.
4. Invariants under audit: INV-015 (failure is eventful when consequential), INV-098 (debug-inspectable, regression-tested acceptance), INV-105 (decision traces and stuck diagnostics are typed/structurally inspectable; display strings may summarize but are not the authoritative substrate).
5. Enforcement surface: the embodied receipt/stuck assembly in `session.rs` — a typed-diagnostic + replay surface. A success path must not append a current `StuckDiagnosticRecorded` unless a prior scheduler-owned diagnostic was already due and de-duplicated; the stuck path emits exactly one current typed diagnostic. Confirm no replay nondeterminism (diagnostic ids derive from typed actor/tick ids, e.g. session.rs:1176-1187) and no actor-knowledge leak (`actor_visible_summary` uses `diagnostic.actor_known_explanation`, 1198).
6. Schema/shape change: no schema shape change (the N/A pole) — clarifies naming and receipt-assembly control flow in `runtime/session.rs`; reuses existing `StuckDiagnosticRecord` / `ValidationReport` / event shapes unchanged and adds no public field. (Explicit N/A pole — `session.rs` trips the reseal heuristic but no reseal occurs.)

## Architecture Check

1. Splitting "prior scheduler-owned diagnostic scan" from "current continuation stuck outcome" in naming and receipt assembly makes the receipt honest and inspectable (INV-105): a reader or test can attribute every diagnostic. This is cleaner than a single undifferentiated diagnostic prefix that conflates pre-existing and current-rejection diagnostics.
2. No backwards-compatibility aliasing/shims: the R3 lock is a regression test asserting `advance_until` ownership is unchanged, not a compatibility wrapper; the diagnostic split renames/reorganizes assembly without a legacy alias.

## Verification Layers

1. INV-015/105 (eventful, typed diagnostics) → `embodied_continue_success_does_not_emit_current_stuck_diagnostic` + `embodied_continue_stuck_emits_one_current_typed_diagnostic`.
2. INV-098 (harsh acceptance) → post-work no-stuck accounting assertion (`stuck_actors` / stuck-diagnostic accounting stays empty for the actor unless a real unresolved stale execution exists).
3. INV-095/098 (stop-reason ownership / R3) → `embodied_continue_receipt_does_not_change_advance_until_stop_reason` (regression: `advance_until` owns stop reasons and `ticks_advanced`).

## What to Change

### 1. Separate prior-vs-current diagnostics

In naming and receipt assembly, distinguish the prior scheduler-owned diagnostic scan from the current continuation stuck outcome so a receipt reader can tell whether a diagnostic predates the successful follow-on, arose from the current rejection/stuck outcome, or is absent.

### 2. Success / stuck path honesty

Ensure the success path does not append or prefix a current `StuckDiagnosticRecorded` for the actor unless a prior scheduler-owned diagnostic was already due and de-duplicated. Ensure the stuck path emits exactly one current typed diagnostic (the `RoutineStepBlocked` rejected report), with no duplicate on re-scan.

### 3. Tests

Add `embodied_continue_success_does_not_emit_current_stuck_diagnostic`, `embodied_continue_stuck_emits_one_current_typed_diagnostic`, a post-work no-stuck-accounting assertion, and `embodied_continue_receipt_does_not_change_advance_until_stop_reason` (R3 lock).

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — only if the prior-vs-current split touches `append_embodied_routine_stuck_diagnostics` or its tests)

## Out of Scope

- Active-intention current-step authority (F-0058-01 → -001) and the temporal gateway (F-0058-02 → -002).
- Changing `advance_until` stop-reason semantics — the R3 test **locks** current behavior; it does not alter it.

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_continue_success_does_not_emit_current_stuck_diagnostic` — a successful move/work/eat follow-on appends no current `StuckDiagnosticRecorded` for the actor.
2. `embodied_continue_stuck_emits_one_current_typed_diagnostic` — a genuine stuck emits exactly one typed current diagnostic with an actor-known explanation; a duplicate scan emits no duplicate.
3. `embodied_continue_receipt_does_not_change_advance_until_stop_reason` — stop reasons and `ticks_advanced` remain owned by `advance_until`.
4. `cargo test --workspace` passes.

### Invariants

1. Success and stuck diagnostics are honest and reader-distinguishable; no success-path-fabricated current stuck (INV-015/105).
2. Marker + follow-on receipt composition does not alter `advance_until` stop reasons (INV-095/098).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (test module), plus `crates/tracewake-core/src/scheduler.rs` test support if the prior-vs-current split requires it — four behavioral tests.

### Commands

1. `cargo test -p tracewake-core embodied_continue_success`
2. `cargo test -p tracewake-core embodied_continue_receipt_does_not_change_advance_until_stop_reason`
3. `cargo test --workspace`
