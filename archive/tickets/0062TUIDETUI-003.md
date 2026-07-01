# 0062TUIDETUI-003: Key-script parser (key tokens → `UiIntent`)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — new `tracewake-tui` key-script parser (`intent/key_script.rs`); registers in `intent/mod.rs`. No core types, no event/schema/view-model changes.
**Deps**: 0062TUIDETUI-001.

## Problem

Spec 0062 §1.1 item 3 requires a key-script driver that parses a script of key tokens (`Tab`, `Down`, `Enter`, `w`, `c`, `?`, `n`, `q`, …) into `UiIntent` values, so the key path exercises real human bindings without a TTY (§4.3). Parsing must be deterministic and fail closed on unknown tokens (§8), never silently ignoring input.

## Assumption Reassessment (2026-07-01)

1. **Existing key bindings (code).** The current line REPL parses commands/aliases in `crates/tracewake-tui/src/input.rs` (`parse_command`; aliases `w`→wait, `c`→continue, etc., `input.rs:68–78`). The key-script parser maps single key tokens to the `UiIntent` variants from 0062TUIDETUI-001 (e.g. `Tab`→`FocusNext`, `Down`→`MoveSelection(Down)`, `Enter`→`ActivateSelection`, `w`→`WaitOneTick`, `c`→`ContinueUntil{..}`, `?`→`ToggleHelp`, `n`→`FocusNotebook`, `q`→`Quit`). Reuse the existing alias meanings where they map, so the key path stays faithful to real bindings.
2. **Target module (code).** The parser is TUI-only and lands under `crates/tracewake-tui/src/intent/`, registered in `intent/mod.rs` (created by 0062TUIDETUI-001). It produces `UiIntent`; it performs no dispatch.
3. **Cross-artifact boundary under audit.** The parser is one of the three producers feeding the single reducer (§4.1). It shares the `UiIntent` contract with the semantic-script parser (004) and the reducer (002); confirm both parsers emit the identical `UiIntent` type so producers cannot fork semantics.
4. **Motivating invariant (INV-018).** `INV-018` ("Deterministic replay is foundational") motivates the deterministic-parse requirement: identical scripts must yield identical `UiIntent` streams across runs. `INV-069` reinforces that the parser produces only typed intents, not rules.
5. **Fail-closed parse enforcement surface.** The parser is a fail-closed input-validation surface (spec §8): unknown tokens must return an explicit typed error, never a silent skip or a default intent. This introduces no leakage or nondeterminism path — parsing is a pure deterministic mapping from token to `UiIntent` with no hidden-truth input and no world/replay surface touched. No actor-knowledge or replay contract is modified.

## Architecture Check

1. A dedicated key-script parser that reuses the real binding meanings keeps the key path a faithful proof of human controls (§4.3), separate from the layout-robust semantic path — cleaner than one parser conflating both grammars. Fail-closed parsing matches the repo's validation discipline.
2. No backwards-compatibility aliasing or shims; the legacy line grammar is not preserved as an undocumented alias (spec §1.2).

## Verification Layers

1. INV-018 (determinism) -> replay/test check: the same key script parses to the same `UiIntent` stream across runs.
2. Fail-closed parse (spec §8) -> test: an unknown key token returns an explicit error, not a silent skip or default.
3. Producer-parity -> codebase grep-proof: the parser emits `crate::intent::UiIntent`, identical to the semantic parser's output type.

## What to Change

### 1. Add the key-script parser

Create `crates/tracewake-tui/src/intent/key_script.rs` with a parser (e.g. `parse_key_script(&str) -> Result<Vec<UiIntent>, KeyScriptError>`) mapping key tokens to `UiIntent` values, reusing existing binding meanings where they map. Unknown tokens produce a typed error (fail closed).

### 2. Register the module

Add `pub mod key_script;` to `crates/tracewake-tui/src/intent/mod.rs` (created by 0062TUIDETUI-001).

## Files to Touch

- `crates/tracewake-tui/src/intent/key_script.rs` (new)
- `crates/tracewake-tui/src/intent/mod.rs` (modify — file created by 0062TUIDETUI-001)

## Out of Scope

- The semantic-script parser (0062TUIDETUI-004).
- The reducer and any dispatch (002).
- The headless runner and transcript (005).
- Reading real key *events* from a terminal (Spec 0065) — this parses scripted key *tokens* only (spec §1.2).

## Acceptance Criteria

### Tests That Must Pass

1. A test parses a representative key script (`Tab Down Enter w c ? n q`) and asserts the exact `UiIntent` sequence.
2. A test asserts an unknown token yields an explicit `KeyScriptError`, not a silent skip.
3. A determinism test: the same script parses to the same stream twice.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`.

### Invariants

1. Identical scripts yield identical `UiIntent` streams (INV-018).
2. Unknown tokens fail closed with a typed error (spec §8 validation discipline).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/intent/key_script.rs` (unit tests) — token→intent mapping, fail-closed unknown-token error, determinism; rationale: proves the key path is faithful and deterministic.

### Commands

1. `cargo test -p tracewake-tui key_script`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-01

Implemented the TUI-only `intent::key_script` parser. The parser maps scripted
key tokens (`Tab`, `Up`, `Down`, `Enter`, `w`, `c`, `?`, `n`, `q`) to the
shared `UiIntent` stream, preserving the existing `w`/`c` line-command meanings
for wait and default continue. Unknown tokens return a typed `KeyScriptError`
and are never silently skipped or converted to a default intent.

The parser is pure and performs no dispatch, app/runtime mutation, hidden-truth
read, or core schema change. The new production source file was classified in
the workspace source census with the existing TUI presentation-boundary
rationale.

Verification:

- `cargo test -p tracewake-tui key_script`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p tracewake-tui`
