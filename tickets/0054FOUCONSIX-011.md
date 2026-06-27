# 0054FOUCONSIX-011: Acceptance capstone — fail-closed status manifest and computed verdict

**Status**: PENDING
**Priority**: HIGH
**Engine Changes**: None — acceptance-only verdict artifact (`reports/0054_foundational_conformance_sixth_hardening_acceptance.md`); no new production logic
**Effort**: Small
**Deps**: 0054FOUCONSIX-007, 0054FOUCONSIX-009, 0054FOUCONSIX-010

## Problem

The spec produces no acceptance verdict itself — the implementing session must publish one (spec §8). The capstone assembles the acceptance artifact following `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, carries the §4.4 machine-readable status manifest computed by the fail-closed state machine, records per-finding closure with real production-path evidence, and must not render `pass` unless the manifest computes pass — i.e. unless every required finding is `closed`, governance independence is enforced (or honestly `pending-governance` computing non-pass), mutation evidence is current and green, and the actual artifact under review was ingested.

## Assumption Reassessment (2026-06-27)

1. The fail-closed state machine (ticket 004), governance computation (ticket 005), PR-blocking mutation gate (ticket 006), the below-foundation doctrine + closed grammar (ticket 007), the standing-campaign denominators (ticket 009), and the truthed conformance rows (ticket 010) all precede this capstone. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is the artifact template (closed grammar landed in ticket 007). The `reports/<NNNN>_<slug>_acceptance.md` naming follows the `0053_foundational_conformance_fifth_hardening_acceptance.md` convention. Confirmed at `7660051`.
2. This capstone `Deps` the doctrine leaf (007), the mutation-run leaf (009 — transitively covers 001/002/003/006/008), and the doc-truthing leaf (010 — covers 001/002/003); together 007+009+010 transitively gate the full F6-01…F6-07 DAG. Confirmed via the dependency graph.
3. Shared boundary under audit: the acceptance artifact as a read model over current evidence. Acceptance-only — introduces no new production logic; it exercises the machinery the earlier tickets composed and renders the computed verdict.
4. INV-098 (harsh acceptance) and the evidence-honesty contract: the verdict is computed from current, independently-acceptable, mutation-green evidence, not asserted in prose. Restated before trusting the narrative.
5. Evidence-consumer surface: this ticket reads the fail-closed-validation / governance / mutation / no-leak / replay enforcement surfaces sealed and built by tickets 001–010; confirm the artifact's status manifest computes pass only when every required finding is `closed`, governance is enforced (or honestly `pending-governance`), mutation evidence is current and green, and the actual artifact under review was ingested — and that its debug/evidence rows stay observer-only (no leakage path). The artifact names its own exact implementation commit, not `7660051`.

## Architecture Check

1. A single acceptance-only capstone whose verdict is the computed output of ticket 004's state machine (fed by 005's governance transcript and 006's mutation evidence) makes a laundered pass structurally impossible: the artifact cannot render `pass` over open/pending/survivor rows because the manifest computes `NonPass`. Folding the per-finding evidence into one artifact (per the §Capstone integration shape) keeps the verdict legible.
2. No backwards-compatibility aliasing/shims: the artifact carries the machine-computed manifest, not a hand-authored verdict beside it.

## Verification Layers

1. Computed verdict (INV-098) → the artifact's `tracewake-acceptance-status` block parses and computes `pass` only when every required finding is `closed`, governance enforced/`pending-governance`, mutation current+green, actual artifact ingested (validated by tickets 004/005/006 machinery).
2. Per-finding evidence → each F6-01…F6-07 row records real production-path evidence (sealed constructor, actor-legible wait receipt, non-inducible debug authority, observed effect, sensitivity/negative-compile proof) + the required-check names + a ruleset API transcript including approval/last-push/required-reviewer fields.
3. Doc-truthing precondition → confirms §6.2 truthing (ticket 010) and §6.1 doctrine (ticket 007) landed before the verdict.

## What to Change

### 1. Assemble the acceptance artifact

Author `reports/0054_foundational_conformance_sixth_hardening_acceptance.md` following template `0003`, carrying the §4.4 machine-readable status manifest, per-finding closure evidence (real production-path), the required-check names, and the ruleset API transcript including approval/last-push/required-reviewer fields.

### 2. Render the computed verdict

The artifact must not render `pass` unless the manifest computes pass. Name the exact implementation commit (not `7660051`).

## Files to Touch

- `reports/0054_foundational_conformance_sixth_hardening_acceptance.md` (new)

## Out of Scope

- Any new production logic (the capstone exercises the pipeline the earlier tickets composed; it does not modify their files).
- The `docs/4-specs/SPEC_LEDGER.md` *Archived implementation specs* row and the `specs/` → `archive/specs/` move — deferred to spec acceptance/archival per `docs/archival-workflow.md` (a cross-spec follow-up, not ticketed).
- Editing any archived spec/ticket/report/acceptance/certification (immutable history).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test acceptance_status_manifest` — the authored artifact's status block parses and the stated `overall_result` matches the computed result.
2. `cargo test -p tracewake-core --test acceptance_artifact_wording` — the artifact's prose introduces no verdict stronger than the computed state.
3. `cargo test -p tracewake-core --test ci_workflow_guards` — the artifact is ingested as the actual acceptance artifact for the closure PR and passes the ingestion guard (or honestly computes `pending-governance` for a sole-maintainer repo).

### Invariants

1. The artifact renders `pass` only when the manifest computes pass — every required finding `closed`, governance enforced or `pending-governance`, mutation current+green, actual artifact ingested.
2. The artifact names its own exact implementation commit and edits no archived history.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; verification is the fail-closed manifest/wording/ingestion guards (tickets 004/005/006) computing the verdict over the authored artifact. No new production logic.`

### Commands

1. `cargo test -p tracewake-core --test acceptance_status_manifest --test acceptance_artifact_wording --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. The acceptance-machinery tests are the correct verification boundary: the capstone adds no production logic, so its proof is the computed manifest verdict over the authored artifact, not a new code test.
