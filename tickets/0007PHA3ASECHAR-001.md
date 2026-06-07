# 0007PHA3ASECHAR-001: Typed actor-known planner facts; derive hidden-truth audit

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` planner/HTN facts (`agent/planner.rs`, `agent/htn.rs`, `agent/routine.rs` condition enum, `agent/methods.rs`, `agent/trace.rs`); minimal scheduler caller update to keep the build green; content fixture `planner_trace_001`
**Deps**: <none>

## Problem

The no-human planner path carries actor-known inputs as raw strings and proves hidden-truth safety by convention (Spec 0007 D-03, D-04, §HTN and local planner, §Forbidden shortcuts). Concretely at the audited commit:

- `build_agent_proposal` seeds `actor_known_inputs: Vec<String>` with self-attestation tokens — `crates/tracewake-core/src/scheduler.rs:441-445` pushes `"reason_available"`, `"reevaluation_scheduled"`, and `format!("day_window:{}", window.window_id)`; `crates/tracewake-core/src/agent/htn.rs:375` pushes another `"reason_available"`.
- The typed `RoutineCondition` enum still defines self-attestation variants whose only "proof" is the caller having typed the string: `Self::ReasonAvailable => "reason_available"`, `Self::RoutePlannerAvailable => "route_planner_available"`, `Self::ReevaluationScheduled => "reevaluation_scheduled"` (`crates/tracewake-core/src/agent/routine.rs:224-249`), parsed back from raw strings at `routine.rs:249-252`.
- The planner hard-codes `HiddenTruthAudit { actor_known_only: true, .. }` in `make_local_plan_trace` (`crates/tracewake-core/src/agent/planner.rs:355`) rather than deriving the flag from input provenance / proof sources.

A scaffold or fixture can therefore make a method applicable, and assert hidden-truth safety, with no modeled belief/observation/routine-assignment backing.

## Assumption Reassessment (2026-06-07)

1. The raw-string self-attestation seeding is confirmed at `crates/tracewake-core/src/scheduler.rs:441-445` and `crates/tracewake-core/src/agent/htn.rs:375`; the `RoutineCondition` self-attestation variants and their `as_str`/parse round-trip are confirmed at `crates/tracewake-core/src/agent/routine.rs:224-252`; the hard-coded audit is confirmed at `crates/tracewake-core/src/agent/planner.rs:355` (`actor_known_only: true`). `HiddenTruthAudit` is defined at `crates/tracewake-core/src/agent/trace.rs:37`.
2. Spec 0007 D-03 requires replacing magic caller strings (`reason_available`, `route_planner_available`, ad-hoc `day_window:*`) with typed actor-known facts unless generated from typed, validated planner context; D-04 / §HTN requires the hidden-truth audit be computed from input provenance and typed proof sources, not hard-coded `true`; §Forbidden shortcuts bars "hidden-truth audit hard-coded to true" and "planner safety proven only by a boolean or by caller promises".
3. Shared boundary under audit: the actor-known planning-fact contract between the scheduler/HTN callers (producers of `actor_known_inputs`) and the planner/condition resolver (consumer). This ticket makes the contract typed and provenance-bearing; the scheduler decision driver (0007PHA3ASECHAR-004) and content validation (0007PHA3ASECHAR-011) are the downstream consumers.
4. Motivating invariants (restated): INV-036 "HTN methods are procedures, not story scripts" — applicability must be state-conditioned and inspectable, never a string that always passes; INV-024 "No telepathy" — a planner fact may enter only from a modeled actor-known source; INV-037 "Bounded local planning is for concrete means" — planning is nested under typed methods, not raw caller assertions.
5. Actor-knowledge-filtering and determinism surface touched: the hidden-truth audit (`HiddenTruthAudit.actor_known_only`) is the no-leak proof for autonomous planning. Deriving it from proof-source provenance (rather than a literal `true`) is the enforcement; a fact with no modeled source must drive `actor_known_only = false` (or condition rejection), never silently pass. Method/condition resolution stays deterministic with stable tie-breaks (no hash-map iteration-order dependence); the audit derivation reads only already-canonical proof-source data, introducing no nondeterministic input.
6. Schema change: `RoutineCondition` self-attestation variants are removed (or their stable IDs regenerated only from typed facts and never parsed back as authority). Consumers of the condition representation: HTN selection (`agent/htn.rs`, `agent/methods.rs`), content routine templates / fixtures (`crates/tracewake-content/src/fixtures/*`, validated by 0007PHA3ASECHAR-011), and the decision trace projection (`agent/trace.rs`). This is a **breaking** representation change for those self-attestation conditions — intentional per D-03; no back-compat string path is retained.
7. Rename/removal blast radius (grepped): `reason_available` → `agent/routine.rs:224,249`, `agent/htn.rs:375`, `scheduler.rs:442`; `route_planner_available` → `agent/routine.rs:227,252`; `reevaluation_scheduled` → `agent/routine.rs`, `scheduler.rs:443`; `day_window:` → `scheduler.rs:444`. All sit in this ticket's Files to Touch except the deeper scheduler oracle removal, which is owned by 0007PHA3ASECHAR-004 (which `Deps` on this ticket). Content fixtures referencing these conditions are re-validated under 0007PHA3ASECHAR-011.

## Architecture Check

1. A typed actor-known fact carrying its proof source, plus an audit flag derived from those sources, replaces "a string the caller promised" with "a fact proven from modeled state" — both safer (no accidental auto-pass, no convention-only safety) and inspectable (the trace shows which source backed each fact). It also gives content validation (0007PHA3ASECHAR-011) a typed surface to reject behavior-looking or unbacked conditions at load time.
2. No backwards-compatibility aliasing/shims: the self-attestation variants and the raw-string seeding are removed outright; no string-condition fallback and no `actor_known_only: true` default are retained.

## Verification Layers

1. INV-036 / INV-037 (explicit state-conditioned methods) -> codebase grep-proof: no `reason_available` / `route_planner_available` / `day_window:` self-attestation seeding remains in `agent/htn.rs` or `scheduler.rs`; conditions resolve from typed facts.
2. INV-024 (no telepathy via the audit) -> replay/golden-fixture check: a planner input with no modeled proof source yields `actor_known_only = false` (or condition rejection) and is recorded in the trace (negative test in `planner_trace_001`).
3. Determinism -> manual review + unit test: audit derivation and condition resolution are stable across runs; proof-source ordering is canonical.

## What to Change

### 1. Typed actor-known facts replace raw `actor_known_inputs` self-attestation

Introduce a typed actor-known fact (carrying kind + proof source) for the facts currently smuggled as `"reason_available"` / `"reevaluation_scheduled"` / `"day_window:*"`. Where a fact is a genuine planner-context value (e.g. the active day window), generate it from typed, validated planner context; where it was pure self-attestation, remove it. Update `agent/methods.rs` condition usage accordingly.

### 2. Remove self-attestation `RoutineCondition` variants

Delete `ReasonAvailable`, `RoutePlannerAvailable`, `ReevaluationScheduled` (and any sibling self-attestation variant) from the condition enum, or regenerate their stable serialization IDs solely from typed facts and never parse them back as authority inside decision logic.

### 3. Derive the hidden-truth audit from provenance

In `make_local_plan_trace` (`agent/planner.rs:355`), compute `HiddenTruthAudit.actor_known_only` from the actual proof sources of the planning inputs (true only when every consumed fact has a modeled actor-known/visible-local source), not the literal `true`. Record the deciding proof sources in the audit notes.

### 4. Minimal scheduler caller update (keep the build green)

Replace the raw `vec!["reason_available", "reevaluation_scheduled", format!("day_window:..")]` seed at `scheduler.rs:441-445` with the typed-fact equivalent so the workspace compiles. The deeper removal of the PhysicalState oracle and the empty-`EpistemicProjection` construction is explicitly deferred to 0007PHA3ASECHAR-004.

## Files to Touch

- `crates/tracewake-core/src/agent/planner.rs` (modify)
- `crates/tracewake-core/src/agent/htn.rs` (modify)
- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/agent/methods.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — minimal caller update only; oracle removal owned by 0007PHA3ASECHAR-004)
- `crates/tracewake-content/src/fixtures/planner_trace_001.rs` (modify)

## Out of Scope

- Removing the PhysicalState oracle and building the real actor-known planning context in the proposal builders (0007PHA3ASECHAR-004, dependent).
- Typed routine-family dispatch on `RoutineExecution` (0007PHA3ASECHAR-002).
- Content-side validation enforcing the typed condition schema (0007PHA3ASECHAR-011).
- Typed candidate generation (0007PHA3ASECHAR-003).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: `grep -rnE 'reason_available|route_planner_available|day_window:' crates/tracewake-core/src/agent crates/tracewake-core/src/scheduler.rs` returns no self-attestation seeding or self-attestation condition variant (only typed-fact-derived IDs, if any, remain).
2. A negative unit test: a planning input with no modeled proof source drives `HiddenTruthAudit.actor_known_only = false` (not `true`); a fully actor-known input drives `true`, with proof sources recorded.
3. `planner_trace_001` asserts the audit flag is derived (a no-source input is reflected as not-actor-known) and contains no self-attestation condition.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. No `actor_known_only: true` literal remains in `agent/planner.rs`; the flag is computed from proof sources.
2. No self-attestation string condition or raw self-attestation seed remains in the HTN/scheduler decision path.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/planner.rs` — unit tests: audit derivation true/false by proof-source presence; deterministic proof-source ordering.
2. `crates/tracewake-core/src/agent/htn.rs` — unit test: a removed self-attestation condition no longer auto-passes applicability.
3. `crates/tracewake-content/src/fixtures/planner_trace_001.rs` — assert derived audit + absence of self-attestation conditions.

### Commands

1. `cargo test -p tracewake-core agent::planner`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
