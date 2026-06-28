# 0055SOLMAIACC-003: Enact the CI governance-audit compensating-control predicate

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — CI config: amends the `governance-required-checks-audit` step in `.github/workflows/ci.yml`
**Deps**: 0055SOLMAIACC-001, 0055SOLMAIACC-002

## Problem

The CI `governance-required-checks-audit` job (`.github/workflows/ci.yml`) fails the build at the independent-acceptor check unless `max_required_approvals >= 1` **or** (`require_last_push_approval` **and** a non-empty `required_reviewers`) — both of which require a second human. This ticket amends that predicate (spec 0055 Deliverable 4) so that, when the ruleset proves the documented compensating-control set, the independence constraint is satisfied even with `required_approving_review_count: 0`.

The audit must still **fail-closed** if the compensating controls are absent — a bypass actor appears, a required check is dropped, enforcement lapses, or a control field cannot be proven — so solo-maintainer mode cannot silently degrade into ungoverned merging. The predicate enacts, operationally, the doctrine defined in arch tier 13 (ticket 001) and exec tier 10 (ticket 002); it `Deps` on both because it must not enact a control set the doctrine tiers have not yet ratified.

This ticket amends only the **audit predicate** in `ci.yml`. It does **not** apply the live ruleset relaxation (that is ticket 004) — after this lands, the audit still passes under the current `required_approving_review_count: 1` (count 1 satisfies the existing branch); the new branch only becomes load-bearing once ticket 004 drops the count to 0.

**Execution precondition**: gated behind 001/002 ratification (the doctrine must define the mode before CI enacts it).

## Assumption Reassessment (2026-06-28)

1. The current audit predicate in `.github/workflows/ci.yml` (verified this session): `independent_review = max_required_approvals >= 1` (L293) and `last_push_required_reviewer = require_last_push_approval and bool(required_reviewers)` (L294); failure appended at L295-298. `max_required_approvals` is the **max across both** `branches/main/protection` (L230-232) and each active ruleset (L268-271). The audit already parses `bypass_actors` (→ `bypass_residuals`, L258-261) and `strict_required_status_checks_policy` (L281-283), and enforces the 8-member `required_checks` set (L198-207). It does **not** parse `current_user_can_bypass`, non-fast-forward rules, or deletion rules — confirmed by `grep -cE "non_fast_forward|rule_type == \"deletion\"|current_user_can_bypass" .github/workflows/ci.yml` → 0. So enacting the compensating-control predicate is *new rule/field parsing*, not merely an added satisfying branch.
2. Spec `specs/0055_…_SPEC.md` Deliverable 4 governs this ticket; the compensating-control set it enacts is defined authoritatively in arch tier 13 (ticket 001) and mirrored in exec tier 10's manifest posture (ticket 002). The predicate must name exactly that set so the three tiers stay consistent.
3. Cross-artifact shared boundary under audit: the `governance-required-checks-audit` step is the operational, machine-readable enactment of the arch13 acceptance-artifact independence contract and the exec10 fail-closed manifest. This ticket is where doctrine becomes an executable, fail-closed gate.
4. INV-098 — Feature acceptance is harsh — restated: behavioral acceptance criteria are untouched; this predicate governs merge-time *governance* independence, not behavioral evidence. Verified no `INV-NNN` mandates a human approving review, so relaxing the human-approval branch (under proven compensating controls) tensions no constitutional invariant.
5. Fail-closed validation surface: the governance audit is a deterministic, fail-closed, build-blocking gate. The new compensating-control branch must be a *positive proof* requirement — the independence constraint is satisfied only when the ruleset proves **all** of: the 8 required checks present; no bypass actors **and** `current_user_can_bypass: never`; non-fast-forward protection; deletion protection; and `strict_required_status_checks_policy`. If any is unprovable or absent (including an API/parse failure), the audit must still fail. Confirm the amendment adds no path that passes on missing evidence: an evidence-gathering failure must read as not-proven, never as pass. No epistemic-leakage or replay-determinism surface is touched (this is repo governance, not simulation).

## Architecture Check

1. Adding a compensating-control satisfying branch alongside the existing approval branches — rather than removing the independent-acceptor check — keeps the audit a positive-proof gate: solo mode must *prove* a stronger-than-default control posture, not merely lack an approver. Parsing the three currently-unparsed controls (`current_user_can_bypass`, non-fast-forward, deletion) closes the gap where the spec's named control set exceeds what the audit can currently see, so the "fail if controls absent" guarantee is actually enforceable.
2. No backwards-compatibility shims: the existing approval branches remain (multi-maintainer operation still passes via `required_approving_review_count >= 1`); the new branch is an additional, independently-provable satisfaction path, not a wrapper that defeats the old check.

## Verification Layers

1. INV-098 (behavioral acceptance bar) → invariants alignment check: the predicate change touches only the merge-governance independence branch; no behavioral check (required status checks, mutation lane) is removed.
2. Fail-closed gate integrity → predicate unit test over synthetic JSON: count-0 + full compensating controls ⇒ pass; count-0 + any one control dropped ⇒ fail; count-1 ⇒ still pass (regression).
3. Cross-artifact consistency → codebase grep-proof: the controls the predicate parses (`current_user_can_bypass`, `non_fast_forward`, `deletion`, plus the already-parsed `bypass_actors` / `strict_required_status_checks_policy` / 8 required checks) are exactly the compensating-control set arch13 (001) defines and exec10 (002) names.

## What to Change

### 1. Add a compensating-control satisfying branch to the independent-acceptor check

Extend the `independent_review` / `last_push_required_reviewer` logic so the independence constraint is also satisfied when the ruleset proves the full compensating-control set with `required_approving_review_count: 0`. When neither a human-approval branch nor the compensating-control branch holds, append the existing not-proven failure.

### 2. Parse the three currently-unparsed controls

Add ruleset parsing for: `current_user_can_bypass` (must equal `never`); the `non_fast_forward` rule (present); and the `deletion` rule (present). Treat a missing/false value, or an inability to read it, as not-proven (fail-closed). The already-parsed `bypass_actors` (no bypass actors), `strict_required_status_checks_policy` (up-to-date branch), and the 8-member `required_checks` set remain part of the proven set.

### 3. Keep the audit fail-closed on absent or unprovable controls

Ensure the compensating-control branch passes only on positive proof of every control; a dropped required check, an added bypass actor, a non-`never` `current_user_can_bypass`, a missing non-fast-forward/deletion rule, lapsed enforcement, or an API/parse failure must still red the audit.

## Files to Touch

- `.github/workflows/ci.yml` (modify)

## Out of Scope

- The arch/exec doctrine amendments (tickets 001, 002) that define the control set this predicate enacts.
- The live ruleset relaxation `required_approving_review_count` 1→0 (ticket 004) — this ticket changes the audit logic, not the ruleset.
- Any behavioral / mutation / required-status-check membership change.

## Acceptance Criteria

### Tests That Must Pass

1. **Positive**: with synthetic ruleset JSON setting `required_approving_review_count: 0`, no bypass actors, `current_user_can_bypass: never`, the 8 required checks present, non-fast-forward + deletion rules present, and `strict_required_status_checks_policy: true`, the predicate prints "Governance required checks audit passed." and exits 0.
2. **Negative, per compensating-control member** (quantifier-expanded — each dropped individually from the positive fixture must red the audit): (a) drop `current_user_can_bypass: never` (or set to a non-`never` value); (b) remove the `non_fast_forward` rule; (c) remove the `deletion` rule; (d) add a bypass actor; (e) drop one of the 8 required checks; (f) set `strict_required_status_checks_policy: false`. Each case must exit non-zero with a governance-not-proven failure.
3. **Regression**: with `required_approving_review_count: 1` (current production config), the predicate still passes, proving the human-approval branch is unbroken.

### Invariants

1. The audit remains a deterministic, fail-closed, build-blocking gate: it passes only on positive proof of either a human-approval branch or the full compensating-control set; missing/unprovable evidence is never a pass.
2. No constitutional invariant is tensioned — the predicate governs merge governance, not behavioral acceptance; INV-098's behavioral bar is untouched.

## Test Plan

### New/Modified Tests

1. `.github/workflows/ci.yml` — amended `governance-required-checks-audit` predicate (the embedded `python3` step), exercised by the synthetic-JSON cases below.
2. A reviewer-run predicate harness (extract the embedded `python3` block, feed crafted `PROTECTION_JSON` / `RULESETS_JSON` env values for the positive + six negative + regression cases). No new committed test file is required; the predicate is self-contained.

### Commands

1. `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('workflow yaml ok')"` — workflow syntax sanity (dry-run verified at decomposition: prints `workflow yaml ok`).
2. Extract the audit's `python3` predicate and run it with synthetic `PROTECTION_JSON`/`RULESETS_JSON` for each Acceptance-Criteria case (positive, the six negatives, regression), asserting exit code 0 vs non-zero.
3. The end-to-end live-API behavior (the audit reading the real ruleset via `gh api`) is verified operationally in ticket 004's negative-control check; the predicate-over-synthetic-JSON harness is the correct local verification boundary here, since the live `gh api` calls require repository auth unavailable in local/unit context.

## Outcome

Completed: 2026-06-28

Implemented the CI governance-audit predicate in `.github/workflows/ci.yml`
after tickets `0055SOLMAIACC-001` and `0055SOLMAIACC-002` established the
architecture and execution doctrine for solo-maintainer mode. The embedded audit
now parses `current_user_can_bypass`, `non_fast_forward`, and `deletion` from
active branch rulesets, builds a per-ruleset compensating-control summary, and
accepts the independent-acceptor requirement at approval count `0` only when one
active ruleset positively proves the complete control set.

The existing human-approval branch remains intact: a ruleset with
`required_approving_review_count: 1` still passes the independent-acceptor
predicate even if the solo-mode controls are not all present. Missing or
unproven solo-mode controls do not pass the count-0 branch.

Verification:

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('workflow yaml ok')"` returned `workflow yaml ok`.
- Extracted the actual embedded Python predicate from `.github/workflows/ci.yml`
  and ran synthetic `PROTECTION_JSON` / `RULESETS_JSON` cases:
  - positive count-0 full controls exited `0`;
  - count-0 with `current_user_can_bypass` not `never` exited non-zero;
  - count-0 missing `non_fast_forward` exited non-zero;
  - count-0 missing `deletion` exited non-zero;
  - count-0 with a bypass actor exited non-zero;
  - count-0 missing one required check exited non-zero;
  - count-0 with `strict_required_status_checks_policy: false` exited non-zero;
  - count-1 human-approval regression exited `0`.
- `grep -nE "current_user_can_bypass|non_fast_forward|deletion|solo-maintainer compensating" .github/workflows/ci.yml` returned the new parsing and failure-message lines.

The live API / ruleset mutation path is intentionally left to
`0055SOLMAIACC-004`; this ticket changed and proved only the local predicate.
