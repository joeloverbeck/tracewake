# 0032ARCTIETEM-012: Cross-cutting budget-exhaustion & fairness/starvation seam (A04/A05/A12/A13)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`, `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`, and `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (one consolidated deterministic-performance / fairness-budget seam). No crate/code, no fixtures.
**Deps**: 0032ARCTIETEM-003 (A04 — budget hooks layer on the scheduler temporal firewall), 0032ARCTIETEM-007 (A12 — time-acceleration fidelity layers on LOD temporal summaries), 0032ARCTIETEM-010 (A05 — budget planner-degradation layers on the affect/learning seam; A05 hub chain), 0032ARCTIETEM-011 (A13 — fairness/starvation artifacts layer on temporal observability). **Execution-blocking precondition**: owner approval per spec 0032 §R-A.

## Problem

Spec 0032 D-R5 (report R5). Foundation `10` says human focus is not privilege and LOD/regional systems must preserve causality and fairness; foundation `14` says scheduler authority is not cognition authority. The completeness determination routes deterministic-performance / fairness budgets across A04/A05/A12/A13. Each doc has the substrate (A04 scheduler limits, A05 decision transactions, A12 LOD/human-focus fairness, A13 observability) but **none represents budget exhaustion and fairness/starvation as a named seam**. Spec Risk R-D requires this be **consolidated, not triplicated** across the temporal tickets — this ticket owns the cross-cutting seam; tickets 003 (A04) and 011 (A13) carry only temporal-specific hooks that point here.

## Assumption Reassessment (2026-06-14)

1. Verified against the live tree (`ea6a05b`): A04 owns scheduler limits + deterministic pipeline; A05 owns actor decision transactions; A12 owns LOD tiers, regional processes, human-focus fairness; A13 owns observability — but none represents budget exhaustion or fairness/starvation evidence. This ticket adds the consolidated seam; it does not reinvent the scheduler/LOD/observability contracts.
2. Verified against spec 0032 §3.2 D-R5 and source report §5 Finding R5 / foundation `10`,`14` / `INV-087`. D-R5's homes are A04/A05/A12/A13. Additive; relaxes nothing. Numeric budgets, scheduling algorithms, fairness formulas, queues, and thresholds route to execution per spec §6.
3. Shared boundary under audit: the scheduler/cognition/LOD/observability fairness contract spanning four docs. This ticket is the **last in every shared-doc chain**: A04 (after 003), A05 (after 004→010), A12 (after 007), A13 (after 011) — its `Deps` enforce that ordering so each per-surface budget statement layers onto the temporal contract already present. **Consolidation rationale (spec Risk R-D)**: budget-exhaustion appears as a hook in D-T4 (003) and D-T10 (011); this ticket owns the single consolidated statement so the contract is not triplicated into three divergent versions.
4. Constitutional invariants motivating this ticket, restated: `INV-087` — human focus is not player privilege (the fairness/starvation artifacts must prove no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing); `INV-105` — decision/scheduler diagnostics must be typed/structurally inspectable (budget outcomes are typed). The scheduler-is-not-cognition-authority principle (foundation `14`) bounds budget-driven degradation.
5. Fairness / no-leak surface (governed here; enforcement deferred to execution `05`/`10`/`12`): A04 — scheduler budgets/ordering are deterministic and diagnosed; budget-limited outcomes are typed deferred/skipped/summarized/degraded/blocked with responsible layer + replay ancestry. A05 — bounded planning may fail/degrade for budget reasons only through typed decision diagnostics, never omniscient shortcuts or marker-action substitution. A12 — LOD/time-acceleration is a declared fidelity mode with fairness constraints (lower detail may summarize, not erase active claims/procedures/leads/obligations or starvation from persistent under-scheduling). A13 — fairness/starvation review artifacts record which holders/processes were deferred, why, for how long / across what source interval, and evidence of no human-proximity/possessed-actor priority bias unless explicitly non-diegetic input routing. Adds doctrine only; deterministic, no leakage path, fairness artifacts observer-only.

## Architecture Check

1. One consolidated ticket (not four per-doc additions) is correct because spec Risk R-D requires a single budget/fairness contract referenced from the temporal tickets rather than triplicated statements; the four per-surface paragraphs are one coherent seam reviewed together, with A04/A13 cross-referencing this ticket from their temporal hooks (003/011).
2. No backwards-compatibility aliasing/shims: additive `what`-level doctrine; an outcome taxonomy and fairness-evidence shape, not algorithms or numeric budgets.

## Verification Layers

1. `INV-087` fairness/no-human-privilege (D-R5) → invariants alignment check: A12/A13 require LOD fairness constraints and fairness/starvation artifacts proving no human-proximity/possessed-actor priority bias unless explicitly non-diegetic input routing.
2. `INV-105` typed budget outcomes → invariants alignment check: A04/A05 require budget-limited outcomes to be typed (deferred/skipped/summarized/degraded/blocked) with responsible layer + replay ancestry, never omniscient shortcuts or marker-action substitution.
3. Consolidation check (spec Risk R-D) → manual review: this ticket owns the single budget/fairness statement; tickets 003 (A04) and 011 (A13) only hook/cross-reference it, with no divergent restatement.
4. Four docs, additive: no replay/golden-fixture or skill-dry-run layer applies (deferred execution `05`/`10`/`12`); the layers above map each engaged invariant to a distinct alignment proof.

## What to Change

### 1. D-R5 — deterministic-performance / fairness-budget seam (consolidated)

Add the consolidated seam across the four docs:

- **A04**: scheduler budgets and ordering policies are deterministic and diagnosed; when budget limits prevent full cognition/procedure execution, the outcome is typed as deferred, skipped, summarized, degraded, or blocked, with responsible layer and replay ancestry. (Cross-referenced from ticket 003's budget-exhaustion hook.)
- **A05**: bounded planning may fail/degrade for budget reasons only through typed decision diagnostics; it must not silently choose omniscient shortcuts or substitute marker actions as progress.
- **A12**: LOD/time acceleration is a declared fidelity mode with fairness constraints — lower detail may summarize but not erase active claims, procedures, leads, obligations, or starvation caused by persistent under-scheduling.
- **A13**: fairness/starvation review artifacts record which holders/processes were deferred, why, for how long or across what source interval, and what evidence proves no human-proximity or possessed-actor priority bias unless explicitly non-diegetic input routing. (Cross-referenced from ticket 011's temporal-observability families.)

Do not choose numeric budgets, scheduling algorithms, fairness formulas, queues, or acceptance thresholds.

## Files to Touch

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` (modify)
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` (modify)
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` (modify)
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` (modify)

## Out of Scope

- **Numeric budgets, scheduling algorithms, fairness formulas, queues, acceptance thresholds, sampling/starvation policy** — execution/implementation (spec §6).
- **A04 scheduler temporal firewall / A13 temporal observability / A12 LOD temporal summaries** — sibling tickets 003 / 011 / 007 (D-T4 / D-T10 / D-T9); this ticket adds only the budget/fairness seam onto them.
- **Budget-exhaustion / fairness/starvation proof procedures, time-acceleration review packets** — execution `05`/`10`/`12` (spec §6, V4).
- **Owner approval itself (spec §R-A)** — execution precondition, not deliverable.
- Crate/code, fixtures, foundation/execution/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **D-R5 landing greps** — each of the four docs carries its budget/fairness paragraph: `grep -niE "budget|deferred|skipped|degraded|fairness|starvation" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` resolves in all four.
2. **Consolidation check** — A04 and A13 budget text cross-references this seam rather than restating a divergent contract (spec Risk R-D).
3. **Invariants alignment review** — upholds `INV-087` (no human-focus privilege; fairness artifacts prove it) and `INV-105` (typed budget outcomes); no numeric budget/algorithm/threshold token introduced.

### Invariants

1. Budget-limited outcomes are typed (deferred/skipped/summarized/degraded/blocked) with responsible layer + replay ancestry; bounded planning never substitutes omniscient shortcuts or marker actions (`INV-105`).
2. LOD/time-acceleration fidelity may not erase active claims/procedures/leads/obligations or starvation; fairness/starvation artifacts prove no human-proximity/possessed-actor priority bias unless explicitly non-diegetic routing (`INV-087`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only architecture-doctrine ticket; verification is command-based (four-doc landing grep) plus an invariants-alignment manual review and a consolidation cross-reference check. No crate/code or fixture changes.`

### Commands

1. `grep -niE "budget|deferred|skipped|degraded|fairness|starvation" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — confirms D-R5 landed in all four docs.
2. `Documentation-only: the Rust pipeline is unaffected; the verification boundary is the four-doc landing grep plus the invariants-alignment review and the Risk-R-D consolidation check.`

## Outcome

Completed: 2026-06-15

Implemented D-R5 across `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`, `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`, and `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`. A04 now states deterministic, diagnosed scheduler budgets and typed deferred/skipped/summarized/degraded/blocked outcomes with responsible layer and replay ancestry, while still pointing to D-R5 as the consolidated seam. A05 now states that budget-limited planning may fail/degrade only through typed diagnostics and cannot substitute omniscient shortcuts or marker actions. A12 now states that LOD/time acceleration fidelity may not erase active claims, procedures, leads, obligations, or starvation caused by persistent under-scheduling. A13 now defines fairness/starvation review artifact shape, including holders/processes affected, reason, source interval, responsible layer, replay ancestry, and evidence against human-proximity or possessed-actor priority bias unless explicitly non-diegetic and quarantined.

The execution-blocking owner-approval precondition from spec 0032 §R-A is satisfied by the user's explicit `$ticket-series implement the series tickets/0032ARCTIETEM*` request for this architecture-tier amendment series.

Verification:

- `grep -niE "budget|deferred|skipped|degraded|fairness|starvation" docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- Manual consolidation check: A04 keeps the D-R5 pointer and A13 carries the fairness/starvation artifact shape, avoiding a divergent restatement of the cross-cutting seam.
- Manual invariants alignment review: the additions preserve `INV-087` by requiring no human-proximity/possessed-actor priority bias evidence, and `INV-105` by requiring typed budget outcomes and diagnostics.
- Manual mechanism-token boundary review: no numeric budget, scheduling algorithm, fairness formula, queue, acceptance threshold, or sampling/starvation policy was introduced.

No crate/code or fixture changes were made for this documentation-only architecture ticket.
