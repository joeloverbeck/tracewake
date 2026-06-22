# 0047TUIAUTWOR-006: Shared log-derived open-duration discovery

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/tracewake-core` (`scheduler.rs` coordinator discovery phase, `need_accounting.rs` shared authority wiring); core test
**Deps**: 0047TUIAUTWOR-005

## Problem

Spec 0047 §4.2 requires the coordinator's open-duration discovery to be rebuilt from the shared event-log authority rather than batch-local pending-start vectors. Today `append_due_completions` (`scheduler.rs:911`) drains `pending_sleep_starts`/`pending_work_starts` — `Vec<EventEnvelope>` local to its enclosing run — so a `SleepStarted` from a prior, independent TUI transaction is invisible to it (spec §3.1). The shared authority `open_body_exclusive_starts` (`need_accounting.rs:124`) already scans authoritative log evidence and fails closed on duplicate terminal evidence. This ticket wires the coordinator's discovery phase (the §4.9 item-3 seam left in 0047TUIAUTWOR-005) to query unresolved body-exclusive starts from the shared log authority, with deterministic ordering and duplicate/orphan fail-closed behavior, so a duration started in any prior transaction is discoverable.

## Assumption Reassessment (2026-06-22)

1. `open_body_exclusive_starts` exists at `crates/tracewake-core/src/need_accounting.rs:124` with signature `pub(crate) fn open_body_exclusive_starts(log: &EventLog, actor_id: &ActorId, as_of_tick: SimTick) -> Result<BTreeSet<EventId>, DuplicateDurationTerminal>` — it scans `log.events()` for `SleepStarted`/`WorkBlockStarted` without terminal partners and returns a deterministically-ordered `BTreeSet`, failing closed on duplicate terminal evidence. It is `pub(crate)`; the coordinator lives in the same crate (`tracewake-core/src/scheduler.rs`), so visibility is sufficient — no re-export needed. The expected terminal tick and duration kind are read from each start's typed payload (`build_sleep_start_event` records `expected_completion_tick` and `body_exclusive`, `sleep.rs:69-73`).
2. `append_due_completions` at `scheduler.rs:911` takes `pending_sleep_starts: &mut Vec<EventEnvelope>` and `pending_work_starts: &mut Vec<EventEnvelope>` and drains them filtered by completion tick — the batch-local authority spec §4.2 replaces/wraps. This ticket makes the coordinator's discovery phase query the shared log authority; the batch-local vectors remain only inside the legacy `run_no_human_day`/`advance_no_human` paths until 0047TUIAUTWOR-010 removes the duplicate authority.
3. Cross-artifact boundary under audit: the coordinator (0047TUIAUTWOR-005) and the shared open-duration authority (`need_accounting.rs`) are the same fact — "which durations are open at tick t" — that must have one source of truth (spec §8: log-derived discovery is authoritative; any index is a derived cache). The discovery phase output feeds the duration-lifecycle phase (0047TUIAUTWOR-007).
4. Constitutional invariant motivating the ticket: `INV-019` (snapshots/compaction may not erase live ancestry — open-duration state is rebuildable from the log with no hidden scheduler memory) and `INV-011` (current-state-only simulation forbidden — discovery scans event ancestry, not a materialized cache).
5. Enforcement surface (deterministic replay + fail-closed): replay must reconstruct the same open-duration set without hidden scheduler memory; the `Result<_, DuplicateDurationTerminal>` return keeps duplicate/orphan terminals fail-closed. The change preserves deterministic ordering (`BTreeSet<EventId>` + existing ordering authorities) and introduces no in-memory pending queue the coordinator depends on — the mid-duration save/rebuild/resume case (§6) is the regression lock.

## Architecture Check

1. Reusing `open_body_exclusive_starts` as the single discovery authority — rather than threading the batch-local pending vectors into the coordinator — is the only design that lets a duration started in a prior independent transaction (human or autonomous) be found and completed after rebuild, which the batch-local vectors structurally cannot do. A performance index, if later added, is a derived cache, never a second source of truth.
2. No backwards-compatibility aliasing/shims: the coordinator queries the shared authority directly; it does not keep a parallel pending-vector path. (The legacy vectors survive only inside the not-yet-refactored runners, removed in 0047TUIAUTWOR-010.)

## Verification Layers

1. `INV-019` ancestry preservation -> replay/golden-fixture check: a duration started in a prior, independent transaction is discovered after rebuild with no in-memory pending queue (new test resuming a pre-existing start).
2. `INV-011` no current-state-only -> codebase grep-proof: the coordinator's discovery phase reads `open_body_exclusive_starts(log, …)`, not a materialized pending vector.
3. Fail-closed -> schema/replay check: duplicate terminal evidence yields `DuplicateDurationTerminal` (the existing fail-closed behavior is preserved by reuse).

## What to Change

### 1. Coordinator discovery phase (`scheduler.rs`)

Replace the empty discovery seam (from 0047TUIAUTWOR-005) with a query over unresolved body-exclusive starts derived from the shared log authority (`open_body_exclusive_starts`). For each open start, read its expected terminal tick and duration kind from the typed payload; deterministically order due candidates by the existing ordering authorities. Output the ordered due-candidate set for the duration-lifecycle phase (0047TUIAUTWOR-007).

### 2. Shared-authority wiring (`need_accounting.rs`)

Expose / adjust the open-duration query as needed for the coordinator's same-crate use (keep `pub(crate)`; do not widen visibility beyond the crate). Preserve the `Result<_, DuplicateDurationTerminal>` fail-closed contract and the `BTreeSet`-ordered output. Do not duplicate the scan logic in `scheduler.rs`.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/need_accounting.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — file created by 0047TUIAUTWOR-005)

## Out of Scope

- Invoking the completion builders / appending terminals (0047TUIAUTWOR-007) — this ticket discovers, it does not complete.
- Removing the batch-local pending vectors from `run_no_human_day`/`advance_no_human` (0047TUIAUTWOR-010).
- Unified need accounting (0047TUIAUTWOR-008) and reservation enforcement (0047TUIAUTWOR-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test world_step_coordinator` — a sleep/work start appended in a prior, independent transaction (no batch-local pending vector available) is discovered by the coordinator's discovery phase at the correct due tick.
2. A duplicate-terminal scenario yields a fail-closed `DuplicateDurationTerminal` error through the discovery path.
3. `cargo test -p tracewake-core` — full core suite passes; `cargo clippy -p tracewake-core --all-targets -- -D warnings` clean.

### Invariants

1. The coordinator's open-duration set is derived solely from the event log (`open_body_exclusive_starts`), with no scheduler-held pending queue as a source of truth (`INV-019`/§8).
2. Deterministic ordering of due candidates is preserved (`BTreeSet`/existing ordering authorities); replay reconstructs the identical open-duration set.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — add "resume from pre-existing independent start" + duplicate-terminal fail-closed cases.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core && cargo clippy -p tracewake-core --all-targets -- -D warnings`
3. The core-suite boundary is correct here: discovery is a kernel-internal phase with no TUI/no-human wiring change yet.
