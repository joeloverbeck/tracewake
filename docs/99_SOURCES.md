# Sources and Research Notes

Research reviewed: 2026-06-05.

This bibliography is not exhaustive. It records the sources most relevant to the foundation pack and the design decisions they informed.

## Commercial / released-game precedents

### Dwarf Fortress

- Bay 12 Games, official development notes and roadmap. https://www.bay12games.com/dwarves/dev.html
- Kitfox Games press kit / Dwarf Fortress description. https://kitfoxgames.notion.site/Dwarf-Fortress-Press-Kit-3323c82092934114a47348d66f41d1ea
- Dwarf Fortress Wiki, world generation and history overview. https://dwarffortresswiki.org/index.php/World_generation

Design relevance: simulated world history, civilizations, sites, populations, artifacts, and event-rich historical substrate.

### RimWorld

- Official site. https://rimworldgame.com/
- Steam page. https://store.steampowered.com/app/294100/RimWorld/
- RimWorld Wiki, AI storytellers. https://rimworldwiki.com/wiki/AI_Storytellers

Design relevance: readable pawn simulation and colony events. Used here mostly as a contrast because the AI-storyteller model deliberately shapes drama.

### The Elder Scrolls V: Skyrim / Radiant quests

- Wired, “Skyrim Will Have Infinite Quests, Director Says.” https://www.wired.com/2011/11/skyrim-infinite-quests
- UESP, Skyrim Radiant quest overview. https://en.uesp.net/wiki/Skyrim:Radiant

Design relevance: repeatable modular tasks and player-progress-based procedural content. Useful as a contrast to institution-generated artifacts.

### Shadows of Doubt

- Official Steam page. https://store.steampowered.com/app/986130/Shadows_of_Doubt/
- ColePowered DevBlog #8, “Simulating a City.” https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- ColePowered DevBlog #15, “Moving in the Citizens.” https://colepowered.itch.io/shadows/devlog/78044/shadows-of-doubt-devblog-15-moving-in-the-citizens
- PC Gamer article on the toilet-queue simulation anecdote. https://www.pcgamer.com/games/sim/why-murders-stopped-in-shadows-of-doubt-because-everyone-was-queueing-for-the-only-toilet/

Design relevance: fully simulated procedural city, NPC routines, evidence-based investigation, and emergent bugs caused by actual simulation.

### Ultima-style NPC schedules

- The Digital Lycaeum, NPC schedules. https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/
- Ultima VII schedule data notes. https://ultima.fandom.com/wiki/Ultima_VII_Internal_Formats_-_SCHEDULE.DAT

Design relevance: towns feel alive when NPCs sleep, eat, work, and close shops. This project should upgrade schedules into defeasible intentions.

### The Sims and smart objects

- Tirrell, “Dumb People, Smart Objects: The Sims and the Distributed Self.” https://www.gamephilosophy.org/wp-content/uploads/confmanuscripts/pcg2012/Tirrell%202012%20-Dumb-People-Smart-Objects-The-Sims-and-the-Distributed-Self.pdf
- Game AI Pro 3, ambient interactions / smart locations. https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

Design relevance: object affordances, motive advertisements, and ordinary-life interaction design.

### Caves of Qud

- Grinblat and Bucklew, “Subverting Historical Cause & Effect: Generation of Mythic Biographies in Caves of Qud.” https://www.pcgworkshop.com/archive/grinblat2017subverting.pdf

Design relevance: generated histories, biographies, and mythic retelling. Useful contrast: this project should establish actual causal chains before mythologizing them.

### Left 4 Dead AI Director

- Valve / Mike Booth, “The AI Systems of Left 4 Dead.” https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Valve, “Replayable Cooperative Game Design: Left 4 Dead.” https://cdn.akamai.steamstatic.com/apps/valve/2009/GDC2009_ReplayableCooperativeGameDesign_Left4Dead.pdf

Design relevance: dramatic pacing director. Treated here as a counterexample: useful for pacing analysis, not as a causal world model.

## Agent cognition and planning

### BDI agents

- Rao and Georgeff, “BDI Agents: From Theory to Practice.” https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency.” https://link.springer.com/chapter/10.1007/3-540-49057-4_1

Design relevance: separating belief, desire, and intention is foundational for agents acting from subjective knowledge.

### GOAP

- Orkin, “Three States and a Plan: The A.I. of F.E.A.R.” https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- Game Developer / AI and Games overview, “Building the AI of F.E.A.R. with Goal Oriented Action Planning.” https://www.gamedeveloper.com/design/building-the-ai-of-f-e-a-r-with-goal-oriented-action-planning
- Higley, “Goal-Oriented Action Planning: Ten Years Old and No Fear!” https://media.gdcvault.com/gdc2015/presentations/Higley_Peter_Goal-Oriented_Action_Planning.pdf

Design relevance: flexible local planning from action preconditions and effects.

### HTN planning

- Kelly, Botea, Koenig, “Planning with Hierarchical Task Networks in Video Games.” https://icaps07-satellite.icaps-conference.org/workshop8/Planning%20with%20Hierarchical%20Task%20Networks%20in%20Video%20Games.pdf
- Menif et al., “SHPE: HTN Planning for Video Games.” https://www.lamsade.dauphine.fr/~cazenave/papers/MenifCGW2014.pdf

Design relevance: high-level decompositions for routines, investigations, institutional procedures, and socially legible behavior.

## Character knowledge, social simulation, and institutions

### Talk of the Town

- Ryan, Mateas, Wardrip-Fruin, “Toward Characters Who Observe, Tell, Misremember, and Lie.” https://eis.ucsc.edu/papers/ryanEtAl_TowardCharactersWhoObserveTellMisrememberLie.pdf
- Ryan, Mateas, “Simulating Character Knowledge Phenomena in Talk of the Town.” https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Ryan et al., “Dialogue Generation in Talk of the Town.” https://cdn.aaai.org/ojs/12877/12877-52-16394-1-2-20201228.pdf

Design relevance: subjective knowledge, rumor, confabulation, lies, memory, and dialogue grounded in what characters believe.

### Prom Week / Comme il Faut

- McCoy et al., “Prom Week: Social Physics as Gameplay.” https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Prom Week site, gameplay and social physics. https://promweek.soe.ucsc.edu/2011/11/12/gameplay-and-social-physics/

Design relevance: explicit social rules and history-sensitive social interactions as gameplay.

### Versu and social practices

- Evans and Short, “Versu — a simulationist storytelling system.” Public materials and papers are scattered; related social-practice framing appears in Emily Short and Richard Evans talks and publications.
- Useful entry point: https://versu.com/ (archival availability may vary)

Design relevance: autonomous characters, social practices, role perspectives, and replaying situations from different characters.

### Normative multi-agent systems

- Normative Multi-Agent Systems resource page. https://icr.uni.lu/normas/
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems.” https://cdn.aaai.org/KR/2004/KR04-028.pdf
- Mahmoud et al., “A Review of Norms and Normative Multiagent Systems.” https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/

Design relevance: obligations, permissions, prohibitions, sanctions, and institutional facts.

## Procedural quests and emergent narrative

### Procedural quest generation

- Doran and Parberry, “A Prototype Quest Generator Based on a Structural Analysis of Quests from Four MMORPGs.” https://ianparberry.com/techreports/LARC-2011-02.pdf
- Breault, Ouellet, Davies, “Let CONAN tell you a story: Procedural quest generation.” https://arxiv.org/abs/1808.06217
- Prins et al., “Procedural Quest Generation Rooted in Variety & Believability.” https://ris.utwente.nl/ws/files/307824333/3582437.3587181.pdf

Design relevance: motivations, structures, and planning approaches for tasks. Used here as subordinate tools, not as the foundation.

### Emergent narrative

- Ryan et al., “Open Design Challenges for Interactive Emergent Narrative.” https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf
- Johnson-Bey et al., “Neighborly: A Sandbox for Simulation-based Emergent Narrative.” https://www.kmjn.org/publications/Neighborly_CoG22.pdf
- Lyu et al., “Emergent Narratives with Composable Story Sifting Patterns.” https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf

Design relevance: stories as bottom-up results of autonomous agents; salience/story-sifting as an observer layer rather than a director.

## Event sourcing and observability

- Martin Fowler, “Event Sourcing.” https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “What do you mean by Event-Driven?” https://martinfowler.com/articles/201701-event-driven.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern.” https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing

Design relevance: append-only event logs, reconstructable state, auditability, and explanatory replay.

## LLM agents and LLM social simulation

### Generative agents

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior.” https://arxiv.org/abs/2304.03442

Design relevance: natural-language memories, reflection, planning, and believable social coordination.

### Social-agent evaluation and cautions

- Zhou et al., “SOTOPIA: Interactive Evaluation for Social Intelligence in Language Agents.” https://openreview.net/forum?id=mM7VurbA4r
- Goel and Zhu, “LIFELONG-SOTOPIA: Evaluating Social Intelligence of Language Agents Over Lifelong Social Interactions.” https://arxiv.org/abs/2506.12666
- Larooij and Törnberg, “Validation is the central challenge for generative social simulation: a critical review of LLMs in agent-based modeling.” https://link.springer.com/article/10.1007/s10462-025-11412-6
- Gao et al., “Large language models empowered agent-based modeling and simulation.” https://www.nature.com/articles/s41599-024-03611-3
- “LLM-Based Social Simulations Require a Boundary.” https://openreview.net/forum?id=1T1SE9xxAB

Design relevance: LLMs are promising for language surfaces and memory summaries, but validation, black-box behavior, stochasticity, cultural bias, long-horizon memory, and social-goal reliability remain serious concerns.

## Research conclusions carried into the design

1. Belief and information flow are more foundational than procedural quest generation.
2. Institutions must be modeled as fallible processes, not omniscient rule systems.
3. Event sourcing is unusually well suited because the game itself is about reconstructing causes.
4. Smart objects and routines are useful only when connected to ownership, norms, belief, and traces.
5. Story recognition/salience is acceptable; story direction violates the premise.
6. LLMs should render structured facts, not decide world truth.
7. The first prototype should be small, inspectable, and forensic.
