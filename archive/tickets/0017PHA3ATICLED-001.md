# 0017PHA3ATICLED-001: Single open-duration authority and duplicate-terminal rejection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`need_accounting`, `scheduler`, `actions/pipeline`, `replay/rebuild`, `events/apply`); one new replay negative gate
**Deps**: `archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-033); no ticket dependencies

## Problem

Three authorities decide whether a body-exclusive duration (sleep/work) is open, and they pair starts to terminals by two different keys: `actions/pipeline.rs::body_exclusive_reservation_conflict` and `need_accounting.rs::terminal_ticks_by_start` pair via `EventCause::Event(start_id)`, while `scheduler.rs::actor_has_open_body_exclusive_duration` pairs via `proposal_id`. A terminal carrying the correct event-id cause but a divergent/absent proposal id closes the duration for two authorities and not the third. Additionally, `terminal_ticks_by_start` resolves contradictory duplicate terminals (e.g. both `WorkBlockCompleted` and `WorkBlockFailed` for one start) with `.and_modify(|tick| *tick = (*tick).min(...))` — silent reconciliation of illegal history instead of a typed error.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `scheduler.rs::actor_has_open_body_exclusive_duration` matches terminals on `event.proposal_id.as_ref() == proposal_id`; `need_accounting.rs::terminal_ticks_by_start` uses `.and_modify(|tick: &mut u64| *tick = (*tick).min(event.sim_tick.value()))`; `pipeline.rs` resolves causes via `EventCause::Event(event_id)`. The shared terminal predicate `events/envelope.rs::is_duration_terminal` already exists (exhaustive, wildcard-free) — the *predicate* is unified, the *pairing key* is not.
2. Spec 0017 §ORD-HARD-033 and §5 tier 1 require a single shared `open_body_exclusive_starts` authority keyed by `EventCause::Event`, consumed by the reservation check, the scheduler window-skip, and the tick classifier, plus typed rejection of >1 terminal per start. Spec §8 orders this ticket first because sibling `-002`'s classifier consumes the shared authority.
3. Not a cross-skill ticket; the shared boundary under audit is intra-crate: the start/terminal pairing contract between the event log shape and its three core consumers.
4. INV-009 motivates the duplicate-terminal correction (restated): meaningful state changes require events with an honest cause model — silently `min()`-reconciling two contradictory terminal events for one start hides an illegal history instead of rejecting it.
5. Deterministic-replay surface touched: replay rebuild (`replay/rebuild.rs`) gains a poisoning condition (duplicate terminal ⇒ rebuild error, `matches_expected` fails). This strengthens, not weakens, replay strictness; no epistemic-leakage surface is involved. All existing golden logs have exactly one terminal per start, so golden checksums must not change in this ticket.
6. Adjacent contradiction classified: the wait/passive awake double-charge (spec ORD-HARD-026) shares the tick-accounting file but is a separate ticket (`0017PHA3ATICLED-002`, which depends on this one); this ticket must not alter need-delta arithmetic.

## Architecture Check

1. One shared pairing function is structurally cleaner than three predicates with two keys: drift becomes impossible by construction rather than caught by tests, matching the spec's tier-1 (compile-time impossibility) lock preference. Keying by `EventCause::Event` is correct because causal ancestry is the authoritative pairing channel (INV-010); `proposal_id` is provenance metadata, not causality.
2. No backwards-compatibility aliasing/shims: the `proposal_id`-keyed matching in the scheduler is deleted, not wrapped or kept as a fallback.

## Verification Layers

1. Single pairing authority -> codebase grep-proof: no `proposal_id`-keyed terminal matching remains in `actor_has_open_body_exclusive_duration`; all three consumers call `open_body_exclusive_starts` (or its tick-classifier equivalent).
2. INV-009 duplicate-terminal rejection -> replay/golden-fixture check: new negative gate `duplicate_duration_terminal_poisons_rebuild_001` appends a second terminal to a copied log and asserts rebuild reports a typed error and `matches_expected` is false.
3. INV-018 replay byte-stability -> existing golden runs (`cargo test --workspace`) pass unchanged — this ticket changes pairing mechanics, not event content.
4. Key-divergence behavior -> unit test: a forged terminal with correct `EventCause::Event` ancestry but mismatched `proposal_id` closes the duration identically for the reservation check, the window-skip, and the classifier.

## What to Change

### 1. Shared open/closed authority in `need_accounting.rs`

Add `pub(crate) fn open_body_exclusive_starts(log: &EventLog, actor_id: &ActorId, as_of_tick: SimTick) -> BTreeSet<EventId>`: collect `SleepStarted`/`WorkBlockStarted` for the actor at or before `as_of_tick`, remove those with a terminal (per `is_duration_terminal`) whose causes contain `EventCause::Event(start_id)`. Rebase `terminal_ticks_by_start` and the duration-interval construction on the same pairing helper so the classifier cannot drift from it.

### 2. Duplicate-terminal typed error

Replace the `.and_modify(min)` reconciliation: encountering a second terminal for an already-terminated start returns a typed error (new variant on the existing apply/replay error vocabulary in `events/apply.rs` / `replay/rebuild.rs` — e.g. `DuplicateDurationTerminal { start_event_id }`). Rebuild treats it as a poisoning application error (existing first-error break path); the live append path rejects the event at application time.

### 3. Consumers

`scheduler.rs::actor_has_open_body_exclusive_duration` delegates to `open_body_exclusive_starts` (delete the `proposal_id` matching). `actions/pipeline.rs::body_exclusive_reservation_conflict` derives its open set from the same function.

### 4. Tests

In-file unit tests for the forged divergent-key and double-terminal cases; the `duplicate_duration_terminal_poisons_rebuild_001` gate in `event_schema_replay_gates.rs`.

## Files to Touch

- `crates/tracewake-core/src/need_accounting.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — typed error variant, as surfaced by the existing error enum layout)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)

## Out of Scope

- Need-delta arithmetic and the wait/passive double-charge (ticket `0017PHA3ATICLED-002`).
- Any change to golden checksums, event payloads, or event kinds.
- Provenance, projection, replay-payload, content, or lock-layer findings (sibling tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core open_body_exclusive` — forged divergent-key terminal closes the duration for all three consumers; reservation, window-skip, and classifier agree.
2. `cargo test -p tracewake-core duplicate_duration_terminal` — double terminal yields the typed error live and poisons rebuild in the new gate.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Exactly one function in the workspace decides whether a body-exclusive start is open; all consumers call it.
2. A log with two terminals for one start is rejected history (INV-009/INV-020 posture), never silently reconciled.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/need_accounting.rs` (unit tests) — divergent-key closure agreement; duplicate-terminal typed error.
2. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — `duplicate_duration_terminal_poisons_rebuild_001` negative replay gate.

### Commands

1. `cargo test -p tracewake-core duplicate_duration_terminal`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-10

What changed:

- Added `need_accounting::open_body_exclusive_starts` as the shared event-cause keyed open-duration authority for body-exclusive `SleepStarted` / `WorkBlockStarted` events.
- Rebased the tick-regime interval builder on checked terminal pairing and changed duplicate duration terminals from silent earliest-tick reconciliation to a typed `DuplicateDurationTerminal` error.
- Rewired the action pipeline reservation check and the no-human scheduler open-duration skip to consume `open_body_exclusive_starts` instead of maintaining independent closed-start / `proposal_id` matching logic.
- Added live agent-apply rejection for duplicate duration terminals by recording event-cause ancestry on ordinary-life episode projection records.
- Added replay invariant detection for duplicate duration terminals and a `duplicate_duration_terminal_poisons_rebuild_001` gate that makes `run_replay(...).matches_expected` false for tampered duplicate-terminal history.
- Corrected `no_human_day_fixture_has_roster_activity_and_metrics_envelope` to complete the `SleepStarted` event returned by the manual sleep proposal it just ran, instead of accidentally selecting an earlier same-actor sleep start from the whole log. The new duplicate-terminal enforcement exposed that stale test selection.

Deviations from original plan:

- Replay duplicate-terminal detection is performed as a log-level invariant before rebuild application, while live rejection is performed in `apply_agent_event` using the ordinary-life episode projection. This keeps tampered replay histories fail-closed and live application typed without changing event schemas or golden checksums.
- The direct forged divergent-key assertion landed in `need_accounting.rs`, proving the shared authority and classifier close by `EventCause::Event` despite a mismatched `proposal_id`. The existing scheduler open-duration test exercises the consumer path.

Verification results:

- `cargo test -p tracewake-core open_body_exclusive` — passed.
- `cargo test -p tracewake-core duplicate_duration_terminal` — passed.
- `cargo test -p tracewake-content --test golden_fixtures_run no_human_day_fixture_has_roster_activity_and_metrics_envelope` — passed after the stale-start test correction.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
