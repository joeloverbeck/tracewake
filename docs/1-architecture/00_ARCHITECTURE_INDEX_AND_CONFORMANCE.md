# Architecture Index and Conformance

## Status

This folder is the architecture contract layer for **Tracewake**.

The architecture layer translates foundation documents into subsystem boundaries, data contracts, authority rules, validation obligations, and anti-drift checks. It is not a second foundation, a task plan, a crate layout, an implementation ticket list, a migration note, or a literature review.

Foundation documents outrank this folder. If an architecture document appears to weaken, dodge, contradict, or blur a foundation constraint, the foundation wins and the architecture document must be fixed.

## Scope of this architecture set

The architecture set defines stable contracts for the first implementable spine of Tracewake:

- authoritative simulation boundaries;
- command, proposal, action, affordance, and scheduling flow;
- event log, replay, projections, save packages, and causal graph duties;
- state/content authoring boundaries and schema validation;
- claims, beliefs, traces, memory, and information flow;
- structured speech acts and language/LLM boundaries;
- transparent agent cognition, routines, planning, and agent LOD;
- institutions, households, norms, records, and procedures;
- ordinary-life economy, settlement, property, custody, and spatial model;
- possession, TUI view models, debug boundaries, and client authority;
- incidents, leads, and story-sifting projections;
- LOD, regional processes, world history, and scale;
- validation, observability, metrics, and acceptance artifacts.

It deliberately does **not** freeze:

- crate names;
- module layout;
- terminal UI library;
- storage engine;
- ECS/database choice;
- final serialization format;
- exact planner algorithm;
- final UI layout;
- final content size.

Those choices are subordinate to the contracts here.

## Priority order

Tracewake architecture obeys this priority:

1. **Playable TUI simulation first.** The first serious target is an inspectable, TUI-playable ordinary-life village.
2. **Research-grade emergent simulation engine second.** Replay, no-human simulation, causal inspection, deterministic validation, and review artifacts are non-negotiable.
3. **Future graphical presentation later.** Graphical clients consume view models. They do not become simulation authority.

## Constitutional spine preserved here

All architecture contracts must preserve:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- every world-affecting player action possible for an ordinary agent under equivalent actor conditions;
- no quest ontology;
- no authored outcome chains;
- authored causal machinery and scenario pressures allowed;
- institutions and households as fallible social machines;
- typed claims/propositions as epistemic currency;
- records, notices, debts, promises, reports, rumors, and leads as source-bound artifacts or projections;
- symbolic, inspectable agents before generative agents;
- LLMs as optional rendering/parsing surfaces behind validation, never authoritative brains;
- TUI-first, playable always;
- no-human simulation in every runnable phase;
- event sourcing, deterministic replay, causal graph inspection, projection rebuild, and deterministic randomness;
- Rust-first authoritative simulation;
- genre-agnostic kernel;
- first serious proof: the missing-property/theft ordinary-life miracle;
- future scale, combat, magic, parties, courts, guilds, armies, regions, companions, and graphical clients passing through the same machinery.

## Architecture document map

Read in order unless working on a specific subsystem.

| File | Contract owned |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Folder map, foundation conformance, universal feature questions, and architecture-layer maintenance rules. |
| `01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` | Rust-first authority boundaries, logical subsystem seams, dependency direction, side-effect rules, and forbidden authority inversions. |
| `02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` | Unified command/proposal/action/affordance/scheduler contract for all world-affecting behavior. |
| `03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` | Event log, causal graph, replay, projection rebuild, snapshots, event schema migration, random streams, and save-package contract. |
| `04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` | Entity/component state model, authored content, domain packs, scenario seeds, schemas, fixtures, and content validation. |
| `05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` | Typed claims, observation, belief, memory, traces, rumors, stale information, absence-as-evidence, and actor-knowledge filtering. |
| `06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | Structured speech acts, deterministic language surfaces, LLM non-authority, validation, and replayable speech-state effects. |
| `07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` | Transparent cognition, needs, routines, intentions, bounded planning, resourcefulness, interruption, and agent-detail levels. |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Households, offices, authorities, clerks, procedures, reports, ledgers, wrong suspicion, and institutional failure modes. |
| `09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` | Settlement map, rooms, containers, custody, physical value tokens, ordinary needs, workplaces, access, and local movement. |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Human controller binding, possession parity, TUI view models, debug separation, inspection surfaces, and client non-authority. |
| `11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md` | Incidents, leads, notices, story-sifting projections, observer queries, and no-director/no-quest enforcement. |
| `12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` | LOD boundaries, regional processes, promotion, ancestry retention, prehistory, long simulation, and scale gates. |
| `13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` | Acceptance artifacts, invariants, golden fixtures, replay proof, no-human proof, provenance review, and metric meaning. |
| `14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Stable research decisions and source-handling principles relevant to architecture. |

## Universal architecture questions

Every architecture proposal must answer these before acceptance:

1. What is the authoritative state owner?
2. What events can change that state?
3. What is the cause model for those events?
4. What data is actor-visible, institution-visible, debug-visible, or globally true but hidden?
5. How does deterministic replay reproduce the behavior?
6. How does no-human simulation exercise it?
7. How does the TUI make it playable or inspectable at this phase?
8. What would be the forbidden quest/director/omniscience shortcut for this subsystem, and how is it blocked?
9. How are LLMs, prose, or UI surfaces prevented from becoming simulation authority?
10. What fixture or golden scenario proves the contract?

## Maintenance rules

Architecture documents should be corrected when they:

- contain session-specific audit ledgers, stale commit IDs, connector histories, or replacement-process notes;
- blur foundation authority;
- make a convenience tradeoff that would erase belief subjectivity, actor parity, physical custody, or replay;
- create a hidden source of world mutation;
- turn projections into causes;
- allow language output to become fact;
- postpone TUI playability or no-human operation out of the runnable phase.

Stable source discipline belongs in architecture documents. Exact fetch ledgers, temporary manifest inventories, and mission-specific provenance belong in audit artifacts, not in committed architecture authority docs.
