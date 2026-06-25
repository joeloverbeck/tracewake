# 0051 Foundational Conformance Third Hardening Acceptance

Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`.

This is a scoped acceptance artifact for spec `0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_PRODUCTION_REACHABILITY_PROCESS_TRANSACTIONS_ACTOR_CENSUS_AND_TUI_DEAUTHORITY_HARDENING_SPEC.md`. It accepts the 0051 remediation line only at the exact implementation commit named below. It is not latest-main certification, not full-project certification, not Phase 4 entry, and not a standing mutation green claim.

## Exact implementation commit under test

- Commit: `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`
- Branch or PR: local `main` at detached clean verification worktree `/tmp/tracewake-0051-acceptance`
- Verification worktree: clean `git status --short` before gate and mutation evidence collection
- Main checkout note: unrelated pre-existing local changes in `.claude/`, `reports/0047-*`, `reports/manifest_2026-06-24_0429a8f.txt`, `specs/`, and the active `tickets/0051FOUCONTHI-012.md` were excluded from this artifact's clean-worktree evidence.

## Gates run

All gates below were run in the clean worktree at `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`.

- `cargo fmt --all --check` — passed; no formatting diff.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed; checked `tracewake-core`, `tracewake-content`, and `tracewake-tui`.
- `cargo build --workspace --all-targets --locked` — passed; built all workspace targets with the lockfile.
- `cargo test --workspace` — passed; all workspace unit, integration, and doc tests completed with zero failures.

## Changed files

Implementation, evidence, and archival changes under review:

- `archive/tickets/0051FOUCONTHI-001.md` through `archive/tickets/0051FOUCONTHI-011.md`
- `.github/workflows/ci.yml`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-core/src/runtime/session.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/tests/*` files touched by tickets `-002` through `-010`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/tests/*` files touched by tickets `-007` through `-009`
- `docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*` status/evidence rows touched by `-011`
- `reports/0051_foundational_conformance_third_hardening_acceptance.md`

Unrelated local worktree changes are not included.

## Parity evidence block

- Target implementation commit: `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`
- Fixture/content fingerprints: content loader handoff evidence from `load::tests::loaded_fixture_hands_off_derived_runtime_due_work`; runtime construction evidence from `generated_cases_enter_through_loaded_runtime_constructor`.
- Capability entries in scope: loaded-world runtime construction, human wait/continue world advance, no-human advancement, declared process invocation, actor-known interval output, and TUI read-only interval rendering.
- Generated coverage report: no new generated report path; certifying coverage is the named clean-worktree command ledger and existing checked-in generative lock corpus.
- Typed causal witnesses: `LoadedWorldRuntime::from_loaded_world`, `LoadedWorldRuntime::wait_one_tick`, `DeterministicScheduler::transact_world_one_tick`, `due_loaded_actor_ids`, `due_process_invocations`, `ActorDecisionTransactionOutcome`, `DeclaredWorldProcessApplied`, `TimeAdvanced`, `ReplayTemporalVerdict`, and `TypedActorKnownIntervalSummary`.
- Actor-known witnesses: `holder_known_interval_projection.rs`, `salient_stop_actor_known.rs`, and the `-010` same-source/same-subject observation replacement witness.
- Rendered golden paths/digests: TUI parity and command-loop tests (`playable_capability_parity.rs`, `parity_adversarial.rs`, `command_loop_session.rs`, `tui_acceptance.rs`) remain the rendered/read-only evidence.
- Anti-leak and debug-quarantine evidence: `negative_fixture_runner.rs`, external negative fixtures, `embodied_flow.rs`, `tui_seam_conformance.rs`, `adversarial_gates.rs`, and `-008` sealed split temporal products.
- Replay/no-human disposition: replay temporal frontier, world-step coordinator, golden fixture, no-human capstone, and generative lock tests passed in `cargo test --workspace`.
- Compiler/source-conformance evidence: `negative_fixture_runner.rs`, `ci_workflow_guards.rs`, doc invariant reference tests, and focused mutation campaigns below.
- Exact commands and verdicts: all four gates passed; all focused mutation campaigns had zero missed selected mutants; the standing campaign completed with 23 misses and is not counted as green.

## Mutation command ledger

All mutation commands below were run in the clean worktree at `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`.

Focused campaigns:

| Command | Result |
|---|---|
| `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -F 'insert_observation|embodied_subject_key' --test-package tracewake-core --timeout 183` | Found 6 mutants; baseline `9s build + 56s test`; `6 mutants tested in 82s: 6 caught`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'latest_current_place_perception_event_id|latest_need_event_id|actor_has_open_body_exclusive_at|append_decision_trace_after_proposal|transact_world_one_tick' --test-package tracewake-core --timeout 183` | Found 56 mutants; baseline `9s build + 58s test`; `56 mutants tested in 5m: 51 caught, 5 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|VerifiedActorKnownIntervalNotice::source_key' --test-package tracewake-core --timeout 183` | Found 15 mutants; baseline `9s build + 56s test`; `15 mutants tested in 2m: 14 caught, 1 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/replay/temporal.rs -f crates/tracewake-core/src/replay/rebuild.rs -f crates/tracewake-core/src/actions/pipeline.rs -F 'validate_time_advanced|rebuild_projection|is_duplicate_need_tick_candidate' --test-package tracewake-core --timeout 183` | Found 36 mutants; baseline `9s build + 55s test`; `36 mutants tested in 4m: 35 caught, 1 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'due_loaded_actor_ids|due_process_invocations|transact_world_one_tick|build_advance_until_result|actor_known_interval_delta_is_salient|step_appended_possessed_duration_terminal' --test-package tracewake-core --timeout 183` | Found 31 mutants; baseline `9s build + 55s test`; `31 mutants tested in 5m: 27 caught, 4 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/events/log.rs -F 'append' --test-package tracewake-core --timeout 183` | Found 8 mutants; baseline `8s build + 57s test`; `8 mutants tested in 79s: 6 caught, 2 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/replay/report.rs -f crates/tracewake-core/src/replay/temporal.rs -F 'run_replay|validate_time_advanced' --test-package tracewake-core --timeout 183` | Found 28 mutants; baseline `8s build + 55s test`; `28 mutants tested in 3m: 27 caught, 1 unviable`. |
| `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|is_salient|salience' --test-package tracewake-core --timeout 183` | Found 16 mutants; baseline `9s build + 56s test`; `16 mutants tested in 2m: 14 caught, 2 unviable`. |

Standing campaign:

- `cargo mutants --timeout 183` — baseline `10s build + 60s test`; selected 3275 mutants; exit 2; `3275 mutants tested in 3h: 23 missed, 2549 caught, 703 unviable`.
- Artifact counts: `2549 mutants.out/caught.txt`, `23 mutants.out/missed.txt`, `703 mutants.out/unviable.txt`, `0 mutants.out/timeout.txt`.
- Verdict: completed and disposition recorded; not green.

Standing missed mutants:

```text
crates/tracewake-core/src/projections.rs:260:5: replace food_source_fact_supersedes -> bool with true
crates/tracewake-core/src/projections.rs:260:5: replace food_source_fact_supersedes -> bool with false
crates/tracewake-core/src/projections.rs:261:9: delete match arm (Some(_), None) in food_source_fact_supersedes
crates/tracewake-core/src/projections.rs:262:9: delete match arm (None, Some(_)) in food_source_fact_supersedes
crates/tracewake-core/src/projections.rs:263:37: replace < with == in food_source_fact_supersedes
crates/tracewake-core/src/projections.rs:263:37: replace < with > in food_source_fact_supersedes
crates/tracewake-core/src/projections.rs:263:37: replace < with <= in food_source_fact_supersedes
crates/tracewake-core/src/scheduler.rs:550:9: replace DeterministicScheduler::restore_from_temporal_projection -> Option<Self> with None
crates/tracewake-core/src/view_models.rs:143:9: replace EmbodiedViewModel::sim_tick -> SimTick with Default::default()
crates/tracewake-core/src/view_models.rs:253:9: replace TypedActorKnownIntervalSummary::start_tick -> SimTick with Default::default()
crates/tracewake-core/src/view_models.rs:257:9: replace TypedActorKnownIntervalSummary::stop_tick -> SimTick with Default::default()
crates/tracewake-core/src/view_models.rs:261:9: replace TypedActorKnownIntervalSummary::start_frontier -> u64 with 0
crates/tracewake-core/src/view_models.rs:261:9: replace TypedActorKnownIntervalSummary::start_frontier -> u64 with 1
crates/tracewake-core/src/view_models.rs:265:9: replace TypedActorKnownIntervalSummary::stop_frontier -> u64 with 0
crates/tracewake-core/src/view_models.rs:265:9: replace TypedActorKnownIntervalSummary::stop_frontier -> u64 with 1
crates/tracewake-core/src/view_models.rs:277:9: replace TypedActorKnownIntervalSummary::no_new_actor_known_information -> bool with true
crates/tracewake-core/src/agent/trace.rs:846:13: delete match arm 24 in StuckDiagnostic::deserialize_canonical
crates/tracewake-core/src/agent/trace.rs:884:36: delete ! in StuckDiagnostic::deserialize_canonical
crates/tracewake-core/src/runtime/session.rs:168:9: replace LoadedWorldRuntime::assign_proposal_sequence -> crate::scheduler::ProposalSequence with Default::default()
crates/tracewake-core/src/runtime/session.rs:275:9: replace LoadedWorldRuntime::rebuild_from_owned_log -> Result<(), RuntimeCommandError> with Ok(())
crates/tracewake-core/src/runtime/session.rs:294:9: replace LoadedWorldRuntime::refresh_actor_current_place_perception with ()
crates/tracewake-tui/src/app.rs:259:33: delete ! in TuiApp::submit_entry_with_world_advance
crates/tracewake-tui/src/transcript.rs:42:50: replace == with != in capture_representative_transcript_sections
```

## Per-finding closure evidence

Each row records the production constructor, public command or boundary, observed effect, and sensitivity proof required by spec §4.9/§7.

| Finding | Closure evidence | Result |
|---|---|---|
| F-01 production loaded-world discovery | Production constructor: `LoadedWorldRuntime::from_loaded_world` / `RuntimeInitialState`. Public command/boundary: loaded fixture handoff and `LoadedWorldRuntime::wait_one_tick`. Observed effect: two loaded actors and declared process due work are derived by runtime without manual scheduler registration. Sensitivity proof: external negative fixture for eligibility seeding plus `generated_cases_enter_through_loaded_runtime_constructor`. | Pass |
| F-02 replay/save reconstruction of runtime authority | Production constructor: runtime rebuild from owned log with manifest id. Public command/boundary: replay restore / TUI rebuild call site. Observed effect: restored scheduler continuation equivalence and loaded actor next opportunity reconstruction. Sensitivity proof: `replay_temporal_frontier.rs` plus focused scheduler mutation coverage for restoration-sensitive paths; standing miss remains recorded, not laundered. | Pass with standing miss disposition |
| F-03 declared-process causal transactions | Production constructor: loaded runtime due process invocation through `transact_world_one_tick`. Public command/boundary: one-tick world step. Observed effect: `DeclaredWorldProcessApplied` world/agent/epistemic events are appended/applied and counted only after commit. Sensitivity proof: `world_step_coordinator.rs`, `negative_fixture_runner.rs`, and focused scheduler mutation coverage. | Pass |
| F-04 closed exhaustive per-tick actor disposition census | Production constructor: runtime-owned scheduler actor census. Public command/boundary: human/no-human one-tick differential through the shared world step. Observed effect: controlled actor is excluded from autonomous due work in the same tick and actor opportunities are dispositioned once. Sensitivity proof: `world_step_coordinator.rs`, `reservation_body_exclusive_census.rs`, `parity_adversarial.rs`. | Pass |
| F-05 full actor decision-transaction consumption | Production constructor: `ActorDecisionTransactionOutcome` consumed by scheduler commit. Public command/boundary: due actor transaction inside `transact_world_one_tick`. Observed effect: decision trace, local plan/proposal lineage, and stuck diagnostic ancestry are persisted for replay/debug. Sensitivity proof: `world_step_coordinator.rs`, `negative_fixture_runner.rs`, and focused trace/scheduler mutation campaigns. | Pass |
| F-06 TUI de-authority atomic cutover | Production constructor: `LoadedWorldRuntime` owns authoritative state/log/scheduler/projection. Public command/boundary: `TuiApp::submit_entry_with_world_advance`, command-loop wait/continue, bind, advance-until, no-human, rebuild, and perception-refresh runtime commands. Observed effect: TUI stores runtime plus presentation state and no longer mutates authoritative aggregates directly. Sensitivity proof: external negative fixtures for perception append, scheduler forwarding, runtime field mutation, and `PipelineContext::new`; standing TUI miss recorded. | Pass with standing miss disposition |
| F-07 sealed split temporal products | Production constructor: core-created `TypedActorKnownIntervalSummary::from_actor_known_delta` from verified interval delta. Public command/boundary: core result returned to TUI and rendered read-only. Observed effect: exact ticks/frontiers/stop reason/notices/no-new-information are sealed behind accessors; debug step reports remain separate. Sensitivity proof: negative fixtures for direct construction/mutation and focused interval/projection mutation campaigns; standing view-model misses recorded. | Pass with standing miss disposition |
| F-08 in-scope mutation closure | Production constructor: focused witnesses exercise production projection and replay/scheduler paths. Public command/boundary: clean-worktree focused mutation commands and full standing campaign. Observed effect: all selected focused mutants for in-scope and preserved 0049/0050 functions were caught or unviable; standing campaign completed. Sensitivity proof: mutation command ledger above. | Pass for focused scope; standing perimeter non-green |
| F-09 live conformance/risk evidence truthing | Production constructor: docs now name `LoadedWorldRuntime::from_loaded_world` and current 0051 witness suites. Public command/boundary: conformance rows name TUI command crossing and runtime CI lane. Observed effect: architecture, execution, reference checklist, and R-27/R-28/R-29 status/evidence rows point at repaired runtime and non-green mutation disposition. Sensitivity proof: `cargo test -p tracewake-core --test doc_invariant_references`, grep checks from `-011`, and no new risk ID. | Pass |

## Preserved properties reconfirmed

| Preserved property from §4.10 | Reconfirmation evidence at exact commit |
|---|---|
| Atomic scratch-state cutover | `world_step_coordinator.rs` duplicate terminal/process/actor tests and `EventLogError::DuplicateEventId` path; standing missed duplicate-event-sensitive families are recorded rather than ignored. |
| Duration completion and log-derived restoration | `world_step_coordinator.rs`, `replay_temporal_frontier.rs`, and focused scheduler/replay mutation campaigns. |
| Single-charge need accounting | `accepted_wait_tick_charge_is_not_replaced_or_double_charged_by_world_step`, `awake_world_step_emits_missing_passive_need_accounting_once`, no-human/golden fixture tests in `cargo test --workspace`. |
| Body-exclusive reservation | `reservation_body_exclusive_census.rs`, `advance_until_stops_at_possessed_duration_terminal`, and reservation conflict tests in core/TUI suites. |
| Accepted empty-tick ancestry | `empty_world_step_appends_time_advanced_and_rebuilds_frontier`, `run_replay_clean_time_marker_still_matches_expected`, and replay temporal frontier tests. |
| Replay aggregate fail-closed on temporal divergence | `run_replay_temporal_violation_fails_aggregate_and_reports_typed_first_divergence` and focused replay mutation campaigns. |
| Duplicate `EventId` rejection | event-log append focused mutation command and world-step/event-log duplicate rejection tests. |
| Positive holder-known interval construction | `holder_known_interval_projection.rs`, `salient_stop_actor_known.rs`, `applied_observations_replace_same_subject_without_erasing_sibling_subjects`, and focused projection mutation command. |

## Evidence item ledger

- Evidence item ID: `E-0051-GATES`
  - Requirement IDs: F-01..F-09, §7 gate ledger
  - Evidence status: pass
  - Fingerprint scope: command transcript
  - Evidence summary: four clean-worktree gates passed at `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`
  - Certification use: counted as certifying pass for gate execution only

- Evidence item ID: `E-0051-FOCUSED-MUTATION`
  - Requirement IDs: F-02, F-05, F-07, F-08, preserved 0049/0050 mutation families
  - Evidence status: pass for selected focused commands
  - Fingerprint scope: command transcript
  - Evidence summary: eight focused clean-worktree mutation commands completed with zero missed selected mutants
  - Certification use: counted as certifying pass only for selected functions/mutants

- Evidence item ID: `E-0051-STANDING-MUTATION`
  - Requirement IDs: F-08, §7 standing disposition
  - Evidence status: fail for standing green claim; pass for completed disposition capture
  - Fingerprint scope: command transcript
  - Evidence summary: `3275 mutants tested in 3h: 23 missed, 2549 caught, 703 unviable`; `0 timeout`
  - Certification use: counted as certifying fail for standing-perimeter green; counted as certifying pass for honest disposition capture

- Evidence item ID: `E-0051-RUNTIME`
  - Requirement IDs: F-01, F-03, F-04, F-06
  - Evidence status: pass
  - Fingerprint scope: replay artifact / command transcript
  - Evidence summary: `LoadedWorldRuntime::from_loaded_world`, loaded-content handoff, generative runtime-constructor witness, and `world_step_coordinator.rs`
  - Path under test and behavior witness: loaded fixture/content state enters runtime; public wait/continue/world-step commands commit actor/process events and rebuild projections; negative fixtures prevent caller-authored authority
  - Certification use: counted as certifying pass

- Evidence item ID: `E-0051-REPLAY-INTERVAL`
  - Requirement IDs: F-02, F-07, preserved §4.10 replay/interval properties
  - Evidence status: pass, with standing misses recorded
  - Fingerprint scope: replay artifact / command transcript
  - Evidence summary: `replay_temporal_frontier.rs`, `holder_known_interval_projection.rs`, `salient_stop_actor_known.rs`, and focused projection/replay mutation commands
  - Certification use: counted as certifying pass for named behavior; standing misses remain unresolved outside focused pass scope

- Evidence item ID: `E-0051-DOC-TRUTH`
  - Requirement IDs: F-09
  - Evidence status: pass
  - Fingerprint scope: parsed semantic content / command transcript
  - Evidence summary: `archive/tickets/0051FOUCONTHI-011.md`; doc invariant reference guard and grep checks
  - Certification use: counted as certifying pass for documentation truthing only

## Staged-abstraction declaration

- What this artifact proves now: the 0051 remediation line reaches the repaired loaded-world runtime, replaces the named evidence overclaims, passes the four clean-worktree gates, catches all selected focused mutation commands, and records the full standing mutation disposition.
- What this artifact deliberately abstracts: it does not remediate the out-of-surface `food_source_fact_supersedes` family or the remaining standing misses listed above.
- What the implementation or proof must not fake while using that abstraction: no artifact may call the standing perimeter green, treat observer-only/debug evidence as actor knowledge, or count sampled/focused mutation evidence as exhaustive standing certification.
- What future feature or doctrine tier the abstraction must not block: future cross-cutting mutation remediation owns the remaining standing misses; spec acceptance owns ledger/spec archival.
- What evidence prevents overclaiming from the current artifact: the standing missed list is printed in full and `E-0051-STANDING-MUTATION` is explicitly a fail for a standing green claim.
- Failure diagnostics: "not implemented yet" applies only to future cross-cutting mutation remediation; "intentionally abstracted" applies to out-of-surface standing misses; "implemented but broken" would be any failed gate or failed focused command; "overclaimed" would be any certification wording that calls the standing perimeter green or latest main verified.

## Residual convention-only items

- The standing mutation perimeter remains non-green with 23 missed mutants. This is not convention-only evidence; it is an explicit residual remediation surface.
- The out-of-surface `food_source_fact_supersedes` misses route to future cross-cutting mutation remediation.
- Spec ledger update, report archival, and spec archival are deferred to normal spec acceptance closeout and are not performed by this artifact.

## Scoped certification wording

Verdict: 0051 foundational conformance third-hardening remediation accepted for exact commit `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953`, with standing mutation perimeter non-green and residual misses listed above.

Forbidden wording:

- Do not state that Tracewake is fully certified.
- Do not state that latest main was independently verified.
- Do not state that later Phase 2+ / Phase 3A+ systems are certified by this pass.
- Do not state that archived specs are live authority.
- Do not state that the project is P0 certified.
- Do not state that SPINE-CERT passed.
