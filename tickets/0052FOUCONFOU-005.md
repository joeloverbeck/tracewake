# 0052FOUCONFOU-005: F4-03 — replay-critical runtime authority reconstruction

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replay restore reconstructs scheduler authority (proposal sequence, actor opportunity, process declaration/cadence/source/random) instead of reseeding a default scheduler; fail-closed on missing authority
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-004

## Problem

Spec 0052 F4-03 (§1.1.3, §4.4): both restore paths reseed a fresh default scheduler. `restore_from_temporal_projection` (`scheduler.rs:544`) and `restore_from_rebuild_report` (`scheduler.rs:562`) each return `Some(Self::from_loaded_world(reconstructed_final_frontier, …))` (scheduler.rs:551/567). `from_loaded_world` (`scheduler.rs:508`) resets the proposal sequence to zero, re-schedules every actor at `current_tick + 1`, re-declares one synthetic every-tick process (`process_loaded_world_tick`, scheduler.rs:523) with no source-event IDs or random provenance, and discards irregular actor opportunity intervals, nonzero proposal sequences, multiple/non-unit-cadence processes, process ancestry, and reservation/deferral state. The current continuation witness starts both branches from that same default topology and compares a shallow next-step projection, so it cannot prove preservation of non-default runtime authority — reconstructing the clock frontier is not reconstructing runtime authority.

This ticket defines replay-critical scheduler authority as an explicit projection or ancestry-preserving snapshot derived from the event log and accepted initial configuration, reconstructing the proposal sequence, per-actor next opportunity and deferral/reservation disposition, every declared process's full ancestry/cadence/random/next-due state, and open duration/reservation state — and makes restore **fail closed** when required authority is absent, ambiguous, unsupported, or inconsistent. Closes standing survivors #8 (`restore_from_temporal_projection -> None`) and #20 (`rebuild_from_owned_log -> Ok(())`, jointly with 004).

## Assumption Reassessment (2026-06-25)

1. `DeterministicScheduler::from_loaded_world` (`scheduler.rs:508`), `restore_from_temporal_projection` (544), and `restore_from_rebuild_report` (562) are as the spec states. The replay machinery is `crates/tracewake-core/src/replay/{mod,temporal,rebuild,report}.rs`; `reconstructed_final_frontier` is defined on the temporal projection (`replay/temporal.rs:7`), rebuild report (`replay/rebuild.rs:31`), and report (`replay/report.rs:53`). `rebuild_from_owned_log` becomes internal-atomic in 0052FOUCONFOU-004; this ticket implements the reconstruction *semantics* it dispatches.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.4; closure-order step 4. The opaque replay seed/snapshot product is from 0052FOUCONFOU-001; the internal atomic rebuild from 0052FOUCONFOU-004.
3. Cross-artifact boundary under audit: the event-log → scheduler-authority reconstruction seam (`replay/*` ↔ `scheduler.rs`). Initial loading may derive declarations from validated content, but continuation must preserve the authority that existed at the replay frontier — a distinct contract from initial construction.
4. Motivating invariants: INV-018 ("Deterministic replay is foundational") and INV-092 ("Deterministic replay is tested"); INV-001/009/010 (eventful causal ancestry); INV-088 (declared process identity/cadence/source/random ancestry). Replay must reconstruct from "initial seed/configuration, authored state, ordered events, meaningful random draw records, ruleset/data versions, and ancestry-preserving snapshots" — the scheduler authority is part of that effective state, not disposable metadata.
5. Fail-closed / deterministic-replay surface (the core enforcement surface of this ticket): restore fails closed when required authority is absent/ambiguous/unsupported/inconsistent with physical/agent/projection state, and does **not** silently synthesize the default process topology. Continuation from a non-default frontier must be byte-identical to uninterrupted execution. This strengthens INV-018/092; it cannot weaken them because the prior behavior (default reseed) is the defect being removed.
6. Serialized-consumer / schema note: §10.4 leaves the representation an implementer-recorded choice — event-derived scheduler projection, ancestry-preserving snapshot, or a combination. If the chosen representation introduces a persisted snapshot that enters the save package / checksum input, it is an additive replay-critical structure whose only consumer is replay/restore (additive, not breaking — no existing serialized field changes shape); if event-derived, no new persisted schema is added. The chosen representation and its additive-vs-breaking status are recorded in the ticket's What-to-Change and the acceptance artifact.

## Architecture Check

1. Reconstructing replay-critical authority (a projection/snapshot derived from log + accepted config) that fails closed is cleaner than reseeding `from_loaded_world` because it makes continuation provably equal to uninterrupted execution and refuses to invent authority it cannot derive — a fresh default scheduler silently diverges while a shallow witness still passes (driver §F4-03). State-machine replication requires replicas to begin from equivalent effective state and process the same ordered requests; scheduler/process opportunity is part of that state.
2. No backwards-compatibility alias: the `from_loaded_world` reseed in the restore paths is replaced, not retained as a fallback — a fallback to default reseed would reintroduce the defect.

## Verification Layers

1. INV-018/INV-092 (deterministic, tested replay) -> uninterrupted-vs-prefix-rebuild differential over the real runtime boundary: ≥3 actors on nonuniform next-opportunity schedules, multiple proposal sequences consumed before save, ≥2 processes with different cadence/source-events/random provenance, an open duration/body reservation, and an actor deferred past the immediate next tick; run a prefix, rebuild from owned history, execute the same next N typed commands, and compare exact event envelopes/order/IDs, proposal ancestry, process triggers, actor dispositions, receipts, physical/agent/epistemic projections, and checksum/frontier.
2. Fail-closed restore -> behavior test: absent/ambiguous/inconsistent authority returns a typed fail-closed result, not a default-topology scheduler.
3. Mutation -> mutants returning `None`, resetting sequences, changing cadence, dropping a process/source, or moving an actor's next tick are killed (#8, #20).

## What to Change

### 1. Replay-critical authority representation (`scheduler.rs`, `replay/*`)

Define replay-critical scheduler authority as an explicit event-derived projection or ancestry-preserving snapshot (representation is the implementer-recorded choice of §10.4, stated here). Reconstruct: next proposal sequence (or deterministic event-derived equivalent); per-loaded-actor next opportunity and deferral/reservation disposition; every declared process identity, trigger/cadence, source-event ancestry, scope, content identity, deterministic random provenance, and next-due state; open duration/reservation state relevant to the next world step.

### 2. Fail-closed restore (`scheduler.rs`, `replay/{temporal,rebuild,report}.rs`)

Replace the `Some(Self::from_loaded_world(...))` reseed in both restore paths with reconstruction from the projection/snapshot; fail closed when authority is absent/ambiguous/unsupported/inconsistent. Do not synthesize the default process topology during continuation.

### 3. Internal atomic rebuild semantics (`session.rs`)

Implement the reconstruction the internal `rebuild_from_owned_log` (made internal in 004) dispatches: replace all aggregates and scheduler authority together, or change nothing.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — replay-authority projection/snapshot; restore reconstructs, fails closed)
- `crates/tracewake-core/src/replay/temporal.rs` (modify — temporal-projection restore reconstruction)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — rebuild-report restore reconstruction)
- `crates/tracewake-core/src/replay/report.rs` (modify — report carries reconstructed authority)
- `crates/tracewake-core/src/runtime/session.rs` (modify — internal atomic rebuild semantics)

## Out of Scope

- Making rebuild a client-vs-internal operation (004 — already internalized).
- The declared-process *transition* effect (006 — this ticket reconstructs the process *declaration/ancestry* on replay; the real transition is 006).
- The actor disposition census product (007).
- The continuation-equivalence corpus packaging and generative-lock mode (009).

## Acceptance Criteria

### Tests That Must Pass

1. The uninterrupted-vs-prefix-rebuild differential (Verification Layer 1) passes with non-default schedules/sequences/processes/reservations and compares exact envelopes/order/IDs/ancestry/projections/checksum — a witness whose restored topology equals the constructor default by construction is rejected as vacuous.
2. Fail-closed restore returns a typed failure (not a default scheduler) when authority is absent/ambiguous/inconsistent.
3. Focused mutation over the restore/reconstruction functions kills #8 and #20; `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Continuation from a reconstructed non-default frontier is byte-identical to uninterrupted execution (INV-018, INV-092).
2. Restore never silently synthesizes the default every-tick process topology (INV-088).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/replay_temporal_frontier.rs` (and replay report tests) — non-default uninterrupted-vs-restored continuation differential; fail-closed cases.
2. `crates/tracewake-core/src/scheduler.rs` test module — focused reconstruction unit tests for #8/#20 sensitivity.

### Commands

1. `cargo test -p tracewake-core --test replay_temporal_frontier`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
