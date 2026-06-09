# 0005 Phase 1 Doc-Code Alignment Conformance and Anti-Drift Spec

**Status**: COMPLETED
**Intended staging path:** `specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md`  
**Final home after acceptance:** `archive/specs/0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md` (archived on acceptance alongside the prior Phase 1 remediation specs `0003`/`0004`; not promoted to the live `docs/4-specs/` tier)  
**Repository:** `joeloverbeck/tracewake`  
**Analyzed commit:** `b210e4069c1ec997ed839dca34840ac72058b477`  
**Spec posture:** `P0-CERT scoped remediation`

This is a Branch A deliverable. The audit found one genuine, material Phase 1 / Phase 1A doc-code alignment failure. The corrective direction is code-yields. No higher-tier document is identified as stale or wrong.

---

## 0. Mandatory exact-commit evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: b210e4069c1ec997ed839dca34840ac72058b477
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run.open exact raw/blob URLs; container.download exact raw URLs where accepted; exact GitHub blob URL fallback for line display on MIME-rejected raw Rust files
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/clippy.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/container_item_move_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/debug_attach_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/door_access_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/replay_item_location_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/view_filtering_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/src/validate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-content/tests/schema_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/accuseprobe.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/checkcontainer.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/inspect.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/movement.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/openclose.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/takeplace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/mutation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/location.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/time.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/acceptance_artifact_wording.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/doc_invariant_references.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/spine_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/readme_sample_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/reports/phase1-third-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/b210e4069c1ec997ed839dca34840ac72058b477/rust-toolchain.toml
Exact blob-display fallback URLs used for line inspection of raw files whose fetched text was tool-normalized:
- https://github.com/joeloverbeck/tracewake/blob/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/actions/pipeline.rs
- https://github.com/joeloverbeck/tracewake/blob/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/src/events/log.rs
- https://github.com/joeloverbeck/tracewake/blob/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-core/tests/spine_conformance.rs
- https://github.com/joeloverbeck/tracewake/blob/b210e4069c1ec997ed839dca34840ac72058b477/crates/tracewake-tui/tests/tui_seam_conformance.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

Line-reference note: for ordinary downloaded raw files, `file:line` references below are source-line references counted from exact raw content. For the few MIME/tool-normalized Rust files, the audit used the exact raw URL as content source and an allowed exact `github.com/.../blob/b210e4069c1ec997ed839dca34840ac72058b477/...` URL only to recover readable line displays. Those exceptions are explicitly called out.

---

## 1. Verdict

**Verdict: positive — one material Phase 1 / Phase 1A misalignment warrants a correction + anti-contamination spec.**

The aligned block is strong across the spine, TUI proof surface, replay, typed diagnostics, deterministic collections, debug quarantine, and CI lock-layer coverage. The material failure is narrower and therefore easy to underestimate: the Phase 1 content/action boundary is not phase-scoped. Later-phase action definitions are registered as `phase1_implemented: true`, and the content loader validates fixtures against an all-phase registry. That allows Phase 2A/3A action IDs to be accepted by the same content-loading seam used by Phase 1 fixtures without an explicit Phase 1 boundary failure.

This does **not** mean the later-phase code is audited or certified here. It means the shared Phase 1 content/schema/action-registry seam currently has no structural way to say, “this fixture/content package is Phase 1 and must reject later-phase actions.” Under live doctrine, archived Phase 2A/3A implementation is historical evidence, not certification, and a feature cannot proceed merely because code exists (`docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md:24-26`, `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md:20-23`, `docs/4-specs/SPEC_LEDGER.md:41-60`).

### Finding inventory

| Finding | Verdict | Material risk | Corrective direction | Upstream doc amendment |
|---|---:|---|---|---|
| `ALIGN-001` — Phase-scoped content/action registry boundary leak | Misaligned | Phase 1 fixture validation and action-registry parity can accept later-phase action IDs because the shared loader registers Phase 2A/3A actions and the registry represents them as Phase 1-implemented. This undermines the audit boundary and weakens `P0-CERT` evidence. | Code yields: make phase/certification scope explicit and structurally enforced. | None. The docs are consistent. |

---

## 2. Path and number confirmation

The live spec tier is intentionally compact: `docs/4-specs/README.md:9-18` lists `README.md`, `SPEC_LEDGER.md`, live `0001`, and the `0003` acceptance template. `docs/4-specs/SPEC_LEDGER.md:25-33` lists only `0001` as an active spec and treats `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` as a template, not a spec package. The same ledger states that archived implementation specs remain history and do not certify the implementation under current doctrine (`docs/4-specs/SPEC_LEDGER.md:41-60`).

This is a Phase 1 remediation spec in the same lineage as the archived Phase 1 hardening specs `archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` and `archive/specs/0004_PHASE_1_THIRD_HARDENING_AND_LOCK_LAYER_RE_AUDIT_SPEC.md`. Per the established convention, on acceptance it archives to `archive/specs/`; it is not promoted to the live `docs/4-specs/` tier (which carries only `0001` and the `0003` acceptance template). `0005_PHASE_1_DOC_CODE_ALIGNMENT_CONFORMANCE_AND_ANTI_DRIFT_SPEC.md` is therefore an acceptable staging number: it does not collide with a live spec. `archive/specs/` already holds a `0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md` from the parallel Phase 3A lineage, but per-lineage duplicate numbering is already the norm there (two each of `0002`/`0003`/`0004`), so `0005` continues the Phase 1 lineage cleanly. This spec declares the one allowed posture required by `docs/4-specs/README.md:31-39`: `P0-CERT scoped remediation`, because it changes code/tests/CI to repair a certification failure. It is not `P0-CERT not applicable`, because it affects simulation-adjacent validation, fixtures, and gates.

---

## 3. Authority chain and gate mapping

This spec is subordinate to, and operationalizes, these controlling documents:

| Tier | Controlling files used here | Binding point |
|---|---|---|
| Foundation | `docs/README.md`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`; `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`; `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`; `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`; `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`; `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Authority layering; constitutional invariants; event/replay; TUI/debug; no-script/no prose-born fact; first-playable gates; truth firewall. |
| Architecture | `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`; `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`; `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`; `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`; `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`; `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Kernel authority; content boundary; event sourcing; shared proposal pipeline; debug quarantine; acceptance artifacts. |
| Execution | `00_EXECUTION_INDEX_AND_AUTHORITY.md`; `01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`; `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`; `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`; `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`; `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`; `08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`; `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`; `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Gate posture, proof order, anti-contamination, no-direct-dispatch, data validation, golden fixture and diagnostics obligations. |
| Reference | `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`; `01_DESIGN_RISK_REGISTER.md`; `02_GLOSSARY.md` | Exact-source discipline, relapse risks, and terminology (`holder-known context` system-wide; `actor-known` for actor cases). |
| Spec | `docs/4-specs/README.md`; `SPEC_LEDGER.md`; `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`; `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` | Spec posture, source discipline, active-spec state, and scoped acceptance wording. |

Gate codes below are cross-references only. This spec does not redefine or weaken `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG`, `DATA-CERT`, or any other execution gate.

---

## 4. Scope and non-goals

### In scope

- Phase 1 / Phase 1A kernel spine: event log, event envelopes/schema versioning, replay, checksums, deterministic ordering/time, action proposal/validation pipeline, no-direct-dispatch, scheduler/no-human gate surface, authoritative state.
- Phase 1 / Phase 1A content seam: schema, deterministic load/canonicalization, content validation, no-script/no-prose-born-fact, fixture contract validation, action registry/affordance parity.
- Phase 1 / Phase 1A TUI seam: command loop, embodied view-model rendering, semantic action submission, transcripts, debug-panel quarantine.
- Structural lock layer: conformance tests, adversarial gates, negative fixtures, clippy profile, CI gate wiring.
- Shared seams only to the extent they affect Phase 1 / Phase 1A. Later-phase code is used as boundary evidence only.

### Non-goals

- No Phase 2A epistemic certification.
- No Phase 3A needs/routines/planner/no-human certification.
- No new mechanics, LLM surface, graphical client, institution expansion, regional/LOD advance, or backwards-compatibility shim.
- No local redefinition of higher-tier gate semantics.
- No amendment of foundation, architecture, execution, or reference doctrine inside this spec. Required upstream amendments, if any, would be recorded separately; this audit found none.

---

## 5. Conformance walk

### 5.1 Condensed conformance matrix

| Dimension | Controlling doctrine and invariants | Code-side evidence | Verdict |
|---|---|---|---|
| Authority layering and archive posture | `docs/README.md:3-14`; `docs/4-specs/SPEC_LEDGER.md:7-23`, `41-60`; `INV-098` (`docs/0-foundation/02...:423-425`) | Workspace keeps live docs separate from archive; CI lock-layer gates run exact named suites (`.github/workflows/ci.yml:64-89`). | Aligned, except `ALIGN-001` needs a new scoped remediation spec rather than treating historical Phase 2A/3A code as certified. |
| Crate dependency direction | Architecture says core owns authority and content/TUI may not invert dependencies (`docs/1-architecture/01...:7-23`, `38-50`). `INV-069`, `INV-098`. | Workspace has exactly core/content/TUI members (`Cargo.toml:1-7`); core has no dependencies (`crates/tracewake-core/Cargo.toml:1-7`); content depends only on core (`crates/tracewake-content/Cargo.toml:7-8`); TUI depends on core+content (`crates/tracewake-tui/Cargo.toml:7-9`). | Aligned. |
| Event log append-only and schema versioning | Meaningful mutation with no event is a bug (`docs/0-foundation/03...:28-43`); event schema evolution mandatory (`docs/0-foundation/02...:91-93` / `INV-020`); event envelope minimum and immutability (`docs/1-architecture/02...:32-55`). | Envelope registry defines `EventSchemaVersion::V1` and registry entries (`crates/tracewake-core/src/events/envelope.rs:9-30`); event envelope carries event type, schema version, stream/global order, causes, proposal/report IDs, random refs, payload, manifest, checksum (`envelope.rs:568-589`); append rejects unsupported schema and assigns global/stream positions before push (exact blob-display fallback `crates/tracewake-core/src/events/log.rs:553-667`; tests at blob-display `:756-810`). | Aligned. |
| Single mutation/application path | Architecture requires proposal→validation→event append→event application and forbids direct mutation (`docs/1-architecture/04...:7-20`, `23-42`). `INV-009`, `INV-010`, `INV-011`, `INV-098`. | State fields are `pub(crate)` BTree maps/sets (`crates/tracewake-core/src/state.rs:121-140`); mutation capabilities can only be minted inside `crate::events` (`events/mutation.rs:1-20`); application dispatches by event stream and mints world capability only inside apply (`events/apply.rs:41-69`, `113-135`). | Aligned. |
| Replay and projection rebuild | Replay must reconstruct from seed/config/events/schema versions and fail loudly where unsupported (`docs/0-foundation/03...:216-228`, `315-356`; `docs/1-architecture/02...:63-73`, `91-101`, `139-165`). `INV-018`, `INV-020`, `INV-092`. | Replay rebuilds from initial state plus log and records unsupported versions/errors (`crates/tracewake-core/src/replay/rebuild.rs:45-158`); event-log canonical serialization/deserialization round-trips by appending events (exact blob-display fallback `events/log.rs:625-663`, tests `:790-810`). | Aligned. |
| Checksum/deterministic coverage | Replay-safe features must be regression-tested (`INV-092`, `INV-098`); deterministic outcomes must avoid unordered iteration and hidden nondeterminism (`docs/2-execution/10...:97-110`). | Authoritative state uses BTree collections (`state.rs:1`, `121-140`); checksums traverse canonical physical and agent coverage (`crates/tracewake-core/src/checksum.rs:20-89`, `131-240`); CI runs event/replay gates (`.github/workflows/ci.yml:80-86`). | Aligned. |
| Deterministic time/order/randomness | Randomness must be seedable/auditable (`INV-017` at `docs/0-foundation/02...:79-81`); deterministic replay foundational (`INV-018`); hidden wall-clock/outcome nondeterminism is forbidden (`docs/1-architecture/02...:103-123`; `docs/2-execution/10...:97-110`). | `SimTick` is a discrete deterministic value (`crates/tracewake-core/src/time.rs:1-22`); passive deltas are deterministic (`time.rs:24-39`); `OrderingKey` is total/orderable and sorted deterministically (`scheduler.rs:51-92`); clippy disallows `HashMap`, `HashSet`, `SystemTime`, `Instant`, threads, process, fs reads, and network calls (`clippy.toml:1-16`); CI runs clippy with `-D warnings` (`.github/workflows/ci.yml:28-43`). | Aligned. |
| Shared action pipeline and no direct dispatch | Every world-affecting action must enter through ordinary proposal/shared pipeline; scheduler may not author primitive behavior (`docs/1-architecture/04...:7-20`, `73-92`, `154-165`); `INV-103`, `INV-104`, `INV-105`. | Pipeline has explicit stages including phase boundary, append, application, projection/debug (exact raw/blob fallback `actions/pipeline.rs:L0-L1` / blob-display `:3061-3158`, `:3202-3541`); typed reports carry failed stage/reason codes/actor-visible/debug fields (`actions/report.rs:9-87`, `205-220`). | Aligned for the single-pipeline/no-direct-dispatch shape; **misaligned for phase scoping** in `ALIGN-001`. |
| Content schema/load/validation | Content must pass schema/version/canonicalization/provenance/action registry parity/no-script/no-player/no-hidden-truth gates before runtime (`docs/2-execution/08...:25-43`, `63-84`, `98-116`); raw prose is not authoritative (`INV-022`); no authored outcome chains (`INV-060`, `INV-061`, `INV-062`, `INV-097`). | Schema has typed affordances (`crates/tracewake-content/src/schema.rs:273-277`) and canonicalization sorting (`schema.rs:412-472`); serializer clones and canonicalizes before output (`serialization.rs:53-67`); validator runs no-player/no-script/determinism/fixture gates (`validate.rs:123-142`); forbidden-content tests reject quest/reward/player/script/truth shortcuts (`forbidden_content.rs:28-99`). | Aligned for no-script/no-prose-born-fact; **misaligned for Phase 1 action-scope validation** in `ALIGN-001`. |
| Possession parity and TUI view boundary | TUI may not apply events/mutate/read hidden truth/maintain special player state (`docs/1-architecture/10...:17-20`); possession does not create player character (`docs/2-execution/07...:24-43`); `INV-065`–`INV-070`, `INV-094`, `INV-095`, `INV-108`. | Embodied render consumes `EmbodiedViewModel` and actor-visible why-not fields (`crates/tracewake-tui/src/render.rs:1-35`, `210-220`); transcript captures semantic action IDs, validation reports, debug sections separately (`tui/src/transcript.rs:19-87`); TUI seam conformance maps debug/view-model requirements to tests (exact blob-display fallback `crates/tracewake-tui/tests/tui_seam_conformance.rs:503-664`). | Aligned. |
| Debug quarantine and truth firewall | Debug may expose truth only as non-diegetic, structurally quarantined; debug output must not feed actor knowledge or tests as knowledge (`docs/2-execution/07...:57-82`, `99-112`); `INV-099`–`INV-108`, especially `INV-107`. | Debug capability cannot be minted externally and carries non-diegetic marker (`crates/tracewake-core/src/debug_capability.rs:1-39`); debug view models expose `debug_only()` and tests assert truth/belief mismatch is debug-only (`view_models.rs:493-646`, `829-938`); debug panel rendering tests assert no embodied checksum/affordance change (TUI conformance evidence at blob-display `tui_seam_conformance.rs:503-664`). | Aligned. |
| Typed diagnostics over strings | Diagnostics must be typed/structurally inspectable; display strings are not authority (`INV-105`; `docs/2-execution/10...:18-23`, `38-52`, `129-144`; `docs/2-execution/07...:73-82`). | `ReasonCode` enum has stable IDs including `PhaseUnsupportedAction` (`actions/report.rs:9-87`); `CheckedFact` stores typed stable keys/source (`actions/report.rs:155-203`); `ValidationReport` separates reason codes, checked facts, actor-visible/debug-only facts and summaries (`actions/report.rs:205-220`). | Aligned. |
| TUI transcript/replay stability | Transcripts are acceptance artifacts but text alone is not proof; tests should assert typed IDs and view-model structures (`docs/1-architecture/10...:90-103`). `INV-095`, `INV-098`. | Transcript captures command/view/debug sections from app APIs and semantic IDs (`tui/src/transcript.rs:19-87`); CI runs TUI adversarial and seam conformance suites (`.github/workflows/ci.yml:87-89`). | Aligned. |
| Structural lock layer in CI | Tests must include adversarial negative gates, not only friendly goldens (`docs/2-execution/01...:51-53`; `docs/2-execution/10...:24-36`, `129-144`). `INV-091`–`INV-098`. | CI runs fmt, clippy, build, test, and named lock-layer gates on pinned toolchain (`.github/workflows/ci.yml:13-16`, `20-26`, `28-43`, `45-62`, `64-89`; `rust-toolchain.toml:1-7`). | Aligned; must be extended for `ALIGN-001`. |
| Phase 2A/3A boundary | Code exists/history does not equal certification (`docs/2-execution/01...:24-37`, `55-64`; `docs/2-execution/03...:20-23`, `37-48`, `70-89`; `SPEC_LEDGER.md:41-60`). | Later-phase action constructors exist in the registry and are callable (`actions/registry.rs:111-147`); content loader currently opts them into validation (`content/load.rs:51-60`). | Boundary held in this audit scope; **the shared seam leaks boundary enforcement** (`ALIGN-001`). |

### 5.2 Detailed finding: `ALIGN-001` — Phase-scoped content/action registry boundary leak

#### Doctrine side

The live execution layer says historical Phase 3A code has landed but is not certified under the post-overhaul doctrine, and current code may inform audits but must not override foundation or architecture (`docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md:24-26`). It further states that exact Phase 3A file names, fixture implementations, and status language remain historical implementation details unless a future certified spec reintroduces them (`docs/2-execution/01...:55-64`). The phase ladder repeats that proof order, not code existence, controls advancement (`docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md:20-23`) and that historically landed phases are not certification (`docs/2-execution/03...:37-48`).

The data authoring execution contract requires content to pass action-registry/affordance parity, no outcome-chain authoring, no player/human privilege, no hidden-truth cognition fields, deterministic canonicalization, and writer-readable/programmer-actionable failures before runtime (`docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md:25-43`). The schema must make forbidden concepts unrepresentable or clearly invalid by structure, not merely by keyword rejection (`docs/2-execution/08...:63-84`). These doctrine requirements bear on `INV-098` feature acceptance (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:423-425`), `INV-060` no authored outcome chains (`:261-263`), `INV-061` authored causal machinery (`:265-267`), `INV-097` no-script compliance tests (`:419-421`), and the truth-firewall invariants `INV-099`–`INV-108` (`:431-451`) where the later-phase actions include epistemic and routine mechanics.

#### Code side

`ActionDefinition` represents the phase boundary as a public boolean named `phase1_implemented` (`crates/tracewake-core/src/actions/registry.rs:21-25`). Both constructors set that boolean to `true` unconditionally (`registry.rs:28-43`). The later-phase registration functions use those same constructors:

- Phase 2A epistemics: `check_container` and `truthful_accuse_probe` (`registry.rs:111-119`).
- Phase 3A ordinary-life routines/actions: `sleep`, `eat`, `work_block`, `continue_routine` (`registry.rs:121-147`).

The shared fixture loader `load_fixture_package` — currently the *only* loader, despite the Phase 1 framing — then builds a registry and registers all of those later-phase action families before validation (`crates/tracewake-content/src/load.rs:40-60`). This means even a genuine Phase 1 fixture is validated against an all-phase registry, and the existing fixture set already mixes phases (see `ALIGN-REQ-002`). The content schema models affordances only as an `ActionId` and target ID (`schema.rs:273-277`), with no fixture/content phase. The parity validator accepts any affordance whose action ID exists in the passed registry and whose target kind is supported (`validate.rs:866-903`). Routine step validation similarly accepts semantic actions if their base action maps to any registered action (`validate.rs:926-965`). The forbidden-content test helper repeats the same all-phase registry setup (`crates/tracewake-content/tests/forbidden_content.rs:15-25`), so the current negative tests prove no-script/truth-field rejection, but not Phase 1 rejection of later-phase action IDs.

The pipeline has a phase-boundary rejection stage and a typed `ReasonCode::PhaseUnsupportedAction`, but the guard is effectively unreachable for registered later-phase actions because the later-phase constructors set `phase1_implemented = true` (`actions/pipeline.rs` exact raw L1; exact blob-display fallback `actions/pipeline.rs:3140-3158`; `actions/report.rs:9-13`, `48-53`).

#### Why this is material

The risk is not that a later-phase fixture exists. Boundary-marker fixtures and modules may exist in the repository as historical implementation evidence. The risk is that a Phase 1 content load path and Phase 1 registry representation cannot structurally distinguish “Phase 1 fixture/action” from “later-phase historical fixture/action.” That creates a false proof surface: a Phase 1 validation pass can be broadened by convenience registration instead of a deliberate certification boundary.

This matters directly to `P0-CERT` evidence because `P0-CERT` must prove all world-affecting behavior enters through proposal/validation/event/replay boundaries, that archived specs remain history, that fixtures include adversarial gates, and that diagnostics name the responsible layer (`docs/2-execution/01...:65-80`). A Phase 1 fixture validator that admits later-phase action IDs without a typed phase boundary cannot produce a clean Phase 1 certification artifact.

#### Verdict and corrective direction

`ALIGN-001` is **misaligned**. Corrective direction is **code-yields**. The live docs are not stale here; they consistently require proof-order discipline, archived-spec humility, and structural validation boundaries.

---

## 6. Structural anti-contamination requirements

Each requirement below exists only to repair `ALIGN-001`. The existing lock layer should be extended rather than replaced.

### `ALIGN-REQ-001` — Replace `phase1_implemented: bool` with a typed phase/certification scope

**Requirement.** Replace `ActionDefinition.phase1_implemented: bool` with a structural scope model that cannot accidentally mark every registered action as Phase 1. Acceptable shapes include:

- `ActionScope::{Phase1, Phase2AHistorical, Phase3AHistorical, ...}` plus no public constructor that defaults to `Phase1`; or
- `PhaseScopedActionRegistry<Phase1Scope>` / `PhaseScopedActionRegistry<HistoricalScope>` type-state wrappers with phase-specific registration APIs; or
- a sealed trait/capability marker where Phase 1 action definitions can only be constructed by Phase 1 registration functions.

A boolean flag is not acceptable because it is too easy to set by convenience constructor and too hard for tests to prove complete coverage.

**Invariant/gate cross-references.** `INV-098`, `INV-097`, `INV-105`; `P0-CERT`, `SPINE-CERT`, `FIXTURE`, `DIAG` as cross-references only.

**Enforcement level.** Compile-time first. The desired end state is that Phase 1 loader code cannot compile if it calls a later-phase registration function or passes a later-phase action registry into Phase 1 fixture validation.

**Existing files to extend.** `crates/tracewake-core/src/actions/registry.rs`; `crates/tracewake-core/src/actions/pipeline.rs`; `crates/tracewake-core/tests/spine_conformance.rs`; `crates/tracewake-core/tests/anti_regression_guards.rs`.

### `ALIGN-REQ-002` — Make content validation phase-scoped before runtime

**Requirement.** Add a typed content/fixture scope to the content crate. A Phase 1 fixture load must use an explicitly Phase 1 registry and reject any later-phase action IDs in affordances or routine semantic actions before runtime. Later-phase loaders may opt into later-phase actions only in later-phase contexts; this spec does not certify those later-phase contexts.

**Each fixture must declare its phase scope as parsed data, not inferred.** Add a structural `FixtureScope` (e.g. `FixtureScope::{Phase1, Phase2AHistorical, Phase3AHistorical}`) carried on the fixture/manifest schema so illegal states are unrepresentable rather than checked after the fact (this promotes the §11 appendix suggestion #3, “parse, don't validate,” from advisory to requirement). The loader selects the registry from the declared scope: a `Phase1` fixture loads under the Phase 1 registry and rejects later-phase action IDs; a later-phase fixture loads under the corresponding historical registry as boundary evidence only, not certification.

**Migration inventory (load-bearing).** Because `load_fixture_package` is currently the only loader and registers all phases, the existing fixture set already mixes phases. As measured against the analyzed commit, of 28 fixture files the following 15 exercise later-phase actions (`eat`/`sleep`/`work_block`/`continue_routine`/`check_container`/`truthful_accuse_probe`) and MUST be declared a later-phase `FixtureScope` so they keep validating and running rather than being rejected by the new Phase 1 boundary: `no_human_day_001`, `ordinary_workday_001`, `sleep_eat_work_001`, `planner_trace_001`, `routine_no_teleport_001`, `routine_blocked_diagnostic_001`, `food_unavailable_replan_001`, `expectation_contradiction_001`, `knowledge_blocker_accuse_001`, `no_hidden_truth_planning_001`, `no_human_epistemic_check_001`, `possession_parity_001`, `possession_does_not_reset_intention_001`, `sound_uncertainty_001`, `view_filtering_001`. The `golden_fixtures_run`, `fixtures_load`, and no-human harnesses iterate `fixtures::all()` and load several of these by name through `load_fixture_package`; each call site must load its fixture under that fixture's declared scope, and the no-human day proof (`no_human_day_001`) must continue to pass. Implementers MUST re-derive this list against the tree at implementation time (grep the fixtures directory for the later-phase action IDs) rather than trusting this snapshot.

The rejection must be typed. Do not rely on display text such as “phase3a” or substring matching. Add or reuse a structured validation phase such as `ValidationPhase::ActionRegistryParity` with a specific error code like `phase_unsupported_action`, or introduce `ValidationPhase::PhaseBoundary` if that is cleaner.

**Invariant/gate cross-references.** `INV-060`, `INV-061`, `INV-097`, `INV-098`, `INV-105`; `FIXTURE`, `DATA-CERT`, `DIAG` as cross-references only.

**Enforcement level.** Compile-time where practical, runtime validation as defense in depth.

**Existing files to extend.** `crates/tracewake-content/src/load.rs`; `crates/tracewake-content/src/validate.rs`; `crates/tracewake-content/src/schema.rs`; `crates/tracewake-content/src/fixtures/mod.rs` and the per-fixture files in `crates/tracewake-content/src/fixtures/` (to carry each fixture's declared `FixtureScope`); `crates/tracewake-content/tests/forbidden_content.rs`; `crates/tracewake-content/tests/schema_conformance.rs`; `crates/tracewake-content/tests/fixtures_load.rs`; `crates/tracewake-content/tests/golden_fixtures_run.rs` (load each fixture under its declared scope).

### `ALIGN-REQ-003` — Add adversarial Phase 1 negative fixtures proving later-phase action rejection

**Requirement.** Add at least one Phase 1 negative fixture that attempts to include each later-phase action family in a Phase 1 content package:

- `check_container`
- `truthful_accuse_probe`
- `sleep`
- `eat`
- `work_block`
- `continue_routine`

The fixture must fail at content validation with typed phase-boundary diagnostics, not at runtime after a pipeline proposal. The test must assert the typed `ValidationPhase` and stable error code, not a display string.

**Invariant/gate cross-references.** `INV-097`, `INV-098`, `INV-105`; `FIXTURE`, `DIAG`, `P0-CERT` as cross-references only.

**Enforcement level.** Adversarial runtime test plus negative fixture. Prefer a compact generated table test over six copy-paste fixtures if it still verifies each action family individually.

**Existing files to extend.** `crates/tracewake-content/tests/forbidden_content.rs`; optionally `tests/negative-fixtures/phase1_rejects_later_phase_actions/` if compile-fail or package-level negative fixtures are used by `negative_fixture_runner`.

### `ALIGN-REQ-004` — Make the pipeline phase-boundary guard reachable and covered

**Requirement.** Preserve the pipeline-level `PhaseBoundaryValidation`/`ReasonCode::PhaseUnsupportedAction` defense, but make it reachable in at least one adversarial test by constructing or obtaining an action definition that is registered but outside the active phase. If `ALIGN-REQ-001` moves phase scope to type-state and makes this impossible for Phase 1 code, then the pipeline test should prove that impossibility at the compile-fail boundary and keep a narrow internal unit test for the typed rejection path.

**Invariant/gate cross-references.** `INV-098`, `INV-105`; `NO-DIRECT`, `SPINE-CERT`, `DIAG` as cross-references only.

**Enforcement level.** Compile-time phase scoping plus one runtime negative guard proving typed diagnostics where an out-of-phase action reaches validation in a non-Phase1 test context.

**Existing files to extend.** `crates/tracewake-core/src/actions/pipeline.rs`; `crates/tracewake-core/src/actions/report.rs` only if a new reason code is needed; `crates/tracewake-core/tests/spine_conformance.rs` or `acceptance_gates.rs`.

### `ALIGN-REQ-005` — Add CI lock-layer coverage for the new boundary

**Requirement.** Extend the existing lock-layer CI job so the new Phase 1 action-scope gates run in named suites. The CI job already runs named strengthened gates (`.github/workflows/ci.yml:64-89`); add the new content/core tests there if they are not already covered by `cargo test --workspace --locked`.

**Invariant/gate cross-references.** `INV-091`–`INV-098`; `P0-CERT`, `FIXTURE`, `DIAG` as cross-references only.

**Enforcement level.** CI gate. This is secondary to type-state/sealed construction but mandatory for audit visibility.

**Existing files to extend.** `.github/workflows/ci.yml`; `crates/tracewake-content/tests/forbidden_content.rs`; `crates/tracewake-core/tests/spine_conformance.rs` or `acceptance_gates.rs`; `crates/tracewake-core/tests/doc_invariant_references.rs` if requirement-to-invariant coverage is encoded there.

### `ALIGN-REQ-006` — Add a source-level regression guard only as defense in depth

**Requirement.** Add a structural/source guard that fails if `load_fixture_package` or the Phase 1 fixture loader directly calls `register_phase2a_*` or `register_phase3a_*`, and fails if a generic action constructor hard-codes Phase 1 scope for non-Phase1 registrations. This guard must be negative-fixture-backed or otherwise self-tested. It must not be the only enforcement mechanism.

**Invariant/gate cross-references.** `INV-097`, `INV-098`, `INV-105`; `FIXTURE`, `DIAG` as cross-references only.

**Enforcement level.** Source/test guard only, explicitly subordinate to compile-time phase typing.

**Existing files to extend.** `crates/tracewake-core/tests/anti_regression_guards.rs`; `crates/tracewake-core/tests/negative_fixture_runner.rs`; `tests/negative-fixtures/...` if this repository keeps source-scanner negative fixtures there.

---

## 7. Required fixtures and tests

### Positive gates

| Gate | Required positive evidence |
|---|---|
| Phase 1 loader accepts Phase 1 fixtures | A known Phase 1 fixture such as movement/open/close/take/place/inspect/wait loads under `FixtureScope::Phase1` and produces the same canonical manifest/checksum as before, except for expected identifier changes. |
| Phase 1 registry exposes only Phase 1 actions | A test enumerates Phase 1 registry action IDs and proves it contains only `move`, `open`, `close`, `take`, `place`, `look`, `inspect_place`, `inspect_entity`, and `wait`, or the exact current Phase 1 list if intentionally changed by a higher-tier/live spec. |
| Later-phase historical fixture path remains explicit | A later-phase fixture can still be loaded only through an explicit historical/later-phase test helper or later-phase scope, with test names and comments saying it is boundary evidence, not Phase 1 certification. |
| Pipeline accepts valid Phase 1 actions | Existing movement/openclose/takeplace/wait pipeline tests still pass and still append/apply events through the shared pipeline. |

### Adversarial / negative gates

| Gate | Required negative evidence |
|---|---|
| Phase 1 content rejects Phase 2A actions | A Phase 1 fixture containing `check_container` and `truthful_accuse_probe` fails before runtime with typed phase-boundary diagnostics. |
| Phase 1 content rejects Phase 3A actions | A Phase 1 fixture containing `sleep`, `eat`, `work_block`, or `continue_routine` fails before runtime with typed phase-boundary diagnostics. |
| Routine semantic action cannot smuggle later phase | A Phase 1 routine step whose semantic base maps to a later-phase action fails with the same typed boundary code; it must not pass merely because the action exists in an all-phase registry. |
| Pipeline phase-boundary diagnostic is covered | A registered-but-out-of-scope action in a non-Phase1/internal test context yields `PipelineStage::PhaseBoundaryValidation` and `ReasonCode::PhaseUnsupportedAction`, or a compile-fail test proves that such a value cannot be constructed across the Phase 1 API. |
| Guard self-coverage | A negative fixture or mutation of a tiny test crate demonstrates that direct Phase 1 loader calls to `register_phase2a_*`/`register_phase3a_*` are caught by the source/CI guard. |
| No display-string authority | Assertions inspect typed `ValidationPhase`, stable error code, `ReasonCode`, or phase-scope type. They must not assert only that a message contains “phase”. |

---

## 8. Required upstream doc amendments

None.

The documentation is not the stale party for `ALIGN-001`. Live doctrine consistently says implementation convenience yields to accepted gates (`docs/README.md:14`), historical later-phase implementation does not certify current gates (`docs/2-execution/01...:24-37`, `55-64`), and future work must follow proof order rather than code-existence order (`docs/2-execution/03...:20-23`).

---

## 9. Acceptance checklist

Do not mark this spec accepted until all mandatory gates pass on the implementation commit that remediates `ALIGN-001`.

### Workspace gates

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`

### New/extended lock-layer gates

- `cargo test --locked -p tracewake-content --test forbidden_content` includes Phase 1 rejection of later-phase action IDs.
- `cargo test --locked -p tracewake-content --test schema_conformance` or equivalent proves fixture/content phase scope is represented structurally.
- `cargo test --locked -p tracewake-core --test spine_conformance` or equivalent proves action registry phase scope and pipeline phase-boundary coverage.
- `cargo test --locked -p tracewake-core --test anti_regression_guards` or equivalent proves Phase 1 loader cannot directly register later-phase action families.
- `.github/workflows/ci.yml` runs the new/extended tests in the lock-layer job, unless the maintainers intentionally rely on `cargo test --workspace --locked` and document why the named lock-layer suite does not need a new line.

### Per-requirement acceptance evidence

| Requirement | Responsible layer | Evidence required | Result field for acceptance artifact |
|---|---|---|---|
| `ALIGN-REQ-001` | `core/actions/registry` | Typed action scope or phase-scoped registry; no constructor that makes later-phase actions Phase 1 by default; compile-fail or API-boundary test. | `<pass/fail>` |
| `ALIGN-REQ-002` | `content/load`, `content/validate`, `content/schema`, `content/fixtures` | Each fixture declares a structural `FixtureScope`; Phase 1 loader uses Phase 1 scope only and rejects later-phase action IDs before runtime with typed diagnostics; the 15 pre-existing later-phase fixtures retain their `golden_fixtures_run`/`fixtures_load`/no-human runs under their declared later-phase scope. | `<pass/fail>` |
| `ALIGN-REQ-003` | `content/tests`, fixtures | Adversarial Phase 1 fixture(s) cover Phase 2A and Phase 3A action IDs individually. | `<pass/fail>` |
| `ALIGN-REQ-004` | `core/actions/pipeline`, `core/tests` | Phase-boundary pipeline guard is reachable and typed, or impossible across Phase 1 API and covered by compile-fail. | `<pass/fail>` |
| `ALIGN-REQ-005` | `workspace/ci` | CI lock-layer job runs the new/extended gates on the pinned toolchain. | `<pass/fail>` |
| `ALIGN-REQ-006` | `core/tests/anti_regression_guards`, negative fixtures | Source-level guard catches direct later-phase registration from Phase 1 loader, with guard self-coverage. | `<pass/fail>` |

### Scoped certification wording

Allowed wording after remediation passes:

> Phase 1 / Phase 1A doc-code alignment scoped remediation accepted for exact commit `<commit_after_fix>`. This contributes scoped evidence toward `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `FIXTURE`, and `DIAG`; it does not certify latest main, later-phase scope, or the full project.

Forbidden wording:

- “Tracewake is fully certified.”
- “Latest main was independently verified.”
- “Phase 2A / Phase 3A systems are certified by this pass.”
- “Archived specs are live authority.”
- “Project is P0 certified.”
- “`SPINE-CERT` passed.”
- “The later-phase action families are Phase 1 because the loader accepts them.”

---

## 10. Boundary demonstration: Phase 2A/3A was not audited here

The audit read later-phase modules and fixtures only to locate shared seams and avoid false positives. `crates/tracewake-core/src/epistemics/**`, `crates/tracewake-core/src/agent/{need,routine,htn,intention,candidate,generation,methods,planner}.rs`, and later-phase action defs such as `eat`, `sleep`, `work`, `checkcontainer`, `continue_routine`, and `accuseprobe` are not certified by this spec.

The only later-phase references that drive a requirement are the places where those later-phase action IDs enter the Phase 1 content/action boundary: registry registration (`actions/registry.rs:111-147`), Phase 1 loader all-phase registration (`content/load.rs:51-60`), action affordance parity (`content/validate.rs:866-903`), routine semantic action parity (`content/validate.rs:926-965`), and the all-phase forbidden-content test helper (`content/tests/forbidden_content.rs:15-25`). That is a shared-seam problem, not a later-phase mechanics audit.

---

## 11. Additional suggestions appendix — not in-scope requirements

These suggestions are intentionally not requirements for this scoped remediation. They are useful next steps for a future maintainer once `ALIGN-001` is fixed.

## Outcome

Completed on 2026-06-09.

Changes:

1. Replaced the core action-registry Phase 1 boolean with typed `ActionScope` metadata, explicit registration scopes, and an active-scope pipeline guard.
2. Added structural `FixtureScope` content declarations, canonical serialization support, phase-keyed fixture registry construction, and typed content validation diagnostics for out-of-scope actions.
3. Migrated all existing fixtures to explicit Phase 1, Phase 2A historical, or Phase 3A historical scope.
4. Added adversarial Phase 1 negative content coverage for each later-phase action family and routine-step smuggling.
5. Added runtime phase-boundary coverage asserting `PipelineStage::PhaseBoundaryValidation` and `ReasonCode::PhaseUnsupportedAction`.
6. Added a source-level loader-registration guard with mutation self-coverage, explicitly secondary to `ActionScope`/`FixtureScope` typing.
7. Extended the CI lock-layer job with named `acceptance_gates` and `schema_conformance` runs.
8. Archived completed tickets `0005PHA1DOCCOD-001` through `0005PHA1DOCCOD-006`.

Deviations:

1. Pipeline guard reachability was covered in `acceptance_gates.rs` rather than `spine_conformance.rs`, because the former owns end-to-end pipeline acceptance behavior.
2. The source guard self-coverage uses an in-memory mutated loader snippet rather than a standalone Cargo negative fixture, matching the repository's source-scan guard style while still proving the guard fails on the targeted mutation.
3. No new `ReasonCode` was needed; the existing `PhaseUnsupportedAction` reason code matched the scoped remediation contract.

Acceptance evidence:

| Requirement | Result | Evidence |
|---|---|---|
| `ALIGN-REQ-001` | PASS | `crates/tracewake-core/src/actions/registry.rs`; `cargo test --locked -p tracewake-core --test anti_regression_guards` |
| `ALIGN-REQ-002` | PASS | `crates/tracewake-content/src/schema.rs`; `crates/tracewake-content/src/load.rs`; `cargo test --locked -p tracewake-content --test fixtures_load`; `cargo test --locked -p tracewake-content --test schema_conformance`; `cargo test --locked -p tracewake-content --test golden_fixtures_run` |
| `ALIGN-REQ-003` | PASS | `cargo test --locked -p tracewake-content --test forbidden_content` |
| `ALIGN-REQ-004` | PASS | `cargo test --locked -p tracewake-core --test acceptance_gates` |
| `ALIGN-REQ-005` | PASS | `.github/workflows/ci.yml`; `grep -nE "forbidden_content|schema_conformance|spine_conformance|acceptance_gates|anti_regression_guards|negative_fixture_runner" .github/workflows/ci.yml` |
| `ALIGN-REQ-006` | PASS | `cargo test --locked -p tracewake-core --test anti_regression_guards`; `cargo test --locked -p tracewake-core --test negative_fixture_runner` |

Scoped certification:

Phase 1 / Phase 1A doc-code alignment scoped remediation accepted for exact commit `115fc07`. This contributes scoped evidence toward `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `FIXTURE`, and `DIAG`; it does not certify latest main, later-phase scope, or the full project.

Verification:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace`

1. **Doctrine-to-code conformance harness.** Build a living traceability matrix mapping each load-bearing invariant (`INV-###`) and gate code to executable tests, compile-fail fixtures, CI jobs, or explicitly review-only artifacts. Treat it as an architecture fitness function: a continuously run check that preserves architectural characteristics over time. Prior art: ThoughtWorks describes architectural fitness functions as objective integrity assessments run continually; ArchUnit shows how architectural rules can be encoded and checked in test suites. [EXT-1], [EXT-2]
2. **Executable spec registry.** Maintain a machine-readable list of doctrine mandates with owner doc, line range, invariant IDs, responsible crate/module, and named test. This should extend, not replace, `doc_invariant_references`.
3. **Prefer “parse, don’t validate” for phase scope.** Phase 1 content/action scope should be represented as parsed/typed data so illegal states are unrepresentable, not merely checked after the fact. [EXT-5] **(Promoted from advisory to a requirement — see `ALIGN-REQ-002`, which mandates a declared per-fixture `FixtureScope`.)**
4. **Use Clippy disallowed APIs as a secondary guard, not primary proof.** The existing `clippy.toml` is valuable for nondeterminism and side-effect hazards; Clippy’s `disallowed_methods` and `disallowed_types` lints are designed for configured bans. They remain lint/test-level controls and should be backed by type/module boundaries where a doctrine invariant is critical. [EXT-3]
5. **Keep deterministic collections explicit.** Rust’s `BTreeMap` iteration is ordered by key, making it a good fit for replay/checksum-sensitive traversal when paired with canonical serialization. This is already reflected in `state.rs` and `clippy.toml`; future harness work should preserve it. [EXT-4]
6. **Mutation-test the guards.** For source scanners and invariant linters, include negative fixtures that intentionally violate the rule and prove the guard fails. This keeps grep-style checks from becoming ceremonial.

### External references

- [EXT-1] ThoughtWorks Technology Radar, “Architectural fitness function,” https://www.thoughtworks.com/radar/techniques/architectural-fitness-function
- [EXT-2] ArchUnit User Guide, rule/layered architecture testing, https://www.archunit.org/userguide/html/000_Index.html
- [EXT-3] Rust Clippy lint documentation, `disallowed_methods` / `disallowed_types`, https://rust-lang.github.io/rust-clippy/master/index.html
- [EXT-4] Rust standard library documentation, `std::collections::BTreeMap`, https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
- [EXT-5] Alexis King, “Parse, don’t validate,” https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

---

## 12. Self-check result

- Exact-commit URL discipline followed; no clone, branch fetch, default-branch lookup, repository metadata, or code search used.
- Manifest used as path inventory only.
- No contamination by a different repository was observed.
- The form matches the verdict: Branch A correction + anti-contamination spec.
- Scope held to Phase 1 / Phase 1A + lock layer. Later-phase code is boundary evidence only.
- Each material finding cites doc-side and code-side evidence and the relevant invariants.
- Corrective direction is code-yields; no upstream amendment is required.
- Anti-contamination lock is structural first, test/CI second, source-scanner third.
- Gate codes are cross-references only; no gate semantics are redefined.
