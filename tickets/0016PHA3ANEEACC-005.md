# 0016PHA3ANEEACC-005: Audit covers structured context; typed stamp; audit-before-selection

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` planning-context restructure (computed accessors), typed audit stamp in the pipeline, transaction-stage reordering, HTN condition tightening
**Deps**: 0016PHA3ANEEACC-004

## Problem

ORD-HARD-020: the hidden-truth audit has scope and ordering gaps (INV-102 — "for action-relevant cognition, a rejection condition" must cover what cognition actually consumes; INV-099; foundation doc 14):

- `ActorKnownPlanningContext` stores six structured fields (`known_food_sources`, `known_sleep_places`, `known_workplaces`, `known_edges`, `known_containers_by_place`, `known_closed_doors`) *independently* of the audited facts; `audit_with` inspects only the facts while the planner and `htn.rs::resolve_condition` read the structured fields directly. A context with populated structured fields and clean facts audits `actor_known_only = true` while driving real proposals — discipline, not structure.
- `actions/pipeline.rs::source_context_check` rejects agent-origin proposals only when the stringly parameter `hidden_truth_audit_actor_known_only` is present **and** equals `"false"`; an absent key or any other value passes.
- In `ActorDecisionTransaction::run`, `select_phase3a_method` executes before the audit gate — a forbidden input shapes method selection even though the proposal is ultimately blocked.
- `htn.rs::resolve_condition` returns satisfied unconditionally for `FixtureAuthoredPossibility` and `SharedPipelinePreconditions` — a latent knowledge-free template escape hatch.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `ActorKnownPlanningContext` structured fields at `agent/actor_known.rs:242–252` (private `facts: Vec<ActorKnownFact>` :251, accessor `actor_known_facts()` :315, `audit_with` :331 inspects facts only); `source_context_check` (`actions/pipeline.rs:811–832`, absent-key pass at :820–821); `select_phase3a_method` at `agent/transaction.rs:139–145` before the audit gate at :160–169; `resolve_condition` unconditional arms at `agent/htn.rs:266–269`; `phase3a_routine_templates()` (`agent/methods.rs:6–111`) uses neither condition; `hidden_truth_audit_from_actor_known_inputs` lives in `agent/decision.rs:199`.
2. Spec/docs: spec 0016 §ORD-HARD-020 (evidence, required correction, structural lock — all six structured fields per the reassessed enumeration); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-099, INV-102; `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` (provenance graph inspectable end-to-end).
3. Shared boundary under audit: the single-source-of-truth relation between the audited fact list and the structured planning fields the planner/HTN actually consume. After this ticket the structured views are *derived from* the audited facts, so the audit's verdict covers everything cognition reads.
4. INV-102 — the rejection condition must cover action-relevant cognition's real inputs, not a parallel copy. INV-099 — truth may not select goals/plans through an unaudited side door. Restated before trusting the ticket narrative.
5. Fail-closed validation enforcement surface: three strengthened gates — (a) the audit covers structured consumption (computed accessors), (b) the pipeline's defense-in-depth requires a typed audit stamp present-and-clean for agent-origin proposals (absent ⇒ reject, replacing the stringly absent-passes parameter), (c) the audit gate is hoisted before method selection so forbidden inputs cannot shape selection. No replay or epistemic weakening — every change narrows what passes.
6. Adjacent contradictions: the spec offers computed accessors *or* an `audit_with` rejection of structured entries lacking a matching fact. This ticket implements computed accessors (the spec's own §5 item 1 names "structured context fields as computed accessors over audited facts" in the preferred compile-time tier); the audit-side rejection would leave two stores to drift again. Required consequence, not a deviation.

## Architecture Check

1. Computed accessors make the audit's coverage structural: one store (audited facts), derived views, nothing for `audit_with` to miss — versus the audit-side rejection alternative, which keeps two stores and polices their divergence forever. Hoisting the audit before `select_phase3a_method` makes fail-closed mean "no cognition consumed forbidden input," not "the proposal was caught later." A typed stamp replaces stringly-parameter parsing per INV-105's typed-diagnostics doctrine.
2. No backwards-compatibility aliasing/shims: the independent structured fields are removed in favor of accessors (consumers recompile against the same names where signatures allow); the stringly pipeline parameter is replaced by the typed stamp, not kept as a fallback.

## Verification Layers

1. INV-102 (audit covers consumption) → divergence test: an in-crate context constructed with structured entries exceeding its facts fails the audit.
2. Defense-in-depth → pipeline negative test: an agent-origin proposal with the audit stamp absent is rejected.
3. INV-099 (no pre-audit cognition) → ordering assertion: on a forbidden input, the decision trace records zero method candidates (runtime proof that no method-selection symbol executed pre-audit).
4. Template escape hatch → unit test: a template using `FixtureAuthoredPossibility` without a matching fixture-possibility-provenance fact does not resolve satisfied.

## What to Change

### 1. Computed accessors over audited facts

All six structured fields of `ActorKnownPlanningContext` become computed accessors derived from the audited fact list (single source of truth). Planner/HTN call sites keep reading the same logical views.

### 2. Typed audit stamp in the pipeline

Replace the `hidden_truth_audit_actor_known_only` string parameter with a typed audit stamp; `source_context_check` requires it present and clean for agent-origin proposals — absent or dirty ⇒ reject.

### 3. Hoist the audit gate

In `ActorDecisionTransaction::run`, evaluate the hidden-truth audit before `select_phase3a_method`; a dirty audit fails closed before any method selection runs.

### 4. Tighten `resolve_condition`

`FixtureAuthoredPossibility` requires a matching fixture-possibility-provenance fact; `SharedPipelinePreconditions` resolves through the shared pipeline's actual precondition surface rather than unconditionally.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/decision.rs` (modify)
- `crates/tracewake-core/src/agent/htn.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — ordering guard with runtime backstop)

## Out of Scope

- The `SourceEventIds` witness and dangling-provenance fail-closed (0016PHA3ANEEACC-004 — this ticket builds on its fact-layer restructure).
- Workplace-fact believed-access attributes (0016PHA3ANEEACC-007 — consumes the accessor structure this ticket creates).
- Wait/stuck discipline in the transaction (0016PHA3ANEEACC-010).

## Acceptance Criteria

### Tests That Must Pass

1. Divergence test: structured-fields-exceed-facts context fails the audit.
2. Pipeline negative test: agent-origin proposal with absent audit stamp is rejected; forbidden-input run's trace records zero method candidates.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Every input the planner/HTN consumes is derived from the audited fact list — no structured store independent of the audit.
2. The hidden-truth audit executes before any method-selection cognition; agent-origin proposals carry a typed, present-and-clean audit stamp.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/actor_known.rs` — divergence test (in-crate construction; structured entries exceeding facts ⇒ audit fails).
2. `crates/tracewake-core/src/actions/pipeline.rs` — negative test for the absent audit stamp.
3. `crates/tracewake-core/src/agent/transaction.rs` — forbidden-input ordering test (zero method candidates in trace).
4. `crates/tracewake-core/src/agent/htn.rs` — `FixtureAuthoredPossibility` provenance-fact requirement test.

### Commands

1. `cargo test -p tracewake-core actor_known && cargo test -p tracewake-core transaction && cargo test -p tracewake-core htn`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
