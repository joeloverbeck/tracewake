# 0048FOUCONHAR-003: Atomic one-tick world-step transaction (additive)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — introduces a typed one-tick world-step transaction in the core scheduler with a single prepare/commit boundary and the ordinary actor / world-process / duration / accounting / perception-projection phases; invokes proposal validation through the existing pipeline seam. Built additively alongside `advance_world_one_tick`; the caller flip is ticket 004. No new event kinds, content, or fixtures.
**Deps**: 002

## Problem

Spec 0048 §4.1: the canonical one-tick coordinator is not a loaded-world step. At `cb3102e`, `DeterministicScheduler::advance_world_one_tick` (`crates/tracewake-core/src/scheduler.rs:360`) validates the expected tick, discovers due durations, appends `TimeAdvanced` + duration/need-accounting events, updates `current_tick`, and returns `WorldStepDueWorkSummary` with `actor_transactions_attempted: 0` and `world_processes_applied: 0` **unconditionally** (`scheduler.rs:410-411`); its physical input is `&PhysicalState` (immutable). It invokes no ordinary actor decision transaction and no world-process registry/cadence. §4.2 adds that the combined transition must be atomic: core must validate and prepare the complete step before committing any event or state mutation, so any failure leaves physical/agent/epistemic/log/frontier unchanged. §4.4 adds that perception/epistemic projection must occur inside the step (after authoritative effects, before stop evaluation) so the step result carries a typed holder-known delta.

This ticket builds the new typed one-tick transaction additively — autonomous-actor opportunities, due world processes, duration lifecycle, one shared need-accounting reconciliation, the perception/projection phase (producing ticket 002's typed holder-known delta), the temporal marker and ancestry — behind one prepare/commit boundary, leaving `advance_world_one_tick` live and the new step unwired. Ticket 004 (§8 closure steps 1-runtime/3) performs the atomic flip: it privatizes the frontier and reroutes the human and no-human callers into this transaction, deleting the old path.

## Assumption Reassessment (2026-06-23)

1. `advance_world_one_tick` (`crates/tracewake-core/src/scheduler.rs:360`) hard-codes `actor_transactions_attempted: 0` / `world_processes_applied: 0` (`scheduler.rs:410-411`) and takes `state: &PhysicalState` (immutable). `advance_until` (`scheduler.rs:416`) loops it. The ordinary proposal pipeline exists at `crates/tracewake-core/src/actions/pipeline.rs` (`run_pipeline`, `PipelineContext` — the seam the TUI currently calls). The no-human runner is `scheduler::no_human` (`scheduler.rs:1050`) with `run_no_human_day` (`scheduler.rs:1179`) and `advance_no_human` (`scheduler.rs:2704`) — ticket 004 reroutes these. Perception is `record_current_place_perception_and_project` / `current_place_knowledge_context` (`crates/tracewake-core/src/agent/perception.rs:35,229`).
2. Spec 0048 §4.1/§4.2/§4.4 and §8 (closure step 2) own this; §4.1 names the homes `scheduler.rs`, the existing agent decision/transaction APIs, `actions/pipeline.rs`, and the existing process-registry/cadence abstraction, and states `tracewake-tui` must implement none of these phases. Execution `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` is the full phase contract; foundation `08` ("waiting runs the simulation") and architecture `04` (due-work routing) are the controlling doctrine.
3. Cross-artifact boundary under audit: the one-tick transaction contract spanning the scheduler step, the proposal/validation pipeline (`actions/pipeline.rs`), the agent decision transaction, the perception/epistemic projection (`agent/perception.rs` producing ticket 002's sealed delta), and the world-process cadence. The step must invoke these owning seams; it must not synthesize cognition from raw truth.
4. Constitutional invariants motivating this ticket: `INV-005`/`INV-006`/`INV-108` (ordinary possession is the normal binding; possession transfers no world knowledge; possession is cognition-neutral — an unpossessed actor must get its ordinary due opportunity through the same seam), `INV-091`/`INV-094` (no-human and possession-parity proof), `INV-103` (the scheduler may select cadence and invoke the decision transaction but is not a cognition authority — it must not build proposals from raw state), `INV-001` (a control labeled "world advance" must run the modeled causes).
5. Enforcement surface (deterministic-replay + actor-knowledge): the step is the authoritative transition, so it must stay deterministic (cadence selection and phase ordering are a pure function of the prepared inputs; no wall-clock, no incidental iteration order) and the perception phase must produce only holder-known output via ticket 002's sealed projection — the scheduler must not read hidden truth into cognition (`INV-103`/`INV-099`). The prepare/commit boundary must be all-or-nothing so a failed prepare leaves replayable history untouched (the failure-atomicity test lands with the caller flip, ticket 004). Substrate-only basis for the leak/determinism guarantees the dedicated tests in tickets 004/005/006 will lock; this ticket confirms the step introduces no leakage/nondeterminism path those tickets must undo.

## Architecture Check

1. One typed transaction that receives all inputs due at the tick (optional controlled-actor slot, autonomous opportunities, due processes, durations, accounting, perception/projection, the marker and ancestry) behind a single prepare/commit boundary is the only shape that makes "waiting runs the loaded world" true and atomic at once: it removes the hard-coded zero counters and gives a single commit point a failed prepare can roll back. Selecting cadence and invoking the existing decision transaction (rather than synthesizing proposals) keeps the scheduler out of cognition authority (`INV-103`). Existing proposal validation and event application remain the owning seams — the step composes them, it does not reimplement them.
2. No backwards-compatibility aliasing/shims: the new transaction is not a wrapper around `advance_world_one_tick`; it is the replacement path, built additively only so the caller flip (ticket 004) can be one atomic diff that deletes the old method. No parallel "old vs new" step survives past ticket 004.

### Implementer-recorded choices (spec §9, settled doctrine — recorded, not re-litigated)

- **Atomic staging strategy (§9.1)**: clone/scratch aggregate state + log, or a prepared event/application batch with explicit commit. Either is acceptable *only if* the all-or-nothing test (ticket 004) proves every authority surface unchanged on failure. Record the chosen strategy and its rollback argument in the implementation.
- **Actor/process cadence source (§9.2)**: which existing scheduler/transaction structures enumerate due autonomous opportunities and due world processes. The step must invoke the owning abstractions without becoming cognition authority; record the chosen enumeration source.

## Verification Layers

1. `INV-001`/`INV-091` loaded-world step -> replay/golden-fixture check: a step with ≥1 due unpossessed actor and ≥1 due world process commits typed events and reports measured nonzero `actor_transactions_attempted` / `world_processes_applied` (no longer hard-coded zero).
2. `INV-103` scheduler-not-cognition -> manual review + codebase grep-proof: the step invokes the existing decision transaction / proposal pipeline (`actions/pipeline.rs`) and the process cadence, and constructs no proposal from raw truth.
3. `INV-099`/`INV-018` no-leak + determinism (substrate) -> manual review (epistemic-leakage + determinism audit): the perception phase emits only ticket 002's holder-known delta, and phase ordering/cadence is a pure function of prepared inputs.
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. Typed one-tick world-step transaction with prepare/commit boundary

In `crates/tracewake-core/src/scheduler.rs`, add a typed one-tick transaction that takes the optional controlled-actor input/proposal slot, deterministic autonomous-actor opportunities due at the tick, deterministic world-process work due at the tick, open-duration lifecycle work, one shared need-accounting reconciliation, the perception/epistemic projection phase, and the temporal marker plus ancestry — staged behind one prepare/commit boundary (per the recorded staging strategy). Return measured `actor_transactions_attempted` / `world_processes_applied` counts.

### 2. Invoke the owning seams

Route the ordinary actor proposal through the existing proposal/validation pipeline (`crates/tracewake-core/src/actions/pipeline.rs`) and the agent decision transaction; invoke the world-process cadence abstraction. The perception/projection phase calls `crates/tracewake-core/src/agent/perception.rs` to produce ticket 002's typed holder-known delta, carried on the step result for stop evaluation (consumed by ticket 005's `advance_until`).

### 3. Phase tests

In `crates/tracewake-core/tests/world_step_coordinator.rs`, add step-level phase tests asserting the new transaction runs the unpossessed-actor and world-process phases and reports nonzero counts (the full held-equal differential is ticket 006; these are the per-phase unit assertions).

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — new one-tick transaction; `advance_world_one_tick` retained until ticket 004; shared with ticket 004)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — invoked inside the transaction; signature/seam adjustments as needed)
- `crates/tracewake-core/src/agent/perception.rs` (modify — perception/projection phase callable inside the step, producing ticket 002's sealed delta)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — per-phase nonzero-count tests; shared with tickets 001/004/006 — append distinct items)

## Out of Scope

- Privatizing `current_tick`, deleting `sync_scheduler_frontier_to_appended_events`, the TUI assignment removals, rerouting the human/no-human callers into the new transaction, and deleting `advance_world_one_tick` — all ticket 004 (the atomic flip).
- The failure-atomicity (all-or-nothing) test and the runtime marker-chain property set — ticket 004 (those exercise the committed authority path the flip establishes).
- Wiring `advance_until`'s salient stop and the interval summary to the typed delta — ticket 005.
- The non-vacuous held-equal human/no-human differential — ticket 006.
- Any change to `TimeAdvanced` emission semantics (§3.1 preserved holding) beyond the step emitting exactly one marker per increment.

## Acceptance Criteria

### Tests That Must Pass

1. A one-tick transaction with ≥1 due unpossessed actor commits that actor's typed ordinary event and reports `actor_transactions_attempted >= 1`.
2. A one-tick transaction with ≥1 due world process commits its typed event and reports `world_processes_applied >= 1`.
3. `cargo test -p tracewake-core` passes with the new transaction built alongside `advance_world_one_tick`.

### Invariants

1. The step constructs no proposal from raw truth — it invokes the existing proposal/validation pipeline and decision transaction (`INV-103`).
2. The reported actor/process counts are measured from work actually attempted/applied, never literals (`INV-001`/`INV-091`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — per-phase tests: unpossessed-actor transaction committed, due world-process applied, measured nonzero counts.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core` (confirms the new transaction + pipeline-seam changes compile crate-wide).

## Outcome

Completed: 2026-06-23

Implemented an additive `DeterministicScheduler::transact_world_one_tick` path in `crates/tracewake-core/src/scheduler.rs`. The new path stages physical state, agent state, epistemic projection, and event log on scratch clones, prepares the time marker/duration lifecycle/accounting work, applies supplied typed world-process events, records/project current-place perception for due actors, invokes `ActorDecisionTransaction`, and commits accepted actor proposals through `run_pipeline`. The commit boundary copies the staged authorities back only after prepare/application succeeds, leaving the existing `advance_world_one_tick` caller path in place for ticket 004.

Actor-known wait framing uses a typed diagnostic frame event caused by the step marker so modeled wait/reevaluation facts retain event-backed provenance accepted by the transaction witness allowlist. The scheduler does not synthesize actor proposals from raw state; it builds the actor-known surface from projected event evidence and hands proposal selection to the existing decision transaction.

Added `world_step_coordinator` tests proving:

1. A due actor transaction commits a typed ordinary `ActorWaited` event and reports a nonzero `actor_transactions_attempted`.
2. A due typed process event is committed and reports a nonzero `world_processes_applied`.

Verification run:

1. `cargo test -p tracewake-core --test world_step_coordinator` — passed.
2. `cargo test -p tracewake-core` — passed.
3. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
4. `git diff --check` — passed.
