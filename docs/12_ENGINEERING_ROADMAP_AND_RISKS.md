# Engineering Roadmap and Risks

## Strategic build order

Build the smallest inspectable causal machine first.

Do not start with:

- world generation;
- combat depth;
- procedural dialogue;
- large maps;
- factions and kingdoms;
- LLM agents;
- art pipeline;
- magic systems.

Start with:

- event-sourced headless simulation;
- entities/components/actions;
- beliefs;
- traces;
- ordinary routines;
- one institution;
- notice lifecycle;
- debug UI.

## Architecture overview

```text
Authoring Data
   ↓
World Kernel ── Event Store ── Replay/Snapshot System
   ↓                 ↓
State Projections    Derived Logs
   ↓                 ↓
Agent Minds       Institution Records
   ↓                 ↓
Action Proposals  Public Artifacts
   ↓                 ↓
Simulation UI / Debug UI / Player UI
```

## Phase 0 — Paper prototype

Goal: validate ontology before code.

Deliverables:

- 20 primitive actions;
- 10 event types;
- 10 belief types;
- 10 trace types;
- theft chain on paper;
- bounty chain on paper;
- institution procedure cards;
- “no quest primitive” review.

Exit criteria:

- Can manually walk through missing-coins scenario.
- Can identify which actors know which facts at each step.
- Can explain how notice appears without quest generator.

## Phase 1 — Headless kernel

Build:

- entity IDs;
- component storage;
- action preconditions/effects;
- event log;
- deterministic tick/event scheduler;
- snapshots;
- basic rooms and movement;
- containers/items/ownership.

Exit criteria:

- Can replay theft event chain.
- Can query “why is item here?” from event history.
- No significant state mutation outside action/event pipeline.

## Phase 2 — Belief and perception

Build:

- actor belief stores;
- observation events;
- direct perception;
- expectation contradiction;
- hearsay report;
- confidence and source;
- belief inspector.

Exit criteria:

- Owner discovers missing coins by checking container.
- Witness can hold uncertain noise belief.
- Guard does not know theft unless told or shown evidence.

## Phase 3 — Routines and needs

Build:

- sleep/eat/work/social routines;
- intention persistence;
- simple HTN decomposition;
- interruptions;
- homes/workplaces/tavern.

Exit criteria:

- Village runs a day without player.
- Routines break for modeled reasons.
- Agents do not oscillate constantly.

## Phase 4 — Institution layer

Build:

- local authority;
- clerk/guard roles;
- incident ledger;
- report action;
- simple investigation;
- bounty decision;
- notice writing/posting.

Exit criteria:

- Theft report becomes case file.
- Bandit report becomes notice.
- Notice can become stale.

## Phase 5 — Player possession and UI

Build:

- controller binding;
- possession switch;
- actor knowledge vs player knowledge UI;
- lead notebook;
- notice board UI;
- causal inspector.

Exit criteria:

- Player can steal as Mara, switch to Tomas, and watch discovery without knowledge leakage.

## Phase 6 — Social propagation

Build:

- tavern gossip;
- rumor mutation;
- relationship trust weighting;
- wrong suspicion;
- lying;
- basic reputation.

Exit criteria:

- Wrong suspect can be questioned for legible reasons.
- Rumors can contradict records.

## Phase 7 — Playable vertical slice

Build only enough presentation to make the village playable.

Exit criteria:

- All scenarios in `11_VERTICAL_SLICE_SPEC.md` pass.
- 30-minute playtest generates at least one unscripted, explainable situation.

## Data strategy

Use data files for:

- action definitions;
- HTN methods;
- norm definitions;
- role definitions;
- trace definitions;
- object affordances;
- routine templates;
- institution procedures;
- notice templates.

Keep code responsible for enforcement, not content specifics.

## Suggested technology posture

A good implementation stack should prioritize:

- deterministic headless simulation;
- fast iteration;
- excellent debugging;
- data validation;
- testability;
- UI later.

Possible approaches:

### Rust/C#/Kotlin/TypeScript core

Good for type safety and maintainability.

### Godot/Unity frontend later

Good for eventual presentation, but dangerous if gameplay code gets trapped in scene scripts.

### Recommendation

Keep the simulation core engine-agnostic at first. Build a debug UI around it. Attach a game engine later.

## Performance risks

### Risk: too many agents planning too often

Mitigations:

- event-driven wakeups;
- routine caching;
- LOD simulation;
- intention persistence;
- planner budgets;
- HTN before GOAP;
- batch low-salience routines.

### Risk: event log grows without bound

Mitigations:

- snapshots;
- event compaction for routine low-salience events;
- keep significant chains in detail;
- derived projections;
- archival summaries.

### Risk: belief stores explode

Mitigations:

- only store salient beliefs;
- merge low-value memories;
- confidence decay;
- semantic summaries;
- cap rumor chains;
- use queries by relevance.

### Risk: debugging emergence becomes impossible

Mitigations:

- causal IDs from day one;
- replay from day one;
- “why did this happen?” from day one;
- property tests;
- scenario golden tests;
- inspector UI before art.

## Design risks

### Risk: accidental protagonist gravity

Symptoms:

- events happen near player too often;
- NPCs wait for player;
- tasks target player;
- UI reveals objective truth.

Mitigation:

- run no-player simulations;
- measure player-involved vs non-player event rates;
- forbid player-conditioned generation in core.

### Risk: procedural quest relapse

Symptoms:

- `Quest` object appears;
- reward spawns on completion;
- notices are menu entries;
- targets exist because tasks require them.

Mitigation:

- review every content feature against questless ontology.

### Risk: LLM contamination

Symptoms:

- generated dialogue invents evidence;
- LLM decides who is guilty;
- agents act on prose not structured beliefs.

Mitigation:

- strict validation layer;
- LLM output maps to existing propositions only;
- no LLM in authoritative event execution.

### Risk: simulation without game

Symptoms:

- impressive logs but no player desire;
- everything is technically causal but boring;
- the player cannot find meaningful handles.

Mitigation:

- build salience UI;
- ensure artifacts are readable;
- make intervention affordances strong;
- focus first slice on theft, law, rumor, notices.

## Milestone names

1. **The Strongbox** — event-sourced theft.
2. **The Witness** — uncertain observation and belief contradiction.
3. **The Office** — report and case record.
4. **The Board** — bounty notice lifecycle.
5. **The Switch** — possession without player ontology.
6. **The Wrong Man** — rumor and false suspicion.
7. **The Village That Notices** — full vertical slice.

## Strong recommendation

The first public prototype, if any, should be framed as an “emergent village forensic simulator,” not an RPG. That gives permission for crude presentation while emphasizing the novel core.
