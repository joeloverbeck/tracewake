# Research brief — Foundation-tier amendment audit & external prior-art survey (post-0006…0025)

**For ChatGPT-Pro Session 2 (the deep researcher).** You are *locked*: produce the deliverable
directly. Do **not** interview, do **not** ask clarifying questions — the requirements below are
final. The interview already happened. **Upload bundle = this prompt + the manifest
`manifest_2026-06-13_f7adc01.txt`.**

Your job is a **determination plus an external prior-art survey**, not an implementation. You are
deciding whether Tracewake's constitutional layer (`docs/0-foundation/*`) has *holes* —
under-specified or missing doctrine — that the hardening campaign (archived specs `0006`–`0025`)
has revealed, and you are deep-researching how mature systems and the research literature handle
those themes so a later Tracewake session can amend the constitution on a best-practice footing.
The external prior-art survey is the **heart** of the deliverable; the internal hole-determination
is its **frame**.

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
1-architecture/→ subsystem contracts: foundation translated into data-flow/authority/boundaries
2-execution/   → gate order, certification sequence, fixtures, proof obligations
3-reference/   → compact lookup, glossary (terminology control), design-risk register
4-specs/       → narrow implementation/corrective specs under live doctrine
archive/       → historical evidence (completed specs/tickets/reports), NOT live authority
```

Foundation outranks architecture outranks execution outranks reference outranks specs. A later
layer may *specialize* foundation doctrine but may not *weaken* it. The foundation layer **defines
what Tracewake is and is not**; it explicitly does **not** define crate names, module layout,
storage engine, serialization format, planning algorithm, or other implementation choices (see
`docs/0-foundation/00_FOUNDATION_INDEX.md`, "Layer boundary"). This `what / how` boundary is the
single most load-bearing fact for your task (see §4).

### The campaign you are auditing

The last **feature-forward** spec was `archive/specs/0005_PHASE_3A_…IMPLEMENTATION_SPEC.md`. Every
spec from `0006` through `0025` was *foundation-violation-fixing, hardening, and anti-regression*
work — not new features. That hardening campaign repeatedly forced new conceptual machinery into
existence (proof constructs, sealing patterns, accounting authorities, emergence ledgers). The
question this brief answers: **across `0006`–`0025`, which lessons reveal that the *constitution
itself* is silent or under-specified on something it should govern — versus machinery that
correctly lives below the foundation tier?**

`docs/4-specs/SPEC_LEDGER.md` carries a one-row-per-spec summary of *what each `0006`–`0025` spec
exposed*. **Read it first** — it is the fastest, highest-signal map of the campaign and tells you
which archived specs to read in depth for any given theme.

### Predecessor research briefs — this is a *delta*, not a cold start

Four prior briefs in `reports/` were authored in this same repo, for you, and **commissioned the
very `0006`–`0025` work** you are now auditing:

- `reports/phase1-doc-code-alignment-research-brief.md`
- `reports/phase1-third-hardening-research-brief.md`
- `reports/phase2a-epistemic-substrate-hardening-research-brief.md`
- `reports/phase3a-ordinary-life-needs-routines-hardening-research-brief.md`

Read the **Phase-2A** and **Phase-3A** briefs in full (named in §2) to (a) inherit the campaign's
house style, source discipline, and settled conventions, and (b) **confirm what external prior art
was already commissioned** so you frame this as a delta. In particular, the predecessor briefs
already commissioned deep research on: BDI / epistemic-logic agent architectures, HTN / GOAP /
STRIPS bounded planning, belief revision (AGM), information-flow / non-interference / object-
capability security, fog-of-war / per-actor view derivation, event-sourcing read-model discipline,
deterministic-simulation nondeterminism control, Rust type-state / sealed-trait / newtype invariant
patterns, architecture fitness-functions, and mutation testing. **Do not re-derive those surveys
from scratch.** Your delta is the *constitutional framing* question: not "how do agents model
belief" but "should a top-level invariant *mandate* X, and how do mature systems state such a thing
as doctrine rather than as code." Where a predecessor already surveyed a field, cite it and extend
only the constitutional-framing increment.

This brief differs in kind from its predecessors: they were *hardening* briefs (find defects,
harden a block). **This is a foundational/doc-overhaul brief** — its amendment target is the
constitution, and its dominant deliverable is an external survey, not an internal defect hunt.

### Baseline commit and fetch discipline (read carefully)

- **Fetch baseline commit:** `f7adc0149963ea4a90b58ad05f633fd6e71e8649` (short `f7adc01`). Every
  file you read must be fetched from **this exact commit**. Construct every raw URL as
  `https://raw.githubusercontent.com/joeloverbeck/tracewake/f7adc0149963ea4a90b58ad05f633fd6e71e8649/<manifest path>`.
- **The manifest (`manifest_2026-06-13_f7adc01.txt`) is path inventory only** — it proves a path
  existed in the tree at `f7adc01`. It is never source text, never authority, never proof of latest
  `main`. Use it only to know which paths are fetchable.
- **Do not** fetch from `main`, `HEAD`, a branch name, code search, repository metadata, connector
  namespace labels, or any URL missing this owner/repo/commit. Trust only exact URLs containing
  `joeloverbeck`, `tracewake`, and `f7adc01`. Abort and say so if a needed file cannot be fetched
  from this commit.
- **Commit hashes quoted *inside* archived specs and reports are that artifact's own provenance**
  (the commit *it* was accepted at) — they are historical, often predate later merges, and are
  **never** fetch targets. Fetch everything from `f7adc01`.
- This discipline exists because Tracewake's own design depends on provenance; the design-risk
  register codifies the failure modes (R-00 exact-commit drift, R-01 repository contamination,
  R-04 stale-manifest trust). Mirror that discipline in your own research.

---

## §2 — Read in full (authority-ordered)

Fetch and read these from `f7adc01`. They split into three roles. **Primary** files are the
amendment candidates — read every one. **Boundary-awareness** files exist so you can run the
tier-fit test (§4) — read them to decide whether a candidate already lives below the constitution;
do **not** treat them as amendment targets. **Evidence** files are the lessons source.

### Primary — the constitution under audit (read all 15 as one document)

The foundation is one inseparable doctrine; a hole can only be judged against the whole. Read in
order:

- `docs/0-foundation/00_FOUNDATION_INDEX.md` — map, authority order, the **`what` / `how` layer
  boundary** (what the foundation may and may not define), the 2026 doc-14 hardening note, the
  "updated foundation spine" (four authority boundaries), and the recurring review questions.
- `docs/0-foundation/01_PROJECT_CHARTER.md` — product identity and priorities.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001`…`INV-110`, the compact
  non-negotiable contract. **This is where most amendments would land.** Re-derive every invariant
  number you cite from *this file* (see §6).
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, replay,
  traces, snapshots, randomness, forensic causality.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — subjective epistemics:
  claims, beliefs, memory, rumor, testimony, records, stale beliefs, absence-as-evidence. **Primary
  home for the provenance-sufficiency and memory-freshness candidates.**
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — ordinary agents, needs,
  routines, intentions, bounded planning, transparent cognition. **Relevant to need-accounting and
  believed-access candidates.**
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action authority,
  affordances, survival needs, property, access. **Relevant to need-accounting and believed-access.**
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — institutional
  fallibility, records, notices.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first play, possession,
  view-model filtering, debug quarantine. **Relevant to believed-access affordances.**
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — no-quest/no-director
  constitution, allowed seeds, forbidden authored outcomes. **Relevant to emergence-as-evidence.**
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — scale discipline,
  LOD, ancestry retention.
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, LLM
  non-authority.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable scope,
  mandatory proof cases, deferral. **Relevant to emergence-as-evidence and falsifiability.**
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — stable research lessons already
  recorded as foundation constraints (so you do not duplicate them).
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the
  constitutional truth firewall ("truth may validate actions, but truth may not plan them"). **This
  doc is itself the precedent**: a hardening insight that was promoted to a new foundation document.
  It is the model for a legitimate Bucket-1 promotion, and the primary home for the believed-access
  candidate.

### Boundary-awareness — for the tier-fit test only (do NOT amend)

You need these to answer "does this candidate already live below the constitution, or genuinely
belong in it?" If a lower tier already owns the doctrine substance, the candidate is **Bucket 2**
(route forward), not a foundation hole.

- `docs/README.md` — the authority map and the `may define / may not define` table per tier.
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — what architecture owns.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — whether
  provenance-sufficiency / believed-access are already architecture contracts.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` —
  whether memory-freshness / staleness is already an architecture contract.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — whether
  emergence-evidence / acceptance-reachability is already an architecture contract.
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — what execution owns; canonical gate
  names and observation obligations.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate sequence
  (`SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, …).
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — whether the
  anti-contamination/truth-firewall *proof* obligations already cover a candidate.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **the likely
  home for verification machinery** (mutation, witnesses, behavioral proof). Read it before
  recommending any proof-discipline amendment to the foundation.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — the operational watchlist. **Many candidates are
  already tracked here as relapse modes** (R-02 provenance collapse, R-09 epistemic leakage, R-16
  no-human ordinary-life failure, R-27 acceptance-evidence reachability overstatement, R-28
  incomplete correction closure, R-29 guard vacuity / decorative locks). A risk-register entry is
  *not* constitutional doctrine — note where a candidate is currently only a watch-risk and ask
  whether it should be promoted to an invariant.
- `docs/3-reference/02_GLOSSARY.md` — prescriptive terminology control. Any new doctrine you
  recommend must use existing canonical terms or explicitly flag a needed glossary addition.

### Evidence — the lessons source (read the map fully; read specs as the map directs)

- `docs/4-specs/SPEC_LEDGER.md` — **read in full.** One row per archived spec stating what each
  `0006`–`0025` spec exposed. This is your index into the campaign.
- `archive/specs/0006_*` … `archive/specs/0025_*` — the hardening series (24 archived specs total,
  all present in the manifest). You need not read all 20 in equal depth, but for **each seeded
  candidate theme (§4)** read the specs the ledger ties to that theme closely enough to quote the
  concrete lesson. Representative anchors: `0006`/`0007`/`0008` (early no-human / anti-contamination
  gaps), `0015`–`0018` (evented cognition, need accounting, provenance witnesses, seed epistemics),
  `0017`/`0020`/`0023` (freshness/staleness, supersession parity), `0019`–`0025` (generative
  reachability, mutation perimeter, meta-witness, guard vacuity, manifest fingerprint honesty).
- `reports/phase2a-epistemic-substrate-hardening-research-brief.md` and
  `reports/phase3a-ordinary-life-needs-routines-hardening-research-brief.md` — **read in full** for
  conventions and to bound what prior art was already commissioned (see §1).
- `reports/0020_ord_life_cert_scoped_acceptance.md` and
  `reports/0025_ord_life_cert_scoped_acceptance.md` — the EMERGE-OBS emergence-evidence baselines;
  concrete evidence for the emergence-as-evidence candidate.

---

## §3 — Settled intentions (final — do not re-open)

These were decided in the authoring interview. Treat them as fixed constraints.

1. **Amendment target is the foundation tier only.** Your *recommended amendments* apply to
   `docs/0-foundation/*` and nothing else. Lower tiers are read for boundary-awareness and tier-fit
   testing, never amended in this deliverable. (Cascading amendments to architecture/execution/
   reference are explicitly the job of *later, separate* Tracewake sessions, tier by tier.)

2. **Foundation governs *what* Tracewake is, not *how* it is proven.** This is your central filter.
   A candidate earns a foundation amendment only if it states a durable truth about the *product /
   epistemics / causality contract* — not a verification technique, fixture shape, gate, or proof
   procedure.

3. **A tier-fit verdict is mandatory for every candidate.** For each, decide: **Bucket 1**
   (genuine foundation hole → recommend a constitutional amendment) or **Bucket 2** (real doctrine
   gap whose correct home is `1-architecture/`, `2-execution/`, or `3-reference/` → flag, briefly
   characterize, and **route forward** to the later tier-by-tier sessions). "Belongs below the
   constitution" is a first-class, valid, expected outcome — especially for verification machinery.

4. **Seeded candidate spine — validate, extend, or reject; do not treat as closed.** The authoring
   session mined these six candidates from `0006`–`0025`. Research each, but you are explicitly
   empowered to reject a candidate (with evidence that the foundation already covers it or that it
   belongs below), and to **add** candidates the campaign evidences that this list misses:

   - **(a) Provenance sufficiency — "no fact without a modeled source."** The campaign forced a
     non-empty source-event-ids newtype, fail-closed witness-compatibility tables, and an audit that
     unbacked actor-known/institution-known facts are impossible. Candidate constitutional rule:
     *every actor-known or institution-known fact must carry non-empty modeled provenance and fail
     closed absent it.* Test against `04`, `14`, and architecture `03`/`06`; note R-02.
   - **(b) Memory freshness / staleness epistemics.** The campaign forced a single classifier of
     *currently-perceived* vs *remembered* facts, with remembered facts retaining their acquisition
     time, staying planning-available, but carrying a downgraded provenance class (remembered ≠
     deleted, remembered ≠ re-confirmed-as-current). Candidate rule: *memory persists with explicit
     degraded freshness; current ground truth never silently overwrites a holder's remembered
     belief.* Test against `04`; note this is adjacent to "stale beliefs / absence-as-evidence"
     already in `04`.
   - **(c) Believed-access-vs-truth in affordance generation.** The campaign forced embodied menus
     to be computed from the actor's *believed* conditions (believed-access, known servings,
     perceived presence) at menu-generation time, and forced an observation-time *snapshot* rather
     than live ground truth. Candidate: is this already implied by doc-14's "embodied affordance
     generation must be fed only by sealed actor-known context," or does the constitution need an
     explicit *affordance-enumeration* clause? Test against `14`, `08`, `06`; note R-09.
   - **(d) Single-charge accounting authority for derived quantities.** The campaign forced a single
     per-actor tick-regime classifier and a single duration open/close keying authority so that need
     deltas are charged *exactly once* across window/action boundaries (no double-charge, no drift
     among three consumers). Candidate: is there a *constitutional* principle here (e.g., a
     conservation / single-source-of-truth rule for replay-deterministic derived state), or is this
     purely an architecture contract? This one is a strong Bucket-2 suspect — test it hard.
   - **(e) Emergence-as-evidence (EMERGE-OBS).** The campaign introduced an emergence-evidence
     ledger recording which unscripted phenomena actually arose, distinct from guards proving
     reachability. The constitution celebrates "no scripting" and "story is retrospective" but may
     not state that a living world must be *empirically demonstrated to emit unscripted emergent
     phenomena*, recorded as acceptance evidence. Candidate: an emergence-evidence acceptance
     principle. Test against `09`, `12`; note R-16, R-27.
   - **(f) Falsifiability / behavioral-proof principle.** The campaign's largest output by volume is
     verification machinery (meta-witness execution proof, mutation perimeter, anti-vacuity /
     "no decorative locks," guard-to-negative bijection). The user's stated concern is that this
     **likely belongs in a *lower* tier** (`1-architecture/` or `2-execution/`), not the
     constitution. Research whether any *single constitutional principle* underlies it — e.g.,
     "a guarantee that cannot be made to fail is not a guarantee" / "every invariant must be
     falsifiable" — and render an explicit tier-fit verdict. Expected (not required) outcome:
     **Bucket 2**, routed to execution `10` / reference. If you conclude there is a genuinely
     constitutional kernel, say precisely what minimal sentence it is and why it cannot live below.

5. **Precedent for a legitimate promotion:** `docs/0-foundation/14` was itself a hardening insight
   (the truth firewall) promoted into the constitution because the earlier foundation "left too much
   room for implementation drift." Use it as the model for what a *real* Bucket-1 promotion looks
   like — a durable `what`-level rule, stated compactly, not a procedure.

6. **The external prior-art survey is the deliverable's heart; the determination is its frame.**
   Spend your depth budget on a cited, comparative survey of how mature systems and the research
   literature handle each surviving candidate. Crisp recommendations follow from the survey.

7. **Locked / no-questions; full source and invariant discipline** (see §1 and §6). No scope creep
   into other tiers' *amendment* work, no new world mechanics, no implementation.

---

## §4 — The task

Produce one consolidated determination-plus-survey. Concretely:

1. **Read the constitution and the campaign evidence** (§2) and, for each seeded candidate (§4d
   above) plus any you add, extract the *concrete lesson* from `0006`–`0025` that puts pressure on
   the foundation — quoting the spec or ledger row that evidences it. State the lesson in
   `what`-terms (a claim about what Tracewake must be), stripped of its `how`-machinery.

2. **Run the tier-fit test** on each candidate against the boundary-awareness files (§2). Assign
   **Bucket 1** (foundation hole) or **Bucket 2** (belongs to a lower tier; name which tier and
   which document, route it forward). Justify the verdict against the `docs/README.md` per-tier
   `may define / may not define` table and the doc-00 layer boundary. Where a candidate is currently
   only a design-risk-register watch item, say so and argue whether watch-risk → invariant
   promotion is warranted.

3. **For each surviving Bucket-1 candidate, deep-research external prior art** (the heart — see §5):
   how do comparable systems and the literature state this as *doctrine* (not as code)? What are the
   accepted formulations, the known failure modes, the trade-offs, the vocabulary? Cite everything.

4. **Recommend the doctrine substance and placement** for each Bucket-1 candidate: *which foundation
   file* it amends (a new invariant in `02`, an addition to `04`/`14`, or — like doc-14 — a new
   foundation document), and *what the doctrine should assert*, in your own compact prose. **Do
   not** write final invariant wording or assign `INV-###` numbers — that is Tracewake's own
   reassess process (see §6). Recommend substance and home, not ratified text.

5. **Render the verdict, form-follows-verdict.** If, after the walk, a theme shows the foundation is
   *already adequate*, say so with evidence and do **not** inflate it into a recommended amendment.
   A well-evidenced "no foundation hole here" is a fully acceptable per-theme outcome. The overall
   report is always produced regardless of how many Bucket-1 holes survive.

---

## §5 — Exploration and external-research mandate (the heart)

This is where the deliverable earns its keep. For every surviving Bucket-1 candidate, and for the
falsifiability candidate's tier-fit argument, survey the relevant prior art deeply and cite it. The
authoring session has already identified the likely-fertile domains; treat these as a *starting
map*, not a ceiling, and **skip what the predecessor briefs already commissioned** (see §1) except
for the constitutional-framing increment.

- **Provenance sufficiency / "no fact without a source":** justification-based and assumption-based
  truth-maintenance systems (JTMS/ATMS), justification logic, provenance & data-lineage models
  (e.g. PROV, why/where/how-provenance in databases), capability/object-capability discipline for
  "ground truth structurally cannot reach a derived surface," and how epistemic-agent platforms
  state "beliefs must be sourced" as an architectural law rather than a runtime check. *Delta over
  Phase-2A:* the predecessor surveyed BDI/belief-revision/non-interference; your increment is how
  such systems phrase provenance-sufficiency as a *top-level invariant*.
- **Memory freshness / staleness:** models of human/agent memory decay and recency, "last-known
  state" / sensory-vs-working-memory seams in game AI (e.g. the F.E.A.R. sensor→working-memory
  pattern), belief revision and temporal/defeasible logics for stale-but-retained belief, and how
  simulations distinguish *perceived-now* from *remembered* without overwriting. State the accepted
  doctrine: memory degrades in confidence/freshness, it is not silently corrected by ground truth.
- **Believed-access affordances:** information-set / fog-of-war affordance generation, "the menu of
  what I can attempt is a function of what I believe, not what is true," server-authoritative vs
  client-believed action sets, structural prevention of "wallhack" affordances, and snapshot-at-
  observation patterns. Argue whether this is a distinct constitutional clause or a corollary of an
  existing truth-firewall invariant.
- **Single-charge accounting (Bucket-2 suspect):** conservation invariants and single-source-of-
  truth accumulation in discrete-event and agent-based simulation, deterministic accumulation for
  replay equality, double-entry-style "charged exactly once" disciplines. Use the survey primarily
  to argue *tier-fit* (is "derived quantities are charged exactly once through a single authority"
  a constitutional truth, or an architecture contract?).
- **Emergence-as-evidence:** validation of emergent behavior in artificial-life and agent-based
  modeling, "surprising"/weak-vs-strong emergence and how it is empirically demonstrated, and
  emergent-narrative / story-sifting systems (e.g. Dwarf Fortress, Caves of Qud, *Talk of the Town*
  / *Bad News* and James Ryan's curationist emergent-narrative work, Versu) for how a living world
  is *shown* to generate unscripted story without a director. Distill the doctrine: a living world
  must be demonstrated to emit unscripted phenomena, recorded as evidence — versus merely asserting
  guards pass.
- **Falsifiability / behavioral-proof (tier-fit argument):** mutation testing and metamorphic
  testing as assurance philosophy, "rotten green tests" / assertion-free or vacuous tests,
  architecture fitness functions, and the epistemology of "a guarantee that cannot fail is not a
  guarantee." Use this to argue precisely where the principle belongs (expected: execution/reference,
  not foundation), and whether any minimal constitutional kernel survives. *Delta over Phase-1/3A:*
  those surveyed mutation testing as a *technique*; your increment is the *tier placement* of the
  philosophy.

Citation requirement: every external claim must carry a source (paper, book, postmortem, talk,
documentation). Distinguish clearly, throughout, between (i) fetched Tracewake source doctrine,
(ii) external research, and (iii) your own inference/recommendation — mirroring the repo's own
provenance discipline (R-02).

---

## §6 — Doctrine and immutable constraints

- **Invariant-citation discipline.** Any `INV-###` you reference must be re-derived from
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` *as fetched from `f7adc01`*. **Never** trust
  `INV-###` numbers quoted inside archived specs or reports — they predate doctrine realignments and
  are frequently wrong. When you recommend a new rule, describe its *substance and home*; do **not**
  invent an `INV-###` number or write ratified constitutional wording. A genuine divergence from an
  existing invariant would require *amending* it first, never designing around it — but the
  amendment authoring is Tracewake's reassess process, not yours; you recommend, you do not ratify.
- **Respect the `what` / `how` layer boundary** (`docs/0-foundation/00`, "Layer boundary"). Do not
  recommend that the constitution define crate names, module layout, storage/serialization,
  algorithms, fixture shapes, gate procedures, or test mechanics. If a candidate's substance is any
  of those, it is Bucket 2 by definition.
- **Gate codes are cross-references only.** Names like `P0-CERT`, `ORD-LIFE-CERT`, `TFW`,
  `NO-HUMAN`, `EPI-CERT` may be *cited* to locate execution doctrine; never redefine, weaken, or
  invent gate semantics.
- **Terminology.** Use the canonical terms in `docs/3-reference/02_GLOSSARY.md` (e.g.
  `holder-known` / `actor-known`, `institution-known`, `observation`, `belief`, `memory`,
  `projection`, `debug truth`). If a recommended doctrine needs a term the glossary lacks, flag the
  needed glossary addition explicitly rather than coining a silent synonym. No backwards-compat
  shims or alias paths.
- **No scope creep.** No architecture/execution/reference *amendment text* (only Bucket-2
  routing flags). No new world mechanics, no implementation, no Phase-4 expansion, no LLM surfaces.
- **Source discipline** per §1 (exact-commit fetch; manifest is inventory; archived-spec commit
  strings are not fetch targets; abort on contamination).
- **Archived specs are history, not certification.** Per the spec ledger and `docs/2-execution/01`,
  `0005`–`0008` (and the whole `0006`–`0025` series) landed historical work and *exposed* the need
  for stronger gates; they do **not** certify the current implementation. Treat them as evidence of
  *pressure on the foundation*, never as proof that a theme is already solved (design-risk R-26).

---

## §7 — Deliverable specification

Produce **one** consolidated, paste-ready markdown document, downloadable, named:

> **`foundation-amendment-research-report.md`** (the user will place it in `reports/`).

It is **always produced**; its internal shape follows the verdict (form-follows-verdict). It is
never a stub and never "it depends." Recommended structure:

1. **Verdict summary (top).** A short table: each candidate theme → Bucket 1 (foundation hole) /
   Bucket 2 (routed to named lower tier) / No-hole (foundation already adequate), each with a
   one-line basis. The reader should see the whole disposition at a glance.
2. **Method & provenance ledger.** The exact-commit fetch list you used, the contamination
   statement, and the (i)/(ii)/(iii) provenance convention from §5.
3. **Per-candidate sections.** For each seeded candidate (and any you add), in this order:
   - *Campaign lesson* — the concrete `0006`–`0025` evidence, quoting the ledger row / spec, stated
     in `what`-terms.
   - *Tier-fit verdict* — Bucket 1 / 2 / no-hole, justified against the authority tables and the
     existing foundation/architecture/execution coverage you fetched.
   - *External prior-art survey* — the cited, comparative deep dive (the heart), for Bucket-1 (and
     for the falsifiability tier-fit argument). For Bucket-2 / no-hole themes, a shorter survey
     sufficient to justify the verdict.
   - *Recommendation* — for Bucket 1: the doctrine substance (your prose) and its home (which
     foundation file; new invariant vs addition vs new doc), explicitly **without** final wording or
     `INV-###` numbers. For Bucket 2: the target tier/document and a one-paragraph hand-off so a
     later session can pick it up. For no-hole: the evidence that closes it.
4. **Bucket-2 routing appendix.** A consolidated backlog of everything routed to lower tiers, so the
   later tier-by-tier sessions inherit a clean, ordered list (theme → target tier → target document
   → the lesson it must encode).
5. **Open questions / residual uncertainty.** Anything the fetched evidence could not settle, stated
   honestly rather than papered over.
6. **References.** Full citation list for the external survey.

---

## §8 — Self-check before returning

Run this against your own output:

- [ ] Every Tracewake file was fetched from `f7adc01`; no `main`/branch/search/metadata fetches; no
      archived-spec commit string used as a fetch target. Contamination statement present.
- [ ] Every candidate has an explicit tier-fit verdict (Bucket 1 / 2 / no-hole) justified against
      the authority tables — not just an assertion.
- [ ] The `what` / `how` filter was applied: no recommended foundation amendment encodes a
      verification technique, fixture, gate, algorithm, or module layout.
- [ ] Verification-machinery / falsifiability was tier-tested, with its expected Bucket-2 routing (or
      a precisely-argued minimal constitutional kernel if you found one).
- [ ] No `INV-###` numbers were invented and no final constitutional wording was authored; every
      cited `INV-###` was re-derived from `02_…` at `f7adc01`.
- [ ] External prior art is the depth-dominant element for surviving Bucket-1 candidates, fully
      cited, with the (i)/(ii)/(iii) provenance distinction maintained throughout.
- [ ] Predecessor-brief surveys (BDI, determinism, type-state, mutation testing, etc.) were treated
      as already-commissioned; your survey adds the constitutional-framing delta, not a redo.
- [ ] Form-follows-verdict honored: no theme inflated into a hole; "foundation already adequate" used
      where evidenced; the report is one complete, paste-ready document.
- [ ] Bucket-2 backlog is consolidated so the later tier-by-tier sessions inherit a clean hand-off.
- [ ] Glossary terms used canonically; any needed new term flagged, not silently coined.

**Locked / no-questions:** produce `foundation-amendment-research-report.md` directly as a
downloadable markdown document. Do not interview, do not ask clarifying questions — the requirements
above are final. If a genuine contradiction makes a requirement impossible, state it inside the
document and proceed with the most faithful interpretation.
