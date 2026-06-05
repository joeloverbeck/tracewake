# Research Survey and Design Conclusions

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
