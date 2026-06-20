# 0042ORDLIFCER-004: ORD-LIFE-03 — durable intention lifecycle, typed ancestry, replacement semantics, and possession neutrality

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-03 must be proven: intentions are durable commitments whose live semantics cover active, continued, suspended, interrupted, completed, failed, abandoned, and replaced states; every transition requires a typed reason and causal ancestry; urgent needs may interrupt but not silently erase an intention; and controller possession changes command authority only — it may not reset or rewrite need, routine, intention, memory, or actor-known state. This ticket records the lifecycle ledger, possession-neutrality witnesses, adversarial terminal-state negatives, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/intention.rs` (`Intention`, `IntentionStatus`, `ActorIntentions`), `agent/decision.rs`, `agent/routine.rs`, `agent/transaction.rs`, `agent/trace.rs`, `events/{envelope,apply}.rs`, `replay/rebuild.rs`, `controller.rs`; the possession fixtures `possession_does_not_reset_intention_001` and `possession_parity_001` and the tui `embodied_flow` binary exist (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-03 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 1/9 govern this ticket; it consumes but does not re-audit the EPI possession guarantee (predecessor EPI-CERT pass, spec §11). Spec §10 risk: lifecycle semantics are split across intention status, decision lifecycle effects, and events — the artifact must prove the full semantic lifecycle and ancestry rather than infer it from one enum.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-03 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-03): `INV-006` (possession transfers no knowledge / resets nothing), `INV-105` (typed decision/stuck traces), `INV-034`/`INV-035` (durable intentions / defeasible routines), `INV-041`, `INV-094`, `INV-108`. Restate before trusting the narrative: every transition is typed + ancestry-backed; possession binding is cognition-neutral.
5. Evidence-consumer surface (audit-reads, does not modify): the possession-binding path and the intention-lifecycle replay rebuild. This ticket runs bind/unbind fixtures and records before/after ledgers; it adds no reset path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Recording a full intention transition ledger (semantic state + implementation encoding + event kind/ID + typed reason + predecessor/successor ancestry) and proving final-state equality cannot hide different transition ancestry is stronger than checking only the current intention — spec §5 requires that two runs with the same current intention but different illegal histories not share certifying evidence.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-006` possession neutrality -> replay/golden-fixture check (`possession_does_not_reset_intention_001`, `possession_parity_001`: bind/possessed-action/unbind leaves need, routine, intention, memory, actor-known state, decisions, and replay continuous).
2. `INV-105`/`INV-034` typed lifecycle + ancestry -> codebase grep-proof + manual review (each adoption/continuation/suspension/interruption/resumption/replacement/completion/failure/abandonment transition carries typed reason + prior/new intention + triggering event).
3. `INV-018` lifecycle replay -> replay/golden-fixture check (clean replay reproduces current intention, terminal history, routine association, transition order, decision-trace lifecycle effect).

## What to Change

### 1. Record the lifecycle and possession-neutrality witnesses

Run the §5 positives: event-backed traces for adoption/activation, continuation, suspension, interruption, resumption-or-replacement, completion, failure, abandonment (typed reason + prior/new intention + triggering need/routine/action event); mild pressure preserving an active intention while severe hunger/fatigue/safety interrupts via explicit event and later resumes/replaces/fails/abandons via another explicit transition; and the possession fixtures comparing before/bind/possessed-action/unbind state and replay. Record the intention transition ledger and possession binding state per row.

### 2. Record adversarial lifecycle and ancestry negatives

Record the §5 adversarial cases: bind/unbind at each lifecycle stage (any reset, source substitution, priority change, memory loss, routine restart, or replacement-without-ancestry is a failure); terminal-state reactivation, duplicate completion, replacement-without-predecessor, continuation-after-abandonment, interruption-without-typed-cause (each rejects or produces the canonical failure path); perturbed event order/replay frontier referencing a future/missing/wrong actor-intention event (reconstruction fails loudly, not plausibly inferred); and two runs with identical current intention but different illegal histories not sharing certifying evidence.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-03 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Re-auditing the EPI-CERT possession-parity guarantee (consumed from the predecessor pass per spec §11).
- Production remediation of any lifecycle/possession defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Routine machinery (`-005`), no-human orchestration (`-010`), and the aggregate verdict / fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-tui --test embodied_flow` pass; `possession_does_not_reset_intention_001` and `possession_parity_001` show controller binding changing command authority only.
2. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes; clean replay reproduces current intention, terminal history, routine association, transition order, and decision-trace lifecycle effect.
3. `cargo test --locked -p tracewake-core --test acceptance_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; recorded adversarial cases (terminal reactivation, duplicate completion, replacement-without-predecessor, interruption-without-cause) reject or hit the canonical failure path.

### Invariants

1. Possession neutrality: bind/unbind never resets or rewrites need, routine, intention, memory, actor-known state, or decision semantics.
2. Typed ancestry: every lifecycle transition is typed and ancestry-backed; final-state equality cannot substitute for legal transition history.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test embodied_flow`

## Outcome

Completed: 2026-06-20

Populated the ORD-LIFE-03 section of `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with command transcript fingerprints, lifecycle evidence, possession-neutrality witnesses, replay/tamper negatives, and a local `pass` result. The report records that embedded unit tests in `agent/intention.rs` and `agent/decision.rs` were covered by the `0042ORDLIFCER-001` `cargo test --workspace --locked` baseline; this ticket's targeted commands provide the integration evidence.

The evidence cites reason-bearing intention status transitions, illegal transition rejection, mild-pressure continuation, urgent/severe-pressure interruption and adoption, event-schema lifecycle matrices, no-human intention/routine ancestry, `possession_fixture_preserves_intention_needs_and_can_continue`, `possession_parity_001`, TUI embodied flow, and continue-routine replay tamper negatives.

Verification:

- `cargo test --locked -p tracewake-core --test no_human_capstone` — passed.
- `cargo test --locked -p tracewake-core --test acceptance_gates` — passed.
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passed.
- `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passed.
- `cargo test --locked -p tracewake-tui --test embodied_flow` — passed.

No production or engine code changed. No remediation was needed or performed.
