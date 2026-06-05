# Information Ecology and Speech Acts

## Core claim

Information movement is as important as object movement.

Truth becomes observation, observation becomes belief, belief becomes speech, speech becomes rumor or record, and rumor or record becomes later action.

## Three realities

### Ground truth

What actually happened.

### Subjective belief

What a specific agent believes.

### Public or institutional record

What a group, office, ledger, notice, or rumor network treats as known.

These must remain separate.

## Information channels

Channels include sight, sound, smell, touch, reading, conversation, overhearing, rumor chain, formal report, institutional order, ledger entry, physical trace inspection, absence from expected state, memory, inference, lie, confession, coercion, and special senses.

Each channel defines reliability, delay, range, distortion modes, required capabilities, and traceability.

## Observation is not interpretation

```yaml
Observation:
  observer: actor_elena
  kind: HeardNoise
  place: room_tomas_bedroom
  time: 142-08-12T02:14
  confidence: 0.34

Belief:
  holder: actor_elena
  proposition: SomeoneWasInBedroomAtNight
  confidence: 0.21
  source: observation_heard_noise
```

This supports ambiguity.

## Trace taxonomy

Physical traces: footprints, blood, broken locks, disturbed dust, missing goods, weapon damage, open doors, smoke, corpses, dropped items, handwriting.

Mental traces: memory, suspicion, fear, expectation violation, changed trust, trauma association.

Social traces: rumor, accusation, promise, insult, debt, alliance, ostracism, reputation shift.

Institutional traces: report, ledger entry, warrant, bounty, notice, judgment, tax debt, guild sanction, arrest record.

Economic traces: price change, unpaid invoice, missing stock, changed route, wage promise.

Erasure traces: cleaned blood, forged record, burned notice, intimidated witness, moved corpse, planted evidence, repaired lock.

Erasure is not absence. It is an event.

## Speech acts

Conversation must be typed, not just prose.

```yaml
SpeechAct:
  kind: Report
  speaker: actor_tomas
  listener: actor_anna_clerk
  propositions:
    - ItemMissing(coin_stack_01)
    - LastSeenIn(strongbox_tomas, yesterday_evening)
  confidence: 0.78
  source_beliefs:
    - belief_tomas_missing_coins
  intention: request_authority_help
```

Speech act kinds: tell, ask, answer, report, accuse, deny, lie, confess, promise, threaten, bargain, recruit, refuse, warn, gossip, instruct, command, testify, joke, deflect, withhold.

## Lies

A lie is an intentional communication that the speaker does not believe, or does not believe at asserted confidence.

```yaml
SpeechAct:
  kind: Lie
  speaker: actor_mara
  listener: actor_elias
  asserted: MaraWasHomeAllNight
  speaker_belief_confidence: 0.05
  intended_effect: reduce_suspicion
```

Lies create risk: contradiction, trust damage, blackmail, guilt, sanction, and future defensive behavior.

## Rumors

Rumors mutate through omission, exaggeration, identity substitution, motive invention, confidence inflation, moral framing, status bias, and stereotype compression. A rumor can become more socially powerful while becoming less accurate.

## Evidence

Evidence is an observed trace or record that supports or contradicts propositions. Evidence is not truth.

```yaml
Evidence:
  trace: trace_lock_scratch_01
  supports:
    - ContainerWasForcedOpen: 0.65
  contradicts:
    - OwnerOpenedNormally: 0.25
```

Institutions have evidence thresholds for opening cases, questioning suspects, searching homes, arresting, and convicting.

## Stale information

Track event time, acquisition time, last verification, confidence decay, update channels, and who has access to updates. Stale notices, old maps, dead suspects, already-looted sites, and moved threats are central gameplay.

## Conversation with the player

If an NPC tells the player-controlled actor about treasure:

1. NPC has belief or chooses to lie.
2. Speech act is generated.
3. LLM may render wording.
4. System validates structured propositions.
5. Current actor receives observation from speech.
6. Player notebook records what the current actor heard.

The LLM’s prose does not create treasure. It only communicates, distorts, conceals, or lies about propositions.

## Minimal model

Implement direct perception, overheard sound, speech report, written notice, missing-item trace, disturbed-container trace, belief confidence/source, stale notice, rumor chain up to three hops, lie speech act, institutional case file, and conversation log with structured claims.
