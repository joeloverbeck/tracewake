# 0039SPICERMUT-001: Permanent standing mutation perimeter, CI convergence, and guard durability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — rewrites `.cargo/mutants.toml`, the mutation jobs in `.github/workflows/ci.yml`, and `crates/tracewake-core/tests/ci_workflow_guards.rs`. No simulation/runtime production logic changes.
**Deps**: None

## Problem

The 0038 SPINE-CERT audit did not render `SPINE-CERT passed`; its aggregate verdict is `SPINE-CERT scoped remediation` because Wave B found 296 missed mutants only after bypassing the standing configuration with `--no-config`. The failure has two inseparable parts (spec §3): survivor remediation (the per-file tickets 002–019) and **perimeter governance** — the standing mutation configuration and scheduled CI must permanently include the full SPINE seam set so the same classes cannot disappear behind the old perimeter again.

Today `.cargo/mutants.toml` is a deny-list (`exclude_globs`) headed "Scoped lock-layer mutation baseline for ORD-HARD-025" that excludes `crates/tracewake-content/**`, `crates/tracewake-tui/**`, `crates/tracewake-core/src/epistemics/**`, `events/**`, `replay/**`, `checksum.rs`, `view_models.rs`, `debug_reports.rs`, `controller.rs`, `state.rs`, and the action proposal/report files — exactly the SPINE seam set. The scheduled `mutants-lock-layer` CI job re-declares a narrow Wave-A `-f` list in shell (`agent/**`, `scheduler*`, `projections*`, `actions/pipeline.rs`, `defs/{eat,sleep,work}.rs`). This ticket converts both to a single checked-in allow-list union covering the standing SPINE perimeter and makes the CI guard test enforce it.

## Assumption Reassessment (2026-06-18)

1. `.cargo/mutants.toml` currently exists as a deny-list with `additional_cargo_args = ["--workspace", "--locked"]` and a large `exclude_globs` block (verified by read); `.cargo/mutants-baseline-misses.txt` is empty (0 lines, verified). `.github/workflows/ci.yml` has two mutation jobs: `mutants-in-diff` (uses `cargo mutants --in-diff … --no-shuffle`, baseline-miss ratchet via `comm`) and `mutants-lock-layer` (scheduled; narrow `-f` list; `cargo install cargo-mutants --version 27.1.0 --locked`). `crates/tracewake-core/tests/ci_workflow_guards.rs` asserts the `mutants-lock-layer` job and a "dated green scheduled mutation run" (verified by grep).
2. Spec §4.2 names the standing union perimeter (28 globs after reassessment, including the three section-6 seam files `state.rs`, `controller.rs`, `epistemics/projection.rs` added by the in-session `/reassess-spec` fix), §4.3 the CI convergence requirements, §4.4 the preflight census the guard must protect; `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` and `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` govern the no-silent-exclusion/CI-guard contract (verified present).
3. Shared boundary under audit: the mutation-perimeter contract spanning the checked-in `.cargo/mutants.toml`, the two `ci.yml` mutation jobs, and the `ci_workflow_guards.rs` enforcement test. The end-state rule: one authoritative checked-in config; CI loads it rather than maintaining a divergent `-f` list; the guard inspects perimeter, `test_workspace`, no-silent-exclusion, tool-version policy, output retention, and baseline-disposition enforcement.
4. Motivating invariant: `INV-098 — Feature acceptance is harsh` ("a runnable feature is done only when it is … regression-tested"), read with the spec's anti-Goodhart discipline (§3, §4.12) — a perimeter that silently excludes SPINE files lets behavior-changing survivors pass as green. This ticket does not weaken `INV-098`; it widens the regression surface to the full SPINE seam set.
5. The perimeter governs the *proof* of fail-closed-validation, deterministic-replay, and actor-knowledge-firewall surfaces (the substrate-only basis): widening `examine_globs` to add `events/**`, `replay/**`, `checksum.rs`, `state.rs`, `epistemics/**` (`knowledge_context.rs` + `projection.rs`), `projections.rs`, `view_models.rs`, `debug_reports.rs`, content `validate.rs`/`serialization.rs`, and `controller.rs` only *adds* mutation coverage of those enforcement surfaces. It introduces no runtime path, no leakage, and no nondeterminism — it is test-perimeter configuration, not engine behavior. `test_workspace = true` strengthens the proof by testing each core mutant against `tracewake-content`/`tracewake-tui` integration witnesses.
6. Removal blast radius: this ticket removes the narrow Wave-A `-f` list from the scheduled `mutants-lock-layer` job and the SPINE-excluding `exclude_globs` from `.cargo/mutants.toml`. Grepped consumers of that perimeter: `.github/workflows/ci.yml` (both mutation jobs) and `crates/tracewake-core/tests/ci_workflow_guards.rs` (asserts the job/perimeter) — both in this ticket's Files to Touch. No other repo path re-declares the perimeter (the in-diff job uses `--in-diff` + the checked-in config, not a `-f` list).

## Architecture Check

1. A single checked-in `examine_globs` allow-list with `test_workspace = true`, loaded by both CI jobs, is cleaner and more durable than a deny-list plus a shell `-f` list that can silently drift apart (the exact 0038 failure). The guard test makes the perimeter machine-enforced rather than convention-enforced, so a future CI edit cannot silently re-narrow it.
2. No backwards-compatibility aliasing/shims: the old `exclude_globs` SPINE-excluding entries and the scheduled-job `-f` list are removed outright, not wrapped or kept alongside the new allow-list. Exclusions, if any remain, may only cover reviewed non-perimeter paths with no overlap against `examine_globs` (spec §4.2).

## Verification Layers

1. No silent SPINE-file exclusion (INV-098 / §4.4) -> `cargo mutants --workspace --list-files` census compared mechanically against the §4.2 standing file list (must include every listed source file plus `events/mod.rs`, `replay/mod.rs`, and the named seam files `state.rs`, `controller.rs`, `epistemics/projection.rs`).
2. Config/CI non-divergence (§4.3) -> `ci_workflow_guards.rs` grep-proof that the scheduled job loads `.cargo/mutants.toml` and declares no independent `-f` perimeter list.
3. Workspace-wide witnesses (§4.2) -> schema check that `.cargo/mutants.toml` sets `test_workspace = true` (or an equivalent workspace-test invocation) and the guard asserts it.
4. Tool-version / output-retention / baseline discipline (§4.3, §4.12) -> `ci_workflow_guards.rs` assertions on the pinned `cargo-mutants` version policy, artifact upload, and baseline-miss-enforcement steps.

## What to Change

### 1. Rewrite `.cargo/mutants.toml` to the standing allow-list

Replace the ORD-HARD-025 deny-list with the spec §4.2 posture: `additional_cargo_args = ["--workspace", "--locked"]`, `test_workspace = true`, and an `examine_globs` block expressing the full standing union (the 28 entries of §4.2, including `crates/tracewake-core/src/state.rs`, `crates/tracewake-core/src/controller.rs`, and `crates/tracewake-core/src/epistemics/projection.rs`). Narrow any retained `exclude_globs` so none overlaps an examined path. Document the cargo-mutants `27.1.0` config-syntax baseline inline (spec §4.2).

### 2. Converge the CI mutation jobs on the checked-in config

In `mutants-lock-layer` (scheduled): drop the narrow `-f` list; invoke the checked-in configuration (`cargo mutants --workspace --no-shuffle`), retaining timeout/shard/output-dir/job-control options that preserve the same perimeter and evidence denominator. In `mutants-in-diff`: change the change-detection step so it triggers across every standing-perimeter path (not only the old Wave-A paths) and loads the same `.cargo/mutants.toml`; keep the baseline-miss ratchet, fail-on-tool-failure, and artifact-upload steps.

### 3. Harden `ci_workflow_guards.rs`

Update the guard so future CI edits cannot silently narrow either job: assert the standing perimeter membership, `test_workspace`, the no-`-f`-divergence rule, the pinned tool-version (or version policy), output retention, and baseline-disposition enforcement.

## Files to Touch

- `.cargo/mutants.toml` (modify)
- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)

## Out of Scope

- Running the full mutation campaign or reconciling survivors (ticket 020).
- Adding behavior-witness tests for any survivor (tickets 002–019).
- SPINE-01…08 re-proof or the replacement acceptance artifact (ticket 021).
- Scheduled frequency, exact shard count, timeout values, runner topology, and output-retention location — implementer-recorded owner decisions (spec §4.6, §12.2); this ticket fixes only the *coverage*, not those parameters.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards` — passes with the hardened assertions (perimeter membership, `test_workspace`, no divergent `-f`, version policy, retention, baseline enforcement).
2. `cargo mutants --workspace --list-files` — the emitted census includes every §4.2 standing source file plus `events/mod.rs`, `replay/mod.rs`, `state.rs`, `controller.rs`, and `epistemics/projection.rs`; no `exclude_globs` entry overlaps an examined path.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked` — clean baseline at the implementation commit.

### Invariants

1. Exactly one authoritative checked-in mutation perimeter; neither CI job maintains a second independent `-f` perimeter list.
2. No `#[mutants::skip]`, exclusion glob, or CI condition silently removes a required SPINE file from the enumerated census.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — extend to assert the standing union perimeter, `test_workspace = true`, the no-`-f`-divergence rule, tool-version policy, output retention, and baseline-miss enforcement against the rewritten config and CI jobs.

### Commands

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
2. `cargo mutants --workspace --list-files`
3. `cargo test --workspace --locked` — full-pipeline confirmation that the config/CI/guard rewrite leaves the unmutated baseline green (the mutation campaign itself is ticket 020's boundary).
