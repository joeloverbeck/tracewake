# 0018 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-11

Target implementation commit under review:
`9e6904f1f0a3e075758e8c1177c7f2d1d597194b`

Scope:
`archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A
provenance witness, episode replay evidence, generative reachability, seed
epistemics, and lock-durability hardening work. It is not latest-main
verification beyond the target implementation commit, not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0018PHA3APROWIT-001` | `644d645` | Provenance witness fail-closed table and census |
| `0018PHA3APROWIT-002` | `4070964` | Duration `action_effect` charge coverage |
| `0018PHA3APROWIT-003` | `157de50` | Episode payload materialization and checksum evidence |
| `0018PHA3APROWIT-004` | `b232a4c` | Payload schema gates |
| `0018PHA3APROWIT-005` | `37adbc2` | Authored food-source knowledge edges |
| `0018PHA3APROWIT-006` | `603da88` | Fixture validation token and tokenless materialization rejection |
| `0018PHA3APROWIT-007` | `bfab87b` | Workplace witness freshness |
| `0018PHA3APROWIT-008` | `403c412` | Extended generative reachability |
| `0018PHA3APROWIT-009` | `4ccb5d5` | Content/fixture/dependency census durability and mutation ratchet |
| `0018PHA3APROWIT-010` | `9e6904f` | Conformance documentation and execution single-charge clause |

Capstone scope adds this report only. It introduces no production logic.

## Verification Commands

Observed results while preparing this capstone:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core provenance_witness` | Passed; 3 transaction-level provenance witness mismatch tests ran. |
| `cargo test -p tracewake-core witness_kind` | Passed; `guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms` ran. |
| `cargo test -p tracewake-content episode_tamper` | Passed; 2 episode tamper tests ran. |
| `cargo test -p tracewake-core materialized_agent_payload_records_keep_payload_fields` | Passed; materialized agent record payload-field census ran. |
| `cargo test -p tracewake-core agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state` | Passed; derived allowlist/materialized-state guard ran. |
| `cargo test -p tracewake-content --test golden_fixtures_run` | Passed; 38 golden fixture tests ran. |
| `cargo test -p tracewake-core --test generative_lock` | Passed; 1 generated corpus lock test ran. |
| `cargo test -p tracewake-content seeded_food_source_unknown` | Passed; seeded food-source unknown fixture proof ran. |
| `cargo test -p tracewake-content --doc` | Passed; 3 compile-fail doc tests ran. |
| `cargo test -p tracewake-content content_negative_registry_covers_validation_policy_variants_and_tests` | Passed; content-negative policy census ran. |
| `cargo test -p tracewake-content --test fixtures_load` | Passed; 16 fixture load/census tests ran. |
| `cargo test -p tracewake-core workspace_dependency_posture_matches_allowlist` | Passed; dependency-posture census ran. |

## 1. Witness Table And Presence-Fact Evidence

| Evidence | Lock | Result |
|---|---|---|
| `provenance_witness_mismatch_fails_closed_before_proposal` | A real source event that cannot witness the asserted actor-known fact is rejected before proposal construction. | Passed under `cargo test -p tracewake-core provenance_witness`. |
| `provenance_witness_notice_only_workplace_presence_fails_closed_before_proposal` | Role-assignment notice alone cannot witness physical workplace presence. | Passed under `cargo test -p tracewake-core provenance_witness`. |
| `provenance_witness_unlisted_stable_id_fails_closed_before_proposal` | Unlisted actor-known stable ids fail closed instead of inheriting a default allow. | Passed under `cargo test -p tracewake-core provenance_witness`. |
| `guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms` | Every no-human actor-known stable id has an explicit `witness_kind_allowed` arm, and `_ => true` is banned. | Passed under `cargo test -p tracewake-core witness_kind`. |

Corrected presence-fact behavior is now source-specific: current-place presence
uses a current-place observation witness, role-assignment notices witness role
or workplace knowledge only, and replay rebuild routes through the same source
event ids rather than accepting builder-local defaults.

## 2. Episode Payload Replay Evidence

| Evidence | Lock | Result |
|---|---|---|
| `episode_tamper_output_tag_poisons_replay` | Rewriting a `WorkBlockStarted.output_tag` payload poisons replay. | Passed under `cargo test -p tracewake-content episode_tamper`. |
| `episode_tamper_proration_poisons_replay` | Rewriting duration/proration payload evidence poisons replay. | Passed under `cargo test -p tracewake-content episode_tamper`. |
| `materialized_agent_payload_records_keep_payload_fields` | Every materialized agent record with semantic event payloads stores `payload_fields`. | Passed under targeted core test. |
| `agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state` | Agent-world no-op allowlist entries cannot cover materialized episode state. | Passed under targeted core test. |

The concrete storage path is `OrdinaryLifeEpisodeRecord::payload_fields`, and
canonical agent checksums include `join_pairs(&episode.payload_fields)`.

## 3. Need Ledgers And Golden Checksum Evidence

`cargo test -p tracewake-content --test golden_fixtures_run` passed all 38
tests on the finished tree. The relevant ledger/checksum evidence is structural
and current-tree based:

| Evidence | Lock | Result |
|---|---|---|
| `no_human_need_ledger_has_no_duplicate_regime_charges` | Reconstructs every `NeedDeltaApplied` charge into `(actor, need, tick)` coverage and rejects duplicate regime charges. | Passed in the golden suite. |
| `wait_then_window_passive_charges_each_tick_once` | Proves a modeled wait tick is not charged again by the following passive window. | Passed in the golden suite. |
| `sleep_spanning_window_boundary_charges_each_tick_once` | Proves duration sleep crossing a window boundary charges each tick once. | Passed in the golden suite. |
| Replay checksum assertions in `no_human_day_real_run_replays_metrics_and_trace_projection` | Rebuilds physical and agent checksums from the log and compares them to live checksums. | Passed in the golden suite. |

Every current golden checksum expectation is recomputed from the live run and
replay rebuild. No unexplained current-tree golden diff remains in this capstone.
The `elapsed_ticks` addition is covered by per-tick ledger expansion for both
`tick_delta` and `action_effect` need charges before the single-charge check
runs.

## 4. Generative Reachability Summary

Primary command: `cargo test -p tracewake-core --test generative_lock`.

| Item | Value |
|---|---|
| Seeds | `0x18_00_00_10`, `0x18_00_00_11`, `0x18_00_00_12`, `0x18_00_00_13`, `0x18_00_00_14`, `0x18_00_00_15`, `0x18_00_00_29`, `0x18_00_00_57` |
| Sequence count | 8 generated cases, one per seed |
| Action vocabulary | `wait`, `eat`, `sleep`, `work_block` |
| Feature masks | `wait_eat`, `wait_work`, `wait_sleep`, `wait_sleep_interrupt`, `eat_work`, `all` |
| Primary test | `generated_sequences_replay_and_satisfy_metamorphic_locks` |

The test asserts nonzero reachability for actor waits, need deltas, food
consumption, sleep blocks, work blocks, duration terminals, sleep interruption,
no-human markers, replay roundtrip, and prefix replay. It also asserts terminal
kind diversity of at least 2, mask diversity of at least 4, and sequence-length
diversity of at least 2.

Differential divergences: zero observed in the capstone run; every generated
case passed live-vs-replay checksum comparison, canonical event-log roundtrip,
prefix replay, and single need charge per actor/tick/kind.

Marker relations: appending the terminal no-human marker leaves both physical
and agent checksums unchanged for every generated case.

## 2026 correction (spec 0019)

Spec 0019 corrected this section's reachability claim. The 0018 generative test
reported duration-terminal and interruption reachability from the final generated log,
but those terminal events were appended after `advance_no_human` by the test-side
`append_generated_duration_terminals` helper. That helper called the sleep/work
completion builders directly, so the old counters proved builder output and replay
stability after a post-hoc append; they did not prove scheduler-path terminal
reachability through the advance entry under test.

The corrected 0019 lock removes that helper from `generative_lock.rs`, has
`advance_no_human` process due completions through the shared scheduler completion
path, and adds a source guard banning direct completion-builder calls from the
generative oracle. Interruption coverage in the 0018-era report should therefore be
read as overstated evidence, not as proof that the old generated advance path emitted
interruption terminals itself.

## 5. Seed-Knowledge Fixture Proof

Command: `cargo test -p tracewake-content seeded_food_source_unknown`.

The test `seeded_food_source_unknown_to_all_actors_omits_seed_belief_and_actor_known_food`
passed. It proves the authored fixture with no known food-source edge mints no
household-food-source starting belief, produces no actor-known food source, and
does not let the planner reach or consume the hidden food through physical truth.

## 6. Unvalidated Materialization Rejection

Command: `cargo test -p tracewake-content --doc`.

All three content doctests passed. The compile-fail proofs show
`FixtureValidationToken` cannot be constructed or minted outside
`tracewake-content`, and `FixtureSchema::to_agent_state` cannot be called without
the validation token returned by the fixture validation path.

## 7. Census Outputs And Mutation Ratchet

| Evidence | Lock | Result |
|---|---|---|
| `content_negative_registry_covers_validation_policy_variants_and_tests` | Every `NumericFieldPolicy` and `StringScanPolicy` variant cites a proving rejection. | Passed. |
| `fixtures_load` integration suite | `positive_fixture_constructor_ids_from_source()` matches `fixtures::all()` and scope coverage remains non-empty. | Passed; 16 tests ran. |
| `workspace_dependency_posture_matches_allowlist` | `tracewake-core` `[dependencies]` is empty and the workspace dependency set matches the committed allowlist. | Passed. |

Mutation configuration is ratcheted in `.github/workflows/ci.yml`:

- `cargo-mutants` is pinned with `cargo install cargo-mutants --version 27.1.0 --locked`.
- The scheduled `mutants-lock-layer` job runs the guarded-layer workspace
  command over `crates/tracewake-core/src/agent/**`,
  `crates/tracewake-core/src/scheduler*`,
  `crates/tracewake-core/src/projections*`, and
  `crates/tracewake-core/src/actions/pipeline.rs`.
- The baseline check normalizes file+mutation+function by stripping line/column
  drift with `sed -E 's/:[0-9]+:[0-9]+:/:/'`, sorts unique entries, and uses
  `comm -23` to fail only on new missed mutants outside the accepted baseline.
- New misses are written to `$GITHUB_STEP_SUMMARY`.

## 8. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0018`. It is
not a claim that the full `ORD-LIFE-CERT` gate has passed, not a claim about
latest main outside the target implementation commit, not full-project
certification, not Phase 4 entry approval, and not `FIRST-PROOF-CERT`.

## Finished-Tree Gate Record

The report file is documentation-only. The finished tree was checked with:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | Passed; no formatting diff. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed; finished with warnings denied. |
| `cargo build --workspace --all-targets --locked` | Passed; finished with locked dependencies. |
| `cargo test --workspace` | Passed; workspace unit, integration, and doc tests completed successfully. |
