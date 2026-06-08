# Research brief — Realignment of `docs/4-specs/*` (spec layer realignment)

> **You are ChatGPT-Pro Session 2.** This prompt is final and self-contained. The interview
> already happened. **Do not ask clarifying questions and do not interview.** Produce the
> deliverables directly as downloadable markdown documents. If a genuine contradiction makes a
> requirement impossible, state it inside the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded manifest is the path inventory of the `joeloverbeck/tracewake` repo — a
causality-first living-world simulation in Rust (event-sourced kernel, subjective epistemics,
fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. If execution conflicts with architecture or foundation, execution is wrong;
if implementation is more convenient than the accepted gates, implementation is wrong.

The **foundation, architecture, execution, AND reference tiers have all just been overhauled**
and are live as of `main` HEAD, commit `167a5985d6730b191a798b2d8daa75b9b7d76ac1` — **fetch every
file from this commit**; the manifest reflects that tree. The overhaul was driven by archived
specs `0005`–`0008`, whose authoring surfaced that the foundational documents were not strong or
concrete enough for specs to be created in full alignment with intent — which let archived spec
`0005` be implemented in a way that diverged from intent and forced three hardening /
anti-contamination specs (`0006`–`0008`).

The **spec tier (`docs/4-specs/*`) was last edited *before* the overhaul and is now stale against
the new spine.** It still cites an old "commit of record" (`3b45d7dde9811f1a0ddbe9497f3d9e0c3743d74e`)
and roughly forty **retired** architecture/execution filenames, frames its central document as
"Phase 0 paper ontology *before* Phase 1" (a phase since landed through specs `0002`–`0008`), uses
pre-overhaul terminology, and contains a foundational-amendments document whose conclusion the
overhaul has falsified. Your job is to bring the whole tier back into alignment.

> **Commit-of-record warning.** Several `docs/4-specs/*` files and the report under `reports/`
> cite earlier baselines (`3b45d7d…`, `93759de…`, `8e3cf3e…`, `ca1cb9d…`). Those are each that
> document's *own* analysis baseline and **predate** the foundation/architecture/execution/reference
> overhaul merges. **Do not adopt any of them.** Your single fetch baseline is
> `167a5985d6730b191a798b2d8daa75b9b7d76ac1`, where all four upper tiers coexist as live doctrine.
> When you correct a stale baseline string inside a spec, follow that spec's own source-discipline
> rule: a commit is named only as audit/spec provenance, never re-presented as current product
> doctrine, and the manifest is path inventory only.

A downstream-staleness report (see §2) already names `docs/4-specs/**` as subordinate-and-stale
until separately audited. This brief commissions that audit-and-realignment.

---

## 2. Read in full (authority order)

Read every file below, in this order, before producing anything. Each is load-bearing for this
realignment. Then explore the rest of the repo as needed (§5).

```
docs/README.md
    Authority table, layering rule, and "Current doctrine status". This is ALSO a collateral
    deliverable: its 4-specs map and maintenance/status text must be updated to match your output.

docs/0-foundation/00_FOUNDATION_INDEX.md
    Foundation map, reading order, anti-drift rules; the tier's house style and "do not cherry-pick".
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
    INV-001…INV-NNN, the non-negotiable contract every spec must declare conformance to and never weaken.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
    First-village scope, missing-property proof, no-human/replay/TUI gates, canonical regression seeds —
    the doctrine the village-ontology spec realizes.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
    Constitutional truth firewall and the "actor-known context" vocabulary the spec must reconcile with
    the broader architecture term. Core formula: "Truth may validate actions, but truth may not plan them."

docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
    Architecture map, replacement/retirement rule, universal conformance questions; current filenames.
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
    Promotes "holder-known context" as the system-wide canonical term and defines "holder" broadly
    (actor, institution, household, role office, speaker, listener, embodied viewer, TUI affordance
    selector, lead interpreter, LOD promotion recipient, regional procedure owner). The single most
    important terminology source for the spec's actor-known/holder-known reconciliation.
docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
    Missing-property first proof, settlement ontology, spaces, property, food, sleep, work, local economy —
    the architecture contract the village-ontology spec must conform to.
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
    Acceptance artifacts, anti-contamination tests, no-human/TUI/replay gates — the validation contract
    the spec's fixtures and "future test mapping" must point at.

docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
    Execution map, retirement rule, and the canonical gate-name table you cross-reference. Verify exact
    gate strings here; do not invent codes.
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
    The post-0008 posture: archived specs 0005–0008 LANDED but are NOT certified under the new doctrine;
    defines `P0-CERT`; defines the EXACT admissibility posture every future spec must declare
    (`P0-CERT passed` | `P0-CERT scoped remediation` | `P0-CERT not applicable`). Reconcile the
    SPEC_LEDGER's archived-spec table against this document's table.
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
    Live first-proof identity and the acceptance gate set (EVENT, TRUTH-FIREWALL, ACTOR-KNOWN,
    POSSESSION-PARITY, NO-HUMAN-ORDINARY-LIFE, MISSING-PROPERTY, VIEW-DEBUG-SPLIT, REPLAY,
    FIXTURE-NEGATIVE). Replaces the retired first-proof/missing-property execution docs the spec cites.
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
    Gate order and the rule that Phase 4 is blocked until certification gates pass — context for the
    spec's "Phase 1 entry" / phase-status language, which is now stale.
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
    Execution-level truth-firewall checks and the mandatory anti-contamination gate every future spec must
    include — what the spec's no-scripting / validation sections must point at.
docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
    Authoring contracts, schema/provenance validation, no outcome-chain data — the contract the spec's
    fixture/validation-mapping sections feed.
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
    Golden fixture families, adversarial scenarios, deterministic replay acceptance — the contract the
    spec's fixture suite realizes.

docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
docs/3-reference/01_DESIGN_RISK_REGISTER.md
docs/3-reference/02_GLOSSARY.md
    The realigned reference tier. Source for the canonical glossary terms (holder-known, truth firewall,
    provenance class, context sealing, etc.), the source-discipline gate, and the risk register the specs
    cross-reference. Use these terms; do not contradict them.

docs/4-specs/README.md
docs/4-specs/SPEC_LEDGER.md
docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md
docs/4-specs/0001_RESEARCH_NOTES.md
    THE REPLACEMENT BASELINE — the five files you are realigning. Strong, partly-still-correct material that
    PREDATES the spine overhaul. Preserve load-bearing content; realign, do not rewrite from scratch.

reports/02_DOWNSTREAM_STALENESS_AND_CODE_IMPLICATIONS_REPORT.md
    Records that 4-specs is subordinate-and-stale until audited, and the post-overhaul rationale.

archive/specs/0005_*  archive/specs/0006_*  archive/specs/0007_*  archive/specs/0008_*
    The specs whose authoring drove the overhaul and whose landed-but-uncertified status the spec tier must
    represent correctly. HISTORY, not live authority.
```

You should ALSO inspect (read-only, to verify the ontology spec against reality, not to emit):
`crates/tracewake-content/src/fixtures/**` (e.g. `strongbox_001.rs`, `expectation_contradiction_001.rs`,
`no_human_day_001.rs`, `sleep_eat_work_001.rs`, `possession_parity_001.rs`) and
`crates/tracewake-content/src/load.rs`. The village roster (`actor_tomas`, `coin_stack_01`,
`strongbox_tomas`, …) is implemented across these fixtures; the ontology spec describes a world that
actually exists in code. Use this to judge whether the spec is a live contract or historical evidence.

---

## 3. Settled intentions (final — do not re-litigate)

These decisions are locked. They pre-empt every clarifying question.

1. **Target & type.** Realign the entire `docs/4-specs/*` tier to the post-overhaul spine
   (foundation/architecture/execution/reference, all live at HEAD `167a5985d6730b191a798b2d8daa75b9b7d76ac1`).
   This is a **foundational / doc-overhaul** target: the downstream cascade from an already-completed
   upper-tier change.

2. **Strictly corrective — no new forward spec.** Your job is to correct, merge, remove, renumber, and
   realign the EXISTING tier. **Do NOT author any new forward implementation spec.** In particular, the
   `P0-CERT` post-0008 certification audit (which execution docs flag as "the next major execution move")
   is **explicitly out of scope** — the maintainer will drive that spec in a later session. You may, and
   should, make the tier *ready* for it (correct status language, correct admissibility posture, a ledger
   that names what comes next), but you must not write the certification spec itself, nor any other new
   forward-looking implementation spec.

3. **The village-ontology spec — your call, justified.**
   `0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` is implemented across ~15
   content fixtures, so it is not merely paper. **You choose** between:
   (a) **realign-in-place** as the live first-proof world ontology + fixture contract; or
   (b) **archive-and-replace** with a compact live spec that points to the fixtures as the now-authoritative
   world source.
   Justify the choice in one line in the change manifest (§7). Either way you MUST: strip the "Phase 0 paper
   ontology *before* Phase 1" framing (Phase 0–3A have landed); replace the stale target commit and the
   ~40 retired architecture/execution filenames with the current ones (verify each against the live tree —
   e.g. the spec's `docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING…`, `03_EVENT_LOG_REPLAY…`,
   `docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY`, `04_PHASE_0_PAPER_ONTOLOGY…`,
   `07_PHASE_3_NEEDS_ROUTINES…` are ALL retired); and reconcile terminology and gate references per §3.5–3.7.
   The roster/place/container/action/event/proposition/fixture content is load-bearing and must be preserved
   except where the new spine genuinely supersedes it.

4. **The two obsolete companions — your call, justified.** `0001_FOUNDATIONAL_DOC_AMENDMENTS.md` concludes
   "no replacement foundational documents are recommended" at the old commit — a conclusion the overhaul has
   **falsified** (foundation/architecture/execution/reference were ALL replaced). `0001_RESEARCH_NOTES.md` is
   the research backing for that spec. **You choose** between (a) **removing both** from the live tier
   (recording the removal in the change manifest and README), or (b) **correcting and keeping** them (rewrite
   the amendments doc to record that the overhaul DID replace the foundational docs and that the spec tier was
   subsequently realigned; refresh the research notes). Justify the choice in one line. Research notes, if
   kept, must remain spec provenance, not product doctrine.

5. **Numbering / structure — free to restructure.** The tier currently has three files all numbered `0001_*`,
   which is confusing. You MAY renumber and reorganize so there is one number per spec package, and may
   merge/split/rename/remove files at your discretion. Honor compactness over symmetry — do not add files
   merely for parity. Reflect every structural change in `docs/README.md` and in the change manifest, with a
   one-line justification each. Restart numbering inside the `4-specs/` folder; never continue numbers across
   tiers. Keep `SPEC_LEDGER.md` and `README.md` as the tier's index/ledger names.

6. **Terminology reconciliation.** Apply architecture-03's rule: `holder-known context` is the system-wide
   canonical term; `actor-known` is the foundation-tier name for the **actor case** of holder-known. In the
   specs, use `holder-known` where the text touches non-actor holders (institution, household, role office,
   speaker, listener, embodied viewer, TUI affordance selector); keep `actor-known` where the case is genuinely
   the actor. Do NOT silently delete the existing actor-known vocabulary — reconcile it. Match the realigned
   `docs/3-reference/02_GLOSSARY.md` exactly; never coin terms it does not define.

7. **Gate vocabulary — cross-reference, never redefine.** Where the specs touch certification, acceptance, or
   first-proof gates, name the canonical gate codes in line — `P0-CERT`, and the first-proof gates `EVENT`,
   `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`,
   `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE` — and point to `docs/2-execution/` as the definition source.
   **Verify the exact gate strings against the execution docs before using them; do not invent or guess codes.**
   The spec tier must NOT define, restate the full meaning of, or own gate semantics. Replace the spec's
   informal phase-status language ("Phase 1 entry", "ready", "earned") with gate-anchored references, and make
   every live spec carry the execution-01 admissibility posture verbatim in spirit
   (`P0-CERT passed` | `P0-CERT scoped remediation` | `P0-CERT not applicable`) — for a documentation-only
   realignment the correct posture is `P0-CERT not applicable`.

8. **Source discipline.** No bare or stale commit hash may be presented as the *current* baseline. Preserve the
   `SPEC_LEDGER.md` source-discipline rule (manifests are path inventory only; branch names / default-branch
   lookups / connector labels / code-search snippets are not proof of target-commit content). Where a spec must
   reference a commit for historical provenance, keep it clearly labeled as audit/spec provenance, not product
   doctrine, consistent with the realigned reference tier.

9. **Authority subordination.** The spec tier sits beneath foundation, architecture, execution, and reference.
   Specs may NOT replace architecture, amend the constitution, weaken execution gates, redefine gate semantics,
   or cite archived specs `0005`–`0008` as live authority. Where the realigned specs name those archived specs,
   they are history that drove the overhaul, never current certification.

---

## 4. The task

Realign the `docs/4-specs/*` tier so it reads as though it had always been written alongside the post-overhaul
foundation/architecture/execution/reference spine: correct every stale commit reference and retired filename,
strip obsolete pre-overhaul phase framing, reconcile actor-known/holder-known terminology against the live
glossary, thread the canonical gate vocabulary through by reference, represent the archived-spec
landed-but-uncertified status correctly, restructure/renumber for one-package-per-number clarity, resolve the
two obsolete companion documents, and update `docs/README.md` to match. Preserve the strong, still-correct
roster/ontology/fixture/ledger material; change only what the new spine genuinely supersedes, and say so when
you drop something. This is a corrective realignment, not a new spec.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — in particular skim the remaining
`docs/0-foundation/**`, `docs/1-architecture/**`, and `docs/2-execution/**` so the realigned specs contradict no
live doctrine, and read the `crates/tracewake-content/src/fixtures/**` files that implement the village roster so
the ontology spec matches what exists in code. Also scan `.claude/skills/reassess-spec` and
`.claude/skills/spec-to-tickets` (read-only) for how the spec tier is consumed operationally, and `tickets/README.md`
if present. This is primarily a repo-internal realignment; deep online research is largely unnecessary. If you do
consult external prior art (event-sourcing/CQRS, BDI/HTN, smart objects/affordances, provenance/firewall patterns,
the social-simulation precedents the existing research notes cite) to sharpen a correction, cite the source for any
external claim that shapes a decision. Do not import outside doctrine into the spec tier.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every realigned spec must remain
  subordinate to it and must declare how it satisfies the live foundation/architecture/execution/reference
  guardrails; a spec may operationalize an invariant but must never expand, weaken, or contradict one.
- Authority order is strict: foundation > architecture > execution > reference > specs. If your text would set
  new doctrine, replace architecture, or define a gate, it belongs in a higher tier — do not smuggle it into a
  spec. Where execution gate codes are involved, definitions come from `docs/2-execution/`, not the spec.
- No backwards-compatibility shims or alias paths in new work; no stale filenames in indexes, examples, or
  fetched-file lists.
- Anti-contamination spine: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first
  playability, validation/replay, and the holder-known truth firewall. The ontology spec's forbidden-collapse and
  no-scripting sections are load-bearing — keep them, realigned.
- Keep debug truth visibly non-diegetic and structurally quarantined; prefer typed claims, typed diagnostics,
  sealed context packets, and replayable records over display strings.
- Restart numbering inside `4-specs/`; avoid stale filenames anywhere.

---

## 7. Deliverable specification

Produce, as downloadable markdown documents:

1. **The full realigned set for `docs/4-specs/*`** — every markdown file the realigned tier should contain, each
   as a complete file. The replacement baseline is the current five:
   - `docs/4-specs/README.md`
   - `docs/4-specs/SPEC_LEDGER.md`
   - `docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`
   - `docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md`
   - `docs/4-specs/0001_RESEARCH_NOTES.md`

   Per §3.3–3.5 you MAY merge / split / rename / renumber / remove files. Deliver every resulting file in full,
   and include a **change manifest** at the top of your response listing, for each file:
   `new | replaces <name> | renamed from <name> (reason) | removed <name> (reason)`, with a one-line justification
   for each structural change against the "no files for symmetry" rule. Name the final delivered file set
   explicitly so the maintainer can map old→new at a glance.

2. **`docs/README.md`** — full replacement, with the `## 4-specs/ — active spec layer` map and the
   `## Current doctrine status` / maintenance text updated to describe the realigned spec tier and any file
   add/rename/remove, and to state that the spec tier is now realigned against the post-overhaul spine. Keep every
   other section of `docs/README.md` accurate; do not introduce changes to the higher-tier maps beyond what the
   4-specs realignment requires.

Do not modify, and do not emit replacements for, any file under `docs/0-foundation`, `docs/1-architecture`,
`docs/2-execution`, `docs/3-reference`, `specs/`, `tickets/`, `archive/`, `.claude/`, `crates/`, or any other
source. You MAY note in prose (not as an emitted file) that
`reports/02_DOWNSTREAM_STALENESS_AND_CODE_IMPLICATIONS_REPORT.md` line about 4-specs being subordinate-and-stale
is resolved by this realignment, but do not rewrite the report. The only files you OUTPUT are the realigned
`docs/4-specs/*` set and `docs/README.md`.

> **Locked / no-questions:** Produce the deliverables directly as downloadable markdown documents. Do not interview,
> do not ask clarifying questions — the requirements above are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- [ ] Every emitted file fetched/reasoned from baseline `167a5985d6730b191a798b2d8daa75b9b7d76ac1`; no earlier
      "commit of record" (`3b45d7d`, `93759de`, `8e3cf3e`, `ca1cb9d`) adopted as the current baseline.
- [ ] Every retired architecture/execution filename the old specs cited is replaced with a filename verified to
      exist in the live tree; no stale filename remains in any spec, fetched-file list, or index.
- [ ] The "Phase 0 paper ontology before Phase 1" framing is gone; the ontology spec's fate (realign-in-place vs
      archive-and-replace) is decided and justified in the change manifest, and its load-bearing
      roster/place/container/action/event/fixture content is preserved except where genuinely superseded.
- [ ] The two obsolete companions (`0001_FOUNDATIONAL_DOC_AMENDMENTS.md`, `0001_RESEARCH_NOTES.md`) are resolved
      (removed or corrected-and-kept), with a one-line justification each; no surviving doc still claims "no
      replacement foundational documents are recommended."
- [ ] Files renumbered to one package per number; every rename/merge/remove appears in both the change manifest
      and the updated `docs/README.md`.
- [ ] Terminology matches the live glossary: `holder-known` used for non-actor holders, `actor-known` reconciled
      (not deleted) for the actor case; no invented terms.
- [ ] Every gate code mentioned is verified against `docs/2-execution`, cross-referenced (never redefined), and the
      old informal phase-status language ("earned", "ready", "Phase 1 entry") is replaced with gate-anchored
      references. Each live spec carries the correct admissibility posture (here `P0-CERT not applicable`).
- [ ] No new forward implementation spec was authored; `P0-CERT` is named as the next move but NOT specified.
- [ ] Archived specs `0005`–`0008` appear only as history that drove the overhaul, never as live certification; the
      SPEC_LEDGER's archived-spec status reconciles with `docs/2-execution/01`'s table.
- [ ] No spec sets new doctrine, replaces architecture, amends the constitution, or weakens/redefines a gate; the
      source-discipline rule (manifest = path inventory only) is preserved.
- [ ] `docs/README.md` matches the delivered 4-specs file set exactly (names, count, status text).
- [ ] The deliverable set equals exactly: the realigned `docs/4-specs/*` files + `docs/README.md`.
