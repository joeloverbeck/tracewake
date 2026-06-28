# 0055SOLMAIACC-002: Amend execution tier 10 — solo-maintainer exception to the fail-closed acceptance manifest

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — doc-only amendment to `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
**Deps**: 0055SOLMAIACC-001

## Problem

Execution tier 10 owns the **Fail-closed acceptance manifest** doctrine. Its pass rule states that overall `pass` is legal only when "every required governance control is enforced by branch protection or active ruleset and independent acceptance is proven where required," and it lists `zero-approval or status-checks-only independence gaps, self-authored-only evidence, missing actual-artifact ingestion … are not pass.` As written, every satisfying configuration of the independence clause requires a second human — which a solo maintainer cannot supply.

This ticket amends `docs/2-execution/10` (spec 0055 Deliverable 2) to carry an explicit **solo-maintainer exception** to the human-approval dimension of that clause: in solo-maintainer mode the independence requirement is met by the documented compensating-control set (defined in arch tier 13 by ticket 001), and the manifest records `solo-maintainer-compensating-control` as its independent-acceptance posture. The rejection of **self-authored-only evidence for behavioral claims** (mutation proof, typed path-under-test) is preserved unchanged — this amendment touches only the human-approval dimension of independence, not behavioral evidence.

This ticket `Deps` on 001 because the execution tier operationalizes the architecture tier: arch13 must define the solo-maintainer acceptance mode and the compensating-control set before exec10 can reference them without dangling. Landing 002 ahead of 001 would let the execution tier cite a mode the higher-authority tier has not yet established.

**Execution precondition**: doctrine-tier amendment proposal — ordinary owner approval for the execution tier (`docs/README.md`), not applied by convention. Gated behind 001's ratification.

## Assumption Reassessment (2026-06-28)

1. `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` carries the target doctrine: the `## Fail-closed acceptance manifest` section (one match) records "independent-acceptance posture" among the manifest's fields, and the pass rule contains the exact clause `zero-approval or status-checks-only independence gaps, self-authored-only evidence` are not pass. The closed status set (`closed|open|routed-forward|pending-governance|historical-only|not-in-scope`) and the `self-authored-only evidence … require typed path-under-test evidence` wording-guard region are present and must remain intact.
2. Spec `specs/0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md` Deliverable 2 governs this ticket. The new posture token `solo-maintainer-compensating-control` is named in the spec's Verification section and must be added to the manifest's independent-acceptance posture vocabulary; the spec's Authority-table reconciliation frames this as scoping clarification + compensating-control substitution (not a net weakening), keeping it within execution-tier authority.
3. Cross-artifact shared boundary under audit: the fail-closed acceptance manifest's independent-acceptance posture is the execution-tier mirror of arch13's acceptance-artifact independence contract (ticket 001) and is enacted operationally by the CI governance audit (ticket 003). The exception authored here must name exactly the compensating-control set 001 defines, so the three tiers stay consistent.
4. INV-098 — Feature acceptance is harsh — restated: the behavioral acceptance criteria (caused, agent-possible, replay-safe, regression-tested, no-human runnable, etc.) are untouched; the manifest still requires typed path-under-test evidence and live negative/sensitivity proof for behavior, mutation, and typed-path claims. Verified no `INV-NNN` mandates a human approving review. The amendment narrows only the human-approval dimension.
5. Enforcement surface governed: the fail-closed acceptance manifest's pass computation (a deterministic, machine-readable, fail-closed gate whose verdict prose cannot upgrade). Confirm the exception keeps the gate fail-closed — the independence requirement is *met* by a proven control set, not waived — and introduces no epistemic-leakage or nondeterminism path: behavioral-evidence independence, mutation closure, actual-artifact ingestion, and the wording guard all remain in force. The amendment adds one posture value and one scoping exception; it removes none of the manifest's behavioral or governance-enforcement requirements.

## Architecture Check

1. Recording `solo-maintainer-compensating-control` as a named independent-acceptance posture (rather than silently dropping the clause) keeps the manifest's machine-readable verdict honest: the artifact still must *prove* a positive posture, and the audit (ticket 003) can fail-closed if the proven controls are absent. This is cleaner than a status-checks-only pass (which the doctrine explicitly rejects) because the posture is an enforced, ruleset-proven control set, not an absence of governance.
2. No backwards-compatibility shims: the exception is an explicit named posture with preconditions, not a fallback that silently downgrades a missing approval into a pass. The pre-existing posture values and pass conditions are retained for multi-maintainer operation and foundational-conformance artifacts.

## Verification Layers

1. INV-098 (behavioral acceptance bar) → invariants alignment check: the amended manifest still requires typed path-under-test evidence and rejects self-authored-only behavioral evidence; no behavioral clause is relaxed.
2. Fail-closed manifest integrity → manual review + codebase grep-proof: `solo-maintainer-compensating-control` is added to the posture vocabulary, and the `self-authored-only evidence` rejection for behavioral claims is retained verbatim.
3. Cross-artifact consistency (arch13 ↔ exec10 ↔ ci.yml) → grep-proof: the compensating-control set the exception names matches the set defined in arch13 (ticket 001) and parsed by the CI audit (ticket 003).

## What to Change

### 1. Add the solo-maintainer exception to the independence clause

In the Fail-closed acceptance manifest pass rule, qualify the `zero-approval or status-checks-only independence gaps … are not pass` clause with an explicit solo-maintainer exception: in solo-maintainer mode the independence requirement is met by the documented compensating-control set (per arch tier 13), so a `required_approving_review_count: 0` configuration backed by that proven control set is not an "independence gap." A bare zero-approval / status-checks-only configuration *without* the proven compensating controls remains not-pass.

### 2. Add the `solo-maintainer-compensating-control` posture value

Add `solo-maintainer-compensating-control` to the manifest's independent-acceptance posture vocabulary, defined as: the manifest records this posture when the ruleset proves the full compensating-control set and the repository is operated by a single maintainer.

### 3. Preserve behavioral-evidence independence and the wording guard verbatim

Leave unchanged: the rejection of self-authored-only evidence for behavioral claims (mutation proof, typed path-under-test); the requirement for current actual-artifact ingestion and green mutation evidence; the wording guard; and the routed-forward bounding rules. Record (Deliverable 6, distributed) the 0054 lineage note where the manifest's independent-acceptance posture is discussed: the standing posture is re-specified, 0054's commit-scoped evidence is unaffected.

## Files to Touch

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- The architecture-tier amendment (ticket 001 — the authoritative definition of the mode and control set), the CI governance-audit predicate (ticket 003), and the ruleset relaxation (ticket 004).
- The `docs/4-specs/SPEC_LEDGER.md` 0055 package row (deferred to acceptance/closeout).
- Any change to the manifest's behavioral-evidence, mutation-closure, actual-artifact-ingestion, or wording-guard requirements.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "solo-maintainer-compensating-control" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` returns the new posture value in the manifest vocabulary.
2. `grep -n "self-authored-only evidence" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` still returns the retained behavioral-evidence rejection.
3. `grep -niE "solo-maintainer" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` shows the exception is scoped to solo-maintainer mode (not a blanket relaxation).

### Invariants

1. The amended manifest still rejects self-authored-only behavioral evidence and still requires typed path-under-test proof — INV-098 behavioral bar intact.
2. The manifest verdict remains a deterministic, fail-closed computation: solo mode is a *proven* posture, not a waiver; absence of the compensating controls is still not-pass.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "solo-maintainer-compensating-control|solo-maintainer mode" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
2. `grep -nE "self-authored-only evidence|typed path-under-test" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
3. A narrower command is correct: this is a doc-only execution-tier amendment with no Rust surface, so verification is grep-based landing/retention checks plus an invariants-alignment review, not a `cargo` build/test.
