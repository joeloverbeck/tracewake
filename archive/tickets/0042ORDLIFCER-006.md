# 0042ORDLIFCER-006: ORD-LIFE-05 — routine temporal premises and modeled adaptation without ground-truth schedule cognition

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-05 must be proven: a routine may use time only through a modeled, source-backed temporal premise available in the holder-known context (assignment, memory, observation, public cue, record, testimony, institutional context, or source-backed inference); ground-truth schedule time, scheduler awakening, day-window selection, or elapsed-time accounting may validate/order execution but cannot by itself justify routine selection; and the mechanism must support a modeled experience or contradiction changing later selection through existing memory/expectation channels. This ticket records the temporal-premise ledger, true-time-without-premise negatives, freshness/supersession witnesses, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/scheduler.rs`, `agent/routine.rs`, `agent/methods.rs`, `agent/htn.rs`, `agent/decision.rs`, `agent/actor_known.rs`, `agent/no_human_surface.rs`, `agent/transaction.rs`, `crates/tracewake-core/src/epistemics/projection.rs`, `events/envelope.rs` (all confirmed in the 0042 reassessment census; `epistemics/projection.rs` was added to §2.2 of the spec as reassessment-confirmed-present).
2. Spec §5 ORD-LIFE-05 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 2/3/9 govern this ticket. Spec §10/§11: this point certifies the routine-premise mechanism ONLY; the consolidated cross-seam temporal bundle (firewall from exec 04, embodied rendering from 07, fixtures from 09, diagnostics from 10) is a routed FIRST-PROOF-CERT obligation, not an ORD-LIFE pass/fail line. Spec §10 risk: `scheduler.rs` supplies routine-window info into the decision path — the artifact must prove true day-window/schedule position is not silently treated as actor-known justification.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-05 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-05): `INV-112` (time may validate, but holder-known time must plan — scheduler/replay clock may order/validate but must not become cognition authority), `INV-100`/`INV-102`/`INV-103` (cognition-authority firewall), `INV-002`, `INV-035`/`INV-036`. Restate before trusting the narrative: a routine's temporal justification must be a source-backed holder-known premise, never a raw scheduler wake or elapsed tick.
5. Evidence-consumer surface (audit-reads, does not modify): the holder-known temporal-premise channel and the scheduler→decision boundary. This ticket runs premise-presence/freshness fixtures and holds-actor-known-history-fixed-while-changing-true-time experiments; it adds no provenance from a scheduler clock, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Holding actor-known history fixed while changing true schedule time/day-window boundaries and requiring equal cognition output (unless a modeled cue/record changes) is the decisive proof that schedule truth is not silent cognition — stronger than asserting the premise field exists.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-112` temporal authority firewall -> replay/golden-fixture check (`no_human_unseen_workplace_assignment_does_not_plan_work_001`: at a schedule tick where work is objectively appropriate but all modeled premise sources removed, the actor waits/fails with missing-knowledge evidence rather than planning work).
2. `INV-100`/`INV-103` premise provenance -> codebase grep-proof + manual review (each routine decision's temporal premise records source category + source-event IDs; a scheduler wake or elapsed tick never appears as temporal-premise provenance).
3. `INV-018`/`INV-036` premise replay + supersession -> replay/golden-fixture check (`stale_workplace_notice_superseded_by_newer_001`, `ordinary_workday_001`, `workplace_assignment_provenance_001`: freshness/supersession and deterministic selection rebuilt on clean replay; stale-premise classification distinct from validation failure and not repaired from truth).

## What to Change

### 1. Record positive temporal-premise and adaptation witnesses

Run a work/sleep routine succeeding because an event-backed assignment / remembered observation / notice / record / testimony / institutional context / source-backed inference supplies the premise; record source category + event ancestry. Use `ordinary_workday_001`, `workplace_assignment_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `stale_workplace_notice_superseded_by_newer_001` for premise presence, freshness/supersession, and deterministic selection. Prove a repeated modeled experience/contradiction/interruption/notice alters a later routine/method/trust choice through holder-known memory/expectation evidence, not an unrecorded learning shortcut. Record the temporal-premise ledger (source category, holder, proposition/fact, source-event IDs, acquisition/current tick, freshness, frontier, selected/rejected routine+method, scheduler inputs excluded from cognition, validation outcome, adaptation ancestry, paired-run result).

### 2. Record adversarial true-time-without-premise negatives

Record the §5 adversarial cases: same world at a schedule tick where work would be objectively appropriate but all modeled premise sources removed (`no_human_unseen_workplace_assignment_does_not_plan_work_001` waits/fails with missing-knowledge evidence); actor-known history held fixed while true schedule time/day-window boundaries change (cognition output stays equal unless a modeled cue/record changes); proof that `routine_window_family`/scheduler input cannot become an unproven cognition fact; and stale/contradicted workplace info + `embodied_workplace_believed_open_truth_closed_commit_fails_001` (stale-premise classification and validation failure remain distinct, not repaired from truth).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-05 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- The consolidated cross-seam temporal bundle (firewall/embodied/fixtures/diagnostics) — routed to FIRST-PROOF-CERT per spec §11; this ticket certifies the routine-premise mechanism only.
- Production remediation of any temporal-firewall defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test no_human_capstone` pass; with all modeled premise sources removed at an objectively-work-appropriate tick, the actor waits/fails with missing-knowledge evidence rather than planning work.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; `stale_workplace_notice_superseded_by_newer_001` and `workplace_assignment_provenance_001` record source category + freshness/supersession with deterministic selection.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes; clean replay rebuilds the same premise classification, source selection, later adaptation, method choice, and diagnostic, and holding actor-known history fixed while changing true time leaves cognition output equal.

### Invariants

1. Temporal-authority firewall: routine selection is justified only by a source-backed holder-known premise; scheduler wake / elapsed tick / day-window position never appears as temporal-premise provenance.
2. Stale ≠ false: stale-premise classification is distinct from validation failure and is never repaired from ground truth.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test no_human_capstone`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`

## Outcome

Completed: 2026-06-20

Recorded ORD-LIFE-05 as a certifying pass in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with evidence rows `0042-ORD05-PREMISE-PROVENANCE`, `0042-ORD05-SCHEDULER-NEGATIVE`, and `0042-ORD05-STALE-REPLAY`.

The evidence records temporal-premise provenance from modeled holder-known sources, scheduler-boundary negatives where true schedule/day-window facts do not become actor cognition, and stale/supersession replay witnesses. The consolidated cross-seam temporal bundle remains explicitly routed to FIRST-PROOF-CERT per the ticket and spec; no ORD-LIFE-05 member was deferred or dropped.

Verification commands run and passed:

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test no_human_capstone`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
