# 0038 SPINE-CERT event-log, replay, projection, pipeline, and no-direct-dispatch certification spec

```text
Staging path: specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md
Target repository: joeloverbeck/tracewake
Target commit: b03ceedc5360d30894f297e40efcbddcc4fd0a7f
Spec series: numbered staging spec 0038; archived to archive/specs/ on acceptance
Status: Draft certification spec; non-executable audit plan and acceptance contract
Work posture: Certification
P0-CERT admissibility posture: P0-CERT passed; consumes the 0037 P0-CERT mutation-remediation replacement acceptance artifact and the P0-CERT gate it supersedes
Phase-certification label: SPINE-CERT, as defined by the execution ladder; this document mints no new gate code, invariant ID, status enum, or obligation code
```

This document is **non-executable**: it specifies what the implementing session must run, prove, record, and package. It renders no pass/fail verdict and does not assert that the current implementation passes.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 0. Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: b03ceedc5360d30894f297e40efcbddcc4fd0a7f
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: exact full-URL fetches using web.open against raw.githubusercontent.com and github.com/blob URLs; one auxiliary exact raw URL download was used only after the same exact URL form was verified
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

Fetched files:

- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/archive/specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/reports/0037_mutation_triage_register.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/checksum.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/envelope.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/projections.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/log.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/events/apply.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/replay/rebuild.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/replay/report.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/checksum.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/state.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/scheduler.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/pipeline.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/proposal.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/registry.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/report.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/defs/eat.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/defs/sleep.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/defs/work.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/actions/defs/wait.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/actor_known.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/transaction.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/decision.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/planner.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/trace.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/routine.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/agent/methods.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/epistemics/knowledge_context.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/view_models.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/debug_capability.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/src/debug_reports.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/src/app.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/src/input.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/src/debug_panels.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/src/render.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/src/transcript.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/manifest.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/load.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/schema.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/serialization.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/validate.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/src/fixtures/mod.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-content/tests/schema_conformance.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/tests/spine_conformance.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/tests/anti_regression_guards.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/tests/adversarial_gates.rs
- https://github.com/joeloverbeck/tracewake/blob/b03ceedc5360d30894f297e40efcbddcc4fd0a7f/crates/tracewake-tui/tests/transcript_snapshot.rs

---

## 1. Determination confirmation: SPINE-CERT is the next admissible move

`SPINE-CERT` is the next admissible Tracewake work item. The execution ladder orders certification as `P0-CERT -> SPINE-CERT -> EPI-CERT -> ORD-LIFE-CERT -> FIRST-PROOF-CERT -> PHASE-4-ENTRY -> SECOND-PROOF-ENTRY`; it defines `SPINE-CERT` as certifying the event log, replay, projection, randomness, save, action pipeline, TUI/debug split, and no direct dispatch. The same ladder says Phase-4 or expansion work is invalid until the preceding gates pass. The live spec ledger records `0037` as the current P0-CERT replacement artifact and records the 0037 verdict as `P0-CERT passed`; it does not declare any later gate as already passed. Therefore this spec is the single next admissible staging spec and does not survey feature expansion or gameplay alternatives.

The 0037 acceptance artifact is used only for the admissibility fact that `P0-CERT passed`. The 0036 failed/scoped artifact and 0037 remediation spec are structural precedent and history; they are not treated as live certification of any SPINE-CERT seam.

## 2. Source discipline, freshness, and admissibility posture

This spec follows the live ledger discipline:

- Manifests are path inventory only. The uploaded manifest was used to verify that every named repository path exists in the user-supplied target tree. The manifest is not proof of file content.
- Exact-commit URLs are the only trusted content source for this document. Branch names, default-branch labels, GitHub repository metadata, connector namespaces, archived commit hashes inside old artifacts, and prior chats are not evidence.
- Archived specs and archived acceptance reports are cited as history unless explicitly referenced for the current admissibility posture. `0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md` supplies only the settled `P0-CERT passed` posture.
- This spec cannot amend invariants, weaken gate semantics, create a gate code, or rename an accepted doctrine term. `SPINE-CERT` is a phase-certification label that composes existing canonical gates and review artifacts.
- Use `holder-known context` as the system-wide term. Use `actor-known` only for the actor case.
- The implementing session must abort a SPINE-CERT pass claim if any evidence depends on unfetched files, branch-relative paths, snippets, repo search, default branch, mutable local state, or an implementation commit other than the target commit recorded in its own acceptance artifact.

## 3. Authority and dependency declarations

The governing authority order is foundation -> architecture -> execution -> reference/spec staging -> implementation. If implementation convenience conflicts with accepted gates or upstream doctrine, implementation is wrong.

Primary foundation dependencies for this spec are:

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`

Primary architecture dependencies are:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

Primary execution dependencies are:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

Primary reference and spec-tier dependencies are:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

The crate dependency direction under certification is core <- content <- TUI in authority terms: the kernel owns simulation truth and event application; content may seed/load/validate into kernel-owned state; TUI may render and submit semantic actions, but may not own simulation authority. The fetched workspace manifests show the concrete Rust workspace members `tracewake-core`, `tracewake-content`, and `tracewake-tui`; `tracewake-content` depends on `tracewake-core`; `tracewake-tui` depends on both `tracewake-core` and `tracewake-content`. SPINE-CERT must preserve that direction and must reject any reverse dependency, hidden side effect, or client-owned truth shortcut.

## 4. Command vocabulary for the implementing session

The commands below define the minimum command vocabulary. Each audit point may narrow to a subset, but the final acceptance artifact must include the full command transcript, exit status, environment, tool versions, and exact implementation commit fingerprint.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
```

Targeted gate commands:

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

The implementing session must also run every golden fixture declared in `crates/tracewake-content/src/fixtures/mod.rs` through the content golden runner and replay acceptance path, including both positive and adversarial fixture families. Fixture names are enumerated in section 5 and may be expanded by the implementer only by adding evidence, not by replacing the required families.

## 5. The SPINE-CERT audit contract

### SPINE-01 — Event log, event envelope, and append-only causal stream

**Seam definition.** Every meaningful simulation state transition must be represented by an event envelope in an append-only stream. Event identity, type, schema version, stream boundary, order, tick, causes, proposal/validation linkages, random draw references, payload, effects summary, and content-manifest identity must be sufficient for replay, audit, and provenance. Correction is by later event, not by rewriting history.

**Audited files/modules.**

- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/mod.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`

**Doctrine to satisfy.** Foundational event-trace invariants require event-before-state causality, replayable changes, seedable/auditable randomness, and deterministic replay. Architecture requires event envelopes, stream boundaries, schema-version handling, no silent rewrite, and projection rebuild rather than projection authority. Binding invariants: `INV-009`, `INV-010`, `INV-011`, `INV-013`, `INV-017`, `INV-018`, `INV-020`.

**Positive fixtures.** The implementing session must prove at least:

- `replay_item_location_001` appends ordered event envelopes whose replay reconstructs the item/location state.
- `container_item_move_001`, `door_access_001`, `strongbox_001`, and `ordinary_workday_001` exercise multiple streams and causal links.
- `sleep_eat_work_001` and `no_human_day_001` prove longer causal continuity rather than a single-event toy trace.

**Adversarial fixtures.** The implementing session must prove rejection or loud failure for:

- Missing or unsupported event schema versions.
- Duplicate event IDs, duplicate global order, wrong stream position, and malformed stream keys.
- Missing causes where doctrine requires causes.
- Event payloads that imply world mutation without a typed event/effects summary.
- Any fixture that attempts to insert hidden truth or prose-born facts through event payloads rather than typed, provenance-bearing simulation events.

**Evidence mechanics.**

- Record canonical event-log serialization fingerprints for each required fixture.
- Rebuild state from the serialized event log and compare canonical state/projection checksums.
- Record the first divergence report for at least one intentionally corrupted log.
- Record event-envelope field coverage: event ID, event type, schema version, stream, stream position, global order, simulation tick, ordering key, actor/process/participants/place, causes, proposal ID, validation report ID, random draw refs, payload hash or payload value, effects summary, and content manifest ID.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Failure-diagnostic layers.** `doctrine mismatch`, `content/schema validation`, `action validation`, `event application`, `projection/replay`, `tests/fixtures`, `documentation status`.

### SPINE-02 — Replay rebuild, divergence reporting, and deterministic replay gates

**Seam definition.** Replay must consume persisted event history and content/schema identity, rebuild the authoritative state/projections, and either match expected fingerprints or fail loudly with typed divergence diagnostics. Replay must not consult live mutable truth, external time, nondeterministic iteration, environment variables, filesystem discovery outside declared manifests, network, process spawning, or UI state.

**Audited files/modules.**

- `crates/tracewake-core/src/replay/mod.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-core/tests/generative_lock.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-content/tests/fixtures_load.rs`
- `tests/negative-fixtures/banned_env_var/src/lib.rs`
- `tests/negative-fixtures/banned_fs_read_and_file_open/src/lib.rs`
- `tests/negative-fixtures/banned_process_command_new/src/lib.rs`
- `tests/negative-fixtures/banned_systemtime_alias/src/lib.rs`

**Doctrine to satisfy.** Replay is an accepted gate family and the spine’s core. It must prove deterministic reconstruction from ordered events and declared configuration. Diagnostics must name the responsible layer, not hide a mismatch as a broad failure. Binding invariants: `INV-018`, `INV-019`, `INV-092`.

**Positive fixtures.** The implementing session must replay all golden fixtures listed in `crates/tracewake-content/src/fixtures/mod.rs`, with explicit coverage for `replay_item_location_001`, `sleep_eat_work_001`, `ordinary_workday_001`, `wait_then_window_passive_charges_each_tick_once_001`, and `sleep_spanning_window_boundary_charges_each_tick_once_001`.

**Adversarial fixtures.** Required corruptions:

- Drop the last event from a passing log.
- Swap two same-tick events with different ordering keys.
- Change the content manifest ID in a replay package.
- Change one schema version to an unsupported future version.
- Insert a valid-looking event whose causes refer to a nonexistent prior event.
- Re-run replay under different host time/timezone and prove identical output or fail-closed diagnostics.

**Evidence mechanics.**

- Record expected vs actual state checksum, projection checksum, event count, diagnostic count, unsupported versions, replay errors, `matches_expected`, and first-divergence details.
- Record canonical replay package fingerprints before and after replay.
- Record deterministic duplicate-run evidence: same package -> same checksums and same report.
- Record negative-run evidence: corrupted package -> typed failure, no repaired success.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-core --test generative_lock
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

`generative_lock` is the existing multi-seed generative/property witness: it generates cases across `GENERATIVE_SEEDS`, runs `run_replay`/`rebuild_projection`, and asserts checksum stability, supplying the duplicate-run determinism and metamorphic-corruption evidence this seam requires rather than only fixed happy-path examples.

**Failure-diagnostic layers.** `content/schema validation`, `scheduler ordering`, `event application`, `projection/replay`, `tests/fixtures`, `documentation status`.

### SPINE-03 — Projection rebuild and non-truth-writer quarantine

**Seam definition.** Projections may read event-derived state and produce review, view, notebook, replay, or debug material, but projections are not authoritative truth writers. Rebuilding projections from the event log must be deterministic, and projection output must not inject actor-visible knowledge, alter state, schedule actions, or mask provenance gaps.

**Audited files/modules.**

- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`

**Doctrine to satisfy.** Projections are rebuildable views and diagnostics. Holder-known context construction must be sealed with source and frontier, and projection output must not become simulation truth. Binding invariants: `INV-006`, `INV-024`, `INV-059`, `INV-067`, `INV-107`.

**Positive fixtures.** The implementing session must prove:

- `view_filtering_001`, `view_model_local_actions_001`, and `possession_parity_001` render expected holder-known or embodied views without raw truth leakage.
- `no_human_observation_facts_cite_log_events_001`, `workplace_assignment_provenance_001`, and `stale_workplace_notice_superseded_by_newer_001` preserve provenance and freshness.
- Rebuilding projections after replay yields the same projection fingerprints as the live run.

**Adversarial fixtures.** Required failures:

- Projection attempts to write authoritative state or append events.
- Projection output gives an actor hidden facts absent from its holder-known context.
- Projection output omits provenance for actor-visible facts.
- Projection output silently chooses a fallback plan when a provenance gap should cause rejection, wait, or diagnostic.
- Debug projection content enters an embodied view.

**Evidence mechanics.**

- Record projection fingerprints per fixture, before and after replay.
- Record holder-known context IDs, hashes, event frontiers, source scopes, provenance entries, and forbidden-truth audit results for representative views.
- Record one negative fixture where a projection leak is rejected and the responsible layer is named.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Failure-diagnostic layers.** `actor-known context construction`, `candidate generation`, `planning/method selection`, `projection/replay`, `view-model rendering`, `debug quarantine`, `tests/fixtures`.

### SPINE-04 — Randomness and random-stream discipline

**Seam definition.** All simulation randomness must be seedable, stream-scoped, replay-visible, and auditable. Hidden random entry points are not allowed. Any random draw that can affect simulation state, scheduling, validation, or view-visible outcomes must be recorded in the event envelope or an equivalent replay package record so replay can reproduce or verify it.

**Audited files/modules.**

- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/agent/generation.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/tests/anti_regression_guards.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`
- `tests/negative-fixtures/banned_rand_entry_points/src/lib.rs`

**Doctrine to satisfy.** Foundation requires seedable and auditable randomness. Architecture requires stream-scoped deterministic random draws and replay-visible random draw records. Execution requires replay and fixture evidence, not “it seems deterministic” assertions. Binding invariants: `INV-017`, `INV-018`.

**Positive fixtures.** The implementing session must prove either:

1. The target commit contains no state-affecting random draw sites beyond event-envelope random draw references, in which case static and compile-fail evidence must show the absence of banned random entry points; or
2. Every state-affecting draw belongs to a named stream, uses a stable seed/configuration, records draw references in event envelopes, and replays deterministically.

At least one fixture must exercise the actual random-stream path if such a path exists. If no such path exists, the acceptance artifact must explicitly mark the path as “not exercised because no state-affecting random draw site exists at this commit,” with static grep/compile-fail evidence and no pass-by-silence.

**Adversarial fixtures.** Required failures:

- Direct use of `thread_rng`, OS entropy, time-seeded RNG, nondeterministic hash iteration, or external random APIs in simulation code.
- Event with random effects but no random draw reference.
- Replay package whose random draw record differs from the event log.
- Same seed/config/package producing different event log or projection checksums.

**Evidence mechanics.**

- Record source scan results for banned random/time/env/process/network entry points and the negative-fixture outcomes.
- Record seed/config identity and random-stream identity for each random fixture.
- Record duplicate-run evidence: same seed/config -> same event log, state checksum, projection checksum, and replay report.
- Record perturbation evidence: different seed changes only permitted random outcomes and not doctrine-critical provenance or debug quarantine.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

The implementing session must add or identify a state-affecting RNG fixture before claiming this seam passed if the existing suite does not exercise one.

**Failure-diagnostic layers.** `doctrine mismatch`, `candidate generation`, `scheduler ordering`, `action validation`, `event application`, `projection/replay`, `tests/fixtures`.

### SPINE-05 — Save package, manifest integrity, schema versioning, and upcast/read discipline

**Seam definition.** A replayable save package must bind event log, content manifest, schema/ruleset identity, snapshot ancestry if snapshots exist, and replay provenance. Content manifests and fingerprints are part of the replay contract; saves may optimize load but may not replace event ancestry. Unsupported schema versions or missing upcasters must fail loudly.

**Audited files/modules.**

- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/serialization.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-content/tests/fixtures_load.rs`
- `crates/tracewake-content/tests/schema_conformance.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`

**Doctrine to satisfy.** Save/replay packages must preserve ancestry, schema identity, content identity, and loud failure for incompatible versions. Materialized views and snapshots are admissible only as optimizations with manifest/fingerprint honesty, not as new truth sources. Binding invariants: `INV-019`, `INV-020`.

**Positive fixtures.** Required:

- Every content fixture in `crates/tracewake-content/src/fixtures/mod.rs` loads under a stable content manifest fingerprint.
- `schema_conformance` proves schema acceptance for all live fixture modules.
- A replay package records and reuses `content_manifest_id` consistently across event envelopes and replay reports.
- If snapshot support exists at the implementation commit, at least one fixture must prove snapshot-assisted load equals event-log replay.

**Adversarial fixtures.** Required failures:

- Content manifest file order changes without semantic changes must not change canonical fingerprint.
- Content byte change must change fingerprint.
- Missing content file, extra unmanifested content file, or mismatched manifest ID must fail loudly.
- Unsupported event schema or content schema must fail at content/schema validation or replay, not in a late panic or silent downgrade.
- Snapshot without event ancestry must be rejected for certification use.

**Evidence mechanics.**

- Record content manifest identity, stable fingerprint, source file list, canonical path ordering, schema versions, and validation diagnostics.
- Record save/replay package manifest including event log fingerprint, content manifest fingerprint, schema/ruleset identity, optional snapshot fingerprint, and snapshot ancestry pointer.
- Record at least one adversarial manifest/fingerprint mismatch artifact.

**Exact commands.**

```bash
cargo test --locked -p tracewake-content --test fixtures_load
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-core --test event_schema_replay_gates
```

**Failure-diagnostic layers.** `content/schema validation`, `event application`, `projection/replay`, `tests/fixtures`, `documentation status`.

### SPINE-06 — Action proposal, validation, scheduling, event append, application, and feedback pipeline

**Seam definition.** Every world-affecting action must enter as an ordinary proposal with source context, pass shared validation, be scheduled only through permitted scheduler ordering, append events, apply events through the kernel mutation capability, and return actor-visible and debug-only feedback through the proper quarantine. Validation truth may accept, reject, or mutate via typed events; it may not propose fallback plans or actor-visible hidden facts.

**Audited files/modules.**

- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/no_human_capstone.rs`

**Doctrine to satisfy.** The canonical action path is proposal -> validation -> scheduling -> event append -> event application -> feedback. No direct state mutation, no hidden-truth planning, no validation fallback plan, no scheduler rewrite of actor reason, and no TUI/debug bypass are allowed. Binding invariants: `INV-043`, `INV-099`, `INV-103`, `INV-104`, `INV-106`.

**Positive fixtures.** Required:

- `sleep_eat_work_001`, `ordinary_workday_001`, `wait_then_window_passive_charges_each_tick_once_001`, `work_block_failed_then_sleep_succeeds_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, and `work_completion_fails_when_actor_displaced_001` exercise proposal/validation/scheduling/application across action definitions.
- `severe_safety_with_known_exit_produces_move_001` proves actor-known proposal can act when knowledge is sufficient.
- `routine_blocked_diagnostic_001` and `method_fallback_requires_new_trace_or_stuck_001` prove diagnostics when planning is blocked.

**Adversarial fixtures.** Required:

- `no_hidden_truth_planning_001`, `prose_born_fact_rejected_001`, `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`, and `scheduler_cannot_rewrite_wait_reason_after_transaction_001` must fail closed or emit the expected diagnostic.
- Direct invocation of an action definition that mutates state without `run_pipeline` must be impossible by API, compile-fail fixture, or explicit test.
- Scheduler may append due completions or passive events only as permitted canonical scheduled events, never by rewriting actor decisions after a transaction.

**Evidence mechanics.**

- Record proposal ID, source, source holder-known context ID/hash/frontier, validation report ID, event IDs appended, actor-visible facts, debug-only facts, and trace diagnostics per representative fixture.
- Record transaction trace for an accepted action and a rejected action.
- Record that event append precedes event application and that application mints the internal mutation capability rather than exposing mutation authority to proposal, TUI, or content.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test no_human_capstone
cargo test --locked -p tracewake-core --test golden_scenarios
cargo test --locked -p tracewake-content --test golden_fixtures_run
```

**Failure-diagnostic layers.** `actor-known context construction`, `candidate generation`, `planning/method selection`, `proposal construction`, `scheduler ordering`, `action validation`, `event application`, `projection/replay`, `tests/fixtures`.

### SPINE-07 — TUI, embodied view models, transcript surface, and debug split

**Seam definition.** The TUI is a client and proof surface, not a simulation authority. It may render embodied view models, accept semantic input, submit proposals through the shared action pipeline, and show quarantined debug overlays only under a debug capability. It must not read hidden truth into embodied views, mutate state, bypass validation, or make debug-only facts diegetic.

**Audited files/modules.**

- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/transcript.rs`
- `crates/tracewake-tui/src/launch.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`
- `crates/tracewake-tui/tests/command_loop_session.rs`
- `crates/tracewake-tui/tests/transcript_snapshot.rs`
- `crates/tracewake-tui/tests/tui_acceptance.rs`

**Doctrine to satisfy.** Embodied views must be derived from holder-known contexts and scoped projections. Debug surfaces are non-diegetic and must be visibly segregated. TUI commands must become semantic proposals and must use the shared pipeline. Binding invariants: `INV-065`, `INV-067`, `INV-068`, `INV-069`, `INV-070`.

**Positive fixtures.** Required:

- `view_model_local_actions_001`, `embodied_view_omits_raw_assignment_without_context_001`, `embodied_menu_lags_truth_change_without_perception_001`, and `possession_parity_001` prove embodied view model filtering.
- `debug_attach_001` proves debug capability use without diegetic leakage.
- TUI command-loop tests prove user input becomes `ProposalSource::TuiSemanticAction` and enters `run_pipeline`.

**Adversarial fixtures.** Required:

- `debug_omniscience_excluded_001` must prove debug omniscience stays out of embodied views.
- TUI attempts to render hidden truth without holder-known provenance must fail or omit it.
- TUI attempts to call action definitions or mutate state directly must fail by code structure, compile-fail fixture, or explicit seam test.
- Debug panels must assert or otherwise prove `debug_only` content, and transcript snapshots must not include debug-only tokens in ordinary embodied transcript output.

**Evidence mechanics.**

- Record an embodied transcript snapshot for a passing scenario and a debug transcript/overlay snapshot for the same state, proving different channels.
- Record semantic-action submission path: input -> proposal -> source context -> pipeline -> appended event -> view refresh from event frontier.
- Record debug capability construction and quarantine evidence; the debug capability must not be constructible by external crates or content.

**Exact commands.**

```bash
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test command_loop_session
cargo test --locked -p tracewake-tui --test transcript_snapshot
cargo test --locked -p tracewake-tui --test tui_acceptance
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test negative_fixture_runner
```

**Failure-diagnostic layers.** `actor-known context construction`, `proposal construction`, `action validation`, `projection/replay`, `view-model rendering`, `debug quarantine`, `tests/fixtures`.

### SPINE-08 — No direct dispatch and full mutation-path closure

**Seam definition.** No caller may bypass the shared proposal/validation/scheduler/event append/application path for world-affecting behavior. Agent planners, scheduler, TUI, content fixtures, debug helpers, tests, and action definitions must not directly dispatch state mutation or forge internal mutation capability. This seam certifies the closure of all bypass routes rather than one happy path.

**Audited files/modules.**

- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-tui/src/app.rs`
- `tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_call_seed_mutators_after_load/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_mutate_agent_state_seed_maps/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/src/lib.rs`
- `tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/src/lib.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`
- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-core/tests/ci_workflow_guards.rs`

**Doctrine to satisfy.** The no-direct-dispatch execution document requires actor transactions, scheduler activity, and TUI/client input to flow through the canonical action pipeline. Hidden truth and validation cannot synthesize actor plans. Internal mutation authority must remain kernel-owned. Binding invariants: `INV-008`, `INV-100`, `INV-103`, `INV-104`.

**Positive fixtures.** Required:

- One accepted actor proposal, one accepted TUI semantic action, one scheduled passive/completion event, and one replay-applied event sequence must all demonstrate legal mutation paths.
- Compile-fail or negative fixture evidence must show external crates cannot forge mutation capability, call post-load seed mutators, mutate seed maps, insert raw epistemic records, or read raw projection maps.

**Adversarial fixtures.** Required:

- Any direct state mutation from TUI, content, action definition, scheduler rewrite, debug panel, projection, or test-only helper must fail by type boundary, compile-fail fixture, explicit negative test, or mutation-test survivor killed by a guard.
- Any action rejection that tries to fallback-plan from validation truth must produce a rejection/diagnostic event or actor-visible blocker, never an alternate hidden-truth action.

**Evidence mechanics.**

- Record API-boundary evidence for `pub`, `pub(crate)`, private marker/capability constructors, and compile-fail negative fixtures.
- Record trace evidence for legal paths and failure evidence for illegal paths.
- Record dependency-boundary evidence that TUI/content cannot import or call internal mutation constructors and that core does not depend on TUI/content for simulation authority.

**Exact commands.**

```bash
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test ci_workflow_guards
cargo test --locked -p tracewake-tui --test tui_seam_conformance
cargo test --locked -p tracewake-tui --test adversarial_gates
```

**Failure-diagnostic layers.** `doctrine mismatch`, `proposal construction`, `scheduler ordering`, `action validation`, `event application`, `view-model rendering`, `debug quarantine`, `tests/fixtures`, `documentation status`.

## 6. Fixture matrix required for a SPINE-CERT pass claim

The acceptance artifact must include a fixture matrix with these minimum families:

| Family | Required positive fixtures | Required adversarial fixtures | Seams |
| --- | --- | --- | --- |
| Event/replay basics | `replay_item_location_001`, `container_item_move_001`, `door_access_001`, `strongbox_001` | corrupted event order, duplicate stream position, unsupported schema | SPINE-01, SPINE-02 |
| Long-running ordinary traces | `sleep_eat_work_001`, `ordinary_workday_001`, `no_human_day_001`, `wait_then_window_passive_charges_each_tick_once_001`, multi-seed `generative_lock` corpus | dropped event, manifest mismatch, same-tick ordering perturbation | SPINE-01, SPINE-02, SPINE-06 |
| Projection and holder-known views | `view_filtering_001`, `view_model_local_actions_001`, `possession_parity_001`, `workplace_assignment_provenance_001` | `debug_omniscience_excluded_001`, hidden truth in view, missing provenance | SPINE-03, SPINE-07 |
| Pipeline and scheduler | `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_completion_fails_when_actor_displaced_001`, `severe_safety_with_known_exit_produces_move_001` | `scheduler_cannot_rewrite_wait_reason_after_transaction_001`, direct mutation bypass | SPINE-06, SPINE-08 |
| Truth firewall | `aged_food_record_surfaces_as_remembered_belief_not_observation_001`, `partial_food_source_knowledge_001` | `no_hidden_truth_planning_001`, `prose_born_fact_rejected_001`, `forbidden_provenance_input_fails_closed_001`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` | SPINE-03, SPINE-06, SPINE-08 |
| Save/manifest/schema | all fixture modules in `crates/tracewake-content/src/fixtures/mod.rs` | content byte change, unmanifested file, unsupported content/event schema | SPINE-02, SPINE-05 |
| RNG discipline | actual state-affecting RNG fixture if one exists; otherwise explicit no-RNG evidence | banned RNG/time/env/process entry points; event with random effect but missing draw ref | SPINE-04 |
| TUI/debug split | `debug_attach_001`, TUI command-loop and transcript snapshots | debug token in embodied transcript, TUI direct mutation, hidden truth render | SPINE-07, SPINE-08 |

The implementing session may add fixtures but may not substitute out these families. A family with no currently exercisable code path, especially RNG, must be marked as non-certifying static evidence unless and until a positive behavior witness is added.

## 7. Mutation-testing posture

SPINE-CERT inherits the 0037 posture that mutation evidence is part of certification, but it cannot merely reuse the 0037 mutation perimeter as complete SPINE-CERT coverage. The fetched `.cargo/mutants.toml` and CI workflow show a configured guarded layer focused on a P0/ordinary-life subset. That subset is useful precedent, not sufficient coverage for the full spine, because SPINE-CERT also covers events, replay, save/content manifest, TUI/debug split, and no-direct dispatch.

### 7.1 Required mutation waves

**Wave A — existing guarded-layer continuity.** Run the existing configured guarded command and record the result exactly:

```bash
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --workspace \
  -f 'crates/tracewake-core/src/agent/**' \
  -f 'crates/tracewake-core/src/scheduler*' \
  -f 'crates/tracewake-core/src/projections*' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  -f 'crates/tracewake-core/src/actions/defs/eat.rs' \
  -f 'crates/tracewake-core/src/actions/defs/sleep.rs' \
  -f 'crates/tracewake-core/src/actions/defs/work.rs' \
  --no-shuffle
```

**Wave B — SPINE seam expansion.** Run a SPINE-CERT-specific mutation configuration whose inclusion list covers at least:

- `crates/tracewake-core/src/events/**`
- `crates/tracewake-core/src/replay/**`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/checksum.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/serialization.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/transcript.rs`

The implementing session must include the exact generated/configured cargo-mutants command in the acceptance artifact. If the current `.cargo/mutants.toml` excludes any Wave B file, the session must either provide a SPINE-CERT-specific cargo-mutants configuration that includes it or mark the seam as scoped remediation. Silent exclusion is a certification failure.

### 7.2 Survivor triage register

The implementing session must create `reports/0038_spine_cert_mutation_triage_register.md` using the 0037 register structure:

```text
# 0038 SPINE-CERT mutation triage register

Status: <no survivors | scoped remediation required>
Version: SPINE-CERT 0038
Baseline misses: <empty | listed with source>
Command transcript:
- <Wave A command and exit status>
- <Wave B command/config and exit status>

## Survivor entries

### <survivor name or mutant id>
- Mutated file:
- Mutated function/module:
- Responsible SPINE seam:
- Responsible failure-diagnostic layer:
- Current behavior witness:
- Why existing tests missed it:
- Required remediation test/fixture:
- Required production remediation, if any:
- Equivalent-mutant claim, if asserted:
- Evidence for equivalent-mutant claim:
- Disposition:
```

No SPINE-CERT pass may be claimed with an untriaged survivor, a survivor that changes spine behavior, or an equivalent-mutant assertion lacking behavioral and doctrine evidence. A scoped remediation posture must name the seam and responsible layer.

## 8. Failure handling

A failed seam produces a named remediation spec/report and does not permit relabeling the phase, skipping to EPI-CERT/ORD-LIFE-CERT/FIRST-PROOF-CERT/Phase-4, or declaring a special exception. The acceptance artifact must identify the failure under one or more responsible layers from the execution doctrine:

- `doctrine mismatch`
- `content/schema validation`
- `actor-known context construction`
- `candidate generation`
- `planning/method selection`
- `proposal construction`
- `scheduler ordering`
- `action validation`
- `event application`
- `projection/replay`
- `view-model rendering`
- `debug quarantine`
- `tests/fixtures`
- `documentation status`

Minimum failure record:

```text
Finding title:
SPINE seam:
Responsible layer:
Failing fixture/test/command:
Expected doctrine:
Observed behavior:
Evidence artifact path:
Replay/provenance fingerprint affected:
Mutation survivor, if applicable:
Remediation required:
Certification posture impact: <SPINE-CERT scoped remediation | not applicable>
```

A `SPINE-CERT scoped remediation` result is admissible only as a blocking remediation state. It is not a pass and cannot feed later ladder gates.

## 9. Acceptance-artifact contract

The implementing session must instantiate the acceptance artifact template in a report such as:

```text
archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md
```

The report must state that this 0038 spec defined the audit and that the report, not this spec, renders the verdict. A passing report must contain all fields below.

### 9.1 Header and scope

```text
Title:
Spec under execution: specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md
Implementation repository: joeloverbeck/tracewake
Implementation commit:
Commit freshness claim:
Spec posture consumed: P0-CERT passed
Gate label under certification: SPINE-CERT
Verdict: <SPINE-CERT passed | SPINE-CERT scoped remediation | not applicable>
Non-executable spec note: this report executes the audit; the spec did not certify results
```

### 9.2 Evidence-status ledger

For every evidence item, include:

```text
Evidence ID:
SPINE seam(s):
Evidence status: <observed run | static review | negative fixture | mutation evidence | historical only | pending>
Fingerprint scope:
Behavior witness:
Replay/provenance record:
Sampling/exhaustiveness claim:
Pending/historical caveat:
Certification use:
Staged-abstraction note:
Artifact path or command transcript:
```

`historical only` evidence may explain lineage but cannot satisfy a live SPINE-CERT requirement. `static review` may inform risk, but cannot replace a required behavior witness unless the code path genuinely has no behavior to execute and the absence itself is the certified fact.

### 9.3 Command transcript and environment

Record:

- Rust toolchain and `rustc --version --verbose`.
- `cargo --version`.
- `cargo-mutants --version`.
- Host OS and architecture.
- Full command transcript for all commands in section 4 and seam-specific commands.
- Exact test filters, if any. A filtered run cannot satisfy a full-suite obligation unless paired with the unfiltered required command.
- Exit status and output artifact location for every command.

### 9.4 Per-seam verdict table

| Seam | Required status for SPINE-CERT passed | Required artifacts |
| --- | --- | --- |
| SPINE-01 event log | all positive and adversarial event-log evidence passed | canonical event log fingerprints, corrupted-log rejection |
| SPINE-02 replay | deterministic replay and loud divergence evidence passed | replay reports, duplicate-run checksums, first-divergence artifact |
| SPINE-03 projection | rebuildable non-truth-writer projection evidence passed | projection fingerprints, holder-known context/provenance records |
| SPINE-04 randomness | RNG stream discipline proven or no-RNG absence proven | seed/stream records, banned entry-point guards, duplicate-run evidence |
| SPINE-05 save/manifest | manifest/schema/replay package integrity passed | manifest fingerprints, schema diagnostics, mismatch failure artifact |
| SPINE-06 pipeline | canonical action pipeline and validation evidence passed | proposal/validation/event trace records, accepted/rejected witnesses |
| SPINE-07 TUI/debug | embodied/debug quarantine evidence passed | transcript snapshots, debug-only artifacts, semantic-action path evidence |
| SPINE-08 no direct dispatch | bypass closure and mutation capability evidence passed | compile-fail negative fixtures, API-boundary evidence, mutation results |

### 9.5 Replay and provenance package

The acceptance artifact must include or link:

- Event-log packages for each required fixture.
- Replay reports for each required fixture.
- State and projection checksums.
- Content manifest fingerprints.
- Holder-known context IDs, hashes, frontiers, and provenance ancestry for representative actor/TUI/projection cases.
- Debug quarantine artifacts, separated from embodied transcripts.
- Corrupted/adversarial packages and their loud-failure reports.

### 9.6 Mutation package

The acceptance artifact must include:

- Wave A command, output, and survivor status.
- Wave B command/config, output, and survivor status.
- `reports/0038_spine_cert_mutation_triage_register.md`.
- Explanation for every missed, timeout, unviable, or equivalent mutant.
- Statement that no Wave B seam was silently excluded by cargo-mutants configuration.

### 9.7 EMERGE-OBS handling

If first-proof living-world acceptance is exercised by the verified corpus, the report may include an `EMERGE-OBS` artifact as observer-only evidence. The existing `crates/tracewake-core/tests/emergence_ledger.rs` builds an `EmergeObsLedger` over the no-human generated corpus and is the concrete observer-only witness available at this commit. `EMERGE-OBS` is not a phase gate, is not a pass/fail threshold, and cannot substitute for any SPINE seam.

## 10. Preliminary static survey — not certification

This section is informative only. It cannot satisfy SPINE-CERT by itself.

Exact-commit static reading suggests the repository already contains recognizable spine seams:

- `EventEnvelope` includes event identity, type, schema version, stream/position/order, tick, ordering key, causal/proposal/validation/random-draw fields, payload/effects summary, and content manifest linkage.
- `EventLog` append paths enforce supported schema versions, global order, stream position, and canonical serialization/deserialization boundaries.
- `apply_event_stream` dispatches event payloads through kernel event application and uses an internal mutation capability rather than broad public mutation authority.
- `ReplayReport` records fixture ID, content manifest ID, expected/actual checksums, event count, diagnostics, unsupported versions/errors, match state, state diff, and first divergence.
- `actions/pipeline.rs` has a `PipelineContext` carrying registry, state, agent state, event log, controller bindings, epistemic projection, content manifest ID, and ordering key; the pipeline appends events before application.
- `actions/proposal.rs` carries proposal source and source holder-known context ID/hash/frontier/provenance ancestry, which is the right structural hook for anti-contamination and TUI provenance evidence.
- `knowledge_context.rs` contains sealed embodied/private/debug context construction with allowed/forbidden sources, event frontier, debug flag, holder-known context ID/hash, provenance entries, and forbidden-truth audit fields.
- `tracewake-tui/src/app.rs` appears to submit TUI semantic actions as proposals through `run_pipeline` and to gate debug views separately from embodied views.
- `debug_capability.rs` uses a private marker/capability shape, and TUI render/debug tests assert debug-only segregation.
- `tracewake-content/src/manifest.rs` computes stable fingerprints from sorted source files and raw bytes.

Static risk flags for the implementing session:

- The current cargo-mutants configuration is too narrow to certify all SPINE seams because it excludes or omits multiple event/replay/save/TUI files. SPINE-CERT must expand mutation coverage or render scoped remediation.
- The fetched files show event-level random draw references, but this static survey did not certify an exercised state-affecting RNG stream. A source scan over `crates/*/src` finds no `rng`/`rand`/PRNG entry point: the only seeded generator at this commit is the test-support `Lcg` in `crates/tracewake-core/tests/support/generative.rs`, which synthesizes test inputs and is not kernel simulation state. This supports branch (1) of SPINE-04 (no state-affecting random draw site), but the implementing session must still re-run that scan as recorded evidence and either certify the absence or add/identify a positive RNG behavior witness — it may not pass this seam by silence.
- Save/package certification may currently be represented by content manifests and replay packages rather than a full save subsystem. The implementing session must prove the package satisfies doctrine or name the save-manifest gap as scoped remediation.
- Static code shape is promising but irrelevant to pass/fail without the required live commands, fixture outputs, replay packages, and mutation evidence.

## 11. Tolerated deferrals and explicit out-of-scope boundaries

SPINE-CERT must not audit or “fix” later-gate substance except where needed to prove the spine boundary. These are tolerated deferrals:

- EPI-CERT: deep epistemic substrate adequacy, belief/memory richness, contradiction modeling, social knowledge propagation, and epistemic UX beyond the spine firewall and provenance requirements.
- ORD-LIFE-CERT: full ordinary-life economy, needs/routines completeness, no-human-day behavioral richness, workplace/household realism, and long-horizon stability beyond fixtures needed for replay and scheduler proof.
- FIRST-PROOF-CERT: first playable proof, missing-property village acceptance, player-facing narrative quality, emergent story sufficiency, and living-world acceptance beyond the spine certification corpus.
- PHASE-4-ENTRY: institutions, records, wrong suspicion, households/norms expansion, and story-sifting features.
- SECOND-PROOF-ENTRY and later: notices, travel, regional scale, LOD/prehistory, and larger simulation proofs.
- LLM/speech surfaces: natural-language generation quality and speech-act depth, except that no prose may create simulation truth and no language surface may bypass event/provenance doctrine.
- EMERGE-OBS: may be packaged as observer-only evidence but never becomes a gate or pass/fail threshold.
- Performance/scale: useful diagnostics may be recorded, but performance is not a SPINE-CERT pass threshold unless performance behavior affects determinism, replay, event order, or evidence honesty.

No deferral may weaken event-sourcing, replay, holder-known context, no-direct-dispatch, debug quarantine, or crate-dependency doctrine.

## 12. External research notes shaping this audit design

These sources are used only for general audit-method design, not for repository facts:

- Martin Fowler’s event sourcing description treats state changes as a sequence of events and highlights rebuild/replay as a central advantage; SPINE-CERT therefore requires append-only event history, replay rebuild, temporal divergence evidence, and no state-only pass claims. Source: <https://martinfowler.com/eaaDev/EventSourcing.html>.
- Microsoft Azure’s event sourcing guidance notes that materialized views can be derived from events and that compensating actions are added rather than mutating prior history; SPINE-CERT therefore requires projections/snapshots to remain derived and corrections to be new events. Source: <https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing>.
- Axon’s event versioning/upcasting guidance describes transforming old event structures on read while retaining history; SPINE-CERT therefore requires schema-version and upcast/read-path evidence rather than silent downgrade. Source: <https://docs.axoniq.io/axon-framework-reference/4.10/events/event-versioning/>.
- The Rust Rand Book distinguishes reproducible RNG APIs and recommends deterministic seeded generators for simulations; SPINE-CERT therefore requires explicit seed/stream identity and rejects hidden entropy/time-seeded paths. Sources: <https://rust-random.github.io/book/guide-rngs.html#reproducibility> and <https://rust-random.github.io/book/guide-seeding.html>.
- The Proptest Book frames property tests as assertions over arbitrary inputs with shrinking; SPINE-CERT therefore requires adversarial/metamorphic log corruption, manifest perturbation, and duplicate-run determinism evidence rather than only fixed happy-path examples. Source: <https://proptest-rs.github.io/proptest/intro.html>.
- cargo-mutants defines mutation testing as checking whether inserted code changes are caught by tests; SPINE-CERT therefore requires guarded-layer mutation waves plus survivor triage. Source: <https://mutants.rs/>.
- Snapshot testing documentation describes comparing complex output to reference snapshots; SPINE-CERT therefore requires transcript/projection/replay package snapshots where raw equality is the useful evidence shape. Source: <https://insta.rs/docs/>.

## 13. Self-check against this spec

- SPINE-CERT is confirmed as the next admissible move and no feature-expansion alternative is proposed.
- All eight spine seams are covered as numbered audit points with audited files, fixtures, evidence mechanics, commands, and failure layers.
- The document is non-executable and renders no verdict.
- The staging filename is `specs/0038_SPINE_CERT_EVENT_LOG_REPLAY_PROJECTION_PIPELINE_AND_NO_DIRECT_DISPATCH_CERTIFICATION_SPEC.md`.
- The header declares `Certification`, `P0-CERT passed`, target commit `b03ceedc5360d30894f297e40efcbddcc4fd0a7f`, and no new gate/invariant/status code.
- Archived specs are history except for the 0037 admissibility fact.
- The acceptance-artifact contract instantiates the evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical, certification-use, and staged-abstraction fields.
- EMERGE-OBS remains observer-only.
