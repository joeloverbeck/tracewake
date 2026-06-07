# 0007PHA3ASECHAR-006: Durable intentions as live no-human commitments

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` intention lifecycle wiring (`agent/intention.rs`, `scheduler.rs`, `events/apply.rs`); possession-parity fixture
**Deps**: 0007PHA3ASECHAR-004

## Problem

Intentions must be live commitments that drive later proposals, not inert structs available only to unit tests (Spec 0007 D-05, §Intentions). The intention machinery and its full lifecycle event vocabulary already exist, but the integrated no-human loop does not consistently adopt, continue, complete, fail, suspend, resume, abandon, or interrupt an active intention through events that stabilize and drive the next decision. §Intentions also requires that possession attach/detach/switch never clears an ordinary actor's active intention, and that the active intention stabilizes selection unless a severe need, blocker, contradiction, or scheduled routine boundary justifies switching.

## Assumption Reassessment (2026-06-07)

1. Confirmed: the intention state machine exists — `Intention` with `adopt`/`record_progress`/`suspend`/`complete`/`fail`/`abandon`/`interrupt`/`reactivate` (`crates/tracewake-core/src/agent/intention.rs:61-170`) and `ActorIntentions::adopt_active` (`intention.rs:171-194`). The lifecycle event kinds all exist: `IntentionStarted`, `IntentionContinued`, `IntentionSuspended`, `IntentionResumed`, `IntentionCompleted`, `IntentionFailed`, `IntentionAbandoned`, `IntentionInterrupted` (`crates/tracewake-core/src/events/envelope.rs:49-`). 0007PHA3ASECHAR-004 wires the real active intention into selection; this ticket owns the lifecycle transitions.
2. Spec 0007 D-05 requires the no-human loop adopt/continue/progress/interrupt/complete/fail/abandon/resume intentions "through events that drive later proposals … not merely available to tests". §Intentions requires possession never clears an active intention and the active intention stabilizes selection against small score jitter.
3. Shared boundary under audit: the `ActorIntentions` / active-intention record in `AgentState` between the no-human decision driver (which emits lifecycle events and reads the active intention to stabilize selection) and the replay rebuild + debug trace (consumers). No new event kind is required; the gap is integrated emission and the stabilization/possession rules.
4. Motivating invariants (restated): INV-034 "Agents need durable intentions" — persistent commitments resist small score jitter and yield only to stronger modeled causes; INV-033 "BDI separation is doctrine" — intentions are commitments, not guaranteed outcomes, distinct from needs/beliefs; INV-006 "Possession transfers no world knowledge" / INV-094 possession parity — controller binding changes input only, leaving the actor's intentions intact.
5. Deterministic-replay surface touched: intention lifecycle events are the replay source for `ActorIntentions`. Transitions must be emitted deterministically and applied in `events/apply.rs` so live and replay-rebuilt intention state match (agent-state checksum). No actor-knowledge leakage: an intention is the actor's own commitment; possession switching does not move it between controllers.

## Architecture Check

1. Driving selection from a durable active intention (emitted and replayed as events) gives the no-human day causal continuity — an actor pursues a commitment across ticks instead of re-deriving a goal from instantaneous scores — and makes interruption an inspectable typed event rather than an implicit re-selection. It also makes possession parity testable: the intention lives with the actor, not the controller.
2. No backwards-compatibility aliasing/shims: the active intention is read from `AgentState`; no parallel per-decision goal cache shadows it.

## Verification Layers

1. INV-034 (durable intentions) -> unit/golden test: an active intention stabilizes selection across ticks and is continued (`IntentionContinued`) linked to the chosen ordinary follow-on action.
2. INV-006 / INV-094 (possession parity) -> replay/golden-fixture check: `possession_does_not_reset_intention_001` proves attach/detach/switch never clears the active intention.
3. INV-018 (replay equality) -> replay/golden-fixture check: agent-state checksum reproduces intention status + provenance after replay.

## What to Change

### 1. Emit intention lifecycle from the integrated loop

In the no-human decision driver, adopt an intention for the selected routine/need goal (`IntentionStarted`), continue it linked to the chosen ordinary follow-on action (`IntentionContinued`), and complete/fail/suspend/resume/abandon/interrupt it with typed causes as the day progresses. Apply each in `events/apply.rs`.

### 2. Stabilize selection on the active intention

Read the active intention to bias selection (resisting small score jitter), switching only on a severe need (cause from 0007PHA3ASECHAR-005), blocker, epistemic contradiction, or scheduled routine boundary.

### 3. Possession leaves intentions intact

Ensure attach/detach/switch paths never clear an ordinary actor's active intention; strengthen `possession_does_not_reset_intention_001`.

## Files to Touch

- `crates/tracewake-core/src/agent/intention.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs` (modify)

## Out of Scope

- Routine step start/progress/failure ancestry (0007PHA3ASECHAR-007) — this ticket owns intention lifecycle, not routine-step events.
- The severe-need interruption *cause* (0007PHA3ASECHAR-005) — consumed here, defined there.
- Debug trace rendering of the selected intention / switch reason (0007PHA3ASECHAR-009).

## Acceptance Criteria

### Tests That Must Pass

1. A no-human run adopts at least one active intention and later continues, completes, fails, or interrupts it, asserted by lifecycle-event scan with ancestry linking the continuation to an ordinary follow-on action.
2. `possession_does_not_reset_intention_001`: attach/detach/switch leaves the active intention intact.
3. Replay reproduces intention status and provenance (agent-state checksum equality).
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. The active intention drives later proposals and stabilizes selection; switching requires a stronger modeled cause.
2. Possession changes input binding only; intentions remain with the actor.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/intention.rs` — unit tests: lifecycle transitions and error cases.
2. `crates/tracewake-core/src/scheduler.rs` — unit test: active intention stabilizes selection; switches on severe need/boundary.
3. `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs` — possession parity over the active intention.

### Commands

1. `cargo test -p tracewake-core agent::intention`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
