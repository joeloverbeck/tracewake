# 0038 SPINE-CERT Mutation Triage Register

**Status**: SCOPED REMEDIATION REQUIRED
**Ticket**: `0038SPICEREVE-010`
**Date**: 2026-06-18
**Posture**: Wave A continuity passed with no survivors. Wave B deliberately
expanded beyond the configured `.cargo/mutants.toml` perimeter and found
survivors that block any SPINE-CERT pass claim until remediated or proven
equivalent with behavioral and doctrine evidence.
**cargo-mutants**: `cargo-mutants 27.1.0`
**Baseline misses**: `.cargo/mutants-baseline-misses.txt` applies to the
configured Wave A perimeter. Wave B used `--no-config`, so all 296 Wave B misses
are new SPINE-CERT scoped-remediation survivors, not accepted baseline misses.

## Preflight

`cargo test --workspace --locked` passed before mutation runs.

`cargo mutants --version` returned `cargo-mutants 27.1.0`; no reinstall was
needed during this run.

Wave B no-silent-exclusion check:

```sh
cargo mutants --no-config --workspace -C=--locked \
  -f 'crates/tracewake-core/src/events/**' \
  -f 'crates/tracewake-core/src/replay/**' \
  -f 'crates/tracewake-core/src/projections.rs' \
  -f 'crates/tracewake-core/src/checksum.rs' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  -f 'crates/tracewake-core/src/actions/proposal.rs' \
  -f 'crates/tracewake-core/src/actions/report.rs' \
  -f 'crates/tracewake-core/src/scheduler.rs' \
  -f 'crates/tracewake-core/src/view_models.rs' \
  -f 'crates/tracewake-core/src/debug_capability.rs' \
  -f 'crates/tracewake-core/src/debug_reports.rs' \
  -f 'crates/tracewake-core/src/epistemics/knowledge_context.rs' \
  -f 'crates/tracewake-content/src/manifest.rs' \
  -f 'crates/tracewake-content/src/load.rs' \
  -f 'crates/tracewake-content/src/schema.rs' \
  -f 'crates/tracewake-content/src/serialization.rs' \
  -f 'crates/tracewake-content/src/validate.rs' \
  -f 'crates/tracewake-tui/src/app.rs' \
  -f 'crates/tracewake-tui/src/debug_panels.rs' \
  -f 'crates/tracewake-tui/src/render.rs' \
  -f 'crates/tracewake-tui/src/transcript.rs' \
  --list-files
```

The preflight listed every Wave B file from the inclusion set, plus module
files `crates/tracewake-core/src/events/mod.rs` and
`crates/tracewake-core/src/replay/mod.rs`. Because the run used `--no-config`,
the broad exclusions in `.cargo/mutants.toml` did not silently remove any Wave B
seam file.

## Commands

Wave A:

```sh
cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle
```

Wave B:

```sh
cargo mutants --no-config --workspace -C=--locked \
  -f 'crates/tracewake-core/src/events/**' \
  -f 'crates/tracewake-core/src/replay/**' \
  -f 'crates/tracewake-core/src/projections.rs' \
  -f 'crates/tracewake-core/src/checksum.rs' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  -f 'crates/tracewake-core/src/actions/proposal.rs' \
  -f 'crates/tracewake-core/src/actions/report.rs' \
  -f 'crates/tracewake-core/src/scheduler.rs' \
  -f 'crates/tracewake-core/src/view_models.rs' \
  -f 'crates/tracewake-core/src/debug_capability.rs' \
  -f 'crates/tracewake-core/src/debug_reports.rs' \
  -f 'crates/tracewake-core/src/epistemics/knowledge_context.rs' \
  -f 'crates/tracewake-content/src/manifest.rs' \
  -f 'crates/tracewake-content/src/load.rs' \
  -f 'crates/tracewake-content/src/schema.rs' \
  -f 'crates/tracewake-content/src/serialization.rs' \
  -f 'crates/tracewake-content/src/validate.rs' \
  -f 'crates/tracewake-tui/src/app.rs' \
  -f 'crates/tracewake-tui/src/debug_panels.rs' \
  -f 'crates/tracewake-tui/src/render.rs' \
  -f 'crates/tracewake-tui/src/transcript.rs' \
  --no-shuffle
```

## Results

| Wave | Exit | Mutants | Caught | Missed | Timeout | Unviable | Artifact snapshot |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| A guarded continuity | 0 | 1128 | 896 | 0 | 0 | 232 | `/tmp/tracewake-0038-mutants-wave-a/` |
| B SPINE seam expansion | 2 | 1679 | 1057 | 296 | 0 | 326 | `/tmp/tracewake-0038-mutants-wave-b/` |

Wave B exit code 2 is the cargo-mutants survivor signal. It is not a tool
execution failure, but it is a certification blocker.

## Survivor Disposition Rule

All Wave B missed mutants below are marked `SCOPED REMEDIATION REQUIRED`.
No survivor is accepted as equivalent in this register. No survivor is treated
as a SPINE-CERT pass condition. Each survivor needs a follow-up remediation test,
fixture, or production fix owned by the listed seam and diagnostic layer.

## Survivor Group Triage

| File | Count | Responsible SPINE seam | Failure-diagnostic layer | Behavior witness | Why tests missed it | Required remediation |
| --- | ---: | --- | --- | --- | --- | --- |
| `crates/tracewake-content/src/load.rs` | 8 | save/content manifest, seed event construction | fixture-load validation | Fixture scope, seeded event index, starting belief, and role notice mutations survived | Current tests do not assert these loader branches and counters at mutation granularity | Add loader fixtures that assert scope classification, causal event count increments, and role notice event ids |
| `crates/tracewake-content/src/schema.rs` | 3 | content schema to agent-state conversion | schema conversion validation | Agent-state bound comparisons survived | Boundary conversion cases are under-specified | Add schema conversion cases around the line 772 branch |
| `crates/tracewake-content/src/serialization.rs` | 17 | save serialization and replay inputs | serialization roundtrip diagnostics | Event log serialization and parser arm deletions survived | Roundtrip tests do not cover all channel, stance, routine, privacy, and vector parser variants | Add canonical save fixtures for every parser variant and negative roundtrip witnesses |
| `crates/tracewake-content/src/validate.rs` | 57 | content validation, no prose-born facts, no direct state/script, determinism | schema/content validator diagnostics | Reference, topology, marker, semantic id, player, epistemic seed, determinism, and roundtrip validators survived | Validation tests assert broad success/failure but miss branch-specific mutants | Add focused invalid fixtures for every branch and marker class; add positive fixtures where boundary changes should fail |
| `crates/tracewake-core/src/actions/proposal.rs` | 3 | action proposal provenance | action availability diagnostics | Proposal source id and TUI context survived | Proposal provenance tests do not inspect all stable ids/context payloads | Add stable-id/context assertions for each proposal source |
| `crates/tracewake-core/src/actions/report.rs` | 13 | no-direct-dispatch checked facts | action rejection diagnostics | Checked fact key parser arm deletions survived | Negative dispatch/report tests do not cover every stable key | Add parser and no-direct-dispatch assertions for every checked fact key |
| `crates/tracewake-core/src/checksum.rs` | 6 | deterministic replay checksum | replay/checksum diagnostics | Checksum string and stable key mutations survived | Checksum tests do not assert all key fragments independently | Add checksum fragment tests for agent state, locations, and routine status |
| `crates/tracewake-core/src/debug_reports.rs` | 19 | debug-only reports, projection/replay diagnostics | debug report layer | Debug-only markers, report filters, stuck views, and rendered rows survived | Debug report tests do not assert all marker/filter/output contracts | Add debug report contract tests for each report type and filter |
| `crates/tracewake-core/src/epistemics/knowledge_context.rs` | 67 | subjective epistemics/no truth leak | epistemic knowledge diagnostics | Stable ids, canonical keys, source keys, known fact fields, and forbidden truth audit survived | Epistemic tests do not assert every canonical/source key and fact field | Add exhaustive knowledge-context fixtures for each fact kind and forbidden-source audit |
| `crates/tracewake-core/src/events/apply.rs` | 44 | event application and causality | event apply diagnostics | Epistemic event application, causal ids, typed payloads, parser arms, intention transitions, item application, and preconditions survived | Apply tests do not cover all event kinds, payload variants, and precondition failures | Add event-apply fixtures for every event kind/parser arm and causality witness |
| `crates/tracewake-core/src/events/envelope.rs` | 9 | event envelope serialization | envelope/replay diagnostics | Stream/source/schedule/cause/random-draw serialization survived | Envelope tests do not assert all stable id mappings and random draw encodings | Add envelope parser/serializer roundtrips for every stream, cause, phase, and random draw |
| `crates/tracewake-core/src/projections.rs` | 5 | projections/action proposal surface | projection diagnostics | Semantic action filters and workplace provenance survived | Projection tests miss negative filter and provenance vector loss cases | Add semantic-action and provenance projection tests |
| `crates/tracewake-core/src/replay/rebuild.rs` | 7 | replay rebuild determinism | replay rebuild diagnostics | Decision-context hash rebuild and latest event selection survived | Replay tests do not isolate rebuild helper failures | Add replay rebuild fixtures for causal chain and latest-perception/latest-need selection |
| `crates/tracewake-core/src/replay/report.rs` | 1 | replay report | replay report diagnostics | Replay mismatch comparison survived | Replay report tests do not assert the mismatch branch | Add a replay report mismatch fixture |
| `crates/tracewake-core/src/scheduler.rs` | 8 | scheduler/no-human-day routine progress | scheduler diagnostics | Routine completion and instant progress predicates survived | Scheduler tests do not assert these no-human helper branches directly | Add no-human routine progress tests for sleep/work completion and instant events |
| `crates/tracewake-core/src/view_models.rs` | 24 | view-model/debug disclosure | view model diagnostics | Stable ids, debug-only flags, diagnostics, and non-diegetic marker survived | View-model tests do not assert marker/debug-only contracts exhaustively | Add view model contract tests for all debug views, stable ids, and diagnostics |
| `crates/tracewake-tui/src/render.rs` | 4 | TUI-first playability/debug presentation | TUI render diagnostics | Embodied view, notebook, and rejection rendering survived | Transcript/render tests do not compare these exact sections | Add representative transcript/render assertions for embodied view, notebook, and rejection strings |
| `crates/tracewake-tui/src/transcript.rs` | 1 | TUI transcript evidence | transcript diagnostics | Representative transcript section capture survived | Transcript tests do not assert the filter comparison | Add transcript section selection tests |

## Full Survivor Inventory

The following inventory records every Wave B survivor from
`/tmp/tracewake-0038-mutants-wave-b/missed.txt`.

```text
crates/tracewake-content/src/load.rs:120:5: replace fixture_scope_from_raw_lines -> Option<FixtureScope> with None
crates/tracewake-content/src/load.rs:123:13: delete match arm (Some("fixture_scope"), Some("phase1"), None) in fixture_scope_from_raw_lines
crates/tracewake-content/src/load.rs:124:13: delete match arm (Some("fixture_scope"), Some("phase2a_historical"), None) in fixture_scope_from_raw_lines
crates/tracewake-content/src/load.rs:127:13: delete match arm (Some("fixture_scope"), Some("phase3a_historical"), None) in fixture_scope_from_raw_lines
crates/tracewake-content/src/load.rs:199:18: replace += with *= in seed_event_log
crates/tracewake-content/src/load.rs:302:15: replace += with *= in append_starting_belief
crates/tracewake-content/src/load.rs:340:19: replace += with -= in append_role_assignment_notices
crates/tracewake-content/src/load.rs:340:19: replace += with *= in append_role_assignment_notices
crates/tracewake-content/src/schema.rs:772:46: replace < with == in FixtureSchema::to_agent_state
crates/tracewake-content/src/schema.rs:772:46: replace < with > in FixtureSchema::to_agent_state
crates/tracewake-content/src/schema.rs:772:46: replace < with <= in FixtureSchema::to_agent_state
crates/tracewake-content/src/serialization.rs:522:5: replace serialize_event_log -> Vec<u8> with vec![]
crates/tracewake-content/src/serialization.rs:564:9: delete match arm "touch_or_search" in parse_channel
crates/tracewake-content/src/serialization.rs:566:9: delete match arm "absence_marker" in parse_channel
crates/tracewake-content/src/serialization.rs:567:9: delete match arm "reading_placeholder_schema_only" in parse_channel
crates/tracewake-content/src/serialization.rs:574:9: delete match arm "believes_true" in parse_stance
crates/tracewake-content/src/serialization.rs:575:9: delete match arm "believes_false" in parse_stance
crates/tracewake-content/src/serialization.rs:578:9: delete match arm "doubts" in parse_stance
crates/tracewake-content/src/serialization.rs:579:9: delete match arm "unknown_or_unresolved" in parse_stance
crates/tracewake-content/src/serialization.rs:598:5: replace parse_optional_tick -> Result<Option<SimTick>, SerializationError> with Ok(None)
crates/tracewake-content/src/serialization.rs:642:9: delete match arm "morning_wake" in parse_routine_family
crates/tracewake-content/src/serialization.rs:646:9: delete match arm "return_home" in parse_routine_family
crates/tracewake-content/src/serialization.rs:649:9: delete match arm "leave_unsafe_place" in parse_routine_family
crates/tracewake-content/src/serialization.rs:650:9: delete match arm "continue_current_intention" in parse_routine_family
crates/tracewake-content/src/serialization.rs:651:9: delete match arm "wait" in parse_routine_family
crates/tracewake-content/src/serialization.rs:652:9: delete match arm "idle_with_reason" in parse_routine_family
crates/tracewake-content/src/serialization.rs:721:9: delete match arm "institution" in parse_privacy_scope
crates/tracewake-content/src/serialization.rs:769:5: replace split_usize -> Result<Vec<usize>, SerializationError> with Ok(vec![0])
crates/tracewake-content/src/validate.rs:214:29: replace + with * in validate_raw_lines
crates/tracewake-content/src/validate.rs:243:44: replace && with || in validate_raw_lines
crates/tracewake-content/src/validate.rs:447:5: replace reject_reserved_or_display with ()
crates/tracewake-content/src/validate.rs:577:44: replace match guard !places.contains(place_id) with false in validate_references
crates/tracewake-content/src/validate.rs:584:52: replace match guard !containers.contains(container_id) with false in validate_references
crates/tracewake-content/src/validate.rs:591:46: replace match guard !actors.contains(actor_id) with false in validate_references
crates/tracewake-content/src/validate.rs:869:5: replace validate_location_reference with ()
crates/tracewake-content/src/validate.rs:870:40: replace match guard !places.contains(place_id) with false in validate_location_reference
crates/tracewake-content/src/validate.rs:877:48: replace match guard !containers.contains(container_id) with false in validate_location_reference
crates/tracewake-content/src/validate.rs:884:42: replace match guard !actors.contains(actor_id) with true in validate_location_reference
crates/tracewake-content/src/validate.rs:884:42: replace match guard !actors.contains(actor_id) with false in validate_location_reference
crates/tracewake-content/src/validate.rs:884:42: delete ! in validate_location_reference
crates/tracewake-content/src/validate.rs:967:5: replace validate_topology with ()
crates/tracewake-content/src/validate.rs:1301:22: replace < with <= in validate_numeric_i32
crates/tracewake-content/src/validate.rs:1308:29: replace > with >= in validate_numeric_i32
crates/tracewake-content/src/validate.rs:1327:29: replace > with == in validate_numeric_i32
crates/tracewake-content/src/validate.rs:1327:29: replace > with >= in validate_numeric_i32
crates/tracewake-content/src/validate.rs:1376:14: replace > with >= in validate_need_band_u16
crates/tracewake-content/src/validate.rs:1452:50: delete ! in validate_routine_rules
crates/tracewake-content/src/validate.rs:1582:5: replace is_no_sleep_diagnostic -> bool with true
crates/tracewake-content/src/validate.rs:1587:9: delete match arm "move" | "open" | "close" | "take" | "place" | "look" | "inspect_place" | "inspect_entity" | "wait" in known_action_scope
crates/tracewake-content/src/validate.rs:1604:9: replace || with && in contains_direct_state_or_script_marker
crates/tracewake-content/src/validate.rs:1602:69: replace && with || in contains_direct_state_or_script_marker
crates/tracewake-content/src/validate.rs:1602:34: delete ! in contains_direct_state_or_script_marker
crates/tracewake-content/src/validate.rs:1602:82: replace != with == in contains_direct_state_or_script_marker
crates/tracewake-content/src/validate.rs:1603:43: replace || with && in contains_direct_state_or_script_marker
crates/tracewake-content/src/validate.rs:1610:5: replace contains_prose_born_fact_marker -> bool with true
crates/tracewake-content/src/validate.rs:1615:9: replace || with && in contains_prose_born_fact_marker
crates/tracewake-content/src/validate.rs:1614:9: replace || with && in contains_prose_born_fact_marker
crates/tracewake-content/src/validate.rs:1631:21: replace && with || in contains_prose_born_fact_marker
crates/tracewake-content/src/validate.rs:1664:17: replace && with || in supports_target_kind
crates/tracewake-content/src/validate.rs:1667:58: replace == with != in supports_target_kind
crates/tracewake-content/src/validate.rs:1698:59: replace == with != in supports_target_kind
crates/tracewake-content/src/validate.rs:1706:50: replace == with != in supports_target_kind
crates/tracewake-content/src/validate.rs:1718:5: replace target_kind -> Option<&'static str> with Some("")
crates/tracewake-content/src/validate.rs:1718:5: replace target_kind -> Option<&'static str> with Some("xyzzy")
crates/tracewake-content/src/validate.rs:1721:46: replace == with != in target_kind
crates/tracewake-content/src/validate.rs:1763:55: replace == with != in target_kind
crates/tracewake-content/src/validate.rs:1772:5: replace validate_semantic_ids with ()
crates/tracewake-content/src/validate.rs:1785:5: replace validate_no_player with ()
crates/tracewake-content/src/validate.rs:2409:5: replace stable_event_cause_id -> String with String::new()
crates/tracewake-content/src/validate.rs:2409:5: replace stable_event_cause_id -> String with "xyzzy".into()
crates/tracewake-content/src/validate.rs:2473:73: replace && with || in contains_authored_outcome_marker
crates/tracewake-content/src/validate.rs:2473:38: delete ! in contains_authored_outcome_marker
crates/tracewake-content/src/validate.rs:2473:86: replace != with == in contains_authored_outcome_marker
crates/tracewake-content/src/validate.rs:2474:47: replace || with && in contains_authored_outcome_marker
crates/tracewake-content/src/validate.rs:2582:53: replace match guard actor_id == &belief.holder_actor_id with true in validate_epistemic_seeds
crates/tracewake-content/src/validate.rs:2603:35: replace < with == in validate_epistemic_seeds
crates/tracewake-content/src/validate.rs:2603:35: replace < with > in validate_epistemic_seeds
crates/tracewake-content/src/validate.rs:2603:35: replace < with <= in validate_epistemic_seeds
crates/tracewake-content/src/validate.rs:2614:20: replace match guard belief.source_kind == InitialBeliefSourceKind::AuthoredPrehistory with true in validate_epistemic_seeds
crates/tracewake-content/src/validate.rs:2686:43: replace < with <= in validate_determinism
crates/tracewake-content/src/validate.rs:2698:46: replace < with <= in is_sorted_unique
crates/tracewake-content/src/validate.rs:2702:5: replace validate_fixture_contract with ()
crates/tracewake-content/src/validate.rs:2702:34: replace || with && in validate_fixture_contract
crates/tracewake-content/src/validate.rs:2716:5: replace validate_serialization_roundtrip with ()
crates/tracewake-content/src/validate.rs:2719:16: replace match guard roundtrip == { let mut expected = fixture.clone(); expected.canonicalize(); expected } with true in validate_serialization_roundtrip
crates/tracewake-core/src/checksum.rs:301:9: replace AgentStateChecksum::as_str -> &str with ""
crates/tracewake-core/src/checksum.rs:301:9: replace AgentStateChecksum::as_str -> &str with "xyzzy"
crates/tracewake-core/src/checksum.rs:575:5: replace location_key -> String with String::new()
crates/tracewake-core/src/checksum.rs:575:5: replace location_key -> String with "xyzzy".into()
crates/tracewake-core/src/checksum.rs:583:5: replace routine_step_status_id -> &'static str with ""
crates/tracewake-core/src/checksum.rs:583:5: replace routine_step_status_id -> &'static str with "xyzzy"
crates/tracewake-core/src/debug_reports.rs:102:9: replace ItemLocationDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:108:9: replace ActionRejectionDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:114:9: replace ProjectionRebuildDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:114:9: replace ProjectionRebuildDebugReport::debug_only -> bool with false
crates/tracewake-core/src/debug_reports.rs:120:9: replace ReplayDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:120:9: replace ReplayDebugReport::debug_only -> bool with false
crates/tracewake-core/src/debug_reports.rs:126:9: replace ControllerBindingDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:126:9: replace ControllerBindingDebugReport::debug_only -> bool with false
crates/tracewake-core/src/debug_reports.rs:132:9: replace Phase3ADebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:138:9: replace NoHumanDayDebugReport::debug_only -> bool with true
crates/tracewake-core/src/debug_reports.rs:217:38: replace == with != in item_location_report
crates/tracewake-core/src/debug_reports.rs:421:52: replace == with != in phase3a_actor_report
crates/tracewake-core/src/debug_reports.rs:503:5: replace is_item_location_event -> bool with true
crates/tracewake-core/src/debug_reports.rs:509:7: replace && with || in is_item_location_event
crates/tracewake-core/src/debug_reports.rs:512:45: replace && with || in is_item_location_event
crates/tracewake-core/src/debug_reports.rs:569:5: replace stuck_views_for_actor -> Vec<Phase3AStuckDiagnosticView> with vec![]
crates/tracewake-core/src/debug_reports.rs:572:51: replace == with != in stuck_views_for_actor
crates/tracewake-core/src/debug_reports.rs:620:5: replace render_decision_trace_row -> String with String::new()
crates/tracewake-core/src/debug_reports.rs:620:5: replace render_decision_trace_row -> String with "xyzzy".into()
crates/tracewake-core/src/projections.rs:927:47: replace == with != in phase3a_semantic_actions
crates/tracewake-core/src/projections.rs:932:31: delete ! in phase3a_semantic_actions
crates/tracewake-core/src/projections.rs:939:19: delete ! in phase3a_semantic_actions
crates/tracewake-core/src/projections.rs:992:5: replace workplace_availability_provenance_refs -> Vec<ActionAvailabilityProvenance> with vec![]
crates/tracewake-core/src/projections.rs:1333:33: replace == with != in proposal_from_semantic_action_entry
crates/tracewake-core/src/scheduler.rs:1115:9: replace no_human::append_routine_step_completed_after_duration_completion with ()
crates/tracewake-core/src/scheduler.rs:1116:13: delete match arm EventKind::SleepCompleted in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1117:13: delete match arm EventKind::WorkBlockCompleted in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1126:58: replace == with != in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1127:55: replace == with != in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1128:59: replace <= with > in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1130:39: replace == with != in no_human::append_routine_step_completed_after_duration_completion
crates/tracewake-core/src/scheduler.rs:1980:57: replace && with || in no_human::is_instant_routine_progress_event
crates/tracewake-core/src/view_models.rs:79:9: replace WhyNotFailureKind::stable_id -> &'static str with ""
crates/tracewake-core/src/view_models.rs:79:9: replace WhyNotFailureKind::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/view_models.rs:319:9: replace ActionAvailability::debug_only_diagnostics -> &[String] with Vec::leak(Vec::new())
crates/tracewake-core/src/view_models.rs:319:9: replace ActionAvailability::debug_only_diagnostics -> &[String] with Vec::leak(vec![String::new()])
crates/tracewake-core/src/view_models.rs:319:9: replace ActionAvailability::debug_only_diagnostics -> &[String] with Vec::leak(vec!["xyzzy".into()])
crates/tracewake-core/src/view_models.rs:354:9: replace ActionAvailabilityProvenanceKind::stable_id -> &'static str with ""
crates/tracewake-core/src/view_models.rs:354:9: replace ActionAvailabilityProvenanceKind::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/view_models.rs:493:9: replace DebugControllerBindingView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:493:9: replace DebugControllerBindingView::debug_only -> bool with false
crates/tracewake-core/src/view_models.rs:506:9: replace DebugEventLogView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:506:9: replace DebugEventLogView::debug_only -> bool with false
crates/tracewake-core/src/view_models.rs:520:9: replace DebugItemLocationView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:520:9: replace DebugItemLocationView::debug_only -> bool with false
crates/tracewake-core/src/view_models.rs:533:9: replace DebugActionRejectionView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:533:9: replace DebugActionRejectionView::debug_only -> bool with false
crates/tracewake-core/src/view_models.rs:546:9: replace DebugProjectionRebuildView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:559:9: replace DebugReplayReportView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:559:9: replace DebugReplayReportView::debug_only -> bool with false
crates/tracewake-core/src/view_models.rs:584:9: replace DebugEpistemicsView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:602:9: replace DebugBeliefsView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:620:9: replace DebugObservationsView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:645:9: replace DebugTruthBeliefMismatchView::debug_only -> bool with true
crates/tracewake-core/src/view_models.rs:649:9: replace DebugTruthBeliefMismatchView::non_diegetic_marker -> &'static str with ""
crates/tracewake-core/src/view_models.rs:649:9: replace DebugTruthBeliefMismatchView::non_diegetic_marker -> &'static str with "xyzzy"
crates/tracewake-core/src/actions/proposal.rs:64:9: replace ProposalSource::stable_id -> &'static str with ""
crates/tracewake-core/src/actions/proposal.rs:64:9: replace ProposalSource::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/actions/proposal.rs:74:9: replace ProposalSource::tui_context -> Option<&ProposalSourceContext> with None
crates/tracewake-core/src/actions/report.rs:123:13: delete match arm "body_exclusive" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:124:13: delete match arm "container_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:126:13: delete match arm "duration_ticks" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:127:13: delete match arm "from_place_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:128:13: delete match arm "item_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:129:13: delete match arm "need_kind" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:130:13: delete match arm "pipeline_slots_9_11" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:131:13: delete match arm "place_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:132:13: delete match arm "reason" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:133:13: delete match arm "sleep_affordance_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:134:13: delete match arm "target_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:135:13: delete match arm "ticks" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/actions/report.rs:136:13: delete match arm "to_place_id" in CheckedFactKey::from_stable_key
crates/tracewake-core/src/epistemics/knowledge_context.rs:22:9: replace ViewMode::stable_id -> &'static str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:22:9: replace ViewMode::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:40:9: replace AllowedKnowledgeSource::stable_id -> &'static str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:40:9: replace AllowedKnowledgeSource::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:63:9: replace ForbiddenKnowledgeSource::stable_id -> &'static str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:63:9: replace ForbiddenKnowledgeSource::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:83:9: replace ScopeFilter::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:146:9: replace KnowledgeProvenanceEntry::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:146:9: replace KnowledgeProvenanceEntry::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:150:9: replace KnowledgeProvenanceEntry::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:150:9: replace KnowledgeProvenanceEntry::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:188:9: replace ActorKnownCurrentPlaceFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:188:9: replace ActorKnownCurrentPlaceFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:192:9: replace ActorKnownCurrentPlaceFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:192:9: replace ActorKnownCurrentPlaceFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:233:9: replace ActorKnownCarriedItemFact::portable -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:237:9: replace ActorKnownCarriedItemFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:237:9: replace ActorKnownCarriedItemFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:241:9: replace ActorKnownCarriedItemFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:241:9: replace ActorKnownCarriedItemFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:290:9: replace ActorKnownWorkplaceFact::believed_access_open -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:294:9: replace ActorKnownWorkplaceFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:294:9: replace ActorKnownWorkplaceFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:306:9: replace ActorKnownWorkplaceFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:306:9: replace ActorKnownWorkplaceFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:354:9: replace ActorKnownFoodSourceFact::believed_servings -> Option<u32> with Some(0)
crates/tracewake-core/src/epistemics/knowledge_context.rs:358:9: replace ActorKnownFoodSourceFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:358:9: replace ActorKnownFoodSourceFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:362:9: replace ActorKnownFoodSourceFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:362:9: replace ActorKnownFoodSourceFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:402:9: replace ActorKnownSleepAffordanceFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:402:9: replace ActorKnownSleepAffordanceFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:406:9: replace ActorKnownSleepAffordanceFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:406:9: replace ActorKnownSleepAffordanceFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:467:9: replace ActorKnownDoorFact::is_open -> bool with false
crates/tracewake-core/src/epistemics/knowledge_context.rs:471:9: replace ActorKnownDoorFact::is_locked -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:475:9: replace ActorKnownDoorFact::blocks_movement_when_closed -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:475:9: replace ActorKnownDoorFact::blocks_movement_when_closed -> bool with false
crates/tracewake-core/src/epistemics/knowledge_context.rs:479:9: replace ActorKnownDoorFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:479:9: replace ActorKnownDoorFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:483:9: replace ActorKnownDoorFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:483:9: replace ActorKnownDoorFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:524:9: replace ActorKnownContainerFact::is_open -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:524:9: replace ActorKnownContainerFact::is_open -> bool with false
crates/tracewake-core/src/epistemics/knowledge_context.rs:528:9: replace ActorKnownContainerFact::is_locked -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:528:9: replace ActorKnownContainerFact::is_locked -> bool with false
crates/tracewake-core/src/epistemics/knowledge_context.rs:532:9: replace ActorKnownContainerFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:532:9: replace ActorKnownContainerFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:536:9: replace ActorKnownContainerFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:536:9: replace ActorKnownContainerFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:578:9: replace ActorKnownItemFact::portable -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:578:9: replace ActorKnownItemFact::portable -> bool with false
crates/tracewake-core/src/epistemics/knowledge_context.rs:582:9: replace ActorKnownItemFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:582:9: replace ActorKnownItemFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:586:9: replace ActorKnownItemFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:586:9: replace ActorKnownItemFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:619:9: replace ActorKnownLocalActorFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:619:9: replace ActorKnownLocalActorFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:645:9: replace ActorKnownRouteFact::source_key -> &str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:645:9: replace ActorKnownRouteFact::source_key -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:649:9: replace ActorKnownRouteFact::canonical_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:649:9: replace ActorKnownRouteFact::canonical_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/knowledge_context.rs:673:9: replace ForbiddenTruthAudit::passed -> bool with true
crates/tracewake-core/src/epistemics/knowledge_context.rs:690:9: replace KnowledgeContextStatus::stable_id -> &'static str with ""
crates/tracewake-core/src/epistemics/knowledge_context.rs:690:9: replace KnowledgeContextStatus::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/epistemics/knowledge_context.rs:1305:5: replace location_key -> String with String::new()
crates/tracewake-core/src/epistemics/knowledge_context.rs:1305:5: replace location_key -> String with "xyzzy".into()
crates/tracewake-core/src/events/apply.rs:270:9: delete match arm EventKind::StartingBeliefRecorded in apply_epistemic_event
crates/tracewake-core/src/events/apply.rs:528:17: replace match guard AGENT_WORLD_NOOP_ALLOWLIST.contains(&kind) with true in apply_agent_event_with_capability
crates/tracewake-core/src/events/apply.rs:534:5: replace caused_event_ids -> Vec<EventId> with vec![]
crates/tracewake-core/src/events/apply.rs:538:13: delete match arm EventCause::Event(event_id) in caused_event_ids
crates/tracewake-core/src/events/apply.rs:590:5: replace validate_typed_diagnostic_payload -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/apply.rs:618:13: delete match arm "true" in validate_typed_diagnostic_payload
crates/tracewake-core/src/events/apply.rs:792:9: delete match arm "believes_false" in parse_stance
crates/tracewake-core/src/events/apply.rs:793:9: delete match arm "expects_true" in parse_stance
crates/tracewake-core/src/events/apply.rs:795:9: delete match arm "doubts" in parse_stance
crates/tracewake-core/src/events/apply.rs:796:9: delete match arm "unknown_or_unresolved" in parse_stance
crates/tracewake-core/src/events/apply.rs:811:9: delete match arm "reading_placeholder_schema_only" in parse_channel
crates/tracewake-core/src/events/apply.rs:992:9: delete match arm "safety" in parse_need_kind
crates/tracewake-core/src/events/apply.rs:1036:9: delete match arm "routine_effect" in parse_need_change_cause
crates/tracewake-core/src/events/apply.rs:1051:9: delete match arm "completed" in parse_intention_status
crates/tracewake-core/src/events/apply.rs:1052:9: delete match arm "failed" in parse_intention_status
crates/tracewake-core/src/events/apply.rs:1053:9: delete match arm "abandoned" in parse_intention_status
crates/tracewake-core/src/events/apply.rs:1054:9: delete match arm "interrupted" in parse_intention_status
crates/tracewake-core/src/events/apply.rs:1065:9: delete match arm "need_pressure" in parse_intention_source
crates/tracewake-core/src/events/apply.rs:1066:9: delete match arm "routine_duty" in parse_intention_source
crates/tracewake-core/src/events/apply.rs:1068:9: delete match arm "safety_interruption" in parse_intention_source
crates/tracewake-core/src/events/apply.rs:1069:9: delete match arm "fixture_routine_assignment" in parse_intention_source
crates/tracewake-core/src/events/apply.rs:1070:9: delete match arm "fallback" in parse_intention_source
crates/tracewake-core/src/events/apply.rs:1101:5: replace parse_optional_routine_template_id -> Result<Option<crate::ids::RoutineTemplateId>, ApplyError> with Ok(None)
crates/tracewake-core/src/events/apply.rs:1116:5: replace parse_u8 -> Result<u8, ApplyError> with Ok(0)
crates/tracewake-core/src/events/apply.rs:1116:5: replace parse_u8 -> Result<u8, ApplyError> with Ok(1)
crates/tracewake-core/src/events/apply.rs:1124:5: replace parse_u64_agent -> Result<u64, ApplyError> with Ok(0)
crates/tracewake-core/src/events/apply.rs:1124:5: replace parse_u64_agent -> Result<u64, ApplyError> with Ok(1)
crates/tracewake-core/src/events/apply.rs:1251:9: delete match arm EventKind::IntentionContinued | EventKind::IntentionResumed in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1252:9: delete match arm EventKind::IntentionSuspended in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1253:9: delete match arm EventKind::IntentionCompleted in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1254:9: delete match arm EventKind::IntentionFailed in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1255:9: delete match arm EventKind::IntentionAbandoned in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1256:9: delete match arm EventKind::IntentionInterrupted in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1282:9: delete match arm EventKind::IntentionResumed in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1298:9: delete match arm EventKind::IntentionCompleted in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1306:9: delete match arm EventKind::IntentionFailed in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1314:9: delete match arm EventKind::IntentionAbandoned in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1322:9: delete match arm EventKind::IntentionInterrupted in apply_intention_transition
crates/tracewake-core/src/events/apply.rs:1390:5: replace expect_bool -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/apply.rs:1501:5: replace apply_item_taken_from_place -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/apply.rs:1563:5: replace apply_item_placed_in_place -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/apply.rs:1568:8: delete ! in apply_item_placed_in_place
crates/tracewake-core/src/events/apply.rs:1651:5: replace require_actor -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/apply.rs:1663:5: replace require_item_location -> Result<(), ApplyError> with Ok(())
crates/tracewake-core/src/events/envelope.rs:1015:9: delete match arm "controller" in stream_from_id
crates/tracewake-core/src/events/envelope.rs:1016:9: delete match arm "replay_debug" in stream_from_id
crates/tracewake-core/src/events/envelope.rs:1074:9: delete match arm "deferred_process" in schedule_phase_from_id
crates/tracewake-core/src/events/envelope.rs:1075:9: delete match arm "replay" in schedule_phase_from_id
crates/tracewake-core/src/events/envelope.rs:1094:9: delete match arm "controller" in deserialize_source_id
crates/tracewake-core/src/events/envelope.rs:1116:9: delete match arm "validation_report" in deserialize_cause
crates/tracewake-core/src/events/envelope.rs:1125:5: replace serialize_random_draw -> String with String::new()
crates/tracewake-core/src/events/envelope.rs:1125:5: replace serialize_random_draw -> String with "xyzzy".into()
crates/tracewake-core/src/events/envelope.rs:1137:20: replace != with == in deserialize_random_draw
crates/tracewake-core/src/replay/rebuild.rs:212:5: replace rebuild_decision_context_hashes -> Vec<Phase3AReplayFailure> with vec![]
crates/tracewake-core/src/replay/rebuild.rs:229:5: replace rebuild_decision_context_hash -> Result<(), Box<Phase3AReplayFailure>> with Ok(())
crates/tracewake-core/src/replay/rebuild.rs:241:17: delete match arm crate::events::EventCause::Event(event_id) in rebuild_decision_context_hash
crates/tracewake-core/src/replay/rebuild.rs:405:17: replace && with || in latest_current_place_perception_event_id
crates/tracewake-core/src/replay/rebuild.rs:404:17: replace && with || in latest_current_place_perception_event_id
crates/tracewake-core/src/replay/rebuild.rs:403:17: replace && with || in latest_current_place_perception_event_id
crates/tracewake-core/src/replay/rebuild.rs:417:21: replace || with && in latest_need_event_id
crates/tracewake-core/src/replay/report.rs:78:38: replace != with == in run_replay
crates/tracewake-tui/src/render.rs:32:12: delete ! in render_embodied_view
crates/tracewake-tui/src/render.rs:246:36: replace && with || in render_notebook
crates/tracewake-tui/src/render.rs:276:5: replace render_rejection -> String with String::new()
crates/tracewake-tui/src/render.rs:276:5: replace render_rejection -> String with "xyzzy".into()
crates/tracewake-tui/src/transcript.rs:42:50: replace == with != in capture_representative_transcript_sections
```
