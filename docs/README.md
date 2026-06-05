# Tracewake Foundation Pack v2

Status: replacement foundation pack. Adopt this set as the new baseline and treat the earlier files as superseded drafts.

## North-star thesis

Tracewake is a causality-first regional life simulation where ordinary agents act from partial beliefs, public adventures arise from world processes, and the human player is only a temporary controller attached to an ordinary actor.

The player fantasy is to intervene, manipulate, and investigate: enter a town, talk to people, read notices, follow leads, hire companions, travel to sites, discover that information is stale or false, and adapt. The world must not know that a metaphysical player exists.

## What v2 changes

The first pack had the correct philosophical center: no sacred player, no telepathy, no quest ontology, event-sourced causality, belief-before-truth, fallible institutions, and ordinary life as substrate.

This pack strengthens what was missing:

- a concrete agent competence architecture;
- a validated speech-act and LLM boundary;
- a TUI-first player interface plan;
- a spatial, regional, route, and travel model;
- long-running history simulation;
- genre/domain pack separation;
- explicit LOD and performance strategy;
- a vertical slice that includes recruiting companions and pursuing stale leads.

## Structure

The docs are organized into four tiers by dependency and change rate. Read top to bottom; later tiers depend on earlier ones.

### `0-foundation/` — the constitution and its rationale (rarely changes)

1. `0-foundation/01_PROJECT_CHARTER.md`
2. `0-foundation/02_FOUNDATIONAL_INVARIANTS.md`
3. `0-foundation/03_RESEARCH_AND_SOURCES.md` — research survey, design conclusions, and the full citation list

### `1-architecture/` — subsystem designs (evolve alongside the code)

4. `1-architecture/04_WORLD_KERNEL_AND_EVENT_SOURCING.md`
5. `1-architecture/05_AGENT_ARCHITECTURE.md`
6. `1-architecture/06_INFORMATION_ECOLOGY_AND_SPEECH_ACTS.md`
7. `1-architecture/07_SOCIAL_INSTITUTIONS_AND_LAW.md`
8. `1-architecture/08_QUESTLESS_CONTENT_AND_LEADS.md`
9. `1-architecture/09_ORDINARY_LIFE_ECONOMY_AND_SETTLEMENTS.md`
10. `1-architecture/10_SPATIAL_REGIONAL_AND_TRAVEL_MODEL.md`
11. `1-architecture/11_PLAYER_MODEL_TUI_AND_UI.md`
12. `1-architecture/12_LLM_DIALOGUE_AND_TEXT_BOUNDARY.md`
13. `1-architecture/13_GENRE_AGNOSTIC_DOMAIN_MODEL.md`
14. `1-architecture/14_WORLDGEN_HISTORY_AND_LONG_SIMULATION.md`
15. `1-architecture/15_SIMULATION_LOD_TIME_AND_PERFORMANCE.md`

### `2-execution/` — build, test, and authoring plans (change frequently during implementation)

16. `2-execution/16_VERTICAL_SLICE_SPEC.md`
17. `2-execution/17_ENGINEERING_ROADMAP.md`
18. `2-execution/18_TESTING_VALIDATION_AND_DEBUGGING.md`
19. `2-execution/19_AUTHORING_GUIDE.md`
20. `2-execution/20_STARTER_DATA_SCHEMAS.md`

### `3-reference/` — lookups and living logs

21. `3-reference/21_OPEN_DESIGN_DECISIONS_AND_RISKS.md`
22. `3-reference/22_GLOSSARY.md`

File numbers are preserved as stable IDs; the tier folder, not the number, expresses dependency level.

## Replacement notes

This is not a patch. It is a restructured replacement set. The old `AGENT_COGNITION`, `PLAYER_MODEL_AND_UI`, `ENGINEERING_ROADMAP_AND_RISKS`, and `OPEN_DESIGN_QUESTIONS` documents are split or expanded because those areas now carry more architectural weight. The former `03_RESEARCH_SURVEY_AND_DESIGN_CONCLUSIONS.md` and `99_SOURCES.md` are merged into `0-foundation/03_RESEARCH_AND_SOURCES.md`.

## First implementation posture

Build a deterministic headless simulation core and a serious playable TUI before any graphical app. The TUI is not a throwaway debug console. It is the first real player interface and should be architected through reusable UI view models that a future desktop UI can consume.

## First miracle

A village that notices what happened, misunderstands some of it, records some of it, talks about it, and continues living.

## Long-term promise

A region that has lived before the player entered it.

## Mantra

No telepathy. No destiny. No quest giver ontology. No sacred player. No invisible author. No boredom director. No simulation fact born from prose.
