# 0007PHA3ASECHAR-013: Status/ledger documentation alignment

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — docs only (`docs/4-specs/SPEC_LEDGER.md`, `docs/4-specs/README.md`)
**Deps**: 0007PHA3ASECHAR-012

## Problem

Status docs and ledgers must not claim Phase 3A has landed far enough to unblock Phase 3B or ordinary-life-dependent Phase 4 work until the Spec 0007 acceptance gates pass (Spec 0007 D-10, §Documentation/status updates required). After the capstone (0007PHA3ASECHAR-012) proves integrated readiness, the ledger must record Spec 0007 and its result, and any 0006 wording implying readiness beyond evidence must be corrected.

## Assumption Reassessment (2026-06-07)

1. Confirmed: `docs/4-specs/SPEC_LEDGER.md` carries a "Next required spec" section reserving "Phase 3A Second Hardening: Integrated No-Human Ordinary-Life Proof" (the 0007 placeholder) and a Spec 0006 entry whose "Remaining blocking result" states integrated readiness is not fully earned. `docs/4-specs/README.md` already states the 0006 reassessment "finds that integrated Phase 3A ordinary-life readiness is still not fully earned" and that a second hardening spec is required.
2. Spec 0007 §Documentation/status updates required: update `SPEC_LEDGER.md` to record this second hardening spec and its result; update `README.md` only if its 0006 wording still implies readiness beyond evidence; add a narrow status errata if any archived completion claim remains misleading; do not rewrite foundation/architecture/execution/reference doctrine.
3. Shared boundary under audit: the spec-package status surface (`SPEC_LEDGER.md` is the authoritative status source per `docs/4-specs/README.md`). This ticket annotates aggregate completion gated on the capstone (0007PHA3ASECHAR-012); it cites no individual engine symbol, so `Deps: <capstone>` is sufficient.

## Architecture Check

1. Recording the 0007 result in the single authoritative status source (the ledger) after the capstone passes keeps the package's readiness claims evidence-bound — preventing a future spec from treating Phase 3A as safe on stale 0006 language. Routing the ledger flip through a capstone-gated docs ticket avoids a staleness window where the ledger claims readiness before the gates pass.
2. No backwards-compatibility aliasing/shims: no doctrine docs are rewritten; only the spec-package status surface is updated.

## Verification Layers

1. Evidence-bound status (no overclaim) -> codebase grep-proof: `SPEC_LEDGER.md` contains a Spec 0007 entry recording the passing capstone result; no remaining line claims Phase 3A unblocks Phase 3B/4 absent the 0007 gates.
2. Doctrine untouched -> codebase grep-proof: `git diff` touches only `docs/4-specs/SPEC_LEDGER.md` and `docs/4-specs/README.md` (and a narrow errata if added), not `docs/0-foundation`/`1-architecture`/`2-execution`/`3-reference`.
3. Single-layer ticket: this is a documentation-status ticket; verification is grep-based against the post-implementation tree, so replay/schema/skill-dry-run layers are not applicable.

## What to Change

### 1. Record Spec 0007 in the ledger

Add a Spec 0007 entry to `docs/4-specs/SPEC_LEDGER.md` (title, status, deliverables produced, what it settles) and convert the "Next required spec" placeholder into the landed 0007 record, stating that Spec 0006 landed useful hardening but did not fully prove integrated Phase 3A readiness, now earned by 0007.

### 2. Correct any residual readiness overclaim

Update `docs/4-specs/README.md` only if its 0006 wording still implies readiness beyond evidence; add a narrow status errata only if an archived completion claim remains misleading.

## Files to Touch

- `docs/4-specs/SPEC_LEDGER.md` (modify)
- `docs/4-specs/README.md` (modify)

## Out of Scope

- All engine/test surfaces (0007PHA3ASECHAR-001…012).
- Rewriting foundation/architecture/execution/reference doctrine.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n '0007' docs/4-specs/SPEC_LEDGER.md` shows a Spec 0007 entry with the capstone result; the "Next required spec" placeholder is resolved.
2. No line in `SPEC_LEDGER.md`/`README.md` claims Phase 3A unblocks Phase 3B or ordinary-life-dependent Phase 4 work absent the 0007 gates.
3. `git diff --name-only` for this ticket lists only `docs/4-specs/SPEC_LEDGER.md` and `docs/4-specs/README.md` (plus a narrow errata if added).

### Invariants

1. `SPEC_LEDGER.md` remains the authoritative, evidence-bound status source.
2. No higher-tier doctrine doc is modified.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n '0007' docs/4-specs/SPEC_LEDGER.md`
2. `git diff --name-only docs/`

## Outcome

Completed: 2026-06-07

Changed behavior:
- Replaced the ledger's "Next required spec" placeholder with a landed Spec 0007 record that names the passed integrated no-human ordinary-life proof and the carried-forward Phase 3A result.
- Updated the docs spec README so Spec 0007 is the current evidence source for Phase 3A no-human ordinary-life readiness.
- Reworded older Spec 0005/0006 status lines so they remain historical audit findings and no longer read as current blockers after Spec 0007.

Deviations:
- No higher-tier doctrine docs or errata files were needed; the misleading status surface was limited to `SPEC_LEDGER.md` and `README.md`.

Verification:
- `grep -n '0007' docs/4-specs/SPEC_LEDGER.md`
- `rg -n "required before Phase 3B|until the second Phase 3A|still not fully earned|unblock Phase 3B|unblocks Phase 3B|ordinary-life-dependent Phase 4 work treats Phase 3A as safe" docs/4-specs/SPEC_LEDGER.md docs/4-specs/README.md` returned no matches.
- `git diff --name-only docs/` listed only `docs/4-specs/README.md` and `docs/4-specs/SPEC_LEDGER.md`.
