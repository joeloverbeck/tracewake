# 0004PHA1THIHAR-011: Acceptance-artifact no-overclaim guard

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` scoped-wording requirement + a forbidden-wording check
**Deps**: None

## Problem

The acceptance artifact for this remediation must use scoped wording — "Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit `<commit>`" — and must not claim latest main, project-wide certification, or later-phase certification (spec §8 THIRD-AC-012, §10.4). Nothing currently enforces the absence of overclaim wording, so a future artifact could silently promote scoped evidence into global authority.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, fail-closed doc-validation ticket. -->

1. `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is the live acceptance-artifact template (committed at `dc711c5`). It is the natural host for the scoped-wording requirement and the forbidden-wording vocabulary.
2. The remediation is spec §8 `THIRD-AC-012` + §10.4 certification wording: the allowed scoped form, and the forbidden phrases ("Tracewake is fully certified", "Latest main was independently verified", later Phase 2+/3A+ certified by this pass, "Archived specs are live authority").
3. Shared boundary under audit: the acceptance-artifact doc ↔ a reusable forbidden-wording check usable against both the template and a produced artifact.
4. Motivating invariants (restated): the spec assigns `INV-091`, `INV-092`, `INV-105` to acceptance-artifact discipline; the operative principle is that scoped historical evidence must not be promoted into project-wide or latest-main certification.
5. Fail-closed doc-validation surface: the forbidden-wording check is a deterministic, blocking gate — a forbidden phrase fails it; the required scoped phrase must be present. It touches only doc text, no actor-knowledge or replay state.

## Architecture Check

1. A forbidden-wording check over the acceptance artifact plus a mandated scoped-wording template makes overclaim a hard failure rather than a review-time catch, and forces the artifact to name the exact commit, the gates run, and residual convention-only items. This is cleaner than relying on reviewer vigilance, which the prior hardening passes show is fallible.
2. No backwards-compatibility shims: the template is amended in place; the check is a new gate, not an alias of an existing one.

## Verification Layers

1. No-overclaim -> test: an artifact containing a forbidden phrase fails the check.
2. Scoped wording present -> test: the required scoped phrase (exact commit, scoped contribution) is asserted present.
3. Doc-template fidelity -> codebase grep-proof: the template carries the exact-commit / gates-run / residual-items sections and the forbidden-wording list.

## What to Change

### 1. Amend the acceptance-artifact template

Update `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` to mandate the scoped-wording form, the exact-commit field, the gates-run-with-output-summaries section, the residual-convention-only-items section, and an explicit forbidden-wording list.

### 2. Add a reusable forbidden-wording check

Add a check that scans an acceptance artifact (and the template) and fails on any forbidden overclaim phrase while requiring the scoped phrase.

## Files to Touch

- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (modify — scoped-wording requirement, required sections, forbidden-wording list)
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` (new) — forbidden-wording / required-scoped-phrase check over the template and a produced artifact

## Out of Scope

- The CI lock that runs the strengthened gates and the §10 acceptance run (ticket 012).
- The gate implementations themselves (their own tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test acceptance_artifact_wording` — a forbidden phrase fails; the scoped phrase is required.
2. `cargo test --workspace` — full pipeline green.
3. `grep -qF 'accepted for exact commit' docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — scoped-wording requirement present in the template.

### Invariants

1. The acceptance artifact names the exact commit and uses scoped wording only.
2. No forbidden overclaim phrase ("fully certified", "latest main verified", later-phase certified, "archived specs are live authority") can pass the check.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_artifact_wording.rs` — forbidden-wording + required-scoped-phrase check; rationale: make overclaim a hard failure rather than a review-time catch.

### Commands

1. `cargo test -p tracewake-core --test acceptance_artifact_wording`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
