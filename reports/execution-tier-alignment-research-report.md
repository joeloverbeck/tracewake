# Execution-tier alignment research report

**Deliverable path:** `reports/execution-tier-alignment-research-report.md`  
**Repository under analysis:** `joeloverbeck/tracewake`  
**Target commit of record:** `c70d119bca7663e4e1229dcd2012d57b5fe72d44`  
**Report kind:** analysis / recommendation report for `docs/2-execution/*`, not a numbered implementation spec.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

This report is a downward-cascade delta. The amended foundation tier and architecture tier are treated as immutable governing reference. The amendment target is only `docs/2-execution/*`. The prior execution doctrine-alignment cascade is treated as settled context and is not re-commissioned.

---

## 1. Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: c70d119bca7663e4e1229dcd2012d57b5fe72d44
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open on exact raw.githubusercontent.com commit URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/foundations-completeness-determination-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/foundation-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/reports/execution-tier-alignment-research-report.md
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Source-provenance note.** "No contamination observed" means no fetched repository source URL, repository fetch target, redirect target observed in the repository-source workflow, or repository-source citation was outside the exact `joeloverbeck/tracewake` raw commit URL form above. External bibliography and external research sources were used only as external rationale where explicitly listed; they were never treated as repository provenance.

**Stale-provenance quarantine.** Several fetched predecessor reports and archived specs contain older commit strings as part of their own historical baselines. Those strings are quarantined as document content only. They are not freshness claims, not fetch targets, and not evidence that any file in this report was fetched from an older baseline. The live governing text for this report is the exact-commit source set above.

---

## 2. Disposition table

| Finding | Driver | Target execution doc(s) | Verdict | One-line basis |
|---|---:|---|---|---|
| F-01 Temporal alignment map and certification placement | T | `00`, `03`, cross-refs to `04`/`05`/`06`/`07`/`09`/`10`/`11`/`12` | Partial coverage | Execution already defines authority, gate labels, and sequence, but it does not yet map the new temporal-authority doctrine into proof placement. |
| F-02 Temporal firewall over scheduler/replay time | T | `04`, `05`, `10` | Partial coverage | The truth-firewall and no-direct-dispatch spine exists; the delta is to prove validator/scheduler time can trigger/order/validate without becoming holder-known planning input. |
| F-03 Holder-known temporal claims in first-playable proofs | T | `04`, `06`, `09`, `10` | Partial coverage | Existing provenance/staleness gates are strong, but temporal claim slots need first-playable positive and adversarial fixture families. |
| F-04 Routine and social-rhythm temporal premises | T | `06`, cross-ref `05`/`10` | Partial coverage | No-human routine proof exists; it must now require source-backed temporal premises for routine choice and routine continuation. |
| F-05 TUI time controls and temporal rendering | T | `07`, `10` | Partial coverage / already-owned-close | Embodied/debug split and staged time acceleration exist; add explicit temporal-label and time-control proof obligations without granting possession temporal privilege. |
| F-06 Institution and procedural-time proof | T | `11`, cross-ref `10`/`08` | Partial coverage | Phase-4 institution locks exist; due/late/open/closed/queue/notice status must be proven as record/procedure-backed rather than omniscient labels. |
| F-07 LOD, deferred scale, and time-acceleration ancestry | T | `12`, `10` | Partial coverage | Deferred second-proof scope exists; the delta is temporal ancestry, fidelity declaration, and fairness evidence for summary/promotion/demotion paths. |
| F-08 Quantity, granularity, fungibility, and custody validation | R | `08`, `09`, `10` | Partial coverage | DATA and fixture gates exist; execution must require proof shape for stocks, lots, ledgers, capacities, custody, split/merge, and fungibility without choosing schemas. |
| F-09 Learning/adaptation without hidden truth-cache | R | `04`, `06`, `10` | Partial coverage | Execution already blocks hidden truth planning; the delta is positive modeled-experience adaptation and adversarial anti-truth-cache learning evidence. |
| F-10 Deterministic budget and fairness proof | R | `05`, `10`, `12` | Partial coverage | Scheduler limits exist; execution must consolidate replayable budget-exhaustion, starvation/fairness, and deferred/LOD equivalence-or-declared-divergence evidence. |
| F-11 Practical bias and social harm as fallible institutional mechanics | R | `11`, cross-ref `08`/`10` | Partial coverage | Wrong suspicion and institutional proof exist; add inspectable bias/social-harm assumptions and evidence without encoding a moral oracle. |
| F-12 Authoring/compiler discipline for proof-bearing content | R | `08`, `09`, `10`, optional source note in `13` | Already-owned-close / partial coverage | Schema/provenance/negative fixture posture exists; execution should make compiler-like fail-closed validation explicit for temporal, quantity, bias, and staged declarations. |
| F-13 Staged-incompleteness declaration discipline | S | `00`, `03`, `10`; future template route to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Belongs in execution | This driver is not carried by the architecture cascade; execution must require every deliberate abstraction to declare proof, abstraction, non-faking, non-blocking, and evidence limits. |

Legend: **T** = temporal-authority cascade. **R** = completeness Block-R execution route. **S** = staged incompleteness.

---

## 3. Method & provenance ledger

### 3.1 Authority and altitude rule

The live docs establish a strict authority stack: foundation governs architecture, architecture governs execution, and lower tiers operationalize rather than override higher tiers. If execution conflicts with foundation or architecture, execution is wrong. This report therefore does not propose foundation or architecture changes; it routes upstream questions forward as open questions rather than repairing them inside execution. Sources: [TW-DOCS-README], [TW-F-02], [TW-F-03], [TW-A-00].

Execution altitude means the target docs should own gate procedure, certification-sequence placement, proof obligations, fixture-family shape, diagnostic expectations, and acceptance-contract additions. Execution should not choose concrete temporal vocabulary, calendar syntax, stale-after thresholds, durations, UI clock format, scheduler algorithms, fairness formulas, money denominations, inventory unit vocabularies, affect update rules, learning decay, or fixture file names. Those choices route to reference or scoped specs.

### 3.2 Files read and role

The source set was read in the order requested: governing foundation, governing architecture, execution target, lower-tier boundary-awareness files, then predecessor reports/specs. All listed paths were checked against the uploaded manifest and fetched only by exact raw commit URL.

| Source group | Role in this report |
|---|---|
| `docs/README.md` | Authority order and layering rule. |
| `docs/0-foundation/*` | Immutable constitutional and doctrinal reference; especially temporal authority, truth firewall, holder-known claims, routines, institution-known time, TUI possession/debug boundaries, LOD temporal ancestry, first-playable gates, and scheduler boundary. |
| `docs/1-architecture/*` | Immutable operational seams to be translated into execution proof: event/replay time, holder-known temporal claims, scheduler trigger-vs-plan firewall, routine temporal premises, temporal claim slots, institution procedure time, quantity/fungibility, TUI time controls, lead staleness, LOD/time acceleration, observability and authoring discipline. |
| `docs/2-execution/00…13` | Amendment target. Existing coverage was recorded before asserting gaps. |
| `docs/3-reference/*` | Forward-routing boundary: terms and risk memory are below execution and not yet temporally amended. |
| `docs/4-specs/*` | Forward-routing boundary: concrete values, fixture packages, and the acceptance-artifact template amendment belong to future scoped specs, not this report. |
| Predecessor reports and archived specs | Settled context and routing seeds. Older commit strings inside those files are quarantined. |

### 3.3 Settled context not re-commissioned

The prior execution-tier doctrine cascade already installed a strong anti-contamination spine: certification labels, truth-firewall gates, no-direct-dispatch gates, no-human routine gates, possession parity, replay/fixture/diagnostic posture, observer-only emergence evidence, Phase-4 institutional entry locks, and authoring/validation posture. This report does not reopen that cascade. Where a finding touches a settled gate family, the recommendation is additive: temporal, Block-R, or staged-incompleteness proof mechanics must be threaded into the existing home rather than re-naming, weakening, or replacing the existing gate discipline. Sources: [TW-E-00], [TW-E-03], [TW-E-04], [TW-E-05], [TW-E-06], [TW-E-07], [TW-E-08], [TW-E-09], [TW-E-10], [TW-E-11], [TW-ARCHIVE-SPEC-0028], [TW-ARCHIVE-REPORT-EXEC-0028].

### 3.4 External-research role

External sources were used only to sharpen proof shape, not to define Tracewake doctrine.

* Discrete-event simulation practice supports deterministic event ordering, explicit same-time tie handling, and the separation of event time from actor knowledge. [EXT-SIMPY]
* Event-sourcing practice supports append-only facts, replayable projections, audit, and diagnostics, which sharpen the replay and temporal-divergence proof shape. [EXT-AZURE-EVENT-SOURCING]
* W3C PROV-DM supports treating lineage, attribution, derivation, and collection membership as first-class evidence, which sharpens temporal-claim and LOD ancestry evidence. [EXT-PROV-DM]
* JSON Schema and policy-as-code practice sharpen the authoring/compiler discipline: content acceptance should be structural, fail-closed, and machine-checkable, while concrete vocabulary choices stay in lower scoped specs. [EXT-JSON-SCHEMA], [EXT-OPA-REGO]
* Deterministic simulation testing sharpens the need to seal clock/randomness/thread-interleaving sources and replay failures exactly, especially for budget/fairness and temporal-firewall tests. [EXT-ANTITHESIS-DST]
* Fair scheduling literature sharpens the requirement that "budget fairness" be evidenced over windows or declarations rather than inferred from aggregate throughput alone. [EXT-FAIR-SCHEDULING]
* Model-documentation practice in ODD, datasheets, NIST AI RMF, and technical-debt literature sharpens staged-abstraction honesty: declare what the model does, what it omits, what assumptions it carries, and what risks remain. [EXT-ODD], [EXT-DATASHEETS], [EXT-NIST-AI-RMF], [EXT-TECH-DEBT]

---

## 4. Execution-doc coverage register

| Execution doc | Current alignment verdict | Coverage note | Recommended posture |
|---|---|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Partial coverage | Strong authority, gate taxonomy, label-class reconciliation, and observation obligations already exist. | Add a temporal/Block-R/staged-incompleteness routing map at index altitude. The index should say where temporal proof, Block-R proof, and staged declarations land, without minting new labels. |
| `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` | Already-owned-close | Correctly keeps historical baseline/archive status from becoming certification. | No primary T/R/S amendment. Cross-reference only if staged-declaration wording needs to remind reviewers that archived historical status is not proof. |
| `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | Partial / close | Defines first proof scope and acceptance stance. | No broad rewrite. Add only narrow cross-reference that first-playable acceptance now includes temporal-firewall evidence through `04`, routine temporal proof through `06`, embodied temporal rendering through `07`, fixtures through `09`, and diagnostics through `10`. |
| `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | Partial coverage | Certification sequence already exists and protects settled gate order. | Add where temporal cascade proof is introduced in the sequence and where staged-abstraction declarations are reviewed. This doc should coordinate, not duplicate, the budget/fairness proof. |
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | Partial coverage | Strong sealed context, provenance, staleness, and fail-closed truth-firewall posture. | Add temporal-firewall proof mechanics: raw event/replay/scheduler time may validate and trigger, but actor/institution planning must consume only holder-known temporal claims. Add hidden-truth-learning negative evidence. |
| `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Partial coverage | Strong action pipeline, transaction, no-direct-dispatch, and scheduler boundary. | Add scheduler trigger-vs-plan temporal firewall and consolidated deterministic budget/fairness evidence. Scheduler time may order opportunities and validate effects, not invent intentions or priority. |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Partial coverage | Strong no-human and ordinary-life proof home. | Add routine/social-rhythm temporal-premise requirements and positive learning/adaptation proof. Routine choice must show the source of temporal premise, not rely on true time alone. |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Partial / close | Strong embodied/debug split, possession parity, freshness display, and staged time-acceleration note. | Add temporal-rendering proof: actor-visible labels need modeled sources; debug exact time and time controls remain non-diegetic; possession must not upgrade temporal knowledge. |
| `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Partial coverage | Strong DATA-CERT, provenance, schema, negative fixture, and migration posture. | Add quantity/fungibility validation and compiler-like authoring discipline for temporal/procedural/bias/staged declarations. Do not choose units, schemas, or denominations here. |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Partial coverage | Strong fixture taxonomy, replay acceptance, adversarial fixture requirements, and semantic proof posture. | Add temporal fixture-family requirements and separate quantity/economy fixture-family requirements. Do not name fixtures or bundle the packages unless a future gameplay feature genuinely needs both. |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Partial coverage | Strong test families, diagnostic standards, responsible-layer labels, EMERGE-OBS observer-only evidence, anti-vacuity, and review templates. | Add temporal-divergence, budget/fairness, LOD/time-acceleration, learning anti-cache, institution procedure-time, quantity lineage, and staged-abstraction evidence fields. Keep observer-only summaries non-certifying and non-simulation-input. |
| `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | Partial coverage | Strong Phase-4 entry lock, institution-known context, record status, wrong-suspicion causality, and locked fixtures. | Add procedural-time proof for due/late/open/closed/queue/notice lifecycle and practical-bias/social-harm proof through records, reports, procedures, refusal, delay, and misfiling. |
| `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Partial coverage | Correctly locks notices/travel/LOD/story-sifting as deferred second proof. | Add LOD/time-acceleration temporal-ancestry and fairness obligations for future second proof: summary interval/cadence/fidelity, information ancestry, skipped/deferred cognition, and declared divergence. |
| `13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` | Already-owned-close / minor route | Correctly records research-source handling and forbidden misreads. | Optional minor addition in a later amendment: source notes for temporal execution proof, deterministic testing, provenance, fairness, and staged-honesty references. Not a primary doctrine home. |

---

## 5. Per-finding analysis and recommendations

### F-01 — Temporal alignment map and certification placement

**Driver.** Driver T descends from the foundation temporal-authority rule: time may validate, but holder-known time must plan; event/replay time, scheduler time, holder-known temporal claims, institution-known procedural time, player-facing time controls, and LOD temporal summaries occupy distinct authority categories. Architecture translates that into an explicit temporal-authority conformance map and assigns seams to event/replay, holder-known contexts, scheduler pipeline, routines, institutions, TUI rendering, LOD, and observability. Sources: [TW-F-02], [TW-F-03], [TW-A-00], [TW-A-02], [TW-A-03], [TW-A-04], [TW-A-05], [TW-A-08], [TW-A-10], [TW-A-12], [TW-A-13].

**Current execution coverage.** `00` already defines the execution tier's authority, label taxonomy, gate names, observation obligations, and maintenance posture. `03` already defines certification sequence and avoids treating historical documentation as proof. The gap is not missing execution authority; it is missing a visible map from the new temporal doctrine to the existing execution homes. Sources: [TW-E-00], [TW-E-03].

**Tier-fit verdict.** Belongs in execution as a coordination and certification-placement delta. The foundation and architecture already own the doctrine; execution owns where the proof enters the gate ladder.

**Recommendation — substance and home.**

* Amend `00` to add a compact "temporal / Block-R / staged-declaration routing" subsection. It should point each temporal proof surface to its execution home: temporal firewall and holder-known time in `04`; scheduler trigger-vs-plan and budgets in `05`; routines in `06`; TUI rendering and time controls in `07`; authoring validation in `08`; fixture families in `09`; diagnostics/review artifacts in `10`; institutions/procedural time in `11`; LOD/time acceleration in `12`.
* Amend `03` to place the temporal cascade in the existing certification sequence. It should not create a new gate code. It should explain where temporal evidence must appear before first proof acceptance, where Phase-4 procedural-time evidence appears, and where second-proof LOD/time-acceleration evidence is deferred but declared.
* Add a guardrail sentence to `00`/`03` that concrete temporal values and terminology are lower-tier decisions, not execution-tier choices.

**External rationale.** Discrete-event simulation prior art treats the event queue and time ordering as deterministic infrastructure, while model behavior remains whatever the modeled processes do at those events. That supports a map separating event ordering from cognition. [EXT-SIMPY]

---

### F-02 — Temporal firewall over scheduler/replay time

**Driver.** Foundation now treats the temporal firewall as the truth firewall applied to time: raw event/replay/scheduler time may order and validate, but holder cognition, routine selection, institution procedure, embodied views, leads, and LOD promotion may use temporal facts only when those facts reached the relevant holder through modeled channels. Architecture places the boundary in event/replay, holder-known contexts, and the action proposal / scheduling / feedback pipeline. Sources: [TW-F-02], [TW-F-03], [TW-F-14], [TW-A-02], [TW-A-03], [TW-A-04], [TW-A-13].

**Current execution coverage.** `04` already owns the truth-firewall / actor-known / anti-contamination gate and fail-closed provenance posture. `05` already owns transaction scheduling and no direct dispatch. `10` already owns diagnostic standards, responsible-layer labels, and evidence review. These are the correct homes. What is missing is the temporal specialization: proof that the scheduler's knowledge of time does not become an actor, institution, lead, or view-model premise. Sources: [TW-E-04], [TW-E-05], [TW-E-10].

**Tier-fit verdict.** Partial coverage. Additive temporal proof belongs in execution because it is a gate and diagnostic obligation, not a new doctrine.

**Recommendation — substance and home.**

* Amend `04` to require temporal-firewall evidence in every truth-firewall certification touching time. The evidence should show the source of every temporal premise consumed by actor-known or institution-known code: observation, memory, record, notice, testimony, public cue, artifact, modeled procedure, or source-backed inference.
* Amend `04` to require adversarial evidence that raw scheduler time, replay order, debug panels, event timestamps, sorted queues, and validator-known future/due states cannot be read as holder knowledge through type shortcuts, cached truth, renamed fields, derived convenience helpers, or prompt/context prose.
* Amend `05` to state the scheduler's temporal authority at execution altitude: it may awaken candidates, order transactions, validate preconditions/effects, and account for budget exhaustion; it may not select intentions, invent reasons, rewrite wait causes, or make routine conclusions by consulting true time alone.
* Amend `10` to require temporal-divergence diagnostics with responsible-layer labels. A failure should identify whether leakage came from candidate generation, sealed context assembly, scheduler dispatch, action validation, projection/view rendering, fixture authoring, or review artifact construction.

**External rationale.** Event-sourcing practice treats an append-only log as authoritative for replay and audit, but projections and decisions must still derive from declared events and state transitions. Deterministic testing practice reinforces that clocks and scheduling sources need to be sealed so reproducible replay can identify the layer where leakage entered. [EXT-AZURE-EVENT-SOURCING], [EXT-ANTITHESIS-DST]

---

### F-03 — Holder-known temporal claims in first-playable proofs

**Driver.** Foundation requires temporal expressions, freshness, lateness, and validity windows to be holder-known claims with provenance rather than silent world-clock updates. The first-playable acceptance surface names temporal-firewall expectations around work time/assignment, stale records, closed/late labels, sleep/wait/travel time, and replay distinction between validator time and holder-known temporal premises. Architecture gives temporal-claim slots to claims/beliefs/memory and routes lead staleness/usefulness labels through source-bound evidence. Sources: [TW-F-04], [TW-F-12], [TW-A-03], [TW-A-06], [TW-A-11], [TW-A-13].

**Current execution coverage.** `04` already has provenance/staleness/fail-closed actor-known gate posture. `06` already owns no-human ordinary-life proof. `09` already defines positive, negative, adversarial, and replay fixture posture. `10` already demands diagnostic standards and review evidence. The existing proof spine is correct, but its examples are not yet explicitly temporal at first-playable altitude. Sources: [TW-E-04], [TW-E-06], [TW-E-09], [TW-E-10].

**Tier-fit verdict.** Partial coverage. The right delta is fixture-family and diagnostic requirements, not final fixture names.

**Recommendation — substance and home.**

* Amend `04` to require temporal claim slots in holder-known context checks when a decision depends on time: event/acquisition/verification/procedure source, valid/expired/stale status as known by the holder, and explicit uncertainty when appropriate.
* Amend `06` to require first-playable no-human scenarios in which routine behavior succeeds from modeled temporal premises and fails or waits when only ground truth time would justify action.
* Amend `09` to require a first temporal-firewall fixture package with both friendly and adversarial families. Friendly families should exercise holder-known work/routine timing, stale-but-believed notices, institutional records, and interruption/wait effects. Adversarial families should attempt raw clock leakage, debug-panel leakage, omniscient due/closed labels, and restamping old knowledge as fresh.
* Amend `10` to require review artifacts to distinguish "validator time used to validate" from "holder-known temporal premise used to plan." Evidence should show both positive acceptance paths and fail-closed negative paths.

**External rationale.** W3C PROV-DM supports treating time-bearing claims as provenance-bearing objects rather than free-floating values. ODD's model-description discipline supports specifying what the model actually does and does not do; this is useful for acceptance artifacts that must not overclaim time cognition. [EXT-PROV-DM], [EXT-ODD]

---

### F-04 — Routine and social-rhythm temporal premises

**Driver.** Foundation routes routines and social rhythms through holder-known temporal premises. Architecture requires routine temporal inputs to come from actor-known assignments, memory, observation, public cues, records, testimony, institutional context, or source-backed inference. Scheduler triggers can create an opportunity, but routine selection remains actor-known. Sources: [TW-F-05], [TW-F-14], [TW-A-04], [TW-A-05], [TW-A-06].

**Current execution coverage.** `06` already owns ordinary-life needs, routines, and no-human proof. It correctly emphasizes needs as causal pressures rather than true targets, routine templates, failure diagnostics, and no-human evidence. `05` already owns pipeline/scheduler boundaries, and `10` owns diagnostics. The gap is that routine temporal premises are not yet explicit proof inputs. Sources: [TW-E-06], [TW-E-05], [TW-E-10].

**Tier-fit verdict.** Partial coverage. The delta belongs in `06`, with cross-references to `05` and `10`.

**Recommendation — substance and home.**

* Amend `06` to require each routine or social-rhythm proof to identify the temporal premise source category it used. Acceptable proof shape is not the vocabulary itself, but the fact that the actor's routine selected or continued because a modeled channel supplied the premise.
* Require negative no-human examples where a routine would be correct under true schedule time but is not selected because the actor lacks the source-backed premise.
* Require positive adaptation examples where repeated modeled experience, contradiction, interruption, or notice changes later routine selection through an actor-known or institution-known path.
* Cross-reference `05` so scheduler awakenings and elapsed-time accounting do not count as routine premise evidence.
* Cross-reference `10` so diagnostics identify whether a routine failure is due to missing knowledge, stale knowledge, budget exhaustion, blocked affordance, or validation failure.

**External rationale.** Agent-based model documentation practice treats scheduling and agent decision rules as separately described components. That supports execution evidence that distinguishes "process activated at a time" from "agent selected a routine using a temporal belief." [EXT-ODD]

---

### F-05 — TUI time controls and temporal rendering

**Driver.** Foundation allows TUI time controls to advance simulation but forbids them from becoming omniscient time skips. Embodied surfaces may show temporal facts only when those facts are actor-known, institution-known, or source-backed; debug exact time remains non-diegetic. Architecture maps this into possession/TUI/view-model boundaries and time-control rendering seams. Sources: [TW-F-08], [TW-F-14], [TW-A-10], [TW-A-13].

**Current execution coverage.** `07` already owns epistemic view models, possession, debug proof, freshness, snapshot proof, and a staged note that player-facing time acceleration is debug-only for the current baseline. `10` owns diagnostics and observer-only evidence. This is close, but temporal rendering needs explicit proof hooks before later player-facing time controls appear. Sources: [TW-E-07], [TW-E-10].

**Tier-fit verdict.** Already-owned-close for the embodied/debug split; partial coverage for temporal rendering and future time controls.

**Recommendation — substance and home.**

* Amend `07` to require temporal labels in embodied views to cite modeled sources, just as other actor-visible facts do. A label such as late, stale, closed, due, soon, recently, or missed should not appear merely because the client can read the clock or queue.
* Require possession-parity evidence: possessing an actor must not refresh, reveal, or reinterpret temporal facts beyond what that actor would otherwise know.
* Require future player-facing time-control proof to show that advancing time produces events and observations through modeled channels, not retroactive omniscient summaries. Missed-event summaries, if later introduced, must carry source and holder visibility.
* Keep debug exact time, raw queues, replay timestamps, and acceleration internals non-diegetic and excluded from actor-known context.
* Amend `10` to include temporal-rendering diagnostics that distinguish embodied, possession, debug, transcript, and observer-only surfaces.

**External rationale.** Deterministic simulation testing recommends controlling time/entropy sources so visible failures can be replayed. Here that supports separating UI controls from actor knowledge and requiring replayable diagnostics for any temporal rendering. [EXT-ANTITHESIS-DST]

---

### F-06 — Institution and procedural-time proof

**Driver.** Foundation treats institution-known procedural time as modeled, record/procedure-backed time: hours, due dates, lateness, queue aging, notice lifecycle, payments, sanctions, and case status must come from institutional records, artifacts, procedures, or reports, not omniscient labels. Architecture places this in institutions/records/procedures and routes procedural bias/social harm into inspectable institutional mechanics. Sources: [TW-F-07], [TW-F-04], [TW-F-14], [TW-A-08], [TW-A-13].

**Current execution coverage.** `11` already owns Phase-4 institutions, records, wrong suspicion, institution-known context, provenance/freshness negatives, record time/status, and locked fixtures. It is the correct primary home. `10` already owns diagnostics. `08` owns authoring validation. The missing delta is procedural-time proof and practical-bias time effects. Sources: [TW-E-11], [TW-E-10], [TW-E-08].

**Tier-fit verdict.** Partial coverage. Execution should add procedure-time proof obligations; it should not choose calendar vocabulary or legal/procedural values.

**Recommendation — substance and home.**

* Amend `11` to require Phase-4 procedural-time evidence for any institution label that has time semantics: open/closed, due/late, expired/current, pending/resolved, queued/aged, notified/served, paid/unpaid, sanctioned, appealed, or similar.
* Require each institutional temporal status to be backed by a modeled institution-known source: record, schedule artifact, notice, ledger entry, procedure state, testimony accepted by procedure, inspection, or modeled staff action.
* Require adversarial institution fixtures where true time would justify a label but institution records do not yet support it, and where stale or mistaken records cause plausible institutional error.
* Cross-reference `08` so content packs cannot author omniscient procedural-time conclusions directly.
* Cross-reference `10` so diagnostics distinguish record error, procedure delay, source staleness, contradictory testimony, missing artifact, and hidden ground-truth leakage.

**External rationale.** Provenance models sharpen this proof: a procedural-time conclusion is useful only if its derivation, source record, activity, and responsible entity are available for review. [EXT-PROV-DM]

---

### F-07 — LOD, deferred scale, and time-acceleration ancestry

**Driver.** Foundation requires LOD/regional summaries to preserve temporal ancestry, information ancestry, interval/cadence, fidelity, known-to-whom status, and fairness. Architecture requires LOD summaries and time acceleration to declare source interval, cadence, resolution, fidelity, information ancestry distinct from event-time ancestry, and fairness constraints. Sources: [TW-F-10], [TW-A-12], [TW-A-13].

**Current execution coverage.** `12` correctly treats notices, travel, regional scale, LOD, and story-sifting as deferred second proof. `10` owns diagnostics and review artifacts. The current docs defer the feature, but they need the acceptance obligations that future second proof must not escape. Sources: [TW-E-12], [TW-E-10].

**Tier-fit verdict.** Partial coverage. This is an execution proof obligation for deferred work, not a current implementation choice.

**Recommendation — substance and home.**

* Amend `12` to require future LOD/time-acceleration proof to declare interval, cadence, resolution, fidelity, event ancestry, information ancestry, and known-to-whom status for every summary or promoted state.
* Require equivalence-or-declared-divergence evidence: if accelerated or regional processing differs from full-resolution processing, the difference must be declared, bounded, and tested against fairness and epistemic constraints.
* Require promotion/demotion proof that actors, institutions, leads, and views do not gain omniscient temporal facts during LOD transitions.
* Require skipped/deferred cognition accounting: time acceleration cannot invisibly starve classes of actors or silently script outcomes.
* Amend `10` to add diagnostic expectations for LOD temporal ancestry and fairness review.

**External rationale.** Event-sourcing and provenance practice both support keeping derivation chains inspectable across projections/summaries. Fair scheduling literature supports treating fairness over windows as an explicit acceptance concern rather than assuming aggregate throughput proves non-starvation. [EXT-AZURE-EVENT-SOURCING], [EXT-PROV-DM], [EXT-FAIR-SCHEDULING]

---

### F-08 — Quantity, granularity, fungibility, and custody validation

**Driver.** Driver R routes quantity/granularity/fungibility to execution. Architecture establishes a representation seam for unique items, lots, stocks, ledgers, balances, household pools, service capacities, split/merge, consume/spoil, reserve/share, transfer/hide/discover, pay/refuse/reimburse, and custody/ancestry/procedure visibility. The completeness report routes execution proof to `08`, plus future inventory/economy fixture specs. Sources: [TW-A-09], [TW-A-13], [TW-REPORT-COMPLETE], [TW-REPORT-ARCH].

**Current execution coverage.** `08` already owns authoring schema, provenance, validation, forbidden forms, domain contracts, schema migration, and data certification. `09` owns fixture taxonomy and replay acceptance. `10` owns diagnostics and review artifacts. These are correct homes, but quantity/fungibility proof is not yet explicit. Sources: [TW-E-08], [TW-E-09], [TW-E-10].

**Tier-fit verdict.** Partial coverage. Execution should specify validation/proof shape, not concrete inventory/economy schemas.

**Recommendation — substance and home.**

* Amend `08` to require content validation for quantity-bearing and fungible/partly fungible entities to preserve identity, quantity, custody, ownership/control, provenance, split/merge lineage, transformation, spoilage/consumption, reservation, transfer, concealment/discovery, and institution-visible record effects as appropriate to the authored domain.
* Require validation to fail closed on ambiguous fungibility, implicit global pools, untracked disappearance/creation, balance edits without ledger ancestry, and authored facts that make an actor or institution know quantity/custody without a modeled channel.
* Amend `09` to require separate fixture families for quantity/economy behavior: positive transfer/consume/split/merge/custody scenarios and adversarial hidden stock/ledger/procedure cases. Do not name fixtures here.
* Amend `10` to require diagnostics that can identify representation error, custody-lineage error, procedure-visibility error, and replay divergence.
* Preserve the bundling guidance: do not bundle quantity/economy proof with temporal proof unless a future gameplay feature genuinely needs both.

**External rationale.** JSON Schema and policy-as-code practice support machine-checkable structural validation, while provenance practice supports custody/derivation lineage. These sources sharpen the validation proof shape without selecting Tracewake's units or schemas. [EXT-JSON-SCHEMA], [EXT-OPA-REGO], [EXT-PROV-DM]

---

### F-09 — Learning/adaptation without hidden truth-cache

**Driver.** Driver R routes learning/adaptation to execution. Foundation and architecture require beliefs, memory, expectations, affect, and learning to evolve through modeled experience and holder-known channels, not through hidden truth caches or omniscient correction. Architecture separates learned expectations from memory and routes affect/learning depths to future specs. Sources: [TW-F-04], [TW-F-05], [TW-F-14], [TW-A-05], [TW-A-06], [TW-A-13].

**Current execution coverage.** `04` already blocks hidden truth planning and owns anti-contamination gates. `06` already owns no-human ordinary-life proof, routine failures, and routine adaptation surfaces. `10` owns diagnostics. The missing delta is an explicit two-sided learning proof: modeled experience changes future behavior, while hidden truth never does. Sources: [TW-E-04], [TW-E-06], [TW-E-10].

**Tier-fit verdict.** Partial coverage. Execution should require positive and adversarial evidence but route update formulas, decay, thresholds, and affect depth to future specs.

**Recommendation — substance and home.**

* Amend `04` to require negative hidden-truth-learning evidence: neither failed actions, scheduler denials, debug facts, true object locations, true schedules, true institutional status, nor replay-only diagnostics may become future actor belief/expectation unless a modeled experience or communication channel emitted a claim, memory, contradiction, observation, notice, or record.
* Amend `06` to require positive adaptation proof: repeated modeled experience, contradiction, or changed routine outcomes can affect future routine/method/trust selection through holder-known memory/expectation channels.
* Amend `10` to require diagnostics that distinguish learned expectation, remembered event, direct observation, testimony, record-derived belief, and prohibited truth-cache update.
* Route affect/learning update rules, strength/decay, trust-update semantics, and thresholds to a future scoped spec.

**External rationale.** Datasheet/model-documentation practice supports explicitly recording intended uses, limitations, and provenance of data/model behavior; here that sharpens evidence for "this adaptation was modeled" versus "this was an accidental truth cache." [EXT-DATASHEETS]

---

### F-10 — Deterministic budget and fairness proof

**Driver.** Driver R routes deterministic performance/fairness budgets to execution. Architecture consolidates this in the scheduler/observability/LOD seams: deterministic ordering, planner budgets, budget exhaustion, skipped/deferred cognition, starvation/fairness, time acceleration, and LOD equivalence-or-declared-divergence must be reviewable and replayable. Sources: [TW-A-04], [TW-A-12], [TW-A-13], [TW-REPORT-COMPLETE], [TW-REPORT-ARCH].

**Current execution coverage.** `05` already constrains action dispatch and scheduler boundaries. `10` already requires diagnostic standards, anti-vacuity, CI/review posture, and responsible layers. `12` already defers LOD/scale. The gap is consolidated deterministic budget/fairness proof. It should not be triplicated across temporal, scheduler, and observability gates. Sources: [TW-E-05], [TW-E-10], [TW-E-12].

**Tier-fit verdict.** Partial coverage. Execution owns evidence shape; future specs own numeric targets and scheduling algorithms.

**Recommendation — substance and home.**

* Amend `05` to require scheduler-budget evidence for deterministic candidate ordering, budget exhaustion, deferred/skipped cognition, and no-direct-dispatch behavior under load. The doc should state that budget pressure is not allowed to become an invisible director that chooses outcomes without typed evidence.
* Amend `10` as the consolidated home for budget/fairness diagnostics. Evidence should identify starvation risk, repeated deferral, actor-class or region-class imbalance, time-acceleration effects, and replay determinism.
* Amend `12` to require deferred-scale fairness declarations for any future LOD/time-accelerated process.
* Do not choose fairness formulas, window sizes, budgets, priority algorithms, or thresholds at execution altitude. Require future specs to choose them and justify conformance.

**External rationale.** Deterministic simulation testing supports replaying scheduler bugs exactly by sealing nondeterministic clocks/randomness/thread interleavings. Fair scheduling literature supports explicit short-term fairness constraints to avoid long starvation periods, reinforcing that aggregate throughput is not enough. [EXT-ANTITHESIS-DST], [EXT-FAIR-SCHEDULING]

---

### F-11 — Practical bias and social harm as fallible institutional mechanics

**Driver.** Driver R routes practical bias/social harm to execution. Architecture says bias and social harm should be represented as inspectable model inputs/effects in institutions, testimony, credibility, records, underfunding, refusal, delay, misfiling, suppressed/ignored records, and wrong suspicion. Foundation requires fallible institutions and forbids omniscient truth shortcuts. Sources: [TW-F-07], [TW-F-14], [TW-A-08], [TW-A-13], [TW-REPORT-COMPLETE].

**Current execution coverage.** `11` already owns wrong suspicion and Phase-4 institution entry. It requires institution-known context, provenance/freshness negatives, record time/status, wrong suspicion causes, and locked fixtures. `08` owns validation and `10` owns diagnostics. The existing home is correct, but practical-bias proof is not explicit. Sources: [TW-E-11], [TW-E-08], [TW-E-10].

**Tier-fit verdict.** Partial coverage. Execution should require proof that bias/social harm flows through modeled institutions and records, not through omniscient guilt/innocence or authorial judgment.

**Recommendation — substance and home.**

* Amend `11` to require Phase-4 practical-bias evidence where institutional outcomes may be shaped by modeled testimony quality, unequal credibility, access barriers, underfunding, refusal, delay, misfiling, contradictory records, stale records, suppressed/ignored records, or staff procedure.
* Require wrong-suspicion proof to show the actor/institution-known path that made a suspicion plausible without consulting hidden culprit truth.
* Amend `08` to require domain-pack assumptions about bias/social harm to be explicit, validated, and reviewable rather than implied by prose.
* Amend `10` to require diagnostics that identify the modeled source of a biased or harmful outcome and distinguish it from hidden truth leakage or arbitrary author fiat.
* Route concrete domain-pack bias assumptions and evaluation criteria to future scoped specs.

**External rationale.** NIST AI RMF and datasheet practice support explicit assumption and risk documentation for systems that affect social outcomes. They sharpen the need for reviewable assumptions but do not define Tracewake's moral or institutional model. [EXT-NIST-AI-RMF], [EXT-DATASHEETS]

---

### F-12 — Authoring/compiler discipline for proof-bearing content

**Driver.** Driver R routes authoring/compiler discipline to execution. Architecture requires validation and observability evidence for content, fixtures, seeds, and authored facts so prose cannot birth simulation facts. Foundation forbids scripting and prose-born facts. Sources: [TW-F-09], [TW-F-14], [TW-A-13], [TW-A-01], [TW-REPORT-COMPLETE].

**Current execution coverage.** `08` already owns schema/provenance/domain contract/migration validation and forbidden forms. `09` already owns adversarial fixture contracts. `10` already owns diagnostics, anti-vacuity, and CI/review posture. This is close, but new temporal/quantity/bias/staged declarations need the same compiler-like discipline. Sources: [TW-E-08], [TW-E-09], [TW-E-10].

**Tier-fit verdict.** Already-owned-close for the proof home; partial coverage for new payload classes.

**Recommendation — substance and home.**

* Amend `08` to make explicit that temporal claims, procedural-time records, quantity/custody records, bias/social-harm assumption packets, and staged-abstraction declarations are proof-bearing authored content and must be structurally validated.
* Require fail-closed behavior for malformed provenance, missing source channels, ambiguous authority category, hidden truth labels in prose fields, restamped freshness, implicit global state, and unreviewable assumptions.
* Amend `09` to require adversarial fixtures that attempt to bypass validation through renamed fields, nested prose, stale strings, generated content, fixture metadata, or review artifact text.
* Amend `10` to require review evidence that static validation ran, negative fixtures failed for the intended reason, and diagnostics identified the responsible layer.
* Optional: amend `13` with source notes for compiler-like validation and policy-as-code references when the execution doc amendment is later drafted.

**External rationale.** JSON Schema supports structural validation for authored data; policy-as-code practice supports declarative constraints over structured inputs. Deterministic testing practice supports negative fixtures and replayable failure witnesses. [EXT-JSON-SCHEMA], [EXT-OPA-REGO], [EXT-ANTITHESIS-DST]

---

### F-13 — Staged-incompleteness declaration discipline

**Driver.** Driver S is routed directly to execution by the completeness determination. It did not primarily pass through the architecture enactment, so execution must not lose it. The discipline: every deliberate abstraction/staged feature must declare what it proves now, what it abstracts, what it must not fake, what future feature it must not block, and what acceptance evidence prevents overclaiming. Sources: [TW-REPORT-COMPLETE], [TW-E-00], [TW-E-03], [TW-E-10].

**Current execution coverage.** Execution already distinguishes historical/archive status from certification, defines gate labels, records deferred second-proof areas, and has evidence honesty in diagnostics/review artifacts. That is a good base. The missing piece is a required declaration discipline that applies whenever an accepted stage intentionally abstracts a future feature. Sources: [TW-E-00], [TW-E-01], [TW-E-03], [TW-E-10], [TW-E-12].

**Tier-fit verdict.** Belongs in execution. This is not a lower-tier value choice. It is an execution acceptance-honesty rule.

**Recommendation — substance and home.**

* Amend `00` to add the authority-level rule: staged proof is allowed only when the staged abstraction is declared and bounded; staged proof must not certify the unimplemented future feature by implication.
* Amend `03` to place staged-declaration review in the certification sequence. The sequence should require the declaration before acceptance evidence is treated as sufficient for a stage.
* Amend `10` to add staged-abstraction review fields: proof currently provided; behavior intentionally abstracted; falsehoods the stage must not fake; future feature or tier it must not block; evidence that prevents overclaiming; diagnostics that would fail if the abstraction leaks into certification.
* Route the acceptance-artifact template addition to a future scoped spec amending `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
* Do not mint a new gate label or observation-obligation code in this report.

**External rationale.** Technical-debt and model-documentation practice both support explicit declarations of deliberate limitation. The important execution move is not to punish staging; it is to make staging legible, bounded, and non-certifying beyond its declared scope. [EXT-TECH-DEBT], [EXT-ODD], [EXT-DATASHEETS]

---

## 6. Forward-routing appendix

Execution is not the lowest tier. The following items should be routed forward after execution wording stabilizes.

### 6.1 Reference-tier hand-offs

Route to a future `docs/3-reference/*` session:

| Reference target | Hand-off |
|---|---|
| `docs/3-reference/02_GLOSSARY.md` | Add glossary entries for temporal authority, event/replay time, scheduler time, holder-known temporal claim, institution-known procedural time, temporal firewall, freshness/staleness, validity window, temporal ancestry, information ancestry, time acceleration, LOD summary cadence, staged abstraction, and false certification. |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Add or amend risks for clock-oracle leakage, raw wall-clock contamination, omniscient lateness/office-closed labels, UI time-acceleration leaks, debug time becoming diegetic, silent LOD temporal fill-in, truth-cache learning, performance pressure as invisible director, budget starvation hidden by aggregate success, staged-abstraction false certification, and quantity/economy lineage collapse. |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Update checklist pointers after the execution amendment lands so reviewers can find the temporal and staged-declaration terms in lower-tier reference docs. |

Reference should not ratify concrete temporal values. It should stabilize terminology and risks after the execution proof posture is known.

### 6.2 Future scoped-spec / amendment-process hand-offs

Route to future `docs/4-specs/*` or later scoped implementation specs:

| Future spec area | Hand-off |
|---|---|
| Concrete temporal vocabulary | Choose day-part labels, yesterday/today-style wording, calendar/date syntax, duration units, stale/late/due vocabulary, and UI display formats. |
| Concrete temporal thresholds | Choose stale-after thresholds, missed-summary thresholds, timeout windows, validity-window semantics, simultaneity/tie-break choices, and institution-specific procedural thresholds. |
| First temporal-firewall fixture package | Specify concrete friendly and adversarial fixture files for holder-known work time, stale notices, institutional records, TUI time rendering, scheduler-trigger leakage, debug-time leakage, and replay divergence. |
| Inventory/economy fixture package | Specify concrete items, stocks, lots, ledgers, balances, household pools, service capacities, units, denominations, split/merge/consume/spoil/transfer/pay/refuse/reimburse cases, and adversarial hidden-stock/custody cases. Do not bundle with temporal fixtures unless one gameplay feature truly needs both. |
| Affect/learning depth | Choose learning update rules, affect/trust dimensions, decay, thresholds, routine adaptation semantics, and evidence payload details. |
| Domain-pack bias assumptions | Define the vehicle for bias/social-harm assumptions, review criteria, allowed assumption fields, and validation failures. |
| Budget/fairness numeric targets | Choose deterministic budget limits, fairness formulas, windows, starvation thresholds, scheduler policy, LOD equivalence tolerance, and time-acceleration fairness targets. |
| Acceptance artifact template | Amend `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` to add staged-abstraction declaration fields: proves now, abstracts, must not fake, must not block, evidence preventing overclaiming, and failure diagnostics. |
| Authoring/compiler policy | Specify concrete schemas, policy rules, negative fixture layouts, and CI hooks for temporal, quantity, institutional, bias, and staged declarations. |

### 6.3 Upstream-route only

No foundation or architecture amendments are recommended by this report. Apparent upstream discomforts should be carried as open questions rather than patched inside execution.

---

## 7. Open questions

1. **Temporal vocabulary ownership.** Which future scoped spec owns the first concrete temporal vocabulary pack: a general time/calendar spec, first-playable fixture spec, or TUI time-control spec?
2. **Calendar and duration syntax.** What are the first concrete date/duration representations, and how much real-world calendar analogy is allowed?
3. **Staleness thresholds.** Which actor/institution/lead domains need explicit stale-after thresholds, and which should remain qualitative or source-procedure-defined?
4. **Same-time ordering.** Which future spec chooses simultaneity and tie-break semantics, and how are those surfaced in replay diagnostics without leaking to actor knowledge?
5. **Scheduler budget policy.** What budget windows, per-actor/per-class fairness metrics, and starvation thresholds are acceptable for first proof and second proof?
6. **Time acceleration.** What player-facing acceleration modes are eventually allowed, and what source-backed missed-event summaries are acceptable?
7. **LOD fidelity.** What minimum temporal ancestry and information ancestry must LOD summaries retain before actors or institutions can consume promoted state?
8. **Institution procedure packs.** Which first institution domain chooses office/due/late/queue/notice/payment/sanction semantics?
9. **Quantity schema.** Which inventory/economy spec owns lots versus unique items versus stocks versus ledgers, and how are household pools represented?
10. **Learning depth.** How deep should first-pass routine adaptation go before it becomes affect/trust modeling that requires its own scoped spec?
11. **Bias domain pack.** What social/institutional domain is used to prove practical bias without overclaiming general social simulation?
12. **Fixture bundling.** Which future gameplay feature, if any, genuinely requires temporal and quantity/economy proof in the same fixture package?
13. **Staged-abstraction template wording.** How should the acceptance-artifact template phrase staged declarations without becoming final gate text?
14. **Reference timing.** Should the temporal glossary/risk register pass occur immediately after execution amendment or after the first concrete temporal fixture spec?

---

## 8. References

### 8.1 Exact-commit repository sources

| Alias | Path | Exact raw URL |
|---|---|---|
| [TW-DOCS-README] | `docs/README.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/README.md |
| [TW-F-02] | `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md |
| [TW-F-03] | `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md |
| [TW-F-04] | `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md |
| [TW-F-05] | `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md |
| [TW-F-07] | `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md |
| [TW-F-08] | `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md |
| [TW-F-10] | `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md |
| [TW-F-12] | `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md |
| [TW-F-14] | `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md |
| [TW-F-00] | `docs/0-foundation/00_FOUNDATION_INDEX.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/00_FOUNDATION_INDEX.md |
| [TW-F-01] | `docs/0-foundation/01_PROJECT_CHARTER.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/01_PROJECT_CHARTER.md |
| [TW-F-06] | `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md |
| [TW-F-09] | `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md |
| [TW-F-11] | `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md |
| [TW-F-13] | `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md |
| [TW-A-00] | `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md |
| [TW-A-02] | `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md |
| [TW-A-03] | `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md |
| [TW-A-04] | `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md |
| [TW-A-05] | `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md |
| [TW-A-06] | `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md |
| [TW-A-08] | `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md |
| [TW-A-09] | `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md |
| [TW-A-10] | `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md |
| [TW-A-11] | `docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md |
| [TW-A-12] | `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md |
| [TW-A-13] | `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md |
| [TW-A-07] | `docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md |
| [TW-A-01] | `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md |
| [TW-A-14] | `docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md |
| [TW-E-00] | `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md |
| [TW-E-01] | `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md |
| [TW-E-02] | `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md |
| [TW-E-03] | `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md |
| [TW-E-04] | `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md |
| [TW-E-05] | `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md |
| [TW-E-06] | `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md |
| [TW-E-07] | `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md |
| [TW-E-08] | `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md |
| [TW-E-09] | `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md |
| [TW-E-10] | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md |
| [TW-E-11] | `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md |
| [TW-E-12] | `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md |
| [TW-E-13] | `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md |
| [TW-R-00] | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md |
| [TW-R-01] | `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/01_DESIGN_RISK_REGISTER.md |
| [TW-R-02] | `docs/3-reference/02_GLOSSARY.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/3-reference/02_GLOSSARY.md |
| [TW-SPEC-LEDGER] | `docs/4-specs/SPEC_LEDGER.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/SPEC_LEDGER.md |
| [TW-SPEC-README] | `docs/4-specs/README.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/README.md |
| [TW-SPEC-0003] | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md |
| [TW-REPORT-ARCH] | `reports/architecture-tier-alignment-research-report.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/architecture-tier-alignment-research-report.md |
| [TW-REPORT-COMPLETE] | `reports/foundations-completeness-determination-research-report.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/foundations-completeness-determination-research-report.md |
| [TW-REPORT-VERDICT] | `reports/verdict-on-foundations.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/verdict-on-foundations.md |
| [TW-REPORT-FOUND-ALIGN] | `reports/foundation-tier-alignment-research-report.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/reports/foundation-tier-alignment-research-report.md |
| [TW-ARCHIVE-SPEC-0031] | `archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md |
| [TW-ARCHIVE-SPEC-0032] | `archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md |
| [TW-ARCHIVE-SPEC-0028] | `archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md |
| [TW-ARCHIVE-REPORT-EXEC-0028] | `archive/reports/execution-tier-alignment-research-report.md` | https://raw.githubusercontent.com/joeloverbeck/tracewake/c70d119bca7663e4e1229dcd2012d57b5fe72d44/archive/reports/execution-tier-alignment-research-report.md |

### 8.2 External sources cited for rationale

| Alias | Source | URL |
|---|---|---|
| [EXT-SIMPY] | SimPy topical guide, "Time and Scheduling"; deterministic event queue behavior and same-time ordering. | https://simpy.readthedocs.io/en/latest/topical_guides/time_and_scheduling.html |
| [EXT-AZURE-EVENT-SOURCING] | Microsoft Azure Architecture Center, "Event Sourcing pattern"; append-only event store, replay, projections, audit/debug rationale. | https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing |
| [EXT-PROV-DM] | W3C PROV-DM, "The PROV Data Model"; provenance entities, activities, agents, derivation, attribution, and collections. | https://www.w3.org/TR/prov-dm/ |
| [EXT-JSON-SCHEMA] | JSON Schema documentation; structural validation keywords and schema-based validation posture. | https://json-schema.org/understanding-json-schema/reference/type |
| [EXT-OPA-REGO] | Open Policy Agent documentation, "Policy Language"; declarative policy evaluation over structured data. | https://www.openpolicyagent.org/docs/policy-language |
| [EXT-ANTITHESIS-DST] | Antithesis documentation, deterministic simulation testing overview; sealing nondeterminism and replayable debugging. | https://antithesis.com/docs/introduction/deterministic_simulation/ |
| [EXT-FAIR-SCHEDULING] | Shahsavari, Shirani, Erkip, "On the Fundamental Limits of Multi-user Scheduling under Short-term Fairness Constraints" (arXiv:1901.07719). | https://arxiv.org/abs/1901.07719 |
| [EXT-NIST-AI-RMF] | NIST AI Risk Management Framework. | https://www.nist.gov/itl/ai-risk-management-framework |
| [EXT-DATASHEETS] | Gebru et al., "Datasheets for Datasets." | https://arxiv.org/abs/1803.09010 |
| [EXT-ODD] | Grimm et al., "The ODD Protocol for Describing Agent-Based and Other Simulation Models"; model description, process overview, scheduling, and design concepts. | https://www.jasss.org/23/2/7.html |
| [EXT-TECH-DEBT] | Martin Fowler, "Technical Debt"; deliberate limitation and risk framing. | https://martinfowler.com/bliki/TechnicalDebt.html |
