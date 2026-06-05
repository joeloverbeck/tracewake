# Tracewake Project Charter

## Name

**Tracewake**.

The name is doctrine. Actions leave wakes. Wakes become traces. Traces are observed, ignored, recorded, distorted, erased, remembered, forgotten, argued over, and institutionalized by ordinary agents.

## One-sentence definition

Tracewake is a causality-first, epistemic, TUI-first ordinary-life simulation where the human temporarily controls ordinary agents inside a world of needs, routines, beliefs, traces, records, institutions, travel, property, work, fallible memory, fallible causality, and consequences.

## Hard product priority

Tracewake is built in this order:

1. **Playable TUI simulation first.**
2. **Research-grade emergent simulation engine second.**
3. **Future graphical presentation later.**

A beautiful graphical shell without a playable causal village is failure. A research engine that cannot be played, inspected, and tested through the TUI is unfinished. A clever narrative system that bypasses ordinary-life causality is not Tracewake.

## First implementation target

The first serious implementation target is a small, inspectable, TUI-playable ordinary-life village simulation.

It is not:

- an expedition game;
- a combat prototype;
- a quest board;
- a Skyrim radiant-quest replacement;
- a RimWorld-style drama-director game;
- a Shadows-of-Doubt clone with omniscient case structure;
- an LLM NPC chatbot;
- a graphical client waiting for simulation later.

The first target is a neutral medieval-ish village without magic. Ordinary agents sleep, eat, work, own things, store things, borrow, steal, notice absence, misremember, lie, gossip, report, read records, make promises, keep or break routines, and act through the same action pipeline as the human-controlled body. The world runs coherently with no human input.

## Implementation-language doctrine

Tracewake is **Rust-first**.

The authoritative simulation core, event/replay machinery, action validation, actor-knowledge filtering, agent decision core, no-human simulation harness, and first TUI client must be implemented in Rust unless a later foundation-level decision explicitly replaces this doctrine.

Rust is a product constraint because Tracewake needs performance, determinism, memory safety, concurrency safety, long-term maintainability, and low-level control without adopting C++ as the project’s foundation. The foundation set does **not** freeze crates, persistence engines, UI libraries, ECS frameworks, scripting runtimes, serialization formats, or database choices. Those belong in later architecture decisions.

Non-Rust tools may exist for documentation, import/export, analysis, optional language surfaces, content pipelines, or research notebooks. They may not become the authoritative simulation brain.

## Product pillars

### 1. Causality before drama

Events happen because prior state, actor intention, need pressure, belief, routine, environment, institution, regional process, or authored initial condition made them possible. No event exists because the player needs pacing.

### 2. Belief before truth

Agents act from subjective beliefs, not authoritative truth. Wrong beliefs, stale information, rumor, lies, misidentification, inference, absence-as-evidence, expectation contradiction, forgetting, and memory distortion are core mechanics.

### 3. Ordinary life before adventure

Adventure is only meaningful when ordinary life exists to be disrupted. Hunger, fatigue, safety, sleep, eating, work, homes, rooms, doors, containers, storage, ownership, money, household routines, local norms, and basic social interaction are not background flavor. They are the substrate.

### 4. The world has no sacred player entity

The authoritative simulation contains no metaphysically special player character.

Allowed doctrine:

```text
HumanController -> ActorId
```

Possession changes input binding, not reality. The previous body remains an ordinary agent with its own beliefs, needs, intentions, relationships, obligations, memories, possessions, suspicions, and plans.

### 5. Every world-affecting player action must be ordinary-agent possible

The current body has the same world-affecting actions an AI agent would have under equivalent physical, social, epistemic, resource, and institutional conditions. UI kindness is allowed. Metaphysical privilege is forbidden.

### 6. Institutions are fallible social machines

Law, households, workshops, offices, guilds, markets, gangs, companies, councils, courts, and future domain-specific religious institutions act through people, roles, obligations, permissions, prohibitions, records, procedure, resources, delay, bias, jurisdiction, fear, corruption, error, and forgetting. No guard, ledger, notice board, clerk, or office reads ground truth.

### 7. Quests are projections, not ontology

Tracewake stores incidents, needs, requests, contracts, notices, obligations, promises, rumors, records, leads, investigations, sanctions, opportunities, and claims. A player-facing objective is a view over source-bound world state. It is not the world process itself.

### 8. Authored causal machinery is allowed; authored outcome chains are forbidden

Designers author actions, affordances, needs, routines, HTN methods, institution procedures, trace types, speech acts, initial conditions, scenario seeds, LOD rules, norms, and test fixtures. Designers do not author guaranteed story beats, player-conditioned events, outcome chains, quest progress, or drama triggers.

### 9. Symbolic, inspectable agents first

V1 agents are explicit symbolic agents: BDI-style beliefs/desires/intentions, durable projects, needs, duties, relationships, traits, HTN routines/procedures, bounded local planning, event-driven replanning, planner budgets, interruption points, and failure events.

Utility scoring may help choose among explicit options. It may not become the whole mind.

### 10. TUI-first, playable always

The TUI is the first real Tracewake client. It is not disposable and not merely a debug shell. Every runnable phase must have TUI reachability, actor-knowledge filtering, why-not explanations, debug inspection, automated TUI or view-model tests, and no-human simulation tests.

### 11. Genre-agnostic kernel; neutral no-magic village first

The kernel must not assume medieval law, fantasy races, divine authority, magic, monsters, adventurers, combat classes, feudal obligation, or supernatural perception. The first domain is neutral medieval-ish ordinary life because it is legible: homes, roads, workshops, storage, taverns, offices, notices, local authority, travel, imperfect evidence, and public records.

Strict fantasy elements belong to future domain packs. Granular combat and injury belong to future systems.

### 12. Story is observed, not directed

Story sifting, salience detection, recaps, summaries, notebooks, and debug explanations may highlight what happened. They may not cause what happens.

## The required first miracle

The canonical early proof is a missing-property incident that emerges from ordinary life:

1. An actor stores or expects property somewhere.
2. Another actor, driven by modeled needs, beliefs, opportunity, risk, relationships, values, or obligation, takes or moves it.
3. The victim later discovers the absence through expectation contradiction or search.
4. A witness may have uncertain, partial, stale, or misinterpreted observations.
5. Testimony, rumor, gossip, or report can propagate.
6. A clerk, reeve, or other authority may open a record from partial information.
7. Wrong suspicion can arise for legible reasons.
8. Notices, reports, and public artifacts can become stale.
9. The human can possess different ordinary actors in debug mode and verify that no knowledge leaks between bodies.
10. Debug inspection can explain what happened, why it was possible, what traces exist, who knows what, who is wrong, and which later events became possible.

This is not an authored mystery script. It is a proof that physical state, belief state, social information, institutional procedure, memory, records, and traces are one causal system.

## First playable loop

The first serious loop is a **life-possession sandbox**:

1. Enter a TUI village as an ordinary body.
2. Inspect actor-known needs, location, possessions, beliefs, intentions, obligations, relationships, and available actions.
3. Sleep, eat, work, talk, open doors, use containers, buy food, read notices, report observations, lie, gossip, search, store, hide, take, or steal.
4. Wait and watch routines continue without the human.
5. Switch possession in debug mode and verify that knowledge, suspicion, obligations, consequences, and memories stay with bodies and institutions rather than the human.
6. Inspect causal chains in debug mode.
7. Follow or ignore stale public leads only after ordinary life, belief, records, storage, travel, and action parity already work.

## Prototype success criteria

A successful prototype demonstrates:

- a village running for days with no human input;
- roughly 10-30 high-detail agents, or at most a few dozen, with inspectable lives;
- homes, rooms, doors, containers, storage, ownership, custody, money, food, sleep, work, routines, travel, and basic social interaction;
- actor beliefs, actor-known leads, source-bound notebooks, memory staleness, and actor-knowledge-filtered TUI views;
- a theft or missing-property incident discovered through expectation contradiction;
- uncertain testimony and rumor propagation;
- a clerk/reeve/authority opening, refusing, delaying, misfiling, or misclassifying a record from partial information;
- wrong suspicion for legible reasons;
- stale public artifacts;
- debug possession switching with no knowledge leakage;
- deterministic replay and causal ancestry inspection;
- LLM-disabled operation.

Combat, full expedition play, procedural world generation, fantasy, open-ended LLM conversation, macroeconomics, and graphical presentation are not prototype success criteria.

## Strong decision

Build a village whose people can be wrong for the right reasons. Do not build a world that flatters the player, waits for the player, reads the player’s mind, scripts outcomes for the player, or lets language invent reality.
