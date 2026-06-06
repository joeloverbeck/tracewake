# 0003PHA1AEXETUI-003: Binary-level integration and no-mutation/no-leak regressions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `crates/tracewake-tui/tests/command_loop_session.rs`; modifies `crates/tracewake-tui/tests/embodied_flow.rs`.
**Deps**: 0003PHA1AEXETUI-002

## Problem

Spec §"Required tests" #1–#4 require: a test that launches the *actual* `tracewake-tui` binary with scripted stdin; a binary-level proof that bare `<n>` resolves to the stable semantic action id (not the menu number); a debug non-leakage regression; and an extension of the no-direct-`apply_event` regression to the new loop module and `main.rs`. Today `embodied_flow.rs:38-47` guards only `app.rs`, `render.rs`, and `input.rs`.

## Assumption Reassessment (2026-06-06)

1. The binary is `tracewake-tui` (`crates/tracewake-tui/Cargo.toml:2`), so integration tests can launch it via `std::process::Command::new(env!("CARGO_BIN_EXE_tracewake-tui"))` — no dev-dependency is required and `Cargo.toml` is unchanged. The loop entry point is `run.rs::run_command_loop`, created by 0003PHA1AEXETUI-002 (Deps).
2. Patterns to mirror (read this session): `tui_acceptance.rs:34-48` (`debug_truth_does_not_enter_embodied_view` — asserts `container:strongbox_tomas` appears only inside the debug panel and `physical_checksum` is unchanged) and the source-include guard `embodied_flow.rs:38-47` (`tui_does_not_call_event_applier`).
3. Shared boundary under audit: this ticket consumes the binary and the `run.rs` module that 002 introduces; it adds no production logic.
4. INV-018 (deterministic replay is foundational) and INV-008 (UI assistance is not authority) motivate the test design — restated: the scripted session must be deterministic (fixed fixture `strongbox_001`, default `actor_tomas`), and the numeric test must prove the *stable semantic id* executed, not merely that input `1` "caused some mutation" (spec §"Required tests" #2). INV-018 and INV-008 headings confirmed present in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
5. No-leak / no-mutation enforcement surface: this ticket extends `embodied_flow.rs`'s `tui_does_not_call_event_applier` to `include_str!("../src/main.rs")` and `include_str!("../src/run.rs")`, asserting neither contains `apply_event` (INV-069); and asserts `debug item` truth never enters the post-debug embodied render while `physical_checksum` is unchanged across debug commands (INV-024 / INV-093). It introduces no determinism or leakage path — it only proves their absence.

## Architecture Check

1. Launching via `CARGO_BIN_EXE_tracewake-tui` + `std::process::Command` proves the *actual* operating surface (spec §"Required tests" #1) with zero new dependencies — lighter than pulling in `assert_cmd`. The in-memory runner test (002) and this process-level test are complementary layers, not duplicates.
2. No shims: this extends the existing `tui_does_not_call_event_applier` guard rather than replacing it.

## Verification Layers

1. INV-008 (index ≠ identity) -> binary numeric test: re-enumerate the menu from the rendered view, submit the bare index, and assert the *stable id from that menu line* (e.g. `open.container.strongbox_tomas` in the `strongbox_001` view) executed — not the literal `"1"`.
2. INV-024 / INV-068 (no leak / debug non-diegetic) -> binary regression: `debug item coin_stack_01` reveals `container:strongbox_tomas` only inside a `DEBUG NON-DIEGETIC` panel; a following `view` omits it; `physical_checksum` is unchanged across the debug commands.
3. INV-069 (no direct mutation) -> codebase grep-proof: `main.rs` and `run.rs` source contain no `apply_event`.

## What to Change

### 1. New `tests/command_loop_session.rs`

(a) Scripted-session test: spawn the binary, pipe a script that exercises readiness line, initial embodied view, action list, an accepted action, an updated view, a rejected (why-not) flow, a `debug log` panel marked non-diegetic, and `quit`; assert the spec §"Required tests" #1 substrings and a clean zero exit. (b) Numeric-selection test: parse the rendered menu, submit the bare 1-based index, and assert the stable semantic id printed on that menu line executed (spec #2). Re-enumerate expected ids/counts from the rendered view at test start rather than hardcoding.

### 2. Extend `tests/embodied_flow.rs`

Add `include_str!("../src/main.rs")` and `include_str!("../src/run.rs")` to `tui_does_not_call_event_applier`, asserting neither contains `apply_event` (spec #4).

### 3. Debug non-leakage coverage

Either extend `command_loop_session.rs` or add a focused test mirroring `tui_acceptance.rs:34-48` at the loop level: debug truth is revealed only in the panel, the next embodied view omits it, and the checksum is unchanged (spec #3).

## Files to Touch

- `crates/tracewake-tui/tests/command_loop_session.rs` (new)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — adds `run.rs`/`main.rs` to the no-`apply_event` guard; `run.rs` is created in-batch by 0003PHA1AEXETUI-002)

## Out of Scope

- Production loop logic (0003PHA1AEXETUI-002) and parsing (0003PHA1AEXETUI-001).
- The README sample-session test (0003PHA1AEXETUI-004 — spec §"Required tests" #5).
- New domain mechanics, Phase 2.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --test command_loop_session` — scripted-session and numeric tests pass against the real binary.
2. `cargo test -p tracewake-tui --test embodied_flow` — the extended guard passes; `main.rs` and `run.rs` contain no `apply_event`.
3. `cargo test --workspace`.

### Invariants

1. The scripted session is deterministic — the same fixture and actor yield the same stdout.
2. No debug command changes `physical_checksum` or leaks hidden truth into the embodied view.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — actual-binary scripted session + numeric-identity proof (spec #1, #2) and debug non-leakage (spec #3).
2. `crates/tracewake-tui/tests/embodied_flow.rs` — extend the no-`apply_event` guard to `main.rs` and `run.rs` (spec #4).

### Commands

1. `cargo test -p tracewake-tui --test command_loop_session --test embodied_flow`
2. `cargo test --workspace`

## Outcome

Completion date: 2026-06-06

What changed:

- Added `crates/tracewake-tui/tests/command_loop_session.rs`, which launches the actual `tracewake-tui` binary with scripted stdin/stdout.
- Covered the executable loop startup, prompt, accepted action output, why-not rejection output, `debug log`, and clean exit.
- Added a binary-level numeric selection proof that bare `1` executes the stable semantic action ID from the rendered menu rather than treating `"1"` as identity.
- Added a binary-level debug non-leakage regression for `debug item coin_stack_01`, a following `view`, and unchanged debug/projection checksum output.
- Extended `embodied_flow.rs`'s no-direct-`apply_event` guard to include `main.rs` and `run.rs`.

Deviations from original plan:

- The debug non-mutation proof compares checksum output from `debug item` and `debug projection` in the same binary session, because embodied views intentionally do not print physical checksums.

Verification results:

- `cargo test -p tracewake-tui --test command_loop_session --test embodied_flow` — passed.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace` — passed.
