# 0008PHA3AANTCON-003: Pipeline read-context exposes `AgentState`; work validator reads authoritative state

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core`: `PipelineReadContext` gains read-only `AgentState`; `work.rs` validates need constraints from authoritative state; proposal live-state echo parameters forbidden
**Deps**: None

## Problem

Spec 0008 Finding 5 (D-0008-05): `actions/defs/work.rs` validates fatigue/hunger via `need_param(proposal, "current_fatigue")` (`work.rs:62`) and `need_param(proposal, "current_hunger")` (`:72`), reading from `proposal.parameters`; `need_param` (`work.rs:337`) defaults missing/malformed values. The scheduler copies live need values into the work proposal (e.g. `current_fatigue` inserted at ~`work.rs:518` in tests). Proposal parameters are caller-provided strings — they can be stale, forged, missing, or produced by a path that never read the current `AgentState`. This violates pipeline authority: validators must read authoritative current need values from the pipeline context.

`PipelineReadContext` (`actions/pipeline.rs:108`) currently exposes `registry`, `state: &PhysicalState`, `controller_bindings`, `epistemic_projection`, `content_manifest_id`, `ordering_key` — but **no `AgentState`**. This ticket adds read-only `AgentState` to the context and makes work validation authoritative. eat/sleep validators were audited and read only target IDs (`sleep_place_id`), not live-need echoes, so they need no change (confirmed `sleep.rs:40`).

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-core/src/actions/defs/work.rs:62,72` read `need_param` from `proposal.parameters`; `need_param` at `:337`; `PipelineReadContext` at `actions/pipeline.rs:108` lacks `AgentState`; `validate_proposal`/`run_pipeline` construct the context at `pipeline.rs:126,145`.
2. Spec §8.4 + §9.4 (D-0008-05): "expose read-only `AgentState` through the validator/read context"; "Proposal parameters for live state echoes such as `current_hunger` and `current_fatigue` are forbidden"; "No action validator may default missing critical state to safe success." §10.4 supplies the stale/forged/missing/malformed-param gates this ticket carries.
3. Cross-artifact boundary under audit: the **pipeline read context ↔ action validator** contract — what authoritative state validators may read (`PipelineReadContext`) vs. what proposals may carry (actor choices / target IDs only, §8.4.4).
4. INV-043 (Action validation is ordinary-agent validation): the pipeline validates attempts against actor state — that state must be authoritative, not caller-echoed. INV-008 (UI assistance is not authority): a caller-supplied parameter must not bypass a precondition.
5. Enforcement surface: fail-closed action validation. Confirm: missing/malformed authoritative need state is a validation **failure**, never a default-to-zero pass; the read-only `AgentState` borrow introduces no mutation path and no nondeterminism (read-only reference threaded through the existing context). No epistemic leakage — `AgentState` is the actor's own authoritative state used for the actor's own action validation.
6. Extends the `PipelineReadContext` schema: **additive** new field `agent_state: &AgentState` (or `Option<&AgentState>` if some call sites lack it — prefer required to keep fail-closed). Consumers/constructors: `pipeline.rs:126` (`validate_proposal`), `pipeline.rs:145` (`run_pipeline` via `PipelineContext`), and `decide_proposal` (`pipeline.rs:435`) plus the per-action validators it dispatches to. All construction sites must supply the new field.
7. Forbids the `current_hunger`/`current_fatigue` proposal parameters (a contract field on the work proposal). Blast radius grep at implementation: `grep -rn "current_hunger\|current_fatigue" crates/` — producer sites (scheduler proposal construction → rewired in -006; test fixtures inserting them) and the `need_param` reader. Reader removed here; producers in scheduler owned by -006; test echoes converted into the §10.4 negative gates.

## Architecture Check

1. Reading live need state from the pipeline context is the only authoritative design: the validator sees what the world holds now, not what a caller claimed. Proposals keep carrying actor choices (target food source, worksite ID, duration) — legitimate inputs — while live-state echoes (a side channel that can forge state) are removed.
2. No backwards-compatibility aliasing: `need_param` for live-state keys is removed, not deprecated-but-honored. No fallback to proposal parameters remains.

## Verification Layers

1. INV-043 ordinary-agent validation → unit test: work validation reads fatigue/hunger from `PipelineReadContext.agent_state`, not from `proposal.parameters`.
2. Fail-closed validation → unit test (§10.4 gate 2): a proposal omitting need params with authoritative fatigue above max fails work (no default-to-zero pass).
3. Forge resistance → unit test (§10.4 gate 1): proposal claims `current_fatigue=0` but authoritative fatigue exceeds the work maximum → work fails.
4. Staleness → unit test (§10.4 gate 4): a proposal built before a need delta is rejected when current authoritative state now violates thresholds.

## What to Change

### 1. Add read-only `AgentState` to `PipelineReadContext`

Thread `agent_state: &AgentState` into `PipelineReadContext` (`pipeline.rs:108`) and every construction site (`pipeline.rs:126,145`; `PipelineContext`). Plumb `AgentState` from the caller (the transaction/scheduler supplies it; here, update signatures + existing callers/tests).

### 2. Authoritative work validation

`work.rs` reads `current_fatigue`/`current_hunger` from `context.agent_state` against `workplace.max_fatigue_to_start`/`max_hunger_to_start`. Remove `need_param` for live-state keys; missing authoritative state → validation failure.

### 3. Forbid live-state echo parameters

Drop `current_hunger`/`current_fatigue` from accepted work-proposal parameters. Proposals retain actor-choice/target-ID parameters only.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify — `PipelineReadContext` + construction sites)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify — authoritative need reads; remove `need_param` live-state path)
- `crates/tracewake-core/src/actions/proposal.rs` (modify — if accepted-parameter contract is enforced here)

## Out of Scope

- Scheduler/transaction no longer copying need values into proposals (0008PHA3AANTCON-006 removes the producer sites).
- eat/sleep validators (audited: target-ID only, no live-need echo — no change).
- Typed diagnostics for work failures (0008PHA3AANTCON-002 stores them; -010 renders them).

## Acceptance Criteria

### Tests That Must Pass

1. Forged-param gate: `current_fatigue=0` in parameters with authoritative fatigue > max → work validation fails (§10.4 gate 1).
2. Missing/malformed-param gate: omitted or malformed need params do not default to a safe pass (§10.4 gates 2–3).
3. Stale-proposal gate: proposal predating a need delta fails when current state violates thresholds (§10.4 gate 4).
4. `cargo test --workspace` green.

### Invariants

1. Work need constraints are validated only from `PipelineReadContext.agent_state`.
2. No work-proposal parameter carries a live need value.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/work.rs` tests — forged/missing/malformed/stale need-param gates against authoritative state.
2. `crates/tracewake-core/src/actions/pipeline.rs` tests — context exposes `agent_state` to validators.

### Commands

1. `cargo test -p tracewake-core actions::defs::work`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- Added required read-only `AgentState` authority to proposal validation via `ProposalValidationContext` and the internal `PipelineReadContext`.
- Threaded authoritative agent state through committed pipeline validation and semantic-action preflight validation.
- Changed work-start validation to read fatigue and hunger from `AgentState` and fail closed with `WorkBlockFailed` when authoritative needs are missing or above workplace thresholds.
- Removed the work validator's `need_param` fallback path for `current_fatigue` and `current_hunger`.
- Added work tests for forged, missing/malformed, and stale proposal need parameters against current authoritative state.

Deviations from original plan:
- `validate_proposal` was converted to take `ProposalValidationContext` rather than adding another positional argument, because the required `AgentState` made the old signature exceed the workspace clippy argument-count gate.
- Scheduler live-need echo producer removal remains deferred to `0008PHA3AANTCON-006`, per this ticket's out-of-scope section.

Verification:
- `cargo test -p tracewake-core actions::defs::work`
- `cargo test -p tracewake-core actions::pipeline`
- `cargo test -p tracewake-core projections`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
