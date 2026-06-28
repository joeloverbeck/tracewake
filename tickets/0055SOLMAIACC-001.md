# 0055SOLMAIACC-001: Amend architecture tier 13 — define solo-maintainer acceptance mode

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — doc-only amendment to `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
**Deps**: None

## Problem

The standing governance barrier and the CI governance audit require an independent human approving review on every merge to `main`, which a solo maintainer cannot satisfy (GitHub forbids a PR author from approving their own PR). Architecture tier 13 owns the acceptance-artifact contract: it says a `pass` claim requires "independent acceptance where required" and treats automated checks and source guards as "topology alarms" that cannot substitute for it — but it does not define what happens when the repository is genuinely single-maintainer.

This ticket amends `docs/1-architecture/13` (spec 0055 Deliverable 1) to define a **solo-maintainer acceptance mode** in which the independent-acceptor role for *routine* merges is fulfilled by a named compensating-control set under a non-bypassable ruleset, rather than a second human — while preserving the strong second-human requirement exactly where the doctrine already scopes it (foundational-conformance acceptance artifacts) and leaving behavioral-evidence independence untouched. Per spec 0055's Authority-table reconciliation, the amendment is framed as a **scoping clarification plus compensating-control substitution** (net acceptance strength preserved), not a net weakening, so it stays a "narrow change under live doctrine."

**Execution precondition**: this is a doctrine-tier amendment *proposal*. It must not be applied by convention — it requires the owning process's ratification (ordinary owner approval for the architecture tier, per `docs/README.md`), including the acceptance-weakening-vs-scoping-clarification determination spec 0055's reassessment surfaced. Authoring this ticket does not authorize landing it.

## Assumption Reassessment (2026-06-28)

1. The acceptance-artifact contract in `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` carries the exact scoping phrase the amendment leans on — `independent acceptance where required` is present (one match), inside the "Acceptance artifacts are read models over current evidence" paragraph, backed by "ruleset or approval evidence." The invalid-pass list carries `required governance control … open, pending, unbounded, or merely historical` (the "Invalid pass conditions" section). Both must be preserved by the amendment.
2. Spec `specs/0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md` Deliverable 1 and its Authority-table reconciliation govern this ticket; `docs/README.md:51` classifies "acceptance weakening" as a `4-specs/` *may-not-define* change (alongside constitutional amendments / architecture replacement) and `docs/README.md:48` forbids `1-architecture/` from defining "weakened doctrine" — the amendment must therefore read as a scoping clarification + compensating-control substitution, not a net weakening, or it is impermissible at this tier.
3. Cross-artifact shared boundary under audit: the acceptance-artifact independence contract in arch13 is the higher-tier source that exec tier 10's fail-closed acceptance manifest (ticket 002) and the CI governance audit (ticket 003) enact. The compensating-control set named here is the single definition those tickets reference; it must be authored once, here, consistently.
4. INV-098 — Feature acceptance is harsh — motivates the careful scoping. Restated: a runnable feature is done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, and regression-tested. None of these clauses is "human-approved merge"; verified that no `INV-NNN` in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (INV-001…INV-112) mandates a human approving review. The amendment changes only the merge-time human-approval dimension, not the behavioral evidentiary bar — so no constitutional amendment is required and INV-098 is untouched.
5. Enforcement surface governed by this doctrine: the acceptance-artifact independence requirement (consumed by the fail-closed acceptance manifest and the CI governance audit). Confirm the change introduces no epistemic-leakage path (it does not touch actor-knowledge filtering, view-model projection, or possession) and no nondeterminism path (it does not touch replay, hashing, or serialization). Behavioral-evidence independence — self-authored-only evidence for mutation proof and typed path-under-test claims — is explicitly preserved as invalid; the amendment touches only the human-approval dimension.

## Architecture Check

1. Amending the higher-authority architecture tier first (before exec tier 10 and the CI predicate) keeps the doctrine cascade honest: exec10 and `ci.yml` enact what arch13 defines, so arch13 must carry the authoritative definition of the mode and its compensating-control set. Defining the mode as an explicit scoping of the existing "where required" language — with a compensating control that keeps merge-governance acceptance positively enforced — is cleaner than removing the approval requirement outright (which would leave the docs asserting a posture the repo no longer enforces) and cleaner than adding a second bot/account identity (acceptance theater for a genuinely solo repo).
2. No backwards-compatibility shims or alias paths: the amendment adds a named, reversible operating mode with explicit preconditions, not a fallback wrapper around the old requirement. The second-human bar remains the literal default for multi-maintainer operation and for foundational-conformance artifacts.

## Verification Layers

1. INV-098 (behavioral acceptance bar) → invariants alignment check: confirm the amended arch13 leaves every behavioral acceptance clause unchanged and adds no behavioral exception.
2. Acceptance-independence doctrine integrity → manual review + codebase grep-proof: the new "solo-maintainer acceptance mode" text lands, and the invalid-pass condition "required governance control … open, pending, unbounded, or merely historical" is retained verbatim.
3. Cross-artifact consistency (arch13 → exec10 → ci.yml) → grep-proof: the compensating-control set named in arch13 (full required-status-check barrier; no bypass actors and `current_user_can_bypass: never`; non-fast-forward and deletion protection; up-to-date branch) is the same set tickets 002 and 003 reference, so no tier enacts a control set the architecture tier did not define.

## What to Change

### 1. Define the solo-maintainer acceptance mode in the acceptance-artifact contract

In the acceptance-artifact contract region (around "independent acceptance where required" / "Acceptance artifacts are read models over current evidence"), add doctrine establishing that when the repository is operated by a single maintainer, the independent-acceptor role for *routine* merges is fulfilled by the standing automated barrier under the named compensating-control set, and an acceptance artifact's independence requirement is satisfied by that posture rather than an approving review. State it explicitly as a scoping of the existing "where required" language, not a new weaker rule.

### 2. Name the compensating-control set; preserve the strong bar where it already applies

Define the compensating-control set: the full required-status-check barrier green (the 8 standing checks, including `mutation in-diff (lock layer)`); active enforcement with no bypass actors and `current_user_can_bypass: never`; non-fast-forward and deletion protection; and `strict_required_status_checks_policy` (up-to-date branch). Keep "required governance control … open, pending, unbounded, or merely historical" as invalid-pass conditions. Keep the second-human requirement in force for foundational-conformance acceptance artifacts and for any multi-maintainer operation.

### 3. Record the 0054 lineage note (Deliverable 6, distributed)

Add a sentence recording that this amendment re-specifies the *standing* independent-acceptance posture that spec 0054 hardened under a second-human interpretation; 0054's archived acceptance evidence remains valid for its exact implementation commit (`24a458243b…`, which the `docs/4-specs/SPEC_LEDGER.md` 0054 row already scopes it to), only the forward-looking standing posture changes.

## Files to Touch

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- The execution-tier amendment to `docs/2-execution/10` (ticket 002), the CI governance-audit predicate (ticket 003), and the ruleset relaxation (ticket 004).
- The `docs/4-specs/SPEC_LEDGER.md` 0055 package row — added at spec acceptance/closeout, not at proposal time (deferred follow-up).
- Editing the 0054 ledger row — this ticket records the lineage relationship; it does not alter 0054's commit-scoped acceptance evidence.
- Behavioral-evidence independence rules (mutation proof, typed path-under-test) — preserved unchanged.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "solo-maintainer" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` returns the new acceptance-mode doctrine.
2. `grep -n "open, pending, unbounded, or merely historical" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` still returns the retained invalid-pass condition (the amendment did not remove it).
3. `grep -niE "foundational-conformance|multi-maintainer" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` shows the strong second-human bar is retained where the doctrine scopes it.

### Invariants

1. The amended arch13 leaves every INV-098 behavioral acceptance clause unchanged — no behavioral exception is introduced; only the merge-time human-approval dimension is scoped.
2. The amendment introduces no constitutional-tier change and carries the owner-ratification execution precondition (it must not be applied by convention).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "solo-maintainer|compensating-control" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
2. `grep -n "open, pending, unbounded, or merely historical" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
3. A narrower command is correct here: this is a doc-only doctrine amendment with no Rust surface, so verification is grep-based landing/retention checks plus an invariants-alignment review (INV-098 unchanged), not a `cargo` build/test.
