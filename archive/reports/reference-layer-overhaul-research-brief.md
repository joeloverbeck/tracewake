# Research brief — Overhaul of `docs/3-reference/*` (reference layer realignment)

> **You are ChatGPT-Pro Session 2.** This prompt is final and self-contained. The interview
> already happened. **Do not ask clarifying questions and do not interview.** Produce the
> deliverables directly as downloadable markdown documents. If a genuine contradiction makes
> a requirement impossible, state it inside the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded manifest is the path inventory of the `joeloverbeck/tracewake` repo — a
causality-first living-world simulation in Rust (event-sourced kernel, subjective epistemics,
fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. If execution conflicts with architecture or foundation, execution is wrong;
if implementation is more convenient than the accepted gates, implementation is wrong.

The **foundation, architecture, and execution tiers have just been overhauled** (the live set
as of `main` HEAD, commit `511dfb50b4ff975f1a4ff6cde0d6ffeb02920ba6` — fetch every file from this
commit). Note: a downstream report under `reports/` records an earlier "commit of record"
(`ca1cb9d…`) as *its own* analysis baseline; that commit predates the execution-layer merge and
still carries the retired execution filenames, so do **not** use it as your fetch baseline — use
`511dfb5`, where foundation, architecture, execution, and that report all coexist as live. This
overhaul was driven by archived
specs `0005`–`0008`, whose authoring surfaced that the foundational documents were not strong or
concrete enough for specs to be created in full alignment with intent. The **reference tier
(`docs/3-reference/*`) was last edited *before* that overhaul and is now stale against the new
spine.** Your job is to bring it back into alignment.

A downstream-staleness report already names `docs/3-reference/**` as subordinate-and-stale until
separately audited (see §2). This brief commissions that audit-and-replacement.

---

## 2. Read in full (authority order)

Read every file below, in this order, before producing anything. Each is load-bearing for this
overhaul. Then explore the rest of the repo as needed (§5).

```
docs/README.md
    Authority table, layering rule, and "Current doctrine status". This is ALSO a collateral
    deliverable: its 3-reference map and status text must be updated to match your output.

docs/0-foundation/00_FOUNDATION_INDEX.md
    Foundation map, reading order, anti-drift rules; the tier's house style.
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
    INV-001…INV-NNN, the non-negotiable contract every reference entry must remain subordinate to.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
    Constitutional source of the truth firewall and the "actor-known context", "provenance class",
    "sealed context", "proposal vs validation" vocabulary the glossary must absorb. Core formula:
    "Truth may validate actions, but truth may not plan them."

docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
    Architecture map, replacement/retirement rule, universal conformance questions; house style.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
    The architecture promotes "holder-known context" as the system-wide canonical term and defines
    "holder" broadly (actor, institution, household, role office, speaker, listener, embodied viewer,
    TUI affordance selector, lead interpreter, LOD promotion recipient, regional procedure owner),
    plus provenance packets, context sealing, debug comparison, and contamination gates. This is the
    single most important terminology source for the glossary realignment.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
    Acceptance artifacts, anti-contamination tests, no-human/TUI/replay gates, diagnostics, review
    checklists — the contract the reference checklist operationalizes.

docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
    Canonical gate-name table (P0-DOC, P0-CERT, TFW, PIPE, NO-DIRECT, NO-HUMAN, POS-PARITY, REPLAY,
    FIXTURE, DIAG) and the universal execution posture. The definition source you cross-reference.
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
    The post-0008 posture: archived specs 0005–0008 have LANDED but are NOT certified under the new
    doctrine. Source for the new risk-register relapse mode.
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
    Certification sequence and additional gate codes (e.g. SPINE-CERT, EPI-CERT, ORD-LIFE-CERT,
    FIRST-PROOF-CERT, PHASE-4-ENTRY, SECOND-PROOF-ENTRY). Verify the exact codes against this file;
    use whatever it actually defines.
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
    Execution-level truth-firewall (TFW) checks and the mandatory anti-contamination gate every
    future spec must include — what the reference checklist must point at.

docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
docs/3-reference/01_DESIGN_RISK_REGISTER.md
docs/3-reference/02_GLOSSARY.md
    The replacement baseline. Strong, doctrine-correct material that PREDATES the spine overhaul.
    Preserve their load-bearing content; realign, do not rewrite from scratch.

reports/02_DOWNSTREAM_STALENESS_AND_CODE_IMPLICATIONS_REPORT.md
    Records that 3-reference is subordinate-and-stale until audited, and the post-overhaul rationale.

archive/specs/0005_*  archive/specs/0006_*  archive/specs/0007_*  archive/specs/0008_*
    The specs whose certification gap drives the new risk entry. HISTORY, not live authority.
```

---

## 3. Settled intentions (final — do not re-litigate)

These decisions are locked. They pre-empt every clarifying question.

1. **Target.** Produce a full replacement set for the `docs/3-reference/*` tier, realigned to the
   post-overhaul spine (foundation/architecture/execution as live at `main` HEAD `511dfb5`).

2. **Structural freedom, bounded.** You MAY merge, split, add, remove, or rename reference files at
   your discretion. BUT the layer must stay **compact and lookup-only** — it is a fast-guardrail tool
   for future AI coding/spec/fixture/schema/prompt/review sessions, not a place for doctrine,
   architecture, execution plans, or expanded policy. The existing index explicitly forbids "adding
   reference files merely for symmetry." Honor that: justify every add/rename/remove in one line, and
   reflect every structural change in `docs/README.md`. If you keep the three existing files, that is
   acceptable; restructuring is permitted, not required.

3. **Gate vocabulary — cross-reference, never redefine.** Where the checklist and risk register touch
   certification, name the relevant canonical gate codes in line (e.g. `P0-CERT`, `TFW`, `PIPE`,
   `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`, and the certification-sequence codes such as
   `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`)
   and point to `docs/2-execution` as the definition source. The reference layer must **not** redefine
   any gate, restate its full meaning, or own gate semantics. Verify the exact gate strings against the
   execution docs; do not invent codes. Replace the current informal "phase claims" / "remains strong"
   language with gate-anchored references.

4. **Terminology reconciliation (glossary).** Add the now-canonical spine vocabulary, drawing
   definitions from foundation 14 and architecture 03:
   - `holder-known context` — the system-wide canonical term;
   - `holder` — and its breadth (actor, institution, household, role office, speaker, listener,
     embodied viewer, TUI affordance selector, lead interpreter, LOD promotion recipient, regional
     procedure owner);
   - `truth firewall` — "truth may validate, but truth may not plan";
   - `provenance class` / context (the provenance-class distinctions and the sealed context packet);
   - `context sealing` — a sealed context packet is immutable for that decision;
   - `institution-known context`;
   - `contamination gate` / contamination review.
   **Reconcile, do not delete, the existing `Actor-known` entry**: present it as the foundation-tier
   name for the actor case of the broader architecture term `holder-known`. Keep `Knowledge context`.
   Keep every existing canonical / restricted / forbidden-as-core term unless genuinely superseded; the
   full term tables are load-bearing.

5. **Post-0008 certification posture — you weight placement.** The single biggest doctrinal shift is
   that archived specs 0005–0008 have LANDED but are NOT certified under the post-overhaul doctrine.
   This MUST be carried by the reference layer. At minimum add a new risk-register relapse mode capturing
   "archived specs 0005–0008 (Phase 3A) treated as certified under post-overhaul doctrine." Place any
   supporting checklist question and glossary entries wherever best serves the layer's lookup purpose.

6. **House style.** Adopt the overhauled tiers' section pattern where it fits without bloating the
   compact docs: `## Status`, `## Authority boundary`, `## Depends on`, `## Maintenance rule`. The
   current reference docs lack `Authority boundary` and `Depends on`; add them (the layer depends on all
   three higher tiers and owns only compact lookup/guardrail/risk material).

7. **Preserve, don't discard.** The existing source-discipline gate, the full risk watchlist
   (`R-00…R-25`), the phase-exit review prompt, and the canonical/context-bound/forbidden term tables are
   load-bearing. Carry them forward; realign wording and cross-references; only drop material that the
   new spine genuinely supersedes, and say so when you do.

---

## 4. The task

This is a **foundational / doc-overhaul** target: the downstream cascade from an already-completed
upstream tier change. Realign the `docs/3-reference/*` tier so it is a faithful, compact, lookup-only
companion to the post-overhaul foundation/architecture/execution spine — correct stale terminology,
absorb the holder-known/truth-firewall canon into the glossary, thread the canonical gate vocabulary
through the checklist and risk register by reference, add the post-0008 certification relapse mode,
adopt the tiers' structural house style, and update `docs/README.md` to match. The output must read as
though it had always been written alongside the new spine, while preserving the strong existing
guardrail material.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — in particular, skim the rest
of `docs/0-foundation/**`, `docs/1-architecture/**`, and `docs/2-execution/**` so the glossary and
checklist do not contradict any live doctrine, and scan `docs/4-specs/**` and `.claude/skills/**`
(read-only) for how the reference layer is pointed to operationally. This is primarily a
repo-internal realignment; deep online research is largely unnecessary. If you do consult external prior
art (e.g. event-sourcing, BDI/HTN, provenance/firewall patterns) to sharpen a definition, cite the
source for any external claim that shapes a decision. Do not import outside doctrine into the reference
layer.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every reference entry must
  remain subordinate to it; a reference doc may operationalize an invariant but must never expand,
  weaken, or contradict it.
- Authority order is strict: foundation > architecture > execution > reference. The reference layer
  "may define compact lookup aids and risk registers" and "may not define hidden policy, implementation
  plans, or expanded doctrine." If your text would set new doctrine, move it up a tier instead — do not
  smuggle it into reference.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination spine: no simulation fact may be born from prose; preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible institutions,
  questless leads, TUI-first playability, validation/replay, and the holder-known truth firewall.
- Keep debug truth visibly non-diegetic and structurally quarantined. Prefer typed claims, typed
  diagnostics, and replayable records over display strings — the glossary's forbidden-as-core table
  must stay intact.
- Restart numbering inside the `3-reference/` folder; avoid stale filenames in indexes and examples.

---

## 7. Deliverable specification

Produce, as downloadable markdown documents:

1. **The full replacement set for `docs/3-reference/*`** — every markdown file that the realigned tier
   should contain, each as a complete file. The replacement baseline is the current three:
   - `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` (replaces existing)
   - `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (replaces existing)
   - `docs/3-reference/02_GLOSSARY.md` (replaces existing)

   Per §3.2 you MAY merge/split/add/remove/rename. If you do, deliver every resulting file in full and
   include a short **change manifest** at the top of your response listing, for each file:
   `new | replaces <name> | renamed from <name> | removed <name> (reason)`, with a one-line
   justification for each structural change against the "no files for symmetry" rule.

2. **`docs/README.md`** — full replacement, with the `## 3-reference/` map and the `## Current doctrine
   status` section updated to describe the realigned reference layer and any file changes (and to state
   that the reference tier is now realigned against the post-overhaul spine).

Do not modify, and do not emit replacements for, any file under `docs/0-foundation`, `docs/1-architecture`,
`docs/2-execution`, `docs/4-specs`, `specs/`, `tickets/`, `archive/`, `.claude/`, or source code. The only
files you output are the realigned `docs/3-reference/*` set and `docs/README.md`.

> **Locked / no-questions:** Produce the deliverables directly as downloadable markdown documents. Do not
> interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run before returning)

- [ ] Every replacement file preserves the load-bearing content of its original (source-discipline gate,
      `R-00…R-25`, phase-exit prompt, full canonical/restricted/forbidden term tables) except where the new
      spine genuinely supersedes it — and each such drop is stated explicitly.
- [ ] The glossary now defines `holder-known context`, `holder` (with its full breadth), `truth firewall`,
      `provenance class`/sealed context, `context sealing`, `institution-known context`, and
      `contamination gate`, and reconciles (does not delete) `Actor-known`.
- [ ] Every gate code mentioned is verified against `docs/2-execution`, is cross-referenced (never
      redefined), and the old "phase claims" / "remains strong" language is gone.
- [ ] A new risk-register entry captures "archived specs 0005–0008 treated as certified under
      post-overhaul doctrine," and the certification posture is reflected wherever the layer's lookup
      purpose is best served.
- [ ] No reference entry sets new doctrine, expands policy, or contradicts foundation/architecture/execution;
      every entry stays compact and lookup-only.
- [ ] `## Status`, `## Authority boundary`, `## Depends on`, and `## Maintenance rule` are present where they
      fit, without bloating the docs.
- [ ] `docs/README.md` matches the delivered file set exactly (names, count, status text); no stale reference
      filenames remain anywhere.
- [ ] The change manifest (if any structural change) lists every add/rename/remove with a one-line
      justification.
- [ ] The deliverable set equals exactly: the realigned `docs/3-reference/*` files + `docs/README.md`.
