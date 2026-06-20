# Architecture-tier alignment research report — `joeloverbeck/tracewake` at `ea6a05b`

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `ea6a05bf5822307cfcbd39804bbb015cdb13215d`  
**Deliverable path:** `reports/architecture-tier-alignment-research-report.md`  
**Deliverable status:** analysis / recommendation report for `docs/1-architecture/*`; not a numbered spec; not ratified architecture prose; not execution certification; not a code audit.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 0. Mandatory exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: ea6a05bf5822307cfcbd39804bbb015cdb13215d
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.open on exact raw.githubusercontent.com commit URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/foundations-completeness-determination-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/foundation-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/4-specs/SPEC_LEDGER.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Exact-commit verification posture.** Each repository fetch URL above was mechanically constructed as `https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/<manifest path>` and was checked before use for owner, repository, commit, and manifest path. No branch name, clone, code search, default-branch lookup, repository metadata, or repository-scoped connector argument was used. The uploaded manifest was used only to validate path presence before exact URL construction.

**Stale-provenance quarantine.** Some fetched historical reports/specs name older commits or ticket-closeout commits inside their own provenance text, including the prior architecture report and the foundation temporal amendment chain. Those strings are treated only as internal document history. They are not fetch targets, freshness claims, or repository identity evidence. The analysis below uses only the exact fetched contents from `ea6a05bf5822307cfcbd39804bbb015cdb13215d`.

**Source aliases used below.**

- [TW-README] `docs/README.md`
- [TW-F-00] `docs/0-foundation/00_FOUNDATION_INDEX.md`
- [TW-F-01] `docs/0-foundation/01_PROJECT_CHARTER.md`
- [TW-F-02] `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- [TW-F-03] `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- [TW-F-04] `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- [TW-F-05] `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- [TW-F-06] `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- [TW-F-07] `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- [TW-F-08] `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- [TW-F-09] `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- [TW-F-10] `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- [TW-F-11] `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- [TW-F-12] `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- [TW-F-13] `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- [TW-F-14] `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- [TW-A-00] through [TW-A-14] `docs/1-architecture/00...14`
- [TW-COMP] `reports/foundations-completeness-determination-research-report.md`
- [TW-VERDICT] `reports/verdict-on-foundations.md`
- [TW-FOUND-ALIGN] `reports/foundation-tier-alignment-research-report.md`
- [TW-SPEC-0031] `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md`
- [TW-SPEC-0026] `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md`
- [TW-SPEC-0027] `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`
- [TW-ARCH-PREV] `archive/reports/architecture-tier-alignment-research-report.md`
- [TW-E-00] through [TW-E-13] `docs/2-execution/00...13`
- [TW-R-00] `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- [TW-R-01] `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- [TW-R-02] `docs/3-reference/02_GLOSSARY.md`
- [TW-SPEC-LEDGER] `docs/4-specs/SPEC_LEDGER.md`

---

## 1. Disposition table

| Finding | Driver | Target architecture doc(s) | Verdict | One-line basis |
|---|---|---|---|---|
| T1 | Temporal authority cascade | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Partial coverage / consolidation | A00 is the architecture conformance map, but the new `INV-112` temporal authority row and target-doc map are not yet present at architecture tier. |
| T2 | Temporal authority cascade | `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Belongs in architecture | A02 already owns event/replay time, envelopes, projections, and deterministic rebuild; it now needs the explicit boundary between authoritative event/replay time and holder-known temporal claims. |
| T3 | Temporal authority cascade | `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Belongs in architecture | A03/A06 already own holder-known context, provenance sufficiency, and freshness; temporal claims must be first-class claim/provenance/freshness inputs rather than raw clock labels. |
| T4 | Temporal authority cascade | `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Partial coverage | A04 already names scheduler limits, validator authority, reservations, and duration accounting; it needs the temporal firewall: scheduler/replay time can trigger and validate but may not plan. |
| T5 | Temporal authority cascade | `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Partial coverage | A05 already covers needs, routines, intentions, and actor-known transactions; routine/social rhythm premises need explicit holder-known temporal-source discipline. |
| T6 | Temporal authority cascade | `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Partial coverage | A08 already makes institutions fallible holder-known processes; institution/procedural time needs a single architecture home for office windows, due/lateness, queue aging, and lifecycle status as record/procedure-backed states. |
| T7 | Temporal authority cascade | `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Belongs in architecture | A10 already owns embodied/debug separation and captured view models; time controls and temporal rendering need explicit actor-visible/debug-only separation. |
| T8 | Temporal authority cascade | `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`; `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Already-owned-close / minor cross-reference | A07/A11 already require speaker/listener context and source-bound leads, but temporal expressions and stale/late lead labels should cross-reference the A03/A06 temporal claim contract. |
| T9 | Temporal authority cascade | `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | Partial coverage | A12 already preserves LOD ancestry; it needs a temporal-summary seam: source interval/cadence, temporal ancestry, information ancestry, and declared acceleration/fidelity limits. |
| T10 | Temporal authority cascade | `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Belongs in architecture | A13 already requires typed observability and behavior witnesses; temporal-firewall evidence, temporal leakage diagnostics, scheduler-budget diagnostics, and LOD temporal ancestry must become artifact-family expectations. |
| R1 | Completeness §5.2 routed theme: play experience / epistemic legibility | `10`; `11` | Partial coverage | Embodied view models, why-not output, story-sifting, and leads are present; architecture should consolidate play-legibility loops without creating quest/director mechanics. |
| R2 | Completeness §5.2 routed theme: quantity / material granularity / fungibility | `09` | Partial coverage | A09 owns ordinary objects, property, custody, food, wages, and economy placeholders; it needs representation seams for unique items, lots/stocks/capacities, ledgers, and split/merge/consume/transfer lineage. |
| R3 | Completeness §5.2 routed theme: bounded affect / emotion | `05`; `06` | Already-owned-close / consolidation | A05/A06 already support pressures, motives, relationship memory, and belief uptake; name affect as source-bearing salience/pressure, not truth or dispatch. |
| R4 | Completeness §5.2 routed theme: learning / adaptation | `05`; `06` | Partial coverage | Memory, stale belief, routine failure, and replanning are present; learned expectations, route/method reliability, trust, risk, and skill must be provenance-bearing derived state distinct from raw memory. |
| R5 | Completeness §5.2 routed theme: deterministic performance / fairness budgets | `04`; `05`; `12`; `13` | Partial coverage | Deterministic ordering, scheduler limits, LOD, and observability are present; budget exhaustion, skipped/deferred cognition, starvation/fairness diagnostics, and time-acceleration declarations need architecture-level seams. |
| R6 | Completeness §5.2 routed theme: practical bias / social harm | `08` | Already-owned-close / consolidation | A08 already treats institutions as fallible and bias-bearing; add inspectable model-input/procedure-effect discipline and domain-pack assumption ownership without importing moral omniscience. |
| R7 | Completeness §5.2 routed theme: authoring / compiler discipline | `13`; secondary `01`/`09`/`14` cross-references | Partial coverage | Content validation and anti-contamination artifacts exist; A13 should explicitly treat validators/static checks/review artifacts as architecture-protecting compiler seams, while execution owns rule mechanics. |

No architecture file at this commit shows a conflict that requires a foundation edit. The report finds architecture gaps and consolidations, not constitutional tension.

---

## 2. Method and provenance ledger

### 2.1 Authority order and altitude rule

The documentation map says the lower tiers cannot override earlier tiers: foundation owns product identity and invariants; architecture owns subsystem boundaries, data flows, authority ownership, diagnostics, and acceptance implications; execution owns gate procedure and proof mechanics; reference owns compact lookup and risk/terminology memory; specs are subordinate implementation proposals [TW-README]. Therefore this report only recommends architecture-level doctrine substance and target homes. It does **not** author final ratified wording, choose identifiers, choose concrete tick/calendar/duration vocabulary, define fixture names, set thresholds, or alter execution gates.

### 2.2 Files read and role

The fetched set includes the full foundation tier, all architecture docs, the primary completeness and temporal seed reports, the prior settled architecture cascade and its archived spec, all execution docs needed for forward routing, the reference risk/glossary docs, and the spec ledger. The current analysis is a documentation-tier conformance and routing pass. No Rust crate source was fetched or used as implementation evidence.

### 2.3 Settled context not re-commissioned

The `0026`/`0027` architecture work is settled. The spec ledger records that `0027` already translated `INV-111` and truth-firewall hardening into A13 observer-only emergence evidence and typed observability, A11 story-sifting evidence scoping, A03/A06 provenance sufficiency and freshness, A10 embodied capture, and the A04/A05/A09 single-charge accounting seam [TW-SPEC-LEDGER]. This report therefore does not reopen provenance sufficiency, memory freshness, believed-access snapshots, single-charge accounting, or observer-only emergence evidence. It builds on them and adds only the new temporal cascade plus the seven completeness-route themes.

### 2.4 External research used

External research shaped tier-fit judgment but did not become Tracewake doctrine. Discrete-event simulation literature reinforced the need to separate a simulation clock/event order from modeled agents' knowledge of time, and to preserve deterministic ordering rather than let “current time” leak everywhere [EXT-DES], [EXT-CT-ABM]. The ODD protocol reinforced that architecture should document temporal/spatial resolution and process scheduling as model description seams while leaving lower-level submodel details to later tiers [EXT-ODD]. W3C PROV-DM reinforced provenance as source ancestry through entities, activities, and responsible agents, which supports the report's recommendation that quantity split/merge and temporal claims carry event/procedure lineage [EXT-PROV-DM]. MDA reinforced the distinction between mechanics/dynamics/player experience, supporting play legibility without authoring drama [EXT-MDA]. Social-simulation emotion surveys support affect as an explanatory decision influence but warn that affect models increase parameters and validation burden, supporting a narrow architecture seam rather than a full affect mechanic now [EXT-EMOTION]. JSON Schema and OPA informed the authoring/compiler-discipline finding as validation/policy separation over structured data, not runtime convenience lint [EXT-JSON-SCHEMA], [EXT-OPA]. NIST AI RMF, NIST bias guidance, and Datasheets for Datasets reinforced that bias/assumption documentation and socio-technical context should be inspectable, not hidden in algorithmic labels or claimed neutral by default [EXT-NIST-AI-RMF], [EXT-NIST-BIAS], [EXT-DATASHEETS]. Deterministic simulation testing and lockstep sources reinforced that replayable determinism requires controlled nondeterminism, reproducible inputs, and observability of timing-related divergence [EXT-DST], [EXT-LOCKSTEP]. Lean MVP material was used only to keep first-playable recommendations small and learning-oriented, not to import startup doctrine into Tracewake [EXT-LEAN].

---

## 3. Foundation delta architecture must now translate

### 3.1 Block T — temporal authority doctrine

`INV-112` now makes explicit that authoritative simulation time may validate event order, intervals, action legality, scheduled consequences, replay, and causal explanation, while cognition, routine selection, institutional procedure, embodied view models, speech interpretation, leads, and LOD promotion may use temporal facts only when those facts reached the relevant holder through modeled channels [TW-F-02]. Foundation `03` gives the conceptual model: authoritative event/replay time; holder-known temporal claims; institution-known procedural time; routine/social rhythm; freshness/staleness authority; and LOD/regional temporal summaries. The temporal firewall is the truth firewall applied to time: the scheduler/replay clock may order and validate, but holder-known or institution-known time must plan [TW-F-03].

Architecture must translate that into subsystem contracts:

- Which subsystem owns authoritative event/replay time and how it is exposed to validation, scheduling, replay, projections, and diagnostics without becoming a cognition oracle.
- What shape a holder-known temporal claim must have at architecture altitude: holder, proposition, event/acquisition/record/verification temporal slots as relevant, provenance, freshness/staleness/contradiction status, and source ancestry.
- Which institutional surface owns procedural-time states such as office windows, filing windows, due states, queue aging, notice lifecycle, payment periods, case delay, and sanctions.
- How routine/social-rhythm planners consume temporal premises without reading a true global schedule.
- How embodied TUI, leads, speech, notebooks, and LOD promotion display or reason about temporal facts only when source-backed.
- What temporal diagnostics and review artifacts execution will need to prove the firewall.

Architecture must not choose tick size, day-part vocabulary, calendar/date syntax, duration units, stale-after thresholds, scheduler queue/data structures, UI clock formats, first-playable temporal vocabulary, fairness algorithms, or fixture/gate names. Spec `0031` explicitly deferred those lower-tier mechanism choices and the architecture/execution/reference cascade [TW-SPEC-0031].

### 3.2 Block R — seven routed non-temporal themes

The completeness determination routed seven non-temporal themes to architecture, each with named architecture homes [TW-COMP]. The routing does not mean every theme is a blank gap. Current architecture already covers much of affect, learning, bias, holder-known view models, institution fallibility, and authoring validation. The task here is validate-before-gap: record existing coverage first, then recommend only the compact architecture seam that is missing or fragmented.

The seven routed themes become architecture work only where they require subsystem contracts:

1. **Play experience and epistemic legibility** — architecture should own the information hierarchy and play-loop surface, not drama objectives.
2. **Quantity, material granularity, and fungibility** — architecture should define object/lot/stock/capacity/ledger boundaries and lineage expectations, not concrete inventory schemas.
3. **Bounded affect/emotion** — architecture should treat affect as source-bearing salience/pressure that can influence selection and memory, not truth or direct dispatch.
4. **Learning/adaptation** — architecture should separate remembered facts from learned expectations/reliability/trust/skill, all with provenance.
5. **Deterministic performance and fairness budgets** — architecture should name budget-exhaustion and starvation/fairness observability seams, not algorithms or numeric thresholds.
6. **Practical bias/social harm** — architecture should make modeled assumptions and institutional/procedure effects inspectable, not grant a morality oracle.
7. **Authoring/compiler discipline** — architecture should specify validation and review artifacts as protective compiler seams, leaving implementation rules to execution/specs.

---

## 4. Architecture sweep coverage register

| Architecture doc | Current alignment verdict | Coverage note and recommended posture |
|---|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Partial coverage | Strong conformance map for the prior hardening cascade. Needs a compact temporal-authority entry and pointers to A02/A03/A04/A05/A08/A10/A11/A12/A13. Do not move doctrine into A00 alone. |
| `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | Already-owned-close | Workspace and side-effect boundaries already support deterministic simulation ownership. No primary Block T/R amendment needed; optional cross-reference if authoring/compiler validation grows out of A13. |
| `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Partial coverage | Owns event envelope, sim tick/window, replay, projections, randomness, and snapshots. Needs explicit authoritative event/replay time contract and temporal ancestry expectations. |
| `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Partial coverage | Strong holder-known/provenance contract from prior cascade. Needs temporal-claim and temporal-provenance specialization: source-backed time, freshness/staleness, and temporal unknowns. |
| `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Partial coverage | Scheduler and validation limits are strong. Needs temporal firewall language for due effects, decision windows, duration completion, and trigger-vs-planning separation, plus budget-exhaustion semantics. |
| `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | Partial coverage | Routines, needs, intentions, and single-charge accounting are present. Needs routine/social-rhythm temporal premises, affect/salience consolidation, learning/adaptation derived-state seam, and planner-budget observability. |
| `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Partial coverage | Claims/beliefs/memory/freshness are strong. Needs temporal-claim/freshness specializations, affect-memory effects, and learned expectations distinct from remembered facts. |
| `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | Already-owned-close / minor cross-reference | Speaker/listener context and structured speech act boundaries are strong. Add only a cross-reference that temporal expressions in speech are claims interpreted through A03/A06 holder-known temporal context. |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Partial coverage | Institution-known procedure, records, fallibility, and bias already exist. Needs procedural-time authority and practical-bias consolidation as inspectable inputs/procedure effects. |
| `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Partial coverage | Property/custody/food/economy placeholders are present. Needs quantity/granularity/fungibility seam and lineage expectations for split/merge/consume/spoil/reserve/share/transfer/hide/discover. |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Partial coverage | Embodied/debug split and observation-time capture are strong. Needs temporal rendering, time-control authority, missed-event summaries, and play-legibility loops without hidden truth. |
| `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | Partial coverage | Source-bound leads and story-sifting quarantine are strong. Needs temporal/staleness/late label discipline and player-legible lead usefulness without questification. |
| `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | Partial coverage | LOD ancestry and summary events are present. Needs explicit temporal+information ancestry, cadence/interval summaries, time-acceleration declarations, and fairness/starvation diagnostics. |
| `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Partial coverage | Strong typed observability, anti-vacuity, and artifact families. Needs temporal-firewall evidence, budget/fairness evidence, compiler/static validation as architecture-protecting seams, and route-forward hooks. |
| `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` | Already-owned-close | Research-source posture already rejects importing generic frameworks as product identity. Add source notes only if the repository amendment process wants to record temporal/bias/authoring research anchors; no doctrine gap. |

---

## 5. Per-finding analysis and recommendations

### Finding T1 — Architecture index and conformance map for temporal authority

**Foundation driver.** `INV-112` is now constitutional, and foundation `03` is the primary temporal-authority home [TW-F-02], [TW-F-03]. A00 is the architecture index/conformance map, so future subsystem authors should find the architecture translation there.

**Current architecture coverage.** A00 is dense with prior hardening rows, including truth-firewall and single-charge references from the 0027 campaign. That makes A00 useful, but also risky: new doctrine can become discoverable only as a conformance row rather than a primary subsystem contract.

**Tier-fit verdict.** Partial coverage / consolidation. A00 should point to the real owners; it should not become the only home of temporal authority.

**Recommendation — substance and home.** Add a compact A00 conformance entry or subsection that records the new temporal-authority architecture cascade and points to owners: A02 for authoritative event/replay time; A03/A06 for holder-known temporal claims; A04 for scheduler/validation temporal boundaries; A05 for routine/social rhythm; A08 for institutional/procedural time; A10/A11/A07 for temporal rendering, leads, and speech; A12 for LOD temporal ancestry; A13 for observability. The entry should say architecture translates `INV-112`; it should not restate foundation doctrine at length or define tick/calendar/duration values.

**Route-forward.** Execution gate additions and fixture names route to execution `04`/`05`/`07`/`09`/`10`/`12`; terminology routes to reference `02` only after architecture chooses stable descriptive terms.

### Finding T2 — Authoritative event/replay time contract

**Foundation driver.** Foundation `03` says simulation time is authoritative for event order, replay, validation, intervals, scheduled consequences, and causal explanation, but not automatically authoritative for cognition [TW-F-03].

**Current architecture coverage.** A02 already owns the event envelope, replay rebuild, projections, snapshots, randomness, and deterministic metrics. It references `sim_tick`/`tick_window` and requires replayable event/projection ancestry. It does not yet name temporal authority as a firewall problem.

**Tier-fit verdict.** Belongs in architecture. This is exactly the subsystem-contract altitude: which component may use the authoritative clock for ordering/validation and how that authority is quarantined from cognition surfaces.

**Recommendation — substance and home.** In A02, near the event envelope and replay sections, add a temporal-authority contract that distinguishes:

- event/replay time as the ordered substrate for validation, scheduling due effects, duration accounting, replay, projection rebuild, and causal explanation;
- temporal facts exported to holders only through events/projections that carry modeled acquisition or record/procedure ancestry;
- projection/snapshot/compaction obligations to preserve temporal ancestry rather than replace it with “current time” labels;
- replay diagnostics for temporal divergence: wrong ordering, missing duration terminals, due-effect drift, unrecorded wall-clock input, and unsupported temporal migration.

The doc may name the seam and data-flow obligations. It must not choose calendar/date syntax, tick size, duration units, priority-queue structure, or stable field names.

**External rationale.** Next-event simulation literature separates the simulation clock and event-order mechanism from the modeled state changes, which supports A02 owning the authoritative ordering substrate while A03/A06 own known temporal claims [EXT-DES]. Continuous-time ABM scheduling sources likewise reinforce that event selection/time stamps are simulation mechanics, not agent knowledge [EXT-CT-ABM].

### Finding T3 — Holder-known temporal claims, provenance, and freshness

**Foundation driver.** `INV-112` says deadlines, lateness, staleness, expected-by-now, yesterday, and office-closed states are claims, procedure states, or holder/institution-known interpretations with provenance, not free truth labels [TW-F-02]. Foundation `04` also treats temporal expressions, acquisition time, event time, record time, freshness, and staleness as claim/qualifier issues rather than silent clock updates [TW-F-04].

**Current architecture coverage.** A03 already defines sealed holder-known context and provenance sufficiency. A06 already owns claims, observations, beliefs, memory traces, records, freshness, and display-string limitations. The prior 0027 cascade deliberately made A03/A06 stronger and should not be reopened [TW-SPEC-0027]. The gap is that temporal claim shape is not named as a first-class specialization.

**Tier-fit verdict.** Belongs in architecture as a specialization of existing A03/A06 contracts.

**Recommendation — substance and home.** Add to A03 a temporal-claim subsection that says any temporal input used by cognition, procedure, affordance selection, speech interpretation, lead interpretation, or LOD promotion must be addressable inside the holder-known context with fact-kind-appropriate provenance. Add to A06 the parallel epistemic data-flow rule: temporal claims should preserve distinct slots where relevant, such as event time asserted or inferred, acquisition time, last verification, record/procedure time, valid/due window, stale risk, contradiction status, and source lineage.

The recommendation should stay abstract. Do not define a struct, field names, stale-after numbers, vocabulary such as “morning/evening,” or exact calendar syntax. The architecture substance is that temporal status is a source-backed claim/procedure state, not a display label or true clock read.

**External rationale.** PROV-DM's entity/activity/agent model and derivation relations support treating temporal claims and later summaries as provenance-bearing lineage rather than unstructured labels [EXT-PROV-DM].

### Finding T4 — Scheduler trigger versus cognition-time boundary

**Foundation driver.** `INV-103` says the scheduler may choose actor/time windows and apply due scheduled effects but may not construct action proposals from raw state; `INV-112` specializes that rule for time [TW-F-02]. Foundation `14` states that the scheduler may advance deterministic time or invoke transactions but cannot become a hidden-planner authority [TW-F-14].

**Current architecture coverage.** A04 is strong: it already states scheduler limits, direct-dispatch prohibitions, validation boundaries, reservations/durations, and single-charge accounting. Execution `05` already operationalizes no direct dispatch and scheduler limits. The architecture gap is the lack of an explicit temporal-firewall formulation.

**Tier-fit verdict.** Partial coverage. A04 already owns the seam; it needs a temporal specialization.

**Recommendation — substance and home.** In A04, near scheduler limits and validation boundary, state that scheduler/replay time may:

- order decision opportunities and process windows;
- detect due effects and duration terminals;
- invoke holder-known transaction construction;
- validate whether a proposed action is temporally legal or whether a due/procedural consequence applies;
- emit typed temporal diagnostics.

It may not:

- turn “it is work time,” “it is meal time,” “deadline passed,” “office is closed,” or “actor is late” into a selected action, route, target, institutional conclusion, or actor-visible reason unless that temporal premise is in the relevant holder-known or institution-known context;
- repair plans using true lateness or hidden schedule truth;
- leak exact future/due timing through actor-visible feedback unless a modeled channel exposes it.

Architecture should also say budget exhaustion is a typed scheduling/decision outcome: if a scheduler cannot process all due cognition, it must defer, skip, summarize, or diagnose through a deterministic policy that execution later defines. Architecture should not define the queue or algorithm.

### Finding T5 — Routine/social rhythm temporal premises

**Foundation driver.** Foundation `05` says routines and schedules consume actor-known or holder-known temporal premises; a worker works because they believe or have source-backed reason to treat it as work time, not because a true global schedule row is read [TW-F-05].

**Current architecture coverage.** A05 already defines actor decision transactions, needs, intentions, routine/HTN method selection, and candidate generation from holder-known context. It already protects against routine labels dispatching actions directly. What is missing is the explicit temporal premise list and source discipline for routine/social rhythm.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A05, add a routine/social-rhythm subsection stating that work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments are defeasible temporal premises inside actor-known context. Candidate generation and method selection may use those premises only if they come through assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference. Routine templates may organize method families and expected temporal rhythms, but a template's presence is not itself an information channel.

This also supports R3/R4: affect and learning can tune salience and method preference only after the same holder-known premise is available; they cannot create a hidden temporal fact.

**Route-forward.** Execution may later define minimal first-playable schedule vocabulary, concrete routine windows, interruption policy, and tests. Architecture should not decide those values.

### Finding T6 — Institution-known procedural time

**Foundation driver.** Foundation `07` now applies temporal authority to institutions: office hours, filing windows, due/lateness states, queue aging, notice lifecycle, payment periods, case delay, and sanctions are institution-known/procedure-backed states, not truth labels [TW-F-07].

**Current architecture coverage.** A08 already owns roles, powers, records, procedures, institution-known context, fallibility, bias/error/corruption, reports, sanctions, and replayable records. Execution `11` already says institutions must mirror actor-known discipline and records preserve time/status [TW-E-11]. The gap is a named architecture home for procedural time as a special authority seam.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A08, add an institutional/procedural-time contract:

- A procedure may maintain due, late, open, closed, pending, delayed, filed, expired, paid, sanctioned, or queue-aged states only as event/record/procedure-backed institution-known state.
- Procedure time is authoritative for the procedure's own lifecycle only through recorded rules and events; it does not grant the institution hidden truth about the underlying world.
- Procedure outputs that become actor knowledge, notices, reports, sanctions, payments, refusals, or records must preserve provenance and access context.
- Bias, misfiling, delay, underfunding, and stale records remain modeled procedure effects, not hidden moral labels.

Do not choose office-hour vocabulary, legal deadlines, payment periods, or concrete status enums.

### Finding T7 — Embodied temporal rendering and time controls

**Foundation driver.** Foundation `08` says time controls may advance authoritative event/replay time, but embodied views render temporal facts only when the possessed actor knows, remembers, infers, reads, hears, or perceives them; debug may show omniscient time separately [TW-F-08].

**Current architecture coverage.** A10 already owns embodied vs debug separation, view-model generation from holder-known context, observation-time capture, semantic actions, why-not split, transcripts, and possession parity. Execution `07` currently keeps player-facing time acceleration staged and debug-gated until a later feature exists [TW-E-07].

**Tier-fit verdict.** Belongs in architecture.

**Recommendation — substance and home.** In A10, add a temporal rendering/time-control contract:

- World-advancing controls are commands that advance authoritative event/replay time through the ordinary pipeline; they are not actor cognition.
- Actor-facing time displays, “missed event” summaries, time-to-work cues, waiting/sleeping summaries, “office closed” messages, and lateness/expectation labels must be derived from the possessed actor's holder-known context or from modeled observations/records/public cues.
- Debug/operator panels may show exact event/replay time, due queues, and hidden temporal comparisons, but those fields are structurally non-diegetic and cannot feed embodied affordances or actor-visible reasons.
- Rejection/why-not output must preserve the actor-visible/debug split: true temporal invalidity may validate rejection, but actor-visible explanation is limited to what the actor can know or observe.

Do not choose UI clock format, time-acceleration speed, wait command vocabulary, or summary thresholds.

### Finding T8 — Temporal speech expressions, leads, and story-sifting labels

**Foundation driver.** `INV-112` explicitly names speech interpretation and leads among the surfaces that may use temporal facts only through modeled channels [TW-F-02]. Foundation `11` keeps speech acts structured and actor-filtered; foundation `12` keeps first-playable acceptance tied to actor-legible, source-bound surfaces [TW-F-11], [TW-F-12].

**Current architecture coverage.** A07 already requires speech acts to carry speaker/listener context and forbids LLM/prose authority. A11 already makes leads source-bound, stale-risk-bearing, and story-sifting observer-only. This is close; the architecture gap is mostly cross-document clarity.

**Tier-fit verdict.** Already-owned-close / minor cross-reference.

**Recommendation — substance and home.** Add short cross-references rather than new doctrine:

- In A07, temporal utterances such as “yesterday,” “late,” “due,” “before the market closed,” or “after the bell” should be treated as structured claims whose interpretation depends on speaker and listener holder-known temporal context, provenance, and ambiguity.
- In A11, lead/notebook labels such as stale, urgent, overdue, recently seen, old report, or no longer useful should be source-bound projections over holder-known temporal claims and records. Story-sifting may compute observer-only temporal summaries for review, but may not create actor-known urgency or quest priority.

This preserves the already-ratified A11 correction that story-sifting may create observer-only review evidence but not diegetic evidence.

### Finding T9 — LOD/regional temporal summaries and time-acceleration declarations

**Foundation driver.** Foundation `10` says LOD/regional summaries must preserve temporal ancestry and information ancestry; `INV-110` and `INV-112` jointly forbid LOD promotion from filling in truth or temporal facts without modeled source [TW-F-10], [TW-F-02].

**Current architecture coverage.** A12 already says LOD is replay-visible ontology, summary events carry inputs/outputs/fidelity limits, promotion/demotion preserve ancestry, and human focus is not privilege. It needs temporal authority specialization.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A12, add an LOD temporal-summary contract:

- Each regional/LOD summary that compresses time should declare source interval, cadence, affected processes, temporal resolution/fidelity limits, and whether it includes scheduled consequences, absence of events, delayed records, or stale claims.
- The summary must preserve information ancestry separately from event-time ancestry: what the aggregate could know, what it merely summarized as truth for replay, and what later promoted actors/institutions may know.
- Time acceleration is a declared simulation mode/projection policy with replay/debug visibility, not a silent performance optimization.
- Promotion may create holder-known temporal claims only through modeled summary events or records that are valid information channels for the promoted holder.

Do not choose LOD equivalence thresholds, promotion algorithms, regional cadence values, or performance budgets.

**External rationale.** ODD's emphasis on temporal/spatial resolution and process scheduling supports making cadence/fidelity explicit in model-facing architecture documentation [EXT-ODD].

### Finding T10 — Temporal-firewall observability and acceptance artifacts

**Foundation driver.** `INV-105`, `INV-111`, and `INV-112` require typed diagnostics, observer-only evidence, and temporal authority separation [TW-F-02]. Foundation `12` asks first-playable proof to include temporal firewall proof without choosing lower-tier values [TW-F-12].

**Current architecture coverage.** A13 is strong after the prior 0027 cascade: it owns artifact families, observer-only emergence evidence, and typed behavior witnesses. Execution `10` already defines a rich diagnostics and evidence-status posture. A13 does not yet name temporal evidence and budget/fairness evidence as architecture-level observability families.

**Tier-fit verdict.** Belongs in architecture.

**Recommendation — substance and home.** In A13, add temporal evidence to the validation/observability contract:

- decision traces identify temporal premises used by candidate generation/method selection and their provenance;
- validation reports identify temporal truth checks separately from actor-visible temporal reasons;
- scheduler diagnostics record due effects, deferred/skipped cognition, budget exhaustion, starvation/fairness symptoms, and layer attribution;
- TUI/view-model reports prove temporal display labels came from holder-known sources, not raw clock/debug truth;
- LOD/replay artifacts preserve temporal and information ancestry;
- acceptance artifacts reject display-string-only proof of temporal correctness.

Architecture should define the required evidence *shape*. Execution owns concrete gate names, fixture families, command output, thresholds, and pass/fail policy.

**External rationale.** Deterministic simulation testing sources reinforce the need for controlled clocks, reproducible inputs, and observability of timing-related divergence [EXT-DST], [EXT-LOCKSTEP].

### Finding R1 — Play experience and epistemic legibility

**Foundation driver.** The completeness report routed play experience and epistemic legibility to A10/A11 as an architecture concern: information hierarchy, friction, leads/notebook usefulness, actor-legible ignorance, and transcript evidence [TW-COMP]. Foundation `08` requires embodied/debug separation and TUI-playable mechanics [TW-F-08].

**Current architecture coverage.** A10 already provides embodied view models, semantic actions, why-not output, debug separation, and transcripts. A11 already covers incidents, leads, notices, story-sifting projections, and observer-only constraints. This is substantial coverage, not a blank gap.

**Tier-fit verdict.** Partial coverage / consolidation.

**Recommendation — substance and home.** In A10, name the embodied play loop at architecture altitude: the player should be able to form plans from actor-known view models, attempt semantic actions, receive actor-visible failure/why-not feedback, inspect source-bound notebook/lead surfaces, and use debug only as non-diegetic review. In A11, make lead usefulness a source-bound projection concern: leads can be stale, ambiguous, partial, contradictory, or actionable, but they do not become objective markers, quest stages, or hidden priority. Transcript evidence should demonstrate the loop without parsing display prose as authority.

**External rationale.** MDA's mechanics/dynamics/aesthetics separation supports focusing architecture on mechanics and readable dynamics, while refusing hidden drama controls [EXT-MDA]. Lean MVP language supports keeping first-playable legibility small and testable rather than broadening scope [EXT-LEAN].

### Finding R2 — Quantity, granularity, and fungibility

**Foundation driver.** Foundation `06` owns ordinary actions, food, money/payment/custody, and physical/resource consequences, while routing concrete inventory/economy mechanics below foundation [TW-F-06]. The completeness report routes representation seams to A09: object/lot/stock/capacity/ledger boundaries and operations that preserve custody/proof/event ancestry [TW-COMP].

**Current architecture coverage.** A09 already covers ordinary life, places, property/custody, finite food, local economy placeholders, wages, and event-sourced economy. It does not explicitly say how unique items differ from fungible quantities, capacities, or ledgers at architecture altitude.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A09, add a quantity/granularity/fungibility seam:

- Unique objects, countable lots, divisible stocks, capacities, debts, wages, and ledgers are separate representation classes because they preserve different identity/provenance/custody constraints.
- Operations such as split, merge, consume, spoil, reserve, share, transfer, hide, discover, pay, refuse, and reimburse must preserve event ancestry, custody/ownership/procedure context, and holder-known visibility.
- Fungible aggregation is allowed only when it does not erase information needed for action validation, provenance, replay, wrong belief, lead interpretation, institutional record, or later promotion.
- Projections may summarize quantities for UI/debug, but the authoritative lineage needed for replay and disputes cannot be replaced by a display total.

Do not choose data structures, unit vocabularies, money denominations, inventory schemas, or economy formulas.

**External rationale.** PROV-DM supports treating quantity transformations as derivation chains rather than simple numeric overwrites [EXT-PROV-DM].

### Finding R3 — Bounded affect / emotion

**Foundation driver.** Foundation `04` and `05` already support memory, salience, needs, intentions, and routines; the completeness report routes bounded affect to A05/A06 as architecture-level source-bearing pressure/salience, not foundation doctrine [TW-COMP].

**Current architecture coverage.** A05 has needs, motives, candidate generation, intention lifecycle, routine failure, and planning diagnostics. A06 has memory, belief uptake, contradiction, traces, and privacy/filtering. That is enough substrate for affect-like behavior but not a named seam.

**Tier-fit verdict.** Already-owned-close / consolidation.

**Recommendation — substance and home.** In A05, define affect as a bounded decision influence: a source-bearing salience/pressure modifier over candidate generation, method selection, interruption, concealment/confession/accusation/repair tendencies, or routine disruption. In A06, define affect-memory effects as provenance-bearing changes to salience, durability, recall priority, or belief uptake. Affect may explain why a holder prioritizes or avoids an option; it may not reveal truth, select hidden targets, bypass planning, overwrite beliefs without events, or force dramatic actions.

**External rationale.** Social-simulation emotion research supports emotions as decision influences and explanatory state, while warning that more affect parameters increase validation burden. That supports a narrow seam now, not a full emotional architecture [EXT-EMOTION].

### Finding R4 — Learning and adaptation

**Foundation driver.** Foundation `04`/`05` already allow beliefs, memories, routines, and planning to change through modeled experience; the completeness report routes learning/adaptation to A05/A06 as learned expectations, route/method reliability, trust, risk, skill, and routine adjustment with provenance [TW-COMP].

**Current architecture coverage.** A05 supports routine failure/replanning and intention state. A06 supports memory and belief changes with provenance. There is no compact distinction between remembering a fact and learning a generalized expectation or reliability estimate.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A05/A06, add a learned-expectation seam:

- A learned expectation is derived state from remembered experiences, modeled instruction, records, testimony, repeated failures/successes, or institution procedure outcomes.
- It is not a raw memory, not a truth cache, and not a global probability table unless its source and scope are modeled.
- Learned state can influence candidate ordering, method applicability, trust, risk aversion, skill confidence, route preference, and routine adaptation.
- It preserves source events, scope, holder, confidence/uncertainty if represented, contradiction/staleness, and reset/decay/override provenance.

Execution owns the depth of learning before first proof, concrete update rules, decay, thresholds, and fixture coverage.

### Finding R5 — Deterministic performance and fairness budgets

**Foundation driver.** Foundation `10` says human focus is not privilege and LOD/regional systems must preserve causality and fairness; foundation `14` says scheduler authority is not cognition authority [TW-F-10], [TW-F-14]. The completeness report routes deterministic performance/fairness budgets to A04/A05/A12/A13 [TW-COMP].

**Current architecture coverage.** A04 already has scheduler limits and deterministic action pipeline. A05 has actor decision transactions. A12 has LOD tiers, regional processes, and human-focus fairness. A13 has observability. What is missing is an architecture-level representation of budget exhaustion and fairness/starvation evidence.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** Add a cross-cutting architecture seam:

- A04 should state that scheduler budgets and ordering policies are deterministic and diagnosed. When budget limits prevent full cognition/procedure execution, the outcome must be typed as deferred, skipped, summarized, degraded, or blocked, with responsible layer and replay ancestry.
- A05 should state that bounded planning may fail or degrade for budget reasons only through typed decision diagnostics; it must not silently choose omniscient shortcuts or substitute marker actions as progress.
- A12 should state that LOD/time acceleration is a declared fidelity mode with fairness constraints: lower detail may summarize, but not erase active claims, procedures, leads, obligations, or starvation caused by persistent under-scheduling.
- A13 should require fairness and starvation review artifacts: which holders/processes were deferred, why, for how long or across what source interval, and what evidence proves no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing.

Do not choose numeric budgets, scheduling algorithms, fairness formulas, queues, or acceptance thresholds.

**External rationale.** Deterministic simulation testing and lockstep sources support reproducible scheduling inputs and controlled nondeterminism; ODD supports documenting temporal resolution and scheduling assumptions [EXT-DST], [EXT-LOCKSTEP], [EXT-ODD].

### Finding R6 — Practical bias and social harm

**Foundation driver.** Foundation `07` says institutions are fallible and can encode biased, stale, underfunded, delayed, refused, or misfiled outcomes. The completeness report routes practical bias/social-harm treatment to A08 as inspectable model inputs/procedure effects and domain-pack assumption ownership [TW-COMP].

**Current architecture coverage.** A08 already names bias, error, corruption, role powers, records, proof rules, sanctions, and institution-known context. Execution `11` already names wrong suspicion from partial, stale, biased, or misleading information. This is strong coverage; it needs consolidation and authoring discipline, not new moral authority.

**Tier-fit verdict.** Already-owned-close / consolidation.

**Recommendation — substance and home.** In A08, add practical-bias discipline:

- Bias and social harm are modeled as inspectable inputs, social-position effects, resource constraints, procedure rules, testimony credibility patterns, record access patterns, institutional memory, staff/resource availability, or prior decisions.
- The kernel remains genre-neutral and does not assert omniscient moral truth; domain packs own cultural/legal/institutional assumptions and must make them reviewable.
- Wrong suspicion, refusal, delay, sanction, misfiling, or unequal treatment must arise from holder/institution-known evidence and procedure state, including biased or stale inputs when modeled.
- Diagnostics should expose the modeled assumptions and procedure steps without granting actors hidden truth or turning social harm into an objective quest condition.

**External rationale.** NIST AI RMF and NIST bias guidance support treating risks to people and society as socio-technical and context-sensitive; Datasheets for Datasets supports documenting motivation, composition, intended uses, and assumptions. The architecture recommendation borrows the documentation/inspectability posture, not an AI compliance regime [EXT-NIST-AI-RMF], [EXT-NIST-BIAS], [EXT-DATASHEETS].

### Finding R7 — Authoring / compiler discipline

**Foundation driver.** Foundation `09` allows authoring causal possibility space but forbids outcome scripts, hidden director controls, quest flags, and player privilege. The completeness report routes authoring/compiler discipline to A13 as validation/observability seams that protect architecture before runtime [TW-F-09], [TW-COMP].

**Current architecture coverage.** A13 already lists validation reports, content validation, static guards, acceptance artifacts, typed observability, and invalid pass conditions. Execution `08` already owns data authoring/schema/provenance validation in detail. The architecture gap is making compiler/static validation a first-class architecture-protection contract, not optional lint.

**Tier-fit verdict.** Partial coverage.

**Recommendation — substance and home.** In A13, add an authoring/compiler-discipline seam:

- Content/schema validators, static guards, manifest checks, and review artifacts are architecture-protecting boundaries. They reject impossible or forbidden authoring forms before runtime, rather than relying on runtime filters to clean contaminated data.
- Validator outputs must be structured and layer-attributed: field/path or authored element, violated doctrine, responsible layer, provenance/source status, and author-actionable failure reason.
- The validation surface must protect against aliases, nested forbidden concepts, display-string-only proof, hidden-truth cognition fields, player/human privilege, silent migrations, incompatible content versions, and outcome chains.
- Architecture may require the seam and evidence shape. Execution/specs choose schemas, rule languages, commands, compatibility policies, and exact error formats.

**External rationale.** JSON Schema supports structural validation vocabularies, while OPA illustrates policy decision-making over structured inputs separated from enforcement. These sources sharpen the recommendation that Tracewake should make validation structured, explicit, and pre-runtime, while preserving its own doctrine as authority [EXT-JSON-SCHEMA], [EXT-OPA].

---

## 6. Forward-routing appendix

Architecture is the middle tier. The following findings should be handed to later tier sessions without amending those tiers here.

### 6.1 Execution-tier hand-offs

| Route | Target execution docs | Hand-off substance |
|---|---|---|
| Temporal firewall proof | `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Prove scheduler/replay time can validate/order/due effects but cannot generate proposals, routine choices, institution conclusions, embodied affordances, speech interpretations, leads, or LOD promotion without holder-known temporal provenance. |
| Temporal first-playable mechanism | `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`; `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`; `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`; `10` | Choose minimal calendar/day-part/“yesterday”/lateness vocabulary only when a scoped execution/spec pass needs it; add positive and negative fixtures for temporal claim provenance and actor-visible/debug split. |
| Scheduler budgets and fairness | `05`; `10`; `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Define concrete budget-exhaustion outcomes, starvation/fairness diagnostics, time-acceleration review packets, and deterministic ordering proof. Keep algorithms and thresholds out of architecture. |
| Institution/procedural time | `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | Later Phase-4 proof should cover office windows, due/lateness, queue aging, delayed/misfiled records, stale notices, payment periods, and sanctions as procedure-backed state with provenance and negative fixtures. |
| TUI time controls | `07`; `10` | Define when world-advancing time controls become player-facing rather than debug/operator-only, how interruptions stop acceleration, and how missed-event summaries remain actor-known. |
| LOD temporal ancestry | `12`; `10` | Define promotion/demotion proof for source intervals, cadence, fidelity limits, temporal ancestry, information ancestry, and replay divergence. |
| Quantity/fungibility proof | `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`; `09`; `10`; future scoped inventory/economy specs | Choose schemas and fixtures for split/merge/consume/spoil/reserve/share/transfer/hide/discover/pay/refuse/reimburse lineage. |
| Affect and learning depth | `06`; `10`; future scoped ordinary-life specs | Decide which affect/learning effects are first-playable, how they update/decay, and what negative fixtures prevent hidden-truth skill/trust updates. |
| Practical bias | `11`; `08`; `10` | Define domain-pack assumption review, biased/stale/partial evidence fixtures, wrong suspicion, refusals, delays, misfilings, and diagnostics. |
| Authoring/compiler checks | `08`; `09`; `10` | Implement concrete schema/static guards, adversarial renamed/nested fields, content-version compatibility, migration/upcast policy, and structured error outputs. |

### 6.2 Reference-tier hand-offs

| Route | Target reference docs | Hand-off substance |
|---|---|---|
| Temporal terminology | `02_GLOSSARY.md` | Add or clarify terms such as temporal authority, holder-known temporal claim, institution-known procedural time, freshness/staleness, due/lateness, time acceleration, and temporal ancestry after architecture wording stabilizes. |
| Temporal risk memory | `01_DESIGN_RISK_REGISTER.md` | Add risk notes for clock oracle leakage, raw-wall-clock contamination, omniscient lateness/office-closed labels, UI time acceleration leaks, and silent LOD temporal fill-in. |
| Quantity/fungibility terminology | `02_GLOSSARY.md` | Clarify unique object, lot, stock, capacity, ledger, custody transfer, payment/refusal, and fungible aggregation if these terms recur in specs. |
| Affect and learning terms | `02_GLOSSARY.md`; `01` | Add cautious terminology for affect/salience and learned expectation/reliability/trust/skill only when execution chooses scope; add risk note for truth-cache learning. |
| Bias/social-harm assumption memory | `01`; `02` | Add risk memory for hidden cultural/legal assumptions, “neutral” institution claims, or social-harm labels used as truth; add glossary terms only as review aids. |
| Authoring/compiler terms | `00`; `01`; `02` | Keep schema/static-policy/error terms subordinate to execution; add reference reminders that validation is doctrine protection, not mere lint. |

### 6.3 Future spec / amendment-process hand-offs

- A future architecture amendment spec should be documentation-only and should stage final wording in the named A02/A03/A04/A05/A08/A10/A11/A12/A13 homes. It should not create new constitutional invariants or execution gates.
- Concrete temporal values should wait for scoped execution/spec work: day-part vocabulary, date/time syntax, duration units, stale-after policy, office-hour representation, queue aging, time-acceleration speed, missed-summary threshold, and simultaneity/tie-break rules.
- A first temporal proof fixture package should include both friendly and adversarial cases: stale work assignment, office closed but actor does not know it, due record seen/not seen, late report with source, debug clock excluded from actor view, and LOD summary promotion without hidden temporal fill-in.
- A quantity/economy package should not be bundled with temporal proof unless the same gameplay feature truly requires it; otherwise it risks creating a broad inventory/economy spec before the architecture seam is stable.
- Acceptance-artifact template changes, if needed, belong below architecture and should preserve the current evidence-honesty posture: status, fingerprint scope, behavior witness, provenance, replay ancestry, and no pending/historical evidence counted as pass.

---

## 7. Open questions

1. **Minimal first-playable temporal vocabulary.** The foundation now owns temporal authority, and this report routes architecture seams, but the first usable vocabulary for day parts, “yesterday,” “last night,” office closed/open, due/late, and recently/stale remains an execution/spec owner decision.
2. **Calendar representation and duration units.** Architecture may name event/replay time and temporal claims, but concrete representation, conversion, display, and migration policy remain unresolved.
3. **Exact freshness/staleness policy.** A03/A06 should require provenance and classification; execution must later decide if and how stale-after windows, record validity periods, or claim decay are represented.
4. **Institution/procedural-time owner surface.** A08 should name the authority boundary, but future implementation must decide whether one procedural-time subsystem, per-institution procedures, or event/projection layers own specific lifecycle states.
5. **Simultaneity and tie-breaking.** Architecture can require deterministic ordering and diagnostics; execution must choose tie-break algorithms and proof surfaces.
6. **Temporal TUI budget.** It is unsettled how much exact time the first playable should render to the player, when time controls become embodied rather than debug-only, and how missed-event summaries should be phrased without hidden truth.
7. **LOD temporal-equivalence fidelity.** A12 should require declared interval/cadence/fidelity limits; concrete equivalence thresholds and promotion guarantees remain future work.
8. **Money/food/quantity representation.** The architecture seam can distinguish unique objects, lots, stocks, capacities, and ledgers, but exact schemas, units, denominations, and fixture contracts remain open.
9. **Affect scope before first proof.** Affect is safe as source-bearing salience/pressure, but the first playable may only need minimal affect-like pressures already covered by needs and memory.
10. **Learning/adaptation depth.** Learned expectations and trust/reliability are architecture-valid, but the project must decide how much learning exists before Phase 4 or second-proof scope.
11. **Domain-pack bias assumption ownership.** A08 should require inspectable assumptions, but the exact vehicle—domain-pack metadata, risk notes, content review packets, or spec appendices—remains owner work.
12. **Budget/fairness quantitative targets.** Architecture can require deterministic budget diagnostics and fairness/starvation observability; numeric targets, sampling policy, starvation thresholds, and algorithms route forward.
13. **Staged-abstraction vocabulary.** The completeness report's staged-incompleteness pressure remains below architecture unless recurring terminology drift justifies reference glossary additions.
14. **Possession-bind perception.** This was already a deferred owner decision in the prior architecture cascade; nothing in the temporal pass should silently decide it. If bind-time perception later exists, it must be a modeled channel for the actor, never a human knowledge transfer.

---

## 8. References

### 8.1 Exact-commit repository sources

- `docs/README.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/README.md
- `docs/0-foundation/00_FOUNDATION_INDEX.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/00_FOUNDATION_INDEX.md
- `docs/0-foundation/01_PROJECT_CHARTER.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/01_PROJECT_CHARTER.md
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- `docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- `reports/foundations-completeness-determination-research-report.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/foundations-completeness-determination-research-report.md
- `reports/verdict-on-foundations.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/verdict-on-foundations.md
- `reports/foundation-tier-alignment-research-report.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/reports/foundation-tier-alignment-research-report.md
- `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md
- `archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0026_FOUNDATION_EMERGENCE_EVIDENCE_ACCEPTANCE_DOCTRINE_AMENDMENT.md
- `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- `archive/reports/architecture-tier-alignment-research-report.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/archive/reports/architecture-tier-alignment-research-report.md
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- `docs/3-reference/02_GLOSSARY.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/3-reference/02_GLOSSARY.md
- `docs/4-specs/SPEC_LEDGER.md` — https://raw.githubusercontent.com/joeloverbeck/tracewake/ea6a05bf5822307cfcbd39804bbb015cdb13215d/docs/4-specs/SPEC_LEDGER.md

### 8.2 External sources cited

- [EXT-PROV-DM] W3C, **PROV-DM: The PROV Data Model** — https://www.w3.org/TR/prov-dm/
- [EXT-DES] Lawrence M. Leemis, **Chapter 5: Next-Event Simulation** — https://www.dmi.unict.it/messina/didat/DES_17_18/leemis_chapter5.pdf
- [EXT-ODD] Grimm et al., **The ODD Protocol for Describing Agent-Based and Other Simulation Models**, JASSS 2020 — https://www.jasss.org/23/2/7.html
- [EXT-CT-ABM] Tim Köster, **A Fast Embedded Language for Continuous-Time Agent-Based Simulation**, JASSS 2024 — https://www.jasss.org/27/1/10.html
- [EXT-MDA] Hunicke, LeBlanc, Zubek, **MDA: A Formal Approach to Game Design and Game Research**, AAAI — https://aaai.org/papers/ws04-04-001-mda-a-formal-approach-to-game-design-and-game-research/
- [EXT-EMOTION] Bourgais et al., **Emotion Modeling in Social Simulation: A Survey**, JASSS 2018 — https://www.jasss.org/21/2/5.html
- [EXT-JSON-SCHEMA] JSON Schema, **A Vocabulary for Structural Validation of JSON** — https://json-schema.org/draft/2020-12/json-schema-validation
- [EXT-OPA] Open Policy Agent, **OPA documentation** — https://www.openpolicyagent.org/docs
- [EXT-NIST-AI-RMF] NIST, **AI Risk Management Framework** — https://www.nist.gov/itl/ai-risk-management-framework
- [EXT-NIST-BIAS] NIST SP 1270, **Towards a Standard for Identifying and Managing Bias in Artificial Intelligence** — https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.1270.pdf
- [EXT-DATASHEETS] Gebru et al., **Datasheets for Datasets** — https://arxiv.org/abs/1803.09010
- [EXT-DST] Antithesis, **Deterministic simulation testing — how it works and when to use it** — https://antithesis.com/docs/resources/deterministic_simulation_testing/
- [EXT-LOCKSTEP] Glenn Fiedler, **Deterministic Lockstep** — https://gafferongames.com/post/deterministic_lockstep/
- [EXT-LEAN] The Lean Startup, **Methodology / principles** — https://theleanstartup.com/principles
