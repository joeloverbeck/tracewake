# 0040EPICERHOL-008: EPI-07 — actor decision transaction, proposal context parity, validation-truth firewall, and feedback split

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2.
**Deps**: archive/tickets/0040EPICERHOL-001.md

## Problem

Spec 0040 audit point EPI-07 (§5) requires certifying that the actor decision transaction consumes a sealed actor-known surface and emits a sealed proposal bound to the exact holder-known context ID/hash/frontier and typed provenance; that candidate generation, planning, and proposal construction use actor-known input only; that authoritative truth may validate or reject but may not originate a proposal, backfill knowledge, rewrite the actor's reason, or leak debug-only facts through feedback; and that a stale/mismatched proposal fails before accepted event append. This ticket collects the EPI-07 witnesses and writes the EPI-07 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. The EPI-07 seam files verified present at `ba9fe1c` (2026-06-19): `agent/transaction.rs` (`SealedProposal`, `ActorDecisionTransaction`), `agent/no_human_surface.rs` (`SealedActorKnownSurface`), `agent/actor_known.rs`, `agent/candidate.rs`, `agent/decision.rs`, `agent/planner.rs`, `agent/trace.rs`, `actions/proposal.rs` (`ProposalSourceContext`), `actions/pipeline.rs`, `actions/report.rs`, `events/envelope.rs`, `view_models.rs`. Fixtures `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `embodied_exits_require_perceived_or_known_route_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `forbidden_provenance_input_fails_closed_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` all verified present.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its EPI-07 section only. Section wording follows spec §5 EPI-07 and the `0003` evidence fields. The candidate/planner files are audited only at the EPI input/output boundary — planner quality, need policy, routine coverage, and ordinary-life completeness remain `ORD-LIFE-CERT` work (spec §5 EPI-07, §11).
3. Shared boundary under audit: the decision-transaction firewall — `SealedActorKnownSurface` → `ActorDecisionTransaction` → `SealedProposal` bound to the `ProposalSourceContext` tuple, validated by `pipeline.rs`, with the actor-safe/debug feedback split in `report.rs`/`trace.rs`. The evidence must, for a rejection, record exactly what authoritative fact validated the action and prove that fact did not become actor-known without a legal channel.
4. `INV-099` (truth may validate but may not plan) and `INV-101` (actor-known context is sealed) motivate this audit point, with `INV-100`/`INV-103`/`INV-104`/`INV-106` (no hidden-truth cognition, scheduler/routines not cognition authority, validation-failure feeds replanning without leakage), `INV-024`/`INV-026`/`INV-108`. Restated: a validation rejection must not add the rejecting hidden fact to the actor's next holder-known context unless a separate modeled observation/feedback event legally conveys it.
5. This ticket audits the truth-firewall / proposal-parity / no-leak surfaces as an **evidence consumer**: it proves paired worlds with identical actor-observable history but different hidden food/route/workplace/other-actor-belief/debug state yield equal candidate set, selected intention/method at the audited boundary, sealed proposal, and actor-safe reasons before observation (no leakage), and that tampering any parity-tuple field (context ID/hash/frontier, bound actor, source-event ancestry, proposal actor, semantic target, validation generation) is rejected before accepted state mutation/event append. It modifies no enforcement. Any failed seam is recorded `fail` and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A dedicated EPI-07 evidence ticket writing only its artifact section is the accepted certification-audit shape; `Large` because the decision transaction is the central firewall seam with the broadest paired-run and tamper matrix. It audits the candidate/planner files at the EPI boundary only, deferring planner-quality breadth to `ORD-LIFE-CERT`.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical live seam (spec §2); direct construction of a decision from `PhysicalState`/`ValidationReport`/debug report/TUI string must be absent or compile/runtime rejected.

## Verification Layers

1. `INV-101` proposal context parity -> replay/golden-fixture check: a successful proposal preserves source context ID/hash/frontier, source-event ancestry, semantic action, accepted validation stage, appended event, actor-visible outcome; replay reproduces the same source context, eligibility, validation result, and actor-safe feedback (`event_schema_replay_gates`, `golden_scenarios`).
2. `INV-099`/`INV-100` truth-firewall paired equality -> replay/golden-fixture check: `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `embodied_exits_require_perceived_or_known_route_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001` show actor-known candidate/proposal behavior under missing knowledge; paired hidden-difference worlds stay equal before observation (`hidden_truth_gates`, `no_human_capstone`).
3. `INV-106` feedback without leakage -> manual review + replay/golden-fixture check: `embodied_workplace_believed_open_truth_closed_commit_fails_001` shows a belief-grounded proposal authoritatively rejected without changing that it arose from the belief; the rejecting hidden fact does not enter the next context absent a legal channel.
4. tamper / contamination rejection -> manual review + negative fixture: tampering each parity-tuple field is rejected before event append; `forbidden_provenance_input_fails_closed_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` stay negative even when the contaminated fact is wrapped in a typed structure (`tracewake-tui` `adversarial_gates`).

## What to Change

### 1. Collect EPI-07 witnesses

Run the EPI-07 commands and package: the sealed actor-known input, candidate/decision trace at the EPI boundary, selected proposal, context-parity tuple, validation facts, actor-safe/debug feedback split, appended events, post-action context, and a paired-run equality/difference report. Replay each accepted and rejected event prefix from a clean projection. For a rejection, record exactly what authoritative fact validated the action and prove it did not become actor-known without a legal channel.

### 2. Write the EPI-07 section of the acceptance artifact

Populate the EPI-07 section with the §9.2 ledger fields per witness (positive, adversarial paired-run, tamper, replay/provenance, contamination controls), each row carrying exactly one §9.2 status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; EPI-07 section only)

## Out of Scope

- Other audit points and the §6.1 compile-fail corpus matrix (owned by their own tickets).
- Planner quality, need policy, routine coverage, ordinary-life completeness — `ORD-LIFE-CERT`, audited here only at the EPI input/output boundary (spec §5 EPI-07, §11).
- The §9.4 verdict table and aggregate verdict (owned by `-015`); the relational capstone matrix (owned by `-013`).
- Any production remediation of a failed seam — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `--test no_human_capstone` — paired truth-firewall equality + actor-known planning.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` and `--test event_schema_replay_gates` — proposal-parity + replay reproduction of accepted/rejected prefixes.
3. `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` — fixture witnesses.
4. `cargo test --locked -p tracewake-tui --test adversarial_gates` — typed-wrapped contamination stays rejected.

### Invariants

1. A sealed proposal binds the exact context ID/hash/frontier + typed provenance; tampering any parity field rejects before event append; truth validates/rejects but never originates the proposal (`INV-099`/`INV-101`).
2. A rejection's authoritative fact does not become actor-known absent a legal observation/feedback channel; paired hidden-difference worlds stay equal at candidate/proposal/actor-safe-reason before observation (`INV-106`/`INV-100`).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the EPI-07 existing gates/fixtures and records the sealed-input → decision-trace → proposal → validation → feedback-split chain, the paired-run report, and the tamper matrix as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test no_human_capstone && cargo test --locked -p tracewake-core --test event_schema_replay_gates`
3. `cargo test --locked -p tracewake-tui --test adversarial_gates` (typed-wrapped hidden-input rejection boundary)

## Outcome

Completed: 2026-06-19

Populated the EPI-07 section of the acceptance artifact with proposal-context parity, truth-firewall/no-leak adversarial, and replay/tamper evidence. The §9.4 EPI-07 row now cites `EPI07-POS-001`, `EPI07-ADV-001`, and `EPI07-REPLAY-001`, while its aggregate result remains pending for the mutation package and capstone verdict. No production or test code was changed.

Verification results:
- `cargo test --locked -p tracewake-core --test hidden_truth_gates`
- `cargo test --locked -p tracewake-core --test no_human_capstone`
- `cargo test --locked -p tracewake-core --test acceptance_gates`
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
- `cargo test --locked -p tracewake-core --test golden_scenarios`
- `cargo test --locked -p tracewake-content --test golden_fixtures_run`
- `cargo test --locked -p tracewake-tui --test adversarial_gates`

No deviations. Planner-quality and ordinary-life completeness remain outside this evidence-only EPI boundary.
