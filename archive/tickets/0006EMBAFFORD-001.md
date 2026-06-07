# 0006EMBAFFORD-001: Add a validate-only (no-commit) proposal preflight to the action pipeline

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`actions/pipeline.rs`: factor a pure decision step out of `run_pipeline`; add `validate_proposal`; likely re-export in `actions/mod.rs`). No event/schema/replay changes; committed-path events and reports are unchanged.
**Deps**: None. (Enables `0006EMBAFFORD-002`.)

## Problem

The embodied action menu needs to know, per candidate action, whether the proposal *would* be accepted or rejected and — when rejected — the actor-visible reason, derived from the **same** validators the submit path uses. Today the only way to evaluate a proposal is `run_pipeline`, which mutates `PhysicalState`, appends events to the `EventLog`, and (for `check_container`) mutates the epistemic projection. There is no validate-only path.

Without one, the menu builder is forced to re-derive a *subset* of preconditions in the view layer (`projections.rs` `semantic_actions`), which has already drifted from the validators and produced "offered-then-denied" bugs (`0006EMBAFFORD-002`; the sibling `0005EMBVIEWFIX-001`). That re-derivation is the "second rule engine" anti-pattern (`docs/1-architecture/10`, risk `R-08`, spirit of `INV-069`).

This ticket adds the single-authority preflight that `0006EMBAFFORD-002` consumes. It changes no committed behavior.

## Assumption Reassessment (2026-06-07)

1. `crates/tracewake-core/src/actions/pipeline.rs:90` `run_pipeline(context: &mut PipelineContext<'_>, proposal: &Proposal) -> PipelineResult` interleaves validation with commit. The per-effect builders (`build_move_event`, `build_open_close_event`, `build_take_place_event`, `build_wait_event`, `build_check_container_event`) validate against `&PhysicalState` and return `Result<EventEnvelope, ActionRejection>` — e.g. `actions/defs/checkcontainer.rs:35-53` returns `ContainerLocked` / `ContainerClosed`; `actions/defs/movement.rs:95-116` returns `DoorClosedBlocksMovement` / `DoorLocked`. Validation is already a pure function of read-only state. Commit happens only afterward: a clone-based dry-run apply at `pipeline.rs:223`, then `context.log.append(...)` + `apply_event(context.state, ...)` at `pipeline.rs:236-260`.
2. `reject()` (`pipeline.rs:520-563`) builds the `ValidationReport` **and** constructs an `ActionRejected` `EventEnvelope` (and appends it). So report-construction and the rejection event are currently fused. A preflight must produce the `ValidationReport` **without** that append. This is the exact seam to factor.
3. `PipelineContext` (`pipeline.rs:74`) holds `registry`, `&mut state`, `&mut log`, `controller_bindings`, `&mut epistemic_projection`, `content_manifest_id`, `ordering_key`. A preflight needs read-only `registry` + `state` (+ optional read-only `epistemic_projection` for the query path `truthful_accuse_probe` at `pipeline.rs:158-166`) and must take neither `&mut state` nor `&mut log` nor `&mut epistemic_projection`.
4. Invariant under audit: `INV-069` (clients/menus must not implement a second precondition rule engine — preconditions have one authority) and risk `R-08` (action-pipeline bypass / duplicated rules). This ticket creates the single-authority surface; it does **not** touch actor-knowledge filtering (`INV-067`/`INV-093`) or deterministic replay (`INV-018`) because the committed path is unchanged.
5. `ActionEffect::CheckContainer` records an observation event plus absence/contradiction events and mutates the epistemic projection on the committed path (`pipeline.rs:261-...`). The preflight must compute **only** the accept/reject decision (the physical precondition rejection inside `build_check_container_event`, `checkcontainer.rs:25-54`) and must not record observations or mutate the projection.
6. Adjacent classification: the duplicated precondition subset in `semantic_actions()` (`projections.rs:281-318`: lock/portable only) is the root cause of the check/move "offered-then-denied" bugs. Replacing it is `0006EMBAFFORD-002` (depends on this ticket). No change to the committed pipeline path (events, reports, ordering) is in scope here.

## Architecture Check

1. Extract the decision `run_pipeline` already performs — "validate the proposal and, if acceptable, produce the candidate event(s)" — into a pure function over `&ActionRegistry` + `&PhysicalState` (+ optional `&EpistemicProjection`) returning either the would-commit events or a `Rejected` `ValidationReport`. `run_pipeline` keeps sole ownership of committing (log append, `apply_event`, observation/absence recording, projection mutation); `validate_proposal` calls the same decision and discards the events. This makes the preflight **provably identical** to submit-path validation (one authority) rather than a parallel reimplementation. Alternatives — re-deriving preconditions in the view layer, or a separate per-action "is-enabled" predicate — are exactly the second-rule-engine anti-pattern this work exists to remove.
2. No backwards-compatibility aliasing/shims. `run_pipeline`'s external signature and its committed events/reports are unchanged; the decision step is a private extraction and `validate_proposal` is a new additive entry point.

## Verification Layers

1. `INV-069` / `R-08` (single precondition authority) -> codebase grep-proof + unit test: `validate_proposal` and the committed path share one decision function; a test asserts `validate_proposal`'s report equals `run_pipeline`'s report (`status`, `reason_codes`, `actor_visible_summary`) for the same proposal+state.
2. No-commit guarantee -> unit test: `validate_proposal` leaves `EventLog` length, `PhysicalState`, and `EpistemicProjection` byte-identical for both rejected and would-accept proposals (including a `check_container` proposal, whose committed path would otherwise record observations).
3. Committed-path unchanged (`INV-018` replay) -> replay/golden-fixture check: existing pipeline + golden-fixture tests (`door_access_001`, `strongbox_001`) pass with identical committed events.

## What to Change

### 1. Factor a pure decision step out of `run_pipeline` (`crates/tracewake-core/src/actions/pipeline.rs`)

Introduce a private decision function — e.g. `decide_proposal(registry: &ActionRegistry, state: &PhysicalState, controller_bindings: Option<&ControllerBindings>, epistemic_projection: Option<&EpistemicProjection>, content_manifest_id: &ContentManifestId, ordering_key: &OrderingKey, proposal: &Proposal) -> PipelineDecision` (or `Result<Vec<EventEnvelope>, ValidationReport>`).

- Move the controller-binding check, registry lookup, actor lookup/enabled check, `phase1_implemented` check, the per-effect `build_*_event` calls, and the clone-based dry-run apply into it.
- On rejection it must build the `Rejected` `ValidationReport` (the report half of today's `reject()`) **without** constructing or appending the `ActionRejected` event.
- Reused by both the committed path and the preflight, so the rejection report text is computed in exactly one place.

### 2. Re-wire `run_pipeline` to commit on top of the decision

`run_pipeline` calls `decide_proposal`, then performs only the committing side effects it does today: append success events / append the `ActionRejected` event for meaningful rejections / `apply_event` to real state / record observation+absence events and mutate the projection for `CheckContainer`. Committed events, ordering, and the returned `PipelineResult` are identical to current behavior.

### 3. Add the validate-only entry point

Add `pub fn validate_proposal(...)` (read-only inputs mirroring `decide_proposal`) that runs `decide_proposal` and returns the `ValidationReport` — `Accepted` when events would be produced, otherwise the `Rejected` report. It appends nothing and mutates nothing. A fixed/stub preflight `OrderingKey` is acceptable since any candidate events are discarded; note this in code.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/actions/mod.rs` (modify — re-export `validate_proposal`; implementer confirms the current re-export layout)

## Out of Scope

- Any change to the embodied menu / `semantic_actions` / view models — that is `0006EMBAFFORD-002`.
- Any change to committed events, schemas, deterministic replay, or the `ActionRejected` event policy.
- New action definitions or any change to existing preconditions.

## Acceptance Criteria

### Tests That Must Pass

1. New `tracewake-core` unit test: for a closed-and-opaque container and a `check_container` proposal, `validate_proposal` returns `status == Rejected` with `reason_codes == [ContainerClosed]` and `actor_visible_summary == "The container is closed."`, and the result equals the `report` field of `run_pipeline` on the same state (parity assertion).
2. New `tracewake-core` unit test: for a closed blocking door and a `move` proposal, `validate_proposal` returns `Rejected` with `[DoorClosedBlocksMovement]` / `"The door is closed."`, matching `run_pipeline`'s report.
3. New `tracewake-core` no-commit test: calling `validate_proposal` for both a would-accept and a rejected proposal (including a `check_container` accept) leaves `EventLog` length, `PhysicalState`, and `EpistemicProjection` unchanged.
4. Full pipeline: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The committed path and the preflight derive accept/reject from one shared decision function — preconditions have a single authority.
2. `validate_proposal` performs no append and no mutation of `state`, `log`, or `epistemic_projection`.
3. Committed events, ordering, and reports for the existing golden fixtures are byte-for-byte unchanged.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/pipeline.rs` (tests module) — preflight↔commit report parity for `ContainerClosed` and `DoorClosedBlocksMovement`; no-commit invariance for accept and reject (including `check_container`).
2. Existing `pipeline.rs` tests (e.g. `same_proposal_validates_same_for_human_and_scheduler_origin`) and golden-fixture replay tests — unchanged and still green, proving the committed path was a pure extraction.

### Commands

1. `cargo test -p tracewake-core actions::pipeline`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-07

What changed:
- Added `validate_proposal` as a read-only action-pipeline preflight exported from `tracewake-core::actions`.
- Factored proposal accept/reject decision construction so `run_pipeline` and `validate_proposal` share the same validator path.
- Kept event appends, physical state mutation, and epistemic projection mutation in the committed `run_pipeline` path only.
- Added pipeline tests for closed-container and closed-door preflight/commit report parity, plus no-commit invariance for accepted and rejected preflights including `check_container`.

Deviations from original plan:
- The extracted decision returns candidate events for accepted mutating actions and boxes rejected reports to satisfy clippy's enum-size lint.

Verification:
- `cargo test -p tracewake-core actions::pipeline`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
