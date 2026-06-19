# 0040 EPI-CERT holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug-quarantine certification spec

```text
Staging path: specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
Archive path on accepted closeout: archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
Target repository: joeloverbeck/tracewake
Target commit: ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab
Spec series: numbered staging spec 0040; archived to archive/specs/ on accepted closeout
Status: COMPLETED — implementing-session acceptance artifact rendered EPI-CERT scoped remediation
Work posture: Certification
Admissibility posture: consumes P0-CERT passed from archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md and SPINE-CERT passed from archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md
Phase-certification label: EPI-CERT, as defined by docs/2-execution/03; this document mints no new gate code, invariant ID, status enum, or obligation code
```

This document is **non-executable**: it specifies what the implementing session must run, prove, record, and package. It renders no pass/fail verdict, does not assert that the target implementation passes, and does not authorize production remediation inside this audit spec.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 0. Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run exact full-URL open/find and container.download exact full-URL fetch for local symbol inspection
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/reports/0037_mutation_triage_register.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/reports/0038_spine_cert_mutation_triage_register.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/reports/0039_spine_cert_mutation_triage_register.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/specs/0013_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/rust-toolchain.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/AGENTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/ids.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/knowledge_basis.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/knowledge_context.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/belief.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/observation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/proposition.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/contradiction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/no_human_surface.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/candidate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/generation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/debug_reports.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/generative_lock.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/doc_invariant_references.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/support/generative.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/support/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/debug_attach_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_exits_require_perceived_or_known_route_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_view_omits_unknown_sleep_affordance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/view_filtering_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_belief_literal/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_belief_literal/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_debug_report/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_debug_report/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/src/lib.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/epistemics/knowledge_context.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/no_human_surface.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/perception.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/transaction.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/banned_float_confidence_types/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/banned_float_confidence_types/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/tests/spine_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/Cargo.lock
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/src/lib.rs
- https://github.com/joeloverbeck/tracewake/blob/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/crates/tracewake-core/src/debug_capability.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

---

## 1. Determination confirmation: EPI-CERT is the next admissible move

`EPI-CERT` is the single next admissible Tracewake staging spec. The live phase ladder orders certification as `P0-DOC -> P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY` and defines gate 3 as certifying actor-known/holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug quarantine. It also makes Phase-4 or expansion work invalid before the preceding gates pass. See [`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md).

The live ledger records the replacement 0037 artifact as `P0-CERT passed` for its stated scoped line and the replacement 0039 artifact as `SPINE-CERT passed` for its stated scoped line at implementation commit `92ba47f14998e0ea2fc95502bc3b76c5909478ca`. It expressly does **not** declare `EPI-CERT` or any later gate. See [`docs/4-specs/SPEC_LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/docs/4-specs/SPEC_LEDGER.md), [`archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md), and [`archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`](https://raw.githubusercontent.com/joeloverbeck/tracewake/ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab/archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md).

Accordingly, this spec consumes those two predecessor-gate facts only within the scopes stated by their artifacts. It does not re-commission `P0-CERT` or `SPINE-CERT`, does not infer latest-main certification, and does not survey gameplay expansion or alternative next features. Historical Phase 2A work is evidence to examine, not an `EPI-CERT` verdict.

The staging sequence in the manifest is contiguous through archived spec `0039`; the live `0001` and review-template `0003` entries do not restart that staging series. The next numbered staging spec is therefore `0040`.

## 2. Source discipline, freshness, and admissibility

The following source-discipline rules are carried verbatim from the live ledger:

- A commit hash named inside a spec is audit/spec provenance only unless a higher-tier document says otherwise.
- A manifest is path inventory only.
- Branch names, default-branch lookups, connector namespace labels, repository metadata, and code-search snippets are not proof of target-commit content.
- Future exact-commit audits must fetch by exact file URL or by a supplied repository export at the target commit.
- Stale baseline strings in historical specs must not be adopted as current product doctrine.

Additional application rules for this audit are binding:

- The uploaded manifest was used only to establish that every named repository path is present in the user-supplied inventory for `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`. The content evidence is the exact-commit fetch ledger in section 0.
- Archived specs, tickets, research briefs, and acceptance artifacts are historical evidence unless this document explicitly consumes an artifact for the settled predecessor-gate posture. Historical `0004` and `0013` contribute lineage and risk memory only; they do not certify the live EPI boundary.
- `EPI-CERT` is a phase-certification label that composes canonical architecture gates and review artifacts. This spec may operationalize that label; it may not define, weaken, rename, or replace its upstream semantics.
- `holder-known context` is the system-wide term. `actor-known` names the actor-holder case. Neither term means world truth, debug omniscience, a validation snapshot, or a prose summary.
- The authoring target is the exact tree at `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`. An implementing session that adds test or evidence harnesses must enumerate every delta and record the exact final commit actually tested. It may not imply that unchanged `ba9fe1c` passed unless the certifying commands actually ran against that unchanged tree. This audit authorizes evidence instrumentation, not production remediation; a substantive failed seam routes to the separate remediation posture in section 9.
- A pass claim is blocked if any result depends on an unfetched repository file, a branch-relative file, a snippet, repository search, mutable metadata, prior-chat memory, or a hidden local modification.
- No backward-compatibility shim, alias path, duplicate constructor, or convenience API may preserve an epistemic bypass. New evidence must exercise the canonical live seam.

## 3. Authority and dependency declarations

The governing order is `docs/0-foundation/` -> `docs/1-architecture/` -> `docs/2-execution/` -> `docs/3-reference/` -> `docs/4-specs/` -> staged spec -> implementation. If execution conflicts with architecture or foundation, execution is wrong. If implementation convenience conflicts with an accepted gate, implementation is wrong.

### 3.1 Primary foundation dependencies

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`

The audit is especially constrained by the event-before-state and replay invariants; subjective-epistemics, privacy, provenance, staleness, and wrong-belief invariants; embodied/debug and possession-parity invariants; evidence-honesty invariants; and the actor-known transaction, validation-truth, hidden-cognition, debug-quarantine, possession-neutrality, observer-only, and temporal-firewall invariants. This spec cross-references existing invariant IDs only and does not create one.

### 3.2 Primary architecture dependencies

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

Architecture requires a sealed holder-known packet, provenance sufficient to reconstruct why each fact is available, replay-derived epistemic projections, strict privacy scopes, decision proposals bound to the exact source context, embodied views with no live truth handle, and a structurally separate non-diegetic debug channel.

### 3.3 Primary execution dependencies

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

Execution requires positive and adversarial fixtures, event/replay/projection witnesses, source-event provenance, possession parity, view-model filtering, debug quarantine, evidence status and fingerprint honesty, and diagnostics attached to an existing responsible layer. `EMERGE-OBS` remains observer-only.

### 3.4 Primary reference and spec-tier dependencies

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

The canonical terms include `holder-known context`, `actor-known`, `observation channel`, `contradiction`, `debug quarantine`, and `EMERGE-OBS`. The risk register's contamination, hidden-truth leakage, possession-parity drift, freshness/staleness, evidence Goodharting, and debug-boundary risks remain active review concerns.

### 3.5 Crate and authority direction

The one-way dependency and authority direction is `tracewake-core <- tracewake-content <- tracewake-tui`:

- `tracewake-core` owns event application, authoritative state, epistemic records/projections, sealed contexts, decision transactions, proposal validation, and debug capabilities.
- `tracewake-content` may declare/load/validate seeds and fixtures through core-owned types, but it may not create simulation fact from prose or bypass event/provenance requirements.
- `tracewake-tui` may render embodied and authorized debug view models and submit semantic actions, but it may not become a truth writer, mint core capabilities, mutate epistemic state, or make a debug report diegetic.

A reverse dependency or client-owned truth path is an EPI-CERT failure even if current tests appear green.

## 4. Command and evidence vocabulary for the implementing session

The commands below are the minimum execution vocabulary. The final artifact must preserve the exact command actually run, working tree/commit identity, exit status, start/end timestamps, tool versions, environment facts relevant to determinism, and full output or a cryptographically fingerprinted transcript with its stated fingerprint scope.

### 4.1 Clean baseline

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --locked -p tracewake-core --doc
```

A failing or flaky unmutated baseline blocks interpretation of later evidence. Retrying until green without retaining and explaining the failure is not certifying evidence.

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

The implementing session may add test filters for diagnosis, but filtered runs do not replace the complete named binaries or the workspace run. Every required fixture in section 6 must be reachable through the committed fixture registry and exercised by the recorded golden runner; a test name merely containing the fixture's name is not proof that its event/replay/view path ran.

### 4.3 Required evidence objects

For each EPI audit point, package the applicable objects below:

1. exact input fixture, run seed, content/ruleset identity, and starting state/projection fingerprints;
2. ordered event envelopes and the source-event IDs cited by every observation, belief, contradiction, actor-known fact, proposal, and view-model evidence row;
3. live and replay-rebuilt authoritative and epistemic projection fingerprints with their exact scopes;
4. sealed holder-known packet identity, context hash, event frontier, schema/projection version, scope filters, source summary, and forbidden-truth audit;
5. proposal source context ID/hash/frontier and validation report split into actor-visible and debug-only evidence;
6. embodied view-model and rendered embodied transcript, plus the separately authorized debug view/report/transcript when debug quarantine is under test;
7. behavior witness and negative control, not merely a source-code assertion or output snapshot;
8. first divergence and responsible-layer diagnostic for any failed comparison; and
9. sampling/exhaustiveness statement that distinguishes a finite named matrix, exhaustive registry run, generated sample, mutation census, and observer-only evidence.

### 4.4 Cross-cutting proof rules

- **Event backed, not state inferred.** An epistemic record or actor-known fact must identify the event-backed witness by which the holder could know it. A matching current world state is not provenance.
- **Replay derived, not projection authored.** The live epistemic projection and a clean rebuild from the same accepted event stream must agree. A projection checksum alone does not prove the projection was not written directly.
- **Relational hidden-truth proof.** At least one paired-run witness must vary only hidden world truth while preserving the holder's observable history; actor-visible context, decision, action availability, and embodied view must remain equal until an admissible observation event distinguishes the runs.
- **Possession parity.** Human possession may change controller binding and command source, not actor knowledge, cognition, intention state, semantic action set, or validation rules.
- **Debug non-interference.** Enabling, rendering, or querying authorized debug surfaces may expose diagnostic truth to the operator, but must not change simulation events, authoritative state, epistemic projection, actor-known packet, proposal, or embodied output.
- **No prose authority.** Text, summaries, diagnostics, LLM output, fixture prose, or debug labels may not create a proposition, observation, belief, event, or decision fact without the typed/event-backed path prescribed upstream.
- **No evidence laundering.** Static structure, historical artifacts, pending work, sampled evidence, observer-only evidence, snapshots without behavior witnesses, or a high mutation score cannot silently satisfy a certifying row.

---

## 5. The EPI-CERT audit contract

The implementing session must execute and report every audit point `EPI-01` through `EPI-11` as one gate. The identifiers are local cross-references within this spec, not new doctrine-level gate codes or invariant IDs.

### Verified implementation seam inventory at the target commit

The exact-commit static inventory used to define the audit points is:

| Area | Verified target file | Load-bearing types/functions/surfaces to exercise |
|---|---|---|
| Sealed context | `crates/tracewake-core/src/epistemics/knowledge_context.rs` | `KnowledgeContext`, `ViewMode::{Embodied, Debug}`, allowed/forbidden source sets, `ScopeFilter`, provenance entries, holder-known ID/hash/frontier, fact families, forbidden-truth audit |
| Knowledge basis | `crates/tracewake-core/src/epistemics/knowledge_basis.rs` | typed knowledge-basis support consumed by the epistemic surface |
| Beliefs | `crates/tracewake-core/src/epistemics/belief.rs` | `Belief`, `HolderKind`, `Stance`, bounded `Confidence`, source/schema/privacy fields |
| Observations | `crates/tracewake-core/src/epistemics/observation.rs` | `Observation`, `Channel`, `Confidence`, `TickWindow`, `EPISTEMIC_RECORD_SCHEMA_V1`, source and privacy fields |
| Propositions | `crates/tracewake-core/src/epistemics/proposition.rs` | typed propositions shared by beliefs, observations, and contradictions |
| Contradictions | `crates/tracewake-core/src/epistemics/contradiction.rs` | `Contradiction`, `ContradictionKind::ExpectedItemAbsentFromContainer`, `detect_expected_absences` and linked derived records |
| Epistemic aggregate | `crates/tracewake-core/src/epistemics/projection.rs` | `EpistemicProjection`, projection checksum/range, context-filtered reads, `ActorKnownProjectionRecord`, freshness/policy classification, notebook/debug views |
| Actor-known facts | `crates/tracewake-core/src/agent/actor_known.rs` | `ActorKnownFact`, checked `SourceEventIds`, `ActorKnownProvenance`, restricted provenance, `HiddenTruthAudit` derivation |
| Sealed cognition input | `crates/tracewake-core/src/agent/no_human_surface.rs` | `SealedActorKnownSurface` and projection-to-typed-witness construction |
| Perception | `crates/tracewake-core/src/agent/perception.rs` | multi-channel perception capture, current-place event creation, projection application |
| Decision transaction | `crates/tracewake-core/src/agent/transaction.rs` | `SealedProposal`, `ActorDecisionTransaction`, dangling/wrong-kind provenance diagnostics |
| Diagnostic trace | `crates/tracewake-core/src/agent/trace.rs` | `HiddenTruthAudit`, typed blockers, and `ResponsibleLayer` including `DebugQuarantine` |
| Proposal/validation | `crates/tracewake-core/src/actions/proposal.rs`, `crates/tracewake-core/src/actions/pipeline.rs`, `crates/tracewake-core/src/actions/report.rs` | `ProposalSourceContext` ID/hash/frontier/actor/action/provenance tuple, validation and actor/debug report split, event-first epistemic updates |
| Embodied source | `crates/tracewake-core/src/projections.rs` | `EmbodiedProjectionSource`, `EmbodiedTruthSnapshot`, preflight source and holder-known sealed-context boundary |
| View models | `crates/tracewake-core/src/view_models.rs` | `EmbodiedViewModel`, `NotebookView`, `ActionAvailability`, `ActionAvailabilityProvenanceKind`, `WhyNotView`, and separate `DebugViewModel`/`Debug*View` families |
| Controller/possession | `crates/tracewake-core/src/controller.rs` | controller binding/authorization, possession-neutrality tests, embodied build inputs and same-action parity |
| Debug authority | `crates/tracewake-core/src/debug_capability.rs`, `crates/tracewake-core/src/debug_reports.rs` | mint-restricted `DebugCapability`, compile-fail authority boundary, capability-sealed debug report types, non-diegetic marker |
| TUI quarantine | `crates/tracewake-tui/src/debug_panels.rs`, `crates/tracewake-tui/src/render.rs`, `crates/tracewake-tui/src/app.rs`, `crates/tracewake-tui/src/transcript.rs` | `debug_only()` assertions, `DEBUG NON-DIEGETIC` labeling, `DEBUG_TOKENS` embodied filtering, separate overlay/transcript behavior |
| Behavior corpus | `crates/tracewake-content/src/fixtures/*`, core/content/TUI test binaries, and `tests/negative-fixtures/*` named in sections 4 and 6 | positive, hidden-truth, contradiction, possession, view, replay, source-provenance, debug, compile-fail, and generated witnesses |

The inventory establishes where to look; it is not evidence that any seam passes.

### EPI-01 — Sealed holder-known context construction, scope, identity, hash, and frontier

**Seam definition.** Every actor-visible decision or view must be derived from a sealed holder-known context whose holder, viewer, mode, tick, event frontier, projection/schema identity, scope filters, admitted sources, excluded sources, provenance entries, actor-known fact families, forbidden-truth audit, and status are fixed before consumption. In the actor case, embodied construction binds viewer and holder to the same actor. Debug construction is capability-gated and non-diegetic. Context identity/hash/frontier must travel unchanged into downstream proposal and view evidence.

**Audited files/modules.**

- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/**`
- `tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/**`
- `tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/**`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`

**Doctrine to satisfy.** `INV-001`, `INV-022`, `INV-024`, `INV-026`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-107`, and `INV-108`; foundation documents `04`, `08`, and `14`; architecture documents `03`, `05`, and `10`; execution documents `04` and `07`.

**Required positive fixtures/properties.**

- Build the embodied holder-known context twice from the same accepted event prefix, projection, actor, tick, and content/ruleset versions; prove identical semantic packet, context ID, context hash, frontier, fact ordering, source summary, and actor-visible reads.
- Prove the context includes only own direct/search/sound/absence observations and source-backed beliefs admitted by the live policy, with actor-private/public scope behavior matching the holder.
- Build `SealedActorKnownSurface` from `EpistemicProjection`, carry its context ID/hash/frontier through `SealedProposal`, and show the pipeline validates the same values before accepting the proposal.
- Exercise `view_filtering_001`, `no_human_epistemic_check_001`, `no_human_observation_facts_cite_log_events_001`, and `workplace_assignment_provenance_001` as concrete packet-construction witnesses.

**Required adversarial fixtures/properties.**

- Paired worlds that differ only in an unobserved hidden item, route edge, workplace truth, other actor's private belief, or attached debug state must produce the same embodied context packet and hash for the focal actor.
- Canonically equivalent provenance/fact input orders and duplicate source IDs must normalize to the same packet; changing any semantic field—holder/viewer, mode, tick, frontier, schema/projection identity, privacy filter, allowed/forbidden source set, provenance source, fact, forbidden-truth result, or status—must either change the hash or fail closed.
- A context for actor A must not admit actor B's private belief, actor B's observation, prior possessed actor knowledge, raw event-log truth, hidden item location, or human debug notes.
- The three compile-fail fixtures above must continue to prove that an external crate cannot forge a debug context or mutate sealed mode/viewer identity.
- A proposal carrying a stale, mismatched, or forged context ID/hash/frontier must be rejected with a typed layer diagnosis and must append no accepted action event.

**Required evidence mechanics.** Record the canonical holder-known packet serialization used for the comparison; fingerprint scope; context ID/hash/frontier; projection/event prefix; complete provenance-to-event table; actor/privacy filter decisions; accepted and rejected proposal source bindings; live/replay context equality; and the first differing field for every negative perturbation. A string match on banned words is not a hidden-truth proof.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Failure-diagnostic layers.** `doctrine mismatch`; `actor-known context construction`; `proposal construction`; `projection/replay`; `debug quarantine`; `tests/fixtures`.

### EPI-02 — Typed propositions, beliefs, stance/confidence, privacy, and freshness

**Seam definition.** Beliefs must remain typed, holder-scoped, fallible, source-backed epistemic records distinct from observations, memories, records, validation truth, and world state. Stance and confidence must be explicit; confidence uses the repository's bounded integer representation rather than floats. Actor beliefs are private to their actor unless an upstream-supported public/institutional path says otherwise. Staleness and supersession must be visible rather than silently converted into current truth.

**Audited files/modules.**

- `crates/tracewake-core/src/epistemics/proposition.rs`
- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/view_models.rs`
- `tests/negative-fixtures/banned_float_confidence_types/**`
- `tests/negative-fixtures/external_crate_cannot_construct_belief_literal/**`
- `tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/**`
- `tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/**`

**Doctrine to satisfy.** `INV-021` through `INV-031`, `INV-099` through `INV-102`, and `INV-112`; foundation `04` and `14`; architecture `03` and `06`; execution `04`, `07`, and `10`.

**Required positive fixtures/properties.**

- `sound_uncertainty_001` must retain an uncertain sound-derived belief with explicit channel/source/confidence and alternatives rather than promote the sound to certain truth.
- `aged_food_record_surfaces_as_remembered_belief_not_observation_001` must distinguish a remembered/source-backed belief from a current observation.
- `embodied_workplace_availability_reflects_belief_not_truth_001` and `embodied_workplace_believed_open_truth_closed_commit_fails_001` must prove that a wrong belief can shape actor-visible availability while authoritative validation may still reject the attempted action.
- `stale_workplace_notice_superseded_by_newer_001` must prove deterministic supersession by source tick and stable event identity, with the older record retained/auditable but not treated as current.
- `partial_food_source_knowledge_001` and `seeded_food_source_unknown_to_all_actors_001` must preserve partial and absent knowledge rather than fill gaps from state.

**Required adversarial fixtures/properties.**

- External literal construction, source/scope mutation, raw projection-map access, and float confidence types must fail through the existing compile-fail corpus.
- Two actors may hold different stances/confidences for the same proposition without one record overwriting or leaking into the other.
- Changing only hidden world truth must not create, remove, strengthen, weaken, or refresh a belief without a corresponding admissible event-backed source.
- A newer but wrong belief must supersede an older belief according to the declared policy without being rewritten into truth; a tie must resolve deterministically.
- A belief with empty, dangling, future-frontier, wrong-holder, wrong-kind, or debug-only provenance must be rejected before it reaches an embodied context.

**Required evidence mechanics.** Package typed proposition/belief serializations, holder/privacy scope, stance, integer confidence, source reference and source events, acquired/current tick, freshness classification, contradiction links, supersession comparison, context-filter result, replay reconstruction, and embodied notebook rendering. The witness must show both the record and the world truth separately where they disagree.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Failure-diagnostic layers.** `content/schema validation`; `actor-known context construction`; `projection/replay`; `view-model rendering`; `tests/fixtures`.

### EPI-03 — Observation channels, capture boundaries, and event-backed insertion

**Seam definition.** An observation is a typed, holder-scoped epistemic record produced by a modeled observation channel and backed by an event/action/cause reference. Direct sight, touch/search, simple sound, and modeled absence have different information strength. The reading channel is schema-only unless the staged implementation supplies the full live path. Observation capture must append causal evidence before updating the epistemic projection; raw external insertion or world-state inference is forbidden.

**Audited files/modules.**

- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/agent/perception.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/**`
- `tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/**`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`

**Doctrine to satisfy.** `INV-001`, `INV-009` through `INV-020`, `INV-024`, `INV-026`, `INV-028`, `INV-099` through `INV-102`, and `INV-112`; foundation `03`, `04`, and `14`; architecture `02`, `03`, and `06`; execution `04`, `07`, and `09`.

**Required positive fixtures/properties.**

- `no_human_observation_facts_cite_log_events_001` must show every actor-known observation fact citing accepted log event IDs.
- `sound_uncertainty_001` must exercise `SimpleSound` with bounded confidence and alternatives.
- `expectation_contradiction_001` must exercise `TouchOrSearch`/`AbsenceMarker` through the event path before contradiction derivation.
- `view_filtering_001`, `hidden_food_closed_container_001`, and `hidden_route_edge_001` must distinguish visible/searchable facts from hidden facts.
- Current-place perception must append its event(s), apply them to the projection, record the event frontier/range, and replay to the same observation set and checksum.

**Required adversarial fixtures/properties.**

- External construction without a source and raw epistemic-record insertion must fail through the compile-fail corpus.
- A closed container, unsearched location, unseen route, or out-of-place target must not generate a sight/search/absence observation merely because the state contains the fact.
- An absence record without a completed modeled search/touch action, wrong place, wrong target, wrong actor, or source event must be rejected or omitted.
- Duplicate application, out-of-order source event, unsupported schema version, or event beyond the context frontier must fail loudly or yield a recorded replay divergence; it must not silently create a second observation.
- A `ReadingPlaceholderSchemaOnly` value must not be accepted as evidence that reading behavior exists unless the implementation supplies and certifies the complete event/provenance path and declares the staged-abstraction change.

**Required evidence mechanics.** For each channel, record observer, place, tick/window, subject/target, raw payload, bounded confidence, alternatives, schema/privacy, source reference, event envelope, append position, projection application, and replay reconstruction. Preserve an append-before-project transcript and prove that deleting or bypassing the event breaks the test.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test golden_scenarios
```

**Failure-diagnostic layers.** `content/schema validation`; `action validation`; `event application`; `projection/replay`; `actor-known context construction`; `tests/fixtures`.

### EPI-04 — Expectation contradiction, mismatch linkage, and absence-without-culprit discipline

**Seam definition.** A contradiction is a typed relation between a prior holder belief/expectation and a later admissible observation. At this baseline, the implemented active case is an expected item absent from a searched container. Detection must preserve holder, belief, observation, proposition, source, tick, and schema links; it may derive that the expected item is missing from the expected location, but it may not infer a culprit, hidden destination, or unobserved cause.

**Audited files/modules.**

- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/proposition.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/**`
- `crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs`

**Doctrine to satisfy.** `INV-021` through `INV-031`, especially fallible belief, provenance, contradiction, and no-telepathy requirements; foundation `04`; architecture `06`; execution `07` and `09`.

**Required positive fixtures/properties.**

- `expectation_contradiction_001` must create an expected-true belief, perform the modeled search, append the absence observation, derive one contradiction linking the prior belief and new observation, and surface the mismatch in the holder's notebook/view.
- Live and replay rebuilds must derive the same contradiction ID, kind, linked record IDs, expected/observed propositions, detected tick, and projection checksum.
- Re-running the same accepted event prefix must not duplicate the contradiction or derived missing-location belief.

**Required adversarial fixtures/properties.**

- No contradiction may be generated for another holder, another container/item, a non-expectation stance, a non-absence observation, an observation outside the relevant tick/window/place, or missing provenance.
- Absence must not identify a thief, mover, current hidden location, or motive; changing those hidden facts while preserving the search result must leave the actor-visible contradiction equal.
- External mutation of contradiction links must fail through the existing compile-fail fixture.
- Removing or corrupting either linked source record must produce a loud provenance/replay failure, not a free-standing contradiction.

**Required evidence mechanics.** Package the prior belief, search/action event, absence observation, contradiction, derived belief if any, source-event ancestry, live/replay equality, and an explicit “not inferred” hidden-cause matrix. If the single implemented contradiction kind is treated as a bounded staged abstraction, the acceptance artifact must say so without claiming general belief revision.

**Exact commands.**

```bash
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
```

**Failure-diagnostic layers.** `actor-known context construction`; `event application`; `projection/replay`; `view-model rendering`; `tests/fixtures`; `doctrine mismatch`.

### EPI-05 — Provenance witnesses, source-event sufficiency, freshness, and hidden-truth audit

**Seam definition.** Every actor-known fact, observation, belief, contradiction, notebook entry, and proposal-relevant assertion must carry sufficient, event-backed provenance. `SourceEventIds` must be non-empty where required, canonical, resolvable in the accepted log, not beyond the context frontier, semantically compatible with the fact, and privacy/holder compatible. Validation truth, debug omniscience, unproven physical truth, prose, and seed metadata are not actor-known provenance. Dangling, wrong-kind, mismatched, or forbidden provenance must fail closed with typed diagnostics.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs`
- `crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs`
- `crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs`
- `crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs`
- `crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs`
- `crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs`

**Doctrine to satisfy.** `INV-001`, `INV-022`, `INV-024`, `INV-026`, `INV-028`, `INV-030`, `INV-031`, `INV-099` through `INV-107`, and `INV-112`; foundation `03`, `04`, and `14`; architecture `03`, `05`, and `06`; execution `04`, `07`, and `10`.

**Required positive fixtures/properties.**

- `no_human_observation_facts_cite_log_events_001`, `workplace_assignment_provenance_001`, and `no_human_workplace_knowledge_requires_notice_event_001` must resolve every fact's source event into the exact accepted log and show semantic agreement between event kind/payload and fact.
- `no_human_known_workplace_requires_provenance_001` and `no_human_sleep_knowledge_requires_observation_or_record_001` must prove that ordinary-life consumers receive only source-backed facts, without certifying the later ordinary-life gate.
- `stale_workplace_notice_superseded_by_newer_001` must retain both ancestry and deterministic freshness/supersession classification.
- A valid proposal must retain source context ID/hash/frontier and source-event ancestry through validation, event append, and actor-visible feedback.

**Required adversarial fixtures/properties.**

- `forbidden_provenance_input_fails_closed_001`, `prose_born_fact_rejected_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` must reject semantic contamination even when no banned string appears.
- Empty, duplicate-only, nonexistent, future-frontier, wrong-event-kind, wrong-actor, wrong-subject, wrong-place, wrong-content-version, validation-only, debug-only, or unproven-physical source witnesses must be rejected with a typed blocker and no accepted proposal/event.
- Seeded knowledge must either be represented by an admissible seed/notice event and source-backed epistemic record or remain unknown; loading a content fact must not by itself create actor knowledge.
- Deleting a source event from the replay input must produce a dangling-provenance or replay failure and identify the responsible layer; it must not retain the derived fact by checksum coincidence.

**Required evidence mechanics.** Produce a machine-readable witness table with one row per derived record/fact containing record identity, holder, privacy scope, semantic kind, source event IDs, event type/schema/payload fingerprint, source tick, context frontier, freshness, derivation rule, downstream consumers, and disposition. Include `HiddenTruthAudit` results derived from structured facts/gaps, `dangling_provenance_diagnostic` output, and mutation witnesses that prove these checks are behaviorally guarded.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-content --test forbidden_content
```

**Failure-diagnostic layers.** `content/schema validation`; `actor-known context construction`; `proposal construction`; `action validation`; `event application`; `projection/replay`; `tests/fixtures`.

### EPI-06 — Epistemic projection rebuild, checksum determinism, context filtering, and non-truth-writer quarantine

**Seam definition.** `EpistemicProjection` is a replay-derived aggregate, not a source of simulation truth. Its observations, beliefs, contradictions, notebook entries, actor-known projection records, projection version, content-manifest identity, and applied-event range must be reconstructible from accepted events/declared starting evidence. Context-filtered reads must enforce holder/privacy/freshness policy. Canonical checksum equality must be stable under irrelevant insertion order and sensitive to every semantic projection field.

**Audited files/modules.**

- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/epistemics/observation.rs`
- `crates/tracewake-core/src/epistemics/belief.rs`
- `crates/tracewake-core/src/epistemics/contradiction.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/src/state.rs`
- `tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/**`
- `tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/**`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`

**Doctrine to satisfy.** `INV-009` through `INV-020`, `INV-023`, `INV-026`, `INV-028`, `INV-099` through `INV-102`, and `INV-112`; foundation `03`, `04`, and `14`; architecture `02`, `03`, and `06`; execution `04`, `07`, `09`, and `10`.

**Required positive fixtures/properties.**

- For every EPI fixture used by this spec, build the live epistemic projection through the accepted event path, rebuild a fresh projection from the serialized accepted log, and prove equality of version, manifest identity, event range/count, typed records, context-filtered reads, freshness/supersession results, notebook entries, actor-known records, and canonical checksum.
- Prove deterministic classification of current versus remembered facts and deterministic workplace supersession by tick then event ID.
- Prove the same accepted event prefix produces the same projection/checksum across at least two clean processes and after canonical serialization/deserialization.
- Demonstrate that the projection's context-scoped APIs disclose own/private and permitted public records while excluding another actor's private records.

**Required adversarial fixtures/properties.**

- The raw insertion/read compile-fail fixtures must remain negative, and a test-only bypass of event application must be caught by a behavior or mutation witness.
- Removing, duplicating, reordering, corrupting, or schema-changing a relevant event must either be rejected or produce a replay report with first divergence; it must not silently retain the original projection checksum.
- Changing authoritative hidden state without an observation/belief event must not change the focal actor's epistemic projection or embodied context.
- Mutating projection bookkeeping—event count/range, checksum serialization, record source identity, location/holder keys, freshness policy, supersession comparison, or context filter—must be killed by the configured mutation campaign.
- `from_initial_beliefs` and seed-knowledge paths must be proven to consume only explicitly admissible, source-backed starting evidence. A convenience constructor is not permission to create provenance-free knowledge.

**Required evidence mechanics.** Retain live and rebuilt projection serializations, checksums and checksum scope, event range, applied-event list/count, record-by-record diff, context-filter traces for at least two actors plus debug mode, freshness/supersession table, replay report/first divergence, and a negative control that demonstrates direct or omitted-event construction would be detected.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test negative_fixture_runner
```

**Failure-diagnostic layers.** `event application`; `projection/replay`; `actor-known context construction`; `content/schema validation`; `tests/fixtures`.

### EPI-07 — Actor decision transaction, proposal context parity, validation-truth firewall, and feedback split

**Seam definition.** The actor decision transaction must consume a sealed actor-known surface and emit a sealed proposal bound to the exact holder-known context ID/hash/frontier and typed provenance. Candidate generation, planning, and proposal construction may use actor-known input only. Authoritative truth may validate or reject the proposal, but it may not originate a proposal, backfill knowledge, rewrite the actor's reason, or leak debug-only facts through feedback. A stale or mismatched proposal must fail before accepted event append.

**Audited files/modules.**

- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/candidate.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`

The candidate/planner files are audited only at the EPI input/output boundary. Planner quality, need policy, routine coverage, and ordinary-life completeness remain `ORD-LIFE-CERT` work.

**Doctrine to satisfy.** `INV-001`, `INV-022`, `INV-024`, `INV-026`, `INV-099` through `INV-106`, and `INV-108`; foundation `05` only as boundary context plus primary foundation `14`; architecture `03`, `04`, and `05`; execution `04`, `05` as a certified dependency, and `07`.

**Required positive fixtures/properties.**

- `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, `embodied_exits_require_perceived_or_known_route_001`, and `severe_safety_without_known_exit_waits_with_knowledge_blocker_001` must show actor-known candidate/proposal behavior under missing knowledge.
- `embodied_workplace_believed_open_truth_closed_commit_fails_001` must show a belief-grounded proposal that is authoritatively rejected without changing the fact that the proposal arose from the belief.
- A successful proposal must preserve source context ID/hash/frontier, source event ancestry, semantic action, accepted validation stage, appended event, and actor-visible outcome.
- A failed knowledge case must produce a typed actor-safe blocker and a separately authorized debug diagnosis, each naming its actual responsible layer.

**Required adversarial fixtures/properties.**

- Tamper each of context ID, hash, frontier, bound actor, source event ancestry, proposal actor, semantic target, and validation generation; the pipeline must reject before accepted state mutation/event append.
- In paired worlds with identical actor-observable history but different hidden food, route, workplace, other-actor belief, or debug state, candidate set, selected intention/method at the audited boundary, sealed proposal, and actor-safe reasons must remain equal before observation.
- A validation rejection must not add the rejecting hidden fact to the actor's next holder-known context unless a separate modeled observation/feedback event legally conveys it.
- `forbidden_provenance_input_fails_closed_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` must remain negative even when the contaminated fact is wrapped in a typed structure.
- Direct construction of a decision from `PhysicalState`, `ValidationReport`, debug report, or TUI string must be absent or compile/runtime rejected.

**Required evidence mechanics.** Package the sealed actor-known input, candidate/decision trace at the EPI boundary, selected proposal, context parity tuple, validation facts, actor-safe/debug feedback split, appended events, post-action context, and paired-run equality/difference report. Replay each accepted and rejected event prefix from a clean projection and prove the same source context, proposal eligibility, validation result, and actor-safe feedback. For a rejection, record exactly what authoritative fact validated the action and prove that fact did not become actor-known without a legal channel.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Failure-diagnostic layers.** `actor-known context construction`; `candidate generation`; `planning/method selection`; `proposal construction`; `scheduler ordering`; `action validation`; `event application`; `tests/fixtures`.

### EPI-08 — Possession parity and cognition-neutral controller binding

**Seam definition.** Possession selects which actor a human controller may command; it does not replace the actor's cognition, grant player knowledge, reset intention state, disclose prior possessed actors' knowledge, change semantic action availability, or bypass normal validation/event application. A human-possessed actor and an autonomous actor in the same epistemic and physical state must receive the same embodied context and action affordances and must be judged by the same pipeline.

**Audited files/modules.**

- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-content/src/fixtures/possession_parity_001.rs`
- `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs`
- `crates/tracewake-content/src/fixtures/debug_attach_001.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-tui/tests/embodied_flow.rs`

**Doctrine to satisfy.** `INV-006`, `INV-007`, `INV-065` through `INV-070`, `INV-101`, `INV-107`, and `INV-108`; foundation `08` and `14`; architecture `10`; execution `07`.

**Required positive fixtures/properties.**

- `possession_parity_001` must run a human-commanded and autonomous path from the same actor-known context and physical state, proving equal holder-known context ID/hash/frontier, embodied view, semantic action availability, proposal validation rules, and equivalent event/state outcome for the same semantic action.
- `possession_does_not_reset_intention_001` must prove attach/detach/rebind does not reset actor intention, belief, observation, projection, or other cognition-owned state.
- `debug_attach_001` must prove controller/debug attachment is visible only in the authorized debug channel and leaves embodied cognition unchanged.
- Controller attach/detach/authorize behavior must remain event/checksum transparent except for the declared controller-binding state itself; the acceptance artifact must state the exact fingerprint scopes compared.

**Required adversarial fixtures/properties.**

- Possess actor A, then actor B: B must not receive A's notebook, beliefs, observations, context provenance, proposals, why-not details, or previously viewed debug truth.
- Attach, detach, and reattach a controller around identical actor state; actor-known context, available semantic actions, intentions, and projection checksum must remain stable.
- An unauthorized controller, mismatched actor, stale embodied view, or forged debug attachment must be rejected without creating actor knowledge or accepted events.
- The human path may select among actor-available actions but must not submit a semantic action absent from the autonomous actor's same-state availability set without ordinary proposal rejection.
- Vary hidden truth while holding the actor-observable history fixed in both human and autonomous runs; parity must hold pairwise before observation.

**Required evidence mechanics.** Record before/after controller bindings, agent/cognition fingerprints, epistemic projection checksum, context parity tuple, embodied view/action set, selected semantic action, proposal, validation, event log segment, and resulting state. Rebuild each autonomous and human-run projection/context from its serialized event input and prove the same-state parity relation survives replay. Explain any expected difference by fingerprint scope. Prove no prior-actor source appears in the newly possessed actor's provenance table.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test command_loop_session
```

**Failure-diagnostic layers.** `actor-known context construction`; `proposal construction`; `action validation`; `view-model rendering`; `debug quarantine`; `tests/fixtures`.

### EPI-09 — Embodied projection source, notebook, action availability, why-not, and stale-snapshot behavior

**Seam definition.** The embodied view model must be built from a sealed holder-known context and a bounded embodied projection/snapshot, never from an unrestricted live `PhysicalState` handle. It must carry context ID/hash/frontier and actor-safe source summaries; render only permitted entities, notebook entries, action availability, and why-not information; preserve intentional staleness until an observation updates the actor; and keep validation/debug detail outside the embodied channel.

**Audited files/modules.**

- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/transcript.rs`
- `tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/**`
- `crates/tracewake-tui/tests/embodied_flow.rs`
- `crates/tracewake-tui/tests/transcript_snapshot.rs`
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`

**Doctrine to satisfy.** `INV-023` through `INV-031`, `INV-065` through `INV-070`, `INV-099` through `INV-108`, and `INV-112`; foundation `04`, `08`, and `14`; architecture `03`, `06`, and `10`; execution `04` and `07`.

**Required positive fixtures/properties.**

- `embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unknown_sleep_affordance_001`, `embodied_view_omits_unobserved_food_at_open_place_001`, and `embodied_exits_require_perceived_or_known_route_001` must omit unknown truth.
- `embodied_menu_lags_truth_change_without_perception_001` must prove that a truth change alone does not refresh the menu; a later admissible observation may update it.
- `embodied_workplace_availability_reflects_belief_not_truth_001`, `view_filtering_001`, and `view_model_local_actions_001` must prove actor-safe availability and local filtering from the sealed context.
- Notebook entries must show source-backed observations, beliefs, contradictions, freshness/staleness, and actor-appropriate “how this could be wrong/what can be checked next” without exposing hidden validation facts.
- `ActionAvailability` and `WhyNotView` must preserve the split between actor-safe explanation/provenance and debug-only diagnostics.

**Required adversarial fixtures/properties.**

- Hidden item, closed-container contents, hidden route, raw workplace assignment, unknown sleep affordance, other actor's private belief, debug report, and validator-only fact must not appear in embodied structs, serialization, rendering, transcript, action labels, why-not text, ordering, counts, or timing-sensitive differences.
- A stale view-model/context pair or mismatched context ID/hash/frontier must not be accepted as a fresh proposal source.
- Creating or reading a privileged debug projection view outside core's authorized API must fail via the compile-fail fixture.
- Toggling debug availability or constructing debug data must not alter the embodied model's semantic serialization or render.
- Actor-visible error text must remain truthful at the actor's epistemic level; it may say an action failed without naming an unobserved hidden cause.

**Required evidence mechanics.** Retain the sealed context, embodied projection source/snapshot, semantic view-model serialization, actor-safe source/provenance table, notebook entries, action/why-not table, rendered transcript, and paired hidden-truth/debug comparisons. Prove no live state handle escapes into the embodied model and that replay rebuilt inputs reproduce the same model.

**Exact commands.**

```bash
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Failure-diagnostic layers.** `actor-known context construction`; `projection/replay`; `view-model rendering`; `proposal construction`; `debug quarantine`; `tests/fixtures`.

### EPI-10 — Debug capability isolation, report/view separation, TUI quarantine, and no feedback path

**Seam definition.** Debug access is a privileged, non-diegetic observer channel. Core alone may mint the unforgeable debug capability; debug contexts, reports, and views require that capability or another equally strict core-owned authority path. Debug surfaces must carry a visible non-diegetic marker and remain structurally separate from embodied models. Rendering or using debug data must not create simulation facts, proposals, events, actor feedback, beliefs, observations, or projection records.

**Audited files/modules.**

- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/app.rs`
- `tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/**`
- `tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/**`
- `tests/negative-fixtures/external_crate_cannot_construct_debug_report/**`
- `crates/tracewake-tui/tests/adversarial_gates.rs`
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`

**Doctrine to satisfy.** `INV-031`, `INV-065` through `INV-070`, `INV-099`, `INV-100`, `INV-107`, and `INV-108`; foundation `08` and `14`; architecture `01` and `10`; execution `04`, `07`, and `10`.

**Required positive fixtures/properties.**

- Authorized debug construction must expose the intended debug report/view, return true from the available `debug_only()` contract, and render the `DEBUG NON-DIEGETIC` marker through the debug panel/channel.
- `debug_attach_001` and `debug_omniscience_excluded_001` must show that attaching debug allows an operator-only diagnostic view while embodied actor output and cognition remain unchanged.
- TUI embodied rendering must suppress the debug-only token set and diagnostics; the separately invoked debug overlay may show them.
- The crate dependency graph must preserve core-owned capability/report types with no reverse dependency or TUI-owned minting path.

**Required adversarial fixtures/properties.**

- The compile-fail doctests in `debug_capability.rs` plus the external negative corpus must prove that an external crate cannot mint/construct the debug capability, construct a debug holder-known context, construct a debug report/view, or directly read privileged projection maps.
- Inject a distinctive hidden debug value/token into every debug report family; assert it appears in the authorized debug output and nowhere in embodied view structs, serialization, transcript, action labels, why-not text, feedback, event payloads, beliefs, observations, or actor-known context hash.
- Enable/disable/open/close/render debug panels around the same run; authoritative state checksum, event log, epistemic projection checksum, actor-known context, selected proposal, and embodied output must remain equal.
- Pasting or routing debug prose into a command/LLM/content surface must not create typed fact; `prose_born_fact_rejected_001` remains the negative witness.
- A debug validation reason must not become the next actor-safe reason unless an independent modeled event conveys that information.

**Required evidence mechanics.** Package capability construction boundary evidence, compile-fail transcripts, debug report/view identities, non-diegetic marker, distinct-channel render snapshots, debug-token absence scan over embodied artifacts, before/after state/event/projection/context checksums, proposal equality, and dependency-graph/source guard evidence. Replay the debug-disabled and debug-enabled event inputs independently and prove each rebuild yields the same authoritative/epistemic result and actor-visible relation. A UI color or label alone is not quarantine proof.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Failure-diagnostic layers.** `debug quarantine`; `view-model rendering`; `actor-known context construction`; `proposal construction`; `event application`; `tests/fixtures`; `doctrine mismatch`.

### EPI-11 — Relational anti-contamination and possession-parity capstone

**Seam definition.** The capstone proves the EPI boundary as a relational property, not merely a collection of single-run examples. Two executions that are equal in the focal holder's admissible observation/event history but differ in hidden world truth, another holder's private state, debug state, or controller source must remain equal at every actor-visible and decision-producing EPI surface until a modeled observation makes the difference knowable. After that observation, any divergence must be explainable by the new event/provenance chain and reproducible under replay.

**Audited files/modules.** All files named in `EPI-01` through `EPI-10`, with the capstone harness committed under the existing certification test surfaces rather than a new phase gate. At minimum, the harness must execute through:

- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`
- `crates/tracewake-tui/tests/embodied_flow.rs`

**Doctrine to satisfy.** The complete upstream EPI doctrine, especially `INV-001`, `INV-022` through `INV-031`, `INV-065` through `INV-070`, and `INV-099` through `INV-108`, plus the event/replay/evidence-honesty requirements.

**Required positive and adversarial paired-run matrix.** Each pair starts from the same focal actor-observable history and differs only in the named hidden variable before the reveal:

| Pair | Hidden difference | Required equality before reveal | Legal divergence trigger |
|---|---|---|---|
| Closed container | item present vs absent inside unopened/unsearched container | actor projection/context, notebook, actions, proposal, embodied render | accepted open/search/touch plus observation event |
| Hidden route | route edge exists vs absent but has not been perceived/recorded | known exits, movement availability, decision trace, proposal | sight/notice/search event that conveys route knowledge |
| Workplace truth | open vs closed while actor holds the same belief/notice | belief, availability, proposal, actor-safe rationale | validation may reject; belief changes only after legal feedback/observation |
| Other actor privacy | actor B's private belief/observation differs | actor A context, notebook, decisions, views | explicit modeled communication/record path, if implemented and in scope |
| Debug state | debug detached vs attached with omniscient report | all simulation, actor-known, proposal, and embodied artifacts | no diegetic divergence is legal from debug alone |
| Possession source | autonomous vs human controller, same actor state and semantic choice | context, available actions, validation rules, event/state result | controller-binding/debug metadata only, outside actor cognition |
| Stale truth | hidden truth changes after snapshot but before perception | existing embodied model/menu and actor decision input | later admissible observation and rebuilt context |

**Metamorphic/property requirements.**

- Generate multiple valid event prefixes and hidden perturbations with recorded seeds; retain every failing seed and its minimized counterexample. Sampling must be labeled as sampled, not exhaustive.
- Permute semantically unordered source/provenance inputs; canonical packet/projection/view outputs must remain equal.
- Change one semantic provenance/context field at a time; the relevant hash or validation result must change.
- Remove or substitute a source event; the derived fact must disappear or fail closed with the correct layer.
- Replay both members of every pair; each member must reproduce itself, and the pre-reveal actor-visible relation must hold after rebuild.
- Mutation operators that remove privacy filters, allow forbidden sources, weaken context parity, skip source checks, invert freshness, bypass debug capability, or render debug tokens into embodied output must be caught by a named capstone witness.

**Required evidence mechanics.** Produce a paired-run ledger with input-difference declaration, focal actor-observable event prefixes, event/projection/context/view/proposal fingerprints, equality domains, excluded fingerprint domains, reveal event, post-reveal divergence ancestry, replay confirmation, seed/sample scope, and first divergence. A whole-world checksum is not the equality oracle because the worlds intentionally differ; compare the actor-visible/decision-producing surfaces named above.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test embodied_flow
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Failure-diagnostic layers.** The first divergence must name one of the live execution layers: `doctrine mismatch`; `content/schema validation`; `actor-known context construction`; `candidate generation`; `planning/method selection`; `proposal construction`; `scheduler ordering`; `action validation`; `event application`; `projection/replay`; `view-model rendering`; `debug quarantine`; `tests/fixtures`; or `documentation status`.

## 6. Required fixture and adversarial-proof matrix

The registered golden corpus must run exhaustively through `crates/tracewake-content/tests/golden_fixtures_run.rs`. The table below identifies the minimum EPI families; it does not authorize omitting other registered fixtures from the full runner.

| EPI family | Required positive witnesses | Required adversarial witnesses | Certifying observation |
|---|---|---|---|
| Context construction and privacy | `view_filtering_001`, `no_human_epistemic_check_001`, `workplace_assignment_provenance_001` | other-actor private record; hidden truth perturbation; stale/forged context tuple; context compile-fail fixtures | canonical sealed packet, context ID/hash/frontier, scope decision, proposal parity |
| Belief, uncertainty, staleness | `sound_uncertainty_001`, `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, `partial_food_source_knowledge_001`, `stale_workplace_notice_superseded_by_newer_001` | float confidence; literal construction; source/scope mutation; hidden truth changes without belief event | typed belief/proposition, holder/privacy, source, confidence, freshness and replay |
| Observation channels | `no_human_observation_facts_cite_log_events_001`, `sound_uncertainty_001`, current-place perception cases | `hidden_food_closed_container_001`, `hidden_route_edge_001`, observation-without-source and raw-insert compile failures | channel-specific event append, observation source, projection application, replay equality |
| Contradiction | `expectation_contradiction_001` | wrong holder/item/container/stance/channel; missing linked record; hidden culprit/location changes | belief + search/absence event + observation + contradiction linkage; no culprit inference |
| Provenance and truth firewall | `workplace_assignment_provenance_001`, `no_human_known_workplace_requires_provenance_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `no_human_sleep_knowledge_requires_observation_or_record_001` | `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, `prose_born_fact_rejected_001` | source-event witness resolves, matches kind/holder/frontier, and survives replay |
| Hidden-truth planning boundary | `no_hidden_truth_planning_001`, `knowledge_blocker_accuse_001`, `severe_safety_without_known_exit_waits_with_knowledge_blocker_001` | `hidden_food_unknown_route_001`, `no_human_unseen_workplace_assignment_does_not_plan_work_001`, typed hidden input | equal decision/proposal before legal observation; typed blocker, no accepted event on failure |
| Validation truth versus belief | `embodied_workplace_availability_reflects_belief_not_truth_001` | `embodied_workplace_believed_open_truth_closed_commit_fails_001` | actor proposal remains belief-grounded; validator may reject without teaching hidden truth |
| Possession parity | `possession_parity_001`, `possession_does_not_reset_intention_001` | prior-actor knowledge carryover; unauthorized controller; human-only hidden affordance | equal context/view/action rules and equivalent semantic outcome; cognition unchanged on bind |
| Embodied view and stale snapshot | `view_model_local_actions_001`, `embodied_exits_require_perceived_or_known_route_001`, `embodied_menu_lags_truth_change_without_perception_001` | `embodied_view_omits_raw_assignment_without_context_001`, `embodied_view_omits_unknown_sleep_affordance_001`, `embodied_view_omits_unobserved_food_at_open_place_001` | no hidden field/token; context-bound model; update only after legal observation |
| Debug quarantine | `debug_attach_001` plus authorized debug-panel/render tests | `debug_omniscience_excluded_001`, debug capability/report/context/projection compile-fail fixtures, debug-token injection | debug marker present only in debug channel; no state/event/projection/context/proposal change |
| Seed knowledge | an event-backed starting notice/record where the corpus supplies one | `seeded_food_source_unknown_to_all_actors_001`; seed fact with no knowledge event | authored truth and actor knowledge remain separate; provenance is explicit |
| Replay/determinism | every EPI fixture above, rebuilt from serialized event input | omitted/duplicated/reordered/corrupted/unsupported event; changed manifest/schema | live/replay projection, context, view, and proposal fingerprints agree or fail loudly |
| Relational capstone | paired runs from section `EPI-11` | each hidden-variable pair plus source removal and debug toggle | equality over actor-visible/decision surfaces before reveal; event-explained divergence after reveal |

### 6.1 Compile-fail boundary corpus

The implementing session must record the full `negative_fixture_runner` output and a case-by-case matrix for at least:

- `banned_float_confidence_types`
- `external_crate_cannot_build_debug_knowledge_context`
- `external_crate_cannot_build_debug_projection_view_without_core_debug_api`
- `external_crate_cannot_construct_belief_literal`
- `external_crate_cannot_construct_debug_report`
- `external_crate_cannot_construct_observation_without_source`
- the `debug_capability.rs` compile-fail doctests for literal construction and `DebugCapability::mint()`
- `external_crate_cannot_insert_raw_epistemic_records`
- `external_crate_cannot_mutate_belief_source_or_scope`
- `external_crate_cannot_mutate_contradiction_links`
- `external_crate_cannot_mutate_knowledge_context_mode`
- `external_crate_cannot_mutate_knowledge_context_viewer`
- `external_crate_cannot_read_raw_epistemic_projection_maps`

Each row must state the forbidden capability, compiler failure class, relevant EPI point, and why the failure closes an external API path rather than merely matching an error string.

### 6.2 Generated and metamorphic evidence

Generated evidence supplements, but does not replace, the named matrix. The implementing session must record generator version, deterministic seeds, population size, shrink/minimization result, omitted domains, and all retained counterexamples. At minimum it must vary actor IDs, source-event order, event frontier, confidence bounds, privacy holder, observation channel, expected/observed proposition pair, hidden-truth perturbation, possession source, and debug state. Every generated claim is labeled `sampled` unless its finite input domain is genuinely exhausted and the artifact demonstrates that exhaustion.

## 7. Mutation-testing posture

Mutation testing is a certifying guarded-layer requirement for EPI-CERT, not an optional quality score. The 0037/0039 artifacts establish the campaign discipline, but their runs are historical predecessor evidence; they do not certify the EPI-specific population at the implementation commit for this spec.

### 7.1 Target-baseline configuration finding

At target commit `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`, `.cargo/mutants.toml` is a checked-in workspace-wide standing perimeter using `additional_cargo_args = ["--locked"]`, `test_workspace = true`, and cargo-mutants `27.1.0` as the CI syntax baseline. It already includes the complete `agent/**` directory and the major proposal/pipeline/event/replay/projection/view/controller/debug/TUI seams. However, its epistemic entries name only `epistemics/knowledge_context.rs` and `epistemics/projection.rs`; `belief.rs`, `observation.rs`, `proposition.rs`, `contradiction.rs`, `knowledge_basis.rs`, and `epistemics/mod.rs` are not inside that explicit standing EPI path. That omission is a mutation-coverage risk, not a static verdict.

### 7.2 Required configured union

Before a passing EPI mutation run, the final checked-in configuration must make the following EPI union visible in `cargo mutants --workspace --list-files`:

```text
crates/tracewake-core/src/epistemics/**
crates/tracewake-core/src/agent/**
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/events/**
crates/tracewake-core/src/replay/**
crates/tracewake-core/src/checksum.rs
crates/tracewake-core/src/state.rs
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/src/controller.rs
crates/tracewake-core/src/debug_capability.rs
crates/tracewake-core/src/debug_reports.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/src/debug_panels.rs
crates/tracewake-tui/src/render.rs
crates/tracewake-tui/src/transcript.rs
```

If `input.rs`, `run.rs`, or another manifest-listed source is shown by the code audit to own a debug/possession capability or embodied-data transformation, it must be added to the configured union and the reason recorded. The implementing session must not narrow a directory glob merely to reduce runtime. Existing non-EPI standing paths may remain and be run as continuity coverage; this spec does not authorize removing them.

The final scheduled/certifying command must use one checked-in configuration. An exploratory `--no-config` or `-f` run may help enumerate a delta, but it cannot be the final pass if CI and the standing config remain narrower.

### 7.3 Required waves and commands

**Wave A — standing continuity census.** On the unmodified target-baseline configuration, retain the existing file and mutant census and the complete outcome. This detects regression relative to the predecessor perimeter but cannot by itself pass EPI-CERT.

**Wave B — EPI expansion census.** Add the omitted EPI files to the checked-in configuration, prove the expanded `--list-files` union, and run the completed population. The final Wave B/union run is the certifying mutation evidence.

```bash
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --version
cargo mutants --workspace --list-files
cargo mutants --workspace --list
cargo mutants --workspace --no-shuffle
```

If the repository intentionally upgrades cargo-mutants, the artifact must record the old and new versions, configuration syntax change, before/after file census, before/after mutant census, and why denominator changes arise. Tool-version drift may not be mistaken for production coverage.

Sharded equivalents are allowed only when every shard uses the same exact commit, worktree, Rust/Cargo/cargo-mutants versions, `Cargo.lock`, checked-in config, test-workspace posture, timeout policy, and denominator; every shard completes; the union is complete and non-overlapping; and all outputs are retained. The final campaign must not use `--iterate` or skip the unmutated baseline.

### 7.4 Outcome requirements

The artifact must report separately:

- files in the configured census and required-file omissions, if any;
- enumerated and tested mutants;
- caught, missed, timeout, unviable, and tool/run failure counts;
- shard count and completeness;
- unresolved timeout retries and their exact commands;
- changes in denominator from source/config/tool version;
- baseline-miss-file use, if any; and
- survivor review dispositions.

A timeout is not a pass. A missing output directory/file is not a pass. A cargo-mutants tool error is not a pass. A high mutation score is not a pass. A survivor may not be hidden in `.cargo/mutants-baseline-misses.txt`, excluded by a new glob, or dismissed because a line moved.

### 7.5 Survivor triage register

The implementing session must create:

```text
reports/0040_epi_cert_mutation_triage_register.md
```

Reuse the `reports/0039_spine_cert_mutation_triage_register.md` shape. Every final survivor or timeout row must contain:

| Field | Required content |
|---|---|
| Historical identity | predecessor identity if relevant; otherwise `new in EPI-CERT run` |
| Final identity | path, enclosing symbol/function, operator/diff, structured-output reference |
| Tool outcome | `caught`, `missed`, `timeout`, `unviable`, or tool/run failure |
| Responsible EPI cross-reference | `EPI-01` through `EPI-11`; local cross-reference only |
| Responsible failing layer | one existing execution `03` layer |
| Certified reachability | invariant/gate consequence and exact production consumers |
| Test/property family | named fixture, paired run, property, metamorphic case, or mutation witness |
| Behavior witness | subsystem consequence, not a getter/literal tautology |
| Replay/provenance ancestry | events, context ID/hash/frontier, projection/replay package, schema/manifest, or justified `not applicable` |
| Negative/contamination control | forbidden shortcut that remains rejected/omitted |
| Certification disposition | killed, equivalent, non-critical, or blocked/untriaged; register disposition only |
| Evidence references | baseline/mutant transcripts, diff, failing witness, output artifact |
| Review signoff | implementer plus accepted independent review evidence |

Tool outcome and review disposition remain separate. A `missed` mutant does not become `caught` because it is argued equivalent. Behavior-relevant survivors default to kill-with-behavior/provenance coverage: the witness must pass unmutated, fail under that mutant, execute the production seam, observe a certified consequence, and carry the required replay/provenance and contamination control.

Equivalent/non-critical disposition is rare. It requires the exact diff, reachable call sites and input domain, a semantic argument that no reachable input distinguishes the mutant or that it lies outside every EPI obligation, and independent signoff. “No test failed,” compiler optimization, a hard-coded getter assertion, or a snapshot of prose is insufficient.

### 7.6 Mutation verdict routing

`EPI-CERT passed` requires a completed configured-union census with zero blocked/untriaged survivors, zero unresolved timeouts/tool failures, and behavior evidence for every required EPI seam. If any behavior-relevant survivor floor remains, the acceptance artifact must render `EPI-CERT scoped remediation`, name the responsible layer and register rows, and route the work to a **later, separately numbered EPI-CERT mutation-remediation and replacement-certification spec**. This document does not author that later spec and does not prescribe its production fix.

## 8. Failure handling and diagnostics

A failure must use one of the live execution-layer names:

- doctrine mismatch;
- content/schema validation;
- actor-known context construction;
- candidate generation;
- planning/method selection;
- proposal construction;
- scheduler ordering;
- action validation;
- event application;
- projection/replay;
- view-model rendering;
- debug quarantine;
- tests/fixtures; or
- documentation status.

A failed gate may not be relabeled as a phase exception. A failure in a predecessor SPINE seam discovered while executing this EPI audit is still reported at its actual layer and blocks any EPI pass that depends on it; the audit does not silently re-certify or waive SPINE-CERT.

### 8.1 Per-audit-point primary diagnostic map

| Audit point | Primary layers | Required first-divergence evidence |
|---|---|---|
| `EPI-01` | actor-known context construction; proposal construction; projection/replay | differing/missing packet field, hash/frontier mismatch, source/filter decision |
| `EPI-02` | content/schema validation; actor-known context construction; projection/replay; view-model rendering | typed belief/proposition/privacy/freshness diff and source record |
| `EPI-03` | action validation; event application; projection/replay | channel/action/source event, append/application position, replay diff |
| `EPI-04` | event application; projection/replay; view-model rendering | prior belief, absence observation, link mismatch, forbidden inference |
| `EPI-05` | actor-known context construction; proposal construction; action validation; projection/replay | unresolved/wrong-kind witness, frontier, semantic mismatch, typed blocker |
| `EPI-06` | event application; projection/replay | live/rebuild record/checksum first divergence and omitted/direct write path |
| `EPI-07` | candidate generation; planning/method selection; proposal construction; action validation | actor-known input, decision trace, proposal tuple, validator/feedback split |
| `EPI-08` | actor-known context construction; view-model rendering; proposal construction; action validation | controller-binding delta versus cognition/context/view/event equality domain |
| `EPI-09` | projection/replay; view-model rendering; debug quarantine | leaked/omitted field, stale-context mismatch, actor-safe/debug provenance split |
| `EPI-10` | debug quarantine; view-model rendering; event application | capability path, leaked token/channel, state/event/projection/context delta |
| `EPI-11` | first responsible layer encountered | paired-run equality domain and exact first actor-visible divergence |
| Mutation | layer owning the mutant's certified consequence | mutant identity/diff, baseline pass, mutant failure or reviewed exception |

### 8.2 Failure package

For every failed row, retain:

1. exact implementation commit and dirty-worktree statement;
2. exact command and unabridged failure transcript;
3. fixture/seed and minimized counterexample where applicable;
4. expected versus actual behavior at the certified boundary;
5. event/provenance/context/replay/view evidence sufficient to locate the first divergence;
6. responsible layer and affected EPI cross-references;
7. whether the result is deterministic, flaky, timeout, tool failure, missing evidence, or doctrine mismatch; and
8. the route to a separately scoped remediation spec/report.

The failure package may recommend an investigation surface, but this audit spec does not pre-author a fix or accept a narrower replacement gate.

## 9. Acceptance-artifact contract

This spec specifies the audit. The implementing session renders the verdict in a separate acceptance artifact only after executing the complete contract.

### 9.1 Artifact identity and scope

Expected staging and closeout paths:

```text
reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md
archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md
```

The artifact must state:

- this spec's exact path and number;
- target/source baseline `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`;
- exact final implementation commit actually tested and whether the worktree was clean;
- every evidence/test/configuration delta from the target baseline;
- the scope-limited predecessor facts consumed from the 0037 and 0039 artifacts;
- Rust, Cargo, cargo-mutants, OS/runner, and relevant environment versions;
- `Cargo.lock`, `.cargo/mutants.toml`, CI workflow, source perimeter, fixture registry, and content/ruleset fingerprints with stated scopes;
- changed-file list and command-transcript index; and
- the limitation that neither this workflow nor the verdict independently verifies current `main`.

### 9.2 Evidence-item ledger fields

Every evidence item cited by a seam row must instantiate the fields from `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`:

- **Evidence item ID:** stable local identifier used by the report.
- **EPI cross-references:** one or more of `EPI-01` through `EPI-11`, mutation, or artifact completeness.
- **Evidence status:** `pass`, `fail`, `pending`, `sampled`, `observer-only`, or `historical` as already allowed by the template; this spec creates no new evidence status.
- **Fingerprint scope:** raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, replay artifact, or justified not applicable.
- **Evidence summary:** command, artifact path, report section, file/symbol, fixture, paired run, or replay reference.
- **Path under test and behavior witness:** production entry, event/trigger/emitter, responsible layer, accepted/rejected stage, negative or mutation-style control, and checked semantic consequence.
- **Replay/provenance ancestry:** event-log segment/IDs, replay artifact, seed/randomness/content/ruleset versions, projection/schema version, context ID/hash/frontier, source provenance, and downstream consumer.
- **Sampling/exhaustiveness:** exhaustive finite perimeter, exhaustive fixture registry, mutation census, finite named matrix, or sampled population with basis and omissions.
- **Pending or historical handling:** missing proof/owner/blocker for pending rows; context/lineage/precedent role for historical rows.
- **Certification use:** counted as certifying pass, counted as certifying fail, or not counted as certifying evidence.
- **Staged-abstraction declaration:** what is proven, abstracted, forbidden to fake, future route, anti-overclaim evidence, and diagnostic split where an abstraction is relied on.

Pending, sampled, observer-only, and historical evidence may inform review but may not silently make a seam pass. A fingerprint may not be cited outside its actual scope.

### 9.3 Command and environment ledger

The artifact must include a row for every command in section 4 and every mutation command actually executed, with:

| Command | Exact commit/worktree | Result | Transcript/artifact | Evidence status | Notes |
|---|---|---|---|---|---|
| `<exact command>` | `<sha; clean/dirty>` | `<exit/status>` | `<path and fingerprint>` | `<status>` | `<retry, shard, filter, omission, or none>` |

Filtered diagnostic reruns are additional rows; they do not replace complete runs. Retries must preserve failed attempts and explain why the final result is trustworthy.

### 9.4 Per-seam verdict table

The artifact must include at least:

| Requirement | Responsible layer(s) | Positive evidence | Adversarial/negative evidence | Replay/provenance evidence | Mutation evidence | Result from certifying evidence |
|---|---|---|---|---|---|---|
| `EPI-01` sealed context identity/scope/hash/frontier | actor-known context construction; proposal construction | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-02` beliefs/privacy/freshness | content/schema validation; projection/replay | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-03` observation channels/event capture | event application; projection/replay | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-04` contradiction/absence discipline | projection/replay; view-model rendering | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-05` provenance/witness sufficiency | actor-known context construction; proposal/action validation | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-06` projection rebuild/non-writer | event application; projection/replay | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-07` decision/proposal parity/truth firewall | candidate/planning/proposal/action validation | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-08` possession parity | actor-known context; view-model; proposal/action validation | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-09` embodied view/notebook/why-not | projection/replay; view-model rendering | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-10` debug quarantine | debug quarantine; view-model rendering | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| `EPI-11` relational capstone | first responsible layer | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |
| Configured EPI mutation perimeter | layer by survivor | `<census/run IDs>` | `<survivor controls>` | `<where applicable>` | `<register/run IDs>` | `<pass/fail/pending>` |
| Artifact/evidence honesty | documentation status; tests/fixtures | `<IDs>` | `<IDs>` | `<IDs>` | `<IDs>` | `<pass/fail/pending>` |

A row passes only from certifying evidence at the exact final implementation commit. Static review, source presence, historical tickets, predecessor mutation output, sampled/generative evidence alone, or observer-only artifacts cannot make a row pass.

### 9.5 Replay and provenance package

The package must retain or link:

- serialized accepted event inputs for every named EPI fixture and paired-run member;
- event envelope/index/fingerprint tables and source-event witness tables;
- live and replay authoritative state checksums with exact scope where relevant;
- live and replay epistemic projection serializations/checksums and record diffs;
- sealed holder-known packet serializations, IDs, hashes, frontiers, schema/projection/content identities, and scope decisions;
- proposal source context tuples, validation results, accepted/rejected events, and actor-safe/debug feedback split;
- embodied view-model and transcript artifacts plus separate authorized debug artifacts;
- contradiction and freshness/supersession matrices;
- relational pair declarations, equality domains, reveal events, and post-reveal ancestry; and
- first-divergence reports for every failure.

A package may use indexed archives for size, but every index/fingerprint must name whether it covers raw bytes, normalized serialization, parsed semantic content, a command transcript, a run seed, or a replay artifact.

### 9.6 Mutation package

The artifact must retain:

- cargo-mutants/Rust/Cargo versions;
- final `.cargo/mutants.toml` and CI fingerprints;
- `--list-files` and `--list` outputs;
- complete unsharded output or all shard outputs plus completeness proof;
- baseline command output;
- caught/missed/timeout/unviable/tool-failure counts;
- timeout retry commands/output;
- the complete `reports/0040_epi_cert_mutation_triage_register.md`;
- proof that no required EPI file was silently excluded;
- proof that `test_workspace = true` or an equivalent workspace-wide witness posture was active;
- statement that the final run did not use `--iterate`; and
- statement and evidence showing that baseline-miss machinery did not launder an actionable EPI survivor.

### 9.7 Relational capstone package

For every section `EPI-11` pair, include:

- exact hidden input difference;
- equality precondition over focal actor-observable event history;
- exact actor-visible/decision-producing fields compared;
- comparison result before reveal;
- legal reveal event and its provenance;
- comparison after reveal;
- replay confirmation for both runs;
- generated seed/sample declaration where applicable; and
- first responsible layer if the relation fails.

The artifact must not compare only whole-world checksums or only final rendered strings. It must compare the typed actor-known, proposal, and embodied semantic surfaces and use rendered output as an additional channel check.

### 9.8 EMERGE-OBS handling

If the verified corpus exercises first-proof living-world acceptance, the package may include the existing observer-only `EMERGE-OBS` ledger/artifact. It must be labeled `observer-only`, may not alter any EPI row, may not become a phase gate or pass/fail threshold, and may not be used as a scheduler objective or quality substitute.

### 9.9 Aggregate verdict rule

The artifact may render:

```text
EPI-CERT passed
```

only when all of the following are true:

1. `EPI-01` through `EPI-11` pass from certifying evidence at the exact final implementation commit;
2. every required positive and adversarial fixture family is present and the registered golden corpus ran exhaustively;
3. live/replay/projection/context/proposal/view evidence is complete and internally consistent;
4. provenance witnesses resolve and all required negative/compile-fail boundaries hold;
5. possession parity and debug non-interference pass relationally;
6. the final configured EPI mutation perimeter completed with no blocked/untriaged behavior-relevant survivors and no unresolved timeout/tool failure;
7. pending, sampled, observer-only, historical, and static evidence were not counted as certifying passes;
8. staged abstractions are explicit and hide no EPI obligation; and
9. evidence and fingerprint scopes are honest and reproducible.

Otherwise the artifact must render:

```text
EPI-CERT scoped remediation
```

and name the failed EPI rows, responsible layers, exact evidence gaps, and survivor-register entries. An actionable mutation survivor floor routes to a later separate EPI-CERT mutation-remediation and replacement-certification spec. No partial aggregate pass, phase exception, or feature-expansion waiver is allowed.

## 10. Preliminary static survey — not certification

This section is informative only. It does not satisfy a seam, run a test, prove replay, or render a verdict.

Exact-commit static reading suggests substantial EPI scaffolding is present:

- `epistemics/knowledge_context.rs` has private sealed context state, `Embodied`/`Debug` modes, explicit admitted and forbidden knowledge sources, actor-private/debug scope filters, context ID/hash/frontier, provenance entries, actor-known fact families, and a forbidden-truth audit.
- `belief.rs`, `observation.rs`, `proposition.rs`, and `contradiction.rs` use typed records with private fields, explicit holder/privacy/source/schema data, bounded integer confidence, and linked contradiction records.
- `epistemics/projection.rs` stores private observation/belief/contradiction/notebook/actor-known maps, tracks applied event range and content identity, exposes context-filtered reads, classifies freshness/supersession, and computes a canonical checksum.
- `agent/actor_known.rs` requires checked, non-empty source event IDs for actor-known facts and distinguishes admissible actor-known provenance from validation/debug/unproven physical truth.
- `agent/no_human_surface.rs`, `agent/transaction.rs`, `actions/proposal.rs`, and `actions/pipeline.rs` provide recognizable seams for sealed input, typed provenance diagnostics, source-context parity, validation, event append, and projection application.
- `projections.rs`, `view_models.rs`, and `controller.rs` provide embodied snapshot/view-model and possession/controller boundaries.
- `debug_capability.rs`, `debug_reports.rs`, `view_models.rs`, `tracewake-tui/src/debug_panels.rs`, and `tracewake-tui/src/render.rs` provide capability, `debug_only()`, `DEBUG NON-DIEGETIC`, and embodied-render suppression structures.
- The target tree contains numerous adversarial golden fixtures and compile-fail boundary fixtures directly relevant to all gate-3 themes.

Static risk flags that the implementing session must resolve with live evidence:

- The standing mutation configuration explicitly includes only two files under `epistemics/`; belief, observation, proposition, contradiction, knowledge-basis, and module glue require inclusion in the final EPI configured perimeter.
- API privacy and type shape do not prove that every live producer goes through accepted events or that every source witness semantically matches its event. The event/replay/provenance package must prove this dynamically.
- `EpistemicProjection::from_initial_beliefs` and authored seed paths require special scrutiny so convenience initialization does not become provenance-free actor knowledge.
- `ReadingPlaceholderSchemaOnly` is an explicit staged breadth boundary, not evidence of a completed reading channel.
- The implemented contradiction surface is currently centered on expected-item absence; that may satisfy the audited present behavior if proven honestly, but it cannot be overclaimed as a general belief-revision engine.
- Debug labels/token filtering are useful defense in depth, but only the paired no-feedback/no-state-change proof closes the debug non-interference claim.
- Existing happy-path fixtures cannot by themselves prove the relational statement that hidden truth changes are unobservable before a legal observation; section `EPI-11` requires direct paired-run evidence.

None of these observations is a pass or failure determination.

## 11. Tolerated deferrals and explicit out-of-scope boundaries

EPI-CERT is comprehensive for gate 3 but must not absorb later-gate substance:

- **SPINE-CERT seams:** event log, replay, general projection infrastructure, randomness/save, action-pipeline ordering, no-direct-dispatch, and the broad TUI/debug split are predecessor-certified via the 0039 artifact. This spec consumes and regression-checks only the parts needed to prove EPI behavior; it does not re-audit or replace SPINE-CERT.
- **ORD-LIFE-CERT:** need/routine/intention policy quality, planner completeness, ordinary-life economy, workplace/household realism, method coverage, no-human-day richness, and long-horizon stability are deferred except where a fixture is used to prove the EPI input/firewall boundary.
- **FIRST-PROOF-CERT:** coherent playable proof, missing-property village acceptance, narrative sufficiency, living-world quality, and emergent-story evaluation are deferred. `EMERGE-OBS` remains observer-only.
- **Data authoring pipeline:** schema/compiler/editor breadth and general content validation are outside EPI-CERT except where authored seed knowledge crosses into an epistemic record and must carry legal provenance.
- **Phase 4:** institutions, records, wrong suspicion, reports, households/norms procedures, and institutional knowledge remain locked behind their entry gate.
- **Second proof and later scale:** notices, travel, regional scale, LOD, prehistory, story-sifting, and broader world simulation remain deferred.
- **Speech/LLM depth:** natural-language generation, dialogue quality, and theory-of-mind language behavior are deferred; the no-prose-authority and no-hidden-truth boundary is not deferred.
- **Broad social epistemics:** communication networks, nested theory-of-mind, group belief revision, rumor institutions, and generalized contradiction taxonomies are not required unless already exercised by the live gate-3 code; present implemented claims must remain honest and privacy-safe.
- **Reading channel breadth:** the schema placeholder may remain staged if clearly declared and unused as pass evidence. A partially implemented live path may not be hidden as a deferral.
- **Performance:** timing and scale data may be recorded, but performance is not an EPI pass threshold unless it changes determinism, staleness, event order, privacy, or evidence integrity.

No deferral may weaken sealed context construction, subjective beliefs, observation provenance, contradiction linkage, replay-derived projection, proposal context parity, possession parity, embodied filtering, debug quarantine, relational hidden-truth non-interference, or the configured mutation requirement.

## 12. External research notes shaping the audit design

External sources inform audit method only; they do not establish any fact about Tracewake's repository or certify an EPI seam.

- Security non-interference is a relation over sets of executions rather than a property visible in one trace. The audit therefore requires paired hidden-truth runs and equality over actor-visible/decision-producing outputs, not just isolated negative examples.[^hyperproperties]
- Self-composition is a standard way to reduce a two-run information-flow relation to a check over paired executions. The `EPI-11` matrix applies that idea as a practical test harness: same observable history, controlled hidden difference, compared EPI outputs.[^self-composition]
- In partially observable decision processes, decisions are based on an information state derived from observation history rather than direct omniscient state. That supports requiring actor-known inputs and testing that hidden world changes do not affect decisions before observation.[^pomdp]
- W3C PROV's entity/activity/agent and generation/use/derivation relations reinforce the requirement to retain stable source identities and explicit derivation chains instead of accepting an unscoped “has provenance” flag.[^prov]
- QuickCheck-style property testing supports generating many valid inputs and shrinking failures, while metamorphic testing supplies useful relations when a single expected output is hard to write. The audit uses both for canonicalization, hidden-truth perturbation, source removal, and possession/debug relations, while labeling generated evidence honestly as sampled unless exhaustive.[^quickcheck][^metamorphic]
- Capability systems treat possession of an unforgeable token as authority and support least-privilege compartmentalization. The debug audit therefore tests both unforgeability and absence of a feedback path; a token type alone is not enough.[^capsicum]
- cargo-mutants documents that tool-version changes may alter generated mutants. The audit therefore pins/records the version and requires a fresh file/mutant census whenever that version changes.[^mutants-stability]

[^hyperproperties]: Michael R. Clarkson and Fred B. Schneider, “Hyperproperties,” *Journal of Computer Security* preprint, <https://www.cs.cornell.edu/fbs/publications/Hyperproperties.pdf>.
[^self-composition]: Gilles Barthe, Pedro R. D'Argenio, and Tamara Rezk, “Secure Information Flow by Self-Composition,” <https://cs.famaf.unc.edu.ar/~dargenio/sites/default/files/pdf/papers/paper-9.pdf>.
[^pomdp]: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, “Planning and Acting in Partially Observable Stochastic Domains,” *Artificial Intelligence* 101 (1998), <https://people.csail.mit.edu/lpk/papers/aij98-pomdp.pdf>.
[^prov]: W3C, “PROV-DM: The PROV Data Model,” <https://www.w3.org/TR/prov-dm/>.
[^quickcheck]: Koen Claessen and John Hughes, “QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs,” <https://dl.acm.org/doi/10.1145/351240.351266>.
[^metamorphic]: “Metamorphic Testing: A Review of Challenges and Opportunities,” ACM DOI <https://dl.acm.org/doi/10.1145/3143561>.
[^capsicum]: Robert N. M. Watson et al., “Capsicum: Practical Capabilities for UNIX,” <https://www.usenix.org/event/sec10/tech/full_papers/Watson.pdf>.
[^mutants-stability]: cargo-mutants documentation, “Stability,” <https://mutants.rs/stability.html>.

## 13. Self-check against this spec

- [x] The spec opens with a cited determination that `EPI-CERT` is the next admissible move after the scope-limited 0037 `P0-CERT passed` and 0039 `SPINE-CERT passed` artifacts; it proposes no feature expansion.
- [x] Actor-known/holder-known contexts, beliefs, observations, provenance, possession parity, view models, and debug quarantine are all covered by numbered audit points with real target-commit modules.
- [x] Every audit point includes positive and adversarial fixtures/properties, event/replay/projection/determinism evidence, applicable provenance/debug evidence, exact commands, and named failure layers.
- [x] The spec is non-executable, fabricates no test/replay/mutation result, and renders no pass/fail verdict.
- [x] It is the audit-spec analogue of 0036/0038, not a mutation-remediation spec. A failed floor is routed to a later separate spec without pre-authoring its fix.
- [x] The staging path is `specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`; target commit and `Certification` posture are explicit.
- [x] The header consumes both predecessor artifacts and treats `EPI-CERT` as a composed label, not a new gate code.
- [x] Source discipline is carried verbatim from the ledger; archived Phase 2A specs remain history only.
- [x] Authority order, crate direction, anti-contamination, replay derivation, possession parity, and debug non-diegesis are preserved.
- [x] The acceptance contract instantiates evidence status, fingerprint scope, behavior witness, replay/provenance ancestry, sampling/exhaustiveness, pending/historical handling, certification use, and staged-abstraction fields.
- [x] The mutation posture expands the configured guarded layer to the complete EPI module set and requires a survivor triage register.
- [x] `EMERGE-OBS` remains observer-only and non-gating.
- [x] Every repository file named in the audit inventory is present in the uploaded manifest and was fetched through an exact-commit URL before use.

## Outcome

Completed: 2026-06-19

The implementing session executed the 0040 ticket series, produced the section 9 acceptance artifact at `archive/reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md`, populated the EPI-01 through EPI-11 evidence sections, retained the §6.1 compile-fail matrix, expanded and ran the configured EPI mutation perimeter, packaged replay/provenance and EMERGE-OBS observer-only evidence, and rendered the aggregate verdict.

Verdict: `EPI-CERT scoped remediation`. The configured EPI mutation perimeter left a 30-mutant survivor floor recorded in `reports/0040_epi_cert_mutation_final_missed.txt` and `reports/0040_epi_cert_mutation_triage_register.md`; no survivor was accepted as equivalent or non-critical. This spec therefore completed as a certification audit with scoped remediation required, not as `EPI-CERT passed`.

Verification recorded by the ticket series:

- `cargo test --workspace --locked` - passed for the capstone.
- `cargo test --locked -p tracewake-core --test emergence_ledger` - passed for observer-only EMERGE-OBS packaging.
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording` - passed for artifact wording/evidence honesty.
- The full closeout gates were run after spec archival: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, and `cargo test --workspace`.
