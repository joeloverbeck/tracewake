# 0059AUTSCHROU-002: Transaction fail-closed / non-override for routine_window_family

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — hardens `ActorDecisionTransaction::run` and routine-window candidate generation in `crates/tracewake-core/src/agent/transaction.rs` and `crates/tracewake-core/src/agent/generation.rs`; reuses the existing `StuckDiagnosticRecorded` diagnostic surface (no new event kind)
**Deps**: 0059AUTSCHROU-001

## Problem

The decision transaction treats a caller-supplied `routine_window_family` as a candidate-generating input without binding it to the active intention. `ActorDecisionTransaction::run` (`crates/tracewake-core/src/agent/transaction.rs:90`) converts `input.routine_window_family.and_then(goal_for_routine_family)` into a `routine_window_goal` (`transaction.rs:132-134`), and `generate_candidate_goals` (`crates/tracewake-core/src/agent/generation.rs:138`) unconditionally pushes a `CandidateGoalSource::RoutineDuty` candidate at `GoalPriority::RoutineWindowDuty` from it — even when the actor has **no** active intention. Today an existing active intention masks this because `GoalPriority::ActiveIntentionContinuation` (`crates/tracewake-core/src/agent/candidate.rs:63`) outranks `RoutineWindowDuty` (`candidate.rs:65`); but with no active intention a clock/window-supplied family becomes a `RoutineDuty` goal ahead of cognition. 0059 §F-0059-02/03/05 require the caller-supplied family to be non-authoritative: consistent-or-ignored, never an override, never a `RoutineDuty` without an active intention, and never a laundering path around the shared follow-on resolver.

## Assumption Reassessment (2026-07-01)

1. The seam is live as described: `transaction.rs:24` `pub routine_window_family: Option<RoutineFamily>`; `transaction.rs:91` `active_intention_for_actor(...)`; `transaction.rs:132-134` builds `routine_window_goal` via `goal_for_routine_family` (`transaction.rs:295`); `generation.rs:138` `if let Some(goal_kind) = input.routine_window_goal { … CandidateGoalSource::RoutineDuty, GoalPriority::RoutineWindowDuty … }`. `GoalPriority` order (`candidate.rs:59`) confirms `ActiveIntentionContinuation` (rank 3) > `RoutineWindowDuty` (rank 5).
2. Spec `specs/0059_…_SPEC.md` §F-0059-02 (non-override / fail-closed), §F-0059-03 (actor-known context validates *after* authority binds, never selects work because at a workplace / eat because food is visible), and §F-0059-05 (no shortcut through the shared resolver `crates/tracewake-core/src/agent/routine_continuation.rs`, `resolve_routine_step_follow_on` at `routine_continuation.rs:20`). The **Q1 decision (2026-07-01)** resolved the §F-0059-02 either-or in favor of option (b): on a conflict between a caller-supplied family and the active-intention-derived family, **ignore the hint and record a typed non-authoritative diagnostic** (do not stuck the transaction).
3. Shared boundary under audit: the producer→consumer contract on `ActorDecisionTransactionInput::routine_window_family`. 0059AUTSCHROU-001 makes the autonomous producer pass an active-intention-bound family (or `None`); this ticket makes the *consumer* treat any supplied family as a non-authoritative hint regardless of producer, so the seam is safe even against a future or test caller that supplies a window-keyed family.
4. Motivating invariants restated: **INV-104 — Routines and needs do not dispatch primitive actions directly** ("None of them may bypass candidate generation, actor-known context, local planning, proposal construction, shared validation") and **INV-103 — The scheduler is not a cognition authority**. A `routine_window_family` that produces a `RoutineDuty` candidate without an active intention is a routine label dispatching cognition.
5. Enforcement surface: fail-closed routine-family derivation inside the actor decision transaction. The change adds typed, replayable outcomes (reusing `EventKind::StuckDiagnosticRecorded` / `build_actor_stuck_diagnostic_event` for the no-chain case and a non-authoritative-hint diagnostic for the conflict case) and weakens no epistemic-leakage or replay guarantee: the diagnostic reasons are deterministic strings over already-actor-known inputs, ordinary non-routine need/idle fallback still flows when `include_idle_fallback` is true, and the shared resolver `resolve_routine_step_follow_on` continues to receive only an already-authorized method/step (no scheduler-chosen family is laundered through it, §F-0059-05).
6. Mismatch + correction record: the §F-0059-02 either-or is resolved to option (b) per Q1; option (a) (reject-with-stuck on conflict) is recorded here as the rejected alternative so a future reader does not re-adopt it. Rejection-with-stuck remains correct for the no-chain / malformed cases (no active intention at all), which yield a typed non-routine outcome, not a `RoutineDuty`.

## Architecture Check

1. Making the caller-supplied family a non-authoritative hint at the *consumer* keeps the active intention as the single planning authority (INV-103/104) regardless of what any producer supplies, which is more robust than fixing only the scheduler producer (001): a future scheduler/debug/no-human caller cannot reintroduce the override. Ignoring a conflicting hint (Q1=b) rather than stucking preserves a valid active intention's continuation, matching the existing priority semantics where `ActiveIntentionContinuation` already outranks `RoutineWindowDuty`.
2. No backwards-compatibility aliasing/shim: the unconditional `routine_window_goal → RoutineDuty` candidate path is replaced with a gated one, not wrapped behind a fallback; the field is retained as a typed hint only because §F-0059-02 explicitly permits an evidence/consistency annotation, not as a second authority source.

## Verification Layers

1. INV-104 (no direct dispatch) -> codebase grep-proof + behavioral test: a `RoutineDuty` candidate is generated from `routine_window_goal` only when an active intention is present and consistent; with no active intention, generation emits no `RoutineDuty` candidate.
2. INV-103 (scheduler/clock not cognition authority) -> behavioral test: a `Some(family)` conflicting with the active intention does not change the selected goal; a typed non-authoritative diagnostic is recorded.
3. INV-105 (decision traces are authoritative typed diagnostics) -> replay/manual review: the no-chain and conflict outcomes are typed, eventful (`StuckDiagnosticRecorded` / diagnostic record), and replayable with source-event ancestry (feeds §F-0059-04, asserted in 003).

## What to Change

### 1. Gate the routine-window candidate on an active intention

In `crates/tracewake-core/src/agent/transaction.rs` (around `transaction.rs:126-135`) and/or `crates/tracewake-core/src/agent/generation.rs` (`generation.rs:138`), only translate a supplied `routine_window_family` into a `RoutineDuty` / `routine_window_goal` candidate when the actor has an active intention whose derived family equals it. When there is no active intention, derive no routine family from the hint (the routine-window result is `None` / a typed non-routine outcome); ordinary non-routine need and idle-fallback candidates still flow when `include_idle_fallback` is true.

### 2. Conflict handling — ignore the hint, record a typed diagnostic (Q1 = b)

When `Some(family)` is supplied but conflicts with the active-intention-derived family, ignore the hint (the active intention wins) and record a typed, replayable non-authoritative-hint diagnostic (reason e.g. `routine_window_family_ignored_conflicts_with_active_intention`). Do **not** reject/stuck the transaction in this case. A `Some(family)` equal to the active-intention-derived family may be retained as an evidence annotation but adds no independent priority. The no-active-intention-at-all and malformed-chain cases continue to produce a typed non-routine / stuck outcome (reusing `build_actor_stuck_diagnostic_event` where a stuck is warranted), never a `RoutineDuty`.

### 3. Preserve the shared resolver boundary

Ensure `resolve_routine_step_follow_on` (`crates/tracewake-core/src/agent/routine_continuation.rs:20`) continues to receive only an already-authorized method/step context derived from the active intention; the hint must not be forwarded into the resolver as a family choice (§F-0059-05).

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/generation.rs` (modify)

## Out of Scope

- The scheduler-side producer rebind (0059AUTSCHROU-001).
- The A1–A10 metamorphic suite (003) and anti-regression guards (004); this ticket adds only the unit-level tests proving the gated/ignored/no-chain behaviors.
- Any change to `EventKind` or the event schema — the conflict/no-chain outcomes reuse the existing `StuckDiagnosticRecorded` surface and a diagnostic reason string (additive, no shape change).
- Severe-need-interrupt path, planner cadence, world-tick coordinator, time-advancing follow-on routing (0059 §2.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core agent::transaction::` — no active intention + supplied `routine_window_family` produces no `RoutineDuty` candidate; ordinary idle/need fallback still flows.
2. `cargo test -p tracewake-core agent::generation::` — a conflicting `Some(family)` does not outrank or replace the active-intention continuation; the active-intention-derived goal is selected and a non-authoritative diagnostic is recorded.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — the four local gates pass.

### Invariants

1. No `routine_window_family` value produces a routine-family candidate or follow-on ahead of, or in the absence of, the active intention (INV-103, INV-104).
2. Conflict and no-chain outcomes are typed and replayable (reuse `StuckDiagnosticRecorded` + diagnostic reason); the shared resolver receives no scheduler-chosen family (INV-105, §F-0059-05).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/transaction.rs` (in-module unit tests) — add cases: `Some(family)` with no active intention → no `RoutineDuty`, typed non-routine outcome; `Some(family)` conflicting with active intention → ignored + diagnostic, active intention selected; `Some(family)` equal to derived family → retained as annotation only. Rationale: lock the §F-0059-02 (Q1=b) contract at the unit level.
2. `crates/tracewake-core/src/agent/generation.rs` (in-module unit tests) — add a case asserting the `routine_window_goal → RoutineDuty` candidate is gated on active-intention presence/consistency.

### Commands

1. `cargo test -p tracewake-core agent::transaction:: agent::generation::` — narrowest boundary for the transaction/generation fail-closed contract (note: multiple positional filters are accepted directly by `cargo test`).
2. `cargo test --workspace` — full-pipeline confirmation that gating the candidate did not regress ordinary no-human need/idle routing.
