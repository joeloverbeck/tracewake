# 0006PHA3ANEEROU-001: Unified event application — live pipeline applies AgentState

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` event-application path (`actions/pipeline.rs`, `events/apply.rs`, `state.rs`); all `run_pipeline` call sites in core/TUI; replay rebuild
**Deps**: None

## Problem

Phase 3A's central substrate does not exist as one live causal loop. The shared action pipeline mutates only `PhysicalState`: `PipelineContext` (`crates/tracewake-core/src/actions/pipeline.rs:78`) carries `&mut PhysicalState` and an optional `&mut EpistemicProjection`, but **no** `AgentState`. `run_pipeline` appends events and calls `apply_event` (`crates/tracewake-core/src/events/apply.rs:66`), which mutates `PhysicalState` only for `EventStream::World` and is a no-op for other streams. Agent-stream events (`NeedDeltaApplied`, intention/routine/trace/stuck events) are applied by a separate `apply_agent_event` path (`apply.rs:166`) used only during replay/projection rebuild. Consequently a `NeedDeltaApplied` emitted by `eat`/`sleep`/`work`/`wait` never reaches live `AgentState`, and the TUI app's owned `agent_state` (`crates/tracewake-tui/src/app.rs`) goes stale after actions. This is audit defect **D-02 / F-02** and the spine the rest of Spec 0006 hangs off; it implements spec §5.1 ("one causal event application contract").

## Assumption Reassessment (2026-06-07)

1. `PipelineContext` (`crates/tracewake-core/src/actions/pipeline.rs:78`) currently has fields `registry`, `state: &mut PhysicalState`, `log`, `controller_bindings`, `epistemic_projection: Option<&mut EpistemicProjection>`, `content_manifest_id`, `ordering_key` — confirmed no `AgentState`. `apply_event` (`events/apply.rs:66`) early-returns unless `event.stream != EventStream::World` is false; `apply_agent_event` (`events/apply.rs:166`) gates on `EventStream::Agent`. Both confirmed present.
2. Spec §5.1 requires world/agent/epistemic effects all applied to live projections by the shared pipeline, replay using the same event semantics with loud mismatch, and "no Phase 3A state mutation occurs without an event cause." Spec §9 forbids "direct mutation of `AgentState` outside event application." The spec explicitly allows a unified `SimulationState`/`StateBundle`, a transaction object, or an architecture-compatible equivalent — the requirement is live/replay equivalence, not a specific type name.
3. Shared boundary under audit: the single event-application contract consumed by (a) the live action pipeline, (b) the no-human scheduler, (c) replay rebuild (`crates/tracewake-core/src/replay/rebuild.rs`), and (d) the TUI app (`crates/tracewake-tui/src/app.rs`). `run_pipeline`/`PipelineContext` consumers verified across ~15 files (`actions/defs/{eat,work,inspect,continue_routine}.rs`, `actions/mod.rs`, `controller.rs`, `debug_reports.rs`, `replay/{rebuild,report}.rs`, `scheduler.rs`, core+content+TUI tests). Every call site must supply the agent projection after this change.
4. Motivating invariants (restated): INV-009 "Meaningful state changes require events" and INV-011 "Current-state-only simulation is forbidden" — live agent state must be event-derived, not a parallel artifact; INV-018 "Deterministic replay is foundational" — live and replay application must share semantics so identical inputs reproduce byte-identical state.
5. Deterministic-replay surface touched: the event-application dispatch shared by live run and replay rebuild. This ticket makes live application route through the **same** dispatch as `apply_agent_event` so the two cannot diverge; it introduces no wall-clock or nondeterministic input into application and preserves existing event ordering (`ordering_key`). Replay must fail loudly (identify event position/kind/version) on an unsupported agent event rather than silently skip — this is strengthened, not weakened. No epistemic-leakage surface is altered (this is application, not projection-to-viewer).

## Architecture Check

1. Routing live application and replay through one dispatch eliminates the structural drift the audit identified (live mutates physical-only; agent state is replay-only). A unified application context (threading `&mut AgentState` into `PipelineContext` alongside the existing `&mut EpistemicProjection`, and having `apply_event` delegate per-stream to the world/epistemic/agent appliers) is the minimal change that preserves the existing pipeline shape while closing the gap; a separate post-hoc "apply agent events" pass at each call site would re-create the same divergence risk.
2. No backwards-compatibility aliasing/shims: the old physical-only application path is replaced, not kept alongside. Call sites are updated in place; no dual write path remains (per project no-shim convention and §9).

## Verification Layers

1. INV-009 / INV-011 (events cause live agent state) -> codebase grep-proof: `PipelineContext` carries the agent projection and `run_pipeline` applies `EventStream::Agent` events live; no `AgentState` mutation outside the application dispatch.
2. INV-018 (live/replay equivalence) -> replay/golden-fixture check: a scenario that emits agent events, run live then rebuilt via `replay/rebuild.rs`, yields identical `AgentState` (agent-state checksum equality).
3. Failure-loud replay -> manual review + test: an unsupported/unknown agent event in replay produces a loud error identifying event position/kind/version, not a silent no-op.

## What to Change

### 1. Thread the agent projection into the application context

Add live `AgentState` to `PipelineContext` (or introduce a unified state bundle threaded through `run_pipeline`), mirroring how `epistemic_projection` is already carried. Update `apply_event` so accepted events dispatch per `EventStream` to the world applier (existing), the epistemic applier (existing), and the agent applier (`apply_agent_event`, now invoked live as well as in replay).

### 2. Update every `run_pipeline` call site

Supply the agent projection at each of the ~15 call sites (`actions/defs/*`, `controller.rs`, `scheduler.rs`, `replay/rebuild.rs`, `debug_reports.rs`, TUI `app.rs`, and the test harnesses). The TUI app must update its owned `agent_state` from emitted agent events after each submitted action.

### 3. Make replay share the same dispatch and fail loudly

Ensure `replay/rebuild.rs` applies all three streams through the same dispatch used live, and that an unsupported agent event version/kind aborts replay with a positioned error.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/debug_reports.rs` (modify)
- `crates/tracewake-core/src/actions/defs/eat.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)

## Out of Scope

- The autonomous decision loop / replacing wait-only `run_no_human_day` (0006PHA3ANEEROU-007).
- Typed HTN conditions (0006PHA3ANEEROU-003), actor-known planner input (0006PHA3ANEEROU-002), `continue_routine` semantics (0006PHA3ANEEROU-004).
- New need/intention/routine candidate behavior (0006PHA3ANEEROU-005/006) — this ticket only makes their events apply live.
- Any new event kind or persisted event-envelope schema change.

## Acceptance Criteria

### Tests That Must Pass

1. A test submitting `eat`/`sleep`/`work`/`wait` through the live pipeline asserts the corresponding live `AgentState` need values changed (not only that a `NeedDeltaApplied` event was appended).
2. A live-vs-replay equivalence test: the agent-state checksum after a live action sequence equals the checksum after replay rebuild of the same log.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No `AgentState` field is mutated outside the unified event-application dispatch (no direct setters at call sites).
2. Live application and replay rebuild use the same per-stream dispatch; an unsupported agent event aborts replay with a positioned error.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` — add live-application + live/replay agent-state equivalence assertions.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` — update fixture runs to assert post-action live `AgentState`.

### Commands

1. `cargo test -p tracewake-core --test acceptance_gates`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings` — full-pipeline gate (CI treats warnings as errors).
