# 0014 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-09

Target implementation commit under review:
`25e7b93e53256ec1f39c0964d945ffbe2e58ee9a`

Scope: `specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A
ordinary-life hardening work. It is not latest-main verification, not
full-project certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Workspace evidence at the target implementation commit:

- Cargo workspace crates: `tracewake-core`, `tracewake-content`, `tracewake-tui`.
- Authority layering: `docs/README.md` and the Phase 3A spec named above.
- Prior ticket commits in this series include the completed tickets
  `0014PHA3AORDLIF-004` through `0014PHA3AORDLIF-009`, archived under
  `archive/tickets/`.
- Capstone report commit adds this report and a lint-only allowance on
  `PhysicalState::from_seed_parts`; no runtime production behavior is changed
  by the capstone.

## Verification Commands

Observed results in this capstone run:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed after adding a narrow lint-only allowance for the seed constructor arity introduced by the sleep-affordance state collection |
| `cargo test --workspace` | Passed |
| `cargo test -p tracewake-core --test no_human_capstone --test anti_regression_guards --test event_schema_replay_gates` | Passed |

## Source Guards

The guard suite is `crates/tracewake-core/tests/anti_regression_guards.rs`.
`cargo test -p tracewake-core --test anti_regression_guards` passed with 40
tests.

Context-builder and proof-surface guards exercised:

| Guard | Lock |
|---|---|
| `guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth` | No-human cognition cannot use raw workplace assignment, raw food tables, raw place traversal, or current-place sleep defaults from scheduler proposal construction |
| `guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition` | Scheduler cannot mutate sealed actor proposals after transaction output |
| `guard_014_transaction_has_no_silent_method_fallback_scan` | Transaction fallback must produce a new coherent trace or typed stuck result |
| `guard_014_decision_hidden_truth_audit_uses_typed_input_refs` | Selected goals and method evidence use typed actor-known input refs, not display strings |
| `guard_014_sleep_validation_requires_modeled_affordance` | Sleep validation requires modeled sleep-affordance state and typed `NoSleepAffordance` rejection |
| `guard_014_embodied_projection_workplaces_are_context_backed` | Embodied workplace actions come from sealed context facts, not raw `state.workplaces` assignment truth |
| `guard_014_no_human_metrics_do_not_scan_display_text` | No-human metrics classify planner failures from typed diagnostic fields, not English payload scans |

## Adversarial Fixtures And Event Evidence

Each listed fixture is registered in `crates/tracewake-content/src/fixtures/mod.rs`
and covered by `cargo test -p tracewake-content` plus the named core/TUI gates.

| Fixture | Evidence excerpt | Test surface |
|---|---|---|
| `no_human_current_place_without_sleep_affordance_does_not_sleep_001` | High fatigue at a place without actor-known sleep affordance yields no selected sleep proposal from no-human cognition | `guard_014_sleep_validation_requires_modeled_affordance`; no-human surface tests |
| `sleep_rejects_current_place_without_sleep_affordance_001` | Forged/missing `sleep_affordance_id=bed_missing` rejects with `ReasonCode::NoSleepAffordance`, appends `ActionRejected`, and appends no `SleepStarted` | `sleep_without_authored_affordance_rejects_with_typed_reason` |
| `no_human_unseen_workplace_assignment_does_not_plan_work_001` | Raw workplace assignment exists without actor-known provenance; no-human transaction does not select hidden work target | `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`; no-human surface guards |
| `embodied_view_omits_raw_assignment_without_context_001` | Raw assigned workplace exists, but baseline embodied context has no workplace fact; embodied semantic actions omit `work_block` / `workplace_tomas` | `embodied_view_omits_raw_workplace_assignment_without_context` |
| `no_human_metrics_require_typed_responsible_layer_001` | Untyped planner-looking text is ignored; typed `responsible_layer=local_planning` / `blocker_code=local_plan_failed` counts exactly one planner failure | `no_human_metrics_rebuild_from_typed_diagnostic_fields`; `no_human_day_metrics_are_independent_event_log_counts` |
| `method_fallback_requires_new_trace_or_stuck_001` | Failed method selection emits coherent typed stuck/proof surface instead of silent fallback | `guard_014_transaction_has_no_silent_method_fallback_scan` |
| `planner_trace_001` | Selected goals, methods, and planner blockers carry typed provenance and responsible-layer diagnostics | `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit` |

Representative event ids and payload shapes are asserted in tests rather than
recorded as golden text files:

- Sleep rejection path: `ActionRejected` event, no `SleepStarted`, reason
  `NoSleepAffordance`.
- No-human capstone path: `NoHumanDayStarted`, `DecisionTraceRecorded`,
  `StuckDiagnosticRecorded`, ordinary action events, `ReplayProjectionRebuilt`,
  `NoHumanDayCompleted`.
- Typed metrics replay path: canonical log with `DecisionTraceRecorded` text
  decoy plus typed `StuckDiagnosticRecorded` rebuilds byte-identical metrics.

## Actor-Known Context Evidence

Context packet evidence is covered by:

- `epistemics::knowledge_context::tests::embodied_context_seals_id_hash_provenance_frontier_and_audit`
- `epistemics::knowledge_context::tests::embodied_context_can_seal_actor_known_workplace_facts`
- `checksum::tests::holder_known_context_hash_is_order_independent_and_input_sensitive`
- `no_human_capstone_proves_typed_ancestry_and_replay`

Representative context id/hash form:

- Context ids: `hkc.<actor_id>.<tick>.<frontier>`, for example
  `hkc.actor_tomas.0.1`.
- Context hashes: stable `hkc1-...` values derived from viewer, bound actor,
  allowed/forbidden sources, scope filters, provenance entries, audit status,
  and sealed actor-known workplace facts.
- Provenance edges: `actor_known_input_ref` entries in selected goal/method
  proof surfaces, `role_assignment_notice` for workplace knowledge,
  `modeled_observation:*` for observed ordinary-life facts, and
  `actor_known_context` for decision input source.

## Validation And No-Leak Rejections

Validation/rejection evidence is typed and actor-safe:

| Surface | Evidence |
|---|---|
| Forged/stale source contexts | `forged_or_stale_source_context_rejected_by_reason_code`; typed reason codes, no hidden-target suggestion |
| Hidden food | `embodied_affordances_exclude_hidden_food_in_closed_container`; hidden food id omitted from embodied actions and summaries |
| Hidden workplace | `workplace_requires_assignment_or_observation_provenance`; local planner fails with actor-known workplace absence |
| Sleep without modeled affordance | `sleep_without_authored_affordance_rejects_with_typed_reason`; `NoSleepAffordance` |
| Scheduler proposal mutation | `guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition`; no post-transaction actor-visible rewrite |

## Typed Metrics And Replay

Metrics evidence:

- `no_human_capstone_proves_typed_ancestry_and_replay` passed.
- `no_human_metrics_rebuild_from_typed_diagnostic_fields` passed.
- `phase3a_no_human_metrics_are_byte_identical_after_log_replay` passed.

Typed metric rule:

- `planner_failures` counts `DecisionTraceRecorded` /
  `StuckDiagnosticRecorded` only when typed fields indicate
  `responsible_layer=local_planning` or a planner-specific `blocker_code`
  (`planner_budget_exhausted`, `empty_local_plan`, `local_plan_failed`).
- English payload fragments such as `planner` or `failed` are not used for
  classification.

## Embodied And Debug TUI Evidence

TUI evidence:

- `embodied_view_omits_raw_workplace_assignment_without_context` passed:
  embodied view shows movement to `workshop_tomas` but no
  `Work at workplace_tomas` action when the sealed context lacks a workplace
  fact.
- `debug_item_does_not_leak_to_following_view_or_change_checksum` passed:
  debug item location renders non-diegetically and the following embodied view
  omits debug-only truth.
- `debug_panel_does_not_change_embodied_affordances` passed.
- `tui_transcript_marks_debug_sections_non_diegetic` passed.

Manual walkthrough command for the same boundary:

```sh
cargo run -p tracewake-tui -- embodied_view_omits_raw_assignment_without_context_001 actor_tomas
```

Expected embodied result: movement affordance is visible; raw assigned workplace
does not appear as an embodied work action. Debug panels remain non-diegetic and
do not feed the embodied action list.

## Responsible-Layer Matrix

| Layer | Evidence |
|---|---|
| `candidate_generation` | Need/goal candidates derive from actor-known context and typed input refs |
| `intention_lifecycle` | Durable intention transitions are evented and replayed |
| `method_selection` | Method fallback either emits a new trace or typed stuck diagnostic |
| `local_planning` | Route/work/food/sleep failures are typed blockers with actor-known inputs |
| `proposal_construction` | Sealed proposal prevents scheduler mutation after cognition |
| `scheduler` | Scheduler orders/calls transactions and records diagnostics, but does not emit primitive actions from raw routines/needs |
| `action_validation` | Shared pipeline validates human and scheduler proposals; forged/stale/raw targets reject with typed reasons |
| `projection` | Embodied projection reads sealed context facts for workplace affordances |
| `view_model` | TUI semantic actions are stable typed IDs and omit debug/raw hidden truth |
| `test_oracle` | Anti-regression guards and adversarial fixtures assert structural locks |
| `debug_quarantine` | Debug capability and panels are non-diegetic and checksum/read-only checked |

## Phase 3A Conformance Index

| Cognitive input | Allowed source | Forbidden source | Lock |
|---|---|---|---|
| Need pressure | Agent state rebuilt from need delta events and fixture initial need state | Scheduler threshold direct primitive dispatch | Typed need-pressure candidate generation plus `guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass` |
| Workplace knowledge | Modeled observation, memory, assignment notice, or sealed actor-known workplace fact | Raw `state.workplaces` assignment truth | `NoHumanActorKnownSurfaceBuilder` notice guard, `embodied_view_omits_raw_assignment_without_context_001`, `guard_014_embodied_projection_workplaces_are_context_backed` |
| Sleep/rest knowledge | Modeled sleep/rest affordance, actor-known sleep affordance id, home/sleep-place knowledge | Current-place default | Sleep-affordance validator, `sleep_rejects_current_place_without_sleep_affordance_001`, `guard_014_sleep_validation_requires_modeled_affordance` |
| Food knowledge | Visible/search/memory/testimony-backed actor-known food source | Raw food table or hidden pantry | Actor-known planner guards, hidden-food tests, `no_hidden_truth_planning_001` |
| Route knowledge | Observed adjacency, memory, or map artifact represented in actor-known edges | Raw hidden place adjacency traversal | Hidden route planner tests and no-human context source guard |
| Wait reason | Transaction/local plan/stuck diagnostic reason from sealed proposal | Scheduler window id injected into actor-visible proposal parameters | Sealed proposal guard and scheduler wait-reason tests |

Each row maps to one allowed source path, one forbidden source class, and one
structural lock.

## Non-Certification Statement

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0014`.
It is not a claim that the full `ORD-LIFE-CERT` gate has passed, not a claim
about latest main outside the target implementation commit, not a full-project
certification, not Phase 4 entry approval, and not `FIRST-PROOF-CERT`.
