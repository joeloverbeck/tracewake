# 0038SPICEREVE-009: SPINE-08 evidence — no direct dispatch and full mutation-path closure

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — fills the SPINE-08 section of the acceptance artifact from existing tests/negative fixtures.
**Deps**: 0038SPICEREVE-001

## Problem

SPINE-CERT seam SPINE-08 (spec §5) requires proof that no caller bypasses the shared proposal/validation/scheduler/event-append/application path for world-affecting behavior — agent planners, scheduler, TUI, content fixtures, debug helpers, tests, and action definitions cannot directly dispatch state mutation or forge the internal mutation capability. This seam certifies the closure of all bypass routes, not one happy path. This ticket gathers the legal-path traces, the compile-fail/negative-fixture closure evidence for external-crate forgery attempts, and the dependency-boundary evidence, and writes the SPINE-08 section of the acceptance artifact created by `-001`.

## Assumption Reassessment (2026-06-16)

1. The audited seams exist (verified during this session's `/reassess-spec`, 2026-06-16): `crates/tracewake-core/src/events/{mutation,apply}.rs`, `actions/{pipeline,proposal}.rs`, `scheduler.rs`, `controller.rs`, `debug_capability.rs`, `state.rs`, `crates/tracewake-content/src/load.rs`, `crates/tracewake-tui/src/app.rs`; the tests `crates/tracewake-core/tests/{negative_fixture_runner,spine_conformance,ci_workflow_guards}.rs`.
2. Spec §5 SPINE-08 names the negative-fixture closure set, all present under `tests/negative-fixtures/`: `external_crate_cannot_forge_mutation_capability`, `external_crate_cannot_call_seed_mutators_after_load`, `external_crate_cannot_mutate_agent_state_seed_maps`, `external_crate_cannot_insert_raw_epistemic_records`, `external_crate_cannot_read_raw_epistemic_projection_maps` (each with `src/lib.rs`, verified).
3. Shared boundary under audit: the kernel-owned internal mutation capability and the canonical-path closure across all callers. The witness must record one accepted actor proposal, one accepted TUI semantic action, one scheduled passive/completion event, and one replay-applied event sequence as legal mutation paths, plus API-boundary evidence (`pub`/`pub(crate)`/private marker constructors) and the compile-fail closure of forgery attempts.
4. `INV-008` (UI assistance is not authority), `INV-100` (hidden-truth cognition is forbidden), `INV-103` (the scheduler is not a cognition authority), and `INV-104` (routines/needs do not dispatch primitive actions directly) motivate this ticket: internal mutation authority remains kernel-owned and no layer synthesizes actor plans from raw truth.
5. This ticket audits the mutation-capability and dependency-boundary surfaces as an **evidence consumer**: it runs `negative_fixture_runner`, `spine_conformance`, `ci_workflow_guards`, and the TUI seam/adversarial suites, then records witnesses; it weakens no boundary. Adversarial cases (any direct state mutation from TUI/content/action-def/scheduler-rewrite/debug-panel/projection/test-helper; any rejection that fallback-plans from validation truth) must fail by type boundary, compile-fail fixture, explicit negative test, or a mutation-survivor killed by a guard; any survivor is recorded `fail` with responsible layer and routed to remediation.

## Architecture Check

1. Auditing the seam via legal-path traces + compile-fail closure of forgery routes + dependency-boundary evidence (core does not depend on TUI/content for authority) proves bypass-closure as a property of the type system, not a single tested path.
2. No backwards-compatibility aliasing or shims introduced.

## Verification Layers

1. `INV-008`/`INV-100` UI/hidden-truth are not authority -> negative-fixture + codebase grep-proof: the five `external_crate_cannot_*` fixtures fail to compile/run (`negative_fixture_runner`); API-boundary visibility recorded.
2. `INV-103`/`INV-104` scheduler/needs are not cognition authority -> replay/golden check + manual review: scheduler appends only permitted canonical scheduled events; no rejection fallback-plans from validation truth.
3. Dependency-direction closure -> codebase grep-proof: core does not import TUI/content for simulation authority; TUI/content cannot import or call internal mutation constructors (`ci_workflow_guards`, `spine_conformance`).

## What to Change

### 1. Capture the SPINE-08 legal-path and boundary witnesses

Record the four legal mutation paths (accepted actor proposal, accepted TUI semantic action, scheduled passive/completion event, replay-applied event sequence); record API-boundary evidence for `pub`/`pub(crate)`/private marker-capability constructors; record dependency-boundary evidence that TUI/content cannot import or call internal mutation constructors and that core does not depend on TUI/content for simulation authority.

### 2. Record bypass-closure rejection evidence

For each illegal path (direct mutation from TUI/content/action-def/scheduler-rewrite/debug-panel/projection/test-helper; capability forgery; fallback-plan-from-validation-truth), record the compile-fail/negative-test/type-boundary witness or the mutation-survivor-killed-by-guard result, and the named responsible layer.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (modify — file created by 0038SPICEREVE-001)

## Out of Scope

- The §4 cross-seam command transcript and §9.3 environment (owned by `-001`).
- The within-pipeline append-before-apply / mutation-capability witness for the happy path (owned by `-007`, SPINE-06) — this ticket records the exhaustive bypass-closure corpus across all callers.
- The mutation-testing waves and survivor triage register (owned by `-010`) — this ticket records the compile-fail/negative closure, not the cargo-mutants survivor analysis.
- Remediation of any bypass route found — recorded `fail` with responsible layer; fix routed to a separate `SPINE-CERT scoped remediation` ticket/spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — the five `external_crate_cannot_*` mutation/forgery fixtures fail closed.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — no-direct-dispatch seams map to named evidence.
3. `cargo test --locked -p tracewake-core --test ci_workflow_guards` — dependency-boundary/guard workflow holds.
4. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` and `--test adversarial_gates` — TUI cannot mutate state or forge a path.

### Invariants

1. The internal mutation capability is kernel-owned and not constructible/forgeable by TUI, content, tests, or external crates (`INV-008`/`INV-100`).
2. Every legal mutation flows through the canonical path; no rejection fallback-plans from validation truth and no scheduler rewrites actor decisions after a transaction (`INV-103`/`INV-104`).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification runs the existing external-crate negative fixtures plus the spine/CI/TUI guard suites, and the captured legal-path traces + compile-fail closure are the deliverable.`

### Commands

1. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
2. `cargo test --locked -p tracewake-core --test spine_conformance && cargo test --locked -p tracewake-core --test ci_workflow_guards`
3. `cargo test --locked -p tracewake-tui --test tui_seam_conformance && cargo test --locked -p tracewake-tui --test adversarial_gates`
