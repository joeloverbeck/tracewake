# 0042ORDLIFCER-003: ORD-LIFE-02 — actor-known candidate generation, deterministic priority, and hidden-target exclusion

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-02 must be proven: need, routine, safety, and current-intention pressures may synthesize candidate goals only from the sealed actor-known surface and source-backed routine inputs; generation must be deterministic for the same packet, retain source/priority evidence for selected and rejected candidates, and never convert hunger, fatigue, safety, fixture possibility, validation truth, or unobserved physical truth into a true target. This ticket records the ORD-LIFE-02 positive determinism witnesses, paired hidden-truth equality, fail-closed provenance negatives, and live/replay evidence into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/generation.rs`, `agent/candidate.rs` (`CandidateGoal`, `CandidateGoalSource`), `agent/need.rs`, `agent/decision.rs` (`DecisionInput`, `select_goal_and_trace`, `ActorKnownInputRef`, `ActorKnownInputSourceClass`), `agent/actor_known.rs`, `agent/no_human_surface.rs` (`SealedActorKnownSurface`, `NoHumanActorKnownSurfaceBuilder::from_projection`), `agent/intention.rs`, `scheduler.rs` (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-02 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 2/7/9 govern this ticket; §4.4 "actor-known cognition" and "relational hidden-truth proof" bind it — the proof is relational over paired runs, not a banned-word scan.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-02 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-02): `INV-024` (no telepathy / actor-knowledge filtering), `INV-099`–`INV-104` (truth-firewall / cognition-authority set), `INV-002`, `INV-039`, `INV-040`. Restate before trusting the narrative: candidate output must be a deterministic function of the sealed actor-known packet alone; hidden physical/validation truth may not enter it.
5. Evidence-consumer surface (audit-reads, does not modify): the actor-knowledge firewall feeding candidate generation, plus deterministic packet/candidate ordering. This ticket runs paired/identical-packet fixtures and records candidate censuses; it adds no admitted source class, introduces no leakage path, and any instrumentation comparing hidden truth to actor-known input stays observer-only and non-diegetic.

## Architecture Check

1. Proving exclusion relationally — paired fixtures differing only in hidden food/route/affordance/workplace/schedule truth must yield equal candidate output until a legal observation/record changes the actor-known packet — is strictly stronger than a forbidden-word scan and is what spec §5/§4.4 require.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-024` / `INV-100`–`INV-104` no-telepathy in cognition -> replay/golden-fixture check (paired hidden-truth fixtures yield equal candidate set/priorities/source refs until a legal observation distinguishes them).
2. `INV-040` actor-known provenance for candidates -> codebase grep-proof + manual review (each candidate carries source class + source-event IDs; empty/dangling/wrong-holder/wrong-kind/debug-only/validation-truth/unproven-physical/future-frontier refs fail closed or exclude with typed evidence).
3. `INV-002`/`INV-039` deterministic generation -> replay/golden-fixture check (identical packet → identical candidate set, priorities, tie-break order, source references under insertion-order canonicalization).

## What to Change

### 1. Record positive determinism and known-target witnesses

Run the §5 positives: with event-backed knowledge of reachable food / sleep place / workplace assignment / safe exit, prove the corresponding candidate family is generated with actor-known source references and deterministic total ordering; with hunger but no known food target, prove search/fallback/wait/stuck-capable candidates rather than a world-synthesized target; prove a durable intention remains a candidate continued under mild pressure and interruptible under severe pressure via a typed lifecycle path; repeat identical-packet generation and record the canonicalized candidate census, priorities, tie-breaks, and source refs.

### 2. Record relational hidden-truth and fail-closed negatives

Exercise the §5 adversarial fixtures (`no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `seeded_food_source_unknown_to_all_actors_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `no_human_known_workplace_requires_provenance_001`, `forbidden_provenance_input_fails_closed_001`). Record paired-run equality, the complete sealed packet + context ID/hash/frontier, the candidate census in deterministic order, and live negatives for empty/dangling/wrong-holder/wrong-kind/debug-only/validation-truth/unproven-physical/future-frontier source references. Prove changing only a rejected candidate's hidden target or fixture prose changes no selected goal, proposal, event, or metric.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-02 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any leakage/determinism defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec per spec §2/§8).
- Routine temporal premises (`-006`), method selection / local planning (`-007`), and planner-trace debug quarantine (`-008`) — consumed here only insofar as candidate generation depends on them.
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` passes; recorded paired runs differing only in hidden truth produce equal actor-known packets and equal candidate output until a legal observation/record distinguishes them.
2. `cargo test --locked -p tracewake-core --test generative_lock` and `cargo test --locked -p tracewake-core --test acceptance_gates` pass; identical-packet generation is deterministic in candidate set, priorities, tie-break order, and source references.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; `forbidden_provenance_input_fails_closed_001` and the dangling/wrong-kind source-reference negatives fail closed with typed evidence.

### Invariants

1. No telepathy in cognition: hidden food/route/affordance/workplace/schedule truth never alters candidate output before a legal observation/record changes the actor-known packet.
2. Provenance fail-closed: every candidate cites source class + source-event IDs; forbidden/dangling/wrong-kind/debug-only/validation-truth provenance is excluded or fails closed.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only / non-diegetic.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test no_human_capstone`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-20

Populated the ORD-LIFE-02 section of `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with command transcript fingerprints, actor-known candidate witnesses, hidden-truth exclusion evidence, provenance handling, and a local `pass` result. The report explicitly records that candidate-generation unit tests in `crates/tracewake-core/src/agent/generation.rs` were covered by the `0042ORDLIFCER-001` `cargo test --workspace --locked` baseline, while this ticket's targeted commands provide the integration evidence.

The evidence cites `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`, hidden-truth gate coverage, no-human decision trace coverage, generated sequence replay coverage, and acceptance-gate source-context coverage. It records that candidate generation has no standalone projection field; the certifying surface is typed candidate/decision trace output from actor-known/event inputs plus no-human/replay preservation, not string or golden-byte scans.

Verification:

- `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passed.
- `cargo test --locked -p tracewake-core --test acceptance_gates` — passed.
- `cargo test --locked -p tracewake-core --test generative_lock` — passed.
- `cargo test --locked -p tracewake-core --test no_human_capstone` — passed.
- `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passed.

No production or engine code changed. No remediation was needed or performed.
