# 0038SPICEREVE-007: SPINE-06 evidence — action proposal, validation, scheduling, event append/application, and feedback pipeline

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — fills the SPINE-06 section of the acceptance artifact from existing tests/fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-06 (spec §5) requires proof that every world-affecting action enters as an ordinary proposal with source context, passes shared validation, is scheduled only through permitted scheduler ordering, appends events, applies them through the kernel mutation capability, and returns actor-visible and debug-only feedback through the proper quarantine — where validation may accept/reject/mutate via typed events but may not propose fallback plans or actor-visible hidden facts. This is the largest seam (16 audited source modules + a broad fixture corpus). This ticket gathers the proposal→validation→schedule→append→apply→feedback traces for accepted and rejected actions, the hidden-truth-planning rejections, and the scheduler/no-direct-mutation guards, and writes the SPINE-06 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/actions/{proposal,pipeline,registry,report}.rs`, `actions/defs/{eat,sleep,work,wait}.rs`, `scheduler.rs`, `agent/{actor_known,transaction,decision,planner,trace,routine,methods}.rs`; the tests `crates/tracewake-core/tests/{spine_conformance,hidden_truth_gates,no_human_capstone}.rs`.
2. Spec §5 SPINE-06 names the positive corpus (`sleep_eat_work_001`, `ordinary_workday_001`, `wait_then_window_passive_charges_each_tick_once_001`, `work_block_failed_then_sleep_succeeds_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_completion_fails_when_actor_displaced_001`, `severe_safety_with_known_exit_produces_move_001`, `routine_blocked_diagnostic_001`, `method_fallback_requires_new_trace_or_stuck_001`) and the adversarial corpus (`no_hidden_truth_planning_001`, `prose_born_fact_rejected_001`, `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001`); all fixture modules exist under `crates/tracewake-content/src/fixtures/`.
3. Shared boundary under audit: the canonical action path `proposal → validation → scheduling → event append → event application → feedback`. The witness must trace, per representative fixture, the proposal ID, source, source holder-known context ID/hash/frontier, validation report ID, appended event IDs, actor-visible facts, debug-only facts, and trace diagnostics — for one accepted and one rejected action — and prove event append precedes application and that application mints the internal mutation capability rather than exposing mutation authority to proposal/TUI/content.
4. `INV-043` (action validation is ordinary-agent validation, no player-privilege branch), `INV-099` (truth may validate but may not plan), `INV-103` (the scheduler is not a cognition authority), `INV-104` (routines/needs do not dispatch primitive actions directly), and `INV-106` (validation failure feeds replanning without leakage) motivate this ticket: no direct state mutation, no hidden-truth planning, no validation fallback plan, no scheduler rewrite of actor reason, no TUI/debug bypass.
5. This ticket audits the proposal/validation/scheduler/event-application surfaces as an **evidence consumer**: it runs `spine_conformance`, `hidden_truth_gates`, `no_human_capstone`, `golden_scenarios`, and the content golden runner, then records witnesses; it weakens no enforcement. Adversarial cases (direct action-def invocation that mutates without `run_pipeline`; scheduler rewriting actor decisions after a transaction; validation proposing a fallback or actor-visible hidden fact) must fail closed or emit the expected diagnostic; any survivor is recorded `fail` with responsible layer and routed to remediation.

## Architecture Check

1. Auditing the full pipeline via accepted/rejected transaction traces + hidden-truth-planning negatives + the append-before-apply / mutation-capability witness proves the no-direct-dispatch and truth-firewall semantics end to end, not just that a happy path runs.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-099`/`INV-106` truth validates but does not plan; failure feeds replanning without leakage -> replay/golden-fixture check + manual review: rejected-action trace shows only modeled consequences, no validator-only fallback (`hidden_truth_gates`, `forbidden_provenance_input_fails_closed_001`).
2. `INV-103`/`INV-104` scheduler/needs are not cognition authority -> codebase grep-proof + replay check: `scheduler_cannot_rewrite_wait_reason_after_transaction_001` and the no-direct-dispatch negatives fail closed.
3. `INV-043` ordinary-agent validation, append-before-apply -> manual review + replay check: one accepted and one rejected transaction trace recorded with proposal/validation/event IDs; application mints the internal mutation capability.

## What to Change

### 1. Capture the SPINE-06 pipeline transaction witnesses

Record, per representative fixture, the proposal ID, source, source holder-known context ID/hash/frontier, validation report ID, appended event IDs, actor-visible facts, debug-only facts, and trace diagnostics; record a full transaction trace for one accepted and one rejected action; record the witness that event append precedes event application and that application mints the internal mutation capability (not exposed to proposal/TUI/content).

### 2. Record adversarial pipeline-bypass and hidden-truth rejections

For the adversarial corpus (hidden-truth planning, prose-born fact, forbidden provenance input, typed-unproven-fact audit, scheduler rewrite after transaction) and the direct-action-def-without-`run_pipeline` case, record the fail-closed witness or expected diagnostic and the named responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The full no-direct-dispatch mutation-capability closure across all callers (owned by `-009`, SPINE-08) — this ticket records the pipeline's own append-before-apply and one direct-dispatch negative, not the exhaustive bypass-closure corpus.
- Remediation of any pipeline bypass or hidden-truth planning found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` — pipeline seams map to named evidence.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — hidden-truth planning / prose-born / forbidden-provenance cases fail closed.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` and `--test golden_scenarios` — no-human ordinary-life pipeline runs end to end.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — positive + adversarial pipeline fixtures behave as specified.

### Invariants

1. Every accepted action traces proposal→validation→schedule→append→apply→feedback; event append precedes application and application mints the internal mutation capability (`INV-043`/`INV-099`).
2. No scheduler rewrites actor reason after a transaction and no validation proposes a fallback or actor-visible hidden fact (`INV-103`/`INV-104`/`INV-106`); a rejected action yields only modeled consequences.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs existing pipeline/hidden-truth/no-human/golden tests and the captured accepted+rejected transaction traces are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-core --test spine_conformance && cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test no_human_capstone && cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-18

Filled the SPINE-06 section of the acceptance report with action pipeline,
source-context, accepted/rejected transaction, scheduler handoff, hidden-truth
firewall, and golden fixture corpus evidence. Recorded representative accepted
and rejected transaction traces, append-before-apply evidence, and the
SPINE-06-scoped direct-dispatch/bypass witnesses while leaving exhaustive
mutation-bypass closure to SPINE-08.

Verification:

1. `cargo test --locked -p tracewake-core --test spine_conformance` passed.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` passed.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` passed.
4. `cargo test --locked -p tracewake-core --test golden_scenarios` passed.
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed.
