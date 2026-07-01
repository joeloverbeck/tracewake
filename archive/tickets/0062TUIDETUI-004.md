# 0062TUIDETUI-004: Semantic-script parser (semantic lines → `UiIntent`)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — new `tracewake-tui` semantic-script parser (`intent/semantic_script.rs`); registers in `intent/mod.rs`. No core types, no event/schema/view-model changes.
**Deps**: 0062TUIDETUI-001.

## Problem

Spec 0062 §1.1 item 4 requires a semantic-script driver that parses semantic lines (`focus actors`, `select actor 1`, `submit semantic_action_id=…`, `wait_one_tick`, `continue_until max_ticks=…`) into the same `UiIntent` stream the key-script parser produces. The semantic path gives layout-robust automated scenarios that survive harmless keybinding/layout changes (§4.3), and parsing must fail closed on unknown lines (§8).

## Assumption Reassessment (2026-07-01)

1. **Semantic surface (spec + code).** Semantic lines map to `UiIntent` variants from 0062TUIDETUI-001: `focus <pane>`→`FocusPane(FocusedPane)`, `select …`/`move …`→`MoveSelection(..)`/`ActivateSelection`, `submit semantic_action_id=<id>`→`SubmitSemanticAction(SemanticActionId)`, `wait_one_tick`→`WaitOneTick`, `continue_until max_ticks=<n>`→`ContinueUntil{max_ticks}`, notebook/help/quit as needed. `SemanticActionId` is `tracewake_core::ids::SemanticActionId`; `FocusedPane` variants (pane names) are at `crates/tracewake-tui/src/screen/model.rs:67`.
2. **Target module (code).** TUI-only, lands under `crates/tracewake-tui/src/intent/`, registered in `intent/mod.rs` (created by 0062TUIDETUI-001). Produces `UiIntent`; no dispatch.
3. **Cross-artifact boundary under audit.** This is the second of three producers feeding the single reducer (§4.1). It must emit the identical `UiIntent` type as the key-script parser (003) so the two producer paths cannot fork semantics; the equivalence is proven by the conformance ticket (006).
4. **Motivating invariant (INV-018).** `INV-018` motivates deterministic parsing: identical semantic scripts yield identical `UiIntent` streams. `INV-069` reinforces typed-intent-only output.
5. **Fail-closed parse enforcement surface.** The parser is a fail-closed input-validation surface (spec §8): unknown line forms, unknown pane names, or malformed `key=value` fields return an explicit typed error, never a silent skip or default intent. It introduces no leakage or nondeterminism path — a pure deterministic mapping with no hidden-truth input and no world/replay surface touched; actor-knowledge and replay contracts are unchanged.

## Architecture Check

1. A separate semantic-script parser gives layout-robust automation (§4.3) decoupled from physical key bindings, while emitting the same `UiIntent` type as the key path so both converge on one reducer — cleaner than overloading the key parser. Fail-closed parsing matches repo validation discipline.
2. No backwards-compatibility aliasing or shims; migrated command strings become a declared scripted-intent grammar, not an undocumented alias (spec §1.2, report §5.4 Stage E).

## Verification Layers

1. INV-018 (determinism) -> replay/test check: the same semantic script parses to the same `UiIntent` stream across runs.
2. Fail-closed parse (spec §8) -> test: an unknown line / pane name / malformed field returns an explicit error.
3. Producer-parity -> codebase grep-proof: emits `crate::intent::UiIntent`, identical to the key parser's output type.

## What to Change

### 1. Add the semantic-script parser

Create `crates/tracewake-tui/src/intent/semantic_script.rs` with a parser (e.g. `parse_semantic_script(&str) -> Result<Vec<UiIntent>, SemanticScriptError>`) mapping each semantic line to a `UiIntent`. Parse `key=value` fields (`semantic_action_id=`, `max_ticks=`) strictly; resolve pane names to `FocusedPane`. Unknown lines/names/fields produce a typed error (fail closed).

### 2. Register the module

Add `pub mod semantic_script;` to `crates/tracewake-tui/src/intent/mod.rs` (created by 0062TUIDETUI-001).

## Files to Touch

- `crates/tracewake-tui/src/intent/semantic_script.rs` (new)
- `crates/tracewake-tui/src/intent/mod.rs` (modify — file created by 0062TUIDETUI-001)

## Out of Scope

- The key-script parser (0062TUIDETUI-003).
- The reducer and any dispatch (002).
- The headless runner and transcript (005).
- Extending the semantic grammar beyond what a driven scenario needs (spec §9 keeps it minimal and closed).

## Acceptance Criteria

### Tests That Must Pass

1. A test parses a representative semantic script (`focus actors`, `select actor 1`, `submit semantic_action_id=…`, `wait_one_tick`, `continue_until max_ticks=…`) and asserts the exact `UiIntent` sequence.
2. A test asserts an unknown line, unknown pane name, and malformed `key=value` each yield an explicit `SemanticScriptError`.
3. A determinism test: the same script parses to the same stream twice.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`.

### Invariants

1. Identical scripts yield identical `UiIntent` streams (INV-018).
2. Unknown input fails closed with a typed error (spec §8 validation discipline).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/intent/semantic_script.rs` (unit tests) — line→intent mapping, fail-closed errors, determinism; rationale: proves the semantic path is layout-robust and deterministic.

### Commands

1. `cargo test -p tracewake-tui semantic_script`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-01

Implemented the TUI-only `intent::semantic_script` parser. The parser maps a
minimal closed semantic grammar to the shared `UiIntent` stream: pane focus,
`select actor 1` / `select action 1` activation, `move up/down`,
`submit semantic_action_id=...`, `wait_one_tick`, `continue_until max_ticks=...`,
`notebook`, `help`, and `quit`. Unknown line forms, unknown pane names,
unsupported non-current selections, malformed fields, invalid semantic action
ids, and invalid max-tick values fail with typed `SemanticScriptError` variants.

The parser is pure and performs no dispatch, app/runtime mutation, hidden-truth
read, or core schema change. The new production source file was classified in
the workspace source census with the existing TUI presentation-boundary
rationale.

Verification:

- `cargo test -p tracewake-tui semantic_script`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p tracewake-tui`
