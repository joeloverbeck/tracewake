# Claims, Beliefs, Traces, Memory, and Information Flow

## Status

This document defines the architecture for typed claims/propositions, truth, observation, interpretation, belief, traces, memory, rumors, records as belief sources, stale information, expectation contradiction, search, and actor-knowledge filtering.

Tracewake is belief-first. Gameplay emerges when ground truth, subjective belief, and public/institutional record diverge.

## Core rule

Embodied actors, ordinary planners, dialogue surfaces, institutions, and embodied TUI views must not read ground truth unless the actor or institution learned it through modeled channels.

Debug may inspect truth. Debug truth must not become actor knowledge without a modeled transfer event.

## Authority

This subsystem owns:

- typed proposition/claim vocabulary;
- observation records;
- interpretation and belief update pipeline;
- belief store contracts;
- source/provenance tracking;
- trace shapes and discovery semantics;
- memory salience, forgetting, misremembering, and stale information;
- rumor packet mutation and source chains;
- absence-as-evidence;
- actor-knowledge filtering contexts.

It is denied:

- hidden truth transfer;
- evidence-as-certainty shortcuts;
- omniscient institutions;
- UI culprit labels in embodied mode;
- LLM fact creation;
- deleting active causal ancestry.

## Three realities

Tracewake separates:

```text
ground truth
  what the authoritative simulation state/event history says happened

subjective belief
  what a specific actor believes, with confidence, stance, source, channel, and time

public/institutional record
  what a household, office, ledger, notice board, rumor network, guild, market,
  or other institution treats as known
```

These layers may agree, diverge, become stale, contradict one another, or influence future action despite being wrong.

## Typed claims and propositions

Typed propositions are the epistemic currency.

Examples:

```text
ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
ItemMissing(coin_stack_01)
ActorWasNearPlace(actor_mara, room_tomas_bedroom, time_window_night)
NoiseHeardNearPlace(room_tomas_bedroom, time_window_night)
PersonReportsClaim(actor_tomas, ItemMissing(coin_stack_01))
InstitutionOpenedIncident(reeves_office, incident_missing_coins_01)
```

Surface language may render these claims, but prose is not the authoritative claim representation.

## Observation versus interpretation

Observation is channel-specific input. Interpretation is a belief update.

```yaml
Observation:
  id: obs_elena_noise_12
  observer: actor_elena
  channel: sound
  observed_at: 142-08-12T02:14
  observer_place: hall_tomas_house
  perceived_origin: near_tomas_bedroom
  raw_payload:
    duration: brief
    intensity: low
    material_hint: metal_or_wood
  confidence: 0.34
  source_event: evt_possible_container_noise

BeliefUpdate:
  holder: actor_elena
  proposition: SomeoneOrSomethingMovedNearTomasRoom
  stance: plausible
  confidence_delta: +0.21
  source: obs_elena_noise_12
  alternatives:
    - house_settling
    - person_moving
    - animal_or_object_shift
```

Elena heard a sound. She did not perceive a theft.

## Belief contract

Important beliefs use a structured, source-bound shape.

```yaml
Belief:
  id: belief_tomas_coins_in_strongbox
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  stance: believed_true
  confidence: 0.86
  source:
    kind: direct_observation
    source_id: evt_tomas_checked_strongbox_yesterday
  channel: sight
  acquired_at: 142-08-11T20:10
  believed_event_time: 142-08-11T20:10
  last_verified_at: 142-08-11T20:10
  decay_profile: ordinary_property_expectation
  emotional_weight: medium
  privacy: private_household
  speakable: true
  admissibility:
    reeves_office: weak_self_report
  contradictions:
    - belief_tomas_strongbox_empty_after_search
```

A proposition is not a global flag. Beliefs are holder-specific.

## Source and provenance

Every important belief, rumor, memory, record claim, lead, and institutional fact needs provenance.

Source categories include:

- direct sight/sound/smell/touch;
- reading;
- conversation;
- overhearing;
- formal report;
- institutional order;
- ledger/notice/record;
- physical trace inspection;
- absence from expected state;
- memory;
- inference;
- rumor chain;
- lie;
- confession;
- coercion;
- summary event;
- boundary process;
- domain-defined special channel.

Provenance must answer who told whom, when, through what channel, with what confidence, and how the claim mutated.

## Belief update pipeline

```text
world event or trace exists
 -> perception channel selects possible observer(s)
 -> observation created with raw channel data
 -> actor interpretation uses prior beliefs, expectations, traits, attention,
    lighting, relationships, credibility, and context
 -> belief update proposed
 -> contradiction links checked
 -> memory salience calculated
 -> possible speech/report/reaction goals generated
 -> belief projection updated
 -> event/provenance committed if future reasoning may care
```

Belief update is not automatic truth transfer.

## Traces

A trace is a world artifact, environmental mark, social mark, mental mark, institutional artifact, economic signal, or erasure artifact that can later influence observations and beliefs.

Trace categories:

- physical: footprints, disturbed dust, missing item, broken lock, open door, blood, dropped item;
- mental: fear, guilt, suspicion, shame, expectation violation;
- social: gossip, accusation, insult, promise, reputation shift;
- institutional: report, ledger entry, warrant/order, notice, case file;
- economic: unpaid debt, missing stock, price change, shortage, wage promise;
- environmental: mud, smoke, weather damage, disease sign, road blockage;
- erasure: cleaned blood, repaired lock, burned notice, forged page, intimidated witness, moved object.

Erasure is an event and often a trace.

```yaml
Trace:
  id: trace_disturbed_strongbox_44
  kind: disturbed_container
  place: room_tomas_bedroom
  entity: strongbox_tomas
  created_by_event: evt_item_removed_from_container
  created_at: 142-08-12T02:13
  visibility:
    sight: low
    touch: medium
  persistence:
    expected_decay: days
    removable_by_actions: [clean_container, rearrange_contents]
  interpretations:
    - proposition: ContainerWasOpenedRecently
      support: 0.55
    - proposition: OwnerOpenedNormally
      support: 0.20
    - proposition: ForcedEntry
      support: 0.10
  discovered_by: []
```

## Absence as evidence

Absence is evidence only for an actor with an expectation.

```text
expected: coin_stack_01 in strongbox_tomas
observed: strongbox_tomas contains no coin_stack_01 during search
contradiction: high
candidate beliefs:
  - coins were moved
  - coins were stolen
  - Tomas remembered wrong
  - Elena borrowed them
  - search was incomplete
```

An empty strongbox is just an empty strongbox to an actor with no expectation.

## Search architecture

Search is an action family, not a UI query over truth.

Search must define:

- actor belief about target;
- search area and precision;
- time cost;
- access and privacy implications;
- perception channels;
- possible traces discovered;
- false negatives;
- false positives/misinterpretations;
- interruption windows;
- social consequences.

Search cannot reveal the true culprit unless the actor discovers evidence supporting that inference.

## Rumors

Rumors are source-bound, mutating information packets.

```yaml
RumorPacket:
  id: rumor_missing_coins_03
  topic: MissingProperty(coin_stack_01)
  current_claims:
    - TomasLostCoins
    - SomeoneWasNearHouseAtNight
  origin:
    source_event: speech_tomas_to_clerk
  chain:
    - speaker: actor_tomas
      listener: actor_anna_clerk
      act: Report
    - speaker: actor_anna_clerk
      listener: actor_evan_guard
      act: Tell
    - speaker: actor_evan_guard
      listener: actor_mara
      act: Gossip
  mutations:
    - omitted_uncertainty_about_noise
    - inflated_value_of_coins
  accuracy: unknown_to_most_actors
```

Rumors can become socially powerful while becoming less accurate.

## Memory, forgetting, and stale information

Memory uses salience, recency, repetition, emotional weight, contradiction, role relevance, relationship relevance, institutional relevance, rehearsal through speech, shame/trauma, and LOD tier.

Forgetting and misremembering are modeled belief/memory changes with provenance. Do not randomly delete causally important ancestry.

Information-bearing artifacts track:

- event time;
- acquisition time;
- last verification;
- source;
- confidence decay;
- update channels;
- access to updates;
- known contradictions;
- stale-risk display where actor has basis.

A stale notice is a world artifact, not a bug.

## Records as belief sources

Records create beliefs only when read, reported, copied, cited, or institutionally relied on.

```text
ledger entry exists
 -> clerk reads it or created it
 -> clerk forms institutional belief
 -> clerk tells guard
 -> guard forms belief from clerk's authority
 -> guard questions suspect
```

The ledger's claim is not automatically known by the village.

## Actor-knowledge filtering

Every embodied view model and ordinary planner query must specify a knowledge context.

```yaml
KnowledgeContext:
  viewer: actor_tomas
  mode: embodied
  allowed_sources:
    - current_perception
    - actor_beliefs
    - actor_memories
    - actor_read_records
    - heard_speech
    - public_notices_seen
  forbidden_sources:
    - ground_truth_unobserved_events
    - hidden_debug_causal_graph
    - other_actor_private_beliefs
    - player_memory_outside_actor
```

Debug mode uses a different context and must be visibly non-diegetic.

## Acceptance implications

Information features must test:

- observation does not equal interpretation;
- belief has typed proposition and source;
- ground truth does not leak to embodied view/planner/institution;
- stale information stays stale until updated through channels;
- wrong beliefs arise legibly;
- records and rumors transfer beliefs only when observed/read/heard;
- absence-as-evidence requires expectation;
- actor possession does not merge notebooks;
- debug explains truth/belief/record mismatch.

## Anti-patterns

- NPC dialogue reads event-log truth.
- UI labels the real culprit in embodied mode.
- Rumor has no source chain.
- Evidence is treated as truth.
- Actor searches exact true location from hearsay.
- Notice automatically updates when truth changes.
- Institution sanctions from hidden violation classification.
- Forgetting deletes source of active suspicion.
- LLM prose creates a fact.
