# Epistemic Model and Information Flow

## Core claim

Tracewake is epistemic. Agents do not act from truth. They act from what they believe, remember, infer, read, hear, mishear, misremember, and trust.

A world where NPCs secretly read the authoritative state is not Tracewake, no matter how sophisticated the event system appears.

## Three separated layers

Tracewake must keep three layers distinct:

1. **Ground truth** — what the authoritative simulation says happened.
2. **Subjective belief** — what each actor, household, institution, or group believes.
3. **Public/institutional record** — what artifacts, ledgers, notices, reports, case files, receipts, orders, and archives claim.

These layers may agree. They often should not.

A record can be false. A witness can be confident and wrong. An actor can know the truth but fail to prove it. An institution can believe a lie. A notice can remain stale. A household can maintain private knowledge that never reaches the office. Ground truth does not correct these layers automatically.

## Belief proposition doctrine

Important beliefs should be represented as propositions with at least the following foundation-level fields:

- holder: actor, household, institution, office, rumor network, or other belief-holding entity;
- content: typed claim, not raw prose authority;
- subject: entity, place, event, object, norm, relationship, identity, time, or condition referenced;
- confidence and uncertainty;
- source: who or what supplied the belief;
- channel: perception, testimony, rumor, record, reading, inference, instruction, memory, institutional notification, etc.;
- acquisition time;
- believed event time when available;
- staleness status or decay model;
- provenance chain back to events, records, or speech acts where possible;
- contradiction links to incompatible beliefs or records;
- identity uncertainty links when relevant;
- scope: private, household-known, office-known, public, restricted, or rumored;
- action implications if any.

Raw prose may render a belief. It does not define the authoritative proposition.

## Information channels

Information reaches agents only through modeled channels. Valid channels include:

- direct perception;
- indirect perception, such as hearing noise through a wall;
- search and inspection;
- expectation contradiction;
- testimony;
- gossip and rumor;
- reading notices, ledgers, letters, contracts, reports, signs, or case files;
- institutional notification;
- household communication;
- instruction, command, or training;
- inference;
- memory retrieval;
- memory distortion;
- social interpretation;
- domain-defined special channels, but only in future domain packs and only with costs, reliability, provenance, traces, and failure modes.

No telepathy. No global awareness. No automatic update because the player knows.

## Perception

Perception is fallible.

A perception event should consider:

- sensory channel;
- distance, lighting, obstruction, noise, fatigue, intoxication, attention, stress, skill, familiarity, and expectation;
- actor state and body constraints;
- environmental conditions;
- salience;
- identity ambiguity;
- whether the actor perceived a fact, a trace, a possibility, or only a vague anomaly;
- confidence;
- what proposition entered memory or belief;
- whether the perception was noticed enough to matter.

A witness hearing a sound at night should not gain “Mara stole the coins.” The witness may gain “heard a dull sound near the miller’s room around late night, low confidence.” Later testimony can distort that into something stronger or wrong.

## Expectation contradiction

Agents discover absence or anomaly through contradiction between expectation and perception/search.

Examples:

- Tomas believes his coins are in the strongbox. He opens it and sees they are absent.
- A clerk expects a ledger in the office chest. It is not there.
- A spouse expects Mara home at dawn. Mara is absent.
- A tavern keeper expects payment after service. Payment is not made.
- A worker expects the shop open during office hours. It is closed.

The contradiction itself is eventful. It can create suspicion, search intentions, reports, accusations, rumors, or revised memories.

## Search

Search is intentional, costly, bounded, and fallible.

A search process should have:

- searcher and motive;
- target belief or question;
- search area;
- method;
- time cost;
- access constraints;
- skill/attention/fatigue modifiers;
- traces inspected;
- possible false negatives;
- possible irrelevant discoveries;
- memory and record effects;
- interruption points;
- failure events.

The TUI may help the human understand where the current actor might search. It may not make the current actor search places or facts the actor has no reason or ability to consider.

## Testimony, rumor, and gossip

Speech transmits claims, not truth.

A testimony or rumor event should preserve:

- speaker;
- listener or audience;
- speech act type;
- claims made;
- speaker’s belief state or disbelief when relevant;
- motive, credibility, relationship, authority, fear, risk, or bias;
- confidence expressed;
- whether the listener accepts, rejects, doubts, ignores, or misremembers it;
- provenance and channel labels;
- future contradiction potential.

Rumor may mutate. A low-confidence observation can become a confident false accusation after several retellings. That mutation is not a bug; it is one of the central pleasures of Tracewake.

## Lies

Lying is supported early because belief fallibility is central.

A lie is not magic. It is a structured speech act containing a claim the speaker does not believe or has reason to doubt.

A lie requires:

- a speaker with motive;
- a claim;
- a listener/audience;
- a risk model;
- credibility factors;
- possible evidence or contradiction;
- memory and provenance effects;
- consequences if believed, doubted, exposed, or forgotten.

Embodied mode must not label a statement as a lie unless the current actor has a modeled reason. Debug mode may reveal speaker belief, motive, and contradiction.

## Records as information channels

Records are not truth. Records are artifacts containing claims.

Reading a record creates beliefs only if the actor can access, read, interpret, and trust or at least understand the record. A record may be stale, false, forged, misfiled, damaged, biased, incomplete, contradicted, or inaccessible.

A record’s claim can travel through society:

```text
report -> office case file -> notice board -> gossip -> household suspicion -> accusation
```

Every step can distort, omit, strengthen, weaken, or contradict the claim.

## Memory fallibility

Full memory fallibility is foundation doctrine even if staged gradually.

Memory mechanics include:

- formation;
- retrieval;
- decay;
- forgetting;
- distortion;
- source confusion;
- confidence drift;
- temporal uncertainty;
- identity uncertainty;
- emotional salience;
- rehearsal through retelling;
- correction through new information;
- resistance to correction;
- contradiction links.

Do not design early data structures that assume perfect, permanent, globally indexed memory.

## Misidentification and identity uncertainty

Identity is often uncertain. The engine must support:

- unknown actor descriptions;
- partial identity claims;
- clothing-based recognition;
- voice recognition;
- social familiarity;
- mistaken identity;
- aliases and roles;
- household/institutional identifiers;
- confidence-ranked candidates;
- contradictory identity claims.

A witness may believe “a tall worker with a green cloak left the mill,” not “Mara did it.” Later inference may narrow or mislead.

## Actor knowledge versus human knowledge

Normal play shows the current actor’s known/perceived world.

The distinction is mandatory:

- **Actor notebook / actor-known leads**: available in embodied play; bounded by actor knowledge; may support actor speech, reports, search, accusations, and plans.
- **Human/debug notes**: optional out-of-world UI convenience; visibly separate; never a source for speech acts, testimony, accusations, reports, persuasion, institutional claims, or action preconditions.
- **Debug inspector**: may reveal truth, causal graphs, all beliefs, planner state, hidden traces, random draws, and human/actor knowledge mismatch diagnostics; visibly non-diegetic.

The human may remember facts learned while possessing other actors. The current actor may not act world-affectingly from those facts unless they have reached that actor through modeled channels.

## UI assistance boundary

The TUI may be kinder than the world. It may not be privileged over the world.

Allowed assistance:

- sorting actor-known facts;
- grouping source-bound leads;
- showing why an actor cannot make a report;
- highlighting contradictions the actor has already noticed;
- filtering known records;
- suggesting actor-possible next actions;
- explaining uncertainty labels;
- showing debug-only truth in debug mode.

Forbidden assistance:

- objective culprit labels in embodied mode;
- omniscient quest markers;
- global notebooks that act as evidence;
- automatic institution recognition from human knowledge;
- speech options based on facts unknown to the current actor;
- hidden truth search results;
- progress bars tied to ground truth.

## Epistemic acceptance for the first miracle

The missing-property scenario is acceptable only if debug can distinguish:

- what happened in ground truth;
- what the thief believed before acting;
- what the victim expected;
- when the victim discovered absence;
- what each witness perceived, if anything;
- which beliefs formed from direct perception, inference, testimony, rumor, reading, or memory;
- where each belief’s confidence came from;
- which records were created and from what claims;
- why wrong suspicion arose;
- what remains stale;
- what the human knows from possession/debug;
- what the current actor is allowed to know.

If the simulation can only answer “the theft quest state advanced,” it has failed.
