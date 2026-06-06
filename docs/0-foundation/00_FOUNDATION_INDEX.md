# Foundation Index

## Status

This folder is the constitutional foundation for **Tracewake**.

The foundation exists to prevent the easy wrong product:

```text
quest engine
player-centered mystery sim
omniscient investigation UI
LLM chatbot world
current-state toy
large region full of props
TUI-less research prototype
story director wearing emergent vocabulary
```

The intended path is narrower and harder:

```text
small playable village
ordinary life
subjective belief
event-sourced causality
fallible social machinery
actor-filtered TUI
replayable explanation
then honest expansion outward
```

## Document set

Read these documents as a single foundation, in order:

1. `00_FOUNDATION_INDEX.md` — map, authority, reading order, and anti-drift rules.
2. `01_PROJECT_CHARTER.md` — identity, product priorities, first village, long-term posture, and hard direction.
3. `02_CONSTITUTIONAL_INVARIANTS.md` — compact non-negotiable rules later layers must satisfy.
4. `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, traces, replay, snapshots, compaction, boundary inputs, and forensic causality.
5. `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — subjective epistemics, propositions, memory, rumor, testimony, records, lies, stale beliefs, and absence-as-evidence.
6. `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — ordinary agents, needs, routines, intentions, local planning, interruption, recovery, and transparent cognition.
7. `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action authority, affordances, ordinary-life verbs, survival needs, property, access, and anti-adventure discipline.
8. `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — households, roles, norms, local authorities, records, ledgers, notices, artifacts, and institutional fallibility.
9. `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first play, actor possession, view-model filtering, debug mode, player parity, and no protagonist metaphysics.
10. `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — the no-quest/no-director constitution, allowed scenario seeds, prehistory, and forbidden authored outcomes.
11. `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — scale discipline, LOD promotion, ancestry retention, regional history, and limits on abstraction.
12. `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, language rendering/parsing, deterministic mocks, and LLM non-authority.
13. `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first playable scope, phase gates, mandatory proof cases, and what must remain deferred.
14. `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — stable research lessons and source-handling decisions that shaped the foundation.

## Authority

The authority order is:

```text
foundation doctrine
-> architecture contracts
-> execution phase gates and fixture contracts
-> implementation specs/code
-> tests and validation reports
```

A later layer may specialize foundation doctrine, but may not weaken it.

When a later document conflicts with the foundation, the later document is wrong.
When a convenience-driven implementation conflicts with the foundation, the implementation is wrong.
When a test encodes a shortcut that violates the foundation, the test is wrong.

## Reading order for different work

### For product decisions

Read all foundation files first. Do not infer product direction from architecture, phase order, fixture names, or early implementation constraints.

### For architecture work

Read this folder, then `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`. Architecture must translate foundation constraints into boundaries and contracts, not reinterpret them.

### For execution planning

Read this folder, then the architecture index, then `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`. Execution may choose order and fixtures, but not product doctrine.

### For implementation

Implementation begins only after the execution gates for the relevant phase are clear. Implementation must preserve actor-grounded play, deterministic replay, event sourcing, and causal explanation even when the first implementation is small.

## Layer boundary

The foundation defines what Tracewake is and is not.

The foundation does **not** define:

- crate names;
- module layout;
- storage engine;
- terminal library;
- ECS/database choice;
- exact event serialization format;
- final UI layout;
- final content volume;
- final planning algorithm;
- final speech rendering strategy.

The foundation **does** define:

- no hidden director;
- no quest ontology;
- no authored outcome chains;
- no sacred player entity;
- no omniscient agent cognition;
- no LLM authority over simulation fact;
- no abstract property model when physical custody is required by the proof;
- no story projection that mutates the world;
- no TUI-less backend phase that cannot be played or inspected;
- no scale/LOD abstraction that discards needed causal ancestry.

## Central foundation decisions

### Tracewake is playable life simulation first

At every runnable stage, someone must be able to enter the TUI, possess an ordinary actor under actor-grounded knowledge, and play meaningfully. Debug/replay tools may exist, but they do not replace embodied play.

### Causality outranks drama

Every meaningful world change must arise from a modeled cause. Dramatic need, quest progress, pacing, and authorial repair are not valid causes.

### Belief outranks omniscience

Agents act from claims, memories, observations, records, testimony, expectations, trust, inference, and routine. They may be wrong for legible reasons.

### Ordinary life outranks adventure

The first proof is not beasts, caravans, wilderness hunts, adventuring parties, combat, travel, or bounty boards. The first proof is missing expected property in ordinary village life.

### Institutions are machines, not truth oracles

Households, clerks, authorities, ledgers, reports, notices, and norms can be procedurally valid and still epistemically or morally wrong.

### Story is retrospective

Story systems may sift, summarize, highlight, query, or explain events that already happened. They may not spawn, pace, repair, complete, or force events.

### Language is surface, not authority

Natural language may render or parse structured acts. It may not create simulation fact, bypass validation, grant omniscience, or mutate world state outside deterministic event pathways.

### Scale is earned

LOD, prehistory, regions, and long simulation are allowed only if promoted actors, incidents, goods, injuries, rumors, records, debts, relationships, and beliefs retain enough causal ancestry to be explained.

## Recurring review questions

Every major design, architecture, fixture, or implementation decision should survive these questions:

1. Can the same world-affecting action be taken by an AI-controlled actor and a human-controlled actor under equivalent actor conditions?
2. Does the acting agent know this through an explicit belief, memory, observation, record, report, trace, or inference?
3. Does this create or mutate world state only through an event with a cause model?
4. Can deterministic replay reproduce it?
5. Can debug/provenance explain why it happened?
6. Does the TUI expose only actor-grounded knowledge during embodied play?
7. Is any institution involved acting from institutional knowledge rather than ground truth?
8. If a story/lead/notice/rumor is visible, is it a projection or artifact over causes rather than a quest state?
9. If language is involved, is a structured speech act authoritative and the prose non-authoritative?
10. If scale/LOD is involved, is causal ancestry preserved enough for later promotion and explanation?

## Expansion posture

Tracewake can eventually support:

- historical world generation;
- travel;
- roads;
- beasts;
- combat;
- magic;
- companion recruitment;
- social reputation;
- courts;
- guilds;
- markets;
- regional politics;
- graphical clients;
- LLM-assisted speech surfaces.

None of those are allowed to outrank the first foundation:

```text
small ordinary village
physical property
subjective belief
fallible records
no-human operation
actor possession parity
TUI play
forensic replay
```

## First standard

The first standard for Tracewake is not that the world feels large.

The first standard is that a small ordinary village can produce this chain without a script:

```text
I expected the coins to be in the chest.
The chest is empty.
I search.
I ask.
I remember.
I suspect.
I report.
A record is made.
Someone may be wrongly suspected.
I delay payment or change my plan.
Replay explains why every actor did what they did.
The same chain can happen with no human present.
```

If a design does not protect that, it is not Tracewake foundation work.
