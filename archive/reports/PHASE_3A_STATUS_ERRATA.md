# Phase 3A Status Errata — Spec Ledger and Index Correction

Repository target audited: `joeloverbeck/tracewake` at `ec1e73f91b7330d87ce12ae93f8cf57ea3671306`  
Related archived spec: `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`

## 1. Purpose

This errata records a status correction. It does not rewrite archived Spec 0005. It records that archived 0005 is historical evidence of intended/attempted work, but exact-commit audit found that Phase 3A acceptance was not earned.

## 2. Status problem

`docs/4-specs/SPEC_LEDGER.md` currently describes Spec 0005 as landed with the Phase 3A substrate complete: bounded needs, durable intentions, defeasible routines, sleep/eat/work/continue/wait actions through the shared pipeline, no-human day runner/metrics, replay/debug proof, fixtures, and TUI panels.

The implementation has many of those shapes, but the behavioral proof is materially insufficient:

- the no-human day runner submits wait proposals rather than an ordinary day of wake/eat/move/work/rest/sleep;
- live action execution does not apply Phase 3A agent events into live `AgentState`;
- HTN method applicability accepts string-prefix and substring conditions;
- hidden-truth planning safety is not impossible by construction;
- tests and fixtures prove smoke/synthetic/snapshot behavior more than integrated ordinary-life behavior.

The ledger should not let a later coding agent proceed to Phase 3B or Phase 4 assuming Phase 3A is safe.

## 3. Recommended ledger wording

Recommended replacement meaning for the Spec 0005 ledger entry:

```text
0005 — Phase 3A: Needs, Routines, and No-Human Day
Status: Archived as historical implementation work. Exact-commit audit at ec1e73f91b7330d87ce12ae93f8cf57ea3671306 found that Phase 3A acceptance was not earned. The implementation contains useful scaffolding and partial surfaces, but the no-human ordinary-life substrate is not robust enough to support later phases. Corrective Spec 0006 is required before Phase 3B or Phase 4.
```

Recommended replacement meaning for the “required result” language:

```text
Intended result: bounded needs, durable intentions, defeasible routines, sleep/eat/work/continue/wait through the shared action/event pipeline, actor-known planning, no-human day metrics, replay/debug visibility, TUI panels, and adversarial fixtures. Audit result: these are not yet proven behaviorally; current code over-relies on wait-only no-human advancement, replay-side agent-state application, string preconditions, synthetic logs, and fixture/debug snapshots.
```

Recommended next-spec language:

```text
Next blocking product-behavior spec: 0006 Phase 3A Needs, Routines, and No-Human Day Hardening. No Phase 3B, Phase 4, or ordinary-life-dependent expansion should proceed until 0006 passes its no-human/replay/debug/TUI/actor-known gates.
```

## 4. Recommended `docs/4-specs/README.md` correction

`docs/4-specs/README.md` should stop implying that only the early current specs matter. It should either list archived specs explicitly or point to `SPEC_LEDGER.md` as the authoritative status source.

Recommended narrow wording:

```text
Current and historical implementation specs are tracked in SPEC_LEDGER.md. Archived specs remain historical evidence, not proof that the corresponding phase is safe if later audit errata says otherwise. Corrective hardening specs block later phase work when they repair a materially incomplete archived phase.
```

If committed alongside the hardening package, add entries for:

```text
PHASE_3A_IMPLEMENTATION_AUDIT.md — exact-commit audit finding Phase 3A acceptance not earned.
0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md — blocking corrective spec before Phase 3B/4.
PHASE_3A_STATUS_ERRATA.md — ledger/index status correction for archived 0005.
```

## 5. Non-goals

This errata does not:

- edit archived 0005;
- decompose work into implementation tickets;
- design Phase 4;
- claim the scaffolding is useless;
- require deleting Phase 3A code;
- weaken any foundation/architecture/execution gate.

## 6. Recommended status after correction

Use this status until the corrective spec passes:

```text
Phase 3A status: Nominally implemented, materially deficient, correction required.
Proceed to Phase 3B/4: no.
Blocking corrective work: Spec 0006 Phase 3A hardening.
Evidence basis: exact-commit Phase 3A implementation audit.
```
