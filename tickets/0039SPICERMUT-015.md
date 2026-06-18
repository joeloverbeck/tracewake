# 0039SPICERMUT-015: Kill `scheduler.rs` SPINE survivors with tick-boundary + routine-completion witnesses

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `scheduler.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 8 missed mutants in `crates/tracewake-core/src/scheduler.rs` (spec §5.15), owning SPINE-06 (scheduler/event pipeline) and SPINE-08 (no direct routine dispatch). The cluster mutates routine-completion append helpers, `SleepCompleted`/`WorkBlockCompleted` arms, tick/equality comparisons, and instant-progress boolean conditions. The witness must observe event append/application and routine lifecycle, not a private predicate alone.

## Assumption Reassessment (2026-06-18)

1. `append_routine_step_completed_after_duration_completion` exists at `crates/tracewake-core/src/scheduler.rs:1108` (mapping `EventKind::SleepCompleted -> (RoutineFamily::SleepNight, "sleep")` at `:1116` and `EventKind::WorkBlockCompleted -> (RoutineFamily::WorkBlock, "work_block")` at `:1117`); `is_instant_routine_progress_event` at `:1975`; `build_routine_step_completed_event` at `:2024` (verified by grep). The 8 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.15 is the implementation contract; `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` and `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` govern scheduler/no-direct-dispatch boundaries (verified present).
3. Shared boundary under audit: the scheduler seam that appends due completion events with deterministic ordering and cause ancestry, without becoming cognition authority.
4. Motivating invariants: `INV-103 — The scheduler is not a cognition authority` and `INV-104 — Routines and needs do not dispatch primitive actions directly`. The witness observes event append/application and routine lifecycle, not a private predicate.
5. This ticket touches the scheduler-not-cognition / no-direct-dispatch surface: due events at ticks immediately before/at/after the boundary must behave as specified; sleep/work completions must append the canonical event exactly once with deterministic ordering key and cause ancestry; an unrelated instant event must not progress a routine while the exact qualifying event does; and scheduler activity must not manufacture primitive proposals from raw truth, rewrite a transaction's wait reason, or become cognition authority. The witnesses only strengthen scheduler discipline — no leakage or nondeterminism is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-06/08 re-proof in ticket 021.

## Architecture Check

1. Before/at/after tick-boundary cases, append-exactly-once assertions with ordering key + cause ancestry, and an unrelated-vs-qualifying instant-event pair catch the comparison and completion-arm mutants through observed event append/application and routine lifecycle.
2. No backwards-compatibility aliasing/shims: witnesses observe appended/applied events and routine state, not a private predicate; an accepted scheduled completion and a blocked actor action keep event scheduling and actor validation distinct.

## Verification Layers

1. INV-104 routine completion -> replay/golden-fixture check: sleep and work-block completions append the canonical event exactly once with deterministic ordering key and cause ancestry; due events at before/at/after the tick boundary behave as specified.
2. INV-103 no scheduler cognition -> no-human-run check: an unrelated instant event does not progress a routine while the exact qualifying event does; scheduler activity does not manufacture proposals from raw truth or rewrite a wait reason.
3. Distinct event-vs-actor paths -> replay/golden-fixture check: replay the no-human sequence and compare event count/order, routine state, needs, and diagnostics; an accepted scheduled completion and a blocked actor action remain distinct.

## What to Change

### 1. Tick-boundary + completion witnesses

In `no_human_capstone.rs` (with seam coverage in `spine_conformance.rs`), exercise due events at ticks immediately before, exactly at, and after the boundary; prove sleep and work-block completions append the canonical event exactly once with deterministic ordering key and cause ancestry.

### 2. No-cognition + distinct-path rows

Prove an unrelated instant event does not progress a routine while the exact qualifying event does; prove scheduler activity does not manufacture primitive proposals from raw truth, rewrite a transaction's wait reason, or become cognition authority; replay the no-human sequence and compare event count/order, routine state, needs, and diagnostics; include an accepted scheduled completion and a blocked actor action.

### 3. Member matrix

Map all 8 historical mutants (plus any new run survivor in this file) to a concrete append/application or routine-lifecycle consequence.

## Files to Touch

- `crates/tracewake-core/tests/no_human_capstone.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Proposal provenance (013) and checked-fact reporting (014).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` — passes with tick-boundary, append-exactly-once, and no-cognition witnesses.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the distinct event-vs-actor path seam assertions.
3. `cargo mutants --workspace -f crates/tracewake-core/src/scheduler.rs --no-shuffle` — all 8 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Completions append the canonical event exactly once with deterministic ordering and cause ancestry; tick-boundary behavior is observed before/at/after.
2. The scheduler never manufactures proposals from raw truth, rewrites a wait reason, or becomes cognition authority; event scheduling and actor validation stay distinct.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/no_human_capstone.rs` — tick-boundary completion + unrelated-vs-qualifying instant-event + replay-sequence comparison.
2. `crates/tracewake-core/tests/spine_conformance.rs` — accepted-scheduled-completion vs blocked-actor-action distinct-path seam assertions.

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/scheduler.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
