---
name: reassess-spec
description: "Use when preparing a Tracewake spec for implementation. Reassesses a spec under specs/ or docs/4-specs/ against the codebase (docs/, .claude/skills/, sibling specs) and the foundation doc pack (docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md and the higher-authority tiers); identifies issues/improvements/additions, presents findings for approval, then writes the updated spec. Produces: findings report + updated spec file. Mutates: the target spec file on user approval."
user-invocable: true
arguments:
  - name: spec_path
    description: "Path to the spec file (e.g., specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md or docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md)"
    required: true
---

# Reassess Spec

Reassess a Tracewake spec against the codebase and the foundation doc pack — `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (the constitution) plus the higher-authority architecture, execution, and reference tiers. Validates assumptions, identifies issues / improvements / additions, presents findings for approval, then writes the updated spec.

<HARD-GATE>
Do NOT Write or Edit the spec file until:
(a) Step 6 findings have been presented and the user has responded — either explicit per-finding disposition (fix / defer / reject), OR no explicit objection to a finding (silence on a finding while answering Questions counts as approval; an explicit objection re-opens that finding and requires re-presenting the corrected recommendation first);
(b) Step 7's pre-apply verification table has been emitted in chat, one check + result row per finding, with any detected mismatch reclassified and — for recommendation-changing mismatches — re-presented for fresh approval;
(c) any open Questions from Step 6 have been answered.

This gate holds under auto mode and any autonomous-execution context. Auto-mode carve-out: when Step 6 findings contain no Issues (CRITICAL/HIGH or invariant hard-fails) and no open Questions, Step 7 may proceed without fresh approval, but the pre-apply verification table MUST still be emitted before any Write/Edit.
</HARD-GATE>

## Invocation

```
/reassess-spec <spec-path> [inline user hint]
```

**Argument** (required): `<spec-path>` — path to the spec file under `specs/` or `docs/4-specs/`. If missing, ask for it before proceeding.

**Glob resolution**: if the argument contains wildcards, resolve via `ls`/`find`; proceed if exactly one match (note the resolution inline), disambiguate if many, stop with an error if none.

**Inline user hint (optional)**: text accompanying the path — a parenthetical, post-dash note, or follow-on message (e.g. `specs/0002_PHASE_1_*.md (Note: I'm worried the determinism contract is under-specified)`) — is an audit-lens constraint. It shapes severity assignment at Step 5 and may reframe Questions at Step 6; it is NOT a second path argument and does NOT override foundation-doctrine alignment or approved recommendations. When a hint materially shaped a finding's classification, cite it in the Step 6 presentation. A hint that would force a doctrine hard-fail (a constitutional-invariant violation or a Phase acceptance-gate violation) is flagged as a CRITICAL Issue, not applied.

## Process Flow

```
Pre-Process: classify the spec (a / b / c / d) + hybrid detection
       |
       v
Step 1: mandatory reads (spec file + docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md)
       |
       v
Step 2: extract references (file paths, types, schema fields, modules, deps, source docs)
       |
       v
Step 3: codebase validation (load references/codebase-validation.md)
       |
       v
Step 4: foundation alignment + constitutional-invariant / acceptance-gate check (load references/foundation-alignment.md)
       |
       v
Steps 5-6: classify findings + present to user (load references/findings-and-questions.md)
       |
       v   [user approval gate — HARD-GATE fires here]
       v
Step 7: pre-apply verification table -> write updated spec (load references/spec-writing-rules.md)
       |
       v
Step 8: final summary + suggested next step
```

## Reference Files

Five reference files, each loaded with the Read tool before its step:

- **Step 3** — `references/codebase-validation.md`
- **Step 4** — `references/foundation-alignment.md`
- **Steps 5-6** — `references/findings-and-questions.md`
- **Step 7** — `references/spec-writing-rules.md`
- **Plan mode** — `references/plan-mode.md`, loaded at entry when plan mode is active.

Load each before the corresponding work begins. Loading all of them in one parallel batch right after Step 1 is the simplest path; on-demand loading per step is also fine. Steps 1, 2, and 8 need no reference file.

## Inputs / Output

**Input**: `spec_path` (required). Plan-mode and worktree-root resolution are auto-detected.

**Output**:
- **Findings report** — presented in chat at Step 6 (Issues / Improvements / Additions, severity-ranked; open Questions; optional Substantial Redesign Flag).
- **Pre-apply verification table** — emitted in chat at Step 7 before any Write/Edit.
- **Updated spec at `<spec_path>`** — edited in place on approval. For classification (d): Status flipped to `Done`/`Accepted-and-landed`, Outcome/Acceptance-evidence populated, and the `docs/4-specs/SPEC_LEDGER.md` entry updated when present.
- **Post-apply confirmation** — emitted at Step 8 (grep-proofs that eliminated references are gone and corrected ones resolve).

## Prerequisites

Before acting, this skill MUST read:

- `<spec_path>` — the target spec, entire contents.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — the constitution (`INV-NNN`). Skip only if read earlier this session and unmodified.
- Every file path, skill directory, and sibling-spec reference extracted at Step 2 — read as part of Step 3.

Reading scope: anything under `specs/`, `docs/4-specs/`, `.claude/skills/`, `docs/`, `reports/`, and — once a Rust workspace lands — the crate/module tree. Tracewake is currently a paper-spec + doctrine repo with no code tree yet; this skill reasons about specs that plan future implementation, against current doctrine.

## Worktree & Plan-Mode Awareness

If working inside a git worktree, ALL paths (reads, writes, globs, greps) resolve from the worktree root. If plan mode is active, load `references/plan-mode.md` at entry.

## Pre-Process: Spec Classification

Classify the spec into exactly one of four classes. Classification drives which Step 3 substeps apply (see the substep table in `references/codebase-validation.md`).

- **(a) New component** — introduces a new doc-governed contract, a new fixture-contract surface, a new module/crate boundary, a new skill (new `.claude/skills/<name>/`), a new validator, a new tool, or a new spec package. Full Step 3 checklist applies.
- **(b) Extension** — extends an existing spec, contract, schema, fixture set, module boundary, skill, or validator without introducing a new one. Most substeps apply; skill-structure (3.5) applies only when a SKILL.md changes structurally; doctrine-contract fidelity (3.8) applies only when the deliverable touches kernel-authority / acceptance-invariant / visibility / determinism-replay / language-boundary semantics. **Removal/teardown specs** (deliverable is *deleting* a field, validator, fixture, module, or symbol) classify as (b) with 3.6 downstream-consumer analysis load-bearing — an un-removed consumer is a correctness break, not mere drift.
- **(c) Refactor** — structural restructuring (of a spec or, later, of code) with no behavioral change. Substeps 3.0–3.4 apply; skip consumer/fidelity/completeness substeps unless boundaries or SKILL.md content move. Focus on symbol/section existence, count accuracy, blast radius.
- **(d) Retroactive** — validation concludes (via Step 3 evidence) that all deliverables already landed. **Not pre-selected** — activates only when every deliverable verifies as implemented. A user hint ("I think this phase already shipped") is a soft signal; only Step 3 evidence confirms (d). Step 7 switches to flipping Status to `Done` + populating Acceptance evidence (see the retroactive branch in `references/spec-writing-rules.md`).

**Per-deliverable already-landed**: when one deliverable (or one Work-item) shipped while others remain pending, the spec stays (a)/(b)/(c) — reframe just that deliverable as historical, cite the delivering commit/spec, and route residual sub-tasks to a deferred note. Do not flip the whole spec to (d).

**Removal target that never existed**: when a spec tasks removing a symbol that returns zero matches AND was never created, there is nothing to delete — classify as a stale-premise Issue (HIGH when restated across multiple sections), drop the removal framing, and where the symbol name still appears in exit/acceptance criteria, keep it as a trivially-passing absence guard.

**Hybrid specs**: apply the union of applicable substeps, using the most rigorous classification's checklist for shared substeps.

**Re-reassessment shortcut**: if the same spec was reassessed earlier this session and not externally modified, Steps 2–3 may scope to only the references affected by the triggering change. Step 1 still applies.

## Step 1: Mandatory Reads

Read both before any analysis:

1. **The spec file** (entire).
2. **`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`** — skip only if read earlier this session and unmodified.

Parse the spec's metadata (Spec ID, Phase, Status if present, Date, authority order, source-authority summary, evidence ledger) and its sections. Tracewake specs do not share one fixed template; the section set varies (e.g. `specs/0002_*` uses Evidence ledger / Source authority summary / Purpose / Scope / Non-goals / Binding invariants / Binding architecture constraints / Relationship to prior spec / Workspace shape / Determinism contract / Event log and replay contract / Entity-component model …). Take the spec's own section set as authoritative, and use sibling specs (`specs/0002_*`, `docs/4-specs/0001_*`) and `docs/4-specs/README.md` as the convention exemplars.

**Non-standard sections**: treat each distinct implementation item, required-area entry, scope line, or numbered deliverable as a deliverable for validation purposes, regardless of the heading it sits under.

## Step 2: Extract References

Extract every concrete codebase / doctrine reference from the spec:

- **File paths / target tree** — both existing (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`, `docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md`, `docs/4-specs/0001_*`) and proposed (the recommended Rust workspace/module paths the spec names for future implementation).
- **Type / record / schema / contract names** — entity kinds, components/records, event kinds and envelope fields, claim/proposition families, view-model fields, checksum/hash fields, fixture names, serialization-boundary fields, content-manifest entries.
- **Module / function / command / tool names** — recommended core/content/TUI modules, CLI tools (`simulate`, `replay-check`, …), and any engine symbols the spec names.
- **Skill names** — `.claude/skills/<name>/`.
- **Spec sequencing** — the `authority order` list, the Relationship-to-prior-spec section's predecessor, the execution phase ladder position (`docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` and the per-phase docs 03–08), and the spec's `docs/4-specs/SPEC_LEDGER.md` entry.
- **Source documents** — when the spec cites a source (an execution phase doc, an architecture contract, Spec 0001, a report, a brainstorm output) in its Purpose / Scope / Required-areas, extract the path AND enumerate its actionable claims at Step 2 itself (read the source; for oversized docs use targeted greps with permissive anchoring). Tag each claim's adjudication status (accept / reject / defer / unadjudicated) by scanning the spec's Scope / Deliverables / Non-goals. This feeds Step 3.10.
- **Code / tree / schema examples** (Rust signatures, JSON/TOML schema snippets, directory trees, event-envelope layouts) — extract for fidelity checking.
- **Verification surfaces / thresholds** — exit/acceptance criteria, determinism/replay checks, fixture lists, golden-trace requirements, and severity mappings (blocker vs warning per the acceptance architecture).

**Reference-count checkpoint**: before Step 3, emit a one-line note with the exact reference count and the tracking decision — e.g. `Reference count: 12 — mental tracking sufficient` or `Reference count: 23 — TaskCreate recommended`. Use an exact integer (not `~20` or `20+`); the >15 threshold-decision must be reproducible. For specs with >15 references spanning unrelated areas, consider `TaskCreate` per-reference tracking; for tightly-clustered sets, mental tracking is acceptable. A fixed closed set checked by one presence grep counts as 1 reference, not N.

**Source-document engagement checkpoint** (when source documents are cited): emit a one-line note naming each source and its per-document adjudication counts — `Source-document engagement: <doc>: N claims enumerated, M adjudicated (accept / reject / defer), (N-M) unadjudicated flagged as findings.` When no source document is cited, emit `Source-document engagement: N/A — no external source cited`.

Prioritize references most likely to have drifted (proposed module/file paths, record/field names the spec extends, fixture names, event-kind vocabularies, sibling-spec sequencing, doc-tier paths after a doc reorg). Stable references (constitutional invariant names, phase names) can be spot-checked.

## Step 3: Codebase Validation

**Load `references/codebase-validation.md` before classification-driven substep selection.** Then validate every Step 2 reference, applying the substep subset for the Pre-Process classification. Collect everything; do not present findings yet.

## Step 4: Foundation Alignment Check

**Load `references/foundation-alignment.md` before alignment classification.** Then check the spec against the foundation doc pack — the constitutional invariants (`INV-NNN`), the kernel-authority and dependency-direction boundaries, the no-scripting / content-is-possibility rule, the determinism/replay contract, the no-leak / actor-knowledge firewall, the LLM/language boundary, and the Phase acceptance gates. Any constitutional invariant the spec would violate, or any Phase acceptance gate it would break, is a CRITICAL Issue. An architecture-changing decision that contradicts an accepted foundation/architecture doc requires a foundational-doc amendment first (per `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`); a missing amendment is a HIGH Issue.

## Steps 5-6: Classify and Present Findings

**Load `references/findings-and-questions.md` before classifying.** Classify findings into Issues (CRITICAL / HIGH / MEDIUM / LOW), Improvements, Additions, and Questions; present using the template there.

**Redesign-count checkpoint**: count deliverables (or Work-items) whose *approach* materially changed (eliminated, replaced with a different mechanism, or restructured beyond a refinement) over the pre-reassessment total. Emit the `N/total` ratio in the Step 6 Classification block. If it exceeds 50%, the Substantial Redesign Flag section is mandatory immediately above Questions.

**Wait for the user before Step 7.** In plan mode, write the plan file per `references/plan-mode.md`, then call ExitPlanMode. Auto-mode / no-stopping directives proceed directly to Step 7 ONLY when there are no Issues (CRITICAL/HIGH) and no open Questions; cite the directive inline.

## Step 7: Write the Updated Spec

**Load `references/spec-writing-rules.md` before writing.** Build the pre-apply verification table and emit it in chat before any Write/Edit. For each finding (keyed `I1`, `M1`, `A1`…), run a targeted check and record the command + result. Reclassify any mismatch (evidence-refining / recommendation-changing / scope-extending) per the reference; re-present recommendation-changing mismatches before applying. Then apply all approved changes, preserving structure and voice.

For classification **(d) retroactive**, Step 7 instead flips Status to `Done` and populates the Acceptance-evidence / Outcome section (see the retroactive branch in `references/spec-writing-rules.md`).

## Step 8: Final Summary

Present:

- Issues fixed, improvements applied, additions incorporated.
- Change inventory grouped by finding type.
- **Post-apply confirmation**: for every eliminated or renamed reference, grep-prove it is gone and corrected references resolve.
- Deferred items and reassessment-driven scope exclusions (with reasons).
- The 1–3 sections that changed most, flagged for review.
- **Classification shift note**, if the effective classification shifted (e.g. "(a) collapsed to (b) after deliverable removal"; "(b) shifted to (d) after Step 3 verified full landing"). Omit if unchanged.
- **Suggested next step**: "Review the updated spec, then begin implementation of its required areas against current doctrine. reassess-spec prepares a spec for implementation but does not implement it." For (d): note the spec is now a historical record (Status `Done`) and remind the user to update its `docs/4-specs/SPEC_LEDGER.md` entry.

Do NOT commit. Leave the file for user review.

## Guardrails

- **Foundation doctrine is authoritative**: never approve a spec change that violates a constitutional invariant (`INV-NNN`) or breaks a Phase acceptance gate, even if requested — flag it as a CRITICAL Issue instead. The doc-tier authority order (foundation → architecture → execution → reference → analysis → spec package) is binding: a spec may not contradict a higher tier.
- **Codebase / doctrine truth**: every reference in the updated spec must be validated. Never propagate stale paths, renamed records, removed symbols, or wrong doc-tier locations through Step 7.
- **No scope creep**: the deliverable is the updated spec file. Do not start implementation, create new specs, or edit sibling spec files — unless a Step 6 question response explicitly authorizes a named sibling-spec edit (record it in Step 8 under a "Cross-spec scope extension" line). Updating this spec's `docs/4-specs/SPEC_LEDGER.md` entry is in scope only when the spec's own documentation-updates content calls for it (e.g. a (d) retroactive Status flip).
- **No greenfield approach proposals**: validate and refine the existing design, not alternatives — except when the approach violates a constitutional invariant or conflicts with an accepted architecture/execution contract, where a minimum-viable alternative is part of the Issue finding.
- **Substantial redesign flag**: if reassessment changes >50% of deliverables' approach, flag at Step 6.
- **Worktree discipline**: inside a worktree, all paths resolve from the worktree root.
- **Plan-mode discipline**: load `references/plan-mode.md`, write the plan file, call ExitPlanMode, then execute Steps 7–8 after approval.
- **Do not `git commit`**: writes land in the working tree; the user reviews and commits.

## Foundation Alignment

| Principle | Step | Mechanism |
|-----------|------|-----------|
| Kernel authority (INV-008, INV-042; arch 01/10) | Step 3.8, Step 4 | Specs are checked to keep the event-sourced world kernel authoritative and TUI/view-models presentation-only; a spec that lets a view or the LLM decide world legality or state is flagged. |
| Kernel boundary + dependency direction (arch 01) | Steps 3.1, 3.8, 4 | Specs touching the kernel are checked so authority flows in the doctrine-mandated direction and no presentation/content layer inverts it; an inversion is a boundary-failure Issue. |
| Content is possibility, not script (INV no-scripting; foundation 09; arch 04) | Step 4 | Specs introducing content/fixtures are checked for behavior-looking fields (selectors, branches, triggers, authored outcome chains); an authored script or director logic is a CRITICAL Issue. |
| Determinism & replay (INV-017, INV-018; arch 03; spec §determinism/§replay) | Steps 3.8, 4 | Specs touching RNG, event ordering, serialization order, hashing, snapshots, or replay are checked to keep identical inputs+versions byte-identical and randomness seedable/auditable; deviations are flagged. |
| No telepathy / no-leak firewall (INV-024, INV-006; arch 05/10) | Steps 3.8, 4 | Specs touching view models, possession, traces, debug views, or projections are checked so hidden information reaches no viewer the actor-knowledge filter forbids; leakage is CRITICAL. |
| LLM/language boundary (INV-042; foundation 11; arch 06) | Step 4 | Specs touching speech acts or text are checked to keep the LLM out of world authority and behind validated extraction; an LLM deciding world state is CRITICAL. |
| Constitutional invariants + Phase acceptance gates (foundation 02/12; execution 02) | Steps 3.8, 4 | Any invariant the spec would violate, or any Phase acceptance gate (no-human run, deterministic replay, TUI playability, why-not, missing-property proof) it would break, is a CRITICAL Issue blocking implementation. |
| Foundational-doc amendment (no ADR system) | Step 4 | A spec making an architecture-changing decision that contradicts an accepted foundation/architecture doc must carry a foundational-doc amendment first (per `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`); a missing amendment is a HIGH Issue. |

## Final Rule

A reassessment is not complete until every reference in the updated spec is validated against the current codebase and the foundation doc pack, every approved finding has a pre-apply verification row proving the fix landed, and every eliminated or renamed reference has a post-apply grep-proof that it is gone.
