# 0042ORDLIFCER-008: ORD-LIFE-07 — planner and decision trace honesty, rejected alternatives, and debug quarantine

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-07 must be proven: decision and local-plan traces must explain selected and rejected candidates/methods, actor-known inputs, lifecycle effects, proposals, failures, blockers, and responsible layers; they are non-diegetic evidence — authorized debug output may compare actor-known input with hidden truth, but trace/debug data may not enter the actor-known packet, candidate generation, planning, proposal, metrics, embodied view, or future memory. This ticket records trace completeness, debug-on/off non-interference fingerprints, no-feedback negatives, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/trace.rs`, `agent/decision.rs`, `agent/planner.rs`, `agent/transaction.rs`, `crates/tracewake-core/src/debug_capability.rs`, `crates/tracewake-core/src/debug_reports.rs`, `crates/tracewake-core/src/view_models.rs`, `crates/tracewake-tui/src/{debug_panels,render,transcript}.rs`; the fixtures `planner_trace_001`, `method_fallback_requires_new_trace_or_stuck_001`, `debug_omniscience_excluded_001`, `debug_attach_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, and the tui `adversarial_gates` + `tui_seam_conformance` binaries exist (`debug_capability.rs`, `debug_reports.rs`, `debug_omniscience_excluded_001`, `debug_attach_001`, `tui_seam_conformance` were §2.2 reassessment-confirmed-present).
2. Spec §5 ORD-LIFE-07 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 7/9 govern this ticket; it consumes rather than re-audits EPI debug authority (predecessor EPI-CERT pass, spec §11). `EMERGE-OBS` (INV-111) may cite these traces as observer-only evidence but cannot satisfy this point alone.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-07 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-07): `INV-107` (planner/decision-trace debug quarantine, no feedback path), `INV-068`/`INV-070` (debug/observability boundaries), `INV-041`, `INV-105`, `INV-111` (observer-only emergence). Restate before trusting the narrative: traces are complete non-diegetic evidence; enabling/reading them changes no cognition, event, state, proposal, metric, or embodied output.
5. Evidence-consumer surface (audited; this ticket reads the debug-quarantine firewall): the authorized debug capability path and the actor-known packet boundary. The proof is a paired debug-off/on relational equality; this ticket adds no admitted debug source class, introduces no leakage path, and confirms debug truth never becomes an admitted source class.

## Architecture Check

1. A paired debug-off/debug-on/debug-read/debug-not-read execution-fingerprint comparison (event log, authoritative state, epistemic projection, actor-known packet, decision, proposal, metrics, embodied output all semantically identical) is the decisive non-interference proof — a changed simulation fingerprint under debug toggling is itself a quarantine failure.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-107` debug non-interference -> replay/golden-fixture check (same fixture with debug disabled/enabled/read/not-read yields semantically identical event log, state, projection, packet, decision, proposal, metrics, embodied output).
2. `INV-024`/`INV-100` no trace→cognition feedback -> codebase grep-proof + manual review (`debug_omniscience_excluded_001`, `debug_attach_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`: debug truth never becomes an admitted source class; feeding a prior trace/report/rendered string/hidden-truth diff into generation/planning fails closed at type/API boundaries).
3. `INV-105` trace completeness -> manual review + replay check (`planner_trace_001`, `method_fallback_requires_new_trace_or_stuck_001`: traces include the complete ordered candidate/method census, rejection reasons, selected goal/method/plan, budget, proposal-or-stuck result, actor-known source refs; clean replay rebuilds the same trace records or the promised replay-derived trace projection).

## What to Change

### 1. Record trace-completeness and debug non-interference witnesses

Run `planner_trace_001` and `method_fallback_requires_new_trace_or_stuck_001`; record traces including the complete ordered candidate and method census, rejection reasons, selected goal/method/plan, budget, proposal-or-stuck result, and actor-known source refs. Enable authorized debug output and record hidden truth and actor-known input side by side with explicit non-diegetic labeling while embodied output retains only actor-known information. Repeat the same fixture with debug disabled/enabled/read/not-read and record semantically identical fingerprints across event log, state, projection, packet, decision, proposal, metrics, and embodied output. Record clean-replay rebuild of decision-trace records (or the promised replay-derived trace projection).

### 2. Record adversarial debug-quarantine negatives

Record the §5 adversarial cases: `debug_omniscience_excluded_001`, `debug_attach_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` (debug truth never an admitted source class); feeding a prior trace / debug report / rendered string / hidden-truth diff / operator annotation into candidate generation or planning (type/API boundaries + live negatives fail closed); omitting rejected candidates/methods/blocker/budget/responsible-layer while leaving a plausible final action (evidence-honesty tests reject the incomplete trace); and altering only debug rendering order/attachment (no actor-visible behavior change; a changed simulation fingerprint is a quarantine failure).

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-07 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Re-auditing EPI-CERT debug-capability quarantine (consumed from the predecessor pass per spec §11).
- Production remediation of any quarantine/leakage defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- The §6.4 generated/metamorphic harness (`-014`) and the aggregate verdict / fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-tui --test adversarial_gates` pass; debug disabled/enabled/read/not-read yields semantically identical event log, state, projection, packet, decision, proposal, metrics, and embodied output.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-tui --test tui_seam_conformance` pass; `debug_omniscience_excluded_001` / `debug_attach_001` show debug truth never entering an admitted source class and feedback attempts failing closed.
3. `cargo test --locked -p tracewake-core --test acceptance_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; `planner_trace_001` traces carry the complete candidate/method census with rejection reasons, budget, blocker, and responsible layer, and an incomplete trace is rejected.

### Invariants

1. Debug quarantine: enabling or reading planner/decision traces changes no cognition, event, state, proposal, metric, or embodied output; a changed simulation fingerprint under debug toggling is a failure.
2. No trace→cognition feedback: trace/debug data never enters the actor-known packet, candidate generation, planning, proposal, metrics, embodied view, or future memory.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only / non-diegetic.`

### Commands

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test adversarial_gates`
6. `cargo test --locked -p tracewake-tui --test tui_seam_conformance`

## Outcome

Completed: 2026-06-20

Recorded ORD-LIFE-07 as a certifying pass in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with evidence rows `0042-ORD07-TRACE-COMPLETE`, `0042-ORD07-DEBUG-QUARANTINE`, and `0042-ORD07-FEEDBACK-NEGATIVES`.

The evidence records planner/decision trace completeness, debug-on/read non-interference for TUI and core debug surfaces, and live negatives proving debug truth and debug command strings do not enter actor-known cognition or embodied actions. EPI-CERT debug-capability authority was consumed per the ticket and spec, not re-audited. No ORD-LIFE-07 member was deferred or dropped.

Verification commands run and passed:

1. `cargo test --locked -p tracewake-core --test no_human_capstone`
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates`
3. `cargo test --locked -p tracewake-core --test acceptance_gates`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-tui --test adversarial_gates`
6. `cargo test --locked -p tracewake-tui --test tui_seam_conformance`
