# 0040EPICERHOL-012: §6.1 compile-fail boundary corpus — case-by-case matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs the existing compile-fail corpus and records the case-by-case matrix. May add test-only instrumentation per spec §2.
**Deps**: 0040EPICERHOL-001

## Problem

Spec 0040 §6.1 requires the implementing session to record the full `negative_fixture_runner` output and a case-by-case matrix for the compile-fail boundary corpus, with each row stating the forbidden capability, the compiler failure class, the relevant EPI point, and **why the failure closes an external API path rather than merely matching an error string**. This cross-cutting negative-evidence section feeds the adversarial columns of EPI-01/02/03/06/10. This ticket runs the corpus and writes the §6.1 section of the artifact created by `-001`. It runs existing code; it renders no verdict.

## Assumption Reassessment (2026-06-19)

1. All §6.1 corpus members verified present at `ba9fe1c` (2026-06-19) under `tests/negative-fixtures/` plus the in-source doctests: `banned_float_confidence_types`, `external_crate_cannot_build_debug_knowledge_context`, `external_crate_cannot_build_debug_projection_view_without_core_debug_api`, `external_crate_cannot_construct_belief_literal`, `external_crate_cannot_construct_debug_report`, `external_crate_cannot_construct_observation_without_source`, the `debug_capability.rs` compile-fail doctests for literal construction and `DebugCapability::mint()` (verified `pub(crate) const fn mint()`), `external_crate_cannot_insert_raw_epistemic_records`, `external_crate_cannot_mutate_belief_source_or_scope`, `external_crate_cannot_mutate_contradiction_links`, `external_crate_cannot_mutate_knowledge_context_mode`, `external_crate_cannot_mutate_knowledge_context_viewer`, `external_crate_cannot_read_raw_epistemic_projection_maps`. The `negative_fixture_runner` test binary and `cargo test -p tracewake-core --doc` both verified available.
2. The acceptance artifact is created `(new)` by `-001`; this ticket `(modify)`s its §6.1 section only. Section wording follows spec §6.1 and the `0003` evidence fields.
3. Shared boundary under audit: the external-crate API boundary — the sealed/private constructors and `pub(crate)` capability mint that make each forbidden capability uncompilable from outside core. The §6.1 quantifier ("at least [list]") expands to **13 enumerated boundary cases** (12 negative-fixture directories + the `debug_capability.rs` doctest pair, counted as one corpus row), each requiring its own matrix row (per §Defect-class closure: a class-stated deliverable enumerates its members).
4. `INV-107` (debug omniscience quarantined) and `INV-031` (human/debug notes non-diegetic) motivate this section, with `INV-021`/`INV-024`/`INV-026` (typed epistemic currency, no telepathy, provenance) — the compile-fail corpus is the structural enforcement that an external crate cannot forge beliefs/observations/contexts/reports or read raw projection maps.
5. This ticket audits the compile-fail enforcement boundary as an **evidence consumer**: it confirms each fixture stays negative (does not compile) and records *why* the failure is an API-closure (sealed field / private constructor / `pub(crate)` mint / missing source argument), not a brittle error-string match. It modifies no enforcement and weakens no API. A corpus member that unexpectedly compiles is recorded `fail` with responsible layer and routed out (§7.6/§8); not fixed here.

## Architecture Check

1. A single cross-cutting compile-fail corpus ticket — rather than duplicating the 13-row matrix across the five EPI points that cite it — keeps the case-by-case matrix authored once and referenced by EPI-01/02/03/06/10, mirroring the consolidated-negative-evidence approach in the predecessor audit series.
2. No backwards-compatibility aliasing or shims; no production logic. New evidence exercises the canonical compile-fail seam (spec §2).

## Verification Layers

1. `INV-107`/`INV-031` debug-forge closure -> negative fixture: `…_build_debug_knowledge_context`, `…_build_debug_projection_view_without_core_debug_api`, `…_construct_debug_report`, and the `debug_capability.rs` mint/literal doctests stay negative.
2. `INV-021`/`INV-026` epistemic-forge closure -> negative fixture: `banned_float_confidence_types`, `…_construct_belief_literal`, `…_construct_observation_without_source`, `…_insert_raw_epistemic_records`, `…_mutate_belief_source_or_scope`, `…_mutate_contradiction_links`, `…_read_raw_epistemic_projection_maps` stay negative.
3. `INV-101` sealed-context immutability -> negative fixture: `…_mutate_knowledge_context_mode`, `…_mutate_knowledge_context_viewer` stay negative.
4. API-closure rationale -> manual review: each matrix row states the forbidden capability, compiler failure class, relevant EPI point, and why the failure closes the API path rather than matching an error string.

## What to Change

### 1. Run the corpus and build the case-by-case matrix

Run `negative_fixture_runner` (full output retained) and the `debug_capability.rs` compile-fail doctests. Build a 13-row matrix; each row: forbidden capability; compiler failure class; relevant EPI point (`EPI-01`…`EPI-10`); and the API-closure rationale (which sealed field / private constructor / `pub(crate)` mint / missing-source-argument makes it uncompilable), explicitly distinct from an error-string match.

### 2. Write the §6.1 section of the acceptance artifact

Populate the §6.1 section with the full runner output reference and the 13-row matrix, each row carrying exactly one §9.2 evidence status.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (modify — file created by 0040EPICERHOL-001; §6.1 section only)

## Out of Scope

- The per-audit-point evidence sections that cite these fixtures (EPI-01/02/03/06/10, owned by their own tickets) — this ticket authors the consolidated matrix they reference.
- The §9.4 verdict table and aggregate verdict (owned by `-015`).
- Any production remediation of a corpus member that unexpectedly compiles — recorded `fail` with responsible layer; routed to a separate `EPI-CERT scoped remediation` ticket/spec (§7.6/§8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — full output captured; all 12 external-crate / banned fixtures remain compile-fail negative.
2. `cargo test --locked -p tracewake-core --doc` — the `debug_capability.rs` compile-fail doctests (literal construction + `DebugCapability::mint()`) remain negative.
3. The §6.1 matrix contains all 13 enumerated rows, each with forbidden capability, compiler failure class, relevant EPI point, and API-closure rationale.

### Invariants

1. Every §6.1 corpus member is negative (does not compile from an external crate / fails the doctest); a member that compiles is recorded `fail`, never silently dropped (`INV-107`/`INV-021`).
2. Each row's rationale is an API-closure argument (sealed field / private constructor / `pub(crate)` mint / missing source), not an error-string match (§6.1).

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification runs the existing compile-fail corpus (negative_fixture_runner + the debug_capability.rs doctests) and records the full output plus the 13-row API-closure matrix as the deliverable. Any test-only instrumentation added stays evidence-only per spec §2.`

### Commands

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
2. `cargo test --locked -p tracewake-core --doc`
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner -- --list` (enumerate the corpus members covered by the matrix)
