# 0032ARCTIETEM-003: A04 scheduler trigger-versus-cognition temporal firewall

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — doctrine edit to `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (temporal-firewall language near scheduler limits + validation boundary; budget-exhaustion-as-typed-outcome note). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-001 (the scheduler orders the A02 authoritative event/replay time this contract quarantines from cognition). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-T4 (report T4). `INV-103` says the scheduler may choose actor/time windows and apply due scheduled effects but may not construct action proposals from raw state; `INV-112` specializes that for time, and foundation `14` states the scheduler may advance deterministic time or invoke transactions but cannot become a hidden-planner authority. A04 already states scheduler limits, direct-dispatch prohibitions, validation boundaries, reservations/durations, and single-charge accounting — but carries **no explicit temporal-firewall formulation**: verified 0 temporal / temporal-firewall / budget-exhaust matches in A04 at `ea6a05b`. Without it, "it is work time / deadline passed / office closed" can leak from the clock into a selected action.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A04 owns scheduler limits, no-direct-dispatch, validation boundary, reservations/durations, single-charge accounting, but no temporal-firewall language (`grep -niE "temporal|may not plan|budget exhaust"` → 0). Execution `05` already operationalizes no-direct-dispatch; this ticket adds the temporal specialization at architecture altitude, it does not restate execution mechanics.
2. Verified against spec 0032 §3.1 D-T4 and source report §5 Finding T4 / `INV-103` / `INV-112` / foundation `14`. D-T4's home is A04, near scheduler limits and the validation boundary. Additive; relaxes no existing A04 limit. Queue/algorithm choices route out per spec §6.
3. Shared boundary under audit: A04 (scheduler/validation) ↔ holder-known/institution-known context. **A04 is also touched by sibling ticket 012** (D-R5 budget/fairness) — this ticket lands first (012 `Deps` on it); the budget-exhaustion-as-typed-outcome note here is the temporal-specific hook, and 012 owns the consolidated cross-cutting fairness seam (spec Risk R-D: consolidate, do not triplicate). Coordinate the A04 merge.
4. Constitutional invariants motivating this ticket, restated: `INV-103` — the scheduler is not a cognition authority (may order windows / apply due effects / invoke the decision transaction, but may not construct proposals from raw state); `INV-112` — the scheduler/replay clock may order and validate, but holder-known time must plan. D-T4 states the temporal half of the no-direct-dispatch firewall.
5. Actor-knowledge / scheduler-not-cognition surface (governed here; enforcement deferred to execution `05`/`10`): the contract permits scheduler/replay time to order decision opportunities, detect due effects/duration terminals, invoke holder-known transaction construction, validate temporal legality, and emit typed temporal diagnostics — and forbids turning "work time / meal time / deadline passed / office closed / actor is late" into a selected action, route, target, institutional conclusion, or actor-visible reason absent a holder-known/institution-known premise, repairing plans with true lateness, or leaking exact future/due timing through actor-visible feedback unless a modeled channel exposes it. Budget exhaustion is a typed scheduling/decision outcome (defer/skip/summarize/diagnose via a deterministic policy execution later defines). Adds doctrine only; no leakage or nondeterminism path, and the budget policy is named as deterministic.

## Architecture Check

1. A04 is the correct home: it already owns the scheduler-limit / no-direct-dispatch seam, so the temporal firewall is a specialization of a contract A04 carries, reviewed coherently beside it. Splitting it elsewhere would separate the temporal trigger rule from the dispatch rule it specializes.
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; the budget-exhaustion note states an outcome taxonomy, not an algorithm.

## Verification Layers

1. `INV-103`/`INV-112` scheduler trigger-vs-plan (D-T4) → invariants alignment check: A04 states the may/may-not list separating ordering/validation/due-detection from selected actions, routes, conclusions, or actor-visible reasons drawn from raw clock truth.
2. `INV-112` budget-exhaustion-as-typed-outcome → invariants alignment check: A04 names budget exhaustion as a typed deferred/skipped/summarized/diagnosed outcome under a deterministic policy, cross-referencing the consolidated seam in sibling ticket 012.
3. Single doc, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `05`/`10`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-T4 — temporal firewall in A04 (near scheduler limits + validation boundary)

Add to `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`. Scheduler/replay time **may**: order decision opportunities and process windows; detect due effects and duration terminals; invoke holder-known transaction construction; validate whether a proposed action is temporally legal or whether a due/procedural consequence applies; emit typed temporal diagnostics. It **may not**: turn "work time / meal time / deadline passed / office closed / actor is late" into a selected action, route, target, institutional conclusion, or actor-visible reason unless that premise is in the relevant holder-known/institution-known context; repair plans using true lateness or hidden schedule truth; leak exact future/due timing through actor-visible feedback unless a modeled channel exposes it.

### 2. D-T4 — budget exhaustion as a typed scheduling/decision outcome

State that if a scheduler cannot process all due cognition, it must defer, skip, summarize, or diagnose through a **deterministic** policy that execution later defines. Cross-reference the consolidated budget/fairness seam (sibling ticket 012). Do not define the queue or algorithm.

## Files to Touch

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)

## Out of Scope

- **Queue/data structures, scheduling algorithm, budget thresholds, fairness formula** — execution/implementation (spec §6).
- **Consolidated budget-exhaustion / fairness/starvation seam across A05/A12/A13** — sibling ticket 012 (D-R5); this ticket states only the A04 temporal-specific hook.
- **No-direct-dispatch proof procedures** — execution `05`/`10` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-T4 landing grep** — A04 carries the temporal firewall: `grep -niE "temporal|may not|budget exhaust|deferred|skipped" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` resolves the may/may-not list and the budget-exhaustion outcome.
2. **Cross-reference check** — the budget-exhaustion note points to the consolidated seam (ticket 012 / D-R5) rather than restating a full fairness contract.
3. **Invariants alignment review** — the firewall upholds `INV-103` (scheduler not cognition authority) and `INV-112` (validate/order, not plan); no algorithm/threshold token introduced.

### Invariants

1. Scheduler/replay time may order/validate/detect-due/invoke but may not turn raw clock truth into a selected action, route, target, conclusion, or actor-visible reason absent a holder-known premise (`INV-103`/`INV-112`).
2. Budget exhaustion is a typed deferred/skipped/summarized/diagnosed outcome under a deterministic policy (`INV-112`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (landing grep) plus an invariants-alignment manual review and a cross-reference check to ticket 012. No crate/code or fixture changes.`

### Commands

1. `grep -niE "temporal|may not|budget exhaust|deferred|skipped" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — confirms D-T4 landed.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the landing grep plus the invariants-alignment review.`
