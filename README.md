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

Run the Phase 1 executable TUI with:

```text
cargo run -p tracewake-tui
```

The binary loads the default `strongbox_001` fixture, binds `actor_tomas`, prints `tracewake-tui ready`, renders the initial embodied view, and enters a simple stdin/stdout command loop.

This is intentionally plain text. It is the Phase 1 operating surface for the current kernel and view-model slice, not a graphical client.

### Commands

```text
help                         show command help
view                         re-render the current embodied view
bind <actor_id>              bind the controller to an actor
<n>                          submit the action at the 1-based menu position from the current view
do <semantic_action_id>      submit a stable semantic action ID shown in the action menu
wait                         wait one tick through the ordinary action pipeline
w                            alias for wait
debug log                    show the non-diegetic event log panel
debug bindings               show non-diegetic controller bindings
debug item <item_id>         show a non-diegetic item-location report
debug rejection              show the latest non-diegetic rejection report, if any
debug projection             rebuild projection from the event log and show the debug report
debug replay                 replay the log and show the debug report
quit                         exit
q                            exit
```

The numbered menu position is never an action identity. Numeric input resolves through the current view to the stable semantic action ID printed beside the action.

Debug panels are marked non-diegetic. They may reveal debug truth, but returning to `view` shows only the actor-filtered embodied view.

### Example session shape

Exact action order and labels may change as fixtures evolve, but the command loop supports this shape:

```text
$ cargo run -p tracewake-tui
tracewake-tui ready
Actor: actor_tomas | Tick: 0
Place: ...
Actions:
1. ... [open.container.strongbox_tomas]
2. ... [wait.1_tick]
tracewake> 1
Accepted.
Actor: actor_tomas | Tick: ...
...
tracewake> view
Actor: actor_tomas | Tick: ...
...
tracewake> do wait.1_tick
Accepted.
Actor: actor_tomas | Tick: ...
...
tracewake> debug log
DEBUG NON-DIEGETIC: Event Log
...
tracewake> debug item coin_stack_01
DEBUG NON-DIEGETIC: Item Location
...
tracewake> quit
```

All ordinary world changes go through the shared semantic action/proposal/pipeline path. The TUI does not directly mutate world state.

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
