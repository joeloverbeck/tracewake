# Tracewake Documentation

Tracewake's documentation is organized as layered authority. Read earlier layers before later layers.

```text
0-foundation/   constitutional doctrine
1-architecture/ subsystem design contracts
2-execution/    implementation and validation planning
3-reference/    compact lookup and active watchlists
```

Later tiers depend on earlier tiers. If execution conflicts with architecture or foundation, execution is wrong. If implementation is more convenient than the accepted execution gates, implementation is wrong. Reference documents are lookup aids, not new doctrine.

Each folder restarts numbering at `01_`. File numbers are local to the folder.

## 0-foundation/ — constitutional layer

The foundation layer defines what Tracewake is and what it must not become. These documents control product identity, causality, epistemics, possession, TUI-first playability, no-scripting policy, scale posture, domain boundary, and acceptance doctrine.

- `01_PROJECT_CHARTER.md` — product identity, priorities, first playable target, and non-negotiable direction.
- `02_FOUNDATIONAL_INVARIANTS.md` — compact constitutional rules that later layers must preserve.
- `03_CAUSAL_SIMULATION_CONTRACT.md` — causality, events, traces, consequences, replay, and forbidden causes.
- `04_EPISTEMIC_MODEL_AND_INFORMATION_FLOW.md` — ground truth, belief, observation, memory, records, stale information, and knowledge filtering.
- `05_AGENT_INTENTION_AND_PLANNING_DOCTRINE.md` — ordinary agents, needs, motives, projects, intentions, routines, symbolic planning, and LLM limits.
- `06_INSTITUTIONS_NORMS_HOUSEHOLDS_AND_RECORDS.md` — households, norms, institutions, records, procedures, fallibility, and social machinery.
- `07_TUI_FIRST_PLAYABILITY_CONTRACT.md` — TUI-first playability, embodied view models, debug separation, action menus, notebooks, and why-not UI.
- `08_NO_SCRIPTING_AND_CAUSAL_AUTHORING_POLICY.md` — causal authoring versus authored outcome chains, quest relapse, and hidden director risks.
- `09_SCALE_LOD_LONG_SIMULATION_AND_REGIONAL_PROCESSES.md` — scale staging, LOD, regional processes, long simulation, summaries, and ancestry preservation.
- `10_DOMAIN_BOUNDARY_AND_FIRST_PLAYABLE_SCOPE.md` — genre-agnostic core, first neutral ordinary-life domain, and deferred domain ambitions.
- `11_VALIDATION_REPLAY_AND_ACCEPTANCE_GATES.md` — no-human simulation, replay, TUI, epistemic filtering, acceptance gates, and failure as proof.
- `12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — research-derived foundation decisions and source notes.

## 1-architecture/ — subsystem design layer

The architecture layer translates foundation doctrine into subsystem contracts. It defines boundaries, data flows, validation responsibilities, and anti-patterns for the Rust-first implementation.

- `01_ARCHITECTURE_CHARTER_AND_REPLACEMENT_DECISIONS.md` — architecture authority, priority order, first slice, deferrals, and document map.
- `02_RUST_WORKSPACE_AND_SYSTEM_BOUNDARIES.md` — logical package boundaries, dependency direction, side-effect policy, storage stance, and forbidden architecture outcomes.
- `03_COMMAND_ACTION_AND_AFFORDANCE_PIPELINE.md` — commands, actions, affordances, proposals, rejection, failure, validation, scheduling, traces, observations, and why-not.
- `04_EVENT_SOURCING_CAUSAL_GRAPH_AND_PROJECTIONS.md` — event envelopes, streams, causal graph, projections, snapshots, compaction, schema evolution, and replay failure handling.
- `05_STATE_MODEL_ENTITIES_COMPONENTS_AND_CONTENT_DATA.md` — entities, components, authored data, domain packs, actions, affordances, records, validation, and versioning.
- `06_TIME_SCHEDULING_REPLAY_AND_RANDOMNESS.md` — discrete time, scheduling, durations, interruptions, reservations, deterministic replay, and random streams.
- `07_EPISTEMIC_INFORMATION_TRACE_AND_MEMORY_ARCHITECTURE.md` — truth, subjective belief, public/institutional record, observation, interpretation, traces, search, rumor, memory, and stale information.
- `08_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — structured speech acts, deterministic rendering, future freeform parsing, LLM boundaries, validation, rejection, repair, and LLM-disabled operation.
- `09_AGENT_COGNITION_PLANNING_AND_ROUTINES.md` — BDI-style cognition, needs, values, projects, intentions, HTN methods, bounded local planning, routines, replanning, planner traces, and agent LOD.
- `10_INSTITUTIONS_NORMS_HOUSEHOLDS_AND_RECORDS.md` — households, local authority, roles, norms, constitutive facts, records, reports, notices, jurisdiction, bias, corruption, and procedure failure.
- `11_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` — village scale, ordinary life, rooms, doors, sound, homes, containers, ownership, sleep, food, work, economy, social actions, and routes.
- `12_PLAYER_POSSESSION_VIEW_MODELS_TUI_AND_DEBUG.md` — human controller binding, possession, embodied/debug view models, actor notebook, human/debug notes, TUI action menus, why-not, and graphical boundary.
- `13_QUESTLESS_LEADS_NOTICES_AND_STORY_SIFTING.md` — incidents, requests, contracts, notices, leads, lead projections, player-facing wording, story sifting, and questless completion replacements.
- `14_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` — simulation-scope tiers, promotion/demotion, summary events, low-LOD people, boundary events, world history, procedural generation stance, and scale policy.
- `15_VALIDATION_TESTING_AND_ACCEPTANCE_ARCHITECTURE.md` — validation gates, test layers, canonical acceptance scenario, no-human gate, no-player privilege gate, actor-knowledge gate, replay gate, and regression artifacts.
- `16_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — architecture-specific research decisions and source notes.

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
