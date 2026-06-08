# Actions, Affordances, Ordinary Life, and Survival

## Core claim

Tracewake's first substrate is ordinary life. Agents must be able to live before they can plausibly investigate, travel, recruit companions, trade, fight, govern, migrate, or conquer.

Ordinary life is not background flavor. It is the causal machinery from which meaningful disruption emerges.

## Action parity

Every committed world-affecting action taken by a human-controlled actor must be an ordinary actor action. The same validator, event pipeline, affordance rules, costs, traces, norms, and failure modes apply to NPCs and human-controlled bodies.

The TUI may offer clearer menus and explanations to the human. It may not create a player-only path.

## Action pipeline doctrine

A world-affecting action should follow this shape:

```text
actor intention / command input / routine / procedure
-> actor-knowledge-bounded action proposal
-> affordance and precondition evaluation
-> cost/risk/norm/access/resource validation
-> possible why-not explanation or failure event
-> scheduled attempt if timeful
-> interruption/failure/success/partial success
-> committed event(s)
-> traces / observations / belief changes / records / consequences
```

A command line is not a mutation API. It is a proposal to an ordinary action system.

## Affordances

An affordance is an entity's possible action surface under conditions.

Affordances may belong to:

- actors: speak, ask, follow, help, refuse, accuse, threaten, trade;
- objects: take, place, inspect, eat, read, write, lock, unlock, break, repair, clean;
- containers: open, close, search, store, hide, remove, inspect contents;
- doors/gates: open, close, lock, unlock, force, knock, listen;
- rooms/places: enter, leave, search, wait, sleep, work, observe, trespass;
- records/notices: read, copy, post, remove, amend, file, misfile, destroy, cite;
- institutions: report, request, pay, refuse, sanction, file, question, post notice;
- routes: travel, avoid, inspect, block, repair, wait for weather;
- traces: inspect, interpret, clean, hide, destroy, record.

Affordances are conditional. Conditions include physical reach, body state, actor knowledge, access permission, willingness to violate norms, tool possession, time, light, cost, risk, skill, institutional authority, and current state.

## Why-not explanations

Blocked actions should explain why in actor-known terms.

Embodied why-not:

```text
Tomas cannot truthfully report that Mara stole the coins: he has no known observation, testimony, record, or trace linking Mara to the missing coins.
```

Debug overlay may add:

```text
Ground truth: Mara took the coins at 02:13. Tomas has not received any modeled information connecting her to the event.
```

Why-not explanations must not leak hidden truth in embodied mode.

## Ordinary action families

The first village should support staged versions of:

- look/perceive surroundings;
- move between adjacent places;
- open/close doors;
- inspect objects and containers;
- take/place/store/hide items;
- eat;
- sleep;
- wait;
- work;
- receive/pay money;
- buy food or simple service;
- search a place/container;
- speak through structured speech acts;
- ask/tell/report/deny/lie/gossip/accuse/refuse;
- read notices and records;
- report to clerk/reeve;
- continue or abandon current intention;
- debug possess another high-detail ordinary actor;
- debug inspect causality, beliefs, traces, records, and replay state.

These actions must be causal, not cosmetic.

## Sleep and fatigue

Sleep is an actor action and routine, not a time skip that grants omniscience.

Sleep/fatigue should affect:

- attention and perception;
- willingness to act;
- work performance;
- travel safety;
- memory formation/retrieval where relevant;
- vulnerability to missing events;
- routine timing;
- social obligations;
- no-human simulation.

A sleeping actor does not receive hidden summaries. They learn what they perceive on waking or what others tell them.

## Food and hunger

Food must come from world channels: household storage, purchase, service, charity, work ration, gift, theft, foraging, or future production.

A meal should not merely fill a bar. It should imply custody, access, source, time, place, possible payment, possible shortage, possible witness, possible household norm, possible theft, and possible memory.

Hunger is pressure, not mind control. It may increase risk tolerance, urgency, irritability, theft temptation, or willingness to accept work, but behavior still depends on beliefs, values, relationships, access, and opportunity.

## Work and wages

Work is a routine/procedure, not a scheduled coin spawn.

Work should imply:

- place;
- route/travel time;
- role/obligation;
- access to workplace, tools, materials, people, or stock;
- absence from home;
- witnesses;
- fatigue;
- payment or wage promise;
- possible delay/refusal;
- social identity;
- failure consequences.

A job may produce wages, food access, reputation, gossip, injury risk, absence evidence, theft opportunity, or resentment. These should arise from events, not hidden payroll ticks.

## Money, payment, and debt

Money must have custody and claim structure. It is not just a numeric stat.

At minimum, support distinctions between:

```text
physical or abstracted money/currency unit
custody: who or what holds it
ownership claim: who is socially/institutionally treated as owning it
access: who may use/take/pay under norms
proof: why a claim can be supported
debt/promise: who owes whom, why, and when
belief: who believes any of this
```

Payment is eventful. Refused, delayed, partial, mistaken, coerced, corrupt, or impossible payment can matter.

## Storage, containers, and property expectations

Storage is central to ordinary life and missing-property play.

Containers and storage places should support:

- physical contents;
- open/closed/locked state;
- access rules;
- privacy norms;
- owner/custodian claims;
- expected contents;
- search methods and false negatives;
- disturbed-container traces;
- hiding;
- borrowing/taking/stealing;
- trace removal;
- household ambiguity.

A missing item only matters to an actor who expected, searched, was instructed, or was told. The container itself does not broadcast a quest.

## Ownership, custody, access, control, and proof

Do not collapse property into `inventory_owner`.

Separate:

```text
ownership claim      who is socially/institutionally treated as owning it
custody              who or what currently holds it
access permission    who may reach/use/open/take it under norms
physical control     who can manipulate it now
proof/provenance     why ownership/custody can be supported or disputed
belief about status  who believes any of the above
```

Examples:

- Tomas owns coins, but the coins are in a strongbox under household custody.
- A spouse can physically open a cupboard but not claim personal ownership of all contents.
- A clerk holds a ledger, but the office has custody and access rules.
- A thief controls hidden coins, but ownership recognition has not changed.
- A forged receipt can influence belief without changing ground truth.

## Search and inspection

Search is an action family with costs and risk.

Search should include:

- actor-known question;
- place/container/route/body/object searched;
- access or trespass status;
- time and attention;
- perception conditions;
- skill/fatigue/stress;
- possible traces found;
- possible false negatives;
- possible social consequences;
- possible record or belief updates;
- failure event when consequential.

A search can disturb a room, make noise, reveal the searcher, violate privacy, damage trust, or create a new suspicion.

## Speech and social action

Talk is action. It consumes time, requires a channel, may fail, carries claims, and changes belief only through listener perception and interpretation.

Social actions include:

- ask;
- tell;
- report;
- gossip;
- lie;
- accuse;
- deny;
- refuse;
- request help;
- promise payment;
- threaten;
- warn;
- give instruction;
- ask permission.

Speech can be sincere, mistaken, hearsay, speculative, evasive, coerced, malicious, or false. The surface utterance is not authoritative. The committed speech act and typed claims are.

## Travel

Travel is foundational because causality is spatial.

Even early travel should model:

- departure and arrival;
- route choice;
- actor belief about route/destination;
- time cost;
- fatigue and hunger costs;
- weather/road effects;
- observation opportunities;
- missed meetings;
- risk events where in scope;
- stale route information;
- traces where relevant.

Off-screen travel may be summarized, but it must not become causality-erasing teleportation.

## Weather and environment in first scope

V1 does not need full climate. It should leave room for environmental causes.

Early environmental effects may include:

- darkness/lighting affecting perception;
- rain or mud affecting tracks and travel;
- cold or heat as simple pressure;
- food spoilage hooks;
- road condition hooks;
- office/shop closure if weather or routine justifies it.

Weather must never begin because the story needs mood.

## Local economy scope

V1 economy is simplified but causal.

Required:

- food as inventory/service under custody;
- hunger and fatigue as pressures;
- work routines;
- wages/payment or promises;
- money custody and ownership/access claims;
- shops/taverns/workplaces/households as stock/service surfaces;
- buying, stealing, storing, hiding, paying, promising payment, refusing payment;
- travel/access costs influencing opportunity;
- no fake macroeconomics.

Deferred:

- full production chains;
- price markets;
- taxation detail;
- inheritance economics;
- long-distance trade networks;
- banking;
- complex craft simulation.

## No-human ordinary life

The village must run without human input. No-human simulation should produce sleep, hunger, meals, work, travel, conversations, missed events, reports, wrong beliefs, stale records, household frictions, and institutional action or failure.

No event should reference player identity. Debug may attach afterward.

## Future action domains

Future systems such as combat, injury, recruitment, animals, disease, magic, vehicles, caravans, courts, armies, settlement conquest, or starship operations must use the same action doctrine.

They need affordances, typed claims, events, traces, access, cost, risk, failure, ordinary/NPC action parity, actor knowledge, records where relevant, and replay/debug explanation.

## Acceptance checks

An ordinary-life action feature is not accepted unless it answers:

```text
What actor can attempt it?
What belief/context is required?
What affordance exposes it?
What physical/social/institutional preconditions apply?
What does it cost?
How can it fail or be refused?
What events and traces does it create?
Who can observe it?
What claims/beliefs/records may result?
Can an NPC do it?
Can the TUI expose it through actor-filtered view models?
Can no-human simulation use it?
Can replay/debug explain it?
```

## 2026 hardening: perceived affordance versus actual affordance

Tracewake must distinguish what an actor can try from what the world will allow.

- **Perceived affordance**: an actor-known possible action exposed through perception, memory, role, record, speech, learned habit, or inference. It can be wrong or stale.
- **Actual affordance**: a capability the authoritative world can validate at resolution time.
- **Embodied menu affordance**: a filtered projection of actor-known/perceived affordances plus why-not capable proposals.
- **Debug affordance**: a non-diegetic inspection of possible truth-space actions and blockers.

An actor may try to open a door believed unlocked and fail because it is locked. An actor may search a container believed to contain food and find nothing. An actor may ask for help at an office believed open and discover no receiver is present. The failed attempt may itself become evidence, memory, trace, or plan-repair input. The planner must not skip the attempt by reading hidden truth.

## Action proposal and validation split

Action proposal is actor-known. Validation is authoritative.

Allowed:

```text
actor believes pantry has food -> proposes inspect/eat/get food -> validator checks actual pantry -> success, failure, rejection, observation, or contradiction
```

Forbidden:

```text
planner reads actual pantry -> chooses only the true food source -> explains afterward that actor wanted food
```

Validation may reject impossible, unavailable, socially blocked, dangerous, or unauthorized actions. It must return structured actor-visible and debug-only reasons so the transaction can replan without leaking hidden truth.

## Ordinary life as anti-demo behavior

Ordinary life is not a meter demo. It must force actors to sustain plausible lives under partial information:

- food can be unavailable, inaccessible, reserved, spoiled, costly, stolen, hidden, socially forbidden, or believed absent;
- sleep can be blocked, interrupted, unsafe, socially inappropriate, or impossible because of location or duty;
- work can be missed, delayed, refused, interrupted, under-resourced, closed, unpaid, socially costly, or misremembered;
- travel can require known routes, time, access, safety assessment, companions, supplies, and current beliefs;
- help seeking can be available, absent, refused, misunderstood, delayed, or socially expensive;
- survival can compete with norms, obligations, reputation, curiosity, fear, fatigue, hunger, and long-term projects.

A no-human day that merely cycles `eat/work/sleep` through privileged truth is not ordinary life. A weaker day with honest failures and repairs is better.

## Required action families for resourcefulness

The constitutional action space must remain open to these families even if many are deferred:

- inspect/search/verify;
- ask/report/testify/lie/refuse/promise/request;
- read/write/post/remove/amend records and notices;
- obtain/use/carry/store/guard/repair tools and supplies;
- wait/rest/sleep/eat/work/travel;
- seek help, recruit, accompany, warn, flee, hide, avoid, patrol;
- negotiate permission, violate permission, suffer detection, confess, accuse, pay, owe, compensate;
- abandon, reconsider, resume, and continue an intention.

These are ordinary action families, not quest verbs.
