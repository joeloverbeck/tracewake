# TUI, Possession, View Models, and Debug

## Core claim

The TUI is the first real Tracewake client.

It is not a disposable debug console, not a placeholder for graphics, and not a shell waiting for a later game. For the foreseeable future, the TUI is where Tracewake becomes playable, legible, testable, and honest.

Every runnable phase must preserve TUI reachability, actor-knowledge filtering, why-not explanations, debug inspection, automated view-model or terminal tests, deterministic replay, and no-human simulation gates.

## Product surface

The TUI must support real embodied play:

- moving through places;
- inspecting actor-perceived surroundings;
- seeing current actor body state, needs, possessions, beliefs, intentions, obligations, relationships, and known records;
- using object/place/person/record affordances;
- opening, inspecting, taking from, and placing into containers when actor-possible;
- eating, sleeping, waiting, working, traveling, searching, reporting, buying, stealing, reading, hiding, storing, and speaking;
- talking through structured speech acts;
- reading notices and records accessible to the current actor;
- maintaining actor-known leads;
- receiving actor-filtered missed-event summaries;
- switching possession in debug mode;
- seeing why actions are unavailable;
- inspecting causal chains, beliefs, records, traces, planners, random draws, and replay state in debug mode.

A mechanic that cannot be reached, used, inspected, and tested through the TUI or the same actor-filtered view models is not complete.

## Kernel and TUI advance together

The implementation order must not become:

```text
headless simulation for months -> TUI later
```

The required pattern is:

```text
minimal kernel + minimal TUI harness
-> new mechanic
-> actor-filtered view model
-> TUI reachability
-> why-not explanation
-> debug inspection
-> automated acceptance test
```

Headless tests are necessary. They are not sufficient after the kernel is runnable.

## Embodied mode

Embodied mode is normal play.

It shows:

- current actor identity and body state;
- perceived place, exits, nearby actors, objects, traces, affordances, and risks;
- actor-known beliefs, memories, claims, leads, obligations, records, suspicions, debts, promises, and relationships;
- actor-available actions;
- actor-accessible conversation options;
- actor-relevant event and conversation summaries;
- uncertainty, staleness, and provenance labels where actor-known.

It hides:

- ground truth not perceived, remembered, inferred, read, told, or otherwise acquired by the actor;
- hidden containers, hidden objects, hidden actors, and hidden routes;
- other minds' beliefs unless communicated or inferred;
- true culprit labels;
- objective quest truth;
- debug causal graphs;
- omniscient institution state;
- human possession history as world fact.

Normal play must not collapse into an omniscient simulation viewer.

## Debug mode

Debug mode is mandatory and visibly non-diegetic.

It may show:

- authoritative event log;
- causal graph;
- component state;
- action proposals and rejected preconditions;
- belief stores for all agents/households/institutions;
- claim provenance and contradiction links;
- hidden traces;
- institutional records and private procedures;
- planner decisions;
- random draw records;
- LOD tier and promotion/demotion events;
- replay/snapshot/projection state;
- speech-act validation state;
- actor/human knowledge mismatch diagnostics;
- possession binding history.

Debug mode may reveal truth. It may not be confused with embodied truth. It may not provide normal-play world-affecting shortcuts.

## View-model boundary

Correct flow:

```text
Simulation core
-> projections
-> actor-knowledge filters
-> view models
-> TUI renderer
-> typed action proposal
-> action validator
-> event pipeline
```

Forbidden:

```text
TUI queries hidden truth in embodied mode
TUI implements action rules
TUI mutates state through custom player-only paths
TUI creates quest progress
TUI notebook grants actor knowledge
TUI map reveals hidden evidence
TUI dialogue prose creates facts
```

The TUI may be friendly. It may not be metaphysically privileged.

## Possession

Normal controller binding is:

```text
HumanController -> ActorId
```

Possession changes input binding only. It does not move beliefs, notebooks, intentions, obligations, memories, money, suspicion, wounds, reputation, relationships, or knowledge between bodies.

Debug possession is an architectural proof of actor parity. It should generally allow high-detail ordinary actors. Abstract regional processes, offices-as-systems, institutions-as-systems, storms, markets, diseases, low-LOD aggregates, and records are not directly possessable unless represented by an ordinary actor/controller surface or promoted into appropriate detail.

Possession may later become diegetic in a domain pack, but the kernel must not depend on possession metaphysics.

## Actor knowledge versus human knowledge

The human may know more than the body.

The body may act only from actor-known claims, perceptions, records, memories, inferences, intentions, and social context.

A player who controlled Mara during theft may switch to Tomas. Tomas may search because he expected coins; he may speculate, suspect, or lie if those speech acts are valid. Tomas may not truthfully report Mara as culprit unless modeled channels gave him support.

The TUI must make this limitation legible without turning it into omniscient help.

## Action menus

Action menus may help the human understand possible actions.

Allowed:

- list actor-possible actions;
- group actions by object, person, place, intention, lead, or record;
- show actor-known risk and uncertainty;
- show why-not explanations;
- offer planning suggestions bounded by actor knowledge;
- show debug-only cause overlays in debug mode.

Forbidden:

- hidden truth actions in embodied mode;
- exact target actions based on human/debug knowledge;
- bypassing travel, access, custody, proof, speech, or institutional procedure;
- player-only verbs;
- silently succeeding blocked actions for convenience;
- “complete objective” as world mutation.

## Why-not explanations

Why-not explanations must be mode-aware.

Embodied:

```text
You cannot make a formal theft report naming Mara: Tomas knows the coins are missing, but he has no actor-known observation, testimony, record, or trace linking Mara to them.
```

Debug overlay:

```text
Ground truth: Mara took the coins. Tomas has not received modeled information connecting her to the event. Human learned this during possession of Mara.
```

Do not leak hidden truth in embodied why-not text.

## Notebooks, leads, and notes

Tracewake needs three separated surfaces:

```text
Actor notebook / actor-known leads
Human/debug notes
Debug inspector
```

The actor notebook is diegetically available only as actor knowledge or actor-carried/remembered records. It contains source-bound claims available to the current actor.

Human/debug notes are optional out-of-world convenience. They may help the human remember but must be visibly non-diegetic and cannot satisfy speech, report, accusation, proof, persuasion, institutional recognition, or action preconditions.

The debug inspector may reveal truth, all beliefs, event graphs, hidden traces, and knowledge mismatch diagnostics.

Lead cards may suggest actions, but suggestions must be source-bound and actor-possible.

## Lead wording

Bad embodied lead:

```text
Objective: Catch Mara, the thief.
Progress: 2/3 clues.
Reward: 10 coins.
Marker: Mara's hiding place.
```

Good embodied lead:

```text
Lead: Missing coins from strongbox
Source: Your search of the strongbox
Known claims: You expected the coins there. They were absent when checked. Elena says she heard a noise near the room.
Uncertainty: No direct witness. Could be theft, borrowing, movement, misremembering, or poor search.
Possible actions: search nearby, ask household, report to clerk, inspect for traces.
```

## Conversation UI

Initial conversation uses structured speech acts with deterministic surface text.

The TUI should expose, at least in developer/debug contexts:

- speaker and listener;
- surface utterance;
- speech-act type;
- typed claims carried;
- source belief references;
- sincerity/lie status only when available to observing actor or in debug;
- recipient interpretation;
- belief/memory effects;
- refusal/failure status;
- institutional consequences if any.

No freeform parser may directly mutate state. Future freeform input can only propose candidate speech acts behind validation.

Embodied mode must not label a statement as a lie unless the current actor has modeled reason.

## Time controls

Waiting is not skipping causality. Waiting runs the simulation.

The TUI should support staged versions of:

- pause;
- step;
- wait;
- sleep as actor action;
- continue current intention;
- time acceleration;
- travel time;
- stop on salient perceived interruption;
- actor-filtered missed-event summaries;
- debug event streaming.

A sleeping actor does not receive omniscient summaries. They learn what reached them through modeled channels or what they perceive later.

Time controls follow temporal authority: they may advance authoritative event/replay time, but embodied views may render temporal facts only when the possessed actor could know or infer them through modeled channels. Debug may show exact simulation time, event order, hidden due effects, or omitted truth, but must be visibly non-diegetic. Summaries such as slept until morning, office closed, payment late, or many events happened while away must be actor-known summaries, record-derived conclusions, or debug-only labels, not hidden-truth leakage.

## Debug inspection surfaces

Minimum permanent debug surfaces should include:

- event log;
- causal graph inspector;
- current state/projection inspector;
- actor belief/memory inspector;
- claim provenance viewer;
- planner/intention inspector;
- action validation report;
- record/institution inspector;
- trace inspector;
- random draw inspector;
- LOD ancestry inspector;
- replay/projection checksum state;
- possession binding history;
- actor/human knowledge mismatch viewer.

Debug tools must help prove the foundation. They are not optional polish.

## Future graphical client

A later graphical app may add richer presentation, layout, animation, sound, maps, visual inspection, accessibility, and alternate input.

It must reuse:

```text
simulation core
action pipeline
actor-knowledge filters
view models
notebooks/leads
speech-act interface
debug/replay architecture
acceptance doctrine
```

It must not become a second simulation engine or introduce mechanics unavailable through TUI/view-model acceptance paths.

## Automated acceptance

Every runnable phase needs automated tests that drive terminal interactions or the same view models consumed by the terminal.

Tests must prove:

- mechanic is reachable through embodied play;
- embodied views are actor-filtered;
- hidden truth is absent in embodied mode;
- why-not explanations work;
- committed actions pass through ordinary validation;
- debug mode can explain truth;
- no hidden player-only mutation exists;
- no-human simulation still runs.

## Review rule

For any feature, ask:

```text
Can an ordinary actor use or experience it?
Can the human use it only through the current actor?
Can the TUI show what the actor knows?
Can the TUI hide what the actor does not know?
Can why-not explain blocked attempts without hidden truth leakage?
Can debug explain what really happened?
Can automated view-model or terminal tests prove it?
Can it run with no human present?
```

If not, the feature is not done.

## 2026 hardening: possession parity and debug quarantine

Human possession is a viewpoint and input binding. It is never a cognition upgrade.

The possessed actor must remain an ordinary actor with the same:

- body limits;
- needs and fatigue;
- current intention and routine state;
- memories and beliefs;
- relationships and obligations;
- knowledge sources and ignorance;
- social risk;
- action registry;
- validation pipeline;
- ability to guess, speculate, search, ask, lie, refuse, and fail.

Possession must not transfer knowledge from the human's prior bodies, debug inspection, external notes, or player memory into actor-known state. If the human tries to act on out-of-character knowledge, the pipeline may allow only actions supportable as actor-known, reckless, speculative, search-based, or socially plausible attempts. It must not silently treat debug truth as actor knowledge.

## Embodied view models

Embodied view models must derive from actor-known context. They may include:

- current perception;
- remembered/known places and entities;
- actor beliefs and expectations;
- actor-readable records and notices;
- affordances the actor can perceive or believe possible;
- actor-filtered why-not;
- uncertainty and stale-risk indicators when actor-relevant.

They must exclude:

- hidden item locations;
- hidden culprits;
- hidden motives;
- debug truth;
- institution truth not known to the actor;
- other possessed actors' knowledge;
- LLM-invented facts;
- future quest/objective markers.

## Debug mode

Debug mode may reveal truth, hidden traces, causal graphs, all beliefs, planner state, scheduler state, hidden-truth audits, validation internals, projection diffs, and replay checksums. It must be visibly non-diegetic and structurally unable to feed ordinary cognition.

Debug surfaces must not be the only way to use or inspect a runnable feature. First-slice systems must be playable in embodied mode and diagnosable in debug mode.

## Why-not discipline

Why-not output has two layers:

- actor-visible reasons, such as "you do not see food here," "the office appears closed," "you do not know whom to accuse," or "you are too tired to continue";
- debug-only reasons, such as true food location, true office staff location, true culprit, hidden route blocker, or validator internals.

The actor-visible layer may inform future actor cognition only when modeled as perception, failed attempt, memory, or explicit feedback. The debug-only layer must remain quarantined.
