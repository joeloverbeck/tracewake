# 0016PHA3ANEEACC-013: Conformance-index rows, overturned-claims record, exec-06 stuck clauses

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation-only (architecture conformance index + execution-tier clarification)
**Deps**: `archive/tickets/0016PHA3ANEEACC-003.md`, `archive/tickets/0016PHA3ANEEACC-008.md`, `archive/tickets/0016PHA3ANEEACC-012.md`

## Problem

Spec 0016 §6 (documentation corrections, same package) and §5 item 6 (conformance index update): once the implementation tickets land, the doc pack must record (a) new conformance rows for the surfaces 0016 hardened, (b) the fact that two of 0015's verified-holding/outcome claims were overturned — audits must not silently rewrite prior audit history — and (c) the cross-tick stuck-detection categories as an execution-tier clarification (verified absent from exec doc 06's clause list). None of this is a doctrine amendment.

## Assumption Reassessment (2026-06-10)

1. Current docs verified: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` carries the "Phase 3A evented cognition conformance" table (§ at ~line 74) with rows for workplace/sleep/food/route knowledge, audit enforcement, completion continuity, and tuning boundary — no rows for need-tick accounting authority, duration terminal-set, context-hash re-derivation gate, cognition-substrate source, embodied workplace availability source, or workspace census. `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` has stuck-diagnostic clauses (lines ~59/78/89/100/111/116) but not the cross-tick categories (no-progress-past-expected-window, repeated-idle) — grep-verified. The overturned 0015 claims are quoted at `archive/specs/0015_PHASE_3A_EVENTED_COGNITION_CHANNELS_AUDIT_ENFORCEMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md:114` ("…do not double-count sleep ticks") and :196–197/:426/:521 (context-hash rebuild lock).
2. Spec/docs: spec 0016 §6 (all three correction bullets), §5 item 6 (the six conformance rows, with the cognition-substrate row carrying the recorded deferral per ticket 008's minimum cut); `docs/README.md` authority order (execution-tier clarification is in-tier, no amendment).
3. Cross-artifact boundary under audit: conformance rows cite implementation surfaces by name (the tick classifier, `is_duration_terminal`, the re-derivation gate, the substrate source + deferral ticket, the embodied availability source, the workspace census) — each row must match the landed symbol names from tickets 001/002/003/007/008/012, which is why `Deps` lists the transitive heads covering all of them.

## Architecture Check

1. Recording the overturned claims in the conformance index keeps the audit chain honest (each hardening pass can see which prior verdicts fell and why) — cheaper and more durable than burying it in the acceptance report alone. The exec-06 clause addition is a clarification of an existing contract (stuck diagnostics), placed in the execution tier where the ORD-LIFE-CERT clause list lives; no foundation/architecture text changes.
2. No backwards-compatibility aliasing/shims: N/A — documentation-only; no doc content is duplicated or aliased (rows link to surfaces, the overturn note cites ORD-HARD-014/016 rather than restating their content).

## Verification Layers

1. Conformance coverage → codebase grep-proof: the six new rows exist in `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` and each row's named symbol resolves in the post-implementation tree.
2. Audit-history honesty → grep-proof: the overturn note names `ORD-HARD-014` and `ORD-HARD-016` and the two overturned 0015 claims.
3. Execution-tier clarification → grep-proof: exec doc 06 lists no-progress-past-expected-window and repeated-idle in its stuck-diagnostic clauses.

## What to Change

### 1. Conformance-index rows

Add rows to the Phase 3A conformance table for: need-tick accounting authority (002), duration terminal-set (001), context-hash re-derivation gate (003), cognition-substrate source + its recorded deferral (008 / `tickets/0016PHA3ANEEACC-015.md`), embodied workplace availability source (007), workspace census (012).

### 2. Overturned-claims record

A note in the conformance index recording that 0015's "passive deltas do not double-count" and "context-hash rebuild" verified-holding/outcome claims were overturned by the 0016 audit, citing `ORD-HARD-014`/`ORD-HARD-016`.

### 3. Exec doc 06 clarification

Add the cross-tick stuck-detection categories (no-progress-past-expected-window, repeated-idle) to the ORD-LIFE-CERT stuck-diagnostic clause list as an execution-tier clarification.

## Files to Touch

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)

## Out of Scope

- The acceptance artifact (0016PHA3ANEEACC-014).
- The `docs/4-specs/SPEC_LEDGER.md` row and the spec's `archive/specs/` move — deferred to spec acceptance/archival per `docs/archival-workflow.md` (cross-spec follow-up, not ticketed).
- Any foundation/architecture doctrine change (clarifications only; no amendment).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proofs: all six conformance rows present with resolving symbol names; overturn note present citing ORD-HARD-014/016; exec-06 lists both cross-tick categories.
2. `cargo test --workspace` remains green (docs-only change; command confirms no accidental tree damage).

### Invariants

1. Doc-tier authority order preserved: execution-tier clarification only; foundation/architecture text unchanged.
2. Prior audit history is corrected by recorded overturn, never silently rewritten.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "need-tick\|terminal-set\|re-derivation\|cognition-substrate\|workspace census" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
2. `grep -n "no-progress\|repeated-idle" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-10

What changed:

- Added six Phase 3A conformance rows for need-tick authority, duration terminal-set, context-hash re-derivation, cognition-substrate source, embodied workplace availability source, and lock-layer workspace census.
- Added an audit-history overturn note for the 0015 passive-delta double-counting and context-hash rebuild claims, citing `ORD-HARD-014` and `ORD-HARD-016`.
- Clarified execution doc 06 with cross-tick stuck-detection categories `no-progress-past-expected-window` and `repeated-idle`.

Deviations:

- None. This was documentation-only; no production code changed.

Verification:

- `grep -n "need-tick\\|terminal-set\\|re-derivation\\|cognition-substrate\\|workspace census" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `grep -n "ORD-HARD-014\\|ORD-HARD-016\\|double-count\\|context-hash rebuild" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `grep -n "no-progress\\|repeated-idle" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `cargo test --workspace`
