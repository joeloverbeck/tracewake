---
name: brainstorm
description: "Use when starting a new feature, design, or architectural decision that needs requirements discovery before implementation. Triggers: vague requests, exploration keywords, uncertainty about what to build, need for external research before designing."
user-invocable: true
arguments:
  - name: request
    description: "The brainstorming topic or question (string). Can be a simple sentence or a detailed description."
    required: true
  - name: reference_path
    description: "Optional path to a reference file (report, brainstorming doc, analysis) to read as context before starting the interview."
    required: false
---

# Brainstorm

Confidence-driven collaborative brainstorming. Interviews you until it understands what you **actually want** — not what you think you should want — then proposes approaches, builds a design, and lets you choose what happens next.

<HARD-GATE>
Do NOT write any code, scaffold any project, invoke any implementation skill, or take any implementation action until you have presented a design (or a triage recommendation) and the user has explicitly approved it. This applies to EVERY topic regardless of perceived simplicity.

Carve-out: when the user's initial request pre-authorizes a deliverable — either contingent on the brainstorm's verdict (e.g. "write a spec if changes are warranted") or as a flat unconditional directive to produce a specific named artifact (e.g. "create CLAUDE.md") — presenting the design or triage recommendation IS the approval that satisfies this gate — see Guardrails §User pre-authorization. The Step 6 menu still fires afterward.
</HARD-GATE>

## Process flow

```
Read context (reference file + classify topic)
        |
        v
Confidence-driven interview loop (target: 95%)
        |
        v
Propose 2-3 approaches with tradeoffs
        |
        v
Present design section by section, get approval per section
        |
        v
[If product-behavior] Validate against docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
        |
        v
Persist the deliverable (design doc / spec / tickets / ...)
        |
        v
Next-steps menu (user chooses)
```

**In plan mode**: the design writes to the plan file instead of `docs/plans/`, and `ExitPlanMode` replaces the next-steps menu.

Throughout this skill, the **alignment authority** is `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — tracewake's constitution of hard rules (`INV-001` … `INV-110`). References to "the invariants" or an "INV-NNN" principle all point there.

**Triage/analysis brainstorms** (evaluate a report or finding-set, or answer a diagnostic question, and produce work items): the interview loop and the propose-approaches step are replaced by a single triage recommendation — see Step 3 §Triage brainstorms and `references/triage-and-deliverables.md`.

## Step 1: Read context

Compose your opening message with sub-step 2's classification rule already in mind: when the category is evident from the request alone, the classification belongs in that first message, before any orientation tool call.

1. **Reference material.**
   - **`reference_path` argument**: read the entire file. Extract its key claims, proposals, and open questions.
   - **Inline file references**: if the user names files inline in the request, read them with the same treatment. Read all of them.
   - **Inline-pasted prose**: if the user pastes reference content (external research output, a report excerpt, a finding-set) directly into the request body, that prose IS the reference — there's no file to read.
   - **Not reference material**: the user's own first-hand problem description (bug report, symptoms, repro steps, pasted error/program output as evidence of the defect) is the request itself, not externally-authored reference content. A file named only as the alignment authority (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`) is also not reference material — it's governed by sub-step 3. Inline-named **internal codebase files read for orientation** (`README.md`, CI/config, source files the user points at to ground the design) are likewise not reference material — they're covered by exploration (sub-steps 6–7) and get no reference summary; reserve the 2-3 sentence summary for inline-named **authored artifacts** (a report, research doc, external spec). The discriminator is authored-reference-content vs. repo-state-for-context, not merely whether a filename was mentioned. When the brainstorm's *subject* is itself a set of internal docs under audit (the docs being evaluated, not reference informing a separate design), they likewise get no reference summary; the audit-trail anchor is a one-line scope confirmation of what's being audited against which authority — not a "Reference summary." An artifact (skill, doc, or template) being **brought in from another repo** and corrected/adapted in place is likewise the *subject*, not reference informing a separate design — give it the same one-line scope confirmation, not a reference summary.
   - **Oversized files**: if a reference exceeds the Read token limit, read in sequential offset/limit chunks until fully covered, and say so in the summary. Run a quick `wc -l <path>` first when you suspect a file is large.
   - **Delegated reference reads**: when the brainstorm uses a delegated audit fan-out (sub-step 7) and an inline-named reference is large and load-bearing for *verification* rather than for *orchestration decisions* (slicing, conventions, finding-ID series), the orchestrator may assign its full read to a dedicated fan-out slice instead of reading it whole — name the delegation in the reference summary. The orchestrator still reads enough of it (scope/contract sections, or the lineage successor that supersedes it) to slice and brief correctly; "read all of them" governs orchestration-load-bearing references, not every page of a verification corpus.
   - **Path resolution**: if an inline filename doesn't resolve, glob the parent directory for the closest match (pluralization, hyphen drift, capitalization) before asking the user to re-specify; announce any correction inline.

   **Emit a 2-3 sentence summary** of what the reference material contains as user-facing text before any further exploration (premise checks or repo-state orientation needed to brief immediate delegation may land in between — but nothing else) — this is the user's audit-trail anchor for verifying you understood the reference; note within it any chunked reads of oversized references and any delegated reference reads. For a multi-item report, summarize its *shape* (item count, structure, headline position) rather than enumerating each item; per-item detail belongs in the Step 3 triage. When the reference makes claims about the codebase or product state, treat them as hypotheses to verify during exploration (sub-step 6), and flag prominently any that exploration contradicts. When there is no reference material at all (pure analysis from the request text), no summary fires — proceed to exploration; a single confirming line ("No reference material — the request itself is the spec; proceeding to exploration") is acceptable as the audit-trail anchor and is distinct from the multi-sentence reference summary. The contradiction-flagging here is not limited to reference claims: a **request-borne premise** (a technology or tool the request names that the repo doesn't actually use) and a contradiction surfaced *between two repo sources* during premise-verification (sub-step 6 or Step 4/5) are surfaced the same way — flag prominently, and where the user's intent is ambiguous, defer the resolution to the user with a concrete trigger rather than silently choosing.

2. **Topic classification.** **When there's no reference and the category is evident from the request itself, announce the classification in your opening message** (even when exploration is dispatched in the same turn) — don't let an eager dive into `ls`/grep push it past the first reply. Otherwise, announce it on its own line as soon as it's determinable — often right after reading the primary reference's frontmatter/intro, and in all cases before the broader project-context exploration in sub-step 7. Don't let a large multi-file reference read push the announcement arbitrarily late; classify from the minimum evidence that settles the category, then keep reading. Pick the category the brainstorm falls into:
   - **product-behavior**: anything that changes how the simulation behaves at runtime or what it stores — the event-sourced world kernel and authoritative state, agent cognition/intentions/competence, the typed action and speech-act pipeline, belief and information channels (perception, testimony, rumor, records, inference), institutions and law, questless content and leads, the ordinary-life economy and settlements, the spatial/regional/travel model, worldgen and long-run history, simulation LOD/time/performance, the LLM dialogue and text boundary, the player/controller model, or the TUI/UI over those surfaces. These are governed by `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
   - **dev-process / docs / tooling**: work that does not change simulation behavior — build/test scripts, dev ergonomics, repo tooling, skill design, or documentation edits that don't change product semantics.

   When genuinely ambiguous, give a one-to-two-sentence justification. Example: `Classification: product-behavior (the design adds a typed action validated by the action pipeline).`

3. **Read `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` for product-behavior topics.** You'll need it in Steps 3 and 4 to validate approaches against the invariants (`INV-001` … `INV-110`) — these are the hard-fail rules. For dev-process/tooling/docs topics the read is optional — read it only if the topic directly engages an invariant. The read is satisfied if the invariants content is already in this session's context; re-read only after compaction or when you need a specific invariant's wording. Delegating the read to subagents (e.g. an audit fan-out whose agents each read the constitution) satisfies it **only if** every `INV-NNN` the deliverable will cite is spot-verified against the constitution before persisting — second-hand citations are hypotheses until checked (see Step 4's premise-verification rule). If a brainstorm reveals that the *right* design conflicts with an invariant, the deliverable may need to be an invariants amendment first — amending an invariant is a deliberate constitutional change, so flag this prominently and get explicit sign-off; never quietly design against the constitution.

4. **Calibrate initial confidence from the reference and request.** A thorough reference (rationale, decisions, structure) or a richly-specified request (problem analysis, evidence, root cause, code locations, a clear ask) can start confidence at 70-95%, reducing the interview to closing operational gaps. Don't ask motivational questions when the user has already demonstrated deep understanding.

5. **Interview-skip threshold.** If exploration and research bring confidence to 95%+ before the interview starts, skip Step 2 and go to Step 3; announce the confidence level. At 85-94% with a user-framed, fully-constrained directive (enumerated alternatives, or a clear "do X" with minimal open choices — triage brainstorms are the common case), also skip the discovery interview: go to Step 3, announce confidence, and carry remaining gaps as named assumptions for the user to correct during approach/triage selection. **When a remaining gap would *materially shape the deliverable*** (which approach, what gets stripped vs. kept, the deliverable's structure or class) rather than only a detail, prefer posing it via `AskUserQuestion` at approach-selection rather than carrying it as a silent assumption; reserve named-assumptions for detail-only gaps. (Mirrors §Guardrails §User pre-authorization, which applies the same shape-confirmation rule before writing.) When such a question decides reference/scope *reach* (how far updates propagate — which files, which classes of reference get rewritten), complete the full reference enumeration first (the Step 5 broad grep, §Persist) so the question's options are exhaustive; a census still in progress yields incomplete options and forces a follow-up round once the missed sites surface.

6. **External research.** If the topic needs domain knowledge beyond the codebase (algorithms, industry practice, competing architectures), launch research agents BEFORE the interview and summarize findings for the user first. If codebase exploration already yields a clear root cause with concrete evidence, research may be skipped — note the skip (the sub-step 8 confidence checkpoint is the natural slot for that note). When the Step 2 interview is skipped (sub-step 5), research may instead run concurrently with exploration — background execution is fine — and must be summarized to the user no later than the approaches/triage presentation. If research was supplied as a reference file or pasted prose, the sub-step 1 summary satisfies the announcement.

7. **Project context.** Explore relevant existing material before the interview — `docs/` (the foundation pack, tiered into `0-foundation/`, `1-architecture/`, `2-execution/`, `3-reference/` and indexed by `docs/README.md`), `specs/`, `tickets/`, `reports/`, prior design docs, and the event-schema / domain-model files (`docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`, `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`, `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`) when the topic touches them. Launch Explore agents for broad surveys. For audit-class triage brainstorms that need deep per-surface *review* (not just location of code), structure a delegated audit fan-out per `references/triage-and-deliverables.md` §Audit fan-out — read that section **before composing agent prompts**; the Step 3 §Triage brainstorms optional-read carve-out covers the triage-structure sections only, never the fan-out contract (its prompt rules — per-agent output cap, findings-header-first final message, no sub-delegation — are exactly what improvised prompts omit). For frustration-triggered brainstorms, also explore the history of attempted fixes — repeated remediation or accumulated workarounds signal that Step 3 should include radical options (reset, strip, rebuild) alongside incremental fixes. For triage brainstorms whose input is a report, check whether that report (or an earlier version) was already triaged into an existing spec/ticket/`docs/triage/` file, and frame the current pass as a delta so already-accepted or already-rejected items aren't silently re-proposed. When the report is **pinned to a specific commit** (an exact-commit audit), verify its findings against current `HEAD`, not the pinned commit — enumerate any intervening commits and re-check any that touched the audited surface, since an intervening fix can make a finding stale; record the pinned-vs-`HEAD` delta when you present the triage.

8. **Confidence checkpoint.** After all context reading, announce post-exploration confidence in one sentence on its own line (e.g. `Post-exploration confidence: ~90% — reference file is exhaustive, scope fully specified by the request; external research skipped (codebase-internal topic)`). This drives the Step 2 decision and is the audit-trail anchor before any triage recommendation. For delegated-audit triages, emit this anchor **before launching the fan-out** — the fan-out is part of exploration, but the pre-delegation decision is the anchor's first render point; it renders again atop the triage (see Step 3 §Triage brainstorms). When external research was skipped (sub-step 6), note the skip in this same checkpoint line rather than leaving it implicit. **This anchor also fires on every follow-up cycle spawned from the Step 6 menu** (see Step 6's follow-up-cycle clause), even when that cycle's exploration is trivial — re-entering at Step 1 re-arms this checkpoint.

## Step 2: Confidence-driven interview

The core of the skill. Reach **95% confidence** about what the user actually wants before proposing solutions.

### The protocol

After each user answer, display a confidence block:

```
Confidence: X%
Gaps: [specific list of remaining unknowns]
```

Keep asking until confidence reaches 95%, then announce: "I'm at 95% confidence. Moving to approaches." Under auto mode with a 0-2 question interview, an inline prose alternative is fine (`Confidence: ~90% — gaps listed as named assumptions below`) provided the gaps surface as named assumptions in the very next message. The same inline form (`Confidence: ~X% — gaps: …`) is acceptable outside auto mode for a short, fully-bounded interview (≤2 rounds, no open-ended motivational gaps), provided the gaps stay explicit; reserve the literal two-line block for longer or open-ended discovery.

### Interview rules

1. **One question per message.** Keep open-ended, motivational probing to one question per message — never batch it. (This does not restrict `AskUserQuestion`, whose multi-question schema is the sanctioned form for batching approach-selection or section-approval decisions outside the discovery loop.) **Exception:** a single `AskUserQuestion` may batch 2–4 *bounded, non-motivational* discovery forks (operational choices among enumerable options) when the user has signaled an efficiency preference or the gaps are purely operational — collapsing rounds is then a service, not a shortcut. Strict one-question-per-message still governs anything probing the user's actual-vs-stated want.
2. **Prefer multiple-choice** when the answer space is bounded; open-ended when it isn't. Use `AskUserQuestion` with labeled options when its schema is available; inline numbered options are an acceptable fallback (and are preferred in plan mode for speed).
3. **Probe motivations before solutions.** Ask "What problem does this solve?" and "What happens if we don't?" before "What do you want built?" The first request often describes a solution, not the problem.
4. **Challenge premature specificity.** If the user jumps to implementation details, ask why that specific approach matters. The constraint is often softer than stated.
5. **Detect "should want" vs "actually want".** Watch for buzzword-heavy descriptions, over-scoped requests, vague success criteria ("it should be good" — probe what "good" means), and solutions stated as requirements.
6. **Name your uncertainty specifically.** "I don't know whether this needs to handle edge case X" is useful; "I need more information" is not.
7. **Respect user expertise.** If the user gives a clear, well-reasoned answer, don't re-ask it in different words. Advance.
8. **Handle delegation gracefully.** If the user says "you decide," re-evaluate the options against the invariants and project constraints, present your reasoned recommendation, and advance. Don't re-ask.
9. **Verify load-bearing technical premises.** When a user (or reference) premise is load-bearing for the recommendation — library status, version compatibility, feature presence, external-tool behavior — verify it (package metadata, docs, targeted codebase read) before adapting the recommendation to it. The 30-second check prevents recommending against a true premise the user momentarily mis-stated.
10. **Present empirical findings before asking questions.** When exploration produces concrete findings (data, root-cause evidence, confirmed/refuted hypotheses), present a concise summary — hypothesis, evidence, verdict per hypothesis — before the first interview question.
11. **Absorb premise corrections.** If the user pushes back on a question's framing rather than answering it (declines to answer, supplies context that reshapes the question), treat the response as a premise correction — fold it in, raise confidence accordingly, and reissue only the questions that survive. Don't re-ask a fork the correction already resolved.

### Confidence scoring

| Range | Meaning | Action |
|-------|---------|--------|
| 0-30% | Don't understand the problem yet | Ask about the problem, not the solution |
| 30-60% | Understand the problem, unclear on constraints | Ask about constraints, success criteria, scope |
| 60-80% | Understand problem + constraints, unclear on priorities | Ask about tradeoffs, what matters most |
| 80-95% | Clear picture, a few edge cases/preferences unknown | Ask targeted questions about specific gaps |
| 95%+ | Ready to propose | Move to Step 3 |

Confidence rises from **both** user answers and research findings; when research closes a gap, note which gaps it closed vs which still need user input.

### Early exit

If the user says "just go" or "that's enough questions," respect it: announce current confidence, list remaining gaps as assumptions you'll make, and proceed. Mark those assumptions explicitly in the design so the user can correct them. Use the format `(N) X — assuming Y` (the unknown as X, the assumption after `— assuming`) so a correction is one line.

### Recovery/reset brainstorms

When triggered by frustration indicators ("huge mistake", "wrong approach", "start over", "strip everything"):

1. **Validate the diagnosis before accepting it.** Present what exploration revealed — both what's broken AND what's working — before agreeing with a scorched-earth instinct. The user may be catastrophizing, or the system may be more salvageable than they think.
2. **Focus confidence on "what's actually broken?"** before "what's the fix?" Is the problem structural (architecture) or tactical (configuration, content, missing inputs)? Is the frustration proportional to the evidence?
3. **Always include a radical option** in Step 3 (strip, delete, rebuild) — even if you recommend a less radical path. Validating the aggressive instinct as a real choice respects the user's judgment.

## Step 3: Propose approaches

Present **2-3 distinct approaches**, each with:

- **Name** — a short descriptive label
- **How it works** — 2-4 sentences
- **Tradeoffs** — what you gain, what you give up

Close with a recommendation: name the chosen option upfront, then justify in 1-3 sentences. The recommendation is one global decision, not a per-approach attribute.

**Sub-variant splitting.** If an option's "How it works" hides two internally distinct paths with materially different cost/risk, split them into sibling options or labeled sub-rows (`C1`, `C2`) rather than bundle — otherwise the user's selection forces an immediate clarification round.

**Comparing concrete artifacts.** When approaches differ in a concrete, comparable artifact (file layout, section set, transformations table, deliverable scope), use `AskUserQuestion` option `preview` fields to render the candidates side by side, so the user compares the actual artifacts rather than labels. When several questions are batched in one `AskUserQuestion` call, a `preview` for one question must stay neutral on any axis another question in the same call decides — otherwise the rendered artifact bakes in an answer the user may contradict in the sibling question, forcing a reconciliation round. Either keep the preview agnostic on the dependent axis, or fold that axis into the previewed option set.

**For product-behavior topics**, annotate each approach with invariants alignment: `Invariants: <INV-NNN> @ <surface> (aligns / tensions — reason naming the surface)`. Surfaces include the world kernel / event log, agent cognition, the action/speech-act pipeline, belief & information channels, institutions, storage/persistence, the TUI/UI, and the LLM boundary. Naming the surface forces a second-order check that the proposal actually touches what the invariant governs. If any approach tensions an invariant, flag it explicitly and get sign-off — do not recommend a violating approach silently.

**If the problem space is fully constrained** (a reference provides a proven design, or requirements eliminate alternatives), present a single approach with a one-sentence rationale naming the constraint that narrows the space. Don't invent artificial alternatives.

**For a Port external skill deliverable** (adapting an external skill into this repo), the approach IS a transformations table — per-element strip / replace / preserve decisions, one row per substitution site (not per source line). Present that table as the approach/design so the user can redirect any row; for a port with many substitution sites the rows MAY be grouped by substitution category (vocabulary remap / file drop / convention removal / preserved machinery) for legibility, provided every site is still itemized as its own row. This inline guidance covers the common case; consult `references/triage-and-deliverables.md` §Deliverable classification for the complete deliverable-shape→destination rules. **A substitution not itemized in the table is out of scope until itemized** — itemize file drops (e.g. a source `examples/` dir), domain re-mappings, and convention removals as rows, not loose prose. **Co-ported dependency files**: a ported skill often depends on co-ported files from the source repo (templates, READMEs, referenced docs the skill hard-depends on); itemize each as its own strip/replace/preserve row — they typically carry the same source-repo residue as the skill itself. This shapes Step 3/Step 4; the Step 5 destination row points back to the same rule. **Approval gate**: the table is the Step 3 approach and the ported files are template-structured, so the same-turn-collapse rule applies — under §User pre-authorization (or auto mode) the table may be written in the same turn as its presentation (see `references/triage-and-deliverables.md` §Design-presentation carve-outs, Non-plan-mode fast-track), Step 3's "wait for the user to choose" being vacuous for a single, fully-constrained approach; the "redirect any row" opportunity is then post-write, and the material-deliverable-shape check still fires before the write.

**If the user challenges a dismissal**, redo the analysis from first principles rather than defending prior reasoning; if it reverses, say where the prior reasoning was incomplete. **If the user clarifies a constraint**, redo the approach proposal under the refined constraint without restarting Step 2 — cite which assumption was sharpened. Brief confirmation suffices when the recommendation still holds; full re-presentation is reserved for clarifications that flip it.

**Wait for the user to choose** (or to ask you to refine/combine) before proceeding.

### Triage brainstorms

When the deliverable is a set of work items evaluated against a report, finding-set, or diagnostic question, replace the approaches step with a **triage recommendation**: which items warrant action, which are dismissed, and why. The user's approval of the triage serves the same gating purpose as choosing an approach. Render the Step 1 sub-step 8 post-exploration confidence sentence on its own line before the triage recommendation as the audit-trail anchor. For delegated-audit triages, where the fan-out is itself part of exploration, the anchor renders twice: once before launching the fan-out (it drives the interview-skip decision), then refreshed on its own line atop the triage once the fan-out results and their verification are in.

**Findings sourced from subagents are hypotheses, not verdicts.** Before presenting the triage, independently verify every finding that is load-bearing for the overall verdict or carries the highest severities in play (blocker/high or equivalent) — a targeted Read or grep of the cited symbol suffices. Subagent claims of *absence* ("no CI workflow", "no lint config") deserve particular suspicion: confirm by checking the path yourself before the claim shapes a finding. Record refuted agent claims in the **validated — no action** subsection (or as `refuted-by-verification` when they map to a source-report item) so the correction is visible rather than silently dropped. Before persisting a deliverable that embeds findings below that verification bar (e.g. medium/low findings carried on subagent evidence alone), spot-verify at least one cited symbol per finding — a single grep suffices — or mark that finding's evidence as agent-reported in the deliverable so the implementation stage knows to re-verify. Record the per-finding outcome in a **verification ledger** in the triage close (`operator-verified` / `agent-reported` per finding — see `references/triage-and-deliverables.md` §Closing structure), and carry any `agent-reported` marking into the persisted deliverable where that finding lands; under a large fan-out, unmarked findings are exactly the ones that skip verification silently, so the ledger is a required output slot *whenever findings came from agents* (omit it entirely for a solo triage where you did all the exploration yourself — per `references/triage-and-deliverables.md` §Closing structure), not a conditional reminder.

**Before producing the triage, consult `references/triage-and-deliverables.md` §Triage recommendation structure** when the triage has source-report verdict buckets (per-item sub-fields and `accept-with-modification` / `defer` / `refuted-by-verification` grouping) — for those the inline summary here is not enough to render a conforming triage, so read it first. For a **no-source-report diagnostic** (a "figure out why X" or audit question with no formal report), the four conventions below are sufficient for the *findings presentation* and the read is optional — though the allowance never extends to §Audit fan-out, which is read before delegating (Step 1 sub-step 7). That file holds the verdict types, per-item structure, verdict-bucket grouping, the out-of-report and no-source-report cases, and the closing structure. Four conventions are the easiest to miss and must hold even if you skip the read: (1) **key findings `O<N>`** for out-of-report and no-source-report items (and `R<N>` for source-report items) — never ad-hoc prefixes like `F<N>`; when the diagnostic extends an existing audit lineage with an established finding-ID series (e.g. a hardening-spec series like `ORD-HARD-NNN`), continue that series' IDs as the finding keys instead of `O<N>`, mirroring the severity-vocabulary allowance below; (2) use the **closed verdict vocabulary** (`accept` / `accept-with-modification` / `reject` / `defer` / `already-resolved` / `refuted-by-verification`) — never coin labels like `ACT`/`DISMISS`; (3) when there is **no source report** (a diagnostic/audit question rather than a formal report), **omit the verdict-bucket section entirely** and route all findings to `O<N>` keys — and for an audit/health-check question ("are there issues with X?", "is Y sound?"), add a brief **validated — no action** subsection naming what was checked and found clean; (4) **close with the §Closing structure slots** — verification ledger (when agents were used), deliverable-shape recommendation, named assumptions (`(N) X — assuming Y`; a one-line pointer to the deliverable's Risks / Open Questions section satisfies the slot when the gaps are recorded there), and a direct-answers section when the request poses explicit non-finding questions (see §Closing structure) — the named-assumptions slot is never silently omitted. A **source-report** triage whose verdicts rest on independent verification (e.g. reproducing an audit's claims against live code) may also carry a brief **validated — no action / verification-confirmed** subsection recording what was checked and found to hold — distinct from `refuted-by-verification`, which fires only when a premise is disproved; see `references/triage-and-deliverables.md` §No-source-report diagnostic case.

`O<N>` findings may carry severity tags. When the diagnostic extends an existing audit series (e.g. a hardening-spec lineage), reuse that series' severity vocabulary (blocker/high/medium/…) rather than inventing a new scale, and **calibrate each finding's level against the closest analogous prior finding in the series** — when the assigned level diverges from that precedent (e.g. a sibling-surface defect the lineage previously rated differently), name the precedent and the reason in the finding's rationale or the named assumptions. Severity tags are not verdicts and don't replace the closed verdict vocabulary, but in a hardening lineage they feed ticket ordering, so a calibration divergence deserves disclosure rather than silent judgment.

For product-behavior diagnostics in a phased project, verify each anomaly against the execution/phase-ladder docs (`docs/2-execution/`) to distinguish a genuine defect from intended-deferred behavior **before** presenting it as a finding or candidate work-item. An anomaly confirmed intentional (a later-phase deferral, an intentionally-inert slot) belongs in the **validated — no action** subsection — never offered as a candidate ticket. Resolve these during Step 1 exploration rather than flagging them as open observations for the user to send you back to investigate.

## Step 4: Present design

**Plan mode**: skip per-section gates. Present key decisions in 1-2 messages with conversation-level checkpoints, then write to the plan file. `ExitPlanMode` carries approval.

**Classification pivot check**: if the design reveals a deliverable type different from the Step 1 classification (e.g. a "docs tweak" now changes validation behavior), re-state the refined classification before presenting Section 1.

Once an approach is chosen, present the design **section by section**, scaling each to its complexity (a sentence for trivial parts, up to ~200 words for nuanced ones). Cover the relevant sections, renaming/combining to fit the topic's natural structure:

1. **Overview** — what this achieves, 1-2 sentences
2. **Architecture / structure** — how the pieces fit together
3. **Key decisions** — important choices and why
4. **Data flow / process** — how information moves
5. **Edge cases** — tricky scenarios and how they're handled
6. **Testing / verification** — how to confirm it works
7. **Invariants alignment** — for product-behavior topics, a 3-5 row table: invariant (`INV-NNN`), stance (`aligns` / `tensions` / `N/A`), and a one-line rationale citing the specific design mechanism and the surface it operates at. List only invariants the design actively engages or tensions. Omit for dev-process/tooling/docs topics unless the design touches an invariant.

**After each section**, ask "Does this section look right?" and wait for confirmation. If the user pushes back, revise before continuing. After 3 consecutive approvals with no pushback (2 under auto mode), batch remaining sections in groups of 2-3 — announce the batching. Keep a section standalone if it's substantially more complex or higher-risk.

**Carve-outs** (see `references/triage-and-deliverables.md` §Design-presentation carve-outs for detail):
- **Small deliverable** (≤4 decisions, ≥85% confidence): present as a single structured artifact approved in one turn. Announce the consolidation outside auto mode.
- **Template-structured deliverable** (a spec, ticket, or skill file with its own canonical sections): the template provides the navigation; present the full draft as one artifact, one approval covering it — and under §User pre-authorization that approval is the recommendation presentation, so the draft may be written directly in the same turn.
- **Multi-deliverable triage**: triage approval covers all N deliverables together; each is then approved as a single template-structured draft (no per-deliverable gate).

**Verify operator-introduced design premises before persisting them.** When the design names a specific field, file path, schema shape, API surface, build/test command, or doctrine citation (an `INV-NNN` number and the meaning attributed to it) from the codebase, verify it exists (a quick Read or grep — for doctrine citations, check the cited invariant's actual wording, not a second-hand paraphrase) before writing it into the deliverable — the 30-second check prevents a premise falsifying during implementation. For removals, enumerate ALL references with a broad grep before drafting the files-to-touch and acceptance criteria.

## Step 5: Persist the deliverable

After design approval, do NOT implement until the user picks an implementation option from the Step 6 menu — the design doc is typically the deliverable; implementation is a separate, user-chosen act. Exception: inline ops/setup and in-place edits are executed during the session (the HARD-GATE still requires approval first — though under §User pre-authorization or auto mode, presenting the consolidated plan IS that approval and execution may proceed same-turn; see §Deliverable classification's inline-ops bullet in `references/triage-and-deliverables.md`), and no separate design doc is written.

**Quick triage of deliverable shape → destination** (full per-type rules in `references/triage-and-deliverables.md` §Deliverable classification):

| Deliverable shape | Destination |
|---|---|
| Inline ops/setup task or mechanical-fix batch | execute inline + summary; no file persisted |
| New skill design | `.claude/skills/<name>/SKILL.md` |
| Modify existing skill file(s) | edit in place |
| Project documentation & root instruction/config files (`README.md`, `docs/*.md`, `CLAUDE.md`, `AGENTS.md`) | edit/create in place |
| New or in-place-edited dev-tooling/CI/config file (`.github/workflows/*`, lint/format/build config — incl. a bug-fix edit to an existing workflow) | create/edit in place |
| Amend `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (constitutional change) | edit in place — flag as a constitution amendment, require explicit sign-off |
| Port external skill into repo | new skill file(s) (`SKILL.md` + any `references/`) + delete source + a transformations table itemizing per-element strip/replace/preserve (see the Step 3 Port-external-skill note + `references/triage-and-deliverables.md` §Deliverable classification) |
| Replaces an existing artifact | new file + delete old + update cross-references |
| System spec | `specs/NNNN_TITLE.md` or `docs/4-specs/NNNN_TITLE.md` (numeric prefix continuing the highest `NNNN` across `specs/`, `docs/4-specs/`, **and any archive** (`archive/specs/`); uppercase snake-case — match the existing convention); record the package in `docs/4-specs/SPEC_LEDGER.md` *per the repo's ledger-timing convention* — check how the most recent sibling spec was recorded (hardening-series specs staged in `specs/` get their ledger row at acceptance/closeout, not at proposal) |
| Implementation tickets | `tickets/<PREFIX>-NNN.md` — honor the repo's existing prefix convention (scan `tickets/` + `archive/tickets/`, consult `tickets/README.md` / `tickets/_TEMPLATE.md`); see §Deliverable classification |
| Triage → ≥2 specs / ≥3 tickets | deliverables + `docs/triage/YYYY-MM-DD-<topic>-triage.md` |
| Triage analysis, all deliverables deferred | `docs/triage/YYYY-MM-DD-<topic>-triage.md` (full verdict content) |
| Design doc (default) | `docs/plans/YYYY-MM-DD-<topic>-design.md` |
| Research brief for an external researcher | `reports/<topic>-research-brief.md` |

A default design doc consolidates all approved sections into a clean document with a "Brainstorm Context" header (original request, reference file if any, key decisions that shaped the design, final confidence and assumptions). Specs and tickets skip the design doc — the spec/ticket IS the design. If a spec-drafting or ticket template, or an existing spec in `specs/`, `docs/4-specs/`, or the archive (`archive/specs/` — completed specs move there and often carry the series' structural convention), establishes a convention, follow it; otherwise use the canonical `specs/` section set — Problem Statement, Approach, Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions — and note at the top of the file when you used this default. Do **not** copy the foundation-pack docs' house style (the `docs/NN_*.md` narrative-document structure) into a `specs/` file: those are design/requirements documents, not specs.

If working in a git worktree, resolve every output path against the **worktree root**, not the main repo root, and confirm the file landed there before the Step 6 menu.

Do NOT commit the file. Leave it for user review.

## Step 6: Next-steps menu

| Deliverable category | Step 6 action |
|---|---|
| Plan mode active (any category) | Call `ExitPlanMode` — skip the menu |
| Spec / ticket / triage producing specs/tickets/deferred items / proposal — or any deliverable whose options describe NEXT phases (reassess / decompose / implement / done) | **Menu MANDATORY**, even when written this same turn — writing the file is pre-implementation, and skipping the menu strips the user's named next-phase options |
| Inline-completion (skill design; port external skill; small inline ops/setup; new or in-place-edited dev-tooling/CI/config files; new or in-place-edited project-doc / root-instruction files — `README.md`, `docs/*.md`, `CLAUDE.md`, `AGENTS.md`; in-place edits to existing spec/ticket/docs where the edit IS the completed work) | Skip the menu; summarize what was done |

Categorize by the middle row first; the inline-completion skip fires only when the deliverable falls outside it. A triage whose verdict resolves entirely into inline-completed edits (no specs/tickets/deferred items, e.g. an action-audit that deletes/merges/corrects docs in place) takes the inline-completion skip row, not the mandatory-menu row — see `references/triage-and-deliverables.md` §No-source-report diagnostic case. Reaching the skip row this way does **not** drop the inline-completion summary's obligations: the `Adjacent findings:` slot below is still owed, even though the route in was a reasoned skip from the mandatory-menu row rather than a deliverable that started inline-completion. When summarizing an inline-completion deliverable, end the summary with a required **`Adjacent findings:`** slot (a required output slot like the triage verification ledger, not a conditional reminder): flagged notes for adjacent improvements discovered during exploration — exploration byproducts qualify, e.g. CI/tooling deprecation warnings, EOL notices, or dated external obligations encountered in logs — each with a concrete trigger, or the literal `none`. These are not menu items and not scope creep; the user can opt in, and opting into such a note is a lightweight continuation, not a full Step 1 re-entry. For a mandatory menu, present options like:

```
Design doc written to docs/plans/YYYY-MM-DD-<topic>-design.md

What would you like to do next?
1. Create a spec from this design (write to specs/)
2. Decompose into implementation tickets (write to tickets/)
3. Start implementing directly
4. Done for now — I'll review the design doc later
```

Adjust options to the deliverable: for a spec, offer reassess / decompose into tickets / implement / done; for tickets, offer begin-first-ticket / reassess / done. Use `AskUserQuestion` when its schema is available; inline numbered options are an acceptable fallback — and the preferred form when the deliverable write and the recommendation presentation collapsed into one turn (see `references/triage-and-deliverables.md` §Non-plan-mode fast-track, Intra-turn ordering). If the user picks an option that invokes another skill, invoke it. If they pick "done," end.

**If a Step 6 option spawns a follow-up brainstorm cycle for related scope** (a next ticket alongside the just-written one, a related spec the menu surfaced — distinct from re-triaging the prior verdicts): re-enter at Step 1, not Step 2; emit the Step 1 sub-step 8 post-exploration confidence anchor for the new cycle (even when exploration is trivial); the prior cycle's context typically elevates initial confidence so the interview-skip threshold applies.

**Post-menu deliverable amendment**: a small user-directed amendment to the just-written deliverable arising from post-menu conversation (a new acceptance-artifact item, an ordering note, a clarified criterion) is a lightweight continuation, not a new cycle — apply it in place with the user's approval and note the delta. Reserve Step 1 re-entry for amendments that add findings, change the triage verdict, or change the deliverable's shape.

## Guardrails

- **YAGNI ruthlessly.** Strip optional extras from every design unless the user explicitly asked for them.
- **No scope inflation.** The design covers what was asked. Resist "while we're at it" improvements.
- **One question at a time** for open-ended/motivational probing during the Step 2 discovery interview. (Does not restrict `AskUserQuestion` batching outside the loop, nor a single `AskUserQuestion` batching 2–4 bounded, non-motivational discovery forks per Interview rule 1.)
- **No implementation before approval.** The HARD-GATE means what it says, subject only to the §User pre-authorization carve-out.
- **The invariants are authoritative.** For product-behavior topics, if an approach violates an invariant (`INV-NNN`), flag it immediately and get explicit sign-off. Never design against the constitution silently; a feature that genuinely needs to diverge requires an invariants amendment first.
- **Respect early exit.** If the user wants to skip ahead, let them — list your assumptions clearly.
- **Worktree discipline.** If working in a worktree, all file paths use the worktree root.
- **Blocker discovery during implementation.** If implementation reveals an architectural issue, a cost/scope re-estimate, or that a CI/lint/test/format gate being introduced fails against the current codebase — anything that blocks the deliverable or invalidates the approach ranking — present the blocker, options, and a recommendation. If the fix is small (< 50 lines, single file, no public-interface change), fix it inline and continue; otherwise create a ticket and either work around it with a documented constraint or implement the fix with user approval. **Pre-write case:** when the blocker is surfaced not during implementation but during a Step 4/5 verification *before* any deliverable is written, and its fix would add a deliverable of a new class or cross the deliverable count, the §User-pre-authorization shape-confirmation rule takes precedence over this fix-inline allowance — pose a one-shot `AskUserQuestion` confirming the expanded shape before writing, even under pre-authorization, however small the fix. (Introducing a lint/test/format gate the code currently fails is the canonical pre-write case: the gate file is a new dev-tooling deliverable but the fix is product-code edits — a new class — so confirm fix-now vs. land-non-blocking vs. land-red via `AskUserQuestion` rather than silently fixing inline.) **Verification integrity:** when a blocker verdict, or a "gate now passes" confirmation, rests on a command's exit status or output, confirm the measured signal isn't masked by a pipe, wrapper, or `|| true` before asserting it — a piped `… | tail`/`| head` reports the pipe's last-stage exit, not the command's, and `|| true` swallows the failure entirely; capture `${PIPESTATUS[0]}`/`$?` or run the command unpiped to read the real verdict. When proving a kill/failure by temporarily mutating a file (a mutation kill-proof, a forced-failure check), apply and revert the mutation surgically — an inverse edit or a patch of only the mutated lines — never `git checkout`/`git restore` on a path carrying uncommitted session work, which wipes the session's own edits along with the mutation; after reverting, confirm via `git status`/grep that the intended session changes survived.
- **Auto mode** applies when a system reminder instructs continuous operation, the user issues an explicit "just go" directive, or a harness `auto_mode` flag is set. Under auto mode: clarifying questions are minimized (gaps become named assumptions), the Step 4 batching threshold drops from 3 to 2 consecutive approvals, and small/template-structured deliverables may be written in the same turn as approval. Approach-selection and section-approval gates still hold, and the HARD-GATE is never relaxed. In an **unattended session** (the harness instructs autonomous operation and the user cannot answer mid-task), blocking questions cannot fire: the §User pre-authorization shape-confirmation `AskUserQuestion` converts to a prominently-surfaced substitution note in the triage/design — name the deliverable-class or count change and the scope basis, proceed, and let the user redirect post-hoc. The §User pre-authorization direction-confirmation exception (reversing a prior user decision) does NOT convert — defer that item with a named trigger instead of proceeding.
- **User pre-authorization.** When the request pre-authorizes a deliverable contingent on the verdict ("create a spec if warranted", "implement if changes are needed"), the HARD-GATE is satisfied by presenting the design or triage recommendation — no separate approval round-trip. A flat *unconditional* directive to produce a specific named deliverable ("create CLAUDE.md", "write the README") is pre-authorization in the same way: for a small or template-structured deliverable the HARD-GATE is satisfied by presenting the draft, which may then be written in the same turn — **unless** a genuine content or scope axis is still open (e.g. lean vs. fuller, which sections to include), in which case settle that axis first: when it is a bounded, enumerable choice, prefer posing it via `AskUserQuestion` (with `preview` fields per Step 3 §Comparing concrete artifacts) before drafting — answering it then permits the same-turn write; reserve present-then-approve for axes that aren't cleanly enumerable. The material-deliverable-shape confirmation below applies in both the contingent and the flat-directive case. Pre-authorization satisfies the HARD-GATE; it does NOT skip the Step 6 menu. The user can still redirect between presentation and write by issuing a counter-instruction. When a remaining gap would materially change the deliverable shape (count crossing a previously-stated boundary — 1↔multi, or N↔N+ when the addition is a new class — type changing class, or a new deliverable file becoming required) — or when a content fork *within a stable shape* carries materially different downstream or cross-artifact consequences (e.g. one option pre-empts or conflicts with another in-flight spec/ticket/decision, or trips a separate guard), even when the deliverable's count and class are unchanged — prefer `AskUserQuestion` to confirm before writing even under pre-authorization (in an unattended session this question converts to a surfaced substitution note — see §Auto mode). This rule takes precedence over the §Guardrails "Blocker discovery" fix-inline allowance when a Step 4/5 pre-write verification surfaces the blocker (see that bullet). **Exception:** if the recommendation would reverse a decision the user previously committed to (a `docs/triage/` decision record, a deliberately-rejected item), surface the reversal prominently and pose a direction-confirmation `AskUserQuestion` before writing — pre-authorization does not extend to silently overturning a prior user decision.
