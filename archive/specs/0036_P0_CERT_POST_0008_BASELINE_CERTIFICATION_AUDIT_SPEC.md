# 0036 — P0-CERT Post-0008 Baseline Certification Audit Spec

**Status**: COMPLETED  
**Spec path:** `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `9f1622244c91c5952bd735da76f29fbe58f39f4b`  
**Posture:** Certification  
**Scope:** post-0008 baseline certification audit only; no gameplay expansion, no Phase 4 entry, no second-proof entry, no pass/fail verdict rendered by this document.

This spec operationalizes the live foundation, architecture, execution, reference, and specs-tier doctrine into the next executable audit. It specifies how the implementing session must prove the historical post-0008 baseline. It does not certify the code by itself.

---

## 0. Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 9f1622244c91c5952bd735da76f29fbe58f39f4b
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open against exact raw.githubusercontent.com URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/need_accounting.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/time.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/no_human_surface.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/epistemics/knowledge_context.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/epistemics/knowledge_basis.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/candidate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/htn.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/methods.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/agent/need.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/defs/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/epistemics/projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/debug_reports.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/validate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/src/ids.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/.cargo/mutants-baseline-misses.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/clippy.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/rust-toolchain.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/replay_item_location_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/ordinary_workday_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/view_filtering_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/debug_attach_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/work_completion_fails_when_actor_displaced_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/9f1622244c91c5952bd735da76f29fbe58f39f4b/crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Manifest mismatch note.** The commissioning brief listed `scheduler/no_human.rs` as a scheduler/no-human exploration seam. That path is not present in the uploaded manifest. The manifest contains `crates/tracewake-core/src/scheduler.rs`; the exact-commit source at that path contains an inline `no_human` module. This spec therefore treats `scheduler/no_human.rs` as shorthand for `crates/tracewake-core/src/scheduler.rs::no_human` and does not fetch or depend on a non-manifest path.

**External research sources consulted.** These sources shaped the audit method but are not repository authority:

- Martin Fowler, `Event Sourcing` — used to sharpen the requirement that the event log is the durable source from which state and projections can be rebuilt: https://martinfowler.com/eaaDev/EventSourcing.html
- Microsoft Azure Architecture Center, `Event Sourcing pattern` — used to frame append-only event capture, materialized views, and replay as separate evidence surfaces: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- W3C, `PROV-DM: The PROV Data Model` — used to require explicit entity/activity/agent-style provenance rows for holder-known context and evidence packaging: https://www.w3.org/TR/prov-dm/
- `cargo-mutants` documentation — used to require mutation/lock-layer evidence for anti-contamination guards, with survived mutants treated as remediation data rather than pass evidence: https://mutants.rs/ and https://mutants.rs/ci.html
- Stryker, `Mutation testing` overview — used for the killed/survived/no-coverage/incompetent mutant vocabulary in the evidence package: https://stryker-mutator.io/docs/mutation-testing-elements/mutation-testing/
- Hypothesis, `Property-based testing` documentation — used to require replayable generated cases and recorded seeds for varied hidden-truth, ordering, and invalid-content checks: https://hypothesis.readthedocs.io/en/latest/
- Antithesis, `Deterministic simulation testing` — used as method guidance for deterministic schedule/replay artifacts, not as a dependency: https://antithesis.com/docs/

---

## 1. Status and admissibility posture

**Status.** Staging implementation spec, authored as `specs/0036_*` and intended to be archived only after acceptance under the repository's implementation-spec closeout convention. It is subordinate to the live authority order: foundation governs architecture, architecture governs execution, execution governs reference/specs, and implementation is measured against all of them.

**Authority posture.** This spec is a **Certification** posture spec. It proves gates without expanding gameplay scope. It creates no new gate code, observation-obligation code, responsible-layer vocabulary, status enum, doctrine tier, fixture taxonomy, or replacement authority. It composes existing gates and labels only:

- canonical execution gates: `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, `DIAG`;
- first-proof acceptance-contract labels: `EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, `FIXTURE-NEGATIVE`;
- observer-only obligation: `EMERGE-OBS`, which remains non-gating and non-threshold.

**Bootstrapping admissibility line.** Future implementation specs must declare one of `P0-CERT passed`, `P0-CERT scoped remediation`, or `P0-CERT not applicable`. This spec is not a consumer of a prior `P0-CERT` result. It is the certification-producing spec. None of the three consumer labels is semantically valid as an input posture here. The implementing session for this spec must produce the artifact that later specs cite as `P0-CERT passed`, or must instead produce a scoped remediation outcome naming the failed gate and responsible layer. A code-affecting future spec may not treat this spec's existence as `P0-CERT passed`; only the completed acceptance artifact may do that.

**No verdict.** This document specifies the audit plan. It does not assert that any target seam has passed. Preliminary code observations below are explicitly non-certifying.

---

## 2. Authority and dependency declarations

### 2.1 Controlling foundation sources

The audit must preserve the constitutional invariants and first-proof boundaries in:

- `docs/README.md` — authority order and tier layering;
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — all `INV-*` cross-references are inherited; none are reworded here;
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — truth may validate; truth may not plan;
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first-village/no-human/replay/TUI proof scope;
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, replay, randomness, schema/version discipline;
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — ordinary action parity and survival needs;
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — possession parity, actor-filtered view models, debug quarantine.

### 2.2 Controlling architecture sources

The audit measures implementation against:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — conformance questions and replacement/retirement rule;
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — event log, replay, projections, save manifests, randomness, schema versioning;
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts, provenance packets, context sealing;
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — proposal pipeline, scheduler limits, validation truth, no direct dispatch;
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — actor decision transaction, needs, intentions, HTN selection, stuck diagnostics;
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — possession, embodied TUI, debug-only truth, client boundaries;
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance artifacts, anti-contamination tests, diagnostics, review checklists.

### 2.3 Controlling execution sources

Execution is the heart of this audit. The implementing session must read and cite:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — canonical gate names, observation obligations, label reconciliation, universal posture;
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md` — the ten P0-CERT proof requirements and code audit boundary;
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — first-proof gate labels and definition of first-proof done;
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — gate order, evidence requirements, future-spec postures;
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — TFW evidence;
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — PIPE and NO-DIRECT evidence;
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — NO-HUMAN evidence;
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — POS-PARITY and VIEW-DEBUG-SPLIT evidence;
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — content schema/provenance rejection evidence;
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` — golden/adversarial fixture and replay evidence;
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — DIAG standard, behavior witnesses, evidence status, fingerprint-scope honesty, and EMERGE-OBS;
- `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` and `docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` only for deferral boundaries;
- `docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md` as source-discipline and forbidden-misread context.

### 2.4 Reference and specs-tier dependencies

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — review guardrails and gate-code lookup posture;
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — relapse risks, especially treating archived specs as certification;
- `docs/3-reference/02_GLOSSARY.md` — terminology: `holder-known context` system-wide, `actor-known` for actor case;
- `docs/4-specs/README.md` — future-spec rules, source discipline, no symmetry files;
- `docs/4-specs/SPEC_LEDGER.md` — active ledger posture and archived-spec source discipline;
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — live first-proof ontology/fixture contract;
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — evidence package fields this spec requires.

Archived specs `0005` through `0035`, prior tickets, and archive reports may be cited only as historical evidence and structural examples. They cannot certify live code, cannot override the replacement doctrine, and cannot supply gate outcomes.

---

## 3. Problem statement

The repository contains historical implementation for the post-0008 baseline: event-sourced kernel surfaces, replay, subjective epistemics, possession parity, needs/routines/no-human ordinary life, anti-contamination hardening, TUI embodiment, content validation, and fixtures. That implementation history is useful, but it is not certification.

`P0-CERT` is the next required implementation target because the live execution tier blocks Phase 4, second-proof work, LLM surfaces, travel, regional scale, and new gameplay systems until a code audit proves the baseline under replacement doctrine. The old phase labels and archived specs locate historical work; they do not prove the live gates.

The gap this spec closes is therefore not a feature gap. It is an evidence gap. The historical code must be audited seam by seam against the live ten-point `P0-CERT` contract, the first-proof acceptance contract, and the gate-evidence requirements. Any pass must be shown by production-path behavior witnesses, replay/projection evidence, actor-known provenance, debug quarantine, positive and negative fixtures, and typed responsible-layer diagnostics. Any failure must become a remediation target rather than a phase-local exception.

---

## 4. Audit approach

### 4.1 Method

The implementing session must perform a code audit, not a documentation assertion. The audit must:

1. fetch target sources only from exact commit URLs or from an exact exported tree supplied by the user;
2. verify every audited path against the manifest path inventory;
3. inspect the listed seams and follow call paths outward only through fetched exact-commit files;
4. run the repository's formatting, build, test, CI-equivalent lock-layer, replay, content-validation, and TUI acceptance commands from an exact target tree;
5. run the configured mutation/anti-regression posture, treating survived mutants, skipped critical seams, or unsupported mutation execution as evidence-status data rather than silent passes;
6. produce an acceptance artifact that maps every gate claim to exact seams, behavior witnesses, replay/projection artifacts, provenance rows, debug-quarantine evidence, fixture outcomes, and typed diagnostics.

### 4.2 Evidence hierarchy

The audit may use these evidence classes, in descending strength:

1. **Production-path behavior witness:** an observed run, fixture, test, or transcript that traverses ordinary source code paths and records source event/proposal/context/projection/replay IDs.
2. **Replay/projection equivalence:** deterministic rebuild from serialized log and fixture manifest, with live projection and replay projection checksums compared by scope.
3. **Actor-known provenance row:** a sealed holder-known context hash/frontier plus source events, records, observations, memories, or belief origins for every action-relevant fact.
4. **Negative fixture failure:** an adversarial fixture that fails closed for the intended responsible layer, not just for a display string or incidental panic.
5. **Static/lock-layer guard:** anti-regression checks for forbidden direct dispatch, hidden-truth access, unordered iteration, wall-clock/randomness entry points, debug leakage, and string-typed shortcuts.
6. **Mutation evidence:** targeted mutants killed by tests or explicitly remediated if survived.
7. **Historical evidence:** archived specs/tickets/reports. This may explain why a seam exists, never certify it.

No artifact-presence check is sufficient alone. Stable bytes are not semantic proof unless paired with behavior witnesses and replay/provenance ancestry.

### 4.3 Fixture strategy

The audit must use both friendly and hostile fixture families. Every protected shortcut requires at least one live negative that would fail if the shortcut were reintroduced, or an explicit explanation of why no live negative can exist. The fixture corpus must cover:

- hidden-truth planning and hidden route/item/workplace denial;
- no-human ordinary life with ordinary actions, modeled waits, and typed stuck/failure records;
- possession parity and possession non-reset;
- replay determinism and projection equivalence;
- embodied view-model filtering and debug-only truth separation;
- content validation rejection of outcome chains, prose-born facts, debug/omniscience fields, forbidden provenance inputs, and direct-dispatch privilege;
- direct dispatch rejection and pipeline ancestry;
- diagnostics by responsible layer.

### 4.4 Determinism and replay strategy

Event-sourcing practice treats the event log as the durable sequence from which state can be reconstructed; this shapes the audit requirement that live state is never itself authority. Replay evidence must therefore include both byte-level/fingerprint scopes and semantic behavior witnesses. The implementing session must provide:

- initial fixture manifest fingerprint and schema version;
- event log serialization fingerprint;
- event envelope ancestry/order/version evidence;
- live projection checksum and replay projection checksum;
- divergence report, or an explicit `no divergence` report with checked scope;
- recorded seeds and random-stream identifiers for any generated/property case.

### 4.5 Mutation and lock-layer posture

Mutation testing is a lock-layer, not a replacement for behavior witnesses. The implementing session must run the configured `cargo-mutants` posture where supported and report killed/survived/timeout/incompetent/uncovered mutants by protected seam. A survived mutant in a critical P0-CERT seam is not automatically a failure if the mutant is semantically irrelevant, but it must be triaged in the acceptance artifact. A survived mutant that permits hidden-truth planning, direct dispatch, scheduler primitive action emission, debug leakage, replay divergence, or provenance bypass is a remediation failure until fixed or justified by a better equivalent guard.

### 4.6 Staged abstractions

This audit may stage only proof abstractions, never feature abstractions. A staged abstraction must declare:

- what current proof it provides;
- what behavior it intentionally abstracts;
- what it must not fake;
- which future tier/feature it must not certify by implication;
- which diagnostic would fail if the abstraction leaked into certification.

`EMERGE-OBS` rows, no-human metrics, and debug-side truth/belief comparison may inform observer review only. They never become pass/fail thresholds, actor cognition, validator input, scheduler input, fixture selectors, content-selection input, difficulty/pacing input, or phase-entry criteria.

---

## 5. Preliminary static seam survey — non-certifying

This section sharpens the audit plan. It is **preliminary, static, and non-certifying**. The authoritative pass/fail result belongs to the implementing session after it runs the required commands and produces the acceptance artifact.

| Area | Exact seams observed at commit `9f16222` | Static implication for the audit | Risk to probe |
|---|---|---|---|
| Scheduler/no-human | `crates/tracewake-core/src/scheduler.rs`, `need_accounting.rs`, `time.rs` | `scheduler.rs` contains an inline `no_human` module and functions such as `run_no_human_day`; the audit should verify it delegates ordinary decisions through actor transactions and pipeline paths. | Scheduler may still synthesize passive deltas or wait markers; prove they are allowed, typed, replayed, and not primitive action shortcuts. |
| Actor-known transaction | `agent/transaction.rs`, `actor_known.rs`, `no_human_surface.rs`, `knowledge_context.rs`, `knowledge_basis.rs`, `trace.rs` | Static names indicate sealed proposals, context hashes/frontiers, provenance diagnostics, and hidden-truth audit surfaces. | A context wrapper can look sealed while carrying truth-derived or fixture-derived facts; prove source ancestry and negative fail-closed paths. |
| Proposal/pipeline | `actions/proposal.rs`, `actions/pipeline.rs`, `actions/report.rs`, `actions/registry.rs`, `controller.rs`, action defs | Proposal origin/source context appears explicit; pipeline seams exist for validation and report emission. | Direct dispatch can hide in controller, TUI input, tests, action defs, or scheduler helpers; search and behavior witnesses must prove all world-affecting actions cross the same path. |
| Event/replay/projection | `events/log.rs`, `events/envelope.rs`, `events/apply.rs`, `events/mutation.rs`, `state.rs`, `projections.rs`, `replay/rebuild.rs`, `replay/report.rs`, `checksum.rs` | Event envelopes, schema version checks, ordering, application, projection, and replay report seams exist. | A test may compare bytes without proving semantics; require projection equivalence and behavior witnesses. |
| TUI/debug split | `tracewake-tui/src/app.rs`, `debug_panels.rs`, `render.rs`, `input.rs`, `debug_capability.rs`, `debug_reports.rs`, `view_models.rs` | Embodied/debug rendering and capability seams exist. | Debug data may be exposed to actor-visible affordances, tests, or possession preflight; require carrier census and debug quarantine negatives. |
| Content/fixtures | `tracewake-content/src/schema.rs`, `validate.rs`, `serialization.rs`, fixtures and tests | Fixture schema and validation seams exist; many adversarial fixture names are present. | Friendly fixture labels can masquerade as proof; every golden must have semantic witness and negative failure reason. |
| Lock layer | `.github/workflows/ci.yml`, `.cargo/mutants.toml`, `clippy.toml`, `rust-toolchain.toml` | CI and mutation configuration seams exist. | CI can drift, mask failures, or omit lock-layer tests; acceptance artifact must prove commands and mutation posture actually ran. |

---

## 6. Gate-evidence requirements checklist

Every per-gate section below must be reflected in the final acceptance artifact. The artifact is incomplete if any row is absent.

| Required evidence element | Where this spec requires it |
|---|---|
| Exact files and seams audited | Sections 7.1–7.10 and Appendix A ledger. |
| Foundation and architecture dependencies | Section 2 and Section 9. |
| Artifact dependencies, including observer-only `EMERGE-OBS` where the corpus exercises first-proof living-world acceptance | Section 8.8 and Section 10. |
| Positive fixtures and negative fixtures | Sections 7.1–7.10. |
| Event/replay/projection evidence | Sections 7.1, 7.5, 7.8, 8.4. |
| Actor-known provenance evidence | Sections 7.2, 7.5, 7.6, 7.8, 8.4. |
| Debug-quarantine evidence | Sections 7.4, 7.7, 8.4. |
| Failure diagnostics by responsible layer | Sections 7.1–7.10 and 8.5. |
| Archived specs/tickets historical-only statement | Sections 1, 2.4, 7.10, 8.7. |
| Tolerated deferrals tied to named gates | Section 10. |

---

## 7. Per-gate deliverables: ten P0-CERT proof requirements

### 7.1 P0-01 — World-affecting behavior enters through proposal, validation, event append, application, projection, and replay

**P0-CERT proof requirement.** All world-affecting behavior enters through proposal, validation, event append, application, projection, and replay boundaries.

**Gates composed.** `P0-CERT`, `PIPE`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG`; first-proof labels `EVENT` and `REPLAY`.

**Concrete seams audited.**

- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/defs/mod.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/events/log.rs`
- `crates/tracewake-core/src/events/envelope.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-core/src/checksum.rs`

**Positive fixtures and tests.**

- `sleep_eat_work_001.rs` — ordinary sleep/eat/work progression through events;
- `ordinary_workday_001.rs` — ordinary workday path;
- `no_human_day_001.rs` and `crates/tracewake-core/tests/no_human_capstone.rs` — no-human ordinary interval;
- `replay_item_location_001.rs` — replay/projection proof for item state;
- `crates/tracewake-content/tests/golden_fixtures_run.rs` and `crates/tracewake-core/tests/golden_scenarios.rs` — golden scenario execution;
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` — event schema/replay lock layer;
- `crates/tracewake-core/tests/spine_conformance.rs` — spine/pipeline conformance suite mapping pipeline seams to named evidence;
- `crates/tracewake-core/tests/anti_regression_guards.rs` — static/lock-layer guards against direct dispatch and event-append bypass.

**Adversarial/negative fixtures and tests that must fail closed.**

- direct-dispatch rejection cases in `crates/tracewake-core/tests/acceptance_gates.rs` and `crates/tracewake-core/tests/hidden_truth_gates.rs`;
- `work_completion_fails_when_actor_displaced_001.rs` — validator rejection when reality invalidates completion;
- negative fixtures in `tests/negative-fixtures/` that block forbidden mutation, filesystem/network/process/time/randomness, and external construction seams;
- any mutation that bypasses proposal construction, pipeline validation, event append, event application, projection, or replay must be killed or triaged as remediation.

**Required event/replay/projection evidence.**

The acceptance artifact must include a behavior witness showing: triggering input or scheduler decision → `Proposal` with origin/source context → pipeline validation report → appended event envelope with schema/version/order/ancestry → event application result → live projection checksum → serialized log → replay rebuild report → replay projection checksum. The artifact must name fingerprint scope separately for raw bytes, normalized serialization, parsed semantic content, run seed, and replay artifact.

**Actor-known/provenance evidence.**

Where the proposal is actor-originated, the witness must include holder-known context ID, context hash/frontier, and action-relevant fact provenance. Where the proposal is non-cognitive passive accounting, the artifact must identify the allowed non-cognitive origin and show it cannot produce primitive action choices.

**Debug-quarantine evidence.**

Debug reports may inspect the event/projection path but must not feed action selection, validation, projection authority, or replay comparison. Debug-derived rows are `observer-only` unless tied to production-path events.

**Typed failure diagnostic.**

Failures must use one of: `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `scheduler`, or `test_oracle`, with expected/actual input source, event/projection/checksum identifiers, replay divergence point, and remediation category.

**Fail condition.**

Any world-affecting mutation that bypasses proposal/pipeline/event/replay, or any proof that relies only on current state without replay ancestry, fails this line.

---

### 7.2 P0-02 — Autonomous decisions use sealed actor-known contexts with provenance, not validation truth or debug truth

**P0-CERT proof requirement.** Autonomous actor decisions use sealed actor-known contexts with provenance, not validation truth or debug truth.

**Gates composed.** `P0-CERT`, `TFW`, `NO-HUMAN`, `FIXTURE`, `DIAG`; first-proof labels `TRUTH-FIREWALL` and `ACTOR-KNOWN`.

**Concrete seams audited.**

- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/no_human_surface.rs`
- `crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/decision.rs`
- `crates/tracewake-core/src/agent/candidate.rs`
- `crates/tracewake-core/src/agent/perception.rs`
- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/checksum.rs`

**Positive fixtures and tests.**

- `no_hidden_truth_planning_001.rs` — actor does not plan toward hidden truth;
- `seeded_food_source_unknown_to_all_actors_001.rs` — seeded world fact remains unknown absent modeled knowledge;
- `no_human_observation_facts_cite_log_events_001.rs` — observation facts cite events;
- `no_human_workplace_knowledge_requires_notice_event_001.rs` and `no_human_known_workplace_requires_provenance_001.rs` — workplace knowledge must have provenance;
- `embodied_workplace_availability_reflects_belief_not_truth_001.rs` — availability reflects belief;
- `stale_workplace_notice_superseded_by_newer_001.rs` if exercised by the implementing session for freshness.

**Adversarial/negative fixtures and tests that must fail closed.**

- `forbidden_provenance_input_fails_closed_001.rs` — forbidden provenance input must not produce an agent-origin proposal;
- `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs` — typed provenance checks cannot be replaced by banned-word scanning;
- `hidden_food_closed_container_001.rs`, `hidden_route_edge_001.rs`, and `no_human_unseen_workplace_assignment_does_not_plan_work_001.rs` — hidden facts cannot be selected;
- mutation that supplies validation truth or debug truth to candidate generation/method selection must be killed.

**Required event/replay/projection evidence.**

The artifact must pair each autonomous proposal with the event/projection ancestry that created the actor-visible belief, observation, memory, routine assignment, known route, or known affordance. Replay must reconstruct the same context hash/frontier and proposal sequence for deterministic seeds.

**Actor-known/provenance evidence.**

For every action-relevant fact in the selected goal and local plan, record source kind, source event/projection/context IDs, freshness/staleness classification, holder, and whether unknown was explicitly represented. A sealed context with empty, dangling, wrong-kind, ambiguous, debug-only, validation-truth, or harness-fabricated provenance fails.

**Debug-quarantine evidence.**

Hidden-truth audit comparison rows and debug truth may appear only in debug/review artifacts. The artifact must show they are absent from candidate-generation and planning inputs.

**Typed failure diagnostic.**

Failures must identify `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, or `test_oracle`, with expected and actual input source and hidden truth excluded/leaked.

**Fail condition.**

Any autonomous action selected from validation truth, raw fixture truth, debug panels, unproven schema data, display strings, or hidden-truth comparison rows fails this line.

---

### 7.3 P0-03 — Human-origin commands bind to ordinary actors and share autonomous action rules

**P0-CERT proof requirement.** Human-origin commands bind to ordinary actors and share the same action rules as autonomous actors.

**Gates composed.** `P0-CERT`, `PIPE`, `NO-DIRECT`, `POS-PARITY`, `FIXTURE`, `DIAG`; first-proof labels `POSSESSION-PARITY`, `EVENT`, and `VIEW-DEBUG-SPLIT`.

**Concrete seams audited.**

- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/actions/proposal.rs`
- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/defs/mod.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/state.rs`

**Positive fixtures and tests.**

- `possession_parity_001.rs` — possessed and autonomous actor share ordinary world rules;
- `view_model_local_actions_001.rs` — actor-visible local actions are derived from embodied state;
- `crates/tracewake-tui/tests/command_loop_session.rs`, `embodied_flow.rs`, `tui_acceptance.rs`, and `tui_seam_conformance.rs` if present in the implementing tree;
- action-definition tests for eat/sleep/work/move/wait/open/take/check/inspect.

**Adversarial/negative fixtures and tests that must fail closed.**

- direct-dispatch rejection tests that try to mutate state from TUI/controller without proposal/pipeline ancestry;
- `debug_attach_001.rs` and `debug_omniscience_excluded_001.rs` when a debug-capable UI exists beside possession;
- `possession_does_not_reset_intention_001.rs` for human binding persistence;
- mutation that gives possessed actor privileged action definitions, hidden knowledge, or direct event append authority must be killed.

**Required event/replay/projection evidence.**

For at least one human-origin command and one analogous autonomous decision, the artifact must show both produce ordinary proposals, traverse the same action validation and event append seams, and replay to equivalent world effects when context permits. Differences in input origin must appear as proposal origin metadata, not as privileged action semantics.

**Actor-known/provenance evidence.**

Human-origin commands may bind to the actor's ordinary holder-known context and actor-visible affordance set. They may not import player omniscience, debug truth, or fixture truth into the actor's memory/beliefs. The artifact must show source context or why the command is a direct ordinary action constrained by embodied affordance rather than cognition.

**Debug-quarantine evidence.**

If debug panels are active, their capability must remain separate from the possessed actor's action menu and embodied view. A debug attach must not alter candidate actions, memories, beliefs, needs, routines, or state authority.

**Typed failure diagnostic.**

Failures must identify `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation`, `event_append`, `debug_quarantine`, or `test_oracle`.

**Fail condition.**

Any human command that bypasses ordinary actor binding, invokes an action unavailable to autonomous actors under the same world facts, or uses debug/player truth as actor knowledge fails this line.

---

### 7.4 P0-04 — Possession never resets needs, intentions, memories, routines, or world-fact access

**P0-CERT proof requirement.** Possession never resets needs, intentions, memories, routines, or access to world facts.

**Gates composed.** `P0-CERT`, `POS-PARITY`, `TFW`, `FIXTURE`, `DIAG`; first-proof labels `POSSESSION-PARITY`, `ACTOR-KNOWN`, and `VIEW-DEBUG-SPLIT`.

**Concrete seams audited.**

- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/agent/need.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`

**Positive fixtures and tests.**

- `possession_parity_001.rs`;
- `possession_does_not_reset_intention_001.rs`;
- `view_filtering_001.rs` and `view_model_local_actions_001.rs`;
- TUI possession tests and transcript snapshots in `crates/tracewake-tui/tests/` where the target tree includes them.

**Adversarial/negative fixtures and tests that must fail closed.**

- debug attach or possession-rebind scenarios that attempt to refresh hidden facts or reset intention lifecycle;
- `embodied_view_omits_raw_assignment_without_context_001.rs` and `embodied_view_omits_unobserved_food_at_open_place_001.rs`;
- mutation that clears needs, intentions, memories, routine assignments, known affordances, or knowledge frontier on possession must be killed.

**Required event/replay/projection evidence.**

The acceptance artifact must include pre-bind and post-bind state/projection fingerprints for the possessed actor: need state, intention lifecycle, routine assignment, belief/memory/observation projections, known routes/affordances, and actor-visible menu. Any change must be attributable to an ordinary event, not possession itself.

**Actor-known/provenance evidence.**

The artifact must show the actor's holder-known context before and after possession, including source IDs and freshness classifications for exposed facts. Possession may change control routing only. It may not create, refresh, delete, or reclassify beliefs.

**Debug-quarantine evidence.**

Debug attach must appear as non-diegetic capability state. It must not alter actor-visible carriers or holder-known context. Debug truth/belief comparison rows are observer-only.

**Typed failure diagnostic.**

Failures must identify `tui_input_binding`, `holder_known_context`, `intention_lifecycle`, `view_model`, `debug_quarantine`, or `projection`.

**Fail condition.**

Any possession bind/rebind that rewrites actor needs, intentions, routines, memories, beliefs, known facts, or action affordances without an ordinary event fails this line.

---

### 7.5 P0-05 — Scheduler paths cannot emit primitive world actions from raw truth, routine labels, need thresholds, or fixture tables

**P0-CERT proof requirement.** Scheduler paths cannot emit primitive world actions from raw truth, routine labels, need thresholds, or fixture tables.

**Gates composed.** `P0-CERT`, `NO-HUMAN`, `PIPE`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG`; first-proof label `NO-HUMAN-ORDINARY-LIFE`.

**Concrete seams audited.**

- `crates/tracewake-core/src/scheduler.rs` including inline `scheduler.rs::no_human`;
- `crates/tracewake-core/src/need_accounting.rs`;
- `crates/tracewake-core/src/time.rs`;
- `crates/tracewake-core/src/agent/transaction.rs`;
- `crates/tracewake-core/src/agent/no_human_surface.rs`;
- `crates/tracewake-core/src/actions/pipeline.rs`;
- `crates/tracewake-core/src/actions/report.rs`;
- `crates/tracewake-core/src/events/log.rs`;
- `crates/tracewake-core/src/events/apply.rs`.

**Positive fixtures and tests.**

- `no_human_day_001.rs`;
- `ordinary_workday_001.rs`;
- `sleep_eat_work_001.rs`;
- `severe_safety_with_known_exit_produces_move_001.rs` and `severe_safety_without_known_exit_waits_with_knowledge_blocker_001.rs` if exercised;
- `wait_then_window_passive_charges_each_tick_once_001.rs` and `sleep_spanning_window_boundary_charges_each_tick_once_001.rs` for single-charge/passive accounting;
- `crates/tracewake-core/tests/no_human_capstone.rs`.

**Adversarial/negative fixtures and tests that must fail closed.**

- `scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs`;
- `no_human_unseen_workplace_assignment_does_not_plan_work_001.rs`;
- `no_human_workplace_knowledge_requires_notice_event_001.rs`;
- `no_human_known_workplace_requires_provenance_001.rs`;
- `method_fallback_requires_new_trace_or_stuck_001.rs`;
- `no_human_metrics_require_typed_responsible_layer_001.rs` — no-human metrics must carry a typed responsible layer, not free counters;
- mutation that has scheduler call action defs directly, emit primitive actions from need thresholds, use routine labels as actions, or read fixture tables as plan sources must be killed.

**Required event/replay/projection evidence.**

The artifact must classify every scheduler-emitted event in the no-human corpus as one of:

1. passive non-cognitive accounting allowed by doctrine, with typed cause and single-charge proof; or
2. ordinary actor decision transaction output with proposal/pipeline ancestry; or
3. modeled wait/stuck/failure record with typed reason.

Replay must reconstruct scheduler order, passive accounting, actor transaction outputs, and no-human metrics byte-identically within declared fingerprint scope.

**Actor-known/provenance evidence.**

Every scheduler-initiated actor decision must show the sealed actor-known context and provenance frontier supplied to the transaction. Routine/need pressure may prioritize attention, but may not become a primitive action or hidden target.

**Debug-quarantine evidence.**

No-human metrics and hidden-truth comparison rows are observer/debug review artifacts only. They must not feed scheduler priority or actor decision selection.

**Typed failure diagnostic.**

Failures must identify `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `event_append`, or `replay`.

**Fail condition.**

Any scheduler path that emits `eat`, `sleep`, `work`, `move`, `take`, `inspect`, `open`, `check`, or other primitive world action directly from truth, routine labels, need thresholds, or fixture tables fails this line.

---

### 7.6 P0-06 — Validation truth may accept/reject/mutate through events but may not propose fallback plans or actor-visible hidden facts

**P0-CERT proof requirement.** Validation truth may accept/reject/mutate through events, but may not propose fallback plans or actor-visible hidden facts.

**Gates composed.** `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `FIXTURE`, `DIAG`; first-proof labels `TRUTH-FIREWALL`, `ACTOR-KNOWN`, and `EVENT`.

**Concrete seams audited.**

- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/actions/registry.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/events/mutation.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-core/src/agent/transaction.rs`
- `crates/tracewake-core/src/agent/trace.rs`

**Positive fixtures and tests.**

- `embodied_workplace_believed_open_truth_closed_commit_fails_001.rs` — actor may believe, validator may reject based on truth;
- `work_completion_fails_when_actor_displaced_001.rs` — completion validation uses truth to reject;
- `sleep_rejects_current_place_without_sleep_affordance_001.rs` if exercised;
- `sleep_interrupted_by_severe_need_prorates_recovery_001.rs` for validation/application of interrupted duration;
- action report tests in `crates/tracewake-core/tests/acceptance_gates.rs`.

**Adversarial/negative fixtures and tests that must fail closed.**

- `hidden_food_closed_container_001.rs` and `hidden_route_edge_001.rs` — validator truth cannot become a new target;
- `forbidden_provenance_input_fails_closed_001.rs`;
- `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs`;
- mutation that turns validator rejection into fallback candidate generation, hidden target selection, actor-visible hint, or belief insertion without modeled event must be killed.

**Required event/replay/projection evidence.**

For accepted validation, the artifact must show event mutations only through the event path. For rejected validation, the artifact must show an action report and any actor-visible failure/stuck diagnostic, not a hidden-truth fallback plan. Replay must reproduce accept/reject outcomes and projection state.

**Actor-known/provenance evidence.**

The selected action must be justified by pre-validation holder-known context. Validator truth may inspect reachability, item presence, resource availability, access rights, and invariant preservation only after proposal construction. It may not add candidate goals, route hints, food targets, suspects, clues, or workplace facts to actor-known context.

**Debug-quarantine evidence.**

Validation diagnostics may include debug-only truth in review surfaces if quarantined. Actor-visible output must not include hidden fact content unless created by a modeled event/observation.

**Typed failure diagnostic.**

Failures must identify `action_validation`, `event_application`, `holder_known_context`, `candidate_generation`, `local_planning`, `proposal_construction`, or `debug_quarantine`.

**Fail condition.**

Any validator path that proposes fallback plans, updates cognition, reveals hidden facts, or converts a rejection into an actor-visible target without modeled information flow fails this line.

---

### 7.7 P0-07 — Debug surfaces are non-diegetic and cannot feed embodied affordances, actor decisions, records, institutions, or tests masquerading as knowledge

**P0-CERT proof requirement.** Debug surfaces are non-diegetic and cannot feed embodied affordances, actor decisions, records, institutions, or tests that masquerade as player knowledge.

**Gates composed.** `P0-CERT`, `TFW`, `POS-PARITY`, `FIXTURE`, `DIAG`; first-proof label `VIEW-DEBUG-SPLIT`; observer-only `EMERGE-OBS` must remain non-gating.

**Concrete seams audited.**

- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/epistemics/projection.rs`
- `crates/tracewake-core/src/agent/actor_known.rs`
- `crates/tracewake-core/src/agent/transaction.rs`

**Positive fixtures and tests.**

- `debug_attach_001.rs` — debug can attach as a quarantined capability;
- `view_filtering_001.rs` and `view_model_local_actions_001.rs` — embodied view-model filtering;
- TUI transcript/adversarial tests under `crates/tracewake-tui/tests/`;
- `debug_reports.rs` evidence surfaces for review-only truth/belief comparisons.

**Adversarial/negative fixtures and tests that must fail closed.**

- `debug_omniscience_excluded_001.rs`;
- `embodied_view_omits_raw_assignment_without_context_001.rs`;
- `embodied_view_omits_unobserved_food_at_open_place_001.rs`;
- `embodied_view_omits_unknown_sleep_affordance_001.rs` if exercised;
- `crates/tracewake-core/tests/anti_regression_guards.rs` — static guard against debug-to-actor carrier leakage;
- mutation that reads debug reports from view-model generation, actor transaction, action menu construction, test oracle as actor knowledge, or content validation as modeled fact must be killed.

**Required event/replay/projection evidence.**

The artifact must include a carrier census of actor-visible surfaces and debug-only surfaces for representative fixtures. Each actor-visible datum must have source event/projection/context IDs and holder-known provenance. Debug-only truth rows must have replay ancestry but be labeled observer-only.

**Actor-known/provenance evidence.**

No actor-known context may cite debug panels, debug capability, debug reports, hidden-truth comparison tables, or `EMERGE-OBS` rows as modeled knowledge.

**Debug-quarantine evidence.**

The artifact must demonstrate structural separation between embodied view, possession preflight/action menu, transcript, debug panel, and observer-only evidence. Tests must not pass because a debug panel displayed a fact unless the test is explicitly a debug quarantine test and the fact remains non-diegetic.

**Typed failure diagnostic.**

Failures must identify `debug_quarantine`, `view_model`, `holder_known_context`, `tui_input_binding`, or `test_oracle`, including actor-visible output and debug-only output.

**Fail condition.**

Any debug-derived fact that becomes actor cognition, embodied affordance, institution/record state, validator input, scheduler input, content-selection input, or a masqueraded actor-knowledge test oracle fails this line.

---

### 7.8 P0-08 — Golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases

**P0-CERT proof requirement.** Golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases.

**Gates composed.** `P0-CERT`, `FIXTURE`, `TFW`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `NO-DIRECT`, `DIAG`; first-proof labels `MISSING-PROPERTY`, `FIXTURE-NEGATIVE`, `REPLAY`, `VIEW-DEBUG-SPLIT`, `NO-HUMAN-ORDINARY-LIFE`.

**Concrete seams audited.**

- `crates/tracewake-content/src/fixtures/mod.rs`
- every fixture named in the evidence ledger for this spec;
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-content/src/serialization.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/tests/golden_fixtures_run.rs`
- `crates/tracewake-content/tests/forbidden_content.rs`
- `crates/tracewake-content/tests/schema_conformance.rs` if present in the target tree;
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-core/tests/negative_fixture_runner.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-core/tests/hidden_truth_gates.rs`
- `crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`.

**Positive fixtures and tests.**

At minimum the acceptance artifact must include representative positives from:

- hidden-truth resistance: `no_hidden_truth_planning_001.rs`, `seeded_food_source_unknown_to_all_actors_001.rs`;
- no-human: `no_human_day_001.rs`, `ordinary_workday_001.rs`, `sleep_eat_work_001.rs`;
- possession: `possession_parity_001.rs`, `possession_does_not_reset_intention_001.rs`;
- replay: `replay_item_location_001.rs`, event schema/replay tests;
- view-model: `view_filtering_001.rs`, `view_model_local_actions_001.rs`;
- content validation: schema conformance and load tests;
- direct-dispatch: acceptance/no-direct tests.

**Adversarial/negative fixtures and tests that must fail closed.**

At minimum the acceptance artifact must include negatives from:

- `hidden_food_closed_container_001.rs`, `hidden_route_edge_001.rs`, `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs`;
- `forbidden_provenance_input_fails_closed_001.rs`;
- `prose_born_fact_rejected_001.rs`;
- `debug_omniscience_excluded_001.rs`;
- `embodied_view_omits_raw_assignment_without_context_001.rs`, `embodied_view_omits_unobserved_food_at_open_place_001.rs`;
- `scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs`;
- direct-dispatch rejection tests and negative-fixture crates that block forbidden APIs/entry points.

**Required event/replay/projection evidence.**

Every golden fixture counted as certification evidence must include both stable serialization/fingerprint data and semantic behavior witnesses. Golden bytes alone are insufficient. Each negative fixture must show it failed for the intended layer and not because of incidental parse/panic/unrelated validation.

**Actor-known/provenance evidence.**

Fixtures that exercise cognition must list the expected holder-known context facts and prohibited hidden facts. The artifact must prove actor-known provenance comes from modeled sources, not fixture prose, comments, labels, or debug fields.

**Debug-quarantine evidence.**

View-model and debug fixtures must classify actor-visible, possessed, debug, transcript, and observer-only surfaces. Debug truth may only support review.

**Typed failure diagnostic.**

Failures must identify `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `test_oracle`, `view_model`, `debug_quarantine`, `replay`, or the specific action/scheduler layer under test.

**Fail condition.**

A fixture corpus that proves only friendly paths, uses display strings instead of typed provenance/diagnostics, treats golden bytes as semantic proof, lacks direct-dispatch negatives, or lacks hidden-truth adversarial cases fails this line.

---

### 7.9 P0-09 — Failure diagnostics name responsible layer

**P0-CERT proof requirement.** Failure diagnostics name the responsible layer: authoring, schema validation, actor-known context, candidate generation, planning, proposal construction, scheduling, action validation, event application, projection, TUI rendering, replay, or debug quarantine.

**Gates composed.** `P0-CERT`, `DIAG`, plus every gate whose failure path is under test.

**Concrete seams audited.**

- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/actions/report.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-core/src/replay/report.rs`
- `crates/tracewake-content/src/validate.rs`
- `crates/tracewake-core/src/controller.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-core/src/debug_reports.rs`

**Positive fixtures and tests.**

- `routine_blocked_diagnostic_001.rs` if exercised;
- `method_fallback_requires_new_trace_or_stuck_001.rs`;
- `work_block_failed_then_sleep_succeeds_001.rs` if exercised;
- `no_human_day_001.rs` and no-human capstone diagnostics;
- `forbidden_provenance_input_fails_closed_001.rs`;
- content validation tests in `crates/tracewake-content/tests/forbidden_content.rs`;
- replay divergence/report tests in `crates/tracewake-core/tests/event_schema_replay_gates.rs`.

**Adversarial/negative fixtures and tests that must fail closed.**

- tests or mutants that replace typed blocker/diagnostic records with display strings, debug strings, substrings, labels, or unlayered `Err(String)`;
- hidden-truth audit negatives without banned-word dependence;
- mutation that reports the wrong responsible layer for hidden-truth, scheduler, validation, replay, projection, or debug-quarantine failures must be killed or remediated.

**Required event/replay/projection evidence.**

Where failure arises after proposal/event/replay seams, diagnostics must carry event/proposal/projection/checksum/replay identifiers. Where failure arises before proposal, diagnostics must record the context/candidate/planning/source-data identifiers available at that layer.

**Actor-known/provenance evidence.**

Diagnostics for actor-known failures must name expected input source, actual input source, holder-known context ID/hash/frontier, missing/forbidden/stale/wrong-kind provenance, and actor-visible output if any.

**Debug-quarantine evidence.**

Diagnostics must distinguish actor-visible output from debug-only output and must name debug leakage or debug exclusion explicitly where relevant.

**Typed failure diagnostic.**

The acceptance artifact must use the live responsible-layer names consistently: `doctrine`, `content_schema`, `content_validation`, `fixture_contract`, `holder_known_context`, `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `view_model`, `debug_quarantine`, `tui_input_binding`, `test_oracle`, plus any exact execution-layer names already accepted by live docs.

**Fail condition.**

Any failure artifact that merely says a test failed, relies on display text, omits responsible layer, omits expected/actual source, or cannot distinguish hidden truth excluded from hidden truth leaked fails this line.

---

### 7.10 P0-10 — Archived specs and tickets are cited only as history

**P0-CERT proof requirement.** Archived specs and tickets are cited only as history.

**Gates composed.** `P0-CERT`, `DIAG`, `FIXTURE`; this line protects all gates from historical-label overclaim.

**Concrete seams audited.**

- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md`
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md`
- `archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs` if present in the target tree;
- `crates/tracewake-core/tests/doc_invariant_references.rs` if present in the target tree;
- acceptance/review artifacts generated by the implementing session.

**Positive fixtures and tests.**

- acceptance artifact wording tests that classify archive references as historical;
- doc invariant/reference tests that prevent stale gate-code or spec-ledger wording;
- generated P0-CERT acceptance artifact sections `pending-historical` and `certification-use`.

**Adversarial/negative fixtures and tests that must fail closed.**

- wording or test fixtures that treat `0005`, `0006`, `0007`, `0008`, or any ticket as live certification;
- archive citations used as substitutes for event/replay/provenance/debug/diagnostic evidence;
- mutation that changes evidence-status `historical` to `pass` for archive-derived claims.

**Required event/replay/projection evidence.**

Archive history does not supply event/replay/projection evidence. If a historical fixture or report points to a behavior, the implementing session must reproduce that behavior from current exact-commit code and current acceptance commands, then cite the live witness separately.

**Actor-known/provenance evidence.**

Historical specs may explain why a holder-known seam exists. They cannot prove that current code assembles a correct context. The live context evidence must come from current code runs.

**Debug-quarantine evidence.**

Historical debug-hardening claims must not be counted as debug quarantine evidence. Current debug/embodied carrier census and live negatives are required.

**Typed failure diagnostic.**

Failures must identify `doctrine`, `documentation status`, or `test_oracle` where wording or artifact status overclaims, and the underlying implementation layer if an archive claim masks a real code failure.

**Fail condition.**

Any acceptance artifact, test name, report, doc, or future spec that treats an archived accepted spec/ticket as a live gate pass fails this line.

---

## 8. Acceptance / evidence artifact

The implementing session must produce a single P0-CERT acceptance artifact conforming to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. The recommended closeout path is:

`archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`

The exact path may follow repo closeout convention, but the artifact must include the fields below and must not claim pass/fail beyond evidence actually produced.

### 8.1 Required header

```markdown
# P0-CERT post-0008 baseline certification acceptance artifact

Spec: archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
Repository: joeloverbeck/tracewake
Target commit: 9f1622244c91c5952bd735da76f29fbe58f39f4b
Freshness claim: user-supplied target commit only; not independently verified as latest main
Verdict: <P0-CERT passed | P0-CERT scoped remediation>
Verdict scope: post-0008 baseline only
Archived evidence posture: historical only
EMERGE-OBS posture: observer-only, non-gating, non-threshold
```

`P0-CERT not applicable` is not a valid verdict for this implementation audit because this spec targets live code, tests, fixtures, debug, replay, and authoring surfaces.

### 8.2 Evidence-status field

Every evidence item must carry exactly one status value from the accepted template vocabulary and execution diagnostic standard:

- `pass` — the artifact actually certifies the checked claim;
- `fail` — the checked claim failed and requires remediation;
- `pending` — the check has not yet produced certifying evidence;
- `sampled` — representative, not exhaustive;
- `observer-only` — review evidence that cannot certify behavior;
- `historical` — archive/spec/ticket evidence used only as context.

`pending`, `sampled`, `observer-only`, and `historical` must never be silently counted as pass.

### 8.3 Fingerprint-scope field

Any fingerprint or stable artifact must declare scope:

- raw bytes;
- normalized serialization;
- parsed semantic content;
- command transcript;
- run seed;
- replay artifact;
- context hash/frontier;
- projection checksum;
- event log checksum.

This list extends the fingerprint-scope vocabulary of `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` with the P0-CERT-specific `context hash/frontier`, `projection checksum`, and `event log checksum` scopes; it does not replace the template's enumeration. A fingerprint is proof only for its declared scope. Byte-stable goldens do not prove semantic behavior without behavior witnesses.

### 8.4 Behavior-witness field

Each protected claim must include at least one behavior witness with:

- gate and P0 requirement line;
- exact file/seam and function/module where known;
- triggering input source: human, no-human scheduler, fixture load, replay rebuild, content validation, TUI binding;
- proposal ID/origin/source context where applicable;
- holder-known context ID/hash/frontier where applicable;
- checked facts and source/provenance status;
- accepted or rejected stage;
- event IDs, projection IDs/checksums, replay IDs/checksums where available;
- actor-visible output if any;
- debug-only output if any;
- hidden truth excluded or leaked;
- responsible layer and diagnostic record.

### 8.5 Replay / provenance field

The artifact must include a replay/provenance section with:

- fixture manifest fingerprint and schema version;
- run seed/random stream IDs;
- event log serialization fingerprint;
- live projection checksum;
- replay projection checksum;
- divergence report or explicit no-divergence report;
- actor-known context provenance rows for selected actions and rejected hidden-truth cases;
- debug-only truth/belief comparison rows labeled observer-only;
- at least one negative provenance case failing closed for missing, empty, dangling, wrong-kind, ambiguous, forbidden-source, and harness-fabricated provenance if the corpus has such cases.

### 8.6 Sampling / exhaustiveness field

The artifact must distinguish:

- exhaustive seam audits, such as source search for direct dispatch, forbidden hidden-truth APIs, debug-to-actor carriers, event append bypasses, unordered iteration, wall-clock/randomness entry points, and fixture schema forbidden fields;
- sampled/golden behavior witnesses, such as representative fixture families and TUI transcript scenarios;
- property/generative evidence, with recorded seeds and replayable failure artifacts (e.g. `crates/tracewake-core/tests/generative_lock.rs` with `crates/tracewake-core/tests/support/generative.rs`).

A sampled fixture family must not certify seams it did not exercise.

### 8.7 Pending / historical field

The artifact must include a section classifying:

- archived specs/tickets/reports as `historical` only;
- unavailable command runs, unsupported mutation execution, untriaged survived mutants, or incomplete property/generative evidence as `pending` or `fail`, never pass;
- remediation-required findings by gate and responsible layer.

### 8.8 Certification-use field

The artifact must explicitly state what later specs may cite:

- If verdict is `P0-CERT passed`: later specs may cite the artifact and name the certified gates consumed, but still must satisfy stricter entry gates for `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or `SECOND-PROOF-ENTRY` as applicable.
- If verdict is `P0-CERT scoped remediation`: only remediation specs addressing named failures are admissible until a replacement certification artifact passes.
- No later spec may cite `EMERGE-OBS` counters as gate pass/fail thresholds.
- No later spec may cite archived specs/tickets as live certification.

### 8.9 Staged-abstraction declaration field

For every staged abstraction, the artifact must declare:

```markdown
Staged abstraction: <name>
Evidence status: <observer-only | sampled | pass | pending | fail>
What it proves now: <specific checked behavior>
What it abstracts: <behavior not modeled/proved>
What it must not fake: <false evidence prohibited>
Future tier/feature it must not certify by implication: <named gate or deferred surface>
Diagnostic that fails if it leaks: <responsible layer and failure mode>
```

At minimum the artifact must include staged-abstraction declarations for:

- `EMERGE-OBS` — observer-only emergence ledger, not a gate and not a threshold, exercised by `crates/tracewake-core/tests/emergence_ledger.rs`;
- no-human canonical corpus sampling, if not exhaustive over all fixtures/seeds;
- mutation testing, if sampled, unsupported, or not exhaustive over all protected seams;
- TUI transcript evidence, if sampled rather than exhaustive over all carrier surfaces;
- temporal evidence in first-proof surfaces, which must not certify Phase-4 procedural time or second-proof LOD/time-acceleration by implication, consistent with `INV-112` (time may validate, but holder-known time must plan).

### 8.10 Required command/evidence transcript

The implementing session's acceptance artifact must include transcripts or equivalent machine-readable artifacts for:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo build --workspace --all-targets --locked`;
- `cargo test --workspace --locked`;
- the repository's lock-layer gate tests for anti-regression, hidden-truth, replay, content, and TUI seams (including `crates/tracewake-core/tests/anti_regression_guards.rs`, `crates/tracewake-core/tests/ci_workflow_guards.rs`, and `crates/tracewake-core/tests/spine_conformance.rs`);
- configured mutation testing using `.cargo/mutants.toml`, or a clearly labeled pending/remediation record if mutation execution is unavailable;
- replay/golden fixture runs and checksum outputs;
- content validation positive and negative runs;
- TUI possession/debug quarantine runs or transcript snapshots;
- static guard scans for forbidden direct dispatch, hidden-truth access, debug leakage, string-typed diagnostics, branch-on-human privilege, unordered iteration, wall-clock time, and unscoped randomness.

---

## 9. Invariants alignment

This spec coins no invariants. The implementing session must cite the exact invariant texts from `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` in the acceptance artifact. The cross-reference map below identifies which families are preserved by each audited gate.

| Audit line | Invariant families preserved | Required proof shape |
|---|---|---|
| P0-01 event/pipeline/replay | `INV-001`, `INV-009`–`INV-020`, `INV-112` (replay/scheduler clock may order and validate but is not cognition authority), plus replay/randomness/schema invariants in the event contract | event log authority, deterministic ordering, schema versioning, projection/replay equivalence, no current-state authority. |
| P0-02 actor-known autonomy | `INV-002`, `INV-006`, `INV-021`–`INV-031`, `INV-032`–`INV-041` | holder-known provenance, no truth-born cognition, explicit unknown/stale facts, typed beliefs/observations/memories/traces. |
| P0-03 human-origin ordinary rules | `INV-004`, `INV-005`, `INV-007`, `INV-008`, action parity invariants in `06` | possessed input binds to ordinary actor and ordinary actions; no player omniscience or privileged body. |
| P0-04 possession non-reset | `INV-005`, `INV-006`, `INV-008`, TUI/debug invariants in `08` | pre/post possession state/context fingerprints and carrier census. |
| P0-05 scheduler/no-human | `INV-003`, `INV-004`, `INV-032`–`INV-041`, ordinary-life/survival invariants in `06` | no-human ordinary actions via transaction/pipeline; passive accounting typed and replayable; no scheduler-authored primitive actions. |
| P0-06 validation truth boundary | `INV-001`, `INV-002`, `INV-021`–`INV-031`, event/action invariants | truth validates accept/reject/mutate through events only; it never plans or reveals hidden facts. |
| P0-07 debug quarantine | `INV-008`, debug/actor-visible split invariants in `08`, observer-only authoring boundary including `INV-111` when `EMERGE-OBS` appears | actor-visible surfaces are holder-known subsets; debug/observer-only evidence cannot feed world. |
| P0-08 fixture corpus | no-prose/no-scripting/no-outcome-chain invariants including `INV-058`–`INV-060`, content/fixture invariants from `09` and `12` | positive and negative fixtures, content validation, missing-property proof without authorial outcome chains. |
| P0-09 diagnostics | typed-claim/provenance/diagnostic invariants across `INV-021`–`INV-041` and execution `DIAG` | responsible-layer diagnostics with expected/actual sources and behavior witnesses. |
| P0-10 historical-only archive use | authority-order invariants and replacement doctrine | archived specs/tickets marked historical; live proof comes from current exact-commit code and artifacts. |

If the implementing session discovers an implementation behavior that would require weakening or reinterpreting an invariant, the correct outcome is not certification. It is a doctrine mismatch and must be escalated before code is treated as accepted.

---

## 10. Out of scope and tolerated deferrals

The audit scope is the post-0008 baseline only: spine, epistemic substrate, ordinary-life proof, possession/TUI/debug split, content validation, fixtures, diagnostics, CI/lock layer, and replay. The following surfaces are explicitly deferred and must not be certified by implication.

| Deferred surface | Named entry gate / boundary | What P0-CERT may say | What P0-CERT must not certify |
|---|---|---|---|
| Institutions, records, reports, wrong suspicion, local procedures | `PHASE-4-ENTRY` | Debug leakage must not feed records/institutions; current baseline may block entry until gates pass. | No institutional behavior, wrong-suspicion mechanics, records procedure, reports, or local norms certification. |
| Notices, travel, regional scale, LOD expansion, story-sifting projections | `SECOND-PROOF-ENTRY` | Current baseline may prove replay/provenance discipline that future work must preserve. | No travel, regional routing, LOD ancestry, story-sifting, long simulation, or second-proof certification. |
| LLM dialogue and speech surfaces | deferred LLM/speech boundary in foundation/architecture | Debug/LLM text must not become authority or actor knowledge in baseline tests. | No LLM integration, dialogue generation, persuasion, testimony, or language-surface certification. |
| Inventory/economy/domain-pack/practical-bias/fairness/budget expansion | future expansion gates after baseline certification | Diagnostics may record observer-only counters if current corpus produces them. | No economy quantity proof, fairness formula, practical-bias formula, budget threshold, or domain-pack bias certification. |
| `EMERGE-OBS` emergence ledger | observer-only obligation in execution `10` | Include rows when canonical corpus is exercised; label observer-only and replay-derived. | Never pass/fail on ledger values; never feed cognition, scheduling, validation, authoring, content selection, LOD promotion, or pacing. |
| Temporal-firewall beyond first-proof surfaces | `INV-112`; staged declarations in execution `03`/`10` | Prove first-proof temporal evidence where current corpus exercises it. | No Phase-4 procedural-time certification or second-proof LOD/time-acceleration certification. |

---

## 11. Risks and open questions

### 11.1 Audit risks

1. **Friendly-only fixture relapse.** A golden that proves the happy path while leaving hidden-truth planning possible is not certification.
2. **Label-implied pass.** A file, spec, fixture name, or archived acceptance report can suggest a gate is solved without live evidence. The artifact must resist this explicitly.
3. **Debug leakage through tests.** Tests can accidentally use debug truth as the oracle for actor-known behavior. The artifact must distinguish debug-only review from actor-visible knowledge.
4. **String-typed diagnostics.** Display labels, substrings, and `Err(String)`-style failures can masquerade as typed responsible-layer diagnostics. Typed records and layer names are required.
5. **Scheduler shortcut drift.** No-human advancement is the highest-risk seam for primitive action emission from needs/routines/truth. It needs both static and behavior-witness evidence.
6. **Validator fallback temptation.** Validation truth can safely reject; it must not become fallback planning or hidden hint generation.
7. **Replay byte fetish.** Stable golden bytes or checksums can prove serialization stability while semantic behavior diverges. Replay evidence must include behavior witnesses.
8. **Mutation overclaim.** Mutation testing is useful but not enough. A green mutation run does not prove actor-known provenance unless the mutants cover that seam and the behavior witnesses show it.
9. **Exact-commit contamination.** Branch labels, default-branch metadata, connector namespaces, and old repository names are not proof. Exact URL source discipline must persist into implementation.
10. **`EMERGE-OBS` threshold creep.** The emergence ledger is review data. Turning it into a drama quota would violate the no-scripting/no-human-focus boundaries.

### 11.2 Open questions to carry, not invent

1. **Mutation breadth.** The docs require lock-layer posture; the implementing session must decide whether configured `cargo-mutants` coverage is exhaustive enough for all protected seams or whether any survived/uncovered critical mutants force remediation.
2. **Legacy helper disposition.** Static reading sees content/schema seams that may include legacy helper names. The audit must decide from behavior and validation evidence whether any helper can create blanket knowledge or whether it is safely constrained.
3. **TUI transcript sufficiency.** The implementing session must decide whether current TUI transcript tests cover all actor-visible carriers or whether additional carrier-census evidence is needed.
4. **Property/generative corpus size.** The audit must record seed counts and sampling scope without inventing a fairness/budget threshold.
5. **CI parity.** The audit must verify whether the workflow commands exactly cover local gate commands, lock-layer tests, and mutation posture, and record any workflow drift as remediation.
6. **Acceptance artifact path.** This spec recommends an archive report path, but repo closeout convention owns the final artifact location.
7. **Pass-with-scoped-remediation boundary.** If a gate mostly passes but one seam fails, the owner must choose whether to produce `P0-CERT scoped remediation` immediately or split evidence into a remediation spec plus re-certification artifact. This spec does not weaken the gate to avoid that decision.

---

## 12. Self-check for implementers

Before closing this spec, the implementing session must confirm:

- [ ] Every exact source path used for evidence is present in the manifest and fetched from `joeloverbeck/tracewake` at `9f1622244c91c5952bd735da76f29fbe58f39f4b` or from an exact exported tree supplied by the user.
- [ ] No branch-name fetch, default-branch lookup, repository metadata, code search, snippets, prior chat memory, or connector namespace label was used as content proof.
- [ ] All ten P0-CERT proof requirements in Section 7 have evidence-status entries.
- [ ] Every canonical gate and first-proof acceptance label is composed only as a cross-reference; no new gate code/status vocabulary is minted.
- [ ] `EMERGE-OBS` is observer-only, non-gating, non-threshold, replay-derived, and quarantined from simulation inputs.
- [ ] Positive and negative fixtures both ran, and negatives failed for the intended responsible layer.
- [ ] Event/replay/projection evidence includes semantic behavior witnesses, not only bytes/checksums.
- [ ] Actor-known context evidence includes provenance, freshness/staleness, source IDs, and hidden-truth exclusion.
- [ ] Debug quarantine evidence includes actor-visible/debug/transcript/observer carrier separation.
- [ ] Diagnostics name responsible layer, expected input source, actual input source, actor-visible output, debug-only output, hidden truth excluded/leaked, replay divergence if any, and remediation category.
- [ ] Archived specs/tickets/reports are labeled historical only.
- [ ] Deferrals are tied to `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, or observer-only obligations as appropriate.
- [ ] No pass/fail result relies on this spec's existence rather than the generated acceptance artifact.

## Outcome

Completed: 2026-06-16

This spec produced
`archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`
and the ticket series `0036P0CERPOS0008-001` through
`0036P0CERPOS0008-012`. The acceptance artifact renders the verdict
`P0-CERT scoped remediation`, not `P0-CERT passed`, scoped to the post-0008
baseline only.

What changed:

1. The artifact now contains evidence-status rows for all ten P0-CERT proof
   requirements, replay/provenance and sampling sections, pending/historical
   classification, certification-use rules, staged-abstraction declarations,
   tolerated deferrals, and a completed implementer self-check.
2. All tickets in the series were closed and moved to `archive/tickets/`.
3. The artifact records remediation finding `0036-MUTATION-REMEDIATION-001` for
   the untriaged missed mutant emitted by the configured mutation attempt:
   `crates/tracewake-core/src/projections.rs:336:5 replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]`.

Deviation from original plan:

The gate evidence rows P0-01 through P0-10 passed, and
`cargo test --workspace --locked` passed, but the configured mutation posture did
not produce certifying pass evidence. The artifact therefore chose the spec's
allowed `P0-CERT scoped remediation` verdict and blocks later specs from citing
this artifact as `P0-CERT passed` until a replacement certification artifact
passes after remediation or equivalent/non-critical triage.

Verification:

1. `cargo test --workspace --locked` passed on 2026-06-16.
2. The capstone gate enumeration found no missing P0-01 through P0-10 markers.
3. The rendered verdict grep returned exactly
   `7:Verdict: P0-CERT scoped remediation`.
4. Archive format checks confirmed the completed tickets include `## Outcome`
   and `Completed:` markers.
