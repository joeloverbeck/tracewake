# Research Decisions and Source Notes

Research reviewed for this replacement foundation set: 2026-06-05.

This document is not a literature review. It records decisions that shape Tracewake doctrine and the precedents that must be borrowed from, bounded, or rejected.

## Executive synthesis

Tracewake should combine:

```text
event-sourced world kernel
+ typed claims/propositions
+ subjective epistemic state
+ BDI-style mental separation
+ durable intentions/projects
+ authored HTN routines and procedures
+ bounded local GOAP/STRIPS-style planning
+ structured speech acts and information channels
+ normative institutions and households
+ ordinary-life survival/economy
+ explicit LOD and regional processes
+ questless source-bound leads
+ story sifting as observer layer
+ TUI-first embodied play
+ optional validated LLM language surfaces
+ Rust-first implementation discipline
```

The strongest conclusion remains:

```text
Do not build Tracewake around quests.
Do not build Tracewake around LLM agents.
Do not build Tracewake around graphics.
Do not build Tracewake around scale-first opacity.
Build it around epistemic causality.
```

## Event sourcing, replay, CQRS, projections, and snapshots

Decision:

```text
Event sourcing is not merely persistence.
It is the forensic backbone.
```

Tracewake needs event sourcing because it must reconstruct significant histories, audit decisions, replay scenarios, explain wrong beliefs, rebuild projections, preserve causal ancestry, and distinguish ground truth from records and beliefs. CQRS/projection patterns are useful because embodied TUI, debug inspectors, actor notebooks, institutional indexes, and story-sifter summaries are read models over event-derived state.

Snapshots and compaction are necessary for long simulation, but they may not erase significant ancestry.

Sources:

- Martin Fowler, “Event Sourcing”: https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “CQRS”: https://martinfowler.com/bliki/CQRS.html
- Martin Fowler, “What do you mean by ‘Event-Driven’?”: https://martinfowler.com/articles/201701-event-driven.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern”: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Greg Young, CQRS/Event Sourcing writings: https://cqrs.wordpress.com/documents/

## Rust-first implementation discipline

Decision:

```text
Rust is a foundation constraint for the authoritative core.
Specific crates, storage engines, UI libraries, and architecture layout are not foundation decisions.
```

Rust is appropriate for the authoritative simulation core because Tracewake requires determinism discipline, memory safety, maintainability, performance, data-model rigor, and serious tooling without C++ as the default compromise.

Sources:

- Rust language project: https://www.rust-lang.org/
- Rust standard library documentation: https://doc.rust-lang.org/std/

## BDI-style agents and bounded rational agency

Decision:

```text
BDI is the mental separation.
It is not a demand for heavyweight academic BDI machinery.
```

Tracewake uses beliefs, desires/values/goals/needs/duties, and intentions as distinct structures because the game requires agents to act from subjective information, retain commitments, and explain decisions. The BDI tradition is valuable because it separates plan selection from execution and treats intentions as commitments, which helps avoid utility jitter.

Tracewake should not assume classical BDI solves memory, social propagation, real-time planning, or institutions. Those are additional systems.

Sources:

- Rao and Georgeff, “BDI Agents: From Theory to Practice”: https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency”: https://link.springer.com/chapter/10.1007/3-540-49057-4_1
- Michael Bratman, *Intention, Plans, and Practical Reason*.

## HTN planning

Decision:

```text
HTN methods are causal procedures, not scripts.
```

HTN-style decomposition fits ordinary routines and institutions: sleep/eat/work, household behavior, search, report, office intake, posting notices, travel preparation, job procedures, and future organization workflows. HTN methods must be state-dependent, interruptible, failure-capable, and NPC-usable.

Sources:

- “Planning with Hierarchical Task Networks in Video Games”: https://icaps07-satellite.icaps-conference.org/workshop8/Planning%20with%20Hierarchical%20Task%20Networks%20in%20Video%20Games.pdf
- “Offline Planning with Hierarchical Task Networks in Video Games”: https://cdn.aaai.org/AIIDE/2009/AIIDE09-014.pdf
- SHPE/HTN game work: https://www.lamsade.dauphine.fr/~cazenave/papers/MenifCGW2014.pdf

## GOAP / STRIPS-style bounded local planning

Decision:

```text
Use bounded local planning for concrete means.
Do not solve entire lives from scratch every tick.
```

GOAP-style planning is useful for reach/open/retrieve/eat/search/flee/deliver/report sequences, but must be nested under explicit intentions or HTN methods. It is an execution tool, not the whole mind.

Sources:

- Jeff Orkin, “Three States and a Plan: The AI of F.E.A.R.”: https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- Game Developer, “Building the AI of F.E.A.R. with Goal Oriented Action Planning”: https://www.gamedeveloper.com/design/building-the-ai-of-f-e-a-r-with-goal-oriented-action-planning
- Peter Higley, GDC “Goal-Oriented Action Planning”: https://media.gdcvault.com/gdc2015/presentations/Higley_Peter_Goal-Oriented_Action_Planning.pdf

## Social simulation, character knowledge, and memory fallibility

Decision:

```text
Information movement is as important as physical movement.
```

Talk of the Town is a key precedent because it treats character knowledge as explicit, propagating, forgettable, misrememberable, and lie-capable. Tracewake should adopt the importance of subjective knowledge but anchor it in typed claims, event sourcing, TUI possession boundaries, and institutional records.

Sources:

- Ryan et al., “Toward Characters Who Observe, Tell, Misremember, and Lie”: https://eis.ucsc.edu/papers/ryanEtAl_TowardCharactersWhoObserveTellMisrememberLie.pdf
- Game AI Pro 3, “Simulating Character Knowledge Phenomena in Talk of the Town”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Talk of the Town dialogue generation: https://cdn.aaai.org/ojs/12877/12877-52-16394-1-2-20201228.pdf

## Prom Week, Comme il Faut, and social physics

Decision:

```text
Social state should be explicit enough to be playable, inspectable, and wrong.
```

Prom Week / Comme il Faut demonstrate the value of explicit social state, relationship-sensitive interactions, social history, and social rules as gameplay material. Tracewake should borrow explicit social machinery while rejecting any move toward authored story beats or social interactions detached from physical/recorded causality.

Sources:

- Prom Week, “Social Physics as Gameplay”: https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Prom Week project page: https://promweek.soe.ucsc.edu/2011/11/12/gameplay-and-social-physics/
- Comme il Faut / social games research: https://users.soe.ucsc.edu/~michaelm/publications/AIIDE10-CiF.pdf

## Normative multi-agent systems and institutions

Decision:

```text
Institutions are social machines, not global enforcement scripts.
```

Normative MAS work is useful because Tracewake needs obligations, permissions, prohibitions, powers, roles, sanctions, proof rules, constitutive norms, and institutional facts. A violation is not detection. A report is not proof. A role may have a power and misuse it.

Sources:

- Normative Multi-Agent Systems resources: https://icr.uni.lu/normas/
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems”: https://cdn.aaai.org/KR/2004/KR04-028.pdf
- “Norms in MAS: Definitions and Related Concepts”: https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/
- “Constitutive Norms in the Design of Normative Multiagent Systems”: https://link.springer.com/chapter/10.1007/978-3-540-87654-0_20

## Agent-based social simulation and validation

Decision:

```text
No-human simulation is necessary but not sufficient.
Validation must inspect causality, beliefs, records, traces, and acceptance gates.
```

Agent-based social simulation supports bottom-up emergence, but Tracewake cannot accept plausible aggregate behavior alone. The product requires replayable, actor-filtered, source-bound explanation. Validation must test no-human operation, but also test actor-knowledge leakage, replay, event ancestry, institutions, and TUI playability.

Sources:

- Epstein and Axtell, *Growing Artificial Societies: Social Science from the Bottom Up*.
- Journal of Artificial Societies and Social Simulation: https://www.jasss.org/
- Verification & Validation of Agent Based Simulations using VOMAS: https://arxiv.org/abs/1708.02361

## Dynamic LOD and multi-resolution simulation

Decision:

```text
LOD is ontology, not merely optimization.
Low detail must still emit structured causal summary events.
```

Large worlds cannot simulate every person at room-level detail forever. Multi-resolution simulation is necessary, but low-LOD people must remain people and promotion/demotion must preserve ancestry.

Sources:

- Van der Wal et al., “Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations”: https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf

## Emergent narrative and story sifting

Decision:

```text
Story sifting is observer/salience machinery.
It must never become a director.
```

Emergent narrative research is valuable for recognizing and summarizing interesting event patterns after they happen. It is dangerous if it becomes a cause of events. Tracewake may use story sifting for debug summaries, no-human recaps, actor-filtered summaries, and salience. It may not spawn clues, select culprits, repair pacing, or escalate stakes.

Sources:

- Ryan et al., “Open Design Challenges for Interactive Emergent Narrative”: https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf
- “Stories from the Bottom Up: Towards Composable Story Sifting Patterns for Simulated Stories”: https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf
- K.M. J. Nelson, “Neighborly: A Sandbox for Simulation-Based Emergent Narrative”: https://www.kmjn.org/publications/Neighborly_CoG22.pdf

## LLM agents and LLM social simulation risks

Decision:

```text
LLMs may render and parse language behind validation.
They may not decide reality, plan agents, create evidence, grant knowledge, or create quests.
```

Generative Agents demonstrated the appeal of LLM-driven social behavior: agents can appear to remember, plan, converse, and coordinate. That is inspiring for language surfaces. It is dangerous for Tracewake authority because Tracewake needs replayability, causal explanation, actor-knowledge boundaries, provenance, heterogeneity, and deterministic tests.

Recent LLM social simulation work reinforces the need for boundaries: stochasticity, black-box behavior, bias, average-persona collapse, hallucination, validation difficulty, and insufficient variance/heterogeneity all threaten simulation claims.

Sources:

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior”: https://arxiv.org/abs/2304.03442
- SOTOPIA: https://openreview.net/forum?id=mM7VurbA4r
- LIFELONG-SOTOPIA: https://arxiv.org/abs/2506.12666
- “LLM-Based Social Simulations Require a Boundary”: https://arxiv.org/abs/2506.19806
- LLM agent-based simulation review: https://www.nature.com/articles/s41599-024-03611-3
- Validation review: https://link.springer.com/article/10.1007/s10462-025-11412-6
- AgentSociety large-scale LLM-driven simulation: https://arxiv.org/abs/2502.08691

## Dwarf Fortress

Borrow:

- long histories;
- persistent populations;
- artifacts;
- causal weirdness;
- world-before-player fantasy;
- records and memory as play material.

Reject for v1:

- scale before forensic clarity;
- opacity as product identity;
- ordinary actors becoming unreadable before the core proof works.

Sources:

- Bay 12 Games development notes: https://www.bay12games.com/dwarves/dev.html
- Dwarf Fortress Wiki, world generation: https://dwarffortresswiki.org/index.php/World_generation

## RimWorld

Borrow:

- readable pawn state;
- needs;
- alerts;
- legible disruption;
- UI clarity for complex state.

Reject:

- AI storyteller causation as a kernel principle;
- drama pacing as event cause.

Sources:

- Official site: https://rimworldgame.com/
- AI Storytellers wiki: https://rimworldwiki.com/wiki/AI_Storytellers

## Skyrim / Oblivion radiant quests

Borrow:

- public work fantasy;
- towns;
- notices;
- taverns;
- rumors;
- local opportunities;
- readable requests.

Reject:

- player-targeted task generation;
- quest-giver faucets;
- objective markers to truth;
- guaranteed targets;
- guaranteed reward payout;
- completion flags replacing evidence/procedure;
- world waiting for player acceptance.

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
- typed claim substrate;
- rumor/testimony mechanics;
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

- schedules become defeasible intentions rather than teleporting timetables;
- routines can fail, misfire, or be interrupted by needs, beliefs, weather, access, social events, or institutions.

Sources:

- Digital Lycaeum on NPC schedules: https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/
- Ultima VII schedule data: https://ultima.fandom.com/wiki/Ultima_VII_Internal_Formats_-_SCHEDULE.DAT

## The Sims and smart objects

Borrow:

- object-local affordances;
- domestic behavior;
- legible needs;
- spatial routines;
- ordinary-life comedy and consequence.

Upgrade:

- affordances include ownership, access, privacy, belief, traces, law/norms, records, and institutions;
- needs are pressures, not the whole mind.

Sources:

- Tirrell, “Dumb People, Smart Objects”: https://www.gamephilosophy.org/wp-content/uploads/confmanuscripts/pcg2012/Tirrell%202012%20-Dumb-People-Smart-Objects-The-Sims-and-the-Distributed-Self.pdf
- Game AI Pro 3, “Ambient Interactions”: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

## Left 4 Dead AI Director

Treat the AI Director as a useful counterexample for Tracewake causality.

Borrow:

- the clarity that pacing systems can shape player experience in other genres;
- diagnostic vocabulary for salience and tension analysis.

Reject:

- runtime dramatic direction as a cause of world events;
- player-state-based event injection as kernel doctrine.

Sources:

- Valve, “The AI Systems of Left 4 Dead”: https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Valve, “Replayable Cooperative Game Design”: https://cdn.akamai.steamstatic.com/apps/valve/2009/GDC2009_ReplayableCooperativeGameDesign_Left4Dead.pdf

## Final decision

Build a small, inspectable, TUI-playable ordinary-life village first.

Use symbolic epistemic agents, event-sourced causality, typed claims, fallible institutions, structured speech acts, actor-filtered view models, no-human simulation, and deterministic replay.

Let story be discovered after the fact.

Do not let quests, LLMs, graphics, scale, or drama direction become the foundation.
