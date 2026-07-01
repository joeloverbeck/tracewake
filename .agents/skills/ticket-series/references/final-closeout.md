# Ticket Series Final Closeout Reference

Use this reference when a ticket series has a live reference spec or a capstone
artifact that must be archived and truthed. The main `SKILL.md` remains
authoritative; this file keeps the detailed audit sequence easy to find.

## Pre-Gate Closeout Audit

Before running final full gates, perform the mechanical closeout checks while
edits are still cheap:

1. Confirm no active ticket paths remain under `tickets/`.
2. Confirm the active spec/report paths are gone and archived paths exist.
3. Confirm archived tickets and specs contain `## Outcome` and `Completed:`.
4. Run a strict stale-live-path sweep for the exact live ticket, spec, and
   report paths.
5. Run an archive-reference audit for expected archived ticket, spec, and
   report paths.
6. Review any matches manually. Historical archived prose can remain only when
   it is not claiming current live location or current status.

If a capstone or acceptance report was created under `reports/` during the
series, reconcile it before final gates:

1. Move the tracked report to `archive/reports/` before archiving the spec.
2. Update report self-paths, links, and evidence-path statements from the live
   path to the archived path.
3. Update ticket outcomes, the spec outcome, and the ledger row so none of them
   describe the live report path or a completed closeout item as still deferred.
4. Reconcile commit-role labels across the report, ticket outcomes, spec
   outcome, ledger row, and final response.
5. Grep the report/ticket/spec outcomes for stale `deferred`, `out of scope`,
   `not run`, live-path, and broad `exact commit` wording before final gates.

When the ticket prefix and reference paths are known, prefer:

```sh
node .agents/skills/ticket-series/scripts/closeout-audit.mjs \
  --ticket-prefix <ticket-prefix> \
  --active-spec <live-spec-path> \
  --archived-spec <archive-spec-path> \
  --active-report <live-report-path> \
  --archived-report <archive-report-path>
```

Then run any missing focused checks manually.

## Final Gates

Run final gates after the last tracked closeout edit. In Tracewake, full
completion requires:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

If any tracked content changes after a gate run, rerun the affected required
gate set after the final edit or record the exact skipped/deviated command and
reason in the spec outcome and final response.

Before final response or goal completion, compare the tree covered by the last
full gate run with the final archive/truthing commit. If tracked or generated
content changed after the gate run, stop and rerun all four required gates
against the final tree. If the gates ran before the commit and the commit only
records the already-gated content, confirm the post-commit status is clean and
record that no content changed between gate run and commit.

## Commit Roles

When an acceptance artifact cannot self-reference the commit that adds it, use
explicit roles consistently across the report, ticket outcome, spec outcome,
ledger row, and final response:

- `implementation baseline commit`: code/docs state tested before the report
  exists;
- `evidence/report commit`: commit that adds the report or capstone evidence;
- `archive/truthing commit`: final spec/report/archive/ledger move.

If a surface intentionally names a different role, say so explicitly.

## Final Response Scaffold

Use the literal final-response scaffold from `SKILL.md` unless the user asked
for a narrower status-only reply. Keep every required label present, including
explicit `None` or `N/A` values, so commit roles, skipped checks, AGENTS gate
deviations, unrelated pre-existing changes, and goal usage are not lost during
long closeouts.

## Post-Commit Audit

After the final commit and final gates, rerun compact current-state checks:

```sh
git status --short
rg --files tickets | rg '<ticket-prefix>'
test ! -e specs/<spec filename> && test -e archive/specs/<spec filename>
rg -P -n '(?<!archive/)specs/<spec filename>|(?<!archive/)reports/<report filename>|(?<!archive/)tickets/<ticket-prefix>' docs reports specs tickets archive/reports archive/tickets archive/specs
rg --files-without-match '^## Outcome' archive/tickets/<ticket-prefix>*.md archive/specs/<spec filename>
rg --files-without-match '^Completed: ' archive/tickets/<ticket-prefix>*.md archive/specs/<spec filename>
```

No output is expected for absence and `--files-without-match` checks. Inspect
nonzero `rg` exits by their output, not by exit code alone.
