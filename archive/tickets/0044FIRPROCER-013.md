# 0044FIRPROCER-013: FIRST-PROOF-13 — routine temporal premises and adaptation in the integrated no-human run

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-13 certifies the **second routed temporal source** (execution `06`) at `U`: consuming the already-certified routine mechanism, its modeled temporal premises, waits, interruptions, contradictions, and adaptation must participate coherently in the first-proof corpus. A selected routine/method must identify its modeled temporal premise and provenance; an actor must be able to wait, retry, fall back, interrupt, or adapt after a modeled observation/notice/outcome/contradiction; duration and passive accounting must charge each tick exactly once across window boundaries and replay; and no-human progress/stuck classification must remain honest while the missing-property path is active. This ticket records the FIRST-PROOF-13 routine-temporal witnesses, true-time non-planning negatives, and accounting/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/{scheduler.rs, time.rs, need_accounting.rs}`, `crates/tracewake-core/src/agent/{routine.rs, intention.rs, planner.rs, trace.rs, need.rs}`, and the ordinary action defs under `crates/tracewake-core/src/actions/defs/` (all confirmed present this session). Fixtures `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `food_unavailable_replan_001`, and `routine_blocked_diagnostic_001` exist.
2. Spec §7 FIRST-PROOF-13 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; `INV-112`, execution `06` "Routine Temporal Premises and Adaptation Proof", and execution `03` temporal cascade bind it. This consumes the already-certified routine mechanism (`ORD-LIFE-CERT`, commit `c819bbe…`) as a participation relation.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-13 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket is temporal source `06` of the five-source bundle, consolidated by `0044FIRPROCER-016`.
4. Motivating invariants (spec §7 FIRST-PROOF-13): `INV-112` (holder-known time must plan), `INV-035` (routines are defeasible intentions, not teleports/puppet strings), `INV-104` (routines do not dispatch primitive actions directly). Restate before trusting the narrative: a source-backed premise produces legal ordinary-life behavior without direct dispatch or true-time fallback.
5. This ticket audits/reads (does not modify) the candidate-generation, scheduler, action-validation, event-application, projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; the scheduler cannot rewrite transaction-authored wait reason or create fallback cognition, and partitioning a time window or inserting an interruption cannot double-charge or omit accounting. No nondeterminism is introduced.

## Architecture Check

1. Packaging the routine selection/rejection trace + premise sources + wait/action events + interruption/adaptation events + accounting ledger + typed blocker + live/replay equality, related to the same integrated capstone as the contradiction, is the only check that proves the temporal premise drives behavior without true-time planning; spec §7 requires that duration/passive accounting charge each tick exactly once across window boundaries.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let true schedule alone select work/wake/sleep or fabricate progress.

## Verification Layers

1. `INV-112`/`INV-035` source-backed routine premise -> replay/golden-fixture check (the selected routine/method identifies its modeled temporal premise and provenance; an actor can wait/retry/fall-back/interrupt/adapt after a modeled observation/notice/outcome/contradiction).
2. `INV-112` no true-time planning -> codebase grep-proof + manual review (true schedule alone cannot select work or wake/sleep behavior; stale premises are not silently corrected; the scheduler cannot rewrite transaction-authored wait reason or create fallback cognition).
3. `INV-104` single-charge accounting + replay -> replay/golden-fixture check (duration and passive accounting charge each tick exactly once across window boundaries and replay; partitioning a window or inserting an interruption cannot double-charge or omit accounting; no-human progress/stuck stays honest while the missing-property path is active).

## What to Change

### 1. Record positive routine-temporal witnesses

Run `no_human_capstone`, `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `food_unavailable_replan_001`, `routine_blocked_diagnostic_001`, and the §7 positive path. Record that the selected routine/method identifies its modeled temporal premise and provenance; an actor can wait, retry, fall back, interrupt, or adapt after a modeled observation/notice/outcome/contradiction; duration and passive accounting charge each tick exactly once across window boundaries and replay; and no-human progress/stuck classification remains honest while the missing-property path is active. Relate the temporal witness to the same integrated capstone as the contradiction.

### 2. Record true-time non-planning and accounting negatives

Record the §7 adversarial cases: true schedule alone cannot select work or wake/sleep behavior; stale premises are not silently corrected; blocked routines cannot loop, teleport, or fabricate progress; the scheduler cannot rewrite transaction-authored wait reason or create fallback cognition; partitioning a time window or inserting an interruption cannot double-charge or omit accounting. Package the routine selection/rejection trace, premise sources, wait/action events, interruption/adaptation events, accounting ledger, typed blocker, and live/replay equality; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-13 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any routine-temporal defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Inventing any temporal unit/threshold/source category (spec §5.4); the temporal firewall (`-012`), embodied temporal rendering (`-014`), temporal fixture families (`-015`), the consolidated five-source line (`-016`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario/temporal tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test acceptance_gates` pass; the selected routine identifies its modeled temporal premise/provenance and adapts after a modeled observation/notice/outcome/contradiction.
2. `cargo test --locked -p tracewake-core --test generative_lock` and `--test event_schema_replay_gates` pass; true schedule alone cannot select work/wake/sleep, stale premises are not silently corrected, and accounting charges each tick exactly once across window boundaries and replay.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; blocked routines cannot loop/teleport/fabricate progress and the scheduler cannot rewrite transaction-authored wait reasons.

### Invariants

1. No true-time planning, no silent stale correction, no incorrect accounting, no untyped blocking, no direct dispatch, no nonreplayable adaptation.
2. Duration/passive accounting is single-charge across window boundaries and replay; no-human progress/stuck classification stays honest with the missing-property path active.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-13 in the shared acceptance artifact as passed for its
routine-temporal and accounting/replay scope. The evidence packet now includes
command-ledger rows, gate/scenario references, a FIRST-PROOF-13 audit section,
temporal source `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, and
two evidence ledger items: `E-0044-013-routine-temporal` and
`E-0044-013-accounting-negative`.

Verification run:

1. `cargo test --locked -p tracewake-core --test no_human_capstone` -> pass, 2 passed.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` -> pass, 12 passed.
3. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` -> pass, 32 passed.
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
