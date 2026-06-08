# Brief template & target-type reads

This file defines (A) the canonical anatomy of the emitted ChatGPT-Pro prompt and (B) the
research-target → load-bearing-reads map. The SKILL.md flow references both.

---

## A. Canonical brief anatomy

The emitted file `reports/<topic>-research-brief.md` is the *prompt the user pastes into
ChatGPT-Pro Session 2*. It is self-contained: Session 2 sees only this prompt plus the
uploaded manifest. Use these eight sections, in order. Scale each to the target; omit a
section only when genuinely N/A and say so.

### 1. Context

One or two sentences. Begin with the manifest pointer, then repo identity, then the **exact
fetch-baseline commit** Session 2 must read every file from (the verified repo HEAD per the
Step 6 baseline-commit rule — never a commit string copied from a report without confirming it
contains the §2 read-list):

> The uploaded manifest is the path inventory of the `joeloverbeck/tracewake` repo —
> a causality-first living-world simulation in Rust (event-sourced kernel, subjective
> epistemics, fallible institutions, TUI-first). Docs are layered authority:
> `0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier
> tiers govern later ones. Fetch every file from commit `<HEAD>` — the manifest reflects that
> tree. (If a referenced report cites a different "commit of record," note the divergence here
> and use the verified HEAD, not the report's string.)

### 2. Read in full (authority order)

An explicit, tiered path list — every file Session 2 must read before producing — each
with a one-line reason it is load-bearing *for this target*. Built from Step 2 exploration.
Order strictly by authority tier. Example shape:

```
Read these in full, in this order:

docs/README.md — authority order and the layering rule.
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; every deliverable must satisfy these.
docs/0-foundation/<file> — <why this target touches it>.
docs/1-architecture/<file> — <subsystem contract this target depends on>.
docs/2-execution/<file> — <current sequencing/gates this target realigns or extends>.
reports/<report> — <prior finding-set this target builds on>.
archive/specs/<spec> — <completed work that established the current state>.
```

### 3. Settled intentions

The decisions the interview resolved — the heart of why Session 2 is *locked*. State each
as a committed decision, not an option. This section pre-empts every clarifying question
Session 2 might otherwise ask. Carry any early-exit gaps here as `assumption: <X>` lines so
they read as defaults the user can override, not as open questions.

### 4. The task

A precise statement of what Session 2 must achieve — the goal behind the deliverable. One
tight paragraph. Name the target type (new spec / thorny fix / hardening / overhaul).

### 5. Exploration + online-research mandate

Authorize depth explicitly:

> Explore the repository as deeply as needed beyond the files listed above. Research online
> as deeply as needed — similar implementations, research papers, prior art — wherever it
> sharpens the deliverable. Cite sources for any external claim that shapes a decision.

### 6. Doctrine & constraints

Pointers Session 2 must honor:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every
  product-behavior decision must satisfy it; a genuine divergence requires amending an
  invariant first, never designing against it silently.
- Authority order: if execution conflicts with architecture or foundation, execution is
  wrong; if implementation is more convenient than the accepted gates, implementation is
  wrong.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible
  institutions, validation/replay.

Trim to the constraints the target actually engages.

### 7. Deliverable specification

Exactly what Session 2 outputs — leave no ambiguity:

- each **downloadable markdown document**, by filename and whether it **replaces** an
  existing file or is **new**;
- for replacements, name the file being replaced and what must be preserved vs. changed;
- the **locked / no-questions** instruction, verbatim intent:

> Produce the deliverables directly as downloadable markdown documents. Do not interview,
> do not ask clarifying questions — the requirements above are final. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed
> with the most faithful interpretation.

### 8. Self-check

A short acceptance checklist Session 2 runs against its own output before returning —
e.g. every replacement preserves the load-bearing content of the original; no new doctrine
weakens an upstream tier; every external claim is cited; the deliverable set matches §7
exactly; the §1 fetch-baseline commit contains every file named in the §2 read-in-full list.

---

## B. Target-type → load-bearing reads

A starting map for §2; always refine against Step 2 exploration. `02_CONSTITUTIONAL_INVARIANTS.md`
and `docs/README.md` are load-bearing for every type.

| Target type | Load-bearing tiers / files (beyond the two universal) |
|---|---|
| **new-spec** | `docs/1-architecture/**` for the touched subsystem; the relevant `docs/2-execution/**` phase + gates; `docs/4-specs/SPEC_LEDGER.md` and sibling specs; `tickets/README.md` if decomposition follows. |
| **thorny-fix** | the architecture contract for the affected subsystem; the relevant code seams; any `reports/**` or `archive/reports/**` audit touching the defect; the execution doc whose gate the fix must still satisfy. |
| **hardening / anti-contamination** | `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` and the firewall/provenance architecture (`docs/1-architecture/03_*`); the subsystem's foundation + architecture docs; prior hardening specs in `archive/specs/**`; the validation/observability docs. |
| **foundational / doc-overhaul** | the full tier being overhauled plus every tier above it (authority flows downward); `docs/README.md` for the authority table; the staleness/downstream report(s) in `reports/**`; cross-references in lower tiers that the overhaul will invalidate. |
| **other** | derive entirely from exploration; default to the universal two plus whatever the target names. |
