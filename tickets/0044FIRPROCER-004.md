# 0044FIRPROCER-004: FIRST-PROOF-04 — absence is discovered by modeled observation, not authoritative truth

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-04 must be proven at `U`: the actor learns absence by performing an allowed local perception/search/check action and receiving an observation record. Authoritative truth may validate the action and determine the observation's result, but it may not notify cognition that an item is missing or choose the search target. An accepted check/inspect event must causally precede a typed observation event naming actor, object/container/place, channel, observed tick, source event, and explicit absence result, entering only the correct holder's epistemic projection. This ticket records the FIRST-PROOF-04 positive observation witnesses, truth-firewall adversarial negatives, and live/replay epistemic-projection equality into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/agent/perception.rs`, `crates/tracewake-core/src/epistemics/{observation.rs, knowledge_context.rs, projection.rs}`, `crates/tracewake-core/src/actions/defs/{checkcontainer.rs, inspect.rs}`, `crates/tracewake-core/src/actions/{proposal.rs, pipeline.rs}`, and `crates/tracewake-core/src/events/{envelope.rs, apply.rs}` (all confirmed present this session). Fixtures `expectation_contradiction_001`, `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`, and `hidden_route_edge_001` exist.
2. Spec §7 FIRST-PROOF-04 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `03`/`04`/`06`/`12`/`14`, architecture `03`/`04`/`06`/`09`, execution `04`/`05`/`09` bind it. Absence is evidence only through an allowed channel.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-04 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-04): `INV-016` (absence becomes evidence only through expectation/perception/instruction/search), `INV-024` (no telepathy), `INV-047` (search is intentional, costly, bounded, fallible), `INV-099`/`INV-100` (truth may validate but not plan; hidden-truth cognition forbidden). Restate before trusting the narrative: truth determines the observation's result but does not author absence or select the target.
5. This ticket audits/reads (does not modify) the holder-known-context, action-validation, event-application, projection, and replay enforcement surfaces. It runs fixtures and records witnesses only; reading current item location, fixture expected outcome, debug report, replay state, or validator data must not directly create the observation (recorded as adversarial negatives). No nondeterminism is introduced; debug rows stay observer-only.

## Architecture Check

1. Auditing the causal chain (actor-known expectation motivates a local check without ground truth selecting the target → validation checks locality/access/container state → accepted check/inspect event causally precedes the typed observation event) is the only check that distinguishes a modeled observation from a truth-authored "item is missing" notification; spec §7 requires the check source event closure and a fail-closed negative when it is removed/substituted.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may let truth, debug, or replay state synthesize an observation.

## Verification Layers

1. `INV-016`/`INV-047` absence-through-channel -> replay/golden-fixture check (an accepted check/inspect event causally precedes a typed `ObservationRecorded`-class event naming actor/object/channel/observed tick/source event/explicit absence result).
2. `INV-024`/`INV-099`/`INV-100` truth firewall -> manual review + codebase grep-proof (ground truth does not select the target; reading current item location, fixture expected outcome, debug, replay, or validator data cannot directly create the observation).
3. `INV-018` replay equality -> replay/golden-fixture check (the observation enters only the correct holder's epistemic projection; live/replay epistemic projection match; rejected checks produce no observation event).

## What to Change

### 1. Record positive observation witnesses

Run the §7 positive fixtures (`expectation_contradiction_001`, `hidden_food_closed_container_001`, `hidden_food_unknown_route_001`, `hidden_route_edge_001`). Record that an actor-known expectation can motivate a local check without ground truth selecting the target; validation checks locality/access/container state; an accepted check/inspect event causally precedes a typed observation event; the observation names actor, object/container/place, channel, observed tick, source event, and explicit absence result; and the observation enters only the correct holder's epistemic projection. Record action proposal/validation, check event ID, observation event ID and cause, source-ref closure, and actor-known context before/after.

### 2. Record truth-firewall adversarial negatives

Record the §7 adversarial cases: no observation is created for an inaccessible, closed-where-inspection-requires-open, unknown-route, nonlocal, or unperformed check; reading current item location / fixture expected outcome / debug report / replay state / validator data cannot directly create the observation; removing the check source event or substituting another actor's event causes fail-closed behavior; an item present in the checked location yields the corresponding presence observation and no false absence. Record live/replay epistemic projection and negative event absence for rejected checks; each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-04 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any observation/firewall defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Expectation provenance (`-003`), contradiction formation (`-005`), truth-firewall integration relation (`-007`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test golden_scenarios` pass; an accepted check/inspect event causally precedes a typed observation naming actor/object/channel/observed-tick/source-event/absence-result.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes; the observation enters only the correct holder's projection and live/replay epistemic projections match.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test adversarial_gates` pass; inaccessible/closed/unknown-route/nonlocal/unperformed checks create no observation, and an item present yields a presence observation with no false absence.

### Invariants

1. No truth-authored absence; no observation without an allowed source action/event; no remote perception; no wrong-holder insertion.
2. Live/replay observation projection equality; rejected checks produce no negative event leakage into cognition.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test adversarial_gates`
