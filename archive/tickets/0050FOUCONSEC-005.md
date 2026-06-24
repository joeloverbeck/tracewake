# 0050FOUCONSEC-005: Unify the actor transition — consume the full decision outcome; delete the dual no-human choreography

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds a closed actor-step outcome consumed atomically by the coordinator; deletes the parallel no-human append choreography
**Deps**: 0050FOUCONSEC-003

## Problem

Spec-0050 §4.3 (driver F-03): the actor decision transaction is not consumed as one authoritative transaction. `ActorDecisionTransactionOutcome` (in `crates/tracewake-core/src/agent/transaction.rs`) carries `Proposed` (proposal, decision trace, trace record, lifecycle effects, local plan) and `Stuck { diagnostic }`, but the coordinator destructures only `Proposed`, takes `proposed.proposal`, `continue`s on `Stuck`, and discards the `PipelineResult`. A separate no-human choreography — `build_agent_proposal` (`scheduler.rs:2045`) plus `append_decision_trace_after_proposal` (`2542`), `append_intention_lifecycle_after_proposal` (`2815`), `append_routine_step_events_after_proposal` (`3005`) — reconstructs a subset of diagnostics after pipeline execution. Two competing choreographies, partial commitment, weakened atomicity.

This ticket makes the coordinator consume and commit the full typed outcome atomically, routes no-human actor opportunities through the same coordinator, and **deletes** the parallel append choreography.

## Assumption Reassessment (2026-06-24)

1. `ActorDecisionTransactionOutcome` is `Proposed(Box<ActorDecisionProposalOutcome>)` / `Stuck { diagnostic: Box<StuckDiagnosticRecord> }` in `crates/tracewake-core/src/agent/transaction.rs`; the coordinator drops trace/lifecycle/plan/pipeline-result; `build_agent_proposal` (`scheduler.rs:2045`) and `append_{decision_trace,intention_lifecycle,routine_step_events}_after_proposal` (`scheduler.rs:2542`/`2815`/`3005`) form the parallel no-human path. The coordinator returns `WorldAdvanceResult` (`scheduler.rs:193`). Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §4.3 and §9.3 are authoritative: §9.3 assigns the *exact internal enum/struct* joining trace, local plan, lifecycle, stuck, pipeline result, and resulting event IDs to the implementer as a recorded choice; the compiler must force every variant handled. `-003` already routes eligible actors into the coordinator (their decision transactions are invoked there); this ticket makes the coordinator *consume the full result*.
3. Shared boundary under audit: the actor-decision boundary between `agent/transaction.rs` (produces the outcome) and the coordinator in `scheduler.rs` (must consume it), plus the no-human module inside `scheduler.rs`. Adjacent contradiction: perception/interval ownership (F-04) shares the coordinator but is `-006`'s scope.
4. `INV-099` (truth validates, does not plan), `INV-103` (scheduler is not cognition authority), `INV-104` (needs/routines do not bypass candidate generation/planning), `INV-105` (decision traces are authoritative diagnostic data), `INV-041` (decision debug traces) motivate this ticket: the full trace/stuck/lifecycle/plan must survive one authoritative transaction, not be reconstructed by scheduler-side choreography.
5. Enforcement surface: actor-knowledge / truth firewall (`INV-099`/`INV-101`/`INV-106`) and deterministic replay (`INV-018`). The committed pipeline result must expose only actor-legible modeled feedback (no validator-truth leak), and all committed events (decision, proposal, validation, lifecycle, diagnostics) must be replay-reconstructible. Deleting the parallel append path removes a second, non-atomic event-emission sequence.
6. **Schema/contract shape change (additive-vs-breaking)**: `WorldAdvanceResult` (`scheduler.rs:193`) is extended with a typed actor-step summary; consumers are the in-workspace callers of the coordinator (the five production entry points and `world_step_coordinator.rs` tests). The extension is additive to `WorldAdvanceResult` (new field/summary), but the *closed actor-step outcome* the coordinator handles is a breaking internal contract (it cannot be reduced to `Option<Proposal>`). Both are in-workspace; updated in this diff.
7. **Removal blast radius**: `build_agent_proposal` and the three `append_*_after_proposal` helpers are deleted. Grep blast radius: their only callers are the no-human loop in `scheduler.rs` (rerouted here); no `docs/`, `specs/`, or `.claude/skills/` consumer references them. The no-human capstone/ordinary-life tests in `crates/tracewake-core/tests/` that assert the diagnostics now assert them via the unified path.

## Architecture Check

1. A closed, exhaustively-handled actor-step outcome that cannot collapse to `Option<Proposal>` forces the coordinator to commit every artifact class — the compiler enforces completeness, replacing the current silent drop. Routing no-human opportunities through the same coordinator eliminates the second choreography rather than maintaining two.
2. No backwards-compatibility shims: `build_agent_proposal` and the `*_after_proposal` helpers are deleted, not wrapped. No dual path remains.

## Verification Layers

1. `INV-105`/`INV-041` (authoritative diagnostics) → replay/golden-fixture check: a real one-tick actor run proves trace → proposal → validation → event ancestry committed in one transaction; a `Stuck` run proves the typed diagnostic is committed with no primitive action.
2. `INV-099`/`INV-106` (no validator-truth leak) → manual review + actor-knowledge negative: a rejection run proves actor-legible feedback/lifecycle without validator-only detail leaking.
3. `INV-094`/`INV-108` (possession parity) → replay/golden-fixture check: the same actor opportunity via possessed wait and no-human scheduling yields identical artifacts apart from controller-origin metadata.
4. `INV-103`/`INV-104` (scheduler not cognition; routines route through planning) → codebase grep-proof: `build_agent_proposal` and the `*_after_proposal` helpers are gone; no scheduler-side post-commit diagnostic append remains.

## What to Change

### 1. Define the closed actor-step outcome and consume it exhaustively

In `crates/tracewake-core/src/agent/transaction.rs` and `scheduler.rs`, define the closed internal outcome (implementer-recorded choice per §9.3) preserving: selected proposal + ancestry; decision trace + trace record; local-plan identity/content; intention/routine lifecycle effects; `StuckDiagnosticRecord`; pipeline accepted/rejected/failed result with actor-legible feedback; resulting event IDs. The coordinator must handle every variant (no `Stuck => continue` drop) and commit all products inside the same scratch transaction.

### 2. Expose a typed actor-step summary in `WorldAdvanceResult`

Extend `WorldAdvanceResult` (`scheduler.rs:193`) with the typed per-actor outcome summary so callers observe committed artifacts.

### 3. Route no-human actor opportunities through the coordinator; delete the parallel choreography

Make `run_no_human_day` / the no-human window machinery schedule actor opportunities through the unified coordinator. Delete `build_agent_proposal` and `append_{decision_trace,intention_lifecycle,routine_step_events}_after_proposal`.

### 4. Witnesses

In `crates/tracewake-core/tests/world_step_coordinator.rs`, add: a `Proposed` ancestry witness; a `Stuck` committed-diagnostic witness; a rejection actor-legible-feedback witness (no truth leak); and the possessed-vs-no-human artifact-equality differential. Each must fail if any artifact append is removed.

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify)

## Out of Scope

- Perception/interval core ownership and TUI de-authority — `0050FOUCONSEC-006`.
- `EventId` uniqueness — `0050FOUCONSEC-007`.
- Salience policy — `0050FOUCONSEC-009`.

## Acceptance Criteria

### Tests That Must Pass

1. `Proposed` run commits trace/trace-record/lifecycle/local-plan/proposal/pipeline-result in one transaction with correct event ancestry; removing any append fails the test.
2. `Stuck` run commits the typed diagnostic with no primitive action; rejection run yields actor-legible feedback without validator-truth leak.
3. Possessed-vs-no-human artifact-equality differential passes; `cargo test -p tracewake-core --test world_step_coordinator` and `cargo build --workspace --all-targets --locked` green.

### Invariants

1. One authoritative actor transaction commits all typed artifacts; the coordinator exhaustively handles a closed outcome that cannot reduce to `Option<Proposal>` (`INV-105`/`INV-041`/`INV-099`).
2. No parallel post-commit diagnostic choreography remains; no-human and possessed paths share the coordinator (`INV-103`/`INV-104`/`INV-094`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — Proposed/Stuck/Rejected committed-artifact witnesses + possessed/no-human artifact-equality differential + per-artifact removal negatives.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

The world-step coordinator now consumes `ActorDecisionTransactionOutcome`
exhaustively. `Proposed` actor steps run the sealed proposal through the shared
pipeline, then commit transaction-owned intention lifecycle and decision trace
artifacts in the same scratch transaction. `Stuck` actor steps now commit the
typed stuck diagnostic instead of silently continuing. `WorldAdvanceResult`
exposes `actor_step_summaries` with per-actor status, proposal/pipeline status,
committed event ids, and diagnostic event id.

The no-human transaction helper no longer drops `Stuck` outcomes: it commits the
transaction-produced stuck diagnostic through the same diagnostic event builder.
The legacy helper names called out by the ticket
(`build_agent_proposal`, `append_decision_trace_after_proposal`,
`append_intention_lifecycle_after_proposal`,
`append_routine_step_events_after_proposal`) were removed from core source and
core tests, and the anti-regression guards now target the renamed actor-decision
transaction helper.

Additional cleanup: the declared world-process cadence constructor no longer
panics on zero cadence; it normalizes to a minimum cadence of one tick so the
scheduler apply/completion no-panic guard remains true.

Verification run:

- `cargo test -p tracewake-core --test world_step_coordinator`
- `cargo test -p tracewake-core --lib scheduler::no_human::tests`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `rg -n "build_agent_proposal|append_decision_trace_after_proposal|append_intention_lifecycle_after_proposal|append_routine_step_events_after_proposal|Stuck \\{.*\\} => None|Stuck \\{.*\\} => continue" crates/tracewake-core/src crates/tracewake-core/tests` (no matches)
- `cargo fmt --all --check`
- `git diff --check`
