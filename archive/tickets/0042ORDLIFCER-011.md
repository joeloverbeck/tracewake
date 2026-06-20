# 0042ORDLIFCER-011: ORD-LIFE-10 — typed stuck diagnostics, blocker taxonomy, and cross-tick no-progress detection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-10 must be proven: an actor that cannot progress must not starve silently, loop on a routine marker, or wait forever without cause; the system must distinguish a legitimate modeled wait from a stuck loop and emit typed diagnostics for at least `no-progress-past-expected-window` and `repeated-idle`, including actor, routine/window, blocker code, responsible layer, resulting intention/routine status, and supporting event ancestry. This ticket records the tick-by-tick progress timelines, typed diagnostic fields, detector negatives, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/trace.rs` (`StuckDiagnostic`, `StuckResultingStatus`, `TypedDiagnosticFields`, `BlockerCategory`, `BlockerCode`, `ResponsibleLayer`), `agent/routine.rs`, `agent/planner.rs`, `agent/transaction.rs`, `scheduler.rs`, `projections.rs`, `events/{envelope,apply}.rs`, `replay/{rebuild,report}.rs` (confirmed in the 0042 reassessment census; `ResponsibleLayer` resolves in `events/apply.rs`).
2. Spec §5 ORD-LIFE-10 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 1/6/8/9 govern this ticket; §4.4 "no silent liveness assumption" binds it. Spec §10 risk: the code-level `ResponsibleLayer` representation is narrower than the full execution-10 review vocabulary — the artifact must prove event-append/application, replay, fixture, and documentation failures can still be attributed without collapsing distinct layers.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-10 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-10): `INV-070` (typed diagnostics / observability boundary), `INV-015` (no silent starvation), `INV-041`, `INV-091`, `INV-105`/`INV-106`. Restate before trusting the narrative: a modeled wait and a stuck loop are distinct; `no-progress-past-expected-window` and `repeated-idle` emit exactly one typed diagnostic per canonical episode with full typed fields.
5. Evidence-consumer surface (audit-reads, does not modify): the stuck-detector / diagnostic-emission path and its replay rebuild. This ticket runs blocker/idle fixtures and records detector state; it adds no diagnostic-writing path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Recording a tick-by-tick progress timeline with the routine window, expected boundary, wait reasons, and detector state — and proving each wait is *either* modeled *or* stuck (never "actor waited") — is the decisive liveness proof; forcing repeated-idle, a single legitimate wait, and intermittent genuine progress to not collapse into one outcome closes the detector against false positives/negatives.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-070` typed diagnostics -> codebase grep-proof + manual review (`routine_blocked_diagnostic_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `food_unavailable_replan_001`: each diagnostic carries actor, routine/window, blocker code, responsible layer, resulting status, source events; omitting any → evidence-oracle/schema rejection).
2. `INV-015` no silent starvation -> replay/golden-fixture check (`no-progress-past-expected-window` and `repeated-idle` cross their declared window boundary and emit exactly one typed diagnostic per canonical episode rather than silently looping or flooding duplicates).
3. `INV-018`/`INV-105` diagnostic replay -> replay/golden-fixture check (`scheduler_cannot_rewrite_wait_reason_after_transaction_001`: scheduler post-processing may not replace the transaction's reason/layer; clean replay reconstructs diagnostics, blocker code, layer, status, source events, metric classification).

## What to Change

### 1. Record positive stuck/wait classification witnesses

Run `routine_blocked_diagnostic_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `food_unavailable_replan_001`, and planner-budget/fallback cases to cover missing knowledge, stale knowledge, budget exhaustion, blocked affordance, and validation failure. Produce a modeled wait with a source-backed reason and expected review boundary; prove it is not prematurely classified as stuck. Produce `no-progress-past-expected-window` and `repeated-idle` sequences; prove each crosses its declared boundary and emits exactly one typed diagnostic per canonical episode. Record the tick-by-tick progress timeline, typed diagnostic fields, source events, resulting status, metric row, and live/replay equality, explaining why each wait is modeled or stuck.

### 2. Record adversarial detector and attribution negatives

Record the §5 adversarial cases: `scheduler_cannot_rewrite_wait_reason_after_transaction_001` (scheduler may not replace the actor transaction's reason/layer); omitting blocker/responsible-layer/actor-window-identity/expected-boundary/source-events (evidence-oracle + schema reject the incomplete diagnostic); changing only debug text/display prose (typed semantics + replay unchanged) versus changing blocker/layer/status (semantic evidence must change even if displayed text is similar); and forcing repeated idle across adjacent windows, a single legitimate wait, and intermittent genuine progress (detector keeps all three distinct). Record how event-append/application, replay, fixture, and documentation failures are attributed despite the narrower code-level `ResponsibleLayer`.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-10 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any detector/diagnostic defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- No-human metric honesty certification (`-010`), scheduler no-direct-dispatch (`-012`), and the aggregate verdict / fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test generative_lock` pass; `no-progress-past-expected-window` and `repeated-idle` each emit exactly one typed diagnostic per canonical episode with actor, routine/window, blocker code, responsible layer, resulting status, and source events.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` passes; `scheduler_cannot_rewrite_wait_reason_after_transaction_001` shows scheduler post-processing cannot replace the transaction's reason/layer, and an incomplete diagnostic is rejected.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; clean replay reconstructs diagnostics, blocker code, layer, status, source events, and metric classification, and a modeled wait is not misclassified as stuck.

### Invariants

1. No silent liveness assumption: every wait identifies its modeled reason and expected boundary; no-progress-past-window and repeated-idle become typed stuck outcomes, not excused by implicit fairness.
2. Typed-field completeness + attribution: each diagnostic carries blocker code, responsible layer, actor/window, expected boundary, and source events; debug prose changes do not change semantics, and semantic changes do change evidence.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-20

Recorded ORD-LIFE-10 as a certifying pass in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with evidence rows `0042-ORD10-TYPED-DIAGNOSTICS`, `0042-ORD10-LIVENESS-DETECTOR`, and `0042-ORD10-REPLAY-ATTRIBUTION`.

The evidence records typed blocker/layer/status diagnostics, distinct no-progress-past-window and repeated-idle detection, scheduler wait-reason preservation, and replayed metric classification for typed stuck diagnostics. The narrower code-level `ResponsibleLayer` vocabulary is recorded in the report evidence and broader review layers remain represented in report responsible-layer fields. No ORD-LIFE-10 member was deferred or dropped.

Verification commands run and passed:

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
