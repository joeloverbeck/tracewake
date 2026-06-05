# Agent Intention and Planning Doctrine

## Core claim

Tracewake agents must be symbolic, inspectable, fallible, and ordinary.

They are not LLM puppets, quest actors, utility meters, or schedule robots. They are bounded agents who believe things, want things, commit to things, fail at things, misread the world, and leave causal evidence when they act.

## Required mental separation

Tracewake uses a BDI-style separation as doctrine:

- **Beliefs**: subjective, sourced, fallible propositions about the world.
- **Desires / values / goals**: needs, motives, duties, projects, preferences, fears, relationships, habits, ambitions, obligations, and social pressures.
- **Intentions**: durable commitments and selected methods the agent is currently trying to carry out.

This separation is not optional vocabulary. It protects the simulation from flattening agents into meter machines or omniscient optimizers.

## Beliefs

Agent beliefs must be subjective, source-bound, confidence-bearing, stale unless updated, contradiction-capable, memory-affected, inspectable in debug, and action-relevant.

Agents choose based on their beliefs, not ground truth. This is especially important for searching, reporting, accusing, lying, buying, stealing, avoiding, trusting, and institutional behavior.

## Desires, values, needs, duties, and projects

Agents should have early simple personality and social structure, not a grand psychology model.

V1 should include enough of the following to support ordinary life and social epistemics:

- food need;
- sleep/fatigue need;
- safety pressure;
- money pressure;
- role duties;
- household obligations;
- work obligations;
- relationships;
- trust/distrust;
- simple values such as honesty, loyalty, greed, fearfulness, duty, pride, kindness, resentment;
- traits that bias perception, risk tolerance, planning, and speech;
- durable projects such as earn wages, feed household, repay debt, maintain reputation, hide wrongdoing, recover property.

Needs are pressures. They should not directly puppet agents. A hungry honest worker and a hungry desperate worker should not behave identically.

## Intentions

Intentions prevent jitter.

An intention is a commitment such as:

- go to work;
- cook or acquire food;
- sleep;
- recover missing property;
- hide stolen coins;
- avoid a suspicious actor;
- report to the clerk;
- repair a tool;
- visit a household member;
- pay debt;
- keep quiet;
- spread a rumor;
- search a room.

Intentions must be durable enough to persist across small score changes, but interruptible by stronger modeled causes such as hunger, threat, fatigue, obligation, discovery, contradiction, blocked access, social command, or institutional procedure.

Adopting, suspending, resuming, abandoning, or completing an important intention should be eventful when it matters.

## HTN methods

HTN-style methods are the right tool for routines, jobs, domestic behavior, social procedures, institution procedures, and recovery/search/report patterns.

Allowed HTN methods must have:

- method name and purpose;
- preconditions based on belief and accessible state;
- resources and costs;
- time model;
- route/access requirements;
- norm and permission checks;
- expected traces;
- possible observations;
- failure modes;
- interruption points;
- alternative methods;
- record or speech-act effects where relevant;
- debug-explainable causal links.

Examples of foundation-appropriate HTN domains:

- morning routine;
- eat from household storage;
- acquire food from tavern/shop/household;
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
- travel to nearby place.

Forbidden HTN use:

- predetermined mystery beats;
- guaranteed clue ordering;
- “player finds X, then NPC flees” scripts;
- story arcs hidden as procedures;
- reward payment independent of custody and verification.

## Bounded local planning

Bounded local GOAP-style planning is appropriate for concrete means:

- reach a room;
- open a door;
- unlock with a known key;
- retrieve an item;
- move an item to a container;
- eat available food;
- search a nearby place;
- speak to a reachable actor;
- deliver an object;
- flee a threat;
- find a route;
- buy from an available seller;
- report at an open office.

This planner is not the whole mind. It runs under an intention or HTN method. It must have budgets, failure events, and debug-visible rejected options.

A planner timeout or failure is a meaningful event if it affects action, belief, schedule, suspicion, fatigue, or future choices.

## Utility scoring boundary

Utility scoring is allowed only as a bounded selection heuristic.

Allowed:

- choose which need deserves attention;
- choose among several known food sources;
- choose whether fear outweighs duty;
- choose whether to search or report first;
- choose a low-risk route over a fast route;
- choose a lie or silence based on motive and risk.

Forbidden:

- utility as the entire mind;
- agents as meter refill machines;
- global utility that sees hidden truth;
- constant jitter between tasks;
- drama-weighted utility;
- player-proximity utility;
- story-pacing utility.

## Event-driven replanning

Agents should re-evaluate intentions when meaningful events occur:

- hunger/fatigue crosses a threshold;
- access is blocked;
- a routine is interrupted;
- a belief contradiction appears;
- a record is read;
- testimony is heard;
- a report is filed;
- money, food, or property custody changes;
- a threat or opportunity is perceived;
- an institution issues an instruction;
- weather or route state changes;
- another agent requests help;
- a plan fails.

Replanning should not occur every tick just because a score changed slightly. Durable intentions remain the stabilizer.

## Routines

Routines are defeasible intentions, not teleports.

A workday routine should depend on beliefs and conditions:

- the actor believes today is a workday;
- the actor knows the workplace;
- the actor can reach it;
- the actor has sufficient health, time, and access;
- the workplace exists and is open or accessible;
- required tools/materials are available or substitutable;
- stronger needs or emergencies do not interrupt;
- relevant norms and obligations matter;
- failure creates consequences.

A routine may be interrupted by missing property, hunger, locked doors, weather, fear, a report, social obligation, illness, or wrong belief.

## Ordinary competence

Agents should be neither stupid props nor omniscient solvers. They are bounded by beliefs, memory, skill, time, cost, distance, fatigue, hunger, fear, status, relationships, access, tools, money, institutional jurisdiction, norms, and planner budgets.

An agent may fail to find obvious evidence because it is not obvious to that agent. An agent may report weak evidence because they overtrust a witness. An agent may steal because need, opportunity, motive, and risk align. These failures are the simulation.

## Speech acts

Initial conversation should be structured speech acts with templated text.

Examples of early speech acts:

- greet;
- ask location of actor/object;
- report observation;
- report missing property;
- accuse;
- deny;
- lie claim;
- promise payment;
- request help;
- refuse;
- threaten;
- warn;
- gossip;
- testify;
- ask permission;
- give instruction;
- ask about rumor;
- read or recite record.

Each speech act should define participants, claims, actor-knowledge preconditions, social risk, credibility modifiers, norm/institution hooks, belief effects, memory effects, record effects where relevant, and possible refusal or failure.

No freeform parser may directly mutate authoritative state. Future natural-language input may propose candidate speech acts only behind validation.

## Lies and deception

Deception should appear early in modest form.

A lie requires:

- a claim;
- a speaker who does not believe the claim or has reason to doubt it;
- a motive;
- an audience;
- risk;
- plausibility/credibility;
- possible contradictions;
- future memory and reputation effects.

A lie should not force belief. Listeners interpret claims through trust, relationship, evidence, status, bias, and prior beliefs.

## LLM boundary for agents

LLMs may later render structured speech acts into prose, paraphrase records, summarize actor-known information, or parse proposed utterances into candidate speech acts.

LLMs may not:

- decide what happened;
- create hidden facts;
- grant beliefs;
- plan authoritative agents;
- choose world actions;
- create quests;
- make institutions accept claims;
- bypass validation;
- replace symbolic agent state.

Core mechanics must work with LLMs disabled.

## Debug inspectability

Debug inspection must answer for an agent decision:

```text
What did the agent believe?
What needs or duties mattered?
What intention was active?
What method or plan was selected?
What alternatives were considered or rejected?
What preconditions blocked actions?
What random draws mattered?
What interruption happened?
What event records the decision, failure, or action?
```

If the answer is “the black box chose it,” the agent architecture is wrong for Tracewake.
