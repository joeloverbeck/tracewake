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

```sh
cargo run -p tracewake-tui
```

**Today** the binary loads the `strongbox_001` golden fixture, binds the default actor
(`actor_tomas`), prints a readiness line (`tracewake-tui ready`), renders **one** embodied
frame of that actor's place, and exits. The frame shows the actor and tick, the current
place, any "why-not" rejection summary, and the visible exits, doors, containers, items,
local actors, and the numbered list of available semantic actions.

### Command vocabulary and debug panels (implemented, not yet wired into the binary)

The interactive operating surface is implemented and tested at the library level
(`crates/tracewake-tui/src/`), but `main.rs` does not yet drive an input loop — it renders
a single frame. The pieces ready to be wired up are:

Command parser (`input.rs`):

| Input | Effect |
|---|---|
| `bind <actor_id>` | Bind the controller to an actor |
| `do <semantic_action_id>` | Submit a stable semantic action by id |
| `<n>` | Submit the action at 1-based menu position `n` (resolves to its stable semantic id) |
| `wait` / `w` | Advance one tick |
| `quit` / `q` | Exit |

Selections resolve to the action's *stable semantic id*, never the menu index, so a menu
re-order never changes which action a given command performs.

Six debug panels (`debug_panels.rs`, surfaced via `TuiApp::render_debug_*`): event log,
controller binding, item location, action rejection, projection rebuild, and replay.

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
