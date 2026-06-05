# Tracewake Foundation Pack

Status: current baseline. This pack is authoritative; any earlier numbered drafts are superseded.

## North-star thesis

Tracewake is a causality-first, ordinary-life simulation where agents act from partial beliefs, public adventures arise from world processes, and the human player is only a temporary controller attached to an ordinary actor.

The player fantasy is to intervene, manipulate, and investigate: enter a town, talk to people, read notices, follow leads, hire companions, travel to sites, discover that information is stale or false, and adapt. The world must not know that a metaphysical player exists.

## Structure

The docs are organized into four tiers by dependency and change rate. Read top to bottom; later tiers depend on earlier ones. The tier folder, not the file number, expresses dependency level.

### `0-foundation/` — the constitution and its rationale (rarely changes)

The same rules appear here in three escalating layers: the Charter pillars (vision form), the Invariants (`INV-NNN` hard rules), and the TUI/scripting contracts (04–05, worked policy). Edit them as a set — a change to one layer must be reflected in the others, and the pillars and contracts cite the invariants they expand.

1. `0-foundation/01_PROJECT_CHARTER.md` — Project Charter
2. `0-foundation/02_FOUNDATIONAL_INVARIANTS.md` — Foundational Invariants (`INV-001` … `INV-052`)
3. `0-foundation/03_RESEARCH_AND_SOURCES.md` — Research Survey, Design Conclusions, and Sources
4. `0-foundation/04_TUI_FIRST_PLAYABILITY_CONTRACT.md` — TUI-First Playability Contract
5. `0-foundation/05_NO_SCRIPTING_AND_CAUSAL_AUTHORING_POLICY.md` — No Scripting and Causal Authoring Policy

### `1-architecture/` — subsystem designs (evolve alongside the code)

6. `1-architecture/06_WORLD_KERNEL_AND_EVENT_SOURCING.md` — World Kernel and Event Sourcing
7. `1-architecture/07_AGENT_COMPETENCE_AND_PLANNING_DECISION.md` — Agent Competence and Planning Decision
8. `1-architecture/08_INFORMATION_ECOLOGY_AND_SPEECH_ACTS.md` — Information Ecology and Speech Acts
9. `1-architecture/09_SOCIAL_INSTITUTIONS_AND_LAW.md` — Social Institutions and Law
10. `1-architecture/10_ORDINARY_LIFE_ECONOMY_AND_SETTLEMENTS.md` — Ordinary Life, Economy, and Settlements
11. `1-architecture/11_QUESTLESS_CONTENT_LEADS_AND_STORY_SIFTING.md` — Questless Content, Leads, and Story Sifting
12. `1-architecture/12_SPATIAL_REGIONAL_AND_TRAVEL_MODEL.md` — Spatial, Regional, and Travel Model
13. `1-architecture/13_PLAYER_MODEL_POSSESSION_AND_UI_VIEW_MODELS.md` — Player Model, Possession, and UI View Models
14. `1-architecture/14_LLM_LANGUAGE_SURFACE_BOUNDARY.md` — LLM Language Surface Boundary
15. `1-architecture/15_GENRE_AGNOSTIC_DOMAIN_MODEL.md` — Genre-Agnostic Domain Model
16. `1-architecture/16_WORLDGEN_HISTORY_AND_LONG_SIMULATION.md` — World Generation, History, and Long Simulation
17. `1-architecture/17_SIMULATION_LOD_TIME_AND_PERFORMANCE.md` — Simulation LOD, Time, and Performance

### `2-execution/` — build, test, and authoring plans (change frequently during implementation)

18. `2-execution/18_LIFE_POSSESSION_VERTICAL_SLICE_SPEC.md` — Life-Possession Vertical Slice Specification
19. `2-execution/19_ENGINEERING_ROADMAP_TUI_FIRST.md` — Engineering Roadmap: TUI-First
20. `2-execution/20_TESTING_VALIDATION_AND_DEBUGGING.md` — Testing, Validation, and Debugging
21. `2-execution/21_AUTHORING_GUIDE.md` — Authoring Guide
22. `2-execution/22_STARTER_DATA_SCHEMAS.md` — Starter Data Schemas

### `3-reference/` — lookups and living logs

23. `3-reference/23_DESIGN_RISK_REGISTER.md` — Design Risk Register
24. `3-reference/24_GLOSSARY.md` — Glossary

## First implementation posture

Build a deterministic headless simulation core and a serious playable TUI before any graphical app. The TUI is not a throwaway debug console. It is the first real player interface and should be architected through reusable UI view models that a future desktop UI can consume.

## First miracle

A village that notices what happened, misunderstands some of it, records some of it, talks about it, and continues living.

## Long-term promise

A region that has lived before the player entered it.

## Mantra

No telepathy. No destiny. No quest giver ontology. No sacred player. No invisible author. No boredom director. No simulation fact born from prose.
