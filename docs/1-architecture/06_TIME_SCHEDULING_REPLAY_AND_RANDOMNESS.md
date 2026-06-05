# Time, Scheduling, Replay, and Randomness

## Status

This document defines deterministic simulation time, scheduling, replay, and randomness contracts.

The same scheduler runs no-human simulation and embodied play. A human-controlled actor receives commands from a human input boundary, but the actor's actions still occupy simulated time, use reservations, expose observations, and can be interrupted.

## Time model

Tracewake uses discrete simulation time.

Time must support multiple scales:

- seconds/minutes for embodied local actions;
- minutes/hours for routines, sleep, work, conversations, and local travel;
- hours/days for offices, reports, notices, shops, local economy, and rumor spread;
- days/weeks for regional processes;
- months/years for future history generation.

The architecture does not require every actor to run every tick. It requires event-driven wakeups and deterministic ordering.

## Scheduler responsibilities

The scheduler owns:

- current simulation time;
- pending wakeups;
- scheduled action instances;
- actor decision points;
- process decision points;
- reservations and release timing;
- interruptions;
- stable ordering;
- time acceleration;
- LOD-aware execution;
- deterministic random stream coordination;
- replay stepping;
- no-human simulation stepping.

The scheduler does not own dramatic pacing, player importance, or quest timing.

## Wakeup model

Agents and processes wake on meaningful triggers.

```text
need threshold crossed
routine phase begins
scheduled action starts/completes/fails
observation contradicts important belief
speech/report/order arrives
danger perceived
resource becomes unavailable
reservation conflict occurs
institution procedure deadline occurs
office/shop opens or closes
travel segment completes
trace discovered
LOD promotion/demotion occurs
human command arrives for possessed actor
replay/debug inspection requests pause
```

This prevents constant full replanning while preserving reactivity.

## Scheduled action instance

Pseudo-structure:

```yaml
ScheduledAction:
  id: sched_000882
  action_definition: remove_item_from_container
  actor: actor_mara
  parameters:
    container: strongbox_tomas
    item: coin_stack_01
  start_tick: 81210
  earliest_completion_tick: 81230
  latest_completion_tick: 81245
  reservations:
    - actor_body: actor_mara
    - container_access: strongbox_tomas
    - item_possession_candidate: coin_stack_01
  interrupt_windows:
    - tick_range: [81210, 81245]
      interruptors: [visible_witness_entry, actor_choice_abort, danger, container_state_changed]
  observation_windows:
    - kind: possible_sound
      tick_range: [81215, 81220]
  status: running
```

Scheduled actions must be observable while running when channels allow it.

## Stable ordering

Same-time events require deterministic tie-breaking.

Ordering policy must include:

- scheduled time;
- event priority class when necessary;
- stream position;
- actor/process deterministic order;
- reservation conflict policy;
- random stream draw order;
- explicit tie-breakers.

Forbidden sources of ordering:

- wall-clock time;
- OS thread scheduling;
- hash-map iteration accident;
- UI frame order;
- network latency;
- live LLM response timing.

## Durations

Actions may be instantaneous, short, extended, or interval-based.

Examples:

- glance at visible object: near-instant;
- open unlocked door: short;
- search container: short to medium;
- write report: medium;
- sleep: interval until rest, interruption, or chosen wake time;
- work block: interval with routine checkpoints;
- travel route segment: duration with possible events;
- institutional review: queued staff-time process.

Durations are simulation facts. They create opportunities for observation, interruption, conflict, and missed expectations.

## Interruptions

Interruptions are events or proposals that affect a running action.

Possible outcomes:

- continue;
- pause;
- abort;
- fail;
- partial completion;
- switch intention;
- produce trace;
- produce social reaction;
- escalate to conflict.

Example:

```text
Mara is opening Tomas's strongbox.
Elena enters the hall and hears metal movement.
The theft action may continue, abort, speed up, or become caught-in-act depending on perception, risk, and actor choice.
```

An interruption does not automatically reveal truth. It creates observations and later beliefs.

## Reservations

Reservations prevent impossible overlaps and model contention.

Reservation categories:

- body/action channel;
- hand/carry capacity;
- item possession;
- container access;
- door operation;
- bed occupancy;
- room/path occupancy if required;
- workplace station;
- record-writing surface;
- institutional staff time;
- funds or custody commitment;
- route segment if needed.

Reservation conflicts should produce legible outcomes: wait, reject, race, interrupt, negotiate, shove, steal, or fail. The outcome is action-defined and scheduler-mediated.

## No wall-clock decisions

Authoritative simulation must not use wall-clock time for:

- event order;
- random seed;
- action duration;
- planner budget outcome;
- LOD promotion;
- NPC reaction;
- institution timing;
- replay behavior.

Wall-clock time may measure performance at boundaries. It must not affect authoritative outcomes.

## Randomness

Tracewake uses seeded random streams.

Randomness must be:

- stream-scoped;
- deterministic under replay;
- purpose-labeled for meaningful draws;
- isolated from UI/rendering randomness;
- stable under unrelated iteration changes where practical;
- recorded or committed enough for audit/debug.

Recommended stream categories:

```text
agent_choice_noise
perception_uncertainty
trace_generation
memory_decay
rumor_mutation
institution_bias_noise
regional_process
content_generation
```

Avoid one global random stream that changes every downstream result when a new decorative draw is added.

## Meaningful draw recording

A draw is meaningful if it can affect future world state, beliefs, traces, records, or institutional action.

Pseudo-record:

```yaml
RandomDrawRecord:
  draw_id: rnd_000441
  stream: perception_uncertainty
  consumer: obs_elena_noise_resolution
  purpose: heard_noise_confidence
  event_context: evt_container_open_attempt
  sequence: 91
  value_policy: raw_in_debug_hash_in_release
  result_category: low_confidence_noise
```

Debug and test builds should record raw values. Other builds may record commitments/checksums if storage cost matters, but replay must remain verifiable.

## Replay contract

Replay takes:

```text
event log
+ content/data version manifest
+ event schema registry/upcasters
+ initial seed state or snapshot
+ projection versions
```

Replay produces:

```text
current state projection
+ projection caches
+ replay checksums
+ causal graph indexes
+ diagnostics
```

Replay must fail loudly on mismatch. It must not silently repair or guess.

## Replay modes

### Authoritative replay

Used for saves, regression tests, and validation. Unknown versions, missing data, checksum mismatch, or invariant violation stops replay.

### Diagnostic replay

Used by debug tools. It may continue after some failures to inspect damage, but it must mark all derived results as non-authoritative.

### Projection-only rebuild

Rebuilds a projection from known-good events. Projection code changes must be versioned and tested.

### Migration replay

Runs with explicit event upcasters or content migration adapters. It must produce a migration report and before/after checksums.

## No-human simulation

No-human simulation is not a special mode. It is normal simulation with no human controller bound.

It must produce ordinary life:

- waking, eating, sleeping, working;
- household movement;
- work routines and failures;
- conversations;
- rumor movement;
- property expectations;
- reports and records when events warrant them;
- wrong beliefs;
- stale information;
- institution action or failure.

A no-human run that produces only idle loops fails architecture validation.

## Embodied play scheduling

When a human controls an actor:

```text
human command -> actor-bound command -> proposal -> action pipeline -> scheduled action/event
```

Possession does not exempt the actor from:

- fatigue;
- hunger;
- safety fear;
- knowledge preconditions;
- social norms;
- travel time;
- office hours;
- reservations;
- interruptions;
- observation limits.

Waiting, sleeping, traveling, working, and continuing a routine are actor actions. They advance the same scheduler used by no-human simulation.

## Time acceleration

Time acceleration is permitted only as scheduler stepping.

Rules:

- the embodied actor must choose or be engaged in a wait/sleep/travel/work/continue action;
- high-salience actor-perceivable interruptions may stop acceleration;
- no hidden events are skipped for convenience;
- actor receives only actor-accessible results in embodied mode;
- debug mode may show full event summaries.

## Deterministic mocks for nondeterministic surfaces

The following are nondeterministic or boundary-sensitive and must be mockable:

- LLM outputs;
- OS/filesystem ordering;
- wall-clock performance timing;
- terminal input timing;
- future network calls;
- graphical frame timing.

Tests must not depend on live nondeterministic output.

## Acceptance implications

Every runnable phase must include:

- no-human scheduler run;
- replay of that run;
- projection rebuild from events;
- same-seed deterministic check;
- different-seed smoke check;
- action duration/interruption tests;
- reservation conflict tests;
- random draw audit for meaningful branches;
- possessed actor parity test;
- TUI wait/sleep/travel stepping test.

## Anti-patterns

- Actor position changes at schedule time without travel events.
- Every actor wakes every tick forever.
- Wall-clock timeout changes planner choice.
- Random draw order depends on hash iteration.
- Human-controlled actor pauses all NPCs except during explicit wait.
- Time acceleration gives the player omniscient summaries.
- Replay ignores unknown event version.
- Low-LOD summary hides the cause of an active suspicion.
