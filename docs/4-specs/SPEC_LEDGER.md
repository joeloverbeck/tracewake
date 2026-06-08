# Spec Ledger

Status document for Tracewake implementation specifications. This file is replacement content produced by **Spec 0008 — Phase 3A Anti-Contamination Hardening**.

This ledger is deliberately conservative. A spec is not accepted as architectural readiness merely because tests pass or scaffolding exists. A phase exits only when its acceptance gates prove the intended architecture, including hidden-truth boundaries, replayability, diagnostic structure, and no-human behavior where applicable.

## Source-discipline note

Any audit or hardening spec that targets a repository commit must include an exact-commit evidence ledger. Manifests may be used as path inventory only. Branch names, default-branch lookups, repository metadata, connector namespace labels, and code search snippets are not proof of target-commit content.

## Current readiness status

**Phase 3A is ready as of the Spec 0008 acceptance gates.**
**Phase 3B and Phase 4 may proceed only from the Spec 0008-hardened substrate, not from earlier audited commits.**

At target commit `8e3cf3eccb94372b7873846ae952441fc1ca44d0`, Phase 3A contained useful scaffolding but was not safe to build on as completed architecture. Spec 0008 hardening closed the required Phase 3A exit gates with canonical actor-decision transactions, sealed actor-known planning, typed diagnostics/replay, adversarial fixtures, anti-regression guards, and the integrated no-human capstone.

## Ledger entries

| Spec | Title | Ledger status | Notes |
|---|---|---|---|
| 0001 | Foundational documentation and early scope specs | Archived / foundation reference | Historical foundation material. Current authority remains the docs hierarchy and exact hardening specs. |
| 0002 | Phase 1 kernel, TUI, event log, replay implementation | Archived | Phase 1 historical implementation spec. Not reassessed here except where replay doctrine applies. |
| 0003 | Phase 1A executable TUI command loop and doc alignment | Archived | Historical TUI execution spec. |
| 0004 | Phase 2A epistemic substrate, expectation contradiction, possession parity | Archived / dependency for Phase 3A | Phase 3A autonomy must use Phase 2 epistemic/belief/projection surfaces rather than bypass them. |
| 0005 | Phase 3A needs, routines, and no-human day implementation | Superseded intent spec; readiness overclaim corrected | Correctly defines the intended causal chain: beliefs/needs/duties -> candidate goals -> durable intention -> HTN routine method -> actor-known local planning -> shared pipeline -> event/replay/debug. It did not prove the final architecture at later commits. |
| 0006 | Phase 3A needs, routines, and no-human day hardening | Superseded hardening pass | Correctly identified early Phase 3A defects: wait-only runner, incomplete live AgentState, string HTN/diagnostics, hidden-truth self-attestation, inert intentions/routines, weak tests, and status overclaims. Later code still requires stronger hardening. |
| 0007 | Phase 3A second hardening / no-human ordinary life proof | Partial hardening; readiness claim corrected | Landed useful scaffolding and tests, but at commit `8e3cf3eccb94372b7873846ae952441fc1ca44d0` the code still resembles multiple shortcuts that 0007 itself forbade. It is not sufficient as Phase 3A exit proof. |
| 0008 | Phase 3A Anti-Contamination Hardening | **Accepted / Phase 3A ready** | Established canonical actor-decision transaction, sealed provenance-rich actor-known context, typed diagnostics/replay, authoritative pipeline validation state, adversarial fixtures, anti-regression gates, and integrated no-human typed-ancestry replay capstone. This is the Phase 3A readiness authority for Phase 3B/Phase 4 dependency decisions. |

## Phase 3A readiness rule

Phase 3A may be marked ready only when all of the following are true:

1. No-human ordinary actions are produced through one canonical actor-decision transaction.
2. Actor-known planning context is sealed and provenance-rich.
3. The no-human path cannot construct actor-known facts from raw physical truth except through explicit visible/perception interfaces.
4. Needs, routines, and projects drive candidate/intention arbitration; they do not directly dispatch primitive actions.
5. Intentions are durable live commitments that drive future proposals.
6. Routine executions are eventful, stateful, defeasible, interruptible, resumable, completable, and explainable.
7. All world-affecting ordinary actions pass through the shared pipeline.
8. Validators read authoritative state from context, not proposal-echo parameters.
9. Decision traces and stuck diagnostics are typed authoritative state and replay-reconstructible.
10. Debug/TUI views derive from typed state and distinguish actor-known uncertainty from physical validation failure.
11. Adversarial fixtures prove hidden truth cannot leak into planning.
12. Static/anti-regression tests fail on the known shortcut classes.
13. Replay validates physical, epistemic, agent, intention, routine, diagnostic, and no-human metric state.

## Post-acceptance status policy

Do not apply any of the following claims to commits before Spec 0008 gates passed:

- “Phase 3A is complete.”
- “No-human ordinary life is architecturally proven.”
- “Actor-known planning is safe by construction.”
- “Spec 0007 closed Phase 3A.”
- “Phase 4 can build on the ordinary-life substrate.”

Acceptable wording after Spec 0008 acceptance:

- “Phase 3A readiness is accepted by Spec 0008 gates.”
- “No-human ordinary-life proof is accepted only on the Spec 0008-hardened substrate.”
- “Phase 3B/Phase 4 may build on the Spec 0008 Phase 3A substrate while keeping their own acceptance gates explicit.”

## Replacement note

This replacement ledger supersedes any previous ledger entry that marked Spec 0007 as sufficient Phase 3A ordinary-life readiness. Spec 0007 remains valuable historical work, but it is not the final Phase 3A foundation gate.
