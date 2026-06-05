# No Scripting and Causal Authoring Policy

## Core claim

Tracewake forbids authored outcome chains.

Tracewake requires authored causal machinery.

This distinction is non-negotiable:

```text
Authored causal machinery is allowed.
Authored outcome chains are forbidden.
```

Designers author the rules, affordances, pressures, procedures, traces, beliefs, records, and initial conditions that make events possible. Designers do not author guaranteed story beats.

## Forbidden scripting

The authoritative simulation must not contain:

- quest beats;
- scenario scripts;
- drama directors;
- pacing managers;
- boredom detectors;
- hidden “when player does X, cause Y” logic;
- NPC behavior that exists only to serve the player;
- player-specific suspicion, forgiveness, protection, hostility, or recognition;
- content that waits indefinitely for the player;
- objective markers pointing to ground truth;
- quest state overriding simulation state;
- reward spawns independent of custody, promise, funds, procedure, and verification;
- authored success or failure outcomes independent of world state;
- automatic culprit reveals;
- automatic stale-information correction because the player arrived;
- dialogue that creates hidden facts because it sounds good;
- LLM-generated facts accepted as authoritative reality;
- region events injected because the local story needs stakes.

Do not hide scripts inside attractive names. A “questless quest,” “scenario controller,” “narrative event injector,” “content director,” or “emergent plot manager” is still forbidden if it mutates authoritative state for dramatic progression rather than modeled causality.

## Allowed and required authoring

Designers may and must author causal materials:

- primitive actions;
- object affordances;
- needs and pressures;
- traits, motives, values, role defaults, and relationship defaults;
- norms, laws, permissions, prohibitions, obligations, sanctions, powers, and procedures;
- HTN methods and routine templates;
- institution procedures;
- domain rules;
- initial world state;
- scenario seeds;
- trace types;
- event schemas;
- belief proposition vocabulary;
- speech-act templates;
- perception models;
- memory models;
- LOD rules;
- regional process models;
- domain packs;
- test fixtures;
- data validation rules;
- UI labels and prose templates.

These create possibility space. They must not guarantee an outcome chain.

## The HTN distinction

HTN methods are allowed when they are state-dependent procedures with:

- preconditions;
- required beliefs or information;
- costs;
- durations;
- resources;
- permissions and prohibitions;
- failure modes;
- interruption points;
- alternative methods;
- trace generation;
- observation hooks;
- institutional consequences;
- causal links.

Allowed:

```text
RecoverMissingProperty(item)
  Method: SearchLastKnownPlace
    Preconditions:
      actor believes item was last stored there
      actor can reach or is willing to trespass
    Failure:
      place inaccessible
      item absent
      search too shallow
      actor interrupted
      actor misidentifies object
    Traces:
      search disturbance
      witness observation
      changed actor belief

  Method: AskHousehold
    Preconditions:
      household members reachable
      relationship/authority/confidence sufficient
    Failure:
      refusal
      lie
      absence
      misunderstanding
      social cost too high

  Method: ReportToAuthority
    Preconditions:
      authority reachable
      office open or emergency exception
      actor believes report worth cost
    Failure:
      report refused
      delayed
      misfiled
      credibility too low
      clerk absent
```

Forbidden:

```text
RecoverMissingProperty(item)
  Step 1: player finds clue
  Step 2: culprit flees when confronted
  Step 3: guard arrives
  Step 4: reward paid
```

The first is causal machinery. The second is a quest script.

## Scenario seeds

Scenario seeds are allowed. They may set initial conditions, pressures, relationships, possessions, routines, debts, rumors, institutions, records, traces, weather state, regional context, and unresolved incidents.

Allowed seed:

```text
Mara begins hungry, indebted, afraid of losing work, and aware that Tomas stores coins in a strongbox.
Tomas believes the coins are in the strongbox.
Elena sleeps nearby and might hear noise depending on sleep state, distance, and event volume.
The clerk's office has limited hours and a small fund.
The household has private and shared storage norms.
```

Forbidden seed:

```text
When the player checks the strongbox, Mara must become the suspect.
When the player reads the notice, the culprit must still be nearby.
If the player is bored, a second theft occurs.
If the player accuses the wrong person, spawn a clue to correct them.
```

Seeds may set up tensions. They may not promise dramatic arcs.

## Public artifacts are not quests

Tracewake may contain:

- notices;
- public work postings;
- contracts;
- promises;
- rewards;
- reports;
- investigations;
- debts;
- obligations;
- rumors;
- leads;
- institutional procedures.

Tracewake must not contain quest as primary ontology.

A notice is a world artifact. It may be stale, forged, ignored, removed, contradicted, paid, unpaid, misread, amended, destroyed, or solved by someone else.

A notice does not imply:

- a target exists now;
- the claim is true;
- the location is correct;
- the issuer is honest;
- the reward fund exists;
- the institution will recognize completion;
- the player is the intended solver;
- the world will wait.

## Rewards and recognition

Rewards are not completion flags.

Payment requires modeled conditions such as:

- promise or public offer;
- issuer identity;
- available funds;
- custody of those funds;
- proof requirement;
- verification procedure;
- institutional belief;
- authority to pay;
- willingness to pay;
- ability to pay;
- absence of dispute, corruption, delay, or refusal.

An actor may do the work and not be paid. An actor may lie and be paid. An institution may pay the wrong person. A promised reward may be stale. A fund may be empty. These are not bugs when causally explained.

## Anti-radiant doctrine

Tracewake explicitly rejects the Skyrim-style radiant-quest failure mode.

Forbidden patterns:

```text
when player accepts notice, spawn target
objective marker points to true target
quest giver waits forever
reward appears on completion
NPC knows player completed task because quest flag changed
world freezes relevant situation until player arrives
```

Allowed patterns:

```text
notice exists because an institution or actor posted it
claim has source and age
target may move, die, be misidentified, or never have existed
other actors may act on the notice
issuer may lack funds
proof may be contested
institution recognizes outcome only through modeled evidence and procedure
```

## Anti-storyteller doctrine

Tracewake explicitly rejects RimWorld-style storyteller causation as a kernel principle.

The engine may have:

- salience detection;
- story sifting;
- recaps;
- notebooks;
- event clustering;
- debug summaries;
- diagnostic pacing analysis.

These are observer layers. They may highlight what happened. They may not cause what happens.

Forbidden:

- betrayal injected for drama;
- storm triggered because play is quiet;
- theft spawned because the human needs a case;
- death delayed because it would be inconvenient;
- clue created because investigation stalled;
- witness timed because pacing demands it.

Diagnostic pacing analysis may be used by designers to revise causal machinery later. It must not mutate authoritative state at runtime.

## Regional events are causal processes, not beats

Regional and exogenous events are allowed only as declared causal processes.

Allowed:

```text
A storm delays travel because the weather process changed road cost.
A caravan arrives because a route schedule, stock state, risk model, and regional boundary emitted arrival.
A tax order reaches town because outside authority process created a record and courier channel delivered it.
A disease pressure rises because regional illness state and contact network made infection possible.
A road closes because flood damage event changed route affordance.
```

Forbidden:

```text
Storm starts when the player leaves town because the scene needs tension.
Caravan arrives because the player has not seen commerce lately.
Tax order appears because the story needs stakes.
Disease begins because the village has been peaceful too long.
Road closes because the player needs a detour.
```

## LLM boundary

LLMs may render language. They may parse proposed language into candidate structured actions. They may summarize source-bound state. They may paraphrase records.

LLMs may not:

- decide truth;
- mutate authoritative state;
- create hidden evidence;
- invent facts;
- plan agents;
- create quests;
- grant knowledge;
- bypass speech-act validation;
- repair plots;
- decide guilt;
- create institutional recognition.

Any state change implied by language must pass through typed validation and ordinary action/event pipelines.

Core mechanics must work with LLMs disabled.

## Review checklist

Every authored element must answer:

```text
What causal machinery does this define?
What state does it require?
What beliefs does it require?
What resources, norms, risks, or costs matter?
What can interrupt or invalidate it?
What traces can it leave?
Who can observe it?
Who can misunderstand it?
Which records, norms, or institutions care?
Can NPCs use it?
Can the player use it only through an ordinary actor?
Can it fail?
Can it become stale?
Does it guarantee an outcome?
Does it smuggle in drama direction, quest state, player privilege, genre assumptions, or LLM authority?
```

If it guarantees an outcome, remove it or convert it into causal machinery.
