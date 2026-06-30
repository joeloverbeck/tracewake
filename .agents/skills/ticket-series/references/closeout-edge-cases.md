# Ticket Series Closeout Edge Cases

Use this reference only when the ticket/spec closeout actually involves these
evidence shapes. Keep the main `SKILL.md` workflow authoritative for ordinary
per-ticket and final-spec closeout.

## Clean Temporary Worktree Evidence

Use a clean temporary worktree when an acceptance artifact, capstone report, or
final verification claim needs clean source/config/test/fixture evidence but
the main checkout has unrelated dirty work that must be preserved.

1. Record the main checkout status and identify the unrelated dirty paths that
   are being excluded from the evidence package.
2. Create the temporary worktree from the exact commit or branch state being
   tested, preferably under `/tmp/<series-or-ticket>-acceptance`.
3. In the temporary worktree, verify `git status --short` before running the
   evidence commands. If the tool creates ignored output directories, record
   that generated-evidence dirtiness separately from source-tree cleanliness.
4. Run the required evidence commands in the temporary worktree and record the
   tested commit as the implementation baseline or evidence/report commit, as
   appropriate for the report.
5. Copy only the durable summary facts back into tracked reports, tickets, or
   specs. Do not stage scratch output from the temporary worktree unless the
   ticket/spec explicitly requires it.
6. Remove the temporary worktree when finished. If removal is blocked by Git
   metadata or sandbox permissions, run or request the narrow cleanup needed,
   such as `git worktree prune`, and report the cleanup result.

## Controlled Temporary-Break Evidence

For compile-break transcripts, mutation probes, or other intentionally failing
temporary changes:

1. Capture the pre-break diff/status for every file that will be edited.
2. Apply the smallest reversible mutation.
3. Run the expected failing command and preserve the relevant error lines.
4. Revert the temporary mutation.
5. Verify the touched files have no remaining diff before recording the
   transcript as evidence.
6. Mark the transcript manual or non-CI when it intentionally breaks the tree.

Do not let a temporary-break artifact become the committed source of truth.

Committed adversarial or negative tests may substitute for a manual temporary
production break only when they prove the same failure mode. Record the
substitution explicitly in the ticket, report, spec outcome, or final closeout
surface that requested deliberate-break evidence, including the committed test
or fixture name and the manual break that was intentionally not performed.

## Generated Baselines

If the series commits verifier baselines or generated outputs:

1. Run the exact comparison command that later checks the baseline.
2. Inspect the generated file format before accepting it as the intended truth
   source.
3. Refresh and commit the baseline only when that file is the intended durable
   artifact.
4. Record the comparison command in the ticket/report/spec outcome that cites
   the baseline.

Leave generated scratch directories untracked unless the ticket/spec explicitly
requires archiving them.

## Ignored Or Bulky Evidence Output

Long-running evidence tools such as mutation, soak, or generator runs often
write ignored or very large output directories. Commit the stable evidence
package rather than the whole transient tree:

- command transcripts;
- summary outcome files;
- list/denominator files;
- manifests or locks;
- report-cited hashes;
- exact survivor or failure lists when the report relies on them.

Force-add ignored files only when they are required summary evidence. Leave
bulky per-case logs, diffs, caches, and scratch outputs untracked unless the
ticket/spec explicitly requires archiving them.

When a report cites an ignored path such as `target/...`, make the durable
status explicit: either inline the relevant content in the tracked report,
commit a small tracked summary artifact, or label the generated path as local
transient evidence.

## Cargo Mutants Campaigns

`cargo mutants` is both long-running evidence and a disk-risk command. Treat a
surviving mutant as evidence to classify, not as a reason to discard the rest of
the run.

1. For a full standing campaign, let the discovery run complete even if
   `missed.txt` becomes non-empty. Record the full caught/missed/unviable/
   timeout disposition from that discovery pass.
2. Batch survivors by owning surface. Avoid one-survivor-at-a-time full
   reruns.
3. After survivor fixes, use focused `cargo mutants` commands for the touched
   surface and `cargo mutants --iterate` against the existing `mutants.out`
   state to retest only remaining missed or timed-out mutants when the campaign
   state is still valid.
4. Run a final clean-baseline full campaign only after the survivor set is empty
   or every residual has an explicitly recorded bounded forcing function. Only
   this final clean run may be cited as canonical green standing evidence.
5. Monitor disk usage during repeated or interrupted mutation work. If a run is
   interrupted, inspect for leaked scratch directories before starting another
   full campaign.
6. In Tracewake, use `tools/clean-build-scratch.sh` dry-run before cleanup and
   `tools/clean-build-scratch.sh --force` only when the user has accepted the
   deletion scope. Do not run WSL shutdown or Windows-side VHD compaction from
   an agent session; `AGENTS.md` reserves that for a human.
7. Record in the ticket/report whether scratch cleanup was run, skipped, or
   left to the user, and keep bulky `mutants.out` trees untracked unless the
   ticket/spec explicitly requires archiving them.
