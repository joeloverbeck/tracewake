# 0044FIRPROCER-005: FIRST-PROOF-05 — event-sourced expectation contradiction and belief update

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-05 — the genuinely new cognition path — must be proven at `U`: a source-backed prior expectation and a legally obtained absence observation are compared; an actor-known expectation contradiction is formed; contradiction links are stable and private; any resulting belief update is event-sourced; and replay rebuilds the same result. Matching must be actor-relative and proposition-specific; the contradiction must link prior expectation belief, contradicting observation, expected/observed proposition, holder, source/cause, and detection time; duplicate processing must be deterministic and not create duplicate contradictions. This ticket records the FIRST-PROOF-05 positive contradiction witnesses, adversarial negatives, compile-fail link-forgery boundary, and replay rebuild/first-divergence into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/epistemics/{contradiction.rs, belief.rs, observation.rs, proposition.rs, projection.rs}`, `crates/tracewake-core/src/events/{envelope.rs, log.rs, apply.rs}`, `crates/tracewake-core/src/replay/{rebuild.rs, report.rs}`, and `crates/tracewake-core/src/checksum.rs` (all confirmed present this session). Fixture `expectation_contradiction_001` and the compile-fail boundary `tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/src/lib.rs` exist.
2. Spec §7 FIRST-PROOF-05 (seam, audited files, doctrine, positive/adversarial evidence, event/replay/projection evidence, exact commands, failure condition) governs this ticket; doctrine foundation `03`/`04`/`12`, architecture `02`/`03`/`06`, execution `02` gates `EVENT`/`ACTOR-KNOWN`/`MISSING-PROPERTY`/`REPLAY` and execution `09` bind it.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-05 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-05): `INV-009` (the contradiction and belief update use normal event application), `INV-018` (replay rebuilds the same result), `INV-025` (wrong/updated beliefs are first-class state), `INV-102` (contradiction links carry provenance). Restate before trusting the narrative: the contradiction is inferred from two typed premises, event-sourced, holder-scoped, and replayable.
5. This ticket audits/reads (does not modify) the event-application, holder-known projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; direct projection insertion or prose-authored contradiction must be impossible/rejected, and external code cannot forge or mutate contradiction links (compile-fail boundary). No nondeterminism is introduced; duplicate processing is deterministic.

## Architecture Check

1. Capturing the ordered segment from source expectation through check/observation to contradiction and belief update — with event IDs, causes, payload versions, holder, source refs, and projection records, then rebuilding from empty projections — is the only check that proves the contradiction is event-sourced rather than projection-inserted; spec §7 requires a perturbed log to produce a localized first divergence.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may permit unstable/forgeable links or direct projection mutation.

## Verification Layers

1. `INV-009`/`INV-018` event-sourced + replay -> replay/golden-fixture check (the contradiction and any updated/missing belief use normal event application; rebuild from empty projections matches physical, epistemic, agent, diagnostic, and checksum surfaces; a perturbed log yields a localized first divergence).
2. `INV-025`/`INV-102` first-class linked belief -> manual review + codebase grep-proof (links connect prior expectation, contradicting observation, expected/observed proposition, holder, source/cause, detection time; unrelated expectations remain unchanged; duplicate processing is deterministic).
3. `INV-024` holder-scoped privacy -> compile-fail + content-validation check (external code cannot forge/mutate contradiction links; direct projection insertion or prose-authored contradiction is rejected; the holder-known and debug projections render under their respective authority rules).

## What to Change

### 1. Record positive contradiction and belief-update witnesses

Run `expectation_contradiction_001` and the §7 positive path. Record that matching is actor-relative and proposition-specific; the contradiction links prior expectation belief, contradicting observation, expected proposition, observed proposition, holder, source/cause, and detection time; the contradiction and any updated/missing belief use normal event application; unrelated expectations remain unchanged; duplicate processing is deterministic and creates no duplicate contradictions; and the holder-known context and debug projection render the contradiction under their respective authority rules. Capture the ordered event segment with IDs, causes, payload versions, holder, source refs, and projection records.

### 2. Record adversarial negatives and replay rebuild

Record the §7 adversarial cases: wrong holder, wrong item, wrong container/place, stale/unrelated observation, present item, unsupported stance, absent provenance, or missing source event produces no accepted contradiction; external code cannot forge or mutate contradiction links (compile-fail); reordering an equivalent expectation collection cannot change semantic output; adding an unrelated expectation cannot change the target contradiction; direct projection insertion or prose-authored contradiction is impossible/rejected. Rebuild from empty projections and compare physical, epistemic, agent, diagnostic, and checksum surfaces; record the localized first divergence for a perturbed log. Each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-05 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any contradiction-path defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Expectation provenance (`-003`), absence observation (`-004`), no-culprit-truth (`-006`), composite replay (`-010`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` and `cargo test --locked -p tracewake-core --test hidden_truth_gates` pass; the contradiction is inferred from two typed premises, links all required fields, and is event-sourced and holder-scoped.
2. `cargo test --locked -p tracewake-core --test generative_lock` and `cargo test --locked -p tracewake-core --test negative_fixture_runner` pass; reordering/adding unrelated expectations cannot change semantic output, link forgery fails, and duplicate processing is deterministic.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; replay rebuilds physical, epistemic, agent, diagnostic, and checksum surfaces identically and a perturbed log yields a localized first divergence.

### Invariants

1. No contradiction inferred without both typed premises; stable, unforgeable, private links; no direct projection mutation; no cross-holder leakage.
2. Deterministic duplicate handling; replay equality with localized first-divergence on a controlled perturbation.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`

## Outcome

Completed: 2026-06-21

Updated
`reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`
to mark `FIRST-PROOF-05` passed for its event-sourced contradiction and belief
update scope. The artifact records positive witnesses for expectation ->
observation -> contradiction event application, stable holder-scoped
prior-expectation / contradicting-observation links, debug rendering,
projection checksums, replay equality, and localized divergence on tampered
logs. It also records adversarial evidence for wrong holder, wrong stance,
wrong target, present item, absent expectation, deterministic duplicate/order
behavior, and the compile-fail boundary preventing external mutation of
contradiction links.

No production code, fixtures, schemas, tests, or mutation configuration changed.
No audit-only instrumentation was needed.

Verification run:

- `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
- `cargo test --locked -p tracewake-core --test hidden_truth_gates`
- `cargo test --locked -p tracewake-core --test generative_lock`
- `cargo test --locked -p tracewake-core --test negative_fixture_runner`
- `cargo test --locked -p tracewake-content --test golden_fixtures_run`

All required commands passed. The artifact remains pending for no-culprit-truth,
truth-firewall integration, possession/no-human/replay/fixture-negative
capstones, full mutation execution, and the final FIRST-PROOF-CERT verdict.
