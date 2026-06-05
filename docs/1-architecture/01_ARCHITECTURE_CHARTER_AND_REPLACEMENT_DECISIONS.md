# Architecture Charter and Replacement Decisions

## Status

This folder is the replacement architectural-level foundational document set for **Tracewake**.

Tracewake is a causality-first, epistemic, TUI-first ordinary-life simulation. The authoritative simulation models ordinary agents in a world of needs, routines, beliefs, traces, records, institutions, households, property, work, travel, memory, and consequences. A human player may temporarily bind controls to an ordinary actor, but the simulation itself must not know or care that a human exists.

These documents are architectural contracts. They are not execution tickets, implementation code, task breakdowns, or a literature review. They define the boundaries that later execution-level documents must obey.

## Priority order

Tracewake development obeys this priority order:

1. **Playable TUI simulation first.** The first serious target is an inspectable TUI-playable village, not a graphical demo, not a text chatbot, and not a content generator.
2. **Research-grade emergent simulation engine second.** The architecture must be rigorous enough for replay, inspection, no-human simulation, and experiments.
3. **Future graphical presentation later.** A future graphical client may consume view models, but it must not become the simulation authority.

## Constitutional rules preserved by this architecture

The following rules are hard constraints:

- Causality before drama.
- Belief before truth.
- Ordinary life before adventure.
- No sacred player entity.
- Every world-affecting player action must be possible for an ordinary agent under equivalent conditions.
- Institutions are fallible social machines.
- Quests are projections, not ontology.
- Authored causal machinery is allowed.
- Authored outcome chains are forbidden.
- Symbolic, inspectable agents come before generative agents.
- TUI-first, playable always.
- The simulation kernel is genre-agnostic.
- Story is observed, not directed.
- LLMs may render or parse language only behind validation.
- LLMs are not authoritative simulation brains.
- Rust-first implementation.
- Event sourcing and forensic causality are foundational.
- No-human simulation is mandatory in every runnable phase.

## Replacement decisions

This set restarts architecture numbering at `01_`. It does not continue foundation numbering and does not preserve the earlier architecture filenames one-to-one.

The previous architecture documents were directionally correct but too often mixed doctrine, examples, and implementation implications in the same place. This replacement set makes sharper architectural cuts:

- The Rust workspace and system boundaries are explicit.
- World-affecting actions get their own command/action/affordance pipeline.
- Event sourcing, causal graph inspection, projections, snapshots, and event schema migration are separated from generic world-kernel doctrine.
- State and authored data are treated as a validation architecture, not as loose content notes.
- Time, scheduling, replay, and randomness are explicit deterministic contracts.
- Epistemics are modeled as data shapes and update pipelines, not just as design philosophy.
- Agent cognition is an interface stack: beliefs, needs, motives, intentions, HTN methods, bounded local planning, action proposals, planner traces.
- Speech acts and LLM surfaces are separated from the epistemic core.
- Households are first-class institutions from the beginning.
- The first public institution is a minimal local authority, not a broad legal system.
- The first serious proof is missing property/theft in ordinary life.
- Road-threat, bounty, expedition, and combat flows are second proof material, not v1 gravity wells.
- Story sifting is an observer/debug layer only.
- LOD and long history are future-enabling architecture, not first-slice pressure.
- Validation and acceptance architecture are first-class.

## First serious vertical slice

The first serious implementation target is a small, deep, TUI-playable ordinary-life village.

Target scale:

- roughly 10-30 high-detail agents;
- optional additional low-LOD background people only when causally honest and promotable;
- homes, rooms, doors, containers, beds, food, sleep, work, storage, simple money/custody, ownership, possession, expected locations, household access, social interaction, a notice board, and a local authority office;
- one reeve, one clerk, and up to two guards if needed;
- office hours, an incident ledger, simple property/theft and trespass/privacy norms, a report action, and a minimal questioning procedure;
- no-human daily simulation using the same scheduler as embodied play;
- TUI action menus built from actor-visible affordances;
- debug inspection able to explain events, beliefs, traces, causal ancestry, planner state, and knowledge mismatch.

The first domain is **neutral medieval-ish ordinary life without magic**. This is a restrained first domain because slow communication, local authority, household privacy, records, physical traces, food, work, doors, storage, and social reputation are legible. It is not a genre commitment by the kernel.

## Canonical first miracle: missing property/theft

The first proof of seriousness is not combat and not a road bounty. It is the missing-property/theft miracle:

```text
actor stores or expects property somewhere
 -> another actor, from modeled need/belief/opportunity/risk, takes or moves it
 -> the victim later discovers absence by expectation contradiction or search
 -> witnesses may hold partial, stale, uncertain, or wrong observations
 -> testimony, gossip, or report propagates source-bound claims
 -> a clerk or local authority may open a record from partial information
 -> wrong suspicion can arise for legible reasons
 -> notices or public artifacts can become stale
 -> possession can switch among actors without knowledge leaks
 -> debug mode explains what happened, why it was possible, what traces exist,
    who knows what, who is wrong, and which later events became possible
```

This proof exercises ordinary life, property, households, epistemics, action parity, traces, institutions, event sourcing, TUI view models, no-human simulation, and debug inspection.

## Deferrals

These are intentionally deferred from the first serious slice:

- freeform player speech;
- live LLM parsing of player input;
- LLM-authored NPC cognition;
- graphical client implementation;
- detailed combat;
- magic or supernatural truth channels;
- full courts, guilds, temples, markets, bandit organizations, adventuring companies, and large legal systems;
- road-threat/bounty/expedition as the primary proof;
- procedural region generation before the hand-authored village ontology works;
- drama directors, pacing managers, quest generators, or protagonist gravity.

Deferred does not mean impossible. Deferred means the first slice must prove the ordinary-life causal substrate before those systems are allowed to consume it.

## Authority map

```text
Authored data
  defines possible entities, actions, norms, routines, methods, channels, templates, seeds
  but cannot force outcome chains

Simulation kernel
  validates proposals, schedules actions, resolves outcomes, commits events,
  maintains authoritative state projections, and owns replay semantics

Event log
  is the forensic source of world mutation and causal ancestry

Projections
  cache current state, records, beliefs, traces, view models, lead cards, summaries,
  and indexes; they are rebuildable and non-authoritative when inconsistent

TUI and future clients
  consume view models and submit commands; they do not implement world rules

Debug tools
  inspect truth, causal graph, event log, projections, beliefs, and planner traces;
  they do not leak into embodied play

LLM/language surfaces
  render or parse structured language proposals behind validation;
  they do not decide truth, plans, facts, success, or world mutation
```

## Required architectural question for every feature

Every feature must answer these questions before it is accepted into the architecture:

- What caused it?
- Who knows it?
- How can they be wrong?
- What traces exist?
- What institution, norm, household, role, or record cares?
- Can an NPC do the same kind of thing under equivalent conditions?
- Can it run without a human controller?
- Can it be played and inspected through the TUI?
- Does debug mode explain the causal chain?
- Can the event log replay and rebuild projections deterministically?
- Does it avoid player privilege, ground-truth leakage, genre leakage, scripting, hidden drama direction, and LLM authority?

## Document map

- `02_RUST_WORKSPACE_AND_SYSTEM_BOUNDARIES.md` defines logical Rust-first package boundaries and dependency direction.
- `03_COMMAND_ACTION_AND_AFFORDANCE_PIPELINE.md` defines the unified world-affecting action pipeline.
- `04_EVENT_SOURCING_CAUSAL_GRAPH_AND_PROJECTIONS.md` defines event envelopes, streams, causal graph, projection rebuild, snapshots, and schema evolution.
- `05_STATE_MODEL_ENTITIES_COMPONENTS_AND_CONTENT_DATA.md` defines entities, components, data authoring, domain packs, and validation.
- `06_TIME_SCHEDULING_REPLAY_AND_RANDOMNESS.md` defines deterministic time, scheduler, action duration, interruptions, replay, and randomness.
- `07_EPISTEMIC_INFORMATION_TRACE_AND_MEMORY_ARCHITECTURE.md` defines observation, belief, trace, memory, rumor, stale information, contradiction, and source filtering.
- `08_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` defines typed speech acts, language rendering, and the LLM boundary.
- `09_AGENT_COGNITION_PLANNING_AND_ROUTINES.md` defines symbolic agent cognition and planner interfaces.
- `10_INSTITUTIONS_NORMS_HOUSEHOLDS_AND_RECORDS.md` defines households, norms, roles, records, local authority, institutional facts, and procedure failure.
- `11_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` defines first-slice ordinary life, simple economy, settlement graph, and spatial causality.
- `12_PLAYER_POSSESSION_VIEW_MODELS_TUI_AND_DEBUG.md` defines possession, actor-filtered view models, TUI menus, debug views, and future graphical boundary.
- `13_QUESTLESS_LEADS_NOTICES_AND_STORY_SIFTING.md` defines lead projections, notices, records, incidents, and observer-only story sifting.
- `14_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` defines LOD, promotion/demotion, regional processes, boundary events, and future long simulation.
- `15_VALIDATION_TESTING_AND_ACCEPTANCE_ARCHITECTURE.md` defines testing gates and acceptance architecture.
- `16_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` records research-derived architectural decisions.
