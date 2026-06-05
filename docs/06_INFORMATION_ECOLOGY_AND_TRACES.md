# Information Ecology and Traces

## Core claim

For this project, information movement is as important as object movement.

A sword changing hands matters. So does a rumor changing confidence. So does a missing coin contradicting an expectation. So does a witness misidentifying a cloak. So does a clerk copying a report into a ledger.

The simulation must model how truth becomes observation, observation becomes belief, belief becomes speech, speech becomes rumor, rumor becomes institutional action, and institutional action becomes public artifact.

## Three realities

### Ground truth

What actually happened in the authoritative world state.

Example:

```text
Mara stole Tomas's coins at 02:13.
```

### Subjective belief

What a specific agent believes.

Example:

```text
Elena believes she heard someone near the bedroom, confidence 0.34.
Tomas believes the coins were in the strongbox last night, confidence 0.86.
The guard believes a theft may have occurred, confidence 0.51.
```

### Public/institutional record

What a group, office, notice, ledger, or rumor network treats as known.

Example:

```text
Town ledger: Tomas reported missing coins on 143 Rainwane.
Notice board: bounty posted for north-road bandits.
Tavern rumor: “Mara was seen near Tomas's place.”
```

These must remain separate.

## Information channels

Model at least these channels:

- direct sight;
- sound;
- smell;
- touch;
- reading;
- conversation;
- overhearing;
- rumor chain;
- formal report;
- institutional order;
- ledger entry;
- physical trace inspection;
- absence from expected state;
- memory;
- inference;
- deliberate lie;
- confession;
- coercion;
- divination or equivalent special system, if present.

Every channel needs:

```yaml
InformationChannel:
  reliability: 0.0-1.0
  delay: duration
  range: local/social/regional
  distortion_modes: []
  required_capabilities: []
  traceability: none/weak/strong
```

## Trace taxonomy

### Physical traces

- footprints;
- blood;
- broken locks;
- disturbed dust;
- missing goods;
- weapon damage;
- open door;
- smoke;
- corpse;
- dropped item;
- handwriting;
- fingerprints if setting supports it.

### Mental traces

- direct memory;
- fear response;
- suspicion;
- expectation violation;
- trauma;
- familiarity;
- changed trust.

### Social traces

- rumor;
- accusation;
- promise;
- insult;
- debt;
- reputation change;
- alliance;
- ostracism.

### Institutional traces

- report;
- ledger entry;
- warrant;
- bounty;
- court judgment;
- posted notice;
- tax debt;
- arrest record;
- guild sanction.

### Economic traces

- price changes;
- unpaid invoice;
- missing stock;
- changed supply route;
- wage promise;
- insurance/compensation claim if setting supports it.

### Erasure traces

- cleaned blood;
- forged record;
- burned notice;
- intimidated witness;
- moved corpse;
- bribed guard;
- altered lock;
- planted evidence.

Erasure is not absence. It is a causal event.

## From event to belief

A useful pipeline:

```text
Event occurs
  -> traces generated
  -> observers perceive event or traces
  -> observations interpreted through prior beliefs and traits
  -> beliefs created or revised
  -> emotions and goals update
  -> agents speak, record, conceal, investigate, or ignore
```

Example:

```text
Mara opens strongbox.
Elena hears faint noise.
Elena forms low-confidence belief: “someone moved in the bedroom.”
Tomas later sees empty strongbox.
Tomas forms high-confidence belief: “coins missing.”
Tomas combines this with Elena's testimony.
Tomas forms medium-confidence suspicion: “someone entered at night.”
```

## Observation is not interpretation

Keep raw observations separate from interpreted beliefs.

```yaml
Observation:
  observer: actor_elena
  kind: HeardNoise
  location: room_tomas_bedroom
  time: 02:14
  confidence: 0.34

Belief:
  holder: actor_elena
  proposition: SomeoneWasInBedroomAtNight
  confidence: 0.21
  source: observation_heard_noise
```

This makes ambiguity possible.

## Rumor mechanics

Rumors should mutate as they travel.

Rumor packet:

```yaml
Rumor:
  proposition: MaraSeenNearTomasHouse
  origin: actor_elena
  chain: [actor_elena, actor_anna, actor_oren]
  confidence: 0.42
  vividness: 0.61
  distortion:
    identity: possible
    time: likely
    motive: speculative
  social_charge: high
```

Mutation modes:

- omission;
- exaggeration;
- identity substitution;
- motive invention;
- confidence inflation;
- moral framing;
- social-status bias;
- compression into stereotype.

A rumor can become more socially powerful while becoming less accurate.

## Lies

A lie is not a false belief. A lie is an intentional communication of a proposition the speaker does not believe, or does not believe with that confidence.

```yaml
SpeechAct:
  kind: Lie
  speaker: actor_mara
  listener: actor_guard
  asserted: MaraWasHomeAllNight
  speaker_belief_confidence: 0.05
  intended_effect: reduce_suspicion
```

Lies create risk:

- contradiction if evidence emerges;
- trust damage;
- blackmail leverage;
- institutional sanction;
- guilt/shame depending on traits.

## Forgetting and memory distortion

Memory should not be perfect unless the setting justifies it.

Decay should depend on:

- emotional weight;
- repetition;
- social rehearsal;
- trauma;
- relevance to goals;
- intelligence/perception skill;
- time;
- contradiction.

Memory distortion can create:

- uncertain time;
- merged people;
- wrong clothing color;
- inflated confidence;
- missing context.

Do not overdo this in the first prototype. Use simple confidence decay and occasional low-stakes distortion first.

## Evidence

Evidence is not truth. Evidence is an observed trace or record that supports or contradicts propositions.

```yaml
Evidence:
  trace: trace_lock_scratch_01
  supports:
    - ContainerWasForcedOpen: 0.65
  contradicts:
    - OwnerOpenedNormally: 0.25
  admissible_by:
    - town_guard
    - reeve
  can_be_forged: true
```

Authorities can have evidence standards:

```yaml
EvidenceThresholds:
  open_case: 0.3
  question_suspect: 0.45
  search_home: 0.65
  arrest: 0.78
  convict: 0.9
```

Different institutions can have different thresholds.

## Stale information

Stale information is central.

Examples:

- a notice remains after bandits are dead;
- a guard searches for a suspect who fled yesterday;
- a merchant avoids a road that is now safe;
- an adventurer travels to a ruin already looted;
- a family waits for someone who died unobserved.

Staleness requires tracking:

- event time;
- acquisition time;
- last verification time;
- confidence decay;
- channels through which updates could arrive.

## Public knowledge

Public knowledge should not be global knowledge. It is knowledge that exists in accessible artifacts or widely shared local beliefs.

Public forms:

- notice board;
- town crier;
- tavern rumor;
- market price board;
- temple announcements;
- court records;
- guild posting;
- funeral/memorial;
- visible patrols;
- closed gates.

A player or NPC must physically or socially access these forms to learn from them.

## Investigation loop

A generic investigation procedure:

1. Receive report or notice anomaly.
2. Create institutional record.
3. Assign priority.
4. Identify last known state.
5. Search for traces.
6. Interview witnesses.
7. Compare contradictions.
8. Generate suspects or hypotheses.
9. Take action: ignore, monitor, question, search, detain, post bounty, close case.
10. Update records and beliefs.

The same loop can power theft, assault, missing persons, unpaid debt, contraband, fire, tax evasion, and monster threats.

## Debug views

The following views should exist early:

- event log by time;
- causal graph for selected event;
- trace map for selected location;
- belief inspector for selected actor;
- rumor chain viewer;
- institutional case file viewer;
- stale information viewer;
- contradiction viewer.

These are not optional. Without them, the simulation will become impossible to reason about.

## Minimal vertical-slice information model

Implement:

- direct perception;
- overheard sound;
- speech report;
- written notice;
- physical missing-item trace;
- simple disturbed-container trace;
- belief confidence;
- belief source;
- stale notice;
- rumor chain of at most three hops;
- institutional case file.

That is enough to make the village feel alive.
