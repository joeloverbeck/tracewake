# Research brief — Foundation-tier alignment (operationalize the time-doctrine promotion)

> **You are ChatGPT-Pro, Session 2.** This prompt is self-contained. Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-14_9c1203f.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first; agents act from
partial belief and every event leaves a replayable trace). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones.** Fetch every file from commit **`9c1203f`** (verified repo `HEAD`) — the
manifest reflects exactly that tree. (The determination report named below cites an older
"target commit of record," `d7fc746`; that was *its* baseline. `HEAD` has since advanced by a
merge. Use `9c1203f`, not the report's string; the foundation-tier files this pass touches are
unchanged between the two, so the report's findings carry forward verbatim.)

**This brief is a delta, not a cold start — it is the next link in a chartered campaign chain.**

- **Originating seed:** `reports/verdict-on-foundations.md` — an external design review that raised
  **nine design-completeness themes** against the foundation (play legibility; time/calendar/social
  rhythm; quantity/granularity/fungibility; bounded affect; learning/adaptation; authoring/compiler
  discipline; deterministic performance/fairness budgets; bias/social-harm practicality; and
  staged-incompleteness as an escape valve from over-constitutionalization).
- **Immediate predecessor (your direct mandate):**
  `reports/foundations-completeness-determination-research-report.md` — the determination that ran
  the **tier-fit test** on those nine themes and decided where each belongs. **Its verdict is settled
  input for you, not something to re-derive.** It promoted **exactly one** theme to the foundation —
  **time / calendar / social rhythm** — and routed the other eight to architecture, execution,
  reference, or future scoped specs. Its `§3.2` (per-theme determination for time), `§5.1` (the
  "Tier 0 — foundation session" hand-off), and `§6` (open questions) are the direct drivers of this
  pass. This is the **freshest, most-specific seed**; where it and the originating verdict differ in
  emphasis, the determination governs.
- **Predecessor brief (provenance only):**
  `reports/foundations-completeness-determination-research-brief.md` — the brief that commissioned the
  determination. Read it to understand the campaign's discipline and boundaries; it is not a finding
  source.
- **Prior structurally-identical campaign (method template + do-not-re-litigate boundary):** a
  previous pass, seeded by the `0006`–`0025` hardening campaign, ran the same shape —
  `archive/reports/foundation-amendment-research-report.md` (its determination) +
  `archive/reports/foundation-amendment-lower-tier-routing.md` (its routing memo) — promoted exactly
  one theme to the constitution, ratified as
  `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`, and drove
  now-complete lower-tier realignment. Use it as the **shape precedent** for your report and as the
  **settled-boundary** you must not re-open (its "settled seven" themes are closed).

**What this pass produces and where it sits.** The determination decided *what* belongs in the
foundation (one thing: temporal authority doctrine). **You decide *how* it should be encoded** — which
foundation documents change, what doctrine each must own, and at what altitude — as a **recommendation
report**, not as ratified doctrine text. This is **iteration 1 of several**: later sessions will take
the eight routed themes down through `1-architecture`, `2-execution`, `3-reference`, and future specs,
each accounting for the foundation temporal amendment landing first. Your forward-routing appendix sets
them up; you do not perform them.

## 2. Read in full (authority order)

Read these before producing. Order is by authority tier. Every path resolves at commit `9c1203f`.

**Primary — the amendment target (the foundation tier in full; this is where every recommendation lands or is ruled out):**

```
docs/README.md — the authority ordering and the layering rule; this IS the tier-fit test you apply.
docs/0-foundation/00_FOUNDATION_INDEX.md — what the foundation currently claims to cover (the map a new section must slot into).
docs/0-foundation/01_PROJECT_CHARTER.md — the ambition and the explicitly-refused wrong products.
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; you must decide whether temporal authority needs a new/amended invariant here (see §3).
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md — PRIMARY home for the temporal authority doctrine; event/replay time lives here.
docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md — cross-ref target: "yesterday/last night/late/stale/expected-by-now" are holder-known claims, not free truth labels.
docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — cross-ref target: routines/schedules consume actor-known/institution-known temporal expectations.
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md — confirm coverage: hunger/fatigue/sleep/wages/debt/travel all run over time; check whether a cross-ref is warranted.
docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md — cross-ref target: institutional/procedural time (office windows, deadlines, filing delays, procedural aging).
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md — confirm coverage: time controls at the product boundary; check whether embodied views need a temporal-firewall cross-ref.
docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md — boundary check: confirm temporal doctrine does not duplicate no-script/prehistory authoring rules.
docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md — cross-ref target: LOD summary intervals and regional/seasonal cadence connect to the temporal model.
docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md — boundary check: confirm temporal claims in speech stay holder-known, not free truth.
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md — cross-ref target: first playable proof must not treat time as an invisible oracle for actor decisions.
docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md — the precedent for how foundation decisions cite research; where a temporal source note would land.
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md — the truth firewall every recommendation must preserve: truth may validate, truth may not plan.
```

**Primary — the mandate (your settled instructions; read closely, do not re-derive):**

```
reports/foundations-completeness-determination-research-report.md — the determination. §3.2 (time per-theme), §5.1 (Tier 0 hand-off), §6 (open questions) drive this pass; its routing of the other eight themes is settled.
reports/verdict-on-foundations.md — the originating nine-theme review (the seed behind the determination).
```

**Method template + do-not-re-litigate boundary (read to mirror shape and respect closed work):**

```
reports/foundations-completeness-determination-research-brief.md — predecessor brief; campaign discipline and boundaries (provenance, not a finding source).
archive/reports/foundation-amendment-research-report.md — prior-campaign determination; report-shape precedent and its "settled seven" closed themes.
archive/reports/foundation-amendment-lower-tier-routing.md — prior-campaign routing memo; the forward-routing-appendix shape precedent.
archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md — the most recent ratified foundation amendment; shows how 02/03 were last amended so your recommendation aligns in altitude and form.
```

**Boundary-awareness — read to bound scope, NOT amendment or conformance targets.** These let you run
the tier-fit test (confirm temporal *mechanism* belongs below the foundation) and shape the
forward-routing appendix. Do not recommend edits to them in this pass and do not audit the code behind
them:

```
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md — where actor-time decision mechanism will land later.
docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md — acquired/stale-after tick mechanism (architecture, not foundation).
docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md — summary-interval / time-acceleration mechanism (architecture, not foundation).
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md — no-wall-clock / deterministic-ordering proof (execution, not foundation).
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md — locked LOD/time-acceleration deferrals (execution, not foundation).
docs/3-reference/01_DESIGN_RISK_REGISTER.md — where temporal-relapse risks would be tracked (reference, not foundation).
docs/3-reference/02_GLOSSARY.md — where temporal terms would be defined (reference, not foundation).
docs/4-specs/SPEC_LEDGER.md — confirms this is a report, not a numbered spec; no ledger entry is produced here.
```

## 3. Settled intentions

These were resolved before this brief was issued. Treat them as final; they pre-empt every question
you might otherwise ask.

1. **One promotion only.** Accept the determination report's verdict as settled. Of the nine verdict
   themes, **only time / calendar / social rhythm** belongs in the foundation. Do **not** re-open,
   re-adjudicate, or promote any of the other eight themes, and do **not** re-litigate the prior
   campaign's "settled seven." Their disposition is closed.
2. **Deliverable is a recommendation report, not ratified doctrine text.** For each foundation finding,
   deliver **substance + home**: *what doctrine the target foundation doc must own* (your own prose, at
   foundation altitude) and *which file it lands in* (new section / inline addition / cross-reference).
   Do **not** write final paste-ready doctrine wording, and do **not** mint identifiers — no new
   `INV-###` numbers, no gate codes, no glossary IDs. Minting identifiers and ratifying text remain the
   repository's own reassess/amend process.
3. **`02_CONSTITUTIONAL_INVARIANTS.md` is in scope.** The determination calls temporal authority
   "constitutional," yet its `§5.1` routing named only `03` plus cross-refs in `04/05/07/10/12` — not
   `02`. You must **explicitly resolve this**: determine and argue whether the temporal-authority
   doctrine warrants a **new or amended constitutional invariant** in `02` (or whether doctrine prose in
   `03` plus the cross-refs is sufficient, with `02` left untouched). Recommend the invariant's
   *content and placement* if warranted; do **not** assign it a number. A "constitutional" doctrine
   absent from the invariants document is incoherent — so either justify its presence in `02` or justify
   why it stays out.
4. **Resolve the placement-shape open question (determination `§6.1`).** Recommend, with rationale,
   whether the temporal doctrine is best encoded as **one new section in `03`** or as a **compact
   cross-doc mini-package** spanning `03` plus targeted cross-references in `04/05/07/10/12` (and, per
   intention 3, possibly `02`). State the trade-off (single authoritative home vs. distributed clarity)
   and commit to a recommendation.
5. **The truth firewall is inviolable.** Every recommendation must preserve `14`: authoritative
   simulation time may **validate** event ordering and action legality, but cognition, institutional
   procedure, embodied views, and LOD promotion may use only temporal facts available through modeled
   channels — perception, memory, records, routines, roles, or accepted summary ancestry. Truth may
   validate; truth may not plan.
6. **Stay at foundation altitude.** The foundation must **not** choose tick size, calendar/date syntax,
   duration units, scheduler data structures, UI clock rendering, or exact stale-after durations. Those
   are architecture/execution/spec choices and route below. The foundation owns only the *conceptual
   authority model*: what kinds of time exist in Tracewake and which kinds may feed cognition.
7. **New foundation findings are flagged, never amended.** If you discover a genuinely new
   foundation-level gap beyond the nine themes, record it as an **open question** for owner decision. Do
   not promote or amend it in this pass.
8. **Forward-routing is the top-tier "route-down" case.** There is no tier above the foundation. The
   eight routed themes belong to the later per-tier sessions; your forward-routing appendix hands each
   to its destination tier (architecture → execution → reference → future specs), noting that each must
   account for the foundation temporal amendment landing first. Do not encode their mechanisms here.

## 4. The task

This is a **foundational / doc-overhaul** pass at the **top tier** (`0-foundation`). The determination
report has already decided that exactly one theme — temporal authority doctrine — is promoted to the
foundation. Your task is to produce a precise, evidence-grounded **recommendation report** that
operationalizes that single promotion: confirm what the current foundation already owns about time,
identify the exact gap (no named authority split among event time, actor-perceived time, institutional
time, routine/social rhythm, deadlines/lateness, freshness/staleness, and LOD summary intervals),
decide whether the doctrine needs a constitutional invariant in `02`, recommend the placement shape,
and specify — per affected foundation document — *what doctrine it must own* and *where*, all while
preserving the truth firewall and staying at foundation altitude. The remaining eight themes are routed
forward, not amended here.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above. Research online as deeply as
needed — discrete-event/agent-based simulation time semantics, temporal ontologies, calendar/social-rhythm
modeling in social simulations, deterministic-replay time handling, and any prior art that sharpens *how
a foundation should name temporal authority without over-specifying mechanism*. The determination report
already cites relevant prior art (DeMO, continuous-time ABM, ABM scheduler literature); extend or
challenge it where useful. Cite sources for any external claim that shapes a recommendation.

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every product-behavior
  decision must satisfy it; a genuine divergence requires amending an invariant first, never designing
  against it silently. (Here you may *recommend* such an amendment — see intention 3 — but not author or
  number it.)
- Authority order: if execution conflicts with architecture or foundation, execution is wrong; if
  implementation is more convenient than the accepted gates, implementation is wrong. The foundation is
  the top of this order; it answers to nothing above it.
- The truth firewall (`14`): authoritative time may validate; only modeled/holder-known temporal facts
  may plan, decide, render in embodied views, or drive institution-known procedure.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, validation/replay.
  A temporal doctrine that let clock omniscience leak into cognition (a worker "knowing" a shop is
  closed because the scheduler read true office hours; a debt becoming late without a modeled due rule;
  a witness recalling exact ticks instead of actor-plausible temporal claims) is exactly the failure
  this promotion exists to prevent.
- No backwards-compatibility shims or alias paths in new work.

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **`reports/foundation-tier-alignment-research-report.md`** — **new** (not a replacement). This is an
  **analysis / recommendation report**, not a numbered spec: it does **not** go in `docs/4-specs/`, gets
  **no** `SPEC_LEDGER.md` entry, and the spec-numbering rules do not apply. Its filename sets the
  campaign-wide per-tier convention (later sessions produce `architecture-tier-alignment-research-report.md`,
  and so on).

Use this structure (the campaign archetype, mirrored from the prior pass's reports):

1. **Executive determination** — the one promotion (temporal authority) restated as a foundation
   amendment plan; your verdict on the `02`-invariant question and the placement shape, in one or two
   paragraphs.
2. **Disposition table** — one row per *affected foundation document* (`02`, `03`, `04`, `05`, `07`,
   `10`, `12`, and any other you justify) → verdict (new section / inline addition / cross-reference /
   no change) → one-line basis. Include a row that explicitly records each *unaffected* foundation doc
   you checked and ruled out, so the table proves tier-wide coverage.
3. **Method & provenance ledger** — exact-commit fetch discipline (every file from `9c1203f`),
   tier-fit test applied, the settled boundaries you did not re-open (the other eight themes; the prior
   "settled seven").
4. **Per-finding sections** — for each affected foundation doc: *driver* (what the determination /
   verdict asks) → *current coverage* (what that doc already owns about time, grounded in named live
   text) → *placement / tier-fit verdict* (foundation-altitude, firewall-preserving) → *recommendation*
   (what doctrine it must own and where, in your prose — no paste-ready wording, no minted IDs). The
   `02`-invariant analysis is its own finding section.
5. **Forward-routing appendix** — the eight routed themes, each handed to its destination tier
   (architecture / execution / reference / future specs), with the note that each later session must
   account for the foundation temporal amendment landing first. This is the top-tier route-down case:
   nothing routes *up*.
6. **Open questions** — owner decisions you cannot settle from the docs (carry forward the
   determination's relevant `§6` items — e.g. minimal first-playable calendar vocabulary, which the
   foundation must not pick but architecture will need — plus any genuinely new foundation gap per
   intention 7).
7. **References** — repository sources (by exact-commit path) and any external sources cited.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a requirement
> impossible, state it in the deliverable and proceed with the most faithful interpretation.

## 8. Self-check

Before returning, verify against your own output:

- Every recommendation lands in a named `docs/0-foundation/*` file and is grounded in that file's
  current live text (current-coverage stated before any gap is claimed).
- The `02`-invariant question is explicitly resolved with an argued recommendation (in, or out, with
  reasons) — not left implicit.
- The placement-shape question (one section in `03` vs. cross-doc package) is resolved with rationale.
- No recommendation weakens the truth firewall (`14`) or any existing invariant; the temporal doctrine
  preserves "truth may validate, truth may not plan."
- No foundation-altitude line picks tick size, calendar syntax, units, scheduler structures, UI clock
  rendering, or stale-after durations.
- Only the one theme is promoted; the other eight appear *only* in the forward-routing appendix; the
  prior "settled seven" are untouched.
- No paste-ready doctrine text and no minted `INV-###`/gate/glossary identifiers.
- The deliverable set matches §7 exactly: one new report, no numbered spec, no ledger entry.
- Every external claim that shaped a recommendation is cited.
- Commit `9c1203f` contains every file named in the §2 read-in-full list (it does).
