# Causality-First Living World — Foundation Pack

Research reviewed: 2026-06-05.

This folder is the project canon for a game whose world does not know who the player is, whose agents act from partial beliefs, and whose “quests” are visible consequences of simulated problems rather than authored content packets.

## North-star thesis

> Build a world where every event can be interrogated as causality, every character acts from partial belief, every institution is made of fallible procedures, and every “quest” is merely a visible scar left by the world trying to solve its own problems.

The player is a controller temporarily attached to one actor. The player is not the metaphysical center of the world.

## The major design decision

The project should not be designed around procedural storytelling. It should be designed around **epistemic causality**:

- what happened;
- who observed it;
- what traces it left;
- who believes what;
- who is wrong;
- who has authority, motive, fear, or duty to act;
- what public artifacts are produced;
- what later decisions depend on those artifacts.

Most games simulate events. This one must simulate the **life of consequences**.

## Recommended reading order

1. `01_PROJECT_CHARTER.md` — the compact statement of what the project is and is not.
2. `02_FOUNDATIONAL_INVARIANTS.md` — hard rules that prevent the design from collapsing into normal RPG logic.
3. `03_RESEARCH_SURVEY.md` — precedents, papers, and what to borrow or reject.
4. `04_WORLD_KERNEL_AND_EVENT_SOURCING.md` — the authoritative simulation core.
5. `05_AGENT_COGNITION.md` — beliefs, desires, intentions, planning, emotion, and interruption.
6. `06_INFORMATION_ECOLOGY_AND_TRACES.md` — observation, rumor, lies, evidence, and stale knowledge.
7. `07_SOCIAL_INSTITUTIONS_AND_LAW.md` — law, authority, offices, records, jurisdiction, and sanctions.
8. `08_QUESTLESS_CONTENT_MODEL.md` — replacing quests with incidents, needs, contracts, and notices.
9. `09_ORDINARY_LIFE_AND_ECONOMY.md` — why eating, sleeping, working, trade, and domestic routines come first.
10. `10_PLAYER_MODEL_AND_UI.md` — possession, player knowledge, salience, journals, and causal inspection.
11. `11_VERTICAL_SLICE_SPEC.md` — the first playable village and its required tests.
12. `12_ENGINEERING_ROADMAP_AND_RISKS.md` — build order, architecture, performance strategy, and traps.
13. `13_TESTING_VALIDATION_AND_DEBUGGING.md` — invariant tests, scenario tests, fuzzing, and forensic replay.
14. `14_AUTHORING_GUIDE.md` — how to author causal materials without writing quests.
15. `15_GLOSSARY.md` — shared terminology.
16. `99_SOURCES.md` — bibliography and research notes.

## What this foundation pack assumes

The first version should be a **headless causal simulation** with a minimal UI, not a beautiful RPG. The correct first milestone is not combat, not dialogue, not terrain, and not world generation. The correct first milestone is a village that can notice something happened.

## What this foundation pack forbids

The following shortcuts should be treated as project-threatening:

- a quest table that generates tasks directly for the player;
- omniscient guards;
- event triggers that exist because the player is nearby;
- NPCs who query global truth rather than belief state;
- scripted investigation outcomes;
- dramatic pacing directors;
- LLM output that mutates world state without simulation validation;
- schedules that override reality;
- fake notice boards that are merely UI menus.

## Project mantra

> No telepathy. No destiny. No quest giver ontology. No sacred player. No invisible author.
