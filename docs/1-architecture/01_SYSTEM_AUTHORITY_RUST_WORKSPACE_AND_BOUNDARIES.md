# System Authority, Rust Workspace, and Boundaries

## Status

This document defines Rust-first architectural boundaries and authority ownership. It does not freeze crate names, third-party crates, storage engines, ECS/database choices, TUI libraries, graphical frameworks, module layout, or deployment model.

Rust is a hard implementation constraint for the authoritative simulation because Tracewake needs deterministic discipline, memory safety, performance, tooling, maintainability, and a serious long-lived simulation codebase.

## Core rule

Tracewake is not a UI shell, game-engine scene graph, LLM chatbot, or database schema with a simulation bolted on. The simulation kernel owns authoritative world rules. Everything else submits proposals, provides authored possibility space, consumes projections, or inspects diagnostics.

## Authority ownership by subsystem

### Authored content and validation

Owns:

- domain packs;
- entity/component templates;
- action definitions;
- affordance definitions;
- norms, roles, institutions, records, routines, HTN methods, speech templates, and scenario seeds;
- schema and semantic validation reports.

Denied authority:

- committing world events directly;
- forcing outcome chains;
- creating player-only actions;
- bypassing belief provenance;
- making LLM output authoritative.

Boundary data:

```text
versioned content packages -> validated registries -> kernel/action/agent/institution/language subsystems
validation reports -> debug/tools/acceptance artifacts
```

### Simulation kernel

Owns:

- authoritative current state projection during execution;
- entity identity allocation policy;
- proposal validation;
- invariant enforcement;
- outcome resolution;
- state mutation plan;
- event commit;
- causal link emission;
- no-player action parity.

Denied authority:

- prose generation;
- dramatic pacing;
- quest state;
- UI layout;
- hidden player privilege;
- live LLM calls;
- filesystem/network decisions during outcome resolution.

Boundary data:

```text
validated proposal + current projections + content registries
  -> event commits, rejections, scheduled actions, traces, observation candidates
```

### Scheduler and time

Owns:

- discrete simulation time;
- stable ordering;
- scheduled action instances;
- reservations;
- wakeups;
- interruption windows;
- time acceleration as scheduler stepping;
- random stream coordination;
- no-human simulation stepping.

Denied authority:

- protagonist priority;
- wall-clock ordering;
- hidden pacing intervention;
- moving actors by fiat;
- privileging possessed actors.

### Event log, replay, and save packages

Owns:

- append-only event semantics;
- event envelopes;
- causal graph links;
- schema versions and upcaster policy;
- replay modes and failure reports;
- save-package manifest;
- random stream state and meaningful draw audit;
- snapshot/projection metadata.

Denied authority:

- editing old history;
- silently repairing replay;
- replacing causality with current-state saves;
- treating projection caches as truth.

### Projections

Owns rebuildable read models:

- current component state;
- actor belief stores;
- trace and observation indexes;
- institutional records;
- notice board state;
- lead cards;
- TUI view models;
- debug views;
- causal graph indexes;
- metrics/report projections.

Denied authority:

- overriding the event log;
- mutating world truth;
- leaking hidden truth into embodied mode;
- inventing facts to simplify display.

### Epistemics

Owns:

- typed claims/propositions;
- observation and interpretation pipelines;
- belief updates;
- memory summaries;
- trace discovery and interpretation;
- rumor propagation;
- stale information;
- actor-knowledge filtering.

Denied authority:

- truth transfer without modeled channel;
- institution or planner ground-truth reads;
- evidence-as-certainty shortcuts.

### Agent cognition

Owns:

- needs, values, traits, relationships, roles, projects, motives;
- durable intentions;
- routine and HTN method selection;
- bounded local planning;
- action proposals;
- planner traces and failure reports;
- per-agent detail tiers.

Denied authority:

- event commit;
- hidden truth reads;
- LLM-brain decisions;
- utility jitter as whole mind;
- direct state mutation.

### Institutions and households

Own:

- roles;
- obligations, permissions, prohibitions, constitutive norms, and evidence thresholds;
- records, reports, notices, procedures, jurisdiction, bias, corruption, sanctions, failures;
- household access, privacy, shared property expectations, domestic roles, and records/secrets.

Denied authority:

- omniscient enforcement;
- sanction from hidden truth;
- global wanted meters;
- records without artifacts or authors.

### TUI and future clients

Own:

- presentation;
- input collection;
- view-model consumption;
- actor-known notebook display;
- menus, why-not display, logs, debug toggles;
- transcript capture for tests.

Denied authority:

- world mutation;
- unique action rules;
- ground-truth embodied display;
- second validation engine;
- player-only verbs.

### Debug, replay, and review tooling

Own:

- non-diegetic inspection;
- event/causal/projection/belief/planner/scheduler/LOD/content/LLM diagnostics;
- deterministic replay tooling;
- validation artifacts and metrics export.

Denied authority:

- silent leakage into embodied play;
- hidden mutation disguised as ordinary action;
- accepting manual demos in place of validation reports.

### Language surfaces

Own:

- deterministic templates;
- optional future prompt packets;
- optional parser proposals;
- validation, repair, rejection, and deterministic mocks.

Denied authority:

- deciding truth;
- choosing plans;
- committing facts;
- inventing clues;
- summarizing uncertainty as certainty;
- requiring live LLM output for simulation correctness.

## Logical Rust workspace seams

These names are examples of logical seams, not frozen crate names:

```text
tracewake-kernel
tracewake-content
tracewake-events
tracewake-replay
tracewake-projections
tracewake-scheduler
tracewake-epistemics
tracewake-agents
tracewake-institutions
tracewake-language-surface
tracewake-tui
tracewake-debug
tracewake-validation
tracewake-cli-tools
tracewake-client-boundary
```

A first implementation may combine several seams into fewer crates. The dependency and authority rules still apply.

## Dependency direction

Preferred direction:

```text
content definitions -> validation -> registries -> kernel/action/agent/institution/language
commands/proposals -> kernel -> event log -> projections -> TUI/debug/replay/metrics
agents/institutions/processes -> proposals -> kernel
language surfaces -> validated speech/action proposals -> kernel
TUI/future clients -> commands -> kernel; view models <- projections
```

Forbidden direction:

```text
TUI -> mutate state directly
TUI -> read hidden truth in embodied mode
LLM -> decide NPC action or commit facts
storage adapter -> define domain rules
domain pack -> create events directly
projection cache -> override event log
debug view -> write actor notebook without modeled transfer
graphical client -> implement unique interaction rules
scheduler -> privilege possessed actor
```

## Side-effect policy

Authoritative simulation code must isolate side effects.

Forbidden inside authoritative rules:

- wall-clock decisions;
- live network calls;
- live LLM calls;
- filesystem reads during outcome resolution except via loaded/versioned content handles;
- unordered map iteration affecting outcomes;
- thread scheduling races affecting event order;
- hidden global random source;
- terminal or graphical frame timing changing simulation state.

Allowed at boundaries:

- loading and validating content;
- saving event/save packages;
- logging;
- debug export;
- TUI input;
- optional language services;
- metrics;
- tooling.

## Storage boundary

Persistence must preserve append-only event semantics, content/schema versions, snapshots, projection metadata, random stream state, and replay diagnostics. The storage engine is not architecture doctrine.

Acceptable early forms include append-only files plus manifests and snapshot/projection cache files. Later storage may use an embedded database or event-store adapter. In all cases, database rows, documents, or objects are persistence strategies. World authority remains validated events plus content versions plus rebuildable projections.

## ECS and game-framework stance

ECS-like storage may help query entities and components. A game framework may help presentation or scheduling at a boundary. Neither is root architecture.

These outrank ECS/framework concerns:

- event sourcing;
- deterministic replay;
- action parity;
- actor-knowledge filtering;
- causal provenance;
- no-human simulation;
- TUI playability;
- content validation;
- debug inspection.

Reject or constrain any framework that encourages frame-driven hidden mutation, graphical-first assumptions, nondeterministic iteration, or UI-rule entanglement.

## Boundary review checklist

A subsystem boundary is acceptable only if it answers:

- What authority does it own?
- What authority is explicitly denied?
- What data crosses the boundary?
- Is the boundary deterministic under replay?
- Can it run with no human attached?
- Can it run with LLMs disabled?
- Does it preserve actor-knowledge filtering?
- Does it submit world-affecting changes through the action/event pipeline?
- Can debug inspect its decisions?
- Can the TUI exercise it without graphical dependencies?

## Anti-patterns

- `Player` crate with privileged verbs.
- `Quest` crate that controls truth or completion.
- `DialogueAI` crate that chooses NPC actions.
- Storage schema treated as current truth without replay.
- Graphical client owning interaction rules.
- Domain pack writing events directly.
- Debug notebook becoming actor memory.
- Scheduler pausing NPCs because a human is thinking.
