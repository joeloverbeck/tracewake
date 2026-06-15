# Architecture Index and Conformance

## Status

Authoritative architecture contract. This directory is a complete replacement set. Architecture files not listed in this document are retired and must not remain as live doctrine.

The foundation layer outranks this layer. Architecture may make foundation doctrine more operational; it may not soften it.

## Purpose / core rule

Every subsystem must preserve the universal Tracewake authority split:

```text
holder-known context -> proposal / cognition / procedure decision
authoritative truth -> validation / resolution / physical mutation / debug comparison
modeled feedback -> future holder knowledge
```

For ordinary actors, cognition must pass through the actor decision transaction. For institutions, procedure must pass through an institution-known transaction. For LOD and regional processes, aggregate truth may simulate summaries, but promoted holders receive only modeled knowledge with ancestry.

## Replacement map

| Live architecture document | Primary contract |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Architecture map, authority order, replacement rule, and universal conformance questions |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | Rust-first authority boundaries, dependency direction, content/domain-pack authority, and forbidden inversions |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Event sourcing, replay, projections, save manifests, random streams, event migration, and diagnostics as replay material |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Holder-known context sealing, provenance, truth firewall, debug quarantine, and contamination review |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Shared action pipeline, proposals, validation truth, scheduler limits, actor-legible feedback, and failure semantics |
| `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Actor decision transaction, needs, commitments, intentions, routines, HTN methods, local planning, and stuck diagnostics |
| `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Typed claims, observation, belief, memory, records, rumors, traces, and information flow |
| `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | Structured speech acts, speaker/listener interpretation contexts, language surfaces, and optional LLM boundary |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Institution-known procedure transaction, households, records, norms, roles, proof, sanctions, and fallibility |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Missing-property first proof, ordinary life, settlement spatial model, property, work, sleep, food, and local economy |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Possession, TUI-first embodied play, actor-legible why-not, debug-only truth, view models, and client boundaries |
| `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Questless incidents, leads, notices, reports, observer-only story sifting, and second-proof deferral |
| `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | LOD as ontology, summary processes, promotion/demotion, regional ancestry, prehistory, and scale |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Acceptance gates, diagnostic artifacts, anti-contamination tests, replay proof, and review checklists |
| `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` | Research distilled into decisions, consequences, and forbidden misreads |

Retired architecture names, including the old state/content authoring document, the old Phase 3A actor-known autonomy document, and the old research-notes document, are absorbed into this replacement map. Do not keep them as parallel live contracts.

## Authority owned

The architecture layer owns subsystem contracts, canonical data-flow boundaries, diagnostic obligations, and acceptance implications. It may name implementation seams as examples only. It does not prescribe tickets, sprint order, task decomposition, branch names, or code patches.

## Authority denied

The architecture layer may not:

- change the project identity set by foundation;
- weaken ordinary actor parity or TUI-first playability;
- bless hidden-truth planning because current code happens to work that way;
- create player-only world verbs;
- treat display strings as proof;
- convert research precedent into a mandate to copy another game;
- defer the missing-property ordinary-life proof behind adventure systems.

## Universal conformance questions

Every feature, subsystem, fixture, test, view model, and report must answer these questions:

1. **Who is the holder?** Actor, institution, household, speaker, listener, embodied viewer, process, or LOD promotion recipient.
2. **What did that holder know at the decision boundary?** List typed claims, observations, memories, records, roles, procedure state, resources, and stale/uncertain information.
3. **Where did each input come from?** Every cognition/procedure input needs provenance: observation, memory, testimony, record, public artifact, schedule, role assignment, modeled rumor, summary event, or explicit “unknown.”
4. **What was sealed before decision?** The holder-known context must be constructed before candidate generation, method selection, local planning, speech interpretation, affordance menu selection, or institutional procedure choice.
5. **What did authoritative truth do?** Truth may validate, reject, resolve, mutate authoritative state, generate modeled observations, and support non-diegetic debug comparison.
6. **What truth was forbidden?** Truth must not select goals, plans, routine steps, speech meaning, view-model affordances, institutional procedure outcomes, leads, story beats, or LOD promotion knowledge.
7. **What feedback became future knowledge?** Rejections, observations, notices, reports, sanctions, and memories become future holder knowledge only through modeled channels.
8. **What replayable evidence proves this?** Event log, context packet, provenance graph, decision trace, stuck diagnostic, validation report, TUI transcript, and debug artifact.

A feature that cannot answer these questions is not architecture-conformant.

Single-charge derived accounting is owned as a compact seam across documents
04, 05, and 09: action/duration accounting owns derived deltas and terminal
lifecycle, actor cognition consumes pressures rather than minting them, and
ordinary-life food/sleep/work/economy share that one event-sourced charge path.
The hardening rows below remain conformance evidence for this seam.

Temporal authority is translated from `INV-112` by subsystem owners, not by
this index alone. Document 02 owns authoritative event/replay time; documents
03 and 06 own holder-known temporal claims, provenance, and freshness; document
04 owns the scheduler/validation temporal boundary; document 05 owns routine
and social-rhythm temporal premises; document 08 owns institutional procedural
time; documents 10, 11, and 07 own temporal rendering, lead labels, and speech
expressions; document 12 owns LOD temporal ancestry; and document 13 owns
temporal observability and review evidence. This entry is a pointer to those
contracts and does not define temporal mechanisms.

## Phase 3A evented cognition conformance

| Surface | Required allowed source | Required evidence |
|---|---|---|
| Workplace knowledge | Event-sourced notice/observation/record with `source_event_ids`; never a raw workplace table read, routine-template label, or fixture assignment copied into cognition. | Actor-known workplace facts cite live log events; negative fixtures prove raw assignment without notice does not plan work. |
| Sleep-surface knowledge | Event-sourced notice/observation/record with `source_event_ids`; current-place sleep affordance truth alone is not an information channel. | Actor-known sleep facts cite live log events; sleep proposals carry modeled affordance ids through the shared action pipeline. |
| Food-source knowledge | Event-sourced notice/observation/record with `source_event_ids`; nearest-food or hidden-container truth cannot select an eat plan. | Actor-known food facts cite live log events; hidden-food fixtures prove inaccessible truth is absent from planning inputs. |
| Route knowledge | Event-sourced notice/observation/record with `source_event_ids`; unseen adjacency/path truth cannot choose embodied exits or plans. | Actor-known route facts cite live log events; hidden-route fixtures prove unseen edges remain unavailable. |
| Typed place visibility | Place concealment is authored as typed `visibility_default` state; display labels and ids are presentation/identity only and cannot decide perception emission. | `visible_exit_perception_follows_typed_visibility_not_id_or_label_prose`, `fixtures_load_place_visibility_default_is_required`, and `guard_014_perception_visibility_uses_typed_place_visibility` lock typed authoring and perception gating. |
| Hidden-truth audit enforcement | Decision traces are not decorative; forbidden source classes fail closed before proposal construction, with typed stuck diagnostics and pipeline defense in depth. | `hidden_truth_input` diagnostics carry responsible layer, blocker code, and replayable ancestry; agent-origin proposals with dirty audits are rejected. |
| Completion continuity | Scheduled duration completions for sleep/work preserve action ancestry, body exclusivity, interruption checks, and prorated effects instead of becoming scheduler-authored outcomes. | Completion/interruption events are caused by the start event and replay byte-identically. |
| Ordinary-life tuning boundary | Passive need rates and sleep duration/recovery are content-authored flavor; need-band thresholds remain kernel semantics. | Content schema requires authored `need_model` and sleep tuning fields; guard tests ban the retired kernel tuning constants. |
| need-tick accounting authority | Passive need deltas are classified from authoritative actor state and tick regimes, not from proposal-supplied need values or fixture assumptions. | `classify_actor_tick_regimes` / `classify_actor_tick_regimes_with_start` drive awake/sleep/work deltas; validator guards reject `current_hunger` / `current_fatigue` proposal parameters as authority. |
| duration terminal-set | Body-exclusive duration completion/interruption semantics are shared through the event kind terminal set, not duplicated in scheduler or action definitions. | `is_duration_terminal` is used by scheduler and pipeline continuity checks; sleep/work completion tests prove terminal closure, reservation closure, and replay. |
| context-hash re-derivation gate | A holder-known context hash carried by a proposal is evidence only when it is recomputed from the current sealed context/frontier. | `source_context_check` compares proposal source fields to the current `KnowledgeContext` and `holder_known_context_hash`; replay rebuilds recompute hashes through `compute_holder_known_context_hash`. |
| cognition-substrate source | No-human cognition and embodied views derive actor-known facts from the durable epistemic projection, not raw authoritative tables or builder-local event-payload scans. | `NoHumanActorKnownSurfaceBuilder::from_projection` consumes `EpistemicProjection::actor_known_records_for_context`; source guards ban raw payload scans in the no-human surface. Closed by `archive/tickets/0016PHA3ANEEACC-015.md`. |
| Embodied workplace availability source | Human embodied affordance menus use the holder-known context, not the raw workplace assignment table, when exposing work options. | `actor_known_workplaces_for_context(context: &KnowledgeContext)` backs `phase3a_semantic_actions`; guard tests ban `actor_known_workplaces_for_context(state)` and raw `state.workplaces` access in that surface. |
| lock-layer workspace census | Anti-contamination gates must prove their own perimeter: every production source, negative fixture directory, and clippy ban is registered and justified. | `WORKSPACE_SOURCE_CLASSIFICATIONS`, `registered_negative_fixtures_match_directory_census`, and `clippy_ban_entries_have_proving_negative_fixtures` fail closed; `archive/reports/0016_ord_hard_025_mutants_baseline.md` records mutation counts and miss dispositions. |
| 0017 tick-charge attribution authority | Every tick-delta need charge, including action-emitted awake deltas, is reconciled by counted `(actor, need, tick)` occurrences rather than regime-label sets. | `assert_single_tick_delta_charge`, `AgentState::need_tick_charges`, `assert_no_duplicate_need_regime_charges`, `assert_single_need_charge_per_actor_tick_kind`, and `wait_then_window_passive_charges_each_tick_once_001` lock the one-charge-per-tick rule. |
| 0017 shared open-duration authority | Open body-exclusive duration state uses event-cause start ids across scheduler, pipeline, and need-accounting consumers; duplicate or orphan terminals are not silently reconciled. | `open_body_exclusive_starts`, `body_exclusive_reservation_conflict`, `terminal_ticks_by_start`, `duplicate_duration_terminal_violations`, and `duplicate_duration_terminal_poisons_rebuild_001` prove shared keying and fail-closed duplicate terminal handling. |
| 0017 projection freshness rule | The epistemic projection exposes one freshness classification for no-human cognition and embodied availability: current observations remain `observed_now`, older usable knowledge becomes remembered belief with original source timing. | `EpistemicProjection::actor_known_records_for_context`, `ActorKnownProjectionFreshness`, `ClassifiedActorKnownProjectionRecord::freshness`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, and `current_place_knowledge_context_uses_latest_projection_window_not_live_truth` lock the rule. |
| 0017 provenance-class audit | Provenance citations must witness the asserted fact kind; merely citing an existing event is insufficient. | `dangling_provenance_diagnostic`, `provenance_witness_mismatch`, and `guard_018_actor_known_facts_require_source_event_witness` fail closed for empty, dangling, or wrong-kind witnesses. |
| 0017 replay payload evidence | Agent event kinds that carry semantic payloads must be replay-visible through materialized state or checksum coverage; pure markers are the only no-op exception. | `AGENT_WORLD_NOOP_ALLOWLIST`, `AGENT_STATE_CHECKSUM_COVERAGE`, `no_human_capstone_proves_typed_ancestry_and_replay`, and tamper tests for continue-routine/candidate-goal payloads lock replay evidence. |
| 0017 believed-access embodied availability | Embodied Phase 3A affordances are enabled from actor-known believed access and known resources, with debug truth separated and commit-time validation still authoritative. | `ActorKnownWorkplaceFact::believed_access_open`, `phase3a_semantic_actions`, `embodied_workplace_availability_reflects_belief_not_truth_001`, and `embodied_workplace_believed_open_truth_closed_commit_fails_001` prove belief/truth divergence. |
| 0017 content policy registry | Content-authored numeric fields and scanned strings are classified by explicit policy; loaders reject out-of-band values instead of silently clamping or partially scanning. | `NUMERIC_FIELD_REGISTRY`, `SCANNED_STRING_FIELDS`, `schema_numeric_fields_are_classified_for_validation_policy`, and `schema_string_fields_are_classified_for_script_scanning` lock registry parity. |
| 0017 lock-layer durability | The lock perimeter includes typed source-event witnesses, workspace-wide cognition-input truth-reader bans, whitespace-normalized guard checks, and a governed mutation baseline. | `SourceEventIds` storage on `ActorKnownFact`, `cognition_inputs_are_context_backed`, CI `mutants-in-diff`, and `.cargo/mutants-baseline-misses.txt` record the durable lock surfaces. |
| 0018 witness-compatibility census | Actor-known fact stable ids must have explicit witness-kind compatibility arms; unlisted facts fail closed instead of inheriting a wildcard allowance. | `witness_kind_allowed`, `guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms`, and `provenance_witness_unlisted_stable_id_fails_closed_before_proposal` lock the witness census and wildcard-true ban. |
| 0018 episode payload checksum coverage | Materialized ordinary-life episode records carry semantic payload fields into agent-state checksum coverage; payload-bearing agent records cannot rely on summaries alone. | `OrdinaryLifeEpisodeRecord::payload_fields`, `join_pairs(&episode.payload_fields)`, `materialized_agent_payload_records_keep_payload_fields`, and episode tamper tests prove payload checksum coverage. |
| 0020 derived apply-arm payload census | Every `events/apply.rs` write into a checksum-covered `AgentState` map must be found by a fail-closed source census and must either require a supported payload/trace/diagnostic schema version at the specific apply arm or carry an explicit typed-column-closure exemption whose consumed payload keys are listed. | `materialized_agent_apply_arms_require_payload_schema_version` derives covered map writes from `AGENT_STATE_CHECKSUM_COVERAGE` and `events/apply.rs`, anchors inline writes to their nearest `EventKind::` arm, fails on unknown write shapes, `&mut state.<map>` rebounds, and raw `payload_fields(` outside gated materialized writes; `typed_column_closure_exemptions_are_rationale_bearing_and_live` derives exemption consumed keys; synthetic regressions cover ungated second arms, `retain`, rebound aliases, raw payload retention, and unlisted consumed keys. |
| 0020/0022 mutation-perimeter completion | The lock-layer mutation perimeter includes actor cognition, scheduler, projections, action pipeline, and the guarded eat/sleep/work action definitions across scheduled and in-diff CI jobs; scheduled/manual mutation runs must not be canceled mid-flight, and workflow concurrency must isolate event classes so scheduled ratchets cannot be canceled by push runs. | `.github/workflows/ci.yml` captures every `cargo mutants` status, lets accepted exit-2 misses flow into the baseline ratchet, scopes scheduled filters to the executing step, evaluates the in-diff guarded-path regex against canaries, rejects perimeter-covering `exclude_globs`, includes `github.event_name` in the concurrency group, and exempts `schedule`/`workflow_dispatch` from concurrency cancellation; `mutation_perimeter_matches_duration_action_rationale_and_ci_filters` includes synthetic failures for status-capture removal, `|| true`, exclusion globs, comment decoys, narrowed regexes, false mutation rationales, missing scheduled `eat.rs` filters, and event-name concurrency removal. |
| 0020/0021 two-sided embodied-field sweep | Every embodied view/status surface enrolled from `view_models.rs`, including selected scalar fields and enum payload fields, must have a reachable non-default producer and a TUI render/app consumer, or a cited deferral; dead view-model surfaces are unfinished mechanics, not harmless shape. | `embodied_view_option_and_collection_fields_have_reachable_producers` derives the field inventory from `view_models.rs`, fails on unmatched census entries, handles `ActionAvailability` enum payloads, scopes producer matches to the owning struct, scans TUI consumers, requires cited `EMBODIED_SURFACE_FIELD_PRODUCERS` entries for external population/deferral, and covers hardwired defaults, enum fields, cross-struct aliases, and produced-but-unconsumed fields with synthetics. |
| 0021 possession-rebind rejection hygiene | Possession rebinds must not carry the previous actor's rejection facts into the new actor's embodied why-not surface; embodied availability provenance must use actor-visible validator facts only, and global context frontiers shown in embodied render output must be marked non-diegetic. | `adversarial_gates_possession_rebind_does_not_transfer_rejection_why_not`, `rejection_report_must_match_viewer_before_embodied_projection_renders_it`, `disabled_embodied_availability_provenance_uses_actor_visible_facts_only`, `embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance`, and `renderer_marks_knowledge_context_frontier_non_diegetic` lock the host, projection, source-guard, and render surfaces. |
| 0021 harness-provenance fidelity | Hidden-truth gates must prove modeled actor knowledge through applied event-log records, not fixture-local planning-context fabricators or fabricated event ids. | `hidden_truth_gates.rs` builds gate contexts through `EventLog` plus `apply_epistemic_event`; `guard_0021_hidden_truth_gates_use_event_log_provenance`, `guard_0021_fabricated_visible_local_event_id_is_retired`, and `guard_0021_actor_known_context_producers_are_projection_backed` lock the harness and production producer surface. |
| 0021/0022 actor-known projection policy dispatch | Projection-record classification, freshness stamping, accessibility-fact minting, and embodied-context inclusion must be declared per record kind and dispatched from the policy table, not open-coded per consumer; the lock is behavioral, not just a source-presence echo. | `ActorKnownProjectionRecord::policy`, `actor_known_projection_policy_kinds`, `classified_actor_known_records_for_context`, `NoHumanActorKnownSurfaceBuilder::consume_projection_record`, `current_place_knowledge_context`, `guard_0021_actor_known_projection_policy_table_has_production_callers`, `actor_known_projection_policy_table_drives_record_behavior`, and `synthetic_policy_table_behavior_drift` lock per-kind behavior; sleep-place accessibility mirrors food from any remembered place while embodied current-place views retain their declared latest-current-place scope. |
| 0022 shared need-delta emitter perimeter | Eat, sleep, work, and scheduler duration paths must not construct `NeedDeltaApplied` envelopes directly; need deltas and threshold crossings route through the shared emitter so all action families share one accounting shape. | `build_need_delta_and_threshold_events`, `guard_006_duration_need_deltas_route_through_shared_emitter`, `direct_duration_need_delta_construction_violations`, and the synthetic `crates/tracewake-core/src/actions/defs/eat.rs` direct `EventKind::NeedDeltaApplied` case lock the shared-emitter perimeter. |
| 0022 embodied debug-render split | Debug rendering is a non-diegetic read-only product, separate from embodied affordance selection, typed ordering, replay, and actor-facing current view state. | `phase3a_debug_surfaces_render_deterministically_and_read_only`, `adversarial_gates_rendering_does_not_change_typed_order_or_replay`, and the debug-panel source scan in `tui_acceptance.rs` prove debug panels render deterministically without calling event application or altering embodied affordances. |
| 0022 planner hidden-truth adversarial restoration | Hidden-truth planner gates require adversarial witness fixtures, not empty contexts or debug-only facts, and the harness must build modeled knowledge by applying event-log records. | `planner_hidden_truth_fixture_witness_errors`, `planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context`, `hidden_food_closed_container_is_not_actor_known_food_source`, `hidden_route_edge_absent_from_actor_context_blocks_route_plan`, and `guard_0021_hidden_truth_gates_use_event_log_provenance` lock the adversarial witness path and fail closed on empty hidden-truth fixtures. |
| 0022 meta-lock tier | Structural lock tests are themselves enrolled in a registry: each shared scan or behavior lock must have a registered negative, a live routing decision, and a nonzero witness floor. | `META_LOCK_REGISTRY`, `META_LOCK_REGISTRY_MIN_ENTRIES`, `meta_lock_registry_covers_structural_locks_and_negatives`, `meta_lock_registry_errors`, `synthetic_meta_lock_without_negative`, and the nonzero-witness synthetic prove the bijection and witness-floor rules; `generative_lock_two_sided_floor_ratchets` and `generative_support_constructs_zero_event_envelopes` are enrolled examples. |
| 0023 embodied-locality epistemic migration | Human embodied locality views derive from sealed actor-known current-place context, not raw world locality truth, while observation events retain replayable concealment and witness ids. | `current_place_knowledge_context`, `EmbodiedProjectionSource::from_sealed_context`, `build_embodied_view_model`, `embodied_projection_source_cannot_carry_raw_physical_state`, and `current_place_knowledge_context_uses_latest_projection_window_not_live_truth` lock the migration and INV-093 absence negative. |
| 0023 meta-lock witness/census/ratchet repairs | Structural lock membership, negative routing, live witness floors, and ratcheted baseline floors are measured by the scans they govern rather than authored as inert prose. | `META_LOCK_REGISTRY`, `meta_lock_registry_covers_structural_locks_and_negatives`, `meta_lock_registry_errors`, `generative_lock_source_uses_two_sided_recorded_floors`, `generative_lock_two_sided_floor_ratchets`, and `synthetic_0023_missing_acceptance_anchor` lock the repaired meta tier. |
| 0023 policy surface-driven behavioral lock | Actor-known projection policy rows must drive both no-human and embodied behavior; a policy row cannot drift while tests only check symbol presence. | `ActorKnownProjectionRecord`, `policy`, `actor_known_projection_policy_table_drives_record_behavior`, `synthetic_policy_table_behavior_drift`, and `classified_actor_known_records_for_context` lock the surface-driven behavior across policy kinds. |
| 0023 debug-overlay production wiring | The debug embodied overlay is a production debug command and remains non-diegetic, token-derived, and separate from ordinary embodied rendering. | `DebugCommand::Overlay`, `render_debug_embodied_overlay`, `render_debug_overlay`, `DEBUG_TOKENS`, `app_debug_overlay_renders_only_for_bound_embodied_controller`, and `debug_overlay_marks_knowledge_context_frontier_non_diegetic` lock the production wiring and derived-token negative. |
| 0024 fixture schema-version gate | Top-level fixture schema versions are part of the content contract and unsupported versions fail during validation/load instead of silently loading. | `validate_schema_version`, `validate_fixture`, `load_fixture_package`, `fixtures_load_unsupported_schema_version_rejected_001`, `fixture_schema_version_unsupported_rejected_001`, and `unsupported_fixture_schema_version` lock `ORD-HARD-140`. |
| 0024 meta-witness completion | Structural guard enrollment must be measured by live witnesses and executed negatives, with no presence-check fallback arm or self-satisfying citation. | `META_LOCK_REGISTRY`, `meta_lock_registry_covers_structural_locks_and_negatives`, `meta_lock_live_witness_count`, `witness_min`, `synthetic_meta_lock_without_negative`, and `synthetic_0024_missing_acceptance_anchor` lock `ORD-HARD-141` and the capstone artifact parity. |
| 0024 embodied truth-access removal | Embodied projection construction may capture a sealed truth snapshot before preflight, but ongoing embodied view construction must not carry raw `PhysicalState` through actor-known projection sources. | `EmbodiedTruthSnapshot`, `EmbodiedProjectionSource::from_sealed_context`, `EmbodiedPreflightSource`, `guard_014_embodied_projection_source_has_no_physical_state_field`, and `embodied_place_label_is_captured_before_truth_preflight` lock `ORD-HARD-143`. |
| 0024 derived apply perimeter | Direct event-apply mutator calls are derived from the public `events/apply.rs` apply surface and are allowed only in replay/pipeline/scheduler seams with rationale-bearing entries. | `APPLY_MUTATOR_ALLOWLIST`, `no_direct_apply_event_outside_event_replay_or_pipeline`, `apply_agent_event`, `apply_story_event`, and `append_and_apply_agent_event` lock `ORD-HARD-144`. |
| 0024 agent cognition priority decision | Phase 3A intentionally keeps the current foundation-defensible order: severe needs first, urgent need/routine duty before active-intention continuation, and active continuation before mild pressure; this records the `ORD-HARD-161` divergence from the archived 0005 recommendation as an owned decision. | `GoalPriority::selection_rank`, `goal_priority_selection_rank_snapshot_pins_decided_order`, `urgent_need_vs_active_intention_follows_documented_order`, and `mild_hunger_continues_active_intention` pin the decided order. |
| 0025 executable meta-witness discipline | Structural locks must witness executed negatives or inspected sites; assertion-token-count and anchor-presence witnesses are forbidden. | `META_LOCK_REGISTRY`, `meta_lock_live_witness_count`, `behavior_assertion_inspected_site_count`, `witness_shape_ban_errors`, and `meta_lock_registry_covers_structural_locks_and_negatives` lock `ORD-HARD-166`/`175` with present-but-vacuous synthetics. |
| 0025 provenance-true perception taint | Perception guards must key taint to argument provenance and inspected emission paths, not raw helper tokens. | `perception_prose_scan_inspected_line_count`, `guard_014_perception_visibility_uses_typed_place_visibility`, `perception_line_is_current_place_label_recording`, and `visible_exit_perception_follows_typed_visibility_not_id_or_label_prose` lock `ORD-HARD-167`. |
| 0025 envelope read-path fail-closed decisions | Canonical event envelope decoding rejects duplicate fields and unsupported schema versions loudly, and the hollow `checksum_after` field remains absent from serialized envelopes. | `EventEnvelope::deserialize_canonical`, `EventEnvelopeParseError::DuplicateField`, `envelope_direct_decode_rejects_unsupported_schema_version`, `event_envelope_direct_decode_rejects_unsupported_schema_version`, and `event_envelope_has_no_hollow_checksum_after_field` lock `ORD-HARD-168`/`171`/`184`. |
| 0025 manifest fingerprint honesty | Fixture manifests fingerprint raw primary and secondary bytes, keep frozen fixture fingerprints, and pin canonical serialization bytes. | `fingerprint_payload`, `fixture_fingerprints_match_frozen_goldens`, `fixture_fingerprint_reprices_secondary_file_bytes`, `fixture_fingerprint_reprices_raw_primary_bytes_with_same_parsed_fixture`, and `fixture_serialization_golden_bytes_are_pinned_001` lock `ORD-HARD-169`/`170`/`183`. |
| 0025 embodied carrier census and observation capture | Embodied views may use preflight truth only through enrolled carriers and must render observed place/carried-item facts rather than later live truth. | `source_shape_errors`, `record_current_place_perception_and_project`, `actor_known_current_place_facts`, `actor_known_carried_item_facts`, `embodied_projection_renders_observed_place_label_not_live_truth`, and `embodied_projection_renders_observed_carried_item_attributes_not_live_truth` lock `ORD-HARD-172`/`173`. |
| 0025 TUI debug gate depth and ControllerMode decision | World-advancing debug commands require intrinsic debug availability, debug dispatch is arm-complete, and only `ControllerMode::Debug` grants debug. | `run_no_human_day_refuses_intrinsically_without_debug_availability`, `debug_dispatch_routes_every_arm_through_availability_gate`, `tui_local_guard_registry_covers_structural_guards`, `debug_available_for`, and `controller_mode_debug_availability_decision_is_explicit` lock `ORD-HARD-174`/`176`/`185`/`186`. |
| 0025 census/oracle derivation closures | Positive proof, actor-known projection, support-file, exemption, ID-field, and `Location` scans derive their populations and fail on synthetic drift. | `positive_proof_fixtures_emit_typed_artifacts_first`, `actor_known_projection_policy_truth_table_detects_predicate_inversion`, `schema_id_type_recognizer_matches_schema_imports`, `fixtures_load_location_embedded_marker_id_rejected_001`, and `acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors` lock `ORD-HARD-177` through `ORD-HARD-182`. |
| 0025 CI and evidence-honesty closure | CI gate commands, cache keys, job documentation, recovery staging, and mutation phase-entry evidence are explicit guarded records rather than silent workflow drift. | `.github/workflows/ci.yml`, `ci_workflow_guards_cover_workflow_integrity`, `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, and `mutation_pending_certification_wording_errors` lock `ORD-HARD-187` through `ORD-HARD-190`. |
| 0021 INV-087 bind-time perception decision | Bind-time perception events remain an unresolved doctrine tension: either move perception into an actor decision transaction or approve a foundation clarification that possession-triggered modeled perceptions are allowed. This pass records the owner decision as deferred and makes no silent constitutional edit. | `archive/tickets/0021PHA3APOSREB-012.md` records the deferred owner decision for `ORD-HARD-095`; no change is made to `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` under this ticket. |
| 0020/0022 mutation-baseline governance | Accepted mutation misses are pinned by normalized count/hash and by a reviewed per-entry disposition ledger with closed disposition tags; the 0022 triage pass must classify every remaining entry as either current accepted baseline or a concrete `warrants-test:<ticket-id>` follow-up. Unledgered misses, stale ledger entries, unsupported tags, missing `warrants-test` tickets, deferred-test prose in `justified-baseline`, and bulk-repeated rationales fail the guard. | `.cargo/mutants-baseline-misses.txt`, `archive/reports/0020_mutants_baseline_disposition.md`, `MUTANTS_BASELINE_NORMALIZED_COUNT`, `MUTANTS_BASELINE_NORMALIZED_FNV1A64`, and `mutation_baseline_misses_are_pinned_and_ledgered` lock the accepted baseline surface with synthetic unledgered append, unrecorded floor raise, repeated rationale, missing-ticket, and deferred-test-language failures. |
| 0018 seed-knowledge grammar | Authored seed knowledge must pass the content validation grammar before it can build agent state; fixture validation mints the only token accepted by `to_agent_state`. | `FixtureValidationToken`, `FixtureSchema::to_agent_state(validation)`, compile-fail doctests, and seed fixtures such as `seeded_food_source_unknown_to_all_actors_001` lock explicit seed-knowledge edges. |
| 0018 zero-dependency census | Crate ownership and dependency posture are checked in-tree: core stays dependency-free and the workspace dependency set must match a committed allowlist. | `workspace_dependency_posture_matches_allowlist` parses workspace `Cargo.toml` files and asserts `tracewake-core` `[dependencies]` remains empty. |
| 0019-corrected generative reachability contract | Bounded generated scheduled no-human sequences exercise wait, eat, sleep, work, duration terminals, interruptions, replay, prefix replay, payload, checksum, and single-charge locks only from events emitted by the `advance_no_human` path under test. This corrects the 0018 overstatement: the old terminal/interruption counters were satisfied after the advance by a test-side builder helper. | `advance_no_human` now processes due completions through the shared `append_due_completions` path; `generated_sequences_replay_and_satisfy_metamorphic_locks` runs without direct completion-builder calls; `generative_lock_cannot_fabricate_duration_terminals` bans `build_sleep_completion_events`, `build_work_completion_events`, and the removed fabricator from `generative_lock.rs`. |
| 0019 mutation-perimeter honesty | The mutation lock layer includes guarded action-definition builders, treats cargo-mutants tool failure as a failed gate, and separates true mutation-perimeter rationale from non-perimeter action-code rationale. | `.cargo/mutants.toml` no longer excludes `actions/defs/**`; CI filters include `actions/defs/eat.rs`, `actions/defs/sleep.rs`, and `actions/defs/work.rs`; `CORE_ACTION_MUTATION_PERIMETER_RATIONALE` is used only for perimeter paths, and `mutation_perimeter_matches_duration_action_rationale_and_ci_filters` checks the mutants config, CI failure semantics, guarded push path, and classification rationale. |
| 0019 seed-knowledge helper containment | Blanket food-source seed knowledge is a legacy fixture convenience, not the default authoring path for new fixtures; partial food-source knowledge must stay expressible and tested. | `known_food_source_blanket_helper_call_sites_are_allowlisted` pins legacy `populate_known_food_sources_for_all_actors` call sites; `partial_food_source_knowledge_001` authors one per-actor `known_food_sources` edge and `partial_food_source_knowledge_seeds_only_authored_actor_edge` proves the edge-bearing actor can plan while the edge-less actor cannot. |

### Audit-history overturns

The 0016 audit overturned two 0015 verified-holding/outcome claims: the claim that passive deltas do not double-count sleep ticks and the claim that context-hash rebuild was locked. They are recorded as `ORD-HARD-014` and `ORD-HARD-016` findings respectively; future audits must cite the overturn rather than silently rewriting the 0015 audit history.

Spec 0017 overturned three 0016 acceptance-evidence claims in `ORD-HARD-034`:
the synthetic prior-version event upcast fixture was not delivered and the
schema contract is loud-rejection-only; the workplace belief/truth divergence
fixture was a placeholder until 0017 delivered the real divergence pair; and
the named content negative fixtures were delivered as inline `validate.rs`
tests and registry census tests. Future audits must cite the 0017 correction
section in `archive/reports/0016_ord_life_cert_scoped_acceptance.md` rather than
silently inheriting the older report text.

## Required architectural spine

For ordinary actors:

```text
trigger
 -> sealed actor-known context with provenance
 -> live needs / commitments / intentions / routines / obligations
 -> candidate generation from actor-known pressures only
 -> intention continuation/adoption/interruption/drop
 -> HTN/routine/procedure method selection
 -> bounded local planning from actor-known context
 -> ordinary action proposal with ancestry
 -> shared action pipeline validation against authoritative truth
 -> event commit or rejection/failure semantics
 -> actor-legible feedback and modeled observations
 -> belief/projection updates
 -> typed decision/stuck diagnostics
 -> replay/debug/TUI projections
```

For institutions:

```text
institution trigger
 -> sealed institution-known context from records, reports, role knowledge, resources,
    jurisdiction, procedure state, public artifacts, and institutional memory
 -> procedure candidate generation / role decision
 -> proposal or institutional action through shared validation/event pipeline
 -> records/notices/orders/sanctions/failures only through modeled authority and resources
 -> public/actor/institution knowledge updates only through modeled channels
 -> debug-visible truth comparison without leakage
```

For LOD and regional processes:

```text
summary truth may maintain aggregate simulation
but promoted actors/institutions receive only knowledge with modeled ancestry
```

## Anti-patterns

Reject any implementation that:

- sends a need threshold directly into a primitive action;
- sends a routine label directly into movement, work, sleep, eat, report, sanction, or travel;
- reads a workplace table, household table, fixture truth, or route graph to choose an actor's plan without holder-known provenance;
- uses validation truth to suggest a better action;
- treats “the actor would know” as enough without a typed source;
- returns actor-facing why-not text that reveals hidden truth;
- uses debug truth in embodied affordance menus;
- creates quest states, pacing managers, clue spawners, reward spawners, drama directors, or authored outcome chains;
- requires an LLM for acceptance, replay, planning, or authoritative truth.

## Cross-document obligations

- The event/replay contract in document 02 must record enough evidence to replay decisions and diagnose contamination.
- The holder-known contract in document 03 is the firewall used by documents 04 through 12.
- The action pipeline in document 04 is the only route to world mutation for ordinary actions.
- The actor transaction in document 05 is the only route from ordinary actor pressure to action proposal.
- Institution procedures in document 08 must be as epistemically bounded as actor cognition.
- TUI behavior in document 10 must expose actor-legible views and debug-only truth as separate products.
- Acceptance in document 13 must fail plausible behavior if provenance or replay evidence is missing.
