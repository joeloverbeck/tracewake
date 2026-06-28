# 0055SOLMAIACC-004: Relax ruleset approval count and verify solo-maintainer acceptance end-to-end

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — operational change to the live GitHub ruleset (rulesets API); no repository file is modified
**Deps**: 0055SOLMAIACC-003

## Problem

After the doctrine amendments (tickets 001, 002) are ratified and the CI governance-audit predicate (ticket 003) accepts a `required_approving_review_count: 0` configuration backed by the compensating-control set, this ticket performs the operational enactment (spec 0055 Deliverable 5): relax the live ruleset `main-standing-conformance-barrier` (id `18200914`) `pull_request` `required_approving_review_count` from `1` to `0` via the GitHub rulesets API, retaining all other rules (the 8 required checks, non-fast-forward, deletion protection, no bypass actors, `current_user_can_bypass: never`, up-to-date branch).

This ticket also serves as the batch's **acceptance capstone**: its verification is spec 0055's §Verification in full — the governance audit passes with count 0 and the compensating-control set proven, fails when any compensating control is removed, and a solo maintainer can merge a PR to `main` with all 8 checks green and no approving review, while self-authored-only behavioral evidence remains invalid.

Because the CI audit aggregates `required_approving_review_count` as the **max across both** the ruleset and `branches/main/protection` (reassessment finding M1), the relaxation must cover whichever surface currently sets a non-zero count: relaxing the ruleset alone leaves the merge blocked if branch protection independently requires an approving review.

**Execution precondition**: this is enactment of not-yet-ratified doctrine and the most consequential, hardest-to-reverse step in the batch. It must follow 001/002 ratification and 003 landing, and must not be applied by convention. It is a reversible operating posture: if a collaborator with write access joins, the count is restored to `1` (the second-human barrier is correct as-is for multi-maintainer operation).

## Assumption Reassessment (2026-06-28)

1. The live ruleset `main-standing-conformance-barrier` (id `18200914`) currently sets `pull_request` `required_approving_review_count: 1`, per spec 0055's operator-verified diagnosis and the `docs/4-specs/SPEC_LEDGER.md` 0054 row ("one required approving review … the standing required status checks"). The exact live state must be re-confirmed against the rulesets API at execution time (`gh api /repos/joeloverbeck/tracewake/rulesets/18200914`) before mutating it, since it is live governance, not a repo artifact.
2. Spec `specs/0055_…_SPEC.md` Deliverable 5 governs this ticket and explicitly orders it after the doc amendments and the CI predicate; depends on ticket 003 because dropping the count to 0 before the audit accepts the compensating-control branch would red the governance audit on every subsequent PR.
3. Cross-artifact shared boundary under audit: the ruleset is the live governance surface that the CI `governance-required-checks-audit` (ticket 003) reads and proves. This ticket changes the surface; ticket 003 changed the reader. Both branch protection and the ruleset are independent approval-count sources the audit aggregates (M1) — both must be confirmed to set 0 (or branch protection to set no review requirement).
4. INV-098 — Feature acceptance is harsh — restated: the behavioral acceptance bar is untouched; this operational change alters only who/what supplies the merge-time independent acceptor. Verified no `INV-NNN` mandates a human approving review, so a count-0 ruleset under the proven compensating controls tensions no constitutional invariant.
5. Enforcement surface consumed (evidence-consumer basis): the CI governance audit is the fail-closed gate that proves the post-relaxation posture. The capstone verification must include the **negative-control check** — in a scratch ruleset, drop a required context or add a bypass actor and confirm the audit reds — to prove the gate is genuinely fail-closed and solo mode cannot degrade into ungoverned merging. No epistemic-leakage or replay-determinism surface is involved (repository governance only).

## Architecture Check

1. Performing the ruleset relaxation as the final, separately-gated step — after the doctrine and the CI predicate land — keeps every intermediate state honest: before this ticket the audit passes via the count-1 branch; only here does the compensating-control branch become load-bearing. Pairing the relaxation with the full §Verification (including the fail-closed negative-control check) in one ticket makes the acceptance evidence inseparable from the enactment, so the posture cannot be relaxed without proving the gate still bites.
2. No backwards-compatibility shims: the change is a single ruleset field value with all other rules retained, and it is reversible by restoring the count to `1` — not a wrapper or alias around the governance barrier.

## Verification Layers

1. Solo-merge capability → operational check: a PR to `main` with all 8 required checks green and no approving review is mergeable.
2. Fail-closed gate (negative control) → scratch-ruleset audit run: dropping a required context or adding a bypass actor reds the `governance-required-checks-audit`.
3. Doctrine ↔ enactment consistency → codebase grep-proof: `docs/1-architecture/13` and `docs/2-execution/10` describe the solo-maintainer mode and the `solo-maintainer-compensating-control` posture (tickets 001/002 landed), and the self-authored-only behavioral-evidence rejection in `docs/2-execution/10` is unchanged.

## What to Change

This is an operational runbook (no repository file is modified), executed only after the execution precondition is met:

### 1. Confirm the live approval-count sources

`gh api /repos/joeloverbeck/tracewake/rulesets/18200914` and `gh api /repos/joeloverbeck/tracewake/branches/main/protection` — record the current `required_approving_review_count` on both the ruleset and (if present) branch protection.

### 2. Relax the count to 0 on whichever source(s) set it

`PUT /repos/joeloverbeck/tracewake/rulesets/18200914` setting `pull_request` `required_approving_review_count: 0`, retaining all other rules. If branch protection independently requires an approving review, relax it there too (or remove the redundant review requirement), so no aggregated source leaves the count ≥ 1.

### 3. Run the §Verification capstone checks

Confirm the governance audit passes with count 0 and the compensating controls proven; run the negative-control check in a scratch ruleset (drop a context / add a bypass actor → audit reds); confirm a solo PR merges with 8 checks green and no approval.

## Files to Touch

- `None — operational ruleset change via the GitHub rulesets API; no repository file is modified. Verification is the live governance-audit run plus a scratch-ruleset negative-control check.`

## Out of Scope

- The arch/exec doctrine amendments (tickets 001, 002) and the CI audit predicate (ticket 003).
- The `docs/4-specs/SPEC_LEDGER.md` 0055 package row and the `archive/specs/` move — deferred to spec acceptance/closeout (Step 6 follow-up).
- Any change to behavioral, mutation, or required-status-check governance — only the approving-review count changes.

## Acceptance Criteria

### Tests That Must Pass

1. The `governance-required-checks-audit` job passes with `required_approving_review_count: 0` on `main` **and** the compensating-control set proven (8 required checks, no bypass actors, `current_user_can_bypass: never`, non-fast-forward, deletion protection, up-to-date branch).
2. **Negative control**: dropping a required context, adding a bypass actor, or otherwise removing a compensating control in a scratch ruleset makes the audit exit non-zero (proving fail-closed).
3. A solo maintainer can merge a PR to `main` with all 8 required checks green and **no** approving review.
4. `grep -nE "solo-maintainer-compensating-control" docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` and `grep -nE "solo-maintainer" docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` both resolve (the mode is documented), and the `self-authored-only evidence` behavioral rejection in `docs/2-execution/10` is unchanged.

### Invariants

1. The standing governance posture remains fail-closed: solo mode passes only on a proven compensating-control set; absence of any control reds the audit.
2. The change is reversible — restoring `required_approving_review_count: 1` reinstates the second-human barrier required for multi-maintainer operation — and tensions no constitutional invariant (INV-098 behavioral bar untouched).

## Test Plan

### New/Modified Tests

1. `None — operational/verification-only ticket; verification is the live governance-audit run plus a scratch-ruleset negative-control check, executed as the runbook above after the execution precondition is met.`

### Commands

1. `gh api /repos/joeloverbeck/tracewake/rulesets/18200914 --jq '.rules[] | select(.type=="pull_request") | .parameters.required_approving_review_count'` — confirm the pre-change count, then the post-`PUT` count reads `0`.
2. After the relaxation, observe the `governance required checks audit` check on a PR to `main` (and the scratch-ruleset negative-control run) to confirm green-on-proof / red-on-missing-control.
3. These are operational `gh api` / live-CI commands and are deliberately **not** dry-run at decomposition: they mutate live repository governance and require repository auth, so they execute only as the gated runbook after 001/002 ratification and 003 landing — there is no safe local dry-run equivalent for a live ruleset mutation.

## Outcome

Completed: 2026-06-28

Executed the operational ruleset enactment after tickets `0055SOLMAIACC-001`,
`0055SOLMAIACC-002`, and `0055SOLMAIACC-003` landed. The active goal request to
implement the named `0055SOLMAIACC` series satisfied the owner-ratification
precondition for the operational change.

Live pre-change evidence:

- `gh api /repos/joeloverbeck/tracewake/rulesets/18200914` showed active
  ruleset `main-standing-conformance-barrier`, no bypass actors,
  `current_user_can_bypass: never`, deletion and non-fast-forward rules,
  strict required status checks with the eight standing contexts, and
  `required_approving_review_count: 1`.
- `gh api /repos/joeloverbeck/tracewake/branches/main/protection` returned
  `Branch not protected (HTTP 404)`, so there was no separate branch-protection
  review-count source to relax.

Applied the live ruleset update with `PUT
/repos/joeloverbeck/tracewake/rulesets/18200914`, changing only the
pull-request rule's `required_approving_review_count` from `1` to `0` while
retaining deletion protection, non-fast-forward protection, no bypass actors,
`current_user_can_bypass: never`, strict required status checks, and the eight
required contexts.

Verification:

- The returned update response showed `required_approving_review_count: 0`,
  no bypass actors, `current_user_can_bypass: never`, deletion and
  non-fast-forward rules, strict status checks, and the same eight required
  contexts.
- The embedded `.github/workflows/ci.yml` governance-audit predicate was run
  locally against the live GitHub API response and printed `Governance required
  checks audit passed.` with `Max required approving review count: 0` and
  `Solo-maintainer compensating-control rulesets:
  ['main-standing-conformance-barrier']`.
- PR `#67` was pushed to head commit
  `eff62acc4588b25e1c3c0bdbb895dcc8f973b75c`; GitHub checks all passed for the
  eight required contexts: `rustfmt`, `clippy`, `build & test`,
  `lock-layer gates`, `public-boundary conformance`,
  `full-surface mutation trigger (lock layer)`, `governance required checks
  audit`, and `mutation in-diff (lock layer)`.
- The scratch negative control created temporary ruleset
  `tracewake-0055-negative-control-scratch` (`18223133`) scoped to a throwaway
  branch pattern with a bypass actor. Running the embedded governance predicate
  against the live rulesets failed with `ruleset
  tracewake-0055-negative-control-scratch has bypass actors`, proving the audit
  reds on missing compensating controls. The scratch ruleset was then deleted,
  and a follow-up ruleset list returned only `main-standing-conformance-barrier`.
- `gh pr view 67 --json mergeable,mergeStateStatus,reviewDecision,isDraft,statusCheckRollup,headRefOid`
  returned `mergeable: MERGEABLE`, `mergeStateStatus: CLEAN`, `isDraft: false`,
  empty `reviewDecision`, and successful required check runs at head
  `eff62acc4588b25e1c3c0bdbb895dcc8f973b75c`, proving the solo maintainer can
  merge the PR with all required checks green and no approving review.

No behavioral-evidence rule was changed. The self-authored-only behavioral
evidence rejection remains documented in execution tier 10 from ticket
`0055SOLMAIACC-002`.
