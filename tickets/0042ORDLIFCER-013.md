# 0042ORDLIFCER-013: ORD-LIFE-12 — deterministic replay-derived ordinary-life projections, metrics, diagnostics, and phase-entry lock

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-12 must be proven: accepted ordinary-life events are the durable authority; a clean rebuild must deterministically reproduce need state/thresholds, intentions and lifecycle ancestry, routine execution, decision/planner traces, stuck diagnostics, duration/open-terminal state, no-human metrics, and decision-context hashes; projections and reports are consumers, never truth writers; and the acceptance package must keep Phase 4 blocked unless all twelve audit points, the mutation floor, and predecessor-gate evidence satisfy the aggregate rule. This ticket records the per-event replay-handling census, live/replay fingerprints, first-divergence reports, and the phase-entry/admissibility evidence into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/events/{envelope,log,apply,mutation}.rs`, `replay/{rebuild,report}.rs` (`rebuild_projection`, `run_replay`, `rebuild_decision_context_hashes`), `projections.rs`, `checksum.rs`, `state.rs`, `scheduler.rs`; the guard test binaries `event_schema_replay_gates`, `acceptance_artifact_wording`, `acceptance_gates`, `ci_workflow_guards`, `doc_invariant_references`; and the doc surfaces `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, `docs/4-specs/SPEC_LEDGER.md` (the last three test binaries were §2.2 reassessment-confirmed-present; the phase-ladder doc and ledger were validated during reassessment).
2. Spec §5 ORD-LIFE-12 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 1/8/9/10 govern this ticket; §4.4 "replay derived, not projection authored" + "no evidence laundering" bind it. The phase-ladder order `… EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT → PHASE-4-ENTRY` (docs/2-execution/03) keeps Phase 4 blocked behind this gate.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-12 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section. The `docs/4-specs/SPEC_LEDGER.md` row and `archive/specs/` move are deferred to spec acceptance (read here for the phase-entry-lock check, not modified).
4. Motivating invariants (spec §5 ORD-LIFE-12): `INV-018` (deterministic replay is foundational), `INV-009` (eventful with cause), `INV-091`/`INV-092` (accounting derivation), `INV-041`, `INV-105`. Restate before trusting the narrative: a matching checksum is insufficient unless a clean rebuild from the accepted log reproduces the ordinary-life projection/metrics without direct writes; Phase 4 stays blocked until the aggregate rule holds.
5. Evidence-consumer surface (audit-reads, does not modify): the replay/rebuild path, the semantic-checksum surface, and the CI/doc phase-entry guards. This ticket runs rebuilds and the guard tests and records fingerprints; it adds no truth-writer path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. A per-event replay-handling census plus byte-vs-semantic fingerprint discipline (hashes named for their actual scope; raw-byte equality cannot substitute for semantic behavior and vice versa) is the decisive replay-derivation proof — spec §5 states a matching checksum alone is insufficient without an empty-projection rebuild.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No projection/metric direct-insertion or post-replay reconciliation path may exist.

## Verification Layers

1. `INV-018` deterministic replay -> replay/golden-fixture check (rebuild from empty projection for all seven fixture families; replay twice from the same log/environment yields identical event handling, context-hash rebuild, diagnostic ordering, metric totals, serialized output).
2. `INV-009`/`INV-091` derived-not-authored -> codebase grep-proof + manual review (direct projection/metric/diagnostic insertion or post-replay reconciliation is exposed by compile/runtime/behavior guards; per-event replay-handling census covers the full ordinary-life event family).
3. Phase-entry lock -> manual review + codebase grep-proof (`acceptance_artifact_wording`, `ci_workflow_guards`, `doc_invariant_references`: ledger/acceptance wording and CI/doc guards do not declare or permit `PHASE-4-ENTRY` from historical/pending/sampled/observer-only/partial evidence).

## What to Change

### 1. Record replay-derivation and determinism witnesses

For the integrated no-human, food-unavailable, no-teleport, possession, hidden-truth, planner-trace, and routine-blocker families, rebuild from an empty projection and compare live/replay semantic fingerprints for every ordinary-life surface in the seam definition. Run replay twice from the same log/environment and record deterministic event handling, context-hash rebuild, diagnostic ordering, metric totals, and serialized semantic output. Produce a per-event replay-handling census covering the ordinary-life event family (no-human markers, need/threshold, intention/routine, continuation, sleep/work duration, decision trace, stuck diagnostic, replay marker). Record exact log bytes + parsed semantic scope, replay command/transcript, applied event range, context-hash comparison, and the aggregate acceptance table inputs.

### 2. Record adversarial replay and phase-entry negatives

Record the §5 adversarial cases: delete/duplicate/reorder/alter-schema-handling/mutate one ordinary-life event payload at a time (rebuild diverges at the first affected event or rejects loudly, not converging by normalization); direct projection/metric/diagnostic insertion or post-replay reconciliation (compile/runtime/behavior guards expose the truth-writer path); semantically-irrelevant map/set insertion-order change (canonical fingerprints equal) versus any semantic field change (fingerprint or failure changes); and acceptance wording promoting historical/pending/sampled/observer-only evidence, skipping the mutation floor, or claiming latest main (artifact-wording + review reject it). Record the first-divergence report and the phase-entry/admissibility evidence keeping Phase 4 blocked.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-12 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any replay/derivation/phase-lock defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- The `docs/4-specs/SPEC_LEDGER.md` row and `archive/specs/` move (deferred to spec acceptance/archival; read here only for the phase-entry-lock check).
- The §7 mutation floor itself (`-015`) and the aggregate verdict (`-016`) — this ticket supplies replay/phase-lock inputs the capstone consumes.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes; an empty-projection rebuild for all seven fixture families reproduces live/replay semantic fingerprints, and two replays from the same log/environment are identical.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; deleting/duplicating/reordering one event payload diverges at the first affected event or rejects loudly, with a recorded first-divergence report.
3. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`, `cargo test --locked -p tracewake-core --test ci_workflow_guards`, and `cargo test --locked -p tracewake-core --test doc_invariant_references` pass; ledger/acceptance wording and CI/doc guards do not permit `PHASE-4-ENTRY` from historical/pending/sampled/observer-only/partial evidence.

### Invariants

1. Replay derived, not authored: a clean empty-projection rebuild reproduces every ordinary-life projection/metric/diagnostic without direct writes; matching checksums alone do not certify.
2. Phase-entry lock: Phase 4 remains blocked unless all twelve audit points, the mutation floor, and predecessor-gate evidence satisfy the aggregate rule; no historical/pending/sampled/observer-only promotion.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
5. `cargo test --locked -p tracewake-core --test doc_invariant_references`
6. `cargo test --locked -p tracewake-core --test no_human_capstone`
7. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
