# Research Survey, Design Conclusions, and Sources

Research reviewed: **2026-06-05**.

This document records the research conclusions that shape Tracewake doctrine. It is not a literature review for its own sake. The goal is to prevent seductive wrong implementations.

## Executive synthesis

Tracewake should combine:

```text
event-sourced world kernel
+ subjective epistemic state
+ BDI-style agent separation
+ durable intentions/projects
+ authored HTN procedures and routines
+ bounded local GOAP-style planning
+ typed speech acts and information channels
+ normative institutions
+ ordinary-life economy
+ explicit LOD
+ questless leads and public artifacts
+ story sifting as observer layer
+ TUI-first embodied play
+ optional validated LLM language surfaces
```

The strongest conclusion is simple: **do not build Tracewake around quests or LLM agents**. Build it around epistemic causality.

## Event sourcing

Event sourcing fits Tracewake because the game must answer how a state came to be, replay histories, reconstruct past states, audit decisions, explain causality, generate records, support debugging, and let beliefs and institutions derive from events rather than omniscient state.

The event log should be authoritative for meaningful changes. Snapshots and compaction are necessary for performance, but significant chains, belief-changing events, institutional records, surviving traces, and active leads must retain ancestry.

Sources:

- Martin Fowler, “Event Sourcing”: https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “What do you mean by ‘Event-Driven’?”: https://martinfowler.com/articles/201701-event-driven.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern”: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing

Design conclusion: event sourcing is not merely persistence. It is the forensic backbone.

## BDI-style agents

The Belief-Desire-Intention model is a good high-level mental architecture because Tracewake needs agents who distinguish what they believe, what they value/want, and what they have committed to doing.

Tracewake should not implement heavyweight academic BDI machinery blindly. It should use the separation as architecture doctrine:

```text
beliefs: subjective, sourced, fallible propositions
desires/values/goals: pressures, motives, life projects, role duties
intentions: durable commitments and selected methods
```

Sources:

- Rao and Georgeff, “BDI Agents: From Theory to Practice”: https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency”: https://link.springer.com/chapter/10.1007/3-540-49057-4_1

Design conclusion: BDI is the mental separation; HTN and bounded local planning are execution tools.

## HTN planning

HTN planning is appropriate for high-level routines and institution procedures because it decomposes abstract tasks into state-dependent methods. Tracewake needs that for workdays, eating, sleeping, reporting theft, investigating, posting notices, recruiting companions, and institutional processes.

HTN methods must remain causal. They describe ways an agent might proceed given beliefs, resources, norms, costs, and risks. They must not guarantee outcome sequences.

Sources:

- Cavazza et al./game-planning lineage and AIIDE material on HTN in games: https://icaps07-satellite.icaps-conference.org/workshop8/Planning%20with%20Hierarchical%20Task%20Networks%20in%20Video%20Games.pdf
- “Offline Planning with Hierarchical Task Networks in Video Games”: https://cdn.aaai.org/AIIDE/2009/AIIDE09-014.pdf
- SHPE/HTN game work: https://www.lamsade.dauphine.fr/~cazenave/papers/MenifCGW2014.pdf

Design conclusion: HTN routines and procedures are allowed causal machinery, not scripts.

## GOAP and bounded local planning

GOAP-style planning is valuable for concrete action sequences: reach a room, open a door, retrieve a tool, inspect a container, speak to a target, flee a place, get food, or deliver an item. It should be bounded and nested under intentions/HTN methods. It should not solve every life problem from scratch every tick.

Sources:

- Jeff Orkin, “Three States and a Plan: The AI of F.E.A.R.”: https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- Game Developer, “Building the AI of F.E.A.R. with Goal Oriented Action Planning”: https://www.gamedeveloper.com/design/building-the-ai-of-f-e-a-r-with-goal-oriented-action-planning
- Peter Higley, GDC “Goal-Oriented Action Planning”: https://media.gdcvault.com/gdc2015/presentations/Higley_Peter_Goal-Oriented_Action_Planning.pdf

Design conclusion: use bounded local planning for tactical means, not as the whole mind.

## Social simulation and knowledge

Talk of the Town is one of the most important references because it treats character knowledge as explicit, propagating, forgettable, misrememberable, and lie-capable.

Prom Week and Comme il Faut show the value of explicit social state, social rules, relationship-sensitive interaction, and social history as gameplay material.

Sources:

- Ryan et al., “Toward Characters Who Observe, Tell, Misremember, and Lie”: https://eis.ucsc.edu/papers/ryanEtAl_TowardCharactersWhoObserveTellMisrememberLie.pdf
- Game AI Pro 3, “Simulating Character Knowledge Phenomena in Talk of the Town”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Talk of the Town dialogue generation: https://cdn.aaai.org/ojs/12877/12877-52-16394-1-2-20201228.pdf
- Prom Week, “Social Physics as Gameplay”: https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Prom Week project page: https://promweek.soe.ucsc.edu/2011/11/12/gameplay-and-social-physics/

Design conclusion: information movement is as important as physical movement.

## Normative multi-agent systems

Normative MAS research is directly relevant because Tracewake institutions need obligations, permissions, prohibitions, roles, sanctions, institutional facts, detection thresholds, and constitutive rules.

A theft is not automatically a prosecution. A report is not automatically trusted. An authority’s belief is not ground truth. A role grants powers and duties but can be violated.

Sources:

- Normative Multi-Agent Systems resources: https://icr.uni.lu/normas/
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems”: https://cdn.aaai.org/KR/2004/KR04-028.pdf
- “Norms in MAS: Definitions and Related Concepts”: https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/
- Springer, “Constitutive Norms in the Design of Normative Multiagent Systems”: https://link.springer.com/chapter/10.1007/978-3-540-87654-0_20

Design conclusion: institutions are simulated social machines, not global enforcement scripts.

## Dynamic LOD

Large-scale agent simulations require explicit level of detail. Tracewake cannot run every citizen at room-level detail for years. It also cannot fake low-detail processes in a way that breaks forensic causality.

Dynamic LOD supports scalability only if lower-detail processes still produce events, preserve ancestry, and can be promoted when relevant.

Sources:

- Van der Wal et al., “Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations”: https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf

Design conclusion: LOD is part of the ontology, not an optimization afterthought.

## Emergent narrative and story sifting

Emergent narrative research is useful only if Tracewake treats story as a phenomenon recognized after events occur. Story sifting, salience scoring, recaps, and notebooks may help users perceive interesting patterns. They must not become directors.

Neighborly is relevant as a community-scale social simulation reference and as evidence that settlement-level social history can be generated through bottom-up simulation.

Sources:

- Ryan et al., “Open Design Challenges for Interactive Emergent Narrative”: https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf
- “Stories from the Bottom Up: Towards Composable Story Sifting Patterns for Simulated Stories”: https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf
- K.M. J. Nelson, “Neighborly: A Sandbox for Simulation-Based Emergent Narrative”: https://www.kmjn.org/publications/Neighborly_CoG22.pdf

Design conclusion: story sifting is an observer/salience layer, not a cause.

## LLM-agent and LLM-social-simulation risks

Generative Agents demonstrated the appeal of LLM-driven social behavior: agents can appear to plan, remember, converse, and form routines. That is inspiring for user experience but dangerous for Tracewake’s authoritative simulation.

LLM social simulation literature repeatedly raises validation problems: black-box behavior, stochasticity, bias, average persona collapse, data contamination, hallucination, inconsistent behavior, and difficulty establishing social-scientific validity. Tracewake’s premise demands replayability, state authority, causal explanation, and epistemic boundaries. An LLM may not be the simulation brain.

Sources:

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior”: https://arxiv.org/abs/2304.03442
- SOTOPIA: https://openreview.net/forum?id=mM7VurbA4r
- LIFELONG-SOTOPIA: https://arxiv.org/abs/2506.12666
- “LLM-Based Social Simulations Require a Boundary”: https://openreview.net/forum?id=1T1SE9xxAB
- Springer validation review: https://link.springer.com/article/10.1007/s10462-025-11412-6
- LLM agent-based simulation review: https://www.nature.com/articles/s41599-024-03611-3

Design conclusion: LLMs render or parse language behind validation. They do not decide reality, plan agents, or grant knowledge.

## Game precedents and anti-precedents

### Dwarf Fortress

Borrow world history, persistent populations, artifacts, causal weirdness, and the fantasy of a world that existed before play. Avoid early opacity and excessive scale before forensic clarity.

Sources:

- Bay 12 Games development notes: https://www.bay12games.com/dwarves/dev.html
- Dwarf Fortress Wiki, world generation: https://dwarffortresswiki.org/index.php/World_generation

### RimWorld

Borrow readable pawn state, needs, alerts, and legible disruption. Reject the AI storyteller as a metaphysical cause.

Sources:

- Official site: https://rimworldgame.com/
- AI Storytellers wiki: https://rimworldwiki.com/wiki/AI_Storytellers

### Skyrim / radiant quests

Borrow the fantasy of public work, notices, taverns, and entering towns. Reject player-targeted task generation, quest-giver faucets, and objective markers that imply truth.

Sources:

- Wired, “Skyrim’s Infinite Quests”: https://www.wired.com/2011/11/skyrim-infinite-quests
- UESP Radiant quests: https://en.uesp.net/wiki/Skyrim:Radiant

### Shadows of Doubt

Borrow independent citizens, homes, jobs, routines, physical inspectability, and evidence-driven investigation. Improve with stronger belief provenance, rumor mechanics, institutions, ordinary life, and no ground-truth UI leakage.

Sources:

- Steam page: https://store.steampowered.com/app/986130/Shadows_of_Doubt/
- ColePowered devblog #8: https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- ColePowered devblog #15: https://colepowered.itch.io/shadows/devlog/78044/shadows-of-doubt-devblog-15-moving-in-the-citizens

### Ultima-style schedules

Borrow daily routine believability. Upgrade schedules into defeasible intentions.

Sources:

- Digital Lycaeum on NPC schedules: https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/
- Ultima VII schedule data: https://ultima.fandom.com/wiki/Ultima_VII_Internal_Formats_-_SCHEDULE.DAT

### The Sims and smart objects

Borrow object-local affordances and domestic behavior. Upgrade affordances with ownership, law, privacy, belief, traces, and institution hooks.

Sources:

- Tirrell, “Dumb People, Smart Objects”: https://www.gamephilosophy.org/wp-content/uploads/confmanuscripts/pcg2012/Tirrell%202012%20-Dumb-People-Smart-Objects-The-Sims-and-the-Distributed-Self.pdf
- Game AI Pro 3, “Ambient Interactions”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

### Left 4 Dead AI Director

Treat as a counterexample for Tracewake causality. Pacing analysis may be diagnostic. Dramatic direction must not cause events.

Sources:

- Valve, “The AI Systems of Left 4 Dead”: https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Valve, “Replayable Cooperative Game Design”: https://cdn.akamai.steamstatic.com/apps/valve/2009/GDC2009_ReplayableCooperativeGameDesign_Left4Dead.pdf

## First corrected target

The first playable target is a TUI village life-possession sandbox. Investigation, institutions, notices, stale leads, and expeditions are important, but they are staged after ordinary life and action parity work. Combat is not in the first serious vertical slice.
