# 0044 FIRST-PROOF-CERT Mutation Triage Register

**Status**: BLOCKING - standing perimeter incomplete
**Date**: 2026-06-21
**Ticket**: `0044FIRPROCER-018`
**cargo-mutants**: `27.1.0`

## Verdict

The corrected focused FIRST-PROOF wave completed and found no surviving or
timed-out mutants. The standing checked-in perimeter wave did not complete: the
supervisor stopped it at the 7200-second wall limit after 2384 of 2901 mutants
were classified. Because spec section 10 treats incomplete campaigns as
blocking, this register does not certify `FIRST-PROOF-CERT passed`.

There are no actionable survivors in the completed focused wave and no survivor
or mutant-level timeout rows in the interrupted standing wave.

## Perimeter Honesty

The initial standing `.cargo/mutants.toml` perimeter omitted two minimum-union
carriers identified during ticket reassessment:

| File | Resolution | Rationale |
|---|---|---|
| `crates/tracewake-core/src/time.rs` | Added to `.cargo/mutants.toml` | Make-honest option; temporal carrier belongs in the standing perimeter. |
| `crates/tracewake-core/src/actions/defs/checkcontainer.rs` | Added to `.cargo/mutants.toml` | Make-honest option; focused and standing waves both need the missing-property carrier. |

After the perimeter edit:

| Census command | Result |
|---|---:|
| `cargo mutants --workspace --no-shuffle --list-files` | 62 files |
| `cargo mutants --workspace --no-shuffle --list` | 2901 mutants |

The expected §10.2 carriers checked during this ticket were present in
`reports/0044_first_proof_cert_mutation_list_files.txt`: `validate.rs`,
`time.rs`, `perception.rs`, `contradiction.rs`, `projection.rs`, `events/apply.rs`,
`replay/rebuild.rs`, and `actions/defs/checkcontainer.rs`.

## Campaigns

### Standing Perimeter

| Field | Value |
|---|---|
| Command | `tools/supervise-command.sh reports/0044_first_proof_cert_mutation_full.supervisor 7200 30 -- cargo mutants --workspace --no-shuffle -o reports/0044_first_proof_cert_mutation_full.out` |
| Supervisor result | `wrapper_wall_timeout` |
| Exit status | 124 |
| Start | `2026-06-21T11:38:48+02:00` |
| End | `2026-06-21T13:38:58+02:00` |
| Baseline | passed, `21s build + 36s test` |
| Mutants found | 2901 |
| Classified before interruption | 2384 |
| Caught | 1869 |
| Missed | 0 |
| Timeout | 0 |
| Unviable | 515 |
| Blocking status | Blocking because the campaign was interrupted before all mutants were classified. |

The standing wave was interrupted during `crates/tracewake-core/src/events/apply.rs`
build-phase mutation execution. Its `stderr.txt` records `ERROR interrupted` and
`scenario execution internal error err=interrupted phase=Build`.

### Focused FIRST-PROOF Wave

The literal ticket command using checked-in config was attempted first and
preserved under:

- `reports/0044_first_proof_cert_mutation_focused.invalid_filter.supervisor`
- `reports/0044_first_proof_cert_mutation_focused.invalid_filter.out`

That attempt was stopped with exit 130 before classification because
`.cargo/mutants.toml` `examine_globs` caused the `-f` list to behave as a union
with the standing perimeter: `--list` still produced 2901 rows. The corrected
focused run used `--no-config`, `-C --locked`, and `--test-workspace true` so the
seven-file filter was actually applied while retaining the locked workspace test
shape.

| Field | Value |
|---|---|
| Command | `tools/supervise-command.sh reports/0044_first_proof_cert_mutation_focused.supervisor 7200 30 -- cargo mutants --no-config --workspace --test-workspace true -C --locked --no-shuffle -f crates/tracewake-core/src/epistemics/contradiction.rs -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/agent/perception.rs -f crates/tracewake-core/src/actions/defs/checkcontainer.rs -f crates/tracewake-core/src/events/apply.rs -f crates/tracewake-core/src/replay/rebuild.rs -f crates/tracewake-content/src/validate.rs -o reports/0044_first_proof_cert_mutation_focused.out` |
| Supervisor result | `child_exit_0` |
| Exit status | 0 |
| Start | `2026-06-21T13:41:26+02:00` |
| End | `2026-06-21T14:11:44+02:00` |
| Baseline | passed, `9s build + 29s test` |
| Mutants found | 719 |
| Caught | 600 |
| Missed | 0 |
| Timeout | 0 |
| Unviable | 119 |
| Blocking status | Non-blocking; no missed or timeout rows. |

Focused per-file outcomes:

| File | Caught | Unviable | Missed | Timeout |
|---|---:|---:|---:|---:|
| `crates/tracewake-content/src/validate.rs` | 235 | 11 | 0 | 0 |
| `crates/tracewake-core/src/actions/defs/checkcontainer.rs` | 8 | 6 | 0 | 0 |
| `crates/tracewake-core/src/agent/perception.rs` | 28 | 6 | 0 | 0 |
| `crates/tracewake-core/src/epistemics/contradiction.rs` | 9 | 9 | 0 | 0 |
| `crates/tracewake-core/src/epistemics/projection.rs` | 106 | 37 | 0 | 0 |
| `crates/tracewake-core/src/events/apply.rs` | 164 | 42 | 0 | 0 |
| `crates/tracewake-core/src/replay/rebuild.rs` | 50 | 8 | 0 | 0 |

## Stable Identity Scheme

Every non-caught focused outcome is mechanically unviable and is identified by
line number in `reports/0044_first_proof_cert_mutation_focused.out/mutants.out/unviable.txt`.
The stable identity format is:

`focused-unviable-<line-number-zero-padded-to-4>`

For example, `focused-unviable-0001` is line 1 of that file:
`crates/tracewake-content/src/validate.rs:52:9: replace FixtureValidationToken::mint -> Self with Default::default()`.

The unviable file is the canonical complete register for all 119 focused
non-caught outcomes. Each line records path, line, column, mutated symbol or
operation, and replacement operator/value. Reproduction uses the corrected
focused command above plus `-F '<exact line text after path:line:column>'` if a
single mutant needs local replay.

## Classification

| Outcome class | Count | Classification | Disposition |
|---|---:|---|---|
| Focused `missed.txt` | 0 | No actionable survivor observed. | Non-blocking. |
| Focused `timeout.txt` | 0 | No unresolved mutant-level timeout observed. | Non-blocking. |
| Focused `unviable.txt` | 119 | Mechanically unviable cargo-mutants outputs; generated replacements fail build/check before tests. Evidence is the focused `unviable.txt`, `outcomes.json`, and `debug.log`. | Non-blocking. |
| Standing `missed.txt` | 0 | No actionable survivor observed before interruption. | Non-blocking by itself. |
| Standing `timeout.txt` | 0 | No mutant-level timeout observed before interruption. | Non-blocking by itself. |
| Standing interruption | 1 | Supervisor wall timeout; campaign incomplete. | Blocking for certification. |

## Required Follow-Up

`FIRST-PROOF-CERT` cannot be marked passed until the standing perimeter campaign
is completed or replaced by an accepted remediation spec that changes the
mutation evidence contract. The focused wave does not need survivor remediation.
