# Research Decisions and Source Notes

Research reviewed and source links checked: **2026-06-05**.

This document is not a literature review. It records the research conclusions that shape Tracewake doctrine and the precedents that must be borrowed from or rejected.

## Executive synthesis

Tracewake should combine:

```text
event-sourced world kernel
+ subjective epistemic state
+ BDI-style mental separation
+ durable intentions/projects
+ authored HTN routines and procedures
+ bounded local GOAP-style planning
+ typed speech acts and information channels
+ normative institutions
+ ordinary-life economy
+ explicit LOD
+ questless leads and public artifacts
+ story sifting as observer layer
+ TUI-first embodied play
+ optional validated LLM language surfaces
+ Rust-first implementation discipline
```

The strongest conclusion is simple:

```text
Do not build Tracewake around quests.
Do not build Tracewake around LLM agents.
Do not build Tracewake around graphics.
Build it around epistemic causality.
```

## Event sourcing

Event sourcing fits Tracewake because the product must answer how a state came to be, replay histories, reconstruct significant past states, audit decisions, support causal debugging, and let beliefs and institutions derive from events rather than omniscient current state.

Design conclusion:

```text
Event sourcing is not merely persistence.
It is the forensic backbone.
```

Snapshots and compaction are allowed for performance only if significant causal ancestry survives.

Sources:

- Martin Fowler, “Event Sourcing”: https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “What do you mean by ‘Event-Driven’?”: https://martinfowler.com/articles/201701-event-driven.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern”: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing

## Rust-first implementation discipline

Tracewake should be Rust-first because the project needs deterministic replay discipline, memory safety, long-term maintainability, performance, data-model rigor, and serious tooling without accepting C++ as the default systems-language compromise.

Design conclusion:

```text
Rust is a foundation constraint.
Specific crates, UI libraries, persistence engines, and ECS/database choices are architecture decisions.
```

The foundation should prevent casual rewrites into dynamic prototypes that cannot preserve replay, safety, and inspectability. It should not freeze libraries prematurely.

Sources:

- Rust language project: https://www.rust-lang.org/
- Rust standard library documentation: https://doc.rust-lang.org/std/

## BDI-style agents

The Belief-Desire-Intention model is useful because Tracewake needs explicit separation between what an agent believes, what pressures or values matter, and what the agent has committed to doing.

Tracewake should not blindly implement heavyweight academic BDI machinery. It should adopt the separation:

```text
beliefs: subjective, sourced, fallible propositions
desires/values/goals: needs, motives, duties, projects, preferences, relationships
intentions: durable commitments and selected methods
```

Design conclusion:

```text
BDI is the mental architecture.
HTN and bounded local planning are execution tools.
```

Sources:

- Rao and Georgeff, “BDI Agents: From Theory to Practice”: https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency”: https://link.springer.com/chapter/10.1007/3-540-49057-4_1

## HTN planning

HTN planning is appropriate for high-level routines and institutional procedures because it decomposes abstract tasks into state-dependent methods.

Tracewake needs HTN-style methods for:

- sleep/eat/work routines;
- household coordination;
- search and recovery;
- reporting;
- institutional intake;
- notice posting;
- travel preparation;
- work procedures;
- ordinary social procedures.

Design conclusion:

```text
HTN methods are allowed causal machinery, not scripts.
```

Sources:

- “Planning with Hierarchical Task Networks in Video Games”: https://icaps07-satellite.icaps-conference.org/workshop8/Planning%20with%20Hierarchical%20Task%20Networks%20in%20Video%20Games.pdf
- “Offline Planning with Hierarchical Task Networks in Video Games”: https://cdn.aaai.org/AIIDE/2009/AIIDE09-014.pdf
- SHPE/HTN game work: https://www.lamsade.dauphine.fr/~cazenave/papers/MenifCGW2014.pdf

## GOAP and bounded local planning

GOAP-style planning is valuable for concrete action sequences: reach a room, open a door, retrieve a tool, inspect a container, flee, get food, deliver an item, or reach a person to speak.

It must be bounded and nested under intentions/HTN methods. It must not become the whole mind.

Design conclusion:

```text
Use bounded local planning for tactical means.
Do not solve every life problem from scratch every tick.
```

Sources:

- Jeff Orkin, “Three States and a Plan: The AI of F.E.A.R.”: https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- Game Developer, “Building the AI of F.E.A.R. with Goal Oriented Action Planning”: https://www.gamedeveloper.com/design/building-the-ai-of-f-e-a-r-with-goal-oriented-action-planning
- Peter Higley, GDC “Goal-Oriented Action Planning”: https://media.gdcvault.com/gdc2015/presentations/Higley_Peter_Goal-Oriented_Action_Planning.pdf

## Social simulation and knowledge

Talk of the Town is one of the most important references because it treats character knowledge as explicit, propagating, forgettable, misrememberable, and lie-capable.

Prom Week and Comme il Faut show the value of explicit social state, social rules, relationship-sensitive interaction, and social history as gameplay material.

Design conclusion:

```text
Information movement is as important as physical movement.
```

Sources:

- Ryan et al., “Toward Characters Who Observe, Tell, Misremember, and Lie”: https://eis.ucsc.edu/papers/ryanEtAl_TowardCharactersWhoObserveTellMisrememberLie.pdf
- Game AI Pro 3, “Simulating Character Knowledge Phenomena in Talk of the Town”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Talk of the Town dialogue generation: https://cdn.aaai.org/ojs/12877/12877-52-16394-1-2-20201228.pdf
- Prom Week, “Social Physics as Gameplay”: https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Prom Week project page: https://promweek.soe.ucsc.edu/2011/11/12/gameplay-and-social-physics/

## Normative multi-agent systems

Normative MAS work is relevant because Tracewake institutions need obligations, permissions, prohibitions, powers, roles, sanctions, institutional facts, detection thresholds, and constitutive rules.

A theft is not automatically a prosecution. A report is not automatically trusted. A role can grant a power while still being misused.

Design conclusion:

```text
Institutions are simulated social machines, not global enforcement scripts.
```

Sources:

- Normative Multi-Agent Systems resources: https://icr.uni.lu/normas/
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems”: https://cdn.aaai.org/KR/2004/KR04-028.pdf
- “Norms in MAS: Definitions and Related Concepts”: https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/
- “Constitutive Norms in the Design of Normative Multiagent Systems”: https://link.springer.com/chapter/10.1007/978-3-540-87654-0_20

## Dynamic LOD

Large-scale agent simulation needs explicit level of detail. Tracewake cannot run every citizen at room-level detail for years. It also cannot fake low-detail processes in a way that breaks forensic causality.

Design conclusion:

```text
LOD is ontology, not merely optimization.
Low detail must still emit structured causal summary events.
```

Sources:

- Van der Wal et al., “Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations”: https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf

## Emergent narrative and story sifting

Emergent narrative research is useful only if story is recognized after events occur.

Story sifting, salience scoring, recaps, and notebooks may help users perceive interesting patterns. They must not become directors.

Neighborly is relevant as a community-scale social simulation reference and as evidence that settlement-level social history can be generated through bottom-up simulation.

Design conclusion:

```text
Story sifting is an observer/salience layer, not a cause.
```

Sources:

- Ryan et al., “Open Design Challenges for Interactive Emergent Narrative”: https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf
- “Stories from the Bottom Up: Towards Composable Story Sifting Patterns for Simulated Stories”: https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf
- K.M. J. Nelson, “Neighborly: A Sandbox for Simulation-Based Emergent Narrative”: https://www.kmjn.org/publications/Neighborly_CoG22.pdf

## LLM-agent and LLM-social-simulation risks

Generative Agents demonstrated the appeal of LLM-driven social behavior: agents can appear to plan, remember, converse, and form routines.

That is inspiring for language surfaces. It is dangerous for Tracewake authority.

LLM social simulation literature repeatedly raises validation hazards: stochasticity, black-box behavior, bias, average persona collapse, hallucination, contamination, inconsistency, and difficulty establishing social-scientific validity.

Tracewake needs replayability, causal explanation, actor-knowledge boundaries, provenance, and testability.

Design conclusion:

```text
LLMs may render or parse language behind typed validation.
They may not decide reality, plan agents, create evidence, grant knowledge, or create quests.
```

Sources:

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior”: https://arxiv.org/abs/2304.03442
- SOTOPIA: https://openreview.net/forum?id=mM7VurbA4r
- LIFELONG-SOTOPIA: https://arxiv.org/abs/2506.12666
- “LLM-Based Social Simulations Require a Boundary”: https://openreview.net/forum?id=1T1SE9xxAB
- LLM agent-based simulation review: https://www.nature.com/articles/s41599-024-03611-3
- Validation review: https://link.springer.com/article/10.1007/s10462-025-11412-6

## Dwarf Fortress

Borrow:

- long histories;
- persistent populations;
- artifacts;
- causal weirdness;
- the fantasy of a world that existed before play.

Reject for v1:

- excessive early opacity;
- scale before forensic clarity.

Sources:

- Bay 12 Games development notes: https://www.bay12games.com/dwarves/dev.html
- Dwarf Fortress Wiki, world generation: https://dwarffortresswiki.org/index.php/World_generation

## RimWorld

Borrow:

- readable pawn state;
- needs;
- alerts;
- legible disruption.

Reject:

- AI storyteller causation as a kernel principle.

Sources:

- Official site: https://rimworldgame.com/
- AI Storytellers wiki: https://rimworldwiki.com/wiki/AI_Storytellers

## Skyrim and radiant quests

Borrow:

- public work fantasy;
- towns;
- notices;
- taverns;
- rumors;
- local opportunities.

Reject:

- player-targeted task generation;
- quest-giver faucets;
- objective markers to truth;
- guaranteed targets;
- guaranteed reward payout;
- completion flags replacing evidence/procedure.

Sources:

- Wired, “Skyrim's Infinite Quests”: https://www.wired.com/2011/11/skyrim-infinite-quests
- UESP Radiant quests: https://en.uesp.net/wiki/Skyrim:Radiant

## Shadows of Doubt

Borrow:

- independent citizens;
- homes;
- jobs;
- routines;
- physical inspectability;
- evidence-driven investigation fantasy.

Improve with:

- stronger belief provenance;
- rumor and testimony mechanics;
- fallible institutions;
- ordinary life as substrate;
- stricter actor-knowledge filtering;
- no ground-truth UI leakage.

Sources:

- Steam page: https://store.steampowered.com/app/986130/Shadows_of_Doubt/
- ColePowered devblog #8: https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- ColePowered devblog #15: https://colepowered.itch.io/shadows/devlog/78044/shadows-of-doubt-devblog-15-moving-in-the-citizens

## Ultima-style schedules

Borrow:

- daily routine believability;
- places and schedules as readable ordinary-life scaffolding.

Upgrade:

- schedules become defeasible intentions rather than teleporting timetables.

Sources:

- Digital Lycaeum on NPC schedules: https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/
- Ultima VII schedule data: https://ultima.fandom.com/wiki/Ultima_VII_Internal_Formats_-_SCHEDULE.DAT

## The Sims and smart objects

Borrow:

- object-local affordances;
- domestic behavior;
- legible needs;
- spatial routines.

Upgrade:

- affordances must include ownership, access, privacy, belief, traces, law, and institution hooks.

Sources:

- Tirrell, “Dumb People, Smart Objects”: https://www.gamephilosophy.org/wp-content/uploads/confmanuscripts/pcg2012/Tirrell%202012%20-Dumb-People-Smart-Objects-The-Sims-and-the-Distributed-Self.pdf
- Game AI Pro 3, “Ambient Interactions”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

## Left 4 Dead AI Director

Treat the AI Director as a useful counterexample for Tracewake causality.

Borrow:

- clarity that pacing systems can shape player experience in other genres;
- diagnostic vocabulary for salience and tension analysis.

Reject:

- runtime dramatic direction as a cause of world events.

Sources:

- Valve, “The AI Systems of Left 4 Dead”: https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Valve, “Replayable Cooperative Game Design”: https://cdn.akamai.steamstatic.com/apps/valve/2009/GDC2009_ReplayableCooperativeGameDesign_Left4Dead.pdf

## Final research decision

Build a small, inspectable, TUI-playable ordinary-life village first.

Use symbolic epistemic agents, event-sourced causality, fallible institutions, structured speech acts, and actor-filtered view models.

Let story be discovered after the fact.

Do not let quests, LLMs, graphics, scale, or drama direction become the foundation.
