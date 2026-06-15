# 0033EXETIETEM-002: exec 05 scheduler temporal authority & budget evidence

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (additive scheduler temporal-authority statement and deterministic budget evidence over the existing `PIPE`/`NO-DIRECT` posture). No crate/code, no fixtures.
**Deps**: 0033EXETIETEM-010 (the consolidated budget/fairness diagnostics home in exec `10`; the budget evidence here cross-references it per spec §R-D). **Execution-blocking precondition**: owner approval per spec 0033 §R-A (execution tier-2 doctrine; not by convention).

## Problem

Spec 0033 D-T2, D-R3 (report F-02, F-10), the exec-`05` slices. Exec `05` owns transaction scheduling, the action pipeline, and no-direct-dispatch (`PIPE`/`NO-DIRECT`) but carries **no** temporal-authority statement or deterministic budget/fairness evidence: verified 0 `temporal`/`INV-112` matches in exec `05` at `c70d119`. Without an execution-altitude statement of the scheduler's temporal authority, an implementation can let scheduler/replay time select intentions or rewrite wait causes; without budget evidence, budget pressure becomes an invisible director choosing outcomes. This ticket states the scheduler's temporal authority and the scheduler-side budget evidence, cross-referencing exec `10` as the single consolidated budget/fairness home.

## Assumption Reassessment (2026-06-15)

1. Verified against the live tree (`c70d119`): exec `05` (`05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`) owns the scheduler/pipeline/no-direct-dispatch boundary and carries no temporal-authority or budget/fairness language (`grep -ciE 'temporal|INV-112' docs/2-execution/05_*` → 0). The gap is real; this ticket adds the statement and evidence obligation, not a new mechanism.
2. Verified against spec 0033 §3.1 D-T2 and §3.2 D-R3, and the ratified upstream architecture A04 scheduler temporal firewall + budget-exhaustion seam (spec `0032`). Scheduler queue/algorithm, fairness formula/window, and numeric budgets route to future scoped specs per spec §6.
3. Shared boundary under audit: exec `05` (scheduler temporal authority + budget evidence) ↔ exec `10` (consolidated budget/fairness diagnostics, ticket 010) ↔ exec `12` (deferred-scale fairness, ticket 008). Per spec §R-D the budget/fairness contract is stated **once** in exec `10`; this ticket states the scheduler-side evidence and cross-references `10` — it must not restate (triplicate) the fairness contract.
4. Constitutional invariants motivating this ticket, restated: `INV-103` (the scheduler is not a cognition authority — it may choose the next actor/window, apply due effects, invoke the transaction, but not construct proposals from raw state/true time); `INV-105` (decision/scheduler diagnostics are authoritative typed data, not display strings). The temporal-authority statement specializes `INV-103` for time.
5. Deterministic-replay / fail-closed surface: the scheduler-budget evidence requires deterministic candidate ordering, budget exhaustion, deferred/skipped cognition, and no-direct-dispatch behavior under load to be replayable and typed; budget pressure may not become an invisible director choosing outcomes without typed evidence. This strengthens determinism (`INV-105`/`INV-018` lineage) and introduces no nondeterministic input. The cross-cutting fairness/starvation diagnostics with responsible layers live in exec `10` (ticket 010); this doc states the scheduler-side obligation only.

## Architecture Check

1. Exec `05` is the correct home for the scheduler's execution-altitude temporal authority and the scheduler-side budget evidence: it already owns the scheduler/pipeline boundary, so this is a specialization, not a new gate. Stating the cross-cutting fairness contract here instead of in exec `10` would triplicate it (spec §R-D) — hence the cross-reference.
2. No backwards-compatibility aliasing/shims: additive statement + evidence obligation over the existing `PIPE`/`NO-DIRECT` posture; no rename, no weakening, no scheduler algorithm chosen.

## Verification Layers

1. `INV-103` scheduler temporal authority → invariants alignment check: exec `05` states the scheduler may awaken candidates, order transactions, validate preconditions/effects, and account for budget exhaustion, but may not select intentions, invent reasons, rewrite wait causes, or conclude routines by consulting true time.
2. `INV-105` deterministic budget evidence → invariants alignment check + codebase grep-proof: exec `05` requires typed, replayable evidence for candidate ordering, budget exhaustion, deferred/skipped cognition, and no-direct-dispatch under load, cross-referencing the consolidated exec `10` home (no triplication).
3. Documentation-only doctrine ticket: replay/golden-fixture and skill-dry-run layers are the exec `09`/`10` proof surfaces (tickets 006/010); the layers above map each engaged invariant to a distinct surface.

## What to Change

### 1. D-T2 — scheduler temporal-authority statement

Add to exec `05` the scheduler's execution-altitude temporal authority: it **may** awaken candidates, order transactions, validate preconditions/effects, and account for budget exhaustion; it **may not** select intentions, invent reasons, rewrite wait causes, or make routine conclusions by consulting true time alone. State it as a gate obligation over the existing pipeline; choose no queue or algorithm.

### 2. D-R3 — deterministic scheduler-budget evidence (cross-referencing the consolidated home)

Require scheduler-budget evidence for deterministic candidate ordering, budget exhaustion, deferred/skipped cognition, and no-direct-dispatch behavior under load, stating that budget pressure may not become an invisible director choosing outcomes without typed evidence. Cross-reference exec `10` as the single consolidated budget/fairness diagnostics home (spec §R-D); do not choose fairness formulas, windows, budgets, or thresholds (§6).

## Files to Touch

- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify)

## Out of Scope

- **The consolidated budget/fairness diagnostics contract** — exec `10` (ticket 010); this ticket cross-references it, per spec §R-D (no triplication).
- **Deferred-scale fairness declarations** — exec `12` (ticket 008).
- **Scheduler queue/algorithm, fairness formula/window, numeric budgets, thresholds** — future scoped specs (§6).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits; minting any new gate code.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T2 landing grep** — exec `05` carries the scheduler temporal-authority statement: `grep -niE 'temporal authority|true time' docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` resolves it.
2. **D-R3 landing + consolidation grep** — scheduler-budget evidence present and cross-references the exec `10` home: `grep -niE 'budget|fairness|invisible director' docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` resolves the evidence and a pointer to exec `10`.
3. **Mechanism-token boundary review** — manual review confirms no fairness formula, window size, numeric budget, scheduler algorithm, or threshold entered exec `05`; no new gate code.
4. **Invariants alignment review** — upholds `INV-103`/`INV-105` and preserves `PIPE`/`NO-DIRECT` (no rename/weaken); budget/fairness stated once (in `10`), referenced here.

### Invariants

1. The scheduler orders/validates/awakens/accounts-for-budget but never selects intentions or concludes routines from true time (`INV-103`).
2. Budget pressure produces typed, replayable evidence, never silent outcome direction; the cross-cutting contract lives once in exec `10` (`INV-105`, spec §R-D).

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (landing greps) plus a mechanism-token-boundary and invariants-alignment manual review. No crate/code or fixture changes.`

### Commands

1. `grep -niE 'temporal authority|true time|budget|fairness|invisible director' docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — confirms D-T2/D-R3 landed and the exec `10` cross-reference is present.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing greps plus the mechanism-token-boundary and invariants-alignment review.`
