# 0044FIRPROCER-006: FIRST-PROOF-06 — no culprit, suspect, clue, theft, quest, or story-sifting truth

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add audit-only instrumentation per spec §5.4 (evidence instrumentation, not production remediation).
**Deps**: 0044FIRPROCER-001

## Problem

Spec §7 audit point FIRST-PROOF-06 must be proven at `U`: the missing-property situation remains an ordinary epistemic situation. The world supplies no culprit flag, suspect token, theft truth, witness assignment, clue chain, quest state, lead board, or story-sifting objective. Actors may later form fallible suspicions only from actor-known evidence, but institutional investigation is not certified here. Content must reject semantically equivalent culprit/suspect/clue/quest/outcome-chain fields even when names avoid banned words; no validator, fixture runner, scheduler, planner, TUI, or debug adapter may choose a suspect from physical custody truth. This ticket records the FIRST-PROOF-06 positive ordinary-situation witnesses, semantic-rejection adversarial evidence, and projection field census into the acceptance artifact.

## Assumption Reassessment (2026-06-21)

1. The audited surfaces exist at `U`: `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`, content `schema.rs`/`validate.rs`/`serialization.rs`, `crates/tracewake-core/src/actions/defs/accuseprobe.rs`, and the holder-known/debug projection surfaces (all confirmed present this session). Fixtures `expectation_contradiction_001`, `knowledge_blocker_accuse_001`, `prose_born_fact_rejected_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` exist.
2. Spec §7 FIRST-PROOF-06 (seam, audited files, doctrine, positive/adversarial evidence, exact commands, failure condition) governs this ticket; doctrine foundation `02`/`04`/`09`/`12`/`14`, architecture `06` and the explicit boundary in `11`, execution `02`/`04`/`08`/`09` and the Phase-4 boundary in `11` bind it. Institutional investigation is deferred to `PHASE-4-ENTRY` (spec §14).
3. Cross-artifact shared boundary under audit: the FIRST-PROOF-06 section of `reports/0044_…_acceptance.md` (created `(new)` by `0044FIRPROCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §7 FIRST-PROOF-06): `INV-060` (no authored outcome chains / automatic culprit reveals), `INV-058` (quest is not a primary data type), `INV-030` (evidence is not truth), `INV-097` (no-script compliance is tested). Restate before trusting the narrative: the authoritative log contains physical and epistemic events, not a culprit fact; any suspicion-like record cites actor-known provenance and stays distinct from authoritative guilt.
5. This ticket audits/reads (does not modify) the content-validation/no-scripting and holder-known/debug-quarantine enforcement surfaces. It runs fixtures and records witnesses only; a hidden-holder permutation must not change actor decisions before modeled evidence, and debug may show physical truth but labels it non-diegetic without translating it into an actor-known accusation target. No nondeterminism is introduced.

## Architecture Check

1. Auditing semantic rejection (renamed/nested/generated culprit-equivalent fields, not just a banned-word list) plus a projection field census proving the log holds no culprit fact is the only check that catches an aliased outcome chain; spec §7 requires that `knowledge_blocker_accuse_001` or equivalent requires knowledge/provenance rather than a hidden culprit.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may import investigation machinery or a hidden-truth accusation target.

## Verification Layers

1. `INV-060`/`INV-058` no outcome chains / no quest type -> content-validation + projection census (content rejects culprit/suspect/clue/quest/outcome-chain fields even under innocuous names; the authoritative log and canonical fixture serialization contain no culprit fact).
2. `INV-030` evidence-is-not-truth -> manual review + codebase grep-proof (no validator/fixture-runner/scheduler/planner/TUI/debug adapter chooses a suspect from physical custody truth; any suspicion-like record cites actor-known provenance and is distinct from guilt).
3. `INV-097` no-script compliance -> replay/golden-fixture + adversarial check (a hidden-holder permutation cannot change actor decisions before modeled evidence; no repository link or historical Phase-2A statement is treated as live target truth).

## What to Change

### 1. Record positive ordinary-situation witnesses

Run `expectation_contradiction_001`, the content corpus, and the §7 positive path. Record that the canonical fixture can represent physical custody plus an actor's expectation/absence/contradiction without identifying a culprit; allowed post-contradiction behavior remains ordinary and actor-relative (search, ask/report when in current scope, misremember, form an unsupported/fallible suspicion through normal cognition, or do nothing); and debug may show physical truth but labels it non-diegetic and does not translate it into an actor-known accusation target. Record the canonical fixture serialization/schema output and the replay projection field census (no culprit/suspect/clue/quest field).

### 2. Record semantic-rejection adversarial evidence

Record the §7 adversarial cases: content rejects semantically equivalent culprit/suspect/clue/quest/outcome-chain fields even when names avoid banned words; no validator/fixture-runner/scheduler/planner/TUI/debug adapter chooses a suspect from physical custody truth; `knowledge_blocker_accuse_001` or equivalent requires knowledge/provenance rather than a hidden culprit; a hidden-holder permutation cannot change actor decisions before modeled evidence; no repository link or historical Phase-2A statement is treated as live target truth. Each negative names the canonical responsible layer (spec §11 vocabulary).

## Files to Touch

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md` (modify — FIRST-PROOF-06 section; file created by 0044FIRPROCER-001)

## Out of Scope

- Production remediation of any culprit-leakage defect (spec §5.4/§11 route a substantive failed seam to a later separately-numbered FIRST-PROOF-CERT remediation spec; record `fail` + responsible layer here, do not fix).
- Institutions, formal investigation, reports, sanctions, wrong-suspicion procedures (deferred to `PHASE-4-ENTRY`, spec §14).
- The consolidated fixture-negative corpus (`-011`) and the §10 mutation perimeter (`-018`); the aggregate verdict (`-019`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-core --test anti_regression_guards` pass; the authoritative log and canonical fixture contain physical and epistemic events but no culprit/suspect/clue/quest fact.
2. `cargo test --locked -p tracewake-content --test forbidden_content` and `--test schema_conformance` pass; content rejects semantically equivalent culprit/suspect/clue/quest/outcome-chain fields even under innocuous names.
3. `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test adversarial_gates` pass; a hidden-holder permutation cannot change actor decisions before modeled evidence and `knowledge_blocker_accuse_001` requires knowledge/provenance.

### Invariants

1. No authored or derived culprit/suspect/clue/quest truth, no hidden-truth accusation target, no imported investigation machinery.
2. Any suspicion-like record cites actor-known provenance and remains distinct from authoritative guilt; debug truth stays non-diegetic.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any audit-only instrumentation added under spec §5.4 stays observer-only and out of the production path.`

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
2. `cargo test --locked -p tracewake-core --test anti_regression_guards`
3. `cargo test --locked -p tracewake-content --test forbidden_content`
4. `cargo test --locked -p tracewake-content --test schema_conformance`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
6. `cargo test --locked -p tracewake-tui --test adversarial_gates`
