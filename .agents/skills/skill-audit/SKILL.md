---
name: skill-audit
description: Use after a Codex skill was exercised in the current session to audit that skill against actual session evidence, Tracewake guidance, and Codex-local skill conventions; also use when the user asks to improve or implement audit findings for a skill under .agents/skills/ or another Codex skill directory.
---

# Skill Audit

Audit a Codex skill against the work just performed. During the audit phase,
report only: do not modify the target skill. Editing is allowed only in a
follow-up implementation phase when the user explicitly asks to implement audit
findings.

## Inputs

- Target skill directory, usually `.agents/skills/<skill-name>`.
- If the user gives a path that is not a directory containing `SKILL.md`, search
  for a single matching `SKILL.md` under `<path>*` and then `<path>**`.
  Use a single unique match and report the path correction. Stop on zero or
  multiple plausible matches.

## Audit Workflow

1. Read the target skill:
   - Read `SKILL.md` and parse `name`, `description`, and body.
   - If the skill has `references/`, `templates/`, `assets/`, `scripts/`, or
     `agents/`, list those directories before making path-specific suggestions.
   - Re-read after compaction or if the skill changed this session.
2. Read alignment documents:
   - Read `AGENTS.md`.
   - Read `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` unless the target
     is purely process/tooling and cannot affect product behavior.
   - If a root `CLAUDE.md` exists and the target was adapted from Claude Code,
     read it only as source-context, not as authority for Codex behavior.
3. Review session evidence:
   - Instructions that were unclear, buried, or ambiguous.
   - Steps skipped, reordered, or worked around.
   - Edge cases or inputs the skill did not anticipate.
   - Places where Codex had to improvise because the skill gave no guidance.
   - Outcomes that diverged from what the skill intended.
   - Branches not exercised this session. Mark them "not exercised" rather than
     speculating about defects.
4. Cross-check alignment for each finding:
   - Tracewake constitutional invariants, by `INV-NNN`, when product behavior is
     in scope.
   - Repository guidance from `AGENTS.md`, by section.
   - Codex skill conventions: concise `SKILL.md`, accurate trigger
     description, progressive disclosure, no unnecessary auxiliary docs, and
     valid optional `agents/openai.yaml`.
5. Classify each finding:
   - Issue: broken, misleading, contradictory, or likely to cause wrong output.
   - Improvement: existing behavior would work better with clearer placement,
     wording, sequencing, or guardrails.
   - Feature: a new capability that fits the skill's stated intent but is
     currently missing.
6. Assign severity:
   - CRITICAL: would corrupt state, produce materially wrong output, or violate
     a Tracewake invariant. Fix before next use.
   - HIGH: missing or effectively hidden guardrail already caused rework or a
     wrong result, or is likely to fail on the next similar use.
   - MEDIUM: caused meaningful friction or non-obvious improvisation, while the
     final outcome remained acceptable.
   - LOW: wording, discoverability, or coverage polish that did not block work.
7. Verify medium-or-higher claims before reporting:
   - If a finding says content is absent, missing, contradicted, or located in a
     specific section, confirm with a targeted read or grep first.
   - Current in-context reads count if the file has not changed since being
     read.
8. Present the report in the conversation. Do not write audit files.

## Report Template

```markdown
# Skill Audit: <skill-name>

**Skill path**: <path>
**Session date**: YYYY-MM-DD
**Session summary**: <1-2 sentences on how the skill was exercised>

## Alignment Check

- **Invariants**: <aligned / N violations found / N/A - process skill>
- **AGENTS.md**: <aligned / N deviations found / skipped - not present>
- **Codex skill conventions**: <aligned / N deviations found>

## Issues

[If none: "No issues identified."]

1. **[SEVERITY]** <title>
   - **What happened**: <session evidence>
   - **Skill gap**: <what the skill says or fails to say>
   - **Suggestion**: <specific change and target path>

## Improvements

[If none: "No improvements identified."]

1. **[SEVERITY]** <title>
   - **Current behavior**: <what the skill currently says>
   - **Why improve**: <session evidence or reasoning>
   - **Suggestion**: <specific change and target path>

## Features

[If none: "No features identified."]

1. **[SEVERITY]** <title>
   - **What's missing**: <gap description>
   - **Why it fits**: <how it aligns with the skill intent>
   - **Suggestion**: <specific addition and target path>

## Not Exercised This Session

[Omit if every important branch was exercised.]

## Summary

**Total**: N issues, N improvements, N features (N findings) - N CRITICAL, N HIGH, N MEDIUM, N LOW
```

## Report Rules

- Every Issue and Improvement needs session evidence. Purely hypothetical gaps
  belong in Features.
- Suggestions must cite an exact target path, such as
  `.agents/skills/<skill>/SKILL.md` or
  `.agents/skills/<skill>/references/<file>.md`.
- If a finding should be visible but not applied by a later "implement all"
  request, mark the title `- skip` or `- informational`, or end the Suggestion
  with `- no change needed`.
- "Implement all", "implement recommended", and "implement suggestions" mean
  apply every numbered finding except those explicitly marked skip,
  informational, or no-change-needed.
- Double-check the final severity and category counts before presenting.

## Follow-Up Implementation

Use this phase only when the user asks to implement audit findings.

1. Resolve scope:
   - Map requests like "Issue 1" or "Improvement 2" to the prior report.
   - If "implement 1 and 3" is ambiguous across sections, ask a concise
     clarification.
   - Treat "implement all" and "implement recommended" as all untagged findings.
2. Re-evaluate current state:
   - Re-read files that will be edited.
   - If a file changed since the audit or a premise is now false, adapt or
     discard that finding and tell the user.
3. Edit with Codex discipline:
   - Announce intended file edits before making them.
   - Prefer `apply_patch` for manual edits.
   - Keep changes minimal and ordered top-to-bottom within each file.
   - Preserve unrelated user changes in the dirty worktree.
4. Run cascade checks:
   - Grep the target skill for related terms, counts, file paths, output paths,
     and section names that could become stale.
   - If a shared surface changes, list `.agents/skills/` and grep sibling skills
     for the affected term before finishing.
   - Update sibling skills only when the requested implementation scope clearly
     includes that shared convention; otherwise report the inconsistency.
5. Verify edited skill files:
   - Re-read edited files or edited regions with context.
   - Confirm headings, numbering, paths, and cross-references are coherent.
   - Confirm YAML frontmatter and `agents/openai.yaml`, if touched, parse as
     valid YAML.
6. Summarize per finding:
   - `implemented`
   - `adapted - <reason>`
   - `skipped - <reason>`
   - `cascade from finding N - <reason>`

## Guardrails

- Audit phase is report-only.
- Do not invent defects for unexercised branches.
- Reject suggestions that would violate Tracewake invariants or crate ownership.
- Evaluate the skill against its stated purpose; do not expand its scope unless
  the missing feature directly supports that purpose.
- Auxiliary investigation should support a concrete hypothesis. Briefly state
  what you are checking before reading or grepping outside the target skill.
