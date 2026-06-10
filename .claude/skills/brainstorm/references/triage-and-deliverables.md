# Triage & Deliverables

Detailed rules for two SKILL.md branches: the triage recommendation (Step 3 §Triage brainstorms) and deliverable classification (Step 5). Also collects the Step 4 design-presentation carve-outs.

---

## Triage recommendation structure

Used when the brainstorm evaluates a report, finding-set, or diagnostic question and produces work items, instead of proposing approaches.

### Audit fan-out (delegated deep review)

When the triage requires deep per-surface review of code or docs (an alignment audit, a hardening pass) rather than mere location of relevant files, delegate with structure instead of improvising:

- **Slice by subsystem × governing docs** — each agent gets one code surface plus the doc-tier files that govern it, so no agent's context dilutes across the whole repo.
- **Use a read-only, review-capable agent type** — Explore-type agents locate code but don't audit it; pick an agent that reads function bodies (e.g. an architecture-review agent restricted to Read/Grep/Glob).
- **Prompt for adversarial verification** — when prior specs claim fixes on the audited surface, instruct agents to verify the claims in code, not trust them ("a passing test is only as good as what it asserts").
- **Fix the evidence format** — grep-stable named symbols (no line numbers), severity from the lineage's existing vocabulary, a why-existing-guards-miss-it line, and a fix-plus-lock sketch per finding.
- **Request a verified-holding list** alongside findings so confirmations are recorded and not re-litigated.
- **Cap per-agent output** (~900 words) and state that the final message is synthesis data, not user prose.

The SKILL.md verification rules then apply to what comes back: blocker/high and load-bearing findings are independently verified before the triage; lower-severity findings are spot-verified (or marked agent-reported) before they're embedded in a persisted deliverable. The per-finding outcome is recorded in the triage close's verification ledger (§Closing structure).

### Per-item structure

Each triage item has:

1. **verdict** — one of the types below.
2. **rationale** — 1-2 sentences naming the invariant / codebase / contract grounds.
3. a **conditional sub-field** — `modification scope` (for accept-with-modification) / `alternative path` (for reject) / `deferred_to` (for defer) / `verification source` (for refuted-by-verification). Absent for accept / already-resolved.

### Verdict types

| Verdict | Sub-field | Use when |
|---|---|---|
| `accept` | none | item warrants action as recommended |
| `accept-with-modification` | `modification scope` | item warrants action with refinements (scope-narrowed, severity-shifted, technique-substituted) |
| `reject` | `alternative path` | item declined with no positive scheduling intent; pair with what to do instead (or "none") |
| `defer` | `deferred_to` | item judged sound but routed to a follow-up deliverable; names the follow-up shape + the trigger condition for re-evaluation |
| `already-resolved` | none | re-triage case: the item was actioned between the original pass and this one; cite the resolving artifact + date |
| `refuted-by-verification` | `verification source` | the item's claimed gap or premise is disproved by codebase/contract verification at triage time; quote the file:line evidence |

The six-verdict vocabulary is closed — don't coin new verdicts. A user-elected skip ("skip the polish for now") is a `defer` whose trigger is the user's batch-scoping choice. An item whose premise is refuted but carries a valid residual best folded into another finding uses the dominant verdict plus a rationale cross-reference to the absorbing item's ID.

### Per-item identifiers

Derive from the source report's own numbering when present (`P1`, `R10`, `F-01`). When unnumbered, use `R<N>` for source-report items. Use `O<N>` for **out-of-report** findings (auditor-discovered, no presence in the source report) — always the literal `O<N>` prefix so they're unambiguous in cross-references. IDs must be stable so the user can reference them by number.

When the reference is a **recommendation document derived from a separate upstream numbered findings report** (e.g. an errata whose recommendations all elaborate an audit's `F-NN`), the direct reference is usually unnumbered: assign fresh `R<N>` to its items, and note the upstream mapping (e.g. "R1–R5 all elaborate audit F-10") in the triage lead so a reader cross-referencing the upstream report can trace the lineage.

### Grouping & out-of-report findings

Group items by verdict bucket so the user can scan by shape (all accept together, all reject together, etc.). In cross-references and the verdict field itself, use the canonical lowercase-hyphenated form (`accept-with-modification`).

Findings discovered during exploration that are NOT in the source report (adjacent pre-existing bugs, schema drift) go in a separate **out-of-report findings** sub-section AFTER the verdict buckets, keyed `O<N>` — not a new verdict bucket. But a correction that refutes a *source-report item's* premise HAS presence in the report, so it's a `refuted-by-verification` (or `already-resolved`) verdict keyed `R<N>` in the buckets. Corrections that reframe the whole triage (tied to no single item) go in the triage lead or a verification headline before the buckets. For a **commit-pinned report** (an exact-commit audit), record the pinned-vs-`HEAD` delta in that verification headline — which intervening commits landed after the pinned commit, and whether any touched the audited surface — so the user can see the findings were re-verified against current code, not just the stale pinned tree.

### No-source-report diagnostic case

When the request is a diagnostic question or exploration prompt rather than a formal report ("figure out why X", "what's happening with Z"), there's no source report to evaluate verdicts against:

- **Omit the verdict-bucket section entirely** — the verdicts are defined against source-report items.
- **Route all findings to the out-of-report sub-section**, keyed `O<N>`. The answers to the user's questions ARE the findings — emit them as `O1`, `O2`, …, not as synthetic `R<N>` items restating the questions.
- **Action-type audits** ("should anything be deleted / merged / corrected?"): the `O<N>` findings may be **grouped by candidate-action-category** (e.g. delete / merge / correct / optional) rather than listed flat, since that mirrors the audit's natural question — keep the `O<N>` keys *within* each group so items stay referenceable. These category headers are a presentation grouping, not verdicts — they don't replace the `O<N>` keys or the closed verdict vocabulary. For a micro-audit (≤~3 findings), carrying the keys inline in prose is fine instead of a separate bucket section.
- **The closing structure still applies.**
- **For an audit/health-check diagnostic** ("are there issues with X?", "is Y sound?"), add a brief **validated — no action** subsection (distinct from the `O<N>` findings) listing what was checked and found healthy. It makes the verdict's completeness legible — the user can see that an unmentioned aspect was *checked and clean*, not silently skipped — and lets a mostly-negative verdict ("no real issues") be trusted. Keep it to one line per checked aspect — for a large set (>~10 aspects), group by surface with one line per surface and the aspects comma-joined, rather than a single dense paragraph; it is not a verdict bucket and carries no `O<N>` keys. This subsection is **not exclusive to no-source-report triages**: a **source-report** triage whose verdicts rest on independent verification (reproducing the report's claims against live code) may carry the same **validated — no action / verification-confirmed** subsection to record what was checked and found to hold — distinct from a `refuted-by-verification` verdict, which fires only when a premise is disproved. An anomaly that *looks* like a defect but is intended-deferred per the phase/execution ladder (verified against `docs/2-execution/` — a later-phase deferral or an intentionally-inert slot) belongs here, not in the `O<N>` findings and not as a candidate work-item; resolve it during Step 1 exploration rather than presenting it as an open observation.
- **When the diagnostic resolves to a recommended *action* (not a set of independent work items)** — i.e. the question is "what should we do about X?" and the answer is one course of action weighed against alternatives — the `O<N>` findings carry the *answer* (the diagnosis), and the close borrows Step 3's recommendation shape in place of (or alongside) the deliverable-shape recommendation: name the recommended action upfront, then the rejected alternatives with their grounds, then any optional add-ons. This is the sanctioned blend for action-shaped diagnostics; don't force an action choice into the flat `O<N>` finding list.
- **When the diagnostic resolves to a set of independent work items** (each `O<N>` finding warrants its own spec/ticket — or doesn't), each `O<N>` MAY carry an `accept` / `reject` tag from the closed verdict vocabulary to mark which findings warrant a deliverable. This is a per-finding shorthand, distinct from the omitted verdict-bucket *grouping*; the closed vocabulary still governs (don't coin new tags), and **validated — no action** items stay separate from the tagged `O<N>` findings. It is the work-item complement to the action-shaped close above.

### Closing structure

Close every triage recommendation with:

1. **Verification ledger** — when findings came from delegated agents: per finding, `operator-verified` (you checked a cited symbol yourself) or `agent-reported` (carried on agent evidence alone — tag it as such where it lands in the persisted deliverable so the implementation stage knows to re-verify). Compact forms are fine (e.g. one line listing the `agent-reported` keys, with everything else declared operator-verified); the requirement is that every finding's status is derivable, so unverified findings cannot hide silently in a large fan-out. Omit when no subagents were used.
2. **Deliverable-shape recommendation** — one spec / N tickets / mixed batch / in-place edits, per §Deliverable classification.
3. **Named assumptions** — remaining gaps in the format `(N) X — assuming Y`.

For a multi-deliverable triage (≥2 specs or ≥3 tickets), make the finding→deliverable mapping explicit in the recommendation (either inline `R3 — <summary> → SPEC-002`, or a `deliverable → findings` map) so the user can see which accepted finding lands where at approval time.

**`AskUserQuestion` vs named-assumptions at close-out:** if a remaining gap is material-deliverable-shape (changes deliverable type / scope / count), prefer `AskUserQuestion` to settle it before proceeding — even under auto mode or pre-authorization — because a shape mismatch requires rewriting rather than refining. For content-level gaps within a stable shape, prefer `AskUserQuestion` outside auto mode; under auto mode or pre-authorization, default to named-assumptions plus the design-approval gate.

### Worked skeleton

```markdown
## Verification headline (only if a correction reframes the whole report, or the report is commit-pinned and needs a pinned-vs-HEAD delta; else omit)

## Triage verdicts

### Accept
- **R<N>** — <summary>[ → <target deliverable, for multi-deliverable triages>]. _Rationale_: <grounds>.

### Accept-with-modification
- **R<N>** — <summary>. _Modification scope_: <refinement>. _Rationale_: <grounds>.

### Defer
- **R<N>** — <summary>. _Rationale_: <reason>. _deferred_to_: <follow-up>; re-evaluate when <condition>.

### Reject
- **R<N>** — <summary>. _Alternative path_: <what to do instead, or "none">. _Rationale_: <grounds>.

### Refuted-by-verification
- **R<N>** — <summary>. _Verification source_: <file:line / grep — independently verified; an unchecked subagent claim is not sufficient>. _Rationale_: <verbatim evidence>.

## Out-of-report findings (auditor-introduced)
- **O<N>** — <description>. <Resolution: landed in <site> | flagged for follow-up>.

## Verification ledger (when agents were used)
<operator-verified: R1, R3, O1, O2…; agent-reported: O4, O5 — tagged as such in the deliverable>

## Deliverable-shape recommendation
<one spec / N tickets / mixed batch — per §Deliverable classification; finding→deliverable map for ≥2 deliverables>

## Named assumptions
(1) <unknown> — assuming <assumption>; (2) ...
```

---

## Deliverable classification

The full per-type rules behind SKILL.md Step 5's quick-triage table.

- **Inline ops/setup task or mechanical-fix batch** — small tooling/ops work or a bounded mechanical-fix batch executed inline with no persisted design artifact (repo setup, local config, a short pre-approved sequence). Skip both the `docs/plans/` design doc and the Step 6 menu; the deliverable is the in-conversation design plus a post-execution summary. The HARD-GATE still requires explicit approval of the consolidated design before executing — but under §User pre-authorization (a flat directive to perform the batch) or auto mode, presenting the consolidated plan satisfies that gate and execution may proceed in the same turn (the material-deliverable-shape check still fires first); absent pre-authorization, present-then-confirm before executing irreversible mutations (renames, deletes). The file edits/new files ARE persisted — only the design doc is elided. If the batch changes a documented convention (a renumbering, a rename, a moved path), reconcile its recording surfaces as part of the same batch — relevant memory, `README.md` / `CLAUDE.md`, the spec/ticket ledger — since those docs assert the now-stale scheme.

- **New skill design** — the deliverable is the skill file at `.claude/skills/<name>/SKILL.md`; the skill file IS the design, so skip the `docs/plans/` doc. Adjust the Step 6 menu (omit "create a spec"). In plan mode, write the plan file first; write the full SKILL.md as the first post-approval implementation step.

- **Modify existing skill file(s)** — the edits ARE the design; skip the design doc. For a merge, include the new unified file, deletion of superseded directories, and updates to any cross-references.

- **Project documentation & root instruction/config files** — edits to (or creation of) `README.md`, `docs/*.md`, or root agent-instruction/config files (`CLAUDE.md`, `AGENTS.md`) where the doc IS the deliverable; the content IS the design. The Step 6 menu may be omitted when it completes inline in the same turn. (A root instruction file matches neither the `README.md` nor `docs/*.md` glob literally but is handled identically — inline-completion, menu-skip.)

- **New dev-tooling/CI/config file** — a created tooling/CI/config file (`.github/workflows/*`, lint/format/build config) where the file IS the deliverable; the file content IS the design. Created in place; the Step 6 menu may be omitted when it completes inline in the same turn. Verify the commands/actions the file invokes (build/test/lint scripts, action versions) before writing, as for any operator-introduced premise. If the gate is a lint/test/format check, run it against the current codebase before writing — a gate that fails the existing code is a pre-write blocker (SKILL.md Guardrails §Blocker discovery): confirm fix-now vs. land-non-blocking vs. land-red via `AskUserQuestion`, since the fix is product-code edits (a new deliverable class).

- **Amend `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`** — a constitutional change. The invariants are hard rules; amending one is deliberate and on purpose. Present the amendment explicitly as a constitution change, name the invariant(s) affected and the downstream specs/features the change unblocks or invalidates, and require explicit user sign-off before writing. Skip the design doc — the amendment IS the deliverable.

- **Port external skill** — deliverable is (a) the new skill file, (b) deletion of the reference source once verified, and (c) a transformations table enumerating per-element strip/replace/preserve decisions (one row per substitution site, not per source line). The approach focuses on identifying extraneous source-repo elements and their repo-appropriate replacements. A substitution not itemized in the table is out of scope until itemized. **Co-ported dependency files**: a ported skill often depends on co-ported files from the source repo (templates, READMEs, referenced docs the skill hard-depends on); itemize each as its own strip/replace/preserve row — they typically carry the same source-repo residue as the skill itself, and a dependency file not itemized is out of scope until itemized.

- **Replaces an existing artifact** — include (a) confirmed deletion of the old artifact, (b) a check for cross-references to it (in other skills, `README.md`, memory), (c) a note of the replacement in the deliverable.

- **System spec** — deliverable is the spec under `specs/` or `docs/4-specs/`; the spec IS the design, so skip the design doc. **Section structure**: if a drafting-rules file or an existing spec in `specs/`, `docs/4-specs/`, or the archive (`archive/specs/`) establishes a convention, follow it; otherwise use the canonical section set — Problem Statement, Approach, Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions — and add a top-of-file note that the default structure was used. Do **not** adopt the foundation-pack docs' narrative house style (the `docs/NN_*.md` design-document structure) for a `specs/` file — those are design/requirements documents, not specs. The Problem Statement should capture the motivation, evidence, and key "considered X, chose Y because Z" decisions from the brainstorm; a brief Brainstorm-Context / provenance header (original request, references, final confidence + assumptions) MAY additionally prefix the spec when those wouldn't otherwise be recorded. **Spec-ID assignment**: scan `specs/`, `docs/4-specs/`, **and any archive (e.g. `archive/specs/`)** for the highest existing `NNNN_` prefix and claim the next integer (zero-padded to 4 digits, uppercase snake-case title, e.g. `0003_TITLE.md`) — completed specs move to the archive, so omitting it from the scan re-mints an already-used ID; record the package in `docs/4-specs/SPEC_LEDGER.md` per the repo's ledger-timing convention — check how the most recent sibling spec was recorded; hardening-series specs staged in `specs/` get their ledger row at acceptance/closeout, not at proposal. The Step 6 menu is mandatory (post-deliverable phase): offer reassess / decompose into tickets / implement / done.

- **Implementation tickets** — deliverable is the ticket file(s) in `tickets/`; the tickets ARE the design. If a ticket template exists, follow it; otherwise use a minimal Title / Context / Acceptance Criteria / Verification shape and note the template's absence. **Namespace**: tickets use per-initiative prefixes (illustratively `COMPILER-NNN`, `VALIDATE-NNN`), not one global sequence. **Reusing the repo's established convention takes precedence over these examples** — scan `tickets/` (and any archive, e.g. `archive/tickets/`) for existing prefixes and consult `tickets/README.md` / `tickets/_TEMPLATE.md` for the house pattern; some repos use a numeric-sequence+descriptor namespace (e.g. `0004PHA2AEPISUB-NNN`) rather than a bare initialism. In a numeric-sequence+descriptor namespace the leading `NNNN` is a repo-global initiative counter, not a per-spec tie: take the highest leading number across `tickets/` and any archive and increment it (the `-NNN` suffix still restarts at `-001` per prefix), even when the new ticket is standalone and not tied to a spec — e.g. a fresh `0007TUIFIXSEL-001` after an archive topping out at `0006`. Only for a genuinely fresh namespace, derive a short uppercase initialism from the deliverable's primary subject and start at `<PREFIX>-001`. Cite the namespace choice in the deliverable lead so the user can redirect. Ticket *creation* presents the Step 6 menu; ticket *update in place* is inline-completed — skip the menu and summarize the delta.

- **Triage producing ≥2 specs or ≥3 tickets** — additionally write `docs/triage/YYYY-MM-DD-<topic>-triage.md` summarizing the source report, accepted items (with the full path to each spec/ticket + a one-line rationale), dismissed items (one-line reason each), and identified-but-unactioned follow-ups. Keep it under ~80 lines; reference deliverables by path rather than duplicating their content. This makes the brainstorm's decisions durable without re-running it. For a single spec or fewer than 3 tickets, skip this file by default — the deliverables are sufficient history.

- **Triage analysis, all deliverables deferred** — when the brainstorm emits verdicts but produces no spec/ticket now (everything deferred) yet the user wants the verdicts persisted, write the decision record to `docs/triage/YYYY-MM-DD-<topic>-triage.md` with the full triage (source, per-item verdicts + rationale, recommended shape, named assumptions). The file IS the deliverable, so it carries full verdict content (the ≤80-line companion budget does not apply). Step 6 offers: re-invoke `brainstorm` on this file to produce the deferred deliverables / adjust a named assumption / done.

- **Design doc (default)** — when none of the above fit, write `docs/plans/YYYY-MM-DD-<topic>-design.md`, where `<topic>` is a kebab-case short name. Consolidate all approved sections into a clean document with a "Brainstorm Context" header (original request, reference file, load-bearing decisions, final confidence + assumptions). In plan mode, write to the plan file instead.

- **Research brief** — a self-contained markdown report at `reports/<topic>-research-brief.md` targeted at an external researcher/LLM whose findings feed a later design. Inline all schemas, evidence, terminology, hard constraints, and explicit research questions, since the audience has no repo access. For product-behavior topics, include a non-negotiable-constraint section naming the invariants (`INV-NNN`) the topic engages and any rejection criteria future recommendations must satisfy. Optimize for completeness over brevity. Step 6: feed the brief to the researcher / wait for findings.

**Deliverable pivot.** If the user redirects the deliverable type mid-brainstorm ("actually, make this a spec"), reclassify and adjust the flow; don't re-confirm — they told you what they want. When the request pre-authorizes a choice among types ("ticket or spec, whichever fits"), the operator may select based on scope evidence from exploration without re-prompting — cite the scope basis in the deliverable lead.

When persisting ≥3 files, track one task per file so progress is visible. Do NOT commit any deliverable — leave it for user review.

---

## Design-presentation carve-outs

Detail for the Step 4 carve-outs. Each keeps the HARD-GATE — explicit approval of the consolidated artifact (or the per-tier unit) is required before any write.

- **Small-deliverable carve-out.** When the design comprises ≤4 distinct *decisions* (user-approveable choice points where the user could meaningfully redirect — atomic facts following from a parent decision count with the parent) AND confidence is ≥85%, present the design as a single structured artifact (a transformations table, a bullet list of decisions, a short enumerated summary) approved in one turn. Permitted by default under auto mode; outside auto mode, announce the consolidation. When the count is borderline, prefer consolidating — the gate that matters is "can the user review this in one turn".

- **Template-structured-deliverable carve-out.** When the deliverable has its own canonical template (a ticket template, the default spec sections, a skill file), the template provides the section navigation; present the full draft as one artifact, and one approval covers it. Covers the common case of a single template-structured deliverable with many small atomic line-items that don't decompose into the Step 4 section list.

- **Multi-deliverable triage navigation.** For triage brainstorms producing ≥2 deliverables, apply the template-structured carve-out per deliverable — each spec/ticket is approved as a single consolidated draft. The triage approval covers all N deliverables together; no per-deliverable gate fires. (A companion `docs/triage/` file is a companion to the set, not a member of it, and doesn't count toward the ≥2 threshold.)

- **Non-plan-mode fast-track.** When confidence is ≥85%, a single approach is approved with named assumptions covering remaining gaps, and the deliverable is template-structured, the consolidated-draft approval may be collapsed with the approach/triage approval: present a delta summary in the same turn as the file write. For triage brainstorms the triage-recommendation approval transitively covers the write in any mode; for non-triage approach-selection the same-turn collapse needs auto mode *or* §User pre-authorization (the consolidated draft is the user's first look at the full surface, but pre-authorization independently satisfies the gate via the recommendation presentation — auto mode is not separately required when the request pre-authorized the deliverable). The material-deliverable-shape check (Guardrails §User pre-authorization) still fires before a same-turn write — if the shape shifts, confirm with `AskUserQuestion` first.

- **Re-emergent interview during design.** If the user asks a discovery-style question or requests enumeration of open decisions during Step 3/4 ("ask me the questions that need settling to amend SPEC-X"), conduct a constrained interview applying the Step 2 rules (one question per message, prefer multiple-choice, name uncertainty, recommend when delegated). Label questions (A, B, C) so the design presentation can cite them. The settled decisions feed the design; the HARD-GATE still holds.

- **Mid-design term-clarification.** When the user asks what a term you introduced means, answer inline with one self-contained explanation (diagram, worked example, or short prose), then continue the section flow — not a labeled-question sequence.

- **Mid-design scope-narrowing.** When the user requests reduced surface area after sections are approved (the architecture stays valid), recompute under the narrowed scope and announce the deltas before re-presenting; name the dropped elements with a concrete re-evaluation trigger each. Update the design doc in place unless the user asks for a fresh one. This is not a register shift and not a term clarification — don't mis-route it as either.

- **Plan-mode interaction.** Per-section approval is replaced by whole-plan approval via `ExitPlanMode`. Present key decisions inline (1-2 messages, grouping related sections) before writing the plan file, pausing after the first message for course corrections. The goal is conversation-level checkpoints, not per-section gates. When the approach is architecturally constrained (single viable option), the confidence announcement, approach proposal, and design presentation may fold into one message.
