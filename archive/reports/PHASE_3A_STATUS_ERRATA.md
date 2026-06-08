# Phase 3A Status Errata

Replacement content produced by **Spec 0008 — Phase 3A Anti-Contamination Hardening**.

Target repository: `joeloverbeck/tracewake`  
Target commit audited: `8e3cf3eccb94372b7873846ae952441fc1ca44d0`  
Freshness claim: user-supplied target commit only; this replacement does not verify current `main`.

## Corrected status

**Phase 3A is not accepted as complete at the audited target commit.**

Specs 0005, 0006, and 0007 remain important historical specs, but their status claims must not be read as proof that Phase 3A is safe for Phase 3B or Phase 4. The target commit still permits architecture-contaminating shortcuts around no-human autonomy, actor-known knowledge boundaries, durable intention/routine causality, validator authority, typed diagnostics, replay proof, and adversarial tests.

## Corrected interpretation of prior specs

### Spec 0005

Spec 0005 defined the correct Phase 3A intent:

```text
actor beliefs/needs/duties
 -> candidate goals
 -> durable intention
 -> HTN routine method
 -> actor-known local planning
 -> shared pipeline
 -> event/replay/TUI/debug explanation
```

Any prior statement that Spec 0005 was fully implemented must be treated as historical overclaim unless it is backed by later Spec 0008 acceptance gates.

### Spec 0006

Spec 0006 correctly identified first-hardening gaps: wait-only/no-human smoke behavior, incomplete live `AgentState`, string-heavy HTN/diagnostics, hidden-truth self-attestation, inert intentions/routines, `continue_routine` marker ambiguity, weak tests, and misleading status claims.

Spec 0006 did not permanently close those risks. It remains superseded by Spec 0008.

### Spec 0007

Spec 0007 correctly raised the bar and demanded no-human ordinary-life proof through actor-known planning, live needs, durable intentions, routines, HTN/local planning, shared validation, event ancestry, replay, and debug/TUI surfaces.

At the target commit, Spec 0007 is **not sufficient** as a Phase 3A exit proof. The audited code still shows these blocker patterns:

1. The no-human path can construct an empty epistemic projection and build actor-known-looking planning inputs from raw physical state.
2. The scheduler can direct-dispatch sleep/eat/work/movement from routine family or need thresholds before full candidate/intention/HTN/local-planner arbitration.
3. Intention lifecycle, routine step, and decision trace records can be appended after the proposal rather than causally driving it.
4. Work validation can read `current_fatigue` and `current_hunger` from caller-provided proposal parameters and default missing/malformed values to zero.
5. Authoritative decision traces and stuck diagnostics are stored as strings rather than typed records.
6. `continue_routine` remains a non-progress marker unless paired with a follow-on ordinary action; integrated no-human proof must enforce that.
7. Tests still rely too much on event labels, substring absence, and friendly fixtures rather than adversarial provenance and causal ancestry.

Any document or comment that says Spec 0007 completed Phase 3A readiness must be corrected or understood as superseded by this errata.

## Blocking correction

Spec 0008 is now the blocking Phase 3A hardening spec. Phase 3A may not be marked safe until Spec 0008 gates pass.

Spec 0008 requires, at minimum:

- one canonical actor-decision transaction for no-human ordinary actions;
- sealed/provenance-rich actor-known planning context;
- no direct routine-family or need-threshold primitive proposal dispatch in scheduler;
- actual epistemic/belief/observation inputs or explicit typed limitations;
- authoritative validator access to live `AgentState`;
- no current-need echoes in proposal parameters;
- typed decision traces and stuck diagnostics in live state, events, replay, debug reports, and TUI projections;
- chronological duration completion handling at transaction boundaries;
- adversarial hidden-truth fixtures;
- stale/forged proposal parameter tests;
- marker-only continuation tests;
- static/anti-regression tests for forbidden shortcuts;
- replacement docs and ledger updates.

## Phase-gate impact

Phase 3B and Phase 4 remain blocked.

Do not begin Phase 3B speech/testimony or Phase 4 institutions/records/wrong-suspicion work on top of the audited target commit as though Phase 3A ordinary life is architecturally safe. Later phases depend on actor-known boundaries, durable intentions, typed traces, and replayable no-human ordinary life. Building on the current shortcut seams would spread contamination into testimony, records, institutions, and causal explanation.

## Accepted status wording

Allowed before Spec 0008 passes:

- “Phase 3A has useful scaffolding but remains in anti-contamination hardening.”
- “Specs 0005/0006/0007 are archived historical steps; Spec 0008 is the active blocker.”
- “No-human ordinary-life proof is not accepted until actor-known autonomy transaction gates pass.”

Not allowed before Spec 0008 passes:

- “Phase 3A is complete.”
- “No-human ordinary life is proven.”
- “Actor-known planning is safe by construction.”
- “Spec 0007 closed the ordinary-life proof.”
- “Phase 4 can build on current Phase 3A autonomy.”

## Replacement note

This file replaces the previous Phase 3A status errata. It is intentionally severe because Phase 3A is foundation-sensitive: later layers will inherit any hidden-truth leakage, inert intention/routine state, proposal-echo validation, or string-only diagnostic architecture left in this layer.
