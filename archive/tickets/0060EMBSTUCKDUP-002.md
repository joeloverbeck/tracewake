# 0060EMBSTUCKDUP-002: TUI command loop keeps the session alive on a runtime rejection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `tracewake-tui` (`run.rs` action-submission error handling); new regression test in `tracewake-tui`. No core, event, schema, or fixture changes.
**Deps**: None. Related: `0060EMBSTUCKDUP-001` removes the specific `DuplicateEventId` that first exposed this. This ticket is defense-in-depth: no offered action should ever crash the session, for **any** recoverable runtime error.

## Problem

In the embodied TUI, selecting an offered action that returns a runtime error other than `SemanticActionNotFound` aborts the entire process. `submit_and_render` (`crates/tracewake-tui/src/run.rs:82-92`) handles `AppError::SemanticActionNotFound` gracefully but its catch-all converts every other error into a fatal `std::io::Error::other(...)`, which `run_command_loop` propagates via `?` up to `main.rs:49` `.expect("command loop runs")` — a full-process panic. This is how the `DuplicateEventId` from `0060EMBSTUCKDUP-001` surfaced as a crash instead of a message.

This contradicts the command loop's own stated posture: the sibling handler at `run.rs:56-63` deliberately keeps the loop alive on a bad menu index ("report it as an input error and keep the loop alive rather than aborting the TUI"). Availability of an action must imply it is safe to *attempt* — a rejection produces a visible why-not/diagnostic, never a crash (aligns with `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` and INV-070).

## Assumption Reassessment (2026-07-01)

1. **Fault site (code).** `crates/tracewake-tui/src/run.rs:87-92` `submit_and_render`: `Err(error) => return Err(std::io::Error::other(format!("{error:?}")))` is the escalation. Reached via `SelectSemanticAction`, `SelectByMenuIndex`, and `WaitOneTick` command arms (`run.rs:53-71`) — the primary "choose an offered action" paths.
2. **Panic surface (code).** `crates/tracewake-tui/src/main.rs:43-49`: `run_command_loop(...).expect("command loop runs")` turns any `Err` returned by the loop into a panic. The loop already returns `std::io::Result<()>`; genuine I/O errors on the terminal remain a legitimate reason to end the loop, so the fix is to stop *manufacturing* fatal errors from recoverable rejections, not to swallow real I/O failures.
3. **Legible rendering exists (code).** `describe_app_error` (`run.rs:196-206`) has a catch-all `other => format!("{other:?}")`, so routing the runtime error through it yields a readable `Error: ...` line for any `AppError` variant, including the nested `WorldAdvance`/runtime errors. `writeln!` I/O errors still propagate as real `io::Result` failures.
4. **Invariant restatement (INV-070).** "Why-not explanations are mandatory": a blocked/failed offered action must render an actor-known explanation and keep the actor playable. Escalating a rejection to a panic destroys the explanation and the session. No hidden-truth leakage risk: `describe_app_error` renders the already-actor-facing error string; it introduces no new debug/embodied leak (the debug/embodied why-not split is owned by the view models, unchanged here).
5. **Adjacent site — `app_result` (code).** `run.rs:192-193` `app_result` similarly maps `AppError` → fatal `io::Error`, used by view-rendering calls (`render_current_view`, `current_view`, `notebook_view`, `advance_until`). A view-render failure aborting the loop is a distinct, more defensible case (nothing to display). Classify as **future cleanup / separate consideration**, not this ticket — this ticket scopes the *action-submission* catch-all that directly realizes "choosing an available action crashes." Do not change `app_result` here.

## Architecture Check

1. Keeping the loop alive with a visible diagnostic matches the loop's existing keep-alive posture (`run.rs:56-63`) and the TUI-playability doctrine, and localizes the change to one match arm reusing the existing `describe_app_error` helper — cleaner than introducing a new error taxonomy. It does not mask genuine terminal I/O failures, which still end the loop through the untouched `?` on `writeln!`.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. INV-070 (why-not mandatory) -> manual review + test: a runtime-rejected offered action renders `Error: ...` (or the typed why-not) and the prompt returns, rather than aborting.
2. TUI-never-aborts-on-recoverable-rejection -> replay/transcript check: the command-loop session test drives a runtime rejection and asserts the loop returns `Ok(())`.

## What to Change

### 1. Render runtime rejections instead of escalating them

In `crates/tracewake-tui/src/run.rs` `submit_and_render` (~87-92): replace the catch-all `Err(error) => return Err(std::io::Error::other(...))` with a render-and-continue arm, e.g. `Err(error) => writeln!(writer, "Error: {}", describe_app_error(&error))`, matching the `BindActor` arm at `run.rs:51`. The `SemanticActionNotFound` arm stays as-is. The function continues to return `std::io::Result<()>`, so real `writeln!` I/O failures still propagate.

## Files to Touch

- `crates/tracewake-tui/src/run.rs` (modify — `submit_and_render` catch-all)
- `crates/tracewake-tui/tests/command_loop_session.rs` (modify — regression test; new)

## Out of Scope

- The core duplicate-emission fix — `0060EMBSTUCKDUP-001`.
- `app_result` view-rendering escalation (`run.rs:192-193`) — separate consideration (Assumption Reassessment item 5); not changed here.
- Removing or changing `main.rs:49` `.expect(...)` — the loop should still surface genuine terminal I/O failures; only manufactured-fatal rejections are removed at the source.

## Acceptance Criteria

### Tests That Must Pass

1. New TUI test in `command_loop_session.rs`: submit an offered action that returns a runtime error (e.g. via a scripted session, or by leveraging the `0060EMBSTUCKDUP-001` repro if that ticket is not yet merged); assert `run_command_loop` returns `Ok(())`, an `Error: ...`/why-not line is written, and the prompt is re-emitted.
2. `cargo test --workspace` (full suite green).

### Invariants

1. No offered/selectable action causes `run_command_loop` to return `Err` or panic on a recoverable runtime rejection (INV-070; TUI playability).
2. Genuine terminal I/O failures still propagate (unchanged `writeln!` `?`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — proves an action-submission runtime rejection keeps the loop alive and renders a diagnostic.

### Commands

1. `cargo test -p tracewake-tui --test command_loop_session`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-07-01

Changed `submit_and_render` so action-submission errors other than
`SemanticActionNotFound` are rendered through the existing actor-facing
`describe_app_error` path instead of being converted into a fatal
`std::io::Error`. The special missing-current-action message remains unchanged,
and genuine writer I/O failures still propagate through `writeln!`.

Added `run::tests::runtime_submit_error_is_rendered_without_aborting_loop` to
prove the submit-error catch-all renders `Error: Runtime(...)` and returns
`Ok(())` for a constructed `AppError::Runtime`. The real command-loop repro from
`0060EMBSTUCKDUP-001`,
`repeated_blocked_continue_routine_renders_why_not_without_panic`, remains the
end-to-end proof that the user's offered-action path keeps the prompt alive and
re-presents the typed why-not.

Verification:

- `cargo test -p tracewake-tui run::tests::runtime_submit_error_is_rendered_without_aborting_loop`
  passed.
- `cargo test -p tracewake-tui --test command_loop_session` passed.
- `cargo fmt --all --check` passed.
- `cargo test --workspace` passed.

Deviation: after `0060EMBSTUCKDUP-001`, the original duplicate-event runtime
error no longer occurs through the real command loop, so the `AppError::Runtime`
catch-all is covered by a direct unit test rather than a newly fabricated
runtime-error fixture in `command_loop_session.rs`.
