# 0044FIRPROCER-011: FIRST-PROOF-11 — fixture-negative, schema, compile-fail, and semantic content rejection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-11 must be proven at `U`: forbidden authoring and public-API paths fail before they can contaminate runtime, and rejection is semantic enough to catch renamed/nested/generated equivalents, not only a banned-word list. All first-proof fixtures must load through the canonical registry and declare supported schema/scope; source-backed epistemic seeds must canonicalize deterministically; valid negative fixtures must reach the intended rejection stage and name the responsible layer; and compile-fail fixtures must prove external callers cannot forge protected epistemic records. This ticket records the FIRST-PROOF-11 positive load/canonicalization witnesses, semantic-rejection adversarial corpus, and compile-fail boundary results (with fingerprint-scope honesty) into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: content `schema.rs`/`validate.rs`/`load.rs`/`serialization.rs` and the fixture registry; suites `forbidden_content.rs`/`schema_conformance.rs`/`fixtures_load.rs` (content) and `negative_fixture_runner.rs`/`anti_regression_guards.rs` (core); the compile-fail fixtures for belief/observation/contradiction under `tests/negative-fixtures/` (confirmed: `external_crate_cannot_construct_belief_literal`, `external_crate_cannot_construct_observation_without_source`, `external_crate_cannot_mutate_contradiction_links`, among the 30 present). Fixtures `prose_born_fact_rejected_001`, `forbidden_provenance_input_fails_closed_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` exist.
2. Spec §7 FIRST-PROOF-11 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `09`/`12`/`14`, architecture `01`/`03`/`13`, execution `02` gate `FIXTURE-NEGATIVE` and execution `04`/`08`/`09`/`10` bind it.
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-11 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-11): `INV-022` (raw prose is not authoritative state), `INV-097` (no-script compliance is tested — hidden outcome chains/objective markers rejected), `INV-020` (unversioned/unknown schema rejected), `INV-061` (authored machinery creates possibility space, not guaranteed arcs). Restate before trusting the narrative: rejection is semantic, fail-closed, and produces no partial runtime state.
5. This ticket audits/reads (does not modify) the content-schema/validation fail-closed and external-crate-forgery enforcement surfaces. It runs fixtures and records witnesses only; a rejected fixture cannot produce partial world or projection state, and fingerprint scopes (raw-byte vs canonical-content vs fixture-registry vs behavior) are recorded distinctly. No nondeterminism is introduced.

## Architecture Check

1. Proving semantic rejection (aliases, nested wrappers, generated metadata, innocuous field names carrying the same prohibited authority) plus compile-fail forgery boundaries, with distinct fingerprint scopes, is the only check that catches a text-only guard evasion; spec §7 requires rejecting fixture "expected outcome" data if runtime code could consume it as a plan.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may admit a renamed/nested/generated equivalent of a forbidden field.

## Verification Layers

1. `INV-022`/`INV-097` no prose-born fact / no-script -> content-validation + content-schema check (forbidden culprit/suspect/clue/quest/outcome-chain, prose-born fact, raw hidden truth, blanket known-food/property helpers, missing provenance, direct-dispatch marker, debug-as-knowledge, and unversioned/unknown schema are rejected, including semantic aliases/nested wrappers/generated metadata).
2. `INV-020` schema versioning -> content-schema check (all first-proof fixtures load through the canonical registry and declare supported schema/scope; source-backed epistemic seeds canonicalize deterministically).
3. `INV-061` possibility-not-script + fail-closed -> compile-fail + manual review (compile-fail fixtures prove external callers cannot forge protected epistemic records; a rejected fixture produces no accepted event/log/projection state; fingerprint scopes are recorded distinctly).

## What to Change

### 1. Record positive load/canonicalization witnesses

Run `fixtures_load`, `schema_conformance`, and the supported schema corpus. Record that all first-proof fixtures load through the canonical registry and declare supported schema/scope; source-backed epistemic seeds canonicalize deterministically; valid negative fixtures reach the intended rejection stage and name the responsible layer; and compile-fail fixtures prove external callers cannot forge protected epistemic records. For valid content, record the canonical semantic fingerprint and replay seed identity, distinguishing raw-byte, canonical-content, fixture-registry, and behavior fingerprint scopes.

### 2. Record semantic-rejection adversarial corpus

Record the §7 adversarial cases: reject culprit/suspect/clue/quest/outcome-chain, prose-born fact (`prose_born_fact_rejected_001`), raw hidden truth (`hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`), blanket known-food/property helpers, missing provenance (`forbidden_provenance_input_fails_closed_001`), direct action dispatch marker, debug-as-knowledge, and unversioned/unknown schema; reject semantic aliases, nested wrappers, generated metadata, or innocuous field names carrying the same prohibited authority; reject fixture "expected outcome" data if runtime code could consume it as a plan; ensure a rejected fixture cannot produce partial world or projection state. Each rejection names the canonical responsible layer (spec §11 vocabulary) and proves no accepted event/log/projection exists.

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-11 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any content-rejection defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Expectation provenance (`-003`), no-culprit-truth (`-006`), temporal fixture families (`-015`), and the §10 mutation perimeter (`-018`).
- The aggregate verdict and reconciled gate/scenario tables (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test fixtures_load` and `--test schema_conformance` pass; all first-proof fixtures load through the canonical registry, declare supported schema/scope, and unversioned/unknown schema is rejected.
2. `cargo test --locked -p tracewake-content --test forbidden_content` and `cargo test --locked -p tracewake-core --test negative_fixture_runner`, `--test anti_regression_guards` pass; semantic aliases/nested wrappers/generated metadata of forbidden authority are rejected and compile-fail forgery boundaries hold.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; a rejected fixture produces no partial world/projection state and fingerprint scopes are recorded distinctly.

### Invariants

1. No runtime consumption of forbidden data, no partial load, no forgeable protected records, no text-only guard evasion, no dishonest fingerprint scope.
2. Source-backed epistemic seeds canonicalize deterministically; raw-byte/canonical-content/fixture-registry/behavior fingerprint scopes are kept distinct.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
2. `cargo test --locked -p tracewake-core --test anti_regression_guards`
3. `cargo test --locked -p tracewake-content --test fixtures_load`
4. `cargo test --locked -p tracewake-content --test forbidden_content`
5. `cargo test --locked -p tracewake-content --test schema_conformance`
6. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
