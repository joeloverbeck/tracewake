# Architecture-tier alignment research brief

**For:** an external deep-research session (ChatGPT-Pro Session 2).
**Deliverable:** one new downloadable markdown document — an analysis / recommendation report for `docs/1-architecture/*`.
**Posture:** locked / no questions. The requirements below are final. Produce the deliverable directly; do not interview.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-14_ea6a05b.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first play). Documentation is
layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`;
**earlier tiers govern later ones.** **Fetch every file from commit `ea6a05b`** (full:
`ea6a05bf5822307cfcbd39804bbb015cdb13215d`) — the manifest reflects that exact tree. Fetch only by exact
commit URL; do not use branch-name fetch, default-branch lookup, repository metadata, code search, or a
clone. If any file you fetch carries an older "commit of record" inside its own provenance text (the seed
reports do), treat that as the report's internal history only — use `ea6a05b`, not the embedded string.

**This brief continues a multi-session campaign. Read it as a delta, not a cold start.** The chain:

1. `reports/verdict-on-foundations.md` — raised nine candidate "foundation completeness" themes.
2. `reports/foundations-completeness-determination-research-report.md` — adjudicated the nine: **promoted
   one** (time / calendar / social rhythm) to the foundation tier and **routed the other eight below
   foundation**, with explicit per-tier forward-routing (its §5). **This is the freshest, most-specific
   seed for the architecture work in this brief** — its §5.2 is the architecture-tier routing.
3. `reports/foundation-tier-alignment-research-report.md` → ratified spec
   `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` — landed the temporal-authority
   doctrine into the foundation tier (new invariant `INV-112`; primary section in foundation `03`;
   application cross-references in foundation `04/05/07/08/10/12/14`, with index/source-note updates in
   `00`/`13`). Spec 0031 §6 **explicitly deferred the architecture/execution/reference cascade of that
   doctrine to later, separate sessions.** This brief commissions the **architecture** half of that deferral.

Separately, an **earlier** campaign already aligned the architecture tier once: the
foundations-emergence amendment `archive/specs/0026_…` cascaded down through
`archive/reports/architecture-tier-alignment-research-report.md` → ratified spec
`archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`. That work (truth-firewall,
provenance sufficiency, freshness classifier, believed-access snapshots, single-charge accounting,
observer-only emergence evidence) is **settled and must not be re-commissioned.** The new report is a
**delta on top of it**: it adds the temporal layer and the seven still-unrouted architecture themes only.

---

## 2. Read in full (authority order)

Read every file below from commit `ea6a05b` before producing. All paths are confirmed present at that
commit.

**Tier −1 — authority map**
- `docs/README.md` — the documentation authority order and the layering rule that governs every tier-fit verdict.

**Tier 0 — foundation (the IMMUTABLE GOVERNING REFERENCE — never an amendment target in this pass).**
Read the whole tier; you were asked to. Load-bearing (primary):
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; **`INV-112` (the new temporal-authority invariant)** and the truth-firewall family `INV-099…INV-111`. Every recommendation must satisfy these.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — **the authoritative home of the temporal-authority doctrine** (the six temporal categories + the temporal firewall). Block T translates this into architecture.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — temporal-claim/freshness application cross-ref; also the foundation driver for learning/adaptation and bounded-affect memory effects.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — routines/schedules consume holder-known temporal premises; foundation driver for affect and learning.
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — institutional/procedural time; foundation driver for practical bias / social-harm.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — time controls and embodied/debug separation; foundation driver for play-legibility.
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — LOD/regional temporal summaries; foundation driver for performance/fairness budgets.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable temporal acceptance + scope/non-goals.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the truth firewall and scheduler boundary; the temporal firewall is its explicit specialization.
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — foundation driver for quantity/granularity/fungibility (food, money, payment, custody).

Read the remaining foundation files (`00` index, `01`, `09`, `11`, `13`) **to bound altitude** — to know what foundation already owns so you do not push lower-tier mechanism upward or duplicate foundation doctrine in architecture.

**Tier 1 — architecture (THE AMENDMENT TARGET).** Read all of `00`…`14`:
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — index + conformance rows; some doctrine currently lives only in hardening rows here and should be named in the primary docs.
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — architecture home of foundation `03`; primary candidate for the temporal-authority/event-time seam.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — architecture home of foundation `14`; the temporal firewall and "holder-known temporal claim" seam.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — scheduler authority / deterministic ordering / budget exhaustion (Block T + perf/fairness).
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — actor-known transaction; routines/temporal premises, bounded affect, learning/adaptation, planner budgets.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — epistemic data flow; temporal claims/freshness, affect memory effects, learned expectations.
- `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — institutional/procedural time; practical bias / social-harm.
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — quantity/granularity/fungibility.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — play-legibility loop; embodied temporal rendering.
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — lead/notebook usefulness; play-legibility.
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — LOD/regional temporal summaries; time-acceleration declaration; perf/fairness.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — authoring/compiler discipline as architecture-protecting seams; temporal-firewall observability; perf/fairness observability.
- `docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` — research-decision/forbidden-misread doctrine; confirm no new external-prior-art import contradicts it.

**Seed reports (the deltas this pass builds on)**
- `reports/foundations-completeness-determination-research-report.md` — **primary seed.** §5.2 = the architecture routing for the seven themes (Block R); §3 = per-theme current-coverage evidence and tier-fit reasoning; §6 = open questions you must carry forward, not resolve.
- `reports/verdict-on-foundations.md` — the original nine themes, un-adjudicated, so you see the source pressure behind the routings.
- `reports/foundation-tier-alignment-research-report.md` — the temporal foundation analysis that produced 0031; the upstream rationale for Block T.

**Archived specs/reports (settled context — what already landed; do not re-commission)**
- `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` — what landed in foundation for temporal authority, the `INV-112` text in its Outcome, and the §6 deferral that defines Block T's scope and its lower-tier route-forward boundary.
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — the prior foundation amendment + the report-shape precedent.
- `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — the **already-ratified** prior architecture alignment; its deliverables are settled (do not re-open).
- `archive/reports/architecture-tier-alignment-research-report.md` — **the canonical report-shape precedent for this exact deliverable** (disposition table → method ledger → foundation-delta-to-translate → architecture sweep coverage register → per-finding sections → forward routing → open questions → references). Reuse this structure. Also: what it already covered tells you what NOT to re-commission.

**Boundary-awareness (read to bound scope and to route findings forward — NOT amendment targets; do not "correct" them):**
- `docs/2-execution/*` (the relevant docs your forward-routing appendix hands off to — scheduler/proof in `05`/`10`, truth-firewall gates in `04`, ordinary-life proof in `06`, authoring/schema validation in `08`, institutions/Phase-4 in `11`, deferred LOD/second-proof in `12`, index/gate order in `00`/`03`).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` and `docs/3-reference/02_GLOSSARY.md` — reference-tier route-forward targets (risk memory + terminology).
- `docs/4-specs/SPEC_LEDGER.md` — confirms campaign state (0026→0031) and the live-spec numbering; you are NOT producing a numbered spec.

---

## 3. Settled intentions (final — do not re-litigate)

1. **Scope = two bodies of work in ONE consolidated architecture-tier report.**
   - **Block T — temporal-authority cascade.** Specialize the now-landed foundation temporal-authority
     doctrine (`INV-112` + foundation `03`'s temporal section, applied across foundation `04/05/07/08/10/12/14`)
     into `docs/1-architecture/*` subsystem contracts. This is the architecture cascade spec **0031 §6
     explicitly deferred.** Architecture is **not yet aligned** to it — a grep of `docs/1-architecture/*` at
     `ea6a05b` finds no `temporal authority`, `holder-known temporal`, `procedural time`, `temporal firewall`,
     or `INV-112` tokens — so treat this as a genuine fresh delta, not a validation pass.
   - **Block R — the seven routed themes** from completeness-report §5.2, each with its named architecture
     home(s): (1) play experience & epistemic legibility → `10`, `11`; (2) quantity / material granularity /
     fungibility → `09`; (3) bounded affect / emotion → `05`, `06`; (4) learning / adaptation → `05`, `06`;
     (5) deterministic performance & fairness budgets → `04`, `05`, `12`, `13`; (6) practical bias /
     social-harm → `08`; (7) authoring / compiler discipline → `13`.
2. **Validate-before-gap for Block R.** The completeness report §3 documents substantial *existing*
   architecture coverage for several of these themes (e.g. affect language already in `05`/`06`; learning
   seams in `05`/`06`; bias/institution doctrine in `08`). For each routed theme you must state current
   coverage first, then use precedent-style verdicts — `belongs in architecture`, `partial coverage /
   consolidation`, `already-owned-close`, or `route-forward` — rather than assuming every routed item is a
   hole. A theme is not a gap merely because the seed named it.
3. **Delta, not cold start.** The 0026/emergence architecture cascade (ratified spec 0027) is settled. Do
   **not** re-commission truth-firewall translation, provenance sufficiency, freshness classifier,
   believed-access snapshots, single-charge accounting, or observer-only emergence evidence. Where Block T
   or Block R touches a doc that 0027 already amended, build on the existing text; note continuity, do not
   re-litigate.
4. **Authority/altitude discipline.** Architecture owns *subsystem contracts*: which component may read,
   write, derive, expose, or preserve what; data flows; subsystem boundaries; diagnostics; acceptance
   implications. Route **forward** (to execution / reference / future specs) anything that is a tick size,
   calendar/date syntax, duration unit, exact stale-after value, scheduler queue/data structure, fairness
   algorithm, UI clock format, first-playable temporal vocabulary, fixture, gate code, command, ratchet, or
   threshold. This mirrors spec 0031 §6 and the precedent report's altitude rule. The foundation tier is the
   **immutable governing reference** — you measure architecture against it; you never propose foundation edits.
5. **Architecture MAY name representational seams; it may not pick concrete values.** Block T should define
   the architecture-level temporal seams — e.g. a holder-known temporal-claim shape with provenance, the
   single authority that owns institution/procedural time and due/lateness state, the freshness classifier
   surface, the scheduler-vs-cognition temporal boundary, and the LOD temporal+information ancestry
   contract — without choosing the day-part vocabulary, calendar representation, duration units, or
   stale-after numbers (completeness-report §6 open questions #2/#3/#7; spec 0031 §6). Where the boundary
   between "representational seam" (architecture) and "concrete vocabulary/unit" (execution/spec) is
   genuinely ambiguous, state your altitude judgment and route the concrete choice forward.
6. **Forward-routing is non-degenerate.** Architecture is mid-stack, so the report's forward-routing
   appendix is real: hand execution/reference/future-spec findings to their later per-tier sessions (mirror
   completeness-report §5.3–5.5 and add the temporal execution/reference items 0031 §6 deferred). Do not
   amend those tiers here.
7. **Substance + home, not ratified text.** For each finding give (a) the doctrine the target architecture
   doc must own — written in *your own prose at architecture altitude* — and (b) the precise file and
   section/addition/correction it lands in. Provide **no** paste-ready final wording and **invent no**
   identifiers (`INV-###`, `A##-` codes beyond your own finding labels, gate codes, glossary IDs). Ratified
   text and identifiers remain the repository's own reassess/amend process.

---

## 4. The task

Determine what realignment the architecture tier (`docs/1-architecture/*`) now warrants, given (a) the
newly-ratified foundation **temporal-authority doctrine** (`INV-112` + foundation `03`; spec 0031), whose
architecture cascade was explicitly deferred, and (b) the **seven non-temporal themes** the foundations-
completeness determination routed to architecture (its §5.2). This is a **foundational / doc-overhaul
downward-cascade realignment**: the upstream tier was amended, and architecture must be measured against it
and brought into alignment, while surplus findings route further down. Produce a single consolidated
analysis / recommendation report that, finding by finding, names the driver, assesses current architecture
coverage, renders a tier-fit verdict, and recommends the doctrine substance + home — leaving ratified
wording and identifiers to the repo's amendment process.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 (other `docs/`, `archive/`, and —
only as supporting documentation evidence, never as independent implementation authority — crate source
under `crates/` if it clarifies a subsystem boundary). Research online as deeply as needed — similar
implementations, research papers, prior art — wherever it sharpens a tier-fit verdict or a recommendation:
discrete-event / agent-based-modeling time semantics and scheduling, deterministic-simulation budgets and
fairness, social-simulation play legibility, emotion/affect modeling, provenance/lineage for fungible
quantities, policy-as-code / schema-as-compiler authoring validation, and inspectable-assumption / bias
documentation practice. The completeness report §7.2 already cites a relevant external set (DeMO,
continuous-time ABM, ABM scheduling, OCC, W3C PROV, JSON Schema, OPA, deterministic-sim testing,
deterministic lockstep, NIST AI RMF, Datasheets for Datasets, Lean MVP, technical debt, ADRs, fitness
functions) — extend or update it as needed. **Cite every external source that shapes a decision.**

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every recommendation must satisfy
  it — especially the truth-firewall family `INV-099…INV-111` and the new temporal-authority invariant
  `INV-112` ("authoritative time may validate ordering/intervals/legality/replay/due effects; cognition,
  routines, institutional procedure, embodied views, speech, leads, and LOD promotion may use temporal facts
  only when those facts reached the holder through modeled channels"). You may not propose weakening or
  re-numbering an invariant; a genuine need to diverge would require a foundation amendment first, which is
  out of scope here.
- **Authority order:** foundation governs architecture. If architecture text conflicts with foundation, the
  architecture text is wrong and the report flags it as a correction. Never recommend that foundation bend
  to architecture.
- **Anti-contamination (the recurring firewall test):** no simulation fact may be born from prose; preserve
  event-sourced causality, subjective/holder-known epistemics, ordinary agents, possession parity, fallible
  institutions, questless leads, validation/replay, and the debug/embodied quarantine. The temporal seam is a
  *tempting* hidden-truth channel (raw clock access does not "feel" like a secret) — every Block T
  recommendation must keep the scheduler/replay clock from becoming a cognition oracle.
- **No backwards-compatibility shims or alias paths** in any recommended doctrine.
- Trim to the constraints the architecture tier actually engages; do not import unrelated doctrine.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **`reports/architecture-tier-alignment-research-report.md`** — **new** (it does not replace the same-basename
  file under `archive/reports/`, which is the prior campaign's archived predecessor and stays put).
- This is an **analysis / recommendation report, NOT a numbered spec.** Do **not** apply spec
  numbering / ledger / epoch rules; do not place anything in `docs/4-specs/`; invent no `INV-###` or gate codes.
- **Structure** (reuse the canonical campaign shape from `archive/reports/architecture-tier-alignment-research-report.md`):
  1. **Disposition table** — one row per finding → target architecture doc(s) → verdict (`belongs in
     architecture` / `partial coverage` / `already-owned-close` / `correction` / `route-forward`) → one-line basis.
  2. **Method & provenance ledger** — exact-commit fetch list, freshness posture, stale-provenance quarantine
     note for the seed reports' embedded older commit strings.
  3. **Foundation delta architecture must now translate** — the temporal-authority doctrine (`INV-112` +
     foundation `03`) and the seven routed-theme foundation drivers, stated as what architecture must encode.
  4. **Architecture sweep coverage register** — one row per `docs/1-architecture/00…14` with a
     current-alignment verdict and a short note (which docs need temporal seams, which routed-theme docs are
     partial vs. already-owned-close, which need no change).
  5. **Per-finding analysis** — for each Block T seam and each Block R theme: foundation driver → current
     architecture coverage → tier-fit verdict → recommendation (doctrine substance + target file/section, your
     own prose, no ratified wording).
  6. **Forward-routing appendix** — execution / reference / future-spec hand-offs (temporal mechanism values,
     fixtures, gates, thresholds, terminology, risk-register entries), mirroring completeness-report §5.3–5.5
     plus the temporal lower-tier items spec 0031 §6 deferred. Architecture is mid-stack, so this appendix is
     substantive — not degenerate.
  7. **Open questions** — owner decisions you cannot settle from the docs (carry forward completeness-report §6
     and any new ones; do not resolve them).
  8. **References** — exact-commit repository sources + every external source cited, with URLs.

Locked / no-questions instruction (honor verbatim):

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable is exactly one new file, `reports/architecture-tier-alignment-research-report.md`, an
  analysis/recommendation report (no numbered spec, no `docs/4-specs/` artifact, no invented identifiers).
- Both scopes are covered: Block T (temporal-authority architecture cascade) AND Block R (the seven §5.2
  routed themes), in one consolidated report.
- Every finding gives doctrine **substance + a target file/section**, in your own architecture-altitude prose
  — no paste-ready ratified wording, no `INV-###` / gate-code / glossary-ID invention.
- Every recommendation satisfies the constitution (`INV-099…INV-112`) and the authority order; none proposes
  weakening foundation or pushing lower-tier mechanism (tick size, calendar syntax, durations, thresholds,
  fixtures, gates) up into architecture — those are routed forward.
- Block R findings state current coverage first and use the validate-before-gap verdicts; no theme is called a
  gap merely because the seed named it.
- Already-settled 0026/0027 architecture work is built upon, not re-commissioned.
- The forward-routing appendix hands execution/reference/spec items to their later sessions (non-degenerate).
- Every external claim that shaped a decision is cited.
- Every file was fetched from commit `ea6a05b`; that commit contains every file named in §2 (it does); any
  embedded older commit string in the seed reports was quarantined, not used as a fetch target.
