# 0060EMBSTUCKDUP-001: Idempotent embodied stuck-diagnostic emission on repeated blocked continue-routine

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (`runtime/session.rs` stuck-outcome path); new regression test(s) in `tracewake-core` and/or `tracewake-tui`. No new events, schemas, or fixtures.
**Deps**: None. Related: `0060EMBSTUCKDUP-002` (TUI keep-alive hardening) — independent; either order is valid.

## Problem

Embodying an actor and repeatedly selecting a **blocked** "Continue routine" action crashes the whole TUI:

```
thread 'main' panicked ... Runtime(WorldAdvance(EventAppend(DuplicateEventId(EventId(StableId(
"event.world_step_stuck_diagnostic.actor_tomas.1.stuck_embodied_continue_routine_time_advancing_actor_tomas_1"))))))
```

Repro (golden `ordinary_workday_001`, `actor_tomas`): `wait` → `continue routine` (moves home→workshop) → `continue routine` (workshop step resolves to a time-advancing `wait` follow-on; correctly refused with the typed stuck why-not "routine continuation cannot safely commit a time-advancing follow-on yet") → `continue routine` **again** → panic.

Root cause: when an embodied continue-routine follow-on is refused, `run_embodied_continue_routine_stuck_outcome` emits a stuck-diagnostic event whose id is deterministic on `(actor, tick, diagnostic_id)`, and the diagnostic id is itself keyed on `(actor, decision_tick)`. The stuck path **does not advance the tick**, so a second selection at the same tick regenerates a byte-identical event id, which `EventLog::append` rejects as a hard `DuplicateEventId`. The error is not a corruption — it is a re-attempt of a first-class, already-recorded stuck outcome. Per `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md:152-157` ("being stuck is a first-class outcome") and `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, re-attempting a blocked action must be safe — it must re-present the typed why-not, not panic.

## Assumption Reassessment (2026-07-01)

1. **Emission site (code).** `crates/tracewake-core/src/runtime/session.rs:754-804` `run_embodied_continue_routine_stuck_outcome` builds the event via `build_actor_stuck_diagnostic_event` and appends it via `append_and_apply_actor_artifact` with no pre-check for an already-present id. It is reached from both stuck branches at `session.rs:604-614` (transaction-returned stuck) and `session.rs:619-632` (recursive `continue_routine`) and `session.rs:633-645` (time-advancing follow-on) — all three share this routine, so the fix must live in the shared routine, not one branch.
2. **Id determinism (code + docs).** Event id built at `crates/tracewake-core/src/scheduler.rs:2330-2337` as `event.world_step_stuck_diagnostic.{actor}.{tick}.{diagnostic_id}`; time-advancing diagnostic id built at `crates/tracewake-core/src/runtime/session.rs:1278-1282` as `stuck_embodied_continue_routine_time_advancing_{actor}_{decision_tick}`. Both keyed on the same unchanging `decision_tick`. Determinism is correct and required by INV-018 — the defect is the missing idempotency guard, not the id scheme. Do **not** add an attempt counter (see Architecture Check).
3. **Append contract (code).** `crates/tracewake-core/src/events/log.rs:25-43` `append` returns `Err(EventLogError::DuplicateEventId)` on any repeated id (hard error, by design — supports deterministic replay). `pub fn events()` (`log.rs:79`) is the public read surface already used at `session.rs:783-790`; `contains_event_id` (`log.rs:83`) is `pub(crate)` and callable from `session.rs`.
4. **Invariant restatement (INV-018 / INV-009).** INV-018 (deterministic replay) requires ordered events with stable ids — preserved. INV-009 (meaningful events only) forbids minting a redundant event for a re-press that carries no new fact — this rules out the attempt-counter alternative and favors idempotent skip. Neither the fail-closed nor actor-knowledge-filtering surface changes: the same typed stuck diagnostic and why-not are produced; the change only avoids re-appending an identical event.
5. **State-apply on skip.** `append_and_apply_actor_artifact` both appends the event and applies its artifact to `agent_state`. On the first emission the artifact is already applied; on a duplicate re-attempt the state already reflects it, so skipping both append and re-apply is correct — re-applying would double-count. The skip path must re-derive the returned `PipelineResult` (report + `appended_events`) from the **existing** logged event, not from a fresh append.
6. **Adjacent contradiction classification.** The TUI's escalation of this recoverable error into a process panic (`crates/tracewake-tui/src/run.rs:92`, `crates/tracewake-tui/src/main.rs:49`) is a **separate** defect — routed to `0060EMBSTUCKDUP-002`, not this ticket. This ticket removes the `Err` at the source so the offered action resolves to a typed why-not exactly as it did on its first selection.

## Architecture Check

1. **Idempotent skip vs. attempt-counter.** An idempotent guard (skip re-append when the id already exists; re-derive the report from the existing event) preserves the deterministic id (INV-018), emits no redundant events (INV-009), and models "re-attempting a blocked action is safe" (`docs/1-architecture/05`, `docs/2-execution/06`). The alternative — embedding an attempt/sequence counter in the id — would make each press a distinct event, polluting the log with duplicate-in-substance diagnostics, growing it unboundedly on repeated presses, and tensioning INV-009. Rejected.
2. No backwards-compatibility aliasing or shims introduced; the change is a guard inside one existing function.

## Verification Layers

1. INV-018 (deterministic replay, no duplicate ids) -> replay/golden-fixture check: a golden/transcript run selecting the blocked continue-routine twice appends the stuck diagnostic exactly once and replays deterministically.
2. INV-009 (meaningful events only) -> codebase grep-proof + test assertion: after N repeated blocked selections at the same tick, the event log contains exactly one `event.world_step_stuck_diagnostic....` id for that `(actor, tick, diagnostic_id)`.
3. INV-070 (why-not mandatory) -> manual review + test: every repeated selection returns the typed stuck report / actor-known why-not ("routine continuation cannot safely commit a time-advancing follow-on yet"), never an `Err`.

## What to Change

### 1. Guard `run_embodied_continue_routine_stuck_outcome` against duplicate emission

In `crates/tracewake-core/src/runtime/session.rs` (`run_embodied_continue_routine_stuck_outcome`, ~754-804): after building `event` via `build_actor_stuck_diagnostic_event`, check whether `self.event_log` already contains `event.event_id` (via `contains_event_id` or an `events().iter().any(...)` scan matching the existing style at 783-790). 

- If **absent**: append+apply as today.
- If **present**: skip `append_and_apply_actor_artifact` entirely; locate the existing logged envelope with that id and use it to build the returned `PipelineResult` (same `embodied_continue_routine_stuck_report` call, `appended_events` derived from the existing event plus `marker_result` / `prior_scheduler_stuck_events` as today). Still call `record_actor_current_place_perception(actor_id)` if that is idempotent; otherwise gate it to the first-emission branch (verify at implementation whether re-recording perception is safe to repeat).

The function must return `Ok(Some(PipelineResult { ... }))` in both branches so the caller re-presents the why-not; it must never surface the `DuplicateEventId`.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — `run_embodied_continue_routine_stuck_outcome`; add regression test in the `#[cfg(test)] mod tests` at ~1311, or a new integration test — see Test Plan)
- `crates/tracewake-tui/tests/command_loop_session.rs` (modify — end-to-end repro test; new)

## Out of Scope

- The TUI's panic-on-runtime-error escalation (`run.rs:92`, `main.rs:49`) — `0060EMBSTUCKDUP-002`.
- The autonomous no-human scheduler stuck path (it advances ticks between emissions, so it does not collide at a fixed tick); no change unless a test proves otherwise.
- Any change to the id scheme, the stuck-diagnostic schema, or `EventLog::append`'s duplicate-rejection contract.

## Acceptance Criteria

### Tests That Must Pass

1. New core test: drive a runtime session (or the `run_embodied_continue_routine_follow_on` path) so a blocked/time-advancing continue-routine follow-on is selected **twice** at the same tick; assert the second call returns `Ok(Some(_))` with the typed stuck report and that the event log holds exactly one matching `event.world_step_stuck_diagnostic....` id.
2. New TUI test in `command_loop_session.rs`: replay the exact repro against golden `ordinary_workday_001` / `actor_tomas` (`wait`, `continue`, `continue`, `continue`) through `run_command_loop`; assert it returns `Ok(())`, the blocked why-not renders on each blocked selection, and no panic occurs.
3. `cargo test --workspace` (full suite green).

### Invariants

1. Selecting the same blocked continue-routine action repeatedly at an unchanged tick appends at most one stuck-diagnostic event (INV-009) and never returns an `Err` to the caller.
2. Deterministic replay over the affected golden reproduces the single stuck diagnostic identically (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (tests module) — targeted proof that repeated blocked emission is idempotent and returns the typed report.
2. `crates/tracewake-tui/tests/command_loop_session.rs` — end-to-end proof the user's exact session no longer panics and re-presents the why-not.

### Commands

1. `cargo test -p tracewake-core runtime::session` and `cargo test -p tracewake-tui --test command_loop_session`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-07-01

Implemented the shared idempotency guard in
`LoadedWorldRuntime::run_embodied_continue_routine_stuck_outcome`: the stuck
diagnostic event is still built with the deterministic `(actor, tick,
diagnostic_id)` id, but an already-recorded event id is reused for the receipt
instead of being appended again. The path still returns a rejected
`PipelineResult` with the typed actor-visible why-not, so repeated same-tick
blocked continue-routine submissions re-present the explanation and preserve a
single `StuckDiagnosticRecorded` event.

Added core and TUI regression coverage:

- `runtime::session::tests::embodied_stuck_outcome_is_idempotent_for_same_tick_diagnostic`
  calls the shared stuck-outcome helper twice at the same tick and asserts one
  logged stuck diagnostic.
- `embodied_continue_stuck_emits_one_current_typed_diagnostic` now repeats the
  blocked embodied continue-routine selection and asserts the receipt reports
  the existing diagnostic while the event log still contains one stuck
  diagnostic.
- `repeated_blocked_continue_routine_renders_why_not_without_panic` drives the
  real TUI command loop with `ordinary_workday_001` / `actor_tomas`
  (`wait`, `1`, `1`, `1`) and confirms the repeated blocked selections render
  why-not output and keep the prompt alive.

Verification:

- `cargo test -p tracewake-core runtime::session::tests::embodied_stuck_outcome_is_idempotent_for_same_tick_diagnostic`
  passed.
- `cargo test -p tracewake-tui --test embodied_flow embodied_continue_stuck_emits_one_current_typed_diagnostic`
  passed.
- `cargo test -p tracewake-tui --test command_loop_session` passed.
- `cargo test -p tracewake-tui --test command_loop_session repeated_blocked_continue_routine_renders_why_not_without_panic`
  passed after formatting.
- `cargo fmt --all --check` passed after formatting.
- `cargo test --workspace` passed after implementation and before the final
  rustfmt-only line wrap in `command_loop_session.rs`.

Deviation: the TUI command-loop reproduction uses numeric menu selection `1`
for the offered continue-routine action because the command parser's `do`
syntax expects a current semantic action id, not the typed action id
`continue_routine`.
