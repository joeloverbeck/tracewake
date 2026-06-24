# 0050FOUCONSEC-011: Focused + standing mutation campaigns

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0050FOUCONSEC-010

## Problem

Spec-0050 §6.3 + §8 (closure step 8): after the remediation lands, run focused `cargo-mutants` campaigns over the new branches, **preserve and rerun** the three `0049` focused commands, then run the configured standing perimeter, recording exact denominators and caught/missed/unviable sets. The narrow campaigns are not a replacement for the standing lane. **No `.cargo/mutants.toml` change** — the perimeter already covers the seams (spec §3.2 / §6.3, verified: `.cargo/mutants.toml` present and covers scheduler/replay/projection/pipeline/TUI).

## Assumption Reassessment (2026-06-24)

1. `.cargo/mutants.toml` exists and already includes the scheduler/replay/projection/pipeline/TUI seams (verified at `HEAD` `8d7c119`; spec §3.2 records the perimeter as already-covering). The three `0049MUTWIT` focused commands are recorded in `archive/tickets/0049MUTWIT-001..003.md`. This ticket runs and records; it changes no config and no source.
2. Spec-0050 §6.3 is authoritative: no perimeter expansion; the delta is behavior that kills the new branches' mutants, plus rerunning the preserved `0049` commands and the standing lane.
3. Shared boundary under audit: the mutation evidence boundary — the standing `.cargo/mutants.toml` perimeter and the focused campaigns over the functions changed by `-001`…`-009`. This is an evidence-collection ticket; it authors no production code.
4. `INV-092` (deterministic replay tested) and `INV-098` (harsh feature acceptance, regression-tested) motivate this ticket: surviving mutants signal a missing witness or an equivalent mutant; closure records the denominator/artifact, not a declaration.
5. Enforcement surface (evidence-consumer basis): the campaigns audit the new discovery/process/actor-outcome/`EventId`/replay-report/salience branches. Running mutation introduces no production behavior, no replay/leak path; it measures whether the `-001`…`-010` witnesses kill the relevant mutants. Reproduce the CI mutation environment (flags/versions) before trusting caught/missed/unviable counts.

## Architecture Check

1. Running focused campaigns during/after each change plus the standing perimeter at closeout — rather than expanding the config — matches the spec's "no second config" posture and keeps the standing lane authoritative. Preserving the `0049` commands prevents regressing their closed survivors.
2. No backwards-compatibility shims: N/A (no code change); no second mutation config is introduced.

## Verification Layers

1. `INV-092`/`INV-098` (regression evidence is measured, not declared) → mutation campaign: focused `cargo mutants` over the changed functions reports caught/missed/unviable with exact denominators; the three `0049` commands rerun with zero new misses; the standing perimeter runs to completion.
2. Single-surface note: this is an evidence ticket; its one surface (mutation posture) maps to the recorded campaign transcripts — no source/replay layer applies because it changes nothing.

## What to Change

### 1. Run focused campaigns over the new branches

Run `cargo-mutants` focused over the functions changed by `-001`…`-009` (actor/process eligibility + cadence; declared-process invocation; closed actor-step outcome handling; `EventId` uniqueness; `matches_expected` temporal conjunct + typed first-divergence; salience predicates). Record caught/missed/unviable and the selected denominator.

### 2. Preserve and rerun the `0049` focused commands

Rerun the three commands recorded in `archive/tickets/0049MUTWIT-001..003.md` exactly; confirm zero new misses (the `0049` witnesses remain non-vacuous).

### 3. Run the configured standing perimeter

Run the standing `.cargo/mutants.toml` lane to completion; record the outcome. Surviving mutants are triaged (missing witness vs equivalent) per the repo's existing mutation-evidence process; remediation of any real survivor routes to a separate ticket.

## Files to Touch

- `None — evidence-only ticket; runs the configured + focused mutation campaigns and records denominators/artifacts. No `.cargo/mutants.toml` or source change (spec §6.3).`

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (spec §6.3: none).
- Fixing a real survivor — routes to a separate remediation ticket if the campaign surfaces one.
- The acceptance artifact assembly — `0050FOUCONSEC-013` (this ticket's recorded results feed it).

## Acceptance Criteria

### Tests That Must Pass

1. Focused campaigns over the new branches record exact caught/missed/unviable sets and denominators; any miss is triaged (missing witness vs equivalent) with a recorded resolution.
2. The three `0049` focused commands rerun with zero new misses; the standing perimeter runs to completion with its outcome recorded.
3. The campaign environment matches CI (flags/versions) so counts are trustworthy.

### Invariants

1. Mutation evidence is measured and recorded with exact denominators, not declared (`INV-092`/`INV-098`).
2. No second mutation config is introduced; the standing perimeter remains authoritative (spec §6.3).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification is the recorded mutation transcripts. No tests change.`

### Commands

1. Focused: `cargo mutants -p tracewake-core` scoped to the changed functions (per `--file`/`--re` over the `-001`…`-009` surfaces).
2. Preserved `0049`: the three commands as recorded in `archive/tickets/0049MUTWIT-001..003.md`.
3. Standing: the configured `.cargo/mutants.toml` lane run to completion (matching the CI mutation environment).

## Outcome

Completed: 2026-06-24

Environment matched the checked-in mutation baseline: `cargo-mutants 27.1.0`,
`rustc 1.93.0 (254b59607 2026-01-19)`, `cargo 1.93.0
(083ac5135 2025-12-15)`. No source or mutation config changed.

Preserved `0049` focused reruns:

1. `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'latest_current_place_perception_event_id|latest_need_event_id|actor_has_open_body_exclusive_at|append_decision_trace_after_proposal|transact_world_one_tick' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 55 mutants.
   - Result: 50 caught, 5 unviable, 0 missed.
2. `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|VerifiedActorKnownIntervalNotice::source_key' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 15 mutants.
   - Result: 14 caught, 1 unviable, 0 missed.
3. `cargo mutants --no-config -f crates/tracewake-core/src/replay/temporal.rs -f crates/tracewake-core/src/replay/rebuild.rs -f crates/tracewake-core/src/actions/pipeline.rs -F 'validate_time_advanced|rebuild_projection|is_duplicate_need_tick_candidate' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 36 mutants.
   - Result: 35 caught, 1 unviable, 0 missed.

Focused `0050` branch campaigns:

1. `cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'due_loaded_actor_ids|due_process_invocations|transact_world_one_tick|build_advance_until_result|actor_known_interval_delta_is_salient|step_appended_possessed_duration_terminal' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 30 mutants.
   - Result: 26 caught, 4 unviable, 0 missed.
2. `cargo mutants --no-config -f crates/tracewake-core/src/events/log.rs -F 'append' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 8 mutants.
   - Result: 6 caught, 2 unviable, 0 missed.
3. `cargo mutants --no-config -f crates/tracewake-core/src/replay/report.rs -f crates/tracewake-core/src/replay/temporal.rs -F 'run_replay|validate_time_advanced' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 28 mutants.
   - Result: 27 caught, 1 unviable, 0 missed.
4. `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|is_salient|salience' --test-package tracewake-core --timeout 183`
   - Selected/current denominator: 16 mutants.
   - Result: 14 caught, 2 unviable, 0 missed.

Standing perimeter:

`cargo mutants --timeout 183` used `.cargo/mutants.toml` and selected 3,182
mutants. The unmutated baseline passed in 10s build + 50s test. Final result:
48 missed, 2,458 caught, 675 unviable, 1 timeout.

Standing missed set:

1. `crates/tracewake-core/src/projections.rs:260:5` — replace `food_source_fact_supersedes` with `true`.
2. `crates/tracewake-core/src/projections.rs:260:5` — replace `food_source_fact_supersedes` with `false`.
3. `crates/tracewake-core/src/projections.rs:261:9` — delete match arm `(Some(_), None)` in `food_source_fact_supersedes`.
4. `crates/tracewake-core/src/projections.rs:262:9` — delete match arm `(None, Some(_))` in `food_source_fact_supersedes`.
5. `crates/tracewake-core/src/projections.rs:263:37` — replace `<` with `==` in `food_source_fact_supersedes`.
6. `crates/tracewake-core/src/projections.rs:263:37` — replace `<` with `>` in `food_source_fact_supersedes`.
7. `crates/tracewake-core/src/projections.rs:263:37` — replace `<` with `<=` in `food_source_fact_supersedes`.
8. `crates/tracewake-core/src/projections.rs:319:38` — replace `>` with `>=` in `actor_known_doors_for_context`.
9. `crates/tracewake-core/src/projections.rs:353:38` — replace `>` with `>=` in `actor_known_containers_for_context`.
10. `crates/tracewake-core/src/scheduler.rs:511:9` — replace `DeterministicScheduler::restore_from_temporal_projection` with `None`.
11. `crates/tracewake-core/src/agent/perception.rs:45:32` — replace `+=` with `-=` in `record_current_place_perception`.
12. `crates/tracewake-core/src/agent/perception.rs:45:32` — replace `+=` with `*=` in `record_current_place_perception`.
13. `crates/tracewake-core/src/agent/perception.rs:58:13` — delete match arm `"observation_id"` in `rename_perception_event`.
14. `crates/tracewake-core/src/agent/perception.rs:59:13` — delete match arm `"source_event_id"` in `rename_perception_event`.
15. `crates/tracewake-core/src/epistemics/projection.rs:370:21` — replace `||` with `&&` in `EpistemicProjection::insert_observation`.
16. `crates/tracewake-core/src/epistemics/projection.rs:369:44` — replace `!=` with `==` in `EpistemicProjection::insert_observation`.
17. `crates/tracewake-core/src/epistemics/projection.rs:370:56` — replace `!=` with `==` in `EpistemicProjection::insert_observation`.
18. `crates/tracewake-core/src/epistemics/projection.rs:916:53` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
19. `crates/tracewake-core/src/epistemics/projection.rs:922:14` — delete `!` in `actor_known_record_is_novel_to_context`.
20. `crates/tracewake-core/src/epistemics/projection.rs:925:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
21. `crates/tracewake-core/src/epistemics/projection.rs:924:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
22. `crates/tracewake-core/src/epistemics/projection.rs:923:28` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
23. `crates/tracewake-core/src/epistemics/projection.rs:924:34` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
24. `crates/tracewake-core/src/epistemics/projection.rs:925:36` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
25. `crates/tracewake-core/src/epistemics/projection.rs:934:63` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
26. `crates/tracewake-core/src/epistemics/projection.rs:941:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
27. `crates/tracewake-core/src/epistemics/projection.rs:949:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
28. `crates/tracewake-core/src/epistemics/projection.rs:956:14` — delete `!` in `actor_known_record_is_novel_to_context`.
29. `crates/tracewake-core/src/epistemics/projection.rs:959:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
30. `crates/tracewake-core/src/epistemics/projection.rs:958:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
31. `crates/tracewake-core/src/epistemics/projection.rs:957:33` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
32. `crates/tracewake-core/src/epistemics/projection.rs:958:36` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
33. `crates/tracewake-core/src/epistemics/projection.rs:959:48` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
34. `crates/tracewake-core/src/epistemics/projection.rs:975:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
35. `crates/tracewake-core/src/epistemics/projection.rs:974:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
36. `crates/tracewake-core/src/epistemics/projection.rs:973:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
37. `crates/tracewake-core/src/epistemics/projection.rs:972:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
38. `crates/tracewake-core/src/epistemics/projection.rs:971:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
39. `crates/tracewake-core/src/epistemics/projection.rs:985:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
40. `crates/tracewake-core/src/epistemics/projection.rs:984:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
41. `crates/tracewake-core/src/epistemics/projection.rs:992:14` — delete `!` in `actor_known_record_is_novel_to_context`.
42. `crates/tracewake-core/src/epistemics/projection.rs:995:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
43. `crates/tracewake-core/src/epistemics/projection.rs:994:17` — replace `&&` with `||` in `actor_known_record_is_novel_to_context`.
44. `crates/tracewake-core/src/epistemics/projection.rs:993:28` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
45. `crates/tracewake-core/src/epistemics/projection.rs:994:34` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
46. `crates/tracewake-core/src/epistemics/projection.rs:995:36` — replace `==` with `!=` in `actor_known_record_is_novel_to_context`.
47. `crates/tracewake-core/src/epistemics/projection.rs:1027:9` — replace `ActorKnownProjectionRecord::embodied_subject_key` with `String::new()`.
48. `crates/tracewake-core/src/epistemics/projection.rs:1027:9` — replace `ActorKnownProjectionRecord::embodied_subject_key` with `"xyzzy".into()`.

Standing timeout:

1. `crates/tracewake-core/src/agent/perception.rs:41:24` — delete `!` in `record_current_place_perception`.

Triage: all standing survivors/timeouts above are treated as real missing
mutation evidence, not equivalent mutants. Remediation is out of scope for this
evidence ticket and must route to a separate survivor-remediation ticket/spec
line before any later artifact claims the standing perimeter is green.
