# 0023 Ordinary-Life Certification Scoped Acceptance

Scope: evidence toward `ORD-LIFE-CERT` for Phase 3A embodied-locality,
meta-lock, witness-integrity, and scan-evasion hardening. This is not
full-project certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

Implementation commits: 0023PHA3AEMBLOC-001 through 0023PHA3AEMBLOC-013.

## Spec Section 7 Evidence Map

| Item | Report anchor | Finding anchors |
|---|---|---|
| 1 | Embodied Locality Migration | `ORD-HARD-121` |
| 2 | Meta-Lock Registry And Witness Repair | `ORD-HARD-122`, `ORD-HARD-123`, `ORD-HARD-124`, `ORD-HARD-129` |
| 3 | Debug Overlay Wiring | `ORD-HARD-125`, `ORD-HARD-135` |
| 4 | Policy Surface-Driven Lock | `ORD-HARD-126` |
| 5 | Scan-Evasion Closures | `ORD-HARD-127`, `ORD-HARD-128`, `ORD-HARD-130`, `ORD-HARD-132`, `ORD-HARD-136` |
| 6 | In-Context Witness And Panic Closure | `ORD-HARD-131` |
| 7 | Canonical Intent And Sleep Positive | `ORD-HARD-137`, `ORD-HARD-138` |
| 8 | Cause Disposition And Baseline Governance | `ORD-HARD-139`, `ORD-HARD-134` |
| 9 | 0022 Evidence-Honesty Correction | `ORD-HARD-133`, `acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors` |
| 10 | Premise-Held Confirmations | `ORD-HARD-121` through `ORD-HARD-139` |
| 11 | Risk Register And Conformance Diffs | `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` |
| 12 | EMERGE-OBS Derivation And Scheduled Run Status | `emerge_obs_v1` |
| 13 | EMERGE-OBS Derivation And Scheduled Run Status | scheduled mutation still pending under the post-0022 posture |
| 14 | Explicit Non-Certification Statement | not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT` |

## 1. Embodied Locality Migration

`ORD-HARD-121` is closed by deriving embodied locality from sealed
`KnowledgeContext` data. `EmbodiedProjectionSource::from_sealed_context` no longer
carries raw `PhysicalState`; `build_embodied_view_model` consumes the sealed
source. The guard `embodied_projection_source_cannot_carry_raw_physical_state`
contains the INV-093 absence negative and fails if `PhysicalState`,
`source.state`, or raw `visible_locality` returns to the embodied builder.

Positive coverage remains in `current_place_knowledge_context_uses_latest_projection_window_not_live_truth`
and the related current-place sleep and workplace-context tests.

## 2. Meta-Lock Registry And Witness Repair

`ORD-HARD-122`, `ORD-HARD-123`, `ORD-HARD-124`, and `ORD-HARD-129` are closed by
the repaired `META_LOCK_REGISTRY` tier. `meta_lock_registry_covers_structural_locks_and_negatives`
checks registered negatives, routing, and nonzero witness floors. Synthetic
failures cover the anchor-miss path, unregistered structural locks, and missing
negative fixtures. Chained floor governance is locked by
`generative_lock_source_uses_two_sided_recorded_floors` and
`generative_lock_two_sided_floor_ratchets`, with the missing-predecessor
synthetic routed through the same scan.

## 3. Debug Overlay Wiring

`ORD-HARD-125` is closed by production command wiring:
`DebugCommand::Overlay` calls `TuiApp::render_debug_embodied_overlay`, which calls
`render_debug_overlay`. `ORD-HARD-135` is closed by deriving overlay assertions
from `DEBUG_TOKENS`; `debug_overlay_marks_knowledge_context_frontier_non_diegetic`
is the derived-token negative surface.

## 4. Policy Surface-Driven Lock

`ORD-HARD-126` is closed by `actor_known_projection_policy_table_drives_record_behavior`.
The test exercises the policy table across projection record kinds and both
no-human and embodied surfaces; `synthetic_policy_table_behavior_drift` mutates a
row and proves the behavior lock, not just a source-presence echo.

## 5. Scan-Evasion Closures

`ORD-HARD-127`, `ORD-HARD-128`, `ORD-HARD-130`, `ORD-HARD-132`, and
`ORD-HARD-136` are closed by widening the scans to the evasion shapes found in
0023. The laundering, receiver/alias, logical shell-line, `.unwrap(`, and
`EventEnvelope` construction synthetics all fire through the production scan
paths. The logical shell-line path is `logical_shell_lines`; the panic guard is
`log_derived_panic_guard_scans_unwrap`; the envelope ban covers struct literal,
default, and direct constructor surfaces.

## 6. In-Context Witness And Panic Closure

`ORD-HARD-131` is closed by moving the hidden-truth discrimination witness into
the real `context()` path in `hidden_truth_gates.rs`. The synthetic hidden-food
injection continues to fail through the same path instead of through a separate
fixture-only assertion.

## 7. Canonical Intent And Sleep Positive

`ORD-HARD-137` is closed by the Mara canonical recovery coherence assertion in
`golden_fixtures_run.rs`; the recorded
`canonical_mara_recovery_resolution=fail_only_empty_food_source` token now has a
runtime outcome assertion. `ORD-HARD-138` is closed by the positive embodied
sleep reachability test in `tui_acceptance.rs`, proving a submittable `sleep`
semantic action rather than only proving absence.

## 8. Cause Disposition And Baseline Governance

`ORD-HARD-139` is closed by making `EventKind::cause_required` an exhaustive
`match` with no default arm. The existing action-emitted event-kind guard remains
as a behavioral cross-check. `ORD-HARD-134` is closed by re-arming non-empty
mutation baseline governance: non-empty baseline entries require ledger
dispositions and the unledgered non-empty synthetic fails.

## 9. 0022 Evidence-Honesty Correction

`ORD-HARD-133` is superseded here rather than silently rewritten: this report
states the 0022 correction and records that the 0023 acceptance report is guarded
by `acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors`. The
synthetic missing-anchor row proves the checklist parity guard fails through the
real checker.

## 10. Premise-Held Confirmations

All nineteen findings, `ORD-HARD-121` through `ORD-HARD-139`, still held at
implementation time. The twelve audit findings and seven reassessment findings
were implemented as live code, test, or documentation locks; none was refuted
and silently dropped.

## 11. Risk Register And Conformance Diffs

The risk register records the literal-witness-count and binding-laundering
shapes under R-29 and the census-membership watch under R-28. The conformance
index now has rows for the embodied-locality epistemic migration, meta-lock
witness/census/ratchet repairs, policy surface-driven behavioral lock, and
debug-overlay production wiring.

## 12. EMERGE-OBS Derivation And Scheduled Run Status

Derivation command:

```sh
cargo test -p tracewake-core --test emergence_ledger -- --nocapture
```

Recorded `emerge_obs_v1` table:

```text
emerge_obs_v1|rows=12
row|corpus_id=generative_seed_18000010|start_tick=1|end_tick=13|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=5
row|corpus_id=generative_seed_18000011|start_tick=0|end_tick=21|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=3|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=6
row|corpus_id=generative_seed_18000012|start_tick=0|end_tick=10|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=6
row|corpus_id=generative_seed_18000013|start_tick=0|end_tick=10|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=2|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
row|corpus_id=generative_seed_18000014|start_tick=2|end_tick=20|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=6
row|corpus_id=generative_seed_18000015|start_tick=2|end_tick=21|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=10
row|corpus_id=generative_seed_18000021|start_tick=1|end_tick=22|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=3|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
row|corpus_id=generative_seed_18000023|start_tick=0|end_tick=10|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=2|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
row|corpus_id=generative_seed_18000024|start_tick=0|end_tick=16|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
row|corpus_id=generative_seed_18000029|start_tick=1|end_tick=24|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=10
row|corpus_id=generative_seed_1800002a|start_tick=1|end_tick=7|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=1|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
row|corpus_id=generative_seed_18000057|start_tick=0|end_tick=13|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=0|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=7
total|expectation_contradictions=0|replans_or_fallbacks=0|interruptions=11|intention_switches_with_causes=0|stuck_by_blocker=none|belief_truth_divergences=0|modeled_channel_corrections=0|distinct_event_kinds=13
```

This is measurement only, with no certification thresholds. The first scheduled
mutation run under the post-0022 posture is still pending in this local evidence
surface.

## 13. Explicit Non-Certification Statement

This artifact is scoped evidence toward `ORD-LIFE-CERT`. It is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.
