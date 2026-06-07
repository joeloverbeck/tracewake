# 0005PHA3ANEEROU-002: Candidate-goal and durable-intention model

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `CandidateGoal`, `GoalKind`, `Intention`, and `IntentionStatus` types to the `agent` module with deterministic tie-break ordering.
**Deps**: 0005PHA3ANEEROU-001

## Problem

Phase 3A actors must turn need pressure and routine duties into inspectable candidate options, then commit to one durable intention that survives time, possession, and replay (Spec 0005 §8.3, §8.4; `INV-033`, `INV-034`). Needs must not directly choose actions — they create pressure and candidate goals, and an actor continues a current intention rather than flipping every tick (Spec §8.3 default selection rule, §3.5). The candidate-goal/intention types are the contract the generation engine (ticket 012), the routine selector (ticket 013), events (ticket 005), and the content schema (ticket 015) all reference, so they land before any decision behavior.

## Assumption Reassessment (2026-06-07)

1. The need vocabulary and cognition IDs this ticket builds on land in ticket 001 (`crates/tracewake-core/src/agent/need.rs`, `IntentionId`/`CandidateGoalId` in `ids.rs`); this ticket adds sibling files under the same `agent` module and depends on 001 having registered `pub mod agent;` in `lib.rs`.
2. Spec §8.3 fixes the candidate-goal field set (actor, tick/window, source, goal kind, priority band/reason, belief inputs, applicability result, rejection reason, selected method, trace ID) and the deterministic tie-break order (severe safety → severe hunger → severe fatigue → active-intention continuation → urgent hunger/fatigue → routine-window duty → return/sleep window → idle-with-reason → stable actor/goal ID). Spec §8.4 fixes the intention field set and the status set (active, suspended, completed, failed, abandoned, interrupted).
3. Shared boundary under audit: `GoalKind` and `IntentionStatus` are string-enum-shaped dispatch vocabularies consumed by ticket 005 (event payloads), ticket 012 (generation), ticket 013 (method selection), and ticket 023 (debug projection). The variant sets are fixed here; a later addition needs an arm at each consumer.
4. Invariants motivating this ticket: `INV-033` — "BDI separation is doctrine" (beliefs, needs/desires/goals, and intentions are distinct; needs do not grant truth; intentions are commitments not guaranteed outcomes) — and `INV-034` — "Agents need durable intentions". The `Intention` type must be a commitment record with status transitions, not a per-tick recomputation, and must carry belief/perception inputs separately from need pressure.
5. Deterministic-replay surface (substrate-only): candidate-goal ordering and intention selection must be stable and replay-reconstructible. This ticket introduces no nondeterminism — tie-breaking terminates on stable actor/goal IDs (by-value `Ord` from ticket 001), and the types derive equality/ordering from structured fields. The enforcing surface (deterministic candidate-goal evaluation + replay) lands in tickets 012 and 006; this ticket guarantees the comparison keys are canonical.

## Architecture Check

1. Separating `CandidateGoal` (a per-tick inspectable option with a rejection reason) from `Intention` (a durable committed record with a status lifecycle) encodes the BDI separation directly in the type system, preventing the "utility-recomputed-every-tick" jitter the spec warns against (§25.4) — a single fused "current action" struct would lose the durability and the rejected-option audit trail. A total tie-break order ending in stable IDs guarantees determinism without RNG.
2. No backwards-compatibility shims: greenfield types in the new `agent` module; no aliasing of existing action/epistemic types.

## Verification Layers

1. BDI separation (`INV-033`) -> invariants alignment check + manual review: `Intention` carries belief/perception inputs and a status distinct from need pressure; no constructor derives a commitment directly from a raw need value.
2. Durable intentions (`INV-034`) -> unit test: an `Intention` transitions through active→{suspended,completed,failed,abandoned,interrupted} and a mild need delta does not by itself replace an active intention (continuation preferred), proven on the tie-break comparator.
3. Determinism (`INV-018`, substrate-only) -> codebase grep-proof + unit test: the candidate-goal ordering is a total order terminating in stable IDs; a sort-stability test proves identical inputs yield identical ordering. Full evaluation/replay enforcement deferred to tickets 012/006 (cited).

## What to Change

### 1. Candidate-goal model

Add `crates/tracewake-core/src/agent/candidate.rs` with `GoalKind` (eat, find food, sleep/rest, go to work, perform work block, return home, continue current intention, idle-with-reason, leave-unsafe) and `CandidateGoal` carrying the §8.3 fields. Implement the deterministic tie-break ordering as a total comparator (Spec §8.3 recommended default order), terminating on stable actor/goal IDs. Provide a rejection-reason field and an applicability-result field; no field reads ground truth.

### 2. Intention model

Add `crates/tracewake-core/src/agent/intention.rs` with `IntentionStatus` (active, suspended, completed, failed, abandoned, interrupted) and `Intention` carrying the §8.4 fields (actor, source/trigger, selected goal, selected routine/method handle, current step, commitment/durability level, start tick, last-progress tick, status, failure/interruption reason, trace ancestry). Provide status-transition helpers that record the reason; an actor holds zero or one current intention plus optional suspended ones.

### 3. Module registration

Add `pub mod candidate;` and `pub mod intention;` to `crates/tracewake-core/src/agent/mod.rs` and re-export the public surface.

## Files to Touch

- `crates/tracewake-core/src/agent/candidate.rs` (new)
- `crates/tracewake-core/src/agent/intention.rs` (new)
- `crates/tracewake-core/src/agent/mod.rs` (modify — file created by 0005PHA3ANEEROU-001; add `pub mod candidate; pub mod intention;`)

## Out of Scope

- Routine template/execution model (ticket 003).
- Decision trace and stuck diagnostic models (ticket 004).
- Candidate-goal generation behavior and intention switching logic (ticket 012).
- Routine method selection (ticket 013).
- Event payloads and replay for intentions (tickets 005, 006).

## Acceptance Criteria

### Tests That Must Pass

1. The candidate-goal comparator is a total order: a fixed set of candidates sorts identically across repeated runs and severe-safety outranks severe-hunger which outranks active-intention continuation which outranks mild need pressure.
2. `Intention` status transitions are reason-bearing and reject illegal transitions (e.g. completed→active) without a fresh adoption.
3. A mild need delta does not by itself reorder above an active-intention continuation in the comparator (jitter guard).

### Invariants

1. An actor has at most one `active` intention; commitment is a stored record, not a per-tick recomputation (`INV-034`).
2. Candidate-goal ordering terminates on stable IDs, never RNG or hash iteration (`INV-033` separation + determinism).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/candidate.rs` (unit tests) — total-order/tie-break, jitter guard, rejection-reason presence.
2. `crates/tracewake-core/src/agent/intention.rs` (unit tests) — status transition legality and reason recording.

### Commands

1. `cargo test -p tracewake-core agent::candidate`
2. `cargo test -p tracewake-core agent::intention`
3. Core-crate unit scope is correct because generation/selection consumers (012, 013) and event consumers (005) land later.

## Outcome

Completed: 2026-06-07

What changed:
- Added `agent::candidate` with `GoalKind`, `CandidateGoalSource`, `GoalPriority`, `ApplicabilityResult`, and `CandidateGoal`.
- Implemented deterministic candidate ordering that ranks severe safety, severe hunger, severe fatigue, active-intention continuation, urgent need pressure, routine duties, return/sleep windows, idle, then mild need pressure, terminating ties on stable actor and candidate-goal IDs.
- Added `agent::intention` with `IntentionStatus`, `IntentionSource`, durable `Intention`, reason-bearing lifecycle transitions, and `ActorIntentions` enforcing at most one active intention.
- Registered and re-exported the new public surfaces from `agent::mod`.

Deviations from original plan:
- Kept belief/perception inputs as canonical string fragments for this substrate ticket; typed belief consumers remain deferred to candidate generation and trace/event tickets.
- Represented the selected routine/method handle as `RoutineTemplateId` until the dedicated routine-template/execution model lands in ticket 003.

Verification results:
- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core agent::candidate` passed.
- `cargo test -p tracewake-core agent::intention` passed.
