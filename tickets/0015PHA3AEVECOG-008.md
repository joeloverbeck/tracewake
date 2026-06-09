# 0015PHA3AEVECOG-008: Author ordinary-life tuning constants in content

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — content schema gains a need-model config block + SleepAffordance duration/recovery fields; core consumes authored values; kernel tuning constants removed; guard + golden-fixture updates
**Deps**: 0015PHA3AEVECOG-003, 0015PHA3AEVECOG-007

## Problem

ORD-HARD-012: the kernel hardcodes ordinary-life tuning facts the content schema already authors elsewhere. `AWAKE_HUNGER_DELTA_PER_TICK = 5` and `AWAKE_FATIGUE_DELTA_PER_TICK = 3` live in `crates/tracewake-core/src/time.rs`; `DEFAULT_SLEEP_DURATION_TICKS = 4`, `FATIGUE_RECOVERY_PER_SLEEP_TICK = 20`, `HUNGER_RISE_PER_SLEEP_TICK = 2` live in `actions/defs/sleep.rs`. Meanwhile `WorkplaceSchema` already authors `fatigue_delta_per_tick`, `hunger_delta_per_tick`, `work_duration_ticks`, and `FoodSupplySchema` authors `hunger_reduction_per_serving` (all in `crates/tracewake-content/src/schema.rs`) — the same fact class, authored for work/food but hardcoded for sleep/idle. The asymmetry proves the authoring path exists and the kernel constants are a shortcut (INV-061: designers author needs/routines/affordances; INV-080: domain packs own flavor; architecture doc 01: content packs define initial conditions).

This ticket authors passive need rates and sleep duration/recovery in content and has core consume the seeded values, with no silent defaults. Per spec §9 it lands **after** ORD-HARD-008/011 so the authored values land in the schema the continuity/perception work already touches.

## Assumption Reassessment (2026-06-09)

1. Current code (verified): kernel constants `AWAKE_HUNGER_DELTA_PER_TICK`/`AWAKE_FATIGUE_DELTA_PER_TICK` (`time.rs`), `DEFAULT_SLEEP_DURATION_TICKS`/`FATIGUE_RECOVERY_PER_SLEEP_TICK`/`HUNGER_RISE_PER_SLEEP_TICK` (`actions/defs/sleep.rs`). Authoring exemplars: `WorkplaceSchema` (`schema.rs:350`) with `work_duration_ticks:354`/`fatigue_delta_per_tick:355`/`hunger_delta_per_tick:356`; `FoodSupplySchema` (`schema.rs:342`) with `hunger_reduction_per_serving:346`; seeded into state at `schema.rs:605–607`. `SleepAffordanceState` is in `state.rs` (`access_open:353/361`); the content seed path that builds it is around `schema.rs:616`. Need-band thresholds live in `crates/tracewake-core/src/agent/need.rs`.
2. Specs/docs: spec 0015 §ORD-HARD-012 (required correction + structural lock) and §9 ordering note; INV-061, INV-080; `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (the conformance index where the kernel/content boundary decision is recorded); `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md:66` ("Content packs define possible objects, places, routines, … and initial conditions").
3. Shared boundary under audit: the kernel/content authority boundary for tuning facts. Need rates and sleep duration/recovery move to content (authored flavor); need-**band thresholds** in `need.rs` stay kernel semantics (they define the pressure vocabulary, not domain flavor) — this boundary decision is recorded in the conformance index (the docs surface is owned by `0015PHA3AEVECOG-010`).
4. INV-061 — designers author needs/routines/affordances; passive need rates and sleep recovery are authored causal machinery, not kernel constants. INV-080 — domain packs own flavor; the rates are flavor. INV-045 — survival must be causal; authored values keep the meters connected to authored world state.
5. Fail-closed validation surface: names the enforcement surface = content validation of the new required fields. Per the no-shims convention, the new schema fields are **required** and validation rejects their absence (no silent defaults). Confirm validation is deterministic and blocking, distinguishes nothing as a warning here (absence is a hard reject), and rejects unknown fields by default. The values are numeric parameters (possibility space), not behavior-looking fields — no selectors/branches/triggers (no-scripting preserved).
6. Schema extension: adds a need-model config block to the fixture/domain schema and `duration`/`recovery` fields on the sleep-affordance schema (mirroring `WorkplaceSchema`). Consumers: the content seed/serialization path (`schema.rs`, `serialization.rs`), core consumption in `time.rs` (passive decay) and `actions/defs/sleep.rs` (duration/recovery), and golden fixtures. **Breaking** (required new fields) — every fixture and consumer updates in lockstep; golden checksums change with explicit values matching current behavior so replay stays explainable.
7. Removal blast radius: the five kernel constants are removed from `time.rs`/`actions/defs/sleep.rs`; grep confirms their only readers are the passive-decay and sleep-completion paths being rewired to read seeded state. A guard bans integer need-delta/duration constants in `time.rs` and `actions/defs/` (allowlist: identity/zero values).

## Architecture Check

1. Authoring the rates in content removes a kernel/content asymmetry and makes ordinary-life tuning a single authored surface, consistent with how work/food already author the identical fact class — cleaner than maintaining two divergent sources of the same fact. Keeping need-band thresholds in the kernel draws the boundary at vocabulary-vs-flavor, recorded explicitly so it is not re-litigated.
2. No shims / no silent defaults: required schema fields with fail-closed validation; the kernel constants are deleted, not retained as fallback defaults.

## Verification Layers

1. INV-061/080 → schema validation: the new need-model + sleep duration/recovery fields are required; content missing them is rejected.
2. INV-045 → replay/golden-fixture check: golden fixtures carry explicit values matching current behavior; replay checksums change explainably and stay stable.
3. kernel/content boundary → codebase grep-proof (guard): no integer need-delta/duration constant remains in `time.rs`/`actions/defs/` (allowlist identity/zero); need-band thresholds remain in `need.rs`.
4. INV-080 → invariants alignment check: the boundary decision (rates authored, bands kernel) is recorded in the conformance index (docs owned by `-010`).

## What to Change

### 1. Content schema: need-model + sleep affordance tuning

Add a need-model config block (passive hunger/fatigue per-tick rates) to the fixture/domain schema, and `duration`/`recovery` fields on the sleep-affordance schema (mirroring `WorkplaceSchema`). Required fields; validation rejects absence.

### 2. Core consumes authored values

`time.rs` passive decay and `actions/defs/sleep.rs` duration/recovery read the seeded values from state instead of the kernel constants. Remove `AWAKE_HUNGER_DELTA_PER_TICK`, `AWAKE_FATIGUE_DELTA_PER_TICK`, `DEFAULT_SLEEP_DURATION_TICKS`, `FATIGUE_RECOVERY_PER_SLEEP_TICK`, `HUNGER_RISE_PER_SLEEP_TICK`.

### 3. Guard + golden fixtures

Guard banning integer need-delta/duration constants in `time.rs`/`actions/defs/`. Update golden fixtures with explicit values matching current behavior.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify — new schema fields + seed plumbing)
- `crates/tracewake-content/src/serialization.rs` (modify — serialize new fields)
- `crates/tracewake-core/src/state.rs` (modify — carry seeded need-model + sleep tuning on state)
- `crates/tracewake-core/src/time.rs` (modify — consume seeded passive rates; remove constants)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify — consume seeded duration/recovery; remove constants)
- `crates/tracewake-core/src/agent/need.rs` (modify — only if the band/threshold boundary needs a comment marker; thresholds stay)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: also 004/005/006/007; 009 rewrites)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — explicit values + checksums)

## Out of Scope

- Recording the boundary decision in the conformance index doc (`0015PHA3AEVECOG-010` owns the docs surface; this ticket provides the decision).
- Need-band threshold semantics (remain kernel; unchanged).
- The continuity/interruption mechanism (`0015PHA3AEVECOG-007`, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. Content missing a required need-model or sleep duration/recovery field is rejected by validation.
2. Golden fixtures with explicit authored values reproduce current behavior; replay checksums stable and explained.
3. No integer need-delta/duration constant remains in `time.rs`/`actions/defs/` (guard); re-adding one fails `cargo test`.
4. `cargo test --workspace` green.

### Invariants

1. Passive need rates and sleep duration/recovery are authored in content and consumed from seeded state; no kernel default.
2. Need-band thresholds remain kernel semantics (the boundary is explicit).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/schema.rs` (or the content validation tests) — required-field rejection cases.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — tuning-constant ban guard.
3. Golden fixtures — explicit-value updates.

### Commands

1. `cargo test -p tracewake-content schema:: validation:: && cargo test -p tracewake-core time:: actions::defs::sleep`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
