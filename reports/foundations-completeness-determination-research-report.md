# Foundations completeness determination research report

**Destination:** `reports/foundations-completeness-determination-research-report.md`  
**Repository:** `joeloverbeck/tracewake`  
**Target commit:** `d7fc746575713202eb6ce5a51b07e71c789c908d`  
**Status:** determination and routing report; not a ratified amendment; not a numbered implementation spec.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetched files only by exact commit URL from `joeloverbeck/tracewake`.

## Executive determination

The foundation is strong enough to build from. The nine verdict themes do **not** expose nine new constitutional pillars. They expose one genuine foundation-level gap and eight lower-tier doctrine/proof gaps.

The one foundation promotion is **time / calendar / social rhythm**. Tracewake already relies on time everywhere — event order, replay, stale beliefs, routine windows, office hours, debt lateness, no-human days, LOD summary intervals, regional cadence — but the foundation does not yet name a temporal authority model. That absence is constitutional because ad hoc tick usage can corrupt replay, leak perfect clock truth into actor cognition, and make institutions/routines treat abstract time as hidden omniscience.

Everything else should be routed below foundation. Play legibility, granularity/fungibility, bounded affect, learning/adaptation, compiler-like authoring enforcement, deterministic performance/fairness budgets, practical bias modeling, and staged incompleteness are important, but their required shape is subsystem contract, execution proof, review risk, glossary discipline, or future scoped spec work. Promoting them all would turn the foundation into a court case for every feature and would violate the project’s own tier discipline.

---

## 1. Disposition table

### 1.1 Verdict themes from `reports/verdict-on-foundations.md`

| # | Theme | Verdict | Target tier + target doc(s) | One-line basis |
|---:|---|---|---|---|
| 1 | Play experience and epistemic legibility | **Route** | Tier 1: `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; Tier 1: `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`; Tier 3: `docs/3-reference/01_DESIGN_RISK_REGISTER.md`; possible future Tier 4 TUI/play-loop spec | Foundation already requires TUI-first, playable, actor-filtered, legible why-not surfaces; the missing work is UX loop, friction control, lead/notebook usefulness, and information hierarchy under uncertainty. |
| 2 | Time, calendar, and social rhythm doctrine | **Promote** | Tier 0: primarily `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`; cross-reference additions to `04`, `05`, `07`, `10`, and `12` as needed; then route mechanisms to architecture/execution | Current docs use time as an implicit substrate everywhere but do not name the authority split among event time, actor-perceived time, institutional time, routine/social rhythm, staleness, deadlines, and LOD intervals. |
| 3 | Quantity, material granularity, and fungibility | **Route** | Tier 1: `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`; Tier 2: `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`; future Tier 4 inventory/economy fixture spec | Foundation already separates ownership/custody/access/control/proof/belief and requires evented food/money/work; object-vs-stock, lots, partial transfers, pooled goods, spoilage, and service capacity are architecture/spec model choices. |
| 4 | Bounded affect/emotion doctrine | **Route** | Tier 1: `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; Tier 1: `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; Tier 3: `docs/3-reference/02_GLOSSARY.md` and `01_DESIGN_RISK_REGISTER.md` | Foundation already treats needs, values, duties, motives, fear, and social pressure as bounded pressures; affect needs architectural typing as actor-known pressure/salience, not new constitutional weight. |
| 5 | Learning and adaptation | **Route** | Tier 1: `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; Tier 1: `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`; Tier 2: `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; Tier 2: `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Memory freshness/provenance is settled; the new issue is experience-derived expectation/method/trust/risk/routine updates that must remain provenance-bearing and actor-known. |
| 6 | Authoring/compiler discipline | **Route** | Tier 2: `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`; Tier 1: `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; Tier 3 risk register | Foundation already bans scripted outcome chains and permits authored causal machinery; “content is executable doctrine and statically inspectable” is enforcement/tooling posture, not a new constitutional pillar. |
| 7 | Deterministic performance and fairness budgets | **Route** | Tier 1: `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`, `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`, `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`; Tier 2: `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`, `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Foundation already requires deterministic replay, auditable randomness, honest LOD, and no invisible human-focus privilege; budgets, starvation evidence, skipped-cognition policy, and time acceleration proofs are scheduler/execution contracts. |
| 8 | Practical bias/social-harm doctrine | **Route** | Tier 1: `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`; Tier 2: `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`; Tier 3: `docs/3-reference/01_DESIGN_RISK_REGISTER.md` and `02_GLOSSARY.md`; future domain-pack guidance/spec | Foundation already requires fallible, biased, stale, underfunded, wrong institutions; the missing lesson is inspectable model inputs, causal diagnostics, and domain-pack ownership of culturally specific assumptions. |
| 9 | Deliberately staged incompleteness | **Route** | Tier 2: `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`, `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; Tier 3: `docs/3-reference/01_DESIGN_RISK_REGISTER.md`; Tier 4: `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Foundation already defines first playable scope, deferred non-goals, and harsh acceptance; the escape valve should be an execution/reference/spec-template declaration discipline for honest abstraction. |

### 1.2 Newly surfaced candidates

No independent newly surfaced candidate warrants separate promotion or routing. External research reinforced the nine verdict themes but did not expose a tenth hole that is not already covered by the nine or by the settled post-0006…0025 campaign. The tempting “domain-pack assumption manifest” candidate is folded into Theme 6 (authoring/compiler discipline) and Theme 8 (practical bias modeling), not treated as a standalone finding.

---

## 2. Method and provenance ledger

### 2.1 Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: d7fc746575713202eb6ce5a51b07e71c789c908d
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open pre-view + container.download exact raw URL fetch
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/foundation-amendment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/foundation-amendment-lower-tier-routing.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/execution-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/reference-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/reports/specs-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/d7fc746575713202eb6ce5a51b07e71c789c908d/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

The fetched current-report files include some historical reports whose contents mention older exact commit URLs as part of their own provenance. Those historical mentions were not used as fetch targets and do not change the evidence boundary for this determination.

### 2.2 Tier-fit test applied

The tier-fit test comes from `docs/README.md`, which defines the documentation hierarchy:

- `0-foundation/` defines project identity, constitutional invariants, first playable proof, and truth-firewall doctrine. It may not encode implementation shortcuts, phase-local exceptions, or player privilege.
- `1-architecture/` owns stable subsystem contracts: authority ownership, data flows, subsystem boundaries, diagnostics, and acceptance implications.
- `2-execution/` owns gate order, certification sequence, fixtures, validation, audit criteria, and review artifacts.
- `3-reference/` owns compact terminology, checklist discipline, source-discipline guardrails, and risk memory.
- `4-specs/` owns narrow implementation or corrective work under live doctrine and does not replace architecture or foundation.

Each verdict theme was tested in this order:

1. **Current coverage first.** I identified what the exact-commit live docs already own. A theme was not treated as a hole merely because the seed named it.
2. **Constitutional necessity.** I asked whether the theme defines what Tracewake is or what it must never become, or whether it instead defines a mechanism, proof obligation, review risk, term, or scoped implementation package.
3. **Truth-firewall safety.** I checked whether the recommendation preserves the rule from `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`: truth may validate, but truth may not plan.
4. **Anti-contamination posture.** I rejected or routed any version of a theme that would allow prose-born facts, hidden truth in cognition, debug truth in embodied views, authored outcome chains, or invisible scheduler authority.
5. **No bloat default.** Ambiguous items went downward, not upward. The foundation receives only material whose absence would let lower tiers invent incompatible doctrine.

### 2.3 Settled seven boundary

The prior campaign adjudicated seven anti-contamination / hardening themes: provenance sufficiency, memory freshness, believed-access affordances, single-charge derived accounting, emergence-as-evidence, falsifiability/anti-vacuity, and acceptance-evidence/fingerprint honesty. This report does not re-open those determinations.

Where the nine verdict themes overlap those seven, this report defers to the settled disposition:

- Learning/adaptation overlaps memory freshness and provenance sufficiency, so this report routes only the additional “experience changes future expectation/method/trust/routine” layer.
- Authoring/compiler discipline overlaps provenance sufficiency, no-script compliance, and acceptance-evidence honesty, so this report routes tooling/static-inspection posture below the already-ratified foundation doctrine.
- Staged incompleteness overlaps falsifiability/anti-vacuity and acceptance-evidence honesty, so this report routes a declaration template rather than adding a new invariant.
- Play legibility overlaps TUI/view-model proof and embodied affordances, so this report routes UX/information-hierarchy doctrine without weakening existing actor-known view rules.

### 2.4 External research role

External research was used to sharpen tier placement, not to override project doctrine. The sources fall into these decision-support buckets:

- Game design and social-simulation precedents: MDA, Talk of the Town, Prom Week / Comme il Faut, Neighborly. These support the play-legibility and living social simulation judgments. [EXT-MDA], [EXT-TOTT], [EXT-PROMWEEK], [EXT-CIF], [EXT-NEIGHBORLY]
- Agent-based/discrete-event time and scheduling literature: DeMO, continuous-time ABM work, reliable ABM scheduler discussion. These support the finding that time semantics are not a mere tick-size detail. [EXT-DEMO], [EXT-CT-ABM], [EXT-ABM-SCHED]
- Affect/emotion modeling: OCC and emotion-modeling surveys. These support bounded affect as a pressure/salience model, not an omniscient mood oracle. [EXT-OCC], [EXT-EMOTION-SURVEY]
- Provenance, schema, and policy-as-code practice: W3C PROV, JSON Schema, Open Policy Agent. These support compiler-like authoring validation and provenance-shaped data discipline. [EXT-PROV-DM], [EXT-JSON-SCHEMA], [EXT-OPA]
- Deterministic simulation/replay practice: deterministic simulation testing and deterministic lockstep discussions. These support replayable budgets and deterministic scheduling as execution evidence, not ad hoc optimization. [EXT-DST], [EXT-LOCKSTEP]
- Bias and documentation practice: NIST AI RMF and Datasheets for Datasets. These support inspectable assumptions, explicit bias-risk ownership, and documentation of domain-pack assumptions. [EXT-NIST-RMF], [EXT-DATASHEETS]
- Staged-incompleteness practice: MVP/validated learning, technical debt, ADRs, and architecture fitness functions. These support explicit abstraction records and objective acceptance checks. [EXT-LEAN-MVP], [EXT-TECH-DEBT], [EXT-ADR], [EXT-FITNESS]

---

## 3. Per-theme determinations

### 3.1 Play experience and epistemic legibility

#### Driver

The seed argues that the docs are excellent at preventing the wrong product but less explicit about why the right product is enjoyable minute to minute: what the player does for thirty minutes, what replaces quest objectives, how much friction is acceptable, how lead cards become useful rather than bureaucratic, and how ignorance is surfaced without making the player feel punished.

The external research supports the concern. The MDA framework’s value here is not its taxonomy but its reminder that a game design cannot be judged only by rules and correctness; mechanics create dynamics that create player-facing experience. Tracewake has unusually strict mechanics — actor-known cognition, no hidden truth, no quest objectives — so its lower tiers need an explicit design contract for the play dynamics those mechanics are meant to create. Talk of the Town, Prom Week, and Neighborly all reinforce that believable social/epistemic simulation needs a playable interaction layer; simulation depth alone is not a play contract. [EXT-MDA], [EXT-TOTT], [EXT-PROMWEEK], [EXT-NEIGHBORLY]

#### Current coverage

The foundation already owns the constitutional part:

- `docs/0-foundation/01_PROJECT_CHARTER.md` frames Tracewake as TUI-first, playable, ordinary-life simulation rather than an omniscient mystery UI or research-only prototype.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` says the TUI is a primary product interface, every runnable phase has a TUI/view-model gate, embodied mode shows actor-known reality, why-not explanations are mandatory, and mechanics hidden from play are unfinished.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` owns embodied mode, debug quarantine, possession, actor-known view models, action menus, why-not explanations, notebooks, leads, notes, lead wording, conversation UI, time controls, and automated acceptance.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` requires the first serious proof to be TUI-playable with view-model gates and actor-knowledge leakage gates.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` prevents any legibility solution from cheating by showing hidden truth in embodied views.

The lower tiers already own part of the mechanism:

- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` says embodied view models are actor-legible, debug views are non-diegetic, view-models are separate, why-not output is actor-legible, and TUI transcripts are acceptance artifacts.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` defines proof obligations around actor-filtered view models and debug quarantine.
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` is the natural home for lead/story-sifting projections without turning them into quest steps.

The gap is not “the foundation forgot play.” It is narrower: the current docs define what the TUI must not leak and that it must be playable, but they do not yet own a lower-tier contract for information hierarchy, friction budgets, inquiry loops, lead/notebook usability, or player comprehension when the correct view is ignorance.

#### Tier-fit verdict

**Route.** The constitutional bar is already met: Tracewake must be TUI-first, actor-filtered, playable, legible, and non-omniscient. The remaining issue is product-surface design under those constraints. That is architecture and reference, with a possible later spec.

Promoting “legible pleasure” to the foundation would be too vague unless it becomes an aesthetic slogan. A foundation sentence like “the game must be fun” would not constrain architecture more than the existing TUI-first and why-not invariants. The useful work is concrete: how view models prioritize actor-known place, needs, risks, leads, blockers, notebook entries, and ordinary actions without leaking truth.

#### Recommendation

Target `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` with a new or expanded embodied-play subsection. At architecture altitude, it should own:

- the player’s ordinary investigation / survival / social-navigation loop under actor-limited knowledge;
- view-model information hierarchy for ignorance, uncertainty, stale belief, contradiction, lead, blocker, and why-not output;
- friction controls that preserve actor-bounded realism but avoid unnecessary menu bureaucracy;
- lead/notebook semantics as attention aids rather than quest instructions;
- transcript evidence that a player can understand useful next ordinary actions without debug truth.

Target `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` for the lead-card/story-sifting side: leads may organize attention and retrospectively surface patterns, but they must not become objective markers, culprit hints, or director steering.

Target `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with a compact relapse risk: Tracewake could become a perfect audit machine that is correct but hostile to play, or could overcorrect by leaking truth for convenience.

A future Tier 4 TUI/play-loop spec is appropriate if it stays narrow: transcript fixtures, sample embodied sessions, view hierarchy examples, and non-leak usability probes. That spec should not define new doctrine.

---

### 3.2 Time, calendar, and social rhythm doctrine

#### Driver

The seed argues that time is everywhere in the docs but lacks a named temporal ontology. It asks Tracewake to distinguish physical time, actor-perceived time, institutional time, routine windows, social rhythm, deadlines, lateness, expected-by-now thresholds, stale records, memory aging, seasonal/regional cadence, and LOD summary intervals.

This is the strongest foundation candidate. Discrete-event and agent-based simulation literature treats time representation as a modeling choice with semantic consequences, not a mere implementation tick size. DeMO’s ontology work is directly relevant because it separates simulation concepts rather than letting tool mechanics collapse them. Continuous-time ABM and scheduler-focused ABM work also reinforce that execution order, event times, and scheduling policy shape model meaning. [EXT-DEMO], [EXT-CT-ABM], [EXT-ABM-SCHED]

#### Current coverage

The current foundation uses time heavily:

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` owns event authority, event anatomy, cause models, traces, deterministic replay, randomness, boundary inputs, decision traces, provenance-carrying events, and LOD summaries.
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` defines belief source, acquisition time, last verification, stale status, memory fallibility, expectation contradiction, stale beliefs, testimony, records, and actor-known cognition.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` owns needs, routines, durable intentions, bounded local planning, event-driven replanning, and routines versus schedules.
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` uses time for hunger, fatigue, sleep, work, wages, payment, debt, travel, search, and no-human ordinary life.
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` uses time in records, notices, procedures, reports, stale or delayed institutional outcomes, office-like public roles, and budgets/resources.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` owns time controls at the product boundary.
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` owns LOD, regional processes, summary events, promotion/demotion, long prehistory, demography, seasons/weather/disasters, regional cadence, and human focus/fairness.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` requires no-human simulation and the missing-property proof to work over ordinary time.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` requires cognition to use sealed actor-known context; perfect truth, including perfect time truth where not actor-known, may validate but may not plan.

Architecture and execution also touch it:

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` has actor decision transactions, need pressures, routines, stuck diagnostics, and action proposal ancestry.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` has acquired ticks, stale-after ticks, memory, rumors, records, and leads.
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` includes summary-event tick windows, daily routines at high detail, seasonal food pressure, regional movement, and promotion ancestry.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` rejects unrecorded wall-clock, unordered iteration, nondeterministic randomness, and direct dispatch.
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` preserves later notices, travel, LOD, and second-proof timing as locked deferrals.

The gap is that these uses are distributed without a single foundation-level authority split. A lower-tier implementer can infer ticks, stale intervals, office windows, and LOD windows piecemeal, but no foundation doc says what kinds of time exist in Tracewake and which ones may feed cognition.

#### Tier-fit verdict

**Promote to foundation.** This is not a request for tick size, calendar syntax, duration units, scheduler data structures, or sleep-window algorithms. Those belong below foundation. The foundation gap is the conceptual authority model.

Time is constitutional in Tracewake because:

- event time determines replay and causality;
- actor-perceived or remembered time determines what an agent may believe or plan from;
- institutional time determines office hours, deadlines, records, lateness, sanctions, and procedure state;
- routine/social rhythm determines expectation and absence-as-evidence;
- stale intervals determine when memory/records remain usable but contested;
- LOD summary intervals determine what was simulated, summarized, or withheld from promoted holders.

Without a foundation temporal doctrine, lower tiers can accidentally create hidden-truth planning through clock omniscience. A worker can “know” a shop is closed because the scheduler read true office hours; a debt can become late without a modeled due rule; a witness can remember exact ticks instead of culturally/actor-plausible temporal claims; a low-LOD summary can compress days but omit what it did not simulate. These are constitutional risks, not merely implementation details.

#### Recommendation

Target `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` as the primary home. Add a foundation-level temporal authority section. At foundation altitude, it should own the distinction between:

- authoritative event/replay time;
- actor-known, actor-perceived, or actor-remembered time;
- institutional/procedural time;
- routine windows and social rhythm;
- expectation thresholds and lateness;
- memory/record freshness and staleness;
- regional/seasonal cadence and LOD summary intervals.

The section should explicitly preserve the truth firewall: perfect simulation time may validate event ordering and action legality, but cognition, institutional procedure, embodied views, and LOD promotion may use only the temporal facts available through modeled channels, records, routines, perception, roles, or accepted summary ancestry.

Cross-reference additions should be modest:

- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`: clarify that temporal claims are claims; “yesterday,” “last night,” “late,” “stale,” and “expected by now” are modeled/holder-known, not free truth labels.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`: clarify that routines and schedules consume actor-known/institution-known temporal expectations.
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`: clarify institutional time for deadlines, office windows, filing delays, and procedural aging.
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`: connect LOD summary intervals and regional cadence to the temporal authority model.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`: require first playable proof not to treat time as an invisible oracle for actor decisions.

Downstream sessions should then encode mechanisms in architecture and execution. The foundation should not pick a tick size, calendar format, UI clock rendering, scheduler algorithm, or exact stale-after durations.

---

### 3.3 Quantity, material granularity, and fungibility

#### Driver

The seed argues that missing-property gameplay will break if money, food, stock, partial transfers, pooled household goods, divisible goods, spoilage, and fungible goods are not modeled with a clear granularity doctrine. It asks when money is a physical object versus balance, when food is an item versus stock/service capacity, how partial transfers leave traces, and how abstraction preserves custody.

External provenance standards support the framing: identity, derivation, activity, and agent relationships matter when a thing or amount is transformed, consumed, moved, merged, split, or represented by a record. W3C PROV is not an inventory model, but it reinforces the principle that derived states should carry usable lineage rather than appearing as raw totals. [EXT-PROV-DM], [EXT-PROV-SEM]

#### Current coverage

The foundation already owns the constitutional property and ordinary-life rules:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` distinguishes ownership, custody, access, control, proof, and belief. It also requires ordinary economy to be simplified but not fake.
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` owns food/hunger, work/wages, money/payment/debt, storage/containers/property expectations, ownership/custody/access/control/proof, search, travel, local economy, and no-human ordinary life.
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` owns households, records, notices, reports, procedure failure, budgets/resources, and role obligations.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` requires first playable actions around taking, storing, hiding, finding, buying, paying, refusing, eating, sleeping, working, searching, reporting, and missing-property proof.

Architecture already narrows the first-slice economy:

- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` owns settlement ontology, spaces, property/custody/access, food/sleep/work, household resources, route adjacency, first-slice economy, and missing-property constraints. It explicitly warns not to introduce market abstractions that erase provenance before the missing-property proof is stable.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` requires content validation for authored beliefs, records, routines, workplaces, home, food knowledge, expectations, and prehistory facts, with provenance.

The current coverage is strong at the level of social/legal/epistemic separations. The weakness is material modeling: the docs do not yet say how a fungible quantity remains traceable when split, mixed, consumed, stored by household, or represented as service capacity.

#### Tier-fit verdict

**Route.** This is not foundation unless the project is missing the high-level rule that custody and proof matter. It is not missing that rule. The remaining issue is how the settlement/economy/property subsystem represents quantities and transformations.

Promoting detailed fungibility doctrine to foundation would overfit the first proof and risk freezing implementation choices too early. The foundation should not decide whether coins are unique entities, lot-counted quantities, ledger entries, or a hybrid. It should require that the chosen representation preserve custody, provenance, transfer, consumption, and proof sufficiently for ordinary action and missing-property play. Architecture and future specs should do the rest.

#### Recommendation

Target `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` with a material granularity/fungibility contract. At architecture altitude, it should own:

- when the system uses unique items, quantity stacks/lots, abstract capacities, or records/ledgers;
- how split, merge, consume, spoil, reserve, share, transfer, hide, discover, and compensate operations preserve event ancestry;
- how household pools differ from individual custody;
- how simple money/payment/debt avoids becoming fake score deltas;
- how food stock and service capacity remain actor-known and institution-known rather than omniscient;
- how missing or short quantities can create absence-as-evidence without implying unique identity where none exists.

Target `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` for validation: content should not be allowed to define stocks, balances, household goods, or service capacities without custody/provenance rules appropriate to their abstraction level.

A future Tier 4 inventory/economy fixture spec is appropriate. It should include adversarial fixtures for partial transfers, pooled household stores, indistinguishable coins/food, consumed goods, stale stock records, and custody/proof disagreements. It should not become a general market simulator spec before the first proof is stable.

---

### 3.4 Bounded affect/emotion doctrine

#### Driver

The seed argues that the docs mention fear, shame, loyalty, affection, greed, duty, pride, resentment, social caution, and relationships, but affect may become either decorative or a hidden utility weight unless it is explicitly bounded. The desired doctrine: emotions are not truth; they are pressures and memory modifiers.

Emotion-modeling research supports the route. OCC-style cognitive emotion models connect emotions to appraisals of events, agents, objects, goals, standards, and attitudes, while social-simulation surveys warn that ad hoc emotion models are common. The useful lesson for Tracewake is not to import a full psychological model; it is to prevent affect from becoming omniscient, decorative, or a hidden director. [EXT-OCC], [EXT-EMOTION-SURVEY]

#### Current coverage

Foundation `05` already owns much of the substance:

- Needs, values, duties, and motives are pressures, not scripts.
- Utility scoring is a bounded heuristic.
- Routines are defeasible, not rigid schedules.
- Event-driven replanning and durable intentions are part of agent doctrine.
- Agents are symbolic and inspectable.
- Decisions require debug traces.

Architecture also already has relevant language:

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` lists hunger, fatigue, safety, social obligation, duty, curiosity, fear, and role pressure as candidate-goal pressures. It forbids need pressure from naming true hidden targets.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` says memories can be shaped by role/relationship and that rumors/testimony carry trust stance, possible distortion, time, speaker/listener, and confidence.

The current foundation does not explicitly name affect as a class, but its existing “pressures, not puppet strings” doctrine covers the constitutional part. The missing work is architectural: how affect becomes candidate pressure, salience, belief uptake, memory durability, confession/concealment pressure, risk avoidance, accusation, repair, or routine disruption without becoming truth.

#### Tier-fit verdict

**Route.** Affect does not require foundation promotion because foundation already gives the governing rule: pressures may influence candidate generation and scoring, but they cannot name hidden truth or compel scripted outcomes. The remaining design needs typed architecture and review risks.

A new foundation affect doctrine would be either too vague (“emotions matter”) or too mechanistic (“anger adjusts accusation weights”). The former adds nothing; the latter belongs below foundation.

#### Recommendation

Target `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` with bounded affect as a subtype or companion of need/motive/relationship pressure. It should own:

- affect as actor-known/internal state derived from modeled events, memories, relationships, values, duties, bodily needs, testimony, or records;
- affect as pressure, salience, bias toward candidate goals, and routine disruption, not truth and not direct action dispatch;
- examples such as fear/risk avoidance, shame/concealment, guilt/repair, grief/routine disruption, anger/accusation or retaliation pressure, affection/help or belief uptake;
- debug traces showing affect input, provenance, and decision effect without implying hidden truth.

Target `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` for memory and belief effects: emotional salience may affect durability, recall, distortion, trust, and rumor spread only through source-bearing actor-known structures.

Target `docs/3-reference/02_GLOSSARY.md` for compact terminology distinguishing need, motive, duty, value, relationship, affect, salience, and mood-like state. Target `docs/3-reference/01_DESIGN_RISK_REGISTER.md` for the relapse risk that affect becomes a hidden utility oracle or decorative display string.

---

### 3.5 Learning and adaptation

#### Driver

The seed argues that memory, belief, staleness, plan repair, and intentions are covered, but learning should be separated from memory. Agents should learn that a route is unreliable, a clerk refuses weak reports, a household member lies under pressure, a food source is often empty, a search method fails, an actor is dangerous, or a routine needs a buffer. The danger is hidden truth caching.

Talk of the Town and related social-simulation work reinforce the importance of modeled knowledge propagation, forgetting, misremembering, and belief update. The relevant lesson is not to build a general ML system. Tracewake needs symbolic, provenance-bearing adaptation that changes future expectation/method/trust/risk/routine selection. [EXT-TOTT], [EXT-NEIGHBORLY]

#### Current coverage

This theme overlaps the settled memory-freshness/provenance campaign, so the foundation is already strong:

- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` owns claims, beliefs, observations, testimony, records, memory fallibility, stale beliefs, identity uncertainty, provenance, false beliefs, negative information, and actor-known cognition.
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` owns bounded local planning, event-driven replanning, durable intentions, routines, ordinary competence, and actor decision transactions.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` is explicit that cognition consumes only sealed actor-known context.

Architecture already provides seams:

- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` includes candidate generation, method selection, local planning, intention lifecycle, stuck diagnostics, and decision traces.
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` provides belief shapes with source, channel, acquired tick, verification, staleness, observation links, contradiction links, and privacy scope.

The missing piece is naming the adaptive update as distinct from passive memory. A stale memory that “the pantry was empty” is not the same as a learned expectation that “that pantry is unreliable around supper,” nor is a failed search the same as a learned method penalty. The docs have the ingredients but not the contract.

#### Tier-fit verdict

**Route.** The foundation already forbids hidden-truth learning: cognition must be actor-known and provenance-bearing; stale beliefs remain meaningful; memory can be wrong. The additional doctrine is an architecture/execution contract for derived expectations and method updates.

Promoting learning to foundation risks re-opening the settled memory freshness/provenance work and risks inviting a vague “agents must learn” invariant. That would be bloat. The useful rule is concrete and lower-tier: learning is a source-bearing update caused by remembered experience or modeled instruction, and it affects future expectation, method selection, trust, risk, skill, or routine.

#### Recommendation

Target `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` for adaptive decision effects. It should own:

- learned reliability of routes, places, routines, search methods, agents, institutions, and resources;
- learned buffers or changed routine windows from repeated modeled failure;
- skill or method-selection changes only where the first-proof scope needs them;
- decision traces showing the source of the learned expectation and the way it changed candidate generation or scoring.

Target `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` for the epistemic representation: learned expectations should be derived claims/beliefs with source events, memory links, time, confidence/trust stance, staleness, and contradiction handling.

Target `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` with anti-hidden-truth learning tests: no route/food/clerk/danger/trust learning may derive from ground truth, fixture labels, debug diagnostics, or omniscient projections.

Target `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` with proof obligations around ordinary adaptation: repeated no-human days may show a routine buffer, search fallback, or trust adjustment only if the causal path is replayable.

---

### 3.6 Authoring/compiler discipline

#### Driver

The seed argues that no-scripting doctrine is philosophically strong but should be made more compiler-like: content is executable doctrine, and authoring tools should statically flag hidden culprit fields, task menus, source-less records, impossible routines, unfunded rewards, seed facts without provenance, action options not NPC-possible, LLM prompt packets with hidden truth, and fixtures that pass because labels imply answers.

External practice supports this as enforcement posture. JSON Schema shows how structure/constraints can be declaratively validated. Policy-as-code systems such as OPA show the value of making policy executable and auditable. W3C PROV reinforces provenance/lineage vocabulary. [EXT-JSON-SCHEMA], [EXT-OPA], [EXT-PROV-DM]

#### Current coverage

This theme is already heavily covered:

- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` states the central rule: authored causal machinery is allowed and required; authored outcome chains are forbidden. Designers author possibility space, not guaranteed arcs.
- The same foundation doc allows and requires authoring of actions, affordances, needs, routines, traits, values, relationships, norms, procedures, speech acts, trace types, claim vocabularies, regional processes, initial state, seeds, fixtures, and validation rules.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` includes no authored outcome chains, authored causal machinery, scenario seeds as tensions, prehistory marking, no-script compliance testing, and harsh feature acceptance.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` forbids holder cognition from hidden truth, debug truth, labels, booleans, or unproven raw physical truth.

Execution `08` is already unusually direct:

- It owns data-authoring acceptance and validation gates.
- It requires syntax/schema/version checks, deterministic IDs, referential integrity, canonicalization determinism, provenance sufficiency, action parity, no outcome chains, no player privilege, no hidden-truth cognition, no debug/omniscience in embodied data, replay compatibility, and useful error reports.
- It says forbidden forms include `player_character`, `quest_state`, `objective_marker`, `culprit`, `stolen_flag`, `npc_knows_truth`, and aliases/clever renames.
- It says keyword rejection is not enough; the schema must make forbidden concepts unrepresentable or clearly invalid by structure.
- It requires adversarial renamed fields and nested/embedded forbidden forms.

The seed’s “compiler discipline” phrase is valuable, but the substance is already foundation + execution. The gap is emphasis, consolidation, and review hand-off.

#### Tier-fit verdict

**Route.** Do not promote. Foundation already contains the rule. Execution already owns authoring validation. Architecture `13` and execution `08` should sharpen the static-inspection posture.

A foundation amendment saying “content is executable doctrine” would be rhetorically useful but not necessary. It would risk duplicating `09` and `INV-097` while blurring the tier line between doctrine and tooling. The right move is to make the lower-tier enforcement posture harder to miss.

#### Recommendation

Target `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` with an explicit compiler/static-inspection framing. At execution altitude, it should own:

- content packages, fixtures, seeds, prompt packets, and authored records as inputs that must be statically inspectable for constitutional violations before runtime;
- structured validation failures that identify the field/path, violated rule, layer, and reason;
- alias/nesting/renaming adversarial coverage for hidden-truth fields;
- rejection of source-less records, guaranteed routines, unfunded rewards, player-only actions, outcome labels, and prompt packets containing hidden truth;
- the rule that a fixture cannot pass because a display label, file name, or test label implies the answer.

Target `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` for the subsystem-level contract: validators and review artifacts are architecture-protecting seams, not best-effort lint.

Target `docs/3-reference/01_DESIGN_RISK_REGISTER.md` only if a compact relapse risk is missing or under-named: authoring data can smuggle director logic even when runtime code appears clean.

---

### 3.7 Deterministic performance and fairness budgets

#### Driver

The seed argues that LOD is correctly treated as ontology, but hundreds of agents require explicit replayable planner budgets, deterministic scheduling, starvation handling, skipped-cognition policy, time-acceleration honesty, debug/projection non-authority, and summary declarations of what was not simulated.

External systems practice supports this. Deterministic simulation testing emphasizes deterministic execution so failures can be reproduced. Deterministic lockstep discussions show that deterministic simulations must control inputs, randomness, and state divergence. ABM scheduler literature reinforces that scheduling policy is semantically meaningful, not just performance plumbing. [EXT-DST], [EXT-LOCKSTEP], [EXT-ABM-SCHED]

#### Current coverage

The foundation already owns determinism and LOD honesty:

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` requires meaningful changes to be evented, randomness to be seedable/auditable, deterministic replay, snapshots/compaction not to erase ancestry, and LOD summaries to preserve replay obligations.
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` says LOD is ontology, not just optimization; low-LOD people are still people; summary events preserve ancestry; promotion/demotion are explicit events; human focus is not player privilege; regional processes are declared causal processes; and scale cannot mean truth-contaminated or impossible fidelity.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` prevents LOD and scheduler machinery from injecting hidden truth into cognition.

Architecture/execution already own parts:

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` owns scheduling/action pipeline boundaries.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` owns actor decision transactions and stuck diagnostics.
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` owns LOD, regional processes, summary events, promotion/demotion, and contamination rules.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` rejects direct primitive world mutation and event application depending on unrecorded wall-clock, unordered iteration, nondeterministic randomness, or display strings.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` owns diagnostic/review evidence.

The gap is explicit budget/fairness language. The docs say “deterministic and auditable,” but they do not yet require planner/cognition budgets, skipped-cognition events, starvation diagnostics, or time-acceleration equivalence/declaration rules.

#### Tier-fit verdict

**Route.** This theme sounds foundational because it includes fairness and determinism, but the constitutional doctrine is already present. The remaining material is scheduler architecture and execution evidence.

The foundation should not define budget amounts, priority queues, turn slices, maximum agents per tick, LOD promotion thresholds, or profiling requirements. It should continue to require deterministic replay, honest abstraction, and no hidden privilege. Lower tiers must make those requirements operational.

#### Recommendation

Target architecture:

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`: scheduler authority, deterministic ordering, budget exhaustion, and feedback events.
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`: planner/cognition budgets and explicit “stuck/skipped/deferred” diagnostics when cognition is not run.
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`: time acceleration, summary intervals, what was actually simulated, what was summarized, and what was withheld from promoted holders.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`: observability contract for budgets/fairness.

Target execution:

- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`: proof that actor scheduling is deterministic, auditable, and not wall-clock/iteration-order dependent.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`: acceptance artifacts for planner budgets, skipped cognition, starvation/fairness, and replay equivalence.
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`: deferred LOD/time-acceleration proof obligations.

Add a reference risk if not already present: performance pressure can become an invisible director by silently skipping low-salience actors or giving high-salience/human-adjacent actors more causal opportunity without evented policy.

---

### 3.8 Practical bias/social-harm doctrine

#### Driver

The seed argues that the docs already require wrong suspicion, bias, credibility, social position, institutional fallibility, poverty, theft, punishment, lying, and social standing, but implementation needs practical anti-designer-bias rules: every bias as inspectable input; no protected/social category assumptions in the kernel; domain packs own culturally specific biases; bias effects debug-visible; wrong suspicion legible; institutional injustice causal rather than author prejudice disguised as realism.

External risk/documentation practice supports a practical rather than moralizing approach. NIST AI RMF emphasizes managing risks to individuals, organizations, and society, including fairness and harmful bias, and Datasheets for Datasets shows how documentation of motivation, composition, collection process, intended uses, and limitations can expose assumptions that otherwise remain invisible. Tracewake is not an AI model deployment, but the documentation lesson transfers: assumptions need owners, scopes, and review surfaces. [EXT-NIST-RMF], [EXT-DATASHEETS]

#### Current coverage

The foundation already names the constitutional part:

- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` includes procedure failure, bias/social position/credibility, budgets/resources/role obligations, fallible procedure chains, wrong or stale records, and institution/household norms.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` requires institutional fallibility to be tested and includes biased, stale, underfunded, refused, delayed, partial, wrong, misfiled, and jurisdiction-blocked outcomes where relevant.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` includes wrong suspicion for reasons, institutional fallibility gates, records, reports, testimony, partial witness information, and no-human proof.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` applies the truth firewall to institutions and records; institution procedure consumes institution-known context, not global truth.

Execution `11` already carries much of the next-stage lock:

- Phase 4 institutions must prove fallible world processes, not quest engines or truth oracles.
- Wrong suspicion may arise from partial, stale, biased, or misleading information.
- Records can be wrong, stale, forged, incomplete, lost, suppressed, or ignored.
- Golden fixtures include wrong, stale, refused, delayed, and misfiled cases.

The gap is practical modeling hygiene. The docs say bias exists and must be testable; they do not yet say enough about how bias enters the system as inspectable model input rather than designer assumption hidden in heuristics.

#### Tier-fit verdict

**Route.** This is not a foundation hole because the foundation already requires institutional fallibility and biased/wrong suspicion as part of Tracewake’s identity. The needed work is mechanism, evidence, terminology, and content/domain-pack ownership.

A foundation amendment saying “bias must be practical, not corporate ethics prose” would be stylistic, not doctrine. Architecture/execution can encode the practical rule: every biased outcome must be causally explainable through norms, roles, resource constraints, records, testimony, credibility models, procedure design, domain-pack assumptions, or actor/institution-known context.

#### Recommendation

Target `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`. At architecture altitude, it should own:

- bias as inspectable model input or procedure effect, never kernel-flavored assumption;
- no protected/social category semantics in genre-neutral kernel unless expressed through domain-pack concepts, norms, roles, records, or modeled social structures;
- culturally specific assumptions owned by domain packs, with reviewable provenance/assumption notes;
- wrong suspicion diagnostics showing the causal path: evidence, stale record, witness, credibility stance, role pressure, resource limit, norm, procedure, bias input, or failure mode;
- institution-known context and actor-known context remaining sealed under the truth firewall.

Target `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` with proof obligations:

- golden fixtures for biased testimony, unequal credibility, underfunded procedure, refusal, delay, misfiling, ignored records, and wrong suspicion;
- negative tests proving wrong suspicion cannot be produced by hidden culprit truth or designer labels;
- debug-only institutional audits that expose bias inputs and procedure effects without leaking into embodied play.

Target `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with a compact risk: “emergent injustice” can become author prejudice if causal inputs and assumptions are not inspectable. Target `docs/3-reference/02_GLOSSARY.md` for tight terms: bias, credibility, social position, protected/social category, domain-pack assumption, institutional fallibility, wrong suspicion.

Future domain-pack guidance may be needed, but it should be scoped; do not make the kernel carry culturally specific categories as universal ontology.

---

### 3.9 Deliberately staged incompleteness

#### Driver

The seed argues that the biggest risk is over-constitutionalization and asks for an operational escape valve: every staged feature should declare what it proves now, what it abstracts, what it must not fake, what future feature it must not block, and what acceptance gate prevents the abstraction from becoming a lie.

External practice supports this as execution/reference/spec discipline. MVP thinking is useful only when “minimum” still produces validated learning. Technical-debt writing is useful because deliberate debt must be named as a tradeoff rather than hidden as quality. ADR practice reinforces recording context and consequences for significant decisions. Fitness functions reinforce objective checks for evolving architecture. [EXT-LEAN-MVP], [EXT-TECH-DEBT], [EXT-ADR], [EXT-FITNESS]

#### Current coverage

The foundation already owns staged scope:

- `docs/0-foundation/01_PROJECT_CHARTER.md` explicitly refuses the easy wrong products and chooses small, inspectable ordinary-life proof first.
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` stages demography, combat/injury, organizations, regional scale, and impossible fidelity while protecting LOD ontology.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` defines first domain, first scale, required ordinary substrate, explicit non-goals, canonical regression seeds, and definition of done.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` includes harsh feature acceptance and rejects shortcuts that fake ordinary life, replay, TUI proof, possession parity, and no-script compliance.

Execution already owns deferral posture:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` defines execution authority and canonical gate posture.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` owns gate order and certification sequence.
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` is explicitly a locked-deferral contract for notices, travel, regional scale, LOD, and second-proof work.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` is the natural place for a structured acceptance/declaration template.

The gap is a standardized operational declaration, not a new foundation rule.

#### Tier-fit verdict

**Route.** The foundation already distinguishes first proof from later ambition and forbids fake shortcuts. The useful escape valve is an execution/reference/spec-template practice that makes staged incompleteness explicit and reviewable.

Promoting staged incompleteness to foundation is risky. If over-written, it could become a license to defer anything; if under-written, it repeats `12` and adds nothing. The better target is execution proof: each staged abstraction must be declared, bounded, and prevented from masquerading as complete acceptance.

#### Recommendation

Target `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` and `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` with a stage-declaration posture: any deliberate abstraction in a future spec or proof must name the current proof claim and the future claim it does not certify.

Target `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` with review-artifact obligations:

- what the staged feature proves now;
- what it deliberately abstracts;
- what it must not fake;
- what future feature it must not block;
- which acceptance evidence prevents the abstraction from becoming a lie;
- which risk-register item remains open until the full feature lands.

Target `docs/3-reference/01_DESIGN_RISK_REGISTER.md` with a relapse risk: staged scope can become permanent false certification if evidence language overstates what was proved.

Target `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` with a future-spec template addition. The specs tier is not an amendment target; this is a later implementation/spec-writing hand-off.

---

## 4. Newly surfaced candidates

No independent newly surfaced candidate is accepted.

### 4.1 Folded watch item — domain-pack assumption manifests

This item is **not from the verdict** and is not a separate route. External bias/documentation practice suggested that future domain packs may need explicit assumption notes: what social categories, cultural norms, credibility heuristics, institutional biases, resource assumptions, and genre facts they introduce. However, this is already captured by Theme 6 and Theme 8.

- **Driver:** Datasheet-style documentation and NIST-style risk management encourage explicit documentation of assumptions, intended scope, limitations, and harmful-bias risks. [EXT-NIST-RMF], [EXT-DATASHEETS]
- **Current coverage:** `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` says the kernel is genre-agnostic and domain packs own flavor; `docs/0-foundation/07...` already owns bias/social position/credibility; `docs/2-execution/08...` already owns authoring/schema/provenance validation.
- **Tier-fit verdict:** Fold into Theme 6 and Theme 8; do not create a tenth amendment topic.
- **Recommendation:** Later authoring/bias sessions should consider whether domain-pack assumption notes belong in execution authoring validation, reference risk memory, or future domain-pack specs. Do not promote to foundation.

---

## 5. Forward-routing appendix

This appendix is the hand-off to later per-tier sessions. It mirrors the earlier lower-tier routing memo: target tier, target doc(s), lesson to encode. The order below is intentional: foundation first, then architecture, execution, reference, and future specs.

### 5.1 Tier 0 — foundation session

| Theme | Target foundation doc(s) | Lesson to encode |
|---|---|---|
| Time, calendar, and social rhythm | Primary: `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`; cross-references as needed in `04`, `05`, `07`, `10`, `12` | Name the temporal authority model: event/replay time, actor-perceived/remembered time, institutional/procedural time, routine/social rhythm, deadlines/lateness, freshness/staleness, and LOD summary intervals. Preserve the truth firewall: authoritative time may validate; only modeled/holder-known temporal facts may plan, decide, appear in embodied views, or drive institution-known procedures. Do not choose tick size or calendar syntax. |

### 5.2 Tier 1 — architecture session

| Theme | Target architecture doc(s) | Lesson to encode |
|---|---|---|
| Play experience and epistemic legibility | `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Encode the embodied play loop, information hierarchy under uncertainty, friction control, lead/notebook usefulness, actor-legible ignorance, and transcript evidence. Preserve actor-filtered view models and no objective markers. |
| Quantity/granularity/fungibility | `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Define object/lot/stock/capacity/ledger representation boundaries and how split/merge/consume/spoil/reserve/share/transfer/hide/discover operations preserve custody, proof, and event ancestry. |
| Bounded affect/emotion | `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Treat affect as source-bearing pressure/salience that can influence candidate generation, scoring, memory durability, belief uptake, concealment/confession, accusation/repair, and routine disruption. Forbid affect as truth or direct dispatch. |
| Learning/adaptation | `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Define learned expectations, route/method reliability, trust, risk, skill, and routine adjustments as provenance-bearing updates from remembered experience or modeled instruction. Distinguish memory from learned expectation. |
| Deterministic performance/fairness budgets | `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`; `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`; `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`; `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Encode deterministic scheduling authority, budget exhaustion semantics, skipped/deferred cognition diagnostics, LOD/time-acceleration declaration, and observability for starvation/fairness. |
| Practical bias/social harm | `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Represent bias as inspectable model inputs/procedure effects with causal diagnostics. Keep kernel genre-neutral; domain packs own culturally specific assumptions. Wrong suspicion must be caused by holder/institution-known evidence and procedure, not hidden truth or designer labels. |
| Authoring/compiler discipline | `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Treat validation/observability seams as architecture-protecting contracts. Content validators and review artifacts enforce doctrine before runtime; they are not optional lint. |

### 5.3 Tier 2 — execution session

| Theme | Target execution doc(s) | Lesson to encode |
|---|---|---|
| Quantity/granularity/fungibility | `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Require content validation for stocks, quantities, ledgers, balances, household pools, records, and service capacities to declare provenance/custody semantics appropriate to their abstraction. |
| Learning/adaptation | `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Add negative/adversarial proof that learning cannot cache hidden truth and positive no-human evidence that repeated modeled experience can affect future routine/method/trust choices. |
| Authoring/compiler discipline | `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Make the compiler-like posture explicit: content, fixtures, seeds, prompt packets, and authored records are executable inputs that must be statically inspectable for constitutional violations. Require clear validation failures and adversarial renamed/nested forbidden fields. |
| Deterministic performance/fairness budgets | `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Require replayable evidence for deterministic scheduling, explicit planner budgets, skipped/deferred cognition, starvation/fairness, and time-acceleration/LOD equivalence or declared divergence. |
| Practical bias/social harm | `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | Add Phase 4 fixture expectations for biased testimony, unequal credibility, underfunding, refusal, delay, misfiling, suppressed/ignored records, and wrong suspicion without hidden culprit truth. |
| Staged incompleteness | `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`; `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`; `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Require every deliberate abstraction/staged feature to declare what it proves, abstracts, must not fake, must not block, and what evidence prevents overclaiming. |

### 5.4 Tier 3 — reference session

| Theme | Target reference doc(s) | Lesson to encode |
|---|---|---|
| Play experience and epistemic legibility | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Track the dual relapse: correct audit machine that is not playable, or “playability” achieved by leaking truth/objectives. |
| Bounded affect/emotion | `docs/3-reference/02_GLOSSARY.md`; `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Define compact terms for affect/salience/motive/need/duty/relationship; track the risk of affect becoming hidden truth or decorative meters. |
| Practical bias/social harm | `docs/3-reference/01_DESIGN_RISK_REGISTER.md`; `docs/3-reference/02_GLOSSARY.md` | Add risk/term discipline for bias, credibility, social position, protected/social category, domain-pack assumption, institutional fallibility, and wrong suspicion. |
| Deterministic performance/fairness budgets | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Track scheduler/LOD performance pressure as a contamination risk: silent starvation, invisible skipped cognition, or human-focus privilege. |
| Staged incompleteness | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Track the risk that deliberately staged abstractions become permanent false certification or acceptance artifacts overstate what was proved. |
| Authoring/compiler discipline | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Track authoring-data smuggling: director logic hidden in schemas, fixtures, prompt packets, display strings, or renamed fields. |

### 5.5 Tier 4 — future scoped specs, not tier amendments

The spec tier is not an amendment target. These are future implementation-spec candidates only after the relevant higher-tier routing lands.

| Theme | Possible future spec home | Lesson to encode |
|---|---|---|
| Play experience and epistemic legibility | New narrow TUI/play-loop fixture spec or amendment to the first-proof TUI fixture scope | Embodied transcripts, lead/notebook examples, information hierarchy snapshots, and usability-without-leak checks. |
| Quantity/granularity/fungibility | Future inventory/economy/household-stock fixture package under the first-proof ontology lineage | Positive and adversarial fixtures for unique items, fungible lots, pooled goods, partial transfers, consumption, spoilage, debt/payment, stale stock records, and custody/proof conflict. |
| Deterministic performance/fairness budgets | Future scheduler/LOD performance-fairness proof spec | Explicit budget traces, deterministic ordering proof, skipped/deferred cognition events, starvation diagnostics, and LOD/time-acceleration evidence. |
| Staged incompleteness | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` or successor template | Add staged-abstraction declarations to acceptance artifacts: current proof, deliberate abstraction, anti-fake boundary, future-unblocked requirement, and evidence status. |
| Practical bias/social harm | Future domain-pack/institution Phase 4 fixture spec | Bias-input assumption notes, wrong-suspicion causal diagnostics, institution-known provenance, and non-leaking debug audits. |

---

## 6. Open questions

These are owner decisions that cannot be settled from the exact-commit docs and should be carried into later amendment/spec sessions.

1. **Temporal doctrine placement shape.** This report recommends `docs/0-foundation/03...` as primary home, but the maintainer may decide whether temporal authority deserves a short new section there or a compact cross-doc mini-amendment package spanning `03`, `04`, `05`, `07`, `10`, and `12`.
2. **Minimal first-playable calendar.** The docs do not settle the first-slice calendar vocabulary: hours, day parts, weekdays, seasons, office windows, meal windows, and “last night” style actor language. The foundation should not pick full syntax, but architecture must choose enough to avoid ad hoc ticks.
3. **Money and food representation in the first proof.** The docs do not decide whether money/food start as unique physical items, fungible lots, ledgered balances, service capacities, or a hybrid. Architecture/spec sessions must choose the smallest representation that preserves custody/proof and missing-property play.
4. **Affect scope for early implementation.** The docs can support bounded affect, but the owner must decide whether first proof implements only role/relationship/need pressure or also named emotions such as fear, shame, guilt, grief, anger, and affection.
5. **Learning depth before Phase 4.** The owner must decide how much learning/adaptation belongs before institutions: route reliability, food-source reliability, search-method failure, routine buffers, interpersonal trust, or only diagnostic-ready placeholders.
6. **Domain-pack bias assumption ownership.** The foundation says the kernel is genre-neutral and domain packs own flavor. Future sessions must decide the documentation vehicle for culturally specific norms, biases, categories, and credibility rules.
7. **Budget/fairness quantitative targets.** The docs require deterministic replay and honest scale, but they do not set initial planner budgets, maximum actor counts, no-human throughput targets, or starvation thresholds. Those should be execution/spec values, not foundation doctrine.
8. **Staged-abstraction evidence status language.** Acceptance artifacts need a vocabulary that distinguishes passed, pending, sampled, observation-only, non-certifying, and deliberately abstracted. The prior campaign routed related artifact honesty below foundation; this report preserves that boundary.

---

## 7. References

### 7.1 Exact-commit repository sources

All repository sources were fetched from the exact target commit via raw URLs listed in the evidence ledger in §2.1. The uploaded manifest was used only as path inventory. No branch-name fetch, clone, code search, repository metadata lookup, default-branch lookup, or connector namespace trust was used.

### 7.2 External sources cited

[EXT-MDA] Robin Hunicke, Marc LeBlanc, and Robert Zubek, “MDA: A Formal Approach to Game Design and Game Research,” AAAI Workshop on Challenges in Game AI, 2004. https://users.cs.northwestern.edu/~hunicke/MDA.pdf

[EXT-TOTT] James Ryan and Michael Mateas, “Simulating Character Knowledge Phenomena in Talk of the Town,” *Game AI Pro 3*. https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf

[EXT-PROMWEEK] Josh McCoy et al., “Prom Week: Social Physics as Gameplay,” ACM, 2012. https://dl.acm.org/doi/10.1145/2159365.2159425

[EXT-CIF] Josh McCoy et al., “Comme il Faut: A System for Authoring Playable Social Models,” AIIDE, 2011. https://cdn.aaai.org/ojs/12454/12454-52-15982-1-2-20201228.pdf

[EXT-NEIGHBORLY] Shi Johnson-Bey, Mark J. Nelson, and Michael Mateas, “Neighborly: A Sandbox for Simulation-based Emergent Narrative,” IEEE CoG, 2022. https://ieee-cog.org/2022/assets/papers/paper_122.pdf

[EXT-DEMO] Glenn A. Silver et al., “DeMO: An Ontology for Discrete-event Modeling and Simulation,” *Journal of Simulation*, 2011. https://pmc.ncbi.nlm.nih.gov/articles/PMC3423901/

[EXT-CT-ABM] Tim Köster, “A Fast Embedded Language for Continuous-Time Agent-Based Models,” *Journal of Artificial Societies and Social Simulation*, 2024. https://www.jasss.org/27/1/10.html

[EXT-ABM-SCHED] Alessio Antelmi et al., “Reliable and Efficient Agent-Based Modeling and Simulation,” *Journal of Artificial Societies and Social Simulation*, 2024. https://www.jasss.org/27/2/4.html

[EXT-OCC] Bas R. Steunebrink, Mehdi Dastani, and John-Jules C. Meyer, “The OCC Model Revisited,” 2009. https://people.idsia.ch/~steunebrink/Publications/KI09_OCC_revisited.pdf

[EXT-EMOTION-SURVEY] Mélanie Bourgais, Patrick Taillandier, and Laurent Vercouter, “Emotion Modeling in Social Simulation: A Survey,” *Journal of Artificial Societies and Social Simulation*, 2018. https://www.jasss.org/21/2/5.html

[EXT-PROV-DM] W3C, “PROV-DM: The PROV Data Model,” W3C Recommendation, 2013. https://www.w3.org/TR/prov-dm/

[EXT-PROV-SEM] W3C, “Semantics of the PROV Data Model,” 2013. https://www.w3.org/TR/prov-sem/

[EXT-JSON-SCHEMA] JSON Schema, “Explore the docs.” https://json-schema.org/docs

[EXT-OPA] Open Policy Agent documentation. https://openpolicyagent.org/docs

[EXT-DST] Antithesis, “Deterministic simulation testing — how it works and when to use it.” https://antithesis.com/docs/resources/deterministic_simulation_testing/

[EXT-LOCKSTEP] Glenn Fiedler, “Deterministic Lockstep,” *Gaffer on Games*, 2014. https://gafferongames.com/post/deterministic_lockstep/

[EXT-NIST-RMF] NIST, “AI Risk Management Framework.” https://www.nist.gov/itl/ai-risk-management-framework

[EXT-DATASHEETS] Timnit Gebru et al., “Datasheets for Datasets,” arXiv:1803.09010. https://arxiv.org/abs/1803.09010

[EXT-LEAN-MVP] Lean Startup Co., “What Is an MVP? Eric Ries Explains.” https://leanstartup.co/resources/articles/what-is-an-mvp/

[EXT-TECH-DEBT] Martin Fowler, “Technical Debt,” 2019. https://martinfowler.com/bliki/TechnicalDebt.html

[EXT-ADR] Architectural Decision Records. https://adr.github.io/

[EXT-FITNESS] AWS Architecture Blog, “Using Cloud Fitness Functions to Drive Evolutionary Architecture,” 2021. https://aws.amazon.com/blogs/architecture/using-cloud-fitness-functions-to-drive-evolutionary-architecture/

---

## 8. Self-check

- All nine verdict themes are adjudicated with explicit verdicts and current-coverage assessments grounded in named live docs.
- Newly surfaced candidates are separated; no independent tenth candidate is accepted.
- The settled seven prior-campaign themes are not re-litigated; overlaps are deferred to their settled disposition.
- No paste-ready doctrine text is provided and no new invariant IDs, gate codes, or glossary identifiers are invented.
- The foundation receives only one promoted item: temporal authority doctrine. Other accepted items are routed to the highest tier their nature requires and no higher.
- External sources that shaped decisions are cited in §7.2.
- The forward-routing appendix gives ordered, specific hand-offs by target tier.
- The deliverable is one new report, not a numbered spec.
- Exact-commit discipline is documented in the evidence ledger.
