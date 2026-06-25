# 0052FOUCONFOU-003: F4-02 — closed runtime command boundary (atomic cutover)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replaces the `advance_world_after_acceptance` boolean / raw proposal route with the closed typed command family; moves proposal-sequence allocation inside the runtime
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-002

## Problem

Spec 0052 F4-02 (§1.1.2, §4.3): the only closed command is one-tick wait, and even that is not the production wait path — `TuiApp::submit_entry` (`app.rs:238`) converts `wait.1_tick` (app.rs:246) into a proposal routed through `submit_controlled_proposal` (`session.rs:171`) with a caller-selected `advance_world_after_acceptance: bool` (session.rs:175); the `false` branch builds a `PipelineContext` and calls `run_pipeline` directly without a world step. The client also allocates ordering authority via `assign_proposal_sequence` (session.rs:167). The TUI thus composes proposal identity/ordering itself and decides whether the rest of the loaded world advances.

This ticket replaces the boolean + raw proposal route with a closed typed command family (built in 001) owned by core: proposal IDs/sequences and ordering keys are allocated **inside** the runtime; the client supplies semantic intent and actor-filtered target selection only; every accepted world-affecting command routes through one core transaction coordinator; and whether a non-wait action consumes time is a **core rule**, not a TUI boolean. Commands return sealed receipts (001), never raw `PipelineResult`/`AdvanceUntilResult`. Closes standing survivors #19 (`assign_proposal_sequence -> default`) and #22 (`submit_entry_with_world_advance` wait branch).

## Assumption Reassessment (2026-06-25)

1. `submit_controlled_proposal` (`session.rs:171`) takes `advance_world_after_acceptance: bool` (175); the `true` branch uses `transact_world_one_tick` (`scheduler.rs:654`), the `false` branch calls `run_pipeline` (`actions/pipeline.rs:159`) on a `PipelineContext` (pipeline.rs:84) returning `PipelineResult` (pipeline.rs:96). `assign_proposal_sequence` (session.rs:167) is public. `TuiApp::submit_entry` (app.rs:238) branches on `semantic_action_id == "wait.1_tick"` (app.rs:246) and forwards through `submit_entry_with_world_advance` (app.rs:250).
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.3; closure-order step 2. The closed `RuntimeCommand` family + sealed receipts this ticket routes through are from 0052FOUCONFOU-001; the production bootstrap is already cut over by 0052FOUCONFOU-002.
3. Cross-artifact boundary under audit: the `tracewake-core` public command surface consumed by `tracewake-tui`. After this ticket the TUI submits typed commands and receives sealed receipts; it never names a proposal sequence, a world-advance boolean, or a raw pipeline route.
4. Motivating invariants: INV-009/INV-018/INV-043 (eventful, ordinary-agent validation with core-owned ordering) and INV-069 (TUI not a simulation authority). INV-043 specifically: action validation is ordinary-agent validation that "does not branch for player privilege" — the controlled and autonomous paths must reach the same validation/event rules with controller binding changing input only.
5. Fail-closed / determinism surface: proposal-sequence/ordering allocation moves inside the runtime (deterministic, event-derived), so no client can perturb canonical ordering (INV-018). Rejected-action atomicity becomes core-decided (advance nothing, commit a consequential failure event, or produce actor-visible feedback per event doctrine) — the rejection path leaks no validator-only detail to the actor (INV-106). The enforcement surface is the single core transaction coordinator (`transact_world_one_tick` and the command dispatcher).
6. Removal blast radius (old menu item 7): the public `assign_proposal_sequence`, the `advance_world_after_acceptance` parameter, the boolean-branched `submit_controlled_proposal`, and the TUI's direct sequence allocation are removed; consumers are `runtime/session.rs` and `tracewake-tui/src/app.rs` (and the `assign_proposal_sequence_advances_monotonically_from_runtime` unit test at session.rs:392, re-expressed as a command-level ordering test). `run_pipeline` direct-dispatch reachability is reduced to crate-internal use behind the coordinator. All in-workspace; updated in this diff.

## Architecture Check

1. One core dispatcher with no client transaction-policy boolean is cleaner than the dual boolean branches because it makes "the client chooses world-step choreography" unrepresentable — the action-to-time policy lives in a single core rule, and proposal ordering is core-allocated, so reviewers cannot point at a closed token while clients bypass it (driver §6.1 recurrence mechanism 2). 
2. No backwards-compatibility alias: the boolean route and public sequence allocator are removed, not retained behind a wrapper. Closure tests use the public production command the TUI uses, not `transact_world_one_tick` directly or a test-sequenced proposal.

## Verification Layers

1. INV-043 (ordinary-agent validation, no player branch) -> differential witness: equivalent controlled and autonomous actions reach the same validation/event rules; controller binding changes input only.
2. INV-018 (deterministic ordering) -> behavior test + mutation: proposal sequence is runtime-allocated, unique and monotonic/event-derived across multiple submitted commands (kills #19).
3. INV-069 (client boundary) -> external compile-fail negative-fixture intent (corpus in 009): a client crate cannot call `run_pipeline` with runtime aggregates, assign proposal sequences, or pass a world-advance boolean.
4. Action-to-time policy / rollback-commit -> table-driven public-boundary suite over wait, non-wait accepted, non-wait rejected, duration start/continue, pause/continue through one dispatcher (kills #22).

## What to Change

### 1. Closed command dispatch (`session.rs`)

Replace `submit_controlled_proposal(..., advance_world_after_acceptance)` with the closed `RuntimeCommand` dispatch: a single entry routing every accepted world-affecting command through one core transaction coordinator. Allocate proposal IDs/sequences/ordering keys inside the runtime. Make whether a non-wait action consumes time a core rule. Return sealed receipts; never return raw `PipelineResult`/`AdvanceUntilResult` to the client. Remove public `assign_proposal_sequence`.

### 2. Core-decided rejected-action atomicity (`session.rs`, `actions/pipeline.rs`)

The core decides whether a rejected proposal advances nothing, commits a consequential failure event, or produces actor-visible feedback (event doctrine). Reduce `run_pipeline` to crate-internal use behind the coordinator.

### 3. Production TUI wait + action path (`app.rs`)

Rewrite `submit_entry` / `submit_entry_with_world_advance` to submit the real typed commands (wait and non-wait) and consume sealed receipts; delete the `"wait.1_tick"` boolean branch and any client sequence allocation.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — closed dispatch, internal sequence allocation, remove boolean + public `assign_proposal_sequence`)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — `run_pipeline` crate-internal behind coordinator)
- `crates/tracewake-tui/src/app.rs` (modify — `submit_entry` uses real commands, eliminate boolean)

## Out of Scope

- Removing raw aggregate getters / internalizing no-human/rebuild/perception/view (004).
- Replay authority reconstruction (005), declared process (006), census (007), embodied/debug split (008).
- The full external negative-fixture corpus and production-boundary conformance lane (009).

## Acceptance Criteria

### Tests That Must Pass

1. A table-driven public-boundary suite sends wait, non-wait accepted, non-wait rejected, duration start/continue, and pause/continue through the one dispatcher and asserts correct world-step/event behavior — using the public production command accepted by the TUI, not `transact_world_one_tick` directly or a test-sequenced proposal.
2. A differential witness: an equivalent controlled and autonomous action reaches the same validation/event rules, controller binding changing input only.
3. A runtime-command test submits multiple commands and proves unique monotonic/event-derived proposal ordering (kills #19); a wait-vs-non-wait differential proves the single core time policy (kills #22).
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No client-facing method allocates a proposal sequence, passes a world-advance boolean, or obtains a raw `PipelineResult`/`AdvanceUntilResult` (INV-069, INV-043).
2. Proposal ordering is runtime-allocated and deterministic across submitted commands (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — table-driven public-command dispatch matrix; wait-vs-non-wait time-policy differential.
2. `crates/tracewake-core/src/runtime/session.rs` test module — re-express `assign_proposal_sequence_advances_monotonically_from_runtime` as a command-level monotonic-ordering test (no public allocator).

### Commands

1. `cargo test -p tracewake-tui --test command_loop_session`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
