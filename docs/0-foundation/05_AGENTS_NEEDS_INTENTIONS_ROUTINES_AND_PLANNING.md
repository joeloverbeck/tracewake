# Agents, Needs, Intentions, Routines, and Planning

## Core claim

Tracewake agents are ordinary, symbolic, inspectable, bounded, and fallible.

They are not LLM puppets, quest actors, schedule robots, utility-meter puppets, or omniscient optimizers. They are people-like agents who believe things, need things, value things, commit to projects, follow routines, violate norms, fail, misread the world, and leave evidence when they act.

## Required mental separation

Tracewake uses a BDI-style separation as foundation doctrine:

```text
Beliefs
  subjective, sourced, stale, confidence-bearing claims about the world

Desires / values / goals / needs / duties
  pressures, motives, projects, role obligations, relationships, fears, habits, values, and preferences

Intentions
  durable commitments and selected methods the agent is currently trying to carry out
```

This separation prevents three major failures:

- agents that act from ground truth because “AI needs to know”; 
- agents that jitter between meter refills because utility scores are the whole mind;
- agents that follow schedules like rail tracks instead of defeasible intentions.

## Beliefs

Beliefs are subjective stances toward typed claims. They are source-bound, confidence-bearing, contradiction-capable, stale unless updated, memory-affected, and debug-inspectable.

Agents choose actions, speech, search, suspicion, reporting, cooperation, avoidance, buying, stealing, and institutional behavior from beliefs, not ground truth.

A thief may act on a false belief that coins are still in a strongbox. A clerk may act on a false report. A witness may confidently misidentify clothing. A victim may suspect the wrong debtor.

## Needs, values, duties, and motives

V1 needs simple but real ordinary motives, not a grand psychology model.

Early agents should have enough of the following to support ordinary life:

- hunger and food access;
- fatigue and sleep pressure;
- safety pressure;
- money pressure;
- role duties;
- household obligations;
- work obligations;
- trust/distrust;
- relationships;
- debt, shame, fear, loyalty, affection, greed, duty, pride, kindness, resentment;
- risk tolerance and social caution;
- modest personality/value traits;
- durable projects such as earn wages, feed household, recover property, repay debt, preserve reputation, hide wrongdoing, protect kin, avoid suspicion.

Needs are pressures. They are not puppet strings. A hungry honest worker and a hungry desperate debtor should not behave identically.

## Intentions and projects

Intentions prevent jitter.

An intention is a durable commitment such as:

- go to work;
- acquire food;
- sleep;
- recover missing property;
- hide stolen coins;
- avoid a suspicious actor;
- report to the clerk;
- repair a tool;
- pay debt;
- keep quiet;
- spread a rumor;
- search a room;
- protect household member;
- finish current delivery.

Intentions persist across small score changes. They are interruptible by stronger modeled causes: hunger, threat, fatigue, social command, blocked access, new evidence, contradiction, obligation, institutional procedure, weather, injury, fear, or plan failure.

Adopting, suspending, resuming, abandoning, or completing an important intention is eventful when it matters.

## Durable projects

A project is an intention-scale structure that may last hours, days, or longer.

Examples:

```text
Earn enough wages this week.
Conceal my role in moving the coins.
Find proof that Rafi took the tool.
Repair household reputation.
Keep Mara fed without exposing debt.
Get a report accepted by the clerk.
```

Projects stabilize behavior and allow ordinary long causal arcs without story scripts. They must still operate through beliefs, methods, costs, interruptions, and failure.

## HTN-style methods

HTN-style methods are the right foundation tool for routines, domestic behavior, job procedures, searches, reports, institution workflows, travel preparation, and future organization procedures.

A valid method defines:

```text
purpose
preconditions based on belief and accessible state
required resources, tools, time, location, access, and permissions
norms or willingness to violate norms
expected traces and observations
possible records or speech acts
failure modes
interruption points
alternative methods
debug explanation hooks
```

Good HTN domains:

- morning routine;
- eat from household storage;
- buy food from tavern/shop;
- go to work;
- perform job shift;
- store wages;
- search last-known place;
- ask household member;
- report missing property;
- receive report at office;
- post notice;
- retrieve stored item;
- hide item;
- lie when questioned;
- repair or replace tool;
- travel to nearby place;
- future recruit, command, patrol, negotiate, treat wound, or manage caravan procedures.

Forbidden HTN use:

- predetermined mystery beats;
- guaranteed clue order;
- “player finds X, then NPC flees” scripts;
- story arcs hidden as procedures;
- reward payment independent of promise, custody, proof, and verification;
- drama escalation methods.

HTN methods are causal machinery. They are not authored outcome chains.

## Bounded local planning

Bounded GOAP/STRIPS-style planning is allowed for concrete means under an intention or method.

Examples:

- reach a room;
- open a door;
- unlock with a known key;
- retrieve an item;
- move an item to a container;
- eat available food;
- find a route;
- search a place;
- speak to a reachable actor;
- deliver an object;
- flee a perceived threat;
- buy from an available seller;
- report at an open office.

This planner is not the whole mind. It must be bounded, budgeted, debug-visible, and allowed to fail.

A planner failure or timeout is meaningful when it affects action, belief, schedule, suspicion, fatigue, opportunity, or future choice.

## Utility scoring boundary

Utility scoring is allowed only as a bounded heuristic for choosing among explicit options.

Allowed:

- choose whether hunger, duty, fear, or debt deserves attention;
- choose among known food sources;
- choose whether to search or report first;
- choose a slower safer route over a faster risky one;
- choose silence, lie, confession, or refusal from explicit speech-act options;
- choose between available HTN methods.

Forbidden:

- utility as the entire mind;
- global utility with hidden truth access;
- player-proximity utility;
- story-pacing utility;
- emotional drama weights;
- constant task jitter;
- meter refill loops disconnected from beliefs and world state.

## Event-driven replanning

Agents re-evaluate at meaningful interruption points, not every tick because a score wobbled.

Replanning triggers include:

- hunger/fatigue/safety crossing meaningful threshold;
- active plan blocked;
- access denied;
- routine interrupted;
- expected state contradicted;
- relevant record read;
- testimony heard;
- report filed;
- property/food/money custody changed;
- threat/opportunity perceived;
- institution issues instruction;
- weather or route state changes;
- request for help;
- promise/debt due;
- planner timeout/failure.

Durable intentions remain the stabilizer.

## Routines

Routines are defeasible intentions, not teleports.

A work routine should depend on:

```text
actor believes it is a workday/work time
actor knows workplace
actor can reach workplace
actor has sufficient health/time/access
workplace exists and is open or accessible
tools/materials are available or substitutable
role duties and wages matter
no stronger need/threat/obligation interrupts
```

Routine failure matters. A missed work shift can create lost wages, suspicion, resentment, absence evidence, rumor, debt pressure, or opportunity.

## Ordinary competence

Agents should be neither stupid props nor omniscient solvers.

They are bounded by:

- belief and memory;
- skill and experience;
- cost and time;
- distance and route knowledge;
- fatigue, hunger, fear, pain, intoxication, illness;
- social position and relationships;
- access, keys, tools, money, and possessions;
- norms, sanctions, jurisdiction, and proof;
- planner budget and attention;
- LOD tier and fidelity limits.

An agent may fail to find obvious evidence because it was not obvious to that agent. An agent may report weak evidence because they overtrust a witness. An agent may steal because need, opportunity, access, motive, and risk align.

These failures are the simulation.

## Routines versus schedules

Schedules can help author ordinary rhythms, but they must not become rails.

Wrong:

```text
08:00 Mara teleports to mill.
12:00 Mara eats because hunger bar refills.
18:00 Mara returns home regardless of weather, fear, locked doors, or active wrongdoing.
```

Right:

```text
Mara believes it is work time.
She intends to work because she needs wages and fears losing reputation.
She tries to reach the mill through known routes.
Hunger, debt, rain, locked doors, gossip, or a suspicious guard may interrupt.
Failure produces consequences.
```

## Social action and speech

Initial conversation must use structured speech acts with deterministic templates. Freeform text may be added later only as candidate generation behind validation.

Early speech acts should include staged versions of:

- greet;
- ask;
- answer;
- tell;
- report;
- gossip;
- deny;
- lie;
- accuse/suspect;
- promise;
- refuse;
- command/instruct where role permits;
- testify or make simple statement;
- withhold/deflect.

Each speech act defines participants, typed claims, source belief requirements, sincerity/lie/speculation mode, social risk, credibility modifiers, listener interpretation, memory effects, record effects, institution hooks, and failure/refusal outcomes.

## Deception

Deception should appear early in modest form.

A lie requires:

```text
speaker
claim
speaker disbelief or reason to doubt
audience/channel
motive
risk
plausibility/credibility
possible contradictions
memory/reputation consequences
```

A lie does not force belief. Listeners interpret through trust, relationship, evidence, status, bias, prior claims, role, and context.

## Agent LOD

High-detail actors should carry full local beliefs, needs, intentions, routines, perception, and planning.

Medium/low-detail actors may summarize routines and reduce decision frequency, but they remain people. When promoted, they need ancestry, current obligations, role, household, salient beliefs, possessions/custody, relationships, and open projects. They must not be created from nothing because the player approached.

## LLM boundary

LLMs may later render structured speech acts into richer prose, paraphrase records, summarize actor-known information, or parse proposed freeform utterances into candidate speech acts.

LLMs may not:

- plan authoritative agents;
- select actions;
- decide truth;
- create hidden facts;
- grant beliefs;
- create quests;
- make institutions accept claims;
- bypass validation;
- replace symbolic state.

Core agent behavior must work with LLMs disabled.

## Debug inspectability

Debug inspection must answer:

```text
What did the agent believe?
What claims were used?
What needs, values, duties, fears, debts, relationships, or projects mattered?
What intention was active?
What method or bounded plan was selected?
What alternatives were rejected?
What preconditions blocked actions?
What random draws mattered?
What interruption occurred?
What event records the decision, failure, or action?
```

If the answer is “the black box chose it,” the agent architecture is wrong for Tracewake.

## Acceptance checks

An agent feature is not accepted unless it proves:

```text
beliefs are subjective and sourced
needs pressure without puppet behavior
intentions persist without jitter
routines can fail and be interrupted
HTN methods are causal procedures, not scripts
bounded plans can fail and explain why
utility scoring is not the whole mind
speech acts carry typed claims
LLMs are not needed
no-human simulation still runs
TUI/debug can inspect decisions
```

## 2026 hardening: cognition architecture stance

Tracewake should use a symbolic, inspectable, deterministic authoritative cognition spine. The foundation blesses the following mixture, while leaving data structures and algorithms to architecture:

- BDI-style separation of beliefs, motives/desires/pressures, and durable intentions;
- HTN-style decomposition for routines, household duties, institutional procedures, and ordinary tasks;
- bounded GOAP/utility/local planning only as tools inside the actor-known transaction;
- plan repair and replanning when expectations fail or action validation rejects a proposal;
- typed decision traces and stuck diagnostics;
- actor-known, provenance-bearing inputs only.

The foundation rejects:

- LLM agents as authoritative brains;
- utility scores as the entire mind;
- behavior trees as hidden authored plot conditionals;
- HTN methods as scripts with guaranteed outcomes;
- global GOAP over hidden truth;
- schedules or need thresholds that directly dispatch primitive actions;
- planner explanations written after the action was chosen from unrelated truth.

## Actor resourcefulness

Ordinary competence is constitutional. Agents must not merely perform happy-path demo routines. Over time, agent cognition must be able to express:

- plan repair after blocked routes, missing resources, closed offices, occupied beds, failed work, rejection, danger, or contradictions;
- information seeking: search, inspect, ask, read, wait, follow up, verify, compare accounts;
- help seeking: ask household members, authorities, coworkers, companions, strangers, or institutions when actor-known context supports it;
- tool and supply acquisition: fetch keys, food, money, tools, transport, written materials, protection, or aid;
- risk assessment and avoidance: hunger, fatigue, safety, social cost, law, reputation, route danger, weather, witnesses, and uncertainty;
- coordination: meet, request, recruit, accompany, delegate, report, promise, refuse, testify, warn, and negotiate;
- opportunistic goal pursuit: notice a chance, exploit it, abandon it, or defer it based on actor-known motives and risks;
- long-term projects: debts, duties, grudges, friendships, property recovery, survival, reputation, institutional roles, and household obligations.

Not every feature is first-playable, but the foundation must not make later resourcefulness impossible.

## Intention lifecycle

An intention is a live commitment, not a tick-local score winner. It must have source and lifecycle:

```text
candidate pressure or project -> adopted intention -> method/local plan -> proposal attempts -> success/failure/interruption -> continuation, repair, suspension, completion, abandonment, or replacement
```

Possession must not reset intentions. LOD demotion must not erase intentions that later promoted behavior depends on. A routine may resume, fail, or be superseded, but the transition must be explainable.

## Actor decision transaction

Every ordinary autonomous action must pass through the foundation doc 14 transaction or an equivalent hard boundary:

1. select actor and trigger;
2. build sealed actor-known context;
3. gather needs, routines, intentions, obligations, memories, expectations, and perceived affordances;
4. propose candidate goals/actions from actor-known inputs only;
5. explain candidates through typed decision traces;
6. select action or stuck/replan outcome;
7. validate against authoritative world state;
8. apply event(s);
9. generate observations, projections, belief updates, and routine/intention lifecycle changes;
10. preserve replay determinism.

Architecture may refine this ordering, but may not weaken the boundary.
