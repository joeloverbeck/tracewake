---
name: ticket-series
description: Use for goals that implement a glob or series of tickets in dependency order from tickets/ with a referenced spec in specs/ or docs/4-specs/, including one-ticket-at-a-time implementation, acceptance verification, per-ticket archival and commits, final spec archival, and repository truthing.
---

# Ticket Series

Use this skill when the user asks to implement a ticket series such as
`tickets/0004PHA1THIHAR*` using a reference spec such as `specs/0004_PHASE_1*`,
especially inside a `/goal`.

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference spec selector: usually a glob under `specs/` or `docs/4-specs/`.
- Any explicit sequencing, verification, commit, or archival constraints from
  the prompt.

If an input glob is ambiguous, inspect matching paths and choose only when the
repo context makes the intended family clear. Ask before proceeding if multiple
families plausibly match.

If an input glob has zero matches, do not silently proceed. Search nearby
prefixes once, such as the same numeric/spec prefix with small spelling
differences. If exactly one obvious correction exists in the live checkout,
report the correction and use that path family. Stop and ask if there are zero
or multiple plausible corrections.

On a resumed active `/goal`, zero live ticket matches may mean the series was
already archived in an earlier turn. Before treating that as selector failure,
check for matching `archive/tickets/` entries, the archived reference spec, and
the final spec-closeout commit. If those exist, route directly to the final
completion audit instead of restarting intake.

## Startup

1. Read the live checkout first. Do not rely on memory or prior runs for current
   ticket/spec state.
2. Confirm repository guidance and authority:
   - `AGENTS.md`
   - `docs/README.md`
   - `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
   - `tickets/README.md`
   - `docs/4-specs/SPEC_LEDGER.md`
   - `docs/archival-workflow.md`
3. Resolve the ticket and spec selectors to concrete paths.
4. Read the resolved spec and tickets. Determine dependency order from explicit
   dependency sections, numbering, ticket prose, and spec sequencing.
5. Check `git status --short` before editing. Preserve unrelated user changes.

## Per-Ticket Loop

Complete exactly one ticket before starting the next.

For each ticket:

1. Reassess assumptions against current code, docs, and crate ownership.
   If the ticket/spec diverges from current truth, correct the ticket/spec first
   and commit that correction separately when it is material.
   Identify explicit preconditions, approvals, or "do not proceed until" clauses
   before editing. If the user's current request clearly satisfies a
   precondition, record that in the ticket/spec `Outcome`; otherwise ask before
   crossing it.
   If acceptance criteria contain an un-expanded quantifier ("every …",
   "both …", "all …", a named defect class) or an unresolved conditional
   ("if nontrivial", "where applicable"), treat that as ticket divergence:
   enumerate the actual members against the current codebase, record the
   enumeration (or the conditional's recorded yes/no resolution) in the ticket
   before implementing, and never read the quantifier as meaning only the
   spec's cited instance. The cited site is where the defect was found, not
   the scope of the fix.
2. Identify the narrow implementation surface and the exact acceptance
   criteria, including the full member list of any enumerated criterion.
3. Make the minimal code/doc/test changes that satisfy the ticket while
   preserving the repository's documented invariants, ownership boundaries, and
   dependency direction.
4. Run targeted checks that prove the ticket acceptance criteria. Use broader
   gates when the touched surface or ticket requires them.
   When acceptance criteria enumerate members (apply arms, surfaces, record
   kinds, files), prove each member individually; a representative instance
   does not satisfy the list.
   For acceptance-artifact or report tickets, run the checks the artifact claims
   after composing or amending it, unless intentionally combining that ticket
   with the spec archive/truthing commit and recording that choice.
   For required long-running evidence commands such as mutation, soak, or
   generator runs, preserve enough evidence to classify the result honestly:
   capture a transcript when practical, check process liveness before
   interrupting a hung wrapper, retain partial output, and record
   `tool-failure/incomplete` in the ticket/report rather than converting an
   incomplete run into a pass. Deterministic reruns or narrower variants may be
   useful supplemental evidence, but they do not replace the exact required
   command unless the ticket/spec explicitly allows that substitution.
5. Update the ticket with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
   The archived ticket outcome must use the canonical heading and completion
   date shape:

```md
## Outcome

Completed: YYYY-MM-DD
```

   Before committing, verify the archived ticket mechanically, for example:

```sh
rg --files-without-match '^## Outcome' archive/tickets/<ticket-id>.md
rg --files-without-match '^Completed: ' archive/tickets/<ticket-id>.md
```

   For `rg --files-without-match`, success means no paths are printed. Exit
   code `1` can be expected when every checked file contains the required
   pattern.

6. Archive the ticket:
   - Create `archive/tickets/` if absent.
   - Prefer `git mv` for tracked tickets.
   - Use plain `mv` only for untracked tickets.
   - Confirm the original `tickets/` path is gone.
   - After `git mv`, stage the move with `git add -A tickets archive/tickets`
     or equivalent rename-aware staging of the relevant old and archive parent
     directories.
     Do not pass the removed source file path to `git add` after `git mv`;
     stage the archive destination plus `git add -u`, or use `git add -A` on
     the relevant parent directories.
     If a path-specific `git add -u tickets/<ticket-id>.md` fails because the
     source path no longer exists, recover with this rename-aware staging:
     `git add -A tickets archive/tickets`. Then inspect
     `git diff --cached --name-status`.
     Run Git index-mutating commands serially; do not parallelize `git add`,
     `git mv`, `git commit`, or related staging commands. If `.git/index.lock`
     appears, check for active Git processes, then retry serially.
7. Sweep active specs, ledgers, docs, and ticket references for stale live
   ticket paths. Update references that should now point to `archive/tickets/`.
8. Review the diff for unrelated changes.
9. Commit the completed ticket work before moving on. Use a concise message that
   names the ticket.

Do not advance to the next ticket on plausible implementation alone. Acceptance
criteria must pass, or the ticket must be explicitly blocked with evidence.
Never narrow an enumerated criterion silently: if a member proves un-fixable,
moot, or wrongly listed mid-implementation, give it an explicit recorded
disposition in the ticket `Outcome` (done, deferred to a named follow-up
ticket, or dropped with rationale and evidence). Silently shipping N-1 of N
members is a failed ticket, not a completed one.

After committing the final ticket in the series, stop before any final response
or `/goal` completion. If the reference spec still exists under `specs/` or
`docs/4-specs/`, continue directly to `## Final Spec Closeout`. A final ticket's
local note that spec archival is out of scope, deferred, or left for later is
not a valid stop condition unless the user explicitly instructed that the
reference spec must remain active.

## Final Spec Closeout

After all tickets in the series are complete:

Complete, archive, and commit the final ticket before spec archival when the
final ticket is ordinary implementation work. If the final ticket is itself the
capstone acceptance artifact or spec-closeout vehicle, it may be combined with
the spec archive/truthing commit; record that choice and any resulting
deviations in the ticket and spec outcomes.

1. Re-read the reference spec and verify every acceptance item is either done,
   explicitly rejected, deferred, or not implemented.
   - If the final ticket says spec archival is out of scope, deferred, or left
     for later, treat that as ticket-local scope only. This final closeout still
     controls unless the user explicitly says not to archive the reference spec.
2. Update the spec with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
3. Archive the spec to `archive/specs/`.
   - Create `archive/specs/` if absent.
   - Prefer `git mv` for tracked specs.
   - Use plain `mv` only for untracked specs.
   - Confirm the original `specs/` or `docs/4-specs/` path is gone.
   After `git mv`, stage the move with rename-aware staging such as
   `git add -A specs archive/specs` or by staging the relevant old and archive
   parent directories, rather than only the now-removed live path.
   Do not pass the removed source file path to `git add` after `git mv`;
   stage the archive destination plus `git add -u`, or use `git add -A` on the
   relevant parent directories.
   If a path-specific `git add -u specs/<spec filename>` fails because the
   source path no longer exists, recover with `git add -A specs archive/specs`
   and then inspect `git diff --cached --name-status`.
   Run Git index-mutating commands serially; do not parallelize `git add`,
   `git mv`, `git commit`, or related staging commands. If `.git/index.lock`
   appears, check for active Git processes, then retry serially.
4. Repair active references and ledgers, especially `docs/4-specs/SPEC_LEDGER.md`
   and any implementation-order or index surfaces found in the repo.
   Use separate concrete sweeps for stale live paths and expected archive
   provenance. First run a strict stale-live-path sweep for the exact live spec
   and ticket paths, for example:

```sh
rg -n '(^|[^/A-Za-z0-9_-])(specs/<spec filename>|tickets/<ticket prefix>)' docs reports specs tickets
rg -P -n '(?<!archive/)specs/<spec filename>|(?<!archive/)tickets/<ticket prefix>' docs reports specs tickets archive/reports
```

   Then run an archive-reference audit for expected archived paths, for example:

```sh
rg -n 'archive/specs/<spec filename>|archive/tickets/<ticket prefix>' docs reports archive
```

   Update active references that should point to `archive/specs/` or
   `archive/tickets/`. Leave intentionally historical archive references alone
   unless they describe current location or current status.
   When sweeping archived files for stale live paths, distinguish historical
   body prose from active/current-state claims. If needed, use archive-excluding
   patterns such as `(?<!archive/)tickets/<ticket prefix>` or
   `(?<!archive/)specs/<spec filename>` to avoid treating expected archived
   provenance as a live-path defect. Quoted user instructions inside archived
   `Outcome` sections are historical provenance unless they claim a current live
   location.
   Check active reports and acceptance artifacts for recorded deferrals, live
   ticket paths, live spec paths, and target-commit claims that became stale
   after the last ticket or spec archive.
   If the series commits verifier baselines or generated outputs, run the exact
   comparison command that will be used later, inspect the generated file
   format, and refresh the committed baseline when that file is the intended
   truth source. Leave transient output directories untracked unless the
   ticket/spec explicitly requires archiving them.
5. Run the relevant final gates after the last tracked closeout edit and before
   the final commit. For full completion, use the exact commands named by
   repository guidance such as `AGENTS.md`. In this repo, those are:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

   The final gate evidence must cover the exact closeout tree that is committed.
   If any tracked or generated content changes after the gates, treat that run
   as an early confirmation and rerun the exact required gates after the last
   closeout edit that the repo checks could cover. If any required final gate is
   not run, or is run with different flags, record which gate was skipped or
   changed and why in both the spec `Outcome` and the final response.
   If a required gate produces too much output to retain after compaction, first
   run the exact required gate when possible. Then rerun a lower-output
   equivalent only as supplemental confirmation, and record both the exact gate
   and the supplemental command truthfully.
6. Re-read updated ticket/spec outcomes and reports after the final verification
   run. Confirm the recorded commands, paths, statuses, and skipped/deviated
   checks match what actually happened. If a report originally recorded a
   deferral that was completed later in the same series, amend the report so the
   final archived state is truthful.
   Mechanically verify that every archived ticket in the series and the archived
   spec use the repository archival format:

```sh
rg --files-without-match '^## Outcome' archive/tickets/<ticket-prefix>*.md archive/specs/<spec filename>
rg --files-without-match '^Completed: ' archive/tickets/<ticket-prefix>*.md archive/specs/<spec filename>
```

   For `rg --files-without-match`, success means no paths are printed. Exit
   code `1` can be expected when every checked file contains the required
   pattern.
   Treat any printed path as incomplete archive truthing; fix it before the
   final commit.
   Also grep active reports and outcomes for stale pending-state language, then
   manually review any matches. Keep this staged so broad historical archive
   prose does not swamp the current-state audit:

   - First sweep active/current-state surfaces and the current archived
     ticket/spec outcomes for the ticket/spec prefix under closeout.
   - Then, only if needed, run the broad historical sweep and treat old problem
     statements, risk notes, and original out-of-scope sections as valid
     archive history unless the outcome now contradicts them.

```sh
rg -n 'pending|remaining|TODO|deferred|out of scope|not run|live path|archive bookkeeping' reports docs/4-specs archive/tickets/<ticket-prefix>*.md archive/specs/<spec filename>
rg -n 'pending|remaining|TODO|deferred|out of scope|not run|live path|archive bookkeeping' reports archive/tickets archive/specs docs/4-specs
```

   Keep final audit output small enough to inspect. Prefer focused commands for
   status, staged diff, archive-format checks, stale-live-path checks, and
   archive-reference checks. Do not bundle broad historical sweeps with large
   diffs or status checks in one parallel batch. If an audit output is truncated
   or too noisy to verify, rerun smaller untruncated checks before committing or
   marking the goal complete.
   If compaction interrupts or obscures a required gate, do not infer success
   from prior intent. Check whether the process is still running, then rerun the
   exact required gate if completion evidence is unavailable.

7. Run a final status/diff check and commit the spec archive/truthing work.
8. Before sending the final response or marking a `/goal` complete, confirm:
   - no matching active ticket paths remain under `tickets/`;
   - the reference spec no longer exists under `specs/` or `docs/4-specs/`;
   - the archived spec exists under `archive/specs/`;
   - active ledgers, reports, specs, docs, and ticket references no longer point
     at stale live paths;
   - there are no staged changes after the final commit;
   - the final status/diff check shows only intended changes, a clean worktree,
     or documented unrelated pre-existing changes that were deliberately left
     untouched;
   - the spec archive/truthing commit exists.
   - if the final gates ran before that commit, no content changed between the
     final gate run and the committed tree; otherwise rerun the required gates.
   - each required final gate from `AGENTS.md` or other repository guidance has
     a literal matching command in the run evidence, or any flag/command
     difference is recorded in the spec `Outcome` and final response.
   - the final response has been checked against the `## Reporting` bullets
     below.
9. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, spec archive, reference repair, required final
   checks, and required commits are done. On a resumed `/goal` turn, re-run the
   active-path, archive-path, reference, status, and final-commit audit against
   the live checkout before marking complete; do not rely on a prior final
   response alone.

## Reporting

For active `/goal` runs, call the goal completion tool after the final audit and
before the final response; do not rely on a prose closeout alone.

Final responses must include:

- `Tickets completed and archived: <list or None>.`
- `Spec archived: <archive path, explicit user no-archive instruction, or live
  blocking evidence>.` A ticket-local deferred/out-of-scope note is not
  sufficient.
- `Verification commands run: <commands>.`
- `Checks not run: <commands and why, or None>.`
- `Required AGENTS gate deviations: <command/flag differences and why, or
  None>.`
- `Enumerated-criterion members deferred/dropped: <recorded dispositions, or
  None>.`
- `Unrelated pre-existing changes left untouched: <paths/summary, or None>.`
- `Commits made: <list or None>.` Include this when the run created
  per-ticket or spec-closeout commits.

These fields may be embedded in concise prose, but the final answer must make
each answer explicit. Before calling the goal completion tool or sending the
final response, draft or mentally check the final answer against this list with
literal labels and `None` values where applicable; do not omit a field because
it seems obvious from the prose. For active `/goal` runs, include the goal-tool
usage summary after marking the goal complete.

## Maintenance

Launcher metadata for this skill lives in
`.agents/skills/ticket-series/agents/openai.yaml`. Keep its default prompt
aligned with this skill's trigger wording and closeout expectations.
