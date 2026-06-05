# Research Survey and Precedent Map

Research reviewed: 2026-06-05.

This document summarizes relevant implementations and research, then states what to borrow and what to reject.

## Executive synthesis

The desired game sits at the intersection of:

- living-world simulation;
- agent belief and knowledge modeling;
- BDI / GOAP / HTN-style planning;
- normative multi-agent systems;
- emergent narrative;
- event sourcing and causal replay;
- procedural content generation, but only as a subordinate tool;
- LLM-based agent work, but with hard limits.

The strongest conclusion from the research pass: **the project should not be built around procedural quest generation**. Procedural quest generation can help create UI projections or institutional artifacts, but the foundation must be belief-rich, event-sourced simulation.

## Game precedents

### Dwarf Fortress

**Relevance:** closest spiritual ancestor for long-running simulated history, civilizations, sites, populations, artifacts, and historical records.

**Borrow:**

- world history as substrate;
- simulation before presentation;
- artifacts and historical figures as event-linked objects;
- acceptance of surprising emergent chains;
- persistent world state not centered on the player.

**Reject or avoid:**

- opacity as a default user experience;
- excessive initial scale;
- too many systems before forensic clarity;
- lore generation that cannot be interrogated through causal chains.

**Design lesson:** Dwarf Fortress proves players will tolerate rough presentation if the world feels causally rich. This project should add sharper epistemic modeling: who knows, who is wrong, who can prove what.

### RimWorld

**Relevance:** a masterclass in colony-scale readability, pawn needs, interpersonal drama, and accessible simulation UI.

**Borrow:**

- visible needs and consequences;
- strong event notifications;
- readable pawn state;
- medical, social, work-priority clarity;
- small-scale drama from interacting systems.

**Reject:**

- AI storyteller as metaphysical director;
- challenge curves and threat pacing;
- events injected because the colony needs drama;
- player-colony centrality.

**Design lesson:** RimWorld is fantastic, but its storyteller model violates causality-first design. The new project should support story recognition, not story authorship.

### Skyrim / Radiant quests

**Relevance:** demonstrates repeatable task generation from modular pieces.

**Borrow:**

- parametrized tasks;
- reusable locations, actors, objects, and rewards;
- repeatable public work such as bounties.

**Reject:**

- player-progress-centered task generation;
- quest giver as content faucet;
- objective markers that imply truth rather than belief;
- infinite errands as substitute for living institutions.

**Design lesson:** A notice to kill bandits should be produced by harm, report, assessment, budget, writing, and posting. It should not be produced by a quest template looking for player content.

### Shadows of Doubt

**Relevance:** likely the strongest modern commercial precedent for a simulated city where many citizens have jobs, routines, homes, and independent behavior.

**Borrow:**

- investigation in a procedural but physically inspectable city;
- citizens with jobs and places to be;
- crimes that leave usable evidence;
- every building/room as explorable substrate;
- willingness to let bugs reveal real simulation interactions.

**Reject or improve:**

- murder-case focus as the whole loop;
- simulation that may still be narrower than a general social/institutional world;
- hard-coded detective-game assumptions.

**Design lesson:** The famous “toilet queue stops murder” anecdote is important because it reveals a real simulation, not fake flavor. The target game should embrace this class of causal weirdness while adding stronger institutional and belief layers.

### Ultima-style NPC schedules

**Relevance:** early proof that towns feel real when people sleep, eat, work, and close shops.

**Borrow:**

- daily routines;
- shops that close;
- homes that matter;
- ordinary activity as immersion substrate.

**Reject:**

- clock scripts that ignore actual conditions;
- schedules as animation loops rather than defeasible intentions.

**Design lesson:** The target game needs “why” behind the schedule. The blacksmith works because she has obligations, materials, expectations, and motives, not because the schedule row says `BLACKSMITH 08:00`.

### The Sims and smart objects

**Relevance:** smart objects and motive advertisements are useful for ordinary-life simulation.

**Borrow:**

- objects advertising possible interactions;
- object-local affordance data;
- environmental support for needs;
- domestic life as systemic content.

**Reject:**

- object affordances that ignore social permission, ownership, law, and belief;
- need satisfaction as the whole mind.

**Design lesson:** Smart objects are excellent if connected to property, norms, traces, and belief. A bed advertises sleep; a stranger using it may create trespass evidence.

### Caves of Qud mythic biographies

**Relevance:** useful contrast for procedural history and cultural mythmaking.

**Borrow:**

- historical artifacts as gameplay texture;
- generated biographies and cultural records;
- acceptance that history can be retold through ideology.

**Reject for this project’s foundation:**

- ex-post rationalization as substitute for causal simulation.

**Design lesson:** Mythmaking is excellent later, but first the game needs actual cause-and-effect chains. After that, cultures can misremember them.

### Left 4 Dead AI Director

**Relevance:** perfect counterexample.

**Borrow:**

- pacing analysis for UI or optional modes only;
- intensity measurement as a diagnostic tool.

**Reject:**

- dramatic pacing as world cause;
- spawning threats to maximize excitement;
- invisible director controlling consequences.

**Design lesson:** The target project should not have an AI Director. It can have an observer that notices salience.

## Research and architecture precedents

### BDI agents: Belief, Desire, Intention

BDI architectures separate what an agent believes, wants, and commits to doing. This is directly aligned with the desired world because agents must act from subjective information rather than global truth.

**Borrow:**

- beliefs as informational state;
- desires/goals as motivational state;
- intentions as committed plans;
- practical reasoning that cannot be reduced to simple utility scoring.

**Design adaptation:** Use BDI as the high-level mental model, not necessarily a formal logic implementation. The game needs inspectable beliefs and durable commitments.

### GOAP: Goal-Oriented Action Planning

GOAP is useful for local flexible planning: given a desired world state, search for action sequences whose preconditions and effects can achieve it.

**Borrow:**

- action preconditions and effects;
- reusable primitive actions;
- planning from current believed state;
- failure-driven replanning.

**Limit:** GOAP alone can make agents myopic or expensive if every actor replans constantly. It needs triage, intention persistence, and hierarchical decomposition.

### HTN planning

HTN planning decomposes high-level tasks into lower-level subtasks. It is useful for socially legible activities such as “investigate theft,” “prepare workday,” “post bounty,” or “recover debt.”

**Borrow:**

- decomposed high-level behaviors;
- authorable methods;
- predictable structure with flexible leaves;
- good fit for institutions and routines.

**Limit:** HTNs can become disguised scripts if not grounded in real preconditions and interruptible execution.

### Talk of the Town

This is one of the most important references. It models character knowledge phenomena: observation, propagation, misremembering, confabulation, lies, and subjective beliefs.

**Borrow:**

- character knowledge as core mechanic;
- beliefs with provenance;
- rumor and memory errors;
- dialogue derived from subjective knowledge;
- town-scale social knowledge propagation.

**Design lesson:** The missing piece in many sims is not more needs or more jobs. It is information ecology.

### Prom Week / Comme il Faut

Prom Week’s social physics treats social state as gameplay: relationships, traits, history, social rules, and interactions determine what characters want and can do.

**Borrow:**

- explicit social state;
- social actions as mechanical affordances;
- rules that produce character-specific behavior;
- history-sensitive social outcomes.

**Design lesson:** Friendship, status, debt, shame, authority, and kinship need mechanical consequences, not just dialogue flavor.

### Versu and social practices

Versu emphasizes autonomous agents, social practices, and the possibility of experiencing a scenario from different characters’ perspectives.

**Borrow:**

- multiple-role perspective;
- social practices as reusable structures;
- character autonomy inside interactive narrative.

**Design lesson:** The player’s ability to switch characters becomes more powerful if social roles and knowledge states are real.

### Normative multi-agent systems

Normative MAS research models obligations, permissions, prohibitions, sanctions, and institutions.

**Borrow:**

- laws as explicit norms;
- violation detection as separate from violation occurrence;
- institutional roles;
- permissions and obligations by status;
- sanctions and enforcement.

**Design lesson:** “Crime” is not a boolean. It is an action interpreted through norms, jurisdiction, evidence, authority, and process.

### Procedural quest generation

Doran & Parberry, CONAN, and later quest-generation systems show that quests can be generated from structures, motivations, planning, and world facts.

**Borrow:**

- taxonomy of motivations;
- planning-based sequencing;
- generating task descriptions from facts;
- world-state constraints.

**Reject as foundation:**

- quest as primary artifact;
- player-targeted reward loops;
- content generation detached from actual institutional life.

**Design lesson:** Use quest research to format public requests and task projections. Do not let it own the simulation.

### Emergent narrative research

Emergent narrative treats stories as arising bottom-up from autonomous agents in a simulated storyworld.

**Borrow:**

- story recognition;
- story support rather than story control;
- modular content;
- salience detection.

**Design lesson:** The game may help the player notice narrative patterns, but should not manufacture them to satisfy arcs.

### Event sourcing

Event sourcing stores state changes as a sequence of events and allows reconstruction of past states from the event log.

**Borrow:**

- authoritative event log;
- replay and debugging;
- causal reconstruction;
- projections for current state, UI, memories, institutions, and analytics.

**Design lesson:** A causal simulation without event sourcing will become impossible to debug. The event log is both architecture and gameplay material.

### LLM generative agents and LLM social simulation

Generative Agents showed that LLM-backed agents can produce believable routines, memories, reflections, and social coordination. SOTOPIA and later validation work show that language agents still struggle with social goal completion, long-term interaction history, validation, black-box behavior, and reliability.

**Borrow:**

- natural-language memory summarization;
- reflection as optional compression;
- dialogue surface generation;
- notice/rumor paraphrase;
- player-facing summaries.

**Reject:**

- LLM as authoritative simulation brain;
- unvalidated generated facts;
- hidden state mutation through prose;
- relying on believability as proof of correctness.

**Design lesson:** The simulator decides. The LLM renders.

## Overall recommendation

The foundation should combine:

```text
Event-sourced world kernel
+ BDI-style subjective agents
+ HTN high-level behavior
+ GOAP local action planning
+ explicit traces and information channels
+ normative institutions
+ questless public artifacts
+ salience UI
+ optional grounded language generation
```

The first playable target should be a small village with theft, rumor, law, notices, work, sleep, and possession switching.
