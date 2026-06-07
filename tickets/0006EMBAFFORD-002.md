# 0006EMBAFFORD-002: Derive embodied menu availability from the validator preflight; remove duplicated precondition logic

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections.rs` `semantic_actions` / `build_embodied_view_model` signature; consumes `validate_proposal`), `tracewake-tui` (`app.rs` render path passes the registry; `tests/embodied_flow.rs`). No event/schema/replay changes.
**Deps**: `0006EMBAFFORD-001` (the `validate_proposal` preflight).

## Problem

The embodied TUI menu presents actions as available and then denies them on submit. Concrete repro (`cargo run -p tracewake-tui`, bound `actor_tomas`, `strongbox_001`):

- The menu lists `1. Check strongbox_tomas [check.container.strongbox_tomas]` while the view shows `strongbox_tomas open=false`. Selecting it yields `Why-not: The container is closed.`
- After `close door_house_street`, the menu lists `Move to street_lane [move.to.street_lane]` while the view shows `door_house_street open=false`. Selecting it yields `Why-not: The door is closed.`

The blocking state (`open=false`) is printed in the very same embodied view the menu is built from, so the actor plainly knows it. Offering these as available is the "menu decoration" `INV-044` forbids, and misuses `INV-070` (why-not is for genuinely blocked *attempts*, not for actions that should never have been offered). This is the doctrine the accepted `0005EMBVIEWFIX-001` established for the sibling phantom-take bug.

Root cause: `semantic_actions()` (`projections.rs:252`) hard-codes `enabled: true` for `check.container` (`:303`) and `move.to` (`:276`), while the authoritative validators reject those same proposals (`actions/defs/checkcontainer.rs:45-53` `ContainerClosed`; `actions/defs/movement.rs:95-106` `DoorClosedBlocksMovement`). The menu re-implements only a *subset* of preconditions (lock for open/close at `:288`/`:316`; portable for take at `:336`) and that subset omitted open-state. The same gap also mis-marks `check.container` on a **locked** container as available (`checkcontainer.rs:35-43` rejects `ContainerLocked`, but the menu never lock-gates `check`) — a third instance beyond the two in the repro.

Fix: derive each menu entry's `enabled` / `why_disabled` from `0006EMBAFFORD-001`'s `validate_proposal`, and delete the ad-hoc precondition branches. One precondition authority; the menu's disabled reason becomes literally the validator's `actor_visible_summary`, so it cannot drift again.

## Assumption Reassessment (2026-06-07)

1. `crates/tracewake-core/src/projections.rs:252-361` `semantic_actions()` sets `enabled: true` unconditionally for `wait`, `move.to` (`:276`), `check.container` (`:303`), `inspect.item`, and `place.item`; evaluates lock only for `open`/`close` door (`:288`) and container (`:316`); evaluates `portable` for `take` (`:336`). The blocking states are **not** re-derivable from the view model alone: `VisibleDoor` (`view_models.rs`) carries `is_open`/`is_locked` but not `blocks_movement_when_closed`; `VisibleContainer` carries `is_open`/`is_locked` but not `contents_visible_when_closed` (the field `checkcontainer.rs:45` reads). Re-deriving locally would mean threading more authoritative fields into the view model — reinforcing that deriving from the validator (which already reads `PhysicalState`) is the correct design.
2. `build_embodied_view_model` (`projections.rs:21`) currently takes `(&PhysicalState, &ActorId, SimTick, Option<&ValidationReport>)`. It has `state` but **not** the `&ActionRegistry` that `validate_proposal` needs for action-definition/effect lookup. The signature must gain `&ActionRegistry` (plus the `ContentManifestId` / `OrderingKey` inputs `validate_proposal` takes, or a fixed preflight ordering key). Callers to update: the TUI render path (`crates/tracewake-tui/src/app.rs`, the `render_current_view` flow) and any other `build_embodied_view_model` caller. The `EmbodiedViewModel` struct literals in `render.rs`/`input.rs` tests construct the view model directly and are unaffected by the function signature.
3. Semantic-action → `Proposal` resolution: the submit path (`crates/tracewake-tui/src/input.rs` / `app.rs` `submit_semantic_action`) already maps a `SemanticActionId` to a `Proposal`. To guarantee `why_disabled` text == the submit-time why-not text, the preflight must validate the **same** proposal. Implementer must locate that resolver and reuse it (or lift its shared core into `tracewake-core` so `build_embodied_view_model` can build each candidate proposal identically); confirm whether resolution currently lives in `tui` or `core` before implementing.
4. Invariants under audit (restate before trusting the narrative): `INV-044` affordances are conditional, not menu decoration; `INV-070` why-not is for genuinely blocked attempts; `INV-069` no second precondition rule engine; risks `R-08` (pipeline bypass / duplicated rules) and `R-15` (TUI playability). The change is view-model-only over already-actor-filtered physical state — it reads no hidden truth, so there is no `INV-067`/`INV-093` epistemic-leakage surface and no deterministic-replay impact.
5. The renderer already appends ` disabled: <reason>` (`crates/tracewake-tui/src/render.rs:82-95`) and is unchanged by this ticket; only the `enabled`/`why_disabled` data feeding it changes. The submit path is unchanged — a disabled entry remains selectable and still yields the identical why-not, now matching the inline reason.
6. `crates/tracewake-tui/tests/embodied_flow.rs:24-35` submits `move.to.back_room` through a closed door and asserts `Rejected` + `Why-not:`. That asserts post-submit behavior and stays valid. This ticket **adds** assertions that the pre-submit menu marks `move.to.back_room` as `disabled` with the door-closed reason. Per the ticket-execution contract, no test is adapted to preserve the bug.
7. Adjacent classification: `check.container` on a *locked* container (not in the repro) is the same root-cause defect and is fixed for free by deriving from `validate_proposal` (which surfaces `ContainerLocked`); it is a required consequence of this change, not a separate ticket. Latent siblings (take from a just-closed container, `place` preconditions) are likewise covered automatically once availability flows from the validator.

## Architecture Check

1. Deriving `enabled`/`why_disabled` from `validate_proposal` collapses two precondition implementations into one authority: the menu's disabled reason **is** the validator's `actor_visible_summary`, so they cannot drift — the failure mode behind both `0005EMBVIEWFIX` and this ticket. The ad-hoc lock/portable/open branches in `semantic_actions()` are deleted, not extended. The alternative (thread `blocks_movement_when_closed` / `contents_visible_when_closed` into the view model and re-evaluate locally) keeps a parallel rule engine that must be re-audited on every precondition change.
2. No backwards-compatibility aliasing/shims. `build_embodied_view_model` gains the `&ActionRegistry` parameter directly with all callers updated in the same diff; no overload or alias is retained.

## Verification Layers

1. `INV-044` (affordances conditional, not menu decoration) -> replay/golden-fixture + unit test: with `strongbox_tomas` closed, the `check.container` entry is `enabled == false` with `why_disabled == "The container is closed."`; with `door_house_street` closed, the `move.to.street_lane` entry is `enabled == false` with the door-closed reason.
2. `INV-070` (why-not honesty) -> unit test: a disabled entry's `why_disabled` string equals the `ValidationReport.actor_visible_summary` the submit path returns for the same proposal (one source of text).
3. `INV-069` / `R-08` (single authority) -> codebase grep-proof: `semantic_actions()` no longer contains hand-written precondition branches (`is_locked` / `portable` / open-state); availability flows from `validate_proposal`.
4. `INV-067`/`INV-093` (no leakage) -> invariants alignment check: inputs are already-actor-filtered physical state; no hidden field is newly read into the embodied view.

## What to Change

### 1. Give the view-model builder the preflight inputs (`crates/tracewake-core/src/projections.rs`)

Extend `build_embodied_view_model` to accept `&ActionRegistry` (and the `ContentManifestId` / `OrderingKey` that `validate_proposal` requires, or a fixed preflight ordering key). Thread these to `semantic_actions`.

### 2. Drive `enabled` / `why_disabled` from `validate_proposal` (`crates/tracewake-core/src/projections.rs`)

For each candidate action, build the corresponding `Proposal` (reuse the existing semantic→proposal resolver per Reassessment item 3) and call `validate_proposal`. Set `enabled = report.status == Accepted`; set `why_disabled = (report.status == Rejected).then(|| report.actor_visible_summary)`. Delete the ad-hoc `is_locked` / `portable` / open-state branches in `semantic_actions()`.

### 3. Pass the registry from the TUI render path (`crates/tracewake-tui/src/app.rs`)

Update the `render_current_view` flow (and any other `build_embodied_view_model` call site) to pass the app's `ActionRegistry` into the builder.

### 4. Tests (`crates/tracewake-core` + `crates/tracewake-tui/tests/embodied_flow.rs`)

Add core view-model tests for: closed container → `check.container` disabled with the closed reason; closed blocking door → `move.to` disabled with the door reason; locked container → `check.container` disabled with the locked reason. Extend `embodied_flow.rs` to assert the pre-submit menu marks `move.to.back_room` disabled before the existing submit/why-not assertions.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/actions/` (modify — only if the semantic→proposal resolver must be lifted into core; implementer confirms in reassessment)
- `crates/tracewake-tui/src/app.rs` (modify — pass `&ActionRegistry` into `build_embodied_view_model`)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — add pre-submit disabled assertions)

## Out of Scope

- The validate-only preflight itself (`0006EMBAFFORD-001`).
- Renderer formatting changes — `render.rs` already prints ` disabled: <reason>` inline.
- Auto-opening doors on `move` — doctrinal that a closed blocking door blocks movement (`docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md`, required failure case "door closed blocks path if configured").
- Any change to submit/why-not behavior or the `ActionRejected` event policy.

## Acceptance Criteria

### Tests That Must Pass

1. New `tracewake-core` view-model test on `build_embodied_view_model`: with `strongbox_tomas` closed-and-opaque, the `check.container.strongbox_tomas` entry has `enabled == false` and `why_disabled == Some("The container is closed.")`.
2. New `tracewake-core` view-model test: with the connecting door closed and blocking, the `move.to.<dest>` entry has `enabled == false` and `why_disabled == Some("The door is closed.")`.
3. New `tracewake-core` view-model test: with a locked container, the `check.container.<id>` entry has `enabled == false` and `why_disabled == Some("The container is locked.")`.
4. Parity test: for one rejected entry, its `why_disabled` equals the `actor_visible_summary` of `submit_semantic_action`'s `ValidationReport` for the same action.
5. Updated `crates/tracewake-tui/tests/embodied_flow.rs`: before submitting `move.to.back_room` through the closed door, the rendered menu marks that entry `disabled`; the existing post-submit `Rejected` + `Why-not:` assertions still pass.
6. Full pipeline: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No embodied menu entry is presented as `enabled` when the authoritative validator would reject the equivalent proposal.
2. A disabled entry's `why_disabled` text is the validator's `actor_visible_summary` — never a second, separately-authored string.
3. `semantic_actions()` contains no hand-written precondition logic; availability is derived solely from `validate_proposal`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (tests module) — closed-container, closed-door, and locked-container disabled-marking tests, plus the why_disabled↔submit-summary parity test.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — pre-submit disabled assertion for `move.to.back_room` alongside the existing submit/why-not assertions.

### Commands

1. `cargo test -p tracewake-core projections:: && cargo test -p tracewake-tui --test embodied_flow`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
