# Strict Phase Ladder and Implementation Order

## Core rule

Tracewake execution is a strict ladder. Later phases do not begin because they are more exciting. Later phases begin only when the prior phase exits by gate, replay, TUI/view-model acceptance, test evidence, and data validation.

```text
Phase 0 -> Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Second Proof
```

A later feature cannot be used as camouflage for an earlier failure.

Bad:

```text
belief leakage is still unresolved, but we can start notice-board travel
ordinary routines are shallow, but we can start companion recruitment
the TUI cannot inspect containers, but we can build a regional map
institution records read ground truth, but we can add proof/payment
```

Good:

```text
each phase proves one irreversible slice of the execution contract
each phase remains playable or view-model-testable
each phase runs without a human controller where runnable
each phase replays deterministically
```

## Dependency map

```text
Phase 0: paper ontology
  proves the missing-property chain can be manually traced without scripts
  unlocks Phase 1

Phase 1: physical/event/TUI spine
  proves actions, places, containers, item movement, event log, replay, and debug attach
  unlocks Phase 2

Phase 2: epistemics/view models/possession parity
  proves belief provenance, actor-known filtering, absence-as-evidence, and no knowledge transfer
  unlocks Phase 3

Phase 3: needs/routines/no-human day
  proves ordinary life is real enough to support disruption
  unlocks Phase 4

Phase 4: institutions/records/wrong suspicion
  proves reports, records, fallible local authority, and wrong suspicion
  completes the first proof
  unlocks the second proof
```

## How to choose the next implementation spec

Choose the smallest next implementation spec that advances the current phase exit criteria.

Use this order:

1. unblock the phase's TUI/view-model gate;
2. unblock deterministic replay;
3. unblock no-human simulation;
4. unblock golden scenario fixture execution;
5. unblock debug explanation;
6. harden failure cases;
7. only then add breadth.

Do not choose a feature because it is narratively attractive. Choose it because it closes a phase gate.

## Universal phase shape

Every phase document and implementation spec must include:

- purpose;
- entry requirements;
- deliverables;
- explicit non-goals;
- TUI/view-model gate;
- no-human simulation gate;
- deterministic replay gate;
- test gate;
- data/fixture gate;
- debug/inspection gate;
- phase exit criteria;
- forbidden shortcuts;
- modeled failure cases;
- what must not be started until this phase passes.

No runnable phase may omit TUI/view-model acceptance. No runnable phase may omit no-human simulation. No phase may accept hidden player privilege.

## Phase 0 — Paper Ontology and Fixture Design

### Purpose

Make the first proof mechanically traceable before building a kernel.

Phase 0 proves the team can describe the missing-property village as action, event, trace, observation, belief, report, record, suspicion, and debug explanation without quest state, hidden scripts, player identity, or LLM authority.

### Entry requirements

- Foundation and architecture docs are stable enough to govern execution.
- The repository manifest and exact source set have been identified.
- The first proof is accepted as The Missing Property Village.
- Road-threat/travel/companion flow is explicitly deferred.

### Deliverables

- primitive action vocabulary;
- primitive event vocabulary;
- primitive proposition vocabulary;
- primitive trace vocabulary;
- primitive speech-act vocabulary;
- first-proof actor roster;
- village map and object inventory;
- strongbox/missing-property chain;
- ordinary workday chain;
- report/record chain;
- wrong suspicion chain;
- possession parity chain;
- no-scripting review;
- TUI screen sketches;
- view-model contracts;
- golden scenario sketches.

### Non-goals

- executable kernel;
- crate layout finalization;
- final data file syntax;
- real planner;
- LLM dialogue;
- road threat;
- notice lifecycle as first proof;
- companion recruitment.

### Gates

TUI/view-model gate: screen sketches and view-model contracts show how embodied and debug modes expose the proof without leaking truth.

No-human gate: paper trace includes a no-human chain and proves no step requires a controller.

Replay gate: every meaningful state change in the paper chain has an event type and causal link.

Test gate: golden scenario outlines define both occurrence and non-occurrence assertions.

Data/fixture gate: fixtures specify initial beliefs, expectations, roles, places, containers, and pressures with provenance.

Debug gate: paper debug answers "what happened," "why possible," "who knows," "who is wrong," and "what later events became possible."

### Modeled failure cases

- locked strongbox cannot be opened by Tomas without key;
- Mara refuses theft if risk or norm cost is too high;
- Elena hears nothing because asleep, distant, or door muffles sound;
- Tomas checks the wrong container and finds nothing;
- Tomas lacks knowledge to accuse Mara;
- office closed;
- Anna absent;
- report refused or delayed;
- wrong suspicion threshold not met;
- replay chain missing a cause.

### Exit criteria

Phase 0 exits when the missing-property proof can be manually traced from causes through action, event, trace, observation, belief, report, record, suspicion, and debug explanation without requiring a quest object, LLM authority, player identity, or hidden script.

### Must not start until Phase 0 passes

- Rust kernel implementation;
- TUI widget work beyond sketches;
- real content loading;
- planner implementation;
- notices/travel/companion systems.

## Phase 1 — Runnable Kernel, TUI Harness, and Event Log

### Purpose

Build the physical/event/action spine before epistemics.

Phase 1 proves a small TUI-playable world can commit and replay ordinary physical state changes using a shared action pipeline. It does not yet prove full belief.

### Entry requirements

- Phase 0 exit accepted.
- Primitive action/event/component vocabulary approved.
- Golden fixture IDs reserved.
- Minimal TUI screen/view-model contracts approved.

### Deliverables

- entity IDs;
- minimal component storage;
- deterministic scheduler;
- command/proposal/action validation framework;
- primitive action registry;
- event envelope and append-only log;
- event ordering and stream positions;
- rooms/places graph;
- doors;
- containers;
- items;
- ownership/possession/custody/expected-location fields, even if expected-location is not fully epistemic yet;
- simple item movement;
- minimal embodied TUI local view;
- action menu;
- why-not basics;
- debug event log;
- debug possession attach command as controller metadata;
- deterministic replay of simple item movement;
- no-human simulation advance with no player references.

### Non-goals

- full belief store;
- observation confidence;
- rumor;
- needs/routines;
- report records;
- institution logic;
- full possession parity proof;
- notice board mechanics;
- travel.

### Gates

TUI/view-model gate: a user can attach to an actor, move between rooms if allowed, inspect actor/place, open/close a door, inspect/open a container, take/place an item if allowed, and see committed events in debug mode.

No-human gate: the scheduler advances with no human controller and can execute simple scheduled or scripted-test neutral proposals without player references.

Replay gate: replay reconstructs simple item location, door state, container state, and event ordering.

Test gate: unit and integration tests cover action rejection, event commit, item movement, container access, door state, no player-only action registry, and view-model reachability.

Data/fixture gate: a minimal village fixture loads with stable IDs and validates entity references.

Debug gate: debug inspector can answer why an item is where it is and which event moved it.

### Modeled failure cases

- actor cannot reach door;
- door locked;
- container inaccessible;
- item absent;
- item too heavy;
- action rejected before mutation;
- reservation conflict;
- no valid route between adjacent places;
- replay mismatch after item movement;
- TUI attempts direct mutation and is rejected by harness.

### Exit criteria

Phase 1 exits when replay can explain why an item is where it is and the TUI can play ordinary movement, door, container, and item operations through the action pipeline.

### Must not start until Phase 1 passes

- full epistemics;
- actor-known notebooks;
- needs/routines;
- local authority;
- suspicion;
- road travel;
- LLM surfaces.

## Phase 2 — Epistemics, View Models, and Possession Parity

### Purpose

Add observation, belief, expectation contradiction, actor-known filtering, debug truth, and possession parity.

Phase 2 proves belief-before-truth. It proves a human can know something while the current actor cannot.

### Entry requirements

- Phase 1 exit accepted.
- Event log and replay stable for physical item movement.
- Minimal TUI and debug attach command exist.
- View-model boundary established.

### Deliverables

- observation events;
- direct perception;
- simple sound observation;
- absence/missing-item observation;
- belief store with holder, proposition, confidence, source, acquisition time, and channel;
- expectation contradiction;
- actor-knowledge filtering;
- embodied view model vs debug view model;
- possession metadata as controller/debug state only;
- actor-known notebook;
- human/debug notes separate from actor knowledge;
- why-not explanations for physical, knowledge, and social blockers;
- possession parity tests;
- no knowledge transfer on body switch.

### Non-goals

- full daily routine system;
- detailed memory distortion;
- institution records;
- broad rumor network;
- freeform speech;
- route-threat notice flow.

### Gates

TUI/view-model gate: embodied mode shows only actor-known/perceived facts. Debug mode shows truth, event log, causal links, hidden beliefs, and possession history as non-diegetic metadata.

No-human gate: the same perception/belief pipeline can run for NPCs with no human bound.

Replay gate: replay reconstructs observations, belief updates, expectation contradiction, embodied view snapshots, and debug truth.

Test gate: tests cover observation vs interpretation, belief provenance, absence-as-evidence, view filtering, possession switch, actor knowledge separation, and why-not knowledge blockers.

Data/fixture gate: initial beliefs and expectations require source-backed fixture entries.

Debug gate: debug inspector can compare truth, belief, source, and possession history.

### Modeled failure cases

- actor cannot perceive item;
- actor sees empty container but has no expectation, so no missing-property contradiction;
- actor expects wrong container;
- sound observation produces low-confidence uncertain belief;
- actor lacks knowledge to report culprit;
- actor tries to accuse from human memory and is blocked or classified as reckless accusation;
- embodied view attempts to leak debug truth;
- possession switch attempts to transfer actor notebook and fails.

### Exit criteria

Phase 2 exits when a player can steal or move property as Mara, switch to Tomas, and Tomas cannot accuse from player memory. Tomas discovers missing property only by valid perception, expectation contradiction, report, inference, or other modeled information channel.

### Must not start until Phase 2 passes

- daily no-human proof as acceptance target;
- institution report intake;
- wrong suspicion as authority behavior;
- notice/travel/companion systems.

## Phase 3 — Needs, Routines, Work, and No-Human Daily Simulation

### Purpose

Make ordinary life mechanically real.

Phase 3 proves agents sleep, eat, work, wait, interrupt routines, and continue life without a human. The missing-property event must occur inside or alongside ordinary life, not as a script pasted onto a static village.

### Entry requirements

- Phase 2 exit accepted.
- Belief, view filtering, possession parity, and replay stable.
- First village fixture has homes, rooms, containers, work sites, and actor roster.

### Deliverables

- hunger;
- fatigue;
- safety;
- sleep;
- eating;
- work;
- homes;
- workplace;
- tavern/social routine if cheap and bounded;
- office hours as routine substrate if needed;
- intention persistence;
- simple HTN-like routines;
- interruptions;
- wait/sleep/continue-routine TUI controls;
- planner trace/debug output;
- no-human day simulation.

### Non-goals

- full economy;
- full market pricing;
- region scale;
- detailed combat/injury;
- companion recruitment;
- formal case procedure beyond substrate;
- live LLM language.

### Gates

TUI/view-model gate: a user can embody any major actor, sleep, eat, work, wait through time, continue routine, and inspect why a routine continued, failed, or was interrupted.

No-human gate: the village runs at least one full simulated day without human input. Agents sleep, eat, work, rest, speak minimally, store/use objects, and interrupt routines for modeled reasons. No event references player identity.

Replay gate: replay reconstructs routines, scheduled actions, interruptions, needs changes, and planner traces for salient decisions.

Test gate: unit, property, golden, and no-human tests cover needs thresholds, sleep, eating, work, routine selection, routine failure, interruptions, and no protagonist gravity.

Data/fixture gate: routines have preconditions, failure modes, interruptions, and source-backed actor expectations. A routine without failure modes is invalid.

Debug gate: planner inspector explains candidate goals, selected intention, selected method, rejected actions, and hidden truth mismatch.

### Modeled failure cases

- actor too hungry to work well;
- actor too tired to perceive faint trace;
- food missing;
- bed occupied;
- actor unwilling to trespass for food;
- workplace locked;
- missing tool interrupts work;
- office closed;
- clerk asleep;
- actor waits but stops on actor-perceived interruption;
- planner budget exhausted;
- stuck actor produces diagnostic event.

### Exit criteria

Phase 3 exits when the village runs at least a day without human input and ordinary life is deep enough that missing property has meaningful effects on sleep, work, movement, speech, privacy, expectation, and institutional timing.

### Must not start until Phase 3 passes

- final report/record proof;
- institutional wrong suspicion;
- notice lifecycle as product feature;
- road-threat/travel flow.

## Phase 4 — Institutions, Records, and Wrong Suspicion

### Purpose

Complete the first proof by adding household/public institutions, report intake, records, local authority procedure, and wrong suspicion.

Phase 4 proves institutions are fallible social machines. They act from claims, records, roles, resources, thresholds, and bias—not ground truth.

### Entry requirements

- Phase 3 exit accepted.
- Ordinary life and no-human daily simulation run.
- Speech-act foundation is available for report/tell/ask/answer/refuse/lie/gossip/suspect.
- Missing-property setup can occur inside routine life.

### Deliverables

- household as minimal institution;
- local authority;
- reeve/clerk/guard roles;
- office hours;
- report action;
- incident ledger;
- record artifact;
- simple questioning/watch procedure;
- property/theft norm;
- trespass/privacy norm;
- suspicion scoring from actor/institutional knowledge;
- wrong suspicion scenario;
- institution debug inspector;
- first proof final acceptance.

### Non-goals

- court system;
- bounty payment;
- companion hiring;
- expedition resolution;
- regional law;
- detailed incarceration;
- route threat;
- public notice lifecycle as central proof;
- full reputation economy.

### Gates

TUI/view-model gate: a player can report missing property to the clerk if the current actor knows enough to report. Accessible records are readable as actor-known artifacts. Debug mode shows record provenance, institutional belief, procedure state, and why suspicion attached to the wrong person.

No-human gate: reports, records, questions, delays, refusals, and wrong suspicion can arise without human input when modeled conditions warrant them.

Replay gate: replay rebuilds records, institutional beliefs, suspicion scores, questioning decisions, actor notebooks, and debug institution state.

Test gate: tests cover report receiver, office hours, report refusal/delay, ledger creation, record provenance, institutional non-omniscience, wrong suspicion, actor/institution thresholds, and no player privilege.

Data/fixture gate: local authority, roles, norms, records, procedures, reports, and suspicion rules validate before run. No record may lack author/provenance/artifact.

Debug gate: institution inspector can answer what the office knows, from whom, with what confidence, in which record, under which procedure, and why it may be wrong.

### Modeled failure cases

- office closed;
- clerk absent;
- reporter lacks enough knowledge to report;
- report refused as too vague;
- report delayed because staff unavailable;
- clerk misrecords uncertainty;
- guard lacks evidence threshold;
- suspicion threshold not met;
- suspicion threshold met for the wrong actor;
- record contradicted;
- institution tries to read ground truth and test fails.

### Exit criteria

Phase 4 exits when the first proof is complete: missing property can be stolen or moved, discovered through expectation contradiction, reported, recorded, partially propagated, misinterpreted, and investigated badly, with debug explanation and deterministic replay.

### Must not start until Phase 4 passes

- road-threat bounty loop;
- stale-lead travel;
- companion recruitment;
- proof/payment;
- regional expansion;
- graphical client;
- live LLM dialogue.

## Phase completion discipline

A phase is not complete because a manual demo worked. A phase is complete only when:

```text
the mechanic runs
+ TUI/view model reaches it
+ no-human variant runs
+ replay rebuilds it
+ tests cover success and failure
+ data validates it
+ debug explains it
+ no script/player/LLM leakage exists
```

Anything less is a prototype note.

## One-line implementation order

```text
paper trace the missing-property village
 -> build physical action/event/TUI spine
 -> add belief/view/possession parity
 -> add needs/routines/no-human day
 -> add local authority/report/record/wrong suspicion
 -> only then build notice/travel/regional proof
```
