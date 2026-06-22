# 0047 TUI Authoritative World Advance Acceptance Artifact

## Exact commit under test

- Commit: `4228e1e2e5efd759e7e7bddb939a599e344742e9`
- Branch or PR: `main`
- Scope: Spec 0047 feature evidence only. This is not a latest-main certification, not Phase-4 entry, and not a second-proof certification.

## Gates run

- `cargo fmt --all --check` - pass; rustfmt reported no diffs.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass; workspace clippy completed under warnings-denied.
- `cargo build --workspace --all-targets --locked` - pass; locked all-targets workspace build completed.
- `cargo test --workspace` - pass; workspace tests completed, including `world_step_coordinator`, `embodied_flow`, `parity_adversarial`, and `playable_capability_parity`.
- Focused parity gate: `cargo test -p tracewake-tui --test playable_capability_parity` - pass; 9 tests passed.

## Changed files

Implementation and evidence files in scope for spec 0047:

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/tests/world_step_coordinator.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/tests/command_loop_session.rs`
- `crates/tracewake-tui/tests/embodied_flow.rs`
- `crates/tracewake-tui/tests/parity/census_families.rs`
- `crates/tracewake-tui/tests/parity/mod.rs`
- `crates/tracewake-tui/tests/parity/runner.rs`
- `crates/tracewake-tui/tests/parity/scenario.rs`
- `crates/tracewake-tui/tests/parity_adversarial.rs`
- `crates/tracewake-tui/tests/playable_capability_parity.rs`

Unrelated local worktree edits under `.claude/skills/spec-to-tickets/` are excluded.

## Parity evidence block

- Target implementation commit: `4228e1e2e5efd759e7e7bddb939a599e344742e9`.
- Fixture/content fingerprints: checked through `cargo test --workspace`; fixture validation, serialization, and golden fixture tests passed. Key fixtures: `strongbox_001`, `sleep_eat_work_001`, `ordinary_workday_001`, `hidden_food_unknown_route_001`.
- Capability entries in scope: 27 total registry entries: 21 preserved spec-0046 base entries plus six `spec0047_tui_authoritative_world_advance` entries:
  - `spec0047.time.human_wait_world_step`
  - `spec0047.time.human_sleep_terminal`
  - `spec0047.time.human_work_terminal`
  - `spec0047.time.open_duration_wait_conflict`
  - `spec0047.time.actor_known_interval_summary`
  - `spec0047.time.advance_until_stop_reason`
- Generated coverage report: `playable_capability_registry_includes_spec0047_time_control_pack` re-enumerates the 21 base rows and six spec-0047 rows from the registry.
- Typed causal witnesses: `wait_command_advances_authoritative_world_one_tick`, `human_sleep_completion_real_pipeline_witness`, `human_work_completion_real_pipeline_witness`, `continue_sleep_stops_at_duration_terminal_without_actor_waited`, and `differential_human_wait_and_no_human_wait_match_authoritative_outcome`.
- Actor-known witnesses: `renderer_prints_actor_known_interval_summary`, `parity_adversarial_hidden_other_actor_interval_source_does_not_render`, and the spec-0047 parity entries' actor-knowledge witnesses.
- Rendered golden paths/digests: existing spec-0046 goldens remain checked; spec-0047 entries are real-pipeline rendered witnesses without new checked-in golden files.
- Anti-leak and debug-quarantine evidence: `parity_adversarial_hidden_other_actor_interval_source_does_not_render`, `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`, and `renderer_keeps_debug_tokens_out_of_embodied_view`.
- Replay/no-human disposition: `human_sleep_completion_real_pipeline_witness`, `human_work_completion_real_pipeline_witness`, `differential_human_wait_and_no_human_wait_match_authoritative_outcome`, `no_human_day_real_run_replays_metrics_and_trace_projection`, and `replay_debug_report` assertions in TUI witnesses.
- Compiler/source-conformance evidence: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, source guards in `anti_regression_guards.rs`, TUI source guards in `app.rs`, and `tui_seam_conformance`.
- Exact commands and verdicts: all gates listed above passed.

## Per-requirement acceptance evidence

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `0047-AC-001 one tick definition` | `core/scheduler`, `tui/app` | `E-001`, `E-002`, `E-008` | pass |
| `0047-AC-002 prior duration terminal after rebuild` | `core/scheduler`, `core/replay` | `E-002`, `E-003`, `E-006` | pass |
| `0047-AC-003 one charge classification` | `core/need_accounting`, `core/scheduler` | `E-004`, `E-008` | pass |
| `0047-AC-004 sleeping/working continuation emits no ActorWaited` | `tui/run`, `core/actions` | `E-002`, `E-003`, `E-005` | pass |
| `0047-AC-005 unpossessed actors advance same timeline` | `core/scheduler`, `core/no_human` | `E-008` | pass |
| `0047-AC-006 replay reconstructs frontier for empty ticks` | `core/replay`, `core/scheduler` | `E-001`, `E-006` | pass |
| `0047-AC-007 interval summary source-bound and actor-known` | `core/projections`, `tui/render` | `E-007`, `E-009` | pass |
| `0047-AC-008 hidden event leaves summary unchanged` | `core/projections`, `tui/render` | `E-009` | pass |
| `0047-AC-009 debug-only details unavailable to embodied surface` | `tui/render`, `tui/debug` | `E-009`, `E-010` | pass |
| `0047-AC-010 human/no-human/replay/parity real-pipeline` | `core`, `tui`, `ci` | `E-002`, `E-003`, `E-008`, `E-011` | pass |
| `0047-AC-011 every changed capability/view-model field has parity disposition` | `tui/parity`, `ci` | `E-011`, `E-012` | pass |

## Evidence item ledger

### E-001 - Coordinator tick and empty tick replay

- Requirement IDs: `0047-AC-001`, `0047-AC-006`
- Evidence status: pass
- Fingerprint scope: replay artifact
- Evidence summary: `empty_world_step_appends_time_advanced_and_rebuilds_frontier` proves an empty world step appends `TimeAdvanced`, advances the frontier, and rebuilds to the same physical/agent state.
- Path under test and behavior witness: `DeterministicScheduler::advance_world_one_tick`; world-step marker; core scheduler/replay; no negative beyond frontier mismatch tests in the same file.
- Replay/provenance ancestry: `TimeAdvanced` carries `ordering_ancestry=canonical_world_step_v1`.
- Sampling/exhaustiveness scope: focused boundary for empty tick semantics.
- Certification use: counted as certifying pass.

### E-002 - Human sleep real-pipeline witness

- Requirement IDs: `0047-AC-002`, `0047-AC-004`, `0047-AC-010`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `human_sleep_completion_real_pipeline_witness` drives `do sleep.here`, rejected ordinary `wait`, `continue`, `bind-debug actor_tomas`, and `debug replay` through `run_command_loop`.
- Path under test and behavior witness: TUI parser/app/core path; `SleepCompleted`; `ReservationConflict`; stop reason `possessed_duration_terminal`; replay reports `matches_expected=true` and `agent_checksum_matches=true`.
- Replay/provenance ancestry: source event `event.sleep_completed.event.sleep_started...` is rendered through actor-known interval summary.
- Sampling/exhaustiveness scope: focused sleep duration completion witness.
- Certification use: counted as certifying pass.

### E-003 - Human work real-pipeline witness

- Requirement IDs: `0047-AC-002`, `0047-AC-004`, `0047-AC-010`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `human_work_completion_real_pipeline_witness` drives movement, `work.block.workplace_tomas`, rejected ordinary `wait`, `continue`, debug bind, and replay through `run_command_loop`.
- Path under test and behavior witness: TUI parser/app/core path; `WorkBlockCompleted`; work need accounting; interval summary `work completed`; replay checksum parity.
- Replay/provenance ancestry: work completion source event appears in rendered interval summary.
- Sampling/exhaustiveness scope: focused work duration completion witness; work interruption remains out of scope because no production builder exists.
- Certification use: counted as certifying pass.

### E-004 - One charge classification

- Requirement IDs: `0047-AC-003`
- Evidence status: pass
- Fingerprint scope: replay artifact
- Evidence summary: `accepted_wait_tick_charge_is_not_replaced_or_double_charged_by_world_step`, `awake_world_step_emits_missing_passive_need_accounting_once`, and `no_human_need_ledger_has_no_duplicate_regime_charges` pass in workspace tests.
- Path under test and behavior witness: core need accounting and scheduler world-step classification.
- Replay/provenance ancestry: `NeedDeltaApplied` events carry accounting phase and source causes.
- Sampling/exhaustiveness scope: focused regime matrix for wait/passive/duration accounting.
- Certification use: counted as certifying pass.

### E-005 - Open duration wait conflict

- Requirement IDs: `0047-AC-004`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `wait_command_during_sleep_is_reservation_conflict_without_world_advance` and `continue_sleep_stops_at_duration_terminal_without_actor_waited` pass.
- Path under test and behavior witness: TUI wait alias and command loop; `ReservationConflict`; absence of `actor_waited` during continuation.
- Replay/provenance ancestry: rejected action report, no accepted wait event.
- Sampling/exhaustiveness scope: sleep open-duration conflict; work conflict covered by core reservation tests and work witness.
- Certification use: counted as certifying pass.

### E-006 - Replay/rebuild checksums and frontier

- Requirement IDs: `0047-AC-002`, `0047-AC-006`
- Evidence status: pass
- Fingerprint scope: replay artifact
- Evidence summary: TUI debug replay panels in sleep/work witnesses report `matches_expected=true` and `agent_checksum_matches=true`; core rebuild tests pass.
- Path under test and behavior witness: `run_replay`, `rebuild_projection`, TUI debug replay panel.
- Replay/provenance ancestry: accepted event log from real TUI command path.
- Sampling/exhaustiveness scope: sleep, work, empty tick, no-human metrics replay.
- Certification use: counted as certifying pass.

### E-007 - Actor-known interval summary

- Requirement IDs: `0047-AC-007`
- Evidence status: pass
- Fingerprint scope: parsed semantic content
- Evidence summary: `actor_known_interval_summary_filters_to_source_bearing_viewer_inputs`, `actor_known_interval_summary_distinguishes_no_new_information`, and `renderer_prints_actor_known_interval_summary`.
- Path under test and behavior witness: `build_actor_known_interval_summary` and `render_embodied_view`.
- Replay/provenance ancestry: source-bearing event IDs are retained only for the viewer actor.
- Sampling/exhaustiveness scope: positive notice and no-new-information cases.
- Certification use: counted as certifying pass.

### E-008 - Human/no-human differential

- Requirement IDs: `0047-AC-001`, `0047-AC-003`, `0047-AC-005`, `0047-AC-010`
- Evidence status: pass
- Fingerprint scope: replay artifact
- Evidence summary: `differential_human_wait_and_no_human_wait_match_authoritative_outcome` compares a human wait plus controller world step against `advance_no_human` over matched initial state/log.
- Path under test and behavior witness: core scheduler and no-human advance loop; matching physical state, physical checksum, need values, and unpossessed sleep completion; differing `TimeAdvanced` origin and human-only `ActorWaited` command report.
- Replay/provenance ancestry: matched `event_sleep_started_unpossessed`; human origin `controller.controller_human`; no-human origin `process.no_human_advance`.
- Sampling/exhaustiveness scope: one-tick differential with one possessed and one unpossessed actor.
- Certification use: counted as certifying pass.

### E-009 - Hidden-source anti-leak

- Requirement IDs: `0047-AC-007`, `0047-AC-008`, `0047-AC-009`
- Evidence status: pass
- Fingerprint scope: parsed semantic content
- Evidence summary: `parity_adversarial_hidden_other_actor_interval_source_does_not_render` renders a fixture-backed embodied view with another actor's interval source and proves only the no-new-information summary renders.
- Path under test and behavior witness: projection summary builder and TUI renderer; source actor and source event are absent from rendered actor surface.
- Replay/provenance ancestry: synthetic interval source is a negative render-boundary probe, not a world event insertion.
- Sampling/exhaustiveness scope: hidden-other actor source negative.
- Certification use: counted as certifying pass.

### E-010 - Debug quarantine

- Requirement IDs: `0047-AC-009`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `adversarial_gates_debug_truth_does_not_enter_actor_surfaces`, `renderer_keeps_debug_tokens_out_of_embodied_view`, and debug command gating tests pass.
- Path under test and behavior witness: TUI render/debug panels and app debug availability gate.
- Replay/provenance ancestry: debug surfaces are non-diegetic and read-only.
- Sampling/exhaustiveness scope: focused debug/embodied separation.
- Certification use: counted as certifying pass.

### E-011 - Parity registry extension

- Requirement IDs: `0047-AC-010`, `0047-AC-011`
- Evidence status: pass
- Fingerprint scope: command transcript
- Evidence summary: `cargo test -p tracewake-tui --test playable_capability_parity` passed with 27 entries: 21 base plus six spec-0047 rows.
- Path under test and behavior witness: `crates/tracewake-tui/tests/parity/*` registry and real-pipeline scenario runner.
- Replay/provenance ancestry: entries cite real TUI/core operations, not screenshots or manual inspection.
- Sampling/exhaustiveness scope: exhaustive over the current registry.
- Certification use: counted as certifying pass.

### E-012 - CI/PAR-011 lane

- Requirement IDs: `0047-AC-011`
- Evidence status: pass
- Fingerprint scope: not applicable
- Evidence summary: `.github/workflows/ci.yml` runs warnings-denied clippy, locked workspace build, `cargo test -p tracewake-tui --test playable_capability_parity --locked`, and locked workspace tests.
- Path under test and behavior witness: CI config lines for clippy/test jobs.
- Replay/provenance ancestry: not applicable.
- Sampling/exhaustiveness scope: ordinary CI lane.
- Certification use: counted as certifying pass.

## Staged-abstraction declaration

- What this artifact proves now: spec 0047's TUI authoritative world-advance behavior is implemented and evidenced for the exact commit named above.
- What this artifact abstracts: it is a feature acceptance packet, not a project-wide certification or latest-main proof.
- What must not be faked: human sleep/work witnesses must remain real parser/app/core command paths; `RunNoHumanDay` cannot satisfy the human path.
- Future feature/doctrine tier not blocked: Phase-4 and second-proof work require separate upstream specs and evidence.
- Evidence preventing overclaiming: scope limitation text, exact commit pin, named test witnesses, and archived ticket outcomes.
- Failure diagnostics:
  - not implemented yet: missing named witness or parity entry;
  - intentionally abstracted: explicitly out-of-scope work interruption builder;
  - implemented but broken: focused witness or replay checksum failure;
  - overclaimed: artifact asserts latest-main, Phase-4, second-proof, or project certification.

## Residual convention-only items

- The final spec ledger row and spec archive move were completed during spec acceptance/closeout per `docs/archival-workflow.md`.
- Release/IP review is not performed by this artifact and remains maintainer-owned if needed.

## Scoped certification wording

Spec 0047 TUI authoritative world-advance feature accepted for exact commit `4228e1e2e5efd759e7e7bddb939a599e344742e9`. This is scoped feature evidence only; it does not certify latest main, Phase-4 scope, a second proof, or the full Tracewake project.
