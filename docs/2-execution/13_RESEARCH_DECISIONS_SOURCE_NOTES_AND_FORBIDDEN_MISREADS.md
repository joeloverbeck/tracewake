# Research Decisions, Source Notes, and Forbidden Misreads

## Status

Live execution source-note document. Research informs judgment; Tracewake doctrine remains authority.

## Authority boundary

This document records execution-level research decisions and forbidden misreads. It does not import generic frameworks into Tracewake.

## Depends on

- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md`
- all live `docs/2-execution/*.md`

## Source-handling policy

Sources are used to sharpen gates, not to redirect product identity. A source is cited here only where it changes or reinforces an execution decision.

## Event sourcing, CQRS, and replay

Source anchors:

- Martin Fowler, “Event Sourcing” — https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “CQRS” — https://martinfowler.com/bliki/CQRS.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern” — https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Microsoft Azure Architecture Center, “CQRS pattern” — https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs

Decision: Tracewake treats events as authority and projections as rebuildable views. Command/action proposal separation matters because it prevents embodied commands, autonomous decisions, and institution procedures from becoming direct state mutation.

Forbidden misread: event sourcing is not “append a debug log beside authoritative mutable state.”

## Provenance

Source anchors:

- W3C PROV-DM — https://www.w3.org/TR/prov-dm/
- W3C PROV-O — https://www.w3.org/TR/prov-o/

Decision: Holder-known context must preserve source ancestry. Provenance is not optional metadata; it is the proof that cognition did not consume truth.

Forbidden misread: add provenance after behavior works.

## Determinism, replay, and randomized testing

Source anchors:

- Glenn Fiedler, “Deterministic Lockstep” — https://gafferongames.com/post/deterministic_lockstep/
- Glenn Fiedler, “Floating Point Determinism” — https://gafferongames.com/post/floating_point_determinism/
- Antithesis, “Deterministic simulation testing” — https://antithesis.com/docs/resources/deterministic_simulation_testing/
- Antithesis, “Property-based testing” — https://antithesis.com/docs/resources/property_based_testing/

Decision: Replayable simulation requires deterministic ordering, scoped random streams, recorded seeds/draws where needed, and failure reproduction artifacts.

Forbidden misread: a single global seed is enough.

## BDI, HTN, and bounded planning

Source anchors:

- Rao and Georgeff, “BDI Agents: From Theory to Practice” — https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency” — https://www.ppgia.pucpr.br/~fabricio/ftp/Aulas/Mestrado/AS/Artigos-Apresentacoes/BDI-Agents/georgeff.pdf
- Ghallab, Nau, and Traverso, *Automated Planning: Theory and Practice* — https://dl.acm.org/doi/book/10.5555/975615
- Georgievski and Aiello, “An Overview of Hierarchical Task Network Planning” — https://arxiv.org/abs/1403.7426
- Jeff Orkin, “Three States and a Plan” — https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf

Decision: Tracewake borrows the separation of beliefs, pressures, intentions, methods, and local plans. The point is inspectability and actor-known grounding, not a generic AI framework.

Forbidden misread: validation truth can rescue a planner that chose from omniscience.

## Smart objects and affordances

Source anchors:

- Kallmann and Thalmann, “Modeling Objects for Interaction Tasks” — https://infoscience.epfl.ch/bitstreams/7e065773-5b1f-403f-b788-f19dbd53f3e1/download
- Game AI Pro 3, “Ambient Interactions” — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

Decision: Objects, places, records, tools, beds, doors, and workplaces expose typed possibilities. Affordances advertise proposals; they do not mutate state or choose plans.

Forbidden misread: smart object equals direct action dispatch.

## Social knowledge and community simulation

Source anchors:

- Talk of the Town knowledge phenomena — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Prom Week / Comme il Faut social physics — https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Neighborly settlement simulation — https://ieee-cog.org/2022/assets/papers/paper_122.pdf
- Neighborly repository — https://github.com/ShiJbey/neighborly

Decision: Tracewake should model knowledge, social claims, relationships, households, and records as machinery. It should not turn social simulation into relationship scores plus global truth.

Forbidden misread: believable social output can replace source-backed claims.

## Normative multi-agent systems and institutions

Source anchors:

- Boella and van der Torre, “Introduction to Normative Multiagent Systems” — https://icr.uni.lu/leonvandertorre/papers/aisb05.pdf
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems” — https://cdn.aaai.org/KR/2004/KR04-028.pdf
- Mahmoud et al., “A Review of Norms and Normative Multiagent Systems” — https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/

Decision: Norms, roles, powers, procedures, detection, records, proof, and sanctions are distinct. Institutions are fallible holder-known processes.

Forbidden misread: norm violation automatically causes enforcement.

## LLM social agents

Source anchors:

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior” — https://arxiv.org/abs/2304.03442
- Larooij and Törnberg, “Validation is the central challenge for generative social simulation” — https://link.springer.com/article/10.1007/s10462-025-11412-6
- Mou et al., “From Individual to Society: A Survey on Social Simulation driven by LLM-empowered Agents” — https://arxiv.org/html/2412.03563v1

Decision: LLMs can be useful language and summarization surfaces. They are not authoritative simulation engines. Core gameplay must run with LLMs disabled.

Forbidden misread: believable prose is a valid source of simulation truth.

## Game precedents and counterexamples

Source anchors:

- Shadows of Doubt developer discussion of city simulation — https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- Dwarf Fortress — https://www.bay12games.com/dwarves/dev.html
- RimWorld AI storytellers — https://rimworldwiki.com/wiki/AI_Storytellers
- Left 4 Dead AI Director paper — https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf

Decision: Tracewake learns from deep ordinary simulation and detective affordances. It rejects hidden drama directors, player-centered quest ontology, and pacing mutation.

Forbidden misread: emergent narrative needs a director that can mutate world state.
