# Project Charter

## Working title

**Causality-First Living World**

This is a placeholder title, not a brand. The important phrase is “causality-first.”

## One-sentence pitch

A living-world simulation where every character acts from partial beliefs, every institution is fallible, and every adventure hook is an artifact produced by the world attempting to solve its own problems.

## Product fantasy

The player enters a village. People are sleeping, working, drinking, arguing, storing valuables, lying, gossiping, worshipping, guarding roads, delivering grain, cheating customers, tending children, and hiding secrets. The player steals coins from a house, leaves, switches to another character, and later watches the owner discover the loss, search the room, question the household, report the matter, and perhaps accuse the wrong person.

A bounty appears on the notice board not because a quest generator fired, but because bandits attacked travelers, testimony reached an authority, the authority allocated funds, a clerk wrote the notice, and someone physically posted it. If the bandits were already killed by unrelated adventurers, the notice remains stale until that information reaches the office.

## North-star experience

The player should often think:

> “This was not written for me, but I can see why it happened.”

The game should reward investigation, opportunism, patience, social manipulation, planning, and curiosity more than completionism.

## Design pillars

### 1. Causality before drama

No event should happen because a story arc demands it. Events happen because local conditions, actors, processes, institutions, or environmental forces made them happen.

A dramatic arc may emerge in hindsight. It must never be an invisible input.

### 2. Belief before truth

Agents do not act from ground truth. They act from beliefs, suspicions, habits, fears, obligations, desires, and memories.

This means mistaken action is first-class design, not a bug.

### 3. Institutions are simulated actors

Law, commerce, religion, guilds, households, and governments are not menus. They are systems made of people, roles, records, budgets, procedures, norms, jurisdiction, corruption, and delay.

### 4. Quests are projections

A quest is not an engine primitive. A “quest” is a player-facing view over an incident, request, contract, threat, obligation, debt, rumor, or opportunity.

### 5. Ordinary life is the substrate

Adventure only matters if ordinary life is real enough to be disrupted. Homes, work, sleep, property, meals, trade, kinship, reputation, and routine are the foundation of the simulation.

### 6. The player is not special

The world should not contain a privileged `PlayerCharacter` concept. The human is attached to an actor through a controller binding. That binding can move.

### 7. Language renders; simulation decides

LLMs may paraphrase, summarize, produce surface dialogue, or write notices from structured facts. They may not invent hidden truth, bypass preconditions, decide outcomes, or mutate authoritative state.

## Non-goals

This project is not:

- a procedural quest generator;
- a storylet engine;
- RimWorld with deeper pawns;
- Skyrim with better radiant quests;
- a generative-AI NPC chat demo;
- an authored narrative RPG;
- a general-purpose civilization simulator at launch;
- a complete medieval economy at launch;
- a game where the player is secretly the chosen causal center.

## Required first miracle

The first miracle is not scale. The first miracle is **forensic intelligibility**.

Given any surprising event, a player or developer should be able to ask:

- What happened?
- Why was it possible?
- Who caused it?
- Who observed it?
- What did it change?
- What traces did it leave?
- Who now believes what?
- What later events became possible because of it?

If the engine cannot answer those questions, it is not yet the target game.

## Definition of success for the prototype

A successful prototype can run a small village for several simulated days and produce non-authored situations such as:

- a theft discovered after a delay;
- an authority investigating with incomplete information;
- a notice posted for a threat;
- a stale notice remaining after the threat is gone;
- a wrong accusation based on partial observation;
- a character failing to work because ordinary preconditions broke;
- a player switching bodies without resetting the world’s interpretation of events.

## Strategic bet

The bet is that players do not need the world to produce perfect stories. They need the world to produce **legible consequences**.

The entertainment loop is not “receive quest, complete quest, collect reward.” It is:

1. notice an artifact;
2. infer a chain;
3. intervene;
4. cause consequences;
5. watch the world misunderstand, adapt, punish, reward, or exploit those consequences.

## Strong opinion

Do not begin with map generation, combat depth, writing, magic, kingdoms, or dialogue. Begin with a small social machine that can be wrong.

A village that can falsely accuse an innocent person from partial evidence is closer to the desired game than a continent with a thousand content templates.
