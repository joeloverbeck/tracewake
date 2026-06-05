# TUI-First Playability Contract

## Core claim

The TUI is the first real Tracewake client. It is not a temporary console, not a debug afterthought, and not a placeholder until graphics arrive.

The simulation core and TUI harness must grow together. The first runnable kernel must be playable through a minimal TUI. Every mechanic that matters must be reachable, inspectable, and testable through the TUI.

## Contract

### The TUI is product surface

The TUI must support real embodied play:

- moving through places;
- inspecting actor-perceived surroundings;
- seeing current actor status, needs, intentions, possessions, and beliefs;
- using object affordances;
- talking through structured speech acts;
- reading notices and records available to the current actor;
- forming actor-known lead cards;
- waiting, sleeping, working, traveling, and continuing routines;
- switching possession in debug/development mode;
- seeing why actions are unavailable;
- inspecting causal chains in debug mode.

A mechanic that cannot be used or inspected from the TUI is not finished.

### Kernel and TUI advance together

The implementation order is not:

```text
headless simulation for months -> TUI later
```

The implementation order is:

```text
minimal kernel + minimal TUI harness
-> new mechanic
-> TUI reachability
-> embodied knowledge filter
-> debug inspection
-> automated TUI acceptance test
```

### Every runnable phase has a TUI acceptance gate

Each phase must define a playable TUI proof. Examples:

- after the action pipeline exists, the TUI can list and attempt local actions;
- after containers exist, the TUI can open, inspect, take, place, and explain blocked access;
- after beliefs exist, the TUI can show actor-known beliefs and hide hidden truth;
- after routines exist, the TUI can wait, sleep, and observe routine state;
- after institutions exist, the TUI can report to a clerk, read records if accessible, and inspect debug institutional state;
- after notices exist, the TUI can read physical notice board artifacts and create actor-known leads;
- after travel exists, the TUI can prepare, depart, traverse, arrive, and inspect route events.

### TUI tests are mandatory

Every phase needs automated tests that drive view models or terminal-level interactions sufficiently to prove playability. Pure headless tests remain necessary but insufficient.

## Embodied mode

Embodied mode is the normal playable mode.

It shows:

- current actor identity and body state;
- perceived place, exits, nearby actors, objects, traces, and affordances;
- actor-known beliefs, memories, leads, obligations, and records;
- actor-available actions;
- actor-accessible conversation options;
- actor-relevant event/conversation log;
- uncertainty and provenance labels where the actor has them.

It hides:

- ground truth not perceived by the actor;
- hidden containers and hidden actors;
- other minds' beliefs unless communicated or inferred;
- true culprit labels;
- objective quest truth;
- debug causal graph;
- omniscient institution state;
- player possession history as world fact.

## Debug mode

Debug mode is mandatory during development and must be visually distinct.

It may show:

- authoritative event log;
- causal graph;
- component state;
- action proposals and rejected preconditions;
- belief stores for all agents;
- hidden traces;
- institution records and private procedures;
- planner decisions;
- LOD tier and promotion/demotion events;
- replay/snapshot state;
- random draw records;
- speech-act validation state;
- actor/player knowledge mismatch diagnostics.

Debug mode must never be mistaken for embodied truth.

## Why-not UI

Blocked actions must explain causal constraints without leaking more than the selected mode permits.

Embodied examples:

```text
You cannot accuse Mara with evidence: Tomas has no actor-known evidence linking Mara to the coins.
```

```text
You cannot take the locked ledger: the office chest is locked, and Tomas does not have the key.
```

Debug overlay may add:

```text
Ground truth: Mara stole the coins at 02:13. Tomas has no observation, testimony, trace interpretation, or record connecting her.
```

## Lead notebook

The notebook stores source-bound information, not objective objectives.

It must distinguish:

```text
Known by human
Known by current actor
Directly observed
Heard as testimony
Rumored
Read from notice
Recorded by institution
Inferred
Contradicted
Stale
Debug-only truth
```

A lead card may suggest possible actions, but suggestions must be actor-possible and source-bound.

## Conversation UI

The conversation interface must expose enough structure to keep language grounded:

- speaker;
- listener;
- surface utterance;
- structured speech act in developer overlay;
- claims learned by current actor;
- confidence and source where available;
- whether the actor treats a statement as hearsay, rumor, report, promise, threat, command, or lie suspicion.

Embodied mode must not label a statement as a lie unless the actor has reason.

## Time controls

The TUI must support:

- pause;
- step;
- wait;
- sleep as actor action;
- continue current intention;
- time acceleration;
- travel time;
- stop on salient perceived interruption;
- missed-event summaries filtered by current actor knowledge.

Waiting is not a skip over causality. It runs the simulation.

## Future graphical client

A later graphical desktop app should reuse:

```text
simulation core
action pipeline
view models
actor-knowledge filters
notebooks
speech-act interface
debug/replay architecture
```

The graphical app should mostly add graphics, animation, layout, sound, presentation, and richer input. It must not become a second rules engine.

## TUI architectural boundary

The TUI renderer must consume view models. It must not query ground truth directly for embodied play. It must not implement simulation logic. It must not contain action rules. It must not invent leads. It must not maintain separate quest state.

Correct:

```text
Simulation core -> projections -> actor-filtered view models -> TUI renderer
```

Forbidden:

```text
TUI button -> custom player-only state mutation
TUI notebook -> objective quest truth
TUI map -> true hidden target location
TUI dialogue -> prose creates fact
```

## Minimal permanent TUI capabilities

Even in early versions, maintain these as first-class surfaces:

- local place view;
- current actor panel;
- needs/status/intention panel;
- nearby actors/objects/traces;
- action menu;
- inventory/storage interface;
- belief/lead notebook;
- conversation/report interface;
- notice board/record reading;
- possession switch in debug mode;
- event log in debug mode;
- causal inspector in debug mode;
- why-not explanations;
- replay test harness.

## Acceptance rule

When reviewing any feature, ask:

```text
Can an ordinary actor use it?
Can the human use it through the current body?
Can the TUI show what the actor knows?
Can the TUI hide what the actor does not know?
Can debug mode explain what really happened?
Can an automated TUI acceptance test prove this?
```

If the answer is no, the feature is not done.
