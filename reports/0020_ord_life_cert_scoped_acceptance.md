# 0020 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-11

Target implementation commits under review:
`01cfcddbe9eb7f25e854247e54bf2e62dd0d3db0` through
`3de52d42f23a38307154e1a6689622ec52b275d0`

Scope:
`archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the Phase 3A cognition
surface freshness, derived-census closure, mutation-perimeter completion, generative
fidelity, and no-human acceptance hardening work. It is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

Ticket commits included in the target implementation:

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0020PHA3ACOGSUR-001` | `01cfcdd` | Supersession parity, remembered-food policy, risk-register entries |
| `0020PHA3ACOGSUR-002` | `3eba3cd` | Derived apply-arm census and typed-column-closure exemptions |
| `0020PHA3ACOGSUR-003` | `e8ad231` | Eat/sleep/work mutation perimeter and governed baseline ledger |
| `0020PHA3ACOGSUR-004` | `17c35a4` | Generative terminal tamper, continuity floor, and seed contributors |
| `0020PHA3ACOGSUR-005` | `7e7356d` | Transitive known-food helper census and explicit fixture consumers |
| `0020PHA3ACOGSUR-006` | `b484ed2` | Exit-blocker surfacing and dead embodied-field sweep |
| `0020PHA3ACOGSUR-007` | `3de52d4` | Duration-completion window progress and routine-start eligibility |

Capstone scope adds this report and the read-only EMERGE-OBS ledger derivation.
It introduces no production logic, event schema, or simulation input.

## Verification Commands

Observed results while preparing the 0020 series:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core --test hidden_truth_gates` | Passed during `-001`; covered supersession parity and remembered-food actor-known policy. |
| `cargo test -p tracewake-core --test anti_regression_guards materialized_agent_apply_arms_require_payload_schema_version -- --nocapture` | Passed during `-002`; derived apply-arm census ran. |
| `cargo mutants -f eat -f sleep -f work --no-shuffle -j 2` | Completed during `-003`; 107 mutants, 76 caught, 24 unviable, 7 missed and ledgered. |
| `cargo test -p tracewake-core --test generative_lock -- --nocapture` | Passed during `-004`; terminal tamper, continuity floor, seed contributors, and replay checks ran. |
| `cargo test -p tracewake-content --test fixtures_load known_food_source_blanket_helper_call_sites_are_allowlisted -- --nocapture` | Passed during `-005`; transitive helper census ran. |
| `cargo test -p tracewake-core projections -- --nocapture` | Passed during `-006`; exit blocker is derived for perceived closed/locked doors and omitted for unperceived blockers. |
| `cargo test -p tracewake-tui render -- --nocapture` | Passed during `-006`; TUI prints the visible-exit blocker summary. |
| `cargo test -p tracewake-core --test anti_regression_guards embodied_view_option_and_collection_fields_have_reachable_producers -- --nocapture` | Passed during `-006`; dead-field sweep ran. |
| `cargo test -p tracewake-core scheduler -- --nocapture` | Passed during `-007`; completion-only-window and routine eligibility tests ran. |
| `cargo test -p tracewake-tui --test command_loop_session no_human_day_command_loop_renders_phase3a_behavior_rows -- --nocapture` | Passed during `-007`; TUI behavior row expectations matched corrected eligibility. |
| `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` | Passed for this capstone; emitted the first EMERGE-OBS baseline and proved canonical-log byte identity. |
| `cargo fmt --all --check` | Passed after each completed ticket. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed after each completed ticket. |
| `cargo build --workspace --all-targets --locked` | Passed after each completed ticket. |
| `cargo test --workspace` | Passed after each completed ticket. |

## 1. Supersession-Parity Proof

Primary ticket: `0020PHA3ACOGSUR-001`.

Evidence:

| Finding | Current proof |
|---|---|
| `ORD-HARD-053` no-human cognition surface supersession | Builder-level contradictory workplace notices now use the shared actor-known projection policy; the newest record wins before planning receives facts. |
| `ORD-HARD-058` equal-tick parity | Embodied and no-human surfaces use the same deterministic tie-break rule, and the two-surface equality assertion passed in the focused core tests. |

The correction did not expose raw workplace truth. It changed how actor-known records
are classified and superseded before consumers read them.

## 2. Remembered Food And Per-Kind Policy

Primary ticket: `0020PHA3ACOGSUR-001`.

Evidence:

| Surface | Policy |
|---|---|
| Food-source knowledge | Remembered actor-known food remains eligible when the actor is away from the source; current observation is preferred when available. |
| Sleep/workplace knowledge | Existing stale/current classifications remain per-kind and provenance-backed rather than deleted from the cognition surface. |

The focused hidden-truth and projection tests passed. No phase deferral cite was
used; the asymmetry was corrected rather than documented as deferred.

## 3. Derived Apply-Arm Census

Primary ticket: `0020PHA3ACOGSUR-002`.

`materialized_agent_apply_arms_require_payload_schema_version` now derives covered
`AgentState` map writes from checksum coverage and `events/apply.rs`. Each derived
apply arm must either check a supported schema/version materialization path or be
listed in `TYPED_COLUMN_CLOSURE_EXEMPTIONS` with rationale.

Observed disposition:

| Disposition | Evidence |
|---|---|
| Version-gated materialized records | Existing payload/trace/diagnostic records remain checked through replay/materialization gates. |
| Typed-column-closure exemptions | Need, intention, and routine lifecycle arms are exempted only where closed typed columns, not free payload fields, define the materialized state. |
| Repricing ledger | No golden payload repricing diff was required by this ticket. |

## 4. Mutation Run, Baseline Ledger, And Canaries

Primary ticket: `0020PHA3ACOGSUR-003`.

Focused mutation result:

```text
107 mutants tested in 5m: 76 caught, 24 unviable, 7 missed
```

The seven new `eat.rs` misses were added to `.cargo/mutants-baseline-misses.txt` and
reviewed in `reports/0020_mutants_baseline_disposition.md`. The normalized baseline
pin is 143 entries with hash `bd1855a5ee82b428`.

`mutation_perimeter_matches_duration_action_rationale_and_ci_filters` passed with the
expanded CI filter set and regex canary. `mutation_baseline_misses_are_pinned_and_ledgered`
passed with the reviewed baseline and synthetic unledgered-append failure.

## 5. Generative Tier Deltas

Primary ticket: `0020PHA3ACOGSUR-004`.

Evidence:

| Delta | Proof |
|---|---|
| Terminal-targeted tamper | `generated_sequences_replay_and_satisfy_metamorphic_locks` now tampers duration terminal payloads directly when a generated run emits one. |
| Continuity-reason floor | The generated corpus exercises continuity failures such as actor displacement and access-closure classes. |
| Predicate-derived seed contributors | Seed-contributor maps are derived from emitted event predicates instead of hardcoded pair assumptions. |
| Severe-band guard | `SLEEP_INTERRUPT_HUNGER_VALUE=930` and `WORK_FAILURE_HUNGER_VALUE=820` are asserted to remain `NeedBand::Severe`. |

The seed set includes `0x18_00_00_2A`, which drives the work-displacement case
through the shared movement path.

## 6. Transitive Helper Census

Primary ticket: `0020PHA3ACOGSUR-005`.

The blanket helper call was removed from
`hidden_truth_adversarial_fixture` and replaced with the explicit
`with_actor_mara_known_hidden_food` wrapper. The only explicit consumers are:

| Fixture wrapper |
|---|
| `hidden_food_closed_container_001` |
| `hidden_food_unknown_route_001` |
| `hidden_route_edge_001` |
| `debug_omniscience_excluded_001` |
| `workplace_assignment_provenance_001` |

`known_food_source_blanket_helper_call_sites_are_allowlisted` now scans fixture
source files for indirect wrapper consumers and includes a synthetic indirect-consumer
regression.

## 7. Exit-Blocker Surfacing And Dead-Field Sweep

Primary ticket: `0020PHA3ACOGSUR-006`.

The deferral re-check of `docs/2-execution/03*` and `docs/2-execution/12*` found no
exit-blocker deferral. `build_embodied_view_model` now derives
`VisibleExit.blocker_summary` for actor-known exits when a visible connected door is
closed or locked. Core tests prove the blocker appears for perceived doors and is
omitted for unperceived blockers; the TUI render test proves the summary is consumed.

`embodied_view_option_and_collection_fields_have_reachable_producers` derives
`Option` and collection fields from embodied view/status structs, scans production
producers, and requires cited external producers or deferrals for fields not populated
in core.

## 8. Completion-Only Window And Routine Eligibility

Primary ticket: `0020PHA3ACOGSUR-007`.

`append_due_completions` now reports appended duration terminal completions so
`run_no_human_day` counts a window whose only legitimate progress is completion as
progress. `no_human_day_counts_duration_completion_as_window_progress` passed.

`routine_window_family` now requires `execution.start_tick <= window.start_tick`.
The grep proof found the corrected `window.start_tick` bound and no remaining
`window.end_tick` eligibility bound. The TUI no-human behavior expectations were
updated to match the corrected routine eligibility.

## 9. Risk Register And Conformance Index

Conformance rows added under
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`:

| Row | Locked claim |
|---|---|
| `0020 derived apply-arm payload census` | Apply arms writing checksum-covered `AgentState` maps are derived and must be gated or explicitly exempted. |
| `0020 mutation-perimeter completion` | The mutation perimeter includes guarded eat/sleep/work action definitions across scheduled and in-diff CI jobs. |
| `0020 dead embodied-field sweep` | Embodied `Option`/collection fields need reachable non-default producers or cited external/deferral entries. |
| `0020 mutation-baseline governance` | Accepted mutation misses are pinned by normalized count/hash and a reviewed ledger. |

Risk-register entries:

| Risk | Current use |
|---|---|
| `R-27 -- Acceptance-Evidence Reachability Overstatement` | Requires emitted-path evidence for reachability claims and guards against harness-fabricated evidence. |
| `R-28 -- Incomplete Correction Closure` | Requires family enumeration, explicit exemptions, and derived guards when a fix could leave sibling surfaces uncorrected. |

## 10. EMERGE-OBS Baseline

Primary command:
`cargo test -p tracewake-core --test emergence_ledger -- --nocapture`.

The initial EMERGE-OBS ledger is an observer-only test derivation over the 12
`GENERATIVE_SEEDS` no-human runs. It derives counters from `EventKind` and typed
payload fields, appends no events, mutates no state, and is not consumed by scheduler,
simulation, content selection, or TUI code. The test proves per-row re-derivation
identity and canonical event-log serialization/deserialization byte identity.

This is measurement only: no thresholds, ratchets, or pass/fail semantics attach to
the counter values.

| Corpus row | Ticks | Contradictions | Replans/fallbacks | Interruptions | Intention switches | Stuck blockers | Belief divergences | Modeled corrections | Distinct kinds |
|---|---:|---:|---:|---:|---:|---|---:|---:|---:|
| `generative_seed_18000010` | 1-13 | 0 | 0 | 0 | 0 | none | 0 | 0 | 5 |
| `generative_seed_18000011` | 0-21 | 0 | 0 | 1 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000012` | 0-10 | 0 | 0 | 0 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000013` | 0-10 | 0 | 0 | 1 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000014` | 2-20 | 0 | 0 | 0 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000015` | 2-21 | 0 | 0 | 0 | 0 | none | 0 | 0 | 8 |
| `generative_seed_18000021` | 1-22 | 0 | 0 | 1 | 0 | none | 0 | 0 | 8 |
| `generative_seed_18000023` | 0-10 | 0 | 0 | 1 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000024` | 0-16 | 0 | 0 | 0 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000029` | 1-24 | 0 | 0 | 0 | 0 | none | 0 | 0 | 8 |
| `generative_seed_1800002a` | 1-9 | 0 | 0 | 1 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000057` | 0-13 | 0 | 0 | 0 | 0 | none | 0 | 0 | 8 |
| `total` | n/a | 0 | 0 | 5 | 0 | none | 0 | 0 | 14 |

The content canonical no-human fixture suite remains covered separately by
`cargo test -p tracewake-content --test golden_fixtures_run` and the full workspace
test gate; those fixtures are not imported into this core-only ledger module.

## 11. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0020`. It records
current-tree evidence for the implementation commits listed above. It is not
full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`, and not a claim
that later, unrelated changes are covered by this report.

## Deviations From Plan

The EMERGE-OBS ledger baseline is implemented as a core-only test over the
`GENERATIVE_SEEDS` no-human corpus. Existing content fixture coverage is cited
separately because `tracewake-core` cannot depend on `tracewake-content`.

## Finished-Tree Gate Record

The finished tree for the implementation commits and capstone artifact is checked
with:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```
