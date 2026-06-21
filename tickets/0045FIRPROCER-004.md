# 0045FIRPROCER-004: Deterministic sharded scheduled/manual mutation completion lane (CI matrix + reconciliation job)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — CI workflow (`.github/workflows/ci.yml`) sharded matrix + reconciliation job and the core CI guard test (`crates/tracewake-core/tests/ci_workflow_guards.rs`); no production/simulation logic change.
**Deps**: 001, 002, 003

## Problem

The scheduled/manual `mutants-lock-layer` job runs a single unsharded `cargo mutants --workspace --no-shuffle` invocation (`ci.yml:198`) with no explicit `timeout-minutes`, accepting cargo-mutants exit `0`/`2` for later miss-baseline processing and uploading one `mutants.out` under `if: always()` — and the workflow `concurrency` block sets `cancel-in-progress` false only for `schedule`/`workflow_dispatch` (`ci.yml:14`). That single invocation is exactly what timed out at 2,384 of 2,901 mutants in 0044 (spec §3.4, §9.6). It has no matrix fan-out, no denominator-union verifier, no merged outcome manifest, and no checked-in completion budget.

This ticket converts the scheduled/manual complete lane into a durable deterministic shard matrix that can classify the full configured population and retain evidence from every shard/attempt: a dedicated baseline job, `N` supervised `k/n` shards with identical arguments/toolchain/config, `fail-fast: false` so one shard's miss/failure does not cancel the others, unique `always()`-uploaded artifacts, explicit per-shard budgets with retention grace, and a trailing reconciliation job that invokes the -003 merger to prove disjoint exact union before the lane is considered complete. The in-diff lane remains the explicitly non-certifying fast detector (-001); this lane is the certifying denominator.

(spec §4.7.2, §4.7.3, §4.8 items 5–9, §5.8, §9.6)

## Assumption Reassessment (2026-06-21)

1. The scheduled/manual job is `mutants-lock-layer` (`ci.yml:177`), gated `if: github.event_name == 'workflow_dispatch' || github.event_name == 'schedule'`, installing `cargo-mutants --version 27.1.0 --locked` (`:193`), running one `cargo mutants --workspace --no-shuffle` (`:198`), with no `timeout-minutes` and one `actions/upload-artifact@v4` under `if: always()` (`:65`-region of the job). Verified this session. The workflow `concurrency` block (`:12`) sets `cancel-in-progress: ${{ github.event_name != 'schedule' && github.event_name != 'workflow_dispatch' }}` — already correct for not cancelling certifying runs; preserve it.
2. The CI guard test `crates/tracewake-core/tests/ci_workflow_guards.rs` asserts on the scheduled job structure: it references `lock-layer-gates` and `mutants-lock-layer` (`:314`-`:315`), a "dated green scheduled mutation run" expectation (`:316`), and a "divergent scheduled mutation perimeter uses -f filters" negative (`:360`). Verified this session. Restructuring the lock-layer job into a matrix will require updating these guard assertions so they describe the matrix + reconciliation shape rather than the single invocation — this is the co-edit that makes the two paths converge.
3. Cross-artifact boundary under audit: the shared `.github/workflows/ci.yml` and `ci_workflow_guards.rs` are also edited by -001 (in-diff trigger + perimeter constants). This ticket `Deps: 001` so the trigger/guard convergence lands first and this matrix restructuring builds on it — the two are a sequenced 2-way edit of both files, not a merge race. The reconciliation job consumes the -003 merger (`Deps: 003`) and each shard is launched under the -002 hardened supervisor (`Deps: 002`).
4. Invariant/rule motivation: the execution mutation-completion / evidence-honesty doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) — an incomplete, canceled, wall-timed-out, internally failed, or missing shard cannot contribute a passing row (spec §4.1 item 12, §6.5) — and the artifact-loss / fail-fast-cancellation risk controls (spec §12). GitHub Actions matrices create one job per combination, `fail-fast` defaults true, and `always()` artifact upload is the documented pattern for retaining failed-shard output (spec §4.7.3, Appendix C); these mechanics shape the implementation but do not by themselves make the lane certifying.
5. Enforcement surface: the CI guard test is the fail-closed gate that keeps this lane honest (no `-f`/`--no-config`/`--in-diff` denominator shrink, no silent single-shard substitution). The change touches no simulation-world fail-closed validation, actor-knowledge filtering, or deterministic-replay surface and introduces no epistemic-leakage path; the shard determinism is `--no-shuffle` + identical arguments + a frozen sharding algorithm, so the classification is reproducible. `--baseline=skip` is permitted on shards only with a dedicated same-commit baseline job every shard depends on, an explicit mutant test timeout, and a baseline fingerprint bound to every shard (spec §4.7.2).

## Architecture Check

1. A deterministic shard matrix with a member-set reconciliation job is cleaner and more durable than a single longer-walled invocation: the historical failure was a single process under one external wall, and merely raising the wall repeats it (spec §9.5). Fan-out plus `fail-fast: false` plus `always()` retention means every shard's evidence survives a sibling failure, and the trailing merger turns "the counts add up" into a proven disjoint exact union. The lane fails loudly on any missing/canceled/incomplete/tool-failed shard rather than silently undercounting.
2. No backwards-compatibility aliasing/shims. The in-diff lane stays explicitly non-certifying and is not promoted to the denominator; `.cargo/mutants.toml` is not shrunk to simplify sharding; the checked-in config is loaded identically by every shard.

## Verification Layers

1. Complete, disjoint shard coverage → core test: `ci_workflow_guards.rs` asserts the matrix enumerates each shard index exactly once, sets `fail-fast: false`, gives every shard a unique `always()`-uploaded artifact, sets explicit `timeout-minutes`, preserves the scheduled/manual no-cancel concurrency, and runs a trailing reconciliation job — with a synthetic negative proving a removed shard index or a `fail-fast: true` flip fails the guard.
2. Reconciliation gates completion → manual review + workflow logic: the reconciliation job invokes the -003 merger and the aggregate workflow result is failure when any shard is missing, canceled, incomplete, tool-failed, contains a miss/timeout, or fails union reconciliation.
3. Denominator not shrunk → core test + grep-proof: the guard still forbids a `-f`/`--no-config` scheduled perimeter (the existing `:360` negative), and `.cargo/mutants.toml` is unchanged.
4. No simulation surface touched → single-layer note: per item 5, no replay/golden-fixture/actor-knowledge mapping applies; verification is the CI guard test plus workflow review.

## What to Change

### 1. Restructure the scheduled/manual lock-layer job into a supervised shard matrix

In `.github/workflows/ci.yml`: replace the single `mutants-lock-layer` invocation with (a) a dedicated baseline job at the same commit/config/toolchain that runs and records the unmutated baseline, and (b) a `strategy.matrix` of `N` shards each running `cargo mutants --workspace --no-shuffle --shard "${K}/${N}" --jobs "${MUTANTS_JOBS}" -o <unique-out>` under `tools/supervise-command.sh`, with `strategy.fail-fast: false`, no `continue-on-error: true` on certifying shards, a unique artifact name per shard uploaded under `if: always()`, an explicit `timeout-minutes` larger than the measured shard wall plus retention grace, and the preserved scheduled/manual no-cancel concurrency. `N`, `MUTANTS_JOBS`, the sharding algorithm, and the budgets are taken from the -002 completion diagnostic (recorded, not invented here). If `--baseline=skip` is used on shards, wire every shard to depend on the dedicated baseline job, supply an explicit mutant test timeout, and bind the baseline fingerprint to every shard (spec §4.7.2).

### 2. Add the trailing reconciliation job

Add a final job (depends on all shards, `if: always()`) that downloads every shard artifact and the canonical list, invokes the -003 merger, and makes the aggregate workflow fail when any shard is missing, canceled, incomplete, tool-failed, contains a miss/timeout, or fails union reconciliation. Preserve the existing baseline-miss ratchet machinery for compatibility, but a passing lane still requires zero final standing misses/timeouts (spec §4.8).

### 3. Co-edit the CI guard to the matrix shape

In `crates/tracewake-core/tests/ci_workflow_guards.rs`: update the `mutants-lock-layer`/scheduled-run assertions (`:314`-`:316`, `:360`) to describe the matrix + reconciliation structure, and add synthetic negatives proving that dropping a shard index, flipping `fail-fast: true`, removing the reconciliation job, or removing a shard's `always()` upload fails the guard. Keep the `-f`/`--no-config` denominator-shrink negative.

### 4. Update execution documentation only where it describes the live lane

If `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (or another live execution doc) describes the single-invocation lock-layer lane, update that text to the durable sharded lane and record the exact runner/timeout/storage assumptions (spec §4.8 item 9, §5.8). No doc change if no live execution text references the lane shape.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — only if live execution text describes the lock-layer lane shape; verify-first)

## Out of Scope

- The in-diff trigger / perimeter-constant convergence (→ -001; landed first as this ticket's `Deps`).
- The supervisor retention contract and completion diagnostic (→ -002).
- The merger tool and its synthetic negatives (→ -003; invoked by the reconciliation job here, not built here).
- Actually running the certifying campaign, capturing the census, and producing the real completion manifest + triage register (→ -005).
- Live FIRST-PROOF re-proof and the replacement acceptance artifact (→ -006).
- Any shrinkage of `.cargo/mutants.toml` or survivor laundering into `.cargo/mutants-baseline-misses.txt`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards` passes with the matrix + reconciliation assertions and the updated `mutants-lock-layer`/scheduled-run guards.
2. Synthetic negatives fail the guard when a shard index is dropped, `fail-fast` is flipped to `true`, the reconciliation job is removed, or a shard's `always()` upload is removed; the `-f`/`--no-config` scheduled-perimeter negative still fails.
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass; the workflow YAML parses (e.g. `actionlint` where available, else a YAML-syntax check, with the reason stated if `actionlint` is absent).

### Invariants

1. The scheduled/manual lane enumerates every shard index exactly once with identical commit/config/toolchain/arguments/denominator, `fail-fast: false`, unique `always()`-uploaded artifacts, and a reconciliation job whose failure fails the aggregate workflow.
2. `.cargo/mutants.toml` is the loaded denominator on every shard; no `-f`/`--no-config`/`--in-diff` subset defines the certifying population, and `.cargo/mutants-baseline-misses.txt` stays empty.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — update the scheduled-lane assertions to the matrix + reconciliation shape and add synthetic negatives for dropped-shard, `fail-fast: true`, missing-reconciliation-job, and missing shard `always()` upload.

### Commands

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. `actionlint .github/workflows/ci.yml` (where available) to confirm the matrix/reconciliation YAML is valid; if `actionlint` is absent on the runner, substitute a YAML-parse check and state the substitution as the reason.
