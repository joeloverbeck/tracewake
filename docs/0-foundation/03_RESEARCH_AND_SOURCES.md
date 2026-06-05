# Research Survey, Design Conclusions, and Sources

Research reviewed: 2026-06-05.

## Executive synthesis

Tracewake should combine:

```text
Event-sourced world kernel
+ subjective BDI-style agents
+ authored HTN methods
+ bounded GOAP local planning
+ explicit speech acts and information channels
+ normative institutions
+ ordinary-life economy
+ spatial/regional LOD
+ questless public artifacts
+ embodied TUI
+ optional grounded LLM dialogue
+ later world/history generation
```

The strongest conclusion: **do not build Tracewake around procedural quests**. Build it around epistemic causality.

## Game precedents

### Dwarf Fortress

Borrow history-before-play, persistent world state, simulated populations, artifacts, and acceptance of surprising causal chains. Avoid opacity and excessive scale before forensic clarity. Tracewake should eventually support “a region that has lived,” but with sharper tracking of who knows what and why.

### RimWorld

Borrow readable pawn state, needs, notifications, and legible disruptions. Reject the AI storyteller as a metaphysical cause. Tracewake may use readability without adopting threat pacing.

### Skyrim and radiant quests

Borrow the fantasy of entering towns, reading notices, and taking work. Reject player-targeted task generation, quest-giver content faucets, and objective markers that imply truth.

### Shadows of Doubt

Borrow physical inspectability, homes, jobs, routines, independent citizens, and evidence-based investigation. Improve with stronger belief provenance, rumor mechanics, institutions, and broader social simulation.

### Ultima-style schedules

Borrow daily routine believability. Upgrade schedules into defeasible intentions.

### The Sims and smart objects

Borrow object-local affordances and domestic interaction. Upgrade affordances with ownership, law, privacy, belief, and trace generation.

### Caves of Qud

Borrow generated history and mythic retelling later. First build real causal chains.

### Left 4 Dead AI Director

Treat as a counterexample. Pacing analysis may be diagnostic; dramatic direction must not cause world events.

## Architecture precedents

### BDI

BDI is the correct high-level mental model because Tracewake requires separation between what agents believe, what they want, and what they have committed to doing.

### HTN

HTN is appropriate for legible high-level procedures: workday, investigate theft, post notice, escort caravan, heal patient, hide evidence, recruit companion. HTN methods must remain grounded in real preconditions and failure modes.

### GOAP

GOAP-like planning is useful for local action sequences: reach place, open door, get tool, inspect container, speak to target. It should be bounded and nested under HTN rather than driving every life choice.

### Talk of the Town

One of the most important references. Borrow observation, knowledge propagation, forgetting, misremembering, lying, and dialogue grounded in subjective knowledge.

### Prom Week / Comme il Faut

Borrow explicit social rules, relationships, history-sensitive interaction, and social state as gameplay.

### Normative multi-agent systems

Borrow obligations, permissions, prohibitions, constitutive norms, status rules, sanctions, and the separation between violation and detection.

### Emergent narrative and story sifting

Borrow story recognition and salience as observer layers. Reject story direction as cause.

### Event sourcing

Event sourcing is the backbone of Tracewake. It supports replay, causal graphing, belief generation, institutional records, historical chronicles, debugging, and player-facing investigation.

### LLM agents

LLMs are useful for dialogue surfaces, memory summaries, notice text, rumor phrasing, and paraphrase. They are dangerous as authoritative planners or truth generators. Use them only behind validation.

### Dynamic LOD

Regional scale requires explicit level of detail. Low-fidelity processes must still produce events and causal ancestry, and may be promoted when relevant.

### Procedural generation and WFC

Procedural generation is promising later for rooms, settlements, roads, ruins, and ecological constraints. It must instantiate causal materials, not bypass them.

## Updated first target

The first playable target is a TUI village that can be entered, talked to, manipulated, and investigated, with a stale notice leading to an expedition whose outcome is not guaranteed by quest logic.

---

## Sources and citations

Full reference list for the precedents and architectures discussed above. (Merged from the former `99_SOURCES.md`.)

### Commercial / released-game precedents

#### Dwarf Fortress

- Bay 12 Games development notes: https://www.bay12games.com/dwarves/dev.html
- Kitfox press kit: https://kitfoxgames.notion.site/Dwarf-Fortress-Press-Kit-3323c82092934114a47348d66f41d1ea
- Dwarf Fortress Wiki, world generation: https://dwarffortresswiki.org/index.php/World_generation

Relevance: world history, civilizations, populations, artifacts, persistent state, world-before-player fantasy.

#### RimWorld

- Official site: https://rimworldgame.com/
- Steam page: https://store.steampowered.com/app/294100/RimWorld/
- AI Storytellers wiki: https://rimworldwiki.com/wiki/AI_Storytellers

Relevance: readable pawn simulation. Counterexample: AI storyteller as drama cause.

#### Skyrim / Radiant quests

- Wired: https://www.wired.com/2011/11/skyrim-infinite-quests
- UESP Radiant quests: https://en.uesp.net/wiki/Skyrim:Radiant

Relevance: public work fantasy and repeatable task structures. Rejected as foundation.

#### Shadows of Doubt

- Steam page: https://store.steampowered.com/app/986130/Shadows_of_Doubt/
- DevBlog #8: https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- DevBlog #15: https://colepowered.itch.io/shadows/devlog/78044/shadows-of-doubt-devblog-15-moving-in-the-citizens
- PC Gamer anecdote: https://www.pcgamer.com/games/sim/why-murders-stopped-in-shadows-of-doubt-because-everyone-was-queueing-for-the-only-toilet/

Relevance: independent citizens, routines, homes/jobs, evidence-driven investigation.

#### Ultima-style schedules

- Digital Lycaeum: https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/
- Ultima VII schedule data: https://ultima.fandom.com/wiki/Ultima_VII_Internal_Formats_-_SCHEDULE.DAT

Relevance: towns feel alive when NPCs sleep, eat, work, and close shops. Upgrade to defeasible intentions.

#### The Sims and smart objects

- Tirrell, “Dumb People, Smart Objects”: https://www.gamephilosophy.org/wp-content/uploads/confmanuscripts/pcg2012/Tirrell%202012%20-Dumb-People-Smart-Objects-The-Sims-and-the-Distributed-Self.pdf
- Game AI Pro 3 smart locations: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

Relevance: object affordances and domestic behavior.

#### Caves of Qud

- Mythic biographies paper: https://www.pcgworkshop.com/archive/grinblat2017subverting.pdf

Relevance: generated histories and mythic retelling after causal chains exist.

#### Left 4 Dead AI Director

- AI Systems of Left 4 Dead: https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Replayable Cooperative Game Design: https://cdn.akamai.steamstatic.com/apps/valve/2009/GDC2009_ReplayableCooperativeGameDesign_Left4Dead.pdf

Relevance: counterexample. Pacing director is not allowed as world cause.

### Agent cognition and planning

#### BDI

- Rao and Georgeff: https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al.: https://link.springer.com/chapter/10.1007/3-540-49057-4_1

Relevance: belief/desire/intention separation.

#### GOAP

- Orkin FEAR paper: https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- Game Developer overview: https://www.gamedeveloper.com/design/building-the-ai-of-f-e-a-r-with-goal-oriented-action-planning
- Higley GDC: https://media.gdcvault.com/gdc2015/presentations/Higley_Peter_Goal-Oriented_Action_Planning.pdf

Relevance: bounded local planning.

#### HTN

- HTN in games: https://icaps07-satellite.icaps-conference.org/workshop8/Planning%20with%20Hierarchical%20Task%20Networks%20in%20Video%20Games.pdf
- SHPE HTN: https://www.lamsade.dauphine.fr/~cazenave/papers/MenifCGW2014.pdf

Relevance: high-level decomposition for routines, investigations, institutions.

### Knowledge, social simulation, institutions

#### Talk of the Town

- Observe, Tell, Misremember, Lie: https://eis.ucsc.edu/papers/ryanEtAl_TowardCharactersWhoObserveTellMisrememberLie.pdf
- Game AI Pro chapter: https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Dialogue generation: https://cdn.aaai.org/ojs/12877/12877-52-16394-1-2-20201228.pdf

Relevance: subjective knowledge, rumor, memory, lies, dialogue grounded in beliefs.

#### Prom Week / Comme il Faut

- Prom Week paper: https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Prom Week site: https://promweek.soe.ucsc.edu/2011/11/12/gameplay-and-social-physics/

Relevance: social state, relationships, rules, history-sensitive interaction.

#### Normative multi-agent systems

- Normative MAS resource: https://icr.uni.lu/normas/
- Boella and van der Torre: https://cdn.aaai.org/KR/2004/KR04-028.pdf
- Review of norms: https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/

Relevance: obligations, permissions, prohibitions, sanctions, institutional facts.

### Procedural quests, emergent narrative, generation

#### Procedural quest generation

- Doran and Parberry: https://ianparberry.com/techreports/LARC-2011-02.pdf
- CONAN: https://arxiv.org/abs/1808.06217
- Variety & Believability: https://ris.utwente.nl/ws/files/307824333/3582437.3587181.pdf

Relevance: useful for formatting requests/contracts, rejected as foundation.

#### Emergent narrative and story sifting

- Open Design Challenges: https://expressiveintelligence.github.io/papers/ryanEtAl_OpenDesignChallengesForInteractiveEmergentNarrative.pdf
- Neighborly: https://www.kmjn.org/publications/Neighborly_CoG22.pdf
- Composable Story Sifting Patterns: https://eprints.soton.ac.uk/502632/2/3723498.3723809.pdf

Relevance: story recognition/salience as observer layer, not director.

#### Wave Function Collapse

- BorisTheBrave: https://www.boristhebrave.com/2020/04/13/wave-function-collapse-explained/

Relevance: later constraint-based spatial generation.

### Event sourcing

- Martin Fowler, Event Sourcing: https://martinfowler.com/eaaDev/EventSourcing.html
- Fowler, Event-Driven: https://martinfowler.com/articles/201701-event-driven.html
- Microsoft Event Sourcing pattern: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing

Relevance: append-only event logs, replay, auditability, temporal queries.

### LLM agents and social simulation

- Generative Agents: https://arxiv.org/abs/2304.03442
- SOTOPIA: https://openreview.net/forum?id=mM7VurbA4r
- LIFELONG-SOTOPIA: https://arxiv.org/abs/2506.12666
- Validation review: https://link.springer.com/article/10.1007/s10462-025-11412-6
- LLM agent-based simulation review: https://www.nature.com/articles/s41599-024-03611-3
- LLM-based social simulations require a boundary: https://openreview.net/forum?id=1T1SE9xxAB

Relevance: LLMs are powerful for surfaces and summaries but dangerous as unvalidated simulation brains.

### Dynamic LOD

- Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations: https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf

Relevance: scalable regional simulation through adaptable fidelity.

### Research conclusions

Belief and information flow are more foundational than quests. Institutions must be fallible. Event sourcing fits the fantasy. Smart objects and routines must connect to ownership, norms, belief, and traces. Story recognition is allowed; story direction is not. LLMs render/propose validated speech acts, not truth. Regional scale requires LOD. World generation comes after hand-authored causal systems. The first prototype should be small, inspectable, and playable as a TUI.
