# Architecture Index and Conformance

## Status

This folder is the replacement architecture contract set for **Tracewake**.

The architecture layer translates the foundation documents into subsystem boundaries, data contracts, authority rules, validation obligations, and anti-drift checks. It is not a second foundation, a task plan, a crate layout, an implementation ticket list, or a literature review.

Foundation documents outrank this folder. If an architecture document appears to weaken, dodge, contradict, or blur a foundation constraint, the foundation wins and the architecture document must be fixed.

## Scope of this replacement set

This set is ready to overwrite `docs/1-architecture/*`.

It deliberately renumbers and merges the earlier draft. The previous set was directionally correct, but repeated foundation doctrine too often and spread some hard architecture across too many files. The replacement keeps the doctrine, sharpens the contracts, and adds explicit save-package, conformance, observability, and review-artifact architecture.

The architecture remains Rust-first and TUI-first, but it does not freeze crate names, storage engines, ECS/database choices, terminal libraries, module layout, or execution phases.

## Priority order

Tracewake development obeys this architectural priority:

1. **Playable TUI simulation first.** The first serious target is an inspectable, TUI-playable ordinary-life village.
2. **Research-grade emergent simulation engine second.** Replay, no-human simulation, causal inspection, deterministic validation, and review artifacts are non-negotiable.
3. **Future graphical presentation later.** Graphical clients consume view models. They do not become simulation authority.

## Constitutional spine preserved here

All architecture contracts must preserve:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- every world-affecting player action possible for an ordinary agent under equivalent conditions;
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

## Replacement document map

Read in order unless working on a specific subsystem.

| File | Contract owned |
|---|---|
| `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Folder map, foundation conformance, universal feature questions, and architecture-layer maintenance rules. |
| `01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` | Rust-first authority boundaries, logical subsystem seams, dependency direction, side-effect rules, and forbidden authority inversions. |
| `02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` | Unified command/proposal/action/affordance/scheduler contract for all world-affecting behavior. |
| `03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` | Event log, causal graph, replay, projection rebuild, snapshots, event schema migration, random streams, and save-package contract. |
| `04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` | Entity/component state model, authored content, domain packs, scenario seeds, schemas, fixtures, and content validation. |
| `05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` | Typed claims, observation, belief, memory, traces, rumors, stale information, absence-as-evidence, and actor-knowledge filtering. |
| `06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | Structured speech acts, deterministic templates, optional future LLM parsing/rendering, and validation boundary. |
| `07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` | Symbolic agent cognition, needs, values, projects, durable intentions, HTN routines, bounded local planning, planner traces, and agent detail tiers. |
| `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | Households, roles, norms, records, reports, notices, institutional facts, evidence thresholds, procedure failure, bias, and jurisdiction. |
| `09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` | First-slice village, homes, rooms, doors, containers, sleep, food, work, simple economy, travel, and settlement causality. |
| `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Human controller binding, possession, actor-filtered embodied view models, actor notebook, TUI harness, debug views, and future graphical boundary. |
| `11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md` | Questless incidents, requests, obligations, leads, actor-known summaries, and observer-only story sifting. |
| `12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` | Dynamic level of detail, regional processes, boundary events, promotion/demotion, summary events, world history, and scale staging. |
| `13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` | Validation gates, no-human runs, deterministic replay checks, observability artifacts, metrics, and acceptance levels. |
| `14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Architecture-relevant research decisions and source notes. |

## Authority model

The architecture assumes these authority boundaries:

```text
Authored content
  defines possible entities, actions, norms, routines, records, templates, fixtures,
  seeds, and scenario pressures; it cannot force outcome chains.

Simulation kernel
  validates proposals, resolves outcomes, mutates authoritative state projection,
  commits events, enforces invariants, and preserves action parity.

Event log and save package
  preserve forensic world mutation, content/schema versions, random stream state,
  replay diagnostics, and projection rebuild metadata.

Projections
  cache current state, beliefs, records, traces, view models, lead cards, indexes,
  and debug summaries; they are rebuildable and subordinate to the event log.

Agents and institutions
  propose actions from beliefs, needs, roles, records, norms, and procedures;
  they do not mutate world truth directly and do not read hidden truth.

TUI and future clients
  consume actor-filtered or debug view models and submit commands; they do not
  implement unique world rules.

Debug tools
  inspect truth, event history, causal graph, projections, beliefs, planners,
  scheduler state, LOD, and validation reports; debug truth must not leak into
  embodied play.

Language/LLM surfaces
  render or parse structured proposals behind validation; they never decide truth,
  plans, success, guilt, sanction, or world mutation.
```

## Universal architectural question for every feature

Every architecture feature must answer:

- What caused it?
- Who knows it?
- How can they be wrong?
- What traces exist?
- What household, institution, norm, role, record, public artifact, or relationship cares?
- Can an NPC do the same kind of thing under equivalent conditions?
- Can it run with no human controller?
- Can it be played through the TUI or equivalent actor-filtered view model?
- Can debug mode explain it?
- Can replay reconstruct it deterministically?
- Does it avoid player privilege, quest ontology, drama direction, ground-truth leakage, hidden scripting, genre leakage, and LLM authority?

## Foundation conformance matrix

This matrix is intentionally compact. It maps foundation authority to architecture contracts without creating a brittle line-by-line duplicate of foundation doctrine.

| Foundation authority | Architecture implementation |
|---|---|
| Project charter and pillars | `00`, `01`, `09`, `10`, `13` define causality-first, epistemic, TUI-first, ordinary-life, no-human, replayable architecture. |
| Constitutional invariants | `00` captures universal invariants; all subsystem docs include denied authority and anti-patterns. |
| Causal event trace and replay | `02`, `03`, `12`, `13` define proposal validation, event envelopes, causal graph links, deterministic scheduling, replay, snapshots, random streams, LOD summaries, and replay failure. |
| Claims, beliefs, memory, information flow | `05`, `06`, `08`, `10`, `11` define typed claims, belief provenance, speech acts, records, rumors, actor notebooks, and actor-filtered leads. |
| Agents, needs, intentions, routines, planning | `07`, with support from `02`, `04`, `05`, and `12`. |
| Actions, affordances, ordinary life, survival | `02`, `04`, `09`, and `13` define affordances, ordinary objects, action parity, needs, reservations, traces, and TUI exercise. |
| Institutions, households, norms, records, artifacts | `08`, with display/projection support in `10` and incident/lead support in `11`. |
| TUI, possession, view models, debug | `10`, supported by projection/replay contracts in `03` and validation in `13`. |
| No scripting, authoring, seeds, prehistory | `04`, `11`, `12`, and `13` define content possibility space, no outcome chains, causal seeds, procedural generation boundary, and no-script validation. |
| Scale, LOD, regional boundaries, long simulation | `12`, supported by save-package and compaction rules in `03` and metrics in `13`. |
| LLM speech acts and language boundary | `06`, with validation and deterministic mocks in `13`. |
| First playable scope and acceptance gates | `09`, `10`, and `13` define the first village, missing-property proof, TUI playability, no-human simulation, and acceptance artifacts. |
| Research decisions and source notes | `14`, with research-derived constraints integrated into relevant subsystem docs. |

## Architectural layer versus execution layer

Architecture may define:

- authority ownership and denied authority;
- data crossing subsystem boundaries;
- semantic contracts for events, actions, beliefs, records, projections, save packages, and view models;
- validation obligations and acceptance implications;
- anti-patterns that prevent drift.

Architecture must avoid:

- task tickets;
- sprint/phase ordering beyond first-slice implications;
- exact crate/module layout;
- storage engine commitments;
- terminal/GUI library commitments;
- executable code;
- implementation-only fixture lists;
- branch, repository, or current-main claims.

Execution documents own concrete phase gates, order of implementation, tickets, fixtures, and test harness details.

## First serious proof

The first proof remains the missing-property/theft ordinary-life miracle:

```text
actor stores or expects property somewhere
 -> another actor, from modeled need/belief/opportunity/risk, takes, borrows, moves,
    hides, damages, or fails to take it through ordinary actions
 -> victim later discovers absence through expectation contradiction or search
 -> witnesses and records hold partial, stale, uncertain, or wrong claims
 -> reports, testimony, rumors, notices, and institutional records propagate source-bound claims
 -> wrong suspicion can arise for legible reasons
 -> possession and TUI view models filter actor knowledge
 -> debug explains what happened, why it was possible, what traces exist,
    who knows what, who is wrong, and which later events became possible
 -> replay reconstructs the chain deterministically
```

This proof is ordinary, not trivial. It exercises nearly every constitutional requirement.

## Maintenance rules

When a new architecture document or major revision is proposed:

- identify which foundation authority it implements;
- identify what authority the subsystem owns and is denied;
- define data crossing boundaries;
- state replay, no-human, TUI, debug, and LLM-disabled implications;
- include anti-patterns where drift is likely;
- remove repeated foundation sermon unless repetition is needed to prevent a common failure;
- update this index and any downstream execution/reference documentation.

## Forbidden architecture outcomes

- A privileged `Player` subsystem with exclusive world verbs.
- A `Quest` subsystem that controls truth, targets, progress, or completion.
- A drama director that creates events, shifts probabilities, or pauses causality for pacing.
- An LLM agent brain or dialogue surface that commits facts.
- A storage schema treated as truth without replay.
- A graphical client that implements unique world rules.
- A domain pack that writes events directly.
- A debug notebook that silently becomes actor knowledge.
