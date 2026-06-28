# Tracewake

A causality-first living-world simulation in Rust: agents act from partial belief,
institutions are fallible, and every meaningful change is an event that leaves a
replayable trace. There is no quest engine and no privileged protagonist — the world
does not know which actor, if any, a human controls.

Phase 1 has landed: a runnable physical/action/event/TUI/replay spine. A human can bind
a controller to an ordinary actor, view a place through that actor, take stable semantic
actions (move, inspect, open/close, take/place, wait), see structured reasons when an
action is impossible, inspect debug provenance, rebuild projections from a fixture plus
ordered events, and replay deterministically to the same physical result.

Phase 2A has landed as the first executable epistemic slice, not all of Phase 2. It adds
typed propositions, observations, source-backed beliefs, expectation contradictions,
actor-known notebooks, debug epistemics, possession parity fixtures, and a bounded
low-confidence sound-observation slice. Richer memory decay, reports, records,
institutions, gossip, wrong suspicion workflows, dialogue, and graphical client work remain
deferred.

Phase 3A has landed as the ordinary-life substrate slice. Actors now have bounded hunger,
fatigue, and safety needs; durable intentions; authored routine duties; local planning;
ordinary sleep/eat/work/continue/wait actions through the shared pipeline; no-human day
markers and metrics; and non-diegetic debug panels for needs, routines, planner traces,
stuck diagnostics, and no-human day summaries. It is still not a smart-dialogue,
institution, or quest phase.

## Workspace layout

A Cargo workspace of three crates with a strict one-way dependency direction
(`tui` → `content` → `core`); core must never depend on the layers above it.

| Crate | Role | Depends on |
|---|---|---|
| `crates/tracewake-core` | Authoritative simulation kernel: event log, replay, actions/affordances, scheduler, projections, view models. | nothing |
| `crates/tracewake-content` | Fixtures, content loading, schema validation. | core |
| `crates/tracewake-tui` | Terminal UI boundary: possession, embodied/debug view models. | core + content |

## Prerequisites

The toolchain is pinned in `rust-toolchain.toml` (Rust `1.93.0`, edition 2021, with the
`rustfmt` and `clippy` components). `rustup` installs it automatically on the first
`cargo` invocation, so no manual setup is required beyond having `rustup` available.

## Build, test, lint

Four gates define "done." CI runs all four, and they must pass locally before any change
is considered complete. CI treats warnings as errors (`RUSTFLAGS: -D warnings`), so run
them exactly as written:

```sh
cargo fmt --all --check                                      # formatting
cargo clippy --workspace --all-targets -- -D warnings        # lints (warnings = errors)
cargo build --workspace --all-targets --locked               # build (lockfile pinned)
cargo test --workspace                                        # tests
```

During iteration you can scope to a single crate — e.g. `cargo test -p tracewake-core`
or `cargo clippy -p tracewake-tui --all-targets -- -D warnings` — but run the full
workspace gates above before claiming completion.

> Note: this is a Rust project; there is no TypeScript. The "lanes" are the four `cargo`
> gates above (the Rust analog of a lint/type/test pipeline).

### Continuous integration

`.github/workflows/ci.yml` runs on every push to `main` and every pull request, as three
parallel jobs that mirror the local gates:

| Job | Command |
|---|---|
| `rustfmt` | `cargo fmt --all --check` |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `build & test` | `cargo build --workspace --all-targets --locked` then `cargo test --workspace --locked` |

Superseded runs on the same ref are cancelled in flight, and the cargo registry/`target`
directories are cached for the clippy and test jobs.

## Running the TUI

Run the executable TUI with the default fixture and actor:

```text
cargo run -p tracewake-tui
```

The no-arg launch still loads `strongbox_001`, binds `actor_tomas`, prints
`tracewake-tui ready`, renders the initial embodied view, and enters a simple stdin/stdout
command loop.

Select another fixture, and optionally an actor, with:

```text
cargo run -p tracewake-tui -- <fixture_id> [actor_id]
```

Launch an explicit non-diegetic operator/debug session with:

```text
cargo run -p tracewake-tui -- --operator-debug <fixture_id> [actor_id]
```

List the authored fixture catalog and actor ids with:

```text
cargo run -p tracewake-tui -- --list
```

If no actor id is supplied, the TUI binds the first authored actor in the fixture. No-human
fixtures such as `no_human_day_001` can be advanced from an operator/debug launch with
`debug run no-human-day`, then inspected through the non-diegetic debug panels, including
`debug no-human-day` and `debug planner <actor_id>`.

This is intentionally plain text. It is the operating surface for the current kernel,
epistemic, and ordinary-life slices, not a graphical client.

### Commands

```text
help                         show command help
view                         re-render the current embodied view
bind <actor_id>              bind the controller to an actor
<n>                          submit the action at the 1-based menu position from the current view
do <semantic_action_id>      submit a stable semantic action ID shown in the action menu
notebook                     show the current actor's source-backed notebook projection
wait                         wait one tick through the ordinary action pipeline
w                            alias for wait
debug log                    show the non-diegetic event log panel
debug bindings               show non-diegetic controller bindings
debug item <item_id>         show a non-diegetic item-location report
debug rejection              show the latest non-diegetic rejection report, if any
debug projection             rebuild projection from the event log and show the debug report
debug replay                 replay the log and show the debug report
debug epistemics             show non-diegetic epistemic projection counts and records
debug beliefs <actor_id>     show non-diegetic beliefs for one actor
debug observations <actor_id> show non-diegetic observations for one actor
debug needs                  show non-diegetic Phase 3A needs for all actors
debug routines               show non-diegetic Phase 3A intentions and routine executions
debug planner <actor_id>     show non-diegetic planner traces and blocked preconditions
debug stuck                  show non-diegetic stuck diagnostics
debug no-human-day           show non-diegetic no-human day metrics
debug run no-human-day       advance the loaded fixture through the gated no-human day operator proof
debug actor <actor_id>       show one actor's non-diegetic needs, routine, trace, and stuck rows
quit                         exit
q                            exit
```

The numbered menu position is never an action identity. Numeric input resolves through the current view to the stable semantic action ID printed beside the action.

`check_container` is the Phase 2A search/check action. In the default strongbox fixture,
the embodied action menu includes the stable semantic action ID
`check.container.strongbox_tomas`. Open physical access if required, then check/search;
the check action records a channel-backed observation and can contradict Tomas's authored
expectation in absence-oriented fixtures without creating culprit knowledge.

Debug panels are marked non-diegetic. They may reveal debug truth, but returning to `view` shows only the actor-filtered embodied view.

### Phase 3A ordinary-life surface

Needs are bounded values for `hunger`, `fatigue`, and `safety`, grouped into bands
(`comfortable`, `rising`, `urgent`, `severe`). A possessed actor sees only their
actor-known need/status summary in the embodied view; debug panels may reveal full
operator truth.

Ordinary-life actions are registered as stable action families: `sleep`, `eat`,
`work_block`, `continue_routine`, and `wait`. The TUI exposes them as semantic action
IDs such as `sleep.here`, `eat.<food_supply_id>`, `work.<workplace_id>`,
`continue.routine.<intention_id>`, and `wait.1_tick` when the current actor and fixture
state make them applicable. They still submit proposals through the same validation,
event-log, replay, and why-not machinery as movement, open/close, take/place, and check.

Routines are authored as defeasible templates and assignments in content fixtures. They
can create durable intentions, select routine methods, emit concrete action proposals,
wait with a modeled reason, or record typed stuck diagnostics when progress is blocked.
The canonical boring-day fixture is `no_human_day_001`; debug with
`debug run no-human-day`, then `debug no-human-day`, `debug planner actor_mara`, `debug stuck`, and
`debug actor actor_tomas` to inspect the no-human metrics, Mara's food replan trace, and
stuck actor rows without feeding that truth back into embodied knowledge.

### Example session shape

Exact action order and labels may change as fixtures evolve, but the command loop supports this shape:

```text
$ cargo run -p tracewake-tui
tracewake-tui ready
Actor: actor_tomas | Tick: 0
Place: ...
Actions:
1. ... [check.container.strongbox_tomas]
2. ... [wait.1_tick]
tracewake>
do open.container.strongbox_tomas
Accepted: open.container.strongbox_tomas
Actor: actor_tomas | Tick: ...
...
tracewake>
do check.container.strongbox_tomas
Accepted: check.container.strongbox_tomas
Actor: actor_tomas | Tick: ...
Knowledge context: knowledge.actor_tomas...
...
tracewake>
notebook
Notebook: actor_tomas
Beliefs:
...
Observations:
...
Contradictions:
...
tracewake>
view
Actor: actor_tomas | Tick: ...
...
tracewake>
do wait.1_tick
Accepted: wait.1_tick
Actor: actor_tomas | Tick: ...
...
tracewake>
debug log
DEBUG NON-DIEGETIC: Event Log
...
tracewake>
debug item coin_stack_01
DEBUG NON-DIEGETIC: Item Location
...
tracewake>
debug epistemics
DEBUG NON-DIEGETIC: Epistemics
...
tracewake>
debug beliefs actor_tomas
DEBUG NON-DIEGETIC: Beliefs
...
tracewake>
debug observations actor_tomas
DEBUG NON-DIEGETIC: Observations
...
tracewake>
quit
```

The embodied portion of the session shows Tomas discovering absence through check/search
and notebook projection. It does not print ground-truth culprit knowledge. Debug panels
remain explicitly non-diegetic.

All ordinary world and epistemic changes go through the shared semantic
action/proposal/pipeline path. The TUI does not directly mutate world state.

## Fixture Contracts

Fixture contracts live with the fixture constructors under
`crates/tracewake-content/src/fixtures/`. Phase 1 fixture contracts remain intact:
physical setup, allowed ordinary actions, expected event/debug/replay artifacts, and
acceptance assertions are still declared for the original seven fixtures.

Phase 2A adds these fixture contracts:

| Fixture | Contract focus |
|---|---|
| `strongbox_001` | Phase 1 physical baseline plus Tomas's authored-prehistory expectation that `coin_stack_01` is in `strongbox_tomas`. |
| `expectation_contradiction_001` | Checking an empty strongbox can record absence, contradiction, and a missing-property belief without naming a culprit. |
| `possession_parity_001` | `actor_mara` is an ordinary actor using the same take/place/check actions; no theft-only shortcut exists. |
| `view_filtering_001` | Actor-private beliefs are visible through actor-known notebook projections and separated from debug truth. |
| `knowledge_blocker_accuse_001` | Accusation probing is blocked by actor-known knowledge preconditions, not by authored culprit flags. |
| `sound_uncertainty_001` | Simple sound evidence is low-confidence and uncertain; it does not become theft knowledge. |
| `no_human_epistemic_check_001` | Epistemic checks are ordinary actor-relative actions and do not require a privileged human controller. |

Phase 3A adds these fixture contracts:

| Fixture | Contract focus |
|---|---|
| `ordinary_workday_001` | Routine windows can drive an ordinary workday path. |
| `sleep_eat_work_001` | Sleep, eat, and work actions affect bounded needs through the shared pipeline. |
| `food_unavailable_replan_001` | Missing accessible food causes a replan/fallback instead of hidden-truth planning. |
| `routine_blocked_diagnostic_001` | Blocked routines record typed stuck diagnostics rather than looping silently. |
| `planner_trace_001` | Candidate goals, selected methods, rejected reasons, and hidden-truth audit rows are traceable. |
| `routine_no_teleport_001` | Routines cannot start remote work without movement ancestry. |
| `possession_does_not_reset_intention_001` | Binding a controller does not reset another actor's needs, intention, or routine execution. |
| `no_hidden_truth_planning_001` | Planner selection uses actor-known inputs, not omniscient fixture truth. |
| `no_human_day_001` | Canonical no-human ordinary day with needs, routines, diagnostics, replayable metrics, and debug inspection. |

Forbidden shortcut fields such as `culprit`, `stolen_flag`, `npc_knows_truth`,
`quest_state`, and `player_memory` are rejected by content validation.

## Where to go next

- `docs/` — layered documentation authority, indexed by `docs/README.md`:
  `0-foundation` → `1-architecture` → `2-execution` → `3-reference`. Earlier tiers govern
  later ones. The non-negotiable contract lives in
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
- `specs/` and `docs/4-specs/` — system specs; routed via `docs/4-specs/SPEC_LEDGER.md`.
- `tickets/` — active implementation tickets (`tickets/README.md`, `tickets/_TEMPLATE.md`).
- `archive/` — completed specs and tickets (e.g. the landed Phase 1 spec under
  `archive/specs/`).
- `CLAUDE.md` / `AGENTS.md` — contributor and agent working conventions.

## License

See [`LICENSE`](LICENSE) (GNU GPL v3).
