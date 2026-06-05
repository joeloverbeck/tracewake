# Open Design Decisions and Risks

## Decisions

### Possession

Not diegetic first. Treat possession as a non-diegetic controller interface. Later domain packs may make possession a world fact.

### Player knowledge

No leakage for speech, accusation, testimony, proof, or institutional action. Indirect exploration metagaming is acceptable because preventing it completely would be unfun and brittle.

### UI assistance

Allowed. The TUI may be more organized than an NPC mind, as long as final actions are valid for the actor.

### Debug tools

Build omniscient tools early. Normal play is embodied mode; optional forensic mode may remain.

### LLM dialogue

Eventually yes, through validated speech acts only. Live LLMs should not be used until deterministic mocks and validators work.

### First population size

30–60 agents. Enough for density, small enough to inspect.

### Background citizens

Meaningful but not always full fidelity. Lower LOD is dormant detail, not garbage.

### First flavor

Use restrained low-fantasy or neutral medieval-ish content. Keep core genre-agnostic.

### Violence

Not first. First violence needs injury/death/traces/fear/proof/institutional consequences. Tactical richness comes later.

### Exogenous incidents

Allowed if causally modeled: weather, migration, disease, caravans, outside politics, animal movement, trade shocks. Never as boredom response.

### World generation

Not first. Hand-author first, add procedural generation after ontology works.

### Unfair outcomes

Allowed and desirable if legible: false accusations, unpaid rewards, lost evidence, dead witnesses, corrupt officials, stale records.

### Public prototype

It may be visually simple, but must be playable. Position it as an embodied emergent village/region forensic simulator, not a full RPG.

## Major risks

### Protagonist gravity

Symptoms: events cluster around player, tasks target player, NPCs wait, UI reveals truth.

Mitigation: no-player simulations, player-conditioned event metrics, code review for player checks.

### Procedural quest relapse

Symptoms: `Quest` object appears, rewards spawn, targets exist for tasks, notices are menu entries.

Mitigation: ontology review, lead/contract/notice model, invariant tests.

### LLM contamination

Symptoms: dialogue invents facts, NPCs know prompt-only truth, prose changes state.

Mitigation: speech-act validation, structured propositions, no LLM in authoritative core, deterministic mock tests.

### Simulation without game

Symptoms: impressive logs but no desire to play, no readable artifacts, no intervention handles.

Mitigation: embodied TUI, leads, notices, conversation, recruitment, why-not UI.

### Scale before clarity

Symptoms: huge region with uninspectable causal mush.

Mitigation: first village, replay from day one, regional LOD after detailed chain works.

### Agents too dumb or too expensive

Symptoms: lifeless routines, planner explosion, oscillation, unbelievable choices.

Mitigation: projects, intention persistence, HTN for high-level behavior, bounded local planning, LOD, why-did-actor-choose-this debugging.

### Genre lock-in

Symptoms: fantasy assumptions in kernel, magic bypasses belief, monsters as quest targets.

Mitigation: domain packs, core ontology review, magic as modeled information/action channel.

## Confidence

The design target is now clear enough to generate implementation-grade foundation docs. It is not ready for code if the first milestone expands beyond the TUI village.
