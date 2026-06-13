# Research brief — Architecture-tier alignment to the amended foundation (cascade pass)

**For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
directly. Do **not** interview, do **not** ask clarifying questions — the requirements below are
final. The interview already happened. **Upload bundle = this prompt + the manifest
`manifest_2026-06-13_fdfd0b9.txt`.**

Your job is an **internal doctrine-conformance analysis with forward routing**, not an
implementation. The foundation tier (`docs/0-foundation/*`) was recently rewritten and then amended
(it now carries a new constitutional document and a new invariant about emergence evidence). The
architecture tier (`docs/1-architecture/*`) was last substantively edited during the `0021`–`0025`
hardening campaign and predates the most recent foundation amendment. You are deciding **what
changes the architecture tier needs so it faithfully translates the *current* foundation** — both
the changes a prior audit already routed to architecture, and any additional ripple the recent
foundation rewrite created that no prior audit anticipated.

---

## §1 — Context, baseline commit, and source discipline

### What Tracewake is

Tracewake is a causality-first living-world simulation in Rust: agents act from partial belief,
institutions are fallible, and every meaningful event leaves a replayable, forensically explainable
trace. The product thesis is deliberately narrow and hard: a *small ordinary village* (not a quest
engine, not a player-centered mystery, not an LLM chatbot world) in which subjective belief,
event-sourced causality, fallible social machinery, and an actor-filtered TUI produce — with no
human present and no authored outcome chain — a chain like:

> I expected the coins to be in the chest. The chest is empty. I search. I ask. I remember. I
> suspect. I report. A record is made. Someone may be wrongly suspected. I delay payment or change
> my plan. Replay explains why every actor did what they did. The same chain can happen with no
> human present.

### Workspace layout (three crates, strict one-way dependency)

- `tracewake-core` — authoritative simulation kernel: event log, replay, actions/affordances,
  scheduler, projections, view models. **Zero dependencies.**
- `tracewake-content` — fixtures, content loading, schema validation. Depends on core only.
- `tracewake-tui` — terminal UI boundary: possession, embodied/debug view models. Depends on
  core + content.

Core must never depend on content or tui.

### Documentation is layered authority

Docs are tiered; earlier tiers govern later ones (index: `docs/README.md`):

```
0-foundation/  → constitutional doctrine: what Tracewake is and must never become
1-architecture/→ subsystem contracts: foundation translated into data-flow / authority / boundaries
2-execution/   → gate order, certification sequence, fixtures, proof obligations
3-reference/   → compact lookup, glossary (terminology control), design-risk register
4-specs/       → narrow implementation/corrective specs under live doctrine
archive/       → historical evidence (completed specs/tickets/reports), NOT live authority
```

Foundation outranks architecture outranks execution outranks reference outranks specs. A later
layer may *specialize* foundation doctrine but may not *weaken* it. **Architecture's job is to
translate foundation `what` into subsystem `how`-contracts** — data flow, authority boundaries,
ownership seams, which component may read/write/derive what — *without* descending to crate-internal
module layout, storage engine, serialization format, or algorithm choice (those are execution /
implementation). This `foundation → architecture` translation fidelity is the single most
load-bearing fact for your task: you are auditing whether the architecture tier still says what the
*current* foundation requires it to say.

### Why this pass exists — the foundation moved under the architecture tier

Two recent foundation changes are the source of the ripple you are tracing:

1. A **full foundation rewrite** expanded and restructured all of `docs/0-foundation/*` and
   introduced a new constitutional document, `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
   (the "truth firewall": *truth may validate actions, but truth may not plan them*).
2. A subsequent **emergence-evidence amendment** added a new invariant to
   `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **INV-111**, "Living-world acceptance
   requires observer-only emergence evidence" — plus reinforcing additions to
   `docs/0-foundation/09_…` ("Emergence observation is not steering") and
   `docs/0-foundation/12_…` ("Emergence evidence as acceptance evidence"). This amendment was
   ratified and archived as `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`.

The architecture tier was last edited around and before those changes and **predates the
emergence-evidence amendment entirely**. At minimum the architecture tier does not yet encode the
observer-only emergence-evidence data contract; the *full* extent of architecture's
non-conformance to the current foundation is what you must determine. (Confirm dates and content
against the fetched files — do not assume.)

### This brief continues a campaign — it is a *delta*, not a cold start

A prior audit already ran the foundation-amendment determination this pass descends from. Read its
three artifacts (named in §2):

- `reports/foundation-amendment-research-brief.md` — the brief that commissioned the determination.
- `reports/foundation-amendment-research-report.md` — the determination itself: it evaluated seven
  candidate themes drawn from the `0006`–`0025` hardening campaign, found exactly **one** foundation
  hole (emergence-as-evidence → archived spec `0026`), and judged the other six themes (plus the
  *mechanism* half of emergence) to belong **below** the foundation tier.
- `reports/foundation-amendment-lower-tier-routing.md` — **the routing memo**: it preserves where
  each below-foundation lesson should land, tier by tier. **Its `docs/1-architecture/*` session
  table is the validated seed for your pass.** It explicitly anticipates *this* session and four
  sibling sessions (`2-execution/`, `3-reference/`, `4-specs/`) to follow.

The routing memo is a **backlog/routing memo, not doctrine and not certification**. Treat each item
it routes to architecture as a *candidate to validate against live source*, not a settled
requirement — the hardening machinery may already cover much of it, and the memo itself says so.
Your pass is the architecture tier's turn: validate the memo's architecture items, **and** sweep for
ripple the memo never anticipated (it was scoped to `0006`–`0025` campaign lessons, not to the full
foundation rewrite).

### Baseline commit and fetch discipline (read carefully)

- **Fetch baseline commit:** `fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9` (short `fdfd0b9`). Every
  file you read must be fetched from **this exact commit**. Construct every raw URL as
  `https://raw.githubusercontent.com/joeloverbeck/tracewake/fdfd0b90f7e4dfc39443aea6d71450bef7d6dfe9/<manifest path>`.
- **The manifest (`manifest_2026-06-13_fdfd0b9.txt`) is path inventory only** — it proves a path
  existed in the tree at `fdfd0b9`. It is never source text, never authority, never proof of latest
  `main`. Use it only to know which paths are fetchable.
- **Do not** fetch from `main`, `HEAD`, a branch name, code search, repository metadata, connector
  namespace labels, or any URL missing this owner/repo/commit. Trust only exact URLs containing
  `joeloverbeck`, `tracewake`, and `fdfd0b9`. Abort and say so if a needed file cannot be fetched
  from this commit.
- **Commit hashes quoted *inside* archived specs and reports are that artifact's own provenance**
  (the commit *it* was accepted at) — they are historical, often predate later merges, and are
  **never** fetch targets. In particular, the routing memo and the foundation-amendment report pin
  themselves to `f7adc01`; that is *their* baseline, not yours. Fetch everything from `fdfd0b9`,
  which is later and contains the merged `0026` amendment. If a referenced artifact cites a
  different "commit of record," note the divergence and use `fdfd0b9`.
- This discipline exists because Tracewake's own design depends on provenance; the design-risk
  register codifies the failure modes (R-00 exact-commit drift, R-01 repository contamination,
  R-04 stale-manifest trust). Mirror that discipline in your own research.

---

## §2 — Read in full (authority-ordered)

Fetch and read these from `fdfd0b9`. Files split into three roles. **Governing reference** is the
current foundation you measure architecture against — read every one. **Amendment target** is the
architecture tier under audit — read every one. **Inputs & boundary-awareness** frame the task and
keep findings routed correctly.

### Governing reference — the current constitution (read all 15 + the index)

Architecture must faithfully translate this. Read in order; treat as one inseparable doctrine.

- `docs/README.md` — authority map and the per-tier `may define / may not define` table. The
  contract for *what architecture is allowed to own* vs. what belongs above or below it.
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — map, authority order, and the **`what` / `how` layer
  boundary** (what foundation may and may not define) — the test for whether a recommendation truly
  belongs in architecture.
- `docs/0-foundation/01_PROJECT_CHARTER.md` — product identity and priorities.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001`…`INV-111`, the compact
  non-negotiable contract. **INV-111 (emergence evidence) is new since architecture was last
  edited.** Re-derive every invariant number you cite from *this file* (see §6).
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, replay,
  traces, snapshots, randomness, forensic causality.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — subjective epistemics:
  claims, beliefs, memory, rumor, testimony, records, stale beliefs, absence-as-evidence. **Primary
  driver for the provenance-sufficiency and memory-freshness architecture items.**
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — ordinary agents, needs,
  routines, intentions, bounded planning, transparent cognition. **Driver for need-accounting and
  believed-access items.**
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action authority,
  affordances, survival needs, property, access. **Driver for need-accounting and believed-access.**
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — institutional
  fallibility, records, notices.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first play, possession,
  view-model filtering, debug quarantine. **Driver for believed-access affordances.**
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — no-quest/no-director
  constitution, allowed seeds, forbidden authored outcomes. **Carries the new "Emergence observation
  is not steering" clause.**
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — scale discipline,
  LOD, ancestry retention.
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, LLM
  non-authority.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable scope,
  mandatory proof cases, deferral. **Carries the new "Emergence evidence as acceptance evidence"
  clause.**
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — stable research lessons already
  recorded as foundation constraints.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **the new
  constitutional truth firewall.** "Truth may validate actions, but truth may not plan them." This
  document is the single biggest source of ripple into architecture (provenance sufficiency,
  believed-access affordances, sealed actor-known context). Read it most carefully.

### Amendment target — the architecture tier under audit (read all 15)

Every one of these is a candidate for a recommended change. Read in order.

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — the conformance index / authority
  map for architecture; where cross-cutting ownership seams (e.g. single-charge accounting) are
  named.
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — **primary home
  for provenance-sufficiency, memory-freshness, believed-access.**
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — **home
  for single-charge accounting seams (duration lifecycle / need-delta authority).**
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  **need-accounting and planning-availability of remembered facts.**
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` —
  **primary home for memory-freshness classifier and provenance on beliefs/traces.**
- `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — relevant to the
  residual institution-known-provenance open question.
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — **need-delta /
  single-charge accounting in ordinary-life economy.**
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — **believed-
  access affordance generation; observation-time snapshot capture.**
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **home for
  the observer-only emergence-evidence data contract (post-INV-111) and the falsifiability /
  anti-vacuity / typed-observability requirement.**
- `docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md`

### Inputs & boundary-awareness — the routing seed, the campaign map, and the forward fences

The first three are **primary inputs** (the validated seed and its provenance). The remainder are
**boundary-awareness** reads (read to bound scope and route findings forward — *not* amendment
targets; do not propose changes to them here):

- `reports/foundation-amendment-lower-tier-routing.md` — **primary input.** The routing memo. Its
  `docs/1-architecture/*` table is your validated candidate seed: themes A (provenance sufficiency),
  B (memory freshness), C (believed-access), D (single-charge accounting), E (emergence mechanism,
  post-foundation), F (falsifiability / anti-vacuity), each with its target arch doc(s) and the
  lesson the subsystem contract must own. Validate every row against live source; do not treat any
  as closed.
- `reports/foundation-amendment-research-report.md` — **primary input.** The determination that
  produced the routing. Its §3–§9 hold the per-candidate reasoning; §10 is the routing appendix;
  §11 the residual open questions (institution-known provenance, possession-bind perception,
  EMERGE-OBS thresholds) — at least two of which it explicitly hands to *this* architecture session.
- `reports/foundation-amendment-research-brief.md` — **primary input.** The scope and source
  discipline of the determination, and the house style this campaign follows.
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — the ratified
  emergence-evidence amendment. Its deliverables and the principle it promoted define exactly what
  the architecture tier must now translate (the observer-only *mechanism* / data contract) and what
  it must **not** (steering, feedback into cognition). Boundary-awareness for the emergence item.
- `docs/4-specs/SPEC_LEDGER.md` — the campaign map: one row per archived spec stating what each
  `0006`–`0025` spec exposed. Use it to trace a routed architecture item back to the concrete
  hardening lesson so your recommendation quotes real pressure, not a paraphrase.
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — what the **execution** tier owns (gate
  names, proof obligations, fixtures). Read to know the architecture/execution boundary so you route
  proof-procedure findings *forward* to the later execution session rather than encoding them as
  architecture contracts.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — the operational watchlist (R-02 provenance
  collapse, R-09 epistemic leakage, R-16 no-human ordinary-life failure, R-27/R-28/R-29 acceptance-
  evidence honesty cluster). Boundary-awareness: a risk-register entry is *not* an architecture
  contract; note where an item is only a watch-risk and route reference-tier changes forward.
- `docs/3-reference/02_GLOSSARY.md` — prescriptive terminology control. Any doctrine substance you
  recommend must use canonical terms (`holder-known` / `actor-known`, `institution-known`,
  `observation`, `belief`, `memory`, `projection`, `debug truth`) or explicitly flag a needed
  glossary addition for the later reference session — never silently coin a synonym.

You may explore any other file in the manifest as needed (e.g. specific `0006`–`0025` archived specs
the ledger ties to a theme, or `core`/`content`/`tui` code seams as corroboration per §3.5), but the
files above are the load-bearing minimum.

---

## §3 — Settled intentions (final — do not re-open)

These were decided in the authoring interview. Treat them as fixed constraints.

1. **Amendment target is the architecture tier only.** Your *recommended changes* apply to
   `docs/1-architecture/*` and nothing else. Foundation is read as the **governing reference** and
   is never amended (it is the authority you measure against). Findings whose correct home is
   `2-execution/`, `3-reference/`, or `4-specs/` are **routed forward** to those later, separate
   per-tier sessions — flagged and briefly characterized, never encoded as architecture contracts
   here.

2. **Recommend substance and home, not ratified text.** For each finding, state (a) *what doctrine
   the architecture tier must own* — in your own compact prose, at the subsystem-contract level —
   and (b) *which architecture file* it lands in (and whether it is a new section, an addition to an
   existing contract, or a correction of stale/contradictory text). **Do not** write final
   architecture prose ready to paste, do not invent `INV-###` or gate-code identifiers, and do not
   redefine existing gate semantics. A later Tracewake reassess session writes the actual diffs.
   (If a single finding is genuinely unambiguous — e.g. mechanically encoding INV-111's observer-only
   data contract — you may sketch a tighter substance statement, but still as substance, not as final
   doc text.)

3. **Full foundation→architecture re-pass — validate *and* sweep.** Do both:
   - **Validate** every architecture item the routing memo names (themes A–F + emergence mechanism),
     against live source. Confirm whether the current architecture tier already owns the contract
     (in which case say so with evidence and close it), partially owns it, or is silent/contradictory.
   - **Sweep** all 15 architecture docs against the *current* foundation (post-rewrite + INV-111) for
     ripple the routing memo never anticipated — it was scoped only to `0006`–`0025` campaign lessons,
     not to the full foundation rewrite or the new doc 14. New or restructured foundation doctrine
     (especially doc 14's truth firewall and INV-111) may require architecture changes outside the
     six seeded themes. Report those as first-class findings.
   The routing memo's six items are a *validated seed*, not a closed list: you may reject a seeded
   item (with evidence the architecture already covers it or that it belongs further down) and add
   findings the foundation evidences that the memo misses.

4. **A tier-fit verdict is mandatory for every finding.** For each, decide: **belongs-in-architecture**
   (recommend the subsystem-contract change) or **route-forward** (the substance is a proof
   obligation, gate, fixture, terminology entry, or implementation spec → name the target tier and
   document, hand it off, do not encode it here). Justify against `docs/README.md`'s per-tier
   `may define / may not define` table and the doc-00 layer boundary. "This belongs in execution /
   reference, not architecture" is a first-class, expected outcome — especially for the
   falsifiability / behavioral-proof machinery, whose proof *procedures* live in execution `10` while
   only the *typed-observability data contract* may be an architecture concern.

5. **Doc-tier analysis is primary; code is corroboration.** The analysis is foundation-text vs
   architecture-contract. You **may** cite `tracewake-core` / `tracewake-content` / `tracewake-tui`
   code seams as corroborating evidence that a contract is already honored (so a routed item is
   already covered and can be closed) or is violated — but the deliverable is about doc conformance,
   not a code audit. Do not turn this into an implementation review; that overlaps the later
   execution session.

6. **External research is a supporting delta, not the heart.** The heart is the internal
   foundation→architecture conformance analysis. Cite external prior art only where it sharpens a
   specific subsystem-contract recommendation (e.g. how mature systems structure a provenance-
   sufficiency boundary or an observer-only telemetry channel as an *architecture* contract). The
   predecessor campaign already commissioned deep surveys on BDI / epistemic logic, belief revision
   (AGM), provenance & data-lineage (PROV, why/where/how-provenance), information-flow / object-
   capability security, fog-of-war view derivation, event-sourcing read-model discipline,
   deterministic-simulation control, Rust type-state / sealed-trait patterns, architecture fitness
   functions, mutation/metamorphic testing, and emergent-narrative systems (Dwarf Fortress, *Talk of
   the Town* / James Ryan's curationist work, Versu). **Treat those as already done** — add only the
   architecture-tier-framing increment, and cite the predecessor where it applies.

7. **Validate against live source; archived specs are pressure, not certification.** Per the
   campaign's discipline (R-26 archived-specs-are-not-certification), the `0006`–`0025` series and
   their acceptance reports are evidence of *pressure on the contracts*, never proof a contract is
   already correctly stated. Re-confirm every cited symbol/path against `fdfd0b9`.

8. **Locked / no-questions; full source and terminology discipline** (see §1 and §6). No scope creep
   into other tiers' *amendment* work (only forward-routing flags), no new world mechanics, no
   implementation.

---

## §4 — The task

Produce one consolidated **architecture-tier alignment report**: a change-proposal that tells a later
reassess session exactly which `docs/1-architecture/*` contracts must change so the architecture tier
faithfully translates the *current* foundation. Concretely:

1. **Establish the foundation delta.** From the fetched foundation (and the `0026` archived spec),
   identify what the current foundation now requires that post-dates or restructures what architecture
   was written against — at minimum INV-111 and the new doc 14, plus any doc 09/12 emergence clauses
   and any rewrite-introduced doctrine. State each as a `what`-level requirement architecture must
   translate.

2. **Validate the routing-memo architecture items** (themes A–F + emergence mechanism). For each,
   quote the routing memo's intended lesson and the campaign evidence behind it (via `SPEC_LEDGER.md`
   / the named spec), then check the live architecture tier: already owned (close it with evidence),
   partial, or absent/contradictory.

3. **Sweep for unanticipated ripple.** Walk all 15 architecture docs against the current foundation;
   surface conformance gaps, stale cross-references, or contradictions the routing memo did not name —
   especially anything flowing from doc 14 and INV-111.

4. **Render a tier-fit verdict for every finding** (belongs-in-architecture vs route-forward), justified
   against the authority tables.

5. **Recommend substance and home** for every belongs-in-architecture finding: which architecture file,
   what the contract must assert (your prose), new-section vs addition vs correction — without final
   wording or invented identifiers.

6. **Carry the residual open questions forward.** The determination's §11 hands this session at least
   two: institution-known provenance (arch `08`) and the `INV-087`-adjacent possession-bind perception
   decision recorded in the architecture conformance index. Address each: is it an architecture-contract
   decision to make now, or an owner decision to surface? Do not silently drop them.

---

## §5 — Exploration and external-research mandate

> Explore the repository as deeply as needed beyond the files listed in §2 — including specific
> `0006`–`0025` archived specs the ledger ties to a theme, and `core` / `content` / `tui` code seams
> as corroboration (per §3.5). Research online only where it sharpens a specific architecture-tier
> recommendation; the deep prior-art survey was already commissioned by the predecessor campaign
> (§3.6) — add only the architecture-framing increment and cite the predecessor where it applies.
> Cite every external claim that shapes a recommendation.

Distinguish clearly, throughout, between (i) fetched Tracewake source doctrine, (ii) external
research, and (iii) your own inference/recommendation — mirroring the repo's own provenance
discipline (R-02).

---

## §6 — Doctrine and immutable constraints

- **Authority order is absolute.** Foundation outranks architecture. If the current architecture text
  conflicts with the current foundation, *architecture is wrong* and must change — never recommend
  weakening or reinterpreting foundation to fit existing architecture. A later layer may *specialize*
  foundation doctrine; it may not *weaken* it.
- **`what` / `how` boundary** (`docs/0-foundation/00`, "Layer boundary"; `docs/README.md` per-tier
  table). Architecture owns subsystem contracts: data flow, authority boundaries, ownership seams,
  read/write/derive permissions. It does **not** own crate-internal module layout, storage engine,
  serialization format, planning algorithm, fixture shapes, gate procedures, or test mechanics — those
  are execution / implementation. If a finding's substance is one of those, it is *route-forward*, not
  an architecture change.
- **Invariant-citation discipline.** Any `INV-###` you reference must be re-derived from
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` *as fetched from `fdfd0b9`*. **Never** trust
  `INV-###` numbers quoted inside archived specs or reports — they predate doctrine realignments and
  are frequently wrong. When you recommend an architecture contract, describe its *substance and home*;
  do **not** invent `INV-###` numbers or write ratified text.
- **Gate codes are cross-references only.** Names like `P0-CERT`, `ORD-LIFE-CERT`, `TFW`, `NO-HUMAN`,
  `EPI-CERT`, `EMERGE-OBS` may be *cited* to locate execution doctrine; never redefine, weaken, or
  invent gate semantics. Proof procedures behind those gates are execution's — route forward.
- **Terminology.** Use the canonical terms in `docs/3-reference/02_GLOSSARY.md`. If a recommended
  architecture contract needs a term the glossary lacks, flag the needed glossary addition for the
  later reference session — do not silently coin a synonym. No backwards-compatibility shims or alias
  paths in new doctrine.
- **Anti-contamination doctrine the architecture tier must protect:** no simulation fact may be born
  from prose; preserve event-sourced causality, subjective epistemics, ordinary agents, possession
  parity, fallible institutions, the truth firewall (truth may validate but not plan), and
  validation/replay. The emergence-evidence contract you encode must be **observer-only**: explainable
  through event-log ancestry, structurally unable to feed cognition / scheduler / validators or set
  dramatic objectives (INV-111; doc 09 "observation is not steering").
- **Source discipline** per §1 (exact-commit fetch from `fdfd0b9`; manifest is inventory; archived-spec
  / report commit strings are not fetch targets; abort on contamination).
- **Archived specs are history, not certification** (R-26). Use the `0006`–`0025` series as evidence of
  pressure on the contracts, never as proof a contract is already correctly stated.

---

## §7 — Deliverable specification

Produce **one** consolidated, paste-ready markdown document, downloadable, named:

> **`architecture-tier-alignment-research-report.md`** (the user will place it in `reports/`).

It is a **change-proposal report for `docs/1-architecture/*`**, *not* rewritten architecture docs and
*not* a foundation amendment. It is **always produced** and is never a stub. Recommended structure:

1. **Disposition table (top).** One row per finding → target architecture doc → verdict
   (belongs-in-architecture / already-owned-close / route-forward[to which tier]) → one-line basis.
   The reader should see the whole change surface at a glance, with the routing-memo themes and the
   swept-in findings both visible.
2. **Method & provenance ledger.** The exact-commit fetch list you used (all from `fdfd0b9`), the
   contamination statement, and the (i)/(ii)/(iii) provenance convention from §5.
3. **Foundation delta.** What the current foundation now requires that architecture was not written
   against (INV-111, doc 14, doc 09/12 emergence clauses, any rewrite-introduced doctrine), each as a
   `what`-level requirement.
4. **Per-finding sections.** For each finding (routing-memo-seeded and swept-in alike), in a stable
   order grouped by architecture doc:
   - *Foundation driver* — the current-foundation requirement (quote the foundation file / INV) that
     creates the obligation, plus the campaign evidence where relevant (quote `SPEC_LEDGER.md` / the
     named spec).
   - *Current architecture coverage* — what the live architecture tier says today: already owns it,
     partial, silent, or contradictory (quote the arch file). Code corroboration where it changes the
     verdict (per §3.5).
   - *Tier-fit verdict* — belongs-in-architecture vs route-forward (named tier/doc), justified.
   - *Recommendation* — for belongs-in-architecture: the doctrine substance (your prose) and its home
     (which arch file; new section / addition / correction), without final wording or invented
     identifiers. For already-owned: the evidence that closes it. For route-forward: the target
     tier/doc and a one-paragraph hand-off.
5. **Forward-routing appendix.** A consolidated backlog of everything routed beyond architecture
   (theme → target tier: `2-execution` / `3-reference` / `4-specs` → target document → the lesson it
   must encode), so the later per-tier sessions inherit a clean, ordered hand-off — the same service
   the routing memo did for this session.
6. **Residual open questions.** The §4.6 carry-forwards (institution-known provenance; possession-bind
   perception) and anything the fetched evidence could not settle, stated honestly.
7. **References.** Full citation list for any external prior art used.

**Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do
not interview, do not ask clarifying questions — the requirements above are final. If a genuine
contradiction makes a requirement impossible, state it in the deliverable and proceed with the most
faithful interpretation.

---

## §8 — Self-check before returning

Run this against your own output:

- [ ] Every Tracewake file was fetched from `fdfd0b9`; no `main`/branch/search/metadata fetches; no
      archived-spec or report commit string (e.g. `f7adc01`) used as a fetch target. Contamination
      statement present.
- [ ] The foundation delta section names the post-architecture foundation changes explicitly (at
      minimum INV-111 and doc 14), each re-derived from the fetched foundation.
- [ ] Every routing-memo architecture item (themes A–F + emergence mechanism) was validated against
      live source — closed-with-evidence, partial, or absent — not merely restated.
- [ ] The 15-doc sweep ran: at least the docs with no routing-memo item were checked for
      rewrite-driven ripple, and any swept-in finding is reported as first-class.
- [ ] Every finding has an explicit tier-fit verdict (belongs-in-architecture / already-owned /
      route-forward) justified against the authority tables — not just asserted.
- [ ] The `what` / `how` filter held: no recommended architecture change encodes module layout,
      storage/serialization, an algorithm, a fixture, a gate procedure, or a test mechanic.
- [ ] No `INV-###` or gate-code identifiers were invented; no final architecture prose was authored;
      recommendations are substance + home.
- [ ] The emergence-evidence recommendation is observer-only: explainable via event-log ancestry,
      structurally unable to feed cognition/scheduler/validators or author outcomes.
- [ ] External prior art is a supporting delta (predecessor surveys treated as done); every external
      claim is cited; the (i)/(ii)/(iii) provenance distinction is maintained.
- [ ] Residual open questions (institution-known provenance; possession-bind perception) are addressed
      or explicitly carried, not dropped.
- [ ] The forward-routing appendix is consolidated so the `2-execution` / `3-reference` / `4-specs`
      sessions inherit a clean hand-off; glossary terms used canonically, any needed addition flagged.
- [ ] The deliverable is one complete, paste-ready `architecture-tier-alignment-research-report.md`.
