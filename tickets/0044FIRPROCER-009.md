# 0044FIRPROCER-009: FIRST-PROOF-09 — no-human ordinary-life and no-direct-dispatch participation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-09 consumes `ORD-LIFE-CERT` and the pipeline portion of `SPINE-CERT`, then proves at `U` that a boring ordinary-life interval and the missing-property situation coexist: the simulation advances without knowing whether a human is present, through actor transactions and the ordinary action pipeline. Multiple actors must sleep/eat/work/wait/search/check/become-blocked through normal transactions without human input; progress must be an accepted ordinary action, modeled wait, or typed stuck/failure outcome, not a marker; liveness evidence must distinguish a modeled wait from silent non-progress. This ticket records the FIRST-PROOF-09 integrated no-human witnesses, no-direct-dispatch negatives, and need-accounting/projection/replay reconciliation into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{scheduler.rs, time.rs, need_accounting.rs}`, `crates/tracewake-core/src/agent/{no_human_surface.rs, need.rs, routine.rs, intention.rs, planner.rs, trace.rs}`, `crates/tracewake-core/src/actions/pipeline.rs`, the ordinary action defs under `crates/tracewake-core/src/actions/defs/`, and `crates/tracewake-core/tests/no_human_capstone.rs` (all confirmed present this session). Fixtures `no_human_day_001`, `no_human_advance_001`, `food_unavailable_replan_001`, `routine_blocked_diagnostic_001`, `routine_no_teleport_001`, and the canonical `expectation_contradiction_001` exist.
2. Spec §7 FIRST-PROOF-09 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `05`/`06`/`12`/`14`, architecture `04`/`05`/`09`, execution `02` gate `NO-HUMAN-ORDINARY-LIFE` and execution `05`/`06` bind it. This consumes `ORD-LIFE-CERT` (commit `c819bbe…`) and the pipeline portion of `SPINE-CERT` (commit `92ba47f1…`) as participation relations.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-09 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-09): `INV-004` (the authoritative world ignores human existence), `INV-103` (the scheduler is not a cognition authority), `INV-104` (routines/needs do not dispatch primitive actions directly), `INV-091` (no-human tests are mandatory). Restate before trusting the narrative: progress is a real action/wait/stuck path, never a marker, and the scheduler authors no state deltas.
5. This ticket audits/reads (does not modify) the candidate-generation, scheduler, action-validation, event-application, and projection enforcement surfaces. It runs fixtures and records witnesses only; the scheduler must not dispatch a primitive action or author state deltas from need/routine labels, and liveness evidence must distinguish a modeled wait from a hung runner (supervisor metadata). No nondeterminism is introduced.

## Architecture Check

1. Packaging an integrated no-human log segment that contains both ordinary-life and missing-property events, reconciled against need accounting / intentions / routines / waits / blockers / progress classification / projections / replay, is the only check that proves coexistence without a scripted scene; spec §7 requires that liveness evidence distinguish a modeled wait from silent non-progress.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let the scheduler author deltas or count `continue_routine`/debug markers as progress.

## Verification Layers

1. `INV-004`/`INV-091` no-human runnable -> replay/golden-fixture check (multiple actors sleep/eat/work/wait/search/check/become-blocked through normal transactions without human input; assigning possession to any actor or none yields the same authoritative ordinary-life rules).
2. `INV-103`/`INV-104` scheduler not cognition / no direct dispatch -> codebase grep-proof + manual review (the scheduler cannot dispatch a primitive action or author state deltas from need/routine labels; action proposals/events have normal causal ancestry).
3. Honest progress/liveness -> replay/golden-fixture + supervisor-metadata check (progress is an accepted ordinary action, modeled wait, or typed stuck/failure; no-human mode cannot teleport, silently starve, spin, rewrite wait reasons, or count `continue_routine`/debug markers as progress; a missing-property contradiction can occur alongside routines without a scripted scene).

## What to Change

### 1. Record positive integrated no-human witnesses

Run `no_human_capstone`, `no_human_day_001`, `no_human_advance_001`, `food_unavailable_replan_001`, and the canonical contradiction fixture together. Record that multiple actors can sleep/eat/work/wait/search/check/become-blocked through normal transactions without human input; progress is an accepted ordinary action, modeled wait, or typed stuck/failure outcome, not a marker; a missing-property contradiction can occur during or alongside ordinary routines without turning the world into a scripted scene; action proposals and events have normal causal ancestry; and liveness evidence distinguishes a modeled wait from silent non-progress. Package the integrated log segment and reconcile need accounting, intentions, routines, waits, blockers, progress classification, physical/epistemic projections, and replay diagnostics.

### 2. Record no-direct-dispatch and no-marker negatives

Record the §7 adversarial cases: the scheduler cannot dispatch a primitive action or author state deltas from need/routine labels; hidden food/property/route truth cannot become a target; no-human mode cannot teleport, silently starve, spin in a loop, rewrite transaction-authored wait reasons, or count `continue_routine`/debug markers as progress; assigning possession to any actor or none produces the same authoritative ordinary-life rules; blocking yields typed responsible-layer diagnostics and legal fallback/wait/stuck behavior. Each negative names the canonical responsible layer (spec §11 vocabulary); the supervisor distinguishes timeout from modeled wait.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-09 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any no-human/dispatch defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Re-auditing `ORD-LIFE-CERT`/`SPINE-CERT` internals (consumed within scope, spec §4); routine temporal premises (`-013`) and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test acceptance_gates` pass; multiple actors advance through real action/wait/stuck paths without human input and a contradiction coexists with ordinary routines.
2. `cargo test --locked -p tracewake-core --test spine_conformance` and `--test event_schema_replay_gates` pass; the scheduler authors no state deltas/primitive dispatch and action proposals/events have normal causal ancestry.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test tui_acceptance` pass; markers cannot count as progress, blocking yields typed diagnostics, and possession of any/no actor yields the same ordinary-life rules.

### Invariants

1. No human special case, no direct dispatch, no scheduler state mutation, no marker-only progress, no silent stall, no hidden-target selection, no teleport.
2. Live/replay need-accounting and projection equality; supervisor metadata distinguishes a modeled wait from a hung runner.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test spine_conformance`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
6. `cargo test --locked -p tracewake-tui --test tui_acceptance`
