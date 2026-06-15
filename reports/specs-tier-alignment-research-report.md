# Specs-Tier Alignment Research Report

**Target repository:** `joeloverbeck/tracewake`

**Target commit:** `cda3325b0777f25101c9a04af3daeef24913f137` (`cda3325`)

**Target path:** `reports/specs-tier-alignment-research-report.md`

**Status:** NEW. The live-path report is absent from the uploaded `cda3325` manifest; the archived epoch-1 namesake under `archive/reports/` is historical context only.

**Scope:** recommendation report for `docs/4-specs/*`: `README.md`, `SPEC_LEDGER.md`, `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`, and `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.

**Deliverable type:** recommendation report only; not ratified wording; not a numbered spec.

**Determination note:** The specs tier is not a doctrine-amendment tier. It operationalizes higher-tier doctrine and cannot define doctrine. A thin concrete amend set is therefore the correct outcome: one near-term `0003` template edit, boundary-awareness no-change verdicts for `README.md`, `SPEC_LEDGER.md`, and `0001`, plus a route-forward backlog for future scoped implementation specs.

**Limitation:** This report does not verify that `cda3325` is the current `main`. It uses the user-supplied commit as the target of record and analyzes only files fetched by exact commit URL from `joeloverbeck/tracewake`.

## Disposition table

| Finding | Target doc / future home | Verdict | One-line basis |
|---|---|---:|---|
| Staged-abstraction declaration fields | `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | amend | E10/F-13 route one concrete template edit: declare proves-now, abstracts, must-not-fake, must-not-block, overclaim-prevention evidence, and failure diagnostics. |
| Specs-tier authority posture | `docs/4-specs/README.md` | boundary-awareness — no change | The README already states specs are the lowest live doctrine tier and may operationalize but not amend, replace, or weaken higher-tier doctrine. |
| Spec ledger campaign lineage and source discipline | `docs/4-specs/SPEC_LEDGER.md` | boundary-awareness — no change | The ledger already records exact-commit/source discipline, `0003` template status, archived-spec historical posture, and campaign lineage through `0026`–`0033`. |
| First-proof ontology/fixture contract | `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | boundary-awareness — no change | `0001` remains a subordinate first-proof ontology/fixture contract; no concrete epoch-2 drift requires editing it in this pass. |
| Concrete temporal vocabulary | future scoped implementation spec | route-forward | Foundation/reference must not choose day-part, due/late/closed/stale strings, date syntax, duration units, or UI clock phrasing. |
| Concrete temporal thresholds and ordering rules | future scoped implementation spec | route-forward | Stale-after thresholds, validity windows, simultaneity, queue tie-breaks, deadline semantics, and office-hour representation belong to scoped specs under `INV-112` and E04/E07/E10. |
| First temporal-firewall fixture package | future scoped implementation spec | route-forward | E04/E07/E09 route friendly/adversarial proof fixtures for holder-known time, raw-clock contamination, debug-time leakage, procedural labels, and temporal rendering. |
| Inventory/economy fixture package | future scoped implementation spec | route-forward | A09/E08/E09 route quantity/fungibility/custody fixtures; this package must not be bundled with temporal-firewall fixtures unless one concrete feature genuinely needs both. |
| Affect/learning depth | future scoped implementation spec | route-forward | A05/A06/E06 lock only bounded affect and source-backed learned expectations; dimensions, update rules, trust/decay, and generalization remain future work. |
| Domain-pack bias vehicle | future scoped implementation spec | route-forward | A08/E08/E11 require explicit, reviewable domain-pack assumptions and modeled bias/procedure; the concrete vehicle is not a reference/specs-tier doctrine decision now. |
| Budget/fairness numeric targets | future scoped implementation spec | route-forward | A13/E05/E10/E12 require observability of deferral/starvation/fairness but defer numeric budgets, windows, formulas, and acceptance targets. |
| Authoring/compiler policy | future scoped implementation spec | route-forward | E08/E09/E10 need concrete schema/policy/negative-fixture rules for temporal, quantity, bias, and staged-proof content; current pass cannot choose the policy engine. |
| TUI/play-loop fixture spec | future scoped implementation spec | route-forward | Completeness routing requires proof that play remains legible without truth/objective leaks; concrete transcript/fixture contracts belong to a later scoped spec. |

## Method & provenance ledger

Requested repository: `joeloverbeck/tracewake`

Target commit: `cda3325b0777f25101c9a04af3daeef24913f137`

Freshness claim: user-supplied target commit only; not independently verified as latest `main`.

Manifest role: path inventory only.

Repository metadata used: no.

Default-branch lookup used: no.

Branch-name file fetch used: no.

Code search used: no.

Clone used: no.

URL fetch method: exact `raw.githubusercontent.com` URLs of the form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/<manifest path>
```

Before each repository file was used, the requested URL was checked for owner `joeloverbeck`, repository `tracewake`, commit `cda3325b0777f25101c9a04af3daeef24913f137`, and a path present in the uploaded manifest.

Contamination observed: no.

Connector/tool namespace trusted as evidence: no.

The predecessor reports and archived specs were read only as files fetched from `joeloverbeck/tracewake` at the target commit. Any older commit strings inside those historical documents are historical content, not fetch targets and not freshness evidence.


## Specs-tier authority posture

The specs tier is subordinate to foundation, architecture, execution, and reference. It may operationalize higher-tier doctrine but may not amend constitutional invariants, replace architecture, define gate semantics, weaken execution gates, or promote archived history into current certification. That posture is already stated in `docs/4-specs/README.md` and `SPEC_LEDGER.md`.

This report therefore refuses to invent new doctrine, new gate labels, new observation-obligation codes, or final template text. It identifies homes and substance only.

## Settled context not re-commissioned

This report treats epoch-1 as already landed in the live `cda3325` docs. It does not re-open, re-derive, rename, or strengthen the `EMERGE-OBS` glossary term, the R-26/R-27/R-28/R-29 acceptance-evidence-honesty risk cluster and archive cross-link, or the `0003` evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical, and certification-use fields.

It also does not re-open the settled anti-contamination themes: event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, validation/replay, and observer-only/non-certifying emergence evidence. Those are baseline guardrails for this epoch-2 delta.

The possession-bind perception question remains open. If it is ever adopted, the bind-time perception must be a modeled event/channel for the actor, never a transfer of human/player knowledge into the actor.


## Finding 1 — `0003` needs staged-abstraction declaration fields

**Driver.** Execution `00` states staged proof is allowed only when declared and bounded: the artifact must say what it proves now, what it deliberately abstracts, what it must not fake, what it must not block, what evidence prevents overclaiming, and what failures should diagnose. E10 repeats this as the consolidated staged-abstraction review-artifact requirement. The execution report §6.2 and archived spec `0033` route the concrete `0003` template edit to the specs tier.

**Current coverage.** `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` already carries the epoch-1 evidence-honesty fields: evidence status, fingerprint scope, behavior witness, replay/provenance evidence, sampling/exhaustiveness, pending/historical classification, certification-use limits, and `EMERGE-OBS` observer-only handling. Those fields remain baseline and should not be rewritten or re-commissioned.

**Tier-fit verdict.** Amend `0003`, but only as a template addition. Do not mint a new gate, a new obligation code, or final paste-ready field wording in this report.

**Recommendation.** In `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, add a staged-abstraction declaration area that requires future acceptance artifacts to state the substance of the following categories:

- what the artifact proves now;
- what it deliberately abstracts;
- what the implementation or proof must not fake while abstracting;
- what future work the abstraction must not block;
- what evidence prevents overclaiming from the current artifact;
- what failures or diagnostics should distinguish “not implemented yet,” “intentionally abstracted,” “implemented but broken,” and “overclaimed.”

This addition should sit near the existing evidence-honesty/provenance/certification-use material, because its job is to prevent false certification. It should say the declaration certifies nothing by itself and remains subordinate to execution `10` and the reference risk/term update. It should not import final glossary wording or create a numbered spec package.

External research note: technical-debt prior art supports the general practice of making intentional incompleteness visible, but Tracewake’s controlling rule is execution E10/F-13, not generic debt terminology.

## Finding 2 — `README.md` is boundary-aware and needs no epoch-2 edit

**Driver.** The brief asks for boundary-awareness conformance checks on specs-tier authority posture.

**Current coverage.** `docs/4-specs/README.md` already states that specs are the lowest live doctrine tier, that future specs must cite exact governing docs, and that specs may operationalize higher-tier doctrine but must not amend, replace, or weaken it. It also distinguishes live specs from archived/historical material and flags `P0-CERT` as a future implementation audit.

**Tier-fit verdict.** Boundary-awareness — no change.

**Recommendation.** Do not edit `docs/4-specs/README.md` for this pass. Its existing posture is exactly what epoch-2 requires: specs do not ratify temporal authority or completeness doctrine; they operationalize future scoped packages under the upper tiers.

## Finding 3 — `SPEC_LEDGER.md` is boundary-aware and needs no epoch-2 edit

**Driver.** The brief asks for a conformance check and notes that the ledger should record the campaign lineage.

**Current coverage.** `docs/4-specs/SPEC_LEDGER.md` already records that the ledger is navigation/source discipline rather than product doctrine; that specs are subordinate; that manifests are path inventory only; that branch/default/metadata/search snippets are not proof; that `0003` is a review-artifact template rather than an active spec package; and that archived specs remain history. Its archived-spec table records the epoch-1 and epoch-2 lineage through `0026`–`0033`, including `0031`, `0032`, and `0033`.

**Tier-fit verdict.** Boundary-awareness — no change.

**Recommendation.** Do not edit `docs/4-specs/SPEC_LEDGER.md` in this pass. A future specs amendment that actually changes `0003` may update the ledger if the repo’s own amendment process requires a historical entry, but this recommendation report does not require it.

## Finding 4 — `0001` remains boundary-aware and needs no epoch-2 edit

**Driver.** The brief asks for a conformance check on `0001`.

**Current coverage.** `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` declares itself a live first-proof ontology and fixture contract, not a certification artifact or doctrine amendment. It remains subordinate to foundation, architecture, and execution. It covers the first-proof village ontology, missing-property fixtures, and narrow action/fixture surfaces, including ordinary-life procedure hooks that are useful boundary context but not concrete epoch-2 temporal doctrine.

**Tier-fit verdict.** Boundary-awareness — no change.

**Recommendation.** Do not edit `0001` in this pass. Its current first-proof scope does not define concrete temporal vocabulary, thresholds, affect/learning depth, bias vehicle, or economy formulas. It can remain as baseline while future scoped specs add temporal-firewall, inventory/economy, bias, budget/fairness, authoring/compiler, and TUI/play-loop packages.

## Backlog register — future scoped implementation specs

The following items are future scoped specs, not current doctrine edits. They should be split unless one concrete gameplay feature truly needs combined proof. The temporal-firewall and inventory/economy packages especially must not be bundled merely because both are epoch-2 outputs.

### Concrete temporal vocabulary

**Owns.** Surface vocabulary for day-parts, dates, durations, due/late/closed/stale/fresh labels, office-hour phrasing, player-visible clock strings, embodied notebook/date phrasing, notices, logs, and speech/rendering conventions.

**Must not bundle.** Threshold math, queue/simultaneity algorithms, inventory/economy lineage, domain-pack bias assumptions, or TUI play-loop proof unless the vocabulary is needed for a specific fixture.

**Governing contracts.** `INV-112`, foundation `03`, A03/A10/A11/A12, E04/E07/E10/E12, and the reference glossary/risk update.

### Concrete temporal thresholds and ordering rules

**Owns.** Stale-after thresholds, validity-window cutoffs, simultaneity/tie-break rules, deadline/lateness calculations, office-hour representation, scheduler queue rules, and concrete acceleration/cadence parameters when they become implementation commitments.

**Must not bundle.** Surface wording, affect/learning update rules, inventory denomination/quantity schemas, or domain-pack bias semantics unless needed for one concrete acceptance feature.

**Governing contracts.** `INV-112`, foundation `03`, A02/A04/A13, E04/E05/E07/E10/E12.

External research note: SimPy’s deterministic event scheduling prior art supports treating same-time ordering as an implementation decision that requires a concrete spec, not a reference-tier doctrine shortcut.

### First temporal-firewall fixture package

**Owns.** Positive and adversarial fixtures proving holder-known/institution-known temporal claims, scheduler time separation, raw wall-clock rejection, debug-time quarantine, procedural-time labels, temporal rendering source links, stale/fresh label source discipline, and LOD/time-acceleration ancestry when in scope.

**Must not bundle.** Inventory/economy proof unless a fixture’s gameplay feature genuinely requires both time and quantity lineage. Must not certify all future temporal vocabulary or thresholds by implication.

**Governing contracts.** E04/E07/E09/E10, with A02/A03/A04/A10/A12/A13 and `INV-112`.

### Inventory/economy fixture package

**Owns.** Concrete fixtures for unique objects, countable lots, divisible stocks, capacities, custody transfer, ledger entries, split/merge, consumption/spoilage/payment/debt, visibility/knowledge of property, and display-total limitations.

**Must not bundle.** Temporal-firewall fixtures unless one gameplay feature genuinely needs both; bias vehicle; money/economy formulas beyond the package’s stated scope.

**Governing contracts.** A09, E08, E09, E10, plus the reference quantity/lineage terms and risk memory.

External research note: W3C PROV-DM supports the general review habit of tracking derivation/responsibility/collections, while Tracewake’s event-sourced custody rules remain the governing contract.

### Affect/learning depth

**Owns.** Concrete affect dimensions, salience intensity, update rules, trust/decay, adaptation scope, generalization rules, memory/expectation interaction, and proof fixtures for anti-truth-cache learning.

**Must not bundle.** Bias vehicle, temporal thresholds, or budget formulas unless a single behavior feature requires combined proof. Must not allow affect or learned expectations to reveal truth, bypass planning, force drama, or overwrite beliefs.

**Governing contracts.** A05/A06, E04/E06/E10, and reference bounded-affect/learned-expectation terms and risks.

External research note: social-simulation emotion-model literature confirms that affect models can vary widely and become complex; this supports future scoped ownership rather than reference-tier overcommitment.

### Domain-pack bias vehicle

**Owns.** Concrete representation of domain-pack assumptions, social categories, credibility/access/norm/procedure defaults, institution-specific bias/error/corruption conditions, review metadata, and negative fixtures for author-prejudice smuggling.

**Must not bundle.** Kernel moral categories, universal harm scores, objective suspicion labels, or unreviewable author prose. Must not decide broad social taxonomy beyond what the domain pack explicitly owns.

**Governing contracts.** A08, E08, E11, E10, and reference practical-bias/social-harm terms and risks.

External research note: NIST AI RMF risk language and datasheet-style documentation support explicit assumption documentation and reviewability; they do not define Tracewake’s categories or doctrine.

### Budget/fairness numeric targets

**Owns.** Concrete scheduler/planner budget numbers, fairness windows, starvation thresholds, actor/region/LOD class accounting, degradation policy, time-acceleration budget equivalence, and acceptance evidence for deferral/skips.

**Must not bundle.** Temporal vocabulary, affect/learning depth, or TUI transcripts unless needed for a single measurable feature. Must not use aggregate success as the sole fairness proof.

**Governing contracts.** A04/A13, E05/E10/E12, and reference budget/fairness terms and risks.

### Authoring/compiler policy

**Owns.** Concrete schemas, compiler validations, fail-closed diagnostics, policy-as-code checks, CI guardrails, and negative fixtures for temporal claims, quantity/custody, institution-known procedure, domain-pack assumptions, staged-abstraction fields, and proof-bearing content.

**Must not bundle.** The policy engine decision with doctrine; a JSON schema, custom validator, or policy engine can enforce doctrine but cannot create it. Must not permit backward-compatible alias paths or silent coercions where execution requires fail-closed behavior.

**Governing contracts.** E08/E09/E10, A08/A09/A13, reference authoring/compiler risk memory, and specs authority posture.

External research note: JSON Schema and policy-as-code tools such as OPA are useful implementation prior art for validation/policy structure, but a future spec must choose concrete tools under Tracewake’s authority order.

### TUI/play-loop fixture spec

**Owns.** Concrete TUI/play-loop fixtures showing that player surfaces remain playable without truth leaks: embodied view snapshots, notebook/lead/status examples, explanation affordances, debug quarantines, transcript evidence, and adversarial checks against quest/objective leaks.

**Must not bundle.** Full temporal vocabulary or all UI clock formats unless the fixture is specifically about time; full social-bias domain packs unless the gameplay feature requires it. Must not solve playability by leaking hidden objectives, omniscient labels, or debug truth.

**Governing contracts.** A10/A11/A13, E07/E10, the completeness determination’s play-legibility route, and reference play-legibility dual-relapse risk.

## Forward-routing appendix

This is a terminal-tier realignment report. Nothing routes further down as a documentation-authority cascade. Remaining work routes only to maintainer/reassess decisions and future implementation specs under `docs/4-specs/`.

Future specs should be scoped and non-doctrinal. They must cite the governing foundation, architecture, execution, and reference sources; declare what they prove now; declare abstractions and non-certification limits; and avoid bundling unrelated packages for convenience.

The possession-bind perception owner question remains carried, not decided. If adopted, it must be represented as a modeled event/channel for the actor and not as a human/player knowledge transfer.

## Open questions

- Which future scoped spec owns first concrete temporal vocabulary, and which owns temporal thresholds/order rules if separated?
- What is the minimum calendar/date/duration vocabulary needed for first temporal fixtures without overcommitting the world model?
- What stale-after/validity-window thresholds belong to first implementation, and how should they be evidenced?
- What same-time ordering/tie-break discipline should implementation expose, and at what tier should it be specified?
- What is the minimum inventory/economy representation for missing-property/ordinary-life proof without collapsing custody lineage?
- How deep should affect and learned expectations go before Phase-4 social simulation requires a dedicated affect/learning spec?
- Which maintainer owner should define the first domain-pack bias vehicle and its assumption-review packet?
- Which numeric budget/fairness targets are acceptable for first proof, and how should starvation be surfaced?
- Should temporal-firewall and inventory/economy fixture packages ever be combined, or should they stay separate until a feature explicitly requires both?
- What exact `0003` staged-abstraction template wording should the repo’s amendment process choose?
- The possession-bind perception owner question remains open as stated above.

## References

### Exact-commit repository sources

Primary governing sources:
- `docs/README.md` — authority order and layer boundaries.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — constitutional invariants, including `INV-112`.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — temporal-authority section.
- `docs/1-architecture/02`, `03`, `04`, `05`, `06`, `08`, `09`, `10`, `11`, `12`, `13` — architecture temporal/completeness contracts.
- `docs/2-execution/00`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12` — execution routing and proof homes.
- `docs/4-specs/README.md`, `SPEC_LEDGER.md`, `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`, and `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — target specs-tier baseline.
- `reports/execution-tier-alignment-research-report.md` — freshest routing seed, especially §6.2 and F-13.
- `archive/specs/0031`, `0032`, `0033` — ratified epoch-2 amendments.
- `archive/specs/0029`, `0030` and `archive/reports/reference-tier-alignment-research-report.md`, `archive/reports/specs-tier-alignment-research-report.md` — settled epoch-1 boundary context.

## Repository source ledger

All repository file sources below were fetched through exact raw URLs under the target owner, repository, commit, and manifest path. The uploaded manifest was used only to confirm that these paths existed in the stated inventory. The two report output paths were absent from the manifest and are therefore new report files, not replacements.

- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/execution-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/architecture-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/foundation-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/foundations-completeness-determination-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/reports/verdict-on-foundations.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0031_FOUNDATION_TEMPORAL_AUTHORITY_DOCTRINE_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0032_ARCHITECTURE_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0033_EXECUTION_TIER_TEMPORAL_AUTHORITY_AND_COMPLETENESS_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0029_REFERENCE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/specs/0030_SPECS_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/reports/reference-tier-alignment-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/cda3325b0777f25101c9a04af3daeef24913f137/archive/reports/specs-tier-alignment-research-report.md


### External sources consulted

- SimPy, “Time and Scheduling” — used only as prior art for deterministic discrete-event scheduling, same-time tie-break discipline, and why exact time values belong to implementation rather than terminology: https://simpy.readthedocs.io/en/latest/topical_guides/time_and_scheduling.html
- W3C PROV-DM — used only as prior art for provenance, derivation, responsibility, and extensibility vocabulary: https://www.w3.org/TR/prov-dm/
- W3C/OGC OWL-Time — used only as prior art for separating temporal concepts, intervals/durations/positions, and concrete temporal reference systems: https://www.w3.org/TR/owl-time/
- NIST AI Risk Management Framework page — used only as prior art for risk review language about harms to individuals, organizations, and society; it does not define Tracewake doctrine: https://www.nist.gov/itl/ai-risk-management-framework
- Gebru et al., “Datasheets for Datasets” — used only as prior art for documenting assumptions, composition, collection/process, and intended uses of content/domain packs: https://arxiv.org/abs/1803.09010
- JSON Schema, “Creating your first schema” — used only as prior art for future authoring/compiler validation vocabulary: https://json-schema.org/learn/getting-started-step-by-step
- Open Policy Agent documentation — used only as prior art for future policy-as-code/CI enforcement options: https://www.openpolicyagent.org/docs
- Martin Fowler, “Technical Debt” — used only as prior art for making deliberate staged incompleteness visible instead of letting it masquerade as completion: https://martinfowler.com/bliki/TechnicalDebt.html
- Bourgais, Taillandier, and Vercouter, “Emotion Modeling in Social Simulation” — used only as prior art showing that affect modeling can become deep and model-specific, supporting a cautious reference-altitude term rather than premature Tracewake doctrine: https://www.jasss.org/21/2/5.html

