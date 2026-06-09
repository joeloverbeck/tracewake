# 0015PHA3AEVECOG-005: Fail-closed hidden-truth audit gate inside the decision transaction

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `ActorDecisionTransaction::run` returns a typed `Stuck` on forbidden-source inputs; pipeline source-context validation asserts a clean audit; new guard + negative fixture
**Deps**: None

## Problem

ORD-HARD-009: the hidden-truth audit is computed but never enforced. `hidden_truth_audit_from_actor_known_inputs` (`crates/tracewake-core/src/agent/decision.rs`) computes `actor_known_only` and a forbidden-source count, and the result is stored in the trace, serialized into event payloads (`hidden_truth_audit_actor_known_only` in `scheduler.rs`), rendered in debug, and asserted in unit tests — but **no site rejects, stuck-fails, or blocks a proposal**. A transaction whose inputs include `DebugOmniscience` or `UnprovenPhysicalTruth` provenance (both defined in `agent/actor_known.rs`) proceeds through the pipeline and commits. A proof artifact that gates nothing is a decorative proof surface (INV-105); for action-relevant cognition, a forbidden source is a rejection condition (INV-102).

This ticket makes the audit **fail closed**: the transaction stuck-fails on any forbidden source class among action-relevant inputs, and the pipeline asserts the audit is clean for agent-origin proposals as defense in depth. It is independent of the ORD-HARD-008 cutover (a backstop that holds even if a future edit reintroduces a forbidden input).

## Assumption Reassessment (2026-06-09)

1. Current code (verified): `hidden_truth_audit_from_actor_known_inputs` in `agent/decision.rs` computes the audit; consumption is store/serialize/debug/test only. `ActorDecisionTransaction::run` in `crates/tracewake-core/src/agent/transaction.rs` already returns `ActorDecisionTransactionOutcome::Stuck { diagnostic }` at several sites (`transaction.rs:117/180/192`) with `(responsible_layer, blocker_code)` derived from `concrete_blocker.as_str()` (`transaction.rs:412`) — none keyed on the audit. Forbidden classes `DebugOmniscience` / `UnprovenPhysicalTruth` are in `agent/actor_known.rs`; `ForbiddenTruthAudit::passed_excluding` (`epistemics/knowledge_context.rs:330`) currently records forbidden sources without failing. `append_rejection_event` is in `actions/pipeline.rs`.
2. Specs/docs: spec 0015 §ORD-HARD-009 (required correction + structural lock); INV-102 ("for action-relevant cognition, a rejection condition"), INV-099, INV-105 (a proof artifact that gates nothing is decorative); `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`.
3. Shared boundary under audit: the decision-transaction outcome contract (`ActorDecisionTransactionOutcome::Stuck` with typed `StuckDiagnosticRecord`) and the pipeline's source-context validation stage. The new `blocker_code = hidden_truth_input` joins the existing typed blocker vocabulary; the `responsible_layer` names the layer that introduced the input.
4. INV-102 — every action-relevant cognition input requires provenance; missing/forbidden provenance is a rejection condition. INV-105 — decision traces / hidden-truth audits must be authoritative diagnostic data, not decorative; the audit must gate. INV-099 — truth may validate, not plan; a forbidden-truth input means the proposal was planned from truth.
5. Fail-closed validation / actor-knowledge / deterministic-replay surface: names the enforcement surface = `ActorDecisionTransaction::run` (primary) and the pipeline source-context validation stage (defense in depth). Confirm fail-closed: a forbidden source among action-relevant inputs produces a typed `Stuck`, no proposal is constructed, and the stuck diagnostic replays byte-identically. No epistemic-leakage weakening — the typed diagnostic carries `responsible_layer`/`blocker_code`/`hidden_truth_referenced = true` without naming hidden targets in actor-visible text (mirroring the verified-holding INV-106 split).
6. Schema/shape: extends the typed blocker/stuck-diagnostic vocabulary with `hidden_truth_input`. Consumers: `StuckDiagnosticRecord` construction (`transaction.rs:412`/`:427`), the trace serialization (`agent/trace.rs`), replay/checksum. Additive-only (new blocker code; existing codes unchanged); the `hidden_truth_referenced` field already exists (`agent/trace.rs:183`).

## Architecture Check

1. Enforcing the audit at the transaction boundary (proposal not constructed) is stronger and earlier than a downstream pipeline rejection; the pipeline assertion is defense in depth, not the primary gate. Reusing the existing typed `Stuck` machinery keeps the diagnostic substrate typed (INV-105) rather than a display string.
2. No shims: the audit result now gates directly; no parallel "soft" path that lets forbidden inputs through under a flag.

## Verification Layers

1. INV-102 → replay/golden-fixture check (fixture `forbidden_provenance_input_fails_closed_001`): a typed forbidden-class input produces a stuck diagnostic; the log contains no proposal/commit for that window.
2. INV-105 → codebase grep-proof (guard): `transaction.rs` contains the audit gate (positive-presence assertion, mirroring the existing `SelectedGoalBundle` guard).
3. INV-018 → replay/golden-fixture check: replay rebuilds the same stuck diagnostic byte-identically, carrying `responsible_layer`, `blocker_code = hidden_truth_input`, `hidden_truth_referenced = true`.
4. INV-099 → manual review: no proposal is constructed when a forbidden source is present; the actor learns only modeled rejection (no hidden target named in actor-visible text).

## What to Change

### 1. Fail-closed gate in `ActorDecisionTransaction::run`

When the hidden-truth audit finds any forbidden source class among action-relevant inputs, return `Stuck` with `blocker_code = hidden_truth_input` and `responsible_layer` naming the introducing layer. No proposal is constructed.

### 2. Pipeline defense-in-depth assertion

In the pipeline's source-context validation stage, assert the attached decision trace's audit is clean for agent-origin proposals; reject with a typed reason otherwise.

### 3. Guard

Add a guard asserting the audit gate exists in `transaction.rs` (positive-presence, mirroring the `SelectedGoalBundle` check guard).

## Files to Touch

- `crates/tracewake-core/src/agent/transaction.rs` (modify — the gate)
- `crates/tracewake-core/src/agent/decision.rs` (modify — expose the audit verdict for gating, if needed)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — defense-in-depth assertion)
- `crates/tracewake-core/src/agent/trace.rs` (modify — only if the blocker-code serialization needs a new arm)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: also touched by 004/006/007/008, rewritten by 009)
- `crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register the fixture)

## Out of Scope

- The ORD-HARD-008 cutover that prevents forbidden inputs from entering in the first place (`0015PHA3AEVECOG-003`); this gate is the independent backstop.
- Embodied projection (`0015PHA3AEVECOG-006`).
- Glob/census guard generalization (`0015PHA3AEVECOG-009`).

## Acceptance Criteria

### Tests That Must Pass

1. `forbidden_provenance_input_fails_closed_001`: a typed forbidden-class input (no banned words in display text) yields a stuck diagnostic; the log has no proposal/commit for that window; replay rebuilds the same diagnostic.
2. Removing the gate fails the positive-presence guard (`cargo test --test anti_regression_guards`).
3. `cargo test --workspace` green.

### Invariants

1. A forbidden source class among action-relevant inputs always produces a typed `Stuck`; no proposal is constructed.
2. The stuck diagnostic is typed (`responsible_layer`, `blocker_code = hidden_truth_input`, `hidden_truth_referenced = true`) and names no hidden target in actor-visible text.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs` — the fail-closed negative.
2. `crates/tracewake-core/src/agent/transaction.rs` — unit test for the gate returning `Stuck` on a forbidden input.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — audit-gate positive-presence guard.

### Commands

1. `cargo test -p tracewake-core agent::transaction && cargo test -p tracewake-content forbidden_provenance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
