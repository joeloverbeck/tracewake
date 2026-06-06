# Tracewake Codex Guidance

Tracewake is a causality-first living-world simulation in Rust. Agents act from
partial belief, institutions are fallible, and every event must leave replayable
traces.

## Authority

Documentation authority is layered in `docs/README.md`:
`0-foundation` -> `1-architecture` -> `2-execution` -> `3-reference`.
Earlier tiers govern later tiers. If implementation conflicts with accepted
docs or gates, fix the implementation or amend the governing doc first.

For product-behavior work, start from
`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`. Every feature must preserve
event-sourced causality, subjective epistemics, ordinary agents, possession
parity, fallible institutions, questless leads, TUI-first playability,
validation/replay, and no simulation fact born from prose.

## Workspace

This is a Cargo workspace pinned by `rust-toolchain.toml`:

- `crates/tracewake-core`: authoritative simulation kernel. Zero dependencies.
- `crates/tracewake-content`: fixtures, content loading, schema validation.
  Depends on core only.
- `crates/tracewake-tui`: terminal UI boundary. Depends on core and content.

Never invert dependency direction: core must not depend on content or tui.

## Verification

Before claiming a code change complete, run the relevant targeted checks and, for
full completion, these gates:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

Run the TUI with `cargo run -p tracewake-tui`.

## Tickets And Specs

- Active tickets live in `tickets/`; use `tickets/_TEMPLATE.md` and
  `tickets/README.md` for ticket authoring and reassessment expectations.
- Specs live in `specs/` and `docs/4-specs/`; consult
  `docs/4-specs/SPEC_LEDGER.md` when routing spec-led work.
- Completed specs and tickets move to `archive/`; follow
  `docs/archival-workflow.md`.

If current code and a ticket/spec diverge, correct the ticket/spec first, then
implement.

## Conventions

- Do not add backwards-compatibility shims or alias paths in new work.
- Keep changes aligned with the documented authority order and crate ownership.
- Prefer narrow, truthful verification over broad claims that were not run.
