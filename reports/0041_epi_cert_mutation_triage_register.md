# 0041 EPI-CERT mutation triage register

Generated: 2026-06-19

## Scope

This register reconciles the 30 historical EPI-CERT survivor identities from
`reports/0040_epi_cert_mutation_final_missed.txt` against the completed 0041
standing mutation run.

Source baseline: `7a17447d7c8bc5fc591d70d7fe783b8f5d0e68f5`

Final implementation commit under test:
`9df694aab4bdd1552662515d6270dbe6835bb09b`

## Toolchain

| Tool | Version |
|---|---|
| Cargo | `cargo 1.93.0 (083ac5135 2025-12-15)` |
| rustc | `rustc 1.93.0 (254b59607 2026-01-19)` |
| cargo-mutants | `cargo-mutants 27.1.0` |

## Preflight

All preflight commands completed successfully before the full mutation run:

| Command | Result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo build --workspace --all-targets --locked` | pass |
| `cargo test --workspace --locked` | pass |
| core EPI suites: `hidden_truth_gates`, `event_schema_replay_gates`, `acceptance_gates`, `anti_regression_guards`, `generative_lock`, `golden_scenarios`, `negative_fixture_runner`, `spine_conformance`, `no_human_capstone`, `emergence_ledger` | pass |
| content EPI suites: `fixtures_load`, `forbidden_content`, `golden_fixtures_run`, `schema_conformance` | pass |
| TUI EPI suites: `adversarial_gates`, `tui_seam_conformance`, `transcript_snapshot`, `tui_acceptance`, `embodied_flow`, `command_loop_session` | pass |

## Census

| Evidence | Value |
|---|---:|
| `reports/0041_epi_cert_mutation_list_files.txt` | 54 files |
| `reports/0041_epi_cert_mutation_list.txt` | 2774 mutants |
| Historical 0040 configured census | 2763 mutants |
| Census delta | +11 mutants |

The final census includes the standing `crates/tracewake-core/src/epistemics/**`
perimeter and the historical survivor-bearing files:

- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/proposition.rs`

The checked-in `.cargo/mutants.toml` was loaded as the denominator of record.
The certifying run did not use `--no-config`, `-f`, `-F`, `--in-diff`,
`--iterate`, or an exclusion option.

`.cargo/mutants-baseline-misses.txt` is empty (`0` bytes). No EPI survivor was
laundered into the baseline-miss file.

## Full run

Command:

```sh
cargo mutants --workspace --no-shuffle -o reports/0041_epi_cert_mutation_full.out
```

Structured output:
`reports/0041_epi_cert_mutation_full.out/mutants.out/`

| Field | Value |
|---|---:|
| Start time | `2026-06-19T21:14:12.33981041Z` |
| End time | `2026-06-19T23:39:18.066054579Z` |
| Total mutants | 2774 |
| Caught | 2182 |
| Unviable | 592 |
| Missed | 0 |
| Timeout | 0 |
| Tool failures | 0 |

Outcome files:

- `caught.txt`: 2182 lines
- `unviable.txt`: 592 lines
- `missed.txt`: 0 lines
- `timeout.txt`: 0 lines
- `mutants.json`: retained
- `outcomes.json`: retained
- `debug.log`: retained

## Historical survivor reconciliation

Tool outcome and certification disposition are separate axes. In this completed
run, every still-generated historical identity was caught. One historical
identity (`ContradictionKind::is_active`) is source-changed/removed: the final
code has no `is_active` accessor, one `ContradictionKind` variant, and the
0041 contradiction-state seam is covered by the expected-absence detection and
contradiction linkage witnesses from tickets 003 and 009.

| # | Historical identity | Final identity / mapping | Tool outcome | Certification disposition | EPI / layer | Evidence |
|---:|---|---|---|---|---|---|
| 1 | `belief.rs:150:9 stale_after_tick -> None` | `belief.rs:155:9 stale_after_tick -> None` | caught | killed | EPI-02/EPI-06 freshness policy | `caught.txt` line 1381 |
| 2 | `belief.rs:150:9 stale_after_tick -> Some(Default::default())` | `belief.rs:155:9 stale_after_tick -> Some(Default::default())` | caught | killed | EPI-02/EPI-06 freshness policy | `caught.txt` line 1382 |
| 3 | `belief.rs:154:9 observation_ids -> empty set` | `belief.rs:159:9 observation_ids -> empty set` | caught | killed | EPI-02/EPI-03/EPI-05 provenance linkage | `caught.txt` line 1383 |
| 4 | `belief.rs:158:9 contradiction_ids -> empty set` | `belief.rs:163:9 contradiction_ids -> empty set` | caught | killed | EPI-02/EPI-04/EPI-05 contradiction linkage | `caught.txt` line 1384 |
| 5 | `contradiction.rs:127:13 replace || with && in detect_expected_absences` | `contradiction.rs:123:13 replace || with && in detect_expected_absences` | caught | killed | EPI-04 expected-absence eligibility | `caught.txt` line 1396 |
| 6 | `contradiction.rs:28:9 ContradictionKind::is_active -> true` | source-changed/removed; final code has no `is_active` accessor and one active contradiction kind | not generated | source-changed closure | EPI-04 contradiction state | final source `contradiction.rs`; no missed final-run identity |
| 7 | `observation.rs:25:9 parts_per_thousand -> 0` | unchanged identity | caught | killed | EPI-03 observation confidence | `caught.txt` line 1515 |
| 8 | `observation.rs:25:9 parts_per_thousand -> 1` | unchanged identity | caught | killed | EPI-03 observation confidence | `caught.txt` line 1516 |
| 9 | `observation.rs:29:9 is_low -> true` | unchanged identity | caught | killed | EPI-03 observation confidence | `caught.txt` line 1517 |
| 10 | `proposition.rs:260:13 delete AtPlace contradiction arm` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1651 |
| 11 | `proposition.rs:266:26 replace == with !=` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1657 |
| 12 | `proposition.rs:266:45 replace && with ||` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1656 |
| 13 | `proposition.rs:266:57 replace == with !=` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1658 |
| 14 | `proposition.rs:267:13 delete CarriedBy contradiction arm` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1652 |
| 15 | `proposition.rs:273:26 replace == with !=` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1660 |
| 16 | `proposition.rs:273:45 replace && with ||` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1659 |
| 17 | `proposition.rs:273:57 replace == with !=` | unchanged line/symbol | caught | killed | EPI-04 proposition contradiction | `caught.txt` line 1661 |
| 18 | `proposition.rs:281:9 Display -> Ok(default)` | unchanged line/symbol | caught | killed | EPI-02/EPI-04/EPI-06 rendering diagnostic | `caught.txt` line 1662 |
| 19 | `proposition.rs:301:13 delete item_carried_by_actor parser arm` | unchanged line/symbol | caught | killed | EPI-02/EPI-03/EPI-04 parser/replay | `caught.txt` line 1665 |
| 20 | `proposition.rs:305:13 delete container_contents_observed parser arm` | unchanged line/symbol | caught | killed | EPI-03/EPI-04 parser/replay | `caught.txt` line 1666 |
| 21 | `proposition.rs:319:13 delete possible_movement_near_place parser arm` | unchanged line/symbol | caught | killed | EPI-07/EPI-09 parser/proposal/view | `caught.txt` line 1669 |
| 22 | `proposition.rs:349:9 delete at_place deserialize_location arm` | unchanged line/symbol | caught | killed | EPI-02/EPI-04 location parsing | `caught.txt` line 1673 |
| 23 | `proposition.rs:351:9 delete carried_by deserialize_location arm` | unchanged line/symbol | caught | killed | EPI-02/EPI-04 location parsing | `caught.txt` line 1675 |
| 24 | `proposition.rs:357:5 render_location -> xyzzy` | unchanged line/symbol | caught | killed | EPI-02/EPI-06/EPI-09 rendering | `caught.txt` line 1677 |
| 25 | `proposition.rs:357:5 render_location -> empty` | unchanged line/symbol | caught | killed | EPI-02/EPI-06/EPI-09 rendering | `caught.txt` line 1676 |
| 26 | `proposition.rs:370:5 validate_location -> Ok(())` | unchanged line/symbol | caught | killed | EPI-02/EPI-03/EPI-05 validation | `caught.txt` line 1678 |
| 27 | `proposition.rs:392:5 require_place -> Ok(())` | unchanged line/symbol | caught | killed | EPI-02/EPI-03/EPI-05 validation | `caught.txt` line 1680 |
| 28 | `proposition.rs:403:5 require_container -> Ok(())` | unchanged line/symbol | caught | killed | EPI-02/EPI-03/EPI-05 validation | `caught.txt` line 1681 |
| 29 | `proposition.rs:51:9 PropositionReferenceError Display -> Ok(default)` | unchanged line/symbol | caught | killed | EPI-02/EPI-05 diagnostics | `caught.txt` line 1635 |
| 30 | `proposition.rs:83:9 PropositionParseError Display -> Ok(default)` | unchanged line/symbol | caught | killed | EPI-02/EPI-05 diagnostics | `caught.txt` line 1636 |

## Additional final-run survivors

There were no additional final-run survivors. `missed.txt` is empty, and
`timeout.txt` is empty.

## Closeout summary

- Historical seed identities reconciled: 30 / 30
- Still-generated historical identities caught: 29 / 29
- Historical identities closed by source-change mapping: 1 / 1
- Newly discovered final-run survivors: 0
- Unresolved timeouts: 0
- Tool failures: 0
- Blocked or untriaged identities: 0
- Exceptions requiring independent signoff: 0
