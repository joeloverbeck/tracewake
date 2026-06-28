# Tracewake

Causality-first living-world simulation in Rust: agents act from partial belief,
institutions are fallible, and every event leaves replayable traces.

## Documentation authority

Docs are layered authority — earlier tiers govern later ones. Index: `docs/README.md`.
`0-foundation` → `1-architecture` → `2-execution` → `3-reference`. If execution conflicts
with architecture or foundation, execution is wrong; if implementation is more convenient
than the accepted gates, implementation is wrong.

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001`…`INV-110`, the
  non-negotiable contract. Every product-behavior change must satisfy it. A change that
  genuinely needs to diverge requires amending an invariant first — never design against
  it silently.

Recurring test for every feature: preserve event-sourced causality, subjective epistemics,
ordinary agents, possession parity, fallible institutions, questless leads, TUI-first
playability, validation/replay, and no simulation fact born from prose.

## Workspace layout

Cargo workspace, three crates with strict one-way dependency direction:

- `tracewake-core` — authoritative simulation kernel: event log, replay, actions/
  affordances, scheduler, projections, view models. **Zero dependencies.**
- `tracewake-content` — fixtures, content loading, schema validation. Depends on core only.
- `tracewake-tui` — terminal UI boundary: possession, embodied/debug view models.
  Depends on core + content.

Never invert this: core must not depend on content or tui.

## Build, test, lint

Toolchain is pinned in `rust-toolchain.toml` (1.93.0, rustfmt + clippy). CI treats all
warnings as errors. Before claiming any change complete, all four must pass:

```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

Run the TUI: `cargo run -p tracewake-tui`.

## Where work lives

- `specs/`, `docs/4-specs/NNNN_*.md` — system specs; recorded in `docs/4-specs/SPEC_LEDGER.md`.
- `tickets/` — active implementation tickets. Author from `tickets/_TEMPLATE.md`; the
  authoring contract is `tickets/README.md`.
- `archive/` — completed specs and tickets.

## Conventions

- No backwards-compatibility shims or alias paths in new work.
- If current code and a ticket/spec diverge, correct the ticket/spec first, then implement.

## Disk hygiene

Local `cargo mutants` runs are the main disk hazard: each parallel build job copies
the whole workspace (with its own `target/`) into `$TMPDIR` and deletes it only on a
clean exit, so an interrupted run (Ctrl-C / OOM / timeout) leaks multi-GB scratch
dirs. Stale `/tmp` git worktrees and compile-fail fixture `target/` dirs add to it.

- Reclaim leaked scratch with `tools/clean-build-scratch.sh` (dry-run by default;
  `--force` to delete).
- **WSL2**: the ext4 `.vhdx` grows to its high-water mark and never shrinks on its
  own, so freeing space inside WSL does not return it to `C:`. Reclaiming `C:`
  requires a **manual, human-run** Windows-side compact that first shuts WSL down —
  this is NOT an agent action (it would kill every running WSL session). Do not run
  `wsl --shutdown` or `diskpart`/`Optimize-VHD` from a session; the recipe for the
  human is in `tools/clean-build-scratch.sh --help`.
