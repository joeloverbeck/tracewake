# 0057EMBROUCON-002: Embodied continuation commits the follow-on ordinary action

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime command (`runtime/session.rs`, `runtime/receipt.rs`) sequences the `continue_routine` marker plus the resolved follow-on in one embodied transaction; `tracewake-tui` (`app.rs`) consumes the follow-on receipt
**Deps**: 0057EMBROUCON-001

## Problem

Spec 0057 §4.2 and the root-cause analysis (§1.1). Today an embodied `Continue routine` submission runs exactly one pipeline transaction — the `ContinueRoutineProposed` marker, which is by design a non-progress marker (`behavioral_progress=false`) — and returns. Nothing commits the routine step's follow-on ordinary action, so the possessed actor never moves toward work or starts a work block. `crates/tracewake-tui/src/app.rs:228` `submit_entry` even documents this: "embodied targeted-command routing is not yet wired ... Borrow [`target_ids`] (no behavioral effect) to keep the field's reachability guard satisfied until a live consumer lands."

This ticket wires the follow-on: a single embodied `Continue routine` submission commits the marker **and** the 0057EMBROUCON-001 resolved follow-on ordinary action through the shared pipeline, in one embodied transaction, so the player observes behavioral progress and the receipt reflects the follow-on outcome (moved / work started / why-not / waited / stuck) — not merely "Accepted" for the marker. Per the Step-4 decision (§9 R1 → option (a)), the follow-on shares the marker's tick and reuses spec 0047's single-charge-per-tick accounting; it triggers no world advance and does not touch `advance_until` stop-reason semantics (§9 R3, §1.2).

## Assumption Reassessment (2026-06-30)

1. Embodied submission path verified: `crates/tracewake-core/src/runtime/session.rs:459` `run_semantic_proposal(...)` builds the proposal and calls `run_pipeline` (`session.rs:510`); reached via `submit_command` (`session.rs:617`) → `RuntimeCommandKind::SubmitSemanticAction` (`session.rs:623`). `crates/tracewake-core/src/projections.rs:1599` `proposal_from_semantic_action_entry` sets `active_intention_id` (from `entry.target_ids[0]`) and `next_action_id` (from `target_ids[1]`) for `continue_routine` (`projections.rs:1659-1668`). The receipt is `RuntimeActionReceipt` (`crates/tracewake-core/src/runtime/receipt.rs:33`), surfaced via `into_action_receipt` (`receipt.rs:124`). `crates/tracewake-tui/src/app.rs:241` already passes `entry.clone()` (carrying `target_ids`) into the runtime command, so core has what it needs; `app.rs:233` is the deferral-witness borrow to remove.
2. Spec assumption: `specs/0057_…_SPEC.md` §4.2 governs. Step-4 Q1 resolved to **(a)**: the follow-on shares the marker's tick, single-charge per 0047 (§9 R1); it must not change `advance_until` stop-reason semantics (§9 R3) and must not auto-run the whole routine (§1.2 — exactly one routine-step's follow-on per submission).
3. **Cross-artifact boundary under audit**: this ticket consumes 0057EMBROUCON-001's shared resolver. The marker+follow-on sequencing lives in the **core runtime command** (`runtime/session.rs`), not the TUI — `app.rs::submit_entry` only forwards the entry and surfaces the returned receipt (spec §8; reassessment finding M6). No simulation authority is added to the TUI boundary.
4. INV-035 (routines are defeasible intentions — continuation must produce real reach-and-act, not a marker no-op), INV-099 (the follow-on target is resolved from actor-known context via 001, never hidden truth), INV-104 (the follow-on is a `Proposal` committed through shared validation/event commitment, not a direct dispatch), INV-008 / INV-069 (UI assistance is not authority; the TUI must not implement simulation rules — the sequencing is core's).
5. **Fail-closed / replay / single-charge enforcement surface**: the shared action pipeline (`run_pipeline`) and 0047's single-charge-per-tick accounting. Confirm: (a) the follow-on shares the marker's tick — no world advance, no `advance_until` invocation, so stop-reason semantics are untouched (§9 R3); (b) needs/time are charged once per tick, reusing the 0047 accounting, so marker + follow-on do not double-charge (§9 R1); (c) the follow-on is an ordinary committed action, replay-reconstructable and physical-checksum stable (INV-018); (d) no actor-knowledge leakage — the follow-on target comes from 001's actor-known resolver, not truth (INV-099/024).
6. **Schema-shape (runtime-authority surfaces touched)**: `runtime/session.rs` and `runtime/receipt.rs` are modified — **no schema shape change**. No public enum/struct is resealed and no new serialized field is added: the follow-on commits existing event kinds (`ActorMoved`, `WorkBlockStarted`/`WorkBlockCompleted`) through the existing pipeline, and the embodied receipt reflects the follow-on's existing `RuntimeActionReceipt`/report content rather than the marker's. Additive behavior only; `into_action_receipt` shape is preserved (its sole consumer is `app.rs:248`).

## Architecture Check

1. Sequencing the marker and the resolved follow-on inside one core runtime command keeps the TUI presentation-only: `submit_entry` forwards an entry; core owns legality, resolution (via 001), and commitment. This is strictly cleaner than the TUI running two pipeline submits (which would put orchestration — and, through a TUI-local resolver, planning — into presentation, breaching INV-008/069). Sharing the marker's tick with 0047's single-charge accounting is the minimal mechanism that yields behavioral progress without double-charging time/needs (§9 R1) or perturbing world-advance stop-reasons (§9 R3).
2. No backwards-compatibility aliasing or shims: the follow-on is wired into the existing `run_semantic_proposal` path — no parallel command kind, no wrapper. The `app.rs` deferral-witness borrow of `target_ids` is removed because a live consumer (core) now exists. Core retains no dependency on tui.

## Verification Layers

1. INV-035 / INV-095 (real reach-and-act; TUI-playable) -> embodied behavioral test: one `Continue routine` submission on `ordinary_workday_001` commits `ActorMoved home_tomas→workshop_tomas` and the receipt reports the move (not bare "Accepted").
2. INV-008 / INV-069 (UI is not authority) -> codebase grep-proof: the marker+follow-on sequencing lives in `runtime/session.rs`; `app.rs::submit_entry` runs no pipeline and resolves no target — it forwards the entry and returns the core receipt.
3. INV-018 + §9 R1 (determinism / single-charge) -> replay + golden check: the follow-on commit replays byte-identically, and a marker+follow-on tick charges needs/time exactly once (asserted against a single-action baseline).

## What to Change

### 1. Sequence the follow-on in the core runtime command

In `run_semantic_proposal` (`runtime/session.rs`), after committing the `ContinueRoutineProposed` marker for a `continue_routine` entry whose `target_ids[0]` names the authoritative active routine intention, call 0057EMBROUCON-001's shared resolver against that intention's `current_step` to obtain the typed result, and commit the resolved follow-on `Proposal` through the same `run_pipeline` in the same transaction/tick. Reuse 0047's single-charge-per-tick accounting so the marker + follow-on charge time/needs once. On a typed blocker / modeled wait, record it rather than a silent Accepted (the typed-outcome surfacing is hardened in 0057EMBROUCON-003).

### 2. Surface the follow-on outcome in the receipt

Make the embodied `RuntimeActionReceipt` (`runtime/receipt.rs`) reflect the follow-on outcome — moved / work started / why-not / waited / stuck — as the progress of record, rather than the marker's `Accepted`. No new field: reflect the follow-on's existing report.

### 3. Drop the TUI deferral witness

Remove the `let _ = &entry.target_ids;` deferral-witness borrow and its comment in `crates/tracewake-tui/src/app.rs::submit_entry` now that core is the live consumer; `submit_entry` continues to forward the entry and surface the receipt unchanged.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — `run_semantic_proposal` sequences marker + resolved follow-on, single-charge per 0047)
- `crates/tracewake-core/src/runtime/receipt.rs` (modify — embodied receipt reflects the follow-on outcome; no shape change)
- `crates/tracewake-tui/src/app.rs` (modify — remove the deferral-witness borrow; forward entry, surface follow-on receipt)

## Out of Scope

- The shared resolver itself (0057EMBROUCON-001, depended on).
- Typed-blocker / modeled-wait / cross-tick stuck eligibility for repeated no-progress continuations (0057EMBROUCON-003) — this ticket commits the follow-on and records the immediate typed result; the stuck-detection hookup is 003.
- Any world advance, multi-step fast-forward, or change to `advance_until` stop-reason semantics (spec §1.2 / §9 R3).
- The marker definition's invariants (0057EMBROUCON-004) and parity/golden fixtures (0057EMBROUCON-005).

## Acceptance Criteria

### Tests That Must Pass

1. Embodied follow-on test (driving `TuiApp` against `ordinary_workday_001`): a single `Continue routine` submission commits `ActorMoved home_tomas→workshop_tomas`; a subsequent `Continue routine` (now at the workshop) starts a `work_block` — by event ancestry, no teleport.
2. Receipt-surfacing test: the receipt from the embodied `Continue routine` reports the follow-on outcome (moved / work started), not a bare "Accepted" for the marker.
3. Single-charge test (§9 R1): a marker+follow-on tick charges needs/time exactly once (compared against a single ordinary-action baseline tick); `advance_until` stop-reason output is unchanged from 0047 (§9 R3).
4. `cargo test --locked -p tracewake-core && cargo test --locked -p tracewake-tui` — core sequencing + TUI receipt consumption green.

### Invariants

1. The marker still self-reports `behavioral_progress=false`; the committed follow-on (an existing ordinary event kind) is the progress of record, resolved from actor-known context via 0057EMBROUCON-001.
2. The marker+follow-on sequencing is owned by the core runtime command; no pipeline run or target resolution occurs in `tracewake-tui`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` — embodied `Continue routine` reaches `workshop_tomas` then starts `work_block` by ancestry; receipt reflects the follow-on.
2. `crates/tracewake-core/src/runtime/session.rs` (in-module tests) — single-charge-per-tick assertion for a marker+follow-on transaction; `advance_until` stop-reason unchanged.

### Commands

1. `cargo test --locked -p tracewake-tui --test embodied_flow` — embodied behavioral progress + receipt surfacing.
2. `cargo test --locked -p tracewake-core` — runtime sequencing, single-charge, replay-stability.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — four-gate suite.
