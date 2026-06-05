# Engineering Roadmap

## Strategic build order

Build the smallest inspectable causal machine first.

Do not start with world generation, graphical UI, combat depth, open-ended LLM agents, large maps, kingdoms, magic systems, or procedural terrain.

Start with event-sourced headless simulation, entities/components/actions, beliefs/traces, ordinary routines, one institution, notice lifecycle, TUI, and debug/replay tools.

## Architecture

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
UI View Models ── TUI Renderer
   ↓
Future Graphical Renderer
```

## Phase 0 — Paper ontology

Deliver 20 primitive actions, 10 event types, 10 belief types, 10 trace types, 8 speech acts, theft chain, stale notice expedition, institution procedure cards, and no-quest review.

Exit: manually trace missing coins and stale notice without quest generator.

## Phase 1 — Headless kernel

Entity IDs, component storage, action framework, event log, deterministic scheduler, snapshots, rooms/places, containers/items/ownership.

Exit: replay theft and query why an item is where it is.

## Phase 2 — Belief and perception

Belief stores, observation events, direct perception, expectation contradiction, speech report, confidence/source, belief inspector.

Exit: owner discovers missing coins by checking; guard does not know unless told/shown.

## Phase 3 — Routines and ordinary life

Sleep/eat/work/social routines, intention persistence, simple HTN, interruptions, homes/workplaces/tavern, minimal economy.

Exit: village runs a day without player; routines break for modeled reasons.

## Phase 4 — Institutions

Local authority, clerk/guard roles, incident ledger, report action, simple investigation, bounty decision, notice writing/posting, proof/payment.

Exit: theft report becomes case; road threat becomes notice; notice can become stale.

## Phase 5 — TUI and possession

View models, embodied mode, possession switch, action menu, actor/player knowledge UI, lead notebook, notice board, debug event log/causal inspector.

Exit: player steals as Mara, switches to Tomas, and watches discovery without leakage.

## Phase 6 — Social propagation

Tavern gossip, rumor mutation, trust weighting, wrong suspicion, lie speech acts, simple reputation.

Exit: wrong suspect questioned for legible reasons.

## Phase 7 — Route and expedition

Route model, travel action, route risk, companion recruitment, stale notice expedition, arrival/departure events.

Exit: player reads notice, recruits help, travels, and finds situation changed for non-quest reasons.

## Phase 8 — Grounded LLM surfaces

Notice paraphrase, speech-act-to-dialogue rendering, case summary, memory summary, validation harness.

Exit: LLM output cannot create facts; mock tests pass deterministically.

## Phase 9 — Vertical slice

Build enough presentation and authored content to satisfy `16_VERTICAL_SLICE_SPEC.md`.

## Phase 10 — Region foundation

Multiple settlements, route network, outside boundary events, migration/trade/disease summaries, regional institutions, LOD promotion/demotion, long simulation snapshots.

## Technology posture

Use deterministic engine-agnostic core, data validation, property tests, golden scenario tests, TUI first, graphical renderer later. Avoid trapping game logic in UI widgets, engine scenes, or prompts.

## Data strategy

Use data files for actions, HTN methods, norms, roles, traces, object affordances, routines, institution procedures, notice templates, speech act templates, domain content, and route/site definitions. Code enforces. Data authors.
