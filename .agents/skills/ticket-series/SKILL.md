---
name: ticket-series
description: Use for goals that implement a glob or series of tickets in dependency order from tickets/, with an optional referenced spec in specs/ or docs/4-specs/, including one-ticket-at-a-time implementation, acceptance verification, per-ticket archival and commits, final spec or ticket-only closeout, and repository truthing.
---

# Ticket Series

Use this skill when the user asks to implement a ticket series such as
`tickets/0004PHA1THIHAR*`, optionally using a reference spec such as
`specs/0004_PHASE_1*`, especially inside a `/goal`.

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference spec selector: optional; usually a glob under `specs/` or
  `docs/4-specs/` when provided.
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
check for matching `archive/tickets/` entries, the archived reference spec and
final spec-closeout commit when applicable, or ticket-only closeout evidence
when no spec existed. If those exist, route directly to the final completion
audit instead of restarting intake.

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
4. Read the resolved tickets and, when a spec selector or obvious live spec is
   present, the resolved spec. Determine dependency order from explicit
   dependency sections, numbering, ticket prose, and spec sequencing when
   applicable.
5. Check `git status --short` before editing. Preserve unrelated user changes.

## Per-Ticket Loop

Complete exactly one ticket before starting the next.

For each ticket:

1. Reassess assumptions against current code, docs, and crate ownership.
   If the ticket/spec diverges from current truth, correct the ticket/spec first
   and commit that correction separately when it is material.
   If a ticket acceptance criterion cites a local guidance file, verifier
   contract, or build/test/lint contract, read it before relying on it, or
   record why `AGENTS.md` or another higher-priority authority supersedes it.
   If a ticket tells you to add an entry to a named census, registry, or
   allowlist at a specific file or line, read the enclosing test/function name,
   assertion, and failure semantics before editing. Do not assume that a path
   list is a general census when its name or test behavior marks it as a
   negative guard, stale-helper allowlist, or legacy exception list; record the
   ticket/spec divergence before implementing instead of satisfying the file
   instruction mechanically.
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
   When an acceptance-artifact check uses an environment variable path such as
   `TRACEWAKE_ACCEPTANCE_ARTIFACT`, remember that Cargo integration tests may
   resolve relative paths from the package test working directory rather than
   the repository root. Use an absolute path or a path relative to that working
   directory, and classify a path-only failure separately from a manifest or
   wording failure before changing artifact content.
   Before archiving or committing a report ticket, audit whether any tracked
   report, ticket, spec, ledger, or evidence file changed after the commands the
   report claims. If yes, either rerun the exact claimed commands against the new
   tree or record the earlier run as preliminary and state the skipped rerun plus
   reason in the ticket `Outcome`, affected report, spec `Outcome` when
   applicable, and final response. Ask the user only when local evidence cannot
   answer the timing or scope question.
   For a capstone report that will be followed immediately by final spec
   archival, treat pre-report gates as implementation-baseline evidence unless
   they were run after all report/doc/spec edits. Run the final full gates after
   the last spec/ledger/report closeout edit, then reconcile the report, ticket,
   spec, ledger, and final response so the command timing is explicit.
   For required long-running evidence commands such as mutation, soak, or
   generator runs, preserve enough evidence to classify the result honestly:
   capture a transcript when practical, check process liveness before
   interrupting a hung wrapper, retain partial output, and record
   `tool-failure/incomplete` in the ticket/report rather than converting an
   incomplete run into a pass. Deterministic reruns or narrower variants may be
   useful supplemental evidence, but they do not replace the exact required
   command unless the ticket/spec explicitly allows that substitution.
   For `cargo mutants`, also follow
   `references/closeout-edge-cases.md#cargo-mutants-campaigns`: complete the
   discovery run before remediation, batch survivors, use focused commands plus
   `cargo mutants --iterate` to converge, and record disk/scratch cleanup facts.
   When reports cite ignored or generated evidence paths such as `target/...`,
   make the cited evidence durable: inline the relevant summary/list content in
   a tracked report, commit a small tracked summary artifact, or label the
   generated path as local transient evidence.
   Capture source/config/test/fixture cleanliness before creating transcript or
   output directories when the evidence package needs a clean-start claim. If
   transcript or output directories already exist, record generated-evidence
   dirtiness separately from source-tree cleanliness instead of collapsing both
   into a single "not clean" status.
   If a required acceptance artifact or capstone proof needs a clean source tree
   but the main checkout has unrelated dirty work, use the clean temporary
   worktree protocol in `references/closeout-edge-cases.md` instead of folding
   unrelated changes into the evidence package.
   For filtered mutation campaigns, verify the selected denominator before the
   expensive run. Repository mutation config can widen or override CLI filters;
   run the corresponding `--list` or `--list-files` command with the same
   selection flags, record the expected count/scope in the ticket or report,
   and make any deliberate `--no-config` or config override explicit.
   For controlled temporary-break evidence, committed adversarial or negative
   witnesses, generated baselines, or bulky long-running tool outputs, follow
   `references/closeout-edge-cases.md` and keep the stable evidence package
   tracked while leaving scratch output untracked unless explicitly required.
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
`docs/4-specs/`, continue directly to `## Final Spec Closeout`. If no matching
live spec existed for the series, continue directly to `## Ticket-Only
Closeout`. A final ticket's local note that spec archival is out of scope,
deferred, or left for later is not a valid stop condition unless the user
explicitly instructed that the reference spec must remain active.
Before the first final response or `/goal` completion, run the current-state
deferral sweep over the relevant report, spec outcome, ticket outcomes, and
ledger surfaces. If the closeout completed an earlier `deferred`, `pending`,
`out of scope`, or `not run` note, amend that surface before final reporting.

## Ticket-Only Closeout

Use this path only when no matching live reference spec exists under `specs/` or
`docs/4-specs/`, and no prompt instruction named a reference artifact that still
requires archival or truthing.

1. Record the no-spec finding in final reporting as:

```md
Spec archived: None - no matching live spec existed under specs/ or docs/4-specs/.
```

2. Verify ticket archive truth mechanically for the completed family:

```sh
rg --files tickets | rg '<ticket-prefix>'
test -e archive/tickets/<ticket-id>.md # repeat for each completed ticket
rg --files-without-match '^## Outcome' archive/tickets/<ticket-prefix>*.md
rg --files-without-match '^Completed: ' archive/tickets/<ticket-prefix>*.md
```

   For absence checks, no output is expected. For `rg --files-without-match`,
   no printed paths is success; exit code `1` can be expected when every checked
   file contains the required pattern.

3. Run stale-live-path sweeps that do not require a spec placeholder:

```sh
rg -P -n '(?<!archive/)tickets/<ticket-prefix>' docs reports specs tickets archive/reports archive/tickets archive/specs
rg --files specs docs/4-specs | rg '<ticket-prefix>|<family-name>'
```

   No output is expected unless there is an active reference to repair. Leave
   historical archived provenance alone unless it claims a current live path or
   current pending state.

4. Run the relevant final gates after the last tracked closeout edit. For full
   completion in this repo, use the exact `AGENTS.md` commands listed in
   `## Final Spec Closeout`.
5. Confirm there are no staged changes, no matching active ticket paths remain,
   the archived tickets exist, stale live paths are absent, and the final status
   shows only intended changes or documented unrelated pre-existing changes.
6. For active `/goal` runs, mark the goal complete only after the ticket-only
   audit is complete.

## Final Spec Closeout

After all tickets in the series are complete:

Use `references/final-closeout.md` for the detailed pre-gate audit, final-gate,
commit-role, and post-commit check sequence. Keep the hard stops below in force.

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
3. Archive any series-owned or spec-required acceptance reports, capstone
   reports, or replacement artifacts under `reports/` before archiving the spec,
   including when the spec names an `archive/reports/` closeout path. Prefer
   `git mv` for tracked reports. Run each report `git mv` serially; do not batch
   report/spec archive moves or any other Git index-mutating commands in a
   parallel tool call. Retarget compiled references, tests, docs, ledgers, and
   report links from the live `reports/` path to the archived path in the same
   change. If a ticket `Outcome` recorded the pre-archive report path as the
   final path, amend that outcome before closeout.
   After `git mv reports/<report filename> archive/reports/<report filename>`,
   stage with rename-aware parent-directory staging such as
   `git add -A reports archive/reports`, or stage the archive destination plus
   `git add -u`, then inspect `git diff --cached --name-status`. Do not pass
   only the removed source path to `git add` after the move.
   Before staging the report closeout, run a focused completed-deferral sweep
   over current report/spec/ticket outcomes for terms such as `deferred to spec`,
   `pending`, and `not run`; if the closeout completed a deferred item, update
   the report/outcome wording before the first final response.
4. Archive the spec to `archive/specs/`.
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
5. Repair active references and ledgers, especially `docs/4-specs/SPEC_LEDGER.md`
   and any implementation-order or index surfaces found in the repo.
   When the ticket prefix and reference paths are known, run
   `.agents/skills/ticket-series/scripts/closeout-audit.mjs` now, or run the
   equivalent checks manually, before the final gates. Run the helper or the
   same focused checks again after the final commit.
   Use separate concrete sweeps for stale live paths and expected archive
   provenance. First run a strict stale-live-path sweep for the exact live spec,
   report, and ticket paths, for example:

```sh
rg -n '(^|[^/A-Za-z0-9_-])(specs/<spec filename>|reports/<report filename>|tickets/<ticket prefix>)' docs reports specs tickets
rg -P -n '(?<!archive/)specs/<spec filename>|(?<!archive/)reports/<report filename>|(?<!archive/)tickets/<ticket prefix>' docs reports specs tickets archive/reports
```

   Then run an archive-reference audit for expected archived paths, for example:

```sh
rg -n 'archive/specs/<spec filename>|archive/reports/<report filename>|archive/tickets/<ticket prefix>' docs reports archive
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
   When an acceptance artifact cannot self-reference the commit that contains the
   artifact, use explicit commit roles instead of one ambiguous "exact commit":
   `implementation baseline commit` for the code/docs state tested before the
   report exists, `evidence/report commit` for the commit that adds the report or
   capstone evidence, and `archive/truthing commit` for final spec/ledger moves.
   Keep those labels consistent across the report, ticket outcome, spec outcome,
   ledger row, and final response. If any surface intentionally names a different
   role, say so instead of letting target-commit wording drift.
   Before the spec archive/truthing commit, grep the current report/spec/ticket
   outcomes for the role labels above and for broad `exact commit` claims; amend
   ambiguous target-commit wording or record why one role intentionally differs.
   If final acceptance depends on changing external repository governance such
   as a GitHub ruleset or branch-protection setting, treat that as a first-class
   evidence step: capture the blocker transcript, request approval before
   mutating the external setting, apply the smallest settings change that
   satisfies the requirement, capture the post-change transcript, rerun the
   affected artifact ingestion/wording checks, and record the external-state
   role separately from implementation, evidence/report, and archive/truthing
   commits.
   For verifier baselines, generated outputs, ignored evidence paths, and bulky
   tool output directories, follow `references/closeout-edge-cases.md`: refresh
   intended truth-source baselines, commit stable summaries or manifests, and
   leave scratch output untracked unless the ticket/spec explicitly requires it.
6. Run the relevant final gates after the last tracked closeout edit. The gate
   evidence must cover the exact closeout tree that will be claimed complete.
   If a temporary worktree is used for clean-source evidence because the main
   checkout has unrelated dirty paths, read
   `references/closeout-edge-cases.md`, record the excluded paths and tested
   commit, copy back only durable summary facts, and remove the temporary
   worktree or explicitly report why cleanup was blocked.
   Either:
   - run the gates before the final commit, then confirm no tracked or generated
     content changed between the gate run and the committed tree; or
   - commit the final closeout first, then run the gates against that committed
     tree and confirm the post-gate status is clean or contains only documented
     unrelated pre-existing changes.
   For full completion, use the exact commands named by repository guidance such
   as `AGENTS.md`. In this repo, those are:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

   If any tracked or generated content changes after the gates, treat that run
   as an early confirmation and rerun the exact required gates after the last
   closeout edit that the repo checks could cover. If any required final gate is
   not run, or is run with different flags, record which gate was skipped or
   changed and why in both the spec `Outcome` and the final response.
   If a required gate produces too much output to retain after compaction, first
   run the exact required gate when possible. Then rerun a lower-output
   equivalent only as supplemental confirmation, and record both the exact gate
   and the supplemental command truthfully.
   Do not mark the series complete if any required full-completion gate from
   `AGENTS.md` or other repository guidance lacks post-closeout evidence unless
   the exact skipped or changed command and the reason are recorded in both the
   spec `Outcome` and the final response. Treat this as a hard stop before
   goal completion, not a formatting preference.
7. Re-read updated ticket/spec outcomes and reports after the final verification
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

   - For ticket-only series, first sweep only the current archived ticket
     outcomes for the ticket prefix under closeout. Broaden to active
     docs/reports only if stale live references or current-state claims appear.
   - For spec-backed series, first sweep active/current-state surfaces and the
     current archived ticket/spec outcomes for the ticket/spec prefix under
     closeout.
   - Then, only if needed, run the broad historical sweep and treat old problem
     statements, risk notes, and original out-of-scope sections as valid
     archive history unless the outcome now contradicts them.

```sh
rg -n 'pending|remaining|TODO|deferred|out of scope|not run|live path|archive bookkeeping' archive/tickets/<ticket-prefix>*.md
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

8. Run a final status/diff check and commit the spec archive/truthing work if it
   has not already been committed before the final gates.
9. Before sending the final response or marking a `/goal` complete, confirm:
   - no matching active ticket paths remain under `tickets/`;
   - for spec-backed series, the reference spec no longer exists under `specs/`
     or `docs/4-specs/` and the archived spec exists under `archive/specs/`;
   - for ticket-only series, no matching live spec existed under `specs/` or
     `docs/4-specs/` and this is stated explicitly in final reporting;
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
   - if implementation, evidence/report, and archive/truthing commits differ,
     those roles are named consistently in the report, ticket outcome, spec
     outcome, ledger row, and final response.
   - the final answer has been drafted using the `## Reporting` scaffold below
     with every field present and explicit `None` values where applicable.
   - the final response has been checked against the `## Reporting` bullets
     below.
   A compact post-commit audit should include, at minimum:

```sh
git status --short
rg --files tickets | rg '<ticket-prefix>'
test ! -e specs/<spec filename> && test -e archive/specs/<spec filename> # spec-backed only
rg -P -n '(?<!archive/)specs/<spec filename>|(?<!archive/)reports/<report filename>|(?<!archive/)tickets/<ticket-prefix>' docs reports specs tickets archive/reports archive/tickets archive/specs
```

   Inspect command output rather than treating nonzero `rg` exits as failure by
   themselves; for absence checks, no output is usually the expected result.
   When the ticket prefix and reference paths are known, prefer running the
   optional helper `.agents/skills/ticket-series/scripts/closeout-audit.mjs`
   first to produce these active-path, archive-path, stale-live-path, and status
   checks; inspect its output, run any missing focused checks manually, and still
   apply judgment for historical archive prose.
10. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, spec archive, reference repair, required final
   checks, and required commits are done. On a resumed `/goal` turn, re-run the
   active-path, archive-path, reference, status, and final-commit audit against
   the live checkout before marking complete; do not rely on a prior final
   response alone.

## Reporting

Final hard stop before reporting: do not send the final response or mark a
`/goal` complete until the repository-required final gates have covered the
last tracked closeout edit, or until every skipped or changed final gate is
recorded with the exact command/flag difference and reason in both the spec
`Outcome` and final response. If a report, ticket, spec, ledger, or reference
changed after a gate run, treat that run as preliminary evidence until the exact
required gates are rerun or the deviation is explicitly recorded.

For active `/goal` runs, call the goal completion tool after the final audit and
before the final response; do not rely on a prose closeout alone.

Final responses must include:

- `Tickets completed and archived: <list or None>.`
- `Spec archived: <archive path, None - no matching live spec existed, explicit
  user no-archive instruction, or live blocking evidence>.` A ticket-local
  deferred/out-of-scope note is not sufficient.
- `Verification commands run: <commands>.`
- `Checks not run: <commands and why, or None>.`
- `Required AGENTS gate deviations: <command/flag differences and why, or
  None>.`
- `Enumerated-criterion members deferred/dropped: <recorded dispositions, or
  None>.`
- `Unrelated pre-existing changes left untouched: <paths/summary, or None>.`
- `Commits made: <list or None>.` Include this when the run created
  per-ticket or spec-closeout commits.
- `Commit roles: <implementation baseline commit / evidence-report commit /
  archive-truthing commit mapping, or N/A if all evidence genuinely refers to
  one commit>.`

These fields may be embedded in concise prose, but the final answer must make
each answer explicit. Before calling the goal completion tool or sending the
final response, draft from the literal scaffold below with every label present
and `None` values where applicable; do not omit a field because it seems obvious
from the prose. For active `/goal` runs, draft the final response from the
scaffold before calling the goal-completion tool; if any field cannot be
answered, do not mark the goal complete until the missing evidence is gathered
or the deviation is recorded. Include the goal-tool usage summary after marking
the goal complete.

Use this scaffold for final responses unless the user requested a narrower
status-only reply:

```md
Tickets completed and archived: <list or None>.
Spec archived: <archive path, None - no matching live spec existed, explicit user no-archive instruction, or live blocking evidence>.
Verification commands run: <commands>.
Checks not run: <commands and why, or None>.
Required AGENTS gate deviations: <command/flag differences and why, or None>.
Enumerated-criterion members deferred/dropped: <recorded dispositions, or None>.
Unrelated pre-existing changes left untouched: <paths/summary, or None>.
Commits made: <list or None>.
Commit roles: <implementation baseline commit / evidence-report commit / archive-truthing commit mapping, or N/A>.
Goal usage: <goal-tool usage summary, or N/A>.
```

## Maintenance

Launcher metadata for this skill lives in
`.agents/skills/ticket-series/agents/openai.yaml`. Keep its default prompt
aligned with this skill's trigger wording and closeout expectations.

## Support Assets

- `references/closeout-edge-cases.md`: optional progressive-disclosure guidance
  for clean temporary worktree evidence, controlled temporary-break evidence,
  generated baselines, `cargo mutants` campaign handling, ignored evidence
  paths, and bulky long-running tool outputs.
- `references/final-closeout.md`: detailed final closeout audit sequence,
  commit-role guidance, and post-commit verification checks.
- `scripts/closeout-audit.mjs`: optional helper that emits mechanical closeout
  checks for active paths, archived paths, stale live references, and Git status
  when the ticket prefix and reference artifact paths are known.
