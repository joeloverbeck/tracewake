# Tracewake Documentation

Tracewake documentation is layered authority. Read lower-numbered layers before higher-numbered layers.

| Layer | Role | Authority boundary |
|---|---|---|
| `0-foundation/` | Constitutional doctrine | Defines what Tracewake is and what it must never become. |
| `1-architecture/` | Stable subsystem contracts | Translates foundation doctrine into data-flow, authority, and subsystem contracts. |
| `2-execution/` | Execution sequencing and proof contracts | Defines gate order, certification requirements, fixtures, validation, and review artifacts. |
| `3-reference/` | Lookup and risk control | Provides compact terminology, checklist, source-discipline guardrails, and risk memory. |
| `4-specs/` | Narrow implementation or corrective specs | Applies live doctrine to scoped implementation work. |
| `archive/` | Historical record | Preserves completed specs, tickets, and reports as evidence, not live authority. |

Foundation outranks architecture. Architecture outranks execution. Execution outranks active specs. Reference summarizes and guards; it does not outrank the layers it points to. Archived specs and tickets do not outrank live doctrine. If execution conflicts with architecture or foundation, execution is wrong. If implementation is more convenient than the accepted gates, implementation is wrong. If a test proves only plausible behavior while bypassing holder-known provenance, the test is wrong.

## Authority boundary

This document owns the documentation map, authority order, and current live-document status. It does not certify implementation code, create tickets, define task breakdowns, redefine execution gates, or override any layer it points to.

## Depends on

- `docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`

## Current doctrine status

The foundation, architecture, execution, and reference tiers have been realigned into the current spine. The spec tier has now been realigned beneath that spine as well.

The live doctrine posture is:

1. **Foundation (`0-foundation/`)** is the constitutional baseline. All later tiers remain subordinate to it.
2. **Architecture (`1-architecture/`)** is the live design contract and current replacement for older architecture filenames.
3. **Execution (`2-execution/`)** is the live implementation and certification posture. Historical specs may have landed code, but they do not certify the current system until the execution gate sequence says so.
4. **Reference (`3-reference/`)** is the live review and vocabulary layer.
5. **Specs (`4-specs/`)** are realigned as the active spec layer, currently containing one live first-proof ontology / fixture package plus the tier index and ledger.

Archived implementation specs `0005`–`0008` remain historical evidence that drove the overhaul. They are not live authority and are not certification. `P0-CERT` remains the next major execution audit named by the execution tier; this docs realignment does not perform it.

The downstream-staleness report's prior note that `4-specs/` was subordinate-and-stale is resolved by the realigned spec-tier files emitted with this update. The report itself remains historical and is not rewritten here.

## Authority order

| Layer | May define | May not define |
|---|---|---|
| `0-foundation/` | Project identity, constitutional invariants, first playable proof, truth-firewall doctrine | Implementation shortcuts, phase-local exceptions, player privilege |
| `1-architecture/` | Authority ownership, data flows, subsystem boundaries, diagnostics, acceptance implications | Tickets, code tasks, speculative schedules, weakened doctrine |
| `2-execution/` | Certification sequence, phase gates, fixture expectations, audit criteria, review artifacts | New doctrine that weakens foundation or architecture; implementation certification without audit |
| `3-reference/` | Compact lookup aids, source-discipline checks, terminology control, and risk registers | Hidden policy, implementation plans, expanded doctrine, gate semantics, code certification |
| `4-specs/` | Narrow changes under live doctrine | Architecture replacement, constitutional amendments, acceptance weakening |
| `archive/` | Historical evidence and landed-work record | Live requirements where it conflicts with foundation, architecture, or execution |

## 0-foundation/ — constitutional layer

The foundation layer defines what Tracewake is and what it must not become: event-sourced causality, subjective epistemics, ordinary agents, no player privilege, no hidden-truth planning, TUI-first embodied play, deterministic replay, no scripting, LLM non-authority, scale discipline, and the actor-known transaction firewall.

- `00_FOUNDATION_INDEX.md` — map, authority, reading order, and anti-drift rules.
- `01_PROJECT_CHARTER.md` — product identity, Rust-first simulation, TUI-first playability, ordinary-life proof, and long-term posture.
- `02_CONSTITUTIONAL_INVARIANTS.md` — compact non-negotiable rules later layers must satisfy.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, replay, traceability, snapshots, randomness, and forensic causality.
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — typed claims/propositions, beliefs, observations, testimony, rumors, records, and knowledge flow.
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — symbolic agents, BDI separation, durable intentions, HTN procedures, bounded local planning, and debug traces.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action parity, affordances, ordinary life, survival needs, search, storage, work, travel, and basic economy.
- `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — households, roles, norms, records, notices, ownership/custody/access, fallible institutions, and future organizations.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first playability, actor-filtered view models, possession, notebooks, why-not explanations, and debug quarantine.
- `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — causal authoring, forbidden outcome chains, seeds, prehistory, records, notices, and no director logic.
- `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — multi-resolution simulation, promotion/demotion, regional processes, long history, and scale posture.
- `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, optional LLM rendering/parsing, prompt boundaries, validation, and LLM-disabled operation.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first village scope, missing-property proof, no-human gates, replay gates, TUI gates, and canonical regression seeds.
- `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — research decisions recorded as foundation constraints.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — constitutional truth firewall: truth may validate, but truth may not plan.

The whole folder is one foundation. Do not cherry-pick.

## 1-architecture/ — subsystem contract layer

The architecture layer translates foundation doctrine into subsystem contracts. Treat the whole `docs/1-architecture/` directory as a complete replacement set; architecture documents not listed here are retired and must not remain as live doctrine. Any implementation that is convenient but violates architecture is wrong.

- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — architecture map, replacement/retirement rule, universal conformance questions, and maintenance rules.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — Rust-first authority boundaries, workspace seams, dependency direction, content/domain-pack authority, and forbidden inversions.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — event log, replay, projections, save manifests, random stream discipline, schema versioning, and migration failure.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts, truth firewall, provenance packets, context sealing, debug comparison, and contamination gates.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — ordinary action proposal pipeline, scheduler limits, validation truth, failure semantics, affordances, and modeled feedback.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — canonical actor decision transaction, needs, commitments, intentions, routines, HTN method selection, local planning, and stuck diagnostics.
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — typed propositions, observations, beliefs, memories, records, rumors, leads, traces, and source-bound information flow.
- `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — structured speech acts, speaker/listener contexts, language rendering/parsing, and optional LLM boundary.
- `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — institution-known transaction, households, norms, roles, proof, records, notices, orders, sanctions, and procedural fallibility.
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — missing-property first proof, settlement ontology, spaces, property, food, sleep, work, and local economy.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — possession, embodied TUI, actor-legible failure views, debug-only truth, view models, and client boundaries.
- `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — questless incidents, leads, notices, reports, observer-only story sifting, and second-proof boundaries.
- `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — LOD as ontology, aggregate truth, promotion/demotion, regional processes, ancestry, and prehistory.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance artifacts, anti-contamination tests, no-human/TUI/replay gates, diagnostics, and review checklists.
- `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` — research distilled into decisions, consequences, and forbidden misreads.

## 2-execution/ — execution and certification layer

The execution layer defines gate order and proof obligations. It does not certify existing code by describing it; it defines what a future audit or implementation spec must prove.

- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — execution map, retirement rule, universal authority posture, canonical gate names, and canonical observation obligations.
- `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — status of archived specs, current code boundary, and certification posture.
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — first proof identity, current baseline status, and acceptance contract.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order, phase sequencing, and the rule that Phase 4 is blocked until certification gates pass.
- `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — truth-firewall execution checks and mandatory anti-contamination gates for every future spec.
- `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — actor transaction, scheduler, proposal, validation, and direct-dispatch audit criteria.
- `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — needs, routines, durable intentions, ordinary-life proof, and no-human day certification.
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — epistemic proof, possession parity, view-model filtering, and debug quarantine.
- `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — authoring contracts, schema/provenance validation, and no outcome-chain data.
- `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — golden fixture families, adversarial scenarios, and deterministic replay acceptance.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — testing, diagnostics, metrics, trace artifacts, and review evidence.
- `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — Phase 4 entry contract and institution/record/wrong-suspicion lock.
- `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — locked deferral contract for notices, travel, regional scale, LOD, and second proof.
- `13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` — execution-level research decisions and forbidden misreads.

The next major execution move is the post-0008 baseline certification gate, not automatic Phase 4 expansion.

## 3-reference/ — compact lookup and watchlist layer

Reference documents stay compact. Use them to preserve terminology, source discipline, checklist discipline, and design-risk memory. Do not use them for architecture replacement, implementation plans, gate definition, code certification, or new doctrine.

- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — compact operating index and review guardrails for future coding, spec, fixture, schema, prompt, and review sessions, including exact-source discipline and gate-code lookup posture.
- `01_DESIGN_RISK_REGISTER.md` — operational watchlist of unresolved, recurring, or easily regressing design risks and relapse modes, including the post-0008 risk that archived specs `0005` through `0008` are treated as post-overhaul certification.
- `02_GLOSSARY.md` — prescriptive terminology control across truth, holder-known context, observation, belief, memory, claim, speech act, trace, record, institutional fact, projection, debug truth, source discipline, deferred terms, and forbidden core ontology.

No reference files have been added, removed, merged, or renamed in the realignment. The three-file split remains because each file has a distinct lookup job and no extra symmetry file is needed.

## 4-specs/ — active spec layer

Spec documents are the lowest doctrine tier. They operationalize the higher tiers; they do not replace them.

- `4-specs/README.md` — spec-tier index, reading order, and future-spec rules.
- `4-specs/SPEC_LEDGER.md` — active spec ledger, source discipline, archived-spec posture, and next known execution move.
- `4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — live first-proof missing-property village ontology and fixture contract, realigned beneath the current foundation / architecture / execution / reference spine.

The spec layer was compacted during realignment: the former pre-overhaul companion-amendment and research-note material is no longer part of the live spec map, and the village ontology file no longer carries obsolete pre-implementation phase framing.

## archive/ — historical evidence

`archive/` preserves completed specs, tickets, reports, and prior implementation-history evidence. It is useful for understanding why the current doctrine hardened. It is not live authority when it conflicts with foundation, architecture, or execution.

Archived specs `0005` through `0008` are especially important as history: they landed Phase 3A-related implementation work and exposed why stronger holder-known, truth-firewall, no-human, replay, fixture, and diagnostic gates were needed. They do not certify the current implementation under the post-overhaul doctrine.

## Maintenance rules

When adding or changing documents:

- preserve authority order: foundation before architecture before execution before reference before specs;
- restart numbering inside each folder rather than continuing numbers across tiers;
- avoid stale filenames in indexes, examples, and reports;
- treat every world-affecting human command as an ordinary actor/process proposal under equivalent conditions;
- require holder-known provenance before cognition, procedure decisions, speech interpretation, view-model affordance selection, leads, institution procedure, or LOD promotion;
- keep debug truth visibly non-diegetic and structurally quarantined;
- keep reference documents short enough to serve as lookup tools;
- prefer typed claims, typed diagnostics, sealed context packets, and replayable records over display strings;
- name execution gate codes only as cross-references to `docs/2-execution/`;
- treat archived specs and tickets as history unless live doctrine explicitly promotes and certifies the relevant claim.

Tracewake's recurring test is simple: every feature must preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first playability, validation/replay, the holder-known truth firewall, and no simulation fact born from prose.
