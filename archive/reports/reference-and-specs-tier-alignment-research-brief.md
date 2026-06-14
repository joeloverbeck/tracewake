# Research brief — realign `docs/3-reference/*` and `docs/4-specs/*` to the ratified 0026/0027/0028 amendments

**You are ChatGPT-Pro Session 2. This prompt is final and locked.** Produce the deliverables
directly as downloadable markdown documents. Do **not** interview, do **not** ask clarifying
questions — every decision you need is settled below. If a genuine contradiction makes a
requirement impossible, state it inside the relevant deliverable and proceed with the most
faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-13_36b4082.txt`) is the path inventory of the
`joeloverbeck/tracewake` repository — a causality-first living-world simulation in Rust
(event-sourced kernel, subjective epistemics, fallible institutions, TUI-first playability).
Docs are layered authority: `0-foundation` → `1-architecture` → `2-execution` → `3-reference` →
`4-specs`; **earlier tiers govern later ones**. **Fetch every file from commit `36b4082`**
(full SHA `36b40823fb07752987531ecd142c78505b8f56da`) — the manifest reflects exactly that tree.

**Commit-of-record divergence (use HEAD, not the report strings).** The predecessor reports you
will read each pin their *own* older baselines — the execution report cites `64a8367`, the
architecture report `fdfd0b9`, the foundation routing memo `f7adc01`. Those are historical
provenance for those reports, authored before the later merges. Fetch everything from `36b4082`;
treat any other commit string inside a fetched file as document content, not as a fetch target
(per `docs/3-reference/01_DESIGN_RISK_REGISTER.md` R-00 exact-commit drift and R-05 stale-metadata
drift).

**This brief continues a multi-session campaign — treat it as a delta, not a cold start.** A single
hardening campaign (`archive/specs/0006`–`0025`) surfaced seven candidate lessons. One
(emergence-as-evidence) became a foundation amendment; the rest were routed below foundation. The
campaign then executed **tier by tier**, each tier ratified and archived before the next:

- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — added
  foundation `INV-111` (observer-only emergence evidence) and first-playable acceptance doctrine.
  Its deliverable **D4** is a *routed hand-off*: coin the emergence-evidence glossary term in the
  reference tier later (i.e. now).
- `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — translated `INV-111`
  and truth-firewall hardening into architecture contracts (A13 observer-only emergence-evidence
  data contract, A03/A06 provenance sufficiency + freshness, A10 embodied capture, A04/A05/A09
  single-charge accounting). Its **§5 forward-routing appendix** hands findings to reference/specs.
- `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — translated the foundation +
  architecture contracts into execution proof mechanics (evidence honesty, anti-vacuity, EMERGE-OBS
  mechanism, freshness, single-owner accounting). Its **§6 Out of Scope** and the supporting
  `reports/execution-tier-alignment-research-report.md` **§6/§7** explicitly route three findings
  forward to the reference and spec tiers: **F01, F02, F03** (defined in §3 below).

**The freshest, most specific seed is `reports/execution-tier-alignment-research-report.md`**
(its §6 per-finding F01/F02/F03 detail + §7 forward-routing appendix) — it supersedes the older
`reports/foundation-amendment-lower-tier-routing.md` where they overlap, because it reflects the
state *after* both upstream tiers were amended. Read the routing memo for original intent, but trust
the execution report's hand-off substance where they differ.

**This is the final cascade pass.** The foundation, architecture, and execution tiers are already
amended (sessions complete). You are realigning the **two remaining lower tiers** —
`docs/3-reference/*` and `docs/4-specs/*` — at the same time. There is no tier below specs;
forward-routing out of these tiers is minimal (an owner decision and possible future implementation
specs only).

---

## 2. Read in full (authority order)

Read these before producing anything. Fetch each from commit `36b4082` by exact raw URL. Order is
strict authority order; entries are marked **primary (load-bearing for this realignment)** or
**boundary-awareness (read to bound scope / validate, not a conformance target you amend)**.

**Universal authority**
- `docs/README.md` — **primary.** Authority order and the layering rule you must not invert.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — **primary.** `INV-001…INV-111`; every
  recommendation must satisfy these. `INV-111` (observer-only emergence evidence) is the new
  invariant the whole cascade descends from.

**Foundation governing references (immutable — you measure against these, never amend them)**
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — **primary.** The
  truth-firewall / provenance / freshness / believed-access doctrine the risk register and glossary
  terms point back to.
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — **primary.**
  Establishes `INV-111` and explicitly routes the glossary term (D4) to the reference tier — your F01.

**Architecture governing references (immutable)**
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — **primary.**
  The observer-only emergence-evidence data contract and typed-observability posture your glossary
  term and risk cluster must stay faithful to.
- `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — **primary.** Its §5
  forward-routing appendix and §6 Out of Scope name the reference/specs hand-offs.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` —
  **boundary-awareness.** Confirms the provenance-sufficiency contract the risk register cross-refs;
  not an amendment target.

**Execution governing references (immutable)**
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — **primary.** Now
  carries (post-0028) the evidence-honesty, anti-vacuity, and EMERGE-OBS proof doctrine that the
  reference risk cluster must track and the acceptance template must operationalize. The risk
  register and template recommend nothing that redefines this; they cross-reference it.
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — **primary.** `EMERGE-OBS` non-gate
  semantics and the canonical gate-code list the reference layer may only cross-reference.
- `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — **primary, freshest spec
  seed.** Its §3 deliverable D9 (evidence honesty) and §6 Out of Scope F01/F02/F03 are the exact
  hand-off you are executing.

**Predecessor reports (seeds — read for hand-off substance, not as doctrine)**
- `reports/execution-tier-alignment-research-report.md` — **primary, freshest seed.** §6 finding
  detail (F01 lines ~414-421, F02 ~425-432, F03 ~436-443, plus E08 evidence-honesty driver
  ~321-336) and §7 forward-routing appendix are the most specific statement of what each target doc
  must own.
- `reports/architecture-tier-alignment-research-report.md` — **primary.** §5 forward-routing
  appendix (emergence-evidence terminology; anti-vacuity → R-29; acceptance/fingerprint honesty)
  and the X10 finding detail.
- `reports/foundation-amendment-lower-tier-routing.md` — **primary.** Its `docs/3-reference/*` and
  `docs/4-specs/*` sections record the original routing (note: it predates 0027/0028, so the
  execution report supersedes it where they differ — e.g. it asserts "4-specs gets no doctrine,"
  but 0028 later routed F03 to the `4-specs` acceptance-artifact template).

**Target tier — the documents you realign**
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — **primary amendment target (F02).** Already carries
  R-27/R-28/R-29 from the 0025 hardening campaign; validate before adding.
- `docs/3-reference/02_GLOSSARY.md` — **primary amendment target (F01).** Has `evidence`,
  `observation`, `projection`, `story sifting`, `salience`, `stale information`, `holder-known
  context` — but **no** emergence-evidence term and no `EMERGE-OBS` entry.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — **primary amendment target (F03).** The
  scoped acceptance template; its per-requirement table currently uses a bare `pass/fail` column.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — **boundary-awareness.** May warrant
  a single checklist question or a contents-line note; not a primary target.
- `docs/4-specs/SPEC_LEDGER.md` — **boundary-awareness.** Already carries rows for 0026/0027/0028 and
  already names "reference/spec-template follow-through" as pending later work (this session).
  Validate it is current; do not assume a gap.
- `docs/4-specs/README.md` — **boundary-awareness.** Already realigned post-overhaul.
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` —
  **boundary-awareness.** Live first-proof ontology; check only for genuine emergence-evidence /
  acceptance-honesty drift, do not redesign.

---

## 3. Settled intentions (these make this session locked)

1. **Exactly three primary routed findings drive this pass.** All three amendment specs converge on
   the same three hand-offs; do not invent a fourth doctrine target.
   - **F01 → `docs/3-reference/02_GLOSSARY.md`.** Coin a canonical term for the observer-only
     emergence-evidence artifact (spec `0026` deliverable D4). The term must carry these properties:
     **retrospective / after-the-fact, non-certifying, backed by event-log ancestry, non-steering
     (never feeds cognition / scheduler / validators / authoring / seed selection / scenario goals /
     outcome gates), and structurally a non-input** to the simulation. It must cross-reference
     foundation `INV-111` and foundation docs `09`/`12`, architecture `13`, and execution `10`/`00`
     (`EMERGE-OBS`). Do **not** silently synonymize it with the existing `evidence`, `observation`,
     `projection`, or `story sifting` entries — those stay distinct. Also assess whether `EMERGE-OBS`
     itself deserves a glossary cross-reference entry (it is currently an execution gate-code label
     only).
   - **F02 → `docs/3-reference/01_DESIGN_RISK_REGISTER.md`.** Validate and, where needed, realign the
     existing **R-27 / R-28 / R-29** acceptance-evidence-honesty cluster so it tracks the six
     distinctions the upstream tiers now enforce: **pending ≠ pass, sampled ≠ certifying,
     observer-only ≠ gate, byte-fingerprint ≠ semantic proof, archive/history ≠ certification,
     artifact-presence ≠ behavior-witness.** Keep R-29 as the named guard-vacuity / decorative-lock
     relapse risk. Confirm the cluster connects to execution `10` as a cross-reference (not a
     redefinition). Also assess whether the emergence-evidence-as-non-gate failure mode (counters
     becoming simulation objectives — the anti-Goodhart risk) is adequately covered by the existing
     register or warrants a watch note; the routing memo links it to R-16/R-27.
   - **F03 → `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.** Recommend template fields so a
     review packet labels evidence by **status** (pass/fail where actually certified; pending where
     not yet proven; sampled where representative but not exhaustive; observer-only where it cannot
     certify behavior; historical where archive/spec evidence is context, not certification) and by
     **fingerprint scope** (raw bytes / normalized serialization / parsed semantic content / command
     transcript / run seed / replay artifact), plus fields for **path-under-test behavior witness**
     and **replay/provenance ancestry**. A fingerprint must not be cited beyond its scope;
     pending/sampled/observer-only/archive evidence must never be silently counted as a pass.

2. **The F03 gate is now satisfied.** Both reports said the template must wait until "execution
   doctrine establishes the rule" / "after `10` is amended." Execution spec `0028` (D9 evidence
   honesty) is ratified and archived, so this template work is **now actionable** — recommend it, do
   not defer it again.

3. **Partial-alignment discipline — validate before you flag.** The reference docs were last
   realigned to the *overhaul* baseline and predate 0026/0027/0028; the glossary (last touched
   2026-06-08) and template (2026-06-09) predate the amendments, while R-27/R-28/R-29 already exist
   from the **0025** hardening campaign and the SPEC_LEDGER already carries 0026/0027/0028 rows. For
   every routed item, **measure current coverage first and close it with evidence — do not assume
   every routed item is a gap.** A finding whose substance is already present should be dispositioned
   "already-owned-close" with the citation, not rewritten.

4. **Primary vs boundary-awareness.** The three primaries (glossary F01, risk register F02, template
   F03) carry the cascade. `docs/3-reference/00`, `docs/4-specs/{README,SPEC_LEDGER,0001}` are
   validate-only: you may flag a genuine, specific drift (e.g. a reference-index review-checklist
   question for acceptance-evidence honesty, a one-line ledger/README note), but do not manufacture
   amendments for symmetry. The reference layer's own maintenance rule forbids adding material future
   sessions will not use at the moment of coding/review.

5. **`F04` possession-bind perception is out of scope here.** It is an *owner decision* (does
   possession binding itself emit modeled perception?), recorded as deferred in architecture `A00`
   and routed to the owner, then execution `07` — not a reference or spec deliverable. Surface it in
   the open-questions section of the relevant report so it is not lost; do **not** decide it.

6. **Authority floor and minimal forward-routing.** Reference and specs are the lowest tiers. The
   foundation, architecture, and execution docs plus specs `0026`/`0027`/`0028` are the **immutable
   governing reference** — you cite them, you never recommend weakening them. No reference or spec
   recommendation may define doctrine, redefine a gate, coin an invariant, or certify code. The only
   things that route *out* of these tiers are the F04 owner decision and any eventual implementation
   specs (e.g. a future `P0-CERT` spec) — neither is authored here.

7. **No invented identifiers, no paste-ready text.** You deliver *substance + home*: what each target
   doc must own, in your own prose at the right tier altitude, and which file/section it lands in.
   Do **not** produce final ratified wording, and do **not** invent new identifiers (`INV-###`, gate
   codes, `R-##` numbers beyond the existing cluster). New risk-register or glossary identifiers, if
   any are warranted, are named by the repo's own reassess/amend process — you recommend "a new watch
   risk connecting X/Y/Z" or "extend R-27's symptom list," not "add R-30."

---

## 4. The task

This is a **downward-cascade doc realignment** (foundational / doc-overhaul type). The foundation,
architecture, and execution tiers have been amended (specs 0026/0027/0028, ratified and archived).
Your job is to determine, with evidence, what the two remaining lower tiers —
`docs/3-reference/*` and `docs/4-specs/*` — must change to stay faithful to those amendments, and to
deliver those changes as **substance + home** recommendation reports. Concretely: execute the three
routed hand-offs (F01 glossary term, F02 risk-cluster realignment, F03 acceptance-template fields),
validating existing coverage before recommending additions, and noting any genuine boundary-doc
drift without inflating scope.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow cross-references,
read adjacent foundation/architecture/execution docs (e.g. foundation `09`/`12`, architecture `06`,
execution `09`) wherever a routed finding's lineage requires it. Research online as deeply as it
sharpens the deliverable — the **test-oracle problem**, **metamorphic / property-based / mutation
testing**, **approval/golden testing's semantic limits**, **deterministic-simulation testing**,
structured-observability practice, and prior art on **emergent-narrative / story-sifting observation
vs. direction** are all relevant to the evidence-honesty and emergence-evidence findings. Cite every
external source that shapes a recommendation. The deep research is yours to run; the repo is the
authority you measure against.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution. Every recommendation must
  satisfy it; nothing here amends or tensions an invariant (`INV-111` included). A reference/spec doc
  may not promote doctrine upward.
- **Authority order is absolute.** If a lower tier conflicts with a higher one, the lower tier is
  wrong. Reference guardrails and spec packages operationalize; they never override foundation,
  architecture, or execution.
- **The reference layer's own boundaries** (from `docs/3-reference/00`–`02`): the risk register owns
  *compact risk memory only* (symptoms, guardrails, evidence-to-watch, escalation, retirement) and
  may name gate codes only as cross-references; the glossary owns *prescriptive naming only* and may
  not smuggle doctrine; the two must stay separate. Honor these — do not recommend the risk register
  define gate semantics or the glossary define doctrine.
- **The spec layer's own boundaries** (from `docs/4-specs/README.md`, `SPEC_LEDGER.md`): specs are
  the lowest tier; `0003` is a *review-artifact template*, not a doctrine package — its fields
  operationalize execution `10`, they do not restate it.
- **Anti-contamination / source discipline** (R-00…R-05): no recommendation born from prose; fetch
  every fact from `36b4082`; keep an evidence ledger; never adopt a stale commit string from a
  fetched file as authority.
- No backwards-compatibility shims or alias paths in new work.

---

## 7. Deliverable specification

Produce **two separate downloadable markdown recommendation reports** — **new files**, not
replacements, and **not** numbered `docs/4-specs/` artifacts (so the spec-numbering / ledger / epoch
rules do **not** apply):

1. **`reports/reference-tier-alignment-research-report.md`** — covers `docs/3-reference/*`
   (primary: F01 glossary, F02 risk register; boundary-awareness: `00` index/checklist).
2. **`reports/specs-tier-alignment-research-report.md`** — covers `docs/4-specs/*` (primary: F03
   acceptance-artifact template; boundary-awareness: `SPEC_LEDGER.md`, `README.md`, `0001`).

Each report uses the **canonical cascade-report shape**:

- **Disposition table** — one row per finding → target doc → verdict (`amend` / `already-owned-close`
  / `boundary-awareness-no-change` / `route-forward`) → one-line basis.
- **Method / provenance ledger** — target repo, commit `36b4082`, manifest role, exact-URL fetch
  method, the files fetched, contamination check, and the commit-of-record divergence note (§1).
- **Per-finding sections** — for each: *upstream driver* (which invariant/architecture/execution
  doctrine + which 0026/0027/0028 deliverable routed it) → *current coverage in the target doc*
  (quote/cite what already exists — this is where partial-alignment discipline applies) → *tier-fit
  verdict* (does this belong in this tier, and is it already owned?) → *recommendation* (substance +
  home: what the doc must own, in your prose, and which section it lands in — **no paste-ready text,
  no invented identifiers**).
- **Forward-routing appendix** — minimal for this terminal tier pair: the F04 possession-bind owner
  decision, and any eventual implementation-spec follow-up. State explicitly that nothing routes
  further down.
- **Open questions** — including F04 and any residual ambiguity (e.g. whether `EMERGE-OBS` warrants
  its own glossary cross-reference entry; whether the reference index checklist should gain an
  acceptance-honesty question).
- **References** — exact-commit source URLs for every fetched repo file, plus cited external sources.

> Produce the deliverables directly as downloadable markdown documents. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- Both reports exist, named exactly as in §7, each in the canonical cascade shape.
- F01, F02, F03 each appear with current-coverage evidence cited from the live doc **before** any
  recommendation — no routed item is assumed to be a gap without checking (Settled Intention 3).
- The glossary recommendation coins one emergence-evidence term with all five properties
  (retrospective, non-certifying, ancestry-backed, non-steering, non-input) and cross-references
  `INV-111` + foundation 09/12 + architecture 13 + execution 10/00.
- The risk-register recommendation validates R-27/R-28/R-29, keeps R-29 named, and ties the six
  honesty distinctions to execution `10` as cross-references, not redefinitions.
- The template recommendation lists status, fingerprint-scope, sampling, observer-only, pending,
  behavior-witness, and ancestry fields, and notes the F03 gate is now satisfied by ratified 0028.
- No recommendation contains paste-ready ratified wording or a newly invented `INV-###` / gate code /
  `R-##`. No recommendation weakens or amends an upstream tier.
- F04 is surfaced as an owner decision, not decided. Boundary-awareness docs are validated, not
  inflated into amendments.
- Every repo claim is fetched from `36b4082`; every external claim is cited; the manifest was used as
  path inventory only.
