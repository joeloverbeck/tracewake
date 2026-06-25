# 0051FOUCONTHI-003: F-02 replay/save reconstruction of runtime authority

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — make loaded-actor eligibility and declared-process declarations reconstructable on replay/restore; rewrite `restore_from_temporal_projection` / `restore_from_rebuild_report`; advance the actor next-decision deterministically.
**Deps**: 0051FOUCONTHI-002

## Problem

`restore_from_temporal_projection` and `restore_from_rebuild_report` each return `Self::new(reconstructed_final_frontier)` (`scheduler.rs:512`, `522`), reconstructing only the clock and discarding loaded-actor eligibility, process declarations, cadence, trigger provenance, and proposal-sequence state. The scheduler never consumes/advances `loaded_actor_next_decision_tick` after running an actor transaction, so an actor remains perpetually due unless an outside caller rewrites the map (F-02, critical; also guards mutation survivor #10, `restore_from_temporal_projection` → `None`). The fix reconstructs all runtime authority on replay/restore through one consistently-chosen design and derives each actor's next opportunity in-core with no outside rescheduling.

## Assumption Reassessment (2026-06-24)

1. Codebase: clock-only restores at `crates/tracewake-core/src/scheduler.rs:508`–`512` (`restore_from_temporal_projection`) and `518`–`522` (`restore_from_rebuild_report`); `TemporalProjection.reconstructed_final_frontier` (`crates/tracewake-core/src/replay/temporal.rs:6`–`7`); `ProjectionRebuildReport` with `reconstructed_final_frontier`/`temporal_violations` (`crates/tracewake-core/src/replay/report.rs:53`–`54`); the live caller is `crates/tracewake-tui/src/app.rs:398` (`restore_from_rebuild_report`). The `-001` runtime owns the due-work state being reconstructed; `-002` established its derivation from loaded content.
2. Specs/docs: spec `0051` §4.3, §9.2 (eligibility representation — event-sourced declarations, deterministic canonical-state derivation, or a dedicated replay projection: implementer-recorded choice), §9.8 (save-artifact boundary). Architecture home `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`.
3. Shared boundary under audit: the save/restore → runtime-authority reconstruction path — restore must rebuild eligibility, cadence/trigger provenance, next-decision state, and proposal ordering, not just the clock frontier.
4. INV-018 (deterministic replay is foundational), INV-092 (deterministic replay is tested), INV-112 (temporal authority): restated — replay reconstructs identical future actor/process selection from initial seed + ordered events + ancestry-preserving snapshots, with no caller repair.
5. Fail-closed / deterministic-replay surface: this is the replay/restore enforcement surface itself. Confirm continuation-equivalence (uninterrupted vs serialize/rebuild/continue) over physical/agent state, event-log suffix, epistemic projection, temporal frontier, actor-opportunity sequence, and process invocations; confirm an actor is not perpetually due after one opportunity (cadence-boundary case); no nondeterministic input enters the reconstructed canonical form. Closes mutation survivor #10.
6. Schema extension (additive-vs-breaking): reconstruction may include a dedicated replay-derived runtime projection in save/restore **and checksums** (`crates/tracewake-core/src/checksum.rs`). Name the consumers — replay rebuild (`replay/report.rs`, `replay/rebuild.rs`) and the checksum input — and choose, per §9.2/§9.8 and INV-020 (event schema evolution): event-sourced runtime declarations projected during rebuild (no save-format change), deterministic derivation from canonical state + event history (no new persisted field), or a new persisted projection field (version-bumped, old saves derive the field). Record the chosen option; default to deterministic derivation to keep the change additive with a derivation fallback. (Keyed distinctly from item 5 because the schema-shape analysis and the replay-determinism analysis are separate obligations.)

## Architecture Check

1. Reconstructing runtime authority through one canonical mechanism (rather than leaving the clock-only restore and patching the registry from outside) is the only design that makes replay byte-faithful for actor/process selection — the §4.3 thesis. `restore_from_temporal_projection` must not *claim* complete restoration if its input cannot reconstruct all runtime authority: either accept the complete runtime projection, or narrow/rename it and prevent production use as a full restore.
2. No backwards-compatibility alias: the clock-only restore semantics are replaced, not retained behind a flag; the next-decision advance is in-core, not a caller hook.

## Verification Layers

1. INV-018 / INV-092 (deterministic replay) -> replay/golden-fixture check: continuation-equivalence over ≥2 actors + a cadenced process across serialize/rebuild/continue.
2. INV-112 (temporal authority) -> replay check: cadence-boundary case proving an actor is not perpetually due after one opportunity.
3. Survivor #10 -> focused mutation kill (run in `-010`): `restore_from_temporal_projection` → `None` is caught by the continuation-equivalence test.

## What to Change

### 1. Reconstruct runtime authority on restore

Make loaded-actor eligibility and process declarations reconstructable through one chosen design (per AR-6). Reconstruction must include process cadence/trigger provenance and actor next-decision state; proposal ordering must be reconstructed or replaced by ordering derived from the event frontier + stable identities.

### 2. Advance next opportunity in-core

After an actor opportunity, the runtime deterministically records or derives the actor's **next** opportunity with no outside rescheduling.

### 3. Honest restore surface

If `restore_from_temporal_projection`'s input cannot reconstruct all runtime authority, narrow/rename it (and bar production use as a full restore) rather than returning a partial `Self::new(frontier)` that claims completeness.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-core/src/scheduler.rs` (modify) — restore paths; next-decision advance; merge-hub contributor
- `crates/tracewake-core/src/replay/temporal.rs` (modify)
- `crates/tracewake-core/src/replay/report.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify — only if the chosen design adds a runtime projection to the checksum input)
- `crates/tracewake-tui/src/app.rs` (modify) — restore call site at `398`
- `crates/tracewake-core/tests/replay_temporal_frontier.rs` (modify) — continuation-equivalence + cadence-boundary

## Out of Scope

- Declared-process transaction effects (F-03 → `-004`); this ticket reconstructs process *declarations/cadence*, not their causal transitions.
- The per-tick census (F-04 → `-005`).
- Restore-mid-duration continuation equivalence beyond eligibility (a §4.10 preserved-property guard; covered in `-009`'s harness).

## Acceptance Criteria

### Tests That Must Pass

1. Continuation-equivalence: load ≥2 actors + a cadenced process, run N ticks, serialize/rebuild, run K more, and compare physical/agent state, event-log suffix, epistemic projection, temporal frontier, actor-opportunity sequence, process invocations, and returned summaries against an uninterrupted run.
2. A cadence-boundary case proves an actor is not perpetually due after one opportunity (next-decision advanced).
3. `cargo test -p tracewake-core --test replay_temporal_frontier` is green.

### Invariants

1. Replay reconstructs identical future actor/process selection without caller repair.
2. The reconstructed canonical form admits no nondeterministic input.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/replay_temporal_frontier.rs` — continuation-equivalence + cadence-boundary cases over runtime authority.

### Commands

1. `cargo test -p tracewake-core --test replay_temporal_frontier`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented deterministic restore reconstruction by changing
`DeterministicScheduler::restore_from_temporal_projection` and
`restore_from_rebuild_report` to rebuild loaded-world due-work authority from
canonical physical state, agent state, reconstructed frontier, and content
manifest id. The chosen design is deterministic derivation from canonical
state plus replay/rebuild frontier, with no persisted save-format or checksum
field added in this ticket.

Updated the TUI rebuild/restore call site to pass the manifest id into
`restore_from_rebuild_report`, so the restored scheduler no longer returns a
clock-only runtime authority. Actor next opportunities now advance in-core after
a due actor transaction is processed, preventing the same eligibility entry from
remaining permanently due unless the caller rewrites it.

Added replay-frontier tests for restore continuation equivalence over a
two-actor loaded world with the derived cadenced process, and for actor
next-opportunity advancement across consecutive ticks.

Deviations:

- Proposal-sequence reconstruction remains represented by deterministic
  scheduler-owned assignment from the restored runtime; no new persisted
  proposal-sequence field was added because this ticket chose canonical
  derivation rather than a save-format extension.

Verification:

- `cargo test -p tracewake-core --test replay_temporal_frontier` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo fmt --all --check` — passed.
