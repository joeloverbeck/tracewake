# 0019PHA3AGENREA-009: 0019 scoped acceptance artifact

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — evidence document only (`reports/`)
**Deps**: `tickets/0019PHA3AGENREA-005.md`, `tickets/0019PHA3AGENREA-006.md`, `tickets/0019PHA3AGENREA-007.md`, `tickets/0019PHA3AGENREA-008.md` (leaf set — -005's Deps transitively cover -001 … -004); `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (§7)

## Problem

Spec 0019 §7 requires a scoped acceptance artifact recording, for the implementation
commits, the evidence that each finding's correction and structural lock actually
landed — the lineage's per-pass evidence document (the 0018 analogue is
`reports/0018_ord_life_cert_scoped_acceptance.md`). Without it, the pass's claims are
unreviewable and the next audit cannot verify-or-refute them — the exact failure mode
(`ORD-HARD-044`'s evidence overstatement) this spec corrects.

## Assumption Reassessment (2026-06-11)

1. Verified against the current tree at `main` `5af8660`: the 0018 exemplar
   (`reports/0018_ord_life_cert_scoped_acceptance.md`) exists and carries the section
   shape this report mirrors (per-ticket commit table, per-finding evidence sections,
   non-certification statement); `reports/0019_ord_life_cert_scoped_acceptance.md` does
   not yet exist (no collision).
2. Verified against `specs/0019_…_HARDENING_SPEC.md` §7 (reassessed 2026-06-11): the
   artifact records nine numbered evidence items — (1) corrected generative corpus
   summary with per-family terminal counts from advance-emitted events and the
   tamper-relation divergence proof; (2) the expanded-perimeter mutation run, reviewed
   baseline, and the in-diff failure-mode demonstration; (3) the non-coincident-tick
   workplace freshness proof; (4) the `NeedThresholdCrossed` forged-version rejection
   proof and derived-census outputs; (5) per-actor need ledgers / checksum diffs for
   the -001 golden repricing, every diff explained (the 0016 §7.1 format); (6) the
   partial-knowledge fixture proof and helper-allowlist census output; (7) the embodied
   interruption surfacing proof (or the recorded deferral downgrade, per -008); (8) the
   corrected conformance row and the 0018 report correction, quoted; (9) the explicit
   non-certification statement.
3. Cross-artifact boundary under audit: the evidence contract between this report and
   the landed code — every claim must be backed by a command re-run at composition
   time against the implementation commits (the 0018 deviation note set the precedent:
   record current-tree evidence, never invent stale pre/post diffs).
4. Lineage evidence-honesty doctrine restated (and INV-091–098 spirit): acceptance
   evidence must claim exactly what the gates prove — this pass exists because a prior
   report overstated; this report's own claims will be adversarially re-verified by the
   next audit, so every assertion names its proving command/symbol.

## Architecture Check

1. A capstone evidence document gated on the full leaf set (rather than per-ticket
   evidence appendices) matches the lineage convention (0016–0018), keeps the per-actor
   repricing ledger in one reviewable place, and gives the next audit a single
   verify-or-refute target. It introduces no production logic — it exercises and
   records what the prior tickets built.
2. No backwards-compatibility aliasing/shims: N/A (documentation artifact); no claims
   are copied forward from 0018 — every command re-runs against the current tree.

## Verification Layers

1. Evidence completeness -> section-presence check: the report carries all nine §7
   items, each citing its proving command and output.
2. Evidence freshness -> every quoted command re-run at composition time against the
   implementation commits (manual runbook; outputs pasted, not recalled).
3. Non-certification honesty -> the §7 item-9 statement present verbatim in spirit:
   scoped evidence toward `ORD-LIFE-CERT`; not full-project certification, not Phase 4
   entry, not `FIRST-PROOF-CERT`.
4. Cross-reference integrity -> the per-ticket commit table maps every
   `0019PHA3AGENREA-00N` ticket to its landing commit (grep-checkable against
   `git log`).

## What to Change

### 1. Compose the report

New `reports/0019_ord_life_cert_scoped_acceptance.md` mirroring the 0018 report's
structure: header (spec path, baseline, scope posture), per-ticket commit table, one
evidence section per §7 item (1–9) with re-run command outputs, deviations-from-plan
section, and the non-certification statement.

### 2. Manual runbook (evidence collection)

For each §7 item, run the proving commands against the landed tree (the ticket-named
`cargo test` targets, the mutation `--list` perimeter proof, the census outputs, the
golden-diff ledger extraction) and paste actual outputs; any gap between a spec claim
and observed output is recorded as a deviation, never papered over.

## Files to Touch

- `reports/0019_ord_life_cert_scoped_acceptance.md` (new)

## Out of Scope

- Any code or test change — gaps discovered while collecting evidence route back to
  the owning ticket, not into this document.
- The `docs/4-specs/SPEC_LEDGER.md` row and the spec's move to `archive/specs/` —
  deferred to spec acceptance per `docs/archival-workflow.md` (cross-spec follow-up,
  outside this batch).
- Certifying `ORD-LIFE-CERT`, Phase 4 entry, or `FIRST-PROOF-CERT`.

## Acceptance Criteria

### Tests That Must Pass

1. `for i in 1 2 3 4 5 6 7 8 9; do grep -q "^## .*$i\." reports/0019_ord_life_cert_scoped_acceptance.md || echo "MISSING item $i"; done`
   (or the equivalent section-presence proof for the chosen heading scheme) — all nine
   §7 items present.
2. `grep -qi "not full-project certification" reports/0019_ord_life_cert_scoped_acceptance.md` —
   non-certification statement present.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
   — the tree the report describes is green at composition time.

### Invariants

1. Every evidence claim in the report names a command re-run at composition time
   against the implementation commits; no inherited or invented evidence.
2. The report claims exactly what the gates prove — concessions and deviations are
   recorded, never overstated.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing
   pipeline coverage is named in Assumption Reassessment.`

### Commands

1. The §7 per-item proving commands (the owning tickets' named test commands, re-run).
2. `cargo test --workspace`
3. A narrower per-item command is correct where named (e.g. `cargo mutants --list -f …`
   for the perimeter proof) because each evidence item is scoped to one gate; the
   workspace run is the closing full-pipeline check.

## Outcome

Completed 2026-06-11.

Created `reports/0019_ord_life_cert_scoped_acceptance.md` with the required
implementation commit table, nine numbered §7 evidence sections, a deviations section,
and an explicit non-certification boundary. The report records current-tree command
evidence for the generative corpus, mutation perimeter, workplace freshness,
payload-version/census locks, need-ledger/checksum repricing, partial-knowledge fixture,
embodied interruption surfacing, conformance correction, and 0018 report correction.

Verification:

1. `for i in 1 2 3 4 5 6 7 8 9; do grep -q "^## .*$i\\." reports/0019_ord_life_cert_scoped_acceptance.md || echo "MISSING item $i"; done`
2. `grep -qi "not full-project certification" reports/0019_ord_life_cert_scoped_acceptance.md`
3. `cargo test -p tracewake-core --test generative_lock -- --nocapture`
4. `cargo test -p tracewake-core --test anti_regression_guards generative_lock_cannot_fabricate_duration_terminals -- --nocapture`
5. `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs'`
6. `cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle --jobs 4`
7. `cargo test -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters -- --nocapture`
8. `cargo test -p tracewake-core --test hidden_truth_gates workplace_requires_assignment_or_observation_provenance -- --nocapture`
9. `cargo test -p tracewake-core --test event_schema_replay_gates forged_threshold_payload_schema_version_rejected_for_materialized_agent_replay_001 -- --nocapture`
10. `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_payload_records_keep_payload_fields -- --nocapture`
11. `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture`
12. `cargo test -p tracewake-core --test anti_regression_guards guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms -- --nocapture`
13. `cargo test -p tracewake-content --test golden_fixtures_run no_human_need_ledger_has_no_duplicate_regime_charges -- --nocapture`
14. `cargo test -p tracewake-content --test fixtures_load known_food_source_blanket_helper_call_sites_are_allowlisted -- --nocapture`
15. `cargo test -p tracewake-content --test golden_fixtures_run partial_food_source_knowledge_seeds_only_authored_actor_edge -- --nocapture`
16. `cargo test -p tracewake-core view_models_embodied_phase3a_salient_interruption_is_viewer_scoped`
17. `cargo test -p tracewake-tui renderer_prints_phase3a_salient_interruption`
18. `cargo fmt --all --check`
19. `cargo clippy --workspace --all-targets -- -D warnings`
20. `cargo build --workspace --all-targets --locked`
21. `cargo test --workspace`
