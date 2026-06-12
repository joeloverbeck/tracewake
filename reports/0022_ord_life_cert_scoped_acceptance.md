# 0022 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-12

Target implementation commits under review:
`7bc71b5` through `efc1851`

Scope:
`specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the 0022 Phase 3A
baseline triage, mutation/CI parity, hidden-truth gate restoration, policy-lock,
debug-render split, runner-only no-human proof, content diagnostic hygiene, and
meta-lock hardening work. It is not full-project certification, not Phase 4 entry,
and not `FIRST-PROOF-CERT`.

## Manifest Evidence

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0022PHA3ABASTRI-001` | `7bc71b5` | Real mutation-baseline triage, closed disposition tags, and filed test-debt tickets |
| `0022PHA3ABASTRI-002` | `9b7a03c` | Acceptance checklist parity guard for report anchors |
| `0022PHA3ABASTRI-003` | `a2b0311` | Mutation CI parity, event-name concurrency isolation, and fail-closed config/glob/capture guards |
| `0022PHA3ABASTRI-004` | `f491d43` | Meta-lock registry, nonzero-witness rule, and two-sided generative floor ratchets |
| `0022PHA3ABASTRI-005` | `d6d373a` | Hidden-truth adversarial gate restoration and discrimination witnesses |
| `0022PHA3ABASTRI-006` | `94aeea9` | Behavioral projection-policy table lock |
| `0022PHA3ABASTRI-007` | `e5d2f9f` | Eat need-crossing shared-emitter proof |
| `0022PHA3ABASTRI-008` | `30229db` | Scheduler typed-error conversions and log-derived panic guards |
| `0022PHA3ABASTRI-009` | `6e9c5dc` | Embodied debug-render split and derived `debug_available` proof |
| `0022PHA3ABASTRI-010` | `b195dd7` | Census call-shape widening, perception scan, derived cause-set, and agent-stream totality guards |
| `0022PHA3ABASTRI-011` | `354a349` | Runner-only canonical-day no-human proof and Q3 recovery-variant resolution |
| `0022PHA3ABASTRI-012` | `79cffd5` | Closed routine diagnostic vocabulary and engine-sourced generative bounds |
| `0022PHA3ABASTRI-013` | `efc1851` | Conformance-index rows for landed 0022 enforcement |

The capstone adds this report and the checklist registration below. It introduces
no production logic, event schema, fixture behavior, or simulation input.

## Verification Commands

Observed on the finished tree:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` | Passed; emitted the refreshed `emerge_obs_v1` table below. |
| `cargo test -p tracewake-core --test anti_regression_guards` | Passed after registering the 0022 checklist anchors; missing-anchor synthetic is in `acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors`. |
| `cargo test -p tracewake-core --test generative_lock` | Passed in `0022PHA3ABASTRI-012`; rerun by `cargo test --workspace`. |
| `cargo test -p tracewake-content` | Passed in `0022PHA3ABASTRI-012`; rerun by `cargo test --workspace`. |
| `cargo fmt --all --check` | Passed for every implementation ticket through this artifact. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed for every implementation ticket through this artifact. |
| `cargo build --workspace --all-targets --locked` | Passed for every implementation ticket through this artifact. |
| `cargo test --workspace` | Passed for every implementation ticket through this artifact. |

## Spec Section 7 Evidence Map

| Section 7 item | Report anchor | Finding anchors |
|---:|---|---|
| 1 | Real Baseline Triage | `ORD-HARD-099` |
| 2 | Concurrency And Fail-Closed Mutation Config | `ORD-HARD-100`, `ORD-HARD-101`, `ORD-HARD-102` |
| 3 | Embodied Render Split | `ORD-HARD-103`, `ORD-HARD-104` |
| 4 | Planner-Gate Adversarial Restoration | `ORD-HARD-105` |
| 5 | Policy Behavioral Table | `ORD-HARD-106`, `ORD-HARD-115` |
| 6 | Eat Crossing Proof | `ORD-HARD-107` |
| 7 | Scheduler Typed Errors | `ORD-HARD-108` |
| 8 | Census And Totality Guard Widening | `ORD-HARD-109`, `ORD-HARD-110`, `ORD-HARD-112`, `ORD-HARD-113` |
| 9 | Runner-Only Canonical-Day Proof | `ORD-HARD-111`, `ORD-HARD-120` |
| 10 | Content And Generative Hygiene | `ORD-HARD-116`, `ORD-HARD-117`, `ORD-HARD-118` |
| 11 | Meta-Lock Tier | section 5.1 |
| 12 | 0021 Report Corrections And Checklist Parity | `ORD-HARD-102`, `ORD-HARD-119` |
| 13 | Risk Register And Conformance Index Diffs | `R-27`, `R-28`, `R-29` |
| 14 | EMERGE-OBS Refresh | `emerge_obs_v1` |
| 15 | Explicit Non-Certification Statement | scoped only |

## 1. Real Baseline Triage

Primary ticket: `archive/tickets/0022PHA3ABASTRI-001.md`.

The real baseline triage is the work product in
`reports/0020_mutants_baseline_disposition.md`, not a governance promise. All 143
remaining normalized baseline entries are ledgered with closed disposition tags.
No entry was retired by focused tests in this slice; instead, test debt was filed
to the follow-up series `0022PHA3ABASTRI-015` through `0022PHA3ABASTRI-023`.

`mutation_baseline_misses_are_pinned_and_ledgered` now rejects unledgered misses,
stale ledger entries, unsupported tags, missing `warrants-test` tickets,
deferred-test language in `justified-baseline`, repeated rationale suffixes, and
unrecorded baseline floor changes. This records the 0021 deviation from
`ORD-HARD-099` explicitly: the 0021 report called governance "Baseline Triage",
but the per-entry triage work landed here.

## 2. Concurrency And Fail-Closed Mutation Config

Primary tickets: `archive/tickets/0022PHA3ABASTRI-002.md` and
`archive/tickets/0022PHA3ABASTRI-003.md`.

CI concurrency now includes `github.event_name`, and scheduled/manual mutation
runs are excluded from cancellation by ordinary push or pull-request jobs. The
mutation parity guard also fails closed for unsupported `mutants.toml` keys,
perimeter-covering `exclude_globs`, commented-out scheduled filters, swallowed
tool failures, missing status capture, and narrowed in-diff regexes.

First scheduled-run result under the new posture: pending. This artifact records
that status honestly; the proof available at this commit is the checked workflow
shape and the firing synthetics in
`mutation_perimeter_matches_duration_action_rationale_and_ci_filters`.

## 3. Embodied Render Split

Primary ticket: `archive/tickets/0022PHA3ABASTRI-009.md`.

`phase3a_debug_surfaces_render_deterministically_and_read_only` renders debug
needs, routines, planner, stuck, no-human-day, and actor panels without mutating
state or feeding embodied affordances. `adversarial_gates_rendering_does_not_change_typed_order_or_replay`
keeps debug replay rendering read-only and deterministic. The debug-panel source
scan continues to reject event-application calls from debug rendering.

The old `debug_available` constant/default shape is replaced by a derived
debug-availability proof surface; no actor-facing command parses debug text as
gameplay authority.

## 4. Planner-Gate Adversarial Restoration

Primary ticket: `archive/tickets/0022PHA3ABASTRI-005.md`.

Hidden food and hidden route truth exist in `PhysicalState`, while actor partial
knowledge is seeded through modeled event records. The restored adversarial gates
prove both sides: a known visible edge remains planner-reachable, while hidden
truth absent from actor-known context stays unavailable.

Evidence names:
`planner_hidden_truth_fixture_witness_errors`,
`hidden_food_closed_container_is_not_actor_known_food_source`,
`hidden_route_edge_absent_from_actor_context_blocks_route_plan`, and
`planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context`. The
empty-context discrimination witness fires if the adversarial fixture loses the
hidden-truth contrast it claims to prove.

## 5. Policy Behavioral Table

Primary ticket: `archive/tickets/0022PHA3ABASTRI-006.md`.

The policy lock now checks behavior, not only source presence.
`actor_known_projection_policy_table_drives_record_behavior` exercises policy
dispatch across record kinds and surfaces. The workplace other-place negative
proves embodied current-place scope does not silently inherit other-place
workplace knowledge. The supersede-subject pairing is locked by the policy
behavior and meta-lock registration through `synthetic_policy_table_behavior_drift`.

## 6. Eat Crossing Proof

Primary ticket: `archive/tickets/0022PHA3ABASTRI-007.md`.

Eat need deltas route through `build_need_delta_and_threshold_events`, matching
the shared threshold-crossing behavior used by sleep/work duration effects.
Tests prove both directions where applicable: eating can emit a hunger-band
crossing when the band changes and omits the crossing when the band does not.

Golden repricing diff: none required. The change is routing and lock coverage;
existing fixture expectations remained valid.

## 7. Scheduler Typed Errors

Primary ticket: `archive/tickets/0022PHA3ABASTRI-008.md`.

Log-derived scheduler paths no longer rely on unchecked `.expect` panics for
malformed state. The guard
`scheduler_apply_and_completion_paths_do_not_panic_on_log_derived_data` covers
the implemented typed-error paths and allowlist rationales, including malformed
completion/start payload handling and duplicate duration terminal reporting.

## 8. Census And Totality Guard Widening

Primary ticket: `archive/tickets/0022PHA3ABASTRI-010.md`.

The census call-shape fix resolves oblique helper calls instead of relying on
literal call text. Perception scans cover the full module instead of one narrow
surface. Action-emitted event kinds carry explicit cause-disposition metadata,
and agent-stream event kinds have total apply-arm coverage.

Evidence names: `typed_column_closure_exemptions_are_rationale_bearing_and_live`,
`guard_014_perception_visibility_uses_typed_place_visibility`,
`action_emitted_event_kinds_have_cause_disposition`, and
`agent_stream_event_kinds_have_explicit_agent_apply_arms`. Each has a firing
synthetic in `anti_regression_guards.rs`.

## 9. Runner-Only Canonical-Day Proof

Primary ticket: `archive/tickets/0022PHA3ABASTRI-011.md`.

The canonical no-human-day proof now uses the runner output only; it does not
manually fabricate work completion ancestry after the run. The Q3 resolution is
recovery-variant behavior, not fail-only intent: the actor can recover later
through modeled routine progression, while the proof still requires runner-owned
ancestry for the observed completion/failure path.

Evidence names: `guard_011_no_human_day_runner_only_evidence` and the no-human
capstone runner assertions.

## 10. Content And Generative Hygiene

Primary ticket: `archive/tickets/0022PHA3ABASTRI-012.md`.

`RoutineStep::FailWithTypedDiagnostic` now carries a closed
`RoutineDiagnosticKind`; unknown diagnostic ids fail closed through
`RoutineStepParseError::InvalidDiagnosticKind`. The no-sleep routine diagnostic
uses the stable snake-case id. The unrelated action `ReasonCode::NoSleepAffordance`
identifier remains live; the removed shape is the routine diagnostic's
free-text/camel-case acceptance.

Generative ordering bounds now come from engine-emitted start and terminal events,
and support generators are guarded against direct `EventEnvelope` construction.
The `mutants.toml` local exclusion parity resolution is in the mutation config
guards from item 2.

## 11. Meta-Lock Tier

Primary ticket: `archive/tickets/0022PHA3ABASTRI-004.md`.

`META_LOCK_REGISTRY` enrolls structural locks with registered negatives, routing,
and witness counts. `meta_lock_registry_covers_structural_locks_and_negatives`
checks the bijection census, missing-negative synthetic, invalid routed-ticket
synthetic, nonzero-witness floor, and unregistered-guard synthetic.

The nonzero-witness adoption list includes shared-scan locks such as
`generative_lock_two_sided_floor_ratchets` and
`generative_support_constructs_zero_event_envelopes`. The two-sided ratchet proof
fails both on unrecorded increases and on unrecorded decreases.

## 12. 0021 Report Corrections And Checklist Parity

Primary tickets: `archive/tickets/0022PHA3ABASTRI-002.md` and this ticket.

The 0021 report deviation for scheduled mutation evidence and baseline triage is
recorded in the 0021 artifact and superseded here with the 0022 work product.
`ORD-HARD-119` is covered by the 0022 risk-register and conformance-index updates
plus this report's explicit item 1 and item 2 records.

The checklist parity guard now runs against this artifact itself through
`acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors`. Its
synthetic missing-anchor case appends item 99 and proves the real checker fails on
a missing report anchor.

## 13. Risk Register And Conformance Index Diffs

Primary tickets: `archive/tickets/0022PHA3ABASTRI-001.md` and
`archive/tickets/0022PHA3ABASTRI-013.md`.

Risk-register `R-29` now names self-echo locks, artifact-shaped synthetics,
meta-lock countermeasures, the bijection census, nonzero-witness rule, and
two-sided ratchet expectations. `R-28` records that "perform work once" findings
need the work product itself as evidence.

Conformance-index rows updated or added under
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`:

| Row | Locked claim |
|---|---|
| `0020/0022 mutation-perimeter completion` | Event-name concurrency isolation, step-scoped mutation status capture, fail-closed filters, and scheduled/manual cancellation exemption. |
| `0021/0022 actor-known projection policy dispatch` | Behavioral policy-table lock, not just source-presence assertion. |
| `0022 shared need-delta emitter perimeter` | Eat/sleep/work/scheduler paths route need deltas through the shared emitter. |
| `0022 embodied debug-render split` | Debug rendering is non-diegetic, deterministic, and read-only. |
| `0022 planner hidden-truth adversarial restoration` | Hidden-truth gates keep adversarial witness fixtures and event-log provenance. |
| `0022 meta-lock tier` | Structural lock tests require registered negatives, routing, and nonzero witnesses. |
| `0020/0022 mutation-baseline governance` | Per-entry triage is closed-tagged and guarded by count/hash/ledger parity. |

## 14. EMERGE-OBS Refresh

Primary command:
`cargo test -p tracewake-core --test emergence_ledger -- --nocapture`.

The EMERGE-OBS ledger remains observer-only. This is measurement only: no
thresholds, ratchets, or pass/fail semantics attach to these counter values.

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

## 15. Explicit Non-Certification Statement

This artifact certifies nothing beyond scoped evidence toward `ORD-LIFE-CERT` for
the 0022 hardening package. It is not full-project certification, not Phase 4
entry, and not `FIRST-PROOF-CERT`. It does not authorize institutions, records,
wrong suspicion, reports, or regional/LOD scope beyond the already accepted Phase
3A surfaces.

## Deviations And Honest Pending Items

- No scheduled GitHub Actions mutation run result was available at artifact time;
  item 2 records the local checked workflow posture and the scheduled-run result
  as pending.
- No mutation baseline entries were retired in tickets 001-014. Focused test-debt
  tickets 015-023 own those follow-up kills.
- `NoSleepAffordance` remains as an unrelated action rejection `ReasonCode`
  identifier; the closed-vocabulary change removed free-text/camel-case routine
  diagnostic acceptance.
