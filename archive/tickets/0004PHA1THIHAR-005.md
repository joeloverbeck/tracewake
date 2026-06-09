# 0004PHA1THIHAR-005: Upgrade the invariant-reference linter to a structured coverage linter

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` tests (`doc_invariant_references.rs` structured-coverage upgrade)
**Deps**: None

## Problem

`doc_invariant_references.rs` proves only that no reference is *dangling*: it builds the defined `INV-###` set and fails when a live doc/spec/test cites an undefined invariant. It does not require each finding/requirement/guard to *carry* an invariant tag, cannot detect a valid-but-stale tag, and produces no report of high-risk invariants abandoned by all tests/specs (spec §6 F-012, §8 THIRD-AC-005). The linter must become a structured coverage linter while keeping dangling detection.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, fail-closed-validation ticket. -->

1. In `crates/tracewake-core/tests/doc_invariant_references.rs`: `doc_invariant_references_are_live()` (`:9`), `live_reference_paths()` (`:100`), `defined_invariants()` (`:62`); referenced IDs are recognized inline by scanning `### INV-` / `**INV-` headings (`:67`) and `INV-` byte tokens (`:81`) — there is no `referenced_invariants` function (confirmed during this session's reassessment).
2. The remediation is spec §8 `THIRD-AC-005` + the §9.3 invariant fixtures (`spec_finding_without_invariants_fails_linter`, `spec_requirement_without_invariants_fails_linter`, `undefined_invariant_reference_fails_linter`), reassessed this session.
3. Shared boundary under audit: the machine-detectable metadata fields the linter will require — `Invariants: INV-…` on findings and `Enforces: INV-…` on requirements — are exactly the fields the live spec packages already use (this spec's §6 findings carry `**Invariants:**`, its §8 requirements carry `**Enforces:**`). The linter's scope is the live, non-archived reference set in `live_reference_paths`.
4. Motivating invariants (restated): `INV-092` (deterministic replay is tested — by analogy, doctrine coverage must be machine-checked, not assumed), `INV-102` (cognition inputs require provenance — the doc-level analogue: every finding/requirement must be grounded in a named invariant).
5. Fail-closed validation surface: the linter is a deterministic, blocking doc-validation gate. Confirm it hard-fails on missing/unknown IDs (blocking) while the orphan/staleness report is advisory (reviewer signoff) — warnings distinguished from blockers. It touches no actor-knowledge or replay state, only doc/spec/test text.

## Architecture Check

1. Requiring a machine-detectable invariant field on every live finding/requirement/guard, plus a generated orphan/staleness report, is strictly stronger than dangling-only detection: it catches an ungrounded finding (no tag) and surfaces abandoned high-risk invariants, while leaving the unautomatable "is it the *right* invariant" to a forced reviewer note. Dangling detection is retained, not replaced.
2. No backwards-compatibility shims. To avoid forcing failures on historical specs that predate the convention, the structured-coverage requirement is scoped to live (non-archived) paths; `archive/specs/` history is exempt and documented as such.

## Verification Layers

1. `INV-092` / coverage -> new tests: `spec_finding_without_invariants_fails_linter`, `spec_requirement_without_invariants_fails_linter`.
2. Dangling detection (retained) -> existing test surface: `undefined_invariant_reference_fails_linter`.
3. Orphan/staleness report -> manual-review surface: the generated report of unreferenced high-risk invariants and renamed/removed targets, for reviewer signoff.

## What to Change

### 1. Structured-field coverage check

Detect `Invariants:` (findings) and `Enforces:` (requirements) metadata lines in live spec/doc paths and fail when a finding/requirement/conformance guard declares no invariant ID. Keep failing on unknown IDs.

### 2. Retain dangling detection

Preserve `doc_invariant_references_are_live` behavior for undefined-reference detection.

### 3. Orphan/staleness report

Generate a reviewer-facing report listing unreferenced high-risk invariants and references whose target invariant was renamed/removed; force a review note when a finding cites only broad/unrelated invariants.

## Files to Touch

- `crates/tracewake-core/tests/doc_invariant_references.rs` (modify — structured-coverage upgrade; scope structured checks to live, non-archived paths)

## Out of Scope

- The conformance evidence-kind matrix (ticket 010), which consumes invariant tags but is a separate test surface.
- Semantic correctness of *which* invariant a finding cites (remains reviewer-verified; the linter only forces the note).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — a synthetic finding/requirement with no invariant tag fails the linter; an unknown ID fails; a well-tagged live spec passes.
2. `cargo test --workspace` — the upgraded linter passes against the current live doc/spec/test set.
3. The orphan/staleness report is produced (printed or written) when invoked.

### Invariants

1. Every live finding/requirement/conformance guard declares ≥1 known invariant ID.
2. No live reference cites an undefined invariant.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/doc_invariant_references.rs` — `spec_finding_without_invariants_fails_linter`, `spec_requirement_without_invariants_fails_linter`, retained `undefined_invariant_reference_fails_linter`; rationale: turn dangling-only checking into coverage enforcement plus an advisory orphan report.

### Commands

1. `cargo test -p tracewake-core --test doc_invariant_references`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed on 2026-06-09.

Upgraded `doc_invariant_references.rs` from dangling-reference-only checking to structured live-spec invariant coverage:

- live `specs/*.md` findings headed `### F-...` must carry `**Invariants:**` metadata with at least one known `INV-###`;
- live `specs/*.md` acceptance requirements headed `### THIRD-AC-...` must carry `**Enforces:**` metadata with at least one known `INV-###`;
- unknown invariant IDs in those structured fields fail the linter;
- broad dangling-reference detection remains intact across the existing live reference set;
- an advisory high-risk invariant review report is produced for reviewer signoff.

Scope note: structured coverage is intentionally limited to live spec packages under `specs/`, matching the current machine-readable convention. Historical archive paths remain exempt, and the high-risk orphan report is advisory rather than blocking.

Verified with:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core --test doc_invariant_references`
3. `cargo build --workspace --all-targets --locked`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo test --workspace`
