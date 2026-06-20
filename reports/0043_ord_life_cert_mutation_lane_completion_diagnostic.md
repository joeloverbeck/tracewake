# 0043 ORD-LIFE-CERT mutation lane completion diagnostic

Date: 2026-06-20
Ticket: `archive/tickets/0043ORDLIFCER-003.md`
Spec: `specs/0043_ORD_LIFE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`

## Summary

The 0042 full configured mutation lane is recorded as
`tool-failure/incomplete`: the PTY/session wrapper remained active after no
cargo-mutants child process was visible and the run was interrupted. The
historical wrapper is not repository-owned and cannot be inspected or repaired
from this checkout. The bounded diagnosis is therefore:

- historical failing layer: external interactive PTY/session transport;
- repository-owned correction: `tools/supervise-command.sh`, a non-PTY GNU
  `timeout` supervisor with explicit child status, wrapper timeout, and
  signal/forced-exit classification;
- certification bypass: final mutation evidence must use the non-PTY supervisor
  or an equivalent direct non-PTY launch, not the historical PTY wrapper;
- completion proof rule: process-list absence is never completion proof.

This ticket does not run the full configured campaign. That remains owned by
`0043ORDLIFCER-004`. It proves the transport mechanism and retains regression
transcripts so `-004` can run the full denominator without relying on the
historical PTY wrapper.

## Historical Symptom

The retained 0042 register records:

- `cargo mutants --workspace --no-shuffle`:
  `tool-failure/incomplete; interrupted after no cargo-mutants process was
  visible`.
- `cargo mutants --workspace --no-shuffle -j 8`:
  `tool-failure/incomplete; interrupted after no cargo-mutants process was
  visible`.

Historical transcript references are `/tmp/0042-015-mutants-no-shuffle.txt` and
`/tmp/0042-015-mutants-no-shuffle-j8.txt`. They are historical inputs recorded
in `reports/0042_ord_life_cert_mutation_triage_register.md`; they are not
required to survive as live files in this checkout.

## Supervisor

Repository-owned supervisor:

```text
tools/supervise-command.sh <output-dir> <wall-seconds> <grace-seconds> -- <command> [args...]
```

Properties:

- runs without a PTY;
- uses GNU `timeout --kill-after=<grace>s <wall>s`;
- records `command.txt`, `metadata.txt`, `stdout.txt`, `stderr.txt`, and
  `status.env`;
- distinguishes `child_exit_0`, `child_nonzero_exit`,
  `wrapper_wall_timeout`, `supervisor_or_spawn_failure`,
  `wrapper_forced_kill`, and `child_signal_or_forced_exit`;
- copies a default `mutants.out` directory to `mutants.out.partial` when one is
  present after the command.

Script fingerprint:

```text
b944544401566644930947796743dbb9c84db8c90782ff01833d7abd0d067179  tools/supervise-command.sh
```

## Regression Controls

| Case | Transcript directory | Exit status | Supervisor result |
|---|---|---:|---|
| normal child | `reports/0043_ord_life_cert_command_transcripts/supervisor_normal_exit/` | 0 | `child_exit_0` |
| nonzero child | `reports/0043_ord_life_cert_command_transcripts/supervisor_nonzero_exit/` | 7 | `child_nonzero_exit` |
| stalled child | `reports/0043_ord_life_cert_command_transcripts/supervisor_stalled_timeout/` | 124 | `wrapper_wall_timeout` |
| killed child | `reports/0043_ord_life_cert_command_transcripts/supervisor_killed_child/` | 143 | `child_signal_or_forced_exit` |

These cases prove that normal exit, nonzero exit, external wall-clock timeout,
and signal-derived child termination produce distinct recorded statuses.

## Cargo-Mutants Non-PTY Check

Command:

```text
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_mutants_list 60 5 -- cargo mutants --workspace --no-shuffle --list
```

Result:

- exit status: 0;
- supervisor result: `child_exit_0`;
- stdout lines: 2877 mutant identities.

This proves cargo-mutants can run through the repository-owned non-PTY
supervisor and emit a complete configured denominator list. It is not the full
certifying campaign; `0043ORDLIFCER-004` must still run
`cargo mutants --workspace --no-shuffle` to completion and reconcile outcomes.

## Evidence Axes

| Axis | 0043-003 disposition |
|---|---|
| Launcher/supervisor result | Recorded in each transcript `status.env`; distinct status classes proven. |
| Denominator completion | Only the list denominator is proven here: 2877 identities. Full outcome denominator remains `-004`. |
| Cargo-mutants run exit | `supervisor_mutants_list` child exited 0. Historical 0042 full run remains incomplete. |
| Per-mutant outcome | Not produced by this ticket except historical/focused evidence from earlier tickets; full outcome matrix remains `-004`. |
| Certification disposition | No certification pass asserted. This ticket supplies transport evidence only. |

## Verification Commands

```text
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_normal_exit 10 1 -- bash -lc 'printf normal-ok'
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_nonzero_exit 10 1 -- bash -lc 'printf nonzero >&2; exit 7'
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_stalled_timeout 1 1 -- bash -lc 'printf stall-start; sleep 5; printf stall-end'
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_killed_child 10 1 -- bash -lc 'printf kill-start; kill -TERM $$; sleep 1'
tools/supervise-command.sh reports/0043_ord_life_cert_command_transcripts/supervisor_mutants_list 60 5 -- cargo mutants --workspace --no-shuffle --list
```

The nonzero, stalled, and killed-child regression commands intentionally return
nonzero shell statuses; their transcript `status.env` files are the pass/fail
evidence for this diagnostic.

## Conclusion

The historical PTY symptom is bounded to an external session/transport layer
because the repository-owned non-PTY supervisor records deterministic child and
timeout statuses and successfully runs cargo-mutants list generation. The final
certifying mutation lane must use this supervisor or an equivalent non-PTY
launch and must retain full `mutants.out` evidence. No process-list heuristic is
acceptable as completion proof.
