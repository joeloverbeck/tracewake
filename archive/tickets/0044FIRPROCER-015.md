# 0044FIRPROCER-015: FIRST-PROOF-15 — temporal positive/adversarial fixture families and replay pairing

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-15 certifies the **fourth routed temporal source** (execution `09`) at `U`: the temporal bundle must have paired positive and adversarial fixtures covering provenance, staleness, interruption, duration boundaries, waiting, replay, and anti-contamination. Merely listing fixtures does not satisfy this point; each must be registry-reachable and behaviorally exercised. This ticket records the FIRST-PROOF-15 paired temporal fixture families (current/stale/superseding premise, duration-boundary single-charge, modeled wait + adaptation, per-scenario replay), the adversarial anti-contamination family, and an explicit exhaustive-vs-sampled coverage statement into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: the fixture registry and runner (`crates/tracewake-content/src/fixtures/mod.rs`, content `golden_fixtures_run.rs`/`fixtures_load.rs`/`forbidden_content.rs`) and the temporal fixtures `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `routine_blocked_diagnostic_001`, plus the temporal-related hidden-truth/TUI fixtures (all confirmed present this session).
2. Spec §7 FIRST-PROOF-15 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; execution `09` temporal fixture families and replay acceptance, execution `03` temporal cascade, and `INV-112` bind it.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-15 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket is temporal source `09` of the five-source bundle, consolidated by `0044FIRPROCER-016`.
4. Motivating invariants (spec §7 FIRST-PROOF-15): `INV-112` (holder-known temporal premise with provenance), `INV-020` (replay rejects unsupported/tampered history rather than inventing repairs). Restate before trusting the narrative: each temporal fixture is registry-reachable and behaviorally exercised, with positive/adversarial pairing — a fixture that loads but does not exercise the intended path is not certifying.
5. This ticket audits/reads (does not modify) the content-schema/validation, fixture-registry, holder-known-context, projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; it records whether coverage is exhaustive over the declared finite registry or sampled, and a tampered event order/payload must report a first divergence. No nondeterminism is introduced.

## Architecture Check

1. Recording, for every temporal fixture, registry ID + semantic fingerprint + trigger + source provenance + event sequence + projection fields + diagnostics + live/replay checksums + positive/adversarial pairing, plus a stated exhaustive-or-sampled scope, is the only check that proves branch reachability rather than mere file presence; spec §7 states listing fixtures does not satisfy this point.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let a temporal fact without source, or a marker-only wait/progress, pass.

## Verification Layers

1. `INV-112` paired temporal families -> replay/golden-fixture check (source-backed current premise; source-backed stale premise handled as stale not truth-corrected; superseding newer modeled notice/observation; duration crossing a window boundary with single charge; modeled wait and later adaptation).
2. `INV-020` replay pairing + tamper localization -> replay/golden-fixture check (replay of each temporal scenario with identical projections/diagnostics; tampered event order/payload reports a first divergence).
3. Anti-contamination family -> content-validation + adversarial check (raw clock, debug timestamp, omniscient due/closed state, queue position, replay index, renamed metadata, nested wrapper, generated prose, and source-restamp attempts reject; a temporal fact without source rejects; changed true schedule with unchanged holder-known inputs leaves output unchanged; marker-only wait/progress excluded).

## What to Change

### 1. Record positive paired temporal fixture families

Run the temporal fixtures through the registry and runner. For every temporal fixture, record registry ID, semantic fingerprint, trigger, source provenance, event sequence, projection fields, diagnostics, and live/replay checksums, with positive/adversarial pairing. Cover: source-backed current premise; source-backed stale premise rendered/handled as stale rather than truth-corrected; superseding newer modeled notice/observation; duration crossing a window boundary with single charge; modeled wait and later adaptation; and replay of each temporal scenario with identical projections/diagnostics. State whether coverage is exhaustive over the declared finite registry or sampled.

### 2. Record adversarial anti-contamination family

Record the §7 adversarial cases: raw clock, debug timestamp, omniscient due/closed state, queue position, replay index, renamed metadata, nested wrapper, generated prose, and source restamp attempts; a temporal fact without source; a changed true schedule with unchanged holder-known inputs; tampered event order/payload with a first-divergence report; marker-only wait/progress. Confirm each named fixture is registry-reachable and behaviorally exercised (not merely loaded); each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-15 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any temporal-fixture defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Inventing any temporal unit/threshold/source category (spec §5.4); the temporal firewall (`-012`), routine temporal premises (`-013`), embodied temporal rendering (`-014`), the consolidated five-source line (`-016`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario/temporal tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test fixtures_load` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; every named temporal fixture is registry-reachable and behaviorally exercised with positive/adversarial pairing.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `--test generative_lock`, and `--test no_human_capstone` pass; duration-boundary single-charge, modeled wait/adaptation, and per-scenario replay reproduce identical projections/diagnostics.
3. `cargo test --locked -p tracewake-content --test forbidden_content` and `cargo test --locked -p tracewake-tui --test transcript_snapshot` pass; raw-clock/debug/queue/replay-index/renamed/nested/generated/restamp attempts reject and tampered order/payload reports a first divergence.

### Invariants

1. No unreached fixture, no unpaired temporal path, no source-free temporal claim, no dishonest fingerprint, no replay drift, no omitted adversarial family.
2. Coverage is explicitly stated as exhaustive over the declared finite registry or sampled; marker-only wait/progress is excluded.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test generative_lock`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --locked -p tracewake-content --test fixtures_load`
5. `cargo test --locked -p tracewake-content --test forbidden_content`
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
7. `cargo test --locked -p tracewake-tui --test transcript_snapshot`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-15 in the shared acceptance artifact as passed for its
temporal positive/adversarial fixture families and replay pairing scope. The
evidence packet now includes command-ledger rows, gate/scenario references, a
FIRST-PROOF-15 audit section, temporal source
`09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`, and two evidence
ledger items: `E-0044-015-temporal-fixture-pairing` and
`E-0044-015-temporal-anti-contamination`. Coverage is recorded as exhaustive
over the declared finite fixture registry exercised by the named commands, with
generated/metamorphic checks sampled by `generative_lock`.

Verification run:

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` -> pass, 32 passed.
2. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` -> pass, 2 passed.
4. `cargo test --locked -p tracewake-content --test fixtures_load` -> pass, 34 passed.
5. `cargo test --locked -p tracewake-content --test forbidden_content` -> pass, 24 passed.
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
7. `cargo test --locked -p tracewake-tui --test transcript_snapshot` -> pass, 3 passed.
