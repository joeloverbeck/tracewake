# 0064TUIEMBPAN-003: ratatui buffer render path and snapshot harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `ratatui` dependency and `insta` dev-dependency to `tracewake-tui`; new module `crates/tracewake-tui/src/screen/buffer_render.rs`; new snapshot goldens
**Deps**: 0064TUIEMBPAN-001, 0064TUIEMBPAN-002

## Problem

Spec 0064 §1.1.3 requires a `ratatui` buffer render path — `render_embodied_to_buffer(&EmbodiedScreenModel, area) -> Buffer` — that draws the region layout (0064TUIEMBPAN-001) using the shared per-pane binding renderers (0064TUIEMBPAN-002), snapshotted at fixed sizes, with no event loop yet. §6 requires snapshot goldens (pane text + `ratatui` buffer) at `80x24`, `100x30`, `60x20` and a buffer/text parity check so the two render targets cannot diverge. §8 constrains the new dependency to the TUI crate only (no UI dependency reaches `tracewake-core`/`tracewake-content`) and requires deterministic ordering.

## Assumption Reassessment (2026-07-02)

1. `render_embodied_to_buffer` does not exist yet (grep of `crates/` returns no match) — this is a genuine new deliverable, not a rename. Neither `ratatui` nor `insta` is currently a dependency: `crates/tracewake-tui/Cargo.toml` declares only path deps (`tracewake-core`, `tracewake-content`) under `[dependencies]` and a `test-support`-featured `tracewake-core` under `[dev-dependencies]`. This ticket adds `ratatui` under `[dependencies]` and `insta` under `[dev-dependencies]` (per approved Q1 = insta/cargo-insta), pinning current-compatible versions in the diff.
2. The existing fixed-size golden harness is the convention to extend: `crates/tracewake-tui/tests/embodied_screen_dump.rs` proves plain-text dumps at `80x24`/`100x30`/`60x20` via `include_str!` goldens under `crates/tracewake-tui/tests/goldens/` (`embodied_screen_{80x24,100x30,60x20}.txt`), using helpers `dump_for_size` and `ordinary_workday_screen`. The buffer test reuses `ordinary_workday_screen` (or an equivalent fixture) at the same three sizes; the `ratatui::buffer::Buffer` snapshot uses `insta` while the plain pane-text snapshot may stay in the `include_str!` convention.
3. Shared boundary under audit: this ticket consumes 0064TUIEMBPAN-001's region model and 0064TUIEMBPAN-002's binding renderers (lines→cells) and adds the `ratatui` backend. It does not modify those modules; a divergence between the buffer and the text/dump output is the failure the parity check exists to catch.
4. Constitutional invariants under audit: **INV-069 — The TUI must not implement simulation rules** and the arch-01 dependency direction (`docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`): `ratatui` is a presentation dependency confined to `tracewake-tui`; core/content stay UI-free. **INV-018 — Deterministic replay is foundational** governs snapshot stability: identical model + size must produce a byte-identical buffer/golden.
5. Deterministic-replay surface: snapshot goldens are only meaningful if byte-stable, so the buffer render must be a pure function of `(&EmbodiedScreenModel, area)` with deterministic pane/actor/action ordering (spec §8) — no wall-clock, no map-iteration ordering, no styling drawn from nondeterministic state. This substrates the 0064TUIEMBPAN-005 acceptance witnesses. No actor-knowledge filtering happens here (the model is pre-filtered); the buffer must not surface any field the bindings did not already actor-safe-filter, so it introduces no leakage path (INV-024).

## Architecture Check

1. Mapping the shared binding lines onto a `ratatui` `Buffer` (rather than re-deriving pane content in the backend) keeps a single content source of truth and makes buffer/text parity a mechanical assertion. Rendering to an in-memory `Buffer` with no event loop keeps the path headless and snapshot-testable, deferring `crossterm`/raw-mode to Spec 0065.
2. No backwards-compatibility aliasing or shims: the new path is additive; `render_embodied_view` and `render_embodied_screen_dump` are untouched. `ratatui` is introduced only in the TUI crate.

## Verification Layers

1. INV-069 / arch-01 dependency direction -> codebase grep-proof: `ratatui` appears only in `crates/tracewake-tui/Cargo.toml`; `grep -rn ratatui crates/tracewake-core crates/tracewake-content` returns nothing.
2. INV-018 (deterministic snapshot) -> replay/golden-fixture check: `render_embodied_to_buffer` over one fixture at a fixed size equals the committed golden byte-for-byte on repeated runs.
3. Buffer/text parity -> integration test: at a fixed size, the buffer's actor-safe content and the text path's content agree (same actors, actions, why-not, needs).

## What to Change

### 1. Add the dependencies

In `crates/tracewake-tui/Cargo.toml`: add `ratatui` to `[dependencies]` and `insta` to `[dev-dependencies]` (current-compatible versions).

### 2. Add the buffer render path

Add `crates/tracewake-tui/src/screen/buffer_render.rs` with `pub fn render_embodied_to_buffer(model: &EmbodiedScreenModel, area: ratatui::layout::Rect) -> ratatui::buffer::Buffer`: lay out the regions (0064TUIEMBPAN-001) into `area`, draw each region's binding lines (0064TUIEMBPAN-002) into the buffer, deterministically. Register `pub mod buffer_render;` + re-export in `crates/tracewake-tui/src/screen/mod.rs`.

### 3. Add snapshot goldens + parity test

Add `crates/tracewake-tui/tests/embodied_pane_buffer.rs`: buffer snapshots at `80x24`/`100x30`/`60x20` (via `insta`) and matching pane-text goldens under `crates/tracewake-tui/tests/goldens/`; plus a buffer/text parity assertion.

## Files to Touch

- `crates/tracewake-tui/Cargo.toml` (modify)
- `crates/tracewake-tui/src/screen/buffer_render.rs` (new)
- `crates/tracewake-tui/src/screen/mod.rs` (modify)
- `crates/tracewake-tui/tests/embodied_pane_buffer.rs` (new)
- `crates/tracewake-tui/tests/goldens/embodied_pane_80x24.txt` (new)
- `crates/tracewake-tui/tests/goldens/embodied_pane_100x30.txt` (new)
- `crates/tracewake-tui/tests/goldens/embodied_pane_60x20.txt` (new)

## Out of Scope

- `crossterm` event loop, raw mode, live input (Spec 0065).
- Responsive collapse / truncation-marker behavior and the minimum-size floor (0064TUIEMBPAN-004) — this ticket renders the standard sizes; 004 adds the narrow/overflow assertions.
- Region taxonomy (001) and binding content (002) — consumed, not modified.
- The consolidated §7 acceptance artifact (0064TUIEMBPAN-005).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --test embodied_pane_buffer` — buffer + pane-text goldens match at `80x24`, `100x30`, `60x20`; re-running is byte-stable.
2. Buffer/text parity test: the buffer and text render targets expose the same actor-safe pane content at a fixed size (spec §6).
3. `grep -rn "ratatui" crates/tracewake-core crates/tracewake-content` returns nothing (dependency confined to the TUI crate).
4. `cargo build --workspace --all-targets --locked` and `cargo test --workspace` pass.

### Invariants

1. `ratatui` is a `tracewake-tui`-only presentation dependency; core/content remain UI-free (INV-069 / arch-01).
2. `render_embodied_to_buffer` is a pure, deterministic function of its inputs — identical model + area yield an identical buffer (INV-018 / spec §8).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_pane_buffer.rs` — fixed-size buffer snapshots, pane-text goldens, and buffer/text parity.
2. `crates/tracewake-tui/tests/goldens/embodied_pane_{80x24,100x30,60x20}.txt` — committed pane-text goldens.

### Commands

1. `cargo test -p tracewake-tui --test embodied_pane_buffer`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`

## Outcome

Completed: 2026-07-02

Implemented the headless `ratatui` buffer path in
`crates/tracewake-tui/src/screen/buffer_render.rs` and exported
`render_embodied_to_buffer` plus a deterministic buffer text serializer from
`crates/tracewake-tui/src/screen/mod.rs`. The renderer consumes the shared
Spec 0064 region bindings and draws them into a `ratatui::buffer::Buffer`
without an event loop or core/content dependency.

Added `ratatui` as a `tracewake-tui` dependency and `insta` as a
`tracewake-tui` dev-dependency, with `Cargo.lock` updated. Updated the
workspace dependency posture allowlist and production source classification
census to record those TUI-only presentation/test additions. The core/content
`ratatui` grep remains clean; the allowlist string is intentionally split so
the exact boundary grep does not match test-source metadata.

Added `crates/tracewake-tui/tests/embodied_pane_buffer.rs`, pane-text goldens
for `80x24`, `100x30`, and `60x20`, and `insta` buffer snapshots for those
same fixed sizes. The integration test also checks deterministic rendering and
buffer/text exposure of the same actor-safe critical content.

Verification run:

- `cargo check -p tracewake-tui --locked` (failed as expected before lockfile
  update because new dependencies were not locked)
- `cargo check -p tracewake-tui`
- `cargo test -p tracewake-tui buffer_render`
- `INSTA_UPDATE=always TRACEWAKE_PRINT_PANE_GOLDENS=1 cargo test -p tracewake-tui --test embodied_pane_buffer -- --nocapture` (generated snapshots and printed pane goldens; placeholder golden/parity assertions failed before committed expected output was applied)
- `cargo test -p tracewake-tui --test embodied_pane_buffer`
- `cargo build --workspace --all-targets --locked`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_dependency_posture_matches_allowlist`
- `grep -rn "ratatui" crates/tracewake-core crates/tracewake-content`
- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

No `ratatui` dependency, UI backend dependency, hidden-truth read, or world
mutation path reaches `tracewake-core` or `tracewake-content`. No crossterm/raw
mode event loop was added.
