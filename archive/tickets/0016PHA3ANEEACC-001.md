# 0016PHA3ANEEACC-001: Duration-terminal predicate and reservation lifecycle closure

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` duration-terminal `EventKind` predicate; reservation-check + completion-appender rewiring; new golden fixture
**Deps**: None

## Problem

ORD-HARD-015: `actions/pipeline.rs::body_exclusive_reservation_conflict` builds its `closed_starts` set from causes of `SleepCompleted | SleepInterrupted | WorkBlockCompleted` only. `work_completion_failure` emits `WorkBlockFailed` (caused by the original `WorkBlockStarted`), which is absent from the closed set — so a failed work block's start stays "open" forever and every later body-exclusive proposal (sleep, work) for that actor is rejected with `ReservationConflict` for the rest of the run. A typed `scheduling/reservation` blocker must reflect a *live* conflict (INV-043/045; architecture doc 04 reservation semantics).

This ticket introduces the single open/closed classification authority the rest of the 0016 batch consumes: a wildcard-free `is_duration_terminal(kind)` predicate shared by the reservation check and the completion appender, so the two can never drift. `ORD-HARD-014`'s tick-regime classifier (ticket 002) and `ORD-HARD-023`'s open-duration window-skip (ticket 010) both consume this predicate (spec §5 item 1).

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `body_exclusive_reservation_conflict` (`crates/tracewake-core/src/actions/pipeline.rs`, closed-set match at ~:416–421) omits `WorkBlockFailed`; `work_failed_from_start_event` (`crates/tracewake-core/src/actions/defs/work.rs` ~:396) emits `EventKind::WorkBlockFailed` caused by the original `WorkBlockStarted`. The only existing reservation test, `overlapping_body_exclusive_action_is_reservation_conflict` (work.rs), exercises the positive conflict on a still-running block only — no test fails a block mid-duration then proposes a later body-exclusive action.
2. Spec/docs: spec 0016 §ORD-HARD-015 (evidence, required correction, structural lock) and §5 item 1 (the predicate is the single open/closed classification authority); `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` reservation semantics; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-043/INV-045.
3. Shared boundary under audit: the duration open/closed classification consumed by three surfaces — the reservation check (`actions/pipeline.rs`), the completion appender (`scheduler.rs::append_due_completions`), and (in sibling tickets) the 002 tick classifier and the 010 window-skip. The predicate lives in core next to `EventKind` (`events/envelope.rs`) so all consumers share one definition.
4. INV-043 — action validation is ordinary-agent validation: a reservation rejection must reflect actor state truthfully. INV-045 — ordinary survival is causal: an actor permanently locked out of sleep/work by a phantom reservation breaks causal survival. Restated before trusting the ticket narrative.
5. Fail-closed validation / deterministic-replay surface: the enforcement surface is the body-exclusive reservation stage of `run_pipeline`. The change *narrows* rejection to live conflicts only — it does not weaken validation (overlap with a genuinely open block still rejects) and does not touch replay semantics; the new fixture must replay byte-identically. The wildcard-free match is the compile-time half: a future duration `EventKind` fails compilation until classified terminal or non-terminal.
6. Adjacent contradictions: `append_due_completions` currently derives its own completion pairing independently of the reservation check — required consequence of this ticket is unifying both on the predicate, not a separate bug ticket.

## Architecture Check

1. A single predicate matched wildcard-free on `EventKind` is structurally drift-proof: the reservation check and completion appender cannot disagree about what closes a duration, and adding a duration kind without classifying it is a compile error rather than a silent forever-open reservation. The alternative — adding `WorkBlockFailed` to the local match arm — fixes today's symptom but leaves the two consumers free to drift again.
2. No backwards-compatibility aliasing/shims: the local `closed_starts` kind-list is replaced by the predicate, not wrapped.

## Verification Layers

1. INV-043 (live-conflict reservation) → replay/golden-fixture check: `work_block_failed_then_sleep_succeeds_001` — displaced work block fails, the actor later sleeps successfully; replay byte-match.
2. INV-045 (causal survival) → runtime test: after `WorkBlockFailed`, a subsequent body-exclusive proposal is not rejected with `ReservationConflict`.
3. Compile-time lock → codebase grep-proof: `is_duration_terminal` matches duration kinds exhaustively with no wildcard arm covering them (grep for `_ =>` inside the predicate's duration handling must come up empty).
4. INV-018 (byte-identical replay, substrate-only) → the new fixture's golden run replays byte-identically; deeper replay-gate enforcement is sibling ticket 003's surface.

## What to Change

### 1. `is_duration_terminal(kind: &EventKind) -> bool` predicate

In `crates/tracewake-core/src/events/envelope.rs` (co-located with `EventKind`), classify the duration-event vocabulary: terminal set `SleepCompleted`, `SleepInterrupted`, `WorkBlockCompleted`, `WorkBlockFailed`; open set `SleepStarted`, `WorkBlockStarted`. Match duration kinds without a wildcard arm so any future duration variant fails compilation until classified.

### 2. Reservation check consumes the predicate

Rewire `body_exclusive_reservation_conflict` (`actions/pipeline.rs`) to build `closed_starts` from causes of events satisfying `is_duration_terminal` — closing the `WorkBlockFailed` hole.

### 3. Completion appender consumes the predicate

`scheduler.rs::append_due_completions` (and its open-duration pairing logic) derives open/closed pairing from the same predicate, removing its independent kind-list.

### 4. Fixture `work_block_failed_then_sleep_succeeds_001`

Authored content fixture: a work block fails mid-duration (actor displaced), the actor later proposes and commits sleep successfully. Registered in `fixtures/mod.rs` and exercised by the golden runner with byte-identical replay.

## Files to Touch

- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-content/src/fixtures/work_block_failed_then_sleep_succeeds_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — run the new fixture)

## Out of Scope

- Routing need deltas through a tick-regime classifier (ticket 0016PHA3ANEEACC-002, which consumes this predicate).
- Skipping decision generation for actors with open body-exclusive durations (ticket 0016PHA3ANEEACC-010).
- The context-hash replay gate (ticket 0016PHA3ANEEACC-003).

## Acceptance Criteria

### Tests That Must Pass

1. `work_block_failed_then_sleep_succeeds_001`: the post-failure sleep proposal commits (no `ReservationConflict`), and the fixture's log replays byte-identically.
2. The existing positive case still holds: `overlapping_body_exclusive_action_is_reservation_conflict` remains green (a still-running block still rejects).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Exactly one duration open/closed classification authority exists in core (`is_duration_terminal`); the reservation check and completion appender both consume it — no independent kind-lists remain.
2. The predicate's duration-kind match has no wildcard arm: a new duration `EventKind` variant fails compilation until classified.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/work_block_failed_then_sleep_succeeds_001.rs` — the ORD-HARD-015 structural-lock fixture (failed block → later sleep succeeds; replay byte-match).
2. `crates/tracewake-core/src/actions/pipeline.rs` — unit test: a `WorkBlockStarted` whose terminal cause is `WorkBlockFailed` is closed for reservation purposes.

### Commands

1. `cargo test -p tracewake-core pipeline && cargo test -p tracewake-content golden`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-10

What changed:

- Added `is_duration_terminal(EventKind)` in `tracewake-core` as the single duration-terminal classification authority, with an exhaustive match and no wildcard arm.
- Rewired the body-exclusive reservation check to close starts from every duration-terminal event, including `WorkBlockFailed`.
- Rewired scheduled completion handling to classify appended duration terminal events through the shared predicate before attempting routine-step completion.
- Added `work_block_failed_then_sleep_succeeds_001`, registered it in the content fixture roster, and updated fixture census tests.
- Added regression coverage proving `WorkBlockFailed` closes a prior work reservation and the later sleep proposal commits without `ReservationConflict`.

Deviations from original plan:

- The core unit regression landed in `crates/tracewake-core/src/actions/defs/work.rs`, where the existing work/reservation tests and helpers live, rather than directly in `actions/pipeline.rs`.
- The scheduler still keeps separate pending sleep/work queues; this ticket removed the independent terminal-event classification where the scheduler consumes appended duration terminal events. The broader open-duration classifier/window-skip work remains scoped to tickets 002 and 010.

Verification results:

- `cargo test -p tracewake-core work_block_failed_closes_body_exclusive_reservation` — passed.
- `cargo test -p tracewake-core overlapping_body_exclusive_action_is_reservation_conflict` — passed.
- `cargo test -p tracewake-content work_block_failed_then_sleep_succeeds_fixture_closes_reservation` — passed.
- `cargo test -p tracewake-core pipeline` — passed.
- `cargo test -p tracewake-content golden` — passed.
- `cargo test -p tracewake-content --test fixtures_load` — passed after updating the fixture census for the new registered fixture.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
