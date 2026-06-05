# Open Design Questions and Provisional Answers

This document lists unresolved design choices. Each question has a recommended default so implementation can proceed without paralysis.

## 1. Is possession diegetic?

### Question

Does the world know that the player can switch bodies?

### Recommendation

Not in the first prototype. Treat possession as a development/player interface, not a supernatural fact.

### Reason

Diegetic possession introduces major setting implications: identity, memory leakage, law, religion, consent, and metaphysics. The first prototype should prove no-sacred-player simulation first.

## 2. Can player knowledge leak into actor action?

### Recommendation

No for speech, accusation, evidence claims, and institutional action. Yes only indirectly through movement/exploration choices, because preventing all metagaming is impossible and likely not fun.

### Rule

Actor can only assert or formally act on modeled actor knowledge.

## 3. Should the player have an omniscient debug mode?

### Recommendation

Yes during development. Later, provide two modes:

- **simulation/debug mode** for development and players who enjoy forensic omniscience;
- **embodied mode** where ground truth is hidden.

The debug tools are too central to delay.

## 4. Should ordinary citizens be fully simulated offscreen?

### Recommendation

No. Use level-of-detail simulation.

### Rule

Full detail where causal chains, player proximity, investigations, or high-salience events require it. Summarize low-salience routine life.

## 5. How much should agents remember?

### Recommendation

Remember salient events, compress routine events, and store semantic summaries.

### Rule

A belief or memory deserves storage if it can affect later action, testimony, relationship, institution, or player inference.

## 6. Should every event leave a trace?

### Recommendation

Every meaningful event should leave some trace, but not necessarily a physical trace.

Examples:

- a private thought may leave a mental/intention trace;
- a legal decision leaves a record;
- a theft leaves absence and maybe physical traces;
- a cleaned crime leaves erasure traces.

## 7. How realistic should the economy be?

### Recommendation

Causal rather than realistic.

The first economy should support disrupted routines, trade-route consequences, theft motives, rewards, and authority budgets. Do not build full price theory first.

## 8. Should combat be deep?

### Recommendation

Not initially.

Combat only needs to support:

- threat;
- injury/death;
- witnesses;
- traces;
- fear;
- proof;
- institutional consequences.

Tactical richness can come later.

## 9. Should the game use LLMs?

### Recommendation

Not in the authoritative simulation. Optionally use them for grounded text after structured systems work.

Good first uses:

- notice paraphrase;
- rumor phrasing;
- case file summary;
- character memory summary;
- dialogue surface.

Bad first uses:

- agent planner;
- hidden truth generator;
- quest generator;
- evidence inventor.

## 10. Should the game guarantee fun events?

### Recommendation

No. It should guarantee readable artifacts and affordances, not authored drama.

Use salience detection to help players find interesting world processes. Do not inject events for pacing.

## 11. How much UI uncertainty is tolerable?

### Recommendation

A lot, if consistently presented.

Use labels:

- observed directly;
- believed by current actor;
- rumored;
- recorded;
- inferred;
- stale;
- contradicted;
- unknown.

Uncertainty is a feature.

## 12. Should the first village be hand-authored or generated?

### Recommendation

Hand-authored.

Generation adds noise. The first slice needs controlled causal tests. Add procedural villages after the ontology is stable.

## 13. How moral should the law be?

### Recommendation

Not perfectly moral. Law should be procedural, biased, resource-limited, and socially situated.

A corrupt or unfair institution is acceptable if the causes are legible.

## 14. Should characters have private internal thoughts?

### Recommendation

Yes, but only if they affect action. Avoid decorative inner monologue at first.

Private beliefs, intentions, and fears matter. Prose thoughts can wait.

## 15. What is the player’s main verb?

### Recommendation

The main verb is **intervene**.

The player observes, possesses, investigates, manipulates, steals, reports, lies, helps, frames, destroys records, posts notices, hides evidence, or simply watches.

The core pleasure is intervention in a causal social machine.
