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
5. Update the ticket with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
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
2. Run the relevant final gates. For full completion, use the exact commands
   named by repository guidance such as `AGENTS.md`. In this repo, those are:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

   If any required final gate is not run, or is run with different flags, record
   which gate was skipped or changed and why in both the spec `Outcome` and the
   final response.
   If a required gate produces too much output to retain after compaction, first
   run the exact required gate when possible. Then rerun a lower-output
   equivalent only as supplemental confirmation, and record both the exact gate
   and the supplemental command truthfully.
3. Update the spec with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
4. Archive the spec to `archive/specs/`, using `git mv` when tracked.
   After `git mv`, stage the move with rename-aware staging such as
   `git add -A specs archive/specs` or by staging the relevant old and archive
   parent directories, rather than only the now-removed live path.
   Do not pass the removed source file path to `git add` after `git mv`;
   stage the archive destination plus `git add -u`, or use `git add -A` on the
   relevant parent directories.
   Run Git index-mutating commands serially; do not parallelize `git add`,
   `git mv`, `git commit`, or related staging commands. If `.git/index.lock`
   appears, check for active Git processes, then retry serially.
5. Repair active references and ledgers, especially `docs/4-specs/SPEC_LEDGER.md`
   and any implementation-order or index surfaces found in the repo.
   Use concrete sweeps for the exact spec filename, ticket prefix, live paths,
   and archive paths, for example:

```sh
rg -n "<spec filename>|<ticket prefix>|specs/<spec filename>|tickets/<ticket prefix>|archive/specs/<spec filename>|archive/tickets/<ticket prefix>" docs reports specs tickets
```

   Update active references that should point to `archive/specs/` or
   `archive/tickets/`. Leave intentionally historical archive references alone
   unless they describe current location or current status.
   Check active reports and acceptance artifacts for recorded deferrals, live
   ticket paths, live spec paths, and target-commit claims that became stale
   after the last ticket or spec archive.
   If the series commits verifier baselines or generated outputs, run the exact
   comparison command that will be used later, inspect the generated file
   format, and refresh the committed baseline when that file is the intended
   truth source. Leave transient output directories untracked unless the
   ticket/spec explicitly requires archiving them.
6. Re-read updated ticket/spec outcomes and reports after the final verification
   run. Confirm the recorded commands, paths, statuses, and skipped/deviated
   checks match what actually happened. If a report originally recorded a
   deferral that was completed later in the same series, amend the report so the
   final archived state is truthful.
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
9. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, spec archive, reference repair, required final
   checks, and required commits are done. On a resumed `/goal` turn, re-run the
   active-path, archive-path, reference, status, and final-commit audit against
   the live checkout before marking complete; do not rely on a prior final
   response alone.

## Reporting

Final responses must include:

- Tickets completed and archived.
- Spec archived or reason it remains active.
- Verification commands actually run.
- Any checks not run and why.
- Any enumerated-criterion members deferred or dropped, with their recorded
  dispositions, or explicitly `None`.
- Any unrelated pre-existing changes left untouched, or explicitly `None`.

## Maintenance

Launcher metadata for this skill lives in
`.agents/skills/ticket-series/agents/openai.yaml`. Keep its default prompt
aligned with this skill's trigger wording and closeout expectations.
