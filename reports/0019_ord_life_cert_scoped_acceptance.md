# 0019 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-11

Target implementation commit under review:
`b8eb874e46d7b51f7b6a73873c3932fc0f42ba5c`

Scope:
`archive/specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A
generative reachability honesty, mutation perimeter, workplace freshness, evidence
closure, and actor-surface hardening work. It is not full-project certification,
not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0019PHA3AGENREA-001` | `6391d94` | Payload-version closure and prose-free agent checksums |
| `0019PHA3AGENREA-002` | `fcd1e35` | Workplace freshness split between remembered notices and current perception |
| `0019PHA3AGENREA-003` | `936e93c` | Generative reachability correction and 0018 evidence correction |
| `0019PHA3AGENREA-004` | `ace92a8` | Generative tamper relation and terminal-family floors |
| `0019PHA3AGENREA-005` | `66667c8` | Mutation perimeter expansion and CI failure semantics |
| `0019PHA3AGENREA-006` | `9da1c09` | Bidirectional no-human witness census |
| `0019PHA3AGENREA-007` | `dcf3496` | Seed-knowledge helper allowlist and partial-knowledge fixture |
| `0019PHA3AGENREA-008` | `b8eb874` | Embodied salient-interruption surfacing |

Capstone scope adds this report only. It introduces no production logic.

## Verification Commands

Observed results while preparing this capstone:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core --test generative_lock -- --nocapture` | Passed; `generated_sequences_replay_and_satisfy_metamorphic_locks` ran. |
| `cargo test -p tracewake-core --test anti_regression_guards generative_lock_cannot_fabricate_duration_terminals -- --nocapture` | Passed; direct completion-builder fabricator guard ran. |
| `cargo mutants --list -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs'` | Listed 83 sleep/work mutants in the expanded perimeter. |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle --jobs 4` | Passed; 83 mutants tested in 3m: 66 caught, 17 unviable. |
| `cargo test -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters -- --nocapture` | Passed; config/CI failure semantics guard ran. |
| `cargo test -p tracewake-core --test hidden_truth_gates workplace_requires_assignment_or_observation_provenance -- --nocapture` | Passed; workplace provenance/freshness no-leak gate ran. |
| `cargo test -p tracewake-core --test event_schema_replay_gates forged_threshold_payload_schema_version_rejected_for_materialized_agent_replay_001 -- --nocapture` | Passed; forged `NeedThresholdCrossed` payload version rejection ran. |
| `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_payload_records_keep_payload_fields -- --nocapture` | Passed; payload-field materialization census ran. |
| `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture` | Passed; materialized apply-arm schema-version census ran. |
| `cargo test -p tracewake-core --test anti_regression_guards guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms -- --nocapture` | Passed; bidirectional witness census ran. |
| `cargo test -p tracewake-content --test golden_fixtures_run no_human_need_ledger_has_no_duplicate_regime_charges -- --nocapture` | Passed; per-actor need-ledger single-charge proof ran. |
| `cargo test -p tracewake-content --test fixtures_load known_food_source_blanket_helper_call_sites_are_allowlisted -- --nocapture` | Passed; blanket helper allowlist census ran. |
| `cargo test -p tracewake-content --test golden_fixtures_run partial_food_source_knowledge_seeds_only_authored_actor_edge -- --nocapture` | Passed; partial-knowledge fixture proof ran. |
| `cargo test -p tracewake-core view_models_embodied_phase3a_salient_interruption_is_viewer_scoped` | Passed; viewer-scoped interruption view-model proof ran. |
| `cargo test -p tracewake-tui renderer_prints_phase3a_salient_interruption` | Passed; embodied TUI render proof ran. |
| `cargo fmt --all --check` | Passed; no formatting diff. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed with warnings denied. |
| `cargo build --workspace --all-targets --locked` | Passed with locked dependencies. |
| `cargo test --workspace` | Passed; workspace unit, integration, and doc tests completed successfully. |

## 1. Corrected Generative Corpus Summary

Primary command:
`cargo test -p tracewake-core --test generative_lock -- --nocapture`.

| Item | Current evidence |
|---|---|
| Seed set | `0x18_00_00_10`, `0x18_00_00_11`, `0x18_00_00_12`, `0x18_00_00_13`, `0x18_00_00_14`, `0x18_00_00_15`, `0x18_00_00_21`, `0x18_00_00_23`, `0x18_00_00_24`, `0x18_00_00_29`, `0x18_00_00_57` from `crates/tracewake-core/tests/support/generative.rs`. |
| Corpus test | `generated_sequences_replay_and_satisfy_metamorphic_locks` passed. |
| Advance-emitted terminal families | The test asserts nonzero `sleep_completed`, `sleep_interrupted`, `work_block_completed`, and `work_block_failed` counts from the generated run log. |
| Tamper relation | `assert_payload_tamper_poisons_replay` runs per generated case and rewrites payload values before replay. |
| Differential divergence | Zero observed; every generated case passed live/replay checksum comparison, canonical log roundtrip, prefix replay, and single need-charge checks. |
| Fabricator exclusion | `generative_lock_cannot_fabricate_duration_terminals` passed, banning direct `build_sleep_completion_events`, `build_work_completion_events`, and the removed fabricator from the generative oracle. |

The corrected corpus summary is current-tree evidence only. It does not rely on the
overstated 0018 post-advance terminal helper.

## 2. Mutation Perimeter And CI Failure Semantics

Primary mutation command:
`cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle --jobs 4`.

Observed output:

```text
Found 83 mutants to test
ok       Unmutated baseline in 8s build + 24s test
INFO Auto-set test timeout to 122s
83 mutants tested in 3m: 66 caught, 17 unviable
```

The expanded perimeter is visible under `cargo mutants --list`, which listed 83
sleep/work mutants including duration builders, interruption predicates, elapsed
arithmetic, target validation, and failure-event constructors. No missed mutants were
reported in this focused current-tree run, so the reviewed baseline did not need a new
miss entry for this perimeter.

`mutation_perimeter_matches_duration_action_rationale_and_ci_filters` passed. That
guard checks `.cargo/mutants.toml`, the CI in-diff mutation filters, direct-push gap
closure, and the rule that a cargo-mutants tool failure is not treated as a pass.

## 3. Non-Coincident-Tick Workplace Freshness

Command:
`cargo test -p tracewake-core --test hidden_truth_gates workplace_requires_assignment_or_observation_provenance -- --nocapture`.

Observed output: one test ran and passed. The underlying Phase 3A freshness surface keeps
actor-known workplace access backed by assignment or observation provenance, and the
newer perception path can supersede place-gating without erasing remembered role
notice evidence. This is cross-checked by the fixture/source paths
`stale_workplace_notice_superseded_by_newer_001` and the perception test
`current_place_knowledge_context_keeps_remembered_workplace_notice_after_newer_perception`.

The report does not claim that raw workplace truth is exposed. The claim is only that
non-coincident notice/perception evidence remains actor-known through the modeled
provenance path.

## 4. NeedThresholdCrossed Forgery Rejection And Derived Censuses

Primary command:
`cargo test -p tracewake-core --test event_schema_replay_gates forged_threshold_payload_schema_version_rejected_for_materialized_agent_replay_001 -- --nocapture`.

Observed output: one test ran and passed. The test forges
`NeedThresholdCrossed.payload_schema_version = "2"` and requires replay/materialization
rejection instead of silent acceptance.

Derived census commands:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_payload_records_keep_payload_fields -- --nocapture` | Passed; verifies materialized agent records keep payload fields and checksum coverage includes those fields. |
| `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture` | Passed; verifies materialized agent apply arms require `payload_schema_version = "1"`. |
| `cargo test -p tracewake-core --test anti_regression_guards guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms -- --nocapture` | Passed; verifies no-human surface fact stable ids are censused in both directions and uncensused synthetic mints fail. |

## 5. Need Ledgers And Checksum Repricing

Command:
`cargo test -p tracewake-content --test golden_fixtures_run no_human_need_ledger_has_no_duplicate_regime_charges -- --nocapture`.

Observed output: one test ran and passed. The ledger proof reconstructs
`NeedDeltaApplied` coverage by actor, need kind, and tick, then rejects duplicate
regime charges. This is the current-tree proof for the per-actor need-ledger side of
the 0019 repricing.

Checksum-side evidence is supplied by the full workspace run and the materialized
payload census in item 4. The golden suite replayed current fixtures successfully, and
the checksum code covers materialized episode, threshold, candidate-evaluation, and
routine-arbitration `payload_fields`. No unexplained current-tree golden diff remains
in this capstone.

## 6. Partial-Knowledge Fixture And Helper Allowlist

Commands:

| Command | Result |
|---|---|
| `cargo test -p tracewake-content --test fixtures_load known_food_source_blanket_helper_call_sites_are_allowlisted -- --nocapture` | Passed; one helper-census test ran. |
| `cargo test -p tracewake-content --test golden_fixtures_run partial_food_source_knowledge_seeds_only_authored_actor_edge -- --nocapture` | Passed; one partial-knowledge fixture proof ran. |

The helper census pins the legacy
`populate_known_food_sources_for_all_actors` call sites and fails on a synthetic
uncensused new fixture path. The partial-knowledge fixture authors one actor's
`known_food_sources` edge and proves that actor can plan from it while the actor
without the edge cannot.

## 7. Embodied Interruption Surfacing

Commands:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core view_models_embodied_phase3a_salient_interruption_is_viewer_scoped` | Passed; one core projection test ran. |
| `cargo test -p tracewake-tui renderer_prints_phase3a_salient_interruption` | Passed; one TUI render test ran. |

The `0019PHA3AGENREA-008` implementation branch fired after re-checking
`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` and
`docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
for an active deferral cite. No such cite was found.

The core projection now derives `salient_interruption` from the viewer actor's own
materialized `ordinary_life_episodes`, limited to `sleep_interrupted` and
`work_block_failed`, choosing the latest record by `sim_tick` and event-id tie-break.
The TUI consumes the populated view-model field and prints `Interruption: ...`.

## 8. Conformance Row And 0018 Report Correction

Current conformance-index rows under
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` include:

| Row | Current correction |
|---|---|
| `0019-corrected generative reachability contract` | States that generated scheduled no-human sequences exercise wait/eat/sleep/work, duration terminals, interruptions, replay, prefix replay, payload, checksum, and single-charge locks only from events emitted by `advance_no_human`; records that 0018 overstated terminal/interruption reachability from a test-side helper. |
| `0019 mutation-perimeter honesty` | States that sleep/work duration-definition builders are inside the mutation perimeter, cargo-mutants tool failure fails the gate, and in-diff mutation checks run on guarded PR and direct-push changes. |
| `0019 seed-knowledge helper containment` | States that blanket food-source seed knowledge is legacy-only and pins partial food-source knowledge as expressible and tested. |

The 0018 report correction under `reports/0018_ord_life_cert_scoped_acceptance.md`
states that the old generative test reported duration-terminal and interruption
reachability from the final generated log, but those terminal events were appended
after `advance_no_human` by a test-side helper that called sleep/work completion
builders directly. It further states that the corrected 0019 lock removes that helper,
routes due completions through the shared scheduler completion path, and adds a source
guard banning direct completion-builder calls from the generative oracle.

## 9. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0019`. It records
current-tree evidence for the implementation commits listed above. It is not
full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`, and not a claim
that later, unrelated changes are covered by this report.

## Deviations From Plan

No implementation deviation was found while composing this report. Two attempted
multi-filter `cargo test` collection commands were rejected by Cargo because Cargo
accepts one test filter at a time; the affected checks were immediately re-run as
separate commands and those successful outputs are the ones recorded above.

## Finished-Tree Gate Record

The report file is documentation-only. The finished tree for the implementation
commits was checked with:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | Passed; no formatting diff. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed with warnings denied. |
| `cargo build --workspace --all-targets --locked` | Passed with locked dependencies. |
| `cargo test --workspace` | Passed; workspace unit, integration, and doc tests completed successfully. |
