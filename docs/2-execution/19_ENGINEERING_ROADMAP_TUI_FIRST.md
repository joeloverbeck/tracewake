# Engineering Roadmap: TUI-First

## Strategic build order

Build the smallest playable and inspectable causal machine first.

Do not start with:

- graphical UI;
- combat depth;
- procedural world generation;
- large maps;
- kingdoms;
- magic systems;
- open-ended LLM agents;
- authored quest boards.

Start with:

```text
event-sourced kernel
+ minimal TUI harness
+ actions/components
+ ordinary rooms/objects
+ needs/routines
+ beliefs/traces
+ possession debug binding
+ one institution
+ notice lifecycle
+ replay/debugging
```

## Architecture posture

```text
Authoring Data
   ↓
World Kernel ── Event Store ── Replay/Snapshot System
   ↓                 ↓
State Projections    Derived Streams
   ↓                 ↓
Agent Minds       Institution Records
   ↓                 ↓
Action Proposals  Public Artifacts / Leads
   ↓                 ↓
Actor-Knowledge Filters
   ↓
UI View Models ── TUI Renderer
   ↓
Future Graphical Renderer
```

The TUI renderer is a client, not a rules engine.

## Phase 0 — Paper ontology and data vocabulary

Deliver:

- 20 primitive actions;
- 10 event types;
- 10 belief proposition types;
- 10 trace types;
- 8 speech acts;
- theft chain;
- ordinary workday chain;
- stale notice chain;
- institution procedure cards;
- no-scripting review;
- TUI screen sketches for embodied/debug modes.

TUI gate: screen sketches and view-model contracts demonstrate how the first runnable kernel will be played and inspected.

Exit: manually trace missing coins, ordinary workday, and stale notice without quest generator.

## Phase 1 — Runnable kernel plus minimal TUI harness

Build:

- entity IDs;
- component storage;
- deterministic scheduler;
- action framework;
- event log;
- rooms/places;
- containers/items/ownership;
- minimal TUI local view;
- action menu;
- debug event log;
- possession attach command.

TUI gate: a user can move between rooms, inspect current actor/place, open a container, take/place an item if allowed, and see events in debug mode.

Exit: replay a simple item movement and query why an item is where it is.

## Phase 2 — Actor knowledge, perception, and why-not UI

Build:

- belief stores;
- observation events;
- direct perception;
- sound observation;
- expectation contradiction;
- speech report basics;
- confidence/source;
- actor-knowledge filter;
- why-not explanations.

TUI gate: embodied mode shows only actor-known facts; debug mode shows truth; blocked actions explain physical/knowledge/social reasons.

Exit: owner discovers missing coins by checking; guard does not know unless told or shown.

## Phase 3 — Needs, routines, and ordinary life

Build:

- hunger/fatigue/safety;
- sleep/eat/work actions;
- homes/workplace/tavern;
- intention persistence;
- simple HTN routines;
- interruptions;
- wait/sleep/continue-routine TUI controls;
- no-player day simulation.

TUI gate: player can embody any major actor, sleep/eat/work, wait through the day, and inspect routine interruption causes.

Exit: village runs a day without player; routines break for modeled reasons.

## Phase 4 — Possession parity and actor/player knowledge separation

Build:

- robust `HumanController -> ActorId`;
- possession history as debug metadata;
- actor knowledge vs human knowledge notebook distinction;
- action parity tests;
- UI labels for embodied vs debug mode.

TUI gate: player steals as Mara, switches to Tomas, and cannot accuse from player memory.

Exit: possession changes input binding only.

## Phase 5 — Institutions and records

Build:

- local authority;
- reeve/clerk/guard roles;
- office hours;
- incident ledger;
- report action;
- simple questioning procedure;
- theft norms;
- record reading permissions;
- institution debug inspector.

TUI gate: player can report a missing item to a clerk; accessible records are readable; debug mode shows institution belief/procedure state.

Exit: theft report becomes case without ground-truth leakage.

## Phase 6 — Notices, leads, and public artifacts

Build:

- notice authoring from structured claims;
- notice lifecycle;
- notice board as physical/public place artifact;
- actor-known lead notebook;
- stale information flags;
- destruction/removal/forgery basics if cheap.

TUI gate: player reads a notice, receives source-bound lead, sees uncertainty/staleness, and debug-inspects notice provenance.

Exit: route threat can become notice; notice can become stale.

## Phase 7 — Social propagation

Build:

- tavern gossip;
- rumor mutation;
- trust weighting;
- lying;
- refusal;
- suspicion;
- wrong-suspect questioning;
- simple reputation.

TUI gate: player can ask, answer, gossip, lie, and inspect actor-known conversation history; debug overlay shows structured speech acts.

Exit: wrong suspect questioned for legible reasons.

## Phase 8 — Route, travel, and stale lead action

Build:

- route model;
- travel action chain;
- route risk;
- companion recruitment;
- payment/share negotiation basics;
- arrival/departure events;
- aftermath threat representation without detailed combat.

TUI gate: player reads notice, recruits help, travels, and encounters changed/uncertain route/site state.

Exit: stale notice expedition works for non-quest reasons.

## Phase 9 — Grounded language surfaces with deterministic mocks

Build:

- template or mock LLM surfaces;
- notice paraphrase;
- speech-act-to-dialogue rendering;
- case summary;
- memory summary;
- validation harness.

TUI gate: dialogue and summaries appear in the TUI, but debug overlay shows structured speech acts and validation status.

Exit: language cannot create facts; core works with LLM disabled.

## Phase 10 — Vertical slice hardening

Build enough authored content, testing, metrics, replay tooling, and TUI polish to satisfy `18_LIFE_POSSESSION_VERTICAL_SLICE_SPEC.md`.

TUI gate: all vertical-slice scenarios pass through TUI-driven acceptance tests.

Exit: The Village That Notices is playable, inspectable, replayable, and no-player capable.

## Phase 11 — Region foundation

Build after the village works:

- multiple settlements;
- route network;
- outside boundary events;
- migration/trade/disease summaries;
- regional institutions;
- LOD promotion/demotion;
- long simulation snapshots;
- story sifting recaps.

TUI gate: player can inspect actor-known regional leads, travel between settlements, and debug LOD ancestry.

Exit: regional scale preserves causality.

## Technology posture

Use:

- deterministic engine-agnostic core;
- strict data validation;
- property tests;
- golden scenario tests;
- TUI first;
- graphical renderer later;
- no game logic in UI widgets;
- no game logic in prompts;
- no quest state.

## Data strategy

Use data files for:

- actions;
- HTN methods;
- norms;
- roles;
- traces;
- object affordances;
- routines;
- institution procedures;
- notice templates;
- speech-act templates;
- domain content;
- route/site definitions;
- LOD rules;
- scenario seeds;
- test fixtures.

Code enforces. Data authors.
