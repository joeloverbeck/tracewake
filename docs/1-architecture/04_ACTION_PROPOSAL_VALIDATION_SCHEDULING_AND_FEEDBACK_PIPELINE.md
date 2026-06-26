# Action Proposal, Validation, Scheduling, and Feedback Pipeline

## Status

Authoritative architecture contract.

## Purpose / core rule

Every world-affecting action must enter through an ordinary proposal and the shared validation/event pipeline. Human commands, actor autonomy, institutional procedures, household processes, LOD promotion consequences, and future language surfaces all use equivalent proposal/validation boundaries.

The scheduler schedules opportunities and invokes known-context transactions. It does not author primitive behavior.

## Authority owned

This document owns the action pipeline, proposal ancestry, scheduler limits, validation authority, rejection/failure semantics, actor-legible feedback, and affordance integration.

## Authority denied

No direct mutation path may exist for convenience. No actor may receive player-only verbs. No scheduler may turn raw needs/routines/tables into actions. No validator may suggest plans from hidden truth.

## Contract

### Pipeline shape

```text
input trigger
 -> holder-known context, if cognition/procedure is needed
 -> proposal construction with ancestry
 -> origin/authority check
 -> action definition lookup
 -> actor/process lookup
 -> target binding
 -> reachability/locality/access validation
 -> physical/social/institutional/resource validation against authoritative truth
 -> mutation/event construction
 -> event append
 -> event application
 -> modeled feedback/observation generation
 -> projection/view/debug updates
```

Validation truth is authoritative. Proposal cognition is not.

### Proposal minimum

A proposal must identify:

```text
proposal:
  proposal_id
  origin
  actor_id_or_process_id
  action_id
  target_ids
  tick_or_window
  holder_known_context_id_or_none
  ancestry:
    trigger_id
    candidate_goal_or_procedure_id
    intention_or_procedure_state_id
    method_or_routine_template_id
    local_plan_id
    source_event_ids
  parameters
```

Human-origin proposals include controller-binding metadata but still bind to an ordinary actor and ordinary action. Possession changes input binding, not ontology.

### Origins

Allowed origins include human controller, autonomous actor transaction, institution procedure, household process, fixture/test harness, LOD/regional process, and validated language/speech parser. Origins are metadata for authorization, audit, and replay. They are not separate action rules.

### Scheduler limits

Schedulers may:

- order ticks and windows;
- trigger holder-known context construction;
- choose eligible actors/processes for decision attempts;
- call ordinary actor/institution decision transactions;
- enqueue proposals returned by those transactions;
- record no-human start/end markers and metrics;
- complete due durations through events whose start event reserved the resource.

Schedulers may not:

- emit `move`, `eat`, `sleep`, `work`, `report`, `search`, `sanction`, or `travel` from raw state;
- convert need thresholds directly into primitive actions;
- convert routine labels directly into primitive actions;
- read true food, bed, workplace, route, suspect, record, or clue targets to select plans;
- count markers such as “continue routine” as behavioral progress unless an ordinary follow-on action or explicit modeled wait/failure is committed;
- repair behavior using debug or validator truth.

### Temporal firewall for scheduling

Scheduler and replay time may order decision opportunities and process windows, detect due effects and duration terminals, invoke holder-known transaction construction, validate temporal legality or due-consequence applicability, and emit typed temporal diagnostics.

Scheduler and replay time may not turn raw temporal truth into selected actions, routes, targets, institutional conclusions, or actor-visible reasons unless the premise is present in the relevant holder-known or institution-known context. They may not repair plans using true lateness or hidden schedule truth, and may not leak exact future or due timing through actor-visible feedback unless a modeled channel exposes that timing to the holder.

Budget exhaustion is a typed scheduling or decision outcome. Scheduler budgets and ordering policies must be deterministic and diagnosed. When budget limits prevent full cognition or procedure execution, the outcome is typed as deferred, skipped, summarized, degraded, or blocked, with responsible layer and replay ancestry; the consolidated budget/fairness seam in D-R5 owns the cross-cutting fairness contract.

### Authoritative world-step coordinator

Loaded-world time progression uses one authoritative world-step coordinator.
Human one-tick wait, explicit continuation controls, no-human progression, and
future acceleration all loop that same one-tick contract. The coordinator owns
deterministic due-work ordering, invokes ordinary proposal routing for human
and autonomous actors, and appends due consequences through the shared
validation/event seams rather than through client or scheduler shortcuts.

Open body-exclusive duration discovery is log-derived from event-sourced start
and terminal evidence. Batch-local queues, cached indexes, and pending vectors
may optimize only when they rebuild from that log authority and fail closed on
duplicate or orphan terminal evidence. Explicit continuation controls do not
masquerade as ordinary actor actions; they authorize lifecycle progression or
interruption through the duration seam and do not create a competing wait
action.

Current 0052 evidence names the production coordinator entry explicitly:
`LoadedWorldRuntime::from_bootstrap(LoadedWorldBootstrap, SimTick)` creates the
runtime from validated content plus an action registry, then the shared world
step derives loaded actor work, declared process work, restored eligibility,
process transactions, and closed actor transaction outcome consumption. The
public client boundary is `TuiApp::submit_semantic_action`,
`TuiApp::advance_until`, and command-loop dispatch to closed `RuntimeCommand`
values; observed effects are committed actor/process events, projection
rebuilds, actor census changes, process ancestry, sealed receipts, and replay
verdicts in the witness suites. `negative_fixture_runner.rs` supplies
compile-fail sensitivity for authority boundaries, `public-boundary
conformance` composes the production-boundary matrix, and
`archive/tickets/0052FOUCONFOU-010.md` records the standing mutation rerun with
zero in-surface misses/timeouts and seven routed-forward `food_source` misses.

### Affordances

Objects, places, records, doors, containers, beds, tools, workplaces, notice boards, and people expose typed affordances. Affordances advertise possible proposals; they do not guarantee success.

There are two affordance layers:

- **Perceived/holder-known affordances:** what a holder may consider or what an embodied view may show.
- **Actual validation affordances:** what authoritative truth allows at event time.

The gap between these layers is gameplay material. A visible chest can be locked. A remembered food source can be empty. A public notice can be stale. A reported suspect can be innocent.

### Validation truth

Validators may inspect authoritative state, live reservations, resource state, social/institutional constraints, and event log facts. They may accept, reject, resolve, mutate state through events, and generate modeled feedback.

Validators may not:

- choose the next goal;
- choose fallback actions;
- reveal hidden truth in actor-visible summaries;
- write actor beliefs directly unless a modeled observation/feedback event justifies it;
- make institutions or actors know the debug reason.

### Rejection and failure semantics

A **rejection** prevents mutation because preconditions were not met or authority was absent. A **failure** may be a committed world event: an action was attempted but produced a failed outcome, trace, observation, cost, delay, noise, suspicion, or injury.

Both rejection and failure must split fields:

```text
validation_feedback:
  actor_visible_summary
  actor_visible_reason_codes
  modeled_observation_or_memory_events
  debug_summary
  validation_truth_details
  checked_facts
  failed_stage
```

Actor-visible output can say “You do not know a reachable food source” or “The door is closed.” It cannot say “Mara stole it,” “the hidden pantry is in Mara's kitchen,” or “validator found culprit=true.”

### Modeled feedback

Feedback becomes future knowledge only through events/projections:

- direct observation;
- absence observation;
- sound/uncertain observation;
- rejection memory;
- reported failure;
- institutional record;
- public notice;
- rumor/testimony;
- debug-only report that is not actor knowledge.

### Reservations and durations

Long actions such as sleeping, work blocks, travel, procedures, hearings, searches, and regional operations require event-sourced starts/completions/interruptions and resource reservation semantics. A body-exclusive action cannot be overlapped by a second body-exclusive action without an interruption/resolution event.

While an actor has an unresolved body-exclusive duration, ordinary embodied
actions for that actor are rejected through the shared validation feedback
contract unless an explicit lifecycle control has authorized continuation,
pause, or modeled interruption. The rule applies equally to human-origin and
autonomous-origin proposals.

Derived need-deltas, elapsed-time effects, duration completion/interruption,
and body-exclusive open/close state flow through one authoritative accounting
seam. Schedulers, validators, action definitions, and projections may consume
that seam, but they may not independently charge the same tick/window,
duplicate terminal handling, or silently reconcile conflicting duration
terminals. Stable replay is insufficient if two consumers causally charge the
same actor/time window twice.

## Required diagnostics / replay / TUI hooks

- Validation reports must be typed and replayable.
- Proposal ancestry must be inspectable in debug and acceptance artifacts.
- Actor-facing why-not views must use actor-legible fields only.
- Debug rejection views may show truth details only with non-diegetic markers.
- Schedulers must emit decision/stuck traces for no proposal, no method, failed local planning, rejected proposal, and bounded wait.
- Static/anti-regression checks must look for forbidden scheduler shortcuts.

## Acceptance implications

A test must prove at least one human and one non-human origin share the same pipeline for the same action class. No-human acceptance must show ordinary pipeline events, not scheduler-authored primitive effects. Rejections must commit diagnostic artifacts without world mutation unless a failed attempt event is intended.

## Anti-patterns

- “Need hunger > 800: call eat(true_nearest_food).”
- “Routine WorkBlock: call work(true_workplace).”
- “Human command open bypasses actor proposal because it is interactive.”
- “Closed container rejection says it contains the missing coin.”
- “Validator failure returns suggested targets.”
- “Debug reason is hidden in a display string but tests parse it.”

## Cross-document obligations

This pipeline consumes holder-known contexts from document 03, actor decisions from document 05, epistemic feedback from document 06, speech actions from document 07, institution proposals from document 08, TUI commands from document 10, and acceptance artifacts from document 13.
