# 0016 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-10

Target implementation commit under review:
`9a9d3d322029d3ef4fe84d9c5e8b46b3b14a6d00`

Scope: `specs/0016_PHASE_3A_NEED_ACCOUNTING_REPLAY_EVIDENCE_AUDIT_COVERAGE_AND_LOCK_DURABILITY_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A
ordinary-life hardening work. It is not latest-main verification beyond the
target implementation commit, not full-project certification, not Phase 4
entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0016PHA3ANEEACC-001` | `43174b0` | Duration terminal closure and work-failure reservation release |
| `0016PHA3ANEEACC-002` | `8b08baa` | Need-regime classifier and per-tick ledger gates |
| `0016PHA3ANEEACC-003` | `ea52ab5` | Context-hash re-derivation replay gate |
| `0016PHA3ANEEACC-004` | `0dd2c99` | Source-event witnesses on actor-known facts |
| `0016PHA3ANEEACC-005` | `de2d6d5` | Hidden-truth audit coverage gates |
| `0016PHA3ANEEACC-006` | `f630f80` | Severe safety flight before hunger |
| `0016PHA3ANEEACC-007` | `96e6c40` | Embodied belief-vs-truth gates and band-only needs |
| `0016PHA3ANEEACC-008` | `8558ead` | Embodied staleness and recorded cognition-substrate deferral |
| `0016PHA3ANEEACC-009` | `117470f` | Content tuning validation |
| `0016PHA3ANEEACC-010` | `6374146` | Stuck detection discipline and modeled wait reason |
| `0016PHA3ANEEACC-011` | `fd15a4b` | Replay robustness and materialized agent projection state |
| `0016PHA3ANEEACC-012` | `eea24db` | Workspace/fixture/clippy census and mutation baseline |
| `0016PHA3ANEEACC-013` | `9a9d3d3` | Conformance rows, overturn note, and exec-06 stuck clauses |

Capstone scope adds this report only. It introduces no production logic.

Recorded deferral: `tickets/0016PHA3ANEEACC-015.md` exists and records the
full no-human actor-known surface unification onto `EpistemicProjection`.

## Verification Commands

Observed results while preparing this capstone:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | Passed in ticket 012 final gate |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed in ticket 012 final gate |
| `cargo build --workspace --all-targets --locked` | Passed in ticket 012 final gate |
| `cargo test --workspace` | Passed after ticket 013 docs; rerun for this capstone |
| `grep -n "need-tick\|terminal-set\|re-derivation\|cognition-substrate\|workspace census" docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Passed; rows resolve to current symbols |
| `grep -n "no-progress\|repeated-idle" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Passed |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' --no-shuffle` | Completed; 919 tested, 581 caught, 145 missed, 192 unviable, 1 timeout |

## 1. Need Ledgers And Golden Diffs

The corrected need-accounting proof is structural, not a new golden checksum
claim:

| Fixture | Evidence | Result |
|---|---|---|
| `no_human_day_001` | `no_human_need_ledger_has_no_duplicate_regime_charges` reconstructs per-actor/per-tick regime attribution and asserts no tick is charged by multiple regimes. | Passed under `cargo test --workspace` |
| `sleep_spanning_window_boundary_charges_each_tick_once_001` | `sleep_spanning_window_boundary_charges_each_tick_once` proves sleep crossing a no-human window boundary charges each tick once. | Passed under `cargo test --workspace` |

The 0015 verified-holding claim that passive deltas did not double-count sleep
ticks was overturned by `ORD-HARD-014` and recorded in the architecture
conformance index by ticket 013. The resulting golden checksum changes are
explained by eliminating duplicate awake/asleep or awake/working charges and by
materializing the corrected need-delta ledger through replay.

## 2. Context-Hash Re-Derivation Gate

The context-hash acceptance lock is the from-log re-derivation gate, not trust
in serialized trace bytes:

- `no_human_decision_context_hash_gate_fails_when_source_evidence_tampered`
  runs `no_human_day_001`, verifies the genuine log has no
  `decision_context_hash_failures`, tampers a source event, and then asserts the
  replay failure contains `decision_context_hash_mismatch`.
- `rebuild_decision_context_hashes` recomputes actor-known inputs from the
  event log and compares the rebuilt hash to each recorded
  `DecisionTraceRecord.actor_known_context_hash`.
- Ticket 013 records the overturned 0015 context-hash rebuild claim as
  `ORD-HARD-016` in the conformance index.

## 3. Duration Closure And Reservation Release

Fixture: `work_block_failed_then_sleep_succeeds_001`.

Evidence surface:

- `work_block_failed_then_sleep_succeeds_fixture_closes_reservation` loads the
  fixture, runs the no-human path, and proves failed work closure does not leave
  a stale body-exclusive reservation that blocks later sleep.
- Duration terminal behavior is centralized through `is_duration_terminal`;
  scheduler and action-pipeline guards assert terminal classification rather
  than duplicated ad hoc event-kind lists.

Representative event chain: `WorkBlockStarted` -> `WorkBlockFailed` closes the
reservation; a later `SleepStarted` / terminal sleep event remains valid and
replayable.

## 4. Severe Safety Flight

Fixture: `severe_safety_with_known_exit_produces_move_001`.

Evidence surface:

- `severe_safety_with_known_exit_produces_move_and_replays` proves severe
  safety pressure selects a move with ancestry before hunger behavior.
- `agent::transaction::tests::severe_safety_with_known_exit_proposes_move_before_hunger`
  locks the transaction-level ordering.
- Replay checks assert no `decision_context_hash_failures` for the genuine log.

This is scoped evidence that severe safety is ordinary actor cognition with
actor-known route evidence, not a scheduler-authored teleport or direct
primitive dispatch.

## 5. Embodied Belief-Vs-Truth And Need Rendering

Fixture: `embodied_workplace_availability_reflects_belief_not_truth_001`.

Evidence surface:

- `embodied_workplace_availability_reflects_belief_not_truth_fixture` proves
  embodied workplace availability follows holder-known belief/context rather
  than raw workplace truth.
- `guard_014_embodied_projection_workplaces_are_context_backed` locks
  `actor_known_workplaces_for_context(context: &KnowledgeContext)` and bans a
  raw-state workplace helper.
- TUI and view-model tests keep needs actor-legible and banded; exact numeric
  need values remain debug/test evidence, not embodied menu authority.

Debug comparison remains allowed, but debug truth is non-diegetic and cannot
feed embodied affordance lists.

## 6. Source-Guard And Census Inventory

Primary guard suite: `crates/tracewake-core/tests/anti_regression_guards.rs`.

| Lock | Evidence |
|---|---|
| Workspace source census | `workspace_source_classification_census_matches_production_tree` asserts every production source across core/content/tui is classified. |
| Guarded-layer exact set | `guarded_layer_source_census_matches_module_tree` and `guarded_layer_entries_are_exactly_the_workspace_guarded_classifications` lock the guarded perimeter. |
| Fixture directory census | `registered_negative_fixtures_match_directory_census` asserts `tests/negative-fixtures/` equals `FIXTURES`. |
| Clippy ban parity | `clippy_ban_entries_have_proving_negative_fixtures` asserts every disallowed type/method has a proving fixture. |
| Banned nondeterminism APIs | `clippy.toml` and fixtures cover `HashMap`, `HashSet`, wall clock, raw floats, `rand::random`, `rand::rng`, thread/process, filesystem read/write/open-options, env var, and network entry points. |

The mutation baseline is recorded in
`reports/0016_ord_hard_025_mutants_baseline.md`, including every missed mutant
and the timeout disposition.

## 7. Mutation Baseline

Mutation command:

```sh
cargo mutants --workspace \
  -f 'crates/tracewake-core/src/agent/**' \
  -f 'crates/tracewake-core/src/scheduler*' \
  -f 'crates/tracewake-core/src/projections*' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  --no-shuffle
```

Result: 919 mutants tested in 63m; 581 caught, 145 missed, 192 unviable, 1
timeout.

Disposition: the 145 misses and 1 timeout are accepted as the 0016 measured
baseline, not as evidence that the lock layer is complete. The per-miss list
and timeout are recorded in `reports/0016_ord_hard_025_mutants_baseline.md` for
follow-up prioritization.

## 8. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0016`. It is
not a claim that the full `ORD-LIFE-CERT` gate has passed, not a claim about
latest main outside the target implementation commit, not a full-project
certification, not Phase 4 entry approval, and not `FIRST-PROOF-CERT`.
