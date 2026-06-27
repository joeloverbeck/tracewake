# 0053FOUCONFIF-003: Merge-enforced standing barrier — branch-protection/ruleset governance audit job

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `.github/workflows/ci.yml` (new governance audit job); `crates/tracewake-core/tests/ci_workflow_guards.rs` (topology alarm extension)
**Deps**: 0053FOUCONFIF-001

## Problem

Spec 0053 §4.4 (F5-04): the named CI jobs (`public-boundary-conformance`, `full-surface-mutation-trigger`, `mutants-lock-layer-reconcile`) exist in `.github/workflows/ci.yml` and `ci_workflow_guards.rs` asserts the workflow topology, but the 0052 acceptance recorded `Branch not protected (HTTP 404)` for `main`. A workflow job definition is **not** an enforced standing gate unless repository settings require it before merge; the recurring seam is specifically an enforcement failure (§3, R-29 decorative locks). This ticket adds the machine-checkable governance audit so the gap cannot be silently re-described as closed; the *operational* branch-protection enablement (§9 step 2) is a governance-owner action (§10.5) recorded as capstone evidence, not a code diff.

## Assumption Reassessment (2026-06-26)

1. `.github/workflows/ci.yml` defines the three named jobs (verified this session at lines 103, 126, 366). `crates/tracewake-core/tests/ci_workflow_guards.rs` (29 KB) asserts the job topology and path filters. Ticket 001 adds a required-check policy assertion to the same `ci_workflow_guards.rs`; this ticket lands after it (`Deps: 001`), so the two edits to that file stack in order (sequential coordination surface, not a parallel merge hub).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §4.4 (required settings + anti-regression guard), §9 step 1–2 (audit scaffolding then enablement), §10.5 (governance owner), §8 (capstone records the API transcript). Sibling precedent: 0052 wired the job topology but no governance enforcement, which is the defect this ticket closes the *audit* half of.
3. Cross-artifact boundary under audit: the boundary between the CI workflow (`ci.yml`), the Rust topology alarm (`ci_workflow_guards.rs`), and the GitHub branch-protection/ruleset API. The audit job queries the API; the Rust test remains a labeled topology alarm only — never merge-enforcement proof (§4.4).
4. Motivating invariant: INV-098 (harsh acceptance) and the INV-098-class evidence discipline — a standing gate that is not merge-required is a first-class anti-regression defect (§4.4), tracked as R-29 (guard vacuity / decorative locks).
5. This ticket builds the **governance enforcement-verification** surface (substrate basis): the audit job is the executable check that the public-boundary conformance lane is merge-required. It introduces no actor-knowledge leakage and no replay nondeterminism — it queries repository settings and fails CI when required checks are not configured. It is fail-closed: when the branch-protection/ruleset API is unavailable (e.g. forked PRs, missing token scope) it reports `pending/unverified` and never `pass` (§4.4 anti-regression guard).

## Architecture Check

1. A CI/governance audit job that queries the branch-protection/ruleset API is the only honest proof that the lane is merge-required — a YAML grep or a `ci_workflow_guards.rs` topology assertion proves the job *exists*, not that it *blocks merge*. Pairing the API audit (enforcement proof) with the retained topology alarm (existence proof) is two independent layers; neither alone suffices, matching §4.4's "a screenshot is weaker than an API transcript, and a YAML grep is not enough."
2. No backwards-compatibility aliasing or shims. The audit job is additive to `ci.yml`; the `ci_workflow_guards.rs` change extends the existing topology assertions without a parallel copy.

## Verification Layers

1. INV-098 (harsh acceptance) -> CI/governance audit job: queries the branch-protection/ruleset API and fails if the required status checks (the workspace gates + `public-boundary conformance` + the lock-layer mutation trigger/reconciliation appropriate to changed paths) are not configured on the exact repository/branch; reports `pending/unverified` (never pass) when API permissions are unavailable.
2. Topology integrity (existence, not enforcement) -> `ci_workflow_guards.rs` grep-proof: the audit job and the three named jobs are present with correct path filters; this layer is explicitly a labeled topology alarm, not merge-enforcement proof.
3. Cross-artifact: the audit job's required-check name list matches the `ci.yml` job names and the manifest's branch-protection enforcement-status field consumed by ticket 010's capstone.
4. Operational layer (out of code scope): the actual branch protection / ruleset enablement and its API transcript are governance-owner evidence captured for the capstone (010), not a code deliverable here.

## What to Change

### 1. Governance audit job (`.github/workflows/ci.yml`, modify)

Add a job that queries the branch-protection/ruleset API for `main` (e.g. `gh api repos/:owner/:repo/branches/main/protection` or the rulesets endpoint) and fails if the required status checks are not configured: the workspace gates, `public-boundary conformance`, and the lock-layer mutation trigger/reconciliation appropriate to changed paths; require PRs for protected paths; disallow normal-maintainer bypass (any admin bypass is an explicit governance residual and cannot be called pass); require up-to-date-with-base or a merge queue. When the API is unavailable, emit `pending/unverified` and do not pass.

### 2. Topology alarm extension (`crates/tracewake-core/tests/ci_workflow_guards.rs`, modify)

Assert the new governance audit job exists with the expected required-check name list and path filters — as a labeled **topology alarm** only, with a comment stating it does not prove merge enforcement (the API transcript in the capstone does).

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)

## Out of Scope

- The actual branch-protection/ruleset enablement on `main` — operational governance-owner action (§10.5); its API transcript is captured as capstone evidence (010), not a code diff.
- The acceptance manifest's branch-protection enforcement-status field mechanism — ticket 001.
- Any mutation campaign run or `.cargo/mutants.toml` change (the perimeter already covers the seams; §4.4 / §7) — 007/009.
- Any production `src/` change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test ci_workflow_guards` — the governance audit job and the three named jobs are asserted present with correct path filters (topology alarm green).
2. CI run of the governance audit job on a branch where `main` is protected: passes; on an unprotected/permission-less context: emits `pending/unverified`, never `pass` (manual/operational verification recorded for the capstone).
3. `grep -nE "branches/main/protection|rulesets" .github/workflows/ci.yml` returns the audit job's API query.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` — no regression.

### Invariants

1. The audit job is fail-closed: unavailable API ⇒ `pending/unverified`, never pass (INV-098-class; R-29).
2. `ci_workflow_guards.rs` remains a labeled topology alarm and is never cited as merge-enforcement proof (§4.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify) — assert the governance audit job topology (existence + required-check list + path filters).
2. `None additional in code — the merge-enforcement proof is the audit job's own API transcript, captured operationally for the capstone (010), not a Rust test.`

### Commands

1. `cargo test -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. The branch-protection API audit cannot be dry-run locally without a scoped GitHub token; its correct verification boundary is the CI run itself plus the capstone's recorded API transcript — stated here rather than as a runnable local command.

## Outcome

Completed: 2026-06-26

Implemented the governance required-check audit job and topology guard:

- Added `.github/workflows/ci.yml` job `governance-required-checks-audit`, named `governance required checks audit`.
- The job queries both `repos/${GITHUB_REPOSITORY}/branches/main/protection` and `repos/${GITHUB_REPOSITORY}/rulesets?targets=branch`, reports `pending/unverified` when the APIs cannot prove enforcement, and exits nonzero unless the required check contexts, pull-request requirement, up-to-date-or-merge-queue requirement, and no-bypass posture are proven.
- The job treats only active branch rulesets as enforcement evidence; ruleset evaluation mode is not accepted as merge enforcement.
- Extended `crates/tracewake-core/tests/ci_workflow_guards.rs` to assert the governance audit job, API endpoints, fail-closed messages, and required check context names as topology alarms only.
- Updated `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` only to keep the existing "Current CI Job Set" inventory truthful for the newly added workflow job. This is not the §6.1 doctrine strengthening owned by ticket 002.

Deviations:

- No live branch-protection/ruleset API transcript was captured locally. Per this ticket's own Test Plan, that operational evidence is captured by the CI job / capstone artifact (ticket 010), not by a local dry run.

Verification:

- `cargo test -p tracewake-core --test ci_workflow_guards` passed.
- `grep -nE "branches/main/protection|rulesets" .github/workflows/ci.yml` returned the audit job's API queries.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
