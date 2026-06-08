# 0008PHA3AANTCON-011: Capstone — integrated no-human ancestry gate + ledger "ready" flip

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test crate: integrated no-human capstone gate; docs: `SPEC_LEDGER.md`/status-errata flip from "still hardening" to "Phase 3A ready"
**Deps**: 0008PHA3AANTCON-007, 0008PHA3AANTCON-008, 0008PHA3AANTCON-009, 0008PHA3AANTCON-010

## Problem

Spec 0008 §10.1 + §13: the capstone must still run `run_no_human_day` once and now prove typed causal ancestry — `NoHumanWindow` trigger → `DecisionTraceRecord` → `ActorKnownPlanningContext` proof graph → candidate ids → selected intention lifecycle event → HTN method id → local plan id → proposal id → shared-pipeline accepted/rejected event ids → routine/need effects → replay-rebuilt typed records. The test must fail if a decision-trace record is inserted after an unrelated proposal, if a proposal origin lacks ancestry, or if typed trace fields do not match event IDs. This is the verification-only capstone that exercises every prior ticket end-to-end; on passing, it flips the ledger/status from "still hardening" to "Phase 3A ready" (per spec §17 item 10, as updated in reassessment).

## Assumption Reassessment (2026-06-08)

1. `run_no_human_day` (`scheduler.rs:289`) exists; capstone hosts alongside `crates/tracewake-core/tests/acceptance_gates.rs` / `golden_scenarios.rs`. `docs/4-specs/SPEC_LEDGER.md` currently reads "Phase 3A is still in hardening" / "Phase 3B and Phase 4 are blocked until Spec 0008 gates pass" (landed in commit `24515f8`); `archive/reports/PHASE_3A_STATUS_ERRATA.md` carries the matching status.
2. Spec §10.1 fixes the ancestry chain to prove; §13 is the full acceptance checklist (21 lines) this capstone enumerates as sub-cases. Spec §17 item 10 (reassessment-updated) assigns the "still hardering"→"Phase 3A ready" flip to "once the §13 acceptance gates pass" — i.e. this capstone.
3. Cross-artifact boundary under audit: the **full no-human cognition pipeline** composed by -001..-010 — this ticket introduces no production logic; it exercises the composed pipeline end-to-end and gates the ledger flip on it.
4. INV-004 (the authoritative world ignores human existence), INV-018 (deterministic replay is foundational), INV-098 (feature acceptance is harsh — done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, regression-tested): the capstone is the harsh-acceptance gate for Phase 3A exit.
5. Enforcement surface: no-human run + deterministic replay + no-leak, asserted together. Confirm the capstone re-enumerates expected counts from the fixture at test start (no hardcoded counts), asserts typed-record/event-ID ancestry (not labels), and verifies replay rebuilds the typed records. The ledger flip is gated strictly on the gate passing — no status change ships if any sub-case fails.
6. (Not applicable — capstone introduces no schema.)
7. Modifies the doc-governed status contract: flips `SPEC_LEDGER.md` and `PHASE_3A_STATUS_ERRATA.md` from "still hardening / 3B-3C blocked" to "Phase 3A ready". Blast-radius grep at implementation: `grep -rn "still in hardening\|still hardening\|blocked until Spec 0008" docs/ archive/ specs/` — update every status line that asserts the pre-exit posture; leave the historical audit narrative (commit-`8e3cf3e` findings) intact as history.

## Architecture Check

1. A single verification-only capstone that exercises the composed pipeline is the correct acceptance shape: it introduces no production logic, so a failure localizes to an upstream ticket, and it is the one place the §13 checklist is proven end-to-end. Gating the ledger flip on it prevents the readiness-overclaim that §12/Finding 12 corrected.
2. No backwards-compatibility aliasing: the capstone adds test + docs only; no production shim.

## Verification Layers

1. INV-004 no-human run → replay/golden-fixture check: `run_no_human_day` once produces ordinary life; the run advances through the transaction.
2. INV-018 deterministic replay → replay check: replay rebuilds the typed `DecisionTraceRecord`/`StuckDiagnosticRecord` and no-human metrics; live == replay.
3. INV-098 harsh acceptance → manual review + test: each §13 checklist line maps to a capstone sub-case or a cited upstream ticket's gate; ancestry asserted by typed record/event-ID linkage, not labels.
4. Ledger-flip gating → codebase grep-proof: `SPEC_LEDGER.md` reads "Phase 3A ready" only after the gate passes; pre-exit status lines updated; historical audit narrative retained.

## What to Change

### 1. Integrated no-human capstone gate

New test exercising `run_no_human_day` once and asserting the §10.1 ancestry chain by typed record/event-ID linkage; fails on post-hoc trace insertion, missing proposal ancestry, or field/event-ID mismatch. Re-enumerate expected counts from the fixture at test start.

### 2. §13 checklist enumeration

Enumerate the §13 acceptance lines as capstone sub-cases or as references to the upstream ticket gates that cover them (no silent omission).

### 3. Ledger / status "ready" flip

Once the gate passes, update `docs/4-specs/SPEC_LEDGER.md` and `archive/reports/PHASE_3A_STATUS_ERRATA.md` from "still hardening / Phase 3B-4 blocked" to "Phase 3A ready"; retain the historical audit narrative.

## Files to Touch

- `crates/tracewake-core/tests/no_human_capstone.rs` (new — integrated ancestry gate)
- `docs/4-specs/SPEC_LEDGER.md` (modify — status flip on gate pass)
- `archive/reports/PHASE_3A_STATUS_ERRATA.md` (modify — status flip on gate pass)

## Out of Scope

- Any production logic (owned by -001..-006).
- The adversarial / anti-regression / content gates themselves (-007/-008/-009) — exercised, not modified.
- Debug/TUI rendering (0008PHA3AANTCON-010) — exercised, not modified.

## Acceptance Criteria

### Tests That Must Pass

1. Capstone proves the §10.1 ancestry chain by typed record/event-ID linkage; fails on post-hoc trace insertion, ancestry-less proposal origin, or field/event-ID mismatch.
2. Replay rebuilds typed records + no-human metrics; live == replay.
3. Every §13 checklist line maps to a capstone sub-case or a cited upstream gate.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. The capstone introduces no production logic; it exercises the composed pipeline.
2. The ledger "Phase 3A ready" flip ships only after the gate passes.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/no_human_capstone.rs` — integrated typed-ancestry gate over a single `run_no_human_day`.

### Commands

1. `cargo test -p tracewake-core --test no_human_capstone`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
