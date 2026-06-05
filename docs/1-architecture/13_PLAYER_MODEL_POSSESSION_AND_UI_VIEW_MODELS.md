# Player Model, Possession, and UI View Models

## Core claim

The player is a temporary controller, not the protagonist of reality. The UI lets the human perceive, decide, and intervene from the current actor's position without granting ground-truth omniscience in embodied play.

## Controller model

```yaml
HumanController:
  id: human_0
  attached_actor: actor_mara
  mode: embodied
  possession_history:
    - actor_tomas
    - actor_mara
```

`possession_history` is development/debug metadata. It is not world knowledge.

## Possession doctrine

Possession changes input binding. It does not:

- reset suspicion;
- clear culpability;
- transfer actor knowledge;
- erase injuries;
- erase fatigue/hunger;
- alter relationships;
- alter obligations;
- alter institution records;
- grant new social permission;
- give the body player-only verbs;
- notify the world.

The previous body continues as an ordinary agent.

## Debug-first status

Non-diegetic possession is default for prototypes. It proves actor parity and prevents the codebase from baking in a sacred player.

Diegetic possession may later be a domain pack. If so, it needs metaphysics, memory leakage rules, consent, detection, law, religion, risk, social consequences, and explicit information channels. The core must not assume it.

## Actor knowledge vs player knowledge

The human may know Mara stole the coins. Tomas may not.

The current actor may act from:

- direct perception;
- memories;
- beliefs;
- speech heard by this actor;
- records read by this actor;
- inferences available to this actor;
- domain-defined channels available to this actor.

The current actor may not act institutionally from:

- the player's memory of another body;
- debug truth;
- hidden event log;
- another actor's private belief store;
- ground truth marker;
- LLM hallucination;
- a lead not known to the current actor.

## Action parity

Every committed action must pass the same action pipeline as NPC actions.

Allowed UI help:

- action list;
- sorting;
- filtering;
- why-not explanation;
- lead cards;
- actor-known notebook;
- route preview based on actor-known map/beliefs;
- debug overlay when explicitly enabled.

Forbidden:

- player-only world actions;
- “use quest item” privilege;
- accusation without actor-known basis;
- payment claim without actor-known claim/proof;
- objective marker to hidden truth;
- inventory manipulation outside events;
- speech act that asserts unheld truth unless deliberately modeled as a lie.

## View model boundary

Correct architecture:

```text
Simulation core
  -> projections
  -> actor-knowledge filter
  -> UI view models
  -> TUI renderer
  -> future graphical renderer
```

The renderer does not decide preconditions. The renderer does not create facts. The renderer does not maintain quest state.

## Embodied view models

Embodied view models include:

```yaml
EmbodiedView:
  current_actor: actor_id
  perceived_place:
    name: string
    exits: [exit_view]
    visible_actors: [actor_view]
    visible_objects: [object_view]
    perceived_traces: [trace_view]
  actor_status:
    hunger: visible_level
    fatigue: visible_level
    safety: visible_level
    injuries: actor_known
    current_intention: actor_known
  actions:
    available: [action_option]
    unavailable_with_reasons: [why_not]
  beliefs:
    actor_known: [belief_view]
  leads:
    actor_known: [lead_card]
  conversation:
    active: optional
  log:
    perceived_recent: [event_summary]
```

## Debug view models

Debug view models include:

```yaml
DebugView:
  authoritative_state: component_snapshot
  event_log: [event]
  causal_graph: graph_ref
  hidden_beliefs: [belief_store]
  planner_trace: [planner_decision]
  institutions: [institution_record]
  traces: [trace_debug_view]
  lod_state: [lod_debug]
  replay_controls: [control]
  controller_binding: controller_debug
```

Debug information must never leak into embodied view models.

## Lead notebook

The notebook distinguishes:

```text
known by human
known by current actor
observed directly
heard from person
rumored
read from notice
recorded by authority
inferred
contradicted
stale
debug truth
```

A graphical UI later may present this beautifully. The TUI must present it correctly.

## Objective markers

Markers represent beliefs, reports, maps, records, or observations.

If a notice claims bandits are near the quarry and they moved yesterday, the marker remains wrong until updated through modeled channels.

## Why-not explanations

Why-not explanations must cite the actual failed precondition or actor-knowledge barrier.

Examples:

```text
You cannot accuse Mara with evidence: Tomas has no actor-known evidence linking her.
```

```text
You cannot claim payment now: the office is closed and the payment ledger is locked inside.
```

```text
You cannot open the strongbox: you do not have the key and are not willing to force it.
```

Debug mode may show more detail.

## Possession tests

Required tests:

- player steals as Mara, switches to Tomas, Tomas does not know culprit;
- player reads notice as Oren, switches to Elena, Elena does not know the notice;
- player learns hidden debug truth, embodied actor cannot testify from it;
- previous body resumes its own needs/intentions after possession switch;
- institution does not reference controller identity;
- action registry contains no player-only world-affecting action.

## Future graphical UI

A future graphical client must reuse:

- action pipeline;
- actor-knowledge filters;
- view models;
- notebooks;
- speech-act interface;
- debug/replay architecture.

Graphics may add presentation, not truth.

## Anti-patterns

- `PlayerCharacter` as world entity;
- player inventory separate from actor possessions;
- possession resets consequences;
- global journal with objective truth;
- UI button that bypasses action pipeline;
- graphical renderer as second simulation;
- debug mode accidentally used for normal play;
- NPCs reacting to controller rather than actor.
