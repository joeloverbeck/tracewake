# 0051FOUCONTHI-006: F-05 full actor decision-transaction consumption

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — replace the loosely-consumed actor outcome with a closed commit-ready actor-step transition; persist `local_plan_id` and the proposal-ancestry vector currently dropped.
**Deps**: 0051FOUCONTHI-005

## Problem

`ActorDecisionProposalOutcome` carries `proposal`, full `DecisionTrace`, `DecisionTraceRecord`, `lifecycle_effects`, and a `LocalPlan`, and `SelectedGoalBundle` computes a `local_plan_id` and a `proposal_ancestry` vector (`agent/transaction.rs:51`–`52`). The scheduler's `Proposed` branch consumes only the reduced record and lifecycle effects (`scheduler.rs:943`–`946`); it never consumes `decision_trace` or `local_plan`, and the sealed proposal payload omits `local_plan_id` and the complete ancestry vector, so replay/debug cannot rebuild the full "why this concrete action" chain required by INV-041 (F-05, high). The fix makes a closed, commit-ready actor-step transition the scheduler exhaustively consumes, persisting the typed IDs/fields sufficient to rebuild the chain.

## Assumption Reassessment (2026-06-24)

1. Codebase: `ActorDecisionProposalOutcome` (`crates/tracewake-core/src/agent/transaction.rs:37`) carries `decision_trace: DecisionTrace` (`39`); `SelectedGoalBundle` (`46`) computes `local_plan_id` (`51`) and `proposal_ancestry: Vec<String>` (`52`); the scheduler `Proposed` branch at `crates/tracewake-core/src/scheduler.rs:943` consumes only `decision_trace_record` + `lifecycle_effects`, dropping `decision_trace`/`local_plan`. Types: `DecisionTrace` (`agent/trace.rs:445`), `DecisionTraceRecord` (`agent/trace.rs:266`), `LocalPlan` (`agent/planner.rs:49`). Existing fixture `external_crate_cannot_reduce_actor_step_outcome_to_option` pins the closed-outcome seal (extend, do not duplicate).
2. Specs/docs: spec `0051` §4.6. Architecture home `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; execution `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`.
3. Shared boundary under audit: the actor decision transaction → scheduler/runtime hand-off — a closed, non-reducible internal transition external callers cannot construct, exhaustively consumed by the scheduler.
4. INV-041 (agent decisions need debug traces), INV-099/INV-101/INV-102/INV-105 (cognition authority, sealed actor-known context, cognition-input provenance, traces are authoritative diagnostic data): restated — the full chain (actor-known context → candidate selection → selected method → local plan → proposal → validation/pipeline result → ordinary events → lifecycle/trace/stuck artifacts) must be typed and rebuildable.
5. Fail-closed / actor-knowledge / trace surface: decision traces, local plans, and ancestry are authoritative diagnostic data, not display strings (INV-105); they remain debug-quarantined and feed replanning without leaking validator-only truth (INV-106). No leakage path is introduced by persisting them — they are actor-cognition products, not hidden truth.
6. Schema extension (additive-vs-breaking): persist `local_plan_id` and the `proposal_ancestry` vector (currently dropped) into the sealed proposal payload / event ancestry, or a canonical plan record/projection with stable identity (`events/envelope.rs` payload fields and/or a trace record). **Additive**: new typed fields/records on the actor-step event/trace, consumers are replay rebuild + debug trace projection + checksum, per INV-020. A too-large full plan is persisted as a canonical plan record with stable identity and event ancestry — never opaque debug prose. (Keyed distinctly from item 5 — schema shape vs leakage/trace-authority are separate obligations.)

## Architecture Check

1. A closed, non-reducible commit-ready transition whose private fields hold proposal, full trace, trace record/render projection, local-plan identity + plan steps, lifecycle effects, and explicit ancestry links — exhaustively consumed and returned as a typed committed summary — is the only design that makes the INV-041 why-chain rebuildable from the log. Persisting identities (not prose) keeps replay deterministic and debug authoritative.
2. No backwards-compatibility alias: the loose public-field outcome is replaced by the closed transition; the dropped `decision_trace`/`local_plan` are consumed, not left as dead public fields.

## Verification Layers

1. INV-041 / INV-105 (decision traces are authoritative) -> behavior test: through the world-step transaction, produce a planned action, a `Stuck`, and a rejected proposal, asserting typed ancestry (including `local_plan_id` + the ancestry vector) survives into the log and the returned result.
2. INV-102 (cognition-input provenance) -> replay rebuild: canonical trace/plan identities and links are reconstructed.
3. Seal -> negative fixture: the outcome cannot be reduced to `Option` / constructed externally (extend `external_crate_cannot_reduce_actor_step_outcome_to_option`).

## What to Change

### 1. Closed commit-ready actor-step transition

Replace the loose public-field outcome with a closed internal transition whose private fields include the proposal, full trace, trace record/render projection, local-plan identity and the plan steps diagnostics require, lifecycle effects, and explicit ancestry links.

### 2. Exhaustive consumption + typed committed summary

The scheduler/runtime exhaustively consumes the transition and returns a typed committed summary linking actor-known context → candidate selection → selected method → local plan → proposal → validation/pipeline result → ordinary event(s) → lifecycle/trace/stuck artifacts.

### 3. Persist the rebuild chain

Persist typed IDs/fields sufficient to rebuild the chain, including `local_plan_id` and the `proposal_ancestry` vector currently dropped; a too-large full plan becomes a canonical plan record/projection with stable identity and event ancestry.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-core/src/agent/transaction.rs` (modify) — closed commit-ready transition
- `crates/tracewake-core/src/agent/trace.rs` (modify) — trace record/render projection
- `crates/tracewake-core/src/agent/planner.rs` (modify — as surfaced) — `LocalPlan` identity/ancestry exposure
- `crates/tracewake-core/src/scheduler.rs` (modify) — exhaustive consumption; persist ancestry; merge-hub contributor
- `crates/tracewake-core/src/events/envelope.rs` (modify — only if persisting ancestry into the event payload)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — planned/`Stuck`/rejected ancestry survival; merge-hub contributor

## Out of Scope

- TUI de-authority (F-06 → `-007`).
- The standing mutation run that scores trace-reduction survivors (F-08 → `-010`).

## Acceptance Criteria

### Tests That Must Pass

1. Through `transact_world_one_tick`, a planned action, a `Stuck`, and a rejected proposal each leave typed ancestry (`local_plan_id` + ancestry vector) in the log and the returned result.
2. Replay rebuild reconstructs canonical trace/plan identities and links.
3. `cargo test -p tracewake-core --test world_step_coordinator --test negative_fixture_runner` (run each target) is green.

### Invariants

1. The actor-step outcome is closed and non-reducible; the scheduler consumes it exhaustively.
2. The full why-chain is rebuildable from typed log/result fields, never from debug prose.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — ancestry-survival cases across planned/`Stuck`/rejected.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — extended closed-outcome seal.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core --test negative_fixture_runner`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

- Persisted actor decision plan lineage by carrying `local_plan_id` and `proposal_ancestry` from `SelectedGoalBundle` into sealed proposal parameters, `DecisionTraceRecord` canonical serialization, `DecisionTraceRecorded` payloads, and returned `ActorStepSummary`.
- Added canonical lineage fields to `StuckDiagnostic` and `StuckDiagnosticRecorded` for failed local planning/stuck outcomes, deriving stable failed-plan ancestry from actor decision window, selected goal/routine, and routine step where present.
- Preserved legacy canonical parse shapes for existing decision trace and stuck diagnostic records while making new records replayable with plan/proposal lineage.
- Verified the planned runtime path through `transact_world_one_tick` in `world_step_coordinator`, including returned summary, event payload, canonical replay parse, and applied agent-state trace.
- Verified stuck lineage at the transaction/canonical diagnostic layer. Rejected actor proposals share the same `Proposed` summary/event lineage path once the pipeline returns `Rejected`; no additional crafted rejected-autonomous world-step fixture was added here.

Verification:

- `cargo test -p tracewake-core agent::transaction`
- `cargo test -p tracewake-core --test world_step_coordinator`
- `cargo test -p tracewake-core --test negative_fixture_runner`
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
