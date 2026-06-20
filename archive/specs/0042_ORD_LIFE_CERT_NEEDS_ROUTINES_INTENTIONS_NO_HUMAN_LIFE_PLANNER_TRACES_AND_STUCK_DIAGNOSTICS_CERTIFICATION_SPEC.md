# 0042 ORD-LIFE-CERT needs, routines, intentions, no-human life, planner traces, and stuck-diagnostics certification spec

```text
Staging path: specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md
Archive path on accepted closeout: archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md
Target repository: joeloverbeck/tracewake
Target commit: 98dc0421211e6c9881d9c6679b9df74525e392bb
Spec series: numbered staging spec 0042; archived to archive/specs/ on accepted closeout
Status: ACCEPTED — executed by the `0042ORDLIFCER` ticket series; verdict `ORD-LIFE-CERT scoped remediation`
Work posture: Certification
Admissibility posture: consumes P0-CERT passed from archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md, SPINE-CERT passed from archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md, and EPI-CERT passed from archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md
Phase-certification label: ORD-LIFE-CERT, as defined by docs/2-execution/03 and docs/2-execution/06; this document mints no new gate code, invariant ID, status enum, or obligation code
Assumption: stage under specs/ through the repository's stage-then-archive convention; relocation does not alter the target commit or audit semantics
```

This document is **non-executable**: it specifies what the implementing session must run, prove, record, and package. It renders no pass/fail verdict, does not assert that the target implementation passes, and does not authorize production remediation inside this audit spec.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.



## 1. Determination confirmation: ORD-LIFE-CERT is the next admissible move

`ORD-LIFE-CERT` is the single next admissible Tracewake staging spec. The live phase ladder orders certification as `P0-DOC -> P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY` and defines gate 4 as certifying needs, routines, intentions, no-human ordinary life, planner traces, and stuck diagnostics under the actor-known transaction. It also blocks Phase-4 and expansion work until the preceding certification gates pass. See [docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).

The live ledger records the 0037 replacement artifact as `P0-CERT passed`, the 0039 replacement artifact as `SPINE-CERT passed`, and the 0041 replacement artifact as `EPI-CERT passed`, each only for the exact scoped line stated by its own artifact. It then states that the next known execution move is a separately numbered `ORD-LIFE-CERT` spec and evidence package. See [docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/SPEC_LEDGER.md), [archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md), [archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md), and [archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md).

Accordingly, this spec consumes those predecessor certifications within their stated scopes. It does not re-commission `P0-CERT`, `SPINE-CERT`, or `EPI-CERT`; does not infer latest-main certification; and does not survey feature expansion. Archived Phase-3A specs `0005` through `0025` are historical evidence to re-prove under live doctrine, not a current `ORD-LIFE-CERT` verdict.

The archived staging sequence is contiguous through `0041`, while the live `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` and `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` entries serve separate live-spec and template roles. The next staging number is therefore `0042`.

## 2. Source discipline, freshness, and admissibility

The following source-discipline rules are carried verbatim from the live ledger:

- A commit hash named inside a spec is audit/spec provenance only unless a higher-tier document says otherwise.
- A manifest is path inventory only.
- Branch names, default-branch lookups, connector namespace labels, repository metadata, and code-search snippets are not proof of target-commit content.
- Future exact-commit audits must fetch by exact file URL or by a supplied repository export at the target commit.
- Stale baseline strings in historical specs must not be adopted as current product doctrine.

Additional binding rules for this audit:

- The uploaded manifest established only that named paths were present in the user-supplied inventory. Content evidence comes from the exact-commit URLs in section 2.1.
- Archived specs, tickets, reports, research briefs, and command transcripts are history unless this document explicitly consumes a predecessor acceptance artifact for admissibility. Historical Phase-3A work contributes risk memory and candidate evidence; it does not certify the live ordinary-life boundary.
- `ORD-LIFE-CERT` is a phase-certification label that composes canonical gates and review artifacts defined upstream. This spec may operationalize the label; it may not define, weaken, rename, or replace its semantics.
- `actor-known` is used for the actor-holder case. `holder-known context` remains the system-wide term. Neither means world truth, validation truth, debug omniscience, fixture prose, or a scheduler clock.
- The authoring target is the exact tree at `98dc0421211e6c9881d9c6679b9df74525e392bb`. An implementing session that adds tests or evidence instrumentation must enumerate every delta and record the exact final commit actually tested. It may not imply that unchanged `98dc042` passed unless all certifying commands ran against that unchanged tree.
- Evidence instrumentation is permitted; production remediation is not part of this audit spec. A substantive failed seam or actionable mutation floor routes to a later, separately numbered ORD-LIFE-CERT remediation and replacement-certification spec.
- A pass claim is blocked if it depends on an unfetched file, branch-relative content, repository search, a snippet, mutable metadata, prior-chat memory, or an unreported local modification.
- No compatibility shim, alias path, duplicate constructor, direct-dispatch shortcut, or convenience API may preserve an ordinary-life bypass.

### 2.1 Exact-commit evidence ledger

The manifest was used as path inventory only. The complete ledger below is part of this spec and records every repository URL fetched before the audit design was written.

<details>
<summary>Complete exact-URL fetch ledger (225 files)</summary>

```text
Requested repository: joeloverbeck/tracewake
Target commit: 98dc0421211e6c9881d9c6679b9df74525e392bb
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open with full raw.githubusercontent.com exact-commit URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/AGENTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/CLAUDE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/archival-workflow.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/rust-toolchain.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/clippy.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0041_EPI_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0040_epi_cert_mutation_triage_register.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0040_epi_cert_mutation_final_missed.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0007_PHASE_3A_SECOND_HARDENING_NO_HUMAN_ORDINARY_LIFE_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0008_PHASE_3A_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0015_PHASE_3A_EVENTED_COGNITION_CHANNELS_AUDIT_ENFORCEMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0016_PHASE_3A_NEED_ACCOUNTING_REPLAY_EVIDENCE_AUDIT_COVERAGE_AND_LOCK_DURABILITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0019_PHASE_3A_GENERATIVE_REACHABILITY_HONESTY_MUTATION_PERIMETER_WORKPLACE_FRESHNESS_AND_EVIDENCE_CLOSURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0024_PHASE_3A_CONTENT_SCHEMA_VERSION_GATE_META_WITNESS_RESIDUE_MUTATION_PERIMETER_DERIVATION_AND_TUI_TIME_DEBUG_QUARANTINE_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/need.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/need_accounting.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/need_events.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/generation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/candidate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/methods.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/htn.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/no_human_surface.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/movement.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/checkcontainer.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/openclose.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/takeplace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/defs/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/events/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/time.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/location.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/ids.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/emergence_ledger.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/support/generative.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/support/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/generative_lock.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-core/tests/spine_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/tests/schema_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/validate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/container_item_move_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/door_access_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_exits_require_perceived_or_known_route_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_view_omits_unknown_sleep_affordance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_advance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_current_place_without_sleep_affordance_does_not_sleep_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_metrics_require_typed_responsible_layer_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/ordinary_workday_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/planner_trace_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/replay_item_location_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/severe_safety_with_known_exit_produces_move_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/severe_safety_without_known_exit_waits_with_knowledge_blocker_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/sleep_rejects_current_place_without_sleep_affordance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/work_block_failed_then_sleep_succeeds_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/work_completion_fails_when_actor_displaced_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs

Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

</details>

### 2.2 Reassessment addendum — referenced surfaces confirmed present at the target commit

The section 2.1 ledger records the original 225-URL fetch pass written before the audit design. The twelve repository paths below are named in the command vocabulary (section 4), the seam inventory and audit points (section 5), and the fixture matrices (section 6) but are absent from that fetch pass. During reassessment each was confirmed present at the exact target commit `98dc0421211e6c9881d9c6679b9df74525e392bb` by direct working-tree inspection (working tree == target commit). They are recorded here as reassessment-confirmed presence, not as part of the original fetch pass:

```text
- crates/tracewake-core/src/epistemics/projection.rs
- crates/tracewake-core/src/debug_capability.rs
- crates/tracewake-core/src/debug_reports.rs
- crates/tracewake-core/tests/acceptance_artifact_wording.rs
- crates/tracewake-core/tests/ci_workflow_guards.rs
- crates/tracewake-core/tests/doc_invariant_references.rs
- crates/tracewake-tui/tests/command_loop_session.rs
- crates/tracewake-tui/tests/readme_sample_session.rs
- crates/tracewake-tui/tests/transcript_snapshot.rs
- crates/tracewake-tui/tests/tui_seam_conformance.rs
- crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs
- crates/tracewake-content/src/fixtures/debug_attach_001.rs
```

Reassessment-confirmed presence is provenance honesty, not certifying evidence. The implementing session must still fetch or read each by exact-commit URL or supplied export and record the exact final tested commit before relying on any of these surfaces.



## 3. Authority and dependency declarations

The governing order is `docs/0-foundation/` -> `docs/1-architecture/` -> `docs/2-execution/` -> `docs/3-reference/` -> `docs/4-specs/` -> staged spec -> implementation. If execution conflicts with architecture or foundation, execution is wrong. If implementation convenience conflicts with an accepted gate, implementation is wrong.

All files in every live documentation tier were read at the target commit. The primary dependencies below control the audit. Other files in those tiers were used for boundary awareness so this spec does not absorb already-certified or downstream work.

### 3.1 Primary foundation dependencies

- [docs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/README.md)
- [docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md)
- [docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md)
- [docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md)
- [docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md)
- [docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md)
- [docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md)

The audit is especially constrained by causal event ancestry, deterministic replay, ordinary-life-before-adventure, needs as pressures rather than puppet strings, durable intentions, defeasible routines, bounded planning, actor-known cognition, validation-truth separation, no-direct-dispatch, typed decision/stuck traces, possession neutrality, debug quarantine, observer-only emergence evidence, and the holder-known temporal firewall. This spec only cross-references existing invariant IDs.

Foundation documents `01`, `04`, `07`, `08`, `09`, `10`, `11`, and `13` define scope boundaries. Their epistemic, possession, institutional, LOD, speech, and authoring concerns are consumed where necessary but are not re-certified here.

### 3.2 Primary architecture dependencies

- [docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md)
- [docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md)
- [docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md)
- [docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md)
- [docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md)
- [docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md)
- [docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md)

Architecture requires a sealed actor-known decision transaction from pressures and durable commitments through candidate generation, method selection, bounded local planning, and a sealed proposal; replay-derived projections; causal movement and affordances; source-backed temporal premises; and review evidence sufficient to reconstruct why an ordinary action occurred or failed.

Architecture documents `04`, `06`, `07`, `08`, `10`, `11`, `12`, and `14` constrain adjacent pipeline, epistemic, TUI, institutional, incident, scale, and terminology boundaries. The general pipeline and epistemic/TUI structures remain predecessor-certified; this audit exercises only their ordinary-life-facing seams.

### 3.3 Primary execution dependencies

- [docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md)
- [docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md)
- [docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md)
- [docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md)
- [docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md)
- [docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md)
- [docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md)
- [docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md)
- [docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md)

Execution document `06` owns the ten `ORD-LIFE-CERT` pass conditions, single-charge and single-owner accounting rules, intention lifecycle, routine validity, temporal-premise mechanism, no-human evidence corpus, canonical recovery, behavioral-progress definition, cross-tick stuck categories, and seven mandatory fixture families. Sections 5 and 6 below map every one explicitly.

Execution documents `07`, `08`, `11`, `12`, and `13` are boundary sources. EPI seams, broad data authoring, Phase-4 institutions, second-proof scale, and research notes are not ORD-LIFE-CERT targets.

### 3.4 Primary reference and spec-tier dependencies

- [docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md)
- [docs/3-reference/01_DESIGN_RISK_REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/01_DESIGN_RISK_REGISTER.md)
- [docs/3-reference/02_GLOSSARY.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/3-reference/02_GLOSSARY.md)
- [docs/4-specs/SPEC_LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/SPEC_LEDGER.md)
- [docs/4-specs/README.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/README.md)
- [docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md](https://raw.githubusercontent.com/joeloverbeck/tracewake/98dc0421211e6c9881d9c6679b9df74525e392bb/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md)

Canonical terms include `actor-known`, `holder-known context`, `candidate goal`, `intention`, `routine`, `behavioral progress`, `stuck diagnostic`, `responsible layer`, and `EMERGE-OBS`. Active risks include hidden-truth leakage, starvation/teleport shortcuts, no-progress loops, single-charge drift, stale-premise routine selection, and metric Goodharting.

### 3.5 Structural and historical precedents

The audit anatomy follows the historical 0036, 0038, and 0040 audit specs. Mutation and replacement-certification discipline is informed by 0037, 0039, and 0041, but this document is not a remediation spec. The 0040 survivor register supplies the register shape. All are history-only except the three passed acceptance artifacts consumed in section 1.

### 3.6 Crate and authority direction

The one-way dependency direction is `tracewake-core <- tracewake-content <- tracewake-tui`:

- `tracewake-core` owns authoritative state, events, need accounting, cognition, planning, proposal validation, scheduling, replay, projections, and diagnostic types.
- `tracewake-content` declares and validates fixtures through core-owned types; it cannot create simulation fact from prose or bypass events, actor-known provenance, affordances, or movement ancestry.
- `tracewake-tui` renders embodied and authorized debug outputs and submits semantic actions; it cannot become a truth writer, cognition source, scheduler, or ordinary-action dispatcher.

A reversed dependency or client-owned ordinary-life truth path is an ORD-LIFE-CERT failure even if a final-state snapshot looks plausible.

## 4. Command and evidence vocabulary for the implementing session

The commands below are the minimum execution vocabulary. The final acceptance artifact must record the exact command, tested commit/worktree identity, exit status, start/end timestamps, Rust and tool versions, determinism-relevant environment facts, and complete output or a cryptographically fingerprinted transcript with its actual fingerprint scope.

### 4.1 Clean baseline

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --locked -p tracewake-core --doc
```

A failing, flaky, or selectively retried unmutated baseline blocks interpretation of later evidence. Every failure and retry must remain in the command ledger.

### 4.2 Named test binaries

```bash
cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-core --test doc_invariant_references
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test spine_conformance

cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-content --test schema_conformance

cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test command_loop_session
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test readme_sample_session
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-tui --test tui_seam_conformance
```

Filtered diagnostic runs and `--nocapture` reruns may supplement but never replace the complete named binaries or workspace run. A fixture name in source or a snapshot byte string does not prove that its live event/replay path ran.

### 4.3 Required evidence objects

For every audit point, package the applicable objects below:

1. exact fixture, run seed, content/ruleset identity, start tick, actor set, and starting state/projection fingerprints;
2. sealed actor-known packet identity, event frontier, context hash, admitted facts, source-event IDs, freshness classification, and forbidden-input audit;
3. ordered candidates with sources, priorities, selected/rejected status, and actor-known input references;
4. intention before/after state, typed transition reason, event ancestry, routine/template/method identity, interruptor/failure/fallback path, and local-planner budget use;
5. sealed proposal source context, parameters, ancestry, reservations, validation result, and accepted/rejected event append path;
6. per-actor/per-need/per-tick accounting ledger, duration starts/terminals, cause/ancestry, and proof that every non-owner consumed rather than charged;
7. no-human markers, behavioral-progress ledger, modeled waits, stuck/failure records, metrics derivation, and independent event classification used to check the metric;
8. live and clean-replay fingerprints for authoritative ordinary-life state, needs, intentions, routine execution, decision traces, stuck diagnostics, and no-human metrics;
9. actor-visible output and separately authorized debug output, plus a debug-on/debug-off non-interference comparison where planner traces are involved;
10. behavior witness and live negative, not merely a source assertion, schema shape, snapshot, historical artifact, or mutation score;
11. first divergence and canonical responsible-layer diagnosis for every failed comparison; and
12. sampling/exhaustiveness declaration distinguishing exhaustive registry runs, finite matrices, generated samples, mutation census, historical context, and observer-only evidence.

### 4.4 Cross-cutting proof rules

- **Event before derived state.** Need changes, threshold crossings, intention/routine transitions, decisions, proposals, duration terminals, waits, stuck outcomes, and no-human markers must be event-backed or replay-reconstructable.
- **Single owner, single charge.** The action-pipeline/ordinary-life boundary owns need and duration accounting. Scheduler, planner, projection, replay, and golden normalization may consume the ledger but cannot independently charge, reconcile, synthesize, or normalize a covered tick/window.
- **Actor-known cognition.** Candidate generation, routine applicability, method selection, and local planning consume only the sealed actor-known packet and source-backed routine/temporal premises. Validation truth may reject; it may not propose.
- **Relational hidden-truth proof.** Paired runs differing only in unobserved food, route, affordance, workplace, or schedule truth must yield equal actor-known packets and equal cognition outputs until an admissible observation or record distinguishes them.
- **Routine machinery, not script.** A routine requires applicability, alternatives or method family, interruptors, explicit failure modes, fallback/wait/stuck semantics, and trace expectations. A happy-path sequence alone is not a routine proof.
- **Meaningful progress only.** A no-human metric may count committed ordinary actions, duration starts/completions/interruptions, modeled waits with typed reasons, and typed stuck/failure outcomes. A pure `continue_routine` marker is not progress.
- **No silent liveness assumption.** Every wait identifies its modeled reason and expected review boundary. No-progress-past-expected-window and repeated-idle must become typed stuck outcomes rather than being excused by an implicit fairness assumption.
- **Possession neutrality.** Binding or unbinding a controller may not reset need, routine, intention, memory, actor-known state, or decision semantics.
- **Debug non-interference.** Planner/decision traces may expose selected and rejected alternatives in authorized debug surfaces, but enabling or reading them cannot change cognition, events, state, proposals, metrics, or embodied output.
- **Replay derived, not projection authored.** A matching checksum is insufficient unless a clean rebuild from the accepted log reproduces the ordinary-life projection and metrics without direct writes.
- **No evidence laundering.** Pending, sampled, observer-only, historical, static, snapshot-only, or mutation-score evidence cannot silently become certifying pass evidence.

## 5. The ORD-LIFE-CERT audit contract

The implementing session must execute and report every audit point `ORD-LIFE-01` through `ORD-LIFE-12` as one certification gate. These identifiers are local cross-references inside this spec, not doctrine-level gate codes, invariant IDs, status enums, or new obligation codes.

### 5.1 Verified implementation seam inventory at the target commit

The exact-commit inventory used to define the audit is:

| Area | Verified target files | Load-bearing surfaces to exercise |
|---|---|---|
| Needs | `crates/tracewake-core/src/agent/need.rs` | `NeedKind::{Hunger,Fatigue,Safety}`, `NeedBand`, `NeedState`, `NeedThresholdCrossing`, `NeedChangeCause`, `NeedPressure` |
| Accounting | `crates/tracewake-core/src/need_accounting.rs`; `crates/tracewake-core/src/actions/defs/need_events.rs` | `TickRegime::{Awake,Asleep,Working}`, `classify_actor_tick_regimes_with_start`, `open_body_exclusive_starts`, duration intervals/terminals, duplicate-terminal detection, `build_need_delta_and_threshold_events`, `NeedDeltaEventSpec` |
| Candidate generation | `crates/tracewake-core/src/agent/generation.rs`; `crates/tracewake-core/src/agent/candidate.rs` | deterministic need/routine/intention candidate synthesis, `CandidateGoal`, `CandidateGoalSource`, goal priority and source refs |
| Intentions/routines/methods | `crates/tracewake-core/src/agent/intention.rs`; `crates/tracewake-core/src/agent/routine.rs`; `crates/tracewake-core/src/agent/methods.rs`; `crates/tracewake-core/src/agent/htn.rs` | durable intention state/transitions, `RoutineTemplate`, conditions, steps, execution/status/diagnostics, `phase3a_routine_templates`, `family_for_goal`, `select_phase3a_method`, condition resolution and typed selection failure |
| Planning/decision/trace | `crates/tracewake-core/src/agent/planner.rs`; `crates/tracewake-core/src/agent/decision.rs`; `crates/tracewake-core/src/agent/trace.rs` | bounded local planning, goal-specific planners/fallback, `LocalPlanTrace`/failure, actor-known input refs/source classes, selected/rejected decision trace, blockers, stuck diagnostics, typed fields, responsible layer |
| Actor-known transaction | `crates/tracewake-core/src/agent/transaction.rs`; `crates/tracewake-core/src/agent/actor_known.rs`; `crates/tracewake-core/src/agent/no_human_surface.rs`; `crates/tracewake-core/src/agent/perception.rs` | sealed surface construction, actor-known facts/provenance/source events, `ActorDecisionTransaction::run`, sealed proposal or stuck outcome, dangling-provenance diagnostics, perception-to-observation path |
| Ordinary actions | `crates/tracewake-core/src/actions/defs/eat.rs`; `crates/tracewake-core/src/actions/defs/sleep.rs`; `crates/tracewake-core/src/actions/defs/work.rs`; `crates/tracewake-core/src/actions/defs/wait.rs`; `crates/tracewake-core/src/actions/defs/continue_routine.rs`; `crates/tracewake-core/src/actions/defs/movement.rs` | affordance and target checks, action proposal/event construction, wait reasons, routine continuation, movement ancestry, duration starts and terminals |
| Proposal/pipeline | `crates/tracewake-core/src/actions/proposal.rs`; `crates/tracewake-core/src/actions/pipeline.rs`; `crates/tracewake-core/src/actions/report.rs`; `crates/tracewake-core/src/actions/registry.rs` | proposal source/context parity, validation, reservation, duplicate-duration-terminal rejection, append/apply order, actor/debug report split |
| State/affordances | `crates/tracewake-core/src/state.rs`; `crates/tracewake-core/src/location.rs`; `crates/tracewake-core/src/time.rs` | sleep affordance state, actor/place/item/work state, tick/time values used only at their authorized boundary |
| No-human orchestration | `crates/tracewake-core/src/scheduler.rs`; `crates/tracewake-core/src/projections.rs` | `run_no_human_day`, `advance_no_human`, day windows/config/report, schedule phases, latest action tick evidence, `no_human_day_metrics`, `NoHumanDayMetrics` |
| Events/replay | `crates/tracewake-core/src/events/envelope.rs`; `crates/tracewake-core/src/events/log.rs`; `crates/tracewake-core/src/events/apply.rs`; `crates/tracewake-core/src/events/mutation.rs`; `crates/tracewake-core/src/replay/rebuild.rs`; `crates/tracewake-core/src/replay/report.rs`; `crates/tracewake-core/src/checksum.rs` | ordinary-life event family, schema/replay handling, append/application, projection rebuild, decision-context rebuild, `run_replay`, stable semantic fingerprints |
| Core test harness | `crates/tracewake-core/tests/no_human_capstone.rs`, `crates/tracewake-core/tests/acceptance_gates.rs`, `crates/tracewake-core/tests/anti_regression_guards.rs`, `crates/tracewake-core/tests/emergence_ledger.rs`, `crates/tracewake-core/tests/event_schema_replay_gates.rs`, `crates/tracewake-core/tests/generative_lock.rs`, `crates/tracewake-core/tests/golden_scenarios.rs`, `crates/tracewake-core/tests/hidden_truth_gates.rs`, `crates/tracewake-core/tests/negative_fixture_runner.rs`, `crates/tracewake-core/tests/spine_conformance.rs`, `crates/tracewake-core/tests/support/generative.rs`, `crates/tracewake-core/tests/support/mod.rs` | no-human, event/replay, generative, anti-contamination, structural, mutation-lock, and evidence-honesty witnesses |
| Content/TUI harness | `crates/tracewake-content/tests/fixtures_load.rs`, `crates/tracewake-content/tests/forbidden_content.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, `crates/tracewake-content/tests/schema_conformance.rs`; `crates/tracewake-tui/tests/adversarial_gates.rs`, `crates/tracewake-tui/tests/embodied_flow.rs`, `crates/tracewake-tui/tests/tui_acceptance.rs`, `crates/tracewake-tui/tests/tui_seam_conformance.rs` | fixture registry/schema, positive/adversarial corpus, embodied/debug separation, possession-facing ordinary-life evidence |

#### 5.1.1 Verified symbol census

The following exact identifiers and explicitly named code surfaces were confirmed at the target commit in the modules above. Their presence fixes the audit surface; it is not a behavioral verdict.

- **Needs/accounting:** `NeedKind`, `NeedBand`, `NeedState`, `NeedThresholdCrossing`, `NeedChangeCause`, `NeedPressure`; `TickRegime`, `TickRegimeCounts`, `classify_actor_tick_regimes_with_start`, `actor_tick_regimes_with_start`, `open_body_exclusive_starts`, `is_duration_terminal`, `terminal_ticks_by_start`, `DuplicateDurationTerminal`, and `DurationInterval`; `build_need_delta_and_threshold_events` and `NeedDeltaEventSpec`.
- **Candidates/routines/intentions:** `CandidateGoal`, `CandidateGoalSource`; `RoutineFamily`, `RoutineStep`, `RoutineCondition`, `RoutineTemplate`, `RoutineExecution`, `RoutineStepStatus`, and `RoutineDiagnosticKind`; `phase3a_routine_templates`, `family_for_goal`, `select_phase3a_method`, `resolve_template_conditions`, and `MethodSelectionFailure`; `Intention`, `IntentionStatus`, and `ActorIntentions`.
- **Planning/decision/trace:** `PlannerGoal`, `LocalPlanRequest`, `plan_local_actions`, `PlannedProposal`, `LocalPlan`, `LocalPlanTrace`, `LocalPlanFailure`, and `DEFAULT_PLANNER_BUDGET`; `DecisionInput`, `select_goal_and_trace`, `ActorKnownInputRef`, and `ActorKnownInputSourceClass`; `DecisionTrace`, `DecisionOutcome`, `DecisionTraceRecord`, `BlockerCategory`, `BlockerCode`, `StuckDiagnostic`, `StuckResultingStatus`, `TypedDiagnosticFields`, and `ResponsibleLayer`.
- **Actor-known transaction:** `ActorDecisionTransaction::run`, `ActorDecisionTransactionInput`, `ActorDecisionTransactionOutcome::{Proposed, Stuck}`, `SealedProposal`, and `dangling_provenance_diagnostic`; `ActorKnownFact`, `ActorKnownProvenance`, `SourceEventIds`, and `ActorKnownPlanningContext`; `SealedActorKnownSurface`, `NoHumanActorKnownSurfaceBuilder::from_projection`, and `NoHumanActorKnownSurfaceRequest`; plus the place-perception-to-observation path in `crates/tracewake-core/src/agent/perception.rs`.
- **Actions/proposals/affordances:** `require_sleep_affordance`, `build_sleep_start_event`, the sleep payload, work/eat/wait/movement definitions, pipeline validation/reservation/duplicate-terminal rejection, `Proposal`, `ProposalSource`, `ProposalSourceContext`, and `SleepAffordanceState`.
- **No-human/events/replay:** `run_no_human_day`, `advance_no_human`, `NoHumanDayConfig`, `NoHumanDayReport`, `DayWindow`, `NoHumanAdvanceReport`, `SchedulePhase`, and `latest_action_tick_delta_tick`; `no_human_day_metrics` and `NoHumanDayMetrics`; `EventEnvelope`, `EventKind`, `EventReplayHandling`, and the ordinary-life event family (`NoHumanDayStarted`, `NoHumanDayCompleted`, `NoHumanAdvanceStarted`, `NoHumanAdvanceCompleted`, `SleepStarted`, `SleepCompleted`, `SleepInterrupted`, `WorkBlockStarted`, `WorkBlockCompleted`, `WorkBlockFailed`, `NeedDeltaApplied`, `NeedThresholdCrossed`, intention and routine-step transitions, `ContinueRoutineProposed`, `ContinueRoutineAccepted`, `ContinueRoutineRejected`, `StuckDiagnosticRecorded`, `DecisionTraceRecorded`, and `ReplayProjectionRebuilt`); `rebuild_projection`, `rebuild_decision_context_hashes`, `run_replay`, and the decision-context/holder-known hashing surfaces.

The inventory identifies where to audit. It is not evidence that any seam passes.

### ORD-LIFE-01 — Bounded event-sourced needs, single-owner accounting, and single-charge ledgers

**Seam definition.** Needs are bounded pressures visible through actor-known state; they influence candidate families but never identify a true target. Every passive or action-emitted need delta, threshold crossing, duration tick, work completion/failure charge, wait charge, and duration terminal is emitted by the single action-pipeline/ordinary-life owning seam and covers each actor/need/tick exactly once. All downstream consumers read the event-backed ledger without independently charging or reconciling it.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/need.rs`
- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/actions/defs/need_events.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-001`, `INV-009`, `INV-015`, `INV-018`, `INV-039`, `INV-045`, `INV-091`, and `INV-092`; foundation `03`, `05`, and `06`; architecture `02`, `05`, and `13`; execution `06`, `09`, and `10`.

**Required positive fixtures/properties.**

- Run `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001`, and the integrated no-human corpus. For every actor/need/tick, independently expand event payloads into a canonical ledger and prove one owner and one charge.
- Exercise awake, asleep, working, modeled-wait, interruption, completion, and failure regimes, including windows beginning with an already-open body-exclusive duration.
- Prove bounded pressure/threshold behavior and deterministic threshold-crossing event order from identical input events.
- Clean replay must reconstruct identical need state, threshold crossings, duration intervals, duplicate-terminal findings, and no-human accounting totals.

**Required adversarial fixtures/properties.**

- Create overlapping passive-window and action-emitted coverage for the same actor/need/tick; the owning seam must reject or deduplicate before accepted state changes rather than double-charge.
- Inject a duplicate or conflicting duration terminal, including a terminal whose proposal ID differs but whose event cause closes the same start; the typed duplicate-terminal path must identify the original start and conflicting terminals.
- Mutate interval boundaries, elapsed-tick expansion, regime classification, terminal matching, or threshold direction; the committed properties must catch off-by-one, omission, double-count, and wrong-cause behavior.
- Prove scheduler, planner, projection, replay, and golden normalization cannot synthesize compensating deltas that hide an owning-seam defect.

**Required evidence mechanics.** Record the canonical actor/need/tick ledger, emitter and event ID for each row, duration start/terminal ancestry, tick-regime classification, pre/post need values, threshold events, live/replay equality, and a negative control showing the duplicate or uncovered tick fails at its responsible layer. Stable golden bytes without semantic ledger equality do not satisfy this point. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `doctrine`; `scheduler`; `action_validation`; `event_append`; `event_application`; `projection`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1, 8, and 9. Mandatory fixture families: Integrated no-human day; Routine blocker (accounting-side failure attribution).

### ORD-LIFE-02 — Actor-known candidate generation, deterministic priority, and hidden-target exclusion

**Seam definition.** Need, routine, safety, and current-intention pressures may synthesize candidate goals only from the sealed actor-known surface and source-backed routine inputs. Candidate generation must be deterministic for the same packet, retain source/priority evidence for selected and rejected candidates, and never convert hunger, fatigue, safety, fixture possibility, validation truth, or unobserved physical truth into a true target.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/generation.rs`
- `crates/tracewake-core/src/agent/candidate.rs`
- `crates/tracewake-core/src/agent/need.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-002`, `INV-039`, `INV-040`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-103`, and `INV-104`; foundation `05`, `06`, and `14`; architecture `03` and `05`; execution `04` and `06`.

**Required positive fixtures/properties.**

- With event-backed knowledge of reachable food, sleep place, workplace assignment, or safe exit, prove the corresponding candidate family is generated with actor-known source references and deterministic total ordering.
- With hunger but no known food target, prove the actor receives search, fallback, wait, or stuck-capable candidates rather than a target synthesized from world state.
- Prove a current durable intention remains a candidate and is continued under mild competing pressure, while severe/urgent pressure may interrupt through a typed lifecycle path.
- Repeat identical packet generation and canonicalize insertion order; candidate set, priorities, tie-break order, and source references must be identical.

**Required adversarial fixtures/properties.**

- Use paired fixtures that differ only in hidden food, route, sleep affordance, workplace truth, or another actor’s private knowledge; candidate output must remain equal until a legal observation/record changes the actor-known packet.
- Exercise `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `seeded_food_source_unknown_to_all_actors_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `no_human_known_workplace_requires_provenance_001`, and `forbidden_provenance_input_fails_closed_001`.
- Present empty, dangling, wrong-holder, wrong-kind, debug-only, validation-truth, unproven-physical-truth, and future-frontier source references; generation must fail closed or exclude the candidate with typed evidence.
- Change only a rejected candidate’s hidden target or fixture prose; no selected goal, proposal, event, or metric may change.

**Required evidence mechanics.** Package the complete sealed input packet, context ID/hash/frontier, candidate census in deterministic order, source class and source-event IDs for each candidate, selected/rejected reason, hidden-truth comparison, and live negative. The proof is relational over paired runs, not a banned-word scan. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `holder_known_context`; `candidate_generation`; `intention_lifecycle`; `test_oracle`.

**Coverage.** Numbered pass conditions: 2, 7, and 9. Mandatory fixture families: Food unavailable; Hidden-truth planning; Integrated no-human day.

### ORD-LIFE-03 — Durable intention lifecycle, typed ancestry, replacement semantics, and possession neutrality

**Seam definition.** Intentions are durable commitments whose live semantics cover active, continued, suspended, interrupted, completed, failed, abandoned, and replaced states. Every transition requires a typed reason and causal ancestry. Urgent needs may interrupt but not silently erase an intention. Controller possession changes command authority only; it may not reset or rewrite need, routine, intention, memory, or actor-known state.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-tui/tests/embodied_flow.rs`

**Doctrine to satisfy.** `INV-034`, `INV-035`, `INV-041`, `INV-094`, `INV-105`, and `INV-108`; foundation `05`, `08`, and `14`; architecture `05` and `10`; execution `06`, consuming but not re-auditing the EPI possession guarantee in `07`.

**Required positive fixtures/properties.**

- Produce event-backed traces for adoption/activation, continuation, suspension, interruption, resumption or replacement, completion, failure, and abandonment, including typed reason, prior intention, new intention, and triggering need/routine/action event.
- Prove mild pressure preserves an active intention while severe hunger/fatigue/safety may interrupt through an explicit event and later resume, replace, fail, or abandon through another explicit transition.
- Run `possession_does_not_reset_intention_001` and `possession_parity_001`; compare before/bind/possessed-action/unbind state and replay to show ordinary cognition and commitment continuity.
- Clean replay must reproduce the same current intention, terminal history, routine association, transition order, and decision-trace lifecycle effect.

**Required adversarial fixtures/properties.**

- Bind and unbind possession at each lifecycle stage; any reset, source substitution, priority change, memory loss, routine restart, or replacement without ancestry is a failure.
- Attempt terminal-state reactivation, duplicate completion, replacement without a predecessor, continuation after abandonment, and interruption without a triggering typed cause; each must reject or produce the canonical failure path.
- Perturb event order and replay frontier so that a lifecycle transition references a future, missing, or wrong actor/intention event; reconstruction must fail loudly rather than infer a plausible state.
- Prove final-state equality alone cannot hide different transition ancestry: two runs with the same current intention but different illegal histories must not share certifying evidence.

**Required evidence mechanics.** Record an intention transition ledger with semantic state, implementation representation, event kind/ID, typed reason, predecessor/successor ancestry, triggering pressure or routine, actor-known context, possession binding state, and live/replay result. Where implementation variants do not map one-to-one to doctrine words, the artifact must explain the exact event/state encoding rather than silently omit a required lifecycle state. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test embodied_flow
```

**Primary failure-diagnostic layers.** `intention_lifecycle`; `event_append`; `event_application`; `replay`; `view_model`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1 and 9. Mandatory fixture families: Possession does not reset intention; Integrated no-human day.

### ORD-LIFE-04 — Defeasible routine templates, HTN method families, interruptors, failure modes, and fallback

**Seam definition.** Routines are defeasible HTN-like causal machinery rather than scripts or guaranteed arcs. Every live routine template must state holder-known applicability, steps or method families, required known affordances/records, interruptors, explicit failure modes, fallback/wait/stuck semantics, and trace/diagnostic expectations. No routine may dispatch a primitive action merely because its label or schedule says so.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-035`, `INV-036`, `INV-037`, `INV-040`, `INV-041`, `INV-104`, `INV-105`, and `INV-106`; foundation `05` and `06`; architecture `05`; execution `06`, `09`, and `10`.

**Required positive fixtures/properties.**

- Enumerate every `phase3a_routine_templates` family and prove its applicability conditions, steps/methods, affordance/record requirements, interruptors, non-empty failure modes, fallback rules, and diagnostic expectations are reachable through committed tests.
- Exercise morning wake, eat meal, find food, go to work, work block, return home, sleep night, leave unsafe place, continue intention, and wait families through at least one accepted method and one rejected alternative where applicable.
- Prove routine progress survives across ticks and proposals, while method failure yields coherent fallback, modeled wait, stuck, failure, or a newly traced method rather than silent restart.
- Run `routine_blocked_diagnostic_001`, `method_fallback_requires_new_trace_or_stuck_001`, `work_block_failed_then_sleep_succeeds_001`, and `severe_safety_with_known_exit_produces_move_001`.

**Required adversarial fixtures/properties.**

- Construct or mutate a routine with no failure modes, empty non-diagnostic steps, a hidden-truth precondition, no interruptor for severe safety, or a fallback that repeats the failed method without new evidence; validation/test oracles must fail it.
- Run `routine_no_teleport_001` and blocked-affordance fixtures; a remote work/sleep step must not skip movement ancestry or affordance validation.
- Force every method precondition to fail, planner budget exhaustion, and validation rejection; each outcome must identify method/routine identity, blocker, responsible layer, and resulting intention/routine status.
- Prove template order or hash-map insertion changes do not alter deterministic selected method when semantic inputs are equal.

**Required evidence mechanics.** Package a routine-template census, per-template positive and negative reachability, method applicability evaluation, selected and rejected method traces, interruptor trigger, failure-mode and fallback edge, resulting routine/intention status, proposal ancestry, event log, and replay equality. Source shape without live failure reachability is non-certifying. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Primary failure-diagnostic layers.** `content_schema`; `content_validation`; `fixture_contract`; `method_selection`; `local_planning`; `intention_lifecycle`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1, 3, and 9. Mandatory fixture families: Routine no-teleport; Routine blocker; Planner trace.

### ORD-LIFE-05 — Routine temporal premises and modeled adaptation without ground-truth schedule cognition

**Seam definition.** A routine may use time only through a modeled, source-backed temporal premise available in the holder-known context: assignment, memory, observation, public cue, record, testimony, institutional context, or source-backed inference. Ground-truth schedule time, scheduler awakening, day-window selection, or elapsed-time accounting may validate/order execution but cannot by itself justify routine selection. The mechanism must also support a modeled experience or contradiction changing later selection through existing memory/expectation channels.

**Audited files/modules.**

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-002`, `INV-035`, `INV-036`, `INV-100`, `INV-102`, `INV-103`, and `INV-112`; foundation `05` and `14`; architecture `03` and `05`; execution `04`, `05`, `06`, `09`, and `10`.

**Required positive fixtures/properties.**

- Run a work or sleep routine that succeeds because an event-backed assignment, remembered observation, notice/public cue, record, testimony, institutional context, or source-backed inference supplies the premise; record source category and event ancestry.
- Use `ordinary_workday_001`, `workplace_assignment_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, and `stale_workplace_notice_superseded_by_newer_001` to prove premise presence, freshness/supersession, and deterministic selection.
- Prove a repeated modeled experience, contradiction, interruption, notice, or changed outcome alters a later routine/method/trust choice through holder-known memory or expectation evidence rather than an unrecorded learning shortcut.
- Clean replay must rebuild the same premise classification, source selection, later adaptation, method choice, and diagnostic.

**Required adversarial fixtures/properties.**

- Run the same world at a schedule tick where work would be objectively appropriate but remove all modeled premise sources; `no_human_unseen_workplace_assignment_does_not_plan_work_001` must wait or fail with missing-knowledge evidence rather than plan work.
- Hold actor-known history fixed while changing true schedule time/day-window boundaries; cognition output must stay equal unless a modeled cue or record changes.
- Prove `routine_window_family` or equivalent scheduler input cannot become an unproven cognition fact. A scheduler wake or elapsed tick must not appear as temporal-premise provenance.
- Use stale or contradicted workplace information and `embodied_workplace_believed_open_truth_closed_commit_fails_001`; stale premise classification and validation failure must remain distinct and must not be repaired from truth.

**Required evidence mechanics.** Package a temporal-premise ledger for each routine decision: source category, holder, proposition/fact, source event IDs, acquisition/current tick, freshness, context frontier, selected/rejected routine and method, scheduler inputs excluded from cognition, validation outcome, adaptation ancestry, and paired-run result. This point certifies the routine-premise mechanism only; the consolidated cross-seam temporal evidence bundle is explicitly routed to FIRST-PROOF-CERT in section 11. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Primary failure-diagnostic layers.** `holder_known_context`; `candidate_generation`; `method_selection`; `scheduler`; `action_validation`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 2, 3, and 9. Mandatory fixture families: Hidden-truth planning; Routine blocker; Integrated no-human day.

### ORD-LIFE-06 — Actor-known method selection, bounded local planning, planner-budget discipline, and coherent fallback

**Seam definition.** Method selection and local planning must cite actor-known provenance for every target, route, affordance, assignment, and temporal premise they use. Planning is bounded and local: it may select among known methods and produce a short proposal sequence, but it cannot search omniscient world state, fabricate provenance, or silently exceed its declared budget. Failure must yield a typed blocker and coherent newly traced fallback, wait, stuck, or intention outcome.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-036`, `INV-037`, `INV-040`, `INV-041`, `INV-100`, `INV-102`, `INV-105`, and `INV-106`; foundation `05`, `06`, and `14`; architecture `03` and `05`; execution `04`, `06`, and `10`.

**Required positive fixtures/properties.**

- For every supported `PlannerGoal`, run the applicable `plan_*` path with a legal actor-known context and prove the `LocalPlan`, `PlannedProposal`, budget use, target, route, affordance, and source references match the selected method.
- Prove `DEFAULT_PLANNER_BUDGET` and any explicit test budgets are deterministic, enforce a hard bound, and appear in trace/diagnostic evidence when exhausted.
- Exercise a primary method failure followed by a different traced fallback or typed stuck result through `method_fallback_requires_new_trace_or_stuck_001` and food-unavailable/routine-blocker fixtures.
- Prove a planned move/eat/sleep/work sequence remains local and proposal-based; each step reaches ordinary validation rather than mutating state directly.

**Required adversarial fixtures/properties.**

- Remove, stale, reorder, or wrong-kind one source witness at a time; `dangling_provenance_diagnostic` or the corresponding typed failure must identify the exact missing/invalid reference before proposal acceptance.
- Set budget to the smallest boundary values and mutate decrement/termination comparisons; zero/one/off-by-one cases must produce deterministic success or `PlannerBudgetExhausted`, never an unbounded loop.
- Provide hidden true food, a hidden route edge, an unobserved sleep affordance, or a true workplace target; planning must not change until actor-known evidence changes.
- Force no applicable method, empty local plan, unsupported action, failed movement ancestry, and validation rejection; every branch must emit a trace and resulting status rather than disappear.

**Required evidence mechanics.** Record the method-candidate census, precondition evaluations, source-event citations, selected/rejected methods, planner request, initial and consumed budget, local-plan steps, proposal context/ancestry, fallback edge, blocker code, responsible layer, and live/replay trace equality. A final successful action without rejected-method and budget evidence is incomplete. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `method_selection`; `local_planning`; `proposal_construction`; `action_validation`; `test_oracle`.

**Coverage.** Numbered pass conditions: 3, 5, and 9. Mandatory fixture families: Food unavailable; Hidden-truth planning; Planner trace; Routine blocker.

### ORD-LIFE-07 — Planner and decision trace honesty, rejected alternatives, and debug quarantine

**Seam definition.** Decision and local-plan traces must explain selected and rejected candidates/methods, actor-known inputs, lifecycle effects, proposals, failures, blockers, and responsible layers. They are non-diegetic evidence: authorized debug output may compare actor-known input with hidden truth, but trace/debug data may not enter the actor-known packet, candidate generation, planning, proposal, metrics, embodied view, or future memory.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/transcript.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`

**Doctrine to satisfy.** `INV-041`, `INV-068`, `INV-070`, `INV-105`, `INV-107`, and `INV-111`; foundation `05`, `08`, and `14`; architecture `05`, `10`, and `13`; execution `06`, `07`, and `10`, while consuming rather than re-auditing EPI debug authority.

**Required positive fixtures/properties.**

- Run `planner_trace_001` and `method_fallback_requires_new_trace_or_stuck_001`; prove traces include the complete ordered candidate and method census, rejection reasons, selected goal/method/plan, budget, proposal or stuck result, and actor-known source refs.
- Enable authorized debug output and show hidden truth and actor-known input side by side with explicit non-diegetic labeling, while embodied output retains only actor-known information.
- Repeat the same fixture with debug disabled/enabled/read/not-read; event log, authoritative state, epistemic projection, actor-known packet, decision, proposal, metrics, and embodied output must be semantically identical.
- Clean replay must rebuild the same decision trace records or the exact replay-derived trace projection promised by the schema.

**Required adversarial fixtures/properties.**

- Run `debug_omniscience_excluded_001`, `debug_attach_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`; debug truth must never become an admitted source class.
- Attempt to feed a prior trace, debug report, rendered string, hidden-truth diff, or operator annotation into candidate generation or planning; type/API boundaries and live negative tests must fail closed.
- Omit rejected candidates/methods, blocker, budget, or responsible layer while leaving a plausible final action; evidence-honesty tests must reject the incomplete trace.
- Alter only debug rendering order or attachment; no actor-visible behavior may change, and a changed simulation fingerprint is a debug-quarantine failure.

**Required evidence mechanics.** Package paired debug-off/on execution fingerprints, authorized capability path, trace serialization, actor-known packet, hidden-truth comparison, embodied and debug renderings, event/projection checksums, source-class audit, and a negative feedback attempt. `EMERGE-OBS` may cite these traces as observer-only evidence but cannot satisfy this point by itself. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test tui_seam_conformance
```

**Primary failure-diagnostic layers.** `debug_quarantine`; `view_model`; `candidate_generation`; `local_planning`; `test_oracle`.

**Coverage.** Numbered pass conditions: 7 and 9. Mandatory fixture families: Planner trace; Hidden-truth planning.

### ORD-LIFE-08 — Ordinary action affordances, causal movement, durations, terminals, and no-teleport behavior

**Seam definition.** Eat, sleep, work, wait, movement, and routine-continuation actions must be proposed through the ordinary transaction and accepted only after current validation of target, location, affordance, access, reservation, and source-context parameters. Remote work/sleep/eat cannot begin without causal movement ancestry or explicitly valid duration semantics. Every open duration has one authoritative start and exactly one legal terminal.

**Audited files/modules.**

- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/actions/defs/movement.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/need_accounting.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/location.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-001`, `INV-009`, `INV-015`, `INV-043`, `INV-044`, `INV-045`, `INV-048`, `INV-104`, and `INV-106`; foundation `03` and `06`; architecture `04`, `05`, and `09`; execution `05`, `06`, and `09`.

**Required positive fixtures/properties.**

- Prove legal sleep at an actor-known sleep affordance, legal eating of an actor-known accessible item, movement to a known workplace followed by work, and modeled wait with typed reason all pass through proposal, validation, event append, application, and replay.
- For each duration action, record start, covered ticks, interruption/completion/failure terminal, need accounting, reservation release, and final actor/place state.
- Run `severe_safety_with_known_exit_produces_move_001`, `sleep_eat_work_001`, `ordinary_workday_001`, and causal container/item movement fixtures as positive ancestry witnesses.
- Prove proposal source context ID/hash/frontier and actor/action/target parameters remain unchanged from sealed transaction to validator.

**Required adversarial fixtures/properties.**

- Run `routine_no_teleport_001`, `sleep_rejects_current_place_without_sleep_affordance_001`, `no_human_current_place_without_sleep_affordance_does_not_sleep_001`, `work_completion_fails_when_actor_displaced_001`, and hidden-route fixtures.
- Forge or stale target, actor place, item location, sleep affordance, workplace assignment, context hash/frontier, reservation, duration start, or terminal cause; validation must reject without an accepted ordinary-action event.
- Attempt duplicate completion/interruption/failure, terminal without start, start while an exclusive duration is open, and completion after displacement; each must identify the exact action-validation/accounting failure.
- Prove a routine label, need band, scheduler window, or validator-discovered true target cannot directly create an action or movement event.

**Required evidence mechanics.** Package the full proposal/validation report, actor-visible and debug-only reasons, context parity tuple, target/route/affordance facts with provenance, movement and duration ancestry, reservations, accepted/rejected event sequence, per-tick ledger, final state, and clean replay. A correct final location without movement ancestry is a failure. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `proposal_construction`; `scheduler`; `action_validation`; `event_append`; `event_application`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1, 4, 5, and 9. Mandatory fixture families: Routine no-teleport; Food unavailable; Routine blocker.

### ORD-LIFE-09 — No-human orchestration, canonical recovery, meaningful progress, and metric honesty

**Seam definition.** The no-human runner must advance multiple ordinary actors through the same sealed actor-known transaction, proposal, validation, event, and replay paths available to a possessed actor. Start/end markers and metrics must derive from accepted events. Behavioral progress is limited to committed ordinary actions; duration starts/completions/interruptions; modeled waits with typed reasons; and typed stuck/failure outcomes. `continue_routine` alone is never progress.

**Audited files/modules.**

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`
- `crates/tracewake-core/tests/emergence_ledger.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-003`, `INV-039` through `INV-045`, `INV-091`, `INV-092`, `INV-103`, `INV-104`, and `INV-111`; foundation `05`, `06`, and `12`; architecture `02`, `05`, and `13`; execution `02`, `03`, `06`, `09`, and `10`.

**Required positive fixtures/properties.**

- Run `no_human_day_001`, `no_human_advance_001`, `sleep_eat_work_001`, and `ordinary_workday_001`; prove multiple actors progress without human commands through ordered day windows and ordinary transactions.
- For every actor/window, show start/end markers, actor-known packet, candidate/rejection list, selected/rejected method, proposal ancestry, validation, ordinary action/wait/failure, need/intention changes, and replay report.
- Independently classify the accepted event stream into the four permitted progress categories and prove `NoHumanDayMetrics` exactly matches that classification.
- Preserve the canonical `no_human_day_001` outcome `fail_only_empty_food_source`: Mara’s empty home source yields a typed ordinary-life failure without consuming or targeting Tomas’s food or hidden fallback food.

**Required adversarial fixtures/properties.**

- Construct windows containing only `continue_routine`, scheduler awakenings, trace records, debug records, projection rebuild markers, or repeated unreasoned idle; metrics must not count them as behavioral progress.
- Change hidden fallback food or another actor’s resources without changing the focal actor’s knowledge; no action/metric may imply hidden recovery.
- Remove start/end markers, duplicate a completed marker, mismatch actor/window IDs, or compute metrics before accepted append/replay; the report must fail loudly.
- Goodharting controls: a fixture that emits many waits, failures, or traces without legitimate typed reason cannot pass merely by increasing a progress count.

**Required evidence mechanics.** Package the authored fixture identity, no-human config and windows, complete event log, per-actor/window transaction bundle, independent progress classification, metric projection, canonical recovery witness, no-human end report, replay report, and observer-only emergence summary. Metrics must cite event IDs and responsible layers for stuck/failure counts. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test emergence_ledger
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `fixture_contract`; `holder_known_context`; `candidate_generation`; `scheduler`; `action_validation`; `event_append`; `projection`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 6, 8, 9, and 10. Mandatory fixture families: Integrated no-human day; Food unavailable; Routine blocker; Planner trace.

### ORD-LIFE-10 — Typed stuck diagnostics, blocker taxonomy, and cross-tick no-progress detection

**Seam definition.** An actor that cannot progress must not starve silently, loop on a routine marker, or wait forever without cause. The system must distinguish a legitimate modeled wait from a stuck loop and emit typed diagnostics for at least `no-progress-past-expected-window` and `repeated-idle`, including actor, routine/window, blocker code, responsible layer, resulting intention/routine status, and supporting event ancestry.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** `INV-015`, `INV-041`, `INV-070`, `INV-091`, `INV-105`, and `INV-106`; foundation `05`, `06`, and `12`; architecture `05` and `13`; execution `03`, `06`, and `10`.

**Required positive fixtures/properties.**

- Run `routine_blocked_diagnostic_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `food_unavailable_replan_001`, and planner-budget/fallback cases to cover missing knowledge, stale knowledge, budget exhaustion, blocked affordance, and validation failure.
- Produce a modeled wait with a source-backed reason and expected review boundary; prove it is not prematurely classified as stuck.
- Produce no-progress-past-expected-window and repeated-idle sequences; prove each crosses its declared window boundary and emits exactly one typed diagnostic per canonical episode rather than silently looping or flooding duplicates.
- Clean replay must reconstruct diagnostics, blocker code, responsible layer, resulting status, source events, and no-human metric classification.

**Required adversarial fixtures/properties.**

- Run `scheduler_cannot_rewrite_wait_reason_after_transaction_001`; scheduler post-processing may not replace the actor transaction’s reason or layer.
- Omit blocker, responsible layer, actor/window identity, expected boundary, or source events; evidence-oracle and schema tests must reject the incomplete diagnostic.
- Change only debug text or display prose; typed diagnostic semantics and replay must remain unchanged. Conversely, changing blocker/layer/status must change semantic evidence even if displayed text remains similar.
- Force repeated idle across adjacent windows, a single legitimate wait, and intermittent genuine progress; the detector must not collapse all three into one outcome.

**Required evidence mechanics.** Package a tick-by-tick progress timeline, routine window and expected boundary, wait reasons, candidate/method/proposal outcomes, detector state, typed diagnostic fields, source events, resulting status, metric row, and live/replay equality. The artifact must explain why each wait is modeled or why the sequence is stuck; “actor waited” is insufficient. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `candidate_generation`; `intention_lifecycle`; `method_selection`; `local_planning`; `scheduler`; `action_validation`; `projection`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1, 6, 8, and 9. Mandatory fixture families: Food unavailable; Routine blocker; Integrated no-human day.

### ORD-LIFE-11 — Scheduler no-direct-dispatch, sealed proposal ancestry, and forged/stale validation rejection

**Seam definition.** The scheduler may order actors, advance time, detect open durations, and invoke the actor decision transaction. It may not choose an ordinary target, convert a need/routine/window directly into a primitive action, call an action definition as cognition, rewrite a sealed wait/stuck reason, or apply state. Every ordinary action must originate in an actor-known transaction, produce a sealed proposal with context parity and ancestry, pass validation, append an event, and only then apply.

**Audited files/modules.**

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`

**Doctrine to satisfy.** `INV-001`, `INV-009`, `INV-043`, `INV-099`, `INV-103`, `INV-104`, and `INV-106`; foundation `03`, `06`, and `14`; architecture `04` and `05`; execution `04`, `05`, and `06`. General SPINE-CERT pipeline correctness is consumed, not re-audited; this point certifies the ordinary-life seam’s use of it.

**Required positive fixtures/properties.**

- Trace at least eat, sleep, work, move, wait, and continue-routine outcomes from scheduler actor selection through `ActorDecisionTransaction::run`, sealed proposal/stuck outcome, pipeline validation, append, application, report, and replay.
- Prove proposal source context holder/actor, context ID/hash/frontier, action kind, target, and actor-known input refs are identical across transaction output and validator input.
- Prove scheduler ordering and skipped-open-duration behavior are deterministic and do not manufacture progress or need charges.
- Use the possessed and no-human paths to show both reach the same ordinary proposal/validation seam.

**Required adversarial fixtures/properties.**

- Attempt scheduler-side direct action construction/dispatch from hunger, fatigue, safety, routine family, true schedule time, or a day window; structural and runtime guards must catch the bypass.
- Forge/stale context hash/frontier, actor, target, route, affordance, reservation, duration start, or proposal ancestry; validation must reject and append no accepted ordinary-action event.
- Present validation truth that discovers a true target; it may reject the proposed action but must not return or schedule a replacement action.
- Exercise `scheduler_cannot_rewrite_wait_reason_after_transaction_001`; any scheduler rewrite of reason, blocker, or responsible layer is a failure.

**Required evidence mechanics.** Package a call/authority diagram tied to concrete event IDs, scheduler input/output, sealed actor-known packet, transaction outcome, proposal parity tuple, validation report, append-before-apply witness, actor/debug feedback split, and negative direct-dispatch attempt. A source scan alone is insufficient; at least one live ordinary-action path and one live forged/stale rejection are required. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `doctrine`; `holder_known_context`; `proposal_construction`; `scheduler`; `action_validation`; `event_append`; `event_application`; `test_oracle`.

**Coverage.** Numbered pass conditions: 4, 5, and 9. Mandatory fixture families: Integrated no-human day; Hidden-truth planning; Routine no-teleport; Routine blocker.

### ORD-LIFE-12 — Deterministic replay-derived ordinary-life projections, metrics, diagnostics, and phase-entry lock

**Seam definition.** Accepted ordinary-life events are the durable authority. A clean rebuild must deterministically reproduce need state and thresholds, intentions and lifecycle ancestry, routine execution, decision/planner traces, stuck diagnostics, duration/open-terminal state, no-human metrics, and decision-context hashes. Projections and reports are consumers, never truth writers. The acceptance package must also keep Phase 4 blocked unless all twelve audit points, the mutation floor, and predecessor-gate evidence satisfy the aggregate rule.

**Audited files/modules.**

- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-core/tests/ci_workflow_guards.rs`
- `crates/tracewake-core/tests/doc_invariant_references.rs`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/4-specs/SPEC_LEDGER.md`

**Doctrine to satisfy.** `INV-009`, `INV-018`, `INV-041`, `INV-091`, `INV-092`, and `INV-105`; foundation `03` and `12`; architecture `00`, `02`, and `13`; execution `00`, `01`, `03`, `06`, `09`, and `10`; spec ledger and acceptance template.

**Required positive fixtures/properties.**

- For the integrated no-human, food-unavailable, no-teleport, possession, hidden-truth, planner-trace, and routine-blocker families, rebuild from an empty projection and compare live/replay semantic fingerprints for every ordinary-life surface named in the seam definition.
- Run replay twice from the same log and environment; prove deterministic event handling, context-hash rebuild, diagnostic ordering, metric totals, and serialized semantic output.
- Produce a per-event replay-handling census covering the ordinary-life event family, including no-human markers, need/threshold, intention/routine, continuation, sleep/work duration, decision trace, stuck diagnostic, and replay marker behavior.
- Prove the ledger/acceptance wording and CI/doc guards do not declare or permit `PHASE-4-ENTRY` from historical Phase-3A evidence, pending evidence, sampled evidence, observer-only `EMERGE-OBS`, or a partial ORD-LIFE result.

**Required adversarial fixtures/properties.**

- Delete, duplicate, reorder, alter schema handling, or mutate one ordinary-life event payload at a time; rebuild must diverge at the first affected event or reject loudly, not converge by projection normalization.
- Attempt direct projection/metric/diagnostic insertion or post-replay reconciliation; compile/runtime/behavior guards must expose the truth-writer path.
- Change only map/set insertion order and other semantically irrelevant order; canonical semantic fingerprints must remain equal. Change any semantic event field; the applicable fingerprint or failure must change.
- Attempt acceptance wording that promotes historical, pending, sampled, or observer-only evidence, skips the mutation floor, or claims latest main; artifact-wording and review must reject it.

**Required evidence mechanics.** Package exact log bytes and parsed semantic scope, replay command/transcript, applied event range, per-event handling census, live/replay projection and metric fingerprints, context-hash comparison, first-divergence report, phase-entry/admissibility evidence, and the aggregate acceptance table. Hashes must name their actual scope; raw-byte equality cannot substitute for semantic behavior, and semantic equality cannot be overclaimed as raw-byte identity. Independently of the point-specific witness, record the originating accepted event or explicit non-event input boundary, the live typed output, every projection/metric/diagnostic field this seam feeds, and a clean-replay plus same-input determinism comparison. If the seam has no dedicated projection field, state that non-applicability and compare its typed output from identical actor-known/event inputs rather than omitting replay/determinism evidence.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-core --test doc_invariant_references
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Primary failure-diagnostic layers.** `doctrine`; `event_append`; `event_application`; `projection`; `replay`; `test_oracle`.

**Coverage.** Numbered pass conditions: 1, 8, 9, and 10. Mandatory fixture families: All seven mandatory fixture families.

## 6. Completeness and fixture coverage matrices

### 6.1 Mapping to all ten numbered ORD-LIFE-CERT pass conditions

The condition text below is the live condition set from execution document `06`. A condition passes only when every mapped audit point has certifying evidence; the mapping does not permit partial substitution.

| Condition | Live requirement | Primary audit points | Required aggregate witness |
|---:|---|---|---|
| 1 | Needs, intentions, routines, and stuck state are event-sourced or replay-reconstructable. | `ORD-LIFE-01`, `03`, `04`, `08`, `10`, `12` | Ordered event ancestry plus clean replay equality for all four state families and duration/accounting effects. |
| 2 | Candidate generation uses actor-known inputs only. | `ORD-LIFE-02`, `05` | Sealed actor-known packet, source refs, paired hidden-truth equality, and true-time-without-premise negative. |
| 3 | Method selection and local planning cite actor-known provenance. | `ORD-LIFE-04`, `05`, `06` | Selected/rejected methods, local plan, budget, target/route/affordance/temporal source-event citations, and dangling-source negatives. |
| 4 | Scheduler cannot dispatch ordinary actions directly from needs or routines. | `ORD-LIFE-08`, `11` | Scheduler -> actor transaction -> sealed proposal/stuck -> pipeline witness plus direct-dispatch negative. |
| 5 | Action validation rejects forged/stale proposal parameters. | `ORD-LIFE-06`, `08`, `11` | Context/actor/target/route/affordance/reservation/duration perturbation matrix with no accepted event on rejection. |
| 6 | No-human metrics count only real progress, modeled waits, or typed stuck/failure outcomes. | `ORD-LIFE-09`, `10` | Independent event classification exactly matching metrics; `continue_routine`/debug/scheduler markers excluded. |
| 7 | Debug output can compare actor-known input against hidden truth without contaminating decisions. | `ORD-LIFE-02`, `07` | Debug-on/off relational equality and explicit actor-known/hidden comparison under non-diegetic authority. |
| 8 | Replay rebuilds ordinary-life metrics and diagnostics. | `ORD-LIFE-01`, `09`, `10`, `12` | Empty-projection rebuild of accounting, progress metrics, blockers/layers, and decision context fingerprints. |
| 9 | Fixture failures identify the responsible layer. | Every audit point; centralized in sections 8 and 9 | Every negative names canonical layer, component, expected/actual, event/source IDs, first divergence, and remediation category. |
| 10 | Phase 4 remains blocked until ORD-LIFE-CERT and preceding certification gates pass. | `ORD-LIFE-09`, `12`, acceptance contract | Predecessor artifact citations plus complete 0042 aggregate evidence, mutation result, and wording/ledger guard; no historical or partial promotion. |

### 6.2 Mandatory ordinary-life fixture families

Every family is required even when an existing file already appears to cover it. The implementing session must prove registry reachability and live execution; if an exact required property lacks a committed witness, it may add a narrowly scoped evidence fixture/test without changing gameplay doctrine.

| Mandatory family | Verified target fixtures and related witnesses | Primary points | Minimum certifying proof |
|---|---|---|---|
| Integrated no-human day | `no_human_day_001`, `no_human_advance_001`, `sleep_eat_work_001`, `ordinary_workday_001` | `01`, `02`, `03`, `05`, `09`, `10`, `11`, `12` | Multiple actors advance without human input through actor-known transactions, real progress or typed failure, and clean replay; canonical empty-food recovery remains fail-only. |
| Food unavailable | `food_unavailable_replan_001`, `partial_food_source_knowledge_001`, `seeded_food_source_unknown_to_all_actors_001`, `hidden_food_unknown_route_001` | `02`, `06`, `09`, `10` | Hunger cannot reveal or target hidden food; actor searches, falls back, waits, or becomes stuck with typed evidence. |
| Routine no-teleport | `routine_no_teleport_001`, `embodied_exits_require_perceived_or_known_route_001`, `hidden_route_edge_001`, `work_completion_fails_when_actor_displaced_001` | `04`, `08`, `11`, `12` | Remote work/sleep/eat requires movement ancestry or legal duration semantics; hidden route truth and displacement cannot be normalized away. |
| Possession does not reset intention | `possession_does_not_reset_intention_001`, `possession_parity_001` | `03`, `09`, `12` | Bind/unbind changes controller authority only; need, routine, intention, memory, actor-known state, decisions, and replay remain continuous. |
| Hidden-truth planning | `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `no_human_known_workplace_requires_provenance_001`, `forbidden_provenance_input_fails_closed_001` | `02`, `05`, `06`, `07`, `11`, `12` | Paired hidden differences do not affect cognition before legal evidence; forbidden/dangling provenance fails closed. |
| Planner trace | `planner_trace_001`, `method_fallback_requires_new_trace_or_stuck_001`, `debug_omniscience_excluded_001` | `04`, `06`, `07`, `09`, `12` | Selected and rejected candidates/methods, budget, plan/fallback, blockers, and actor-known refs are visible in debug but cannot feed cognition. |
| Routine blocker | `routine_blocked_diagnostic_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001`, `work_block_failed_then_sleep_succeeds_001` | `01`, `04`, `05`, `06`, `08`, `09`, `10`, `11`, `12` | Blocked routine emits typed blocker/layer/status and coherent wait/fallback/stuck/failure; no silent loop, reason rewrite, or accounting drift. |

### 6.3 Additional cross-cutting fixture families required by the audit

| Cross-cutting proof | Verified fixtures | Audit use |
|---|---|---|
| Single-charge and duration boundaries | `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001` | `ORD-LIFE-01`, `08`, `12` |
| Sleep affordance | `sleep_rejects_current_place_without_sleep_affordance_001`, `no_human_current_place_without_sleep_affordance_does_not_sleep_001`, `embodied_view_omits_unknown_sleep_affordance_001` | `ORD-LIFE-02`, `06`, `08` |
| Temporal premise/freshness | `ordinary_workday_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `stale_workplace_notice_superseded_by_newer_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001` | `ORD-LIFE-05`, `06`, `12` |
| Safety interruption | `severe_safety_with_known_exit_produces_move_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001` | `ORD-LIFE-02`, `03`, `04`, `08`, `10` |
| Metric/layer honesty | `no_human_metrics_require_typed_responsible_layer_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001` | `ORD-LIFE-09`, `10` |

### 6.4 Generated and metamorphic evidence

The implementing session must commit or identify generated properties covering at least:

- actor/need/tick single-charge conservation under window partitioning, action-duration insertion, interruption, and replay;
- determinism under map/set/input-order permutation;
- hidden-truth metamorphism: alter only unobserved food/route/affordance/workplace/schedule truth and require equal actor-known cognition output;
- provenance deletion/substitution: remove or wrong-kind one source at a time and require fail-closed behavior;
- lifecycle sequence generation: legal transitions replay identically; illegal terminal/reactivation/replacement sequences reject;
- planner-budget boundaries and coherent fallback;
- progress classification: adding a non-progress marker cannot increase progress, while replacing it with a permitted committed event changes the metric exactly once;
- debug/possession non-interference; and
- replay prefix/suffix perturbation with first-divergence attribution.

Generated evidence is `sampled` unless the artifact proves an exhaustive finite perimeter. Seeds, generator version, case count, shrink result, and omitted population must be recorded.

## 7. Mutation-testing posture

Mutation testing is a certification requirement for the guarded ordinary-life layer, not an optional quality score. A baseline-only or in-diff-only mutation run is phase-entry evidence, not a substitute for the complete configured perimeter.

### 7.1 Target-baseline configuration finding

The exact-commit `.cargo/mutants.toml` already covers broad `agent/**`, scheduler, projections, action pipeline, selected action definitions, events, replay, checksum, and related guarded surfaces. Static inspection does not by itself establish complete ORD-LIFE coverage. In particular, the final census must explicitly demonstrate inclusion of `need_accounting.rs` and every ordinary action/routine definition used by the gate; an implicit glob assumption is not certifying evidence.

### 7.2 Required configured union

The implementing session must produce a `cargo mutants --workspace --list-files` and `--list` census whose semantic union covers at least:

```text
crates/tracewake-core/src/agent/**
crates/tracewake-core/src/need_accounting.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/actions/registry.rs
crates/tracewake-core/src/actions/defs/need_events.rs
crates/tracewake-core/src/actions/defs/eat.rs
crates/tracewake-core/src/actions/defs/sleep.rs
crates/tracewake-core/src/actions/defs/work.rs
crates/tracewake-core/src/actions/defs/wait.rs
crates/tracewake-core/src/actions/defs/continue_routine.rs
crates/tracewake-core/src/actions/defs/movement.rs
crates/tracewake-core/src/events/**
crates/tracewake-core/src/replay/**
crates/tracewake-core/src/checksum.rs
crates/tracewake-core/src/state.rs
```

Content validation and TUI/debug files need inclusion only where an ORD-LIFE behavior or quarantine claim depends on them. The acceptance artifact must state included and excluded files, generated mutant count, skipped functions/reasons, baseline exclusions, and why each exclusion does not remove a gate-4 behavioral branch.

### 7.3 Required commands and waves

```bash
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --version
cargo mutants --workspace --list-files
cargo mutants --workspace --list
cargo mutants --workspace --no-shuffle
```

Run order:

1. unmutated workspace and named-suite baseline;
2. committed in-diff/ratchet job, retained as supporting evidence;
3. complete configured ORD-LIFE file and mutant census;
4. complete configured `--no-shuffle` run against a stable baseline;
5. deterministic rerun of any timeout, tool failure, flaky, or ambiguous result with all original evidence retained; and
6. final survivor classification and behavior-witness closure.

### 7.4 Required mutation properties

The perimeter must generate and tests must kill behavior-relevant mutants across:

- need bounds, threshold direction, tick coverage, interval boundaries, duplicate terminal detection, and single-owner accounting;
- candidate source classification, priority/tie breaks, hidden-target exclusion, and active-intention continuation/interruption;
- routine applicability, failure-mode presence, fallback selection, temporal-premise checks, and interruptors;
- planner budget decrement/termination, method/plan failure classification, provenance checks, and proposal construction;
- scheduler transaction invocation, no-direct-dispatch guards, progress classification, cross-tick stuck detection, and canonical recovery;
- affordance/location/reservation/duration validation and append-before-apply behavior;
- event replay handling, projection/metric derivation, diagnostic fields/layers, and semantic checksums; and
- planner-trace/debug non-interference where a mutant could create a feedback path.

A mutant is not closed merely because it lies in code reached by a test. The artifact must tie each caught mutant family to a behavior witness or property that would fail under the mutation.

### 7.5 Survivor triage register

Create `reports/0042_ord_life_cert_mutation_triage_register.md` using the historical `archive/reports/0040_epi_cert_mutation_triage_register.md` shape. Each row must record:

- stable historical mutant identity and final rerun identity;
- file, function, operator, source span, and tool version;
- tool outcome and exact command/transcript;
- mapped `ORD-LIFE-01` through `ORD-LIFE-12` point;
- responsible layer;
- reachability and fixture/property family;
- behavior witness and replay/provenance evidence;
- live negative or reason a negative cannot exist;
- disposition: caught, rigorously equivalent/unreachable, actionable survivor, timeout/tool failure, or pending; and
- evidence item IDs and certification use.

Equivalent/unreachable dispositions require a proof specific to the mutated semantics; a comment, low perceived risk, or high aggregate score is insufficient.

### 7.6 Mutation verdict routing

`ORD-LIFE-CERT passed` is impossible while any in-perimeter mutant remains actionable, untriaged, pending, timed out without closure, or hidden by a broken baseline/tool run. A completed survivor floor yields `ORD-LIFE-CERT scoped remediation`; the acceptance artifact must identify affected audit points and route to a later separate ORD-LIFE-CERT mutation-remediation and replacement-certification spec. This audit does not pre-author that remediation.

## 8. Failure handling and responsible-layer diagnostics

A failed audit point must produce a named failure package. It cannot be relabeled as a phase exception, deferred feature, observer-only concern, or acceptable historical debt.

Use the canonical execution-10 layer names consistently:

```text
doctrine
content_schema
content_validation
fixture_contract
holder_known_context
candidate_generation
intention_lifecycle
method_selection
local_planning
proposal_construction
scheduler
action_validation
event_append
event_application
projection
replay
view_model
debug_quarantine
tui_input_binding
test_oracle
```

When summarizing at the phase-ladder level, map these to the live document-03 categories: doctrine mismatch; content/schema validation; actor-known context construction; candidate generation; planning/method selection; proposal construction; scheduler ordering; action validation; event application; projection/replay; view-model rendering; debug quarantine; tests/fixtures; documentation status. Do not invent a new layer to avoid attribution.

### 8.1 Primary per-point diagnostic map

| Audit point | Primary layers |
|---|---|
| `ORD-LIFE-01` | `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `test_oracle` |
| `ORD-LIFE-02` | `holder_known_context`, `candidate_generation`, `intention_lifecycle`, `test_oracle` |
| `ORD-LIFE-03` | `intention_lifecycle`, `event_append`, `event_application`, `replay`, `view_model`, `test_oracle` |
| `ORD-LIFE-04` | `content_schema`, `content_validation`, `fixture_contract`, `method_selection`, `local_planning`, `test_oracle` |
| `ORD-LIFE-05` | `holder_known_context`, `candidate_generation`, `method_selection`, `scheduler`, `action_validation`, `replay` |
| `ORD-LIFE-06` | `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `test_oracle` |
| `ORD-LIFE-07` | `debug_quarantine`, `view_model`, `candidate_generation`, `local_planning`, `test_oracle` |
| `ORD-LIFE-08` | `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application`, `replay` |
| `ORD-LIFE-09` | `fixture_contract`, `holder_known_context`, `candidate_generation`, `scheduler`, `projection`, `replay`, `test_oracle` |
| `ORD-LIFE-10` | `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `scheduler`, `projection`, `replay` |
| `ORD-LIFE-11` | `holder_known_context`, `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application` |
| `ORD-LIFE-12` | `doctrine`, `event_append`, `event_application`, `projection`, `replay`, `test_oracle`, plus phase-ladder `documentation status` |

### 8.2 Required failure package

Every failed seam, fixture, property, replay comparison, or mutant must record:

1. gate label and local audit point;
2. canonical responsible layer and component/file/function;
3. exact command, tested commit/worktree, fixture/seed, and first failing event/tick;
4. expected and actual typed values, not only rendered prose;
5. actor/holder, context ID/hash/frontier, source-event IDs, proposal/event IDs, routine/intention/method/plan identity, and duration start/terminal where applicable;
6. actor-known input, hidden truth excluded or leaked, debug-only output, and validation truth separately;
7. live/replay first divergence and fingerprint scope;
8. metric/accounting impact and whether a wait was modeled or stuck;
9. mutation identity/disposition when applicable;
10. evidence status and whether it counts as certifying fail, pending, sampled, observer-only, or historical; and
11. remediation category and route.

A failed mutation floor or substantive seam routes to a later separately numbered ORD-LIFE-CERT remediation/replacement spec and a scoped-remediation acceptance artifact. It does not authorize a production fix inside this audit document.

## 9. Acceptance-artifact contract

The implementing session must instantiate `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` in a scoped artifact such as:

```text
reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md
```

On accepted closeout, repository convention may archive that artifact under archive/reports/. The exact final path must be recorded; path existence is not evidence of a verdict.

### 9.1 Artifact identity and scope

The artifact must state:

- exact commit actually tested and whether it equals the target `98dc0421211e6c9881d9c6679b9df74525e392bb`;
- branch/PR or detached-worktree identity as context only, never content proof;
- clean/dirty status and every evidence-only or implementation file changed;
- target repository and the exact-commit source discipline used;
- predecessor artifacts and their scoped certification use;
- `Work posture: Certification` and `ORD-LIFE-CERT` as a composed phase label;
- explicit statement that archived specs/tickets/reports are history only;
- scope: needs, routines, intentions, no-human life, planner traces, stuck diagnostics, and the actor-known ordinary-life transaction;
- excluded/downstream scope from section 11; and
- final aggregate wording limited to the exact tested commit and evidence package.

### 9.2 Gates and command ledger

Record every command in sections 4 and 7 with exact invocation, result, transcript/fingerprint, toolchain/tool versions, timestamps, relevant environment, retries, flakes, timeouts, and deviations. A missing, pending, skipped, or irreproducible required command blocks aggregate pass.

### 9.3 Per-requirement and per-seam verdict tables

Include one table for `ORD-LIFE-01` through `ORD-LIFE-12`, one for the ten live pass conditions, and one for the seven mandatory fixture families. Each row must name responsible layer, certifying evidence item IDs, negative controls, mutation entries, and result computed only from certifying evidence.

No row may pass from historical, pending, sampled-only, observer-only, static-only, snapshot-only, or aggregate-score evidence.

### 9.4 Evidence-item ledger fields

For every cited item, record exactly the template fields:

- **Evidence item ID:** stable local ID;
- **Requirement IDs:** applicable local audit points and live pass conditions;
- **Evidence status:** `pass`, `fail`, `pending`, `sampled`, `observer-only`, or `historical`;
- **Fingerprint scope:** raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, replay artifact, or not applicable;
- **Evidence summary:** command, artifact, file/line, report section, fixture, trace, or replay reference;
- **Behavior witness:** path under test; command/event/trigger/emitter/scheduler entry; responsible layer; accepted/rejected action or validation stage; live negative or reason none applies; checked facts/invariants;
- **Replay/provenance ancestry:** event segment/IDs; replay artifact; seed/randomness/content/ruleset versions; extraction/projection version; actor-known source provenance;
- **Sampling/exhaustiveness scope:** exhaustive perimeter or sampled population, basis, omissions, and representativeness;
- **Pending/historical handling:** missing proof and owner/blocker, or context/lineage/precedent classification; and
- **Certification use:** counted as certifying pass, counted as certifying fail, or not counted as certifying evidence.

### 9.5 Ordinary-life behavior-witness package

For each mandatory fixture family and audit point, attach or reference:

- fixture definition/registry identity and schema validation;
- no-human or possessed entry path;
- actor-known packet and temporal/routine source evidence;
- candidates, methods, local plan, budget, proposal, validation, and event ancestry;
- need/accounting and intention/routine transition ledgers;
- movement/affordance/duration evidence;
- progress/wait/stuck classification and metrics;
- live negative/adversarial perturbation;
- replay rebuild and first-divergence report; and
- authorized debug trace and non-interference comparison where applicable.

### 9.6 Replay and provenance package

Include exact log bytes and parsed semantic scope, event schema/replay handling census, applied event range, content/ruleset/seed identity, live/replay fingerprints for all ordinary-life projections, actor-known source-event table, proposal context parity, per-need/tick accounting ledger, metric derivation, stuck-diagnostic derivation, and decision-context hash rebuild. State explicitly what each fingerprint does and does not cover.

### 9.7 Mutation package

Include cargo-mutants version, configuration fingerprint, file/mutant census, baseline, in-diff supporting run, full configured run, caught/missed/timeout/tool-failure counts, survivor register, reruns, equivalence proofs, mapped audit points/layers, and final floor. Pending is not pass.

### 9.8 Staged-abstraction declaration

Instantiate the template’s staged-abstraction fields for the bounded first ordinary-life proof:

- what the artifact proves now;
- what it deliberately abstracts;
- what it must not fake while using that abstraction;
- what later feature/doctrine tier it must not block;
- what evidence prevents overclaiming; and
- diagnostics distinguishing not implemented, intentionally abstracted, implemented but broken, and overclaimed.

The declaration is non-certifying by itself. It may not convert the staged success-recovery variant, broader economy, institutions, travel, or FIRST-PROOF temporal bundle into an ORD-LIFE pass claim.

### 9.9 Residual convention-only items

List every item that cannot be structurally or mechanically enforced, why review is still required, the exact review surface, owner, and evidence status. “Convention only” cannot excuse hidden-truth cognition, no-direct-dispatch, single-charge, possession reset, trace feedback, or replay derivation.

### 9.10 EMERGE-OBS handling

An `EMERGE-OBS` ordinary-life observation artifact may be included as an evidence-package member with `status = observer-only`. It may summarize emergent patterns, actor variety, surprising but causal outcomes, or long-run observations. It never becomes a phase gate, pass/fail threshold, scheduler objective, scenario goal, mutation substitute, or code-quality score.

### 9.11 Aggregate verdict rule

The acceptance artifact may render **`ORD-LIFE-CERT passed`** only if:

1. all twelve local audit points pass from certifying evidence;
2. all ten live pass conditions pass;
3. all seven mandatory fixture families have positive/adversarial, replay, provenance, and layer-attribution evidence;
4. the clean baseline and every required named command pass without unresolved flake or omission;
5. the configured mutation perimeter is complete and has no actionable, pending, untriaged, timed-out, or tool-failed floor;
6. evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling, pending/historical, certification-use, and staged-abstraction fields are complete and honest;
7. predecessor-gate citations remain scoped and Phase 4 remains blocked until this artifact is accepted; and
8. no tolerated deferral was used to hide an implemented-but-broken in-scope seam.

If a substantive audit point or mutation floor remains, the artifact must render **`ORD-LIFE-CERT scoped remediation`**, name every failed row/layer/evidence gap/survivor, and route to a later separate remediation/replacement spec. This specification itself renders neither verdict.

## 10. Preliminary static survey — not certification

This section is informative only. It does not satisfy an audit point, run a test, prove replay, or render a verdict.

Exact-commit static reading suggests substantial ordinary-life scaffolding is present:

- typed need kinds/bands/pressures, threshold changes, tick regimes, duration intervals, terminal detection, and need-event construction;
- deterministic candidate-goal structures and source classes, actor-known packet construction, decision transactions, and sealed proposal/stuck outcomes;
- routine templates with conditions, method families, steps, failure modes, fallback rules, method selection, bounded local planning, and typed traces/blockers;
- eat/sleep/work/wait/movement action definitions, proposal validation, duration handling, no-human scheduling, metrics, and ordinary-life event/replay families;
- a broad positive/adversarial fixture corpus directly corresponding to the seven mandatory families and several single-charge, temporal-premise, provenance, and debug risks; and
- configured mutation and CI infrastructure inherited from the preceding certification campaign.

Static risk flags the implementing session must resolve dynamically:

- Live doctrine names intention lifecycle semantics `active`, `continued`, `suspended`, `interrupted`, `completed`, `failed`, `abandoned`, and `replaced`. The visible implementation representations are split across intention status, decision lifecycle effects, and events. The audit must prove the full semantic lifecycle and ancestry rather than infer it from one enum.
- `scheduler.rs` supplies routine-window information into the decision path. The temporal-premise audit must prove that true day-window/schedule position is not silently treated as actor-known justification.
- `scheduler.rs` also participates in passive need-event construction while live doctrine assigns a single action-pipeline/ordinary-life accounting owner. The audit must establish the exact ownership/delegation boundary and show that scheduler logic does not become a second charger.
- The standing mutation configuration broadly includes `agent/**` but does not make complete coverage of `need_accounting.rs` and every ordinary-action definition self-evident. The final file/mutant census must close the gap.
- The code-level `ResponsibleLayer` representation visible in `crates/tracewake-core/src/agent/trace.rs` is narrower than the full execution-10 review vocabulary. The artifact must prove that event append/application, replay, fixture, documentation, and other required failures can still be attributed without collapsing distinct layers.
- Existing snapshots and happy paths cannot by themselves prove relational hidden-truth non-interference, debug/possession non-interference, single-charge conservation, modeled temporal premises, or cross-tick stuck detection. Those require paired/generated/negative evidence.

None of these observations is a pass or failure determination.

## 11. Tolerated deferrals and explicit out-of-scope boundaries

ORD-LIFE-CERT is comprehensive for gate 4 but must not absorb predecessor or downstream certification substance:

- **Predecessor-certified SPINE-CERT seams:** general event-log authority, replay infrastructure, append/apply ordering, action-pipeline architecture, no-direct-dispatch architecture, and general TUI/debug separation are consumed from the scoped 0039 pass. This audit regression-checks only their ordinary-life-facing use; it does not re-audit SPINE-CERT.
- **Predecessor-certified EPI-CERT seams:** general holder-known context construction, belief/observation/provenance machinery, possession parity, embodied view models, and debug capability quarantine are consumed from the scoped 0041 pass. This audit proves ordinary-life cognition and trace behavior on top of them; it does not re-audit EPI-CERT.
- **Temporal cascade:** `ORD-LIFE-05` certifies the routine temporal-premise mechanism. The consolidated bundle joining the temporal firewall from execution `04`, routine temporal proof from `06`, embodied temporal rendering from `07`, temporal fixtures from `09`, and temporal diagnostics from `10` remains a routed `FIRST-PROOF-CERT` obligation. It is not an ORD-LIFE pass/fail line.
- **Success-recovery variant:** the canonical `no_human_day_001` outcome remains `fail_only_empty_food_source`. Public food services, neighbor/public pantry access, and the success-recovery variant remain staged to Phase 3B routine/economy work. This deferral cannot authorize hidden fallback food now.
- **FIRST-PROOF-CERT:** coherent first-playable composition, missing-property village acceptance, embodied temporal bundle, broader living-world quality, and integrated first-proof narrative sufficiency are downstream. `EMERGE-OBS` remains observer-only.
- **PHASE-4-ENTRY:** institutions, records, wrong suspicion, reports, households/norm procedures, and institutional failure remain locked behind Phase 4 entry.
- **SECOND-PROOF-ENTRY:** notices as a broad system, travel, regional scale, LOD, prehistory, and long-range world processes remain downstream.
- **Broader simulation:** personality depth, complex social drama, full economy, gossip, LLM conversation, and generalized learning/trust semantics are explicit non-goals unless an already-implemented behavior crosses an in-scope ordinary-life seam and is broken.
- **Performance:** timing and scale may be recorded, but are not ORD-LIFE thresholds unless they change determinism, bounded planning, progress/stuck semantics, or evidence integrity.

No deferral may weaken actor-known-only cognition, source-backed temporal premises, single-owner/single-charge accounting, durable intentions, defeasible routine failure modes, bounded local planning, causal movement/affordances, no-direct-dispatch, forged/stale validation rejection, meaningful progress metrics, typed stuck diagnostics, planner-trace quarantine, or replay derivation.

## 12. External research notes shaping the audit design

External sources inform audit method only. They establish no fact about Tracewake’s repository and cannot certify a seam.

- HTN work such as SHOP2 treats planning as method-based task decomposition with applicability conditions and alternative decompositions. The audit therefore requires each routine family to expose applicability, rejected methods, explicit failure/fallback edges, and bounded plan construction; a recorded happy-path action list is not enough.[^shop2]
- BDI literature treats intentions as a deliberative commitment distinct from beliefs and motivations. That sharpens the requirement to test persistence, interruption, replacement, terminal states, and explicit reasons rather than merely recomputing the highest-pressure goal each tick.[^bdi]
- Motivation/autonomy work distinguishes internal motivations such as hunger or self-preservation from the goals generated to address them. This supports the repository doctrine that a need may generate candidate families but cannot itself name a true target or primitive action.[^motivation]
- Partial-observability planning maintains an information state from observation history instead of choosing directly from omniscient state. Tracewake is not being claimed as a POMDP; the practical audit consequence is a strict actor-known information-set comparison and paired hidden-truth runs.[^pomdp]
- QuickCheck-style property testing checks declared properties over generated inputs, while metamorphic testing compares related executions when a single expected output is hard to enumerate. The audit applies both to accounting conservation, permutation determinism, hidden-truth perturbation, provenance deletion, debug/possession non-interference, and replay relations.[^quickcheck][^metamorphic]
- Mutation testing asks whether injected behavior changes are detected, not merely whether lines execute. The audit therefore requires a configured ordinary-life mutant census and behavior-linked survivor triage rather than a coverage percentage or aggregate mutation score.[^cargo-mutants]
- Liveness arguments can depend on hidden fairness assumptions. The no-human proof therefore records modeled wait reasons and review boundaries, and treats no-progress-past-window/repeated-idle as explicit outcomes instead of assuming the scheduler will eventually make an actor progress.[^fairness]

[^shop2]: Dana S. Nau et al., “SHOP2: An HTN Planning System,” *Journal of Artificial Intelligence Research* 20 (2003), 379–404, DOI 10.1613/jair.1141, <https://www.jair.org/index.php/jair/article/view/10362>.
[^bdi]: Anand S. Rao and Michael P. Georgeff, “BDI Agents: From Theory to Practice,” ICMAS 1995, <https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf>.
[^motivation]: Michael Luck and Mark d’Inverno, “A Formal Framework for Agency and Autonomy,” ICMAS 1995, <https://cdn.aaai.org/ICMAS/1995/ICMAS95-034.pdf>.
[^pomdp]: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, “Planning and Acting in Partially Observable Stochastic Domains,” *Artificial Intelligence* 101 (1998), 99–134, DOI 10.1016/S0004-3702(98)00023-X, <https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf>.
[^quickcheck]: Koen Claessen and John Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs,” ICFP 2000, DOI 10.1145/351240.351266, <https://dl.acm.org/doi/10.1145/351240.351266>.
[^metamorphic]: T. Y. Chen, S. C. Cheung, and S. M. Yiu, “Metamorphic Testing: A New Approach for Generating Next Test Cases,” HKUST-CS98-01 (1998), <https://www.cse.ust.hk/faculty/scc/publ/CS98-01-metamorphictesting.pdf>.
[^cargo-mutants]: cargo-mutants documentation, “Welcome” and “How cargo-mutants works,” <https://mutants.rs/> and <https://mutants.rs/how-it-works.html>.
[^fairness]: Rob van Glabbeek and Peter Höfner, “Progress, Justness and Fairness,” *ACM Computing Surveys* 52(4) (2019), DOI 10.1145/3329125, <https://dl.acm.org/doi/fullHtml/10.1145/3329125>.

## 13. Self-check against this specification

- [x] The spec opens with evidence-based confirmation that ORD-LIFE-CERT is the next admissible move after the scoped 0037 P0-CERT, 0039 SPINE-CERT, and 0041 EPI-CERT passes; it proposes no feature expansion.
- [x] Needs, routines, intentions, no-human ordinary life, planner traces, stuck diagnostics, and the actor-known transaction are covered by twelve numbered local audit points with verified target-commit modules.
- [x] Every audit point includes doctrine, positive and adversarial properties, exact commands, event/replay/projection evidence, applicable actor-known/debug evidence, named layers, and condition/fixture coverage.
- [x] Section 6 maps all ten numbered live pass conditions and all seven mandatory fixture families; none is omitted.
- [x] The routine temporal-premise mechanism is audited in `ORD-LIFE-05`; the consolidated temporal cascade is routed to FIRST-PROOF-CERT.
- [x] The document is non-executable, fabricates no command/replay/mutation result, and renders no pass/fail verdict.
- [x] This is the 0042 audit-spec analogue of 0036/0038/0040, not a remediation spec. A failed floor routes to a later separate spec without pre-authoring its fix.
- [x] The header uses target commit `98dc0421211e6c9881d9c6679b9df74525e392bb`, `Work posture: Certification`, staging under `specs/`, and the three predecessor artifacts.
- [x] `ORD-LIFE-CERT` is treated as a composed label; no new gate code, invariant ID, status enum, or obligation code is minted.
- [x] Archived Phase-3A specs remain historical evidence only; `EMERGE-OBS` remains observer-only and non-gating.
- [x] Authority order, `core <- content <- tui`, actor-known-only cognition, single-charge, no-direct-dispatch, possession neutrality, no teleport, planner-trace non-diegesis, and replay derivation are preserved.
- [x] The acceptance contract instantiates evidence status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical handling, certification use, and staged-abstraction fields.
- [x] The mutation posture requires a complete configured guarded-layer census, full run, behavior-linked survivor triage register, and scoped-remediation route.
- [x] Every repository path named in the authority list, seam inventory, audit points, and fixture matrices is present at the target commit. The 225 paths in the section 2.1 fetch pass were fetched by exact-commit URL before use; the twelve additional referenced paths enumerated in the section 2.2 reassessment addendum were confirmed present at the target commit by direct working-tree inspection (working tree == target commit) rather than in that fetch pass.

## Outcome

Completed: 2026-06-20

This specification was executed by the `0042ORDLIFCER-001` through `0042ORDLIFCER-016` ticket series. The acceptance evidence lives in `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md`, and the mutation posture evidence lives in `reports/0042_ord_life_cert_mutation_triage_register.md`.

Final verdict: `ORD-LIFE-CERT scoped remediation`, not `ORD-LIFE-CERT passed`. Local ORD-LIFE-01 through ORD-LIFE-12 evidence, the ten live pass conditions, the seven mandatory fixture families, generated/metamorphic evidence, replay/provenance packaging, staged-abstraction handling, and observer-only `EMERGE-OBS` handling were recorded. The aggregate pass is blocked because the configured mutation lane did not complete and the partial exact run exposed three actionable missed `need_accounting.rs` mutants mapped to `ORD-LIFE-01`, `ORD-LIFE-08`, and `ORD-LIFE-12`.

Remediation is routed to a later separately numbered ORD-LIFE-CERT mutation remediation/replacement spec. `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, latest-main certification, and full-project certification remain unasserted.
