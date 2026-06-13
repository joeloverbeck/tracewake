# 0025 Ordinary-Life Certification Scoped Acceptance

Scope: evidence toward `ORD-LIFE-CERT` for Phase 3A meta-witness execution
discipline, perception kill-set provenance, envelope read-path fail-closed
behavior, manifest fingerprint honesty, embodied carrier capture, TUI debug gate
depth, census closures, recovery staging, CI workflow hygiene, and mutation
phase-entry evidence rules. This is not full-project certification, not Phase 4
entry, and not `FIRST-PROOF-CERT`.

Implementation commits: 0025PHA3AMETWIT-001 through 0025PHA3AMETWIT-010, plus
this capstone artifact and parity guard.

## Spec Section 7 Evidence Map

| Item | Report anchor | Finding anchors |
|---|---|---|
| 1 | Executable Witness Discipline | `ORD-HARD-166`, `ORD-HARD-175` |
| 2 | Provenance-True Kill Set | `ORD-HARD-167` |
| 3 | Envelope Fail-Closed Decisions | `ORD-HARD-168`, `ORD-HARD-171`, `ORD-HARD-184` |
| 4 | Manifest Fingerprint Honesty | `ORD-HARD-169`, `ORD-HARD-170`, `ORD-HARD-183` |
| 5 | Embodied Carrier Census And Staleness | `ORD-HARD-172`, `ORD-HARD-173` |
| 6 | TUI Gate Depth And Mode Decision | `ORD-HARD-174`, `ORD-HARD-176`, `ORD-HARD-185`, `ORD-HARD-186` |
| 7 | Census And Oracle Closures | `ORD-HARD-177`, `ORD-HARD-178`, `ORD-HARD-179`, `ORD-HARD-180`, `ORD-HARD-181`, `ORD-HARD-182` |
| 8 | CI And Evidence Honesty Records | `ORD-HARD-187`, `ORD-HARD-188`, `ORD-HARD-189`, `ORD-HARD-190` |
| 9 | Premise-Held Confirmations | `ORD-HARD-166` through `ORD-HARD-190` |
| 10 | Documentation Diffs | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` |
| 11 | EMERGE-OBS Derivation | `emerge_obs_v1` |
| 12 | Mutation Run Status | scheduled mutation still pending; dated green scheduled mutation run required before readiness |
| 13 | Explicit Non-Certification Statement | not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT` |

## 1. Executable Witness Discipline

`ORD-HARD-166` and `ORD-HARD-175` are closed by the repaired meta-lock witness
shape. `META_LOCK_REGISTRY`, `meta_lock_live_witness_count`,
`behavior_assertion_inspected_site_count`, and `witness_shape_ban_errors` enforce
nonzero live witnesses and reject assertion-token-count or anchor-presence
fallbacks. `meta_lock_registry_covers_structural_locks_and_negatives` includes
the present-but-vacuous synthetics that drop measured witness counts below their
floor.

## 2. Provenance-True Kill Set

`ORD-HARD-167` is closed by provenance-keyed perception taint checks rather than
raw token scans. `perception_prose_scan_inspected_line_count` and the
`guard_014_perception_visibility_uses_typed_place_visibility` family inspect the
emission path, while the renamed-argument and payload-value synthetics prove the
kill set fires through argument provenance.

## 3. Envelope Fail-Closed Decisions

`ORD-HARD-168` is closed by `EventEnvelope::deserialize_canonical` rejecting
duplicate canonical fields with `EventEnvelopeParseError::DuplicateField`.
`ORD-HARD-171` is recorded as a direct-decode loud-rejection decision:
`envelope_direct_decode_rejects_unsupported_schema_version` and
`event_envelope_direct_decode_rejects_unsupported_schema_version` prove unknown
event schema versions do not decode silently. `ORD-HARD-184` is recorded as the
delete-hollow-field decision: `event_envelope_has_no_hollow_checksum_after_field`
locks the absence of a serialized `checksum_after` field on event envelopes.

## 4. Manifest Fingerprint Honesty

`ORD-HARD-169` and `ORD-HARD-170` are closed by fingerprinting raw source bytes
through `fingerprint_payload` and by frozen fixture fingerprints checked in
`fixture_fingerprints_match_frozen_goldens`. `fixture_fingerprint_reprices_secondary_file_bytes`
and `fixture_fingerprint_reprices_raw_primary_bytes_with_same_parsed_fixture`
prove secondary files and byte-only changes reprice. `ORD-HARD-183` is closed by
`fixture_serialization_golden_bytes_are_pinned_001`.

## 5. Embodied Carrier Census And Staleness

`ORD-HARD-172` is closed by the `PhysicalState` carrier census around embodied
projection construction. `source_shape_errors` fails when a new carrier appears
without enrollment, and its `semantic_actions` synthetic proves action surfaces
cannot regain direct truth reads. `ORD-HARD-173`
is closed by observation-time capture for place labels and carried-item
attributes: `embodied_projection_renders_observed_place_label_not_live_truth` and
`embodied_projection_renders_observed_carried_item_attributes_not_live_truth`
prove stale live truth cannot rewrite the embodied view.

## 6. TUI Gate Depth And Mode Decision

`ORD-HARD-176`, `ORD-HARD-174`, and `ORD-HARD-185` are closed by intrinsic
world-advancing debug gating, arm-complete debug dispatch, and TUI-local guard
membership. `run_no_human_day_refuses_intrinsically_without_debug_availability`,
`debug_dispatch_routes_every_arm_through_availability_gate`, and
`tui_local_guard_registry_covers_structural_guards` lock the path. `ORD-HARD-186`
is recorded as the `ControllerMode::Debug` decision: `debug_available_for` grants
debug only for debug bindings, with `controller_mode_debug_availability_decision_is_explicit`
pinning all three modes.

## 7. Census And Oracle Closures

`ORD-HARD-177` through `ORD-HARD-182` are closed by derived census/oracle guards:
effect-complete positive proof fixtures, actor-known projection truth-table
oracles, support-directory parity, broadened exemption detection, schema ID
recognizer parity, and `Location` descent. The locked surfaces include
`positive_proof_fixtures_emit_typed_artifacts_first`,
`actor_known_projection_policy_truth_table_detects_predicate_inversion`,
`schema_id_type_recognizer_matches_schema_imports`, and
`fixtures_load_location_embedded_marker_id_rejected_001`.

## 8. CI And Evidence Honesty Records

`ORD-HARD-187` is closed by recording the `fail_only_empty_food_source` recovery
decision and the staged success-recovery trigger in
`docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.
`ORD-HARD-188` and `ORD-HARD-189` are closed by CI cache-key hygiene, top-level
permissions, and `ci_workflow_guards_cover_workflow_integrity`, which checks
verbatim gates, masking constructs, action pinning, cache keys, and doc/workflow
job parity. `ORD-HARD-190` is closed at the rule layer by
`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`:
pending mutation status is not a pass, and readiness requires a dated green
scheduled mutation run.

## 9. Premise-Held Confirmations

All twenty-five findings, `ORD-HARD-166` through `ORD-HARD-190`, still held at
implementation time or were re-verified at the ticket seam named by the spec.
No finding premise was refuted and silently dropped.

## 10. Documentation Diffs

The conformance index now records 0025 rows for executable witness discipline,
provenance-true perception taint, envelope fail-closed decisions, manifest
fingerprint honesty, embodied carrier census, TUI gate depth and mode decision,
census/oracle closures, and CI evidence honesty.

Quoted destinations:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

## 11. EMERGE-OBS Derivation

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

## 12. Mutation Run Status

The scheduled mutation run under the post-0025 posture is still pending in this
local evidence surface. This report records that status honestly rather than
converting a pending run into a pass. Pending is not a pass, and any future
`ORD-LIFE-CERT` readiness claim that depends on the lock layer requires a dated
green scheduled mutation run.

## 13. Explicit Non-Certification Statement

This artifact is scoped evidence toward `ORD-LIFE-CERT`. It is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Verification Commands

Unmasked commands run for this capstone:

```sh
cargo test -p tracewake-core --test emergence_ledger -- --nocapture
cargo test -p tracewake-core --test anti_regression_guards
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

All listed non-mutation gates passed on 2026-06-13.
