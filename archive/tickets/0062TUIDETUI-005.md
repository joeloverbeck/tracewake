# 0062TUIDETUI-005: Headless runner and transcript emission

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` headless runner (`intent/headless.rs`); registers in `intent/mod.rs`; consumes the landed Spec 0061 screen-dump seam and `transcript.rs`. No core types, no event/schema/view-model changes.
**Deps**: 0062TUIDETUI-002, 0062TUIDETUI-003, 0062TUIDETUI-004.

## Problem

Spec 0062 §1.1 item 5 requires a headless runner that runs either a key script or a semantic script against a `TuiApp` with no raw-mode TTY, emitting the Spec 0061 screen dumps plus a transcript that includes mode, bound actor, terminal size, key/intent inputs, semantic action ids, rendered pane text, and debug markers (§4.4). This makes CI need no terminal and lets scripted agents drive the same authority a human does.

## Assumption Reassessment (2026-07-01)

1. **Landed screen-dump seam (code).** Spec 0061 landed `render_embodied_screen_dump(&EmbodiedScreenModel) -> String`, `EmbodiedScreenModel`, `build_embodied_screen_model`, and structured `ScreenDump` under `crates/tracewake-tui/src/screen/` (`mod.rs`, `text_dump.rs`, `struct_dump.rs`). The runner renders through this seam; it does not add a new dump path. (0061 is accepted/archived — landed code, not a pending ticket dependency.)
2. **Transcript surface (code).** `crates/tracewake-tui/src/transcript.rs` carries the existing transcript emission. The runner extends the emitted transcript **additively** with the scripted key/intent inputs and semantic action ids required by §1.1.5; this is a TUI-only presentation/evidence artifact (not a core serialized or replayed schema), so it introduces no event-schema evolution obligation. Confirm current transcript fields before adding, and keep additions append-only.
3. **Cross-artifact boundary under audit.** The runner composes the reducer (0062TUIDETUI-002) with both parsers (003, 004): it parses a script to `UiIntent`, applies each through the reducer against a `TuiApp` + `PresentationState`, and renders a dump/transcript after each. It exercises the same authority a human does (INV-108) with no raw-mode TTY.
4. **Motivating invariants (INV-095, INV-018, INV-024).** `INV-095` ("TUI/view-model tests are acceptance tests") — the runner's dumps/transcripts are acceptance evidence. `INV-018` — same script → byte-stable dumps/transcripts. `INV-024` (no telepathy) — embodied dumps must carry no debug-only world truth.
5. **No-leak / determinism enforcement surface.** The runner is a projection/evidence surface touching view-model rendering and transcripts. It must not leak hidden information: embodied-mode dumps carry only actor-known view-model content (a debug *marker* flag is allowed; debug-only world truth is not), and debug dumps stay separate and visibly marked (report §6.2). Determinism: identical scripts must yield byte-identical dumps and transcripts — no wall-clock, no hash-map iteration order, no nondeterministic input in the rendered form. No new authoritative/replay surface is added; the runner reuses the kernel-routed reducer and the deterministic 0061 dump.

## Architecture Check

1. A headless runner that surrounds the pure core→view-model→screen-model→dump chain as an input/output adapter (report §5.5) preserves the settled pure-render seam and makes CI terminal-free — cleaner than embedding script-driving in the interactive loop. Reusing the landed 0061 dump and existing `transcript.rs` avoids a parallel rendering path.
2. No backwards-compatibility aliasing or shims; no `crossterm`, no raw mode (spec §1.2, §8).

## Verification Layers

1. INV-018 (determinism) -> replay/golden check: the same script produces byte-identical dumps and transcript across runs.
2. INV-024 (no telepathy) -> test + manual review: embodied dumps contain no debug-only world truth; debug output is separate and marked.
3. INV-095 (TUI tests are acceptance) -> test: the runner drives wait/continue/action/focus/selection/notebook and emits the asserted dumps and transcript sections.

## What to Change

### 1. Add the headless runner

Create `crates/tracewake-tui/src/intent/headless.rs` with a runner that: (a) accepts a key or semantic script and the chosen parser; (b) parses it to `Vec<UiIntent>`; (c) applies each intent via `intent::reducer::reduce` against a `TuiApp` + `PresentationState`; (d) after each step renders the 0061 screen dump and appends a transcript section (mode, bound actor, terminal size, key/intent input, semantic action id, rendered pane text, debug markers). No raw-mode TTY.

### 2. Extend transcript emission additively

Add the scripted key/intent inputs and semantic action ids to the transcript in `crates/tracewake-tui/src/transcript.rs` as append-only fields, consistent with Architecture 10's transcript-obligation list. Keep debug-only world truth out of embodied transcripts/dumps.

### 3. Register the module

Add `pub mod headless;` to `crates/tracewake-tui/src/intent/mod.rs` (created by 0062TUIDETUI-001).

## Files to Touch

- `crates/tracewake-tui/src/intent/headless.rs` (new)
- `crates/tracewake-tui/src/intent/mod.rs` (modify — file created by 0062TUIDETUI-001)
- `crates/tracewake-tui/src/transcript.rs` (modify — additive transcript fields for scripted intent inputs)

## Out of Scope

- The `UiIntent` enum / presentation-state model (001), reducer (002), and parsers (003, 004).
- The conformance/fixture tests and dump-equivalence proof (0062TUIDETUI-006).
- Any `crossterm` event loop or raw-mode TTY (Spec 0065).
- Changing the 0061 screen-dump shape or the 0047 time-control contract (spec §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. A test runs a script through the runner with no TTY and asserts a screen dump plus a transcript containing mode, bound actor, terminal size, intent inputs, semantic action ids, and rendered pane text.
2. A determinism test: the same script yields byte-identical dumps and transcript across two runs.
3. A no-leak test: an embodied-mode dump/transcript contains no debug-only world truth.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`.

### Invariants

1. Identical scripts yield byte-identical dumps and transcripts (INV-018).
2. Embodied dumps/transcripts leak no debug-only world truth (INV-024); dumps are acceptance evidence (INV-095).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/` (or `intent/headless.rs` unit tests) — headless run producing dump + transcript, determinism across runs, and an embodied no-leak assertion; rationale: proves terminal-free drivability and the no-leak/determinism contract.

### Commands

1. `cargo test -p tracewake-tui headless`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-01

Implemented the TUI-only headless intent runner in `intent::headless`. The
runner accepts key or semantic scripts, parses them through the ticket-003/004
parsers, applies each `UiIntent` through the single reducer against
`TuiApp` + `PresentationState`, and renders the landed Spec 0061
`EmbodiedScreenModel` / text dump after each step with no raw-mode TTY.

Extended transcript emission additively with `ScriptedTranscriptStep` and
`ScriptedInputSource` in `transcript.rs`. Scripted transcript sections now carry
input source, raw input, intent text, semantic action id when present, mode,
bound actor, terminal size, focused pane, debug marker presence, separate debug
output, and rendered pane text. Embodied dumps remain actor-filtered screen
dumps; debug output is separate and marked.

The runner adds no core type, event/schema change, world-mutation path, or
hidden-truth read. The new production source file was classified in the
workspace source census with the existing TUI presentation-boundary rationale.

Verification:

- `cargo test -p tracewake-tui headless`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p tracewake-tui`
