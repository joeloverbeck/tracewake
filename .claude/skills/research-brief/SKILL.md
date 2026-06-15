---
name: research-brief
description: "Use when you need to hand a research task to an external deep researcher (ChatGPT-Pro) and want the comprehensive prompt authored here, with full repo access, instead of in a throwaway ChatGPT-Pro session. Interviews you to 95% confidence, then emits a self-contained, paste-ready requirements prompt and refreshes the upload manifest. Triggers: needing a next spec, a thorny fix, a hardening / anti-contamination pass, or a foundational/doc overhaul deep-researched externally. Produces: reports/<topic>-research-brief.md + a refreshed reports/manifest_<date>_<shortsha>.txt. Mutates: only reports/ on user approval."
user-invocable: true
arguments:
  - name: research_target
    description: "What the external deep researcher (ChatGPT-Pro) should produce — the thing to be deep-researched (string). A sentence is fine; the skill sharpens it through exploration and interview."
    required: true
  - name: reference_path
    description: "Optional path to a report, finding-set, or analysis to fold into the brief as established context."
    required: false
---

# Research Brief

Author the comprehensive, paste-ready prompt for an **external deep-research session** (ChatGPT-Pro) — here, where Claude has direct access to the whole repository — instead of reconstructing it interactively in ChatGPT-Pro.

This skill replaces **"Session 1"** of the user's two-stage routine:

- **Session 1 (replaced by this skill)**: explore the repo, interview the user to 95% confidence about their *actual* intent, then author a requirements-style prompt.
- **Session 2 (ChatGPT-Pro, the actual deep researcher)**: receives the uploaded manifest + the emitted prompt, explores and researches online as deeply as needed, and **produces the deliverable directly** — it does not re-interview.

The emitted brief is **self-contained**: ChatGPT-Pro Session 2 has none of this session's context, so everything it needs must live in the prompt plus the uploaded manifest.

<HARD-GATE>
Do NOT Write the brief file or refresh the manifest until BOTH hold:
(a) the interview has reached ~95% confidence (or the user issued an early-exit "just go" — in which case remaining gaps are written into the brief as explicit, labeled assumptions); AND
(b) the Step 5 brief outline + settled-intentions summary has been presented in chat and the user has approved it (silence on a section while answering other questions counts as approval; an explicit objection re-opens that section and requires re-presenting the correction first).
The skill mutates only `reports/`. It NEVER edits `docs/`, `specs/`, `tickets/`, `.claude/skills/`, or source code.
</HARD-GATE>

## Invocation

```
/research-brief "<what ChatGPT-Pro should deep-research>"   [reference_path]
```

## Step 1: Classify the research target

First, read `references/brief-template.md` in full — §A is the canonical brief anatomy you author to in Step 6, and §B is the target-type→reads map you apply in this step. Do not reconstruct these conventions from predecessor briefs; read the template directly so the brief is anchored to the canonical anatomy even when no predecessor exists.

Then read `research_target` (and `reference_path` in full if given). Classify into one type — this selects the load-bearing "read in full" set for Session 2 (the type→reads map is §B of `references/brief-template.md`):

- **new-spec** — what to build/create next for the repo.
- **thorny-fix** — diagnose and resolve a stubborn defect or design knot.
- **hardening / anti-contamination** — strengthen an existing system against drift, contamination, or weak proof.
- **foundational / doc-overhaul** — overhaul a doc tier (or the cascade from an upstream tier change).
- **other** — anything else; build the read set from exploration alone.

Announce the classification on its own line as your **first user-facing output** — after reading the template (so §B informs the choice) but **before any Step 2 repo-exploration tool call** — emit `Classification: <type>` so the audit trail records it independent of your reasoning. (The template read itself is not gated by this rule; it is the prerequisite for classifying.) When ambiguous, give a one-sentence justification.

A target may carry a **dominant type plus a secondary** (e.g. a hardening pass whose deliverable is a new spec). When so, classify by the dominant type to pick the primary read-set, then union in the secondary type's load-bearing reads, and name both in the announcement (`Classification: <dominant> (secondary: <type>)`).

## Step 2: Explore the repo to ground the brief

The point of authoring here is that Claude can read the repo directly — so the *user never types out what the researcher should read*. Build, from exploration:

- the **authority-ordered read list** (`docs/0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`, then relevant `reports/`, `specs/`, `archive/`), each with a one-line reason it is load-bearing for *this* target;
- the **relevant code seams** Session 2 should inspect (name files/modules, don't paste them — Session 2 reads them itself);
- any **prior report / spec / archived work** that already bears on the target, so the brief frames the task as a delta rather than a cold start. When the target is one pass of a multi-block campaign or a follow-up to an earlier brief, name the predecessor `reports/<...>-research-brief.md` explicitly and state what it already delivered (see `references/brief-template.md` §1) so Session 2 does not re-commission completed work.
- for a **cascade pass** (a lower tier realigned to an already-amended upstream tier — see `references/brief-template.md` §B *downward-cascade realignment*), establish the **amendment timeline** before drafting: confirm via `git log -1 -- <tier>` that the target tier predates the upstream amendment, and check whether it is *already partially aligned* (carries tokens, dependencies, or doctrine the upstream change introduced). If partially current, make "validate what is already present and close it with evidence — do not assume every routed item is a gap" an explicit settled intention. Distinct from partial-alignment to *this* amendment: the target tier may also carry **settled doctrine from a prior, separate cascade campaign** on the same tier (e.g. an earlier doctrine-alignment pass whose work predates the amendment now cascading) — record that existing coverage first, recommend only the delta, and carry "do not re-commission prior-campaign settled work" as an explicit settled intention. The reports' recurring *"Settled context not re-commissioned"* section is the home for it (template §A §3).

Launch Explore agents for broad surveys. Verify any repo claim in `research_target` or `reference_path` against the actual tree; flag contradictions prominently.

## Step 3: Light online research (optional)

Only to **sharpen scope and interview questions** — surface the named techniques, prior art, or decision axes the interview should resolve. The *deep* research is Session 2's job; do not do it here. If skipped, announce it in-session as a one-liner (e.g., "Online research: skipped — repo-internal realignment") so the audit trail is visible independent of the brief body. **The research is optional; the run-or-skip announcement is not** — emit it either way, as a peer of the Step 1 `Classification:` line. Commit to a binary **run** or **skip** at emit time; do not defer with a non-committal "will decide after exploration" (a later sharpening lookup, if one happens, gets its own one-liner).

## Step 4: Interview to 95% confidence

Reach **95% confidence about what the user actually wants** — not what they think they should want — before drafting. Display this block after each answer. **When sending a batched `AskUserQuestion`, emit the block immediately before the batch and again after the answers** (the after-block is subsumed only when the answers reach threshold, in which case the "95% — drafting the brief" announcement replaces it):

```
Confidence: X%
Gaps: [specific remaining unknowns]
```

Rules: ask one *conceptual* question at a time when probing motivation or uncertainty sequentially, where each answer reshapes the next; but batch independent, already-scoped bounded choices into a single `AskUserQuestion` call (≤4 questions). A reshaping conceptual question *may* ride in a batch with bounded ones only when the bounded options stay valid across every plausible answer to it (and you supply recommendations); if any bounded option would change meaning depending on the reshaping answer, ask the reshaping question standalone first. Distinguish *invalid/misleading* from merely *moot*: a bounded rider that becomes **moot** under some answers (its choice simply no longer applies) — but whose recommended default stays correct across all of them — is safe to batch with its recommendation attached; reserve the standalone-first rule for riders whose meaning would become wrong or actively misleading under some answer. Prefer bounded multiple-choice (`AskUserQuestion` when available). Probe motivation before solution; challenge premature specificity; name uncertainty specifically; respect demonstrated expertise and "you decide" delegation (re-evaluate and recommend, don't re-ask). Confidence rises from both answers and exploration findings; note which gaps each closes. On receiving batched answers, re-display the `Confidence / Gaps` after-block before proceeding — unless confidence reaches threshold, in which case the "95% — drafting the brief" announcement subsumes it. This applies between **every** two consecutive question rounds, batched or single: when the answers leave you below threshold and the next action is *another* round of questions (not the drafting announcement), re-display the block first — do not chain question rounds without it. Announce "95% — drafting the brief" when reached.

**Early exit**: if the user says "just go," announce current confidence, list remaining gaps, and carry them into the brief as labeled assumptions (`assumption: X`) so Session 2 — which will not ask — treats them as decisions the user can later correct.

## Step 5: Present the brief outline (HARD-GATE)

Before writing, present in chat:

1. the **settled intentions** — the resolved decisions the interview produced (these become §3 of the brief and are what make Session 2 "locked");
2. the **deliverable spec** — exactly which downloadable markdown docs Session 2 must produce (replace vs. new, filenames); for *determination-plus-conditional* targets ("decide if X is needed, and if so produce X"), state both the required verdict and which of the three production modes governs the artifact — unconditionally (verdict embedded), only on a positive verdict, or always-produce-form-follows-verdict (one document whose shape — e.g. spec vs. rationale report — depends on the verdict; this mode needs both Branch A / Branch B shapes specified) — see `references/brief-template.md` §7;
3. the **read-in-full list** (authority-ordered, with the one-line reasons).

Before presenting, confirm both audit-trail announcements were emitted earlier this run: the Step 1 `Classification:` line and the Step 3 online-research run-or-skip one-liner. If either was missed, emit it now. Also confirm every path in the read-in-full list resolves at the fetch-baseline commit before you present it (run the §2-completeness check from Step 6): drop or correct any that don't — predecessor reports often cross-reference files since deleted — so the list you present is the list Session 2 can actually fetch.

Get approval (per the HARD-GATE). Revise on pushback before writing.

## Step 6: Write the brief and refresh the manifest

On approval, do BOTH:

1. **Write the brief** to `reports/<topic>-research-brief.md`, following the canonical anatomy in `references/brief-template.md` §A (read in Step 1). `<topic>` is a short kebab-case slug of the target. Mind the one-word gap from the deliverable: the brief you write now is `<topic>-research-brief.md`, while the §7 deliverable Session 2 writes is often `<topic>-…-research-report.md` (same slug, `brief` vs. `report`). Do not conflate them when choosing the file path you Write, when naming the §1 manifest pointer, or when specifying §7 — writing the brief to the report's path is a silent foot-gun.
2. **Refresh the manifest**: write the current repository path inventory to `reports/manifest_<today>_<shortsha>.txt`, where `<today>` is the real current date (`date +%F`), `<shortsha>` is the fetch-baseline commit's short hash (`git rev-parse --short HEAD`), and the inventory is that exact commit's tree — `git ls-tree -r --name-only HEAD` (use the same `<baseline>` you pin in the brief's §1, not `git ls-files`, so the manifest provably equals the commit Session 2 fetches from). The `<shortsha>` suffix keeps each brief's bundle 1:1 with its §1 commit: a second brief authored the same calendar day at a different HEAD gets a distinct filename and never overwrites a predecessor brief's manifest — a date-only name would silently invalidate the older brief's upload bundle (its §1 pins a different commit than the regenerated manifest then reflects). Name this exact file in the brief's §1 manifest pointer so Session 2 uploads the matching inventory. Leave older manifests in place for the user to clean; regenerate only when one already exists for this exact `<today>_<shortsha>` (a re-run at the same commit), rather than trusting the stale one.

**Baseline-commit rule.** The brief instructs Session 2 to fetch every file from one exact commit, so the manifest must list exactly that commit's tree. Derive the fetch-baseline commit from verified repo HEAD (`git rev-parse HEAD`) at manifest-refresh time, and generate the manifest from that same commit (`git ls-tree -r --name-only HEAD`) — do NOT use `git ls-files`, which reflects the staged index and silently diverges from HEAD under any uncommitted add/delete/rename. If you do fall back to `git ls-files`, first confirm `git status --porcelain` is clean (or reconcile every listed delta) and note the check in the Step 7 summary, since otherwise the manifest and the §1 fetch baseline will not agree. NEVER adopt a commit string copied from a report, doc, or `research_target` without confirming it contains every file in the §2 read-in-full list (`git ls-tree <commit> <path>` / `git cat-file -e <commit>:<path>`) — a "commit of record" cited inside a report is that report's *own* baseline and often predates later merges. If a referenced source cites a different commit, call out the divergence inside the brief rather than propagating it. **Verify cross-commit equivalence claims.** A divergence note often comes paired with a reassurance — "the predecessor's findings still apply because the target files are unchanged between the report's commit and this baseline." That reassurance is a factual claim Session 2 will rely on inside a locked, no-questions brief; **verify it before writing it** with `git diff --stat <report-commit> <baseline> -- <target paths>` (the §2 read-list files the carried-forward findings depend on). State the check, not just the conclusion; if the diff is non-empty, scope the carried-forward claim to the files that genuinely did not change, and flag the rest. Never assert equivalence from memory. **§2-completeness check (the verified-HEAD case too).** Even when the baseline is verified HEAD rather than a borrowed string, confirm every file in the §2 read-in-full list — including files discovered transitively, e.g. a predecessor report's own cross-references — actually exists at the baseline before presenting the list (Step 5) and before writing (`git ls-tree <baseline> <path>` / `git cat-file -e <baseline>:<path>`). A path present in your working tree but absent at the baseline (cleaned up in an earlier commit, so it never appears in `git status`) would ship as a dangling §2 reference and break Session 2's fetch — self-containment is the contract. Drop or correct any that do not resolve, and flag the contradiction. **Untracked-input branch.** A §2 path that is *untracked* — working-tree-present but never committed (it shows as `??` in `git status` and fails `git cat-file -e HEAD:<path>`, distinct from a path cleaned up in an earlier commit) — cannot be fetched from the baseline. Do **not** reflexively drop it: if it is **load-bearing** (e.g. the seed the brief triages), reproduce it **verbatim inline in the brief** (an appendix) so it travels with the pasted prompt — self-containment over fetchability — and note in §1/§2 that it is inlined-because-untracked; if it is **non-essential**, drop it or ask the user to commit it and re-baseline. (A blind "drop" would ship a brief missing the one input it exists to triage.) **Dirty-tree path caution:** when the working tree has staged or unstaged moves/renames/deletes affecting files named in the brief's §2 read-list (a predecessor brief mid-archival, a relocated doc), cite those files at their **baseline-commit** path — confirm with `git ls-tree HEAD <path>` — not the working-tree path, so every §2 reference resolves against the same commit Session 2 fetches from.

Resolve both paths against the worktree root if in a worktree. Do NOT commit.

**Re-baseline an existing brief (HEAD moved).** When a brief authored earlier this session must be
re-issued because HEAD has since changed (the user committed, merged, or pushed), do **not** re-run
the Step 1–4 interview — reuse the settled intentions verbatim. Instead: (a) regenerate the manifest
at the new HEAD (new `<shortsha>` → new filename, so it never overwrites the old one); (b) replace
every commit string in the brief's §1, §7, and §8 with the new baseline; (c) re-resolve every §2
path against the new baseline — a committed rename/move may have relocated files (e.g.
`reports/… → archive/reports/…`) and a previously-untracked inlined input may now be fetchable (add
it as a normal §2 read; you may keep the inline appendix as a convenience copy); (d) re-run the
§2-completeness check end-to-end and grep the brief for the stale shortsha; (e) leave the superseded
manifest in place for the user to clean. This is the expected sequel to authoring against a dirty
tree — see the Step 7 heads-up.

## Step 7: Summarize

Report:

- the two written files (brief + refreshed manifest) — the **upload bundle** for ChatGPT-Pro Session 2;
- a one-line reminder that Session 2 is **locked / no-questions**: paste the brief, upload the manifest, and ChatGPT-Pro should produce the deliverable directly;
- any labeled assumptions carried from an early exit, so the user can correct them before pasting.
- the brief + manifest you just wrote always leave `git status` dirty (two new untracked files); this is **benign** and does **not** invalidate the §1 baseline — the manifest is `git ls-tree HEAD`, which excludes untracked files, so it still equals the pinned commit. Only a later commit/merge/push that moves HEAD does. This is distinct from the dirty-tree cases below.
- if `git status` is dirty at write time — pending moves/renames affecting §2 paths, and/or an untracked load-bearing input reproduced inline — proactively warn the user that committing (or merging/pushing) will move HEAD and invalidate the §1 baseline, triggering the **re-baseline** sequence in Step 6. Surfacing it at authoring time means the user isn't surprised when the bundle needs re-issuing.
- if a predecessor brief's manifest shows as deleted or renamed (`D`/`R`) in `git status` — whenever the deletion happened, not only this session — note that the predecessor brief's own §1 manifest pointer is now dangling: its upload bundle no longer resolves locally, so the user can reconcile (e.g. `git checkout <path>` to restore it) or re-run that brief. Clarify the provenance (deleted this session vs. pre-existing working-tree state, confirmable via `git cat-file -e HEAD:<path>`) so the user knows whether your run caused it; this is distinct from the new manifest you just wrote.

This is an inline-completion deliverable — no next-steps menu. Surface any adjacent improvement spotted during exploration as a flagged note with a concrete trigger, not as scope creep.

## Guardrails

- **Self-containment is the contract.** Session 2 has none of this session's context. Every path, decision, constraint, and acceptance check it needs lives in the brief or the uploaded manifest — never implied.
- **Claude authors; ChatGPT-Pro researches.** Don't perform the deep research here. The brief *commissions* it.
- **Locked, no questions.** The emitted brief instructs Session 2 to produce directly and NOT interview or ask clarifying questions — the interview already happened here.
- **Mutates only `reports/`.** Never touch `docs/`, `specs/`, `tickets/`, `.claude/skills/`, or source.
- **No scope inflation.** The brief commissions what was asked. Resist "while we're at it" additions to the deliverable spec.
