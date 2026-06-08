# Downstream Staleness and Code Implications Report

## Status

Replacement report after the execution-layer overhaul. The foundation, architecture, and execution layers have now been replaced as live doctrine for the target commit of record: `ca1cb9d5c5885cc11aa68e071b0868606ef712d8`.

This report does not certify implementation code. It records downstream documentation status and the remaining future code-audit work.

## Authority boundary

This report owns downstream status, replacement rationale, stale-document implications, and future audit scope. It does not certify implementation code, define implementation work, create tickets, or override live foundation, architecture, or execution doctrine.

## Depends on

- `docs/README.md`
- `docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`

## Evidence basis

This replacement report is based on exact-commit reads of:

- `docs/README.md`;
- every file under `docs/0-foundation/**`;
- every file under `docs/1-architecture/**`;
- every previous file under `docs/2-execution/**`;
- archived specs `0005`, `0006`, `0007`, and `0008`;
- representative Rust seams for actor-known context, actor decision, planner, scheduler, action pipeline, state/projections, view models, content schema, validation, fixtures, and tests where exact-commit fetch was available;
- the uploaded manifest as path inventory only.

The target commit was user-supplied. This report does not claim the commit is current `main`.

## Replacement outcome

The live execution layer is replaced by:

```text
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
```

`docs/README.md` is replaced to state that the execution layer is now live-replaced and that old execution filenames are retired if they remain in the tree.

## Decision matrix for previous `docs/2-execution/**`

| Previous file | Decision | Reason |
|---|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Replaced in full | Needed new map, retirement rule, and post-0008 authority posture. |
| `01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md` | Replaced/merged | First-proof scope now lives in document `02`; baseline status lives in document `01`. |
| `02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` | Replaced/renamed | Old ladder did not make post-0008 certification the next gate. |
| `03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md` | Replaced/merged | Missing-property proof is retained but reframed as a certification contract, not phase-local scenario plan. |
| `04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | Replaced/merged | Paper ontology content is absorbed into data, fixture, and first-proof contracts. |
| `05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md` | Replaced/merged | Kernel/TUI/replay content is now gate evidence under `03`, `05`, `09`, and `10`. |
| `06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` | Replaced/renamed | Epistemic and possession proof now live in document `07` with stronger debug quarantine. |
| `07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` | Replaced/renamed | Ordinary-life doctrine now lives in document `06` and treats specs `0005` through `0008` as history, not certification. |
| `08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md` | Replaced/renamed | Phase 4 is locked behind entry gates in document `11`. |
| `09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md` | Split | Data/schema/provenance is document `08`; golden fixtures/replay is document `09`. |
| `10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md` | Replaced/centralized | Testing/observability is now a mandatory conformance gate in document `10`. |
| `11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md` | Replaced/renamed | Deferred work remains in execution as a locked contract in document `12`. |
| `12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Replaced/renamed | Research notes are sharpened as decisions and forbidden misreads in document `13`. |

## Archived specs `0005` through `0008`

Promoted into durable execution doctrine:

- needs as pressures, not scripts;
- durable intentions and typed lifecycle transitions;
- routines as defeasible HTN-like causal machinery;
- no-human day as ordinary actor transaction proof;
- actor-known planning context with provenance;
- proposal ancestry and shared action pipeline;
- hidden-truth audit rows as debug-only evidence;
- typed stuck/failure diagnostics;
- adversarial fixtures and anti-contamination tests;
- validation rejection of player/quest/culprit/outcome-chain content.

Kept as historical implementation detail:

- exact old phase labels and ticket decomposition;
- exact code/module names as proof by themselves;
- status claims saying Phase 3A is done under weaker doctrine;
- string-heavy diagnostics or display-label shortcuts;
- fixture-specific friendly paths that do not prove adversarial negative gates.

## Future code audit required

A future code audit must prove the current implementation satisfies the live docs. It must inspect, at minimum:

- actor-known context construction and provenance boundaries;
- actor decision transaction flow;
- candidate generation, HTN/routine selection, and local planning;
- scheduler no-human loop and duration/completion handling;
- action proposal construction, validation, rejection, modeled failure, and event append;
- event application, projections, replay rebuild, checksums, random-source handling;
- view models, possession binding, notebooks, why-not output, and debug panels;
- content schema, validation, canonicalization, and forbidden-field rejection;
- golden fixtures and adversarial tests;
- static anti-regression guards.

The audit must answer the questions in `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` and emit a pass/fail/scoped-remediation artifact.

## Phase 4 status

Phase 4 institutions, records, wrong suspicion, reports, and local procedures may not proceed immediately. They are locked behind the certification sequence in `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` and the entry contract in `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`.

## Anti-contamination and testing status

The replacement execution layer centralizes anti-contamination and testing:

- `04` defines the mandatory truth-firewall gate for every future spec.
- `05` defines no direct dispatch and pipeline audit.
- `09` defines golden fixture success and forbidden-shortcut coverage.
- `10` defines diagnostics and review artifacts as a central conformance gate.

These are proof mechanisms, not support topics.

## Data/fixture split

The old combined data/fixture document is split because authoring validation and golden acceptance have different authority boundaries.

- Data authoring, schema, provenance, and validation are live in `08`.
- Golden fixtures, scenarios, adversarial paths, and replay acceptance are live in `09`.

## Deferred second-proof status

Deferred notices, travel, regional expansion, LOD, and story-sifting remain in execution as locked contracts. They are not moved to reference because they must actively block premature expansion.

## Remaining downstream implications

`docs/3-reference/**`, `docs/4-specs/**`, active tickets, and archived tickets may still contain language that overclaims phase status or uses old execution filenames. They should be treated as subordinate to this replacement set until separately audited.

## Non-goals

This report does not implement code, create tickets, certify tests, claim CI results, or verify that the target commit is current `main`.
