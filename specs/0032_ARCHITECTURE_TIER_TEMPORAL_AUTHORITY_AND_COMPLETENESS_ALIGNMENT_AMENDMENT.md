# Spec 0032 — Architecture-Tier Temporal-Authority and Completeness Alignment Amendment

This spec **proposes a set of architecture-tier (`docs/1-architecture/*`) amendments**. It is a
design/proposal artifact: it specifies the *substance and home* of each amendment so Tracewake's
reassess / ticket process can author the final architecture wording. **It does not itself author
ratified architecture prose.** Architecture is tier-1 doctrine but is not constitutional, so
enactment requires owner approval rather than the constitutional sign-off that a foundation
amendment demands (cf. `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`,
`archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`).

> Section set: this file uses the canonical `specs/` section set (Problem Statement, Approach,
> Deliverables, Invariants Alignment, Verification, Out of Scope, Risks & Open Questions) because
> `specs/` carries no template at authoring time and this is not a hardening implementation spec.
> It deliberately does **not** copy the foundation-pack docs' narrative house style.

**Status:** PROPOSED. Not yet enacted into `docs/1-architecture/*`.

**Admissibility posture:** `P0-CERT not applicable`. This is a doctrine-alignment proposal; it
certifies no code and performs no gate audit.

**Authority:** A spec is subordinate to architecture and *may not replace architecture*
(`docs/4-specs/SPEC_LEDGER.md`). This spec does not exercise that forbidden authority. It
**operationalizes higher-tier (foundation) doctrine** — the temporal-authority invariant `INV-112`
and the foundation `03` "Temporal authority" section (`docs/0-foundation/02`, `docs/0-foundation/03`,
ratified by archived spec `0031`), plus the seven completeness-routed themes the foundation
completeness determination handed to architecture — by staging *additive* refinements (and minor
cross-references) so the architecture tier translates that doctrine fully. No deliverable weakens or
contradicts existing architecture; the final wording, once authored, is architecture-tier doctrine
and this spec becomes historical provenance.

**Provenance:** derived from `reports/architecture-tier-alignment-research-report.md` (external deep
research, pinned to commit `ea6a05bf5822307cfcbd39804bbb015cdb13215d` = current `HEAD` `ea6a05b`)
and its brief `reports/architecture-tier-alignment-research-brief.md`. The report is the planned
`docs/1-architecture/*` session of the temporal-authority cascade that began with the foundation
amendment `0031`. The report's verdict-critical premises were independently re-verified against live
`HEAD` during authoring (see Verification §V1); per the deep-research-spec convention, the report's
fabricated `#Lxxxx` line anchors were ignored in favor of named symbols and sections.

---

## 1. Problem Statement

Foundation `0031` promoted **temporal authority** to constitutional doctrine: `INV-112` ("Time may
validate, but holder-known time must plan") plus a primary foundation `03` "Temporal authority"
section and application hooks across foundation `04`/`05`/`07`/`08`/`10`/`12`/`14`. The temporal
firewall is the truth firewall applied to time: the scheduler/replay clock may **order and
validate**; cognition, routine selection, institutional procedure, embodied view models, speech
interpretation, leads, and LOD promotion may use temporal facts only when those facts reached the
relevant holder through **modeled channels**. `0031` deliberately deferred the
architecture/execution/reference cascade.

The architecture tier has **not yet absorbed that delta**. Verified against live `HEAD` `ea6a05b`,
the primary architecture docs that own the affected subsystems carry *no* temporal-authority,
temporal-firewall, temporal-claim, or procedural-time contract: A02 (event/replay time), A03/A06
(holder-known claims / provenance / freshness), A04 (scheduler/validation), A08 (institutions) all
return empty on temporal probes. The subsystem boundaries exist; the **temporal specialization of
those boundaries does not**. A future subsystem author cannot apply `INV-112` from architecture
alone. This is the **Block T** gap: ten findings (T1–T10) translating the temporal firewall into
subsystem contracts.

Separately, the foundation completeness determination routed **seven non-temporal themes** to
architecture as subsystem-contract concerns (Block R, R1–R7): play experience / epistemic
legibility, quantity / material granularity / fungibility, bounded affect, learning / adaptation,
deterministic performance / fairness budgets, practical bias / social harm, and authoring /
compiler discipline. Current architecture already covers much of this substrate (needs, memory,
institution fallibility, view models, validation artifacts); the gap is that each theme lacks a
**named, compact architecture seam**, so the contracts are fragmented or merely implicit.

None of these is a foundation hole — `INV-112` and the completeness routing are already ratified.
Each is a **subsystem-contract** gap: foundation states the `what`; architecture must own the
data-flow / authority boundary; execution and reference own the proof mechanics, vocabulary, and
thresholds. The architecture-tier alignment report dispositioned seventeen findings; this spec
stages every one that warrants an architecture amendment and routes the rest forward. Two
architecture docs (A01, A14) were swept and found conformant with **no** Block T/R amendment
required (Out of Scope §6).

## 2. Approach

Stage seventeen compact, additive architecture refinements (Block T: T1–T10; Block R: R1–R7), each
landing in the primary subsystem doc that *owns* the contract, with cross-links where the report
identifies a secondary anchor. Keep every change at the `what may read/write/derive/order/validate/
expose/preserve` level. Route every tick size, calendar/date syntax, day-part/"yesterday" /
lateness vocabulary, duration unit, stale-after threshold, scheduler queue/data structure, fairness
algorithm, UI clock format, inventory/economy schema, money denomination, affect/learning update
rule, status enum, fixture name, command, and gate procedure **out** of architecture to execution
`docs/2-execution/*` and reference `docs/3-reference/*` (the report's forward-routing appendix §6
already enumerates these hand-offs).

The seventeen deliverables are the only architecture-tier items; the report's forward-routing
hand-offs are deferred to later tier sessions (Out of Scope §6). Final architecture wording is
authored on enactment (by reassess / ticket), not in this spec. This follows the precedent set by
`0027`, which staged the prior (`INV-111`/truth-firewall) architecture cascade in exactly this form.

This spec stages **both blocks in one package** because both are architecture-tier, documentation-
only, and derive from the same report; the report itself prescribes "a future architecture amendment
spec" (singular, §6.3). If an owner prefers, Block T and Block R are cleanly separable into two
specs — the deliverable IDs and target homes are disjoint enough to split without rework.

## 3. Deliverables

All deliverables are **proposed amendment substance**, to be authored into the named architecture
file by the reassess / ticket process. None prescribes final wording, schema fields, data
structures, function/table/row names, vocabularies, thresholds, or fixtures.

### 3.1 Block T — temporal-authority cascade (translates `INV-112` + foundation `03`)

| # | Report ID | Target file | Kind | Substance |
|---|---|---|---|---|
| D-T1 | T1 | `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Addition (conformance entry) | A compact conformance entry/subsection recording the temporal-authority architecture cascade and pointing to owners: A02 (authoritative event/replay time); A03/A06 (holder-known temporal claims); A04 (scheduler/validation temporal boundary); A05 (routine/social rhythm); A08 (institutional/procedural time); A10/A11/A07 (temporal rendering, leads, speech); A12 (LOD temporal ancestry); A13 (observability). It states that architecture **translates** `INV-112`; it must **not** restate foundation doctrine at length or become the sole home of temporal authority, and must not define tick/calendar/duration values. |
| D-T2 | T2 | `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Addition (subsystem contract) | An authoritative event/replay-time contract near the event-envelope and replay sections: (a) event/replay time is the ordered substrate for validation, scheduling due effects, duration accounting, replay, projection rebuild, and causal explanation; (b) temporal facts are exported to holders **only** through events/projections carrying modeled acquisition or record/procedure ancestry; (c) projection/snapshot/compaction must preserve temporal ancestry rather than replace it with "current time" labels; (d) replay diagnostics for temporal divergence (wrong ordering, missing duration terminals, due-effect drift, unrecorded wall-clock input, unsupported temporal migration). Names the seam and data-flow obligations only — no calendar/date syntax, tick size, duration units, priority-queue structure, or stable field names. |
| D-T3 | T3 | `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (xref `06`) | Addition (specialization of existing A03/A06) | **A03:** a temporal-claim subsection — any temporal input used by cognition, procedure, affordance selection, speech interpretation, lead interpretation, or LOD promotion must be addressable inside the holder-known context with fact-kind-appropriate provenance. **A06:** the parallel epistemic data-flow rule — temporal claims preserve distinct slots where relevant (event time asserted/inferred, acquisition time, last verification, record/procedure time, valid/due window, stale risk, contradiction status, source lineage). Abstract only: no struct, field names, stale-after numbers, day-part vocabulary, or calendar syntax. The substance is that temporal status is a source-backed claim/procedure state, not a display label or true clock read. Specializes the `0027` provenance-sufficiency / freshness contracts; does not reopen them. |
| D-T4 | T4 | `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Addition (temporal firewall) | Near scheduler limits and the validation boundary, state that scheduler/replay time **may** order decision opportunities/process windows, detect due effects and duration terminals, invoke holder-known transaction construction, validate temporal legality / due-consequence applicability, and emit typed temporal diagnostics. It **may not** turn "work time / meal time / deadline passed / office closed / actor is late" into a selected action, route, target, institutional conclusion, or actor-visible reason unless that premise is in the relevant holder-known/institution-known context; may not repair plans using true lateness or hidden schedule truth; may not leak exact future/due timing through actor-visible feedback unless a modeled channel exposes it. Also: budget exhaustion is a typed scheduling/decision outcome (defer/skip/summarize/diagnose via a deterministic policy execution later defines). No queue or algorithm. (Cross-references D-R5.) |
| D-T5 | T5 | `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Addition | A routine/social-rhythm subsection: work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments are **defeasible temporal premises** inside actor-known context. Candidate generation and method selection may use those premises only if they arrive through assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference. Routine templates may organize method families and expected rhythms, but a template's presence is **not** itself an information channel. (Supports D-R3/D-R4: affect and learning tune salience/method preference only after the same holder-known premise is available; they cannot create a hidden temporal fact.) |
| D-T6 | T6 | `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Addition (procedural-time authority) | An institutional/procedural-time contract: (a) a procedure may maintain due, late, open, closed, pending, delayed, filed, expired, paid, sanctioned, or queue-aged states only as event/record/procedure-backed institution-known state; (b) procedure time is authoritative for the procedure's **own lifecycle** only through recorded rules and events — it does not grant the institution hidden truth about the underlying world; (c) procedure outputs that become actor knowledge, notices, reports, sanctions, payments, refusals, or records preserve provenance and access context; (d) bias, misfiling, delay, underfunding, and stale records remain modeled procedure effects, not hidden moral labels. No office-hour vocabulary, legal deadlines, payment periods, or concrete status enums. |
| D-T7 | T7 | `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Addition (temporal rendering / time controls) | A temporal-rendering / time-control contract: (a) world-advancing controls are commands that advance authoritative event/replay time **through the ordinary pipeline** — they are not actor cognition; (b) actor-facing time displays, "missed event" summaries, time-to-work cues, waiting/sleeping summaries, "office closed" messages, and lateness/expectation labels are derived from the possessed actor's holder-known context or from modeled observations/records/public cues; (c) debug/operator panels may show exact event/replay time, due queues, and hidden temporal comparisons, but those fields are structurally non-diegetic and cannot feed embodied affordances or actor-visible reasons; (d) rejection/why-not output preserves the actor-visible/debug split (true temporal invalidity may validate rejection; actor-visible explanation is limited to what the actor can know). No UI clock format, acceleration speed, wait-command vocabulary, or summary thresholds. |
| D-T8 | T8 | `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`; `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | **Cross-reference** (already-owned-close) | Short cross-references, not new doctrine. **A07:** temporal utterances ("yesterday," "late," "due," "before the market closed," "after the bell") are structured claims whose interpretation depends on speaker and listener holder-known temporal context, provenance, and ambiguity (via A03/A06). **A11:** lead/notebook labels (stale, urgent, overdue, recently seen, old report, no longer useful) are source-bound projections over holder-known temporal claims and records; story-sifting may compute observer-only temporal summaries for review but may **not** create actor-known urgency or quest priority. Preserves the `0027`-ratified A11 observer-only carve-out. |
| D-T9 | T9 | `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | Addition | An LOD temporal-summary contract: (a) each regional/LOD summary that compresses time declares source interval, cadence, affected processes, temporal resolution/fidelity limits, and whether it includes scheduled consequences, absence of events, delayed records, or stale claims; (b) the summary preserves **information ancestry separately from event-time ancestry** (what the aggregate could know vs. what it merely summarized as truth for replay vs. what later promoted holders may know); (c) time acceleration is a **declared** simulation mode/projection policy with replay/debug visibility, not a silent performance optimization; (d) promotion may create holder-known temporal claims only through modeled summary events or records that are valid information channels for the promoted holder. No LOD equivalence thresholds, promotion algorithms, regional cadence values, or performance budgets. |
| D-T10 | T10 | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Addition (typed observability) | Add temporal evidence to the validation/observability contract: decision traces identify the temporal premises used by candidate generation/method selection and their provenance; validation reports separate temporal **truth checks** from actor-visible temporal **reasons**; scheduler diagnostics record due effects, deferred/skipped cognition, budget exhaustion, starvation/fairness symptoms, and layer attribution; TUI/view-model reports prove temporal display labels came from holder-known sources, not raw clock/debug truth; LOD/replay artifacts preserve temporal and information ancestry; acceptance artifacts reject display-string-only proof of temporal correctness. Defines required evidence **shape** only; execution owns concrete gate names, fixture families, command output, thresholds, and pass/fail policy. (Cross-references D-R5.) |

### 3.2 Block R — seven completeness-routed themes

| # | Report ID | Target file | Kind | Substance |
|---|---|---|---|---|
| D-R1 | R1 | `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Addition (consolidation) | **A10:** name the embodied play loop at architecture altitude — the player forms plans from actor-known view models, attempts semantic actions, receives actor-visible failure/why-not feedback, inspects source-bound notebook/lead surfaces, and uses debug only as non-diegetic review. **A11:** lead usefulness is a source-bound projection concern — leads can be stale, ambiguous, partial, contradictory, or actionable, but they do **not** become objective markers, quest stages, or hidden priority; transcript evidence demonstrates the loop without parsing display prose as authority. No drama objectives or director controls. |
| D-R2 | R2 | `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Addition (representation seam) | A quantity/granularity/fungibility seam: (a) unique objects, countable lots, divisible stocks, capacities, debts, wages, and ledgers are separate representation classes because they preserve different identity/provenance/custody constraints; (b) operations such as split, merge, consume, spoil, reserve, share, transfer, hide, discover, pay, refuse, and reimburse must preserve event ancestry, custody/ownership/procedure context, and holder-known visibility; (c) fungible aggregation is allowed only when it does not erase information needed for action validation, provenance, replay, wrong belief, lead interpretation, institutional record, or later promotion; (d) projections may summarize quantities for UI/debug, but authoritative lineage for replay/disputes cannot be replaced by a display total. No data structures, unit vocabularies, money denominations, inventory schemas, or economy formulas. |
| D-R3 | R3 | `05_ACTOR_DECISION_TRANSACTION_…PLANNING.md`; `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Addition (consolidation) | **A05:** define affect as a **bounded decision influence** — a source-bearing salience/pressure modifier over candidate generation, method selection, interruption, concealment/confession/accusation/repair tendencies, or routine disruption. **A06:** define affect-memory effects as provenance-bearing changes to salience, durability, recall priority, or belief uptake. Affect may explain why a holder prioritizes or avoids an option; it may **not** reveal truth, select hidden targets, bypass planning, overwrite beliefs without events, or force dramatic actions. Narrow seam, not a full emotional architecture. |
| D-R4 | R4 | `05_ACTOR_DECISION_TRANSACTION_…PLANNING.md`; `06_CLAIMS_BELIEFS_…INFORMATION_FLOW.md` | Addition (derived-state seam) | A learned-expectation seam distinguishing remembered facts from learned generalizations: (a) a learned expectation is derived state from remembered experiences, modeled instruction, records, testimony, repeated failures/successes, or institution-procedure outcomes; (b) it is **not** a raw memory, a truth cache, or a global probability table unless its source and scope are modeled; (c) it can influence candidate ordering, method applicability, trust, risk aversion, skill confidence, route preference, and routine adaptation; (d) it preserves source events, scope, holder, confidence/uncertainty if represented, contradiction/staleness, and reset/decay/override provenance. Execution owns learning depth before first proof, update rules, decay, thresholds, and fixtures. |
| D-R5 | R5 | `04_…PIPELINE.md`; `05_…PLANNING.md`; `12_LOD_REGIONAL_…SCALE.md`; `13_VALIDATION_OBSERVABILITY_…ARTIFACTS.md` | Addition (cross-cutting seam) | A deterministic-performance / fairness-budget seam. **A04:** scheduler budgets and ordering policies are deterministic and diagnosed; when budget limits prevent full cognition/procedure execution, the outcome is typed as deferred, skipped, summarized, degraded, or blocked, with responsible layer and replay ancestry. **A05:** bounded planning may fail/degrade for budget reasons only through typed decision diagnostics — never via omniscient shortcuts or marker-action substitution as progress. **A12:** LOD/time-acceleration is a declared fidelity mode with fairness constraints (lower detail may summarize, but not erase active claims, procedures, leads, obligations, or starvation caused by persistent under-scheduling). **A13:** fairness/starvation review artifacts (which holders/processes were deferred, why, for how long / across what source interval, and evidence that no human-proximity or possessed-actor priority bias exists unless explicitly non-diegetic input routing). No numeric budgets, scheduling algorithms, fairness formulas, queues, or thresholds. (Consolidates the budget-exhaustion notes cross-referenced from D-T4/D-T10.) |
| D-R6 | R6 | `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Addition (consolidation) | Practical-bias discipline: (a) bias and social harm are modeled as inspectable inputs, social-position effects, resource constraints, procedure rules, testimony-credibility patterns, record-access patterns, institutional memory, staff/resource availability, or prior decisions; (b) the kernel remains genre-neutral and asserts no omniscient moral truth — domain packs own cultural/legal/institutional assumptions and must make them reviewable; (c) wrong suspicion, refusal, delay, sanction, misfiling, or unequal treatment must arise from holder/institution-known evidence and procedure state, including biased or stale inputs when modeled; (d) diagnostics expose the modeled assumptions and procedure steps without granting actors hidden truth or turning social harm into an objective quest condition. |
| D-R7 | R7 | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Addition (compiler-discipline seam) | An authoring/compiler-discipline seam: (a) content/schema validators, static guards, manifest checks, and review artifacts are **architecture-protecting boundaries** that reject impossible or forbidden authoring forms before runtime, rather than relying on runtime filters to clean contaminated data; (b) validator outputs must be structured and layer-attributed (field/path or authored element, violated doctrine, responsible layer, provenance/source status, author-actionable failure reason); (c) the validation surface must protect against aliases, nested forbidden concepts, display-string-only proof, hidden-truth cognition fields, player/human privilege, silent migrations, incompatible content versions, and outcome chains; (d) architecture requires the seam and evidence shape; execution/specs choose schemas, rule languages, commands, compatibility policies, and exact error formats. |

D-T8 is a pair of cross-references; all other deliverables are additive subsystem contracts or
consolidations. All are consistent with existing architecture and with foundation doctrine.

## 4. Invariants Alignment

| Invariant / doctrine | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-112` — time may validate, but holder-known time must plan | aligns (translates) | The whole Block T cascade gives architecture the subsystem homes for the temporal firewall: D-T2 (event/replay-time substrate @ event log), D-T3 (holder-known temporal claims @ holder-known/epistemic surfaces), D-T4 (scheduler trigger-vs-plan boundary @ action pipeline), D-T5 (routine premises @ actor decision), D-T6 (procedural time @ institutions), D-T7 (temporal rendering @ TUI), D-T8 (speech/lead temporal claims @ language/incidents), D-T9 (LOD temporal ancestry @ scale), D-T10 (temporal evidence @ observability). None feeds simulation behavior with hidden temporal truth. |
| `INV-103` — the scheduler is not a cognition authority | aligns (specializes) | D-T4 specializes the scheduler-may-not-construct-proposals rule for time at the action-pipeline surface: scheduler/replay time may order/validate/detect-due but may not turn temporal truth into a selected action, route, target, conclusion, or actor-visible reason absent a holder-known premise. |
| `INV-105` — actor decision traces are authoritative typed diagnostics | aligns (extends) | D-T10 and D-R5 require temporal premises, temporal truth-vs-reason separation, and budget/fairness outcomes to be typed/structurally inspectable at the observability surface, not display strings. |
| `INV-110` — LOD/summary processes must preserve the firewall | aligns (specializes) | D-T9 forces LOD temporal summaries to preserve temporal + information ancestry and to create holder-known temporal claims only through modeled channels at the scale surface, so promotion cannot fill in temporal truth. |
| `INV-111` — observer-only emergence evidence | aligns (preserves) | D-T8 and D-T10 keep story-sifting temporal summaries observer-only and reject display-string temporal proof; D-R5 fairness/starvation artifacts are review evidence, not simulation inputs. The `0027`-ratified A11 carve-out is preserved, not weakened. |
| `INV-087` — human focus is not player privilege | aligns | D-R5 requires fairness/starvation artifacts to prove no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing, at the scheduler/LOD/observability surfaces. |
| Truth firewall family `INV-099…INV-110`; `INV-006` possession transfers no world knowledge; `INV-108` human possession is cognition-neutral | aligns | The temporal firewall is the truth firewall applied to time; D-T7 keeps embodied temporal rendering on holder-known/observed material so possession is not a temporal-knowledge upgrade. |
| Questless / no-director, genre-neutral kernel | aligns (preserves) | D-R1 (no quest stages/markers), D-R3 (affect may not force dramatic actions), D-R6 (no morality oracle / no objective social-harm quest condition) all add inspectable seams without importing drama or moral omniscience. |

No invariant is weakened or tensioned. All deliverables are additive/clarifying `what`-level
architecture doctrine; no `how`-level mechanism (tick size, calendar syntax, threshold, fixture,
command, schema, algorithm) enters architecture. **No architecture file at this commit shows a
conflict requiring a foundation edit** — the report finds architecture gaps and consolidations, not
constitutional tension.

## 5. Verification

- **V1 — Premises confirmed against live `HEAD` (done at authoring).** Re-verified on `HEAD`
  `ea6a05b` (= the report's pinned commit), ignoring the report's fabricated line anchors:
  `INV-112` is present in `docs/0-foundation/02` with the "time may validate, but holder-known time
  must plan" wording; foundation `03` carries the "Temporal authority" section enumerating
  authoritative event/replay time, holder-known temporal claims, institution-known procedural time,
  freshness/staleness, and LOD/regional temporal summaries plus the temporal-firewall statement;
  and the architecture homes carry **no** temporal contract today — A02 (no temporal-authority /
  temporal-ancestry language), A03 (no temporal-claim / temporal-provenance language), A04 (no
  temporal-firewall / budget-exhaustion language), A08 (no procedural-time language) all return
  empty on targeted probes. `INV-087/103/105/110/111` were confirmed present with the meanings cited
  in §4. The Block T gaps are real; the Block R themes are consolidations over existing substrate.
- **V2 — Enactment acceptance (on implementation).** Each deliverable is accepted only when its
  substance is authored into the named architecture file as a compact subsystem contract (not an
  A00 row only), D-T8 adds cross-references without new doctrine, the `0027` A11 observer-only
  carve-out is preserved, and no execution-tier mechanism token (tick size, calendar/date syntax,
  duration unit, day-part/lateness vocabulary, stale-after number, scheduler queue/algorithm,
  fairness formula, UI clock format, status enum, schema field, fixture, command, threshold) enters
  architecture.
- **V3 — Boundary check.** Review each newly added architecture passage for mechanism tokens;
  architecture must hold the data contract and authority boundary only. A whole-file grep is not the
  proof surface because architecture legitimately discusses these concepts in the abstract.
- **V4 — Forward-routing follow-through (later sessions, not this spec).** The execution
  `04`/`05`/`06`/`07`/`08`/`09`/`10`/`11`/`12` proof obligations and the reference `01`/`02` risk
  and glossary wiring are enacted by their own tier sessions; this spec only records the hand-offs
  (Out of Scope §6 and report §6).

## 6. Out of Scope

- **Final architecture wording, schema fields, newtypes, data structures, function/table/row names,
  vocabularies, thresholds, commands, status taxonomies, fixtures.** Owned by the reassess/ticket
  enactment and by execution/reference.
- **All concrete temporal values.** Day-part vocabulary, "yesterday"/"last night"/"office
  closed/open"/"due/late"/"recently/stale" terms, calendar/date syntax and conversion, duration
  units, stale-after policy / record-validity periods / claim decay, office-hour representation,
  queue-aging, time-acceleration speed, missed-summary thresholds, and simultaneity/tie-break rules
  route to execution/spec sessions (report §6.1, §7).
- **Proof mechanics for D-T1…D-R7.** Temporal-firewall negative fixtures (stale work assignment,
  office closed but actor does not know, due record seen/not seen, late report with source, debug
  clock excluded from actor view, LOD summary promotion without hidden temporal fill-in), budget/
  fairness diagnostics and packets, quantity-lineage fixtures, affect/learning negative fixtures,
  domain-pack bias review, and authoring/compiler guards go to execution `04`/`06`/`07`/`08`/`09`/
  `10`/`11`/`12`.
- **Reference-tier terminology and risk memory.** Temporal-authority / holder-known-temporal-claim /
  procedural-time / freshness / time-acceleration / temporal-ancestry glossary terms, and risk notes
  for clock-oracle leakage, raw-wall-clock contamination, omniscient lateness/office-closed labels,
  UI time-acceleration leaks, silent LOD temporal fill-in, and truth-cache learning, are authored by
  the reference tier after architecture wording stabilizes (report §6.2).
- **Foundation edits.** `INV-112` and the foundation `03` temporal-authority section are already
  ratified (spec `0031`); this spec changes no `docs/0-foundation/*` file and creates no new
  invariant.
- **Architecture docs `01` and `14` — verified conformant, no deliverable.** A01 (authority
  boundaries / workspace / dependency rules) already supports deterministic-simulation ownership;
  A14 (research decisions / forbidden misreads) already rejects importing generic frameworks as
  product identity. The report's sweep found neither requires a Block T/R amendment (A14 may
  optionally record temporal/bias/authoring research anchors if the amendment process wants them —
  not a doctrine gap). Their absence from §3 is deliberate, not an oversight.
- **The quantity/economy (D-R2) and temporal blocks should not be bundled into one execution proof
  package** unless a single gameplay feature truly requires it (report §6.3); this spec only stages
  the architecture seams, which is bundling-neutral.
- **New world mechanics, Phase-4 expansion, LLM surfaces, crate/code changes.**

## 7. Risks & Open Questions

- **R-A — Architecture enactment requires owner approval.** Authoring D-T1…D-R7 edits tier-1
  doctrine across thirteen architecture docs (A00 + A02–A13; A01 and A14 carry no deliverable).
  Lighter than constitutional sign-off, but it must
  not proceed by convention; this spec stages the substance, it does not authorize the edits.
- **R-B — Block size / split option.** Seventeen deliverables across two blocks is large. The spec
  bundles them per the report's singular-spec recommendation and the user's "a spec" directive;
  Block T (temporal) and Block R (completeness themes) are cleanly separable into two specs if an
  owner prefers smaller enactment units. Splitting requires no rework — the deliverable IDs and
  homes are disjoint.
- **R-C — Mechanism leakage during enactment.** The chief failure mode is authoring concrete
  temporal vocabulary, thresholds, schemas, or algorithms into architecture (V2/V3 guard against
  this). The temporal block is especially exposed because "morning," "yesterday," and "stale-after"
  feel like architecture but are execution-owned.
- **R-D — Budget-exhaustion / fairness lives in three deliverables (D-T4, D-T10, D-R5).** Enactment
  must consolidate, not triplicate: D-R5 owns the cross-cutting seam; D-T4/D-T10 cross-reference it
  for the temporal-specific scheduler/observability hooks. Avoid three divergent statements of the
  same budget contract.
- **R-E — `INV-111` observer-only relapse.** D-T8/D-T10/D-R5 introduce temporal/fairness review
  summaries. These must stay observer-only and non-certifying; counters or summaries becoming
  simulation objectives or actor-known urgency is forbidden authored-outcome machinery. Preserve the
  `0027` A11 carve-out exactly.
- **R-F — Possession-bind perception (deferred owner decision; do NOT silently amend).** The prior
  architecture cascade recorded an unresolved `INV-087`/`INV-108`-adjacent tension about whether
  possession *binding* may emit modeled perception. Nothing in this temporal pass decides it; D-T7
  preserves the deferred state. If decided later, the perception must be a modeled event/channel for
  the actor, never a human/player knowledge transfer.

## 8. Provenance & Source Discipline

- The source report is pinned to `ea6a05b` and was re-verified against that same live `HEAD`; no
  intervening commit drift applies (`ea6a05b` is the current `HEAD` merge of the `0031` enactment).
- External research cited in the report (DES/next-event simulation, ODD, PROV-DM, MDA, social-sim
  emotion surveys, JSON Schema/OPA, NIST AI RMF / bias / Datasheets, deterministic-simulation
  testing / lockstep, Lean MVP) shaped tier-fit judgment only; none becomes Tracewake doctrine. The
  report §2.4 and §8.2 hold the full citation list; this spec does not restate it.
- Commit hashes named here are audit/spec provenance only.
- This spec adds **no** `docs/4-specs/SPEC_LEDGER.md` row at proposal time, per the staged-spec
  convention (the `0026`/`0027`/`0031` precedent: the ledger row lands at acceptance/closeout, not
  at proposal).
