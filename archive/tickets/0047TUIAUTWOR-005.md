# 0047TUIAUTWOR-005: Core one-tick coordinator + `TimeAdvanced` marker

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/tracewake-core` (new world-step coordinator type boundary in `scheduler.rs`; `TimeAdvanced` emission + envelope classifier reconciliation in `events/envelope.rs`); new core test
**Deps**: 0047TUIAUTWOR-001, 0047TUIAUTWOR-002, 0047TUIAUTWOR-003

## Problem

Spec 0047 §4.1 requires a reusable, kernel-owned authoritative one-tick coordinator in `tracewake-core` that accepts a typed world-advance request, executes exactly one canonical tick under a single frozen deterministic intra-tick phase order, and returns a typed step result — atomic with respect to acceptance (either a coherent ordered event set + projection update, or a fail-closed error with no half-applied tick). Today `DeterministicScheduler::advance_one_tick` (`scheduler.rs:212`) only increments a counter; no symbol expresses "advance one authoritative world tick." This ticket establishes the **type boundary and the tick-ancestry marker** (§4.9 decomposition item 2): the typed request/result, the coordinator skeleton with its phase ordering, the `TimeAdvanced` marker emission (§4.5), and the proof that an otherwise-empty tick rebuilds to the same temporal frontier. The duration-discovery, completion, and unified-accounting phases are seams filled by 0047TUIAUTWOR-006/007/008.

## Assumption Reassessment (2026-06-22)

1. `DeterministicScheduler::advance_one_tick` exists at `crates/tracewake-core/src/scheduler.rs:212` and is counter-only (`self.current_tick = self.current_tick.next()`). Its only consumers are in-crate: `scheduler.rs:2195` and `:2240` (inside `advance_no_human`) and one test at `:5454`. Renaming or privatizing it (so public callers cannot advance the frontier by mutating the counter alone, per §4.1) has a small, in-crate blast radius — all three sites land in this ticket or are covered by the 0047TUIAUTWOR-010 refactor; this ticket updates the two `advance_no_human` call sites to go through the coordinator's private increment or leaves `advance_no_human` on the existing path until 0047TUIAUTWOR-010 (the seam is preserved, not double-defined).
2. `TimeAdvanced` is already an `EventKind` variant (`events/envelope.rs:91`) but is **defined-but-unemitted** — it appears only in a test fixture (`events/log.rs`), never constructed in the action pipeline. Spec §3.1 records this and §4.5 directs operationalizing it (subject to a narrow semantic audit). No `EventKind` enum change is needed; the change is emission + classifier reconciliation.
3. Cross-artifact boundary under audit: the coordinator is the kernel-owned authoritative world-step seam that the no-human runner (0047TUIAUTWOR-010) and the TUI (0047TUIAUTWOR-011) both loop. Its typed request/result and frozen phase order operationalize the architecture `04`/`05` coordinator standard (0047TUIAUTWOR-002/003). The phase order itself is left singular-and-shared but is frozen only after compatibility tests against existing ordinary-life/replay fixtures (spec §9 open question 1) — this ticket fixes the skeleton order and the empty-tick endpoints; 006/007/008 populate the inner phases without reordering.
4. Constitutional invariants motivating the ticket: `INV-018`/`INV-092` (deterministic replay is foundational and tested — an empty tick must rebuild to the same frontier), `INV-112` (temporal authority — the coordinator makes authoritative time validate/order; temporal facts reach actors only through holder-known projections), and `INV-010` (every event needs a cause model — see the `TimeAdvanced` reconciliation in item 6).
5. Enforcement surface (fail-closed validation + deterministic replay): the coordinator's atomic-with-respect-to-acceptance boundary (no half-applied tick on error) and the replay rebuild of the temporal frontier. The `TimeAdvanced` marker is world/replay evidence and must **never** enter holder-known projections (`INV-024`/`INV-067`) — it carries controller/process origin as non-epistemic scheduling metadata only. The change introduces no nondeterministic input into the canonical form (no wall-clock; the marker's prior/resulting tick come from the authoritative `SimTick`), and the empty-tick rebuild is the leakage/determinism regression lock.
6. Schema extension (event emission, not enum shape): `TimeAdvanced` transitions from defined-but-unemitted to **emitted on every executed world step**. The enum variant pre-exists, so this is *additive* at the type level. But emitting it changes event-log content, so it is **breaking for existing golden traces and replay checksums**: consumers are the envelope classifiers in `events/envelope.rs` (`is_duration_terminal` → already `false`; `EventStream::World`; `physical_mutating` → true; `cause_required` → currently `false` at `envelope.rs:474-476`), the replay/rebuild path, projections, and any golden/checksum baseline. Per spec §6 and the reassessment M2 finding, affected golden traces/checksums are regenerated under review as an intentional update; the marker is never suppressed to keep a golden green (§8 single tick definition). Per the reassessment M1 finding, this ticket must **reconcile the `cause_required(TimeAdvanced)` classification** with §10's `INV-010` claim that the marker carries a cause model (controller/process origin + ordering ancestry): record an explicit decision — keep the kind cause-exempt with a stated rationale, or move it into the cause-required set and supply the cause payload — so the classifier and the `INV-010` alignment do not silently diverge (spec §1 item 6, §4.5).

## Architecture Check

1. A single kernel-owned coordinator with one frozen deterministic phase order is the only design that lets human wait, duration continuation, no-human progression, and future acceleration all loop the *same* primitive (§8: one semantic definition of a loaded-world tick). The alternative — a TUI-side or per-mode tick loop — would create a second time-progression authority, the exact defect spec 0047 exists to remove.
2. No backwards-compatibility aliasing/shims: `advance_one_tick` becomes the coordinator's private clock increment (or is renamed) rather than kept as a parallel public "advance" that bypasses the canonical step; no wrapper preserving the counter-only path is introduced.

## Verification Layers

1. `INV-018`/`INV-092` deterministic replay -> replay/golden-fixture check: an otherwise-empty `TimeAdvanced` tick rebuilds to the same temporal frontier (new test in `tests/world_step_coordinator.rs`).
2. `INV-010` cause model -> codebase grep-proof + invariants alignment check: the recorded `cause_required(TimeAdvanced)` decision in `events/envelope.rs` is consistent with the marker's carried origin/ancestry; the reconciliation is documented in the diff.
3. `INV-024`/`INV-067` no-leak -> manual review (epistemic-leakage audit): the `TimeAdvanced` marker is asserted world/replay-only and is not present in any holder-known projection constructor.
4. `INV-112` temporal authority -> replay/golden-fixture check: the rebuilt temporal frontier matches after the empty tick; the coordinator's typed result exposes the resulting tick without a private possessed-actor clock.

## What to Change

### 1. Typed world-advance request + step result (`scheduler.rs`)

Introduce a typed world-advance request (carrying the expected/frontier tick and controller/process origin) and a typed step result (resulting tick, appended event ids, due-work summary, replay-relevant ancestry). These are the kernel's public contract for "advance one world tick"; the TUI and no-human runner submit the request and consume the result.

### 2. Canonical one-tick coordinator skeleton (`scheduler.rs`)

Add the coordinator that performs, in one owned transaction boundary with a single frozen deterministic intra-tick phase order: (1) frontier validation — reject a stale/mismatched expected tick fail-closed; (2) tick-ancestry append (`TimeAdvanced`); (3) [seam] log-derived open-duration discovery — stub returning empty until 0047TUIAUTWOR-006; (4) [seam] duration lifecycle — until 0047TUIAUTWOR-007; (5) actor dispositions — the possessed actor's ordinary action when free + due autonomous proposals, all through the ordinary pipeline; (6) [seam] reservation enforcement — until 0047TUIAUTWOR-009; (7) [seam] unified accounting — until 0047TUIAUTWOR-008; (8) due world processes; (9) perception/epistemics; (10) projection/replay finish. The step is atomic with respect to acceptance: either it appends a coherent ordered event set and updates projections, or it returns a fail-closed error with no half-applied tick.

### 3. `advance_one_tick` → private clock increment (`scheduler.rs`)

Make the counter-only `advance_one_tick` the coordinator's private clock increment (or rename it so it cannot be mistaken for a complete world step). Public callers must not advance the authoritative frontier by mutating that counter alone.

### 4. `TimeAdvanced` emission + classifier reconciliation (`events/envelope.rs`)

Emit one `TimeAdvanced` marker per executed world step, carrying prior/resulting tick, controller/process origin as non-epistemic scheduling metadata, and ordering ancestry sufficient to reproduce the step. Reconcile the envelope classification (the `cause_required(TimeAdvanced)` decision per item 6 / M1; confirm `EventStream::World` and physically-mutating remain correct for an empty-tick marker). Acceleration is repeated one-tick markers, never a single `t`→`t+n` jump.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (new)

## Out of Scope

- Log-derived open-duration discovery (0047TUIAUTWOR-006), duration completion/interruption (0047TUIAUTWOR-007), unified accounting (0047TUIAUTWOR-008), reservation enforcement (0047TUIAUTWOR-009) — the coordinator exposes these as seams here.
- Refactoring `run_no_human_day`/`advance_no_human` onto the coordinator (0047TUIAUTWOR-010).
- Any TUI wiring (0047TUIAUTWOR-011) or interval projection (0047TUIAUTWOR-013).
- Regenerating downstream golden traces beyond what the empty-tick proof needs — the broad golden/checksum regeneration lands with the no-human refactor (0047TUIAUTWOR-010) per M2.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test world_step_coordinator` — an empty world step appends exactly one `TimeAdvanced` marker, advances the frontier by one tick, and rebuilds (replay) to the identical temporal frontier.
2. A fail-closed test: a world-advance request with a stale/mismatched expected tick is rejected with a typed error and appends no events (no half-applied tick).
3. `cargo test -p tracewake-core` — full core suite passes; any existing replay/golden assertion that now observes a `TimeAdvanced` marker is updated as an intentional, reviewed regeneration (M2), and `cargo build -p tracewake-core --all-targets --locked` is clean.

### Invariants

1. There is exactly one public way to advance the authoritative frontier — the coordinator; `advance_one_tick` is no longer a public complete-world-step path (`INV-112`/§8).
2. `TimeAdvanced` appears in no holder-known projection constructor (`INV-024`); its `cause_required` classification is reconciled and recorded against `INV-010`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (new) — empty-tick rebuild proof + stale-frontier fail-closed test; the standing home later tickets (006/007/008) extend.
2. `crates/tracewake-core/src/events/envelope.rs` (cfg(test)) — assert the reconciled `cause_required(TimeAdvanced)` decision and the marker's `EventStream::World` classification.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core && cargo clippy -p tracewake-core --all-targets -- -D warnings`
3. `cargo test --workspace` is the full-pipeline boundary once downstream goldens are regenerated; for this ticket the core suite plus the new coordinator test is the correct narrower boundary since no TUI/no-human wiring changes yet.

## Outcome

Completed: 2026-06-22

Implemented the core one-tick coordinator skeleton in
`crates/tracewake-core/src/scheduler.rs`:

- Added typed `WorldAdvanceRequest`, `WorldAdvanceOrigin`,
  `WorldAdvanceResult`, `WorldStepDueWorkSummary`, and `WorldAdvanceError`.
- Added `DeterministicScheduler::advance_world_one_tick`, which validates the
  expected frontier, appends a caused `TimeAdvanced` marker, advances the
  authoritative frontier by one tick, and returns a typed result. The
  duration-discovery, duration-lifecycle, accounting, actor-transaction, and
  world-process seams intentionally report zero work here for later tickets.
- Renamed the counter-only `advance_one_tick` to private
  `increment_clock_one_tick`; existing no-human paths still use that internal
  counter path until 0047TUIAUTWOR-010 refactors them onto the coordinator.
- Reconciled `TimeAdvanced` in `events/envelope.rs` by making it cause-required
  while preserving its `World` stream and physical world-no-op replay behavior.
  The marker carries prior/resulting tick, origin, and ordering ancestry
  payload fields and is caused by a scheduler/process-origin cause.
- Added `crates/tracewake-core/tests/world_step_coordinator.rs` for empty-tick
  replay/frontier proof, stale-frontier fail-closed behavior, and marker
  metadata classification.

Cause-model decision: `TimeAdvanced` is now `cause_required = true`. Controller
origin is represented as non-epistemic scheduler/process ancestry in the marker
cause plus payload, rather than adding a new cause enum variant in this ticket.
That keeps the marker replay/world evidence only and avoids treating controller
metadata as actor knowledge.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core --test world_step_coordinator` passed: 3 tests.
- `cargo test -p tracewake-core` passed, including the anti-regression guard
  that rejects panic-capable scheduler/apply/completion paths.
- `cargo clippy -p tracewake-core --all-targets -- -D warnings` passed.
- `cargo build -p tracewake-core --all-targets --locked` passed.
- `rg -n "TimeAdvanced|time_advanced" crates/tracewake-core/src/agent crates/tracewake-core/src/epistemics crates/tracewake-core/src/projections.rs crates/tracewake-core/src/view_models.rs crates/tracewake-core/src/debug_reports.rs`
  printed no matches, confirming the marker is not consumed by holder-known
  projection or view-model constructors.

No broad golden/checksum regeneration was done in this ticket; the existing
no-human paths remain on the internal counter path until 0047TUIAUTWOR-010, so
there was no downstream golden update to review here.
