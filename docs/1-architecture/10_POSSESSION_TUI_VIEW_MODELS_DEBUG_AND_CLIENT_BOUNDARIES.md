# Possession, TUI View Models, Debug, and Client Boundaries

## Status

This document defines human controller binding, possession, actor-filtered view models, TUI play, actor notebooks, human/debug notes, why-not display, debug inspection, testable TUI harnesses, and future graphical client boundaries.

Tracewake has no sacred player character in the authoritative simulation.

## Core model

Allowed model:

```text
HumanController -> ActorId
```

Possession changes input binding only. The possessed actor remains an ordinary actor with body, beliefs, needs, intentions, memories, possessions, relationships, obligations, suspicions, and plans.

The simulation does not contain a privileged `Player` entity.

## Authority

This subsystem owns:

- controller binding metadata;
- embodied actor-filtered view models;
- TUI presentation and input;
- actor-known notebook display;
- why-not display;
- debug view models and commands;
- TUI transcript/replay harness;
- future graphical client boundary.

It is denied:

- world-rule validation except display sanity;
- direct state mutation;
- player-only verbs;
- ground-truth embodied display;
- knowledge merge across possessions;
- graphical-client rule forks;
- debug truth leakage.

## Possession state

Possession is controller/debug metadata, not world truth.

```yaml
ControllerBinding:
  controller: human_01
  bound_actor: actor_tomas
  mode: embodied
  started_at_tick: 81000
  previous_binding: actor_mara
  world_truth_effect: none
  event_stream: ControllerStream
```

Possession may be recorded for replay/debugging. It grants input authority only.

## Possession rules

- Binding to an actor does not erase intentions.
- Unbinding from an actor does not freeze that actor forever.
- The actor may resume autonomy according to scheduler/agent policy.
- The human can choose only actor-available actions in embodied mode.
- The current actor's knowledge is not merged with other possessed actors.
- Debug possession may switch freely but must mark non-diegetic context.
- No world mutation occurs solely because possession changed.

## Embodied view model

Embodied TUI consumes actor-filtered view models.

```yaml
EmbodiedViewModel:
  viewer: actor_tomas
  mode: embodied
  place_view:
    known_place: bedroom
    visible_entities:
      - strongbox_tomas
      - bed_tomas
    perceived_traces:
      - trace_strongbox_empty_if_searched
  body:
    hunger: actor_known_level
    fatigue: actor_known_level
    safety_feeling: uneasy
  beliefs:
    salient:
      - coin_stack_01_should_be_in_strongbox
      - strongbox_now_empty
  affordances:
    - search_strongbox
    - ask_elena_about_noise
    - report_missing_property
    - wait
  why_not_available_on_request: true
```

Embodied view models must not query hidden truth.

## View-model versioning and derivation

View models are projections. They declare:

- projection version;
- viewer/knowledge context;
- source event range or projection checkpoint;
- content/domain version;
- hidden-truth policy;
- stale-risk policy;
- affordance registry version.

A TUI or future client may format the same view differently. It may not change semantics.

## Actor-known notebook

The actor-known notebook is a projection of what the current actor knows, believes, remembers, heard, read, inferred, suspects, or wrote as an actor.

It includes:

- source-bound claims;
- confidence/uncertainty indicators;
- last verification;
- stale risk if actor has basis;
- records/notices read by actor;
- conversations heard by actor;
- unresolved contradictions;
- actor's own notes if note-taking is modeled;
- actor-known leads.

It excludes:

- hidden truth;
- debug annotations;
- knowledge from previously possessed actors;
- player memory outside the actor;
- unsupported LLM-generated facts.

## Human/debug notes

Human/debug notes are separate and non-diegetic.

```yaml
HumanDebugNote:
  controller: human_01
  text: "I saw in debug that Mara hid coins under her mattress."
  visibility: debug_only
  actor_knowledge_effect: none
```

A human may remember debug truth. The embodied UI must not convert that into actor knowledge. If the human commands an actor using out-of-character knowledge, the action pipeline still enforces actor knowledge preconditions or classifies the action as valid guess/speculation/search where appropriate.

## TUI action menus

TUI action menus are generated from actor-visible affordances and why-not-capable proposals.

Menu entries expose:

- stable semantic action ID;
- label;
- target;
- actor-known requirements;
- actor-known uncertainty;
- estimated duration/cost if actor can estimate;
- social risk if actor understands it;
- why-not details on rejection;
- debug expansion in debug mode.

Example:

```text
[1] Search strongbox
[2] Ask Elena what she heard
[3] Go to reeve's office
[4] Report missing coins to clerk
[5] Wait until morning office hours
[6] Accuse someone... (requires selecting basis or making reckless accusation)
```

The menu must not offer “recover coins from true hiding place” unless the actor learned that location.

## Why-not display

Why-not comes from action validation and is filtered by mode.

Embodied example:

```text
You cannot report this now: the office is closed and you do not see a clerk or guard here.
Possible alternatives: wait, go home, leave a written note if you have paper, ask a nearby person.
```

Debug expansion:

```text
Rejected at procedure check: no valid receiver with report-intake role in current place.
Scheduler: clerk asleep at home; guard on south patrol route.
```

## Debug view model

Debug view models are visibly non-diegetic.

They may show:

- ground truth state;
- event log;
- causal graph;
- hidden traces;
- all actor beliefs;
- belief/truth mismatches;
- planner state;
- scheduler queue;
- reservations;
- action validation reports;
- projection diffs;
- replay checksums;
- LOD transitions;
- content validation errors;
- LLM prompt/output validation if enabled.

Debug truth must never write to actor notebooks without a modeled action.

## Debug commands

Debug commands may exist for testing and diagnosis. They must be explicit.

Categories:

- inspect truth;
- inspect event log;
- inspect causal graph;
- inspect actor beliefs;
- inspect planner trace;
- inspect scheduler/reservations;
- force replay/projection rebuild;
- validate content;
- switch possession;
- inject test fixture event only in test/debug mode with explicit debug classification.

Debug injection must never be confused with ordinary play.

## TUI acceptance harness

The TUI must be testable, not merely manually inspected.

Harness capabilities:

- load deterministic fixture;
- bind controller to actor;
- inspect embodied view model;
- select action by stable semantic ID, not screen coordinates;
- advance scheduler;
- assert committed events;
- assert actor notebook updates;
- assert hidden truth absent from embodied view;
- switch to debug view and assert truth/causal graph available;
- replay input transcript against event log.

## TUI-first implications

Every first-slice feature must be playable through the TUI.

The TUI must support:

- movement between known places;
- inspecting visible objects;
- object affordance menus;
- speech-act menus;
- actor-known notebook;
- waiting/sleeping/travel stepping;
- reports/records/notices where accessible;
- why-not;
- debug toggle/view;
- event/causal inspection in debug;
- possession switching in debug.

A system is not first-slice ready if it only works through debug commands, tests, prose, or a planned graphical client.

## Future graphical client boundary

A graphical client may consume:

- actor-filtered embodied view models;
- debug view models;
- place graph/projection data;
- animation hints;
- affordance menus;
- event summaries.

It may not:

- implement world rules;
- read hidden truth in embodied mode;
- mutate world state directly;
- add player-only verbs;
- drive dramatic pacing;
- bypass event sourcing;
- bypass actor knowledge.

Graphical rendering is a client, not a second simulation.

## Acceptance implications

Player/TUI features must test:

- no `Player` entity in simulation state;
- human command uses same action pipeline as AI proposal;
- possession switch does not transfer knowledge;
- embodied view hides hidden truth;
- debug view reveals truth visibly;
- actor notebook source filtering;
- why-not generated from validation checks;
- no player-only actions in action registry;
- TUI no-human replay compatibility;
- future graphical boundary consumes view models only.

## Anti-patterns

- Player character has exclusive verbs.
- UI directly edits inventory.
- Possession resets needs/intentions.
- Actor notebook includes debug truth.
- Human notes become actor beliefs.
- Embodied map marker points to hidden truth.
- Debug view visually indistinguishable from embodied view.
- TUI duplicates action preconditions as a second rule engine.
- Graphical client becomes simulation authority.
- NPCs freeze when not possessed without modeled reason.
