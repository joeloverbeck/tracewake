# 0019PHA3AGENREA-008: Embodied salient-interruption surfacing

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections.rs` embodied status derivation); `tracewake-tui` render assertions as surfaced; fixture/test coverage
**Deps**: `specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md` (ORD-HARD-052)

## Problem

The embodied `salient_interruption` surface is dead (`ORD-HARD-052`).
`projections.rs::phase3a_status` constructs
`Phase3AEmbodiedStatus { …, salient_interruption: None }` — the field's only producer —
so the TUI render arm (`render.rs::render_embodied_view`'s
`if let Some(interruption) = &status.salient_interruption`) is unreachable.
Sleep/routine interruption is a live Phase 3A mechanic (`SleepInterrupted` events are
materialized; the no-human scheduler computes interruption metrics), but none of that
signal reaches the possessed actor's own status line — a mechanic hidden from play is
unfinished (INV-066/INV-071; INV-070 in spirit).

## Assumption Reassessment (2026-06-11)

1. Verified against current code at `main` `5af8660`: `salient_interruption: None` is
   the sole producer site in `projections.rs::phase3a_status`; the render arm exists at
   `crates/tracewake-tui/src/render.rs` (`if let Some(interruption) = …`);
   `SleepInterrupted` and work-failure episodes are materialized into
   `state.rs::OrdinaryLifeEpisodeRecord` keyed by actor (with full `payload_fields`
   post-0018), so a viewer-keyed actor-known derivation source exists.
2. Verified against `specs/0019_…_HARDENING_SPEC.md` ORD-HARD-052 + §9 (reassessed
   2026-06-11): the 0019 audit found no phase-ladder deferral cite for embodied
   interruption surfacing in `docs/2-execution/03` or `12` — but per spec §9, this
   ticket's first implementation step is to re-check for such a cite; if one exists,
   downgrade to a recorded deferral (conformance-index note), do not implement, and
   record the downgrade in this ticket's Outcome.
3. Cross-artifact boundary under audit: the embodied status contract between the
   kernel-side projection (`projections.rs::phase3a_status`, deriving from sealed
   actor-known context plus the viewer's own state) and the TUI renderer
   (presentation-only consumer of the view model) — the TUI must not derive or invent
   the interruption signal itself (INV-069).
4. INV-066/INV-071 restated: every runnable phase's mechanics must be reachable,
   usable, and inspectable through the TUI or the same actor-filtered view models;
   mechanics hidden from play are not complete. INV-070: blocked/disrupted activity
   should be explained in actor-known terms.
5. Actor-knowledge surface touched (no-leak preserved): `salient_interruption` derives
   exclusively from the *viewer's own* interruption evidence — the viewer-keyed
   materialized episode records (`SleepInterrupted`, work-failure) that are already
   actor-known by construction (the actor experienced them). No other actor's state,
   no scheduler-internal metrics, and no debug data enter the derivation; the
   hidden-truth gates (`debug_truth_never_enters_holder_known_context_hash`) and
   possession-parity tests must remain green.

## Architecture Check

1. Deriving the signal in the kernel-side projection from already-materialized
   viewer-keyed episode records keeps the TUI presentation-only (INV-069) and reuses
   the evidence layer 0018 hardened (episode records now carry full payloads) instead
   of threading a parallel interruption channel from the scheduler. Salience policy
   (which interruption, how recent) lives in one place — the projection — so the
   embodied and any future debug rendering stay consistent.
2. No backwards-compatibility aliasing/shims: the dead `None` is replaced by the real
   derivation; no feature flag preserves the dead path.

## Verification Layers

1. INV-071 reachability -> fixture-backed test: interrupt the bound actor's sleep
   (reuse an existing interruption-exercising fixture if one exists, else add one —
   as surfaced); assert `phase3a_status.salient_interruption.is_some()` and that the
   embodied render emits the interruption line.
2. INV-024/INV-067 no-leak -> negative assertion: another actor's interruption does
   NOT surface in the viewer's status; hidden-truth gates remain green.
3. INV-018 determinism -> the derivation is a pure function of materialized records;
   golden suite green (view models are not checksummed, so no repricing expected).
4. Deferral honesty (spec §9) -> if a phase-ladder deferral cite is found at
   implementation start, the recorded-deferral downgrade path is taken and documented
   instead — Outcome records which branch fired.

## What to Change

### 1. Deferral re-check (first step)

Re-check `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
and `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` for an embodied
interruption-surfacing deferral. If found: record the deferral in the conformance
index, leave the code untouched, close this ticket as a documented downgrade.

### 2. Viewer-keyed derivation

`projections.rs::phase3a_status`: derive `salient_interruption` from the viewer's own
materialized interruption evidence (most-recent `SleepInterrupted` / work-failure
episode within a salience window — policy recorded in code comments per the
actor-known framing), replacing the hardwired `None`.

### 3. Locks

The fixture-backed positive test and the other-actor negative test per Verification
Layers 1–2; confirm the existing render arm displays the populated value (adjust
`render.rs` only if the arm's formatting needs the new payload shape).

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify — as surfaced, if the status type's fields need threading)
- `crates/tracewake-tui/src/render.rs` (modify — as surfaced; arm exists, change only if formatting requires)
- `crates/tracewake-content/src/fixtures/` (new fixture — only if no existing fixture interrupts the bound actor's sleep; implementation-discovered, parent directory verified)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify — only under the deferral-downgrade branch)

## Out of Scope

- Interruption *mechanics* (severe-need predicates, proration — unchanged; this ticket
  surfaces existing evidence).
- Debug-panel interruption views (debug already has scheduler metrics; non-diegetic
  surface unchanged).
- Why-not explanations for blocked actions (existing surface; not this signal).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core phase3a_status` (or the named new tests) — bound
   actor's interruption surfaces; other actors' interruptions do not.
2. `cargo test -p tracewake-core --test hidden_truth_gates` — no-leak gates green.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. `salient_interruption` derives only from the viewer's own actor-known interruption
   evidence; no cross-actor or debug-sourced signal.
2. No dead embodied surface remains: the render arm is reachable, or the deferral is
   recorded in the conformance index (exactly one of the two).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (unit/integration test) — positive
   surfacing + other-actor negative.
2. Render assertion (tui test or view-model test, as surfaced) — the interruption line
   renders for the populated status.

### Commands

1. `cargo test -p tracewake-core phase3a_status`
2. `cargo test --workspace`

## Outcome

Completed 2026-06-11.

The required deferral re-check found no embodied interruption-surfacing deferral in
`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` or
`docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`,
so the implementation branch fired rather than the documented-downgrade branch.

`crates/tracewake-core/src/projections.rs::phase3a_status` now derives
`salient_interruption` from the viewer actor's own materialized
`ordinary_life_episodes` evidence, limited to `sleep_interrupted` and
`work_block_failed` terminal records and selecting the latest by simulation tick with
event-id tie-break. The TUI remains presentation-only; it consumes the populated view
model field through the existing render branch.

Regression coverage:

1. `view_models_embodied_phase3a_salient_interruption_is_viewer_scoped` proves the
   bound actor sees the latest own interruption and not another actor's or older
   terminal evidence.
2. `renderer_prints_phase3a_salient_interruption` proves the embodied renderer emits
   the populated `Interruption:` line.

Verification:

1. `cargo test -p tracewake-core view_models_embodied_phase3a_salient_interruption_is_viewer_scoped`
2. `cargo test -p tracewake-tui renderer_prints_phase3a_salient_interruption`
3. `cargo test -p tracewake-core phase3a_status`
4. `cargo test -p tracewake-core --test hidden_truth_gates`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `cargo build --workspace --all-targets --locked`
8. `cargo test --workspace`
