# 0005PHA3ANEEROU-011: continue_routine action and body-exclusive reservation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `continue_routine` action and a body-exclusive reservation/conflict check in the pipeline's reservation slot.
**Deps**: 0005PHA3ANEEROU-003, 0005PHA3ANEEROU-008, 0005PHA3ANEEROU-009, 0005PHA3ANEEROU-010

## Problem

`continue_routine` must submit the next ordinary proposal for an actor's active intention or produce a typed why-not — it must not be a debug cheat, jump steps, mutate the intention directly, or ignore action validation (Spec 0005 §9.9, §3.4). Phase 3A must also prevent one actor body from performing overlapping body-exclusive actions (cannot simultaneously sleep, work, move, eat), typed as `scheduling/reservation` conflicts (Spec §9.5). This action plus reservation closes the ordinary-action set the cognition engine and runner drive.

## Assumption Reassessment (2026-06-07)

1. The pipeline (`crates/tracewake-core/src/actions/pipeline.rs`) already exposes a `ReservationConflictCheck` stage (confirmed `pipeline.rs:36,61`) currently inert; this ticket makes it real for body-exclusivity. The routine execution model (ticket 003) provides the active intention's next step and reserved-resource slot; the duration actions sleep/eat/work (008/009/010) are the body-exclusive actions whose overlap is rejected.
2. Spec §9.9 fixes `continue_routine` behavior: no current intention → reject with `NoCurrentIntention` why-not; completed/failed intention → reject or surface summary; next step produces an ordinary action → submit that proposal; next step needs routing/planning → invoke bounded planner (ticket 014) and trace it; blocked next step → routine failure/interruption/replan or stuck diagnostic; possessed actor → use the same state/execution, no reset/copy. Spec §9.5 fixes reservation: body-exclusive non-overlap minimum; bed/workstation conflicts typed `scheduling/reservation`; unimplemented exclusivity stated honestly in the trace, not faked.
3. Shared boundary under audit: `continue_routine` is the action both human possession (`continue` command, ticket 022) and the no-human runner (ticket 017) submit to advance a routine; it must behave identically for both origins (Spec §3.4, §9.9). The action rejection reasons extend `crates/tracewake-core/src/actions/report.rs`.
4. Invariant motivating this ticket: `INV-035` — "Routines are defeasible intentions" (continue must surface failure/interruption, not silently proceed) — and `INV-040` — "Agents are bounded but competent" (continue invokes a bounded planner, not omniscient routing). Continue must never mutate the intention directly; it submits proposals through the shared pipeline.
5. Deterministic-replay / reservation surface: reservation conflicts are typed `scheduling/reservation` blockers feeding diagnostics (ticket 004) and replay (ticket 006). This ticket adds no nondeterminism — the reservation check is a deterministic predicate over current body-exclusive holds, and continue's next-step selection is the routine's authored order. No leakage: continue reads only actor-known routine state.

## Architecture Check

1. Activating the existing inert `ReservationConflictCheck` slot (rather than a new ad-hoc guard) keeps body-exclusivity inside the one validation pipeline every origin shares, so the conflict is typed and replayable for human and autonomous actors alike. Implementing `continue_routine` as a proposal-submitting action (never a direct intention mutator) structurally prevents the "debug cheat / step jump" failure (Spec §9.9, §21).
2. No backwards-compatibility shims: the reservation slot is filled, not replaced; continue is a new registry action; honest "exclusivity not enforced for resource X" is stated in the trace rather than faked.

## Verification Layers

1. Defeasible continue (`INV-035`) -> unit test: `continue_routine` with no intention rejects with `NoCurrentIntention`; with a blocked next step it emits a routine failure/interruption or stuck diagnostic, never a silent proceed.
2. Possession parity (Spec §3.4/§9.9) -> integration test: `continue_routine` from a Human origin and a Scheduler/Agent origin against the same actor/intention/state produce the same next proposal and validation result.
3. Reservation (`Spec §9.5`) -> unit test: overlapping body-exclusive proposals (sleep while moving) are rejected with a `scheduling/reservation` blocker; an unenforced resource is reported as unenforced in the trace, not faked.

## What to Change

### 1. continue_routine action

Add `crates/tracewake-core/src/actions/defs/continue_routine.rs` implementing the §9.9 branches: read the active intention's routine execution (ticket 003), pick the next step, and either submit the mapped ordinary proposal, invoke the bounded planner (ticket 014, via a handle), or emit failure/interruption/stuck. It never mutates the intention directly. Register via the Phase 3A registry fn and declare in `defs/mod.rs`.

### 2. Body-exclusive reservation

Implement the `ReservationConflictCheck` stage in `crates/tracewake-core/src/actions/pipeline.rs`: reject a proposal that would overlap an in-progress body-exclusive action (sleep/work/move/eat) for the same actor, typed as a `scheduling/reservation` blocker (ticket 004 category). Bed/workstation reservation uses the routine execution's reserved-resource slot; unenforced exclusivity is reported honestly.

### 3. Rejection reasons

Extend `crates/tracewake-core/src/actions/report.rs` with the `NoCurrentIntention`, completed/failed-intention, and reservation-conflict why-not reasons.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (new)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — declare `continue_routine`)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register `continue_routine`)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — implement `ReservationConflictCheck`)
- `crates/tracewake-core/src/actions/report.rs` (modify — continue/reservation why-not reasons)

## Out of Scope

- The bounded planner that resolves routing/planning next steps (ticket 014) — this ticket calls it via handle.
- Candidate-goal generation and intention adoption (ticket 012).
- TUI `continue` command rendering (ticket 022).
- The possession-does-not-reset golden fixture (ticket 020).

## Acceptance Criteria

### Tests That Must Pass

1. `continue_routine` with no active intention rejects with `NoCurrentIntention`; with a completed intention surfaces a summary/rejection; with a producible next step submits that ordinary proposal.
2. Overlapping body-exclusive proposals for one actor are rejected with a `scheduling/reservation` blocker; non-overlapping ones pass.
3. `continue_routine` produces identical next proposals/validation for Human vs Scheduler/Agent origins on equivalent state.

### Invariants

1. `continue_routine` never mutates the intention directly and always routes proposals through the shared pipeline (`INV-035`, Spec §9.9).
2. One actor body cannot hold two overlapping body-exclusive actions; conflicts are typed, not silent (Spec §9.5).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/continue_routine.rs` (unit tests) — all §9.9 branches.
2. `crates/tracewake-core/src/actions/pipeline.rs` (unit tests) — reservation conflict typing and honest-unenforced reporting.

### Commands

1. `cargo test -p tracewake-core actions::defs::continue_routine`
2. `cargo test -p tracewake-core actions::pipeline`
3. Core-crate scope is correct; possession-parity end-to-end is exercised by ticket 020 and the capstone (025).
