# Design Risk Register

This is a living watchlist, not a constitution. The design decisions that were once tracked here are now settled and codified — possession and player ontology in `INV-005`–`INV-010` and `13_PLAYER_MODEL_POSSESSION_AND_UI_VIEW_MODELS.md`, the LLM boundary in `INV-029`/`INV-050`–`INV-052`, questlessness in `INV-035`–`INV-039`, scope and phase order in `INV-040`–`INV-043` and the Charter. Consult those for the rules. This file tracks the failure modes that threaten them and the symptoms by which you'd catch each one drifting in.

## Accepted tradeoffs

A few decisions intentionally tolerate something the invariants might otherwise seem to forbid. They are recorded here so they are not mistaken for bugs:

- **Indirect exploration metagaming is tolerated.** `INV-008` forbids the actor *acting* on player-only knowledge, but it does not try to stop the human from exploring more efficiently because they remember prior runs. Preventing indirect metagaming completely would be unfun and brittle; only world-affecting actions are gated.
- **An optional forensic mode may persist into normal builds.** `INV-004` requires debug omniscience to be visibly separate from embodied play. Building omniscient tools early is encouraged, and an optional forensic view may remain available alongside normal embodied mode rather than being stripped from shipping builds.

## Risks

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
