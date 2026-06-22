# 0047TUIAUTWOR-003: Execution amendments (05/06/07/10)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — docs only: `docs/2-execution/{05,06,07,10}_*.md`
**Deps**: None

## Problem

Spec 0047 §5 routes execution-tier amendments (substance + home) operationalizing the human-authoritative-world-advance capability. Per the driver report dispositions D-10…D-13: execution `05` must specify the canonical step's ownership, typed request/result, deterministic phase contract, event route, due-duration scan, actor-transaction cadence, and no-direct-dispatch guard; execution `06` must extend ordinary-life proof to human-driven ticks; execution `07` must **correct** its current staging sentence that says embodied time controls remain unavailable outside debug (superseded by this feature) while preserving the debug no-human-day distinction; execution `10` must add the command/evidence matrix. These approved execution semantics must exist before code lands (decomposition ticket 1, execution portion).

## Assumption Reassessment (2026-06-22)

1. All four amendment targets exist at their cited tier/prefix: `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`, `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (all confirmed present).
2. Spec 0047 §5 enumerates the per-doc substance and home; §2 lists the same docs as the governing execution anchors and notes the post-`FIRST-PROOF-CERT` baseline (execution `02`/`03`). This ticket carries the execution tier only; foundation/architecture/reference are 0047TUIAUTWOR-001/002/004.
3. Cross-artifact boundary under audit: the execution tier operationalizes architecture `04`/`05`/`10` (amended in 0047TUIAUTWOR-002) — the canonical step's concrete ownership and phase contract in execution `05` must not contradict the architecture-tier coordinator standard, and execution `07`'s corrected staging posture must stay consistent with foundation `08`'s amended Time-controls doctrine (0047TUIAUTWOR-001).
4. Constitutional invariant motivating the amendment: `INV-091` (no-human tests are mandatory — execution `06`'s human/no-human equivalence and the differential witness keep the no-human path authoritative) and `INV-103` (the scheduler is not cognition authority — execution `05`'s no-direct-dispatch guard).
5. Enforcement surfaces governed by doctrine (substrate-only basis): the no-direct-dispatch / single-owner event route (`INV-103`/`INV-104`; execution `05`), single-charge need accounting and duration-regime classification (`INV-045`; execution `06`), and the actor-known stop-reason / interval-summary anti-leak boundary plus structurally-separate debug diagnostics (`INV-024`/`INV-067`/`INV-068`; execution `07`/`10`). The amendments must preserve the debug no-human-day quarantine while adding embodied proof, introducing no path by which debug-only exact details reach embodied rendering/planning.
6. Mismatch + correction: execution `07` currently carries a staging sentence asserting embodied time controls remain unavailable outside debug. This is now false under spec 0047. The required correction (§5) does not delete the debug no-human-day distinction — it preserves the debug quarantine and replaces only the "not exposed outside debug" posture, adding embodied one-tick/advance-until proof, actor-known stop reasons, interval-summary anti-leak rows, and structurally separate debug diagnostics. Classify: a required consequence of this feature, not a separate bug.

## Architecture Check

1. Amending each execution doc in its established home (transaction/scheduler → `05`, ordinary-life/no-human → `06`, epistemic/debug proof → `07`, testing/evidence → `10`) keeps execution gates where the phase ladder already points, rather than creating a parallel time-control proof doc that would split the gate authority.
2. No backwards-compatibility aliasing/shims: execution `07`'s stale "unavailable outside debug" posture is corrected, not shadowed by a parallel statement; the debug-quarantine distinction is retained explicitly.

## Verification Layers

1. `INV-103`/`INV-104` no-direct-dispatch -> invariants alignment check: execution `05` amendment specifies the single event-append/application route and no-direct-dispatch guard.
2. `INV-091` no-human mandatory -> invariants alignment check: execution `06` amendment extends single-charge / duration-regime / open-duration completion proof to human-driven ticks with a human/no-human equivalence witness.
3. `INV-024`/`INV-068` debug quarantine -> manual review (epistemic-leakage audit): execution `07` corrected posture preserves the debug no-human-day distinction and adds interval-summary anti-leak rows with structurally separate debug diagnostics.
4. Correction landing -> codebase grep-proof: the stale "unavailable outside debug" posture no longer reads as a live constraint in execution `07`; the evidence matrix lands in execution `10`.

## What to Change

### 1. Execution `05` — transaction/scheduler/pipeline

Specify the canonical step's concrete ownership, typed request/result, deterministic phase contract, event append/application route, due-duration scan, actor-transaction cadence, and the no-direct-dispatch guard.

### 2. Execution `06` — ordinary-life / no-human proof

Extend ordinary-life proof to human-driven ticks: existing wait-charge evidence, passive suppression, duration-regime classification, open-duration completion, and human/no-human equivalence.

### 3. Execution `07` — epistemic view models / debug proof (correction)

Correct the staging sentence that says embodied time controls remain unavailable outside debug: preserve the debug no-human-day distinction, but replace the "not exposed outside debug" posture once this feature is accepted; add embodied one-tick / advance-until proof, actor-known stop reasons, interval-summary anti-leak rows, and structurally separate debug diagnostics.

### 4. Execution `10` — testing / observability / evidence

Add: command transcripts, typed event ledgers, checksums, replay reports, hidden-truth negative witnesses, and parity evidence expected from implementation and acceptance.

### 5. Execution precondition (recorded, not auto-applied)

Execution-tier amendments require ordinary execution-owner approval before application (`docs/README.md`). The ticket records substance + home; it is not merged by convention.

## Files to Touch

- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- Any code change — owned by 0047TUIAUTWOR-005 onward.
- Foundation `08` (0047TUIAUTWOR-001), architecture (0047TUIAUTWOR-002), reference (0047TUIAUTWOR-004).
- Promoting the debug no-human-day runner into gameplay (§1.2 non-goal) — execution `07` keeps the quarantine; only the "unavailable outside debug" posture is corrected.
- Specifying Rust type names, function signatures, or test names (substance + home only).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "canonical step|no-direct-dispatch|phase" docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` shows the canonical-step ownership and no-direct-dispatch guard landed.
2. `grep -niE "human|equivalence|single charge|duration" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` shows human-driven tick proof + human/no-human equivalence.
3. `grep -niE "advance-until|interval summary|anti-leak|debug" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` shows the corrected posture (embodied controls proven, debug quarantine retained).

### Invariants

1. Execution `07` no longer asserts embodied time controls are unavailable outside debug as a live constraint, while still naming the debug no-human-day distinction (`INV-068` quarantine preserved).
2. No amended section permits a TUI-owned simulation rule, direct event application, or debug-runner reuse for gameplay (`INV-069`/`INV-103`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep landing + invariants-alignment review); the governed enforcement surfaces are implemented/tested by 0047TUIAUTWOR-005/008/010/011/012/013/016/017.`

### Commands

1. `for d in 05 06 07 10; do echo "== $d =="; grep -ciE "canonical step|equivalence|advance-until|anti-leak|transcript|no-direct-dispatch" docs/2-execution/${d}_*.md; done`
2. `grep -niE "outside debug|unavailable" docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
3. Narrower command is correct: execution doc amendments are verified by landing greps + a manual `INV-091`/`INV-103`/`INV-024` alignment read (and confirming the corrected `07` posture), not by `cargo` tests — no code changes in this ticket.
