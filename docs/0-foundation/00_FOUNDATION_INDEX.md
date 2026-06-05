# Foundation Index

## Authority

This folder is the constitutional layer for Tracewake. Architecture, execution, fixtures, content data, tests, TUI work, and later domain packs may refine these rules. They may not weaken them.

Tracewake is a causality-first, epistemic, TUI-first ordinary-life simulation engine and playable terminal-first simulation. The world is authoritative whether or not a human is attached to an actor. The human is a temporary input binding, not a metaphysical entity in the world.

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

## Replacement set

This replacement set uses the following documents:

1. `00_FOUNDATION_INDEX.md` — map, authority, reading order, and anti-drift rules.
2. `01_PROJECT_CHARTER.md` — identity, product priorities, first village, long-term posture, and hard direction.
3. `02_CONSTITUTIONAL_INVARIANTS.md` — compact non-negotiable rules later layers must satisfy.
4. `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, traces, replay, snapshots, compaction, boundary inputs, and forensic causality.
5. `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — typed claims/propositions, beliefs, memories, observations, testimony, lies, rumors, records, and knowledge flow.
6. `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — symbolic agents, BDI separation, durable intentions, HTN procedures, bounded local planning, utility boundaries, and debug traces.
7. `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — ordinary action parity, affordances, survival substrate, search, storage, work, travel, and basic economy.
8. `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — households, roles, norms, records, notices, ownership/custody/access, institutional fallibility, and future organizations.
9. `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first playability, actor-filtered view models, possession, notebooks, why-not explanations, and debug separation.
10. `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — causal authoring, forbidden authored outcome chains, seeds, authored prehistory, records, notices, and no director logic.
11. `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — honest multi-resolution simulation, promotion/demotion, regional processes, long history, and future scale.
12. `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, optional LLM rendering/parsing, validated extraction, prompt boundaries, and LLM-disabled operation.
13. `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first village scope, missing-property proof, no-human gates, replay gates, TUI gates, and canonical regression seeds.
14. `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — decisions drawn from research and precedent, recorded as design constraints rather than literature review.

## Reading order

Read `01_PROJECT_CHARTER.md` and `02_CONSTITUTIONAL_INVARIANTS.md` first. They define identity and hard limits.

Read `03` through `11` as the core doctrine. These documents divide one system into event causality, epistemics, agents, ordinary action, social machinery, TUI, authoring, scale, and language boundaries.

Read `12` before planning implementation phases. It defines what the first serious proof must show.

Read `13` when a design argument asks why this foundation rejects quests, drama directors, LLM agent brains, or opaque scale.

## Layer boundary

Foundation documents may name required properties, required separations, forbidden failure modes, and acceptance gates. They should not freeze crate choices, storage engines, terminal libraries, exact Rust module layout, ECS/database decisions, planner algorithm internals, serialization formats, UI layout, or phase task tickets.

Architecture documents should translate this doctrine into subsystem contracts.

Execution documents should choose implementation order, fixtures, and gates.

Reference documents should remain compact lookup material.

If later layers conflict with this folder, later layers are wrong.

## Central decisions

Tracewake is built around these decisions:

```text
event-sourced causal history
+ typed claims/propositions as epistemic currency
+ subjective belief before truth
+ ordinary agents with needs, duties, routines, and durable intentions
+ households and institutions as fallible social machines
+ records and notices as world artifacts
+ source-bound actor leads, not quests
+ TUI-first embodied play
+ possession as input binding only
+ no-human simulation as acceptance proof
+ optional LLM language surfaces over structured speech acts
+ scale through explicit LOD and summary ancestry
```

## Recurring review questions

Every feature, fixture, scenario seed, UI surface, domain pack, region process, and future ambition must answer:

```text
What caused it?
What event or declared boundary input represents it?
Who knows it?
What typed claims carry it?
How can those claims be false, stale, partial, or contradicted?
What traces exist, including erased traces?
What household, institution, norm, record, or public artifact cares?
Can an ordinary NPC attempt or experience the same kind of thing?
Can it run without a human?
Can it be played through the TUI or the same actor-filtered view model?
Can debug mode explain truth, belief, records, traces, and random draws?
Can replay reconstruct the meaningful chain?
Does it smuggle in player privilege, quest ontology, drama direction, genre assumptions, or LLM authority?
```

An answer of “the quest state says so,” “the UI knows,” “the LLM said it,” “the story needed it,” “the player accepted it,” or “the region summary just declares it” is a foundation failure.

## Expansion posture

Tracewake must eventually have room for parties, companions, expeditions, trade routes, disease, animal migration, storms, inheritance, corruption, gangs, guilds, armies, councils, courts, religions, companies, wars, settlement politics, territorial control, detailed combat, fantasy packs, and science-fiction packs.

Those are future domain/system expansions. They do not get exceptions.

A future army conquest must arise from recruitment, command, loyalty, logistics, morale, travel, supply, violence, fear, rumor, negotiation, institutional authority, territorial control, records, and consequences. It must not be a conquest quest script.

A future magical vision must be a causal information channel with cost, reliability, provenance, distortion, traces, counters, and failure modes. It must not be omniscient UI with a fantasy label.

A future companion relationship must be an ordinary social relationship with beliefs, promises, loyalty, fear, need, obligation, memory, and refusal. It must not be party-slot privilege.

## First standard

Build a small, TUI-playable, replayable ordinary village whose people can be wrong for the right reasons. Then expand outward through the same machinery.
