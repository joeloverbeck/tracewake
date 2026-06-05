# Research Decisions and Source Notes

## Status

This document records architecture-specific research decisions. It is not a literature review, a bibliography dump, or a substitute for the subsystem contracts.

Dates and links are included for traceability. Source availability may change; the architectural conclusions are the important part.

## Event sourcing and projections

Sources:

- Martin Fowler, "Event Sourcing" — https://martinfowler.com/eaaDev/EventSourcing.html
- Microsoft Azure Architecture Center, "Event Sourcing pattern" — https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Microsoft Azure Architecture Center, "CQRS pattern" — https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs

Decision:

Tracewake uses event sourcing because the simulation must explain how state came to be, not merely what state is now. Event history supports replay, audit, causal debugging, projection rebuild, actor/NPC parity checks, belief provenance, institutional records, stale notices, no-human simulation review, and long-lived saves.

CQRS-like separation is useful only as a projection discipline: commands/proposals go through the kernel; read models/view models are derived. Do not overbuild distributed microservice architecture for v1.

## Event schema evolution

Sources:

- Microsoft Azure Architecture Center, "Event Sourcing pattern" versioning notes — https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Axon Framework docs, "Event Versioning" — https://docs.axoniq.io/axon-framework-reference/5.1/events/event-versioning/
- Marten docs, "Events Versioning" — https://martendb.io/events/versioning.html

Decision:

Events are immutable. Event envelopes carry event type and schema version. Old events remain readable through version-specific handlers or upcasters. Replay fails loudly without a migration path. This is essential for long-lived saves and research reproducibility.

## Deterministic simulation and replay

Sources:

- Glenn Fiedler, "Deterministic Lockstep" — https://gafferongames.com/post/deterministic_lockstep/
- Glenn Fiedler, "Floating Point Determinism" — https://gafferongames.com/post/floating_point_determinism/
- Random ASCII, "Floating-Point Determinism" — https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/

Decision:

Tracewake is not primarily a multiplayer lockstep game, but the same discipline applies: deterministic outcomes require stable inputs, stable ordering, seed discipline, data versions, and isolation from wall-clock and runtime accident. Random streams are scoped, and meaningful draws are recorded or committed for audit/debug. Tests use deterministic mocks for nondeterministic surfaces.

## BDI agents

Sources:

- Anand Rao and Michael Georgeff, "BDI Agents: From Theory to Practice" — https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- Michael Georgeff et al., "The Belief-Desire-Intention Model of Agency" — https://link.springer.com/chapter/10.1007/3-540-49057-4_1

Decision:

Tracewake uses a BDI-style separation because beliefs, motives, and intentions must be inspectable and distinct. An agent should not merely maximize an opaque score every tick. Durable intentions prevent jitter; beliefs enforce knowledge boundaries; motives and roles explain why actions were proposed.

## HTN planning and bounded local planning

Sources:

- Malik Ghallab, Dana Nau, Paolo Traverso, *Automated Planning: Theory and Practice* — https://dl.acm.org/doi/book/10.5555/975615
- Jeff Orkin, "Three States and a Plan: The A.I. of F.E.A.R." — https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf

Decision:

Tracewake uses HTN-style methods for routines and procedures because ordinary life is full of reusable, explainable task decompositions: eat, sleep, work, report, search, receive report, post notice. It uses bounded GOAP-style local planning for concrete action sequences such as reaching a room, opening a door, searching a container, or traveling to an office.

GOAP is not the whole mind. Utility scoring is not the whole mind. HTN methods are not plot scripts.

## Smart objects and affordances

Sources:

- Marcelo Kallmann, "Modeling Objects for Interaction Tasks" — https://graphics.ucmerced.edu/publications/1998_EGCAS_Kallmann.pdf
- Game AI Pro 3, "Ambient Interactions: Improving Believability by Leveraging Rule-Based AI" — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter35_Ambient_Interactions_Improving_Believability_by_Leveraging_Rule-Based_AI.pdf
- Historical design discussions around The Sims smart-object approach.

Decision:

Tracewake objects expose typed affordances: doors, beds, containers, records, notice boards, tools, workplaces, roads, and people advertise possible actions. Affordances do not decide success. They feed the shared action pipeline with actor authority, knowledge, social norms, reservations, scheduling, traces, and events.

## Social simulation precedents

Sources:

- James Ryan et al., Talk of the Town work on simulated character knowledge/dialogue — https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- Prom Week / Comme il Faut social physics — https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- Neighborly settlement simulation — https://ieee-cog.org/2022/assets/papers/paper_122.pdf and https://github.com/ShiJbey/neighborly

Decision:

Tracewake borrows the lesson that relationships, knowledge, history, and social facts can be simulated as first-class machinery. It does not copy any one system. Tracewake's distinguishing requirement is stricter forensic causality, event sourcing, TUI-first play, no sacred player, fallible institutions, and actor-knowledge filtering.

## Normative multi-agent systems

Sources:

- Guido Boella and Leendert van der Torre, "Introduction to Normative Multiagent Systems" — https://icr.uni.lu/leonvandertorre/papers/aisb05.pdf
- Mahmoud et al., "A Review of Norms and Normative Multiagent Systems" — https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/

Decision:

Institutions and norms must be explicit. Norms classify permissions, obligations, prohibitions, constitutive facts, evidence thresholds, and procedures. Agents can violate norms. Institutions can fail. Violation, detection, suspicion, record, proof, and sanction remain separate.

## Dynamic level of detail

Sources:

- Navarro et al., "Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations" — https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf
- Multi-level agent-based simulation design literature, for example https://hal.science/hal-01691388/document

Decision:

Tracewake cannot simulate a large region at room-level detail forever. It uses explicit event-sourced LOD: detailed local simulation, active settlement simulation, regional process simulation, and historical/external abstraction. LOD promotion/demotion must be inspectable and preserve causal ancestry. Low-LOD people remain promotable people, not props.

## Emergent narrative and story sifting

Sources:

- Talk of the Town / Bad News lineage for generated social history and player exploration.
- Prom Week for social physics producing emergent story-like outcomes from social simulation.
- Neighborly for settlement-scale emergent narrative research.

Decision:

Tracewake permits observer-only story sifting: grouping event chains, recapping no-human simulation, highlighting contradictions, and surfacing wrong beliefs. Story sifting cannot spawn events, select culprits, repair pacing, or reveal hidden truth in embodied mode.

## LLM agents and validation risks

Sources:

- Park et al., "Generative Agents: Interactive Simulacra of Human Behavior" — https://arxiv.org/abs/2304.03442
- Larooij and related work on validation as a central challenge for generative social simulations — https://link.springer.com/article/10.1007/s10462-025-11412-6
- Position/review work on boundaries and validation for LLM-based social simulation, including https://openreview.net/forum?id=1T1SE9xxAB and https://arxiv.org/html/2507.19364v1

Decision:

Generative-agent work shows that LLMs can produce believable social surfaces, memory summaries, and language-rich behavior. Tracewake rejects LLMs as authoritative simulation brains because the core requirements are replayability, validation, actor-knowledge filtering, causal inspection, and deterministic tests. LLMs may render or parse language behind validation and deterministic mocks.

## Simulation/game precedents and counterexamples

### Dwarf Fortress

Sources:

- Dwarf Fortress Steam page — https://store.steampowered.com/app/975370/Dwarf_Fortress/
- Bay 12 Games development notes — https://www.bay12games.com/dwarves/dev.html
- Dwarf Fortress wiki world generation notes — https://dwarffortresswiki.org/index.php/World_generation

Decision:

Dwarf Fortress validates the ambition of deep procedural history and simulation. Tracewake should learn from the importance of history and consequences, but stage world generation after a hand-authored causal village proves the ontology.

### Shadows of Doubt

Sources:

- ColePowered, Shadows of Doubt — https://colepowered.com/games/shadows-of-doubt/
- ColePowered devblog on simulating a city — https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- Steam page — https://store.steampowered.com/app/986130/Shadows_of_Doubt/

Decision:

Shadows of Doubt shows the power of citizens with homes, jobs, routines, and inspectable procedural evidence. Tracewake preserves the lesson of citizens independent of the player, but its first target is TUI-first ordinary life and epistemic causality rather than detective-noir graphical city scale.

### The Sims smart-object lineage

Decision:

The Sims lineage supports object affordances and needs-driven ordinary behavior. Tracewake uses that lesson while strengthening provenance, actor belief, institutions, event sourcing, no-human simulation, and forensic replay.

### RimWorld storytellers and Left 4 Dead AI Director as counterexamples

Sources:

- Valve, "The AI Systems of Left 4 Dead" — https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- RimWorld wiki, AI Storytellers — https://rimworldwiki.com/wiki/AI_Storytellers

Decision:

Pacing directors are successful for their games, but Tracewake rejects hidden director logic. No system may spawn or adjust events to maximize drama, excitement, raid pacing, or player engagement. Story is observed after causality, not directed before it.

### Skyrim Radiant quests as counterexample

Sources:

- Wired, 2011 report on Skyrim's radiant/infinite quests — https://www.wired.com/2011/11/skyrim-infinite-quests/
- Elder Scrolls community documentation on radiant quests — https://elderscrolls.fandom.com/wiki/Radiant_Quests

Decision:

Radiant quests demonstrate scalable task variation but preserve quest ontology: targets, rewards, and player-directed tasks. Tracewake rejects quest ontology. Notices, requests, contracts, and leads are fallible world artifacts and projections.

## Rust-first architecture conclusion

Research did not identify a reason to adopt a game engine architecture as the root. Tracewake's root should be a Rust-first simulation architecture with explicit boundaries, deterministic scheduler, event sourcing, data validation, TUI client, replay/debug tooling, and optional future language/graphical surfaces.

Candidate crates are implementation details. The architectural invariants are not.
