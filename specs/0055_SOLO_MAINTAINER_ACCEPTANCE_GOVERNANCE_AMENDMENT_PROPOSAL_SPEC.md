# 0055 — Solo-Maintainer Acceptance Governance Amendment (Proposal)

**Status**: PROPOSED

**Classification**: dev-process/docs — this spec changes no simulation runtime
behavior. It proposes amendments to architecture- and execution-tier governance
doctrine plus their CI/ruleset enactment. It retains an Invariants Alignment
section because it operationalizes the acceptance-related invariants.

> Structure note: this proposal uses the canonical `specs/` section set
> (Problem Statement, Approach, Deliverables, Invariants Alignment, Verification,
> Out of Scope, Risks & Open Questions) rather than the archived hardening
> lineage's narrative structure, because it is a doctrine-amendment *proposal*,
> not a hardening implementation. Per `docs/4-specs/SPEC_LEDGER.md`, a spec may
> not itself amend architecture/execution doctrine: this spec **proposes** the
> amendment's substance and home and stages it for the owning process to enact
> under that process's authority. It authors no ratified wording and mints no
> doctrine identifier. The ledger row is added at acceptance/closeout, not at
> proposal.

## Brainstorm Context

- **Original request**: diagnose why the `governance required checks audit
  (pull_request)` CI job is failing on PR #67, and resolve the related merge
  block "At least 1 approving review is required by reviewers with write
  access," which a solo developer cannot satisfy by self-approving their own PR.
- **Diagnosis (operator-verified, 2026-06-28)**: two independent problems.
  - **O1 (resolved inline, not part of this spec)**: the live ruleset
    `main-standing-conformance-barrier` (id `18200914`) was missing the
    `mutation in-diff (lock layer)` required status check that the CI audit
    (`.github/workflows/ci.yml:198-207`) and `docs/2-execution/10_…md:386-388`
    require. The missing context was added to the ruleset via the GitHub API;
    the audit job was re-run and is green. This was a drift-correction that
    aligns the ruleset *with* existing doctrine and needed no amendment.
  - **O2 (this spec)**: the merge block is the independent-acceptance doctrine
    meeting the reality of solo maintenance.
- **Decision shaping this spec**: the maintainer chose the "solo-maintainer
  doctrine amendment" path over "add a second reviewer identity" or "leave the
  barrier as-is," on the assumption the repository remains solo-operated.
- **Confidence / assumptions**: ~93% at proposal. Load-bearing assumption: the
  repository remains single-maintainer for the foreseeable future. If
  collaborators join, the current second-human barrier is correct as-is and this
  amendment should not be enacted (or should be reverted) — see Risks.

## Problem Statement

The standing governance barrier requires an independent human approving review
on every merge to `main`:

- The ruleset `main-standing-conformance-barrier` sets `pull_request`
  `required_approving_review_count: 1`, with `required_reviewers: []` and
  `require_last_push_approval: false`.
- The CI governance audit (`.github/workflows/ci.yml:293-298`) fails the build
  unless `required_approving_review_count >= 1` **or**
  (`require_last_push_approval` **and** a non-empty `required_reviewers`). Every
  satisfying branch requires a *second human*.
- Execution doctrine `docs/2-execution/10_…md:363-365` states that
  "zero-approval or status-checks-only independence gaps, self-authored-only
  evidence … are not pass."
- Architecture doctrine `docs/1-architecture/13_…md:127-134` requires
  "independent acceptance where required," backed by "ruleset or approval
  evidence," and treats automated checks and source guards as "topology alarms"
  that cannot substitute for it. Spec `0054` deliberately hardened this
  "independent-acceptance governance."

The structural consequence: **there is no ruleset configuration that both passes
the governance audit and lets a solo author merge their own PR.** GitHub forbids
a PR author from approving their own PR, and the audit/doctrine forbid
zero-approval and status-checks-only independence. The solo maintainer is
permanently merge-blocked, and the doctrine assumes a second human who does not
exist for this repository.

Considered and rejected:

- **Add a second reviewer identity (bot/second account)** — preserves the
  doctrine literally but is acceptance theater for a genuinely solo repo and
  adds an identity to maintain. Rejected by the maintainer.
- **Remove the approval requirement without amending doctrine** — flips the
  audit from O1's failure to an "independent acceptor constraint not proven"
  failure, and leaves `docs/**` asserting a posture the repo no longer enforces.
  Rejected as undocumented divergence.

## Approach

Define an explicit, documented **solo-maintainer acceptance mode** in which the
independent-acceptor role on *routine* merges is fulfilled by a hardened
automated barrier plus a non-bypassable ruleset, rather than a second human —
while preserving the strongest independence requirements exactly where the
doctrine says they matter (foundational-conformance acceptance artifacts).

The key reconciliation: the doctrine already scopes the strong requirement with
the phrase "independent acceptance **where required**" and ties it to
foundational-conformance acceptance artifacts. This amendment makes that scoping
explicit and supplies a compensating-control regime for the routine-merge case,
so that:

- behavioral-evidence independence is **untouched** — self-authored-only
  evidence remains invalid for behavior, mutation, and typed-path-under-test
  claims; and
- merge-governance independence for routine changes is satisfied by a defined
  control set instead of an approving review.

No constitutional (foundation-tier) amendment is required: no `INV-NNN` mandates
a second-human approving review. The amendment is architecture- and
execution-tier doctrine plus its CI/ruleset enactment, so it needs no
constitutional sign-off. Its approval bar is the owning process's to set: this
proposal treats it as a narrow change under live doctrine (see Authority-table
reconciliation below), but if the owning process judges it a net acceptance
weakening rather than a scoping clarification, the heavier spec-package ceremony
applies.

### Authority-table reconciliation

The `docs/README.md` authority-order table classifies **"acceptance weakening"**
as a change `4-specs/` *may not define* (the same column as *constitutional
amendments* and *architecture replacement*), and forbids `1-architecture/` from
defining *"weakened doctrine"* and `2-execution/` from defining *"new doctrine
that weakens foundation or architecture."* Taken at face value, no tier may
*define* an acceptance weakening — so the amendment's permissibility turns on
whether it is one.

This proposal's position is that it is **not** a net weakening but a **scoping
clarification plus compensating-control substitution**, which is a "narrow change
under live doctrine":

- The strong independent-acceptance bar already scopes itself to
  *foundational-conformance* acceptance artifacts (`docs/2-execution/10`'s
  fail-closed acceptance manifest governs "foundational conformance remediation
  artifacts"; `docs/1-architecture/13` says "independent acceptance **where
  required**"). The CI governance audit, however, applies the independent-acceptor
  check to **every** merge to `main`, exceeding the doctrine's own scope. Aligning
  the audit to that scope is clarification, not weakening.
- For the routine-merge case the audit thereby releases, the amendment does not
  drop the bar to nothing: it substitutes a named, ruleset-proven
  compensating-control set (Deliverable 3) for the human approver, so the
  merge-governance acceptance posture remains positively enforced.
- The strong second-human bar is **retained unchanged** for
  foundational-conformance acceptance artifacts and for any multi-maintainer
  operation, and behavioral-evidence independence is untouched.

The owning process ratifies whether this reconciliation holds; if it concludes the
substitution is a net weakening, the amendment must carry the corresponding
higher-ceremony sign-off rather than ordinary owner approval.

## Deliverables

This spec proposes the following substance and homes. It does **not** author
ratified wording; the owning process enacts each item.

1. **Architecture tier — `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.**
   Amend the "independent acceptance where required" doctrine (around the
   acceptance-artifact contract, ~L114-134, and the "Invalid pass conditions"
   list, ~L191-216) to define a **solo-maintainer acceptance mode**: when the
   repository is operated by a single maintainer, the independent-acceptor role
   for routine merges is fulfilled by the standing automated barrier under a
   named compensating-control set (below), and an acceptance artifact's
   independence requirement is satisfied by that posture rather than an approving
   review. The amendment must (a) keep "required governance control … open,
   pending, unbounded, or merely historical" as invalid-pass conditions, and
   (b) keep the second-human requirement in force for foundational-conformance
   acceptance artifacts and for any multi-maintainer operation.

2. **Execution tier — `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.**
   Amend the Fail-closed acceptance manifest doctrine (~L342-388) so the
   "zero-approval or status-checks-only independence gaps … are not pass" clause
   carries an explicit solo-maintainer exception: in solo-maintainer mode the
   independence requirement is met by the documented compensating-control set,
   and the manifest records `solo-maintainer-compensating-control` as its
   independent-acceptance posture. The rejection of **self-authored-only
   evidence for behavioral claims** (mutation proof, typed path-under-test) is
   preserved unchanged — this amendment touches only the human-approval
   dimension of independence, not behavioral evidence.

3. **Compensating-control set (defined by the doctrine amendments above).**
   Solo-maintainer mode is legitimate only while the ruleset proves all of:
   the full required-status-check barrier green (the 8 checks, including
   `mutation in-diff (lock layer)`); active enforcement with **no bypass actors**
   and `current_user_can_bypass: never`; non-fast-forward and deletion
   protection; and `strict_required_status_checks_policy` (up-to-date branch).
   The exact control list is the owning process's to ratify; this spec proposes
   the substance.

4. **CI enactment — `.github/workflows/ci.yml` governance audit.**
   Amend the independent-acceptor check (~L293-298) so that, when the ruleset
   proves the compensating-control set, the independence constraint is satisfied
   even with `required_approving_review_count: 0`. The audit must still fail if
   the compensating controls are absent (e.g. a bypass actor appears, a required
   check is dropped, or enforcement lapses), so solo mode cannot silently
   degrade into ungoverned merging. Note that the current audit does **not** yet
   parse every named compensating control: it checks `bypass_actors`
   (`ci.yml:258-261`) and `strict_required_status_checks_policy`
   (`ci.yml:281-283`), but **not** `current_user_can_bypass`, non-fast-forward
   rules, or deletion-protection rules. Enacting the predicate therefore adds new
   rule/field-parsing paths, not merely a new satisfying branch — otherwise the
   "fail if the compensating controls are absent" guarantee is unenforceable for
   the controls the audit cannot currently see. The exact predicate is the
   enacting step's to author.

5. **Ruleset enactment — `main-standing-conformance-barrier` (id `18200914`).**
   After the doc amendments are ratified, relax `pull_request`
   `required_approving_review_count` from `1` to `0`, applied via the GitHub
   rulesets API (`PUT /repos/joeloverbeck/tracewake/rulesets/18200914`). All
   other rules are retained: the 8 required checks, non-fast-forward, deletion
   protection, no bypass actors. Because the governance audit aggregates
   `required_approving_review_count` as the **max across both** the ruleset
   (`ci.yml:268-271`) *and* `branches/main/protection` (`ci.yml:230-232`), the
   relaxation must cover whichever surface currently sets a non-zero count:
   relaxing the ruleset alone leaves the solo maintainer merge-blocked if branch
   protection independently requires an approving review. **Not applied by this
   spec** — this is enactment of not-yet-ratified doctrine and must follow the
   amendments.

6. **Lineage note — spec `0054`.**
   Record that this amendment re-specifies the *standing* independent-acceptance
   posture that `0054` hardened under a second-human interpretation. `0054`'s
   archived acceptance evidence remains valid for its exact implementation
   commit; only the forward-looking standing posture changes. Specifically, the
   `docs/4-specs/SPEC_LEDGER.md` 0054 row cites "one required approving review" as
   confirmed governance; after Deliverable 5 relaxes that to `0`, the clause
   remains valid only as commit-scoped history (commit `24a458243b…`, which the
   ledger already scopes 0054 to) — not as a current statement about `main`. The
   lineage note must say so, so a future reader does not read the ledger row as
   currently true.

7. **Ledger — `docs/4-specs/SPEC_LEDGER.md`.**
   Add the package row at acceptance/closeout per the repo's ledger-timing
   convention (specs staged in `specs/` are recorded at acceptance, not
   proposal). No row is added at proposal time.

## Invariants Alignment

| Invariant | Stance | Rationale |
|---|---|---|
| (no `INV-NNN` mandates human approving review) | N/A | The independent-acceptance requirement lives in architecture/execution doctrine and the CI audit, not the constitution; no constitutional amendment is needed at the governance/CI surface. |
| INV-098 — Feature acceptance is harsh | aligns | Behavioral acceptance criteria (caused, agent-possible, replay-safe, regression-tested, no-human runnable) are untouched; the amendment changes only who/what supplies the merge-time *independent acceptor*, not the evidentiary bar for behavior. |
| INV-077 — LLM-disabled operation is an acceptance gate | N/A | The amendment does not alter the LLM-disabled gate or any LLM boundary at the governance/CI surface. |

The amendment tensions no invariant. It narrows a non-constitutional doctrine
(human independent acceptance for routine merges) while preserving every
behavioral-evidence and governance-enforcement requirement the invariants and
the acceptance doctrine actually protect.

## Verification

- After enactment, the `governance required checks audit` job passes with
  `required_approving_review_count: 0` **and** the compensating-control set
  proven, and **fails** when any compensating control is removed (verified by a
  negative check: drop a required context or add a bypass actor in a scratch
  ruleset and confirm the audit reds).
- A solo maintainer can merge a PR to `main` with all 8 required checks green
  and no approving review.
- `docs/1-architecture/13` and `docs/2-execution/10` describe the
  solo-maintainer mode and its compensating controls, and the manifest's
  independent-acceptance posture vocabulary includes
  `solo-maintainer-compensating-control`.
- Self-authored-only behavioral evidence (mutation, typed path-under-test)
  remains invalid — confirmed unchanged in `docs/2-execution/10`.

## Out of Scope

- O1 (the `mutation in-diff (lock layer)` ruleset drift) — already resolved
  inline and verified; not part of this spec.
- Any constitutional (foundation-tier) amendment — none is required.
- Multi-maintainer governance — unchanged; the second-human barrier remains
  correct when more than one maintainer operates the repo.
- The behavioral-evidence independence rules (mutation proof, typed
  path-under-test) — preserved unchanged.

## Risks & Open Questions

1. **Residual risk of self-acceptance.** Relying on the automated barrier for
   routine merges means a logic error that passes all 8 checks merges without
   independent human judgment — the exact failure the original doctrine guarded
   against. The compensating controls (full check barrier, no bypass,
   non-fast-forward, up-to-date) mitigate but do not fully replace a second
   reviewer. This is the deliberate cost of solo operation and must be stated
   plainly in the amended doctrine.
2. **Reversibility on collaborator onboarding.** Solo-maintainer mode should be
   a named, reversible operating posture. Open question for the enacting
   process: should the audit *require* second-human approval automatically once
   the repo has >1 maintainer/collaborator with write access (auto-exit solo
   mode), or is exiting solo mode a manual doctrine action?
3. **Scope of the routine-vs-conformance split.** Open question: should
   foundational-conformance acceptance artifacts retain a *stricter* independence
   requirement even in solo mode (e.g. a structurally-independent acceptance
   manifest as the acceptor-of-record), or does the compensating-control set
   cover them too? The proposal recommends retaining the stricter bar for
   conformance artifacts; the enacting process ratifies the boundary.
4. **Audit predicate fragility.** The amended audit must fail-closed if it
   cannot prove the compensating controls (API error, ruleset rename, missing
   field), so an evidence-gathering failure never reads as a pass.
