# 0052FOUCONFOU-011: F4-08 — enforced standing barrier: required CI lane + branch-protection governance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the required public-boundary conformance CI lane and full-surface mutation trigger breadth; records required-check + branch-protection governance
**Deps**: 0052FOUCONFOU-009, 0052FOUCONFOU-010

## Problem

Spec 0052 F4-08 / §4.10 Layer B & C; closure-order step 11. The repository has the right mutation components but not the required standing barrier: the full canonical census / shards / reconciliation run only on `workflow_dispatch` or weekly `schedule` (ci.yml lines 7–9), not as a merge-required PR check; in-diff mutation (`mutants-in-diff`, ci.yml line 103) cannot detect witness-only regressions that weaken a test/fixture/transcript/CI trigger without touching a protected production line; and a red scheduled result does not block merge unless governance makes it mandatory.

This ticket composes the existing tests into one named **required public-boundary conformance CI lane** (the §4.10 Layer B path and minimum behavior matrix, built as tests in 009), adds the full-surface mutation trigger breadth (Layer C), and makes the named conformance job and mutation reconciliation required branch-protection checks. It mints no new doctrine gate code; the barrier is distributed across the existing certification ladder (§4.10 certification-ladder placement).

## Assumption Reassessment (2026-06-25)

1. `.github/workflows/ci.yml` currently defines `fmt`, `clippy`, `test`, `lock-layer-gates`, and `mutants-in-diff` jobs; triggers are `push`/`pull_request`/`workflow_dispatch`/`schedule` (cron `17 3 * * 1`). The `mutants-in-diff` guarded file-list (ci.yml line 119) already enumerates runtime/scheduler/replay/events/view_models/pipeline/TUI surfaces; `cargo-mutants` is pinned to `27.1.0`, timeout `183`. The canonical perimeter is `.cargo/mutants.toml`.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.10 Layer B/C; closure-order step 11. The proving tests for the lane are authored in 0052FOUCONFOU-009; the standing campaign that must be green is run in 0052FOUCONFOU-010.
3. Cross-artifact boundary under audit: the CI workflow (`ci.yml`) and repository branch-protection governance. A workflow file alone cannot prove branch protection is configured; the required-check name and repository-governance confirmation are operational evidence recorded in the acceptance artifact (013).
4. Motivating invariant: INV-098 (harsh acceptance) — the barrier makes a green standing mutation perimeter plus a required public-boundary lane an enforced merge property; supported by INV-092/093/094 (regression sensitivity). No new gate code is minted (the barrier rides SPINE-CERT / EPI-CERT / ORD-LIFE-CERT / FIRST-PROOF-CERT as an artifact dependency).
5. Fail-closed / governance surface (CI-enforcement basis): the conformance lane gates the production-bootstrap → sealed-receipt → replay path and the all-feature negative fixtures (the replay/no-leak/parity/validation enforcement surfaces of 002–009); the full-surface mutation trigger fires whenever production files, tests, fixtures, negative fixtures, mutation config/baseline, CI workflow, merge/supervisor tooling, or live conformance evidence for this surface changes — closing the witness-only-regression gap. The lane adds no production behavior and weakens no enforcement surface; it makes their regression detection merge-blocking. Exit 2/3, tool errors, incomplete census, stale artifacts, and absent output are failures, not passes.

## Architecture Check

1. Composing existing tests into one required named lane (no new doctrine gate code) plus a path-sensitive full-surface mutation trigger is cleaner than relying on scheduled/manual runs because it binds production reachability, type-level API authority, and standing mutation sensitivity into a merge property — a scheduled-only barrier lets a witness-weakening PR merge before the next scheduled run. Keeping the canonical perimeter as the single `.cargo/mutants.toml` (no parallel spec-0052 config) prevents drift.
2. No backwards-compatibility shim: the lane is additive CI composition; no production alias. The weekly scheduled campaign is retained as drift detection but a red scheduled result is treated as merge-blocking until repaired.

## Verification Layers

1. INV-098 (enforced acceptance) -> CI config review + a synthetic negative: the named conformance job runs the 009 behavior matrix on every PR touching production **or its witnesses/configuration**; deliberately switching the production bootstrap or removing a receipt effect makes the lane fail even if docs are unchanged.
2. Trigger breadth -> CI config grep-proof: the full-surface mutation trigger path-list includes production + tests + fixtures + mutation config/baseline + CI workflow + merge/supervisor tooling + live conformance evidence.
3. Governance -> recorded operational evidence (in 013): required-check name + branch-protection confirmation; full canonical reconciliation (commit/config/toolchain/census fingerprints) before remediation acceptance.

## What to Change

### 1. Required public-boundary conformance lane (`ci.yml`)

Add a named CI job composing the 009 production-bootstrap-to-sealed-receipt behavior matrix + all-feature negative fixtures; trigger it whenever production files, tests, fixtures, negative fixtures, mutation config/baseline, CI workflow, merge/supervisor tooling, or live conformance evidence for this surface changes. Mint no new doctrine gate code.

### 2. Full-surface mutation trigger breadth (`ci.yml`)

Require a full canonical surface campaign (not only in-diff) when a change can weaken old witnesses without touching old production lines; keep in-diff for fast feedback; keep the weekly scheduled campaign as drift detection but treat a red scheduled result as merge-blocking until repaired. Fail on misses/timeouts/missing shards/missing census members/tool error/stale baseline/fingerprint mismatch.

### 3. Branch-protection governance (operational; recorded in 013)

Make the named conformance job and mutation reconciliation required branch-protection checks; the required-check name and repository-governance confirmation are recorded in the acceptance artifact (013), since a workflow file cannot prove branch protection.

## Files to Touch

- `.github/workflows/ci.yml` (modify — required conformance lane; full-surface mutation trigger breadth; scheduled-red merge-block policy)

## Out of Scope

- Editing `.cargo/mutants.toml` coverage (§4.9 — perimeter already covers the seams; do not restate or create a parallel config).
- The proving tests themselves (009) and the standing campaign run (010).
- Any production-code change.
- The repository branch-protection *setting* itself (operational; this ticket adds the job and records the required-check name/confirmation in 013).

## Acceptance Criteria

### Tests That Must Pass

1. The named conformance job is present in `ci.yml`, composes the 009 behavior matrix + all-feature negative fixtures, and triggers on production **and** witness/config/CI changes for this surface.
2. A synthetic negative (switch the production bootstrap or remove a receipt effect) makes the lane fail even with docs unchanged.
3. The full-surface mutation trigger path-list and scheduled-red merge-block policy are present; `cargo build --workspace --all-targets --locked && cargo test --workspace` (the lane composes existing green tests).

### Invariants

1. The conformance lane and mutation reconciliation are required merge checks; witness-only/config/CI changes cannot weaken old mutation sensitivity and still merge (INV-098).
2. The canonical perimeter remains the single `.cargo/mutants.toml`; no parallel config is created (INV-092).

## Test Plan

### New/Modified Tests

1. `None — CI-configuration ticket; verification is CI config review + the 009 behavior matrix the lane runs + the 010 standing campaign it gates. Branch-protection confirmation is operational evidence recorded in 013.`

### Commands

1. `grep -nE "conformance|mutants|workflow_dispatch|schedule" .github/workflows/ci.yml` — confirm the lane + trigger breadth + scheduled policy.
2. `cargo build --workspace --all-targets --locked && cargo test --workspace` — the composed lane runs existing green tests.
