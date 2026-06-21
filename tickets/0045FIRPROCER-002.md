# 0045FIRPROCER-002: Mutation-lane completion diagnostic + transport-honest supervisor retention

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — supervisor tooling (`tools/supervise-command.sh`) and a new mutation-lane completion diagnostic report; no production/simulation logic change.
**Deps**: None

## Problem

The 0044 standing configured mutation campaign did not complete: a single unsharded `cargo mutants --workspace --no-shuffle` invocation was cut off by the external supervisor after 7,200 seconds (30-second grace), exiting `124` with `supervisor_result=wrapper_wall_timeout`, having classified 2,384 of 2,901 mutants (interrupted during a build associated with `crates/tracewake-core/src/events/apply.rs`; `reports/0044_first_proof_cert_mutation_full.supervisor/stderr.txt` records `ERROR scenario execution internal error err=interrupted phase=Build`). The blocking floor is therefore campaign *completion*, not a known survivor — there is no recorded `missed` or mutant-level `timeout` in either the classified standing portion or the completed focused wave (spec §3.1–3.2).

Two preconditions must land before any sharded completion lane (-004) or certifying run (-005) is trustworthy:

1. **A measured completion diagnostic** that explains why the historical single invocation exceeded the wall and selects a topology with real margin — not "increase the timeout" or "the run is probably clean" (spec §4.4, §5.1).
2. **A transport-honest supervisor** that retains evidence on every non-normal termination and keeps its own wall timeout distinct from cargo-mutants' own exit codes, so partial/failed attempts are diagnosable rather than lost (spec §4.6).

This ticket delivers both. It does not run the certifying campaign; it makes the campaign measurable and its evidence retention honest.

(spec §3.1–3.2, §4.4, §4.6, §5.1, §13.1 phases 2–4)

## Assumption Reassessment (2026-06-21)

1. `tools/supervise-command.sh` exists and is the standing supervisor. Verified this session: it takes `<output-dir> <wall-seconds> <grace-seconds> -- <command>` (`:6`), runs the child under `timeout --kill-after="${grace_seconds}s" "${wall_seconds}s"` (`:57`), writes `status.env` (`:27`) with `wall_seconds` (`:44`) and `supervisor_result` (`:84`) classified across `child_exit_0`, `wrapper_wall_timeout`, `supervisor_or_spawn_failure`, `wrapper_forced_kill`, `child_signal_or_forced_exit`, `child_nonzero_exit` (`:62`–`:77`), and copies `mutants.out` to `mutants.out.partial` (`:90`). Gaps vs spec §4.6: it relies on `timeout`'s own process handling rather than explicitly supervising/reaping the full process group, and the partial copy is not unconditionally guaranteed in a `finally`/`always`-style trap on every exit path. These are the hardening targets.
2. The retained 0044 evidence is the diagnostic's measured input. Verified this session: `reports/0044_first_proof_cert_mutation_full.supervisor/status.env` (`exit_status=124`, `supervisor_result=wrapper_wall_timeout`), `metadata.txt` (`wall_seconds=7200`, `grace_seconds=30`), `full.out/mutants.out/{caught,missed,unviable,timeout}.txt` (1,869 / 0 / 515 / 0 ⇒ 2,384 classified, 517 unclassified), and the focused wave `focused.out/mutants.out/` (600 / 0 / 119 / 0 = 719; `focused.supervisor/status.env` `child_exit_0`). The diagnostic must derive its symptom record from these exact retained paths, not from memory.
3. Cross-artifact boundary under audit: the four-axis separation the package must preserve — launcher/supervisor result vs denominator completion vs cargo-mutants process exit code vs per-mutant tool outcome (spec §4.5). The supervisor owns the first axis; conflating a wrapper wall timeout (exit 124) with a cargo-mutants per-mutant timeout (exit 3) or a clean partial run is the central failure mode (spec §12 "Wrapper/mutant timeout conflation").
4. Invariant/rule motivation: the execution evidence-honesty / mutation-completion doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) — pending, incomplete, tool-failed, sampled, observer-only, or historical evidence cannot be silently counted as a pass — and the anti-Goodhart controls (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`; spec §10.3). cargo-mutants documents conservative `--jobs` tuning, distinct exit codes, and incremental `mutants.out` writes that are diagnostically useful but non-certifying until completion is independently proven (spec §4.4, §4.6, Appendix C).
5. Enforcement surface: the supervisor's retention contract is the evidence-honesty gate that everything downstream depends on. It touches no simulation-world fail-closed validation, actor-knowledge filtering, or deterministic-replay surface, and introduces no epistemic-leakage or replay-nondeterminism path — it is process supervision over the mutation toolchain. The change must not let a lost-evidence cancellation read as a pass: a host cancellation that prevents retention is a tool/infrastructure failure, not a clean result (spec §4.6).

## Architecture Check

1. Hardening the existing supervisor and writing a measured diagnostic is cleaner than re-running the same single process under the same wall and calling the retry a remedy. The diagnostic's measured throughput and resource-saturation evidence is what lets -004 size sharding with margin rather than matching the historical estimate; the supervisor's unconditional retention is what keeps every trial (including failed ones) diagnosable. A brittle "bigger timeout" alone repeats the 0044 failure mode (spec §9.5).
2. No backwards-compatibility aliasing/shims. The supervisor keeps its existing CLI contract and `status.env` field names where they already match spec §4.6; new fields are additive. No simulation code path is touched, and the standing denominator is not modified.

## Verification Layers

1. Transport honesty (four-axis separation) → manual review + command: `tools/supervise-command.sh` records command/cwd/env-affecting vars/timestamps/PIDs/signals/exit, distinguishes its own wall timeout from the child exit code, and emits a machine-readable `status.env` whose supervisor and child fields are separate — exercised by a smoke run whose child exits non-zero and whose output is still retained.
2. Unconditional retention → command: a smoke run that is force-killed (or whose child exits non-zero) still leaves the output tree + raw stdout/stderr + `status.env` under a non-overwriting path; a second attempt does not clobber the first.
3. Diagnostic sufficiency → manual review against spec §5.1: the report answers all ten required questions (why >7,200 s, time-fraction breakdown where measurable, steady/skewed/stalled progress, selected topology + measured margin, retention-on-every-termination, host-wall vs cargo-mutants-exit vs per-mutant-timeout distinctions, union-by-set-not-count, fail-fast non-cancellation, exact `pass` condition, exact `scoped remediation` condition) and is backed by retained commands/timing/resource/fingerprint evidence rather than assertion.
4. No simulation surface touched → single-layer note: per item 5, no replay/golden-fixture/actor-knowledge mapping applies; verification is supervisor smoke + diagnostic review.

## What to Change

### 1. Harden `tools/supervise-command.sh` to the spec §4.6 retention contract

Bring the supervisor to: launch without a PTY unless a documented platform constraint requires one; supervise and reap the complete process group (not just the immediate `timeout` child); record command, cwd, environment-affecting variables, start/end timestamps, process IDs, signals, and exit statuses; send graceful termination then a recorded forced kill after bounded grace; copy/preserve the output directory and raw stdout/stderr in an unconditional `trap`/`finally`-style path on every exit (normal, non-zero, wall timeout, forced kill, spawn/IO failure); never overwrite a previous attempt's output; and emit a machine-readable `status.env` (or JSON) with separate supervisor and child fields. Keep the existing CLI contract and the existing `supervisor_result` vocabulary; extend it only additively.

### 2. Author the mutation-lane completion diagnostic

Create `reports/0045_first_proof_cert_mutation_lane_completion_diagnostic.md` per spec §4.4/§5.1: the complete 0044 symptom record with exact retained evidence paths; the 7,200 s / 30 s grace / exit-124 process state at interruption; measured baseline build/test timing; elapsed-vs-classified progress sampled from retained logs where available; per-file/per-symbol runtime distribution where derivable; CPU/memory/disk/cache/runner constraints per trial; the identified bottleneck; the alternatives evaluated (longer wall, conservative `--jobs`, deterministic sharding, larger/self-hosted runner, combination); the selected topology and why it has margin (not merely matching the historical estimate); a completion-risk calculation including slowest shard, artifact-upload grace, and retry cost; regression controls for each termination class; and the exact acceptance test for the supervisor and (forward-referenced) union merger. Measurement trials are **development evidence only** and must be labelled non-certifying (spec §4.10).

### 3. Record the wall-clock scale estimate as orientation, not prediction

Include spec §9.5's rough scalar (≈0.331 classified mutants/s from the retained wrapper interval) explicitly as a non-prediction used only to reject a brittle "same single process, bigger wall" design — the final topology is justified by measured trial evidence, not extrapolation.

## Files to Touch

- `tools/supervise-command.sh` (modify)
- `reports/0045_first_proof_cert_mutation_lane_completion_diagnostic.md` (new)

## Out of Scope

- The in-diff trigger / CI guard convergence (→ -001).
- The set-union merger tool and its synthetic negatives (→ -003).
- The sharded scheduled/manual CI matrix and reconciliation job (→ -004).
- Running the certifying campaign, census, completion proof, and triage register (→ -005).
- Live FIRST-PROOF re-proof and the replacement acceptance artifact (→ -006).
- Any change to `.cargo/mutants.toml` or `.cargo/mutants-baseline-misses.txt` (the denominator is preserved; spec §4.3, §4.8).

## Acceptance Criteria

### Tests That Must Pass

1. A supervisor smoke run whose child exits non-zero retains the output tree, raw stdout/stderr, and a `status.env` recording separate supervisor and child results — verified by inspecting the retained files after the run.
2. A second supervisor invocation against a populated output path does not overwrite the first attempt's retained evidence (non-overwriting paths).
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass (no Rust change here; this confirms the workspace is unaffected).

### Invariants

1. The supervisor's own wall timeout is recorded distinctly from the child's exit status, and evidence is retained on every termination class — a lost-evidence cancellation is recorded as a tool/infrastructure failure, never a pass.
2. The completion diagnostic's claims are each backed by a retained command, timing, resource datum, supervisor status, or fingerprint; no claim rests on assertion or extrapolation alone.

## Test Plan

### New/Modified Tests

1. `None — supervisor + diagnostic ticket; verification is command-based (supervisor smoke runs) and manual review against spec §4.6/§5.1. No `cargo` test target changes; the workspace four-gate is a regression sanity check only.`

### Commands

1. `bash tools/supervise-command.sh "$(mktemp -d)" 5 2 -- sh -c 'echo hi; exit 7'` then inspect the output dir — confirm stdout/stderr + `status.env` with `child_nonzero_exit` are retained.
2. `bash tools/supervise-command.sh "$(mktemp -d)" 1 1 -- sh -c 'sleep 30'` — confirm `wrapper_wall_timeout` is recorded distinctly and any partial output is retained.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked` — workspace unaffected. (A `shellcheck tools/supervise-command.sh` pass is recommended where `shellcheck` is available; state its absence as the reason if it is not installed on the runner.)
