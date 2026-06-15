# Reference- and Specs-Tier Alignment — ChatGPT-Pro research brief

You are ChatGPT-Pro Session 2. Produce the deliverables directly. Do **not** interview or
ask clarifying questions — the requirements below are final.

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-15_cda3325.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier tiers
govern later ones. **Fetch every file from commit `cda3325` (full:
`cda3325b0777f25101c9a04af3daeef24913f137`) — the manifest reflects that exact tree.**

**This brief continues a multi-session campaign — the "temporal authority + completeness alignment"
epoch (epoch-2).** It is a *delta*, not a cold start. The campaign chain, in order:

1. `reports/verdict-on-foundations.md` — the original nine-theme seed.
2. `reports/foundations-completeness-determination-research-report.md` — the umbrella determination:
   promoted **time/calendar/social rhythm** to the foundation; routed the other eight themes below
   foundation. Its §5.4 (Tier-3 reference) and §5.5 (Tier-4 specs) forward-routing tables are direct
   inputs for this pass.
3. `reports/foundation-tier-alignment-research-report.md` → ratified as archived spec
   `0031` (foundation `INV-112` + temporal-authority section).
4. `reports/architecture-tier-alignment-research-report.md` → ratified as archived spec `0032`
   (architecture Block T temporal + Block R completeness). Its §6.2 routes reference-tier hand-offs.
5. `reports/execution-tier-alignment-research-report.md` → ratified as archived spec `0033`
   (execution Block T + Block R + Block S staged-incompleteness). **This is the freshest, most-specific
   seed for this pass:** its §6.1 enumerates reference-tier hand-offs and §6.2 the future-scoped-spec
   hand-offs. When it and earlier predecessors conflict, prefer it.

The upstream tiers `0-foundation`, `1-architecture`, `2-execution` have all been amended and ratified
(`0031`/`0032`/`0033`, now archived). **The two lowest tiers — `docs/3-reference/*` and
`docs/4-specs/*` — have not yet been realigned to those amendments. That realignment is this pass.**

**Prior epoch (epoch-1) is settled and must not be re-commissioned.** A separate, earlier campaign
(`EMERGE-OBS` / observer-only emergence-evidence doctrine) already cascaded through reference (archived
spec `0029`) and specs (archived spec `0030`). Its reference/specs research lives at
`archive/reports/reference-tier-alignment-research-report.md` and
`archive/reports/specs-tier-alignment-research-report.md` (pinned to the older commit `36b4082`; those
strings are that epoch's own baseline — do not adopt them, use `cda3325`). Epoch-1's landed reference
content (the `EMERGE-OBS` glossary term, the R-27/R-28/R-29 acceptance-evidence-honesty cluster, the
R-26 archive cross-link) and epoch-1's landed specs content (the `0003` evidence-status /
fingerprint-scope / behavior-witness / replay-provenance / sampling / pending-historical /
certification-use fields) are **already in the live docs at `cda3325`**. Treat them as the current
baseline; add only the epoch-2 delta on top.

---

## 2. Read in full (authority order)

Read these from commit `cda3325` before producing. Authority flows downward: tiers 0–2 are the
**immutable governing reference** (you measure the target tiers against them; you never amend them).
Tiers 3–4 are the **amendment targets**.

`docs/README.md` — authority order and the layering rule.

**Tier 0 — foundation (governing reference; immutable).**
- *Primary:* `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; every
  recommendation must satisfy these. `INV-112` is the new temporal invariant ("time may validate, but
  holder-known time must plan").
- *Primary:* `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — the ratified
  temporal-authority section; the constitutional home of the doctrine this pass terminologizes.
- *Boundary-awareness (read to bound scope, not a conformance target):*
  `docs/0-foundation/00,01,04,05,06,07,08,09,10,11,12,13,14` — confirm what the constitution already
  owns so the reference report does not re-state foundation doctrine. `14` (truth firewall) governs the
  anti-contamination risks; `04`/`05`/`06`/`07`/`10`/`12` carry the temporal cross-references `0031`
  added.

**Tier 1 — architecture (governing reference; immutable).**
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — index/conformance map.
- *Primary* (the subsystem contracts the epoch-2 amendment touched; their wording is what reference
  terminology/risk memory must track): `02` (event/replay time, holder export ancestry), `03`
  (holder-known temporal claims, provenance, freshness), `04` (scheduler temporal firewall + budget),
  `05` (routine/social-rhythm premises, affect, learning, planner budgets), `06` (claims/beliefs,
  affect/memory, learned expectations), `08` (institutions, practical bias), `09` (quantity/granularity/
  fungibility/custody), `10` (possession/TUI view models, play-legibility), `11` (incidents/leads/
  story-sifting), `12` (LOD/regional/time-acceleration), `13` (validation/observability/budget-fairness).
- *Boundary-awareness:* `01,07,14`.

**Tier 2 — execution (governing reference; immutable; freshest upstream wording).**
- *Primary:* `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` and
  `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` (temporal routing map, certification
  placement, staged-declaration discipline); `04` (temporal firewall gates), `05` (scheduler/budget
  proof), `06` (routines/no-human/learning proof), `07` (epistemic view models / temporal rendering),
  `08` (data authoring / quantity / compiler discipline), `09` (golden fixtures / fixture families),
  `10` (testing/diagnostics — consolidated budget/fairness + staged-abstraction review), `11`
  (institutions/wrong-suspicion/practical bias), `12` (deferred LOD/time-acceleration ancestry).
- *Boundary-awareness:* `01,02,13`.

**Tier 3 — reference (AMENDMENT TARGET — deliverable report 1).**
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — checklist + tier boundary +
  source-discipline; receives checklist pointers/questions for the new terms.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — risk watchlist currently R-00…R-29; receives the new
  temporal + completeness relapse risks.
- `docs/3-reference/02_GLOSSARY.md` — canonical terms; receives the new temporal + completeness terms.

**Tier 4 — specs (AMENDMENT TARGET — deliverable report 2).**
- `docs/4-specs/README.md` — specs-tier authority statement.
- `docs/4-specs/SPEC_LEDGER.md` — active/archived spec ledger; records the campaign lineage `0026…0033`.
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — first-proof
  ontology/fixture spec; boundary-awareness conformance check.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the live review-artifact template; the one
  concrete near-term edit target (staged-abstraction declaration fields).

**Live reports (epoch-2 predecessors — the seeds; read for routing substance).**
- `reports/execution-tier-alignment-research-report.md` — **freshest seed**; §6.1 reference hand-offs,
  §6.2 future-spec hand-offs, §7 open questions.
- `reports/architecture-tier-alignment-research-report.md` — §6.2 reference hand-offs.
- `reports/foundation-tier-alignment-research-report.md` — foundation delta context.
- `reports/foundations-completeness-determination-research-report.md` — umbrella determination; §5.4
  (reference) and §5.5 (specs) forward-routing tables.
- `reports/verdict-on-foundations.md` — original nine-theme seed.

**Archived epoch-2 amendment specs (read their Out-of-Scope / forward-routing sections — they
explicitly enumerate what is routed to reference and specs).**
- `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` — §6 Out of Scope.
- `archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md` —
  §5 V4 + §6 Out of Scope (the "Reference-tier terminology and risk memory" bullet is a direct spec).
- `archive/specs/0033_EXECUTION_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md` —
  §5 V5 + §6 Out of Scope (reference-tier terms/risks + future-spec routing + the `0003` template route).

**Archived epoch-1 (SETTLED context — boundary-awareness; read to know what NOT to re-commission).**
- `archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — epoch-1 reference amendment.
- `archive/specs/0030_SPECS_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — epoch-1 specs amendment (`0003`
  evidence-honesty fields).
- `archive/reports/reference-tier-alignment-research-report.md` and
  `archive/reports/specs-tier-alignment-research-report.md` — epoch-1's terminal reports (note: pinned
  to `36b4082`; that is their own baseline — do not propagate it).

---

## 3. Settled intentions

These decisions are final. They pre-empt every clarifying question.

1. **This is a downward-cascade realignment (epoch-2), terminal tiers.** `docs/3-reference/*` and
   `docs/4-specs/*` are realigned to the already-ratified upstream amendments `0031`/`0032`/`0033`.
   Tiers `0`/`1`/`2` are the **immutable governing reference** — measure against them; never recommend
   amending them. If you find an apparent upstream gap, record it as an open question routed upstream,
   never patch it into a reference/specs recommendation.

2. **Both drivers, full coverage — reference report.** The epoch-2 amendment carried two bodies of
   work, and both were routed to reference; cover both:
   - **Block T — temporal authority:** glossary terms and risk memory for event/replay time, scheduler
     time, holder-known temporal claim, institution-known procedural time, temporal firewall,
     freshness/staleness, validity window, temporal ancestry, information ancestry, time acceleration,
     LOD summary cadence; relapse risks for clock-oracle leakage, raw-wall-clock contamination,
     omniscient lateness/office-closed labels, UI time-acceleration leaks, debug-time-becoming-diegetic,
     and silent LOD temporal fill-in.
   - **Block R + Block S — completeness themes:** quantity/granularity/fungibility (custody/lineage
     terms; quantity/economy lineage-collapse risk), bounded affect (affect/salience vs. truth terms;
     affect-as-hidden-truth / decorative-meter risk), learning/adaptation (learned-expectation vs.
     memory terms; truth-cache-learning risk), deterministic performance/fairness (budget/starvation
     terms; performance-pressure-as-invisible-director and budget-starvation-hidden-by-aggregate-success
     risks), practical bias/social harm (bias/credibility/social-position/protected-category/
     domain-pack-assumption/wrong-suspicion terms; emergent-injustice-as-author-prejudice risk),
     staged incompleteness (staged-abstraction / false-certification terms; staged-abstraction
     false-certification risk), and play-legibility (dual-relapse risk: correct-but-unplayable audit
     machine vs. "playability" achieved by leaking truth/objectives).
   - **Altitude:** add completeness-theme terms/risks at **review-aid altitude, conditioned on what
     `0032`/`0033` actually locked** — not speculative doctrine and not concepts execution left to
     future specs. Where architecture/execution explicitly deferred a concept's depth to future specs
     (affect/learning depth, bias vehicle, budget numerics), the reference entry is a cautious review
     aid or a deferred-term note, not a settled definition.

3. **Specs report = one concrete edit + a backlog register + boundary checks.** The specs tier is
   doctrinally **not an amendment target** (it operationalizes higher tiers; it cannot define doctrine).
   The specs report therefore delivers:
   - **the one concrete near-term edit:** staged-abstraction declaration fields for
     `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — *proves now / deliberately abstracts /
     must not fake / must not block / evidence preventing overclaiming / failure diagnostics* (the
     `0033` D-S1 / execution F-13 hand-off). State substance and home; do not write final template
     wording or mint a new gate/obligation code;
   - **a future-scoped-spec backlog register** with scope boundaries for each: concrete temporal
     vocabulary, concrete temporal thresholds, the first temporal-firewall fixture package, the
     inventory/economy fixture package, affect/learning depth, the domain-pack bias vehicle,
     budget/fairness numeric targets, authoring/compiler policy, and a TUI/play-loop fixture spec.
     For each: what it owns, what it must not bundle (the temporal-firewall and inventory/economy
     packages must **not** be bundled unless one gameplay feature truly needs both), and which upstream
     contract governs it;
   - **boundary-awareness conformance checks** on `docs/4-specs/README.md`, `SPEC_LEDGER.md`, and
     `0001` — verify no doctrine drift against the amended upstream; epoch-1 found these no-change, so
     the expected verdict is *boundary-awareness / no-change* unless you find a concrete drift, in which
     case state it precisely.

4. **Settled context not re-commissioned.** Do not re-open or re-derive epoch-1's landed work: the
   `EMERGE-OBS` glossary term, the R-27/R-28/R-29 acceptance-evidence-honesty cluster + R-26 archive
   cross-link, and the `0003` evidence-status/fingerprint-scope/behavior-witness/replay-provenance/
   sampling/pending-historical/certification-use fields. These are the current baseline at `cda3325`;
   the epoch-2 staged-abstraction fields are additive on top of them. Likewise do not re-open the
   "settled seven" anti-contamination themes. Each report must carry an explicit
   *"Settled context not re-commissioned"* section naming this boundary.

5. **Recommendation reports — substance + home, not ratified text.** For each finding give: *what
   doctrine the target reference/specs doc must own* (your own prose at that tier's altitude) and
   *which file + which section/addition/correction* it lands in. Do **not** write paste-ready glossary
   entries, final risk-register wording, or final template text, and do **not** invent identifiers
   (`R-30`, new `INV-###`, gate codes, `EMERGE-OBS`-style tokens). Numbering and final wording remain
   the repo's own reassess/amend process. Naming a *recommended* future spec number (e.g. "this becomes
   spec `0034` for reference, `0035` for specs") as forward-routing guidance is fine.

6. **Reference ratifies no concrete temporal values.** Reference stabilizes **terminology + risk memory
   + checklist pointers**. It must not pick day-part vocabulary, "yesterday"/"last night"/"office
   closed"/"due/late"/"stale" phrasing, calendar/date syntax, duration units, stale-after thresholds,
   office-hour representation, time-acceleration speed, or simultaneity/tie-break rules — all of those
   route forward to future scoped specs. Glossary entries name the *concept and authority*, not the
   chosen surface string.

7. **Terminal tier — forward-routing degenerates.** Nothing routes further *down*. The forward-routing
   appendix of each report must say so explicitly and route only to (a) owner/reassess decisions and
   (b) future implementation specs. Carry — do **not** decide — the deferred **possession-bind
   perception** owner question (epoch-1 F04; `0032` Risk R-F): if ever decided, bind-time perception
   must be a modeled event/channel for the actor, never a human/player knowledge transfer.

8. **Doctrine guardrails (see §6).** Every recommendation must satisfy the constitution, preserve the
   truth firewall, and add no doctrine that weakens an upstream tier.

---

## 4. The task

This is a **foundational / doc-overhaul pass — specifically a downward-cascade realignment of the two
terminal documentation tiers** to ratified upstream amendments. Produce two recommendation reports that
tell the maintainer exactly what `docs/3-reference/*` and `docs/4-specs/*` must own so they are
consistent with the temporal-authority + completeness doctrine now ratified in foundation/architecture/
execution (`INV-112`, foundation `03`, and the `0032`/`0033` cascades), without re-stating upstream
doctrine, without ratifying concrete values, and without re-commissioning the settled epoch-1 work.
Each finding must be grounded in the named upstream wording at `cda3325` and routed to a precise home.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2. Research online as deeply as
needed — comparable social-simulation / agent-based-simulation projects, glossary and risk-register
discipline, temporal-ontology and discrete-event-simulation terminology, provenance/anti-contamination
practice, and any prior art that sharpens how a reference tier should terminologize temporal authority
and bounded affect/learning/bias without over-committing. Cite every external source that shapes a
recommendation. External research informs *tier-fit and terminology judgment only*; it never becomes
Tracewake doctrine and never overrides the repo's own authority order.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every recommendation must
  satisfy it (notably `INV-112`, and the truth-firewall / anti-contamination invariants). A genuine
  divergence requires amending an invariant first (an upstream-routed open question), never designing
  against it in a lower tier.
- **Authority order:** if a reference/specs recommendation would conflict with architecture or
  foundation, the recommendation is wrong. Reference and specs are subordinate; they describe and
  operationalize, they do not redefine.
- **Truth firewall:** authoritative/perfect time, ground truth, debug truth, and fixture labels may
  *validate*; only modeled / holder-known / institution-known facts may *plan, decide, appear in
  embodied views, or drive procedures*. Every recommended term and risk must preserve this.
- **Anti-contamination:** no simulation fact may be born from prose; preserve event-sourced causality,
  subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads,
  validation/replay, and observer-only (non-certifying) emergence evidence.
- **No backwards-compatibility shims or alias paths** in recommended doctrine.
- **No invented identifiers or paste-ready text** (per §3.5).

---

## 7. Deliverable specification

Produce **two new downloadable markdown documents** (analysis / recommendation reports — **not**
numbered specs, so the spec-ledger/numbering/epoch rules do **not** apply to the report files
themselves):

### Deliverable 1 — `reports/reference-tier-alignment-research-report.md` (NEW)
A recommendation report for `docs/3-reference/*`. An archived epoch-1 namesake exists at
`archive/reports/reference-tier-alignment-research-report.md`; that is expected and is **not** a
replacement or collision — the live `reports/` path is confirmed absent at `cda3325`. Scope per §3.2.

### Deliverable 2 — `reports/specs-tier-alignment-research-report.md` (NEW)
A recommendation report for `docs/4-specs/*`. Same archived-namesake note. Scope per §3.3.

**Canonical report shape (both reports):**
1. Header — target repo, target commit `cda3325`, scope, deliverable type ("recommendation report
   only; not ratified wording; not a numbered spec").
2. **Disposition table** — one row per finding → target doc → verdict (`amend` / `add` /
   `boundary-awareness — no change` / `route-forward`) → one-line basis.
3. **Method & provenance ledger** — exact-commit fetch discipline; manifest = path inventory only.
4. **Settled context not re-commissioned** — the epoch-1 boundary (per §3.4) and the "settled seven".
5. **Per-finding sections** — driver → current coverage (what the live doc already owns at `cda3325`)
   → tier-fit verdict → recommendation (substance + home, no final wording, no invented IDs).
6. **Forward-routing appendix** — degenerate (per §3.7): state nothing routes further down; route only
   owner/reassess decisions and future implementation specs.
7. **Open questions** — owner decisions the docs cannot settle.
8. **References** — exact-commit repository sources + external sources cited.

**Determination note for the specs report:** because the specs tier is not an amendment target, the
specs report's disposition is expected to be dominated by `route-forward` (future scoped specs) plus
the single `0003` `amend` row and `boundary-awareness — no change` rows for README/LEDGER/0001. State
this posture explicitly so a thin amend-set reads as the *correct terminal-tier outcome*, not as
missed coverage.

**Locked / no-questions instruction:**

> Produce the deliverables directly as downloadable markdown documents. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- Two documents produced, with the exact filenames and `NEW` status in §7.
- Every recommendation is grounded in named upstream wording read from commit `cda3325`; no reliance on
  the predecessor reports' older baseline strings (`36b4082`, etc.).
- The reference report covers **both** Block T (temporal) and Block R/S (completeness) per §3.2, at
  review-aid altitude conditioned on what `0032`/`0033` actually locked.
- The specs report delivers the `0003` staged-abstraction edit + the future-spec backlog register +
  boundary checks per §3.3, with the terminal-tier posture stated.
- No upstream (tier 0/1/2) amendment is recommended; apparent upstream gaps are open questions only.
- No epoch-1 settled work is re-commissioned (`EMERGE-OBS` term, R-26/R-27/R-28/R-29, `0003`
  evidence-honesty fields, the settled seven); each report carries the "Settled context not
  re-commissioned" section.
- No concrete temporal values are ratified in the reference report (§3.6).
- No paste-ready wording and no invented identifiers anywhere (§3.5).
- The possession-bind perception question is carried forward, not decided.
- Every external claim that shapes a recommendation is cited.
- The `cda3325` baseline contains every file named in §2 (it does — verified at brief-authoring time).
