# Epistemic Information, Trace, and Memory Architecture

## Status

This document defines the architecture for truth, observation, belief, traces, memory, records, rumors, stale information, inference, expectation contradiction, and actor-knowledge filtering.

Tracewake is belief-first. Gameplay emerges when ground truth, subjective belief, and public/institutional record diverge.

## Three realities

Tracewake separates:

```text
ground truth
  what the authoritative simulation state says happened or is true

subjective belief
  what a specific actor believes, with confidence, source, channel, and time

public/institutional record
  what a household, office, ledger, notice board, rumor network, guild, market,
  or other institution treats as known
```

These layers may agree, diverge, become stale, contradict one another, or influence future action despite being wrong.

## Rule against ground-truth leakage

Embodied actors, embodied TUI, dialogue surfaces, institutions, and ordinary agent planners must not read ground truth unless the actor/institution has learned it through modeled channels.

Allowed:

```text
Debug causal graph shows Mara stole the coins.
Tomas's embodied view shows "coins missing from expected place" and "Elena reported a noise".
The clerk record shows "Tomas reports missing coins".
```

Forbidden:

```text
Tomas's notebook says "Mara stole the coins" because the event log knows it.
Guard AI targets Mara because the norm engine classified true theft.
Notice points to the true hiding place.
Dialogue labels a lie as a lie without actor-known basis.
```

## Observation vs interpretation

Observation is raw or channel-specific input. Interpretation is a belief update.

```yaml
Observation:
  id: obs_elena_noise_12
  observer: actor_elena
  channel: sound
  observed_at: 142-08-12T02:14
  place_of_observer: hall_tomas_house
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
  confidence_delta: +0.21
  source: obs_elena_noise_12
  interpretation_notes:
    - could_be_house_settling
    - could_be_person
```

Elena heard a sound. She did not perceive a theft.

## Belief shape

Important beliefs use a structured shape.

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

Beliefs are actor-specific. A proposition is not a global flag.

## Source and provenance

Every important belief, rumor, record, memory, lead, and institutional fact needs provenance.

Source categories:

- direct sight;
- sound;
- smell;
- touch;
- reading;
- conversation;
- overhearing;
- formal report;
- institutional order;
- ledger entry;
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

Provenance must preserve enough data to answer: who told whom, when, through what channel, with what confidence, and how it mutated.

## Belief update pipeline

```text
world event or trace exists
 -> perception channel selects possible observer(s)
 -> observation created with raw channel data
 -> actor interpretation uses prior beliefs, expectations, traits, attention, lighting,
    relationship, credibility, and context
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
- social: gossip, accusation, insult, promise, reputation shift, blackmail;
- institutional: report, ledger entry, warrant, notice, case file, payment record;
- economic: unpaid debt, missing stock, price change, shortage, wage promise;
- environmental: mud, smoke, weather damage, disease sign, road blockage;
- erasure: cleaned blood, repaired lock, burned notice, forged page, intimidated witness, moved object.

Erasure is an event and often a trace.

## Trace shape

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

Trace interpretation depends on observer competence, expectations, lighting, attention, prior beliefs, and access.

## Absence as evidence

Absence is evidence only when an actor has an expectation.

```text
expected: coin_stack_01 in strongbox_tomas
observed: strongbox_tomas contains no coin_stack_01 during search
contradiction: high
belief candidates:
  - coins were moved
  - coins were stolen
  - Tomas remembered wrong
  - Elena borrowed them
  - strongbox was searched incorrectly
```

An empty strongbox is just an empty strongbox to an actor with no expectation about its contents.

## Expectation contradiction

Expectations are beliefs about what should be true now or at a future time.

Actors maintain expectations for:

- owned items and storage;
- household members at home;
- work schedules;
- shop hours;
- office hours;
- route arrival times;
- debts and promises;
- bed occupancy;
- doors and locks;
- food stores;
- record locations;
- social obligations.

Contradiction triggers:

- surprise event;
- belief update;
- emotional modulation;
- search/ask/report/conceal candidate goals;
- trace inspection;
- institutional report possibility;
- memory salience increase.

## Search architecture

Search is an action family, not a UI query over truth.

Search must define:

- actor belief about target;
- search area and precision;
- time cost;
- required access;
- visibility/perception channels;
- possible traces discovered;
- false negatives;
- false positives/misinterpretations;
- social/normative implications;
- interruption windows.

Example:

```text
Search strongbox
 -> inspect visible contents
 -> touch/rearrange if allowed or willing to violate privacy
 -> discover absence if actor expected item
 -> maybe discover disturbed-container trace
 -> maybe miss faint trace due low light/fatigue
```

Search cannot reveal the true culprit unless the actor finds evidence that supports that inference.

## Rumor architecture

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
  confidence_social: rising
  accuracy: unknown_to_most_actors
```

Rumors can become socially powerful while becoming less accurate.

## Memory and forgetting

Agents do not have perfect infinite logs.

Memory uses:

- salience;
- recency;
- repetition;
- emotional weight;
- contradiction;
- role relevance;
- relationship relevance;
- institutional relevance;
- rehearsal through speech;
- trauma or shame;
- LOD tier.

Forgetting and misremembering are modeled belief/memory changes with provenance. Do not randomly delete causally important ancestry.

## Stale information

Track for information-bearing artifacts:

- event time;
- acquisition time;
- last verification;
- source;
- confidence decay;
- update channels;
- access to updates;
- known contradictions;
- stale-risk display for actor-known UI.

A stale notice is not a bug. It is a world artifact whose continued existence can cause wrong action.

## Records as belief sources

Records create beliefs only when read, reported, copied, cited, or institutionally relied on.

```text
ledger entry exists
 -> clerk reads it
 -> clerk forms institutional belief
 -> clerk tells guard
 -> guard forms belief from clerk's authority
 -> guard questions suspect
```

The ledger's claim is not automatically known by the village.

## Actor-knowledge filtering

Every embodied view model and actor planner query must specify a knowledge context.

```yaml
KnowledgeContext:
  viewer: actor_tomas
  mode: embodied
  allowed_sources:
    - actor_current_perception
    - actor_beliefs
    - actor_memories
    - actor_read_records
    - actor_heard_speech
    - actor_public_notices_seen
  forbidden_sources:
    - ground_truth_unobserved_events
    - hidden_debug_causal_graph
    - other_actor_private_beliefs
```

Debug mode uses a different context and must be visibly non-diegetic.

## Acceptance implications

A feature involving information must test:

- observation does not equal interpretation;
- belief has source/provenance;
- ground truth does not leak to embodied view;
- stale information remains stale until updated through channels;
- false/wrong beliefs can arise legibly;
- records and rumors transfer beliefs only when observed/read/heard;
- absence-as-evidence requires expectation;
- actor possession does not merge notebooks;
- debug can explain truth/belief mismatch.

## Anti-patterns

- NPC dialogue reads event log truth.
- UI labels the real culprit in embodied mode.
- Rumor has no source chain.
- Evidence is treated as truth.
- Actor searches exact true location from hearsay.
- Notice automatically updates when threat moves.
- Institution sanctions from hidden violation classification.
- Forgetting deletes the source of an active suspicion.
- LLM prose creates a fact.
