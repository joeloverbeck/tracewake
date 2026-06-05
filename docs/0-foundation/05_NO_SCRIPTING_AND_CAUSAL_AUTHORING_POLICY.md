# No Scripting and Causal Authoring Policy

This policy is the worked-policy expansion of `INV-038` (no authored outcome chains), drawing on the questless-content and story-sifting rules `INV-035`–`INV-037` and `INV-039`, the trace rule `INV-013`, and the LLM boundary `INV-050`–`INV-051`. The invariants are the testable summary; this document is the detailed obligation. Keep them in sync.

## Core claim

Tracewake forbids authored event sequences. Tracewake requires authored causal machinery.

This distinction is non-negotiable:

```text
Authored causal machinery is allowed.
Authored outcome chains are forbidden.
```

## Forbidden scripting

The following are not allowed in the authoritative simulation:

- authored outcome chains;
- quest beats;
- scenario scripts;
- drama directors;
- boredom detectors;
- hidden “when player does X, cause Y dramatic event” logic;
- NPC behavior that exists only to serve the player;
- content that waits indefinitely for the player;
- objective markers pointing to ground truth;
- quest state that overrides simulation state;
- reward spawns independent of custody and verification;
- authored success/failure outcomes independent of state;
- player-specific suspicion, forgiveness, or institutional recognition;
- dialogue that creates hidden facts because it sounds good;
- LLM-generated events accepted as truth;
- automatic correction of stale information because the player arrived.

## Allowed and required authoring

Designers must author:

- primitive actions;
- object affordances;
- needs and pressures;
- traits, motives, roles, and value defaults;
- norms, laws, permissions, prohibitions, obligations, sanctions, and roles;
- HTN methods and routine templates;
- institution procedures;
- domain rules;
- initial world state;
- scenario seeds;
- trace types;
- belief proposition vocabulary;
- speech-act templates;
- LOD rules;
- domain packs;
- test fixtures;
- data validation rules;
- UI view model labels and templates;
- prose templates for records/notices/dialogue surfaces.

These are causal materials. They create possibility space, not guaranteed plots.

## The HTN distinction

HTN methods are allowed when they are state-dependent procedures with:

- preconditions;
- required beliefs or information;
- costs;
- durations;
- resources;
- permissions/prohibitions;
- failure modes;
- interruption points;
- alternative methods;
- trace generation;
- observation hooks;
- institutional consequences;
- causal links.

HTN methods are forbidden when they secretly encode a predetermined story.

Allowed:

```text
RecoverMissingProperty(item)
  Method: SearchLastKnownPlace
    Preconditions: actor knows last known place; access possible or actor willing to trespass
    Failure: place inaccessible, item absent, actor interrupted, search too poor
  Method: AskHousehold
    Preconditions: household members reachable; relationship or authority sufficient
    Failure: refusal, lie, absence, misunderstanding
  Method: ReportToAuthority
    Preconditions: authority reachable; office open or emergency exception
    Failure: report refused, delayed, lost, low credibility
```

Forbidden:

```text
RecoverMissingProperty(item)
  Step 1: player finds clue
  Step 2: culprit flees when confronted
  Step 3: guard arrives
  Step 4: reward paid
```

## Scenario seeds

Scenario seeds are allowed. They may set initial state, pressures, relationships, possessions, rumors, institutions, records, and unresolved incidents.

A seed may say:

```text
Mara begins desperate, indebted, hungry, and aware that Tomas stores coins in a strongbox.
Tomas believes the coins are in the strongbox.
Elena sleeps in a nearby room and can possibly hear noise.
The reeve's office has limited funds and a clerk with office hours.
```

A seed may not say:

```text
When the player checks the strongbox, Mara must be suspected.
When the player reads the notice, the bandits must still be at the quarry.
If the player is bored, a second theft occurs.
```

## Domain events vs scripted beats

A domain event is allowed if it arises from causal process.

Allowed:

```text
A storm delays travel because weather process changed road cost.
A hungry wolf enters pasture because food scarcity and territory drove movement.
A tax order arrives because outside authority boundary process emitted an order.
A guard questions the wrong person because rumor and bias raised suspicion.
```

Forbidden:

```text
Storm starts when the player leaves town because scene needs tension.
Wolf attacks because the player accepted a notice.
Tax order arrives because the story needs stakes.
Guard questions the wrong person because the mystery needs a red herring.
```

## Public artifacts are not quests

Notices, contracts, rumors, reports, and requests are world artifacts. They may be acted on by any agent with access, motivation, and ability. They can be ignored, destroyed, forged, updated, contradicted, or stale.

A notice does not imply:

- the target exists now;
- the reward will be paid;
- the location is true;
- the player is intended solver;
- the world will wait;
- completion will be recognized.

## Leads are UI projections

A lead card may collect claims and suggest actor-possible actions. It may not create objective progress state.

Good:

```text
Posted Notice: Road threat reported
Source: market notice board
Issuer: reeve's office
Claim: travelers report attacks near old quarry
Last verified: two days ago
Reward: 80 coins if proof satisfies the reeve
```

Bad:

```text
Quest: Kill bandits at old quarry
Objective marker: bandit leader
Reward: 80 coins
```

## Story sifting boundary

Story sifting may identify patterns after they occur:

- “the same rumor now has three variants”;
- “Mara's theft chain caused Tomas's report”;
- “Elias suspects the wrong person because Elena's uncertain testimony mutated”;
- “the notice is stale because the threat moved.”

Story sifting may not cause:

- a betrayal for drama;
- a clue for pacing;
- an interruption because a quiet period lasted too long;
- a culprit reveal;
- a conveniently timed witness.

## Review checklist

Every authored element must answer:

```text
What causal machinery does this define?
What state does it require?
What state can interrupt or invalidate it?
What traces can it leave?
Who can observe it?
Who can misunderstand it?
Which norms or institutions care?
Can NPCs use it?
Can the player use it only through an ordinary actor?
Can it fail?
Can it become stale?
Does it guarantee an outcome?
```

If it guarantees an outcome, remove it or convert it into causal machinery.

## Strong rule

Do not hide scripts inside nicer names. A “scenario controller,” “content director,” “questless quest,” “pacing manager,” or “narrative event injector” is still a script if it causes events for dramatic progression rather than modeled causality.
