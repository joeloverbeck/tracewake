# 0004PHA2AEPISUB-008: Knowledge-blocker why-not and truthful-accuse probe

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds `ReasonCode::KnowledgePreconditionNotMet`, a knowledge-basis validation helper, and a non-mutating `truthful_accuse_probe` to `tracewake-core`.
**Deps**: 0004PHA2AEPISUB-007

## Problem

Possession transfers no world knowledge: the current actor may not truthfully accuse from facts it did not acquire through a modeled channel (`INV-006`), and blocked actions must explain the missing precondition in actor-known terms (`INV-070`). Spec 0004 §10.6 requires a new `KnowledgePreconditionNotMet` reason (stable id `knowledge_precondition_not_met`), a validation helper that checks whether an actor holds a source-backed belief/observation supporting a claim, and a minimal `truthful_accuse_probe` (QueryOnly, non-mutating) to exercise why-not and knowledge filtering without committing speech, accusation, suspicion, or institutional effects.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/actions/report.rs` defines `ReasonCode` (`report.rs:10`) with physical/controller reasons and no `KnowledgePreconditionNotMet`; this ticket adds it. The probe joins `actions/defs/` and registers in `actions/defs/mod.rs` (created/extended by ticket 006).
2. The why-not embodied/debug wording split and the probe's non-mutating contract are fixed by Spec 0004 §10.6; the knowledge-basis check reads the holder's beliefs/observations from the projection (ticket 003) — in particular the missing-property belief from ticket 007.
3. Shared boundary under audit: `report.rs` `ReasonCode` is consumed by the rejection-event/debug-report surfaces and the TUI why-not renderer (ticket 011); `registry.rs`/`defs/mod.rs` are shared with ticket 006 (sequential via the dep chain). The new reason's stable id is fixed here.
4. Invariant motivating this ticket: `INV-006` (possession transfers no world knowledge), `INV-070` (why-not explanations are mandatory, actor-filtered, debug truth separated), and `INV-008` (UI assistance is not authority — a why-not may clarify but must not grant knowledge or reveal hidden truth in embodied mode).
5. Actor-knowledge / no-leak surface: the embodied why-not summary must state the blocker in actor-known terms ("Tomas knows the coins are missing, but has no actor-known source linking Mara") and must not leak ground truth; only the debug expansion may show "Mara took coin_stack_01". The probe is `QueryOnly`/non-mutating — it commits no event that changes world or epistemic state — so it adds no nondeterminism and no leakage; the knowledge-basis helper reads only the holder's own records via `KnowledgeContext`.

## Architecture Check

1. A reusable knowledge-basis helper (does actor X hold a source-backed belief/observation supporting proposition P?) centralizes the precondition that future speech/report/accuse actions will reuse, rather than re-deriving it per action. The probe exercises it without modeling accusation consequences, keeping Phase 2A scope bounded.
2. No backwards-compatibility shims: the reason code and probe are additive; existing reason codes and action defs are unchanged.

## Verification Layers

1. No knowledge from possession (`INV-006`) -> integration test: after Mara (human-controlled) takes the coin and the human switches to Tomas, `truthful_accuse_probe` against Mara rejects with `KnowledgePreconditionNotMet` because Tomas holds no source linking Mara.
2. Why-not actor-filtered, debug separated (`INV-070`/`INV-008`) -> manual review + test: the embodied why-not summary contains no culprit/ground-truth string; the debug expansion may name the true mover, clearly separated.
3. Non-mutating probe (`INV-008`) -> grep-proof + test: the probe is `QueryOnly`, emits no world/epistemic mutation event, and creates no suspicion/report/speech state.

## What to Change

### 1. KnowledgePreconditionNotMet reason

Add `ReasonCode::KnowledgePreconditionNotMet` (stable id `knowledge_precondition_not_met`) to `report.rs` and its label/serialization arms.

### 2. Knowledge-basis helper

Add a validation helper (in `epistemics/` and surfaced to the action pipeline) `actor_has_source_backed_support(ctx, actor, proposition) -> bool` reading only the holder's own beliefs/observations through a `KnowledgeContext`.

### 3. truthful_accuse_probe action

Add `crates/tracewake-core/src/actions/defs/accuseprobe.rs` as a `QueryOnly`/non-mutating action that runs the knowledge-basis check for an accusation claim and, when unmet, produces the `KnowledgePreconditionNotMet` rejection plus an actor-known why-not summary and an optional debug expansion. Register in `actions/defs/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/actions/report.rs` (modify — add `KnowledgePreconditionNotMet`)
- `crates/tracewake-core/src/actions/defs/accuseprobe.rs` (new)
- `crates/tracewake-core/src/actions/defs/mod.rs` (modify — register probe; shared with ticket 006, sequential)
- `crates/tracewake-core/src/epistemics/mod.rs` (modify — expose knowledge-basis helper; file created by 0004PHA2AEPISUB-001)

## Out of Scope

- Full accusation/speech/report/suspicion/institution consequences (later phases).
- TUI exposure and rendering of the probe/why-not (ticket 011).
- The missing-property belief itself (ticket 007).

## Acceptance Criteria

### Tests That Must Pass

1. With Tomas knowing only that the coins are missing, `truthful_accuse_probe` against Mara rejects with `knowledge_precondition_not_met`.
2. The embodied why-not summary contains no culprit/ground-truth substring; the debug expansion may.
3. The probe commits no world or epistemic mutation event (state checksum unchanged before/after).

### Invariants

1. The knowledge-basis helper reads only the holder's own records via a `KnowledgeContext`.
2. The probe is non-mutating; why-not clarifies but grants no knowledge (`INV-008`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/accuseprobe.rs` (unit tests) — reject-when-unsupported, non-mutation, embodied/debug wording split.
2. `crates/tracewake-core/tests/golden_scenarios.rs` (extend) — Tomas cannot truthfully accuse Mara after switch.

### Commands

1. `cargo test -p tracewake-core actions::defs::accuseprobe`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. `cargo build --workspace --all-targets --locked`
