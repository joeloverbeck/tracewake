# Issue report — Human-mode `wait` does not run the simulation; duration actions (`sleep`, `work_block`) never complete

> **Status:** Design/spec-level gap. Not auto-fixed. Intended to seed a future research brief.
> **Reported:** 2026-06-22
> **Baseline commit:** `241d0d0` (`docs/4-specs/SPEC_LEDGER.md` and a few skill files were dirty in the tree at report time; the code paths cited are unaffected)
> **Discovered by:** TUI bug-hunt loop, iteration 8 (playing scenarios as a human user)
> **Surfaces:** `tracewake-tui` command loop (possessed/human-driven actor) + `tracewake-core` scheduler/duration subsystem

---

## 1. Summary

When a human-possessed actor submits a **duration action** from the TUI — `sleep`
(`sleep.here`) or `work_block` — the action is *accepted* and a duration-start event is
appended, but the duration **never completes** and produces **no recovery / no output**.
Subsequent `wait` commands advance the bound actor's tick and *charge* needs (fatigue/hunger
keep rising) but never run the scheduler that would complete the sleep/work duration, run
other actors, or apply due effects.

Net player-visible effect: **an actor can never recover fatigue by sleeping in the TUI.**
`do sleep.here` reports `Accepted` and then does nothing observable; waiting only makes the
actor more tired.

This contradicts the foundational TUI doctrine that **"Waiting runs the simulation."**

---

## 2. Reproduction

```
$ cargo run -p tracewake-tui -- --operator-debug sleep_eat_work_001 actor_tomas
tracewake> debug needs                 # actor_tomas fatigue value=620 band=urgent cause=fixture_initial
tracewake> do sleep.here
Accepted: sleep.here
tracewake> debug needs                 # fatigue STILL value=620 cause=fixture_initial  (no recovery applied)
tracewake> wait
tracewake> debug needs                 # fatigue value=623 cause=tick_delta  (DEGRADED, not recovered)
```

Observations:
- `sleep.here` is `Accepted` but applies **zero** fatigue change at the tick it is submitted
  (contrast `eat`, which immediately moves hunger with `cause=action_effect:eat`).
- The sim tick does **not** advance on `sleep.here` (stays at tick 0); a `SleepStarted`
  body-exclusive duration event is appended, but nothing consumes it.
- Following `wait`s advance the tick and apply ordinary `tick_delta` need charges — fatigue
  keeps **rising**. The cause is never `action_effect:sleep`; the actor is not actually asleep
  across those ticks.
- `wait` submitted *after* `sleep.here` is **Accepted** (not `ReservationConflict`), so the
  body-exclusive sleep reservation is not being enforced against the human in the single-step
  path either — part of the same gap.

The same shape applies to `work_block`: it starts a duration whose completion is scheduled,
and the human path never reaches the scheduled completion (the only `work_block` completion
seen in the TUI today is the *immediate failure* path when truth contradicts belief, e.g.
`embodied_workplace_believed_open_truth_closed_commit_fails_001`).

---

## 3. Doctrine (what the docs require)

`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`:

- **L259:** *"Waiting is not skipping causality. Waiting runs the simulation."*
- **L261–272:** *"The TUI should support **staged versions of**: pause; step; wait; sleep as
  actor action; continue current intention; time acceleration; travel time; stop on salient
  perceived interruption; actor-filtered missed-event summaries; debug event streaming."*
- **L274:** *"A sleeping actor does not receive omniscient summaries. They learn what reached
  them through modeled channels or what they perceive later."*
- **L276:** time controls "may advance authoritative event/replay time," and summaries such as
  *"slept until morning"* are actor-known/record-derived conclusions.

Reading: sleep is a first-class actor action that **spans time**; waiting **advances the
authoritative simulation** (not just the possessed actor's local clock). The actor should wake
recovered, learning a record-derived "slept until …" summary. The word **"staged"** is the key
tension: this is an explicitly phased capability, so today's partial behavior may be an
unfinished stage rather than a regression — but the current behavior (accept-then-no-op,
fatigue only ever rising) is below even a minimal honest stage.

---

## 4. Root cause (where the gap is in code)

The simulation has a complete, deterministic duration-completion engine — it is simply **not
wired into the human submit path**.

- **Duration starts are emitted by the action defs:**
  - `crates/tracewake-core/src/actions/defs/sleep.rs:18` `build_sleep_start_event` (emits
    `SleepStarted`, `body_exclusive=true`, carries `fatigue_recovery_per_tick`).
  - `crates/tracewake-core/src/actions/defs/work.rs` `build_work_block_*` — start event's
    `effects_summary` is literally *"work block started; completion is duration scheduled"*
    (`work.rs:310`).

- **Duration completions are scheduler-driven only:**
  - `crates/tracewake-core/src/scheduler.rs:344` `run_no_human_day(...)` is the only routine
    that walks ticks and fires completions.
  - It calls `build_sleep_completion_events` (`scheduler.rs:996`, def `sleep.rs:167`) and
    `build_work_completion_events` (`scheduler.rs:1052`, def `work.rs:119`) when a start is
    "due" (`is_due_sleep_start` `scheduler.rs:686`, `is_due_work_start` `scheduler.rs:691`),
    enforcing duration-terminal closure (`is_duration_terminal`, `scheduler.rs:1020/1076`).
  - `DeterministicScheduler::advance_one_tick` exists (`scheduler.rs:212`) but the human path
    never drives the completion walk.

- **The TUI human submit path never runs that engine:**
  - `crates/tracewake-tui/src/app.rs` `submit_entry` (≈L246–302) calls `run_pipeline` once,
    sets `self.scheduler.current_tick = last_event.sim_tick`, records current-place perception,
    and returns. There is **no** call into sleep/work completion or a per-tick scheduler walk.
  - `wait` (`UiCommand::WaitOneTick` → `run.rs`, and `build_wait_events`) emits wait/need-charge
    events for the bound actor only. It advances the tick and charges needs but does **not**
    complete due durations or run other actors.
  - The only scheduler-driven advance reachable from the TUI is the debug-gated
    `run_no_human_day` (`app.rs:308`), i.e. *omniscient batch* advance — not "waiting runs the
    simulation" for the possessed actor.

So `wait` today means "advance my local clock and charge my needs," which is precisely
"skipping causality" — the thing L259 forbids.

---

## 5. Scope & impact

- **Gameplay:** fatigue is unrecoverable in human play; sleeping is a no-op; work blocks never
  produce wages/output via the human path. This blocks any human-driven "ordinary day."
- **Doctrine:** direct conflict with `08_TUI…:259` ("Waiting runs the simulation") and the
  intent of `:276` ("slept until morning" summaries).
- **Consistency:** `eat` (instantaneous effect) works; the gap is specific to **scheduled-
  completion duration actions** (`sleep`, `work_block`) and to `wait`'s contract.
- **Not a crash / not a contamination leak:** the event log stays consistent and replay is
  deterministic; this is a *missing capability*, not a correctness violation of the existing
  pipeline.

---

## 6. Why this was not auto-fixed (and the invariant risk)

A faithful fix wires per-tick simulation advance (including duration completion and need
accounting) into the human `wait`/time-control path. That touches the **most invariant-sensitive
subsystem** in the kernel. Relevant locked invariants (see
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):

- **0017 tick-charge attribution** — every tick-delta need charge reconciled by counted
  `(actor, need, tick)` occurrences; one charge per tick (`assert_single_tick_delta_charge`,
  `wait_then_window_passive_charges_each_tick_once_001`). A naive "wait then also run sleep
  recovery" risks double-charging or double-recovering a tick.
- **0017 shared open-duration authority** — body-exclusive duration start/terminal keying is
  shared across scheduler, pipeline, and need-accounting; duplicate/orphan terminals must
  fail closed (`duplicate_duration_terminal_poisons_rebuild_001`).
- **Replay determinism** — any new event-emitting advance path must round-trip identically
  under replay/rebuild.
- **Subjective epistemics** — a slept-through interval must yield *actor-known / record-derived*
  summaries (`08_TUI…:274/276`), never hidden-truth leakage; the possessed actor must not gain
  omniscient missed-event summaries.

Getting these wrong silently corrupts need accounting or replay. This is spec-and-ticket work,
not an autonomous TDD patch.

---

## 7. Possible directions (for the research brief to evaluate, not decisions)

1. **Reuse the no-human per-tick engine for the possessed actor.** Factor a single
   `advance_one_tick`-style step out of `run_no_human_day` that completes due durations and
   applies tick charges deterministically, and have human `wait`/time-controls drive it for the
   authoritative timeline while filtering what the possessed actor *learns* through modeled
   channels (per `08_TUI…:274/276`). Open question: do other (unpossessed) actors also advance
   when a human waits, and how are their results surfaced (missed-event summaries)?
2. **Sleep/work as explicit "advance until terminal or interruption."** Selecting `sleep.here`
   advances authoritative time to the sleep's scheduled completion (or earliest salient
   interruption per `:270`), applies recovery, and renders a record-derived "slept until …"
   summary. Requires defining interruption precedence and the actor-known summary surface.
3. **Enforce the body-exclusive reservation in the human path** regardless of which option is
   chosen, so `wait`/conflicting actions during an open duration behave coherently
   (`ReservationConflict` or continuation rather than silent acceptance).

## 8. Open questions to resolve in the brief

- What exactly does "waiting runs the simulation" mean for **other actors** in a single-actor
  TUI — full world tick, or just the possessed actor's own durations + perception?
- How is time-skip (sleep, time acceleration, travel time) reconciled with the **one-charge-
  per-tick** invariant and replay determinism?
- What is the **actor-known summary** surface for a slept-through interval, and how is
  hidden-truth leakage prevented (`08_TUI…:274/276`)?
- Should `sleep`/`work_block` remain submit-once-then-scheduler-completes, or become
  advance-to-terminal in one human step? Which composes better with `pause`/`step`/`stop on
  salient interruption` (`:261–270`)?
- Is the missing body-exclusive reservation enforcement in the human path in scope for the same
  spec, or a separate hardening item?

---

## 9. Evidence index (file:line)

- Doctrine: `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md:259,261-272,274,276`
- Invariants: `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (rows "0017 tick-charge attribution authority", "0017 shared open-duration authority")
- Sleep start/complete: `crates/tracewake-core/src/actions/defs/sleep.rs:18,167`
- Work start/complete: `crates/tracewake-core/src/actions/defs/work.rs:119,310`
- Scheduler completion engine: `crates/tracewake-core/src/scheduler.rs:212,344,686,691,996,1020,1052,1076`
- TUI human submit path (no completion call): `crates/tracewake-tui/src/app.rs` `submit_entry` (≈246–302), `run_no_human_day` (308)
- TUI wait dispatch: `crates/tracewake-tui/src/run.rs` (`UiCommand::WaitOneTick`)

## 10. Cross-reference

Found during the TUI bug-hunt loop documented in `/tmp/tracewake-tui-bugfixes.md` as
"FINDING #8". That loop fixed seven self-contained TUI defects (view duplication ×3, broken
`place`, broken `inspect`, two command-loop crash classes, notebook contradiction-link loss);
this finding is the first that is design/spec-level rather than a clean defect.
