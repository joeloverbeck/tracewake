# 0050 Foundational Conformance Second Hardening Acceptance

## Status

Scoped spec-0050 hardening accepted for the implementation/evidence line ending
at `57d1be0ce5873c6c3f05f949a4cc36ef087cecf7` plus this report commit.

This is not latest-main certification, not Phase-4 entry, not second-proof
entry, and not a green standing mutation-perimeter certification. The
configured standing mutation campaign recorded 48 missed mutants and one
timeout; those findings are preserved below as follow-up remediation evidence,
not converted into a pass.

## Scope

Spec-0050 re-audited the 0047/0048 loaded-world/time-control seam and the
0049MUTWIT mutation-witness line. The driver findings were:

| Finding | Result |
|---|---|
| F-01 production world steps did not derive loaded-world due actors | Closed by `0050FOUCONSEC-001` and `0050FOUCONSEC-003`; `due_loaded_actor_ids` is core-owned and wired into `transact_world_one_tick`. |
| F-02 world processes crossed the request boundary as raw envelopes | Closed by `0050FOUCONSEC-002` and `0050FOUCONSEC-003`; declared process cadence produces private due invocations. |
| F-03 actor transaction outcome was partially consumed | Closed by `0050FOUCONSEC-005`; the coordinator consumes `ActorDecisionTransactionOutcome` variants and pipeline results. |
| F-04 TUI authored post-step perception and interval deltas | Closed by `0050FOUCONSEC-006`; TUI consumes the core interval product read-only. |
| F-05 temporal replay violations did not fail aggregate replay | Closed by `0050FOUCONSEC-008`; `run_replay` reports temporal violations as `matches_expected == false`. |
| F-06 salient stop accepted routine observation churn | Closed by `0050FOUCONSEC-009`; salience is typed and discriminates quiet, novel, hidden, and replay cases. |
| F-07 0048 evidence overstated production reachability | Closed by `0050FOUCONSEC-010`; replacement evidence exercises production-shaped world steps, parity, replay, and compile-fail boundaries. |
| F-08 0049MUTWIT lacked a source-discipline record | Closed by `0050FOUCONSEC-012`; `docs/4-specs/SPEC_LEDGER.md` now records the line as test-only mutation-witness remediation. |

## Evidence Matrix

| Evidence class | Command or artifact | Result |
|---|---|---|
| Workspace format | `cargo fmt --all --check` | Pass, exit 0. |
| Workspace lint | `cargo clippy --workspace --all-targets -- -D warnings` | Pass, exit 0. |
| Workspace build | `cargo build --workspace --all-targets --locked` | Pass, exit 0. |
| Workspace tests | `cargo test --workspace --locked` | Pass, exit 0. |
| Focused core suites | `cargo test -p tracewake-core --test world_step_coordinator --test replay_temporal_frontier --test holder_known_interval_projection --test salient_stop_actor_known --test reservation_body_exclusive_census --test generative_lock --test negative_fixture_runner` | Pass, exit 0. |
| Focused TUI suites | `cargo test -p tracewake-tui --test playable_capability_parity --test parity_adversarial --test tui_seam_conformance --test command_loop_session --test embodied_flow` | Pass, exit 0. |
| Replay/golden lanes | `cargo test -p tracewake-content --test golden_fixtures_run && cargo test -p tracewake-core --test replay_temporal_frontier --test golden_scenarios --test event_schema_replay_gates` | Pass, exit 0. |
| Documentation truthing | `grep -nE "0049MUTWIT" docs/4-specs/SPEC_LEDGER.md`; `git diff --name-only -- docs/0-foundation/`; `grep -rL . docs/0-foundation/ >/dev/null`; `grep -nE "R-2[789]" docs/3-reference/01_DESIGN_RISK_REGISTER.md`; `git diff --check`; `cargo test -p tracewake-core --test doc_invariant_references --test anti_regression_guards` | Pass; no foundation-tier edits; R-27/R-28/R-29 updated without minting a new risk ID. |

## Behavior Witnesses

- Production loaded-world derivation and transaction choreography:
  `world_step_coordinator.rs` passed 22 tests, including due actor/process
  discovery, human/no-human differential, event-id uniqueness across intervals,
  duration discovery, malformed due-duration rejection, stale expected-tick
  rejection, and replay frontier checks.
- Deterministic generated evidence:
  `generative_lock.rs` passed 6 tests covering mixed schedules, prefix replay,
  order permutation determinism, sampled metadata, targeted duration tamper,
  and duplicate marker append fail-closed behavior.
- Replay temporal fail-closed behavior:
  `replay_temporal_frontier.rs` passed 13 tests, including the aggregate
  `run_replay_temporal_violation_fails_aggregate_and_reports_typed_first_divergence`
  witness.
- Holder-known interval and salience behavior:
  `holder_known_interval_projection.rs` passed 4 tests and
  `salient_stop_actor_known.rs` passed the four-case quiet/novel/hidden/replay
  policy witness.
- Compile-time boundaries:
  `negative_fixture_runner.rs` passed 5 tests, including registered directory
  census, clippy-ban coverage, debug-report capability construction failure,
  actor-step outcome boundary, due-work injection, and TUI perception export
  negative fixtures.
- TUI parity and adversarial evidence:
  `playable_capability_parity.rs` passed 9 tests and
  `parity_adversarial.rs` passed 8 tests, including missing actor invocation,
  missing world-process invocation, hidden same-actor source, removed temporal
  marker, witness-drop, uncovered action, debug hidden-truth leak, and source
  guard target removal failure cases.

## Evidence-Honesty Negatives

The capstone did not edit production logic to create temporary manual breaks.
It relies on the committed adversarial and negative witnesses that mutate the
production evidence contract or production source shape under test:

| Required break class | Witness |
|---|---|
| Disable actor discovery | `parity_adversarial_missing_actor_invocation_fails_typed_evidence`; `authoritative_loaded_world_differential_is_non_vacuous`. |
| Disable process discovery | `parity_adversarial_missing_world_process_fails_no_human_evidence`; `transactional_world_step_applies_due_world_process_event`. |
| Drop decision-trace/typed actor evidence | `parity_adversarial_witness_drop_fails_runner`; actor transaction outcome tests covered by `world_step_coordinator.rs` and `cargo test --workspace --locked`. |
| Swallow `Stuck`/typed failure evidence | Workspace and golden lanes include stuck/failure diagnostics such as `routine_blocked_fixture_records_access_failure_without_silent_loop`; the closed outcome boundary is also compile-fail guarded by `negative_fixture_runner.rs`. |
| Re-enable TUI post-step perception or client event writing | `external_crate_cannot_call_tui_perception_append_helper` via `negative_fixture_runner.rs`; `consuming_core_interval_product_is_read_only` via `cargo test --workspace --locked`; `source_scan_smoke_tui_does_not_call_event_applier`. |
| Omit temporal failure from replay success | `parity_adversarial_removed_temporal_marker_fails_replay_evidence`; `run_replay_temporal_violation_fails_aggregate_and_reports_typed_first_divergence`. |
| Collapse salience novelty/materiality | `typed_salience_policy_distinguishes_quiet_novel_hidden_and_replay_cases`; focused salience mutation campaign recorded in `0050FOUCONSEC-011`. |

## Mutation Evidence

`archive/tickets/0050FOUCONSEC-011.md` is the durable mutation transcript
summary for this package.

Preserved 0049 focused reruns:

| Campaign | Denominator | Caught | Unviable | Missed |
|---|---:|---:|---:|---:|
| Scheduler 0049 functions | 55 | 50 | 5 | 0 |
| Projection/source-key 0049 functions | 15 | 14 | 1 | 0 |
| Replay/pipeline 0049 functions | 36 | 35 | 1 | 0 |

0050 focused campaigns:

| Campaign | Denominator | Caught | Unviable | Missed |
|---|---:|---:|---:|---:|
| Scheduler branches | 30 | 26 | 4 | 0 |
| Event log identity | 8 | 6 | 2 | 0 |
| Replay temporal verdict | 28 | 27 | 1 | 0 |
| Interval salience | 16 | 14 | 2 | 0 |

Standing configured perimeter:

- Command: `cargo mutants --timeout 183`
- Selected/current denominator: 3,182 mutants.
- Result: 2,458 caught, 675 unviable, 48 missed, 1 timeout.
- Verdict: non-green standing perimeter. The missed set and timeout are real
  missing mutation evidence and must route to separate survivor remediation
  before any artifact claims the standing perimeter is green.

## Documentation and Source Discipline

`0050FOUCONSEC-012` updated the live conformance/evidence homes after the
executable evidence existed:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` now cites 0050
  production-path witnesses for loaded-world temporal conformance.
- Execution docs `05`, `06`, `07`, and `10` identify the core-owned
  actor/process discovery, actor-transaction artifacts, read-only interval
  product, separated evidence classes, and non-green standing mutation status.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` updates existing R-27/R-28/R-29
  evidence/status only.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` names the live
  executable evidence for loaded-world/time-control review prompts.
- `docs/4-specs/SPEC_LEDGER.md` records `0049MUTWIT` as a ticket-only
  mutation-witness follow-up implemented by
  `archive/tickets/0049MUTWIT-001.md`,
  `archive/tickets/0049MUTWIT-002.md`, and
  `archive/tickets/0049MUTWIT-003.md`.

No `docs/0-foundation/` files were modified for this package. No archived
0048 or 0049 artifact was rewritten.

## Verdict

Spec-0050's implementation, focused evidence, replay/golden evidence,
compile-fail boundaries, TUI parity/adversarial evidence, and documentation
truthing are accepted for the scoped loaded-world/time-control hardening line
at the implementation/evidence commit named above.

The acceptance is deliberately scoped:

- It supersedes the 0048 overbroad production-reachability evidence for this
  seam.
- It preserves the 0049MUTWIT focused mutation witnesses and gives them a
  source/navigation record.
- It does not certify latest main or any future feature surface.
- It does not certify the standing mutation perimeter as green because the
  configured campaign recorded 48 missed mutants and one timeout.

Final spec closeout must add the archived 0050 row to
`docs/4-specs/SPEC_LEDGER.md`, archive this report to `archive/reports/`, retarget
references from `reports/0050_foundational_conformance_second_hardening_acceptance.md`
to the archived report path, and move the spec to `archive/specs/`.
