# 0015 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-09

Target implementation commit under review:
`4672cf20fa32b6caa5e2acb5c044b167c2208e57`

Scope: `archive/specs/0015_PHASE_3A_EVENTED_COGNITION_CHANNELS_AUDIT_ENFORCEMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A
evented cognition, audit enforcement, and anti-contamination hardening work.
It is not latest-main verification beyond the target implementation commit,
not full-project certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0015PHA3AEVECOG-001` | `5e62248` | Seed-time knowledge events |
| `0015PHA3AEVECOG-002` | `3e15b63` | Perception events |
| `0015PHA3AEVECOG-003` | `3eec912` | Sealed event-backed actor-known surface with `source_event_ids` |
| `0015PHA3AEVECOG-004` | `fcc8dad` | Source guards, context-hash rebuild, negative fixtures |
| `0015PHA3AEVECOG-005` | `cfff778` | Fail-closed hidden-truth audit gate |
| `0015PHA3AEVECOG-006` | `9b6fffb` | Context-backed embodied food/sleep/route/work surfaces |
| `0015PHA3AEVECOG-007` | `6b6572d` | Completion continuity and prorated interruption effects |
| `0015PHA3AEVECOG-008` | `0677bbc` | Content-authored ordinary-life tuning |
| `0015PHA3AEVECOG-009` | `774b4bd` | Guard glob/census durability |
| `0015PHA3AEVECOG-010` | `4672cf2` | Conformance docs and TFW clarification |

Capstone scope adds this report only. It introduces no production logic.

Golden fixture count re-derived from `crates/tracewake-content/src/fixtures/mod.rs::all`:
45 registered fixtures, including the 0015 adversarial fixtures cited below.

## Verification Commands

Observed results in this capstone run:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo build --workspace --all-targets --locked` | Passed |
| `cargo test --workspace --quiet` | Passed |
| `cargo test -p tracewake-core --test anti_regression_guards` | Passed with 44 tests |

## 1. Source-Guard Inventory

Primary guard suite: `crates/tracewake-core/tests/anti_regression_guards.rs`.

| Guard | Lock |
|---|---|
| `guarded_layer_source_census_matches_module_tree` | New files under `src/agent/**`, `src/scheduler*`, or `src/projections*` must be classified in the guard census. |
| `guard_015_ord_hard_008_cognition_channel_stays_evented_and_sealed` | No-human cognition stays event-log backed; no post-seal `extend_actor_known_facts` / `add_actor_known_fact` path remains. |
| `guard_015_hidden_truth_audit_fails_closed_in_transaction` | Hidden-truth audit failures use typed `hidden_truth_input` diagnostics and reject in the pipeline. |
| `guard_014_embodied_projection_workplaces_are_context_backed` | Embodied workplace affordances come from sealed context facts, not raw assignment truth. |
| `guard_014_sleep_validation_requires_modeled_affordance` | Sleep proposals require modeled sleep affordance ids and typed `NoSleepAffordance` rejection. |
| `guard_015_ordinary_life_tuning_comes_from_authored_state` | Passive need rates and sleep duration/recovery are read from authored state; retired kernel constants are banned. |

The source discovery helper now sorts production source paths before scanning,
so guard iteration is deterministic.

## 2. Adversarial Fixture Evidence

The fixtures below are registered in `crates/tracewake-content/src/fixtures/mod.rs`
and exercised by `cargo test --workspace`.

| Fixture | Concrete excerpt asserted by tests | Test surface |
|---|---|---|
| `no_human_observation_facts_cite_log_events_001` | Every `DecisionTraceRecord.actor_known_inputs` entry contains `source_events=...`, and every cited event id exists in the same log. | `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash` |
| `no_human_workplace_knowledge_requires_notice_event_001` | No ordinary pipeline event is produced, no `WorkBlockStarted` exists, and a `StuckDiagnosticRecorded` event has `input_source=holder_known_context`. | `no_human_workplace_knowledge_requires_notice_channel` |
| `no_human_sleep_knowledge_requires_observation_or_record_001` | No ordinary pipeline event is produced, no `SleepStarted` exists, and a `StuckDiagnosticRecorded` event has `input_source=holder_known_context`. | `no_human_sleep_knowledge_requires_observation_or_record_channel` |
| `forbidden_provenance_input_fails_closed_001` | Forbidden provenance produces a typed stuck diagnostic instead of a proposal/commit path. | `forbidden_actor_known_input_fails_closed_before_proposal`; `guard_015_hidden_truth_audit_fails_closed_in_transaction` |
| `embodied_view_omits_raw_assignment_without_context_001` | Raw assigned workplace exists, but embodied semantic actions omit the work action when the sealed context lacks a workplace fact. | `embodied_view_omits_raw_workplace_assignment_without_context` |
| `hidden_food_closed_container_001` / `hidden_food_unknown_route_001` / `hidden_route_edge_001` | Hidden food/routes exist in authoritative state but are absent from actor-known planner inputs and embodied affordances. | `hidden_truth_gates.rs` hidden-food and hidden-route tests |
| `sleep_interrupted_by_severe_need_prorates_recovery_001` | Interrupted sleep emits typed interruption events and prorated `NeedDeltaApplied` fatigue/hunger effects. | `sleep_completion_interrupts_on_severe_need_with_prorated_recovery`; fixture contract |

Representative event kinds asserted across the adversarial and capstone paths:
`RoleAssignmentNoticeRecorded`, `StartingBeliefRecorded`, `ObservationRecorded`,
`DecisionTraceRecorded`, `StuckDiagnosticRecorded`, `NoHumanDayStarted`,
`NoHumanDayCompleted`, `SleepInterrupted`, and `NeedDeltaApplied`.

## 3. Context Hash And Replay Evidence

Context-hash evidence is reproducible through these tests:

- `no_human_decision_actor_known_inputs_cite_log_events_and_recompute_hash`:
  recomputes `compute_holder_known_context_hash(record.actor_known_inputs.clone()).hash`
  for every decision trace and asserts equality with
  `record.actor_known_context_hash`.
- `no_human_capstone_proves_typed_ancestry_and_replay`: runs a no-human day,
  rebuilds projection state from the event log, and asserts:
  - no unsupported world or agent event versions;
  - no invariant violations;
  - no epistemic application errors;
  - no physical `state_diff`;
  - final physical checksum equals live physical checksum;
  - final agent checksum equals live agent checksum;
  - rebuilt intentions, routine executions, decision traces, and stuck
    diagnostics equal live state.
- `phase3a_no_human_metrics_are_byte_identical_after_log_replay`: serializes
  and deserializes the log, then asserts rebuilt no-human metrics are byte
  identical and replay checksums match.

The capstone test asserts required no-human event coverage: `NoHumanDayStarted`,
`NoHumanDayCompleted`, `NeedDeltaApplied`, `SleepStarted`, `SleepCompleted`,
`FoodConsumed`, `ActorWaited`, `ActorMoved`, `WorkBlockStarted`,
`WorkBlockCompleted`, `WorkBlockFailed`, `IntentionStarted`,
`IntentionContinued`, `RoutineStepStarted`, `RoutineStepCompleted`,
`RoutineStepFailed`, and `DecisionTraceRecorded`.

## 4. Seed-Time Knowledge Events

`phase3a_load_emits_authored_prehistory_seed_events` verifies deterministic
seed-time knowledge events for `sleep_eat_work_001`:

- a `RoleAssignmentNoticeRecorded` event exists;
- its payload includes `source_kind=authored_prehistory`;
- its payload includes `workplace_id=workplace_tomas`;
- `StartingBeliefRecorded` events exist for authored starting knowledge.

This is the INV-063 prehistory path: authored initial knowledge is evented
instead of inferred later from raw content tables.

## 5. Interrupted Sleep Replay Evidence

The interrupted sleep surface is covered by both direct sleep unit tests and
golden fixture coverage:

- `sleep_completion_interrupts_on_severe_need_with_prorated_recovery` asserts
  severe need pressure interrupts sleep and applies elapsed-duration prorated
  fatigue recovery / hunger rise.
- `sleep_interrupted_by_severe_need_prorates_recovery_001` declares expected
  `SleepInterrupted` and `NeedDeltaApplied` evidence in its fixture contract.
- `cargo test --workspace` also exercises replay equality through the broader
  no-human replay tests listed in section 3.

The relevant event chain is `SleepStarted` caused by proposal, followed by
`SleepInterrupted` caused by the start event, plus prorated `NeedDeltaApplied`
events caused by the same start event.

## 6. Embodied/Debug Raw-Vs-Context Evidence

The no-leak boundary is exercised without production changes:

- `embodied_view_omits_raw_workplace_assignment_without_context` proves raw
  assignment truth does not produce an embodied work affordance without a
  context fact.
- `embodied_affordances_exclude_hidden_food_in_closed_container` proves hidden
  food exists in authoritative state but is absent from embodied affordances.
- `hidden_route_edge_absent_from_actor_known_edges_blocks_route_plan` proves
  unseen route truth does not become a route plan.
- `debug_item_does_not_leak_to_following_view_or_change_checksum`,
  `debug_panel_does_not_change_embodied_affordances`, and
  `tui_transcript_marks_debug_sections_non_diegetic` prove debug truth remains
  non-diegetic and does not feed later embodied output.

The TUI/debug surfaces remain separate: debug comparison may inspect truth, but
embodied affordance selection consumes only holder-known context.

## 7. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0015`. It is
not a claim that the full `ORD-LIFE-CERT` gate has passed, not a claim about
latest main outside the target implementation commit, not full-project
certification, not Phase 4 entry approval, and not `FIRST-PROOF-CERT`.
