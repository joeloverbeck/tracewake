# 0044FIRPROCER-007: FIRST-PROOF-07 — truth-firewall and actor-known participation across the combined corpus

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-07 consumes `EPI-CERT` and re-proves its boundary as an integration relation at `U`: actor decisions and contradiction formation use sealed actor-known inputs; validation truth only accepts/rejects; changing unobserved truth does not change cognition until a modeled information event occurs. Transaction inputs must enumerate actor-known facts and source refs; candidate/method/plan/proposal outputs must cite the context/source frontier they consumed; validation may reject stale or impossible proposals without proposing a fallback or revealing actor-illegible reasons; and a modeled observation/notice may legally change a later decision. This ticket records the FIRST-PROOF-07 paired-run non-interference witnesses, fail-closed negatives, and debug-on/off equality into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/agent/{actor_known.rs, transaction.rs, decision.rs, candidate.rs, generation.rs, htn.rs, methods.rs, planner.rs, trace.rs}`, `crates/tracewake-core/src/epistemics/{knowledge_context.rs, knowledge_basis.rs}`, `crates/tracewake-core/src/actions/{proposal.rs, pipeline.rs}`, and `crates/tracewake-core/tests/hidden_truth_gates.rs` (all confirmed present this session). The hidden-food/route/workplace/provenance fixtures referenced by §7 exist in the fixtures registry.
2. Spec §7 FIRST-PROOF-07 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `04`/`14`, architecture `03`/`04`/`05`, execution `02` gates `TRUTH-FIREWALL`/`ACTOR-KNOWN` and execution `04`/`05` bind it. This consumes `EPI-CERT` (commit `726b2a1f…`) as an integration relation, not a re-audit of its internals.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-07 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-07): `INV-100` (hidden-truth cognition forbidden), `INV-101` (actor-known context is sealed), `INV-099` (truth validates but does not plan), `INV-106` (validation failure feeds replanning without leakage). Restate before trusting the narrative: paired runs varying only unobserved truth leave pre-observation decisions and context fingerprints equal.
5. This ticket audits/reads (does not modify) the holder-known-context, candidate/method/plan/proposal, and action-validation enforcement surfaces. It runs paired fixtures and records witnesses only; validation failure must not leak hidden alternatives into actor-visible diagnostics, and debug-on/off runs must have identical authoritative events and actor-known outputs. No nondeterminism is introduced.

## Architecture Check

1. Paired runs varying only hidden food/route/workplace/item-custodian/debug-rows/validator-time, with recorded context ID/hash/frontier equality before observation, are the only check that proves cognition does not read unobserved truth; a single run cannot demonstrate the hyperproperty (spec §7 requires the equal-fingerprint relation plus a fail-closed provenance-deletion negative).
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let a validator author a goal/fallback or leak hidden alternatives into actor-visible diagnostics.

## Verification Layers

1. `INV-100`/`INV-101` sealed actor-known cognition -> replay/golden-fixture + paired-run check (transaction inputs enumerate actor-known facts and source refs; candidate/method/plan/proposal outputs cite the consumed context/source frontier; paired runs with only hidden differences keep pre-observation decisions and context fingerprints equal).
2. `INV-099`/`INV-106` truth validates-not-plans, no leakage -> manual review + codebase grep-proof (validation rejects stale/impossible proposals without proposing a fallback or revealing actor-illegible reasons; forged/stale context/actor/target/route/affordance/duration/reservation data cannot be accepted).
3. Debug quarantine -> adversarial check (debug-on/off runs have identical authoritative events and actor-known outputs; a modeled observation/notice can legally change a later decision).

## What to Change

### 1. Record positive actor-known participation witnesses

Run the §7 positive path. Record that transaction inputs enumerate actor-known facts and source refs; candidate/method/plan/proposal outputs cite the context or source frontier they consumed; validation can reject stale or impossible proposals without proposing a fallback or revealing actor-illegible reasons; and a modeled observation/notice can legally change a later decision. Record the sealed context ID/hash/frontier, candidate and method traces, proposal provenance, validation decision, and accepted event or explicit non-event with a clean replay.

### 2. Record paired-run non-interference and fail-closed negatives

Record the §7 adversarial cases: paired runs vary only hidden food, route, workplace, item custodian, debug rows, or validator-only time and keep pre-observation decisions and context fingerprints equal; deleting/substituting one provenance source fails closed; forged/stale context, actor, target, route, affordance, duration, or reservation data cannot be accepted; validation failure does not leak hidden alternatives into actor-visible diagnostics. Record that debug-on/off runs have identical authoritative events and actor-known outputs; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-07 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any firewall/actor-known defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Re-auditing `EPI-CERT` internals (consumed within scope, spec §4); absence observation (`-004`), the cross-gate metamorphic harness (`-017`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test acceptance_gates` pass; paired runs varying only unobserved truth keep pre-observation decisions and context fingerprints equal.
2. `cargo test --locked -p tracewake-core --test generative_lock` passes; deleting/substituting one provenance source fails closed and forged/stale inputs cannot be accepted.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test adversarial_gates` pass; validation failure leaks no hidden alternatives and debug-on/off runs have identical authoritative events and actor-known outputs.

### Invariants

1. No actor output dependent on unobserved truth; no validator-authored goal/fallback; no provenance loss; no debug-dependent cognition.
2. A modeled observation/notice legally changes a later decision; debug-on/off authoritative-event and actor-known-output equality holds.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test generative_lock`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test adversarial_gates`

## Outcome

Completed: 2026-06-21

Recorded FIRST-PROOF-07 in the shared acceptance artifact as passed for its
actor-known/truth-firewall scope. The evidence packet now includes
command-ledger rows, gate/scenario references, a FIRST-PROOF-07 audit section,
and two evidence ledger items: `E-0044-007-actor-known-noninterference` and
`E-0044-007-validation-fail-closed`.

Verification run:

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` -> pass, 17 passed.
2. `cargo test --locked -p tracewake-core --test acceptance_gates` -> pass, 12 passed.
3. `cargo test --locked -p tracewake-core --test generative_lock` -> pass, 5 passed.
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run` -> pass, 42 passed.
5. `cargo test --locked -p tracewake-tui --test adversarial_gates` -> pass, 15 passed.
