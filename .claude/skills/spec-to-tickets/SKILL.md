---
name: spec-to-tickets
description: "Use when decomposing a Tracewake spec into actionable implementation tickets aligned with the foundation doc pack. Reads the spec, validates its assumptions against the codebase and doctrine, then writes one ticket per reviewable diff to tickets/<PREFIX>-NNN.md. Produces: ticket files. Mutates: only tickets/ (never specs/, docs/, or .claude/skills/)."
user-invocable: true
arguments:
  - name: spec_path
    description: "Path to the spec file (e.g., specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md or docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md)"
    required: true
  - name: namespace
    description: "Ticket namespace prefix, used as <PREFIX>-<NNN>.md (e.g., 0002PHA1KERTUI). If omitted, the skill derives one matching the repo's existing ticket-prefix convention (spec number + Phase + abbreviated title slug for phase specs, e.g. 0002PHA1KERTUI / 0003PHA1AEXETUI; spec number + title slug for non-phase certification/hardening specs, e.g. 0002TUIPROOSUR) and asks the user to confirm."
    required: false
---

# Spec to Tickets

Break a Tracewake spec into small, actionable implementation tickets a reviewer can merge one at a time, each validated against the current codebase and aligned with the foundation doc pack — `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (the constitution, `INV-001`…`INV-110` — the `## 2026 hardening invariants` block adds the truth-firewall / cognition-authority set INV-099–110) atop the architecture / execution / reference authority tiers.

<HARD-GATE>
Do NOT Write any ticket file at `tickets/<PREFIX>-<NNN>.md` until ALL of the following hold:

(a) Pre-flight has verified `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, `tickets/_TEMPLATE.md`, `tickets/README.md`, and `<spec_path>` are all readable; if any is missing the skill aborts before Step 1.

(b) Step 2 (codebase validation) has completed, and every surfaced Issue has an explicit user disposition — one of: fix-before-decomposition, defer-to-follow-up-ticket (named dependency), reject-with-rationale (route back to `/reassess-spec`), expand-scope-in-place (decompose against the wider surface the codebase requires; the spec text is not edited), or drop-as-moot (named target doesn't exist AND intent is covered by sibling deliverables or is a structural no-op). **Additionally, if the spec's own text prohibits decomposition** (an "Implementation posture: do not decompose", "not … into tickets", or "no … ticket breakdown" directive — detected at Step 1), that is a blocking conflict, not a codebase Issue: the skill must obtain an explicit user override before Step 2. It is NOT auto-resolvable and is exempt from the clause (c) carve-out.

(c) Step 4 has emitted the decomposition summary table in chat (numbered tickets with Title, Scope, Effort, Deps, INV, Notes) AND the user has explicitly approved it, OR the auto-mode carve-out has fired (auto mode active AND Step 2 surfaced no Issues AND no `/reassess-spec` findings were deferred by the user). The carve-out never fires when the spec prohibits its own decomposition (clause (b)); that conflict always requires explicit user override regardless of mode.

(d) Every `Deps` reference resolves to a ticket produced in this run, or to a pre-existing `tickets/` / `specs/` / `docs/4-specs/` path verified at Pre-flight or at Step 4's cross-spec Deps check before approval.

This gate is authoritative under auto mode and any autonomous-execution context. Invoking the skill does not constitute approval of the decomposition.
</HARD-GATE>

## Process Flow

```
Pre-flight: verify required files readable; derive + confirm <namespace> if omitted
       |
       v
Step 1: mandatory reads (spec, tickets/_TEMPLATE.md, tickets/README.md, docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md)
       |
       v
Step 2: codebase validation (load references/codebase-validation.md); surface Issues; await per-Issue disposition
       |
       v
Step 3: decompose the spec (load references/decomposition-patterns.md)
       |
       v
Step 4: present decomposition summary table; await user approval
       |
       +-- [HARD-GATE fires here]
       |
       v
Step 5: batched ticket writes (one or a few messages, parallel Write calls, one per ticket)
       |
       v
Step 6: final summary (cross-ticket Deps check, deliverable coverage, dependency graph, suggested order). Do NOT commit.
```

## Inputs / Output

**Input**: `spec_path` (required); `namespace` (optional, derived + confirmed if omitted). Plan-mode and worktree-root resolution are auto-detected.

**Output**:
- **Ticket files at `tickets/<PREFIX>-<NNN>.md`** — one per reviewable diff, each following `tickets/_TEMPLATE.md` exactly.
- **Decomposition summary table** — emitted in chat at Step 4 before any Write.
- **Final summary** — emitted at Step 6 (cross-ticket Deps verification, deliverable coverage mapping, dependency graph, suggested implementation order).

This skill emits markdown tickets only. It operates at pipeline scope: it produces tickets that feed implementation, so foundation-doctrine alignment applies even though it writes no simulation content (events, content/domain-packs, fixtures) or engine code itself.

## Prerequisites

Before acting, this skill MUST read:

- `<spec_path>` — the target spec, entire contents (Step 1).
- `tickets/_TEMPLATE.md` — the canonical ticket structure; every ticket must follow it exactly (Step 1).
- `tickets/README.md` — the ticket authoring contract (Step 1).
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the non-negotiable design contract. Skip only if read earlier this session and unmodified (Step 1).
- Every file path, module/crate, skill directory, type, schema field, and spec reference extracted from the spec — read on demand at Step 2.

Reading scope: anything under `specs/`, `docs/4-specs/`, `.claude/skills/`, `docs/`, `reports/`, `tickets/`, and the crate/module tree (`crates/`). Tracewake is partially implemented: completed phases have a landed Rust workspace (`crates/tracewake-{core,content,tui}` + a root `Cargo.toml`), while specs for future phases may still be paper. Do not assume a fixed repo state — for current implementation status consult `docs/4-specs/SPEC_LEDGER.md` (landed phases) and `find crates -name '*.rs'`. Validate any surface that already exists against code, and reason about not-yet-built surfaces against doctrine. This skill does not author simulation content (events, content/domain-packs, fixtures) or engine code; it emits tickets.

## Reference Files

- **Step 2** — `references/codebase-validation.md`
- **Step 3** — `references/decomposition-patterns.md`

Load each before the corresponding step. Loading both right after Step 1 is the simplest path; on-demand is also fine.

## Worktree & Plan-Mode Awareness

Inside a git worktree, ALL paths (reads, writes, globs, greps) resolve from the worktree root. If plan mode is active, present the decomposition in the plan file and call `ExitPlanMode` in lieu of the Step 4 chat-table approval; write tickets only after approval.

## Pre-flight Check

Before Step 1, verify:
1. `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` exists and is readable.
2. `tickets/_TEMPLATE.md` exists and is readable.
3. `tickets/README.md` exists and is readable.
4. `<spec_path>` exists and is readable. If it is a glob (e.g. `specs/0002_*`), resolve first: exactly one match → use it (note the resolution); zero or many → abort or ask to disambiguate. Specs live under `specs/` and `docs/4-specs/`.
5. `<namespace>` is provided, OR derive one matching the repo's existing ticket-prefix convention — for a phase spec, spec number + Phase + abbreviated title slug (`0002PHA1KERTUI`, `0003PHA1AEXETUI`); e.g. `specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` → `0002PHA1KERTUI`. For a non-phase certification/hardening spec (no `PHASE_N` token in the title — common in Tracewake, e.g. a `P0-CERT`-keyed spec), omit the Phase token: spec number + abbreviated title slug; e.g. `specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` → `0002TUIPROOSUR`. Ask the user to confirm or override before Step 1.

If any of checks 1–4 fails, abort with a clear missing-file error. If check 5's Phase/title parsing is ambiguous, ask the user for the namespace directly.

## Step 1: Mandatory Reads

Read ALL of: the spec file (entire), `tickets/_TEMPLATE.md`, `tickets/README.md`, and `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (skip the last only if read earlier this session and unmodified).

**House-style calibration (optional)**: when prior tickets already exist under `tickets/`, skim one as a depth/voice exemplar — the citation density, Verification-Layers granularity, and `INV-NNN`-referencing the repo expects. Structure is still governed by `tickets/_TEMPLATE.md`; the exemplar only calibrates detail level.

Parse the spec's metadata (Spec ID, Phase, Status if present, Date, authority order, source-authority summary, evidence ledger) and its sections. Tracewake specs do not share one fixed template; the section set varies (e.g. `specs/0002_*` uses Evidence ledger / Source authority summary / Purpose / Scope / Non-goals / Binding invariants / Binding architecture constraints / Relationship to prior spec / Workspace shape / Determinism contract / Event log and replay contract / Entity-component model …). Take the spec's own section set as authoritative, and use sibling specs (`specs/0002_*`, `docs/4-specs/0001_*`) and `docs/4-specs/README.md` as the convention exemplars.

**Non-standard deliverables**: if the spec uses named sections or a numbered scope list instead of a deliverables table, treat each distinct implementation item, required-area entry, scope line, or numbered deliverable as a deliverable for decomposition.

**Anti-decomposition posture check**: scan the parsed spec (metadata posture lines, Purpose, Scope, Non-goals, and any closing "deliverables required" note) for a directive that prohibits ticketizing this spec — e.g. `Implementation posture: do not decompose into tickets`, `Ticket decomposition` listed as a non-goal, or `No … ticket breakdown is part of this specification`. On a hit, STOP before Step 2 and surface it as a blocking conflict in the 1-problem / 3-options / 1-recommendation format (options typically: override-and-decompose, hold, or `/reassess-spec` first to flip the posture). Proceed only on explicit user override. This conflict is exempt from the auto-mode carve-out (HARD-GATE clauses (b)/(c)) — auto mode must not silently decompose a spec that forbids it. The skill never edits the spec to remove the directive (No spec edits); the contradiction between an unflipped directive and the produced tickets is noted in the Step 6 summary.

## Step 2: Codebase Validation

**Load `references/codebase-validation.md`.** Validate the spec's assumptions against the current codebase and doctrine, surface Issues, and obtain a per-Issue disposition before Step 3. A spec that was reassessed via `/reassess-spec` earlier this session with all findings resolved qualifies for the abbreviated spot-check path documented in the reference.

## Step 3: Decompose the Spec

**Load `references/decomposition-patterns.md`.** Identify discrete work units — each a reviewable diff — map dependencies into each ticket's `Deps`, order by dependency graph and criticality, and ensure every spec deliverable is covered (no silent skipping). The reference documents the deliverable-coverage categories, the merge/split rules, and the recurring ticket-shape patterns (capstone integration ticket, cross-cutting docs ticket).

## Step 4: Present Summary for Approval

Before writing any ticket files, present a numbered summary table:

| # | Ticket ID | Title | Scope | Effort | Deps | INV | Notes |
|---|-----------|-------|-------|--------|------|-----|-------|
| 1 | <NS>-001  | …     | <5-10 word scope> | Small  | None | — | — |
| 2 | <NS>-002  | …     | <5-10 word scope> | Medium | 001  | INV-018 | shared file set |

Column roles: **Title** matches the ticket's first line; **Scope** is the deliverable mapping (`D1+D6`) or acceptance surface — must not duplicate the Title; **Effort** Small/Medium/Large; **Deps** other tickets in this batch or pre-existing tickets/specs (state once if all independent); **INV** a constitutional invariant only when notable (e.g. `INV-018` deterministic replay, `INV-024` no telepathy, `INV-008` kernel authority), `—` otherwise; **Notes** merged/split deliverables, shared files, multi-dependency validation tickets.

**Cross-spec Deps verification (before HARD-GATE fires)**: run `test -f` (or equivalent) on every cross-spec `Deps` path introduced during Step 3 that was not verified at Pre-flight (typically `specs/<sibling>.md`, `docs/4-specs/<sibling>.md`, or `tickets/<PREFIX>-NNN.md` from a prior batch). Abort with a missing-Deps error if any fails. Cite the result alongside the table (e.g. `Cross-spec Deps verification: N/A — all Deps resolve to tickets produced in this run`).

**Intra-batch create-then-modify Deps pre-check (before presenting the table)**: for every planned `(new)` file, confirm that each sibling ticket whose Files-to-Touch will `(modify)` that file lists the creator ticket in its `Deps` — and that any ticket consuming a module/symbol another ticket introduces depends on that producer. Bake these structural Deps into the table BEFORE approval, so the approved Deps already reflect file-creation and producer/consumer ordering. The same create-then-modify rule is enforced again at Step 5's pre-write existence check, but surfacing it here prevents an approved-but-incomplete Deps set that composition then has to expand (see `references/decomposition-patterns.md` §Intra-batch create-then-modify chains). Also count the distinct tickets that will `(modify)` each shared file — a pre-existing central test file, a shared registry, or a registration hub — and flag any file touched by ≥3 mutually-independent siblings in the table Notes as an N-way mechanical-merge hub (see `references/decomposition-patterns.md` §Shared-file merge hubs), so the coordination cost is visible at approval rather than discovered at Step 6.

**Wait for user approval or adjustments.** Do not write files until the user confirms. **Auto-mode / no-stopping carve-out**: when auto mode (or an in-session "work without stopping" directive) is active AND Step 2 surfaced no Issues AND no `/reassess-spec` findings were deferred AND the spec carries no anti-decomposition directive (Step 1 check), auto-approve and proceed; announce it inline and cite the directive. Any open Issue, deferred finding, or unresolved anti-decomposition conflict holds the wait-gate per HARD-GATE clause (c). When every Issue carries an explicit recommended disposition under a no-stopping directive, the operator MAY proceed by applying the named dispositions, citing each before the writes; the user can redirect.

## Step 5: Batched Ticket Writes

**Post-approval refinement (mechanical only)**: while composing, you MAY apply a *mechanical tightening* to the approved Step-4 table without re-approval — specifically, removing a `Deps` entry that composition shows is unnecessary, adding a structurally-required `Deps` entry that composition reveals (a create-then-modify dependency on the file's creator, or a producer/consumer dependency on a ticket that introduces a module/symbol this one uses), or relocating a sub-feature between already-approved sibling tickets of the same deliverable — provided it adds no ticket, creates no new file, crosses no deliverable boundary, and does not change the ticket count. Disclose every such tightening in the Step 6 summary (original table entry → applied change). Anything beyond that — adding/removing a ticket, changing the count, moving work across deliverables, or introducing a new file/deliverable — must round-trip to the user for re-approval per HARD-GATE clause (c).

**Pre-write rehearsal (mandatory)**: in the turn immediately before the writes, state the exact number of Write calls the next response will contain. **On a cold session, rehearse 1 Write/turn — 1 is always safe — and ratchet up after any successful write turn strictly one step at a time (1 → 2 → 3, never 1 → 3); the first ratchet — following the first successful single Write, not a prior multi-Write success (which cannot exist before the first multi-Write attempt) — goes to 2, not 3, since the 1 → 3 jump is the most divergence-prone step.** The per-batch ceiling is **≤3 parallel Writes** (a maximum, not a target); "batching" means emitting N Write tool calls in a *single* assistant message, not N across separate turns. The cap ratchets up only by demonstrated success, one step at a time (1 → 2 → 3), so a cold-session decomposition starts at 1 Write/turn and compresses into fewer batches only once a larger batch has succeeded; fewer batches is a warm-session optimization, never a cold-session target. You MAY state the full batch plan once upfront (e.g. `5 batches: [001,002,003], [004,005,006], …`); each subsequent write turn then needs only a one-line restatement of the current batch, not a fresh standalone rehearsal turn. The one-line restatement MAY share the turn with that batch's Write calls — a restatement sentence plus the N parallel Writes in a single turn is compliant; what is forbidden is emitting more than N Writes in the turn, or splitting the N across multiple turns. If the emitted count diverges from the rehearsal, the next turn is a zero-Write acknowledgment that restates the remaining count and re-batches at ≤ the last successfully-emitted count — do not emit a single catch-up Write "to keep momentum." **Divergence stabilization**: after two or more divergences in a run, stop attempting ratchet-ups and hold the batch size at 1 for the remaining tickets — each failed ratchet-up costs another zero-Write turn, so a reliable one-Write-per-turn cadence finishes faster than chasing a larger batch. More generally, rehearse the number you are confident you will emit (1 is always safe): throughput is a warm-session optimization, never a cold-session target, so when emission reliability is uncertain, rehearse 1.

**Pre-write existence checks** (same rehearsal turn): for every `(modify)` Files-to-Touch entry across the composed tickets, run `test -f` against the working tree; correct or reclassify any path that doesn't resolve. A `(modify)` entry pointing to a file another ticket creates `(new)` in this batch is valid only when the modifying ticket declares `Deps:` on the creator (per `references/decomposition-patterns.md` §Intra-batch create-then-modify chains). For every command in a ticket's Test Plan, confirm it resolves against the repo. Enumerate `(modify)` entries individually, not as a collapsed "all new" claim. **Discovered-set / wildcard modify-targets**: when the exact set of files a ticket will modify is genuinely unknowable until implementation, do NOT write a bare glob that `test -f` cannot validate — either name the candidate files explicitly with an "as surfaced" qualifier, or mark the entry as an implementation-discovered set and `test -f` its parent directory instead, so the existence check stays honest about what is verified versus deferred. **Symbol spot-grep**: for every type, function, or module a composed ticket names (in Files-to-Touch, What-to-Change, or Assumption Reassessment) that was NOT already verified against the codebase in the spec or its in-session reassessment, `grep` it before the write — confirm the symbol exists at the cited location and add its host module to that ticket's Files-to-Touch, or correct the name. Decomposition-introduced symbols (a type named only in the source audit/spec prose, not yet grepped) are the common gap, since the `test -f` file checks and the Abbreviated Spot-Check Path validate paths and spec references but not symbols first introduced at decomposition time; an unverified symbol reaching the Step-4 table or a ticket is exactly the spec→ticket staleness this skill's guardrails forbid. Also run a **section-presence self-check** on every composed ticket before emitting the writes — assert each `_TEMPLATE.md` `## ` header (Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Test Plan) is present in the draft; a missing section (e.g. an omitted `## Verification Layers`) is cheaper to fix pre-write than to catch at Step 6.

**Flow**: compose each batch's tickets in full before emitting that batch's Write calls (parallel Write calls, one per ticket), in as few batched messages as the current tested ceiling allows. "Compose first" means do NOT alternate compose → Write *within* a batch — it does NOT mean composing all N tickets before the first Write; on a cold session, where the cadence forces multiple write-turns, the workable reading is compose-batch-then-write-batch across turns. The batch *count* is governed by the cadence above, not a fixed target: a cold session starts at 1 Write/turn (so 10+ tickets take 10+ turns the first time) and compresses into fewer, larger batches only once a multi-Write batch has succeeded.

For each approved ticket, compose its full content following `tickets/_TEMPLATE.md` exactly — every required field/section present (Status, Priority, Effort, Engine Changes, Deps, Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Test Plan). For the **Assumption Reassessment** menu (per `_TEMPLATE.md`): items 1–3 are always required; for items 4+ **Select** the menu items matching this ticket's scope, **Rewrite** each selected item's number to its position in the surviving list (starting at 4), and **Verify** the final list reads `1, 2, 3, 4, …` with no gaps (a list like `1, 2, 3, 6` means the rewrite step was skipped). The conditional menu items are: item 4 (a constitutional invariant `INV-NNN` or Validation Rule motivates the ticket — restate it), item 5 (the ticket touches fail-closed validation, actor-knowledge filtering, or deterministic-replay surfaces — name the enforcement surface and confirm no epistemic-leakage / replay-determinism weakening), item 6 (extends an existing schema — name it, its consumers, and additive-vs-breaking), item 7 (renames/removes a skill, validation rule, doc-governed contract, or schema field — grep blast radius), item 8 (adjacent contradictions classification), item 9 (mismatch + correction). **Items 5 and 6 frequently co-occur and must be keyed separately, not merged**: an `EventKind` / event-record extension, a view-model-projection extension, or a content-schema extension is *also* a deterministic-replay / actor-knowledge / fail-closed-validation surface, so such a ticket carries BOTH item 6 (name the schema, its consumers, additive-vs-breaking) AND item 5 (leakage/determinism on that surface) as distinct numbered items — folding the additive-vs-breaking analysis into item 5's prose silently drops item 6 and fails the Step 6 applicability check. **Substrate-only tickets**: when a ticket builds the *inputs* to an invariant enforcement surface a later phase — or a later ticket in this same decomposition — will implement (e.g. a schema/contract that feeds future fail-closed validation, deterministic replay/hash, or the no-leak / actor-knowledge firewall — but no validator exists yet), item 5 still applies: satisfy it by naming the deferred enforcement surface and confirming the data-model change introduces no leakage or nondeterminism path the later surface would have to undo, citing the phase or sibling ticket that will enforce it. In a greenfield foundation batch this is the common case, not the exception: because the replay, validation, and no-leak enforcement surfaces are themselves separate tickets in the batch, most foundation tickets are substrate-only and cite a sibling ticket (not a later phase) as the enforcer. Every ticket modifying existing behavior must cite the change rationale in Assumption Reassessment (no silent retcon — `tickets/README.md` change-rationale requirement; a durable architecture-changing decision is gated by a foundational-doc amendment, see `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`).

After the batch returns, verify every ticket file exists; retry any failed Write before Step 6. If a system-reminder shows a ticket was externally edited (e.g. a linter hook), treat the edit as authoritative and re-verify sibling references against the edited content before the final summary.

## Step 6: Final Summary

After writing all files:

1. **Cross-ticket dependency consistency**: for each `Deps`, confirm the depended-on ticket actually produces what the dependent needs; `test -f` every `Deps` path at emission time. If a `(modify)` Files-to-Touch entry names a file a sibling creates `(new)` in this batch without a declared `Deps` on the creator, flag it.
2. **Template fidelity**: confirm every required section is present and that each ticket's Assumption Reassessment uses gapless sequential numbering starting at 1. Section-presence check (run per ticket): `for s in "## Problem" "## Assumption Reassessment" "## Architecture Check" "## Verification Layers" "## What to Change" "## Files to Touch" "## Out of Scope" "## Acceptance Criteria" "## Test Plan"; do grep -qF "$s" tickets/<PREFIX>-NNN.md || echo "MISSING $s in <PREFIX>-NNN"; done` — must print nothing. Numbering check: `awk '/^## Assumption Reassessment/,/^## Architecture Check/' tickets/<PREFIX>-NNN.md | grep -oE '^[0-9]+'` should be strictly sequential. Also confirm each applicable conditional menu item is present — a constitutional invariant `INV-NNN` / Validation Rule motivated → item 4; a fail-closed-validation / actor-knowledge-filtering / deterministic-replay surface touched (including substrate that feeds a deferred enforcement surface — see Step 5) → item 5; an existing schema extended → item 6; a skill / validation rule / doc-governed contract / schema field renamed or removed → item 7. Applicability heuristic (run per ticket, separate from the numbering check — the gapless-numbering grep passes `1 2 3 4 5` without catching a missing item 6): if a ticket's Files-to-Touch `(modify)`s `events/envelope.rs`, a `*schema*` file, a `view_models.rs` / view-model projection, or a state record, grep its Assumption Reassessment for an additive-vs-breaking statement — absent ⇒ item 6 is missing; if it `(modify)`s a validation / replay / possession / actor-knowledge surface, confirm item 5 is present.
3. **Deliverable coverage mapping**: list each spec deliverable and the ticket(s) covering it (`D1→001`, `D3→003+004` for a split), including the exempt categories from `references/decomposition-patterns.md`. Flag any uncovered deliverable. For each **distributed or merged** deliverable, additionally enumerate the spec-named files/symbols it covers and confirm each appears in some ticket's Files to Touch — deliverable-granularity coverage can read green while a constituent file-level surface is owned by no ticket (a silent file-level skip). A named subsystem that requires both a selection-layer and a definition-layer change names two surfaces; a ticket owning only one leaves the other uncovered.
4. List: all ticket files created, the dependency graph, the suggested implementation order, any **deferred `/reassess-spec` findings** ("may warrant separate tickets"), any **cross-spec follow-ups** surfaced by the spec's Risks / Assumptions section or discovered during decomposition, and any **post-approval mechanical refinements** applied during Step 5 (per §Step 5 Post-approval refinement — original table entry → applied change).
5. **Shared-file overlaps**: enumerate files that ≥2 mutually-independent tickets each modify — tickets with no `Deps` on each other, even when they share a common upstream `Deps` (parallel siblings off one foundation ticket still merge-conflict on a shared file), and whether the shared file is pre-existing or created in-batch by that foundation ticket — so implementers coordinate mechanical merges.

Do NOT commit. Leave files for user review.

## Guardrails

- **Foundation doctrine is authoritative**: never approve a decomposition that violates a constitutional invariant (`INV-NNN`) or breaks a Phase acceptance gate — flag it as a CRITICAL Issue at Step 2 and await disposition. The doc-tier authority order (foundation → architecture → execution → reference → analysis → spec package, per `docs/README.md`) is binding.
- **Template fidelity**: every ticket uses `tickets/_TEMPLATE.md` exactly — no ad-hoc sections, no missing fields, no "simplified" variants. Template evolution is a separate spec.
- **Ticket fidelity**: never silently skip a deliverable. If one seems wrong, use the 1-problem / 3-options / 1-recommendation format and ask.
- **Codebase truth**: file paths, module names, types, and schema references in tickets must be validated against the actual codebase (or doctrine, until code lands), not assumed from the spec. Stale references propagated spec → ticket are a skill failure.
- **Reviewable size**: each ticket should be reviewable as a single diff. When in doubt, split. **Atomic-cutover exception**: when a spec's own thesis forbids incremental landing (an anti-incremental / atomic-flip requirement — e.g. "remove the bypass, do not hide it behind a wrapper"), a deliberately-large single-diff ticket that performs the whole cutover at once is correct, because a split that merged a piece while leaving the old path live would violate the spec's integrity. Override the split default with a rationale in the ticket's Architecture Check and flag the deliberate size in the Step 4 table Notes (see `references/decomposition-patterns.md` §Sizing rules).
- **Respect the spec's decomposition posture**: if the spec prohibits its own decomposition (Step 1 anti-decomposition check), never proceed without an explicit user override — even under auto mode. The override does not authorize editing the spec; the unresolved directive-vs-tickets contradiction is recorded in the Step 6 summary and routed to `/reassess-spec` if the user wants the spec made self-consistent.
- **Explicit dependencies**: declare inter-ticket ordering in `Deps`; never leave it implicit. Every `Deps` entry resolves to a ticket produced this run or a verified pre-existing path.
- **No spec edits**: this skill never edits the source spec. If decomposition reveals a spec defect, flag it as an Issue and route the fix to `/reassess-spec`.
- **Worktree discipline**: inside a worktree, all paths resolve from the worktree root.
- **Archival**: follow `docs/archival-workflow.md` as the canonical archival process when a ticket supersedes or retires prior work.
- **Do not `git commit`**: writes land in the working tree; the user reviews and commits.

## Foundation Alignment

| Principle | Step | Mechanism |
|-----------|------|-----------|
| Kernel authority (INV-008, INV-042; arch 01/10) | Step 2 | Deliverables moving setup, legality, validation, state transitions, event application, scoring, RNG, semantic effects, view projection, replay/hash, or serialization out of the authoritative kernel — or letting a view/TUI/the LLM decide world legality — are flagged. |
| Kernel boundary + dependency direction (arch 01) | Step 2 | Deliverables introducing a domain/mechanic noun into the kernel, or inverting the doctrine-mandated dependency direction, trip a boundary-failure Issue. |
| Content is possibility, not script (no-scripting; foundation 09; arch 04) | Step 2 | Deliverables introducing content/fixtures with behavior-looking fields (selectors, branches, triggers, authored outcome chains, director logic) are flagged; an authored script is a CRITICAL Issue. |
| Universal acceptance invariants (foundation 02/12; arch 13) | Step 2 | Deliverables proposing validation must stay deterministic, fail-closed, and blocking and distinguish warnings from blockers; deliverables touching view models, possession, traces, debug views, or replay exports must keep hidden information non-leaking (INV-024, INV-006); replay/hash/serialization must stay deterministic (INV-017/018/019). Deviations are flagged. |
| Phase acceptance gates (execution 02; foundation 12) | Step 2 | Any acceptance gate the decomposition would break (no-human run, deterministic replay, TUI playability, why-not, missing-property proof) is a CRITICAL Issue. |
| Foundational-doc amendment; no silent retcon | Step 5 | Every ticket modifying existing behavior cites the change rationale in Assumption Reassessment (`tickets/README.md`); an architecture-changing decision (replay/hash/snapshot semantics, visibility/actor-knowledge contracts, kernel vocabulary, no-scripting boundary, LLM boundary) must carry a foundational-doc amendment first (`docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`). |

## Final Rule

A decomposition is not complete until every spec deliverable maps to a ticket OR to an explicit non-goal OR to a documented exempt category, every `Deps` resolves to a real target, every ticket's Files to Touch matches the current codebase (or doctrine, until code lands), and every doctrine-impacting deliverable has been validated against `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and the foundation doc pack before its ticket was written.
