# 0044FIRPROCER-003: FIRST-PROOF-03 — source-backed expectation provenance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-03 must be proven at `U`: the expectation "item should be in container/place" exists as a typed actor-held belief/expectation with mandatory source provenance **before** absence is discovered. It may originate from modeled prehistory, memory, observation, record, communication, or another allowed channel; it may not be invented by fixture prose, UI narration, debug truth, or a missing-property script. The expectation must record holder, proposition, stance, source reference, acquisition tick, privacy/scope, and the relevant source event; the sealed actor-known context must include it and exclude unrelated holder data; fixture loading must create it only through an allowed typed seed/event path; and live and replay projections must preserve identical provenance. This ticket records the FIRST-PROOF-03 positive provenance witnesses, fail-closed adversarial negatives, compile-fail forgery boundary, and replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `crates/tracewake-core/src/epistemics/{belief.rs, proposition.rs, knowledge_basis.rs, knowledge_context.rs, projection.rs}`, `crates/tracewake-core/src/agent/{actor_known.rs, transaction.rs}`, `crates/tracewake-core/src/events/{envelope.rs, apply.rs}`, and content `schema.rs`/`serialization.rs`/`validate.rs` (all confirmed present this session). Fixtures `expectation_contradiction_001`, `forbidden_provenance_input_fails_closed_001`, `prose_born_fact_rejected_001`, and the compile-fail boundary `tests/negative-fixtures/external_crate_cannot_construct_belief_literal/src/lib.rs` exist.
2. Spec §7 FIRST-PROOF-03 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `04`/`12`/`14`, architecture `03`/`06`, execution `02` gates `ACTOR-KNOWN`/`MISSING-PROPERTY`/`FIXTURE-NEGATIVE`, and execution `04`/`08`/`09` bind it. The artifact must distinguish seed authoring provenance from live actor observation.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-03 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-03): `INV-102` (cognition inputs require provenance; missing provenance is a rejection condition for action-relevant cognition), `INV-026` (important beliefs record holder/claim/stance/source/channel/acquisition time/scope), `INV-101` (actor-known context is sealed and excludes validator-only truth), `INV-022` (raw prose is not authoritative state). Restate before trusting the narrative: every expectation is typed and source-backed before absence is discovered; nothing is forged from prose, truth, or debug.
5. This ticket audits/reads (does not modify) the holder-known-context sealing, content-validation fail-closed, and replay-provenance enforcement surfaces. It runs fixtures and records witnesses only; it introduces no provenance-free cognition path and no nondeterminism. The compile-fail boundary proves external crates cannot construct a raw belief literal or mutate source/scope through public APIs; debug rows recorded stay observer-only.

## Architecture Check

1. Auditing provenance by expanding the belief's holder/proposition/stance/source-event/acquisition-tick/scope and proving the sealed context excludes unrelated holder data (rather than trusting that the fixture "looks right") is the only check that catches a prose-seeded or debug-seeded expectation; spec §7 requires fail-closed behavior on absent/dangling/wrong-holder/future/debug-only/prose-only source.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may relax the fail-closed provenance requirement or admit a banned-free-but-unproven fixture field.

## Verification Layers

1. `INV-102`/`INV-026` provenance sufficiency -> schema-validation + replay/golden-fixture check (expectation records holder/proposition/stance/source ref/acquisition tick/scope and its source event; live and replay projections preserve identical provenance).
2. `INV-101` sealed actor-known context -> manual review + codebase grep-proof (the sealed context includes the expectation and excludes unrelated holder data and validator-only truth).
3. `INV-022` no prose-born fact -> content-validation + compile-fail check (`prose_born_fact_rejected_001` / `forbidden_provenance_input_fails_closed_001` reach the intended rejection stage; the external-crate compile-fail boundary proves protected belief records cannot be forged).

## What to Change

### 1. Record positive provenance witnesses

Run the §7 positive path. For the canonical expectation, record holder, proposition, stance, source reference, acquisition tick, privacy/scope, and the relevant source event; record that the sealed actor-known context includes the expectation and excludes unrelated holder data; record that fixture loading creates the expectation only through an allowed typed seed/event path; and record that live and replay projections preserve identical expectation provenance (holder-known context hash/frontier, canonical serialization, replay equality). Distinguish seed authoring provenance from live actor observation.

### 2. Record fail-closed adversarial negatives and forgery boundary

Record the §7 adversarial cases: absent, dangling, wrong-holder, wrong-kind, future, debug-only, or prose-only source fails closed; an external crate cannot construct a raw belief literal or mutate source/scope through public APIs (compile-fail boundary); adding banned-free but semantically unproven fixture fields is rejected; changing only hidden item truth leaves the expectation unchanged. Each negative names the canonical responsible layer (spec §11 vocabulary) and proves no accepted event/projection state results.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-03 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any provenance defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Absence observation (`-004`), contradiction formation (`-005`), no-culprit-truth (`-006`), the consolidated fixture-negative corpus (`-011`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` pass; the expectation is typed and source-backed with holder/proposition/stance/source-event/acquisition-tick/scope, and replay preserves identical provenance.
2. `cargo test --locked -p tracewake-content --test schema_conformance`, `--test forbidden_content`, and `cargo test --locked -p tracewake-core --test negative_fixture_runner` pass; absent/dangling/wrong-holder/future/debug-only/prose-only source fails closed and the compile-fail belief boundary holds.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; the sealed actor-known context includes the expectation and excludes unrelated holder data, and changing only hidden item truth leaves the expectation unchanged.

### Invariants

1. No expectation without sufficient source ancestry; no expectation forged from prose/truth/debug; no cross-holder leakage.
2. Seed authoring provenance is distinguished from live actor observation; live/replay provenance equality holds.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
4. `cargo test --locked -p tracewake-content --test schema_conformance`
5. `cargo test --locked -p tracewake-content --test forbidden_content`
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
