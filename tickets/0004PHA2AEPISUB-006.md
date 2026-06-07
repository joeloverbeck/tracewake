# 0004PHA2AEPISUB-006: `check_container` action, effect, and pipeline epistemic-slot activation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds `ActionEffect::CheckContainer`, the `check_container` action definition, and activates the pipeline's epistemic slot in `tracewake-core`.
**Deps**: 0004PHA2AEPISUB-005

## Problem

Absence becomes evidence only through an intentional, modeled channel (`INV-016`); search is intentional and eventful (`INV-047`). Spec 0004 §10 requires a `check_container` action (stable id `check_container`, semantic shape `check.container.<container_id>`) that passes through the shared proposal/validation/event pipeline, records a container-contents observation for the actor, and proves in the event log that the actor intentionally checked. Opening a container must not by itself create absence evidence. The pipeline already reserves a `KnowledgePerceptionPlaceholder` slot for exactly this; Phase 2A must activate it rather than build a parallel rule engine.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/actions/registry.rs` defines `ActionEffect { QueryOnly, Move, Open, Close, Take, Place, Wait }` (`registry.rs:6`) with no `CheckContainer`; `crates/tracewake-core/src/actions/pipeline.rs` defines the `KnowledgePerceptionPlaceholder` stage (`pipeline.rs:27`, `pipeline.rs:51`) confirmed inert by its own assertions (`pipeline.rs:572`). `crates/tracewake-core/src/actions/defs/` holds `movement.rs`, `openclose.rs`, `takeplace.rs`, `inspect.rs`, `wait.rs`, `mod.rs` — a new `checkcontainer.rs` joins them.
2. Action validation conditions and the §10.4 outcome list are fixed by Spec 0004 §10.3/§10.4; the action must reuse the existing proposal-origin/controller-binding parity already in the pipeline (§10.7). Epistemic events come from ticket 004; projection application from ticket 005.
3. Shared boundary under audit: the action registry (`registry.rs`), the pipeline stage activation (`pipeline.rs`), and the `defs/mod.rs` registration list are the surfaces this ticket extends; ticket 008 also extends `registry.rs`/`defs/mod.rs` (sequential via its dep on 007→006).
4. Invariant motivating this ticket: `INV-016` (absence becomes evidence only through expectation/perception/instruction/intentional search), `INV-047` (search is intentional, costly, bounded, fallible), and `INV-043` (action validation is ordinary-agent validation — no player-privilege branch).
5. Actor-knowledge / deterministic-replay surface: `check_container` produces an `ObservationRecorded` (+ `ContainerChecked`) event through the shared pipeline, so the observation is event-backed and replayable (ticket 005), and uses an actor `KnowledgeContext` (ticket 003) for any knowledge-dependent precondition. This ticket creates no missing-property belief itself (that is ticket 007's contradiction step) and reads no hidden truth — the observation records only what the modeled channel reveals. It introduces no nondeterminism: the same proposal over the same state emits the same event.

## Architecture Check

1. Activating the existing `KnowledgePerceptionPlaceholder` pipeline slot (rather than a side path) keeps the single-pipeline contract: `check_container` is validated and committed exactly like physical actions, so no player-only or epistemic-only mutation path exists. `QueryOnly` is insufficient because the check is eventful (it must append an observation), so a distinct `CheckContainer` effect is required.
2. No backwards-compatibility shims: the new effect and action def are additive; existing effects and the inert-slot assertions are updated in place to reflect the now-active slot.

## Verification Layers

1. Absence-through-channel (`INV-016`) -> integration test: an `open`-only flow creates no observation/absence; only `check_container` records a container-contents observation.
2. Single-pipeline / no player privilege (`INV-043`, `INV-007`) -> integration test: human-origin check requires controller binding; scheduler-origin check does not; both traverse the same validation and emit the same event shape.
3. Eventful intentional search (`INV-047`) -> replay/golden-fixture check: a `ContainerChecked`/`ObservationRecorded` event proves the intentional check occurred and survives replay (ticket 005).

## What to Change

### 1. CheckContainer effect and registry entry

Add `ActionEffect::CheckContainer` to `registry.rs` and register the `check_container` action with semantic-id shape `check.container.<container_id>` and TUI label `Check <container>`.

### 2. check_container action definition

Add `crates/tracewake-core/src/actions/defs/checkcontainer.rs` validating §10.3 conditions (actor exists/enabled, at required place, container exists/reachable, open-or-visible-enough for the declared method, locked/closed respected, knowledge context where applicable, proposal origin human/scheduler/agent, controller binding only for human origin, no hidden-truth use, optional one-tick cost). On success it emits the proof event(s) and records a contents observation (§10.4 steps 1–2); contradiction/belief (steps 3–5) are ticket 007.

### 3. Pipeline slot activation

Activate the `KnowledgePerceptionPlaceholder` stage in `pipeline.rs` for epistemic-capable effects so `check_container` produces its observation through the shared stage, and update the inert-slot assertions accordingly. Register the new def in `actions/defs/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (new)
- `crates/tracewake-core/src/actions/registry.rs` (modify — add `CheckContainer` effect + registry entry; shared with ticket 008, sequential)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — activate `KnowledgePerceptionPlaceholder` slot)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — register `checkcontainer`)

## Out of Scope

- Expectation contradiction and missing-property belief (ticket 007).
- Knowledge-blocker why-not and the accusation probe (ticket 008).
- TUI exposure of the action (ticket 011).
- Sound observation generation (ticket 014).

## Acceptance Criteria

### Tests That Must Pass

1. `check_container` on an open/reachable container records a contents observation and appends a `ContainerChecked`/`ObservationRecorded` event; `open` alone records no observation.
2. A check of a closed/locked/inaccessible container is rejected with a physical/access blocker and creates no hidden-contents observation.
3. A scheduler-origin check commits without controller binding through the same pipeline a human-origin check uses.

### Invariants

1. `check_container` traverses the shared proposal/validation/commit pipeline; no parallel epistemic mutation path exists.
2. The check records observation only; it reads no hidden truth and (in this ticket) creates no belief.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (unit tests) — validation conditions, observation emission, rejection cases.
2. `crates/tracewake-core/tests/golden_scenarios.rs` (extend) — check produces observation through the pipeline; open does not.

### Commands

1. `cargo test -p tracewake-core actions::defs::checkcontainer`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. `cargo build --workspace --all-targets --locked`
