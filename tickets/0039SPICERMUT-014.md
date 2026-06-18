# 0039SPICERMUT-014: Kill `actions/report.rs` SPINE survivors with a checked-fact-key + pipeline-consequence matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `actions/report.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 13 missed mutants in `crates/tracewake-core/src/actions/report.rs` (spec §5.6), owning SPINE-06 (validation feedback) and SPINE-08 (no-direct-dispatch checked facts). The cluster deletes parser arms in `CheckedFactKey::from_stable_key` for body exclusivity, container, duration, from/to place, item, need kind, pipeline slots, place, reason, sleep affordance, target, and ticks. A direct `from_stable_key(key).is_some()` table is supporting evidence only; the certifying witness must include the report's pipeline consequence.

## Assumption Reassessment (2026-06-18)

1. `CheckedFactKey` is an enum at `crates/tracewake-core/src/actions/report.rs:97` with `from_stable_key(key: impl Into<String>) -> Self` at `:118` (verified by grep). The 13 seed-mutant identities (one per deleted key arm) are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.6 is the implementation contract; `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` and `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` govern validation feedback and no-direct-dispatch (verified present).
3. Shared boundary under audit: the checked-fact reporting seam where stable keys round-trip through the serialization/report path used by accepted and rejected pipeline reports.
4. Motivating invariants: `INV-008 — UI assistance is not authority` (why-not/feedback clarifies, it does not create facts) and `INV-106 — Validation failure feeds replanning without leakage`. Reports must be semantically consumed by a diagnostic or replay/trace artifact.
5. This ticket touches the no-direct-dispatch / validation-feedback surface: every stable key must round-trip through the real report path and be semantically consumed by a diagnostic or replay/trace artifact; unknown/misspelled/duplicate/unsupported keys must fail loudly with no generic fallback; both accepted and rejected proposals must be present so a report that emits all facts unconditionally cannot pass; and no-direct-dispatch evidence must show validation occurred before event append/application. The witnesses only strengthen feedback fidelity — no leakage is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-06/08 re-proof in ticket 021.

## Architecture Check

1. A full stable-key table round-tripped through the report path, each key carried into a real validation report whose checked fact is consumed by a diagnostic/replay/trace artifact, catches each deleted arm; including accepted and rejected proposals prevents an unconditional emit-all report from passing.
2. No backwards-compatibility aliasing/shims: the certifying witness includes the report's pipeline consequence, not a `from_stable_key(...).is_some()` table in isolation.

## Verification Layers

1. SPINE-06 checked-fact fidelity -> spine-conformance seam check: every supported stable key round-trips through the report path and produces a report whose checked fact is semantically consumed by a diagnostic or replay/trace artifact.
2. Fail-loud unknown keys -> spine-conformance seam check: unknown/misspelled/duplicate/unsupported keys fail loudly; no generic fallback turns them into another fact.
3. INV-106 / no-direct-dispatch -> spine-conformance seam check: accepted and rejected proposals are both present, and checked-fact evidence shows validation occurred before event append/application.

## What to Change

### 1. Checked-fact-key + pipeline-consequence matrix

In `spine_conformance.rs`, construct a table containing every stable key supported by the final enum, round-trip each through the same serialization/report path used by accepted and rejected pipeline reports, and for each key produce at least one real validation report whose checked fact is semantically consumed by a diagnostic or replay/trace artifact.

### 2. Fail-loud + accepted/rejected + no-dispatch rows

Prove unknown/misspelled/duplicate/unsupported keys fail loudly with no generic fallback; include both accepted and rejected proposals; tie no-direct-dispatch evidence to checked facts (validation before append/application).

### 3. Member matrix

Map each of the 13 deleted-arm mutants (plus any new run survivor in this file) to a concrete pipeline-consequence failure.

## Files to Touch

- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/actions/report.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Proposal source provenance (ticket 013).
- Scheduler ordering (ticket 015).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the full stable-key table, pipeline-consequence rows, fail-loud negatives, and accepted/rejected proposals.
2. `cargo mutants --workspace -f crates/tracewake-core/src/actions/report.rs --no-shuffle` — all 13 historical deleted-arm survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Each deleted key arm is killed via a report pipeline consequence, not a `from_stable_key(...).is_some()` table alone.
2. Unknown/duplicate/unsupported keys fail loudly; a report emitting all facts unconditionally cannot pass (accepted + rejected proposals both present).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` — full `CheckedFactKey` stable-key table round-tripped through accepted/rejected pipeline reports with diagnostic/replay/trace consumption + fail-loud and no-direct-dispatch rows.

### Commands

1. `cargo test --locked -p tracewake-core --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/actions/report.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
