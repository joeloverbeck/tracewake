# Project Charter

## Name

**Tracewake**.

The name is literal doctrine: actions leave wakes; wakes become traces; traces are observed, ignored, recorded, distorted, erased, and interpreted by ordinary agents.

## One-sentence definition

Tracewake is a causality-first, epistemic, ordinary-life simulation where the human temporarily controls ordinary agents inside a world of needs, routines, beliefs, records, institutions, traces, travel, and fallible causality.

## First implementation target

The first serious implementation target is not an expedition game, combat prototype, procedural quest board, LLM NPC demo, or graphical shell.

It is a small, TUI-playable, neutral medieval-ish village life simulation where ordinary agents sleep, eat, work, own things, store things, talk, expect, notice, misinterpret, remember, forget, lie, report, and act through the same action pipeline as the human-controlled body. The world continues coherently without the human. Possession is only controller binding. Meaningful state changes are event-sourced. No outcome is scripted. LLMs are optional future language surfaces, not simulation brains.

## Product fantasy

You enter a village already in motion.

A miller wakes hungry, checks a strongbox, and finds coins missing because he expected them to be there. A spouse remembers hearing a faint sound but is not sure what it meant. A desperate worker hides money, keeps a job appointment, lies awkwardly when questioned, and later changes plans because fear and hunger conflict. A clerk receives a partial report, creates a record, and the reeve decides whether the office has time and funds to respond. A notice board contains real public artifacts: some current, some stale, some forged, some ignored. The human may possess any ordinary body allowed by the debug harness, but the world does not know or care which body is controlled.

The core pleasure is not being assigned a quest. The core pleasure is discovering a living causal machine from a limited position and intervening through the same affordances available to everyone else.

## Long-term fantasy

Tracewake eventually supports regions with histories before play begins. People age, die, marry, migrate, inherit, join institutions, lose work, spread rumors, bury evidence, create public records, abandon houses, open shops, change routes, and misremember the past. The player enters a current world state that has causes.

Long simulation must not become lore prose. It must create inspectable causal ancestry, public memory, stale information, institutional records, and present consequences.

## Product pillars

### 1. Causality before drama

Events happen because prior state, actor intention, need pressure, belief, routine, environment, institution, regional process, or authored initial condition made them possible. No event exists because the player needs pacing.

### 2. Belief before truth

Agents act from subjective beliefs, not ground truth. Wrong beliefs, stale information, rumor, lies, misidentification, inference, absence-as-evidence, and expectation contradiction are first-class mechanics.

### 3. Ordinary life before adventure

Adventure is meaningful only when ordinary life exists to be disrupted. Hunger, fatigue, safety, sleep, eating, work, homes, storage, ownership, household routines, and basic social interaction are not background flavor. They are the substrate.

### 4. The world has no sacred player entity

The authoritative simulation contains no metaphysically special player character. The human is represented as:

```text
HumanController -> ActorId
```

Possession changes input binding, not reality.

### 5. Every world-affecting player action must be agent-possible

The current body has the same world-affecting actions an AI agent would have under equivalent physical, social, epistemic, resource, and institutional conditions. UI clarity is allowed. Exclusive player verbs are not.

### 6. Institutions are fallible social machines

Law, guilds, households, temples, markets, offices, gangs, and companies act through people, roles, obligations, permissions, prohibitions, records, procedures, resources, delay, bias, jurisdiction, and failure. No guard, court, ledger, or notice board reads ground truth.

### 7. Quests are projections, not ontology

The engine stores incidents, needs, requests, contracts, notices, obligations, promises, rumors, records, leads, investigations, sanctions, and opportunities. A player-facing objective is a view over world state, not an authoritative process.

### 8. Authored causal machinery is allowed; authored event sequences are forbidden

Designers author actions, affordances, norms, roles, needs, routines, HTN methods, institution procedures, trace types, speech acts, initial conditions, scenario seeds, LOD rules, and test fixtures. Designers do not author outcome chains, quest beats, drama triggers, player-conditioned events, or guaranteed story arcs.

### 9. Symbolic agents first

Tracewake v1 agents are inspectable symbolic/planner-driven agents: BDI-style beliefs/desires/intentions, durable projects, HTN routines and procedures, bounded local planning, event-driven replanning, budgets, and failure events. LLMs may later render language. LLMs do not decide truth, plan agents, create hidden facts, or mutate authoritative state.

### 10. TUI-first, playable always

The TUI is not a disposable debug shell. It is the main product interface for a long while. The kernel and TUI harness grow together. Every runnable phase must have a playable TUI acceptance test. A mechanic that cannot be reached, inspected, or tested through the TUI is not done.

### 11. Genre-agnostic core, neutral low-fantasy first domain

The kernel must not contain fantasy assumptions. The first domain is restrained medieval-ish low fantasy because it provides intuitive homes, roads, taverns, offices, notices, imperfect evidence, and fallible institutions. Magic, species, religion, combat, procedural terrain, and graphical presentation are deferred.

### 12. Story is observed, not directed

Story recognition, salience, recap, notebooks, and “interesting event” surfacing are observer layers. They may sift what happened. They may not cause what happens.

## Non-goals

Tracewake is not:

- a quest generator;
- a combat-first RPG;
- an authored narrative RPG;
- a colony sim centered on a player faction;
- a RimWorld-style drama director;
- a Skyrim radiant quest replacement;
- a Shadows-of-Doubt clone with omniscient cases;
- a generic LLM NPC chatbot;
- a graphical game waiting for simulation later;
- a fantasy kernel with generic labels pasted on top.

## The required first miracle

For any surprising event, Tracewake must be able to answer:

- what happened;
- why it was possible;
- who or what caused it;
- what intention, need, belief, routine, institution, environment, or regional process contributed;
- who observed it;
- what traces it left;
- who believes what;
- who is wrong;
- what record, rumor, or institution noticed or ignored it;
- what later events became possible because of it.

If the engine cannot answer these questions, the event is not Tracewake-quality.

## First playable loop

The first serious loop is a **life-possession sandbox**:

1. enter a TUI village as an ordinary body;
2. inspect actor-known needs, location, possessions, beliefs, intentions, and available actions;
3. sleep, eat, work, talk, open doors, use containers, buy food, read notices, report observations, lie, gossip, or steal;
4. switch possession in debug mode and verify that knowledge, suspicion, obligations, and consequences stay with bodies and institutions rather than the human;
5. watch routines continue without the human;
6. inspect causal chains in debug mode;
7. pursue a stale public lead only after ordinary life, belief, records, and travel are already working.

## Prototype success

A successful prototype demonstrates:

- a village running for days without player input;
- hunger, fatigue, safety, sleep, eating, work, storage, homes, doors, rooms, ownership, and possession;
- a theft discovered through expectation contradiction;
- an uncertain witness whose testimony can become rumor;
- a clerk or authority opening a record from partial information;
- a wrong suspicion arising for legible reasons;
- a routine interrupted by modeled causes;
- a notice whose claims can become stale;
- embodied TUI play with actor-knowledge filters;
- debug possession switching with no knowledge leakage;
- deterministic replay of major consequences.

Combat, full expedition play, procedural world generation, open-ended LLM conversation, and graphical presentation are not prototype success criteria.

## Strong decision

Build a town whose people can be wrong for the right reasons. Do not build a world that flatters the player, waits for the player, reads the player's mind, scripts outcomes for the player, or lets language invent reality.
