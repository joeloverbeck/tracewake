# Execution Index and Authority

## Status

Live replacement execution index. This file replaces the previous execution index and retires the old `docs/2-execution/` document set.

Historical implementation has landed through archived specs `0005` through `0008`, but the implementation is not certified under the post-overhaul foundation, architecture, and execution doctrine.

## Authority boundary

This document owns execution-layer reading order, replacement status, gate and observation-obligation vocabulary, and the rule that execution planning cannot weaken foundation or architecture.

This document does not certify code, create tickets, decompose work, or override foundation or architecture.

## Depends on

- `docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## Replacement and retirement rule

All pre-replacement execution files are retired as live doctrine. Their concepts may survive only where promoted by this replacement set.

The retired old set included phase-local documents that treated Phase 3A hardening as if it naturally certified the baseline and positioned Phase 4 as the next work by default. That posture is unsafe after specs `0006`, `0007`, and `0008`. The live execution layer therefore inserts an explicit post-0008 baseline certification gate before Phase 4, wrong-suspicion work, notices, travel, regional expansion, or LLM surfaces.

## Live execution document map

| File | Owns |
|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Execution map, retirement rule, universal authority posture. |
| `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` | Status of archived specs, current code boundary, and certification posture. |
| `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | First proof identity, current baseline status, and acceptance contract. |
| `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` | Gate order, phase sequencing, and the rule that Phase 4 is blocked until certification gates pass. |
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | Truth-firewall execution checks and mandatory anti-contamination gates for every future spec. |
| `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Actor transaction, scheduler, proposal, validation, and direct-dispatch audit criteria. |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Needs, routines, durable intentions, ordinary-life proof, and no-human day certification. |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Epistemic proof, possession parity, view-model filtering, and debug quarantine. |
| `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Authoring contracts, schema/provenance validation, and no outcome-chain data. |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Golden fixture families, adversarial scenarios, and deterministic replay acceptance. |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Testing, diagnostics, metrics, trace artifacts, and review evidence. |
| `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` | Phase 4 entry contract and institution/record/wrong-suspicion lock. |
| `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` | Locked deferral contract for notices, travel, regional scale, LOD, and second proof. |
| `13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` | Execution-level research decisions and forbidden misreads. |

## Universal execution posture

Every future spec, audit, fixture, or code change must answer the following before being accepted:

1. Which foundation and architecture documents control this change?
2. Which holder-known context supplies cognition or procedure input?
3. Where is truth allowed to validate, and where is truth forbidden to plan?
4. Which action proposals enter the shared validation/event pipeline?
5. Which events, projections, diagnostics, and replay artifacts prove the behavior?
6. Which debug surfaces expose truth, and how is that truth quarantined?
7. Which golden fixtures prove success and which prove forbidden shortcuts fail?
8. Which layer owns each failure diagnostic?
9. Which archived specs or tickets are being treated only as historical evidence?
10. Which gate prevents premature Phase 4 or second-proof expansion?

A document that cannot answer these questions is not ready to govern implementation.

## Canonical gate names

| Gate | Meaning |
|---|---|
| `P0-DOC` | Foundation and architecture dependencies are named and no live doctrine is contradicted. |
| `P0-CERT` | Post-0008 baseline certification: current implementation has been audited against replacement foundation, architecture, and execution docs. |
| `TFW` | Truth-firewall gate: cognition/procedure/view input is holder-known; truth only validates. |
| `PIPE` | Pipeline gate: every world-affecting action uses proposal, validation, event append, projection, and replay boundaries. |
| `NO-DIRECT` | No scheduler/action/direct-dispatch shortcut exists. |
| `NO-HUMAN` | Simulation progresses through ordinary actor transactions without a human controller. |
| `POS-PARITY` | Possession changes input binding only; human and autonomous actors use the same world rules. |
| `REPLAY` | Deterministic event log, projection rebuild, random-stream, and save/replay behavior are proven. |
| `FIXTURE` | Golden fixtures prove success paths and forbidden shortcuts. |
| `DIAG` | Failure diagnostics identify responsible layer and do not leak hidden truth to embodied holders. |

## Canonical observation obligations

Observation obligations are required-to-produce evidence artifacts. They are
not certification gates: they block nothing and pass/fail nothing, and they
must never appear in the phase ladder as gates.

| Code | Meaning |
|---|---|
| `EMERGE-OBS` | Emergence-evidence ledger over the canonical seeded no-human corpus, produced with every subsequent acceptance artifact that exercises that corpus; defined in `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. Observer-only; never a simulation input; never a pass/fail threshold without a dedicated future spec. |

## Maintenance rule

Execution documents may be radical about stale sequencing and conservative about authority. They may rename, merge, split, or retire old execution docs. They may not soften foundation or architecture to fit current implementation.
