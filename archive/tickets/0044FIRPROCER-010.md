# 0044FIRPROCER-010: FIRST-PROOF-10 — composite replay, projection rebuild, deterministic diagnostics, and divergence localization

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-10 consumes `SPINE-CERT` internals but certifies the larger corpus at `U`: every first-proof scenario and the integrated capstone rebuild physical, epistemic, agent, embodied-support, and diagnostic state from fixture plus ordered events. Each event must have a stable ID, causal ancestry, schema/version, deterministic order, and a replay effect or explicit non-world diagnostic role; replay must start from the declared fixture/content/ruleset version and empty projections; serialized logs and reports must be deterministic for identical inputs; and the combined corpus must be replayable scenario-by-scenario and as the designated integrated capstone. This ticket records the FIRST-PROOF-10 per-scenario replay package, deterministic-diagnostics witnesses, and controlled-tamper first-divergence into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/events/{envelope.rs, log.rs, apply.rs, mutation.rs}`, `crates/tracewake-core/src/replay/{rebuild.rs, report.rs}`, `crates/tracewake-core/src/{checksum.rs, projections.rs, need_accounting.rs, debug_reports.rs}`, `crates/tracewake-core/src/epistemics/projection.rs`, and the suites `event_schema_replay_gates.rs`/`generative_lock.rs`/`golden_scenarios.rs` plus content `golden_fixtures_run.rs` (all confirmed present this session).
2. Spec §7 FIRST-PROOF-10 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `03`, architecture `02`, execution `02` gates `EVENT`/`REPLAY` and execution `09`/`10` bind it. This consumes `SPINE-CERT` (commit `92ba47f1…`) internals as a participation relation over the larger corpus.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-10 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-10): `INV-018` (deterministic replay is foundational), `INV-020` (event schema evolution — replay rejects unsupported history rather than inventing repairs), `INV-019` (snapshots/compaction preserve live ancestry), `INV-009` (every meaningful change is event-backed). Restate before trusting the narrative: identical inputs + versions produce byte-identical output and a controlled perturbation localizes the first divergence.
5. This ticket audits/reads (does not modify) the event-append, event-application, projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; replay cannot silently skip an event kind or regenerate actor-known facts from current truth, debug rows do not alter authoritative checksums, and equal final physical state with different causal/epistemic history is not misreported as equal evidence. No nondeterminism is introduced.

## Architecture Check

1. Recording, for every scenario, fixture fingerprint + event-kind census + root causes + live/replay checksums by projection + diagnostic fingerprint + serialized-log hash + a controlled first-divergence, with reported comparator semantics, is the only check that proves deterministic rebuild rather than coincidental final-state equality; spec §7 states "asserted unequal" is insufficient.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let replay skip an event kind or repair from current truth.

## Verification Layers

1. `INV-018`/`INV-009` deterministic event-sourced replay -> replay/golden-fixture check (each event has stable ID, causal ancestry, schema/version, deterministic order, and replay effect or explicit non-world diagnostic role; physical, epistemic, agent/routine/accounting, diagnostic, and view-support projections match live state).
2. `INV-020` schema rejection -> replay/golden-fixture check (unknown/duplicate/wrong-version event, broken cause, reordered events, omitted contradiction event, or changed payload rejects or localizes the first mismatch).
3. `INV-019` ancestry-preserving partitioning -> replay/golden-fixture + manual review (prefix/suffix and checkpoint partitioning preserve the same final result when semantically equivalent; equal final physical state with different causal/epistemic history is not misreported as equal; debug rows do not alter authoritative checksum).

## What to Change

### 1. Record per-scenario replay and deterministic-diagnostics witnesses

Run the §7 positive path across the combined corpus. For every scenario record fixture fingerprint, event count, event-kind census, root causes, live checksums by projection, replay checksums by projection, diagnostic fingerprint, and serialized-log hash; record that replay starts from the declared fixture/content/ruleset version and empty projections; that physical, epistemic, agent/routine/accounting, diagnostic, and view-support projections match live state; that serialized logs and reports are deterministic for identical inputs; and that the corpus replays scenario-by-scenario and as the integrated capstone. Report the exact comparator semantics.

### 2. Record controlled-tamper and replay-integrity negatives

Record the §7 adversarial cases: unknown/duplicate/wrong-version event, broken cause, reordered events, omitted contradiction event, changed payload, or changed fixture fingerprint rejects or localizes the first mismatch; replay cannot silently skip an event kind or regenerate actor-known facts from current truth; debug rows do not alter authoritative checksum; equal final physical state with different causal/epistemic history is not misreported as equal evidence; prefix/suffix and checkpoint partitioning preserve the same final result. Record the first divergence for at least one controlled perturbation; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-10 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any replay/determinism defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Re-auditing `SPINE-CERT` internals (consumed within scope, spec §4); the contradiction path (`-005`), temporal replay pairing (`-015`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-core --test generative_lock` pass; each event has stable ID, causal ancestry, schema/version, deterministic order, and a replay effect or explicit diagnostic role.
2. `cargo test --locked -p tracewake-core --test golden_scenarios`, `--test no_human_capstone`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; every projection class matches live state and the corpus replays scenario-by-scenario and as the capstone.
3. `cargo test --locked -p tracewake-tui --test transcript_snapshot` passes; a controlled tamper (unknown/duplicate/wrong-version/reorder/omit/changed-payload) rejects or localizes the first divergence with reported comparator semantics.

### Invariants

1. No missing event totality, no nondeterministic output, no projection mismatch, no nonlocalized divergence, no truth-derived replay repair, no diagnostic drift.
2. Debug rows do not alter authoritative checksums; equal final physical state with different causal/epistemic history is reported as unequal evidence.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test generative_lock`
3. `cargo test --locked -p tracewake-core --test golden_scenarios`
4. `cargo test --locked -p tracewake-core --test no_human_capstone`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
6. `cargo test --locked -p tracewake-tui --test transcript_snapshot`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-10 in the shared acceptance artifact as passed for its
composite replay, deterministic diagnostics, and controlled-divergence scope.
The evidence packet now includes command-ledger rows, gate/scenario references,
a FIRST-PROOF-10 audit section, and two evidence ledger items:
`E-0044-010-composite-replay` and `E-0044-010-controlled-divergence`.

Verification run:

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` -> pass, 32 passed.
2. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
3. `cargo test --locked -p tracewake-core --test golden_scenarios` -> pass, 16 passed.
4. `cargo test --locked -p tracewake-core --test no_human_capstone` -> pass, 2 passed.
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
6. `cargo test --locked -p tracewake-tui --test transcript_snapshot` -> pass, 3 passed.
