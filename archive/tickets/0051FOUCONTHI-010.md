# 0051FOUCONTHI-010: F-08 in-scope mutation closure (focused + standing)

**Status**: COMPLETE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — add production-path mutation witnesses for the in-scope survivor families; run the focused and full standing campaigns and publish the disposition.
**Deps**: 0051FOUCONTHI-009

## Problem

The standing campaign recorded at the `-013` acceptance (3,182 selected / 2,458 caught / 675 unviable / 48 missed / 1 timeout) is honestly non-green; `-014`/`-015`/`-016` added credible focused witnesses but no fresh standing run exists (F-08, mutation-evidence gap). In-scope-for-this-surface survivors that must close: **#10** (`restore_from_temporal_projection` → `None`, guarded by `-003`), **#15–17** (`EpistemicProjection::insert_observation` retain condition), and **#47–48** (`embodied_subject_key` → constant). This ticket adds the production-path witnesses for #15–17 and #47–48, reproduces the preserved focused commands, and runs the full standing campaign after all code/test work.

## Assumption Reassessment (2026-06-24)

1. Codebase: `insert_observation` is `pub(crate)` (`crates/tracewake-core/src/epistemics/projection.rs:354`) with the retain condition at `370` (`existing.embodied_subject_key() != record.embodied_subject_key()`); `embodied_subject_key(&self) -> String` (`projection.rs:1026`); `.cargo/mutants.toml` (standing config) and an empty `.cargo/mutants-baseline-misses.txt` exist. Survivor #10 is closed by `-003`'s continuation-equivalence; out-of-surface family #1–7 (`food_source_fact_supersedes`) is out of scope.
2. Specs/docs: spec `0051` §4.8, §9.9 (mutation-survivor ownership — current line for surface-relevant families, future cross-cutting line for others). Execution home `docs/2-execution/10_*` (testing/mutation).
3. Shared boundary under audit: the standing mutation perimeter — focused commands prove their selected functions only; the standing campaign proves the perimeter. Reproduce the exact preserved focused commands from `0049MUTWIT` and `0050FOUCONSEC-014/-015/-016` against a clean `0429a8f`-descendant tree.
4. INV-092 (deterministic replay is tested), INV-098 (feature acceptance is harsh): restated — measured mutation evidence replaces stale/overclaimed closure; a timeout is not converted into an accepted baseline to obtain green.
5. Fail-closed / actor-knowledge surface (evidence-consumer basis): the #15–17 witnesses exercise the `insert_observation` retain condition (same-source/same-subject vs different-source/different-subject) and the #47–48 witnesses exercise `embodied_subject_key` subject separation — both on the epistemic-projection / embodied-output surfaces; the witnesses introduce no leakage path and are evidence-only.

## Architecture Check

1. Adding production-path witnesses (not only direct unit witnesses) for the retain condition and subject-key is what makes the focused kills meaningful at the boundary `-008` sealed; the standing run is the only evidence that the perimeter is closed. Not editing `.cargo/mutants.toml` to restate coverage, and not padding `.cargo/mutants-baseline-misses.txt`, keeps the denominator honest.
2. No backwards-compatibility alias: N/A (test-only additions + campaign run). No survivor is classified equivalent without a defensible semantic argument.

## Verification Layers

1. INV-092 (replay/mutation tested) -> focused mutation kill: `cargo mutants -f` over `epistemics/projection.rs` kills #15–17 and #47–48; the reproduced `0049MUTWIT`/`0050FOUCONSEC-014..016` commands re-run 0-miss for their selected functions.
2. INV-098 (harsh acceptance) -> standing campaign: `cargo mutants` with the standing config completes and the full caught/missed/unviable/timeout disposition is published (in the `-012` artifact).
3. Survivor #10 -> covered by `-003` (continuation-equivalence); re-confirmed in the standing run.

## What to Change

### 1. #15–17 witnesses (`insert_observation` retain condition)

Add production-path same-source/same-subject vs different-source/different-subject witnesses plus focused kills for the retain condition.

### 2. #47–48 witnesses (`embodied_subject_key` → constant)

Add direct and production-path subject-separation witnesses plus focused kills.

### 3. Reproduce + run

Reproduce the exact preserved focused commands from `0049MUTWIT` and `0050FOUCONSEC-014/-015/-016` against a clean `0429a8f`-descendant tree; then run the full standing campaign (`cargo mutants --timeout 183` with the standing config) **after** all code/test work, publishing the selected denominator and the complete disposition (consumed by `-012`).

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify) — direct inline `#[cfg(test)]` witnesses for #15–17 and #47–48
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (modify) — production-path subject-separation + retain-condition witnesses

## Out of Scope

- Editing `.cargo/mutants.toml` to restate existing coverage, or padding `.cargo/mutants-baseline-misses.txt`.
- Out-of-surface family #1–7 (`food_source_fact_supersedes`) — routes to a future cross-cutting mutation-remediation spec (owner/reassess), not a lower tier.
- Publishing the final disposition table (rendered in `-012`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/projection.rs` kills #15–17 and #47–48 (0 miss for those functions).
2. The reproduced `0049MUTWIT`/`0050FOUCONSEC-014..016` focused commands re-run with 0 miss for their selected functions.
3. The full standing `cargo mutants --timeout 183` campaign completes; its caught/missed/unviable/timeout disposition is captured for `-012` (perimeter not called green before this run).

### Invariants

1. No survivor is classified equivalent without a defensible semantic argument.
2. A focused 0-miss run is evidence only for its selected functions/mutants and exact source tree; a timeout is not converted into an accepted baseline.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (`#[cfg(test)]`) — #15–17 / #47–48 direct witnesses.
2. `crates/tracewake-core/tests/holder_known_interval_projection.rs` — production-path witnesses.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/projection.rs`
2. `cargo mutants --timeout 183` (standing config — full campaign, run last)
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-24

Added the in-scope production and direct witnesses for the epistemic projection
mutation survivors:

1. `crates/tracewake-core/src/epistemics/projection.rs`
   - `observation_insert_replaces_only_same_source_and_embodied_subject`
     proves same-source/same-subject visible-container observations replace the
     stale fact while same-source sibling subjects are retained.
   - `embodied_subject_keys_distinguish_same_source_local_subjects` proves
     local container, door, and item subject keys remain distinct under a shared
     source event.
2. `crates/tracewake-core/tests/holder_known_interval_projection.rs`
   - `applied_observations_replace_same_subject_without_erasing_sibling_subjects`
     applies real `ObservationRecorded` envelopes through `apply_epistemic_event`
     and then consumes production `current_place_knowledge_context`, proving the
     latest strongbox fact and sibling crate fact both survive at the embodied
     boundary.

Focused and preserved mutation evidence:

1. `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -F 'insert_observation|embodied_subject_key' --test-package tracewake-core --timeout 183`
   - Found 6 mutants; baseline passed in `9s build + 56s test`; result:
     `6 mutants tested in 86s: 6 caught`.
   - This is the true focused run for #15-17 and #47-48. The ticket's bare
     `cargo mutants -f crates/tracewake-core/src/epistemics/projection.rs`
     command consumed the repository standing config instead of staying file
     scoped, so the `--no-config` run records the selected-function proof.
2. Preserved `0049MUTWIT` focused reruns:
   - `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'latest_current_place_perception_event_id|latest_need_event_id|actor_has_open_body_exclusive_at|append_decision_trace_after_proposal|transact_world_one_tick' --test-package tracewake-core --timeout 183`
     -> `56 mutants tested in 5m: 51 caught, 5 unviable`.
   - `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|VerifiedActorKnownIntervalNotice::source_key' --test-package tracewake-core --timeout 183`
     -> `15 mutants tested in 2m: 14 caught, 1 unviable`.
   - `cargo mutants --no-config -f crates/tracewake-core/src/replay/temporal.rs -f crates/tracewake-core/src/replay/rebuild.rs -f crates/tracewake-core/src/actions/pipeline.rs -F 'validate_time_advanced|rebuild_projection|is_duplicate_need_tick_candidate' --test-package tracewake-core --timeout 183`
     -> `36 mutants tested in 4m: 35 caught, 1 unviable`.
3. Preserved `0050FOUCONSEC-014..016` focused reruns:
   - `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'due_loaded_actor_ids|due_process_invocations|transact_world_one_tick|build_advance_until_result|actor_known_interval_delta_is_salient|step_appended_possessed_duration_terminal' --test-package tracewake-core --timeout 183`
     -> `31 mutants tested in 5m: 27 caught, 4 unviable`.
   - `cargo mutants --no-config -f crates/tracewake-core/src/events/log.rs -F 'append' --test-package tracewake-core --timeout 183`
     -> `8 mutants tested in 79s: 6 caught, 2 unviable`.
   - `cargo mutants --no-config -f crates/tracewake-core/src/replay/report.rs -f crates/tracewake-core/src/replay/temporal.rs -F 'run_replay|validate_time_advanced' --test-package tracewake-core --timeout 183`
     -> `28 mutants tested in 3m: 27 caught, 1 unviable`.
   - `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|is_salient|salience' --test-package tracewake-core --timeout 183`
     -> `16 mutants tested in 2m: 14 caught, 2 unviable`.

Standing mutation disposition:

1. `cargo mutants -f crates/tracewake-core/src/epistemics/projection.rs`
   - Consumed `.cargo/mutants.toml` standing config and selected the full
     perimeter: `3275 mutants tested in 3h: 23 missed, 2549 caught, 703 unviable`.
2. `cargo mutants --timeout 183`
   - Final standing campaign, run after all code/test work: `3275 mutants tested
     in 3h: 23 missed, 2549 caught, 703 unviable`.
   - Artifact counts: `2549 caught`, `23 missed`, `703 unviable`, `0 timeout`.

Standing misses captured for `-012` disposition; this ticket does not call the
standing perimeter green:

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

Post-mutation verification:

1. `cargo fmt --all --check` -> passed.
2. `cargo clippy --workspace --all-targets -- -D warnings` -> passed.
3. `cargo build --workspace --all-targets --locked` -> passed.
4. `cargo test --workspace` -> passed.
5. `git diff --check` -> passed.
