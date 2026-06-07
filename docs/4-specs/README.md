# Tracewake Specs

This directory contains specification packages that sit beneath foundation, architecture, execution, and reference doctrine.

Specs are not replacement constitutions. They are reviewable contracts that turn accepted doctrine into concrete phase inputs.

## Authoritative status source

`SPEC_LEDGER.md` is the authoritative status source for every implementation spec, current and historical. Archived specs remain historical evidence of intended/attempted work; they are **not** proof that the corresponding phase is safe if a later audit errata says otherwise. Corrective hardening specs block later phase work when they repair a materially incomplete archived phase. The lists below are a convenience index — when they disagree with `SPEC_LEDGER.md`, the ledger wins.

## Current specs

- `SPEC_LEDGER.md` — package-level ledger, authoritative spec status, and retread-prevention guidance.
- `0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — Phase 0 ontology and fixture contracts for the Missing Expected Property first proof.
- `0001_RESEARCH_NOTES.md` — focused external research used for Spec 0001.
- `0001_FOUNDATIONAL_DOC_AMENDMENTS.md` — amendment decision for foundation, architecture, execution, and reference docs.

## Proposed / blocking specs

- `../../specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md` — proposed corrective hardening spec; blocks Phase 3B / Phase 4 until it passes its no-human / replay / debug / TUI / actor-known gates. See `SPEC_LEDGER.md`.

## Archived specs

- `archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md` — Phase 1 runnable kernel, TUI/view-model harness, event log, and replay.
- `archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` — completed Phase 1A corrective executable TUI command loop and documentation alignment spec.
- `archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md` — Phase 2A epistemic substrate, expectation contradiction, and possession parity.
- `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` — Phase 3A needs/routines/no-human-day; archived as historical work, acceptance **not earned** at exact-commit audit (see `SPEC_LEDGER.md`, `archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md`, `archive/reports/PHASE_3A_STATUS_ERRATA.md`).

## Authority

The authority order remains:

1. foundation doctrine;
2. architecture contracts;
3. execution phase gates and fixtures;
4. reference guardrails;
5. current analysis and research synthesis;
6. this spec package.

If this package appears to conflict with a higher authority document, the higher authority document wins and this package should be corrected.
