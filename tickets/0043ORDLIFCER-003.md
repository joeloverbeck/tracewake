# 0043ORDLIFCER-003: Diagnose the configured mutation lane's PTY non-completion and establish a transport-honest supervised launch

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — diagnostic report, supervised-launch mechanism, and regression controls; no production/simulation logic change.
**Deps**: 0043ORDLIFCER-001

## Problem

The 0042 configured mutation lane did not complete: `cargo mutants --workspace --no-shuffle` left its PTY wrapper active after no cargo-mutants process was visible and was interrupted; a deterministic `-j 8` rerun likewise failed to produce a complete summary. A process-list observation is not completion proof, and an interrupted wrapper is not a cargo-mutants outcome. Spec §3.2 elevates lane completion to a co-equal blocker alongside the survivor kills: the center of gravity is `kill + COMPLETE + reconcile + triage + re-prove`.

This ticket diagnoses the non-completion to a reproducible root cause or a bounded external-environment diagnosis, and establishes a transport-honest, non-PTY supervised launch (with regression controls proving the supervisor terminates deterministically) that ticket -004 then uses to run the campaign to completion.

(spec §3.2, §4.4, §4.5, §5.6)

## Assumption Reassessment (2026-06-20)

1. The historical failure is recorded in `reports/0042_ord_life_cert_mutation_triage_register.md` (verified this session): the `--workspace --no-shuffle` lane and the `-j 8` rerun are both logged `tool-failure/incomplete; interrupted after no cargo-mutants process was visible`, with retained transcripts `/tmp/0042-015-mutants-no-shuffle.txt` and `/tmp/0042-015-mutants-no-shuffle-j8.txt`. The configured census is 60 files / 2877 mutants. These `/tmp` transcripts may not survive into this session; the diagnostic references them as historical evidence, not as reproducible inputs.
2. The standing config `.cargo/mutants.toml` pins `additional_cargo_args = ["--locked"]`, `test_workspace = true`, and the 60-file `examine_globs` seam; the scheduled `mutants-lock-layer` CI job runs `cargo mutants --workspace --no-shuffle` and uploads `mutants.out` under `if: always()` (verified this session). cargo-mutants is pinned to 27.1.0. The final certifying run uses this config, so the supervised launch must be validated with it in effect — hence `Deps: -001`, which settles the perimeter/CI posture first.
3. Cross-artifact boundary under audit: the process-supervision lane around cargo-mutants — launcher/supervisor → cargo-mutants child/process-group → `mutants.out` (`lock.json`, `mutants.json`, `outcomes.json`, `caught.txt`/`missed.txt`/`timeout.txt`/`unviable.txt`, logs) — and the requirement to keep launcher result, denominator completion, cargo-mutants run exit, per-mutant outcome, certification disposition, and evidence status as separate axes (spec §4.5). A `std::process::Child` dropped without wait is not reaped; GNU `timeout` has distinct timeout/failure statuses; `script -e/--return` propagates child status.
4. Invariant/rule motivation: INV-098 (harsh feature acceptance — completion must be proven, not inferred) and the execution mutation-completion / evidence-honesty doctrine (`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`). Mutation-infrastructure/configured-perimeter completion is a transport/tooling responsible layer, not a new canonical gate code (spec §3.5, §6.2).
5. Enforcement surface (evidence-consumer/substrate basis): this ticket produces the launch mechanism whose output ticket -004 consumes as certifying evidence. It must guarantee the supervised launch introduces no nondeterminism into the certifying denominator (same commit/config/version/args/shuffle posture/timeout policy), retains partial outputs on every failure, and never classifies "no cargo-mutants process visible" as completion. It touches no simulation-world replay/leak/validation surface; the determinism it protects is the mutation evidence's, supplied to -004's reconciliation and -005's verdict.

## Architecture Check

1. Diagnosing to a reproducible root cause (or a bounded external-environment diagnosis) plus a non-PTY supervised launch is cleaner than retrying the failed wrapper: a generic "unknown PTY issue" cannot certify, and re-running the same wrapper reproduces the same incomplete outcome. A process-group supervisor with explicit wait/kill-then-wait and distinct termination statuses makes completion provable rather than inferred.
2. No backwards-compatibility aliasing/shims; no mutation-exclusion tricks, no perimeter shrinkage, no `#[mutants::skip]` to dodge the failing lane. If the historical wrapper is external and uninspectable, it is identified as the non-certifying failing layer, its historical evidence retained, and it is permanently bypassed for the certifying lane (spec §4.4) — not hidden.

## Verification Layers

1. Completion is proven, not inferred (INV-098) → manual review + command transcript: the diagnostic records command tree, child PID/process group, parent/wrapper PID, start/end timestamps, signals, exit statuses, and output paths per reproduction attempt, and a direct/replacement-supervised launch that completes where the historical wrapper did not.
2. Supervisor terminates deterministically → regression control: a normally-exiting child, a nonzero child, a deliberately-stalled child, and a killed child each terminate the supervisor and produce distinct, recorded statuses (launcher result axis kept separate from cargo-mutants exit 3).
3. Transport honesty → manual review: the diagnostic distinguishes launcher/supervisor result, denominator completion, cargo-mutants run exit, per-mutant outcome, and certification disposition (spec §4.5 axes), and states explicitly that no process-list heuristic was used as completion proof.
4. Artifact retention → codebase grep-proof: partial `mutants.out`/logs are copied under `always`/finally behavior even when the launcher/wrapper/baseline/shard fails; retained paths are stable and fingerprinted.

## What to Change

### 1. Lane-completion diagnostic report

Author `reports/0043_ord_life_cert_mutation_lane_completion_diagnostic.md` recording (spec §4.4): the exact historical symptom and retained 0042 transcript references; whether the PTY wrapper was repository/host/CI/session-tool owned; per reproduction attempt the command tree, PIDs/process group, timestamps, signals, exit statuses, and output paths; whether cargo-mutants exited, was killed, blocked, or lost its controlling terminal; the actual failure point (stdout/stderr closure, child reaping, pipeline status, or artifact copying); whether `mutants.out` files existed and advanced; a minimal reproducer where the problem is repository/wrapper owned; and the corrective change or the exact external-environment bypass used for certification. A generic "unknown PTY issue" is insufficient.

### 2. Non-PTY supervised launch + wall-clock supervisor

Establish the certifying launch without a PTY (PTY only for diagnosis or where a platform absolutely requires it; if `script` is used, `script -e/--return` must propagate child status and the raw typescript is retained). A repository-owned Rust launcher must explicitly wait-for or kill-then-wait every child. A wall-clock supervisor must supervise the process group, send graceful-then-recorded-forced-kill after a bounded grace period, preserve a wrapper-timeout/kill status distinct from cargo-mutants exit 3, copy partial `mutants.out`/logs under finally behavior, record why termination occurred, and never classify "no cargo-mutants process visible" as completion. GNU `timeout` is acceptable if its exact invocation is recorded and `--preserve-status` is not used in a way that erases the wrapper-timeout distinction.

### 3. Supervisor regression controls

Add regression controls (transcripts retained under `reports/0043_ord_life_cert_command_transcripts/`) proving a normally-exiting child, a nonzero-exit child, a deliberately-stalled child, and a killed child each terminate the supervisor with distinct statuses.

## Files to Touch

- `reports/0043_ord_life_cert_mutation_lane_completion_diagnostic.md` (new)
- `reports/0043_ord_life_cert_command_transcripts/` (new — supervisor regression + reproduction transcripts; `test -f` validated at parent `reports/`)
- `<repository-owned launcher/supervisor script or harness>` (new — only if the corrective change is repository-owned rather than a pure external-environment bypass; path chosen per the diagnostic, as surfaced)

## Out of Scope

- Running the full configured campaign to completion and producing the triage register (→ -004); this ticket establishes and proves the launch mechanism, it does not run the certifying campaign.
- The CI in-diff trigger / guard convergence (→ -001) and the survivor kills (→ -002).
- Live ORD-LIFE re-proof and the replacement acceptance artifact (→ -005).
- Any perimeter shrinkage, `#[mutants::skip]`, or filter trick to make the lane "complete" (forbidden — spec §4.3, §4.15).
- Minting a new gate/status code for mutation-infrastructure completion (it is a transport/tooling responsible layer only — spec §3.5).

## Acceptance Criteria

### Tests That Must Pass

1. The diagnostic report records a reproducible root cause OR a bounded external-environment diagnosis (the same commit/command/config/environment completes under a direct or replacement-supervised launch), with the historical wrapper identified as the non-certifying failing layer if uninspectable.
2. Regression controls demonstrate normal-exit, nonzero-exit, stalled, and killed children each terminate the supervisor with distinct recorded statuses.
3. The launch is non-PTY for certification; if `script` is used, child status is propagated and the typescript retained.
4. Partial `mutants.out`/logs are retained on launcher/wrapper/baseline/shard failure at stable, fingerprinted paths.

### Invariants

1. Completion is proven by terminated-supervisor + retained outputs, never by a process-list heuristic (spec §4.4, §5.6).
2. Launcher result, denominator completion, cargo-mutants run exit, per-mutant outcome, and certification disposition remain separate axes; a wrapper wall-clock timeout is never recorded as a cargo-mutants per-mutant `timeout` (spec §4.5).

## Test Plan

### New/Modified Tests

1. `None — diagnostic/evidence ticket; verification is command-based (supervisor regression transcripts) and review-based, with no `cargo` unit test added.` The supervisor regression controls are runnable shell/launcher checks whose transcripts are retained under `reports/0043_ord_life_cert_command_transcripts/`.

### Commands

1. Supervisor regression: run the four controlled children (normal-exit / nonzero-exit / stalled / killed) under the supervisor and confirm four distinct recorded statuses; retain transcripts.
2. Direct/replacement-supervised non-PTY launch of `cargo mutants --workspace --no-shuffle` (with `.cargo/mutants.toml` in effect) reaches a terminal, complete summary where the historical wrapper did not — or, if external, the bounded-diagnosis equivalent is recorded. (The full certifying run and its reconciliation are -004's scope.)
3. `grep -RIl . reports/0043_ord_life_cert_command_transcripts/` — confirm regression/reproduction transcripts are retained at stable paths.
