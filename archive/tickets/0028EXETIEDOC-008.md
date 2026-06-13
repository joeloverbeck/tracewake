# 0028EXETIEDOC-008: Execution proof-method research notes and forbidden misreads

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` (a proof-methodology source-notes section + forbidden misreads). No crate/code, no fixtures.
**Deps**: None (ticket-internal). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D11 (report E10). Execution owns proof methodology, and the 0028 pass is specifically about how proof obligations should be framed (mutation/metamorphic/property-based testing, golden/approval and deterministic-simulation testing, structured observability, the test-oracle problem). `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` already records decisions for event-sourced replay, provenance/data lineage, deterministic simulation, property-based testing, BDI/affordance/social-simulation precedents, the LLM boundary, and TUI/game precedents — but not mutation testing as behavior sensitivity, metamorphic testing as relation-based oracle support, approval/golden testing as useful-but-semantically-limited, structured observability as typed evidence, or the test-oracle problem as the reason evidence status must be honest. This ticket adds a proof-methodology source-notes section + forbidden misreads. Source-note hygiene only — no product doctrine, no implementation design.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` records event-sourced replay, provenance/data lineage, deterministic simulation, property-based testing, BDI/affordance/social-simulation/institution precedents, the LLM boundary, and TUI/game precedents, but carries no notes for mutation testing, metamorphic testing, approval/golden testing's semantic limits, structured observability, or the test-oracle problem (the proof-method sources newly load-bearing for the 0028 execution-evidence doctrine).
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D11 and the source report `reports/execution-tier-alignment-research-report.md` §E10 + §9 (External references). The report's §9 enumerates the external proof-methodology sources (Stryker/mutation, Segura et al. metamorphic survey, the oracle-problem survey, Hypothesis property testing, ZIO shrinking, ApprovalTests, Antithesis deterministic-simulation, OpenTelemetry logs/data-model/events). These sources support D8/D9/D11 and are research support, not Tracewake authority.
3. Shared boundary under audit: execution `13`'s source-notes/forbidden-misreads layer (research provenance for the execution proof methodology) versus the proof obligations themselves in `10`/`04`/`06`/`07` (which this ticket does not author). `13` records why the proof methodology is framed as it is; it creates no gate and no product doctrine.
4. Adjacent contradictions: none. The forbidden-misreads added here (e.g. "mutation coverage is not certification by itself") reinforce, rather than contradict, the anti-vacuity / evidence-honesty standards landed by D8/D9 in `10` (sibling ticket 0028EXETIEDOC-006); they are research-provenance notes for those standards, not a competing rule.

## Architecture Check

1. The notes live in `13` because `13` is the execution research-decisions / source-notes / forbidden-misreads layer; recording proof-method provenance there keeps the methodology auditable without putting research citations into the normative proof docs (`10` etc.). This mirrors how `13` already records event-sourcing / property-testing / deterministic-simulation provenance.
2. No backwards-compatibility aliasing/shims: additive source-notes + forbidden-misreads entries; no existing `13` decision is altered, and no normative gate is created.

## Verification Layers

1. Proof-method source-notes completeness → codebase grep-proof: `13` cites mutation testing, metamorphic testing, property-based testing (incl. shrinking), approval/golden testing, deterministic-simulation testing, structured observability, and the test-oracle problem.
2. Forbidden misreads → manual review: `13` records that mutation coverage is not certification by itself; a surviving mutant is not harmless without disposition; a golden's byte stability is not semantic truth; deterministic replay is reproducibility, not correctness; structured-log existence is not behavior evidence; observer-only emergence evidence is not a pass/fail gate.
3. Non-authority boundary → manual review: the external sources are recorded as research support, not Tracewake authority, and create no product doctrine or implementation design.

## What to Change

### 1. `13` — proof-methodology source-notes section (D11)

Add a source-notes section citing mutation testing, metamorphic testing, property-based testing (including shrinking), approval/golden testing, deterministic-simulation testing, OpenTelemetry-style structured observability, and the test-oracle problem (report §9), recorded as research support, not Tracewake authority.

### 2. `13` — forbidden misreads (D11)

Add forbidden misreads: mutation coverage is not certification by itself; a surviving mutant is not harmless without disposition; a golden's byte stability is not semantic truth; deterministic replay is reproducibility, not correctness; structured-log existence is not behavior evidence; observer-only emergence evidence is not a pass/fail gate.

## Files to Touch

- `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` (modify)

## Out of Scope

- **The proof obligations these sources support** (anti-vacuity / evidence honesty in `10`; provenance/freshness/accounting in `04`/`06`/`07`) — sibling tickets 0028EXETIEDOC-003/004/005/006; `13` records provenance only.
- **Treating any external source as Tracewake authority** — they are research support; `13` creates no product doctrine.
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Source-notes landing grep** — `13` cites the new proof-method sources: `grep -niE "mutation testing|metamorphic|approval|golden|deterministic.simulation|observability|oracle" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` resolves the methodology section.
2. **Forbidden-misreads landing grep** — `13` records the misreads: `grep -niE "not certification|surviving mutant|byte stability|reproducibility, not correctness|not behavior evidence|not a pass/fail" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` resolves the forbidden-misreads list.
3. **Non-authority review** — the external sources are recorded as research support, not Tracewake authority; no gate or product doctrine is created.

### Invariants

1. `13` records the proof-method provenance and forbidden misreads without creating any normative gate or product doctrine.
2. The forbidden misreads reinforce the D8/D9 anti-vacuity / evidence-honesty standards (ticket 006); they introduce no competing rule.

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution research-notes ticket; verification is command-based (source-notes + forbidden-misreads landing greps) plus a non-authority manual review. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "mutation testing|metamorphic|approval|deterministic.simulation|observability|oracle" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — confirms the proof-method source notes landed.
2. `grep -niE "not certification|surviving mutant|byte stability|reproducibility, not correctness|not a pass/fail" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — confirms the forbidden misreads.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the non-authority manual review.`

## Outcome

Completed: 2026-06-13

Owner approval precondition: satisfied by the user's active `$ticket-series`
goal to implement `tickets/0028EXETIEDOC*` against
`specs/0028_EXECUTION*`.

Changed:

- Added a proof-methodology source-notes section to
  `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
  covering mutation testing, metamorphic testing, property-based testing with
  shrinking, approval/golden testing, deterministic-simulation testing,
  structured observability, and the test-oracle problem.
- Added forbidden misreads stating that mutation coverage is not certification,
  surviving mutants need disposition, byte-stable goldens are not semantic
  truth, deterministic replay is reproducibility rather than correctness,
  structured-log existence is not behavior evidence, and observer-only
  emergence evidence is not a pass/fail gate.

Verification:

- `grep -niE "mutation testing|metamorphic|approval|golden|deterministic.simulation|observability|oracle" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
  resolved the proof-method source-notes section.
- `grep -niE "not certification|surviving mutant|byte stability|reproducibility, not correctness|not behavior evidence|not a pass/fail" docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
  resolved the forbidden-misreads language.
- Non-authority review: the section records research support only and creates no
  new gate, product doctrine, or implementation design.

Deviations:

- None. The Rust gate pipeline was not run for this ticket because the accepted
  verification boundary is documentation grep plus non-authority review.
