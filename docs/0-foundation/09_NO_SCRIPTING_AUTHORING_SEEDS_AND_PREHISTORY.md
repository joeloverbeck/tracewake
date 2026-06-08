# No Scripting, Authoring, Seeds, and Prehistory

## Core claim

Tracewake forbids authored outcome chains.

Tracewake requires authored causal machinery.

```text
Authored causal machinery is allowed.
Authored outcome chains are forbidden.
```

Designers author the rules, affordances, pressures, institutions, records, traces, speech acts, regional processes, and initial conditions that make events possible. They do not author guaranteed story beats.

## Forbidden runtime scripting

The authoritative simulation must not contain:

- quest beats;
- scenario scripts;
- drama directors;
- pacing managers;
- boredom detectors;
- hidden “when player does X, cause Y” logic;
- NPC behavior that exists only to serve the player;
- player-specific suspicion, forgiveness, protection, hostility, recognition, or reward;
- content that waits indefinitely for the player;
- objective markers pointing to ground truth;
- quest state overriding simulation state;
- reward spawns independent of promise, custody, funds, procedure, and verification;
- automatic culprit reveals;
- automatic stale-information correction because the player arrived;
- dialogue that creates hidden facts because it sounds good;
- LLM-generated facts accepted as authoritative reality;
- regional events injected because the local story needs stakes.

Do not hide scripts behind prettier names. A “scenario controller,” “questless quest,” “content director,” “emergent plot manager,” “narrative scheduler,” or “salience injector” is still forbidden if it mutates authoritative state for dramatic progression rather than modeled causality.

## Allowed and required authoring

Designers may and must author causal material:

- primitive actions;
- object/place/person affordances;
- needs and pressures;
- traits, motives, values, role defaults, and relationship defaults;
- norms, laws, permissions, prohibitions, obligations, sanctions, powers, proof rules, and procedures;
- HTN methods and routine templates;
- institution procedures;
- household procedures;
- domain rules;
- initial world state;
- scenario seeds;
- authored prehistory;
- generated prehistory rules;
- trace types;
- event schemas;
- claim/proposition vocabulary;
- speech-act templates;
- perception models;
- memory models;
- LOD rules;
- regional process models;
- domain packs;
- test fixtures;
- data validation rules;
- UI labels and deterministic prose templates.

These define possibility space. They must not guarantee outcome chains.

## Causal machinery versus script

Allowed:

```text
RecoverMissingProperty(item)
  Method: SearchLastKnownPlace
    Preconditions:
      actor believes item was last stored there
      actor can reach place or is willing to trespass
    Failure:
      inaccessible place
      item absent
      search too shallow
      actor interrupted
      actor misidentifies object
    Traces:
      search disturbance
      witness observation
      actor belief update

  Method: AskHousehold
    Preconditions:
      reachable household members
      sufficient relationship, authority, confidence, or desperation
    Failure:
      refusal
      lie
      absence
      misunderstanding
      social cost too high

  Method: ReportToAuthority
    Preconditions:
      authority reachable
      office open or valid exception
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

Scenario seeds are allowed. They may establish initial state, tensions, relationships, resources, beliefs, routines, debts, rumors, institutions, records, traces, weather, regional context, and unresolved incidents.

Allowed seed:

```text
Mara begins hungry, indebted, afraid of losing work, and aware through gossip that Tomas stores coins in a strongbox.
Tomas believes the coins are in the strongbox because he placed or checked them previously.
Elena sleeps nearby and might hear a noise depending on sleep state, distance, and event volume.
The clerk's office has limited hours and a small fund.
The household has private/shared storage norms.
```

Forbidden seed:

```text
When the player checks the strongbox, Mara becomes the suspect.
When the player reads the notice, the culprit stays nearby.
If the player is bored, trigger a second theft.
If the player accuses the wrong person, spawn a corrective clue.
```

Seeds create conditions. They do not promise arcs.

## Authored prehistory

Scenario authors may seed old artifacts, old records, unresolved accusations, long-past events, inherited property disputes, buried traces, stale notices, remembered rumors, migration histories, institutional succession, and boundary conditions.

These must be marked with structured provenance:

```text
authored_initial_state
  created directly by scenario author as starting condition

authored_prehistory_event
  long-past event declared as pre-simulation cause

generated_prehistory_summary
  output of pre-simulation world/history generator or summary process

declared_boundary_input
  exogenous input from outside the simulated local region
```

Prehistory may be compact. It may not be ungrounded lore prose when it affects gameplay.

An old record should say who authored/issued it in-world if known, what claims it carries, when it claims to have been made, where it is, who can access it, and what provenance status it has.

## Boundary conditions

Boundary conditions are allowed when the local simulation is not yet modeling the full outside world.

Examples:

- a regional tax order arrives;
- a storm front moves in;
- a caravan is expected;
- refugees carry a rumor;
- disease pressure increases;
- a road is closed outside town;
- distant war changes trade;
- animal migration affects crops.

Each boundary input must declare source, cadence/trigger, scope, input state, random model if any, delivery channel, local entry event, traces, affected claims/records/beliefs, causal ancestry, and replay/debug visibility.

Boundary input is not permission to inject story beats.

## Public artifacts are not quests

Tracewake may contain:

- notices;
- work postings;
- contracts;
- debts;
- promises;
- rewards;
- reports;
- investigations;
- accusations;
- obligations;
- rumors;
- leads;
- institutional procedures.

A public artifact is not a quest.

A notice may be stale, forged, ignored, removed, contradicted, paid, unpaid, misread, amended, destroyed, solved by someone else, or based on partial information.

A notice does not imply target existence, target location, issuer honesty, reward availability, player ownership, world waiting, or automatic completion recognition.

## Rewards and recognition

Rewards are not completion flags.

Payment requires modeled conditions such as:

- public promise or private contract;
- issuer identity;
- funds;
- custody of funds;
- authority to pay;
- proof requirement;
- verification procedure;
- institutional belief;
- willingness to pay;
- absence or presence of dispute;
- delay, corruption, fear, or refusal.

An actor may do the work and not be paid. An actor may lie and be paid. An institution may pay the wrong person. A promised reward may be stale. A fund may be empty. These are valid outcomes if causally explained.

## Anti-radiant doctrine

Tracewake rejects the radiant-quest failure mode.

Forbidden:

```text
when player accepts notice, spawn target
objective marker points to true target
quest giver waits forever
reward appears on completion
NPC knows player completed task because quest flag changed
world freezes relevant situation until player arrives
```

Allowed:

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

Tracewake rejects storyteller causation as a kernel principle.

Allowed observer layers:

- salience detection;
- story sifting;
- recaps;
- notebooks;
- event clustering;
- debug summaries;
- diagnostic pacing analysis;
- actor-filtered summaries after events occur.

Forbidden runtime causes:

- betrayal injected for drama;
- storm triggered because play is quiet;
- theft spawned because the human needs a case;
- death delayed because it would be inconvenient;
- clue created because investigation stalled;
- witness timed because pacing demands it;
- region escalation because local story needs stakes.

Diagnostic analysis may help designers revise causal machinery later. It must not mutate authoritative state at runtime.

## LLM boundary

LLMs may render, paraphrase, summarize, or parse candidate speech acts behind validation. They may not author runtime facts, decide truth, repair plots, create hidden evidence, create quests, grant knowledge, plan agents, or mutate state.

An LLM-generated utterance that appears to share information must be mapped by a deterministic or validated extraction layer to candidate typed claims with source belief references. Belief transfer occurs only through committed speech/perception events.

## Future adventure content

Adventure-like content must be ordinary causal expansion.

A road threat begins as modeled danger, travel risk, reports, rumor, traces, institutional limits, and public artifacts. It does not begin as “spawn bandits for player.”

A companion joins through relationship, belief, need, trust, loyalty, payment, fear, obligation, persuasion, and refusal. It does not join because party slot is empty.

A settlement is conquered through logistics, command, fear, morale, violence, supplies, records, rumor, diplomacy, territorial control, institutional collapse, and consequences. It is not a conquest quest.

## Authoring review checklist

Every authored element must answer:

```text
What causal machinery does this define?
What state does it require?
What beliefs or claims does it require?
What resources, norms, risks, costs, or procedures matter?
What can interrupt or invalidate it?
What traces can it leave?
Who can observe it?
Who can misunderstand it?
Which records, households, norms, or institutions care?
Can NPCs use or suffer it?
Can the player use it only through an ordinary actor?
Can it fail?
Can it become stale?
Does it guarantee an outcome?
Does it smuggle in drama direction, quest state, player privilege, genre assumptions, or LLM authority?
```

If it guarantees an outcome, remove it or convert it into causal machinery.

## 2026 hardening: no authored causal chains disguised as systems

The no-scripting rule applies to hidden truth, institutions, routines, LOD summaries, notices, bounties, and language surfaces.

Forbidden:

```text
spawn bounty because the plot needs a monster hunt
make the guard suspect the true culprit because the theft happened
send adventurers because the notice is a quest
move a monster because the player needs danger
make a caravan survivor report because a scenario chain requires a report
choose food/work/sleep actions from raw state and explain them as routine behavior afterward
```

Allowed:

```text
a monster migrates because territory pressure, injury, hunger, weather, conflict, fear, or regional process events move it
a caravan is attacked because route, timing, threat behavior, preparedness, and chance align
a survivor reports because they survive, know a receiver, judge reporting useful, and can travel or speak
an authority posts a notice because a report/procedure/resource threshold leads to that decision
travelers act on a notice because they read it, believe or doubt it, have motives/resources, and decide to pursue it
another actor may already have killed the monster because the world continued causally
```

The famous monster/bandits/caravan/bounty/adventurers chain is a diagnostic example of future emergent richness, not an authored acceptance script. Every link in such a chain must be independently optional, interruptible, misinformed, delayed, contradicted, or fail-able.

## Seeds, fixtures, and prehistory

Seeds may establish possibility space: entities, places, relationships, memories, records, routine assignments, tensions, histories, debts, injuries, territories, institutional roles, rumors, resources, and environmental pressures.

Seeds must not establish hidden outcome obligations. A seed may say "there is a posted notice," "a hunter believes a beast moved north," or "the court record contains a false accusation." It must not say "the player will find the beast," "the thief will be revealed," or "the town will post a bounty after three days unless the player intervenes."

Authored starting knowledge must still have provenance: prehistory event, memory source, record source, inherited institutional ledger, rumor source class, or explicit fixture setup marker. The marker may be abstract, but it cannot be absent.

## Anti-contamination authoring review

Content review must reject:

- hidden objective states;
- guaranteed causal chains;
- culprit-truth fields consumed by ordinary cognition;
- notice boards acting as task menus;
- rewards spawned without institutional resource/payment process;
- records without source;
- routines without failure modes;
- regional events without summary ancestry;
- LLM prompts that include hidden truth as narrative context for ordinary speech;
- fixtures whose tests pass because debug truth is accidentally available.
