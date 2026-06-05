# Simulation LOD, Time, and Performance

## Core claim

Tracewake cannot simulate a large region at room-level detail for years. It also cannot fake everything without destroying its premise.

The answer is explicit event-sourced level of detail.

LOD is honest when abstractions produce events, preserve causal ancestry, promote when relevant, remain inspectable, and never hide a drama director.

## Time model

Use discrete simulation scheduling with event-driven wakeups.

Scales:

- seconds/minutes for embodied local action;
- minutes/hours for routines and local travel;
- hours/days for shops, offices, reports, and ordinary economy;
- days/weeks for regional processes;
- months/years for history generation.

Do not run every actor every tick.

## Decision points

Agents act or reconsider when:

- need thresholds cross;
- routine phases begin;
- plan steps complete;
- observations contradict beliefs;
- reports/orders arrive;
- danger is detected;
- travel segment reached;
- appointment occurs;
- resource becomes unavailable;
- institution procedure triggers;
- LOD promotion occurs;
- player takes embodied action;
- debug/replay requests inspection.

## LOD tiers

### Tier 0 — Embodied local simulation

Current actor location, active scenes, high-salience investigations, immediate danger.

Full:

- action pipeline;
- perception;
- speech;
- traces;
- path/reservations;
- beliefs;
- local planning;
- detailed event commits.

### Tier 1 — Active settlement simulation

Routines, shops, reports, notices, rumor spread, institutional procedures, simplified movement, salient traces, actor schedules as defeasible intentions.

### Tier 2 — Regional process simulation

Roads, caravans, disease, migration, ecology, patrols, trade, summary travel, route state, population/resource changes, institutional messages.

### Tier 3 — Historical/external abstraction

Outside world and long spans. Boundary events, seasonal summaries, migration pressure, wars/famines as external causes, coarse economic/ecological changes.

## Promotion triggers

Promote detail when:

- player embodies an actor nearby;
- a lead points to a place;
- investigation targets a chain;
- significant event occurs;
- contradiction arises;
- institution opens case;
- route becomes dangerous;
- actor is socially important;
- player travels there;
- record/testimony references it;
- debug replay asks for detail.

Never promote because the player is bored.

## Demotion

Demote when:

- scene is inactive;
- no unresolved local chains remain;
- no embodied actor is nearby;
- traces are summarized or preserved;
- routines can be batched;
- event streams are snapshotted;
- no institution or lead requires local detail.

Demotion may create summary events.

## Event compaction

Preserve:

- significant causal chains;
- surviving traces;
- belief-changing events;
- institutional records;
- active leads;
- events later used in testimony;
- ownership/custody changes;
- wrong-belief origins;
- LOD transition causes.

Summarize low-salience routine spans.

## Belief store control

Use:

- salience filtering;
- relevance indexing;
- confidence decay;
- semantic memory summaries;
- rumor chain caps;
- contradiction merging;
- forgetting of low-impact details;
- source preservation for important beliefs;
- actor-specific memory budgets.

Do not turn forgetting into random deletion of important causal ancestry.

## Planner budgets

Use:

- HTN before local planning;
- bounded local planners;
- cached routine plans;
- event-driven replanning;
- intention persistence;
- limited candidate goals;
- per-agent budgets;
- LOD-specific planning depth;
- fallback actions;
- explicit hesitation/failure events.

A stuck actor is a bug unless the stuckness is itself modeled.

## No-player tests

The simulation must run without human input and produce:

- routines;
- trade;
- eating/sleeping/work;
- conversations;
- rumors;
- reports;
- notices;
- wrong beliefs;
- stale records;
- migrations or boundary events when modeled;
- injuries/deaths if modeled;
- institutional action or failure.

No protagonist gravity is allowed.

## Metrics

Track:

- significant events;
- routine events;
- causal coverage;
- no-cause events;
- belief updates;
- contradictions;
- stale beliefs;
- reports/cases/notices;
- rumor mutations;
- LOD promotions/demotions;
- planner time;
- stuck actors;
- replay failures;
- compaction rate;
- memory size;
- player-conditioned event rate;
- TUI action coverage.

A rising player-conditioned event rate is suspicious unless it is caused by player actions through ordinary channels.

## Anti-patterns

- every agent full-detail forever;
- every agent replans constantly;
- summaries erase active causal chains;
- hidden director creates incidents;
- low-LOD actors become props;
- player proximity alone causes interesting events;
- debugging impossible because LOD hid causes;
- no-player sim produces nothing but idle loops.
