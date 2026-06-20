# 0043 ORD-LIFE-CERT mutation triage register

Generated: 2026-06-20

## Frozen run identity

- Final implementation commit: `d1cd9abda3e75f06570e4d60534c8db7c512720e`
- Certifying command: `cargo mutants --workspace --no-shuffle -o reports/0043_ord_life_cert_mutation_full.out`
- Supervisor transcript: `reports/0043_ord_life_cert_command_transcripts/004_full_campaign/`
- Supervisor status: `exit_status=2`, `supervisor_result=child_nonzero_exit`, `end_time=2026-06-20T20:53:25+02:00`
- Cargo-mutants denominator: 60 files / 2877 mutants
- Final outcomes: 2257 caught, 5 missed, 0 timeout, 615 unviable
- Completion posture: complete configured denominator; cargo-mutants returned nonzero because final missed mutants exist, not because the launcher, baseline, or denominator failed.

## Fingerprints

| Artifact | SHA-256 |
| --- | --- |
| `Cargo.lock` | `f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59` |
| `.cargo/mutants.toml` | `258109189fc4500ab88c6fa28f47d06292632ac9b274a237589e49a90ef05b11` |
| `.cargo/mutants-baseline-misses.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `.github/workflows/ci.yml` | `ca7274d4acebf19f388a336446aeac66dac089b8338aa8c584f6a05ef56e4639` |
| `reports/0043_ord_life_cert_mutation_list_files.txt` | `510a1c626efda2b5118a6244184bf071c1a1fcda5237d9fa572861938046e50e` |
| `reports/0043_ord_life_cert_mutation_list.txt` | `d9b430fbe111f5ea9a8c9db70bec032b3296444b38dac7a78aeecf672444c0a7` |
| `reports/0043_ord_life_cert_mutation_final_missed.txt` | `d2904d13cfc5404481434cb3d370a9e7b85a53c4f4e343a8b670eec86f1430bb` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/caught.txt` | `7c140e2531c384a5307e646b8754cc59bd713e69cc236388d4b77454631c8c19` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/missed.txt` | `d2904d13cfc5404481434cb3d370a9e7b85a53c4f4e343a8b670eec86f1430bb` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/timeout.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/unviable.txt` | `ea8a56d15463be8f3c7ef4e3f2c7c2d6b12175dfbb9ca6cadaa64f23a216d6ef` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/outcomes.json` | `b1cb26b63643a294d9a1a26039f750f7aabaeddbe1978ba60417eae8fd8dd968` |
| `reports/0043_ord_life_cert_mutation_full.out/mutants.out/mutants.json` | `dc6a2b52391624ae5a2f39115d5a5838e3bf209e9bea6ba3a16b325d153b29bd` |

## Denominator reconciliation

- `reports/0043_ord_life_cert_mutation_list_files.txt` contains 60 configured files.
- `reports/0043_ord_life_cert_mutation_list.txt` contains 2877 configured mutant identities.
- Final outcome files reconcile exactly to the denominator: `2257 caught + 5 missed + 0 timeout + 615 unviable = 2877`.
- `reports/0043_ord_life_cert_mutation_final_missed.txt` is byte-identical to the final `missed.txt` and contains the five canonical cargo-mutants identities.
- Required ORD-LIFE paths are present in the final file denominator: `need_accounting.rs`, `actions/registry.rs`, `actions/defs/continue_routine.rs`, `actions/defs/movement.rs`, `actions/defs/need_events.rs`, and `actions/defs/wait.rs`.

## Historical survivor reconciliation

| Historical identity | Final identity | Final tool outcome | Certification disposition | Evidence |
| --- | --- | --- | --- | --- |
| `crates/tracewake-core/src/need_accounting.rs:88:25: replace < with <= in DurationInterval::contains_tick` | unchanged | caught | killed by 0043ORDLIFCER-002 witnesses and caught in the full configured run | `reports/0043_ord_life_cert_mutation_full.out/mutants.out/caught.txt:474` |
| `crates/tracewake-core/src/need_accounting.rs:106:13: replace && with || in duration_intervals` | unchanged | caught | killed by 0043ORDLIFCER-002 witnesses and caught in the full configured run | `reports/0043_ord_life_cert_mutation_full.out/mutants.out/caught.txt:478` |
| `crates/tracewake-core/src/need_accounting.rs:109:45: replace == with != in duration_intervals` | unchanged | caught | killed by 0043ORDLIFCER-002 witnesses and caught in the full configured run | `reports/0043_ord_life_cert_mutation_full.out/mutants.out/caught.txt:481` |

## New final-run survivors

| Final identity | ORD-LIFE cross-reference | Responsible layer | Certified reachability | Test family gap | Behavior witness needed | Replay/provenance ancestry | Negative control | Disposition | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `crates/tracewake-core/src/actions/defs/movement.rs:77:43: replace && with || in build_move_event` | ORD-LIFE movement/reachability; no-direct-dispatch; possession-continuity ancestry | core action definition, door endpoint matching | reachable: a move can be allowed through an incorrectly matched door endpoint pair | action-level movement adjacency tests do not kill reversed/incorrect endpoint matching | movement must accept only actual `from -> to` or `to -> from` door endpoints and reject asymmetric false positives | actor-moved event must preserve proposal ancestry only after valid physical reachability | synthetic endpoint mismatch must reject movement and remove this survivor | real survivor; routed to `0043ORDLIFCER-006` | `reports/0043_ord_life_cert_mutation_final_missed.txt:1` |
| `crates/tracewake-core/src/actions/defs/movement.rs:78:50: replace && with || in build_move_event` | ORD-LIFE movement/reachability; no-direct-dispatch; possession-continuity ancestry | core action definition, door endpoint matching | reachable: a move can be allowed through a door where only one endpoint matches | action-level movement adjacency tests do not kill partial endpoint matching | movement must reject partial door endpoint matches in either orientation | actor-moved event must preserve proposal ancestry only after valid physical reachability | synthetic partial endpoint mismatch must reject movement and remove this survivor | real survivor; routed to `0043ORDLIFCER-006` | `reports/0043_ord_life_cert_mutation_final_missed.txt:2` |
| `crates/tracewake-core/src/actions/defs/movement.rs:78:33: replace == with != in build_move_event` | ORD-LIFE movement/reachability; no-direct-dispatch; possession-continuity ancestry | core action definition, door endpoint matching | reachable: a move can be allowed when the second door endpoint is not the current place | action-level movement adjacency tests do not kill reversed endpoint predicate inversion | movement must require the current place and destination to be exactly the two door endpoints | actor-moved event must preserve proposal ancestry only after valid physical reachability | synthetic reversed endpoint mismatch must reject movement and remove this survivor | real survivor; routed to `0043ORDLIFCER-006` | `reports/0043_ord_life_cert_mutation_final_missed.txt:3` |
| `crates/tracewake-core/src/actions/defs/movement.rs:78:69: replace == with != in build_move_event` | ORD-LIFE movement/reachability; no-direct-dispatch; possession-continuity ancestry | core action definition, door endpoint matching | reachable: a move can be allowed when the first door endpoint is not the destination | action-level movement adjacency tests do not kill destination endpoint predicate inversion | movement must require the current place and destination to be exactly the two door endpoints | actor-moved event must preserve proposal ancestry only after valid physical reachability | synthetic reversed destination mismatch must reject movement and remove this survivor | real survivor; routed to `0043ORDLIFCER-006` | `reports/0043_ord_life_cert_mutation_final_missed.txt:4` |
| `crates/tracewake-core/src/actions/defs/wait.rs:169:5: replace is_autonomous_wait -> bool with true` | ORD-LIFE wait/need accounting; proposal ancestry; no-direct-dispatch | core wait action definition, autonomous-origin classification | reachable: user-origin wait can be incorrectly treated as autonomous for candidate-goal reevaluation | wait tests do not kill the scheduler/agent-only origin predicate | user-origin wait must keep `candidate_goal_reevaluation=false`; scheduler/agent wait may set it true when thresholds cross | actor-waited and need events must retain proposal ancestry and not rewrite origin semantics | synthetic user-origin wait with threshold crossing must not set reevaluation and must remove this survivor | real survivor; routed to `0043ORDLIFCER-007` | `reports/0043_ord_life_cert_mutation_final_missed.txt:5` |

## Certification disposition summary

- Historical 0042 `need_accounting.rs` survivors: 3 caught, 0 missed.
- New final-run survivors: 5 missed, 0 timeout.
- Tool failures/timeouts: 0.
- Baseline laundering: none; `.cargo/mutants-baseline-misses.txt` remains empty.
- Final replacement certification cannot render `ORD-LIFE-CERT passed` until `0043ORDLIFCER-006` and `0043ORDLIFCER-007` kill these five final missed mutants and the configured campaign evidence is refreshed or superseded at the new frozen commit.
