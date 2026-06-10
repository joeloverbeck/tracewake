# 0017 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-10

Target implementation commit under review:
`3f8a82a94d38ed9bb980dab9451f43677a950754`

Scope:
`archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A tick
ledger, epistemic staleness, replay payload, and generative lock hardening
work. It is not latest-main verification beyond the target implementation
commit, not full-project certification, not Phase 4 entry, and not
`FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0017PHA3ATICLED-001` | `60fc7a2` | Shared open-duration authority |
| `0017PHA3ATICLED-002` | `11f1b66` | Tick ledger and action-emitted awake delta accounting |
| `0017PHA3ATICLED-003` | `897d518` | Projection freshness and remembered-belief provenance |
| `0017PHA3ATICLED-004` | `a558c09` | Provenance witness compatibility and fail-closed diagnostics |
| `0017PHA3ATICLED-005` | `35980dc` | Replay payload materialization and tamper-flip gates |
| `0017PHA3ATICLED-006` | `11049d1` | Believed-access embodied availability |
| `0017PHA3ATICLED-007` | `9208a6d` | Content numeric policies and union text scanning |
| `0017PHA3ATICLED-008` | `731effa` | Lock durability and mutation workflow |
| `0017PHA3ATICLED-009` | `999222f` | Bounded in-tree generative lock tier |
| `0017PHA3ATICLED-010` | `3f8a82a` | Audit-history corrections and conformance rows |

Capstone scope adds this report and refreshes
`.cargo/mutants-baseline-misses.txt` to the raw `mutants.out/missed.txt` format
consumed by the scheduled workflow. It introduces no production logic.

## Verification Commands

Observed results in this capstone run:

| Command | Result |
|---|---|
| `grep -n "2026 correction (spec 0017)" reports/0016_ord_life_cert_scoped_acceptance.md` | Passed; correction section present. |
| `grep -c "0017" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Passed; count `12`. |
| `git diff --check` | Passed before `-010` commit. |
| `cargo fmt --all --check` | Passed at post-`-010` tree. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed at post-`-010` tree. |
| `cargo build --workspace --all-targets --locked` | Passed at post-`-010` tree. |
| `cargo test --workspace` | Passed at post-`-010` tree. |
| `cargo mutants --version` | `cargo-mutants 27.1.0`. |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' --no-shuffle` | Completed in 67m; 937 tested, 593 caught, 148 missed, 195 unviable, 1 timeout. |
| `diff -u .cargo/mutants-baseline-misses.txt mutants.out/missed.txt` | Passed after refreshing the baseline file to the finished-tree raw missed set. |

## 1. Need Ledgers And Golden Diffs

The current proof surface is occurrence-counted, not regime-label-counted:

| Fixture or test | Evidence | Result |
|---|---|---|
| `no_human_day_001` | `no_human_need_ledger_has_no_duplicate_regime_charges` reconstructs every `NeedDeltaApplied` tick span into `(actor, need, tick)` charge occurrences. | Passed under `cargo test --workspace`. |
| `wait_then_window_passive_charges_each_tick_once_001` | `wait_then_window_passive_charges_each_tick_once` proves an autonomous wait tick is not recharged by the next passive no-human window. | Passed under `cargo test --workspace`. |
| Generated corpus | `assert_single_need_charge_per_actor_tick_kind` runs against every generated history. | Passed in `generated_sequences_replay_and_satisfy_metamorphic_locks`. |

Before `ORD-HARD-026`, the audited failure mode was an `ActorWaited` tick also
covered by the next passive window. After `-002`, the scheduler advances the
passive frontier from action-emitted tick deltas and the event-application path
tracks `AgentState::need_tick_charges`. Golden checksum changes from this batch
are explained by removing that duplicate charge and by adding checksum coverage
for the materialized agent ledgers.

## 2. Provenance Class And Witness Proofs

| Evidence | Concrete lock | Result |
|---|---|---|
| `aged_food_record_surfaces_as_remembered_belief_not_observation_001` | `aged_food_record_surfaces_as_remembered_belief` proves an aged food projection surfaces as `remembered_belief`, not a restamped `observed_now`; replay also sees the same class. | Passed under `cargo test --workspace`. |
| `ActorDecisionTransaction::run` unit test | `provenance_class_mismatch_fails_closed_before_proposal` returns a typed `BlockerCode::ProvenanceClassMismatch`. | Passed under `cargo test --workspace`. |
| `ActorDecisionTransaction::run` unit test | `provenance_witness_mismatch_fails_closed_before_proposal` returns a typed `BlockerCode::ProvenanceWitnessMismatch` for a real but non-witnessing source event. | Passed under `cargo test --workspace`. |
| Guard | `guard_018_actor_known_facts_require_source_event_witness` verifies `ActorKnownFact` stores `SourceEventIds`, not raw empty vectors. | Passed under `cargo test --workspace`. |

## 3. Replay Payload And Context-Hash Evidence

The replay-payload fix makes semantic agent payloads checksum-visible instead
of allowing them to remain decorative no-ops.

| Evidence | Lock | Result |
|---|---|---|
| `continue_routine_tamper_kind_flip_poisons_replay` | Flipping `ContinueRoutineAccepted` to `ContinueRoutineRejected` makes replay fail expected checksums. | Passed under `cargo test --workspace`. |
| `continue_routine_tamper_reason_rewrite_poisons_replay` | Rewriting a continue-routine reason makes replay fail expected checksums. | Passed under `cargo test --workspace`. |
| `no_human_capstone_proves_typed_ancestry_and_replay` | Capstone asserts replay equality and empty `decision_context_hash_failures`. | Passed under `cargo test --workspace`. |
| `agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state` | Remaining `AGENT_WORLD_NOOP_ALLOWLIST` entries are explicit marker/no-op cases, not materialized semantic state. | Passed under `cargo test --workspace`. |

## 4. Embodied Divergence Pair

| Fixture | Evidence | Result |
|---|---|---|
| `embodied_workplace_availability_reflects_belief_not_truth_001` | `embodied_workplace_availability_reflects_belief_not_truth` proves believed-closed/truth-open work availability renders unavailable in the embodied surface while debug can expose the divergence separately. | Passed under `cargo test --workspace`. |
| `embodied_workplace_believed_open_truth_closed_commit_fails_001` | `embodied_workplace_believed_open_truth_closed` proves believed-open/truth-closed work availability renders available, then commit fails through validation truth. | Passed under `cargo test --workspace`. |
| Guard | `guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability` bans literal-`true` Phase 3A affordance availability. | Passed under `cargo test --workspace`. |

## 5. Numeric Policy And Negative Rejections

Content validation is registry-backed and fail-closed:

| Evidence | Lock | Result |
|---|---|---|
| `schema_numeric_fields_are_classified_for_validation_policy` | Every numeric fixture field is classified by explicit policy. | Passed under `cargo test --workspace`. |
| `schema_string_fields_are_classified_for_script_scanning` | Every scanned string field is routed through the recorded scan policy. | Passed under `cargo test --workspace`. |
| `fixture_initial_need_out_of_band_rejected_001` | Out-of-band initial needs reject instead of clamping silently. | Passed under `cargo test --workspace`. |
| `fixture_workplace_zero_duration_rejected_001` | Zero-duration workplace content rejects. | Passed under `cargo test --workspace`. |
| `fixture_output_tag_script_marker_rejected_001` | Script marker text in `output_tag` rejects. | Passed under `cargo test --workspace`. |
| `fixture_relief_zero_and_need_gate_out_of_band_are_rejected_001` | Zero relief and out-of-band need gates reject. | Passed under `cargo test --workspace`. |

## 6. Generative Tier Summary

The first generative tier is dependency-free and in-tree:
`crates/tracewake-core/tests/support/generative.rs`.

| Item | Value |
|---|---|
| Seeds | `0x17_00_00_01`, `0x17_00_00_29`, `0x17_00_00_57`, `0x17_00_00_91` |
| Sequence count | 4 generated cases, 4 windows each, 16 scheduled wait windows total |
| Generator | Seeded LCG over bounded no-human wait windows |
| Primary test | `generated_sequences_replay_and_satisfy_metamorphic_locks` |

The generative test asserts live-vs-replay physical and agent checksums,
canonical event-log roundtrip, prefix replay, marker-append physical no-op
behavior, and single need charge per actor/tick/kind. Reachability counters
assert the corpus emitted waits, need deltas, no-human markers, replay
roundtrips, and prefix replay. The first corpus reported zero differential
divergences because every seed passed the replay and metamorphic assertions.

Kani adoption was skipped without prejudice for this pass; the bounded
metamorphic generator is the recorded lock tier.

## 7. Mutation Baseline

Pinned mutation command:

```sh
cargo mutants --workspace \
  -f 'crates/tracewake-core/src/agent/**' \
  -f 'crates/tracewake-core/src/scheduler*' \
  -f 'crates/tracewake-core/src/projections*' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  --no-shuffle
```

Result at the post-`-010` tree: 937 tested, 593 caught, 148 missed, 195
unviable, 1 timeout. The timeout is recorded in `mutants.out/timeout.txt`:
`crates/tracewake-core/src/agent/transaction.rs:196:53: replace == with != in
ActorDecisionTransaction::run`.

Disposition: `.cargo/mutants-baseline-misses.txt` was refreshed from
`mutants.out/missed.txt` because the scheduled workflow compares raw
`missed.txt` output. The committed file now has 148 raw entries, and
`diff -u .cargo/mutants-baseline-misses.txt mutants.out/missed.txt` passes.

## 8. Corrected 0016 Report

`reports/0016_ord_life_cert_scoped_acceptance.md` now contains
`## 2026 correction (spec 0017)`. The correction records:

- event-schema migration evidence is loud-rejection-only, backed by
  `unsupported_event_schema_version_replay_fails_loudly`;
- the 0016 embodied workplace divergence claim was overstated and is replaced
  by the real 0017 divergence pair;
- the named content negative fixtures were delivered as inline
  `validate.rs` tests and registry census tests.

`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` records the
corresponding 0017 conformance rows and audit-history overturn note.

## 9. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0017`. It is
not a claim that the full `ORD-LIFE-CERT` gate has passed, not a claim about
latest main outside the target implementation commit, not full-project
certification, not Phase 4 entry approval, and not `FIRST-PROOF-CERT`.
