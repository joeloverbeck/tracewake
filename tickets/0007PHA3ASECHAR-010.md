# 0007PHA3ASECHAR-010: Replay & provenance for Phase 3A state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` replay rebuild and report (`replay/rebuild.rs`, `replay/report.rs`), checksums (`checksum.rs`), provenance projection (`projections.rs`)
**Deps**: 0007PHA3ASECHAR-004, 0007PHA3ASECHAR-005, 0007PHA3ASECHAR-006, 0007PHA3ASECHAR-007

## Problem

Replay must rebuild the meaningful Phase 3A state and metrics from the event log, and live-vs-replay checksums must include Phase 3A agent state and no-human metrics (Spec 0007 §Replay and provenance, D-02/D-05; Binding constraint 9). At the audited commit replay rebuilds physical state and some agent state, but replay equivalence must compare live and replay-rebuilt projections for needs, routine executions, intention state, no-human metrics, and decision/stuck traces, the no-human report must be reproducible from the event log (not side-channel scheduler state), and provenance for ordinary actions must link decision trace → method/routine step → planned proposal → validation report → emitted event → state change.

## Assumption Reassessment (2026-06-07)

1. Confirmed surfaces: `crates/tracewake-core/src/replay/rebuild.rs`, `crates/tracewake-core/src/replay/report.rs`, `crates/tracewake-core/src/checksum.rs` (`AgentStateChecksum` at `checksum.rs:160`, `AgentStateChecksumReport` at `checksum.rs:177`), `crates/tracewake-core/src/projections.rs`. The agent-state event kinds (needs, intentions, routine steps, `DecisionTraceRecorded`, `StuckDiagnosticRecorded`, `NoHumanDayStarted/Completed`, `ReplayProjectionRebuilt`) all exist (`events/envelope.rs`). Tickets 004–007 emit them with correct ancestry; this ticket proves replay reconstructs them.
2. Spec 0007 §Replay and provenance requires replay rebuild of physical/agent/epistemic/no-human-metrics/routine/intention/decision/stuck state; live-vs-replay checksums including Phase 3A agent state and no-human metrics; log-derived no-human report; and the full provenance chain. Binding constraint 9 names needs, routine executions, intention state, no-human metrics, and decision/stuck traces as the state that matters.
3. Shared boundary under audit: the replay-rebuild contract between the event log (source of truth) and the rebuilt projections + checksums (consumers). Every Phase 3A agent-state field that tickets 004–007 emit must be reconstructed identically and folded into `AgentStateChecksum`; the no-human report must derive from the log.
4. Motivating invariants (restated): INV-018 "Deterministic replay is foundational" — replay reconstructs significant outcomes from ordered events; INV-019 "Snapshots and compaction may not erase live ancestry"; INV-011 "Current-state-only simulation is forbidden" — enough ancestry survives to explain consequences; INV-017 "Randomness must be seedable and auditable".
5. Deterministic-replay surface touched (central): this ticket *is* the determinism enforcement for Phase 3A. Live and replay-rebuilt need/intention/routine/metric/trace state must be byte-identical; the no-human report must be regenerable from the log alone. No actor-knowledge leakage: replay exports are not embodied views; the actor-knowledge firewall on view models is owned by 0007PHA3ASECHAR-009. Any change to checksum *semantics* (adding Phase 3A agent-state fields to the canonical checksum) is an extension of the existing `AgentStateChecksum` contract — additive over agent-state fields the prior phase already checksums, not a new hash algorithm, so no foundational-doc amendment is required; confirm the canonical field ordering stays stable.

## Architecture Check

1. Folding the Phase 3A agent-state fields into `AgentStateChecksum` and deriving the no-human report from the log makes replay equivalence a real gate over the state that matters, not just physical state — catching any side-channel scheduler state that fails to round-trip. Reusing the existing checksum/report infrastructure (extended, not replaced) keeps one canonical determinism surface.
2. No backwards-compatibility aliasing/shims: the no-human report reads the log, not side-channel scheduler state; no parallel non-replayed metric path remains.

## Verification Layers

1. INV-018 (replay equality) -> replay/golden-fixture check: live-vs-replay physical checksum equality AND agent-state checksum equality over the no-human fixture (needs, routine executions, intention state, decision/stuck traces).
2. INV-018 (log-derived metrics) -> replay/golden-fixture check: log-derived no-human metrics are byte-identical to the live metrics; the no-human report regenerates from the event log.
3. INV-011 (provenance chain) -> manual review + test: an ordinary action's provenance links decision trace → method/routine step → planned proposal → validation report → emitted event → state change.

## What to Change

### 1. Rebuild Phase 3A agent state in replay

Extend `replay/rebuild.rs` to reconstruct needs, routine executions, intention state, no-human metrics, decision traces, and stuck diagnostics from the event log.

### 2. Extend agent-state checksum and the no-human report

Fold the Phase 3A agent-state fields into `AgentStateChecksum` (`checksum.rs`) with stable canonical ordering, and make `replay/report.rs` derive the no-human report from the log. Add a provenance projection in `projections.rs` linking the full ordinary-action chain.

## Files to Touch

- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/replay/report.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)

## Out of Scope

- Emitting the Phase 3A events with ancestry (0007PHA3ASECHAR-004/005/006/007, dependencies) — this ticket reconstructs them.
- The debug replay-comparison panel rendering (0007PHA3ASECHAR-009).
- The capstone end-to-end assertion (0007PHA3ASECHAR-012) — this ticket provides the replay equivalence the capstone invokes.

## Acceptance Criteria

### Tests That Must Pass

1. Live-vs-replay physical checksum equality and agent-state checksum equality over the no-human fixture.
2. Log-derived no-human metrics are byte-identical; the no-human report regenerates from the event log.
3. Decision-trace and routine/intention state survive replay; an ordinary action's full provenance chain is reconstructible.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Every meaningful Phase 3A state field round-trips through replay and is folded into the canonical agent-state checksum with stable ordering.
2. The no-human report derives from the event log, not side-channel scheduler state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/replay/rebuild.rs` — unit tests: needs/intention/routine/metric reconstruction.
2. `crates/tracewake-core/src/checksum.rs` — agent-state checksum includes Phase 3A fields; stable ordering.
3. `crates/tracewake-core/tests/golden_scenarios.rs` — live-vs-replay equality over the no-human fixture.

### Commands

1. `cargo test -p tracewake-core replay`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets -- -D warnings`
