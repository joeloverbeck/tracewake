# 0002PHA1KERTUI-008: Shared proposal/validation/commit pipeline and action registry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `actions` module (registry, proposal, pipeline, mutation plan, validation reports, reason codes) to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-003, 0002PHA1KERTUI-004, 0002PHA1KERTUI-006

## Problem

One shared command/proposal/action validation path is the spine of Phase 1 (Spec 0002 §14). Human commands, test harnesses, the no-human scheduler, and future agents must all use it; origin is provenance metadata only and must not change physical validation. This ticket builds the action registry, the proposal data contract (§14.2), the full pipeline stages (§14.3, including the four inert architectural slots required by `docs/2-execution/05_…REPLAY.md`), mutation-plan construction, validation-report creation, and the structured rejection reports with stable reason codes (§14.5). Concrete action definitions (move/open/close/take/place/inspect/wait) register against this in tickets 009–011.

## Assumption Reassessment (2026-06-06)

1. No `actions` module exists; registers `pub mod actions;` in `crates/tracewake-core/src/lib.rs` (001), using `state`/`location` (003), `OrderingKey`/proposal-sequence (004), and `events` append/apply (006).
2. The pipeline stages are `specs/0002_…_SPEC.md` §14.3 — note this list was corrected during reassessment to **19 stages** including the four inert slots (8 knowledge/perception, 9 social/norm, 10 cost/duration, 11 reservation/conflict) mandated by `docs/2-execution/05_…REPLAY.md` ("must exist as architectural slots so later phases do not bolt on parallel rules"). The proposal contract is §14.2; reason codes are §14.5 (including the reassessment-added `target_reserved`); registry parity is §14.7.
3. Shared boundary under audit: the `ActionRegistry`, `Proposal`, `ValidationReport`, and `ReasonCode` types consumed by every action definition (009–011), controller binding (014), no-human advance (015), debug reports (016), and the TUI (020). Fixed here.
4. Invariant motivating this ticket: INV-007 (every world-affecting player action must be NPC-possible) and INV-043 (action validation is ordinary-agent validation, no player-privilege branch); INV-070 (why-not explanations are mandatory) drives the structured rejection report; INV-008 (UI is not authority) keeps validation in the kernel.
5. Fail-closed validation surface: the pipeline is the kernel's authority gate. It must be deterministic and blocking, produce a structured report (not a bare "no"), and emit events only via the ticket-006 apply path. The four inert slots are present-but-vacuous (Phase 1 has no epistemics/norms/cost/reservation); they introduce no leakage or nondeterminism and are documented as deferred enforcement surfaces for Phase 2+. Possession parity (014) and registry-parity tests (ticket 022) enforce the no-player-branch invariant.

## Architecture Check

1. A single registry + single pipeline (with origin as metadata) structurally guarantees human/test/no-human/agent parity: there is no second code path a player-only verb could live in. Branching validation by origin (the alternative) would violate INV-043 and INV-007. The inert slots are kept as explicit stages rather than omitted so Phase 2 attaches epistemics without forking the pipeline.
2. No backwards-compatibility shims: greenfield; no origin-specific fast path.

## Verification Layers

1. Shared-path parity (INV-007/043) -> unit test: the same (actor, action, targets, state) yields the same `ValidationReport` regardless of origin metadata.
2. Why-not structure (INV-070) -> unit test: a rejected proposal produces a `ValidationReport` with `failed_stage`, stable `reason_codes`, `checked_facts`, an actor-safe summary, and a debug summary — never an unstructured string.
3. Event-only mutation (INV-009; §14.3 stages 15–17) -> codebase grep-proof: accepted proposals reach state only by constructing world events and calling the ticket-006 applier; rejected proposals call no applier.
4. Pipeline-slot fidelity (exec-05) -> codebase grep-proof: all four inert slots (stages 8–11) are present in the pipeline definition.

## What to Change

### 1. Registry + proposal contract

Add `crates/tracewake-core/src/actions/registry.rs` (the single list of ordinary action definitions) and `actions/proposal.rs` (the §14.2 proposal fields: `proposal_id`, `origin`, `actor_id`, `action_id`, `target_ids`, `parameters`, `requested_tick`, optional `source_view_model_id`).

### 2. Pipeline

Add `crates/tracewake-core/src/actions/pipeline.rs` running the §14.3 19-stage flow (origin intake → controller-binding check → action lookup → actor lookup → target binding → locality/reachability → physical precondition → the four inert slots → phase-boundary → mutation-plan → validation-report → event-envelope construction → append → apply → projection/view update → debug linkage). Inert slots are explicit no-op stages with a documented deferred-enforcement comment.

### 3. Validation report + reason codes

Add `actions/report.rs` with the §14.5 `ValidationReport` fields and the stable `ReasonCode` enum (every §14.5 code, including `target_reserved`).

### 4. Registration

Add `pub mod actions;` to `crates/tracewake-core/src/lib.rs`; the registry starts empty and is populated by tickets 009–011.

## Files to Touch

- `crates/tracewake-core/src/actions/mod.rs` (new)
- `crates/tracewake-core/src/actions/registry.rs` (new)
- `crates/tracewake-core/src/actions/proposal.rs` (new)
- `crates/tracewake-core/src/actions/pipeline.rs` (new)
- `crates/tracewake-core/src/actions/report.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod actions;`; file created by ticket 001)

## Out of Scope

- Concrete action definitions and their event emission (tickets 009–011).
- Controller-binding authorization mechanics beyond the pipeline's binding-check stage hook (ticket 014).
- Active epistemic/norm/cost/reservation logic for the four slots (Phase 2+; the slots stay inert).

## Acceptance Criteria

### Tests That Must Pass

1. Identical (actor, action, targets, state) proposals with `origin = human` vs `origin = test/scheduler` produce identical report status, reason codes, and resulting events.
2. A rejected proposal produces a structured `ValidationReport` (failed stage + stable reason codes + checked facts), and no world event / no state mutation.
3. The pipeline definition contains all four inert slots (stages 8–11) (grep/assertion).

### Invariants

1. No action validation branches on origin.
2. Accepted proposals mutate state only through the event-application path; rejected proposals mutate nothing.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/pipeline.rs` (unit tests) — origin parity, structured rejection, event-only mutation, slot presence.
2. `crates/tracewake-core/src/actions/report.rs` (unit tests) — reason-code stability.

### Commands

1. `cargo test -p tracewake-core actions`
2. `cargo build --workspace`
3. Core-crate scope is correct: the pipeline operates on in-crate state/events; full human-vs-no-human parity over a fixture is exercised in ticket 022.
