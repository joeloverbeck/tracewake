# 0042ORDLIFCER-012: ORD-LIFE-11 — scheduler no-direct-dispatch, sealed proposal ancestry, and forged/stale validation rejection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-11 must be proven: the scheduler may order actors, advance time, detect open durations, and invoke the actor decision transaction, but may not choose an ordinary target, convert a need/routine/window directly into a primitive action, call an action definition as cognition, rewrite a sealed wait/stuck reason, or apply state; every ordinary action must originate in an actor-known transaction, produce a sealed proposal with context parity and ancestry, pass validation, append an event, and only then apply. This ticket records the call/authority diagram, sealed-proposal parity witnesses, direct-dispatch and forged/stale negatives, and live/replay evidence into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/scheduler.rs`, `agent/transaction.rs` (`ActorDecisionTransaction::run`, `ActorDecisionTransactionOutcome::{Proposed,Stuck}`, `SealedProposal`), `agent/no_human_surface.rs`, `agent/decision.rs`, `actions/proposal.rs` (`ProposalSourceContext`), `actions/pipeline.rs`, `actions/registry.rs`, `actions/report.rs`, `events/{log,mutation,apply}.rs` (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-11 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 4/5/9 govern this ticket. General SPINE-CERT pipeline correctness is consumed from the predecessor 0039 pass (spec §11), not re-audited; this point certifies the ordinary-life seam's use of it.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-11 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-11): `INV-099` (cognition-authority firewall / scheduler is not cognition), `INV-103` (no-direct-dispatch), `INV-001`/`INV-009` (eventful, append-before-apply), `INV-043`, `INV-104`/`INV-106`. Restate before trusting the narrative: the scheduler orders/advances/invokes but never chooses a target, dispatches an action directly, or rewrites a sealed reason; every ordinary action flows transaction → sealed proposal → validation → append → apply.
5. Evidence-consumer surface (audit-reads, does not modify): the scheduler→transaction→proposal→pipeline authority chain and its append-before-apply replay. This ticket runs end-to-end and direct-dispatch-negative fixtures and records the authority diagram; it adds no dispatch shortcut, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. A call/authority diagram tied to concrete event IDs (scheduler input/output → sealed packet → transaction outcome → proposal parity tuple → validation report → append-before-apply witness) plus a live forged/stale rejection is the decisive no-direct-dispatch proof — spec §5 states a source scan alone is insufficient; at least one live ordinary-action path and one live forged/stale rejection are required.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No direct-dispatch shortcut may preserve a bypass.

## Verification Layers

1. `INV-099`/`INV-103` no-direct-dispatch -> codebase grep-proof + replay/golden-fixture check (scheduler-side direct action construction/dispatch from hunger/fatigue/safety/routine-family/true-schedule-time/day-window is caught by structural + runtime guards; `spine_conformance` + `anti_regression_guards`).
2. `INV-001`/`INV-009` append-before-apply + parity -> manual review + replay check (proposal source context holder/actor, context ID/hash/frontier, action kind, target, actor-known input refs identical across transaction output and validator input; append precedes apply).
3. `INV-105` sealed-reason integrity -> replay/golden-fixture check (`scheduler_cannot_rewrite_wait_reason_after_transaction_001`: any scheduler rewrite of reason/blocker/responsible layer is a failure).

## What to Change

### 1. Record positive authority-chain and parity witnesses

Trace at least eat, sleep, work, move, wait, and continue-routine outcomes from scheduler actor selection through `ActorDecisionTransaction::run`, sealed proposal/stuck outcome, pipeline validation, append, application, report, and replay. Prove proposal source context holder/actor, context ID/hash/frontier, action kind, target, and actor-known input refs are identical across transaction output and validator input. Prove scheduler ordering and skipped-open-duration behavior are deterministic and manufacture no progress or need charges. Use the possessed and no-human paths to show both reach the same ordinary proposal/validation seam.

### 2. Record adversarial direct-dispatch and forged/stale negatives

Record the §5 adversarial cases: scheduler-side direct action construction/dispatch from hunger/fatigue/safety/routine-family/true-schedule-time/day-window (structural + runtime guards catch the bypass); forged/stale context hash/frontier, actor, target, route, affordance, reservation, duration start, or proposal ancestry (validation rejects, no accepted ordinary-action event appended); validation truth discovering a true target (may reject but must not return or schedule a replacement action); and `scheduler_cannot_rewrite_wait_reason_after_transaction_001` (any scheduler rewrite of reason/blocker/responsible layer is a failure).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-11 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Re-auditing general SPINE-CERT pipeline/no-direct-dispatch architecture (consumed from the predecessor 0039 pass per spec §11; this ticket certifies only the ordinary-life seam's use).
- Production remediation of any dispatch/parity defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Ordinary-action affordance certification (`-009`), replay-derivation lock (`-013`), and the aggregate verdict / fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` and `cargo test --locked -p tracewake-core --test anti_regression_guards` pass; scheduler-side direct action construction/dispatch from needs/routines/schedule is caught by structural and runtime guards.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` passes; the proposal context-parity tuple (holder/actor, context ID/hash/frontier, action kind, target, actor-known input refs) is identical across transaction output and validator input, with append before apply.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; forged/stale parameters are rejected with no accepted ordinary-action event, and `scheduler_cannot_rewrite_wait_reason_after_transaction_001` forbids any scheduler reason/layer rewrite.

### Invariants

1. No direct dispatch: the scheduler never chooses an ordinary target, converts a need/routine/window into a primitive action, calls an action definition as cognition, rewrites a sealed reason, or applies state.
2. Sealed proposal ancestry: every ordinary action flows transaction → sealed proposal (context parity) → validation → append → apply; forged/stale parameters append no accepted event.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test spine_conformance`
2. `cargo test --locked -p tracewake-core --test anti_regression_guards`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-core --test no_human_capstone`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-20

Recorded ORD-LIFE-11 as a certifying pass in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with evidence rows `0042-ORD11-AUTHORITY-CHAIN`, `0042-ORD11-DIRECT-DISPATCH-NEGATIVES`, and `0042-ORD11-FORGED-STALE`.

The evidence records the scheduler-to-transaction-to-sealed-proposal-to-validation-to-append/apply authority chain, structural no-direct-dispatch guards, forged/stale source-context rejection, append-before-apply ordering, and scheduler wait-reason integrity. General SPINE-CERT pipeline correctness remains consumed from predecessor evidence as scoped; no ORD-LIFE-11 member was deferred or dropped.

Verification commands run and passed:

1. `cargo test --locked -p tracewake-core --test spine_conformance`
2. `cargo test --locked -p tracewake-core --test anti_regression_guards`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-core --test no_human_capstone`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
