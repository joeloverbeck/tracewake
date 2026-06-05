# Rust Workspace and System Boundaries

## Status

This document defines logical Rust-first architectural boundaries. It does not freeze final crate names, third-party crates, storage engines, ECS libraries, TUI libraries, or database choices.

Rust is a hard implementation constraint. Tracewake needs safety, determinism discipline, performance, tooling, maintainability, and a serious alternative to C++ for a long-lived simulation codebase.

## Boundary doctrine

Tracewake is not a game-engine shell with a simulation bolted on. The simulation kernel owns the authoritative rules. Everything else submits proposals or consumes projections.

Hard boundary rules:

- UI must not become a second rules engine.
- Persistence must not become the simulation model.
- LLM surfaces must not become agent brains.
- Domain packs must not bypass the action/event pipeline.
- Debug tools must not leak hidden truth into embodied play.
- Graphical presentation must remain a client boundary, not an architectural driver.
- ECS-like storage may be useful; adopting an ECS/game framework is not doctrine.

## Logical workspace shape

The names below are proposed logical package names, not frozen crate names.

```text
tracewake-workspace
  tracewake-kernel
  tracewake-content
  tracewake-scheduler
  tracewake-epistemics
  tracewake-agents
  tracewake-institutions
  tracewake-events
  tracewake-projections
  tracewake-tui
  tracewake-debug
  tracewake-replay
  tracewake-test-harness
  tracewake-language-surface
  tracewake-graphical-client-boundary
  tracewake-cli-tools
```

A smaller first implementation may combine several of these into fewer crates. The logical seams still matter.

## Dependency direction

Allowed direction:

```text
content definitions -> validation -> kernel/action registry
kernel -> events -> projections -> TUI/debug/replay
agents -> action proposals -> kernel
institutions -> action proposals/procedure hooks -> kernel
language surfaces -> validated speech/action proposals -> kernel
TUI -> commands -> kernel
future graphical client -> commands/view models only
```

Forbidden direction:

```text
TUI -> mutate world state directly
TUI -> read ground truth in embodied mode
LLM -> mutate authoritative state
LLM -> choose NPC plans
storage adapter -> define domain rules
domain pack -> create events without kernel validation
debug view -> supply knowledge to actor notebook
projection -> override event log
future graphical client -> implement unique action rules
```

## Core package responsibilities

### Simulation kernel

Owns authoritative world state projection, entity identity, action validation, reservation checks, outcome resolution, state mutation, event commit, simulation invariants, and causality links.

The kernel does not own prose, dramatic pacing, quest state, presentation layout, LLM prompts, or debug-only affordances.

### Event system

Owns event envelopes, event schema identifiers, append-only semantics, event ordering, causal links, stream/index contracts, snapshot contracts, projection subscriptions, replay metadata, and event schema migration/upcasting policy.

It does not choose story salience or decide player-facing wording.

### Content/data loading and validation

Owns loading authored data, domain packs, scenario seeds, fixtures, action definitions, object affordances, norms, routines, HTN methods, speech-act templates, record definitions, and validation reports.

It does not commit world events. It creates possibility space, not outcome chains.

### Scheduler/time

Owns discrete simulation time, event-driven wakeups, actor decision points, scheduled action progress, stable ordering, interruptions, reservations, no-human simulation stepping, time acceleration, and deterministic randomness integration.

It does not privilege human-controlled actors.

### Epistemics

Owns observation data, perception filtering, belief update rules, memory models, trace discovery, rumor propagation, contradiction links, stale information, provenance, actor-knowledge filtering, and expectation contradiction.

It must never let embodied UI, dialogue, or institutions read ground truth directly.

### Agent cognition

Owns needs, pressures, values, roles, relationships, motives, goals, projects, durable intentions, HTN method selection, bounded local planning, action proposals, replanning triggers, failure handling, and planner trace/debug output.

It does not own event commit or authoritative mutation.

### Institutions

Owns roles, obligations, permissions, prohibitions, constitutive norms, procedures, evidence thresholds, records, reports, notices, institutional facts, jurisdiction, bias/corruption processes, and procedure failure.

Institutions act from institutional knowledge and records, not ground truth.

### TUI client

Owns terminal presentation, input collection, menus, actor-known notebook rendering, debug toggles, log panes, why-not displays, and testable TUI view consumption.

It submits commands. It does not validate world rules except for local display sanity.

### Debug/replay tooling

Owns non-diegetic inspection of event logs, causal graphs, projection diffs, actor beliefs, hidden truth, planner traces, scheduler queues, action rejections, event upcasters, replay failures, and knowledge mismatch diagnostics.

Debug tools may expose truth, but only through visibly non-diegetic view models.

### Language surface

Owns deterministic templates in v1, optional future LLM prompt packets, parsing proposals, generated prose, validation reports, repair/rejection paths, deterministic mocks, and LLM-disabled operation.

It cannot commit speech acts or facts directly.

### Future graphical client boundary

Owns graphical rendering, animations, camera, spatial presentation, and richer affordance display. It consumes the same view models as the TUI or explicitly versioned graphical view models derived from the same projections.

It must not fork the simulation rule system.

## Candidate crates and libraries

Candidate crates may be considered where useful, but none are doctrine:

- serialization and validation candidates: `serde`, `schemars`, `jsonschema`, `ron`, `toml`, `serde_yaml`;
- deterministic testing candidates: `proptest`, `insta`, snapshot tests, seeded RNG crates;
- TUI candidates: `ratatui`, `crossterm`;
- graph/debug candidates: graph libraries or export formats such as DOT/GraphML;
- persistence candidates: append-only files, SQLite, embedded key-value store, or later event-store-specific storage.

Do not choose a library because it is fashionable. Choose only when it preserves determinism, replay, testing, source provenance, and maintainability.

## Storage boundary

The architecture requires append-only event semantics, replayability, snapshots, projection rebuild, and versioned events. It does not require a final database engine now.

Acceptable first storage shape:

```text
append-only event segment files
+ content/data version manifest
+ snapshot files
+ projection cache files
+ replay checksum manifest
```

Possible later evolution:

```text
embedded database or event-store adapter
+ indexed streams
+ snapshot store
+ projection store
+ migration/upcaster registry
```

The storage adapter must not become the domain model. Database rows, tables, documents, indexes, or object stores are persistence strategies. World truth is defined by validated events plus content/data versions and rebuildable projections.

## ECS and game-framework stance

ECS-like storage can be useful for entity/component queries. It is not a substitute for Tracewake architecture.

The following outrank ECS concerns:

- event sourcing;
- action parity for human and AI actors;
- actor-knowledge filtering;
- deterministic replay;
- causal provenance;
- projection rebuild;
- no-human simulation;
- TUI playability;
- validation of authored content;
- debug inspection.

A game framework may be used behind boundaries only if it does not invert authority. If a framework encourages frame-driven mutation, hidden systems, graphical-first thinking, nondeterministic iteration, or UI-rule entanglement, it must be constrained or rejected.

## Side-effect policy

Authoritative simulation code must isolate side effects:

- no wall-clock decisions inside rules;
- no live network calls during authoritative simulation;
- no filesystem reads during outcome resolution except through loaded/versioned content handles;
- no live LLM call as simulation authority;
- no unordered map iteration affecting outcomes unless order is stabilized;
- no thread scheduling race that changes event order;
- no hidden global random source.

Side effects belong at boundaries: loading, saving, logging, debug export, TUI input, optional language services, and tooling.

## Boundary review checklist

A new package/module boundary is acceptable only if it answers:

- What authority does it own?
- What authority is explicitly denied?
- What data crosses the boundary?
- Is the boundary deterministic under replay?
- Can it run with no human attached?
- Can it run with LLMs disabled?
- Does it preserve actor-knowledge filtering?
- Does it submit through the action/event pipeline?
- Can debug inspect its decisions?
- Can the TUI exercise it without graphical dependencies?

## Forbidden architecture outcomes

- A `Player` package with privileged verbs.
- A `Quest` package that drives objective truth or world waiting.
- A `DialogueAI` package that selects actions for agents.
- A storage schema that is treated as current truth without replay.
- A graphical client that implements its own interaction rules.
- A domain pack that writes events directly.
- A debug notebook that silently becomes actor knowledge.
- A scheduler that treats the possessed actor differently from ordinary agents.
