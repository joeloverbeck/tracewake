# TUI-First Playability Contract

## Core claim

The TUI is the first real Tracewake client.

It is not a disposable debug console, not a placeholder for graphics, and not a shell waiting for a later game. For the foreseeable future, the TUI is where the product becomes playable, legible, testable, and honest.

Every runnable phase must preserve TUI reachability, actor-knowledge filtering, why-not explanations, debug inspection, automated view-model or terminal acceptance tests, and no-human simulation gates.

## Product surface

The TUI must support real embodied play:

- moving through places;
- inspecting actor-perceived surroundings;
- seeing the current actor's body state, needs, possessions, beliefs, intentions, obligations, and relationships;
- using object affordances;
- opening, inspecting, taking from, and placing into containers when actor-possible;
- eating, sleeping, waiting, working, traveling, searching, reporting, buying, stealing, reading, and speaking;
- talking through structured speech acts;
- reading notices and records accessible to the current actor;
- maintaining actor-known leads;
- receiving actor-filtered missed-event summaries;
- switching possession in debug mode;
- seeing why actions are unavailable;
- inspecting causal chains in debug mode.

A mechanic that cannot be reached, used, inspected, and tested through the TUI is not complete.

## Kernel and TUI advance together

The implementation order must not become:

```text
headless simulation for months -> TUI later
```

The required order is:

```text
minimal kernel + minimal TUI harness
-> new mechanic
-> actor-filtered view model
-> TUI reachability
-> why-not explanations
-> debug inspection
-> automated acceptance test
```

Pure headless tests are necessary. They are not sufficient after the kernel is runnable.

## Embodied mode

Embodied mode is normal play.

It shows:

- current actor identity and body state;
- perceived place, exits, nearby actors, objects, traces, affordances, and risks;
- actor-known beliefs, memories, leads, obligations, records, and suspicions;
- actor-available actions;
- actor-accessible conversation options;
- actor-relevant event and conversation summaries;
- uncertainty and provenance labels where the actor has them.

It hides:

- ground truth not perceived, remembered, inferred, read, or told to the actor;
- hidden containers, hidden objects, and hidden actors;
- other minds' beliefs unless communicated or inferred;
- true culprit labels;
- objective quest truth;
- debug causal graph;
- omniscient institution state;
- the human's possession history as a world fact.

Normal play must not collapse into an omniscient simulation viewer.

## Debug mode

Debug mode is mandatory and visibly non-diegetic.

It may show:

- authoritative event log;
- causal graph;
- component state;
- action proposals and rejected preconditions;
- belief stores for all agents;
- hidden traces;
- institutional records and private procedures;
- planner decisions;
- random draw records;
- LOD tier and promotion/demotion events;
- replay/snapshot state;
- speech-act validation state;
- actor/human knowledge mismatch diagnostics;
- possession binding history.

Debug mode may reveal truth. It may not be confused with embodied truth. It may not provide world-affecting shortcuts except explicitly marked development tools outside normal acceptance paths.

## View-model boundary

The TUI renderer consumes view models.

Correct:

```text
Simulation core
-> projections
-> actor-knowledge filters
-> view models
-> TUI renderer
-> validated action proposal
-> action pipeline
-> events
```

Forbidden:

```text
TUI queries ground truth directly in embodied mode
TUI implements action rules
TUI mutates world state through custom player-only paths
TUI creates objective quest progress
TUI map reveals hidden truth
TUI notebook grants actor knowledge
TUI dialogue prose creates facts
```

The TUI may be friendly. It may not be metaphysically privileged.

## Action menus and why-not explanations

Action menus may help the human understand what the current actor can attempt.

The menu may:

- list available actor-possible actions;
- group actions by object, person, place, intention, or lead;
- show risk and uncertainty known to the actor;
- explain missing preconditions;
- offer planning suggestions bounded by actor knowledge;
- show debug-only causes in debug overlay.

The menu must not:

- reveal hidden truth in embodied mode;
- invent hidden targets;
- bypass required travel, access, custody, proof, speech, or institutional procedure;
- give the human a verb that an equivalent NPC could not attempt;
- make a failed or blocked action silently succeed for convenience.

Why-not explanations must be mode-aware.

Embodied example:

```text
You cannot accuse Mara with evidence: Tomas has no known observation, testimony, record, or trace linking Mara to the coins.
```

Debug overlay may add:

```text
Ground truth: Mara took the coins at 02:13. Tomas has not received any modeled information connecting her to the event.
```

## Notebooks and leads

Tracewake needs three separated surfaces:

```text
Actor notebook / actor-known leads
Human/debug notes
Debug inspector
```

The actor notebook is available in embodied play. It contains only source-bound information available to the current actor.

The human/debug notes surface is optional out-of-world convenience. It may help the human remember. It must be visibly non-diegetic and must never serve as a source for speech acts, reports, accusations, persuasion, testimony, institutional claims, or action preconditions.

The debug inspector may reveal truth, causal graphs, all beliefs, and knowledge mismatches. It is non-diegetic.

A lead card may suggest actions, but suggestions must be actor-possible and source-bound.

Good lead:

```text
Known by Tomas
Source: reported by Elena at breakfast
Claim: Elena heard a noise near the pantry last night
Confidence: uncertain
Possible actor actions: ask Elena for details, search pantry, report missing coins
```

Bad lead:

```text
Objective: confront Mara, the thief
Marker: Mara's hiding place
Progress: 2/3 clues found
```

## Conversation UI

Initial conversation must be structured speech acts with templated surface text.

The TUI should expose, at least in developer or debug overlay:

- speaker;
- listener;
- surface utterance;
- speech-act type;
- claims carried;
- sincerity/lie suspicion only when available to the observing actor or in debug;
- source and channel;
- recipient belief effects;
- refusal/failure state;
- institutional consequences if any.

No freeform parser is required in the foundation. No unvalidated open natural-language input may mutate authoritative state.

Embodied mode must not label a statement as a lie unless the current actor has modeled reason.

## Time controls

Waiting is not skipping causality. Waiting runs the simulation.

The TUI must support staged versions of:

- pause;
- step;
- wait;
- sleep as an actor action;
- continue current intention;
- time acceleration;
- travel time;
- stop on salient perceived interruption;
- actor-filtered missed-event summaries;
- debug event streaming.

A sleeping actor does not receive omniscient summaries. A worker returning home learns what reached them through modeled channels or what they observe on return.

## Debug possession

The TUI must support debug possession as an architectural proof of actor parity.

The normal binding is:

```text
HumanController -> ActorId
```

Possession changes input binding only. It does not move beliefs, notebooks, intentions, obligations, money, suspicion, wounds, reputation, or knowledge between bodies.

Debug possession should generally allow any high-detail ordinary actor. Abstract regional processes, institutions-as-systems, low-LOD aggregates, storms, offices, records, and non-agent processes are not directly possessable unless represented by an ordinary actor/controller surface or promoted into appropriate detail.

## Future graphical client

A later graphical app may add:

- presentation;
- layout;
- animation;
- sound;
- richer input;
- better maps;
- visual inspection;
- accessibility surfaces.

It must reuse:

```text
simulation core
action pipeline
actor-knowledge filters
view models
notebooks
speech-act interface
debug/replay architecture
acceptance doctrine
```

It must not become a second simulation engine or introduce mechanics unavailable through the TUI and ordinary-agent action pipeline.

## Minimal permanent TUI surfaces

Even early versions should preserve these surfaces as first-class:

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
- replay/view-model test harness.

The design may simplify these surfaces. It may not make them afterthoughts.

## Automated TUI acceptance

Every runnable phase needs automated tests that drive either terminal interactions or the same view models consumed by the terminal.

Acceptance tests must prove that:

- a human can reach the mechanic through embodied play;
- embodied views are actor-filtered;
- debug views can explain causal truth;
- blocked actions explain why;
- committed actions pass through the ordinary action pipeline;
- no hidden player-only mutation exists;
- no-human simulations still run.

## Review rule

For any feature, ask:

```text
Can an ordinary actor use or experience it?
Can the human use it only through the current actor?
Can the TUI show what the actor knows?
Can the TUI hide what the actor does not know?
Can why-not explain blocked attempts without leaking truth?
Can debug mode explain what really happened?
Can an automated view-model or terminal test prove it?
Can the feature run with no human present?
```

If not, the feature is not done.
