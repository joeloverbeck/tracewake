# Research Decisions and Forbidden Misreads

## Status

Authoritative architecture source-note contract. This document is not a bibliography dump. Research is recorded only as decisions, consequences, and forbidden misreads.

## Purpose / core rule

Tracewake borrows lessons from event-sourced systems, agent architectures, planning, social simulation, institutional modeling, dynamic LOD, emergent narrative, LLM social simulations, and game precedents. It does not copy any precedent wholesale. The architecture remains governed by foundation doctrine.

## Decisions

### Event sourcing, CQRS, replay, and event versioning

Source anchors:

- Martin Fowler, “Event Sourcing” — https://martinfowler.com/eaaDev/EventSourcing.html
- Martin Fowler, “CQRS” — https://martinfowler.com/bliki/CQRS.html
- Microsoft Azure Architecture Center, “Event Sourcing pattern” — https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Microsoft Azure Architecture Center, “CQRS pattern” — https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs
- Axon Framework, “Event Versioning” — https://docs.axoniq.io/axon-framework-reference/5.1/events/event-versioning/
- Marten, “Events Versioning” — https://martendb.io/events/versioning.html

Decision: Tracewake uses event sourcing because causality, replay, and forensic inspection are product identity, not infrastructure preference. CQRS is useful as a command/projection discipline, not as microservice ceremony.

Consequence: current state is projection; event envelopes need schema versions; projection rebuild is an acceptance gate; upcasters/migrations are explicit and loud; save packages carry event/projection/random manifests.

Forbidden misread: “Event sourcing means append logs for debugging while state remains the real authority.”

### Provenance

Source anchors:

- W3C PROV-DM — https://www.w3.org/TR/prov-dm/
- W3C PROV-O — https://www.w3.org/TR/prov-o/

Decision: Tracewake uses provenance-like structure for holder-known context: claims, observations, records, memories, and diagnostics must preserve source and derivation.

Consequence: every cognition/procedure input needs source ancestry; unknowns are explicit; debug truth is not a source for embodied holders.

Forbidden misread: “Provenance is a nice audit field we can add after behavior works.”

### Determinism and scoped randomness

Source anchors:

- Glenn Fiedler, “Deterministic Lockstep” — https://gafferongames.com/post/deterministic_lockstep/
- Glenn Fiedler, “Floating Point Determinism” — https://gafferongames.com/post/floating_point_determinism/
- Random ASCII, “Floating-Point Determinism” — https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/

Decision: Tracewake applies deterministic simulation discipline even though it is not primarily a multiplayer lockstep game.

Consequence: stable ordering, scoped random streams, recorded draws where needed, no wall-clock authority, deterministic mocks for nondeterministic surfaces.

Forbidden misread: “A single global seed is deterministic enough.”

### BDI-style agents and durable intentions

Source anchors:

- Rao and Georgeff, “BDI Agents: From Theory to Practice” — https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Georgeff et al., “The Belief-Desire-Intention Model of Agency” — https://link.springer.com/chapter/10.1007/3-540-49057-4_1

Decision: Tracewake uses BDI-style separation because beliefs, pressures, commitments, intentions, and actions must be inspectable and distinct.

Consequence: needs are pressures, intentions are durable, beliefs may be wrong, and decision traces must show why an actor continued, switched, waited, failed, or replanned.

Forbidden misread: “BDI means opaque utility scoring with belief labels attached.”

### HTN methods and bounded local planning

Source anchors:

- Ghallab, Nau, and Traverso, *Automated Planning: Theory and Practice* — https://dl.acm.org/doi/book/10.5555/975615
- Jeff Orkin, “Three States and a Plan: The A.I. of F.E.A.R.” — https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf
- GDC Vault, “Three States and a Plan” — https://gdcvault.com/play/1013394/Three-States-and-a-Plan

Decision: Tracewake uses HTN-style methods for explainable routines/procedures and bounded local planning for concrete action sequences.

Consequence: routines are decompositions, not scripts; GOAP-like planning is local and actor-known; methods can be rejected; fallback is typed and bounded.

Forbidden misread: “Planning can use the true world because validation will catch impossible actions.”

### Smart objects and affordances

Source anchors:

- Kallmann and Thalmann, “Modeling Objects for Interaction Tasks” — https://graphics.ucmerced.edu/publications/1998_EGCAS_Kallmann.pdf
- Game AI Pro 3, “Ambient Interactions” — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf

Decision: Objects expose typed affordances that feed proposals. Affordances advertise possibility; validators decide success.

Consequence: doors, beds, containers, records, notice boards, tools, workplaces, roads, and people can contribute action options without becoming scripts.

Forbidden misread: “An object affordance can directly mutate state or choose the actor's plan.”

### Social knowledge simulation

Source anchors:

- Talk of the Town knowledge phenomena — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Prom Week / Comme il Faut social physics — https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Neighborly — https://ieee-cog.org/2022/assets/papers/paper_122.pdf and https://github.com/ShiJbey/neighborly

Decision: Social facts, relationships, histories, beliefs, rumors, and records are first-class machinery.

Consequence: speech, records, gossip, memory, and institutions are modeled channels with possible error, not omniscient fact propagation.

Forbidden misread: “Social simulation can be approximated by relationship scores plus global truth.”

### Normative multi-agent systems and institutions

Source anchors:

- Boella and van der Torre, “Introduction to Normative Multiagent Systems” — https://icr.uni.lu/leonvandertorre/papers/aisb05.pdf
- Boella and van der Torre, “Regulative and Constitutive Norms in Normative Multiagent Systems” — https://cdn.aaai.org/KR/2004/KR04-028.pdf
- Mahmoud et al., “A Review of Norms and Normative Multiagent Systems” — https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/

Decision: Institutions, roles, rules, powers, procedures, and sanctions must be explicit and fallible.

Consequence: violation, detection, suspicion, record, proof, sanction, and appeal/failure remain distinct. Institution-known context mirrors actor-known context.

Forbidden misread: “A norm violation can directly trigger enforcement.”

### Dynamic LOD and scale

Source anchors:

- Navarro, Flacher, and Corruble, “Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations” — https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf
- Multi-level agent-based simulation literature — https://hal.science/hal-01691388/document

Decision: LOD is replay-visible ontology. Detail changes are events with ancestry.

Consequence: aggregate processes are allowed, but promoted actors/institutions receive only modeled knowledge, records, memories, and summary ancestry.

Forbidden misread: “Low-LOD people are counters until promoted, then fill from truth.”

### Emergent narrative and story sifting

Source anchors:

- Talk of the Town / Bad News lineage
- Prom Week social physics
- Neighborly settlement simulation

Decision: Story sifting is observer/projection only.

Consequence: summaries can group, highlight, and explain event chains after the fact. They cannot cause events, select culprits, repair pacing, or expose hidden truth in embodied mode.

Forbidden misread: “Emergent narrative needs a hidden director to keep things interesting.”

### LLM social simulation

Source anchors:

- Park et al., “Generative Agents: Interactive Simulacra of Human Behavior” — https://arxiv.org/abs/2304.03442
- Generative Agents project repository — https://github.com/joonspk-research/generative_agents
- Validation/boundary work on LLM social simulation — https://link.springer.com/article/10.1007/s10462-025-11412-6 and https://openreview.net/forum?id=1T1SE9xxAB

Decision: LLMs are useful future language surfaces, not authoritative agents.

Consequence: core gameplay runs with LLMs disabled; LLMs may render, parse, or summarize behind validation and deterministic mocks; prompt packets are holder-filtered.

Forbidden misread: “Believable LLM behavior is enough to be simulation truth.”

### Game precedents and counterexamples

Source anchors:

- Dwarf Fortress — https://store.steampowered.com/app/975370/Dwarf_Fortress/ and https://www.bay12games.com/dwarves/dev.html
- Shadows of Doubt — https://colepowered.com/games/shadows-of-doubt/ and https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- Ultima-style schedules and The Sims smart-object lineage
- RimWorld AI storytellers — https://rimworldwiki.com/wiki/AI_Storytellers
- Left 4 Dead AI Director — https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- Skyrim/Oblivion radiant quests — https://www.wired.com/2011/11/skyrim-infinite-quests/

Decision: Tracewake learns from deep procedural history, ordinary schedules, citizen routines, social simulation, and smart affordances. It rejects player-directed quest ontology, pacing directors, and hidden drama management.

Consequence: authored causal machinery is allowed; authored outcome chains are forbidden. Later adventure content must be ordinary causal expansion.

Forbidden misread: “Successful games use directors/quests, so Tracewake should add a carefully hidden one.”

## Final architecture conclusion

Research reinforces the same contract: Rust-first event-sourced simulation, holder-known cognition/procedure, shared validation, fallible institutions, TUI-first playability, optional language surfaces, and replayable diagnostics. Any precedent that conflicts with those invariants is a counterexample, not a template.
