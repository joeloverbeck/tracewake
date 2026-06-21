# 0045 FIRST-PROOF-CERT mutation lane completion diagnostic

**Status**: development diagnostic for `archive/tickets/0045FIRPROCER-002.md`; non-certifying.
**Date**: 2026-06-21
**Scope**: explains the 0044 configured mutation-lane non-completion, records
the supervisor retention hardening evidence, and selects the completion topology
for the later sharded lane. This report does not run or certify the final 0045
mutation campaign.

## Evidence Inputs

Retained 0044 evidence used:

- `reports/0044_first_proof_cert_mutation_full.supervisor/command.txt`
- `reports/0044_first_proof_cert_mutation_full.supervisor/metadata.txt`
- `reports/0044_first_proof_cert_mutation_full.supervisor/status.env`
- `reports/0044_first_proof_cert_mutation_full.supervisor/stdout.txt`
- `reports/0044_first_proof_cert_mutation_full.supervisor/stderr.txt`
- `reports/0044_first_proof_cert_mutation_full.out/mutants.out/{caught,missed,timeout,unviable}.txt`
- `reports/0044_first_proof_cert_mutation_full.out/mutants.out/{mutants,outcomes}.json`
- `reports/0044_first_proof_cert_mutation_focused.supervisor/status.env`
- `reports/0044_first_proof_cert_mutation_focused.out/mutants.out/{caught,missed,timeout,unviable}.txt`
- `reports/0044_first_proof_cert_mutation_list_files.txt`
- `reports/0044_first_proof_cert_mutation_list.txt`

Local orientation environment observed while authoring this diagnostic:

- `cargo 1.93.0 (083ac5135 2025-12-15)`
- `rustc 1.93.0 (254b59607 2026-01-19)`
- Linux WSL2 kernel `6.6.114.1-microsoft-standard-WSL2`
- `nproc`: 12
- memory: 19 GiB total, 15 GiB available at measurement time
- repository filesystem: 1007 GiB total, 627 GiB available at measurement time

These local measurements are development context only. The certifying run in
`0045FIRPROCER-005` must record its own final-run environment.

## 0044 Symptom Record

The retained full configured command was:

```text
cargo mutants --workspace --no-shuffle -o reports/0044_first_proof_cert_mutation_full.out
```

The supervisor metadata records a 7,200 second wall with 30 second grace. The
status file records:

```text
exit_status=124
supervisor_result=wrapper_wall_timeout
```

The retained stdout/stderr show:

- `Found 2901 mutants to test`
- unmutated baseline succeeded in `21s build + 36s test`
- stderr recorded `ERROR interrupted` and `scenario execution internal error err=interrupted phase=Build`

The retained outcome files classify 2,384 of 2,901 generated identities:

| Outcome | Count |
|---|---:|
| caught | 1,869 |
| missed | 0 |
| mutant-level timeout | 0 |
| unviable | 515 |
| unclassified at wrapper timeout | 517 |

The focused non-certifying wave completed separately with `child_exit_0` over
719 focused mutants: 600 caught, 0 missed, 0 timeout, 119 unviable. That focused
result is useful development evidence but does not classify the standing
denominator.

## Timing And Progress

The full retained `outcomes.json` contains 2,385 outcome entries: one baseline
entry plus 2,384 mutant outcomes. The canonical `mutants.json` contains 2,901
generated identities.

Measured baseline phases from retained `outcomes.json`:

| Phase | Duration |
|---|---:|
| Build | 21.71752344 s |
| Test | 36.509925377 s |

Measured per-outcome phase-duration totals in the classified portion:

| Summary | Count | Phase-duration sum |
|---|---:|---:|
| CaughtMutant | 1,869 | 6,717.694 s |
| Unviable | 515 | 388.117 s |

Largest retained per-file phase-duration sums in the classified portion:

| File | Classified count | Phase-duration sum |
|---|---:|---:|
| `crates/tracewake-core/src/scheduler.rs` | 203 | 676.909 s |
| `crates/tracewake-core/src/events/apply.rs` | 173 | 574.816 s |
| `crates/tracewake-core/src/epistemics/knowledge_context.rs` | 169 | 540.272 s |
| `crates/tracewake-core/src/epistemics/projection.rs` | 143 | 463.609 s |
| `crates/tracewake-core/src/projections.rs` | 123 | 426.407 s |
| `crates/tracewake-core/src/agent/actor_known.rs` | 79 | 405.259 s |
| `crates/tracewake-core/src/agent/trace.rs` | 146 | 372.716 s |
| `crates/tracewake-core/src/debug_reports.rs` | 53 | 307.696 s |

The historical scalar is approximately `2384 / 7200 = 0.331` classified
mutants per wall-clock second. This is orientation only, not a prediction:
the retained log shows skew by file and by mutant cost, plus one 73.919 s
classified mutant in `debug_reports.rs`.

## Bottleneck Assessment

The 0044 failure is denominator completion under a single wrapper wall, not a
known semantic survivor floor.

Observed bottleneck signals:

- the unmutated baseline succeeded quickly enough to interpret later outcomes;
- no retained `missed` or mutant-level `timeout` rows exist in the classified
  portion;
- the wrapper interrupted a build phase while the campaign was still running;
- `debug.log` records cargo-mutants using a 12-task jobserver on this machine;
- the classified set was large enough that the single process consumed the
  entire 7,200 second external wall before reaching the final 517 identities.

Most likely bottleneck: serialized denominator size plus skewed per-mutant
build/test cost inside one externally supervised invocation. The retained
evidence does not show a semantic survivor bottleneck, a failing baseline, or a
mutant-level timeout bottleneck. It also does not prove that simply raising the
single-process wall would be reliable, because the remaining 517 identities were
never classified and the next run could shift cost after source changes.

## Alternatives Evaluated

| Alternative | Diagnostic result |
|---|---|
| Re-run one unsharded command with a larger wall | Rejected as the primary design. It repeats the 0044 single-process failure shape and gives no independent proof of denominator completion until the end. |
| Increase local `--jobs` | Rejected as the primary design. The retained run already used 12 local tasks; more local concurrency risks memory/build thrash and spurious timeouts. |
| Conservative local `--jobs` with deterministic shards | Selected. Shards reduce per-job denominator size, allow independent retention, and enable exact set-union reconciliation. |
| Larger/self-hosted runner only | Deferred. It may help, but topology should not depend on an unrecorded machine upgrade. |
| Focused filters or `--in-diff` | Rejected for certification. They are development/change-detection evidence only and do not define the standing denominator. |

## Selected Topology For `0045FIRPROCER-004`

The scheduled/manual lane should use:

- dedicated same-commit baseline job;
- deterministic `N = 8` shard matrix, indices `0/8` through `7/8`;
- `MUTANTS_JOBS = 2` per shard unless a later checked-in diagnostic update
  records stronger runner evidence;
- each shard launched through `tools/supervise-command.sh`;
- shard supervisor wall: `7200` seconds;
- shard supervisor grace: `120` seconds;
- GitHub job `timeout-minutes`: at least `130`;
- `strategy.fail-fast: false`;
- unique shard output and artifact names;
- `if: always()` upload for every shard artifact; and
- a final `if: always()` reconciliation job invoking the 0045 merger.

Why this has margin: it keeps the historical full-run wall budget per shard
while each shard owns roughly one eighth of the canonical identity set. Even
allowing for skew, the largest retained per-file phase-duration sum
(`scheduler.rs`, 676.909 s in classified outcomes) is far below the 7,200 s
per-shard supervisor wall. The topology also reduces local contention by using
two cargo-mutants jobs per shard instead of trying to intensify the already
12-task single host run.

This topology is wired by archived ticket `0045FIRPROCER-004`, but remains
non-certifying until `0045FIRPROCER-005` runs the complete campaign at the final
implementation / evidence commit.

## Completion Risk Calculation

Historical full-run wall: 7,200 s for 2,384 classified identities before
wrapper timeout. A uniform eight-shard split would average approximately 363
generated identities per shard for the 2,901-row authoring-baseline population.
The chosen per-shard wall remains 7,200 s, giving a large budget above the
historical average shard share.

Risk controls:

- `N = 8` reduces the denominator per job without shrinking the canonical
  population;
- `MUTANTS_JOBS = 2` avoids relying on high local parallelism for margin;
- 120 s supervisor grace is reserved for graceful termination and artifact
  synchronization;
- 130 GitHub job minutes leaves CI headroom beyond the 7,200 s supervised wall
  plus retention work;
- `fail-fast: false` prevents one shard's miss/failure from canceling sibling
  evidence collection; and
- the merger must fail the aggregate lane if any shard is absent, incomplete,
  duplicated, mismatched, or supervisor-failed.

Retry cost is bounded to the failed shard plus reconciliation for development
diagnosis, but certification still requires one complete final accepted union at
one exact commit.

## Supervisor Retention Evidence

`tools/supervise-command.sh` now preserves the existing CLI:

```text
tools/supervise-command.sh <output-dir> <wall-seconds> <grace-seconds> -- <command> [args...]
```

The supervisor now launches the child without a PTY in its own process group,
records command/cwd/environment/timestamps/PIDs/status fields, separates
supervisor result from child status, terminates the process group on wall
timeout, preserves stdout/stderr/status on every tested non-normal path, and
uses a non-overwriting `.attempt-N` output directory when the requested output
path is already populated.

Development smoke results from 2026-06-21:

| Smoke | Expected | Observed |
|---|---|---|
| child prints output and exits 7 | retained stdout/stderr/status, `child_nonzero_exit` | `exit_status=7`, `supervisor_result=child_nonzero_exit`, partial output copied |
| child writes partial output and sleeps past wall | retained stdout/status, wall timeout distinct from child signal | `exit_status=124`, `supervisor_result=wrapper_wall_timeout`, `child_exit_status=143`, partial output copied |
| second invocation against populated output path | no overwrite | original file remained in requested path; new evidence went to `.attempt-1` |

`shellcheck` was not available in the local environment (`command -v shellcheck`
returned no path), so shell verification used `bash -n tools/supervise-command.sh`
plus the smoke commands above.

## Outcome Vocabulary Checks

The hardened supervisor keeps the required axes separate:

- supervisor result: `child_exit_0`, `child_nonzero_exit`,
  `wrapper_wall_timeout`, `wrapper_forced_kill`, `supervisor_canceled`, or
  `supervisor_or_spawn_failure`;
- child status: `child_exit_status` and optional `child_signal`;
- retention status: `artifact_copy_status` and `artifact_copy_error`;
- mutation denominator completion: left to the later merger, not inferred from
  supervisor exit alone; and
- per-mutant outcomes: left in cargo-mutants output files.

A wrapper wall timeout remains distinct from cargo-mutants exit `3` and from a
per-mutant timeout row. A nonzero child result is retained and recorded; it is
not converted into a pass.

## Acceptance Test For Later Merger

The later merger must accept a run only when every shard has a normal supervisor
result, matching commit/config/toolchain fingerprints, complete assigned and
outcome identity sets, pairwise disjoint shard sets, exact union with the
canonical list, and zero final `missed` and zero final mutant-level `timeout`
rows. It must reject missing shards, duplicate indices, overlapping identities,
mismatched fingerprints, malformed/truncated output, non-normal supervisor
results, and text/JSON outcome disagreement.

## Certification Status

This diagnostic is not certifying evidence for `FIRST-PROOF-CERT passed`.
It supports the later lane implementation and final campaign run only.
