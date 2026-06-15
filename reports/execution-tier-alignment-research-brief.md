# Execution-tier alignment research brief (for ChatGPT-Pro Session 2)

**You are the deep researcher.** Produce the deliverable directly. Do **not** interview, and do
**not** ask clarifying questions — every requirement below is final. If a genuine contradiction
makes a requirement impossible, state it inside the deliverable and proceed with the most faithful
interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-15_c70d119.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust (event-sourced
kernel, subjective epistemics, fallible institutions, TUI-first). Docs are layered authority:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; **earlier tiers
govern later ones**. Fetch every file from commit **`c70d119bca7663e4e1229dcd2012d57b5fe72d44`**
(short `c70d119`) — the manifest reflects exactly that tree. Construct each fetch URL as
`https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/<manifest path>`.
Use no branch name, clone, code search, default-branch lookup, or repository metadata.

**This brief continues a multi-session campaign.** It is the **execution-tier session** of the
*temporal-authority + foundations-completeness* cascade. The chain so far, freshest-seed last:

1. `reports/verdict-on-foundations.md` + `reports/foundations-completeness-determination-research-report.md`
   — the **root determination**: adjudicated nine themes; promoted exactly one to foundation (time /
   calendar / social rhythm) and **routed the other eight below foundation**, including direct
   hand-offs to the **execution tier** (its §5.3 table).
2. `reports/foundation-tier-alignment-research-report.md` → staged as `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`
   → **enacted** into `docs/0-foundation/*` (added `INV-112` and the foundation `03` "Temporal
   authority" section + application hooks across `04/05/07/08/10/12/14`).
3. `reports/architecture-tier-alignment-research-report.md` → staged as `archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md`
   → **enacted** into `docs/1-architecture/*` (findings T1–T10 temporal cascade + R1–R7 completeness
   themes). **This architecture report is the freshest, most-specific seed for your task**: its §6.1
   "Execution-tier hand-offs" table already enumerates what flows down to execution. Its §6.1/§6.2/§6.3
   supersede the older completeness routing where they overlap, but the completeness report's §5.3
   carries one execution driver the architecture pass did **not** (staged incompleteness).

You are authoring the **next** report in that chain: the **execution-tier alignment research
report** for `docs/2-execution/*`. Treat this as a **delta**, not a cold start. Do **not**
re-commission the foundation or architecture work (already enacted) or the earlier first cascade
(see §3.3).

**Predecessor reports cite older "commit of record" strings** (the architecture report pins
`ea6a05b`; the completeness report pins `d7fc746`). Those are each report's own historical baseline.
Ignore them as fetch targets and freshness claims — fetch everything from `c70d119`, which is the
current `HEAD` that merged the `0031` and `0032` enactments. The architecture report's recommendations
were **pre-enactment proposals**; the *authoritative* statement of what actually landed in architecture
is the **current `docs/1-architecture/*` files at `c70d119`** (cross-check the report against them, do
not treat the report as the architecture text).

---

## 2. Read in full (authority order)

Read these from `c70d119`, in this order, before producing.

### Tier 0 — foundation (GOVERNING REFERENCE, immutable — never an amendment target here)

Primary (load-bearing — the temporal doctrine you cascade downward):

- `docs/README.md` — the authority order and the layering rule you must honor.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; **`INV-112`** ("time may
  validate, but holder-known time must plan") plus the truth-firewall family (`INV-099…INV-111`),
  `INV-087`, `INV-102/103/105/110`. Every recommendation must satisfy these.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — the **primary home** of the
  temporal-authority model (six categories + the temporal firewall).
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — temporal claims/freshness as
  holder-known claims (drives the learning + temporal-claim execution proofs).
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — routine/social-rhythm
  temporal premises (drives the no-human + routine temporal proofs).
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — institution-known
  procedural time (drives the Phase-4 institution proofs).
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — time controls / embodied vs debug
  temporal rendering (drives the view-model + TUI-time proofs).
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — LOD/temporal-summary
  ancestry + fairness (drives the deferred-scale/LOD proofs).
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-playable temporal-firewall
  acceptance expectations.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the temporal case of
  the truth firewall (scheduler boundary).

Boundary-awareness (read to bound scope, confirm no foundation gap — not a conformance target):
`docs/0-foundation/00_FOUNDATION_INDEX.md`, `01_PROJECT_CHARTER.md`, `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`,
`09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`, `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`,
`13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`.

### Tier 1 — architecture (GOVERNING REFERENCE, immutable — the seams you operationalize)

Primary (load-bearing — these carry the now-landed T1–T10 / R1–R7 seams you translate into proof):

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — the temporal-authority conformance
  map + owner pointers.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — authoritative event/
  replay-time contract + temporal divergence diagnostics.
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known
  temporal-claim contract.
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — scheduler
  trigger-vs-plan temporal firewall + budget-exhaustion seam.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` —
  routine premises, affect, learning, planner-budget seams.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — temporal-
  claim slots, affect-memory, learned-expectation seams.
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — procedural-time +
  practical-bias seams.
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — quantity/
  granularity/fungibility representation seam.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — temporal
  rendering/time-control + embodied play-loop seams.
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — temporal/staleness
  lead labels + lead-usefulness seams.
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — LOD temporal ancestry +
  time-acceleration declaration + fairness seam.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — temporal/
  budget-fairness observability + authoring/compiler-discipline evidence shape.

Boundary-awareness: `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
(temporal-utterance cross-reference), `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`,
`14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` (swept conformant in `0032`).

### Tier 2 — execution (THE AMENDMENT TARGET — read all in full)

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — execution authority, canonical gate names
  (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`),
  observation obligations (`EMERGE-OBS`), and label-class reconciliation. **Staged-incompleteness driver lands here.**
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order/sequence.
  **Staged-incompleteness driver lands here.**
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — temporal-firewall
  + anti-hidden-truth-learning gates land here.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — scheduler
  temporal-trigger-vs-plan + budget/fairness proof land here.
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — routine temporal premises
  + ordinary-adaptation (learning) no-human proof land here.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — temporal rendering /
  TUI-time-control / actor-visible-vs-debug split proof lands here.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — quantity/fungibility
  validation + authoring/compiler-discipline land here.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — temporal fixture families
  + replay acceptance land here.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — temporal/budget/
  fairness diagnostics + staged-incompleteness review artifacts land here.
- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — institution/
  procedural-time + practical-bias Phase-4 proof land here.
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — deferred
  LOD/time-acceleration proof obligations land here.
- `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — execution research
  decisions / forbidden misreads.

### Tiers below (FORWARD-ROUTING boundary-awareness — where surplus findings hand off, not amended here)

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`, `01_DESIGN_RISK_REGISTER.md`,
  `02_GLOSSARY.md` — note: reference is **not yet temporally amended** (its last cascade was the
  first-campaign `0029`); temporal terminology/risk memory routes to a **future reference session**.
- `docs/4-specs/SPEC_LEDGER.md`, `docs/4-specs/README.md`, `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
  — where staged-abstraction declarations + concrete fixture/vocabulary choices route as **future
  scoped specs**.

### Seeds & settled context (read in full)

- `reports/architecture-tier-alignment-research-report.md` — **freshest seed**; §4 coverage register,
  §5 per-finding recommendations, and **§6.1 execution-tier hand-offs** are your primary routing input.
- `reports/foundations-completeness-determination-research-report.md` — **root seed**; §3 per-theme
  determinations and **§5.3 Tier-2 execution session** table, including the **staged-incompleteness**
  hand-off (exec `00/03/10`) that the architecture pass did not carry.
- `reports/verdict-on-foundations.md` — the nine origin themes (context).
- `reports/foundation-tier-alignment-research-report.md` — what the foundation pass delivered (context;
  do not re-commission).
- `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` — its §6 "Out of Scope"
  explicitly defers the execution cascade to a later session (this one).
- `archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md` —
  what landed in architecture (its §3 deliverable table D-T1…D-R7) and its §6 "Out of Scope" routing
  all concrete temporal values + proof mechanics to **execution** (your tier).
- `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` and
  `archive/reports/execution-tier-alignment-research-report.md` — the **first cascade's** execution
  work (truth-firewall / `INV-099…INV-111` doctrine alignment). **Settled — do not re-commission.**
  Read to know what execution already owns, so your delta is additive.

---

## 3. Settled intentions (these are final — they replace any interview)

### 3.1 This is a downward-cascade realignment, not a fresh design

The amended `docs/0-foundation/*` (spec `0031`: `INV-112` + foundation `03` temporal-authority section)
and `docs/1-architecture/*` (spec `0032`: T1–T10 + R1–R7) are the **immutable governing reference** —
read in full, **never** an amendment target in this pass. `docs/2-execution/*` is the **amendment
target**. Measure execution against the *currently-landed* upstream docs at `c70d119` (cross-check the
predecessor reports' proposals against the actual architecture text; the text governs). If you find an
apparent foundation/architecture gap, **flag and route it** — do not "fix" upstream tiers here.

### 3.2 Three in-scope drivers — all three are commissioned

The execution tier carries **three distinct drivers**. Enumerate and cover each; none is optional.

- **Driver T — temporal-authority cascade.** Translate the now-landed temporal firewall (foundation
  `INV-112`/`03`; architecture T1–T10) into **execution gate procedure and proof mechanics**. Primary
  homes per the seeds: temporal-firewall proof → exec `04`/`05`/`10`; temporal first-playable
  mechanism/fixtures → exec `06`/`07`/`09`/`10`; scheduler budgets & fairness → exec `05`/`10`/`12`;
  institution/procedural time → exec `11`; TUI time controls → exec `07`/`10`; LOD temporal ancestry →
  exec `12`/`10` (architecture report §6.1).
- **Driver R — completeness Block-R execution routes.** The five completeness themes with execution
  hand-offs (completeness §5.3 + architecture report §6.1): quantity/granularity/fungibility → exec
  `08` (+ future inventory/economy fixture spec); learning/adaptation → exec `04`/`06`; deterministic
  performance/fairness budgets → exec `05`/`10`/`12`; practical bias/social harm → exec `11`;
  authoring/compiler discipline → exec `08`.
- **Driver S — staged incompleteness.** Routed **only** to execution (completeness §3.9 / §5.3): a
  stage-declaration discipline in exec `00`/`03`/`10` — each deliberate abstraction/staged feature must
  declare what it proves now, what it abstracts, what it must not fake, what future feature it must not
  block, and what acceptance evidence prevents overclaiming. (The acceptance-artifact *template* change
  itself, `docs/4-specs/0003_*`, routes forward as a future spec — see §3.5.)

### 3.3 Validate-before-gap — record existing execution coverage first

Execution was **already amended by the first cascade** (`archive/specs/0028_*`): it owns strong
truth-firewall / anti-contamination gates (`P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`,
`POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`), the `EMERGE-OBS` observation obligation, Phase-4 institution
locks (exec `11`), authoring/validation gates (exec `08`), and the diagnostics/review-artifact posture
(exec `10`). For **every** finding, record what execution **already** covers before asserting a gap —
many routed items are *partial coverage / consolidation*, not blank holes. Recommend only the
temporal / Block-R / staged-incompleteness **delta**. Do **not** reopen settled `0028` (or earlier
`0009…0025`) work.

### 3.4 Execution-doc altitude — substance + home, not ratified gate text

For each finding, deliver **(a) the execution doctrine the target doc must own** — in your own prose, at
**execution altitude**: gate procedure, certification-sequence placement, proof obligations,
golden-fixture *families* (positive + adversarial), diagnostic/observability expectations, and
acceptance-contract additions — and **(b) which execution file it lands in** (new gate / new subsection /
addition / correction). Do **not**:

- author final ratified gate wording or **invent new gate codes / observation-obligation codes /
  fixture names** (those remain the repo's own reassess/ticket process; you may *propose that a new gate
  or fixture family is needed* and describe its shape/intent, but not mint its identifier);
- choose **concrete temporal values** — day-part / "yesterday" / "office closed" / "due/late" / "stale"
  vocabulary, calendar/date syntax, duration units, stale-after thresholds, scheduler queue/algorithm,
  fairness formulas, UI clock format, missed-summary thresholds, simultaneity/tie-break rules;
- choose **inventory/economy schemas**, unit vocabularies, money denominations, or affect/learning
  update rules/decay/thresholds.

All such concrete choices **route down to future scoped implementation specs** (§3.5). Execution docs own
the *gate and proof obligation*; the future spec chooses the *value that satisfies it*.

### 3.5 Forward-routing is real, not degenerate

Execution is tier 2; **reference (tier 3) and specs (tier 4) sit below it**, so the forward-routing
appendix is substantive (this is **not** a lowest-tier terminal pass):

- **→ reference session (tier 3):** temporal-authority / holder-known-temporal-claim / procedural-time /
  freshness-staleness / time-acceleration / temporal-ancestry **glossary terms**, and **risk-register**
  notes (clock-oracle leakage, raw-wall-clock contamination, omniscient lateness/office-closed labels,
  UI time-acceleration leaks, silent LOD temporal fill-in, truth-cache learning, performance-pressure-
  as-invisible-director, staged-abstraction false certification). Reference is **not yet temporally
  amended** — these are net-new hand-offs to a future reference pass, authored after execution wording
  stabilizes.
- **→ future scoped specs (tier 4):** concrete temporal vocabulary/calendar/duration/thresholds; the
  first temporal-firewall fixture package (friendly + adversarial); inventory/economy fixtures;
  affect/learning depth + update rules; domain-pack bias assumption vehicle; budget/fairness numeric
  targets; the staged-abstraction declaration addition to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
  The spec tier is **not** an amendment target — these are future implementation-spec candidates only.

### 3.6 Preserve upstream invariants and existing bundling guidance

No recommendation may weaken `INV-099…INV-112`, `INV-087` (human focus is not player privilege), the
`INV-111` **observer-only** emergence-evidence carve-out (temporal/fairness review summaries stay
observer-only, non-certifying, never simulation inputs, never actor-known urgency), or possession parity
(`POS-PARITY` / `INV-006`/`INV-108`: possession is not a temporal-knowledge upgrade). Carry the seeds'
bundling guidance: the **quantity/economy proof package should not be bundled with the temporal proof
package** unless one gameplay feature genuinely needs both (architecture report §6.3); and the
budget/fairness seam lives in one consolidated place (architecture `0032` R-D), so its execution proof
should not be triplicated across temporal + scheduler + observability gates — cross-reference instead.

### 3.7 Filename and placement

`assumption:` the deliverable is `reports/execution-tier-alignment-research-report.md` (the `-alignment-`
suffix matches the cascade's per-tier convention — `foundation-tier-alignment`, `architecture-tier-alignment`).
An older first-cascade namesake exists at `archive/reports/execution-tier-alignment-research-report.md`;
that is a **different campaign** (0028 doctrine alignment) and is not overwritten — your report is a new
file in live `reports/`.

---

## 4. The task

Produce an **execution-tier alignment research report** (a foundational/doc-overhaul **downward-cascade
realignment**) that determines precisely what `docs/2-execution/*` must change so the execution tier
fully operationalizes (a) the now-ratified temporal-authority doctrine in foundation/architecture and
(b) the completeness-routed Block-R themes and (c) the staged-incompleteness declaration discipline —
at execution altitude (gates, certification sequence, proof obligations, fixture families, diagnostics,
acceptance contracts), recording existing coverage first and recommending only the delta, while routing
all concrete values and terminology to the reference and spec tiers below.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above (the manifest is the full path
inventory at `c70d119`). Research online as deeply as needed — discrete-event/agent-based simulation
time and scheduling, deterministic simulation testing / replay, provenance (PROV), schema/policy-as-code
validation, fairness/starvation in agent scheduling, staged-delivery / acceptance-honesty practice, and
any prior art that sharpens an **execution proof obligation** — wherever it sharpens the deliverable.
The predecessor reports' §8/§7 reference lists are a strong starting bibliography. Cite every external
source that shapes a decision. External research informs tier-fit and proof shape; it never becomes
Tracewake doctrine.

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every recommendation must
  satisfy `INV-001…INV-112`. A genuine divergence would require amending an invariant first; you may
  not design execution gates against it. If you find such tension, surface it as an open question /
  upstream-route, do not encode it.
- Authority order: if execution conflicts with architecture or foundation, **execution is wrong**; if
  implementation convenience conflicts with the accepted gates, implementation is wrong. Execution may
  be radical about stale sequencing but conservative about authority — it may rename/merge/split/retire
  execution docs, but may **not** soften foundation or architecture (exec `00` maintenance rule).
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination (the spine of this campaign): **no simulation fact may be born from prose**; the
  **temporal firewall** is the truth firewall applied to time — the scheduler/replay clock may order
  and validate, but cognition, routine selection, institutional procedure, embodied views, speech
  interpretation, leads, and LOD promotion may use temporal facts **only** when those facts reached the
  relevant holder through modeled channels. Preserve event-sourced causality, subjective epistemics,
  ordinary agents, possession parity, fallible institutions, questless leads, validation/replay, and
  observer-only emergence evidence.

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

- **`reports/execution-tier-alignment-research-report.md`** — **new** (not a replacement; not a numbered
  spec). This is an **analysis / recommendation report** for `docs/2-execution/*`. The numbered-spec
  numbering / ledger / epoch rules do **not** apply (this is not a `docs/4-specs/` artifact and mints no
  spec number). It delivers **substance + home, not ratified execution text** — for each finding, the
  execution doctrine the target doc must own (your prose, at execution altitude) and which execution
  file it lands in, explicitly **without** final gate wording or invented gate/fixture identifiers.

Use the canonical report shape this campaign reuses:

1. **Exact-commit evidence ledger** — repository, target commit `c70d119`, manifest-as-path-inventory
   posture, fetched-files list, no-branch/no-clone/no-code-search attestation, and a **stale-provenance
   quarantine** note for the older commit strings inside fetched predecessor reports.
2. **Disposition table** — one row per finding → driver (T / R / S) → target execution doc(s) → verdict
   (`belongs in execution` / `partial coverage` / `already-owned-close` / `route-forward`) → one-line
   basis.
3. **Method & provenance ledger** — authority/altitude rule; files read and role; settled context **not**
   re-commissioned (the `0028` first cascade + `0009…0025` hardening); external-research role.
4. **Execution-doc coverage register** — one row per `docs/2-execution/00…13`, current alignment verdict,
   coverage note + recommended posture (so high file count stays legible and you do not "correct" docs
   the drivers do not touch).
5. **Per-finding analysis & recommendations** — for each finding: **driver** (the foundation/architecture
   contract it descends from, cited to the live doc), **current execution coverage** (validate-before-gap),
   **tier-fit verdict**, **recommendation — substance and home** (execution-altitude proof obligation +
   target file, no concrete values/identifiers), and **external rationale** where it shaped the call.
6. **Forward-routing appendix** — **reference-tier hand-offs** (glossary + risk register) and **future
   spec / amendment-process hand-offs** (concrete temporal values, fixture packages, the
   `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` staged-abstraction addition, inventory/economy, affect/learning
   depth, bias domain-pack vehicle, budget/fairness numeric targets). This appendix is **substantive**,
   not degenerate — reference and specs are real tiers below execution.
7. **Open questions** — owner decisions that cannot be settled from the docs (carry forward, do not
   invent answers).
8. **References** — exact-commit repository sources (raw URLs at `c70d119`) + external sources cited.

**Locked / no-questions:** Produce the deliverable directly as a downloadable markdown document. Do not
interview, do not ask clarifying questions — the requirements above are final. If a genuine contradiction
makes a requirement impossible, state it in the deliverable and proceed with the most faithful
interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable set matches §7 exactly: one new `reports/execution-tier-alignment-research-report.md`,
  an analysis/recommendation report, no numbered spec, no invented gate/observation/fixture codes and no
  final ratified gate wording.
- All three drivers (T temporal, R completeness Block-R, S staged-incompleteness) are covered, each
  finding tied to its **live** foundation/architecture driver at `c70d119` (not to a predecessor report's
  pre-enactment proposal).
- Validate-before-gap is visible: every finding records existing execution coverage before recommending
  a delta; no `0028`/`0009…0025` settled work is re-commissioned.
- No recommendation weakens an upstream tier; `INV-099…INV-112`, `INV-087`, the `INV-111` observer-only
  carve-out, and possession parity are preserved; no concrete temporal value / schema / threshold /
  identifier is chosen at execution altitude (all routed forward).
- The forward-routing appendix hands temporal terminology/risk to a future reference session and concrete
  values/fixtures/template changes to future scoped specs.
- Every external claim that shapes a decision is cited.
- The `c70d119` fetch-baseline contains every file named in §2 (the manifest is that exact tree); no fetch
  used a branch name, clone, or code search.
