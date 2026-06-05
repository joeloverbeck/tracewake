# Tracewake Documentation

Tracewake's documentation is organized as layered authority. Read earlier layers before later layers.

```text
0-foundation/   constitutional doctrine
1-architecture/ subsystem design contracts
2-execution/    implementation and validation planning
3-reference/    compact lookup and active watchlists
```

Later tiers depend on earlier tiers. If execution conflicts with architecture or foundation, execution is wrong. If implementation is more convenient than the accepted execution gates, implementation is wrong. Reference documents are lookup aids, not new doctrine.

The foundation and architecture folders open with a `00_` index document; execution and reference start at `01_`. File numbers are local to each folder.

## 0-foundation/ — constitutional layer

The foundation layer defines what Tracewake is and what it must not become. These documents control product identity, causality, claims and epistemics, ordinary agents and action, possession and TUI-first playability, no-scripting policy, scale posture, the LLM/language boundary, and acceptance doctrine.

- `00_FOUNDATION_INDEX.md` — map, authority, reading order, and anti-drift rules.
- `01_PROJECT_CHARTER.md` — identity, product priorities, first village, long-term posture, and hard direction.
- `02_CONSTITUTIONAL_INVARIANTS.md` — compact non-negotiable rules later layers must satisfy.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, traces, replay, snapshots, compaction, boundary inputs, and forensic causality.
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — typed claims/propositions, beliefs, memories, observations, testimony, lies, rumors, records, and knowledge flow.
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — symbolic agents, BDI separation, durable intentions, HTN procedures, bounded local planning, utility boundaries, and debug traces.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — ordinary action parity, affordances, survival substrate, search, storage, work, travel, and basic economy.
- `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — households, roles, norms, records, notices, ownership/custody/access, institutional fallibility, and future organizations.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first playability, actor-filtered view models, possession, notebooks, why-not explanations, and debug separation.
- `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — causal authoring, forbidden authored outcome chains, seeds, authored prehistory, records, notices, and no director logic.
- `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — honest multi-resolution simulation, promotion/demotion, regional processes, long history, and future scale.
- `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, optional LLM rendering/parsing, validated extraction, prompt boundaries, and LLM-disabled operation.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first village scope, missing-property proof, no-human gates, replay gates, TUI gates, and canonical regression seeds.
- `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — decisions drawn from research and precedent, recorded as design constraints.

## 1-architecture/ — subsystem design layer

The architecture layer translates foundation doctrine into subsystem contracts. It defines boundaries, data flows, validation responsibilities, and anti-patterns for the Rust-first implementation.

- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — folder map, foundation conformance, universal feature questions, and architecture-layer maintenance rules.
- `01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` — Rust-first authority boundaries, logical subsystem seams, dependency direction, side-effect rules, and forbidden authority inversions.
- `02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` — unified command/proposal/action/affordance/scheduler contract for all world-affecting behavior.
- `03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` — event log, causal graph, replay, projection rebuild, snapshots, event schema migration, random streams, and save-package contract.
- `04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` — entity/component state model, authored content, domain packs, scenario seeds, schemas, fixtures, and content validation.
- `05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` — typed claims, observation, belief, memory, traces, rumors, stale information, absence-as-evidence, and actor-knowledge filtering.
- `06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — structured speech acts, deterministic templates, optional future LLM parsing/rendering, and validation boundary.
- `07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` — symbolic agent cognition, needs, values, projects, durable intentions, HTN routines, bounded local planning, planner traces, and agent detail tiers.
- `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — households, roles, norms, records, reports, notices, institutional facts, evidence thresholds, procedure failure, bias, and jurisdiction.
- `09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` — first-slice village, homes, rooms, doors, containers, sleep, food, work, simple economy, travel, and settlement causality.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — human controller binding, possession, actor-filtered embodied view models, actor notebook, TUI harness, debug views, and future graphical boundary.
- `11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md` — questless incidents, requests, obligations, leads, actor-known summaries, and observer-only story sifting.
- `12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` — dynamic level of detail, regional processes, boundary events, promotion/demotion, summary events, world history, and scale staging.
- `13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` — validation gates, no-human runs, deterministic replay checks, observability artifacts, metrics, and acceptance levels.
- `14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — architecture-relevant research decisions and source notes.

## 2-execution/ — implementation and validation planning layer

The execution layer defines the order of work, phase gates, first proof, fixtures, validation expectations, and what must remain deferred. It does not replace foundation doctrine or architecture contracts.

- `01_EXECUTION_CHARTER_AND_SCOPE_DECISIONS.md` — execution authority, first proof, second proof, strict phase ladder, TUI-first policy, no-human policy, LLM deferral, data posture, and anti-scope-creep rules.
- `02_STRICT_PHASE_LADDER_AND_IMPLEMENTATION_ORDER.md` — Phase 0 through Phase 4 dependency order, phase shape, gates, exit criteria, and second-proof unlock rules.
- `03_FIRST_PROOF_THE_MISSING_PROPERTY_VILLAGE.md` — identity, target scale, first domain, map scope, actor roster, required systems, miracle chain, scenarios, definition of done, and failure conditions.
- `04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_DESIGN.md` — paper ontology, primitive vocabularies, manual causal chains, TUI sketches, view-model contracts, no-scripting review, and fixture gates.
- `05_PHASE_1_RUNNABLE_KERNEL_TUI_AND_EVENT_LOG.md` — runnable physical/event/action/TUI spine, event log, basic replay, local interaction, debug attach, and no-human advance.
- `06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` — observation, belief, expectation contradiction, actor-known filtering, embodied/debug view split, possession parity, and notebook projection.
- `07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_DAILY_SIM.md` — hunger, fatigue, safety, sleep, eating, work, households, routines, intentions, interruptions, TUI time controls, no-human day, metrics, and planner traces.
- `08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md` — households, local authority, roles, report action, incident ledger, norms, suspicion scoring, questioning/watch procedure, institution debug inspector, and first-proof final acceptance.
- `09_DATA_AUTHORING_SCHEMAS_AND_GOLDEN_FIXTURES.md` — authoring posture, validation pipeline, content manifest, core schema shapes, and golden fixtures.
- `10_TESTING_VALIDATION_DEBUGGING_AND_METRICS.md` — testing matrix, per-phase minimums, properties, golden scenarios, fuzzing, TUI/view-model tests, epistemic leakage tests, institution tests, replay tests, LLM-boundary tests, debug requirements, and metrics.
- `11_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md` — post-first-proof notice lifecycle, stale leads, route travel, companion recruitment, proof/payment, regional expansion, and second-proof gates.
- `12_RESEARCH_DECISIONS_AND_EXECUTION_SOURCE_NOTES.md` — execution-specific research decisions and source notes.

## 3-reference/ — compact lookup and watchlist layer

The reference layer is deliberately small. It should help maintain consistency without duplicating the foundation, architecture, or execution layers.

- `01_DESIGN_RISK_REGISTER.md` — living watchlist of unresolved or recurring design risks that can threaten Tracewake's doctrine during implementation, authoring, validation, scaling, or presentation work.
- `02_GLOSSARY.md` — canonical terminology-control document for domain, architecture, schema, test, TUI, prompt, and documentation vocabulary.

Reference documents should remain compact, stable, and low-duplication. Do not use the reference layer for essays, design constitutions, implementation plans, or new doctrine. If a reference document grows into policy, move the policy to the correct authority layer and leave only the lookup/watchlist material here.

## Maintenance rules

When adding or changing documents:

- preserve layer authority: foundation before architecture before execution before reference;
- restart numbering inside each folder rather than continuing numbers across tiers;
- avoid brittle cross-references to exact invariant numbers unless unavoidable;
- avoid stale filenames in indexes and examples;
- keep reference documents short enough to serve as lookup tools;
- prefer source-bound concepts over synonyms that imply quests, protagonist privilege, omniscience, scripts, or LLM authority.

Tracewake's recurring test is simple: every feature must preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first playability, validation/replay, and no simulation fact born from prose.
