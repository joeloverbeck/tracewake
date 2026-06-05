# Information Ecology and Speech Acts

## Core claim

Information movement is as important as object movement.

Truth becomes observation, observation becomes belief, belief becomes speech, speech becomes rumor or record, and rumor or record becomes later action. At every step, information can be delayed, distorted, ignored, contradicted, weaponized, erased, or misunderstood.

## Three realities

Tracewake must separate:

### Ground truth

What actually happened in authoritative state.

### Subjective belief

What a specific agent believes.

### Public or institutional record

What a group, office, ledger, notice, rumor network, family, guild, or court treats as known.

These realities often diverge. That divergence is gameplay.

## Information channels

Channels include:

- sight;
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
- lie;
- confession;
- coercion;
- special senses defined by domain packs.

Each channel defines reliability, delay, range, distortion modes, required capabilities, social context, traceability, and possible countermeasures.

## Observation is not interpretation

```yaml
Observation:
  observer: actor_elena
  kind: HeardNoise
  place: room_tomas_bedroom
  time: 142-08-12T02:14
  confidence: 0.34
  raw_payload:
    direction: near_strongbox_room
    duration: brief

Belief:
  holder: actor_elena
  proposition: SomeoneWasInBedroomAtNight
  confidence: 0.21
  source:
    kind: sound_observation
    source_id: obs_elena_noise
```

This supports ambiguity. Elena heard a sound. She did not perceive the theft.

## Belief model

Important beliefs include:

- holder;
- proposition;
- truth value as believed;
- confidence;
- source;
- channel;
- acquired time;
- believed event time if known;
- emotional weight;
- decay profile;
- contradiction links;
- whether it is speakable, private, shameful, privileged, or institutionally admissible.

Beliefs are not global flags. They are agent-specific state.

## Expectations

Agents maintain expectations about:

- owned items and storage;
- work schedules;
- who should be home;
- shop hours;
- road safety;
- authority behavior;
- debts;
- social promises;
- household norms;
- object states;
- route arrival times.

Expectation contradiction creates surprise, candidate goals, trace inspection, reports, emotional shifts, and memory salience.

## Trace taxonomy

### Physical traces

Footprints, blood, broken locks, disturbed dust, missing goods, weapon damage, open doors, smoke, corpses, dropped items, handwriting, torn fabric, spoiled food.

### Mental traces

Memory, suspicion, fear, shame, expectation violation, changed trust, trauma association, private guilt, confusion.

### Social traces

Rumor, accusation, promise, insult, debt, alliance, ostracism, reputation shift, witness pressure, blackmail.

### Institutional traces

Report, ledger entry, warrant, bounty, notice, judgment, tax debt, guild sanction, arrest record, case file, payment record.

### Economic traces

Price change, unpaid invoice, missing stock, changed route, wage promise, shortage, debt, tax arrears.

### Environmental traces

Weather damage, mud, animal tracks, fire spread, disease signs, crop failure, road blockage.

### Erasure traces

Cleaned blood, forged record, burned notice, intimidated witness, moved corpse, planted evidence, repaired lock, replaced ledger page.

Erasure is not absence. It is an event.

## Evidence

Evidence is an observed trace or record that supports or contradicts propositions. Evidence is not truth.

```yaml
EvidenceInterpretation:
  trace: trace_lock_scratch_01
  observer: actor_elias
  supports:
    - ContainerWasForcedOpen: 0.65
  contradicts:
    - OwnerOpenedNormally: 0.25
  uncertainty:
    - old_damage_possible
    - owner_carelessness_possible
```

Institutions have evidence thresholds for opening cases, questioning suspects, searching homes, arresting, convicting, paying rewards, or closing files.

## Speech acts

Conversation is typed. Surface prose is secondary.

```yaml
SpeechAct:
  kind: Report
  speaker: actor_tomas
  listeners: [actor_anna_clerk]
  propositions:
    - ItemMissing(coin_stack_01)
    - LastSeenIn(coin_stack_01, strongbox_tomas, yesterday_evening)
    - ElenaHeardPossibleNoise
  asserted_confidence: 0.78
  source_beliefs:
    - belief_tomas_missing_coins
    - belief_tomas_elena_noise_hearsay
  intention: request_authority_help
```

Speech act kinds:

```text
tell, ask, answer, report, accuse, deny, lie, confess, promise,
threaten, bargain, recruit, refuse, warn, gossip, instruct, command,
testify, joke, deflect, withhold, apologize
```

## Lies

A lie is intentional communication that the speaker does not believe, or does not believe at the asserted confidence.

```yaml
SpeechAct:
  kind: Lie
  speaker: actor_mara
  listeners: [actor_elias]
  asserted:
    - MaraWasHomeAllNight
  speaker_belief_confidence: 0.05
  intended_effect: reduce_suspicion
  risk:
    contradiction: medium
    trust_damage: high
```

A lie is valid if the actor has motive and can attempt it. A hallucinated fact from an LLM is not a valid lie.

## Rumors

Rumors mutate through:

- omission;
- exaggeration;
- identity substitution;
- motive invention;
- confidence inflation;
- moral framing;
- status bias;
- kinship bias;
- stereotype compression;
- conflation with older events;
- source loss.

A rumor can become more socially powerful while becoming less accurate.

## Memory and forgetting

Agents do not retain infinite perfect logs.

Memory uses salience, emotional weight, recency, repetition, social importance, contradiction, and role relevance. Forgetting and misremembering are allowed only as modeled belief changes with provenance.

## Stale information

Track:

- event time;
- acquisition time;
- last verification;
- source;
- confidence decay;
- update channels;
- who has access to updates;
- known contradictions.

Stale notices, old maps, dead suspects, already-looted sites, moved threats, closed shops, unpaid contracts, and outdated rumors are central mechanics.

## Conversation with the player

When an NPC tells the current body about treasure:

1. NPC has belief, rumor, record, or chooses to lie.
2. Speech act is generated.
3. Optional LLM renders wording.
4. Structured propositions are validated.
5. Current actor observes speech.
6. Belief update occurs.
7. Actor notebook records source-bound claim.

The LLM's prose does not create treasure.

## Player speech

Free player text, if supported, is parsed into proposed speech acts. The action pipeline validates whether the current actor can assert truthfully, lie deliberately, speculate, ask, accuse, report, promise, or refuse.

Tomas cannot truthfully say “Mara stole my coins” unless he learned it. He may say “my coins are gone,” “I suspect Mara,” “I heard Mara was nearby,” or lie if he chooses.

## Minimal first model

Implement:

- direct perception;
- sound observation;
- reading notices;
- speech report;
- missing-item/absence trace;
- disturbed-container trace;
- belief source/confidence;
- expectation contradiction;
- stale notice;
- rumor chain up to three hops;
- lie speech act;
- institutional case file;
- conversation log with structured claims;
- actor-filtered TUI notebook.

## Anti-patterns

- omniscient NPC dialogue;
- rumor that has no source chain;
- record without author;
- evidence treated as truth;
- objective marker from hearsay;
- stale info automatically corrected;
- UI labeling lies without actor basis;
- player notebook becoming actor knowledge;
- LLM prose creating facts.
