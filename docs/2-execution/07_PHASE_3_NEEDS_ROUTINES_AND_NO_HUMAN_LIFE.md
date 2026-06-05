# Phase 3: Needs, Routines, and No-Human Life

## Purpose

Phase 3 makes ordinary life mechanically real.

A missing item matters only if the world has homes, sleep, food, work, routine expectations, fatigue, hunger, safety, interruption, and time. Phase 3 proves agents are not props waiting for the player. They live ordinary days and can be disrupted for modeled reasons.

The phase ends when the village runs at least one full simulated day without human input and remains playable/inspectable through the TUI.

## Entry requirements

Phase 3 may start only when Phase 2 exits.

Required:

- physical action/event/TUI spine works;
- observation, belief, expectation contradiction, and view filtering work;
- possession parity works;
- replay rebuilds item, observation, and belief projections;
- first village fixture has homes, rooms, beds, food/storage, workplace, and major actors;
- routines have Phase 0 paper sketches.

## Deliverables

### Needs

Start with exactly these needs unless a stronger reason appears:

- hunger;
- fatigue;
- safety.

Need values must affect decisions. Cosmetic meters are rejected.

Need pressures create candidate goals:

```text
hunger -> eat at home, buy/eat at public food place, ask, steal food if desperate
fatigue -> sleep at home, nap, delay work, fail perception/action if severe
safety -> avoid place, seek help, stay home, flee local risk, refuse risky act
```

Needs are not scripts. A hungry clerk may still receive a report if role duty, office hours, and social cost outweigh hunger.

### Sleep

Deliver:

- beds;
- sleep action;
- sleep duration;
- fatigue recovery;
- interruption windows;
- expected bed occupancy;
- sleep as missed-observation condition;
- no-human sleep behavior.

Sleep must use scheduler time. It is not instant meter refill.

### Eating

Deliver:

- food item/stack or simple food service;
- household food storage;
- eating action;
- hunger reduction;
- access/permission checks;
- possible purchase or payment placeholder if public food place exists;
- stealing food as future/optional desperate method using same action pipeline.

Do not build a full nutrition economy.

### Work

Deliver:

- at least one workplace;
- at least one work routine for Tomas;
- at least one work/survival routine for Mara;
- office routine for Anna if needed;
- guard/authority routine for Elias/Reeve if used;
- work block action;
- work interruption;
- simple output: payment/debt/service/visibility/fatigue/absence from home.

Work creates causal consequences: location, visibility, fatigue, money/custody, missed events, social interactions, and expectations.

### Homes and household ordinary life

Deliver:

- households with members;
- sleep places;
- storage access;
- food access;
- private/public rooms;
- household privacy;
- expected home/work presence;
- domestic routines.

Households are first-class domestic institutions, even if Phase 4 adds more formal institution logic.

### Routine HTN-like methods

Routines are defeasible methods, not timetables that teleport actors.

Required routine families:

```text
MorningWake
EatMeal
GoToWork
WorkBlock
ReturnHome
SleepNight
FindFood
RespondToImportantContradiction
ContinueCurrentIntention
BasicSocialVisitOrTavernStop if cheap
OfficeHoursRoutine for Anna/Reeve if cheap
```

Each routine must define:

- applicability;
- steps;
- action categories;
- failure modes;
- interruptions;
- expected duration;
- debug trace output.

### Intention persistence

Agents should not jitter every tick.

Deliver:

- current intention;
- intention source;
- commitment level;
- plan/method stack;
- continuation policy;
- abandonment/failure event;
- replanning triggers.

Replanning triggers:

- need threshold;
- routine phase;
- action completion/failure;
- observation contradiction;
- speech/report/order;
- danger;
- resource unavailable;
- social request;
- office/work hours;
- possession attachment only as input binding, not mind reset.

### Interruptions

Implement interruptions as events or action consequences.

Required interruption causes:

- hunger/fatigue threshold;
- missing item contradiction;
- locked door/container;
- workplace unavailable;
- actor receives report or request;
- actor sees/hears salient event;
- office closes/opens;
- action rejected/fails;
- safety concern.

Interruption must be inspectable. Do not silently overwrite actor location or intention.

### TUI time controls

Deliver:

- wait;
- sleep;
- work/continue routine;
- step until actor-perceived interruption;
- debug fast-advance with full event summary;
- no-human run command in debug/test harness.

Embodied time acceleration must stop or notify only for actor-perceivable salient interruptions. Debug can show hidden events.

### Planner/debug trace

For salient decisions, debug must show:

- trigger;
- needs active;
- candidate goals;
- belief inputs;
- routine/method selected;
- plan steps;
- rejected actions;
- precondition failure;
- interruption cause;
- hidden truth mismatch if relevant.

## Explicit non-goals

Phase 3 does not include:

- full market economy;
- deep prices/debt law;
- courts;
- bounty/proof/payment;
- road threat;
- companion recruitment;
- large regional simulation;
- detailed injury/combat;
- LLM planner/dialogue;
- mature rumor network beyond minimal speech if cheap.

## TUI/view-model gate

A user can:

1. embody Tomas, Mara, Anna, Elena, or Elias;
2. inspect current needs/status;
3. sleep in an available bed if conditions allow;
4. eat from available food if allowed;
5. begin/continue work if routine conditions allow;
6. wait through time;
7. continue current routine;
8. stop when current actor perceives a salient interruption;
9. inspect why a routine continued, failed, or was interrupted;
10. switch debug and inspect planner trace.

The action menu must be actor-filtered. It must not include debug truth or special player routine controls.

## No-human simulation gate

The village must run at least one full simulated day with no human controller bound.

Minimum pass conditions:

- agents wake, eat, work, rest, and sleep;
- at least one work routine completes or fails for modeled reasons;
- needs change and affect choices;
- agents move through rooms/places by action/event;
- no event references player identity;
- no actor freezes because unpossessed;
- no protagonist gravity appears;
- no-human event log replays.

Preferred pass conditions:

- social speech occurs minimally;
- routine interruption occurs;
- missing-property setup can occur without human input under deterministic fixture;
- debug attach afterward can inspect state.

## Deterministic replay gate

Replay must reconstruct:

- need values at salient ticks;
- routine start/completion/interruption;
- scheduler wakeups;
- work/sleep/eat events;
- movement caused by routines;
- action rejections/failures;
- random draws if any;
- planner trace summaries for salient decisions;
- no-human run event log.

Replay from snapshot and replay from genesis must agree for covered state if snapshots exist.

## Test gate

### Unit tests

- hunger/fatigue/safety update;
- need threshold candidate goal creation;
- sleep action scheduling/interruption;
- eat action access/cost/effect;
- work block start/completion/failure;
- routine applicability;
- intention persistence;
- replanning trigger;
- planner trace output.

### Property tests

- no actor has two simultaneous body-exclusive actions;
- sleep cannot complete without scheduled duration;
- need values remain bounded;
- routine cannot teleport actors;
- every meaningful routine state change has event ancestry;
- no-human events contain no player/controller world fact;
- time acceleration does not skip committed events.

### Golden tests

- `no_human_day_001`;
- `ordinary_workday_001`;
- `sleep_interrupt_001`;
- `food_missing_001`;
- `work_interrupted_by_missing_property_001`;
- `routine_no_teleport_001`;
- `planner_trace_001`.

### TUI/view-model tests

- wait/sleep/continue controls advance scheduler;
- embodied view shows actor-known interruption only;
- debug view shows full hidden event stream;
- needs panel does not reveal hidden truth;
- why-not shows hunger/fatigue/social/physical blockers.

## Data/fixture gate

Routines must validate:

- actor role/home/workplace references;
- preconditions;
- steps;
- failure modes;
- interruption triggers;
- expected duration;
- needs affected;
- TUI action coverage;
- planner trace labels.

Invalid routine examples:

```text
08:00 actor appears at work
routine with no failure modes
work succeeds regardless of hunger/fatigue/tool/place
sleep restores fatigue instantly without event
food meter refills without food/service/access
```

## Debug/inspection gate

Debug must answer:

- why did Mara skip work?
- why did Tomas go to the mill?
- why did Anna refuse or delay a report because office was closed or she was absent?
- why did an actor wake, eat, sleep, or continue routine?
- what interrupted the routine?
- what precondition failed?
- was hidden truth used?
- did replay rebuild the same routine chain?

## Failure cases to model

Required:

- actor too tired to work;
- actor too hungry to continue work;
- actor has no accessible food;
- actor lacks permission to use food;
- bed occupied/unavailable;
- office closed;
- workplace locked;
- route/path blocked within village graph;
- tool/item missing;
- action interrupted;
- routine abandoned after repeated failure;
- planner budget exhausted;
- actor stuck with diagnostic event;
- no-human run finds idle loop.

## Metrics

Track at least:

- events/day;
- routine events/day;
- meals completed/missed;
- sleep completed/interrupted;
- work blocks completed/failed;
- need threshold crossings;
- routine interruptions;
- planner failures;
- stuck actors;
- no-human run duration;
- replay failures;
- TUI action coverage;
- player-conditioned event rate in embodied comparison runs.

## Forbidden shortcuts

- schedules teleport actors;
- needs are cosmetic meters;
- unpossessed actors freeze;
- time acceleration skips hidden events;
- no-human run is an idle loop;
- work creates payment from nowhere without event/custody placeholder;
- routine failures disappear silently;
- planner uses ground truth;
- debug commands substitute for ordinary actions.

## Phase exit checklist

Phase 3 exits when:

- hunger/fatigue/safety affect behavior;
- sleep/eat/work/wait function through TUI;
- routines are defeasible and inspectable;
- no-human day runs;
- ordinary life supports the missing-property setup;
- replay rebuilds routine chains;
- tests cover success and failure;
- debug explains planner/routine decisions;
- no player/quest/LLM shortcuts exist.

Only then begin Phase 4.
