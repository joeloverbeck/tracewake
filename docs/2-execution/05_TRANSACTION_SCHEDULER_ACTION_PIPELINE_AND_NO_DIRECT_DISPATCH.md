# Transaction, Scheduler, Action Pipeline, and No Direct Dispatch

## Status

Live execution doctrine for action authority and scheduler audit.

## Authority boundary

This document owns execution checks for actor transactions, scheduling, proposal construction, action validation, event append, and no direct dispatch. It does not define Rust APIs or implementation code.

## Depends on

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`

## Core rule

No world-affecting behavior may bypass the shared pipeline.

Human commands, autonomous actor decisions, no-human advancement, institutional procedures, household processes, future speech acts, LOD promotion consequences, and fixture/test harness actions must become ordinary proposals with ancestry. They are validated against truth, committed as events, applied deterministically, projected, and replayed.

## Canonical actor-decision transaction

An actor autonomy opportunity must move through these reviewable stages:

1. Trigger: tick, routine window, need pressure, active intention, interruption, observation, speech, danger, procedure, or human input requiring world action.
2. Sealed actor-known context with provenance.
3. Candidate generation from actor-known pressures and commitments.
4. Intention continuation, adoption, suspension, interruption, completion, failure, abandonment, or replacement.
5. HTN/routine/procedure method selection from actor-known conditions.
6. Bounded local planning from actor-known routes, places, targets, records, and affordances.
7. Proposal construction with ancestry.
8. Shared action pipeline validation.
9. Event commit, rejection, modeled failure, wait, or stuck diagnostic.
10. Actor-legible feedback and modeled observations.
11. Projections, debug diagnostics, and replay artifacts.

Skipping a stage is a certification failure unless the audited behavior is not a cognition path.

## Scheduler limits

Schedulers may:

- order ticks and windows;
- select which actors or processes receive decision opportunities;
- build or request holder-known contexts;
- call actor or process transactions;
- enqueue proposals returned by transactions;
- complete reserved durations through committed completion events;
- record no-human markers and metrics;
- record typed scheduler diagnostics.

Schedulers may not:

- emit movement, eating, sleeping, work, search, accusation, report, travel, sanction, or container actions from raw state;
- turn hunger, fatigue, safety, routine label, work window, or fixture table directly into primitive actions;
- mint need deltas, charge durations, supply proposal-side current-need values
  as authority, or let routine labels charge time;
- choose targets from true world location, true route graph, true workplace, true food source, true suspect, true record, or true clue;
- treat marker actions as behavioral progress;
- repair plans using debug reports or validator rejection truth;
- special-case possessed actors except for input binding.

## Scheduler Temporal Authority and Budget Evidence

Schedulers may awaken candidates, order transactions, validate temporal
preconditions and effects through the shared action pipeline, and account for
budget exhaustion. They may not select intentions, invent reasons, rewrite wait
causes, or conclude routines by consulting true time alone. True time may
trigger review; it is not a holder-known premise unless the transaction receives
that premise through a modeled source in its sealed context.

Scheduler-budget evidence must prove deterministic candidate ordering, budget
exhaustion, deferred or skipped cognition, and no-direct-dispatch behavior under
load. Budget pressure may not become an invisible director choosing outcomes
without typed evidence. Cross-cutting budget and fairness diagnostics are owned
once by `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; this
document records the scheduler-side proof obligation and points there for
starvation, repeated-deferral, actor-class, region-class, time-acceleration,
and replay-determinism review.

## Canonical world-step execution contract

The canonical step for loaded-world advancement is owned by the core
transaction/scheduler/pipeline boundary. It accepts a typed world-advance
request, produces a typed result, and follows one deterministic phase contract:
frontier validation, tick-boundary ancestry, due-duration discovery from the
shared event log, duration lifecycle resolution, ordinary actor/process
transactions, reservation enforcement, single-owner accounting, due world
processes, and projection/replay completion.

All accepted events from that step use the shared event append and application
route. Human-origin wait, autonomous actor opportunities, no-human
advancement, continuation, and future acceleration may choose different inputs
or stop conditions, but they may not own alternate event routes or dispatch
primitive behavior directly. Due-duration scans are log-derived from
event-sourced starts and terminals, and actor transactions run only at their
deterministic cadence or due opportunity.

The no-direct-dispatch guard must prove that no TUI path, debug runner,
scheduler shortcut, routine label, need threshold, or duration queue mutates
world state outside this canonical step and shared pipeline.

## Proposal ancestry

A proposal must carry enough ancestry to reconstruct why it exists:

- origin class;
- actor or process identity;
- triggering event/window/input;
- holder-known context identifier;
- candidate goal or procedure step;
- intention or procedure state;
- method/routine template;
- local plan step;
- source event/provenance identifiers;
- action family and semantic identity;
- target identities known to the holder;
- parameters that are actor-known or explicitly actor-supplied.

A validator must reject forged or stale parameters that claim actor-known state without provenance.

## Validation boundary

Validators may read authoritative truth to decide whether a proposal can commit. Validators may create events, rejections, modeled failures, observations, traces, reservations, or diagnostic artifacts.

Validators may not choose an actor's next plan, leak hidden reason into actor-visible text, or create beliefs without modeled observation/feedback events.

## Direct-dispatch audit gate `NO-DIRECT`

A `NO-DIRECT` audit must prove:

1. No scheduler path calls primitive world mutation directly.
2. No actor-decision path mutates state before proposal validation.
3. No action family has an alternate possessed-player path.
4. No debug or fixture helper is used by live gameplay paths.
5. No continuation marker is counted as action progress without a follow-on ordinary action, modeled wait, or typed stuck/failure record.
6. No event application path depends on unrecorded wall-clock, unordered iteration, nondeterministic randomness, or display strings.
7. Rejections and failures are typed and layer-attributed.

## Replay and projection coupling

A passing pipeline gate must show that accepted proposals produce events that replay into the same physical, epistemic, actor-state, routine, diagnostic, and debug projections.

Projection rebuild is not a convenience test. It is how Tracewake proves that current state is not the authority.

## Failure diagnostics

Pipeline failures must identify the responsible stage:

- missing holder-known input;
- candidate not generated;
- method not applicable;
- local plan failed;
- proposal malformed;
- origin unauthorized;
- target binding invalid;
- reachability/access/resource validation failed;
- reservation conflict;
- action definition missing;
- event append failure;
- event application divergence;
- projection rebuild mismatch;
- actor-visible/debug boundary violation.

A diagnostic that says only “action failed” is not sufficient.
