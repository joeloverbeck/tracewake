# 0005PHA3ANEEROU-010: Workplace model and work action (duration-based)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds a workplace/workstation entity to core state and a duration-based `work_block`/`begin_work` action producing a non-economic completion marker.
**Deps**: 0005PHA3ANEEROU-005, 0005PHA3ANEEROU-007

## Problem

Phase 3A work must be causal but not a full economy: a duration-based work block at a valid workplace that costs time, raises fatigue/hunger, and produces only a non-economic completion marker — never money from nowhere (Spec 0005 §9.8, §16.3, §21; `INV-046`, `INV-045`). Work is a required ordinary action through the shared pipeline (Spec §3.4, §9.1), and its failure-due-to-need/access path is half of the canonical physical-blocker proof (Spec §12).

## Assumption Reassessment (2026-06-07)

1. Action defs/registry/state surfaces are as established by tickets 008/009 (`actions/defs/`, `registry.rs`, `state.rs`). This ticket adds a workplace entity and a `work_block` action def reusing the duration-completion pattern established for sleep (ticket 008) in `scheduler.rs`.
2. The need model (001), work-lifecycle event kinds (005), and need-delta machinery (007) provide fatigue/hunger impact and start/complete/fail events. Spec §9.8 fixes validation (actor enabled; workplace/workstation assignment or routine method; at workplace or routable there; access permits work; not blocked by hunger/fatigue/severe-safety per thresholds; not already body-exclusive) and effects (work intention/routine ancestry, start event, scheduled completion or failure, time cost, fatigue/hunger impact, absence-from-home via ordinary movement ancestry, simple output marker).
3. Shared boundary under audit: the workplace schema fields fixed here are mirrored by the content work schema (ticket 015, §16.3) and validated (ticket 016); the action is consumed by `GoToWork`/`WorkBlock` routines (ticket 013) and the workday fixtures (tickets 019/021). The output-tag contract (`work_block_completed`/`service_completed_placeholder`/`office_available_placeholder`) is fixed here as non-economic.
4. Invariants motivating this ticket: `INV-046` — "V1 economy is simplified but not fake" (the preferred Phase 3A direction is no wages/payment, only a non-economic completion marker; any payment must be eventful and custody-aware) — and `INV-045` (work raises needs causally; no fake effects). Spec §9.8/§21 forbid creating money from nowhere.
5. Deterministic-replay / placeholder surface: work start/completion/failure events feed replay (ticket 006); the output marker is an inert, validated, non-authoritative placeholder (Spec §5.2 placeholder discipline) — it must not become wages, debt, or institutional reporting. This ticket adds no nondeterminism (completion on a deterministic tick) and no economy authority; the placeholder is debug-visible and carries no money custody.

## Architecture Check

1. A duration-based work action producing only a typed completion marker (rather than a wage/payment effect) keeps Phase 3A out of economy authority while still making work causal and absence-producing (`INV-046`, Spec §5.2). Reusing the sleep duration-completion pattern (ticket 008) avoids a second timer mechanism and keeps both on the scheduler's deterministic ordering.
2. No backwards-compatibility shims: workplace extends entity state additively; the output marker is an inert placeholder, not a stubbed economy that a later phase must unwind.

## Verification Layers

1. Simplified-not-fake economy (`INV-046`) -> unit test + grep-proof: completing a work block emits only a non-economic completion-marker event; no money/custody/value mutation occurs, and no payment field exists.
2. Causal needs (`INV-045`) -> unit test: a work block raises fatigue/hunger by the deterministic amount over its duration and costs time; it fails (not silently succeeds) when the actor is too fatigued/blocked.
3. Shared pipeline + blocker (`Spec §3.4`/§9.8) -> integration test: a non-human-origin `work_block` validates through the shared pipeline; a locked/inaccessible workplace yields a `physical`/`access` blocker, no teleport, no completion.

## What to Change

### 1. Workplace entity

Extend `crates/tracewake-core/src/state.rs` with a workplace/workstation entity: stable ID, place/station, assigned actors or routine-template references, work window, work duration, access preconditions, need thresholds affecting availability, output tag (non-economic).

### 2. Work action def

Add `crates/tracewake-core/src/actions/defs/work.rs` implementing §9.8 validation and a duration effect: emit a work-start event, schedule completion (reusing ticket 008's scheduler hook), apply fatigue/hunger impact over duration (ticket 007 deltas), and on completion emit a completion event carrying the non-economic output marker; failure (need/access/resource) emits a typed blocker event. Action IDs use `work_block` (and/or `begin_work`) per Spec §9.1.

### 3. Registry + module registration

Register the work action via the Phase 3A registration fn in `crates/tracewake-core/src/actions/registry.rs` and declare it in `crates/tracewake-core/src/actions/defs/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/work.rs` (new)
- `crates/tracewake-core/src/state.rs` (modify — add workplace/workstation entity)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — declare `work`)
- `crates/tracewake-core/src/actions/registry.rs` (modify — register Phase 3A `work_block`/`begin_work`)

## Out of Scope

- Wages/payment/debt/money (explicitly excluded; non-economic marker only — Spec §9.8, §20).
- `GoToWork`/`WorkBlock` routine selection and routing (tickets 013, 014).
- Content work schema and validation (tickets 015, 016).
- Workday golden fixtures (tickets 019, 021).

## Acceptance Criteria

### Tests That Must Pass

1. Starting work emits a start event and schedules completion at a deterministic tick; completing emits only the non-economic output-marker event.
2. A work block raises fatigue/hunger over duration and costs time; a too-fatigued or access-blocked actor fails with a typed `physical`/`access`/need blocker, no teleport, no completion.
3. A grep-proof confirms the work path mutates no money/value/custody state and has no payment field.

### Invariants

1. Work creates no money from nowhere; output is an inert non-economic marker (`INV-046`).
2. Work is duration-based and body-exclusive, raising needs causally through the shared pipeline (`INV-045`, Spec §3.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/work.rs` (unit tests) — validation, duration completion, need impact, blocker failure, non-economic output.
2. `crates/tracewake-core/src/state.rs` (unit tests) — workplace entity fields.

### Commands

1. `cargo test -p tracewake-core actions::defs::work`
2. `cargo test -p tracewake-core actions`
3. Core-crate scope is correct; end-to-end work within `ordinary_workday_001`/`no_human_day_001` is exercised by tickets 019/021/025.
