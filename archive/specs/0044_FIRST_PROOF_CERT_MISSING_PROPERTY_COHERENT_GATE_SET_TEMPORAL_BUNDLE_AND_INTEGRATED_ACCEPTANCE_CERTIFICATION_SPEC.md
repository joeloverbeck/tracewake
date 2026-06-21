# 0044 FIRST-PROOF-CERT missing-property coherent gate set, temporal bundle, and integrated-acceptance certification spec

```text
Staging path: specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md
Archive path on accepted closeout: archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md
Target repository: joeloverbeck/tracewake
Target commit: 1541da274180ecd40f52583d86704990cb55e74c
Spec series: numbered staging spec 0044; archived to archive/specs/ on accepted closeout
Status: COMPLETED — implementing session rendered scoped-remediation verdict
Work posture: Certification
Admissibility posture: consumes P0-CERT passed from archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md, SPINE-CERT passed from archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md, EPI-CERT passed from archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md, and ORD-LIFE-CERT passed from archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md
Consumed certified building blocks: P0-CERT; SPINE-CERT; EPI-CERT; ORD-LIFE-CERT
Phase-certification label: FIRST-PROOF-CERT, as defined by docs/2-execution/02 and docs/2-execution/03; this document mints no new gate code, invariant ID, status enum, obligation code, or temporal vocabulary
Assumption: stage under specs/ through the repository's stage-then-archive convention; relocation does not alter target-commit provenance or audit semantics
```

This document is **non-executable**. It specifies what an implementing session must inspect, run, prove, record, and package. It renders no pass/fail verdict, does not assert that commit `1541da274180ecd40f52583d86704990cb55e74c` passes, and does not perform or authorize production remediation inside this audit spec.

I am not verifying that this commit is the current `main`. I am using the supplied commit as the target of record and the authoring analysis uses repository files fetched only by exact commit URL from `joeloverbeck/tracewake`. References to other repositories inside those validly fetched files are file content, not provenance contamination.


## 1. Determination confirmation: FIRST-PROOF-CERT is the next admissible move

`FIRST-PROOF-CERT` is the single next admissible certification spec. The live ladder orders the campaign as `P0-DOC -> P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY`. Gate 5 is defined as “certify the first missing-property playable proof as a coherent gate set” and is blocked only until gates 1 through 4 pass. See [docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).

The live ledger and the four replacement acceptance artifacts establish the admissibility lineage consumed here:

| Consumed certification | Exact accepted implementation commit | Controlling acceptance artifact | Use in this spec |
|---|---|---|---|
| `P0-CERT passed` | `a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441` | [archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md) | Baseline anti-contamination and post-0008 conformance are accepted within the artifact's stated scope. |
| `SPINE-CERT passed` | `92ba47f14998e0ea2fc95502bc3b76c5909478ca` | [archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md) | Event, replay, projection, action-pipeline, and no-direct-dispatch internals are accepted within the artifact's stated scope. |
| `EPI-CERT passed` | `726b2a1f1318381e75d4ffc4eff6b5103fbdd2c3` | [archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md) | Holder-known/actor-known, provenance, possession parity, embodied-view, and debug-quarantine internals are accepted within the artifact's stated scope. |
| `ORD-LIFE-CERT passed` | `c819bbee0282eb83386f7b58cab752b9e639a4af` | [archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md) | Needs, routines, intentions, no-human ordinary life, planner traces, and stuck-diagnostic internals are accepted within the artifact's stated scope. |

The ledger's source and admissibility posture is recorded in [docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/SPEC_LEDGER.md). With gates 1 through 4 passed, gate 5's entry block is lifted. A Phase-4, second-proof, feature-expansion, or gameplay-expansion spec before `FIRST-PROOF-CERT` passes would violate the ladder. This document therefore commissions no new gameplay.

The four predecessor artifacts were produced at divergent commits. Their consumption does **not** certify latest `main`, does **not** certify commit `1541da274180ecd40f52583d86704990cb55e74c` by inheritance, and does **not** prove that their formerly separate pass results still compose. The new work of `FIRST-PROOF-CERT` is one integrated coherence re-proof against one immutable unified code/evidence baseline.


## 2. Source discipline, freshness, and admissibility

### 2.1 Ledger rules carried verbatim

The following rules are carried verbatim from [docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/SPEC_LEDGER.md):

- A commit hash named inside a spec is audit/spec provenance only unless a higher-tier document says otherwise.
- A manifest is path inventory only.
- Branch names, default-branch lookups, connector namespace labels, repository metadata, and code-search snippets are not proof of target-commit content.
- Future exact-commit audits must fetch by exact file URL or by a supplied repository export at the target commit.
- Stale baseline strings in historical specs must not be adopted as current product doctrine.

### 2.2 Exact-commit authoring provenance

The uploaded manifest was used only to confirm path presence. The authoring acquisition used full URLs mechanically formed as:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/<manifest path>
```

Authoring acquisition summary:

```text
Requested repository: joeloverbeck/tracewake
Target commit: 1541da274180ecd40f52583d86704990cb55e74c
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: full exact-commit raw URLs
Requested file count: 244
Successfully verified file count: 244
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

The complete append-only URL list accompanies this deliverable as `first_proof_exact_commit_acquisition_ledger.txt`. A repository path not present in the supplied manifest, or not successfully fetched at its exact URL, is not target-repository evidence for this spec.

### 2.3 History and live certification

Archived implementation specs, tickets, research briefs, command transcripts, and non-replacement acceptance reports are historical evidence only. They may explain a seam or a past failure but cannot establish current repository state or current certification. The four replacement acceptance artifacts in section 1 are consumed only for their explicit predecessor certification scopes.

The audit anatomy follows the historical certification lineage in [archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md), [archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md), [archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md), and [archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md). Their remediation successors `0037`, `0039`, `0041`, and `0043` inform non-executable structure and mutation discipline only. The failed-floor shapes in [archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md) and [reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md), and the triage formats in [archive/reports/0040_epi_cert_mutation_triage_register.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/reports/0040_epi_cert_mutation_triage_register.md) and [reports/0043_ord_life_cert_mutation_triage_register.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/reports/0043_ord_life_cert_mutation_triage_register.md), are precedent only. They do not predict this audit's result. The Phase-2A root in [archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md) is likewise historical evidence to re-prove, never live certification.

`FIRST-PROOF-CERT` is a phase-certification label that composes the canonical gate set and review artifacts defined upstream. It does not create a tenth first-proof gate. The local identifiers `FIRST-PROOF-01` through `FIRST-PROOF-17` below are navigation labels for this audit document, not canonical gate codes or new obligation vocabulary.

### 2.4 Unified-baseline rule

The implementing session must name one immutable **unified baseline commit** `U` and derive all certifying behavior, replay, fixture, diagnostic, content, TUI, and mutation evidence from that same `U`.

- The initial candidate is `1541da274180ecd40f52583d86704990cb55e74c`.
- Audit-only tests or evidence instrumentation may be added only when needed to make an existing doctrine claim observable. Every such delta must be enumerated, and the resulting exact commit becomes `U`.
- Production remediation is out of scope. Discovery of a substantive implementation defect stops the pass path and routes to a separate remediation spec.
- Evidence from predecessor certification commits may justify consumption boundaries but may not be spliced into `U`'s integrated result.
- Results from different code, fixture, schema, test, mutation-configuration, or toolchain commits may not be combined into a pass.
- A later report-only or archive-only closeout commit may package evidence for `U` only when it changes no audited implementation, test, fixture, schema, or mutation perimeter. The artifact must distinguish the unified code/evidence commit from any documentation-only closeout commit.
- Any audited change after a certifying command invalidates that command's certifying status until rerun at the new `U`.

This rule proves composition at one baseline; it still does not verify latest `main`.


## 3. Authority and dependency declarations

### 3.1 Controlling order

Authority is strictly:

1. `docs/0-foundation/`
2. `docs/1-architecture/`
3. `docs/2-execution/`
4. `docs/3-reference/`
5. `docs/4-specs/`
6. this staging spec
7. implementation convenience

If execution conflicts with architecture or foundation, execution is wrong. If implementation convenience conflicts with an accepted gate, implementation is wrong. This spec may operationalize doctrine but may not amend an invariant, replace architecture, weaken a gate, promote archived history, or invent a compatibility alias.

The primary first-proof charter is [docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md). The primary execution contract is [docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md); the sequencing and temporal-cascade authority is [docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).

### 3.2 Foundation dependencies

| Source | Dependency carried into this audit |
|---|---|
| [docs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/README.md) | Tier ordering and conflict resolution. |
| [docs/0-foundation/00_FOUNDATION_INDEX.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/00_FOUNDATION_INDEX.md) | Foundation boundary map. |
| [docs/0-foundation/01_PROJECT_CHARTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/01_PROJECT_CHARTER.md) | The executable reason for the first proof: an ordinary world that runs without needing a human. |
| [docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md) | All constitutional constraints, especially event-sourced causality, subjective epistemics, ordinary-agent parity, no contamination, `INV-111`, and `INV-112`. |
| [docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md) | Stable event identity, causal ancestry, deterministic ordering, projection authority, and replay. |
| [docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md) | Claims, beliefs, expectation, observation, memory, contradiction, and provenance. |
| [docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md) | Ordinary action, conditional affordance, search, survival, and no teleport. |
| [docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md) | Possession parity and embodied/debug separation. |
| [docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md) | Missing-property first proof, non-goals, no-human world, and observer-only emergence doctrine. |
| [docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md) | Actor-known transaction and truth firewall. |

Foundation documents `05`, `07`, `09`, `10`, and `11` are read as scope boundaries. Already-certified ordinary-life internals are consumed rather than re-audited; institutions, LOD/scale, and LLM/speech remain deferred.

### 3.3 Architecture dependencies

| Source | Dependency carried into this audit |
|---|---|
| [docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md) | Canonical gate/review-artifact composition; phase labels do not mint new gate codes. |
| [docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md) | One-way crate direction `core <- content <- tui`. |
| [docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md) | Deterministic replay, projection rebuild, and first-divergence localization. |
| [docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md) | Sealed holder-known contexts, actor-known inputs, source sufficiency, freshness, and truth firewall. |
| [docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md) | Ordinary proposal-to-validation-to-event boundary and no alternate mutation path. |
| [docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md) | Actor transaction composition and no direct dispatch. |
| [docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md) | Expectation contradiction and epistemic projection contract. |
| [docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md) | Items, places, containers, doors, access, custody, and property distinctions. |
| [docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md) | Embodied holder-known view model and non-diegetic debug boundary. |
| [docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md) | Negative boundary: no investigation, clue, lead, quest, or story-sifting machinery in this gate. |
| [docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md) | Typed diagnostics and evidence-package obligations. |

### 3.4 Execution, reference, and live-spec dependencies

| Source | Role |
|---|---|
| [docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md) | Canonical execution gate index. |
| [docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md) | Code-audit boundary and admissibility vocabulary. |
| [docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md) | Nine gates, nine scenario families, definition of done, and non-goals. |
| [docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md) | Gate 5, temporal cascade, failure handling, and evidence requirements. |
| [docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md) | Truth/actor-known and temporal-firewall evidence. |
| [docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md) | Pipeline participation and scheduler boundary. |
| [docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md) | Integrated no-human proof and routine temporal evidence. |
| [docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md) | Embodied, possession, debug, and temporal-rendering evidence. |
| [docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md) | Positive/adversarial fixture families, replay, and temporal fixtures. |
| [docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md) | Evidence honesty, responsible layers, review artifacts, `EMERGE-OBS`, and temporal diagnostics. |
| [docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md) | Forward boundary for institutions and full investigation. |
| [docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md) | Forward boundary for second-proof scope. |
| [docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md) | Review checklist. |
| [docs/3-reference/01_DESIGN_RISK_REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/3-reference/01_DESIGN_RISK_REGISTER.md) | Hidden-truth, protagonist-gravity, replay, temporal, marker-only, and Goodhart risks. |
| [docs/3-reference/02_GLOSSARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/3-reference/02_GLOSSARY.md) | Required terminology. |
| [docs/4-specs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/README.md) | Future-spec posture and staging rules. |
| [docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md) | Live village and missing-property fixture ontology. |
| [docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md) | Evidence-item and staged-abstraction fields. |
| [docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/4-specs/SPEC_LEDGER.md) | Source discipline and admissibility lineage. |

### 3.5 Boundary-awareness sources

The remaining exact-commit tier documents were read to prevent scope drift: foundation `05`, `07`, `09`, `10`, `11`, and `13`; architecture `07`, `08`, `12`, and `14`; execution `08` and `13`. They do not enlarge this audit. In particular, institutional procedures, LLM/speech, long-history/LOD, research-note orientation, and story-sifting remain outside the certifying surface except where a higher-tier negative boundary is cited.

### 3.6 Binding vocabulary and constitutional consequences

Use `holder-known context` for the system-wide epistemic concept and `actor-known` for the actor cognition/decision case. Use `missing property` and `expectation contradiction` exactly. `Truth` means authoritative simulation state available to validation; it is not an actor input. `Debug truth` is non-diegetic. A `no-human proof` must not know which actor, if any, is possessed.

The audit must preserve, among the complete invariant set:

- every meaningful world change through event ancestry and replay;
- absence as actor-relative evidence, not authorial theft truth;
- typed, source-backed belief and observation;
- explicit unknown and stale states;
- ordinary action and search through normal affordances and costs;
- possession as input binding only;
- no human special case or protagonist gravity;
- no scheduler-authored world deltas or routine-to-action dispatch;
- no debug-to-cognition or debug-to-embodied leakage;
- `INV-111`: emergence evidence remains observer-only;
- `INV-112`: temporal claims derive from the owning modeled source and retain temporal ancestry.

### 3.7 Workspace dependency boundary

The integrated proof may compose all three crates, but dependency direction remains `tracewake-core <- tracewake-content <- tracewake-tui`. The TUI may call public core/content surfaces; content may instantiate core contracts; core may not import content or TUI. Evidence helpers must not invert that direction or create test-only authority leaks.


## 4. Composition and consumption statement

`FIRST-PROOF-CERT` consumes four certified building blocks and audits their participation in the integrated first-proof corpus. It does not reopen their internal certification questions unless the integrated run exposes a cross-boundary regression.

| Building block | Internals not re-audited here | Participation newly required at `U` |
|---|---|---|
| `P0-CERT` | The complete post-0008 baseline audit and its prior mutation remediation. | Its source-discipline, anti-contamination, failure-layer, and cross-crate locks must participate in the unified command set with no regression. |
| `SPINE-CERT` | Event-log, replay, projection, deterministic infrastructure, action pipeline, no-direct-dispatch, and their prior mutation remediation in isolation. | `EVENT` and `REPLAY` must cover the combined first-proof corpus, including the new contradiction path, temporal bundle, diagnostics, and all nine scenario families. The pipeline must remain the only accepted mutation route. |
| `EPI-CERT` | General holder-known context, beliefs, observations, provenance, possession parity, view models, and debug quarantine in isolation. | `ACTOR-KNOWN`, `POSSESSION-PARITY`, and `VIEW-DEBUG-SPLIT` must hold while the missing-property situation and ordinary-life corpus run together. |
| `ORD-LIFE-CERT` | General needs, intentions, routines, no-human day, planner trace, stuck diagnostics, and their mutation remediation in isolation. | `NO-HUMAN-ORDINARY-LIFE` must coexist with missing-property cognition, temporal premises, possession, TUI, replay, and negative fixtures. |

This composition posture has four consequences:

1. Prior pass artifacts are admissibility evidence, not a substitute for current integrated execution.
2. A point below that references an already-certified subsystem tests only its integration contract, cross-gate non-interference, and participation at `U`; it must not duplicate the predecessor's full internal audit.
3. The genuinely new certification surface is the missing-property sequence: source-backed expectation, legally obtained absence observation, event-sourced actor-known contradiction, and no culprit/suspect/clue/quest truth.
4. The consolidated temporal evidence bundle becomes certifying only here, when all five routed sources cohere at `U`.

No individual fixture can pass this certification by itself. No aggregate fingerprint can substitute for semantic witnesses. No predecessor artifact can be promoted into latest-main certification.


## 5. Audit execution protocol

### 5.1 Final-baseline identity and evidence freeze

Before certifying commands:

1. record `git rev-parse HEAD` as candidate `U`;
2. record `git status --porcelain=v1` and require an empty worktree, except for explicitly enumerated generated transcript/output directories excluded from source;
3. record `rustc --version --verbose`, `cargo --version --verbose`, and `cargo mutants --version`;
4. hash `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/mutants.toml`, `.github/workflows/ci.yml`, all audited fixture source files, and every test binary source named in this spec;
5. run the mutation file/mutant census;
6. freeze the final audit inventory;
7. run every certifying command against that same source tree;
8. after the run, repeat commit, status, and configuration hashes.

A changed hash, changed commit, changed test selection, or changed mutation configuration invalidates the corresponding evidence until rerun.

### 5.2 Mandatory baseline and integrated commands

The implementing session must capture command, start/end time, exit status, stdout, stderr, tool versions, working-directory identity, and baseline SHA for every command. Commands that can stall must be run under the repository's `tools/supervise-command.sh`; the transcript must distinguish normal exit, nonzero exit, timeout/stall termination, and supervisor failure.

```bash
git rev-parse HEAD
git status --porcelain=v1
rustc --version --verbose
cargo --version --verbose
cargo mutants --version

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --locked -p tracewake-core --doc

cargo test --locked -p tracewake-core   --test acceptance_artifact_wording   --test acceptance_gates   --test anti_regression_guards   --test ci_workflow_guards   --test doc_invariant_references   --test emergence_ledger   --test event_schema_replay_gates   --test generative_lock   --test golden_scenarios   --test hidden_truth_gates   --test negative_fixture_runner   --test no_human_capstone   --test spine_conformance

cargo test --locked -p tracewake-content   --test fixtures_load   --test forbidden_content   --test golden_fixtures_run   --test schema_conformance

cargo test --locked -p tracewake-tui   --test adversarial_gates   --test command_loop_session   --test embodied_flow   --test readme_sample_session   --test transcript_snapshot   --test tui_acceptance   --test tui_seam_conformance

cargo mutants --workspace --no-shuffle --list-files
cargo mutants --workspace --no-shuffle --list
```

The workspace command is not replaced by the named commands, and the named commands are not replaced by the workspace command. The first establishes whole-workspace coherence; the second makes the expected gate corpus explicit and prevents silent test-discovery drift.

### 5.3 Evidence honesty

Each certifying assertion must identify:

- exact path and symbol or behavior seam;
- trigger and source fixture;
- actor/holder and holder-known context identity where applicable;
- accepted proposal or explicit rejection/non-event;
- event IDs, causes, schema/version, and ordering;
- source-provenance references;
- live projection and replay projection;
- live and replay checksums/fingerprints with their precise scope;
- first divergence when equality fails;
- positive and adversarial witness;
- responsible layer;
- whether evidence is exhaustive, sampled, observer-only, pending, or historical.

A frozen fixture fingerprint proves only the bytes or canonical semantics within its stated scope. It does not prove that a fixture exercised the expected path. A test name proves nothing without its live command result and behavior witness.

### 5.4 Audit-only instrumentation

The implementing session may add narrowly scoped tests, fixture assertions, transcript emitters, or report extraction needed to observe an existing doctrine property. It may not:

- add gameplay behavior;
- add compatibility aliases or duplicate authority paths;
- add a culprit, suspect, clue, theft, quest, or story-sifting field;
- make debug data actor-legible;
- bypass event/application/replay architecture;
- weaken content rejection;
- change temporal units, thresholds, or source categories;
- turn `EMERGE-OBS` into a target;
- fix a substantive implementation failure under this audit spec.

A missing witness may be instrumented. A failing behavior must be reported and routed.


## 6. Verified audit inventory at `1541da274180ecd40f52583d86704990cb55e74c`

Every path below appeared exactly in the supplied manifest and was fetched by its full exact-commit URL. The implementing session must re-census symbols at `U`; renamed, deleted, or newly introduced integration carriers must be recorded rather than inferred.

### 6.1 Missing-property and epistemic transaction

- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/proposition.rs`
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/agent/perception.rs`
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs`
- `crates/tracewake-core/src/actions/defs/inspect.rs`
- `crates/tracewake-core/src/actions/defs/accuseprobe.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/checksum.rs`

### 6.2 Actor-known transaction and truth firewall

- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/candidate.rs`
- `crates/tracewake-core/src/agent/generation.rs`
- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`

### 6.3 Physical custody, ordinary action, and no-human participation

- `crates/tracewake-core/src/location.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/time.rs`
- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/agent/need.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/actions/defs/movement.rs`
- `crates/tracewake-core/src/actions/defs/openclose.rs`
- `crates/tracewake-core/src/actions/defs/takeplace.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/actions/defs/need_events.rs`

The full `crates/tracewake-core/src/actions/`, `agent/`, `epistemics/`, `events/`, and `replay/` modules are in the authoring acquisition ledger so cross-file carriers cannot be silently omitted.

### 6.4 Embodied TUI and debug quarantine

- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/launch.rs`
- `crates/tracewake-tui/src/lib.rs`
- `crates/tracewake-tui/src/main.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/src/transcript.rs`

### 6.5 Content, fixture, and validation carriers

- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/serialization.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-content/src/fixtures/mod.rs`
- `crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs`
- every one of the 59 manifest-listed paths under `crates/tracewake-content/src/fixtures/`, including all positive, adversarial, temporal, no-human, epistemic, possession, debug, replay, and content-boundary families
- `crates/tracewake-content/tests/fixtures_load.rs`
- `crates/tracewake-content/tests/forbidden_content.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-content/tests/schema_conformance.rs`

### 6.6 Integrated test and negative-boundary harnesses

Core:

- `acceptance_artifact_wording.rs`
- `acceptance_gates.rs`
- `anti_regression_guards.rs`
- `ci_workflow_guards.rs`
- `doc_invariant_references.rs`
- `emergence_ledger.rs`
- `event_schema_replay_gates.rs`
- `generative_lock.rs`
- `golden_scenarios.rs`
- `hidden_truth_gates.rs`
- `negative_fixture_runner.rs`
- `no_human_capstone.rs`
- `spine_conformance.rs`
- `support/generative.rs`
- `support/mod.rs`

TUI:

- `adversarial_gates.rs`
- `command_loop_session.rs`
- `embodied_flow.rs`
- `readme_sample_session.rs`
- `transcript_snapshot.rs`
- `tui_acceptance.rs`
- `tui_seam_conformance.rs`

Compile-fail boundaries include at least:

- `tests/negative-fixtures/external_crate_cannot_construct_belief_literal/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/src/lib.rs`

### 6.7 Census output required at `U`

The acceptance packet must include a table mapping every path above to:

- relevant public/private symbols;
- audit points that depend on it;
- mutation inclusion or reason mutation is not applicable;
- positive fixtures;
- adversarial fixtures;
- event kinds emitted/applied;
- projections/checksums affected;
- responsible diagnostic layer;
- changed/unchanged status relative to `1541da274180ecd40f52583d86704990cb55e74c`.

A manifest path or static file presence is not behavior evidence.


## 7. FIRST-PROOF-CERT audit contract

A point passes only when its positive witness, adversarial witness, replay/projection evidence, responsible-layer diagnostic, and exact-command evidence all pass at `U`. A predecessor artifact can establish that a subsystem's internals were formerly certified; it cannot satisfy an integrated point without live participation at `U`.

### FIRST-PROOF-01 — Unified baseline, predecessor consumption, and coherent artifact set

**Seam definition.** Establish that all nine first-proof gates and all nine scenario families are exercised against one immutable baseline, one fixture/schema registry, one test corpus, one toolchain, and one mutation configuration. Confirm `core <- content <- tui` and prevent evidence splicing across the predecessor commits.

**Audited files/modules.** Workspace manifests; `.github/workflows/ci.yml`; `.cargo/mutants.toml`; `crates/tracewake-core/tests/acceptance_gates.rs`; `spine_conformance.rs`; `anti_regression_guards.rs`; `ci_workflow_guards.rs`; `doc_invariant_references.rs`; `crates/tracewake-content/tests/fixtures_load.rs`; `golden_fixtures_run.rs`; `crates/tracewake-tui/tests/tui_seam_conformance.rs`; all files named in section 6.

**Doctrine.** Execution documents `00`, `02`, and `03`; architecture `00`, `01`, and `13`; spec ledger and spec README. Prior certification is consumed only within stated scope. The integrated result must be new evidence at `U`.

**Required positive evidence.**

- one baseline identity and before/after clean-worktree record;
- one command ledger covering every mandatory command;
- all named tests discovered and run, with no silent ignore/filter drift;
- exact fixture registry and fixture fingerprint inventory;
- exact mutation file/mutant census;
- cross-crate dependency direction mechanically checked;
- per-point evidence rows and completeness tables reconciled to the same SHA.

**Required adversarial evidence.**

- an artifact guard rejects mixed SHAs, missing transcripts, stale fixture fingerprints, omitted named tests, or a report that cites predecessor command output as current execution;
- dependency/conformance tests reject a reverse dependency or TUI/core bypass;
- changing an audited test, fixture, config, or source hash after a command makes that command non-certifying until rerun.

**Replay/projection/determinism.** Aggregate live and replay fingerprints must identify their exact scope. The packet must prove that every scenario's event log and replay artifact were produced at `U`; a single aggregate checksum without per-scenario ancestry is insufficient.

**Exact commands.**

```bash
git rev-parse HEAD
git status --porcelain=v1
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-core --test doc_invariant_references
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-tui --test tui_seam_conformance
```

**Primary failure-diagnostic layers.** `doctrine`; `content_schema`; `fixture_contract`; `test_oracle`; `documentation status`.

**Failure condition.** Any mixed baseline, omitted gate/family, reverse dependency, stale command, missing census, or untraceable aggregate artifact fails this point.

---

### FIRST-PROOF-02 — Physical custody baseline participates in EVENT, embodied play, and replay

**Seam definition.** Prove the concrete village substrate through ordinary item/container/door/access/take/place/open/close/inspect/check behavior, and prove that the missing-property setup starts from physical state rather than narrative implication.

**Audited files/modules.** `location.rs`; `state.rs`; `actions/defs/movement.rs`; `openclose.rs`; `takeplace.rs`; `inspect.rs`; `checkcontainer.rs`; `actions/pipeline.rs`; `events/envelope.rs`; `events/apply.rs`; `projections.rs`; `view_models.rs`; TUI input/render/run/transcript; content fixtures `strongbox_001.rs`, `container_item_move_001.rs`, `door_access_001.rs`, `replay_item_location_001.rs`, and `expectation_contradiction_001.rs`.

**Doctrine.** Foundation `03`, `06`, `08`, `12`; architecture `02`, `04`, `09`, `10`; execution `02`, `05`, `09`. Ownership, custody, access, location, belief, and proof remain distinct.

**Required positive fixtures and witnesses.**

- a legal open/check/inspect or take/place sequence traverses proposal -> validation -> event append -> application -> projection;
- the canonical missing-property fixture has a real item at a real authoritative location/custodian while the expecting actor's source-backed belief points elsewhere;
- embodied TUI displays only actor-legible local affordances;
- replay rebuilds physical state and embodied projection.

**Required adversarial fixtures and witnesses.**

- locked/inaccessible/nonlocal container operations reject without an accepted mutation event;
- moving the hidden item between equally unobserved lawful locations does not alter the expecting actor's pre-observation cognition or embodied view;
- no action teleports an actor or item to a true target;
- debug-only item location cannot become an embodied affordance.

**Event/replay/projection evidence.** Record proposal source, validation result, accepted event IDs/causes, item/container/door projection deltas, live/replay physical checksums, TUI view-model fingerprint, and first divergence on a deliberately perturbed replay copy.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test command_loop_session
cargo test --locked -p tracewake-tui --test transcript_snapshot
```

**Primary failure-diagnostic layers.** `fixture_contract`; `proposal_construction`; `action_validation`; `event_append`; `event_application`; `projection`; `replay`; `view_model`; `tui_input_binding`.

**Failure condition.** Physical state born from prose, an accepted alternate mutation path, nonlocal/hidden affordance leakage, teleportation, or replay mismatch fails this point.

---

### FIRST-PROOF-03 — Source-backed expectation provenance

**Seam definition.** Prove that the expectation “item should be in container/place” exists as a typed actor-held belief or expectation with mandatory source provenance before absence is discovered. It may originate from modeled prehistory, memory, observation, record, communication, or other allowed channel; it may not be invented by fixture prose, UI narration, debug truth, or a missing-property script.

**Audited files/modules.** `epistemics/belief.rs`; `proposition.rs`; `knowledge_basis.rs`; `knowledge_context.rs`; `epistemics/projection.rs`; `agent/actor_known.rs`; `agent/transaction.rs`; `events/envelope.rs`; `events/apply.rs`; content schema/serialization/validation; `expectation_contradiction_001.rs`; `forbidden_provenance_input_fails_closed_001.rs`; `prose_born_fact_rejected_001.rs`; compile-fail belief fixture.

**Doctrine.** Foundation `04`, `12`, `14`; architecture `03`, `06`; execution `02` gates `ACTOR-KNOWN`, `MISSING-PROPERTY`, and `FIXTURE-NEGATIVE`; execution `04`, `08`, and `09`.

**Required positive evidence.**

- expectation holder, proposition, stance, source reference, acquisition tick, privacy/scope, and relevant source event are recorded;
- the sealed actor-known context includes the expectation and excludes unrelated holder data;
- fixture loading creates the expectation only through an allowed typed seed/event path;
- live and replay projections preserve identical expectation provenance.

**Required adversarial evidence.**

- absent, dangling, wrong-holder, wrong-kind, future, debug-only, or prose-only source fails closed;
- an external crate cannot construct a raw belief literal or mutate source/scope through public APIs;
- adding banned-free but semantically unproven fixture fields is rejected;
- changing only hidden item truth leaves the expectation unchanged.

**Event/replay/projection evidence.** Cite the source event or explicitly authorized prehistory event, belief identity, holder-known context hash/frontier, canonical serialization, and replay equality. The artifact must distinguish seed authoring provenance from live actor observation.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `content_schema`; `content_validation`; `fixture_contract`; `event_append`; `event_application`; `projection`; `holder_known_context`; `replay`.

**Failure condition.** An expectation without sufficient source ancestry, an expectation forged from prose/truth/debug, cross-holder leakage, or replay provenance drift fails this point.

---

### FIRST-PROOF-04 — Absence is discovered by modeled observation, not authoritative truth

**Seam definition.** Prove that the actor learns absence by performing an allowed local perception/search/check action and receiving an observation record. Authoritative truth may validate the action and determine the observation's result, but it may not notify cognition that an item is missing or choose the search target.

**Audited files/modules.** `agent/perception.rs`; `epistemics/observation.rs`; `knowledge_context.rs`; `actions/defs/checkcontainer.rs`; `inspect.rs`; `actions/proposal.rs`; `actions/pipeline.rs`; `events/envelope.rs`; `events/apply.rs`; `epistemics/projection.rs`; `expectation_contradiction_001.rs`; `hidden_food_closed_container_001.rs`; `hidden_food_unknown_route_001.rs`; `hidden_route_edge_001.rs`.

**Doctrine.** Foundation `03`, `04`, `06`, `12`, `14`; architecture `03`, `04`, `06`, `09`; execution `04`, `05`, and `09`. Absence is evidence only through an allowed channel.

**Required positive evidence.**

- an actor-known expectation can motivate a local check without ground truth selecting the target;
- validation checks locality/access/container state;
- an accepted check/inspect event causally precedes an `ObservationRecorded`-class event or equivalent typed observation;
- the observation names actor, object/container/place, channel, observed tick, source event, and explicit absence result;
- the observation enters only the correct holder's epistemic projection.

**Required adversarial evidence.**

- no observation is created for an inaccessible, closed where inspection requires open, unknown-route, nonlocal, or unperformed check;
- reading current item location, fixture expected outcome, debug report, replay state, or validator data cannot directly create the observation;
- removing the check source event or substituting another actor's event causes fail-closed behavior;
- an item present in the checked location yields the corresponding presence observation and no false absence.

**Event/replay/projection evidence.** Record action proposal and validation, check event ID, observation event ID and cause, source-ref closure, actor-known context before/after, live/replay epistemic projection, and negative event absence for rejected checks.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Primary failure-diagnostic layers.** `holder_known_context`; `candidate_generation`; `proposal_construction`; `action_validation`; `event_append`; `event_application`; `projection`; `replay`.

**Failure condition.** Truth-authored absence, observation without an allowed source action/event, remote perception, wrong-holder insertion, or an observation/replay mismatch fails this point.

---

### FIRST-PROOF-05 — Event-sourced expectation contradiction and belief update

**Seam definition.** Prove the genuinely new cognition path: a source-backed prior expectation and a legally obtained absence observation are compared; an actor-known expectation contradiction is formed; contradiction links are stable and private; any resulting belief update is event-sourced; replay rebuilds the same result.

**Audited files/modules.** `epistemics/contradiction.rs`; `belief.rs`; `observation.rs`; `proposition.rs`; `epistemics/projection.rs`; `events/envelope.rs`; `events/log.rs`; `events/apply.rs`; `replay/rebuild.rs`; `replay/report.rs`; `checksum.rs`; `expectation_contradiction_001.rs`; compile-fail contradiction-link fixture.

**Doctrine.** Foundation `03`, `04`, `12`; architecture `02`, `03`, `06`; execution `02` gates `EVENT`, `ACTOR-KNOWN`, `MISSING-PROPERTY`, and `REPLAY`; execution `09`.

**Required positive evidence.**

- matching is actor-relative and proposition-specific;
- the contradiction links prior expectation belief, contradicting observation, expected proposition, observed proposition, holder, source/cause, and detection time;
- the contradiction and any updated/missing belief use normal event application;
- unrelated expectations remain unchanged;
- duplicate processing is deterministic and does not create duplicate contradictions;
- the holder-known context and debug projection render the contradiction under their respective authority rules.

**Required adversarial evidence.**

- wrong holder, wrong item, wrong container/place, stale/unrelated observation, present item, unsupported stance, absent provenance, or missing source event produces no accepted contradiction;
- external code cannot forge or mutate contradiction links;
- reordering an equivalent expectation collection cannot change semantic output;
- adding an unrelated expectation cannot change the target contradiction;
- direct projection insertion or prose-authored contradiction is impossible/rejected.

**Event/replay/projection evidence.** Capture the ordered segment from source expectation through check/observation to contradiction and belief update; list event IDs, causes, payload versions, holder, source refs, and projection records. Rebuild from empty projections and compare physical, epistemic, agent, diagnostic, and checksum surfaces. A perturbed log must produce a localized first divergence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `event_append`; `event_application`; `projection`; `holder_known_context`; `replay`; `test_oracle`.

**Failure condition.** A contradiction inferred without both typed premises, unstable/forgeable links, direct projection mutation, duplicate nondeterminism, cross-holder leakage, or replay drift fails this point.

---

### FIRST-PROOF-06 — No culprit, suspect, clue, theft, quest, or story-sifting truth

**Seam definition.** Prove that the missing-property situation remains an ordinary epistemic situation. The world supplies no culprit flag, suspect token, theft truth, witness assignment, clue chain, quest state, lead board, or story-sifting objective. Actors may later form fallible suspicions only from actor-known evidence, but institutional investigation is not certified here.

**Audited files/modules.** Live fixture ontology `docs/4-specs/0001`; content schema/validation/serialization; `expectation_contradiction_001.rs`; `knowledge_blocker_accuse_001.rs`; `prose_born_fact_rejected_001.rs`; `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs`; `actions/defs/accuseprobe.rs`; holder-known and debug projections; content tests.

**Doctrine.** Foundation `02`, `04`, `09`, `12`, `14`; architecture `06` and the explicit boundary in `11`; execution `02`, `04`, `08`, `09`, and the Phase-4 boundary in `11`.

**Required positive evidence.**

- the canonical fixture can represent physical custody plus an actor's expectation/absence/contradiction without identifying a culprit;
- allowed post-contradiction behavior remains ordinary and actor-relative: search, ask/report when implemented within current scope, misremember, form an unsupported/fallible suspicion through normal cognition, or do nothing;
- debug may show physical truth but labels it non-diegetic and does not translate it into an actor-known accusation target.

**Required adversarial evidence.**

- content rejects semantically equivalent culprit/suspect/clue/quest/outcome-chain fields even when names avoid banned words;
- no validator, fixture runner, scheduler, planner, TUI, or debug adapter chooses a suspect from physical custody truth;
- `knowledge_blocker_accuse_001` or equivalent requires knowledge/provenance rather than a hidden culprit;
- a hidden-holder permutation cannot change actor decisions before modeled evidence;
- no repository link or historical Phase-2A statement is treated as live target truth.

**Event/replay/projection evidence.** Show that the authoritative log contains physical and epistemic events, not a culprit fact. Show canonical fixture serialization/schema output and replay projection field census. Any suspicion-like record in current scope must cite actor-known provenance and remain distinct from authoritative guilt.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Primary failure-diagnostic layers.** `doctrine`; `content_schema`; `content_validation`; `fixture_contract`; `holder_known_context`; `candidate_generation`; `method_selection`; `view_model`; `debug_quarantine`; `test_oracle`.

**Failure condition.** Any authored or derived culprit/suspect/clue/quest truth, hidden-truth accusation target, or imported investigation machinery fails this point.

---

### FIRST-PROOF-07 — Truth-firewall and actor-known participation across the combined corpus

**Seam definition.** Consume `EPI-CERT` and re-prove its boundary as an integration relation: actor decisions and contradiction formation use sealed actor-known inputs; validation truth only accepts/rejects; changing unobserved truth does not change cognition until a modeled information event occurs.

**Audited files/modules.** `agent/actor_known.rs`; `transaction.rs`; `decision.rs`; `candidate.rs`; `generation.rs`; `htn.rs`; `methods.rs`; `planner.rs`; `trace.rs`; `epistemics/knowledge_context.rs`; `knowledge_basis.rs`; `actions/proposal.rs`; `pipeline.rs`; `hidden_truth_gates.rs`; hidden-food/route/workplace/provenance fixtures.

**Doctrine.** Foundation `04`, `14`; architecture `03`, `04`, `05`; execution `02` gates `TRUTH-FIREWALL` and `ACTOR-KNOWN`; execution `04` and `05`.

**Required positive evidence.**

- transaction inputs enumerate actor-known facts and source refs;
- candidate/method/plan/proposal outputs cite the context or source frontier they consumed;
- validation can reject stale or impossible proposals without proposing a fallback or revealing why in actor-illegible terms;
- a modeled observation/notice can legally change a later decision.

**Required adversarial evidence.**

- paired runs vary only hidden food, route, workplace, item custodian, debug rows, or validator-only time; pre-observation decisions and context fingerprints remain equal;
- deleting/substituting one provenance source fails closed;
- forged/stale context, actor, target, route, affordance, duration, or reservation data cannot be accepted;
- validation failure does not leak hidden alternatives into actor-visible diagnostics.

**Event/replay/projection evidence.** Record sealed context ID/hash/frontier, candidate and method traces, proposal provenance, validation decision, accepted event or explicit non-event, and clean replay. Debug-on/off runs must have identical authoritative events and actor-known outputs.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Primary failure-diagnostic layers.** `holder_known_context`; `candidate_generation`; `method_selection`; `local_planning`; `proposal_construction`; `action_validation`; `debug_quarantine`; `test_oracle`.

**Failure condition.** Any actor output dependent on unobserved truth, any validator-authored goal/fallback, provenance loss, or debug-dependent cognition fails this point.

---

### FIRST-PROOF-08 — Possession parity, embodied view, and debug split in the missing-property run

**Seam definition.** Consume `EPI-CERT` and certify participation: binding a controller changes input routing only. It must not reset intention, needs, routine, memory, expectation, contradiction, provenance, or world access; it must not reveal the hidden item's location; debug remains non-diegetic.

**Audited files/modules.** `controller.rs`; `view_models.rs`; `debug_capability.rs`; `debug_reports.rs`; TUI `app.rs`, `input.rs`, `render.rs`, `debug_panels.rs`, `run.rs`, `transcript.rs`; fixtures `possession_parity_001.rs`, `possession_does_not_reset_intention_001.rs`, `debug_attach_001.rs`, `debug_omniscience_excluded_001.rs`, `view_filtering_001.rs`, `embodied_menu_lags_truth_change_without_perception_001.rs`.

**Doctrine.** Foundation `08`, `12`; architecture `10`; execution `02` gates `POSSESSION-PARITY` and `VIEW-DEBUG-SPLIT`; execution `07`.

**Required positive evidence.**

- bind/unbind/rebind changes controller ownership and input origin only;
- the possessed actor sees the same holder-known expectation/observation/contradiction available to the autonomous actor;
- embodied menus derive from holder-known context and current legal affordances;
- debug panels are explicitly marked and may compare truth with actor-known state;
- autonomous and human-origin proposals traverse the same validators and event path.

**Required adversarial evidence.**

- possession does not refresh stale belief, add a memory, disclose hidden custody, reset intention, or create a special action;
- debug attach/detach does not alter authoritative event log, actor-known context hash, candidate set, plan, or replay;
- a truth change without perception leaves embodied view stale in the modeled way;
- stale/forged semantic input IDs reject rather than falling back to hidden truth;
- switching which actor is human-controlled does not change no-human-world rules.

**Event/replay/projection evidence.** Compare pre/post bind actor state, context hash/frontier, intention/routine IDs, event sequence, projection checksum, view-model fingerprint, and transcript. Debug rows are packaged separately and excluded from embodied/cognition fingerprints.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-tui --test transcript_snapshot
```

**Primary failure-diagnostic layers.** `holder_known_context`; `intention_lifecycle`; `projection`; `view_model`; `debug_quarantine`; `tui_input_binding`; `test_oracle`.

**Failure condition.** Possession-caused cognition/state change, human-only affordance, debug leakage, hidden-truth view update, or unequal validation/event semantics fails this point.

---

### FIRST-PROOF-09 — No-human ordinary-life and no-direct-dispatch participation

**Seam definition.** Consume `ORD-LIFE-CERT` and the pipeline portion of `SPINE-CERT`, then prove that a boring ordinary-life interval and the missing-property situation coexist. The simulation advances without knowing whether a human is present, through actor transactions and the ordinary action pipeline.

**Audited files/modules.** `scheduler.rs`; `time.rs`; `agent/no_human_surface.rs`; `need.rs`; `routine.rs`; `intention.rs`; `planner.rs`; `trace.rs`; `need_accounting.rs`; `actions/pipeline.rs`; ordinary action definitions; `no_human_capstone.rs`; fixtures `no_human_day_001.rs`, `no_human_advance_001.rs`, `food_unavailable_replan_001.rs`, `routine_blocked_diagnostic_001.rs`, `routine_no_teleport_001.rs`, and the canonical contradiction fixture.

**Doctrine.** Foundation `05`, `06`, `12`, `14`; architecture `04`, `05`, `09`; execution `02` gate `NO-HUMAN-ORDINARY-LIFE`; execution `05` and `06`.

**Required positive evidence.**

- multiple actors can sleep/eat/work/wait/search/check/become blocked through normal transactions without human input;
- progress is an accepted ordinary action, modeled wait, or typed stuck/failure outcome, not a marker;
- a missing-property contradiction can occur during or alongside ordinary routines without turning the world into a scripted scene;
- action proposals and events have normal causal ancestry;
- liveness evidence distinguishes a modeled wait from silent non-progress.

**Required adversarial evidence.**

- scheduler cannot dispatch a primitive action or author state deltas from need/routine labels;
- hidden food/property/route truth cannot become a target;
- no-human mode cannot teleport, silently starve, spin in a loop, rewrite transaction-authored wait reasons, or count `continue_routine`/debug markers as progress;
- assigning possession to any actor or no actor produces the same authoritative ordinary-life rules;
- blocking yields typed responsible-layer diagnostics and legal fallback/wait/stuck behavior.

**Event/replay/projection evidence.** Package an integrated no-human log segment containing ordinary-life and missing-property events; reconcile need accounting, intentions, routines, waits, blockers, progress classification, physical/epistemic projections, and replay diagnostics.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test tui_acceptance
```

**Primary failure-diagnostic layers.** `candidate_generation`; `intention_lifecycle`; `method_selection`; `local_planning`; `proposal_construction`; `scheduler`; `action_validation`; `event_application`; `projection`; `test_oracle`.

**Failure condition.** Human special case, direct dispatch, scheduler state mutation, marker-only progress, silent stall, hidden-target selection, teleport, or replay/accounting drift fails this point.

---

### FIRST-PROOF-10 — Composite replay, projection rebuild, deterministic diagnostics, and divergence localization

**Seam definition.** Consume `SPINE-CERT` internals but certify the larger corpus: every first-proof scenario and the integrated capstone rebuild physical, epistemic, agent, embodied-support, and diagnostic state from fixture plus ordered events.

**Audited files/modules.** `events/envelope.rs`; `events/log.rs`; `events/apply.rs`; `events/mutation.rs`; `replay/rebuild.rs`; `replay/report.rs`; `checksum.rs`; `projections.rs`; `epistemics/projection.rs`; `need_accounting.rs`; `debug_reports.rs`; `event_schema_replay_gates.rs`; `generative_lock.rs`; `golden_scenarios.rs`; `golden_fixtures_run.rs`.

**Doctrine.** Foundation `03`; architecture `02`; execution `02` gates `EVENT` and `REPLAY`; execution `09` and `10`.

**Required positive evidence.**

- each event has stable ID, causal ancestry, schema/version, deterministic order, and replay effect or explicit non-world diagnostic role;
- replay starts from the declared fixture/content/ruleset version and empty projections;
- physical, epistemic, agent/routine/accounting, diagnostic, and view-support projections match live state;
- serialized logs and reports are deterministic for identical inputs;
- the combined corpus can be replayed scenario-by-scenario and as the designated integrated capstone.

**Required adversarial evidence.**

- unknown/duplicate/wrong-version event, broken cause, reordered events, omitted contradiction event, changed payload, or changed fixture fingerprint rejects or localizes the first mismatch;
- replay cannot silently skip an event kind or regenerate actor-known facts from current truth;
- debug rows do not alter authoritative checksum;
- equal final physical state with different causal/epistemic history is not misreported as equal evidence;
- prefix/suffix and checkpoint partitioning preserve the same final result when semantically equivalent.

**Event/replay/projection evidence.** For every scenario record fixture fingerprint, event count, event-kind census, root causes, live checksums by projection, replay checksums by projection, diagnostic fingerprint, serialized-log hash, and first divergence for at least one controlled perturbation. Report the exact comparator semantics; “asserted unequal” is insufficient.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test transcript_snapshot
```

**Primary failure-diagnostic layers.** `event_append`; `event_application`; `projection`; `replay`; `view_model`; `test_oracle`.

**Failure condition.** Missing event totality, nondeterministic output, projection mismatch, nonlocalized divergence, truth-derived replay repair, or diagnostic drift fails this point.

---

### FIRST-PROOF-11 — Fixture-negative, schema, compile-fail, and semantic content rejection

**Seam definition.** Prove that forbidden authoring and public-API paths fail before they can contaminate runtime. Rejection must be semantic enough to catch renamed/nested/generated equivalents, not only a list of banned words.

**Audited files/modules.** Content `schema.rs`, `validate.rs`, `load.rs`, `serialization.rs`, fixture registry; `forbidden_content.rs`; `schema_conformance.rs`; `fixtures_load.rs`; `negative_fixture_runner.rs`; `anti_regression_guards.rs`; compile-fail fixtures for belief, observation, contradiction; `prose_born_fact_rejected_001.rs`; `forbidden_provenance_input_fails_closed_001.rs`; typed-unproven hidden-truth fixture.

**Doctrine.** Foundation `09`, `12`, `14`; architecture `01`, `03`, `13`; execution `02` gate `FIXTURE-NEGATIVE`; execution `04`, `08`, `09`, `10`.

**Required positive evidence.**

- all first-proof fixtures load through the canonical registry and declare supported schema/scope;
- source-backed epistemic seeds canonicalize deterministically;
- valid negative fixtures reach the intended rejection stage and name the responsible layer;
- compile-fail fixtures prove external callers cannot forge protected epistemic records.

**Required adversarial evidence.**

- reject culprit/suspect/clue/quest/outcome-chain, prose-born fact, raw hidden truth, blanket known-food/property helpers, missing provenance, direct action dispatch marker, debug-as-knowledge, and unversioned/unknown schema;
- reject semantic aliases, nested wrappers, generated metadata, or innocuous field names that carry the same prohibited authority;
- reject fixture “expected outcome” data if runtime code could consume it as a plan;
- ensure a rejected fixture cannot produce partial world or projection state.

**Event/replay/projection evidence.** For rejected content, prove no accepted event/log/projection exists. For valid content, record canonical semantic fingerprint and replay seed identity. Distinguish raw-byte, canonical-content, fixture-registry, and behavior fingerprint scopes.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `content_schema`; `content_validation`; `fixture_contract`; `event_append`; `test_oracle`.

**Failure condition.** Runtime consumption of forbidden data, partial load, forgeable protected records, text-only guard evasion, or dishonest fingerprint scope fails this point.

---

### FIRST-PROOF-12 — Temporal firewall: modeled temporal premises, ancestry, freshness, and non-contamination

**Seam definition.** Certify the first routed temporal source from execution `04`. Any time-sensitive actor belief, expectation, candidate, contradiction, or proposal must derive from an allowed holder-known source with temporal ancestry. Raw scheduler time, replay order, debug timestamps, queue state, and validator-only due/open facts may validate but may not become cognition.

**Audited files/modules.** `time.rs`; `scheduler.rs`; `agent/actor_known.rs`; `transaction.rs`; `decision.rs`; epistemic belief/observation/knowledge files; `actions/proposal.rs`; `hidden_truth_gates.rs`; fixtures `stale_workplace_notice_superseded_by_newer_001.rs`, `embodied_workplace_believed_open_truth_closed_commit_fails_001.rs`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs`.

**Doctrine.** `INV-112`; foundation `04`, `14`; architecture `03`, `05`, `06`; execution `04` temporal-firewall section and execution `02` integrated acceptance.

**Required positive evidence.**

- every actor-legible temporal premise names source category, source event/record/observation, observed/acquired time, freshness state where doctrine requires it, and uncertainty/staleness representation;
- a newer modeled source can supersede an older source through event/projection logic;
- validator time is separately identified and cannot masquerade as holder-known time;
- replay preserves source ancestry and supersession.

**Required adversarial evidence.**

- change true schedule/open/due/current time without changing actor-known sources; candidate/plan/view/contradiction remains unchanged;
- inject raw clock, scheduler queue, replay index, debug timestamp, or validator future fact; actor-known construction rejects or excludes it;
- restamping stale content without a modeled information event cannot refresh it;
- possession/debug toggles cannot refresh temporal knowledge.

**Event/replay/projection evidence.** Record source-event chain, temporal premise type, acquisition/observation tick, supersession decision, context hash/frontier, accepted/rejected proposal, and live/replay temporal fields. No new unit or threshold may be invented by the audit.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Primary failure-diagnostic layers.** `holder_known_context`; `candidate_generation`; `method_selection`; `proposal_construction`; `action_validation`; `projection`; `replay`; `debug_quarantine`.

**Failure condition.** Actor cognition derived from raw/validator/debug time, untraceable freshness, silent restamping, or temporal replay drift fails this point.

---

### FIRST-PROOF-13 — Routine temporal premises and adaptation in the integrated no-human run

**Seam definition.** Certify the second routed temporal source from execution `06`. Consume the already-certified routine mechanism; prove that its modeled temporal premises, waits, interruptions, contradictions, and adaptation participate coherently in the first-proof corpus at `U`.

**Audited files/modules.** `scheduler.rs`; `time.rs`; `agent/routine.rs`; `intention.rs`; `planner.rs`; `trace.rs`; `need.rs`; `need_accounting.rs`; ordinary action definitions; fixtures `stale_workplace_notice_superseded_by_newer_001.rs`, `embodied_workplace_believed_open_truth_closed_commit_fails_001.rs`, `sleep_spanning_window_boundary_charges_each_tick_once_001.rs`, `wait_then_window_passive_charges_each_tick_once_001.rs`, `food_unavailable_replan_001.rs`, `routine_blocked_diagnostic_001.rs`.

**Doctrine.** `INV-112`; execution `06` “Routine Temporal Premises and Adaptation Proof”; execution `03` temporal cascade.

**Required positive evidence.**

- selected routine/method identifies its modeled temporal premise and provenance;
- an actor can wait, retry, fall back, interrupt, or adapt after a modeled observation/notice/outcome/contradiction;
- duration and passive accounting charge each tick exactly once across window boundaries and replay;
- no-human progress/stuck classification remains honest while the missing-property path is active.

**Required adversarial evidence.**

- true schedule alone cannot select work or wake/sleep behavior;
- stale premises are not silently corrected;
- blocked routines cannot loop, teleport, or fabricate progress;
- scheduler cannot rewrite transaction-authored wait reason or create fallback cognition;
- partitioning a time window or inserting an interruption cannot double-charge or omit accounting.

**Event/replay/projection evidence.** Package routine selection/rejection trace, premise sources, wait/action events, interruption/adaptation events, accounting ledger, typed blocker, and live/replay equality. Relate the temporal witness to the same integrated capstone as the contradiction.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `candidate_generation`; `intention_lifecycle`; `method_selection`; `local_planning`; `scheduler`; `action_validation`; `event_application`; `projection`; `replay`.

**Failure condition.** True-time planning, silent stale correction, incorrect accounting, untyped blocking, direct dispatch, or nonreplayable adaptation fails this point.

---

### FIRST-PROOF-14 — Embodied temporal rendering, possession neutrality, and debug quarantine

**Seam definition.** Certify the third routed temporal source from execution `07`. Labels such as late, stale, closed, due, soon, recently, or missed may appear in embodied output only when supported by modeled holder-known temporal evidence. Possession and debug do not upgrade temporal knowledge.

**Audited files/modules.** `view_models.rs`; `controller.rs`; `debug_reports.rs`; TUI `render.rs`, `debug_panels.rs`, `app.rs`, `input.rs`, `run.rs`, `transcript.rs`; fixtures `embodied_menu_lags_truth_change_without_perception_001.rs`, `embodied_workplace_believed_open_truth_closed_commit_fails_001.rs`, `stale_workplace_notice_superseded_by_newer_001.rs`, `debug_omniscience_excluded_001.rs`, `possession_parity_001.rs`.

**Doctrine.** `INV-112`; foundation `08`; architecture `10`; execution `07` embodied temporal rendering; execution `03` temporal cascade.

**Required positive evidence.**

- each temporal label/affordance in embodied output cites holder-known source ancestry or a documented non-temporal derivation;
- view output updates only after a modeled information event/projection update;
- possession preserves the same temporal context and freshness;
- debug can display current truth alongside actor-known belief with non-diegetic marking;
- transcript snapshots are deterministic.

**Required adversarial evidence.**

- changing raw clock, queue, true schedule, or debug timestamp without an actor-known source does not change embodied output;
- bind/unbind does not refresh, reinterpret, or reveal temporal facts;
- debug panel output never feeds semantic input choices or actor-visible affordances;
- stale semantic commands reject rather than resolve against truth;
- observer-only time controls cannot be mistaken for actor knowledge.

**Event/replay/projection evidence.** Record holder-known context ID/hash/frontier, source event, view-model field, rendered text/semantic action IDs, debug-only rows, transcript hash, and replay output. Compare debug-off/on and possessed/autonomous relations.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
```

**Primary failure-diagnostic layers.** `holder_known_context`; `projection`; `view_model`; `debug_quarantine`; `tui_input_binding`; `test_oracle`.

**Failure condition.** Clock/debug-derived embodied timing, possession refresh, stale-command truth fallback, temporal view/replay drift, or unmarked debug truth fails this point.

---

### FIRST-PROOF-15 — Temporal positive/adversarial fixture families and replay pairing

**Seam definition.** Certify the fourth routed temporal source from execution `09`. The temporal bundle must have paired positive and adversarial fixtures covering provenance, staleness, interruption, duration boundaries, waiting, replay, and anti-contamination. Merely listing fixtures does not satisfy this point; each must be registry-reachable and behaviorally exercised.

**Audited files/modules.** Fixture registry and runner; `stale_workplace_notice_superseded_by_newer_001.rs`; `embodied_workplace_believed_open_truth_closed_commit_fails_001.rs`; `aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs`; `sleep_spanning_window_boundary_charges_each_tick_once_001.rs`; `wait_then_window_passive_charges_each_tick_once_001.rs`; `routine_blocked_diagnostic_001.rs`; temporal-related hidden-truth and TUI fixtures; golden fixture tests.

**Doctrine.** Execution `09` temporal fixture families and replay acceptance; execution `03` temporal cascade; `INV-112`.

**Required positive evidence.**

- source-backed current premise;
- source-backed stale premise rendered/handled as stale rather than truth-corrected;
- superseding newer modeled notice/observation;
- duration crossing a window boundary with single charge;
- modeled wait and later adaptation;
- replay of each temporal scenario with identical projections/diagnostics.

**Required adversarial evidence.**

- raw clock, debug timestamp, omniscient due/closed state, queue position, replay index, renamed metadata, nested wrapper, generated prose, and source restamp attempts;
- temporal fact without source;
- changed true schedule with unchanged holder-known inputs;
- tampered event order/payload with first-divergence report;
- marker-only wait/progress.

**Event/replay/projection evidence.** For every temporal fixture, record registry ID, semantic fingerprint, trigger, source provenance, event sequence, projection fields, diagnostics, live/replay checksums, and positive/adversarial pairing. State whether coverage is exhaustive over the declared finite registry or sampled.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test transcript_snapshot
```

**Primary failure-diagnostic layers.** `content_schema`; `content_validation`; `fixture_contract`; `holder_known_context`; `scheduler`; `projection`; `replay`; `view_model`; `test_oracle`.

**Failure condition.** Unreached fixture, unpaired temporal path, source-free temporal claim, dishonest fingerprint, replay drift, or omitted adversarial family fails this point.

---

### FIRST-PROOF-16 — Temporal diagnostics and consolidated five-source acceptance line

**Seam definition.** Certify the fifth routed temporal source from execution `10` and close the complete bundle. Diagnostics must distinguish validator time from holder-known premises; name the responsible layer; preserve replay ancestry; and reconcile the outputs of points 12–15 into one acceptance line at `U`.

**Audited files/modules.** `actions/report.rs`; `agent/trace.rs`; `debug_reports.rs`; `replay/report.rs`; no-human metrics/diagnostics; TUI transcripts/debug panels; `acceptance_artifact_wording.rs`; `emergence_ledger.rs`; acceptance template `docs/4-specs/0003`; relevant tests.

**Doctrine.** Execution `10` evidence honesty and temporal diagnostics; execution `03` temporal cascade; architecture `13`; `INV-111` and `INV-112`.

**Required positive evidence.**

- each temporal failure/success reports canonical responsible layer, component, actor/holder, source IDs, expected/actual, accepted/rejected stage, and first divergence where applicable;
- diagnostics state whether the time fact was validator-only or holder-known;
- live and replay diagnostics are deterministic;
- the acceptance artifact has one row for each routed source: `04`, `06`, `07`, `09`, `10`;
- staged abstractions and pending items cannot be counted as pass.

**Required adversarial evidence.**

- a generic “stuck” or “time mismatch” without responsible layer is non-certifying;
- debug timestamp or observer metric cannot become actor-visible reason;
- a replay mismatch without first-divergence identity is non-certifying;
- omission of any routed source fails bundle completeness;
- no temporal metric or `EMERGE-OBS` value may become a scheduler objective or threshold.

**Event/replay/projection evidence.** Reconcile source event, holder-known temporal premise, routine behavior, embodied output, fixture result, diagnostic, and replay record in one trace. The consolidated row must link, not merely collect, the five source-specific evidence items.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
```

**Primary failure-diagnostic layers.** Every canonical layer as applicable; especially `holder_known_context`, `scheduler`, `action_validation`, `projection`, `replay`, `view_model`, `debug_quarantine`, and `test_oracle`.

**Failure condition.** Missing routed source, ambiguous layer, diagnostic/replay mismatch, actor-visible debug reason, thresholded observer evidence, or an unlinked bundle fails this point.

---

### FIRST-PROOF-17 — Cross-gate relational, property-based, metamorphic, and integration closure

**Seam definition.** Prove that independently certified subsystems retain their contracts when combined. The point does not re-audit subsystem internals; it targets cross-boundary interaction faults, hidden-truth non-interference, deterministic composition, and corpus completeness.

**Audited files/modules.** `support/generative.rs`; `generative_lock.rs`; `acceptance_gates.rs`; `golden_scenarios.rs`; `hidden_truth_gates.rs`; `no_human_capstone.rs`; `golden_fixtures_run.rs`; TUI adversarial/embodied/transcript tests; all integration carriers identified by the final census.

**Doctrine.** Execution `02`, `03`, `09`, `10`; architecture `13`; all nine gates as a coherent set.

**Required generated/metamorphic relations.**

1. **Hidden-custody permutation:** move the item among unobserved legal custodians/containers; before legal observation, actor-known contexts, decisions, embodied views, and event proposals remain equal.
2. **Observation intervention:** add the legal check/observation event; only then may expectation contradiction and later cognition diverge.
3. **Premise deletion/substitution:** remove or change one provenance source at a time; behavior fails closed, never truth-falls-back.
4. **Presence/absence dual:** same expectation and check, but item present versus absent; presence yields no false absence contradiction.
5. **Collection/order permutation:** reorder maps, fixtures, expectations, or unrelated events where semantics permit; canonical output and replay remain deterministic.
6. **Unrelated-fact non-interference:** add an unrelated actor expectation or unrelated debug row; target actor output is unchanged.
7. **Possession non-interference:** autonomous, possessed, rebound, and unpossessed runs produce equal authoritative outcomes for equivalent ordinary inputs.
8. **Debug non-interference:** debug off/on changes debug artifacts only.
9. **Human-presence non-interference:** choose any actor or no actor as controller; no-human rules, validators, and world causality remain ordinary.
10. **Temporal hidden-truth non-interference:** alter true schedule/clock/queue while preserving holder-known sources; actor outputs remain equal.
11. **Temporal legal-update relation:** add a modeled notice/observation/outcome; later behavior may change, with source ancestry.
12. **Window partition relation:** split/merge equivalent modeled advancement intervals; accounting and final projections match without double charge.
13. **Replay partition relation:** replay whole log or legal prefixes/checkpoints plus suffix; final projections and diagnostics match.
14. **Tamper localization:** delete/reorder/change one event; first divergence identifies the earliest affected event/projection.
15. **Marker monotonicity:** adding non-progress markers cannot increase no-human progress; adding a permitted committed ordinary event changes classification exactly once.
16. **Content semantic alias rejection:** rename/nest/generated-wrap prohibited authority; validation still rejects.
17. **Cross-fixture determinism:** repeat the complete registry with fixed inputs and record identical per-fixture semantic/replay outputs.

**Evidence obligations.** Record generator implementation/version, seed, case count, shrink path, generated population, omitted population, relation preconditions, and whether the perimeter is finite/exhaustive or sampled. One friendly seed is not enough. A relation that cannot reach the intended branch is a failed witness, not a pass.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test transcript_snapshot
```

**Primary failure-diagnostic layers.** The first responsible canonical layer reached by the failed relation, plus `test_oracle` when reachability, generator fidelity, or assertion scope is wrong.

**Failure condition.** Any cross-gate relation that leaks hidden truth/debug/human privilege, loses provenance, changes nondeterministically, double-counts time, bypasses event/replay, or cannot demonstrate branch reachability fails this point.


## 8. Completeness and coverage matrices

`P` means the audit point carries primary certifying proof for that gate/family. `I` means the point proves integrated participation or non-regression. `—` means the point is not relied on for that column. A column passes only when every primary point and its required integrated-participation points have certifying evidence.

### 8.1 Audit-point to nine-gate coverage

| Audit point | `EVENT` | `TRUTH-FIREWALL` | `ACTOR-KNOWN` | `POSSESSION-PARITY` | `NO-HUMAN-ORDINARY-LIFE` | `MISSING-PROPERTY` | `VIEW-DEBUG-SPLIT` | `REPLAY` | `FIXTURE-NEGATIVE` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `FIRST-PROOF-01` | I | I | I | I | I | I | I | I | I |
| `FIRST-PROOF-02` | P | I | I | I | — | I | I | P | — |
| `FIRST-PROOF-03` | I | P | P | — | — | P | — | I | I |
| `FIRST-PROOF-04` | P | P | P | — | — | P | — | I | I |
| `FIRST-PROOF-05` | P | — | P | — | — | P | — | P | I |
| `FIRST-PROOF-06` | — | P | I | — | — | P | I | — | P |
| `FIRST-PROOF-07` | I | P | P | — | I | I | — | I | I |
| `FIRST-PROOF-08` | — | — | I | P | — | I | P | I | I |
| `FIRST-PROOF-09` | I | I | I | I | P | I | — | I | I |
| `FIRST-PROOF-10` | P | — | I | — | I | I | I | P | I |
| `FIRST-PROOF-11` | — | P | I | — | — | I | I | I | P |
| `FIRST-PROOF-12` | I | P | P | — | I | I | I | P | I |
| `FIRST-PROOF-13` | I | I | I | — | P | I | — | P | I |
| `FIRST-PROOF-14` | — | — | I | P | — | I | P | I | I |
| `FIRST-PROOF-15` | I | I | I | I | I | I | I | P | P |
| `FIRST-PROOF-16` | I | I | I | I | I | I | I | P | I |
| `FIRST-PROOF-17` | P | P | P | P | P | P | P | P | P |

Gate completeness summary:

| First-proof gate | Primary audit points | Required integrated result |
|---|---|---|
| `EVENT` | `02`, `04`, `05`, `10`, `17` | Physical, epistemic, ordinary-life, temporal, and diagnostic changes share stable causal event ancestry. |
| `TRUTH-FIREWALL` | `03`, `04`, `06`, `07`, `11`, `12`, `17` | Truth validates but does not choose missing-property, ordinary-life, temporal, suspect, or embodied targets. |
| `ACTOR-KNOWN` | `03`, `04`, `05`, `07`, `12`, `17` | Expectation, observation, contradiction, planning, and temporal premises are source-backed and holder-scoped. |
| `POSSESSION-PARITY` | `08`, `14`, `17` | Controller binding changes input only across the complete corpus. |
| `NO-HUMAN-ORDINARY-LIFE` | `09`, `13`, `17` | The integrated world advances through real action/wait/stuck paths without a human special case. |
| `MISSING-PROPERTY` | `03`, `04`, `05`, `06`, `17` | Expectation -> observed absence -> contradiction, with no culprit truth or investigation machinery. |
| `VIEW-DEBUG-SPLIT` | `08`, `14`, `17` | Embodied output remains holder-known; debug is non-diegetic and non-influential. |
| `REPLAY` | `02`, `05`, `10`, `12`–`17` | The complete corpus rebuilds projections and deterministic diagnostics with first-divergence reporting. |
| `FIXTURE-NEGATIVE` | `06`, `11`, `15`, `17` | Positive and forbidden paths cover hidden truth, direct dispatch, human privilege, debug leakage, culprit/quest data, marker life, temporal contamination, and replay drift. |

### 8.2 Audit-point to nine-scenario-family coverage

| Audit point | `Physical custody baseline` | `Expectation contradiction` | `Possession parity` | `Epistemic filtering` | `No-hidden-truth planning` | `No-human ordinary day` | `Routine blocking` | `Replay rebuild` | `Content rejection` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `FIRST-PROOF-01` | I | I | I | I | I | I | I | I | I |
| `FIRST-PROOF-02` | P | I | I | I | — | — | — | P | — |
| `FIRST-PROOF-03` | — | P | — | P | I | — | — | I | I |
| `FIRST-PROOF-04` | I | P | — | P | P | — | — | I | — |
| `FIRST-PROOF-05` | — | P | — | P | — | — | — | P | — |
| `FIRST-PROOF-06` | — | P | — | I | P | — | — | — | P |
| `FIRST-PROOF-07` | — | I | — | P | P | I | — | I | — |
| `FIRST-PROOF-08` | I | I | P | P | — | — | — | I | — |
| `FIRST-PROOF-09` | I | I | I | — | I | P | P | I | — |
| `FIRST-PROOF-10` | P | P | I | I | I | P | I | P | I |
| `FIRST-PROOF-11` | — | I | — | I | P | — | — | I | P |
| `FIRST-PROOF-12` | — | I | — | P | P | I | I | P | — |
| `FIRST-PROOF-13` | — | — | — | — | I | P | P | P | — |
| `FIRST-PROOF-14` | — | — | P | P | I | — | — | I | — |
| `FIRST-PROOF-15` | — | I | I | I | P | P | P | P | P |
| `FIRST-PROOF-16` | — | I | I | I | I | I | P | P | I |
| `FIRST-PROOF-17` | P | P | P | P | P | P | P | P | P |

### 8.3 Scenario-family evidence plan

| Scenario family | Required positive fixtures/witnesses | Required adversarial or relational witnesses | Minimum audit points |
|---|---|---|---|
| Physical custody baseline | `strongbox_001`, `container_item_move_001`, `door_access_001`, `replay_item_location_001`, physical portion of `expectation_contradiction_001` | inaccessible/nonlocal/locked check; hidden custody permutation; no teleport; debug-only location excluded | `02`, `10`, `17` |
| Expectation contradiction | `expectation_contradiction_001` with source-backed expectation, modeled check, absence observation, contradiction, and belief update | item-present dual; wrong holder/item/container; missing source; duplicate/reordered expectations; no culprit truth | `03`–`06`, `10`, `17` |
| Possession parity | `possession_parity_001`, `possession_does_not_reset_intention_001`, equivalent autonomous/human command path | bind/unbind/rebind; different possessed actor; no actor possessed; no hidden memory/action/truth grant | `08`, `14`, `17` |
| Epistemic filtering | `view_filtering_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, holder-known contradiction rendering | hidden item/workplace/menu truth change without perception; wrong-holder source; debug attach/off relation | `03`–`05`, `07`, `08`, `14`, `17` |
| No-hidden-truth planning | `no_hidden_truth_planning_001`, legal actor-known target selection after observation | hidden food/container/route/workplace/custody/time changes; forbidden provenance; no teleport; validator cannot propose | `03`, `04`, `06`, `07`, `12`, `17` |
| No-human ordinary day | `no_human_day_001`, `no_human_advance_001`, `food_unavailable_replan_001`, integrated contradiction ordinary-day run | possession/no-possession relation; empty/hidden food; no direct dispatch; no marker-only progress; no human special case | `09`, `13`, `17` |
| Routine blocking | `routine_blocked_diagnostic_001`, legal wait/fallback/stuck path | `routine_no_teleport_001`; silent spin; rewritten wait reason; stale true-time fallback; marker progress | `09`, `13`, `16`, `17` |
| Replay rebuild | `replay_item_location_001`, every golden fixture, integrated capstone | dropped/reordered/changed event; unknown version; changed fixture; equal physical state but unequal epistemic history | `05`, `10`, `12`–`17` |
| Content rejection | `fixtures_load` and supported schema corpus | `prose_born_fact_rejected_001`; forbidden provenance; typed-unproven truth; culprit/quest/outcome aliases; compile-fail forgery | `03`, `06`, `11`, `15`, `17` |

The implementing session must prove fixture registry reachability and branch reachability. A fixture file that loads but does not exercise the intended path is not certifying evidence.

### 8.4 Required adversarial property inventory

The combined packet must include explicit evidence for all of the following, whether by named fixture, targeted test, compile-fail test, or generated relation:

| Required negative | Minimum evidence |
|---|---|
| No culprit/suspect/clue/quest truth | Semantic content rejection, projection census, actor-decision hidden-custody relation, and canonical fixture witness. |
| No hidden-truth planning | Paired hidden food/route/workplace/custody/time runs with equal actor-known outputs before legal evidence. |
| No teleport to true food/property | Routine/action/path witness requiring legal movement/access; absence of accepted remote mutation event. |
| Possession does not reset or privilege | Pre/post bind state/context/event comparison and different-controller relation. |
| No human special case | Same world/validation behavior with any actor or no actor bound. |
| No debug leakage | Debug off/on equality for authoritative log and actor-known outputs; debug-only view separation. |
| No direct dispatch | Scheduler-to-transaction-to-proposal-to-pipeline witness and guard against primitive action/state-delta emission. |
| No marker-only ordinary life | Independent event classification; markers cannot increase progress. |
| No silent stall | Modeled wait or typed stuck/failure with responsible layer; supervisor distinguishes timeout from modeled wait. |
| No replay drift | Per-projection live/replay equality and controlled tamper first-divergence witness. |
| No prose-born fact | Content/schema rejection and no partial runtime state. |
| No source-free expectation/observation/contradiction | Runtime fail-closed plus external compile-fail boundaries. |
| No temporal wallhack | True clock/schedule/debug change without source leaves actor/view output unchanged. |
| No stale-premise auto-correction | Older source remains stale until modeled superseding information event. |
| No crate-direction inversion | Workspace dependency/conformance evidence at `U`. |

### 8.5 Consolidated temporal evidence bundle

The five routed sources are not five independent attachments. They form one causal chain whose links must be mutually consistent at `U`.

| Routed execution source | Owning obligation | Primary audit point | Required evidence item | Integrated acceptance line |
|---|---|---|---|---|
| `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | Temporal premise is holder-known, source-backed, freshness-aware, and not born from raw/validator/debug time. | `FIRST-PROOF-12` | Temporal-firewall paired run, source-event ancestry, context hash/frontier, fail-closed negative. | The premise used by cognition in the capstone is the same premise rendered, exercised, diagnosed, and replayed below. |
| `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | Routine selection, wait, interruption, adaptation, and accounting use modeled temporal premises. | `FIRST-PROOF-13` | Routine trace, premise source, ordinary event/wait/stuck outcome, accounting ledger, replay. | The source-backed premise produces a legal ordinary-life behavior without direct dispatch or true-time fallback. |
| `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Embodied temporal language/affordance reflects holder-known evidence; possession does not refresh; debug is quarantined. | `FIRST-PROOF-14` | View-model field/source citation, possession/debug relation, transcript/replay fingerprint. | The same modeled premise is actor-legible only through the allowed view and remains unchanged by possession/debug. |
| `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Positive and adversarial temporal fixtures are registry-reachable, semantically paired, and replayable. | `FIRST-PROOF-15` | Fixture IDs/fingerprints, positive/adversarial trigger, event/projection/replay records. | The premise/behavior/view chain is exercised by committed positive and anti-contamination fixtures. |
| `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Diagnostics distinguish holder-known premise from validator time, name the layer, and replay deterministically. | `FIRST-PROOF-16` | Typed diagnostic, first divergence where relevant, evidence-status/scope fields, bundle reconciliation row. | One acceptance row links source -> routine behavior -> embodied rendering -> fixture -> diagnostic/replay without inventing temporal vocabulary. |

The bundle fails if any routed source is missing, if the evidence items refer to different baselines or scenario semantics, or if they merely coexist without a source-to-behavior-to-view-to-fixture-to-diagnostic trace.


## 9. Property-based, metamorphic, and sampling posture

The generated/metamorphic relations in `FIRST-PROOF-17` are mandatory because ordinary example tests can accidentally share the same hidden assumption. They are evidence amplifiers, not replacements for the named fixtures.

The acceptance artifact must record for each generated family:

- generator source path and version/hash;
- input domain and legality constraints;
- actor/item/place/container/time ranges;
- seed;
- number of generated and discarded cases;
- branch/reachability counters;
- shrink result and minimal counterexample;
- relation asserted;
- projections/checksums compared;
- whether the declared perimeter is finite and exhaustively enumerated or sampled;
- any population omitted and why.

A sampled property may support a pass only beside direct certifying witnesses for the same semantic claim. It may not be presented as exhaustive. A vacuous property—one whose precondition discards every intended case, never reaches contradiction creation, or never reaches the validation branch—fails the test-oracle requirement.

At least one generated family must compose all of these in one run: source-backed expectation; hidden-custody variation; legal check; presence/absence dual; contradiction creation; possession/debug toggle; ordinary no-human advancement; replay rebuild. The relation need not be a new gameplay fixture, but it must exercise the public/runtime path rather than directly constructing protected state.


## 10. Mutation-testing posture

Mutation testing is a certification requirement for the guarded first-proof integration layer. It is not a quality score and cannot be replaced by line coverage, a baseline file, an in-diff-only run, or predecessor mutation results.

### 10.1 Scope principle

The campaign has two required waves:

1. **Standing configured perimeter.** Run the checked-in workspace configuration to prove that the unified baseline has no cross-gate regression in the protected layer. This is integration evidence; it does not reopen every predecessor subsystem's design.
2. **Focused FIRST-PROOF wave.** Emphasize the genuinely new missing-property path and its integration carriers: expectation/observation/contradiction, check/perception, event application/projection/replay, fixture validation, integration harness, and any temporal-bundle adapters identified by the census.

The final `--list-files` and `--list` output—not assumptions about globs—determines coverage. If an intended file is absent, the implementing session must make the checked-in perimeter honest or run an explicit focused file selection and record why. A configuration change becomes part of `U`.

### 10.2 Minimum semantic union

The mutation census must include, at minimum, the live carriers under `U` corresponding to:

```text
crates/tracewake-core/src/epistemics/belief.rs
crates/tracewake-core/src/epistemics/contradiction.rs
crates/tracewake-core/src/epistemics/observation.rs
crates/tracewake-core/src/epistemics/proposition.rs
crates/tracewake-core/src/epistemics/knowledge_basis.rs
crates/tracewake-core/src/epistemics/knowledge_context.rs
crates/tracewake-core/src/epistemics/projection.rs
crates/tracewake-core/src/agent/perception.rs
crates/tracewake-core/src/agent/actor_known.rs
crates/tracewake-core/src/agent/transaction.rs
crates/tracewake-core/src/agent/decision.rs
crates/tracewake-core/src/agent/no_human_surface.rs
crates/tracewake-core/src/actions/defs/checkcontainer.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/events/envelope.rs
crates/tracewake-core/src/events/apply.rs
crates/tracewake-core/src/events/log.rs
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/replay/rebuild.rs
crates/tracewake-core/src/replay/report.rs
crates/tracewake-core/src/checksum.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/time.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/src/debug_reports.rs
crates/tracewake-content/src/load.rs
crates/tracewake-content/src/schema.rs
crates/tracewake-content/src/validate.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/src/debug_panels.rs
crates/tracewake-tui/src/render.rs
crates/tracewake-tui/src/transcript.rs
```

Test-only integration assertions are not themselves the protected production layer, but the mutation evidence must name which tests kill each critical mutant and which audit point/gate/family the kill supports.

### 10.3 Exact mutation commands

Run the census first:

```bash
cargo mutants --workspace --no-shuffle --list-files \
  > reports/0044_first_proof_cert_mutation_list_files.txt

cargo mutants --workspace --no-shuffle --list \
  > reports/0044_first_proof_cert_mutation_list.txt
```

Run the complete checked-in perimeter under command supervision:

```bash
cargo mutants --workspace --no-shuffle \
  -o reports/0044_first_proof_cert_mutation_full.out
```

Run the focused new-surface wave under command supervision:

```bash
cargo mutants --workspace --no-shuffle \
  -f 'crates/tracewake-core/src/epistemics/contradiction.rs' \
  -f 'crates/tracewake-core/src/epistemics/projection.rs' \
  -f 'crates/tracewake-core/src/agent/perception.rs' \
  -f 'crates/tracewake-core/src/actions/defs/checkcontainer.rs' \
  -f 'crates/tracewake-core/src/events/apply.rs' \
  -f 'crates/tracewake-core/src/replay/rebuild.rs' \
  -f 'crates/tracewake-content/src/validate.rs' \
  -o reports/0044_first_proof_cert_mutation_focused.out
```

If the final symbol/path census identifies additional direct integration carriers, append them to the focused command and record the exact final command. Do not silently substitute a smaller command.

### 10.4 Completion and triage

For each campaign, package `mutants.json`, `outcomes.json`, `caught.txt`, `missed.txt`, `timeout.txt`, `unviable.txt`, `debug.log`, command transcript, supervisor metadata, source/config hashes, and exact cargo-mutants version. Reconcile counts across all files.

Reuse the survivor-triage shape established by the 0040 and 0043 registers. Every non-caught outcome must record:

- stable mutant identity;
- exact path, line/symbol, and mutation operator;
- campaign and command;
- audit points, first-proof gates, scenario families, and temporal source affected;
- reproduction command;
- behavior that should distinguish original from mutant;
- current test reachability and assertion;
- classification supported by tool/source evidence;
- disposition and owner;
- linked killing evidence or reason the mutant is mechanically unviable;
- whether it blocks certification.

`missed`, unresolved `timeout`, tool failure, incomplete campaign, lost output, irreconcilable count, or an actionable survivor prevents `FIRST-PROOF-CERT passed`. A mechanically unviable mutant may be classified only with concrete compile/tool evidence. A survivor may not be waived by asserting that a predecessor gate once passed.

A failed floor yields the posture `FIRST-PROOF-CERT scoped remediation` in the implementing session's artifact and routes to a later, separately numbered mutation-remediation and replacement-certification spec. This audit does not author that future spec.


## 11. Failure handling and responsible-layer diagnostics

A failure must be reported at the earliest responsible layer. It may not be relabeled as a phase exception, a fixture quirk, an emergence shortfall, or “expected because this is only the first proof.”

Canonical diagnostic layers used by this spec are:

`doctrine`, `content_schema`, `content_validation`, `fixture_contract`, `holder_known_context`, `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `view_model`, `debug_quarantine`, `tui_input_binding`, and `test_oracle`.

Where the ladder uses a broader label—such as doctrine mismatch, planning/method selection, projection/replay, tests/fixtures, or documentation status—the artifact must record both the ladder category and the canonical responsible layer when available.

### 11.1 Point-to-layer failure map

| Audit point | First responsible layers to consider |
|---|---|
| `FIRST-PROOF-01` | `doctrine`, `fixture_contract`, `test_oracle`, documentation status |
| `02` | `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `view_model`, `replay` |
| `03` | `content_schema`, `content_validation`, `fixture_contract`, `event_application`, `projection`, `holder_known_context` |
| `04` | `holder_known_context`, `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection` |
| `05` | `event_append`, `event_application`, `projection`, `holder_known_context`, `replay`, `test_oracle` |
| `06` | `doctrine`, `content_schema`, `content_validation`, `holder_known_context`, `candidate_generation`, `view_model`, `debug_quarantine` |
| `07` | `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation` |
| `08` | `holder_known_context`, `intention_lifecycle`, `projection`, `view_model`, `debug_quarantine`, `tui_input_binding` |
| `09` | `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `scheduler`, `action_validation`, `event_application` |
| `10` | `event_append`, `event_application`, `projection`, `replay`, `test_oracle` |
| `11` | `content_schema`, `content_validation`, `fixture_contract`, `test_oracle` |
| `12` | `holder_known_context`, `candidate_generation`, `method_selection`, `proposal_construction`, `projection`, `replay` |
| `13` | `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `scheduler`, `event_application`, `projection` |
| `14` | `holder_known_context`, `projection`, `view_model`, `debug_quarantine`, `tui_input_binding` |
| `15` | `content_schema`, `content_validation`, `fixture_contract`, `projection`, `replay`, `test_oracle` |
| `16` | owning behavior layer plus `replay`, `view_model`, `debug_quarantine`, or `test_oracle` |
| `17` | earliest behavior layer reached; `test_oracle` for vacuity, generator fidelity, or assertion-scope failure |

### 11.2 Required failure record

Every failed witness must state:

- audit point, gate, scenario family, and temporal source if applicable;
- unified baseline SHA and exact command;
- fixture/generator seed and input;
- expected and actual behavior;
- responsible layer and component;
- last valid source/provenance/event;
- accepted event or proof that no event should exist;
- live/replay projection and diagnostic comparison;
- first divergence;
- whether the failure is doctrine, implementation, evidence instrumentation, test oracle, content, tool, or mutation-floor related;
- minimum remediation boundary without designing the remediation.

A substantive implementation or mutation failure ends the certification pass path. The implementing session may finish evidence collection needed to characterize the failure, but it may not conceal or repair the defect under this audit spec. The result routes to a separate FIRST-PROOF-CERT remediation/replacement campaign.


## 12. Acceptance-artifact contract

The implementing session must create a scoped artifact at a path such as:

```text
reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md
```

The exact filename may be normalized to repository convention, but it must remain unambiguously tied to spec 0044. The artifact must instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and may not claim a verdict before the evidence is complete.

### 12.1 Header and provenance

Record:

- spec path and final archived path if closeout occurs;
- repository;
- authoring target commit `1541da274180ecd40f52583d86704990cb55e74c`;
- exact unified code/evidence commit `U`;
- any later documentation-only closeout commit;
- clean-worktree evidence;
- toolchain and cargo-mutants versions;
- Cargo.lock, toolchain, workspace, CI, mutation-config, fixture-registry, and test-source fingerprints;
- source-discipline statement;
- predecessor artifact paths, exact commits, scopes, and “consumed, not rerun as isolated subsystem audits” posture;
- freshness statement: not independently verified as latest `main`;
- explicit statement that archived implementation specs are historical only.

### 12.2 Gate, scenario, audit-point, and temporal results

Include four reconciled tables:

1. `FIRST-PROOF-01` through `FIRST-PROOF-17`;
2. all nine first-proof gates;
3. all nine first-proof scenario families;
4. the five temporal-cascade sources.

Each row must cite evidence item IDs. Only evidence marked certifying under the live evidence-status rules may determine a pass. Pending, historical, sampled-only, or observer-only rows cannot silently produce a pass.

### 12.3 Evidence item ledger fields

For every cited item record:

- **Evidence item ID**
- **Requirement/audit-point/gate/scenario IDs**
- **Evidence status**
- **Fingerprint scope**
- **Evidence summary**
- **Path under test and behavior witness**
  - path and symbol/behavior seam;
  - command, trigger, event, emitter, scheduler entry, or TUI input;
  - responsible layer;
  - accepted/rejected action or validation stage;
  - live negative or reason no negative applies;
  - checked facts/invariants.
- **Replay/provenance ancestry**
  - fixture/content/ruleset version and fingerprint;
  - seed/randomness source where applicable;
  - event-log segment and IDs;
  - source references and holder-known context identity;
  - projection/extraction version;
  - live/replay checksums and first divergence.
- **Sampling/exhaustiveness scope**
- **Pending or historical handling**
- **Certification use**

Fingerprints must never be cited beyond their scope. A fixture fingerprint is not a behavior witness; a transcript snapshot is not authoritative world state; a debug artifact is not actor knowledge.

### 12.4 Mandatory behavior witnesses

At minimum, the packet must contain separately inspectable witnesses for:

1. physical custody and legal container check;
2. source-backed expectation;
3. modeled absence observation;
4. event-sourced expectation contradiction and belief update;
5. no culprit/suspect/clue/quest truth;
6. hidden-custody/food/route/workplace/time non-interference;
7. possession parity and possession-does-not-reset;
8. debug off/on non-interference and embodied filtering;
9. no-human ordinary day with real action/wait/stuck progress;
10. routine blocker and no teleport;
11. temporal five-source chain;
12. complete replay rebuild;
13. controlled replay tamper with first divergence;
14. content/schema/compile-fail rejection;
15. complete configured and focused mutation results;
16. full cross-gate generated/metamorphic relation.

### 12.5 Replay package

For each required fixture and the integrated capstone, record:

- fixture identifier and canonical semantic fingerprint;
- initial state/config/content/ruleset/seed;
- event count and ordered event-kind census;
- stable event IDs and causal roots;
- source/provenance refs;
- physical, epistemic, agent/accounting, diagnostic, and embodied-support live checksums;
- corresponding replay checksums;
- serialized log/report hashes;
- first-divergence result;
- whether replay began from empty projections;
- any non-authoritative debug/observer artifacts and their exclusion from authoritative checksums.

### 12.6 Command and mutation package

Include exact command transcripts for section 5 and section 10, supervisor metadata, exit status, timing, environment/tool versions, output artifact hashes, and count reconciliation. The packet must make it impossible to confuse a modeled wait with a hung test or mutation process.

### 12.7 Staged-abstraction declaration

If any bounded abstraction is used, state:

- what is proven now;
- what is deliberately abstracted;
- what must not be faked;
- which future feature/tier is not certified by implication;
- evidence preventing overclaim;
- diagnostics distinguishing not implemented, intentionally abstracted, implemented but broken, and overclaimed.

No staged abstraction may defer the missing-property chain, the nine gates, the nine scenario families, the temporal bundle, mutation completion, or replay determinism while still producing `FIRST-PROOF-CERT passed`.

### 12.8 EMERGE-OBS package member

Because the corpus exercises the first-proof living world, the package must include the observer-only `EMERGE-OBS` artifact produced under execution document `10`.

The artifact must be labeled `observer-only`, retrospective, event-log-ancestry-backed, and non-input. Its metrics, patterns, narrative interpretations, or comparative values:

- are not a gate;
- are not a pass/fail threshold;
- are not a mutation score;
- are not a scheduler/planner objective;
- are not a fixture expected outcome;
- are not a scenario goal;
- are not evidence that substitutes for any audit point.

The evidence package is incomplete if the required observer artifact is omitted, but no observed value inside it can make a gate pass or fail.

### 12.9 Verdict rules

This spec itself renders no verdict. The implementing session may write `FIRST-PROOF-CERT passed` only when all of the following are true at one `U`:

- all `FIRST-PROOF-01` through `FIRST-PROOF-17` have certifying evidence;
- all nine gates pass as one coherent artifact set;
- all nine scenario families have positive and required adversarial evidence;
- the five-source temporal bundle closes;
- workspace and every named command pass;
- replay/projection/diagnostic determinism and first-divergence evidence pass;
- mutation campaigns complete with no actionable missed or unresolved timeout/tool floor;
- every failure/negative names the responsible layer;
- evidence-status and fingerprint-scope rules are honest;
- `EMERGE-OBS` is present and observer-only;
- no pending required item, evidence splice, or substantive deferral remains.

If the behavioral audit exposes a defect, the artifact must name the failed point/layer and must not claim pass. If the configured mutation lane leaves a survivor floor or cannot complete, the appropriate posture is `FIRST-PROOF-CERT scoped remediation`, routed to a later separate remediation and replacement-certification spec. This audit does not pre-author that spec.

Allowed scope wording must say that the result applies only to the exact unified commit and the FIRST-PROOF-CERT contract. It must not say “latest main,” “fully certified,” “Phase 4 passed,” “second proof passed,” or imply that deferred institutions, travel, LOD, story-sifting, or LLM dialogue are certified.


## 13. Preliminary static survey — informative, not certification

This section records only what the exact-commit source reading suggests should be reachable. It is **not** a verdict, does not count as an evidence item, and must not be copied into a pass row without live execution at `U`.

1. [crates/tracewake-core/src/epistemics/belief.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-core/src/epistemics/belief.rs) contains a typed expectation stance and source-bearing belief structure rather than a free-form “missing item” flag.
2. [crates/tracewake-core/src/epistemics/observation.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-core/src/epistemics/observation.rs) represents observations with modeled channels, source references, holder scope, and observed time.
3. [crates/tracewake-core/src/epistemics/contradiction.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-core/src/epistemics/contradiction.rs) contains a typed expected-item-absent contradiction path that links a prior expectation and a contradicting observation and appears designed to produce a source-backed missing belief.
4. [crates/tracewake-core/src/agent/perception.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-core/src/agent/perception.rs) and [crates/tracewake-core/src/actions/defs/checkcontainer.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-core/src/actions/defs/checkcontainer.rs) provide plausible modeled perception/check carriers rather than a direct truth notification.
5. [crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs) places the item in physical custody distinct from the expecting actor's source-backed expectation and describes check/observation/contradiction/belief-update events without a culprit field.
6. The fixture's historical Phase-2A lineage and the existence of code are not live certification. The integrated path, registry reachability, negative branches, temporal bundle, TUI participation, replay, and mutation kill set remain to be proven.
7. The checked-in mutation configuration appears broad, but static reading cannot prove that every new integration carrier—especially `actions/defs/checkcontainer.rs` and the final harness path—is in the actual census. `--list-files` and the focused campaign are therefore mandatory.

This survey identifies plausible seams and risks only. It neither predicts nor asserts `FIRST-PROOF-CERT passed`.


## 14. Tolerated deferrals and explicit non-goals

The following are outside `FIRST-PROOF-CERT` and must not be audited, implemented, or implied by this result:

| Deferred scope | Route |
|---|---|
| Institutions, formal records, wrong-suspicion procedures, full investigation, reports, sanctions, and local institutional process | `PHASE-4-ENTRY` under [docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md) |
| New notice systems, regional travel, regional scale, LOD/time acceleration, long-history generation, and story-sifting projections | `SECOND-PROOF-ENTRY` under [docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/1541da274180ecd40f52583d86704990cb55e74c/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md) |
| Quest boards, clue chains, culprit scripting, adventure routes, protagonist-centered plot | Not part of the first proof; any future proposal remains subordinate to the no-scripting and phase-entry doctrine. |
| LLM dialogue, language-model-authored state, or LLM cognition authority | Locked by foundation/architecture/execution speech boundaries. |
| Graphical client work | Explicit first-proof non-goal. |
| Latest-main verification | Not performed by this workflow. It would require an independently supplied current-main SHA and matching manifest. |
| Re-audit of the four predecessor gates' complete internals | Already certified within their exact artifact scopes. This spec tests only their participation and cross-gate coherence at `U`. |
| A FIRST-PROOF-CERT mutation-remediation/replacement spec | Authored only after an actual failed floor; not anticipated or embedded here. |

Existing temporal notices or records used as source-backed fixture evidence may be exercised only to prove the currently implemented provenance contract. Doing so does not authorize a new notice-board system, institution procedure, or second-proof scope.


## 15. External research basis — method only, never repository state

External sources inform test design only. They do not establish what exists in Tracewake, do not override repository doctrine, and cannot substitute for an unfetched repository file.

1. **Partial observability and information state.** Kaelbling, Littman, and Cassandra describe decision-making under partial observability using the history of actions/observations and an information or belief state rather than direct access to hidden world state. This supports paired-run tests where hidden custody changes but actor-known inputs do not. Tracewake is not being claimed to implement a POMDP; the research sharpens the anti-contamination test relation.  
   Source: Leslie P. Kaelbling, Michael L. Littman, and Anthony R. Cassandra, “Planning and Acting in Partially Observable Stochastic Domains,” *Artificial Intelligence* 101 (1998), [author-hosted paper](https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf), [DOI](https://doi.org/10.1016/S0004-3702(98)00023-X).

2. **Reason-maintenance and explicit dependency records.** Doyle's truth-maintenance work records reasons for beliefs so contradictions and revisions remain inspectable rather than becoming unexplained overwrites. This supports the audit requirement that expectation, observation, contradiction, and revised belief retain explicit links. Tracewake's event/provenance doctrine remains controlling and is stricter about actor scope and causal replay.  
   Source: Jon Doyle, “A Truth Maintenance System,” *Artificial Intelligence* 12(3) (1979), [MIT record](https://dspace.mit.edu/handle/1721.1/5733), [DOI](https://doi.org/10.1016/0004-3702(79)90008-0).

3. **Property-based testing.** QuickCheck's central method—state general properties and generate many inputs with shrinking—supports source-deletion, holder/item permutation, presence/absence dual, and replay-partition relations. Generated evidence remains sampled unless the finite perimeter is exhaustively enumerated.  
   Source: Koen Claessen and John Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs,” ICFP 2000, [DOI](https://doi.org/10.1145/351240.351266).

4. **Metamorphic testing.** Metamorphic testing tests relations between executions when a single oracle is difficult. That directly motivates hidden-truth non-interference, debug/possession non-interference, temporal legal-update, and replay partition relations.  
   Source: T. Y. Chen, S. C. Cheung, and S. M. Yiu, “Metamorphic Testing: A New Approach for Generating Next Test Cases,” [technical report](https://www.cse.ust.hk/faculty/scc/publ/CS98-01-metamorphictesting.pdf).

5. **Interaction faults and integration coverage.** NIST research on combinatorial interaction testing supports explicitly covering cross-subsystem factor interactions rather than assuming independently tested components compose. This spec therefore requires a gate-by-scenario matrix and cross-gate relation set; it does not use pairwise coverage as a substitute for the mandatory complete corpus.  
   Source: D. Richard Kuhn, Dolores R. Wallace, and Albert M. Gallo Jr., “Software Fault Interactions and Implications for Software Testing,” *IEEE Transactions on Software Engineering* 30(6), [NIST preprint](https://csrc.nist.gov/CSRC/media/Projects/automated-combinatorial-testing-for-software/documents/kuhn-wallace-gallo-tse-preprint.pdf), [DOI](https://doi.org/10.1109/TSE.2004.24).

6. **Liveness assumptions must be explicit.** Work distinguishing progress, justness, and fairness shows that liveness conclusions depend on the execution assumptions being made. This supports separating a legitimate modeled wait from a silently stalled runner, and recording the exact progress/fairness premise rather than inferring “the world is alive” from elapsed time. Tracewake's typed progress/stuck doctrine defines the actual acceptance rule.  
   Source: Rob van Glabbeek and Peter Höfner, “Progress, Justness, and Fairness,” *ACM Computing Surveys* 52(4), [arXiv](https://arxiv.org/abs/1810.07414), [DOI](https://doi.org/10.1145/3329125).

7. **Deterministic replay and localization.** Record/replay work emphasizes reproducing executions so failures can be diagnosed rather than merely observed once. Tracewake's internal doctrine is stronger: it requires deterministic projection/diagnostic rebuild and first-divergence location. External research supports the diagnostic purpose, while repository doctrine defines the exact acceptance contract.  
   Source: Gautam Altekar and Ion Stoica, “ODR: Output-Deterministic Replay for Multicore Debugging,” SOSP 2009, [paper](https://www.sigops.org/s/conferences/sosp/2009/papers/altekar-sosp09.pdf), [DOI](https://doi.org/10.1145/1629575.1629594).

8. **Mutation testing mechanics.** The cargo-mutants documentation distinguishes mutant listing, result artifacts, missed mutants, timeouts, and unviable mutants. This supports the required census, complete result package, and explicit timeout/survivor triage. Repository doctrine—not the tool documentation—sets the zero-actionable-floor certification rule.  
   Sources: [cargo-mutants getting started](https://mutants.rs/getting-started.html), [output files](https://mutants.rs/mutants-out.html), and [timeouts](https://mutants.rs/timeouts.html).

9. **Goodhart risk.** Manheim and Garrabrant analyze how proxy measures can fail under optimization pressure. This supports keeping `EMERGE-OBS` descriptive and outside scheduler objectives, scenario goals, and pass/fail thresholds. The binding rule remains `INV-111` and execution document `10`.  
   Source: David Manheim and Scott Garrabrant, “Categorizing Variants of Goodhart's Law,” [arXiv:1803.04585](https://arxiv.org/abs/1803.04585).

These sources shape the audit method only: paired interventions, explicit dependency links, relational oracles, interaction matrices, explicit liveness assumptions, replay localization, mutation census/triage, and observer-only metrics.

## 16. Implementing-session completion checklist

Before issuing an artifact verdict, confirm:

- [ ] One immutable unified baseline `U` is named; no certifying evidence is spliced across commits.
- [ ] The authoring target and any audit-only deltas are enumerated.
- [ ] All mandatory workspace, core, content, TUI, replay, and mutation commands ran at `U`.
- [ ] `FIRST-PROOF-01` through `FIRST-PROOF-17` each have positive, adversarial, replay/projection, diagnostic, and command evidence.
- [ ] All nine first-proof gates are present in the gate table and have certifying evidence.
- [ ] All nine scenario families are present and registry/branch reachable.
- [ ] Source-backed expectation is proven.
- [ ] Absence is discovered by modeled observation, not truth notification.
- [ ] Expectation contradiction is event-sourced, holder-scoped, linked, and replayable.
- [ ] No culprit/suspect/clue/quest/story-sifting truth is present or used.
- [ ] Hidden custody/food/route/workplace/time changes do not alter cognition before legal evidence.
- [ ] Possession changes input routing only and does not reset or privilege the actor.
- [ ] Debug is non-diegetic and has no effect on authoritative or actor-known output.
- [ ] No-human ordinary life uses normal transaction/pipeline paths and no human special case.
- [ ] Blocking is a modeled wait/fallback/stuck/failure with a responsible layer, not a silent stall.
- [ ] Marker-only activity is excluded from progress.
- [ ] Replay rebuilds every authoritative and required diagnostic projection and localizes a controlled divergence.
- [ ] Positive and semantic adversarial content/compile-fail fixtures pass.
- [ ] Temporal evidence from execution `04`, `06`, `07`, `09`, and `10` is linked into one coherent bundle.
- [ ] No new temporal unit, threshold, source category, gate code, invariant, status enum, or obligation code was invented.
- [ ] The configured and focused mutation campaigns complete and reconcile with no actionable floor.
- [ ] `EMERGE-OBS` is present, event-ancestry-backed, and observer-only; no value is used as a threshold or input.
- [ ] Every evidence item declares status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical handling, certification use, and staged-abstraction posture.
- [ ] All deferrals are named and routed; no Phase-4 or second-proof scope is implied.
- [ ] The wording says exact-commit scoped certification only and makes no latest-main claim.

## 17. Required outcome of this spec

This document commissions one non-executable audit. The implementing session must either:

1. produce the complete artifact set and, only if every rule above is satisfied, render `FIRST-PROOF-CERT passed` for exact unified baseline `U`; or
2. produce an evidence-complete failed/scoped artifact naming the failing points and responsible layers, with `FIRST-PROOF-CERT scoped remediation` where an actionable mutation or implementation floor remains.

No result from this specification authoring session is a pass/fail verdict. No remediation spec is included here.

## Outcome

Completed: 2026-06-21

The `0044FIRPROCER` ticket series produced the FIRST-PROOF-CERT acceptance
artifact and supporting mutation package:

- `reports/0044_first_proof_cert_missing_property_coherent_gate_set_temporal_bundle_and_integrated_acceptance_certification_acceptance.md`
- `reports/0044_first_proof_cert_mutation_triage_register.md`
- `reports/0044_first_proof_cert_mutation_list_files.txt`
- `reports/0044_first_proof_cert_mutation_list.txt`
- supervisor transcripts under `reports/0044_first_proof_cert_mutation_*.supervisor/`

The implementing session rendered `FIRST-PROOF-CERT scoped remediation`, not
`FIRST-PROOF-CERT passed`. Behavioral audit points, gates, scenario families,
and the temporal bundle were reconciled as passing for their behavioral scope,
and the focused mutation campaign completed with 719 mutants tested, 600 caught,
119 unviable, 0 missed, and 0 timeouts. The configured standing mutation
campaign timed out after classifying 2384 of 2901 mutants, so the mutation-floor
evidence did not complete. Per this spec's section 10, section 12.9, and section
17, that incomplete configured campaign blocks a pass verdict and routes to a
later separately numbered FIRST-PROOF-CERT mutation remediation/replacement
campaign.

Verification results recorded by the capstone ticket:

- `cargo test --locked -p tracewake-core --test acceptance_gates`
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
- `cargo test --locked -p tracewake-core --test emergence_ledger`
- `cargo test --workspace --locked`

All four verification commands passed on 2026-06-21. The result is exact-commit
scoped to the documented artifact baseline and does not certify latest main,
Phase 4 entry, second-proof entry, or future feature surfaces.
