# 0024 Ordinary-Life Certification Scoped Acceptance

Scope: evidence toward `ORD-LIFE-CERT` for Phase 3A schema-version,
meta-witness, embodied truth-access, mutation-perimeter, content-loader, TUI
debug, projection-policy, oracle, and 0005-coherence hardening. This is not
full-project certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

Implementation commits: 0024PHA3ACONSCH-001 through 0024PHA3ACONSCH-014.

## Spec Section 7 Evidence Map

| Item | Report anchor | Finding anchors |
|---|---|---|
| 1 | Fixture Schema-Version Gate | `ORD-HARD-140` |
| 2 | Meta-Witness Completion | `ORD-HARD-141`, `ORD-HARD-142`, `ORD-HARD-145`, `ORD-HARD-146`, `ORD-HARD-155` |
| 3 | Truth-Access Removal | `ORD-HARD-143`, `ORD-HARD-154` |
| 4 | Derived Apply Perimeter | `ORD-HARD-144` |
| 5 | Content Loader Closures | `ORD-HARD-150`, `ORD-HARD-151`, `ORD-HARD-164`, `ORD-HARD-165` |
| 6 | TUI Debug Quarantine | `ORD-HARD-152`, `ORD-HARD-153` |
| 7 | Policy And Projection Closures | `ORD-HARD-147`, `ORD-HARD-156`, `ORD-HARD-148`, `ORD-HARD-149` |
| 8 | Oracle Closures | `ORD-HARD-157`, `ORD-HARD-158`, `ORD-HARD-159`, `ORD-HARD-160` |
| 9 | 0005 Coherence Decisions | `ORD-HARD-161`, `ORD-HARD-162`, `ORD-HARD-163` |
| 10 | Premise-Held Confirmations | `ORD-HARD-140` through `ORD-HARD-165` |
| 11 | Documentation Diffs | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` |
| 12 | EMERGE-OBS Derivation | `emerge_obs_v1` |
| 13 | Mutation Run Status | scheduled mutation still pending |
| 14 | Explicit Non-Certification Statement | not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT` |

## 1. Fixture Schema-Version Gate

`ORD-HARD-140` is closed by validating top-level fixture schema versions through
`validate_schema_version`, `validate_fixture`, and `load_fixture_package`.
`fixtures_load_unsupported_schema_version_rejected_001` and
`fixture_schema_version_unsupported_rejected_001` prove unsupported
`fixture.schema_version` values fail with `unsupported_fixture_schema_version`.
The golden fixture path remains pinned by `fixture_schema_v1_golden_bytes_load_001`
and the per-version fixture constructors.

## 2. Meta-Witness Completion

`ORD-HARD-141` and `ORD-HARD-146` are closed by the live
`META_LOCK_REGISTRY` witness checks. `meta_lock_registry_covers_structural_locks_and_negatives`
rejects missing negatives, missing routing, missing anchors, and witness counts
that fall below `witness_min`; the formerly decorative presence-fallback shape
now fails through the measured witness count. `ORD-HARD-142` is closed by the
orphaned-definition synthetic on write-shaped deferral cites. `ORD-HARD-145` is
closed by the genesis-anchored generative floor ratchet and its fabricated-ledger
synthetic. `ORD-HARD-155` is closed by rationale-bearing census exemptions in the
meta-lock and typed-column closure guards.

## 3. Truth-Access Removal

`ORD-HARD-143` is closed by replacing direct embodied `PhysicalState` access with
`EmbodiedTruthSnapshot`, `EmbodiedProjectionSource::from_sealed_context`, and
`EmbodiedPreflightSource`. `guard_014_embodied_projection_source_has_no_physical_state_field`
locks the absence of a raw physical-state field on the embodied projection
source. `ORD-HARD-154` is closed by
`embodied_place_label_is_captured_before_truth_preflight`, which proves the
place label is captured before later truth mutation.

## 4. Derived Apply Perimeter

`ORD-HARD-144` is closed by deriving the apply-mutator surface from
`events/apply.rs` instead of hand-picking a token subset. The guard
`no_direct_apply_event_outside_event_replay_or_pipeline` derives `apply_*`
tokens, checks the rationale-bearing `APPLY_MUTATOR_ALLOWLIST`, and fails the
synthetic direct `apply_agent_event` and synthetic `apply_story_event` cases.

## 5. Content Loader Closures

`ORD-HARD-150` is closed by production raw-line scan wiring and
`fixtures_load_raw_line_prose_born_fact_rejected_001`. `ORD-HARD-151` is closed
by the ID-field shortcut scan and its negative fixture. `ORD-HARD-164` is closed
by explicit authored need seeds and `fixture_missing_actor_need_seed_rejected_001`.
`ORD-HARD-165` is closed by golden serialization bytes through
`fixture_serialization_golden_bytes_are_pinned_001` and
`fixture_schema_v1_golden_bytes_load_001`.

## 6. TUI Debug Quarantine

`ORD-HARD-152` and `ORD-HARD-153` are closed by routing `run no-human-day` behind
the debug namespace and availability checks. `DebugCommand::RunNoHumanDay`,
`TuiApp::debug_available`, `debug_commands_refuse_without_debug_availability`,
and `top_level_no_human_day_command_is_not_a_play_verb` prove the operator-only
classification. The staging decision is recorded in
`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`.

## 7. Policy And Projection Closures

`ORD-HARD-147` and `ORD-HARD-156` are closed by policy-driven workplace
accessibility and a classifier-derived projection oracle:
`workplace_accessible_fact_requires_from_any_place_scope`,
`actor_known_projection_policy_table_detects_synthetic_row_mutations`, and
`expected_embodied_presence` now exercise the workplace accessibility and latest
current-place axes. `ORD-HARD-148` is closed by the closed-world subset check in
`assert_context_excludes_unseeded_hidden_counterparts` and the hidden food/route
injection synthetics. `ORD-HARD-149` is closed by the provenance-tracked
`perception_visibility_prose_branch_violations` scan and its renamed-parameter
and payload-value relay kill set.

## 8. Oracle Closures

`ORD-HARD-157` is closed by the widened TUI render reachability guard and its
synthetic uncalled renderer. `ORD-HARD-158` is closed by the Mara tampered-log
negative `synthetic_mara_autonomous_food_consumed_fails_resolution`, which fails
the `fail_only_empty_food_source` consumed-food arm. `ORD-HARD-159` is closed by
the registry-derived ordinary-action positive census in
`positive_proof_fixtures_emit_typed_artifacts_first` and its sleep-positive
removal synthetic. `ORD-HARD-160` is closed by widening the generative support
`EventEnvelope` ban to all `tests/support/*` sources and the `support/mod.rs`
alias synthetic.

## 9. 0005 Coherence Decisions

`ORD-HARD-161` is resolved by keeping the current foundation-defensible candidate
priority order: severe needs first, urgent need and routine duty before active
intention continuation, and active continuation before mild pressure.
`goal_priority_selection_rank_snapshot_pins_decided_order` and
`urgent_need_vs_active_intention_follows_documented_order` pin the decision.
`ORD-HARD-162` is closed by disjoint default day windows
`(0..=3),(4..=9),(10..=17),(18..=23),(24..=32)` and
`default_day_windows_are_disjoint_and_cover`. `ORD-HARD-163` is closed by
requiring `EventKind::EatFailed` in `assert_required_acceptance_events` and by
the capstone's visible empty Mara food source.

## 10. Premise-Held Confirmations

All twenty-six findings, `ORD-HARD-140` through `ORD-HARD-165`, still held at
implementation time. Sixteen were operator-verified at audit and ten were
re-verified at source during spec reassessment at the same baseline. No finding
premise was refuted and silently dropped.

## 11. Documentation Diffs

The conformance index now records rows for the fixture schema-version gate
(`ORD-HARD-140`), meta-witness completion (`ORD-HARD-141`), embodied truth-access
removal (`ORD-HARD-143`), and derived apply perimeter (`ORD-HARD-144`). The
agent-cognition conformance row also records the `ORD-HARD-161` decision to keep
the current priority order.

Quoted destination: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## 12. EMERGE-OBS Derivation

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

This is measurement only, with no certification thresholds.

## 13. Mutation Run Status

The scheduled mutation run under the post-0022 posture is still pending in this
local evidence surface. This report records that status honestly rather than
converting a pending run into a pass.

## 14. Explicit Non-Certification Statement

This artifact is scoped evidence toward `ORD-LIFE-CERT`. It is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Verification Commands

Unmasked commands run for this capstone:

```sh
cargo test -p tracewake-core --test emergence_ledger -- --nocapture
cargo test -p tracewake-core --test anti_regression_guards --test emergence_ledger
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

All listed non-mutation gates passed on 2026-06-12.
