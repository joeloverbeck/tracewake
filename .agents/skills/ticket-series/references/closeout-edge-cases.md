# Ticket Series Closeout Edge Cases

Use this reference only when the ticket/spec closeout actually involves these
evidence shapes. Keep the main `SKILL.md` workflow authoritative for ordinary
per-ticket and final-spec closeout.

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
