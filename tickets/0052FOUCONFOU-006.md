# 0052FOUCONFOU-006: F4-04 — real declared-process causal transaction

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — a due declared process performs a process-specific causal transition (or is honestly demoted to a non-`WorldNoOp` diagnostic); `world_processes_applied` counts only committed process transactions
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-003

## Problem

Spec 0052 F4-04 (§1.1.4, §4.5): the default declaration (`process_loaded_world_tick`, `scheduler.rs:523` — first due next tick, cadence one, empty source-event IDs, `None` random provenance) builds a `DeclaredWorldProcessApplied` event that `events/apply.rs:196` classifies as `ApplyOutcome::WorldNoOp`, while the world-step loop increments `world_processes_applied` for `ApplyOutcome::Applied` **or** `ApplyOutcome::WorldNoOp` (`scheduler.rs:808`). The event is simultaneously treated as proof a process transaction occurred and as a no-op at application time — an observability marker with an authoritative-sounding name and counter, which does not satisfy the declared-causal-process burden of INV-088.

This ticket implements one honest model. **Recommended (and the implementer-recorded default, §10.5): a minimal real process transaction** — validated content declares a concrete process kind and causal inputs; a due invocation identifies a process kind + trigger witness and runs a process-specific transition emitting one or more meaningful events/effects with source/cadence/random ancestry through the **same atomic cutover** as actor work; only committed, successfully applied effects increment an applied-process count; the declaration is reconstructed on replay per 0052FOUCONFOU-005. The §4.5 alternative (retain a due-process observation as an explicit diagnostic/non-`WorldNoOp` marker, rename counters, never present it as application) is the recorded fallback only if a real bounded effect proves infeasible this pass. The chosen path is stated in the acceptance artifact (0052FOUCONFOU-013).

## Assumption Reassessment (2026-06-25)

1. `events/apply.rs:196` maps `EventKind::DeclaredWorldProcessApplied => Ok(ApplyOutcome::WorldNoOp)`; `scheduler.rs:808` increments `world_processes_applied` for `ApplyOutcome::Applied | ApplyOutcome::WorldNoOp` (matched at scheduler.rs:807). The synthetic declaration is `process_loaded_world_tick` (scheduler.rs:523). `EventKind::DeclaredWorldProcessApplied` is the `EventKind` variant at `events/envelope.rs:92`, with stream/label/codec arms at envelope.rs:154/275/313/387/448.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.5 and §10.5 (implementer-recorded choice). Closure-order step 5. The atomic world-step cutover this routes through is the command coordinator from 0052FOUCONFOU-003; replay reconstruction of the declaration ancestry is 0052FOUCONFOU-005.
3. Cross-artifact boundary under audit: the content-declared process kind/inputs → core process-transaction → event-application seam. Validated content declares the process; core runs the transition; `events/apply.rs` applies the effect. Content stays possibility (a declared process kind with parameters/inputs), not script — no authored outcome chain or director logic (INV-060, no-scripting).
4. Motivating invariant: INV-088 ("Regional processes are declared causal processes") — a world process requires declared source, cadence/trigger, inputs, random model, scope, delivery channel, traces, affected beliefs/records, local entry events, ancestry, and replay/debug visibility; supported by INV-001/009/010 (modeled causes, meaningful eventful transitions).
5. Fail-closed / deterministic-replay surface: the process transition runs through the same atomic cutover as actor work (all-or-nothing), and its random draws use seeded, auditable provenance (INV-017) so replay reconstructs the effect deterministically (INV-018); a `WorldNoOp` outcome must **not** increment the applied count. The enforcement surface is the world-step transaction and `events/apply.rs`.
6. Schema extension (event record — kept distinct from item 5): the chosen model touches the `EventKind` schema in `events/envelope.rs`. A real transition either (a) emits an existing meaningful event kind whose payload already carries effect data (additive — no `EventKind` change), or (b) introduces/retypes the process-effect event so `DeclaredWorldProcessApplied` carries process-specific effect + source/cadence/random ancestry (additive to the envelope enum and its stream/label/codec arms at envelope.rs:154/275/313/387/448 — every arm enumerated and updated, no existing kind removed). Consumers of `EventKind`: `events/{envelope,apply,log}.rs`, `scheduler.rs`, replay codecs. The diagnostic-demotion fallback instead reclassifies the marker out of `WorldNoOp` and renames the counter (no new effect event). Additive-vs-breaking is recorded with the chosen model in the acceptance artifact.

## Architecture Check

1. A minimal real process transaction is cleaner than the counted no-op marker because it makes no-human progression and replay continuation non-vacuous — a marker's presence, a counter of one, or an event name containing "Applied" is not evidence of a transition. A diagnostic marker may *accompany* a transition but cannot *be* it; counting only committed transactions removes the simultaneous "occurred / no-op" contradiction.
2. No backwards-compatibility alias: the `WorldNoOp`-counted-as-applied path is removed (or the marker is honestly reclassified and renamed), not retained beside a real transition. If the fallback is chosen, the counter is renamed so no caller reads "applied" for a non-application.

## Verification Layers

1. INV-088 (declared causal process) -> process fixture with pre-due / due / post-due assertions of process-specific state/projection/event effects, declaration/source/cadence/random ancestry, and exact replay continuation.
2. INV-018/INV-017 (deterministic, seeded) -> replay check: the effect replays from its ancestry; random provenance is recorded and reproducible.
3. Mutation -> kills for `WorldNoOp` application, removed concrete effect, altered cadence/source ancestry, and counting no-op as applied.
4. No-scripting (content is possibility) -> schema validation: the declared process kind/inputs are typed possibility (parameters/metadata), not selectors/branches/authored outcomes.

## What to Change

### 1. Real process transition (`scheduler.rs`, `events/apply.rs`)

A due invocation identifies a process kind + trigger witness and runs a process-specific transition emitting meaningful event(s)/effect(s) with source/cadence/random ancestry through the same atomic world-step cutover as actor work. Only committed, successfully applied effects increment `world_processes_applied`; a `WorldNoOp` outcome does not. (Fallback path: reclassify the marker out of `WorldNoOp` and rename the counter; do not present it as application.)

### 2. Event schema for the process effect (`events/envelope.rs`, `events/log.rs`)

If a new/retyped effect event is needed, extend the `EventKind` enum and update every stream/label/codec/log arm (additive-only; enumerate all arms). Carry process-specific effect + ancestry in the payload.

### 3. Declaration ancestry (`scheduler.rs`)

The declared process carries concrete source-event IDs and random provenance (no longer empty/`None`), reconstructed on replay by 0052FOUCONFOU-005.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — real process transition; count committed transactions only; declaration ancestry)
- `crates/tracewake-core/src/events/apply.rs` (modify — process effect application; no-op not counted as applied)
- `crates/tracewake-core/src/events/envelope.rs` (modify — `EventKind` / effect payload, additive; all arms enumerated)
- `crates/tracewake-core/src/events/log.rs` (modify — log/codec arm for the effect event)

## Out of Scope

- Replay-authority reconstruction machinery (005 — this ticket supplies the declaration ancestry it reconstructs).
- The actor disposition census (007).
- Content-pack authoring of additional process kinds beyond the minimal bounded witness (future scale work; INV-088 regional processes at larger LOD are not in this seam).
- The process-effect mutation/standing campaign packaging (010).

## Acceptance Criteria

### Tests That Must Pass

1. A process fixture asserts pre-due / due / post-due process-specific state/projection/event effects, declaration/source/cadence/random ancestry, and exact replay continuation.
2. A negative fixture proves external crates cannot construct due invocations or inject raw process events.
3. A `WorldNoOp` outcome does not increment the applied-process count; focused mutation kills `WorldNoOp` application, removed concrete effect, altered cadence/source ancestry, and counting no-op as applied.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. A due declared process performs a committed causal transition with source/cadence/random ancestry; only committed transactions increment `world_processes_applied` (INV-088, INV-001/009/010).
2. The process effect replays deterministically from recorded ancestry (INV-017, INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — declared-process effect before/at/after cadence + replay continuation; counter counts committed only.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` (corpus extended in 009) — external crates cannot construct due invocations / inject raw process events.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
