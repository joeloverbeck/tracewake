# 0003PHA1AEXETUI-001: Extend command parser for help, view, numeric, and debug commands

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — extends `tracewake-tui` `input.rs` (`UiCommand`, `parse_command`, `InputError`) and adds a `DebugCommand` enum; in-file unit tests only.
**Deps**: None

## Problem

The executable command loop (0003PHA1AEXETUI-002) needs to classify the full Phase 1A command vocabulary. Today `parse_command` (`crates/tracewake-tui/src/input.rs:21-43`) recognizes only `bind`, `do`, `wait`/`w`, and `quit`/`q`. The spec's §"Minimum commands" also requires `help`, `view`, bare numeric `<n>`, and the `debug …` family. Numeric selection currently exists only as the `semantic_id_for_selection` helper (`input.rs:45-56`), not as a parsed command — exactly the gap the spec §"Numeric selection requirement" calls out.

## Assumption Reassessment (2026-06-06)

1. `parse_command` and `UiCommand` live in `crates/tracewake-tui/src/input.rs` (read this session): `UiCommand` has four variants (`BindActor`, `SelectSemanticAction`, `WaitOneTick`, `Quit`); `parse_command` is prefix/equality based. `semantic_id_for_selection(view, one_based)` already maps a 1-based menu index to a stable `SemanticActionId` (`input.rs:45-56`) — this ticket does NOT duplicate that resolution; it only adds a variant that carries the parsed index.
2. Spec `specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` §"Minimum commands" and §"Numeric selection requirement" authorize numeric handling "in `parse_command`, in a loop-level pre-parser, or through a new parser function." This ticket places classification in `parse_command`; identity resolution stays in 0003PHA1AEXETUI-002.
3. Shared boundary under audit: the `UiCommand` / new `DebugCommand` enum is the producer/consumer contract between this ticket (producer) and 0003PHA1AEXETUI-002 (the loop, consumer). `debug item <item_id>` carries a `tracewake_core::ids::ItemId` (already used in `app.rs:240`).
4. INV-008 (UI assistance is not authority) motivates the design — restated: menus and numeric selection "may clarify [but] may not create facts … or bypass preconditions." The parser must therefore classify a bare number as a *menu position*, never as action identity; identity resolution is deferred to the view-grounded `semantic_id_for_selection`. (INV-008 heading confirmed at `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:41`.)

## Architecture Check

1. Extending the existing `parse_command` keeps a single input-classification surface that the loop delegates to, matching spec §"Required implementation shape" ("only classify user input and delegate to `TuiApp` or debug render accessors"). A typed `DebugCommand` sub-enum keeps debug verbs structured rather than re-parsed inside the loop.
2. No backwards-compatibility shims: variants are added to `UiCommand`; existing variants and their string spellings are unchanged; no aliasing.

## Verification Layers

1. INV-008 (menu index ≠ action identity) -> codebase grep-proof + unit test: `parse_command("1")` yields a position-bearing variant (`SelectByMenuIndex(usize)`), not a `SemanticActionId`; mirrors the existing `selection_returns_semantic_action_id_not_menu_index` guard (`input.rs:101-107`).
2. Parser totality -> unit tests asserting every §"Minimum commands" string maps to a `UiCommand` or a typed `InputError`, including `debug item <id>` and an unknown `debug` subcommand.
3. Single-classification-layer ticket: parsing is unit-testable in isolation; the integration proof (dispatch, numeric resolution against a live view) lives in 0003PHA1AEXETUI-002 and the binary tests in 0003PHA1AEXETUI-003 — named here, not collapsed into this ticket.

## What to Change

### 1. Extend `UiCommand` and add `DebugCommand`

Add `UiCommand` variants `Help`, `View`, `SelectByMenuIndex(usize)`, and `Debug(DebugCommand)`. Introduce `DebugCommand` with `EventLog`, `ControllerBindings`, `ItemLocation(ItemId)`, `Rejection`, `ProjectionRebuild`, `Replay`.

### 2. Extend `parse_command`

Add equality matches for `help` and `view`; a `debug ` prefix that sub-parses the remainder (`log`, `bindings`, `item <item_id>`, `rejection`, `projection`, `replay`); and a bare-numeric branch (all-ASCII-digit token) producing `SelectByMenuIndex(n)`, treating `0` as out-of-range to match `semantic_id_for_selection`. A malformed `debug item` id or unknown `debug` subcommand returns a typed `InputError`, preserving the existing actor-safe error shape.

### 3. Add `InputError` variants as needed

e.g. `BadDebugCommand(String)` for an unrecognized debug subcommand, keeping errors actor-safe (no hidden-truth leakage in the message).

## Files to Touch

- `crates/tracewake-tui/src/input.rs` (modify)

## Out of Scope

- The command loop / dispatch itself (0003PHA1AEXETUI-002).
- Numeric index → stable-id resolution (uses the existing helper in 002, not here).
- Any world mutation, the `TuiApp` facade, debug rendering.
- `ratatui`/`crossterm` backends, new domain mechanics, Phase 2 (spec §Non-goals).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --lib input` — new unit tests for `help`, `view`, numeric, and `debug …` parsing pass.
2. A unit test asserts `parse_command("1")` produces `SelectByMenuIndex(1)`, not a `SemanticActionId` carrier (menu index is never identity).
3. `cargo fmt --all --check && cargo clippy -p tracewake-tui --all-targets -- -D warnings`.

### Invariants

1. The parser never resolves action identity — it carries a menu position; resolution structurally requires a view (`SelectByMenuIndex(usize)`).
2. The `UiCommand` extension is additive; the four existing variants and their spellings are unchanged.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/input.rs` (in-file `#[cfg(test)]` module) — cases for `help`, `view`, `1`, `0`, `debug log`, `debug item coin_stack_01`, and an unknown `debug bogus`.

### Commands

1. `cargo test -p tracewake-tui --lib`
2. `cargo fmt --all --check && cargo clippy -p tracewake-tui --all-targets -- -D warnings`

## Outcome

Completion date: 2026-06-06

What changed:

- Extended `tracewake-tui` input parsing with `Help`, `View`, `SelectByMenuIndex`, and structured `DebugCommand` variants for `debug log`, `debug bindings`, `debug item <item_id>`, `debug rejection`, `debug projection`, and `debug replay`.
- Kept bare numeric input as a 1-based menu position and left stable semantic action identity resolution to `semantic_id_for_selection`.
- Added in-file parser tests covering the new command vocabulary, invalid numeric selection, invalid debug subcommands, and malformed item IDs.

Deviations from original plan:

- Added explicit `BadMenuIndex` and `BadItemId` error variants in addition to `BadDebugCommand` so malformed numeric overflow and malformed debug item IDs remain typed and actor-safe.

Verification results:

- `cargo test -p tracewake-tui --lib input` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy -p tracewake-tui --all-targets -- -D warnings` — passed.
