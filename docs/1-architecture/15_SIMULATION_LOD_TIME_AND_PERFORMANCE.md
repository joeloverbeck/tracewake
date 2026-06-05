# Simulation LOD, Time, and Performance

## Core claim

Tracewake cannot simulate a Skyrim-sized region at room-level detail for years. It also cannot fake everything without destroying its premise. The answer is explicit event-sourced level of detail.

LOD is honest when abstractions produce events, causal ancestry is preserved, important processes can be promoted, summaries are inspectable, and no hidden director injects drama.

## Time model

Use discrete simulation scheduling with event-driven wakeups. Seconds/minutes for embodied local action; hours for routines/travel; days for settlements/economy; weeks/months for regional trends; years for history generation. Do not run every actor every tick.

## Decision points

Agents act when need thresholds cross, routine phases begin, plan steps complete, observations contradict beliefs, reports/orders arrive, danger is detected, travel segment reached, appointments occur, resources become unavailable, or LOD promotion happens.

## LOD tiers

### Tier 0 — Embodied local simulation

Current actor location, active scenes, high-salience investigations, immediate danger. Full action pipeline, perception, speech, traces, pathing/reservations, beliefs, and local planning.

### Tier 1 — Active settlement simulation

Routines, shops, reports, notices, rumor spread, institutional procedures, simplified movement, salient traces.

### Tier 2 — Regional process simulation

Roads, caravans, disease, migration, ecology, patrols, trade. Summary travel, risk events, route state, population/resource changes, institutional messages.

### Tier 3 — Historical/external abstraction

Outside world and long spans. Boundary events, seasonal summaries, migration pressure, wars/famines as external causes, coarse economic/ecological changes.

## Promotion triggers

Promote detail when the player embodies an actor nearby, a lead points to a place, an investigation targets a chain, significant event occurs, contradiction arises, institution opens case, route becomes dangerous, actor is socially important, player travels there, or debug replay asks for detail.

Never promote because the player is bored.

## Demotion

Demote when scene is inactive, no unresolved local chains remain, no embodied actor is nearby, traces are summarized or preserved, routines can be batched, and event streams are snapshotted. Demotion may create summary events.

## Event compaction

Preserve significant causal chains, surviving traces, belief-changing events, institutional records, active leads, and events later used in testimony. Summarize low-salience routine spans.

## Belief store control

Use salience filtering, relevance indexing, confidence decay, semantic memory summaries, rumor chain caps, contradiction merging, and forgetting low-impact details.

## Planner budgets

Use HTN before GOAP, bounded local planners, cached routine plans, event-driven replanning, intention persistence, limited candidate goals, per-agent budgets, LOD-specific planning depth, and fallback actions with explicit hesitation/failure events.

## No-player tests

The simulation must run without player input and produce routines, trade, rumors, reports, notices, wrong beliefs, stale records, migrations, deaths/injuries if modeled, and institutional action/failure.

## Metrics

Track significant events, routine events, causal coverage, no-cause events, belief updates, contradictions, stale beliefs, reports/cases/notices, LOD promotions/demotions, planner time, stuck actors, replay failures, compaction rate, memory size, and player-conditioned event rate.

## Anti-patterns

Every agent replans constantly, all agents full-detail forever, summaries erase causal chains, hidden director creates incidents, low-LOD actors become non-people, player proximity alone causes interesting events, or debugging becomes impossible because LOD hid causes.
