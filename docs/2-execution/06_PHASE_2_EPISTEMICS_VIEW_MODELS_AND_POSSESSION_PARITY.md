# Phase 2: Epistemics, View Models, and Possession Parity

## Purpose

Phase 2 proves the central Tracewake claim: belief comes before truth.

It adds observations, beliefs, expectation contradiction, actor-known filtering, embodied/debug view-model separation, and possession parity. After Phase 2, the player may know a fact while the current actor does not, and the system must behave accordingly.

The phase ends only when a player can steal or move property as one actor, switch to another actor, and fail to use the first actor's knowledge unless a modeled information channel transfers it.

## Entry requirements

Phase 2 may start only when Phase 1 exits.

Required:

- event log and replay work for item movement;
- TUI can play movement/door/container/item operations;
- debug attach and event log exist;
- view-model boundary exists;
- minimal fixture has Tomas, Mara, Elena, strongbox, coin item, rooms, and place graph;
- Phase 0 epistemic vocabularies are approved.

## Deliverables

### Observation events

Add observation as a first-class event family.

Required channels:

- direct sight;
- direct touch/search result;
- simple sound;
- reading placeholder if records are visible later;
- absence marker when an expectation exists.

Observation is not interpretation. Elena hearing a sound does not create knowledge of theft.

Observation shape:

```yaml
Observation:
  observer: actor_elena
  channel: sound
  observed_target: evt_possible_container_noise
  place: hall_tomas_house
  time: sim_time
  raw_payload:
    intensity: low
    origin_estimate: near_tomas_bedroom
    material_hint: metal_or_wood
  confidence: 0.34
```

### Belief store

Add a source-backed belief store.

Minimum belief fields:

```yaml
Belief:
  holder: actor_id | institution_id
  proposition:
    type: proposition_kind
    args: map
  stance: believed_true | believed_false | uncertain
  confidence: 0.0-1.0
  source:
    kind: direct_observation | sound | search_absence | speech | record | inference | authored_initial
    source_id: event_or_belief_or_record
  acquired_at: sim_time
  believed_event_time: optional
  channel: optional
  contradictions: [belief_id]
  privacy: public | household | private | privileged
```

No important belief may lack holder and source.

### Expectations and absence-as-evidence

Add expectations as beliefs about what should be true.

Required first expectation:

```text
Tomas believes coin_stack_01 is in strongbox_tomas
source: prior direct observation or seed event
confidence: high
```

When Tomas searches/checks the strongbox and the coin is absent, the system creates:

- observation of container contents;
- absence marker;
- contradiction link;
- belief update;
- candidate goals: search, ask, report, conceal, suspect if basis exists.

An empty container creates no missing-item contradiction for an actor with no expectation.

### Actor-known filtering

Every embodied view and planner query must use a knowledge context.

Embodied allowed sources:

- current perception;
- actor beliefs;
- actor memories;
- actor-read records;
- actor-heard speech;
- actor-observed traces;
- actor's own actions.

Forbidden sources:

- unobserved event-log truth;
- debug causal graph;
- other actors' private beliefs;
- hidden item location;
- human/debug notes;
- previous possessed actor knowledge.

### Embodied vs debug view models

Embodied view model:

- actor-filtered;
- uncertainty preserving;
- source-backed;
- no hidden truth;
- no debug annotations;
- no player memory.

Debug view model:

- non-diegetic;
- may show truth, event log, causal graph, hidden beliefs, hidden traces, validation reports, possession history, and projection diffs;
- must not write into actor notebook.

### Possession parity

Possession metadata remains:

```text
HumanController -> ActorId
```

It is controller/debug state, not world truth.

Tests must prove:

- binding changes input only;
- previous actor remains ordinary agent;
- beliefs stay with actors;
- intentions stay with actors;
- possessions and custody stay in world state;
- human possession history does not become world fact;
- no action gains special authority when human-controlled;
- embodied view after switch shows current actor knowledge only;
- debug view can show mismatch.

### Actor-known notebook

Add a notebook projection for the current actor.

It may show:

- source-bound beliefs;
- confidence;
- acquisition time;
- contradiction links;
- actor-known recent observations;
- speech heard/said by actor;
- actor-known possible actions/leads;
- stale risk if actor has basis.

It excludes hidden truth, debug notes, and previous-body knowledge.

### Human/debug notes

Add debug notes as separate non-diegetic artifacts if useful.

They must never satisfy action preconditions in embodied play.

### Why-not explanations

Phase 2 why-not must distinguish:

- physical blocker;
- knowledge blocker;
- social/norm placeholder blocker;
- debug-only blocker.

Embodied why-not is actor-filtered. Debug why-not may show hidden validation details.

Example embodied:

```text
You cannot accuse Mara as a truthful claim: Tomas does not know evidence linking Mara.
Possible actions: search the room, ask Elena what she heard, report missing property, make a reckless accusation if modeled.
```

## Explicit non-goals

Phase 2 does not include:

- full needs/routines;
- no-human daily village acceptance;
- institution report records;
- wrong suspicion as authority behavior;
- full memory distortion;
- broad rumor mutation;
- live LLM dialogue;
- road threat;
- travel/companion systems.

## TUI/view-model gate

Required TUI acceptance path:

1. attach to Mara in debug;
2. move to Tomas's house if physically possible;
3. open/search strongbox if action preconditions allow;
4. take or move coin;
5. switch to Tomas;
6. inspect Tomas embodied view;
7. verify Tomas does not know Mara acted;
8. search/check strongbox as Tomas;
9. see missing-property contradiction;
10. inspect Tomas actor-known notebook;
11. switch to debug;
12. inspect truth/belief mismatch and causal chain.

Embodied mode must not show "Mara stole the coins" unless Tomas has a modeled source for that belief.

## No-human simulation gate

No-human Phase 2 must prove:

- perception and belief updates can occur for NPCs;
- an actor can discover absence without a human;
- no event references player identity;
- possession metadata absence does not alter outcomes;
- no-human replay rebuilds belief projections.

The no-human run may still be short and fixture-controlled. It does not yet need full ordinary daily life; that is Phase 3.

## Deterministic replay gate

Replay must rebuild:

- observations;
- belief updates;
- expectation contradiction;
- actor-known notebook projection;
- embodied view-model snapshots for key moments;
- debug truth/belief mismatch;
- possession controller stream as non-world metadata.

Replay failure must identify event position, content version, projection version, and unsupported event/schema issue.

## Test gate

### Unit tests

- observation creation;
- observation does not equal belief;
- belief store insert/update;
- source required;
- contradiction link creation;
- absence requires expectation;
- actor-known filtering;
- debug view truth access;
- why-not knowledge blocker.

### Property tests

- no important belief lacks source;
- holder is never omitted;
- debug truth never appears in embodied view;
- possession switch never copies beliefs;
- actor cannot truthfully report/accuse from hidden truth;
- same event log supports different actor-filtered views;
- replay rebuilds belief projection.

### Golden tests

- `expectation_contradiction_001`;
- `possession_parity_001`;
- `view_filtering_001`;
- `strongbox_001` partial variant;
- `knowledge_blocker_accuse_001`;
- `sound_uncertainty_001`.

## Data/fixture gate

Fixtures must provide:

- source-backed initial beliefs;
- expectations with check contexts;
- observation channel parameters;
- visibility/sound profiles for rooms/doors at minimal level;
- actor privacy flags;
- actor-known notebook projection rules;
- no hidden truth fields such as `NPC.knows_truth`;
- no objective marker to true item location.

Example:

```yaml
InitialBelief:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  confidence: 0.86
  source:
    kind: prior_direct_observation
    source_id: seed_evt_tomas_checked_strongbox
```

## Debug/inspection gate

Debug must answer:

- what is ground truth?
- who observed what?
- which observations created which beliefs?
- which beliefs contradict?
- what does each possessed actor know?
- which knowledge precondition blocked an action?
- what hidden truth differs from actor belief?
- what came from debug metadata only?

Canonical debug query:

```text
Why can Tomas not accuse Mara?
```

Expected answer:

```text
Tomas has belief ItemMissing(coin_stack_01) from search_absence.
Tomas has belief ElenaHeardNoiseNearRoom if Elena told him.
Tomas has no actor-known belief connecting Mara to the strongbox.
Debug truth shows Mara removed the item, but that event is unobserved by Tomas.
```

## Failure cases to model

Required:

- actor cannot perceive item behind closed container;
- actor sees absence but has no expectation;
- actor expected wrong location;
- low-confidence sound observation;
- observation misinterpreted;
- actor tries to report fact not held as belief;
- actor tries to accuse from debug truth;
- view model attempts to include hidden item location;
- possession switch copies notebook in bug test and fails;
- replay missing belief source fails.

## Forbidden shortcuts

- belief as global proposition flag;
- actor notebook reading event log truth;
- guard AI reading true theft classification;
- UI hiding truth manually after querying ground truth;
- possession history as world fact;
- direct debug notes satisfying actor knowledge;
- LLM prose creating a belief;
- perfect clues without causal reason.

## Phase exit checklist

Phase 2 exits when:

- observations and beliefs are source-backed;
- expectation contradiction works;
- embodied/debug view models are separated;
- possession parity passes;
- actor-known notebook excludes hidden truth;
- why-not explains knowledge blockers;
- no-human epistemic run works;
- replay rebuilds belief projections;
- golden tests pass.

Only then begin Phase 3.
