# 0047TUIAUTWOR-009: General body-exclusive reservation enforcement

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-core` (`actions/pipeline.rs` reservation predicate generalization + continuation exemption); core tests
**Deps**: 0047TUIAUTWOR-006

## Problem

Spec 0047 §4.4 requires that while an actor has an unresolved body-exclusive start, every ordinary embodied action for that actor (including ordinary `wait`) is rejected with the existing reservation-conflict report; only typed lifecycle controls (continue, pause, modeled interruption/cancellation) are permitted. Today the predicate `body_exclusive_reservation_conflict` (`actions/pipeline.rs:398`) only derives an actor from a candidate that *itself* starts a new body-exclusive duration, so sleep-then-`wait` is wrongly accepted (a `wait` sets `body_exclusive: "false"` and never trips the check). This makes continuation semantics incoherent — an overlapping `wait` remains accepted while a duration is open. This ticket generalizes the predicate and adds the continuation exemption, applying the rule identically to human and autonomous proposal origins.

## Assumption Reassessment (2026-06-22)

1. `body_exclusive_reservation_conflict` exists at `crates/tracewake-core/src/actions/pipeline.rs:398`; it currently keys on the *candidate* being `SleepStarted`/`WorkBlockStarted` with `body_exclusive: "true"`, so it never fires for an ordinary `wait` (which is `body_exclusive: "false"`, `wait.rs:158`). The reassessment confirmed this is the bug: "sleep-then-wait is wrongly accepted." The conflict is reported via `ReasonCode::ReservationConflict` (`actions/report.rs:46`, mapped at `pipeline.rs:196-197`).
2. The shared authority for "does this actor have an unresolved body-exclusive start at tick t" is `open_body_exclusive_starts` (`need_accounting.rs:124`), wired into the coordinator by 0047TUIAUTWOR-006. The generalized predicate keys on the *proposing actor's* open-start state at the proposal tick, derived from that same authority — not on the candidate's own kind.
3. Cross-artifact boundary under audit: the reservation predicate (`pipeline.rs`) and the open-duration authority (`need_accounting.rs`) must agree on "is this actor mid-duration". A continuation control is not an `ActorWaited` event and creates no competing reservation; when interruption is modeled, the existing duration closes through a terminal (0047TUIAUTWOR-007) before any new ordinary action is accepted. The rule applies identically to human and autonomous origins (no player-privilege branch).
4. Constitutional invariant motivating the ticket: `INV-043` (action validation is ordinary-agent validation — it does not branch for player privilege) and `INV-007` (every world-affecting player action must be NPC-possible — a human mid-sleep is rejected exactly as an autonomous actor would be).
5. Enforcement surface (fail-closed validation): the reservation predicate is a fail-closed pipeline gate. Generalizing it must reject every ordinary embodied action (including `wait`) for an actor with an unresolved body-exclusive start, while exempting only typed continuation/lifecycle controls. The change must not weaken any existing rejection (the current sleep-then-sleep rejection still holds) and must not introduce an origin-dependent branch (human vs autonomous treated identically) — the human and autonomous sleep-then-wait rejection tests are the regression lock.

## Architecture Check

1. Keying the predicate on the proposing actor's open-duration state (via the shared log authority) rather than on the candidate's own kind is the only generalization that catches sleep-then-`wait`; the current candidate-kind keying structurally cannot, because the conflicting action is an ordinary `wait`, not a new body-exclusive start. A continuation exemption keyed on the typed control kind keeps lifecycle controls flowing without reopening the overlap hole.
2. No backwards-compatibility aliasing/shims: the predicate is generalized in place; no parallel "legacy reservation check" is retained, and the continuation exemption is a typed control discriminant, not a player-privilege flag.

## Verification Layers

1. `INV-043` ordinary-agent validation -> replay/golden-fixture check: sleep-then-ordinary-`wait` is rejected with `ReservationConflict` for both human and autonomous origins; sleep-then-continuation succeeds (world advances, no `ActorWaited` for the sleeping actor).
2. `INV-007` NPC-possible -> codebase grep-proof: the generalized predicate has no origin-dependent branch; the same code path evaluates human and autonomous proposals.
3. Continuation exemption -> manual review: a typed continuation control is exempt from the reservation conflict and creates no competing reservation.

## What to Change

### 1. Generalize `body_exclusive_reservation_conflict` (`actions/pipeline.rs`)

Change the predicate to: if the proposing actor has an unresolved body-exclusive start at the proposal tick (derived from `open_body_exclusive_starts`), reject every ordinary embodied action for that actor — including ordinary `wait` — with `ReasonCode::ReservationConflict`. Remove the reliance on the candidate itself being a new body-exclusive start.

### 2. Continuation/lifecycle exemption (`actions/pipeline.rs`)

Exempt typed lifecycle controls (continue time, pause, modeled interruption/cancellation) from the reservation conflict. A continuation control is not an `ActorWaited` event and creates no competing reservation. When interruption is modeled, close the existing duration through its terminal (0047TUIAUTWOR-007) before accepting any new ordinary action. Apply the rule identically to human and autonomous origins.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005)

## Out of Scope

- The typed continuation/advance-until control surface itself (0047TUIAUTWOR-012) — this ticket exempts continuation controls from the conflict; it does not define the control's command path.
- Closing durations / terminal builders (0047TUIAUTWOR-007).
- TUI wiring (0047TUIAUTWOR-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — a sleep-then-ordinary-`wait` proposal is rejected with `ReservationConflict`, for **both** a human-origin and an autonomous-origin proposal.
2. A sleep-then-continuation proposal succeeds: the world advances and no `ActorWaited` event is emitted for the sleeping actor.
3. The existing sleep-then-sleep (candidate-is-new-start) rejection still holds; `cargo clippy -p tracewake-core --all-targets -- -D warnings` clean.

### Invariants

1. The reservation predicate rejects every ordinary embodied action (incl. `wait`) for an actor with an unresolved body-exclusive start, exempting only typed lifecycle controls (`INV-043`).
2. No origin-dependent branch: human and autonomous proposals traverse the identical predicate (`INV-007`/`INV-108`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — sleep-then-`wait` rejection (human + autonomous), sleep-then-continuation success, sleep-then-sleep still-rejected regression.

### Commands

1. `cargo test -p tracewake-core reservation`
2. `cargo test -p tracewake-core && cargo clippy -p tracewake-core --all-targets -- -D warnings`
3. The core-suite boundary is correct: the reservation predicate is a kernel pipeline gate; no TUI surface changes here. `cargo test -p tracewake-core reservation` is the single-positional-filter form (one filter; multiple filters would go after `--`).

## Outcome

Completed: 2026-06-22

Generalized `body_exclusive_reservation_conflict` in `crates/tracewake-core/src/actions/pipeline.rs` so it now keys on the proposing actor's unresolved body-exclusive starts from `open_body_exclusive_starts` at the proposal tick, rather than only checking whether the candidate action itself starts a new body-exclusive duration. Ordinary `wait` and a second sleep now reject with `ReasonCode::ReservationConflict` while an actor has an open sleep/work duration. The predicate has no origin-specific branch; human and scheduler-origin proposals traverse the same reservation check after their normal origin/source gates.

Added a typed lifecycle-control exemption for `continue_routine`. The full continuation command path is still owned by later 0047 tickets, so this ticket verifies the gate behavior and leaves end-to-end sleep-then-continuation wiring to that scope.

Added pipeline tests for human sleep-then-wait rejection, scheduler sleep-then-wait rejection, and sleep-then-sleep regression. Updated the generative lock to expect a typed reservation-conflict rejection instead of a mid-work displacement movement/continuity terminal, matching the new generalized reservation rule.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p tracewake-core reservation` — passed.
3. `cargo test -p tracewake-core --test generative_lock` — passed during focused repair.
4. `cargo test -p tracewake-core` — passed.
5. `cargo clippy -p tracewake-core --all-targets -- -D warnings` — passed.
