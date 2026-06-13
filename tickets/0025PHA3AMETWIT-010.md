# 0025PHA3AMETWIT-010: CI workflow-lint guard, cache-key hygiene, doctrine reconciliation, and the mutation phase-entry rule

**Status**: PENDING
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — `.github/workflows/ci.yml`, a new `tracewake-core` test target (`ci_workflow_guards.rs`), `docs/2-execution/10`, and a membership line in the core anti-regression guards.
**Deps**: 0025PHA3AMETWIT-001

## Problem

Spec 0025 findings `ORD-HARD-188` (low), `ORD-HARD-189` (low), `ORD-HARD-190`
(low). CI's `actions/cache@v4` steps cache `target/` keyed only on
`hashFiles('**/Cargo.lock')` — omitting the toolchain channel and `Cargo.toml` —
and no in-repo guard asserts any workflow integrity property (gate-command
fidelity, no exit-status masking, action pinning posture); the research corpus
(actionlint/zizmor rule catalogs as grep checklists; GitHub's default `run:`
shell is `bash -e` *without* `pipefail`) ports as a zero-dep workflow-lint test.
CI is also stricter than the documented four-gate contract — workflow-level
`RUSTFLAGS: "-D warnings"`, `cargo test --workspace --locked`, and a
twelve-invocation `lock-layer-gates` job — none named in
`docs/2-execution/10` (safe-direction drift, unreconciled authority). And the
scheduled `mutants-lock-layer` run's "pending" status never converts to a
required signal: the 0024 report §13 records "still pending" and nothing requires
a dated green run before `ORD-LIFE-CERT` can clear it.

## Assumption Reassessment (2026-06-12)

1. Verified against `9e33d7a`: the `target`-bearing cache blocks keyed on
   `Cargo.lock` only; `RUSTFLAGS` workflow env, `cargo test --workspace --locked`,
   and the twelve `cargo test --locked -p` invocations in the `lock-layer-gates`
   job (`.github/workflows/ci.yml`); zero `lock-layer`/`RUSTFLAGS`/`--locked`
   mentions in `docs/2-execution/10`; the 0024 report's §13 "still pending" with
   no conversion rule. All operator-verified per spec 0025 §8 (189/190 at
   reassessment). Note: the originally reported cache *poisoning* mechanism was
   refuted in part at spec triage (cargo re-fingerprints changed sources; cache
   writes require trusted runs) — the residue is key hygiene plus the absent
   workflow-lint guard, which is what this ticket implements.
2. Verified against spec 0025 §4/§2 (research adoptions: SHA pinning for
   third-party actions, declared `permissions:`, no pipes/`||` on gate steps,
   verbatim gate commands) and the four-gate contract in `CLAUDE.md` /
   `docs/2-execution/10`.
3. Shared boundary under audit: the documented gate contract ↔ the workflow YAML —
   CI must run exactly the documented gates (supersets documented, never silent),
   with no masking construct on a gate step.
4. Rule restated before trusting the narrative (gate fidelity; spec 0025 §2
   research): a piped gate command under GitHub's default shell passes green on a
   failing first stage; gate steps must be pipe-free or `pipefail`-defaulted, and
   the documented commands must appear verbatim so CI cannot silently weaken.
5. Enforcement surface touched: the CI gates are the enforcement layer for every
   other lock in the repo. The workflow-lint guard adds assertions and relaxes
   nothing; cache-key tightening can only invalidate caches (rebuilds), never skip
   gates; no replay/actor-knowledge surface is involved.

## Architecture Check

1. A zero-dep in-repo workflow-lint `#[test]` (reading `.github/workflows/*` via
   `include_str!`/fs and asserting the ported checklist) is cleaner than adopting
   actionlint/zizmor as CI tools: it survives the no-new-dependencies constraint,
   versions with the repo, and its assertions are exactly the documented contract
   — drift fails in the same suite as every other guard. Documenting the CI
   superset in doc 10 reconciles authority instead of weakening CI to match docs.
2. No backwards-compatibility aliasing/shims: one cache-key scheme (toolchain +
   `Cargo.toml` + `Cargo.lock`); no legacy key kept; the lint guard has no
   suppression list without rationale.

## Verification Layers

1. Workflow-lint guard (`ORD-HARD-188`) → new test target asserting: the four
   documented gate command-lines appear verbatim; no pipe/`||`/`continue-on-error`
   on gate steps; `permissions:` declared; non-`actions/*` `uses:` are SHA-pinned;
   `target`-bearing cache keys include toolchain + `Cargo.toml` hashes. A
   synthetic-workflow negative (each violation class) must fire.
2. Doctrine reconciliation (`ORD-HARD-189`) → grep-proof: `docs/2-execution/10`
   enumerates the actual CI job set and flag posture (`RUSTFLAGS`, `--locked`,
   `lock-layer-gates`); the lint guard asserts the documented superset list
   matches the workflow jobs.
3. Mutation phase-entry rule (`ORD-HARD-190`) → the rule recorded in doc 10
   (certification claims require a dated green scheduled-mutation run); the
   acceptance-artifact wording guard extended so a report claiming certification
   readiness with a "pending" mutation row fails (lands with ticket
   `0025PHA3AMETWIT-011`'s artifact; the rule text lands here).
4. Guard deletion-lock → a required-member line in
   `anti_regression_guards.rs` asserts the `ci_workflow_guards` test target's
   guards exist (under the 0025PHA3AMETWIT-001 repaired routing).
5. Whole-suite regression → four-gate workspace run; CI itself green on the
   tightened keys.

## What to Change

### 1. Cache-key hygiene (`ORD-HARD-188`)

Add the toolchain channel (from `rust-toolchain.toml`) and
`hashFiles('**/Cargo.toml')` to every `target`-bearing cache key in `ci.yml`.

### 2. Workflow-lint guard (`ORD-HARD-188`)

New `crates/tracewake-core/tests/ci_workflow_guards.rs` implementing the ported
checklist (Verification layer 1) over `.github/workflows/*`, with one firing
synthetic per violation class; membership-locked via `anti_regression_guards.rs`.

### 3. Doctrine reconciliation (`ORD-HARD-189`)

Add the CI job-set/flag-posture row to `docs/2-execution/10`; the guard asserts
doc↔workflow parity.

### 4. Mutation phase-entry conversion rule (`ORD-HARD-190`)

Record in `docs/2-execution/10`: clearing the mutation "pending" status (and any
`ORD-LIFE-CERT` claim) requires the latest scheduled run's dated green result;
hand the artifact-wording enforcement spec to ticket `0025PHA3AMETWIT-011`.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (new)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — membership lock)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- Adopting actionlint/zizmor/shellcheck or any external CI tooling (zero-dep
  constraint; their catalogs port as greps only).
- The 0025 acceptance artifact and its wording-guard extension — ticket
  `0025PHA3AMETWIT-011`.
- Branch-protection / required-checks configuration (GitHub-side, not auditable
  in-repo; spec 0025 §3 records it for maintainer confirmation).

## Acceptance Criteria

### Tests That Must Pass

1. The workflow-lint guard passes against the tightened `ci.yml` and each
   violation-class synthetic fires (masked gate step, unpinned third-party
   action, missing `permissions:`, hygiene-less `target` cache key, missing
   verbatim gate command, undocumented job).
2. `docs/2-execution/10` carries the job-set row and the phase-entry conversion
   rule (grep-proofs); the membership lock fails when the new test target's
   guards are removed.
3. `cargo test -p tracewake-core --test ci_workflow_guards` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`)
   pass; CI runs green on the tightened cache keys.

### Invariants

1. CI runs the documented gates verbatim with no masking construct; any superset
   job is doctrine-listed; doc↔workflow parity is guard-enforced.
2. A certification claim cannot clear the mutation row without a dated green
   scheduled-run result.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — the ported checklist
   guards + per-class synthetics.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — membership lock for
   the new target.

### Commands

1. `cargo test -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
