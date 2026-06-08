# 0003 Phase 1 Spine Anti-Contamination and Structural Hardening Spec

**Intended repository path:** `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` for staging; final home `docs/4-specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`. (Number `0003` continues the post-overhaul active spec sequence: `0001` is live, the post-overhaul `0002` TUI-proof spec is now archived. `0002` is deliberately not reused because two archived specs already carry it.)

**Status:** COMPLETED. This spec does not replace, rewrite, or restate as local authority any foundation, architecture, execution, reference, archived-spec, or code file.

**Repository analyzed:** `joeloverbeck/tracewake`  
**Analyzed commit:** `a1a31edb2659bff17fddd5882967d6f3b76381a7`  
**Target type:** hardening / anti-contamination structural spec  
**Secondary type:** new spec  
**Primary surface:** non-TUI Phase 1 / Phase 1A kernel spine: event log, event envelopes, replay, projections, checksums, deterministic scheduling, action proposal / validation / scheduling pipeline, no-direct-dispatch mutation boundary, content schema / load / validation / serialization / golden fixtures.  
**Secondary surface:** TUI / view-model / debug seam re-verification against the completed TUI-proof remediation.  
**Declared execution admissibility posture:** **`P0-CERT scoped remediation`**.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

## 0. Exact-Commit Evidence Ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: a1a31edb2659bff17fddd5882967d6f3b76381a7
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run.open exact raw.githubusercontent.com URLs; container.download saved exact raw URLs where permitted; no repository-scoped fetches
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/reports/tui-proof-surface-hardening-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/archive/specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/.github/workflows/ci.yml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/rust-toolchain.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/events/envelope.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/events/log.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/events/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/replay/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/replay/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/checksum.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/time.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/movement.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/takeplace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/openclose.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/checkcontainer.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/debug_capability.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/debug_reports.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/agent/generation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/schema.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/validate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/manifest.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/replay_item_location_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/no_human_advance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/tests/fixtures_load.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/tests/forbidden_content.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/tests/golden_fixtures_run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/readme_sample_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/container_item_move_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/debug_attach_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/door_access_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/ordinary_workday_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/planner_trace_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/possession_parity_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/strongbox_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/view_filtering_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/accuseprobe.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/inspect.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/a1a31edb2659bff17fddd5882967d6f3b76381a7/crates/tracewake-core/src/actions/defs/work.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

**Ledger notes.** The uploaded manifest was used only to confirm repository paths. Archived specs were read as historical evidence only. Historical baseline commit URLs embedded inside archived specs were not used as fetch targets. All analysis below refers to the exact-commit files listed above, unless a sentence is explicitly marked as external prior art.

---

## 1. Header and Baseline Statement

This spec hardens the Phase 1 / Phase 1A block after the foundation / architecture / execution realignment. Its purpose is not to invent mechanics. Its purpose is to convert the current runtime discipline into structural locks so future work cannot casually reintroduce contamination: direct mutation outside the event path, truth leaking into actor-known cognition, nondeterminism in outcome paths, unversioned event drift, prose-born simulation facts, display-string diagnostics, or debug surfaces masquerading as diegetic affordance.

The baseline code is substantially clean. The audit confirms deterministic ordered collections in authoritative state, event envelopes with a schema constant and stream metadata, replay rebuild with checksum reports, deterministic scheduler ordering, typed proposal/source-context diagnostics, content validation against no-script fields, a sealed-ish actor-known context, a private debug capability, and TUI debug panels marked as non-diegetic.

The same audit also finds that the clean posture is not yet structural enough. `PhysicalState` and `AgentState` expose public mutable fields, so the "single mutation path" is a convention plus test evidence rather than a compile-time property. Event-schema migration discipline is present as loud rejection for unsupported versions but not yet a total version/migrator gate. Checksum coverage is hand-written and not guarded against new fields escaping coverage. The absence of `HashMap`, `HashSet`, wall-clock, thread, and randomness hazards in the audited spine is a present fact, not yet a mandatory CI/lint rule. This is exactly the kind of drift surface that a hardening spec should close.

### 1.1 Execution posture

The sole declared posture is **`P0-CERT scoped remediation`**.

Justification:

- The code already provides substantial evidence toward the event/replay/determinism/pipeline and TUI/debug split surfaces, but the evidence is distributed across ordinary tests, source scans, and conventions.
- The changes required by this spec affect core type boundaries, conformance tests, lint/source-scan gates, content validation tests, and CI surface. They are therefore remediation, not documentation-only review.
- This spec must not claim full project certification. It contributes scoped hardening evidence toward the relevant Phase 1 / Phase 1A gates and leaves later epistemic / ordinary-life / institution certification to later blocks.

---

## 2. Authority Chain and Gate Mapping

### 2.1 Authority chain

The authority order remains: foundation, then architecture, then execution, then reference, then specs. `docs/README.md:1-14` states the repository doctrine layering and that earlier tiers govern later tiers. `docs/README.md:31-39` frames live doctrine versus archived material. `docs/4-specs/README.md:30-38` requires future specs to declare one posture, use gate codes only as cross-references, use holder-known terminology, and treat archived specs as history.

The controlling foundation documents for this spec are:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`: invariant contract. Load-bearing invariants include `INV-008`, `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-020`, `INV-022`, `INV-058` through `INV-063`, `INV-065` through `INV-070`, `INV-091` through `INV-108`.
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`: event trace / replay spine. Meaningful changes require events (`:28-44`), event anatomy includes schema version and deterministic order (`:62-85`), replay requires ordered streams, random draw records, schema versions, and migrations/upcasters (`:216-228`), unsupported history rejects loudly (`:264-270`).
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`: no scripting, no authored outcome chains, seeds not scripts.
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`: truth may validate but not plan or render actor cognition.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`: embodied TUI / debug split.
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`: first-proof scope and acceptance posture.

The controlling architecture documents are:

- `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`: one-way crate dependency direction and no hidden mutation.
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`: event log, stream separation, replay, projections, schema evolution, randomness. It requires event sourcing to be the forensic backbone (`:7-29`), immutable commit and envelope minimum (`:32-55`), projections as derived/read-only surfaces (`:57-72`), schema evolution/upcasting and loud failure (`:91-101`), disciplined random streams (`:103-123`), and replay acceptance that detects missing, reordered, unsupported, or contaminating events (`:139-165`).
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`: every world-affecting action enters the ordinary proposal / validation / event pipeline and no actor or scheduler bypasses it (`:7-20`, `:23-40`, `:73-93`, `:154-165`).
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`: holder-known contexts and provenance.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`: TUI boundary.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`: acceptance and observability posture.

The controlling execution documents are:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`: gate vocabulary and execution subordination.
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`: archived specs are historical evidence only and future specs must choose one admissible posture.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`: sequence for the relevant certification surfaces.
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`: truth may validate, not plan (`:19-23`), every future spec must carry truth-firewall sections (`:55-68`), and forbidden contamination includes hidden-truth affordances, debug parsing, and culprit/quest content (`:70-83`).
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`: no world-affecting bypass, including human, autonomy, no-human, and harness paths (`:19-24`), scheduler limits (`:43-63`), audit burden (`:90-100`), and replay/projection coupling (`:102-127`).
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`: content schema/provenance/no-script validation.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`: fixture/replay acceptance.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`: required tests include static anti-regression, replay, view-model, content validation, and certification audit (`:24-36`); diagnostics require typed fields (`:38-52`); random testing must record seed/order and prove replay equality and debug quarantine (`:97-110`); display-string test patterns are forbidden (`:135-144`).
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`: TUI/view-model/debug re-verification surface.

The reference documents used for risk and terminology are:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`

The spec-tier controls are:

- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md`
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`

### 2.2 Gate mapping as cross-references only

This spec references `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, and `DIAG` only as upstream gate labels. It does not define or weaken their semantics.

- `SPINE-CERT`: event log, replay, projection rebuild, randomness/determinism, save/replay, action pipeline, no-direct-dispatch, TUI/debug split. This spec contributes scoped evidence and remediation for the Phase 1 / Phase 1A slice of that surface.
- `EPI-CERT`: referenced only for the TUI/view-model/debug re-verification and holder-known / actor-known truth-firewall seams. This spec does not re-spec epistemic mechanics.
- `P0-CERT`: this spec is scoped remediation, not a pass/fail certification declaration for the whole project.
- `NO-DIRECT`: the mutation and scheduler requirements below operationalize the no-bypass doctrine.
- `REPLAY`: replay/checksum/schema-version requirements operationalize deterministic replay acceptance.
- `FIXTURE`: content/golden-fixture requirements bind validation and canonical serialization.
- `DIAG`: typed reason/provenance diagnostics requirements bind the diagnostic surface.

---

## 3. Scope and Non-Goals

### 3.1 In scope

- Harden the event envelope, event log, event application, replay rebuild, projection rebuild, checksums, deterministic scheduler, and action proposal/validation/pipeline spine.
- Harden the content schema, deterministic load, validation phases, canonical serialization, fixture contracts, and golden replay tests.
- Re-verify the TUI / view-model / debug seam against the completed TUI-proof remediation, including current-view command submission, source-context validation, typed availability/why-not, debug quarantine, no-human quarantine, typed notebook leads, transcript determinism, and adversarial gates.
- Add structural enforcement: compile-time capabilities/sealing where practical, lint/source-scan/dependency gates, conformance tests, schema-version/migration discipline, checksum coverage discipline, and doc/invariant CI checks.

### 3.2 Non-goals

- No new world mechanics.
- No re-specification of epistemics, ordinary-life needs/routines, institutions, speech/LLM, regional scale, or graphical clients.
- No backward-compatibility shims, alias paths, or historical baseline promotion.
- No gate-semantics redefinition.
- No test that uses display labels, string prefixes, or debug summaries as authority.
- No assumption that the analyzed commit is the current `main`.

---

## 4. External Prior Art Used to Shape Structural Gates

External references are not Tracewake authority. They shaped enforcement mechanisms only.

- Event sourcing practice treats the event log as the source of truth for rebuilding state, not merely as debug output.[^fowler-event-sourcing] Akka's event-sourcing guide similarly frames durable state as an append-only sequence of serialized events.[^akka-event-sourcing]
- Event versioning systems use explicit upcasting/migration paths and fail loudly when older/newer event shapes cannot be interpreted.[^axon-event-versioning]
- Deterministic lockstep simulation literature treats even small nondeterminism sources as replay/desync hazards.[^gaffer-lockstep]
- Rust's `HashMap` is randomly seeded, while `BTreeMap` is key-ordered; for Tracewake's outcome paths, that supports the design choice to use ordered collections and ban unordered iteration in causally relevant paths.[^rust-hashmap][^rust-btreemap]
- Rust module privacy, private fields, and non-public constructors are the right primitive for making privileged operations unreachable from the wrong layer.[^rust-privacy]
- "Parse, don't validate" is the relevant design principle for turning prose/schema facts into typed, constrained domain values before the kernel can use them.[^parse-dont-validate]
- Clippy supports `disallowed-types` and `disallowed-methods`, which can express some banned API gates inside the pinned Rust toolchain.[^clippy-config]
- Architecture fitness functions turn architectural rules into executable tests that fail the build when a layer boundary is violated.[^thoughtworks-fitness]
- Object-capability discipline supports sealed capabilities for debug truth, mutation authority, and privileged context construction.[^ocap-guide]

---

## 5. Audit Findings

Each finding is tagged as either **already-satisfied** or **needs-hardening**. "Already-satisfied" does not mean no requirement is needed; it means the present implementation behaves correctly and the spec should lock that behavior structurally. "Needs-hardening" means the present behavior is correct only by convention, ordinary tests, or source-scan smoke tests, or lacks total coverage against future drift.

### 5.1 Summary table

| Finding | Dimension | Responsible layer | Status | Invariants / gates borne on | Evidence | Required hardening |
|---|---|---:|---|---|---|---|
| AF-01 | Authority layering and spec posture | docs/specs | already-satisfied | `P0-CERT` posture discipline | `docs/README.md:1-14`, `docs/README.md:31-39`, `docs/4-specs/README.md:30-38` | Keep this spec subordinate; no gate redefinition. |
| AF-02 | Event envelope schema/version/stream metadata | core/events | already-satisfied | `INV-009`, `INV-017`, `INV-018`, `INV-020`, `INV-092` | `events/envelope.rs:9`, `:11-19`, `:21-77`, `:79-148`, `:151-226`, `:487-544` | Add total event-kind registry tests and schema-version migrator gate. |
| AF-03 | Append/apply path rejects unsupported schema and non-world mutation | core/events | already-satisfied | `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-107` | `events/apply.rs:40-58`, `:106-126`, `:1503-1560`, `:1613-1627`; `events/log.rs:3-57` | Keep unknown-version loud failure; add version/migration discipline and stream-separation conformance test. |
| AF-04 | Replay rebuild applies event streams and checksums rebuilt projections | core/replay | already-satisfied | `INV-017`, `INV-018`, `INV-092`, `REPLAY` | `replay/rebuild.rs:45-150`, `:67-110`, `:131-150`, `:491-545`, `:589-642` | Add total field coverage proof so new state cannot escape replay/checksum. |
| AF-05 | Checksum canonicalization is deterministic and insertion-order independent | core/checksum | already-satisfied but drift-prone | `INV-017`, `INV-018`, `INV-092` | `checksum.rs:47-152`, `:189-292`, `:443-474` | Add checksum coverage manifest/macro/test for every authoritative field. |
| AF-06 | Authoritative state uses ordered collections | core/state | already-satisfied | `INV-017`, `INV-018`, `INV-092` | `state.rs:1`, `:113-139`, `:197`, `:237`, `:316` | Add banned unordered collection gate for outcome paths. |
| AF-07 | Authoritative state is publicly mutable | core/state | needs-hardening | `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-099`, `INV-104` | `state.rs:122-139` shows public fields on `PhysicalState` / `AgentState`; event application mutates through `EventApplicationContext` at `events/apply.rs:28-58` | Seal mutation with private fields, read-only accessors, and an event-application mutation capability. |
| AF-08 | Deterministic tick/scheduler ordering and no-human sequencing | core/scheduler/time | already-satisfied but needs CI lock | `INV-017`, `INV-018`, `INV-091`, `INV-092`, `INV-103`, `INV-104`, `INV-108` | `time.rs:1-22`, `:45-66`; `scheduler.rs:7-9`, `:52-90`, `:294-306`, `:1770-1840`, `:2929-3020` | Add nondeterminism bans and scheduler no-direct conformance tests. |
| AF-09 | Action proposals carry origin/source context and typed diagnostics | core/actions | already-satisfied | `INV-099` through `INV-106`, `DIAG`, `NO-DIRECT` | `actions/proposal.rs:10-28`, `:54-87`; `actions/report.rs:4-10`, `:48-85`, `:91-169`, `:206-219`; `actions/registry.rs:1`, `:47-143` | Add forged/stale/privileged source-context adversarial tests and public API sealing. |
| AF-10 | Pipeline is the ordinary append/apply path | core/actions/events | already-satisfied but monolithic/drift-prone | `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-104`, `NO-DIRECT` | `actions/pipeline.rs` defines pipeline stages, validates source context, constructs events, appends to `EventLog`, then calls `apply_event_stream`; action definitions build events using `EVENT_SCHEMA_V1`, e.g. `actions/defs/eat.rs` exact-commit fetch; `events/apply.rs:40-58` | Add conformance tests that no world mutation path bypasses pipeline/log/apply. |
| AF-11 | Actor-known context is sealed-ish and provenance-aware | core/agent | already-satisfied but must stay sealed | `INV-099` through `INV-108` | `agent/actor_known.rs:9-17`, `:142-162`, `:184-260`, `:276-298`, `:335-360`; `agent/transaction.rs:13-21`, `:43-72`, `:86-141`, `:168-260` | Keep constructors private/crate-private; add compile-fail/public API tests and hidden-truth gates. |
| AF-12 | Debug capability is private and explicitly non-diegetic | core/debug + TUI/debug | already-satisfied | `INV-068`, `INV-093`, `INV-099`, `INV-107` | `debug_capability.rs:1-34`; `debug_panels.rs:10-14`, `:36-38`, `:62-64`, `:81-83`, `:102-104` | Keep capability private; add adversarial debug-truth leakage tests and TUI source scans. |
| AF-13 | Content load/validation is deterministic and rejects no-script/prose-born facts | content | already-satisfied but schema drift-prone | `INV-020`, `INV-022`, `INV-058` through `INV-063`, `INV-097`, `FIXTURE`, `DIAG` | `load.rs:40-89`, `:147-185`; `validate.rs:21-58`, `:82-118`, `:141-228`, `:263-432`, `:913-1050`, `:1202-1324`, `:1426-1607`, `:1670-1727`; `forbidden_content.rs:23-95`, `:130-198`; `fixtures_load.rs:130-235` | Add schema-field registration discipline, no-prose-born-fact regression fixture, and typed diagnostic coverage. |
| AF-14 | TUI current-view command path and debug split hold | TUI | already-satisfied; re-verify in CI | `INV-008`, `INV-065` through `INV-070`, `INV-093` through `INV-095`, `INV-107`, `INV-108`, `EPI-CERT` cross-reference | `app.rs:135-240`, `:285-367`; `render.rs:4-27`, `:210-225`; `input.rs:93-99`, `:148-159`; `transcript.rs:19-121`; `embodied_flow.rs:7-90`; `transcript_snapshot.rs:9-40` | Keep TUI-proof gates as required regression tests; expand direct-applier/source-scan coverage. |
| AF-15 | CI runs core Rust gates but lacks explicit spine anti-contamination gates | workspace/CI | needs-hardening | `INV-017`, `INV-018`, `INV-092`, `INV-105`, `P0-CERT` scoped remediation | `.github/workflows/ci.yml:13-16`, `:20-26`, `:28-43`, `:45-62`; `rust-toolchain.toml:1-7`; no `clippy.toml` or `deny.toml` path appears in the manifest inventory | Add conformance tests under `cargo test`; add clippy/source-scan gates expressible in pinned toolchain. |
| AF-16 | Existing anti-regression tests are useful but not total | core/tests | needs-hardening | `INV-009`, `INV-017`, `INV-018`, `INV-099` through `INV-108`, `NO-DIRECT`, `DIAG` | `anti_regression_guards.rs:1-20`, `:22-63`, `:65-105`, `:107-165`; `hidden_truth_gates.rs:99-309` | Promote smoke/source scans into a named spine conformance suite with total lists and typed reports. |

### 5.2 Event log, append-only path, event schema, and stream separation

**Finding AF-02/AF-03: already-satisfied; needs structural lock.**

The envelope layer defines `EVENT_SCHEMA_V1` (`events/envelope.rs:9`), a typed `EventStream` split (`:11-19`), an `EventKind` set (`:21-77`), and metadata mapping kind to stream and whether the event is physical-mutating (`:79-148`, `:151-226`). `EventEnvelope::new_v1` and schema checking sit on the envelope rather than on display strings (`:487-544`). Event application routes by stream, applies world events through `apply_event`, routes agent/epistemic streams separately, and treats diagnostic/controller/replay-debug events as non-mutating (`events/apply.rs:40-58`). `apply_event` rejects unsupported schema versions, mismatched stream, and non-world direct application (`events/apply.rs:106-126`). Tests demonstrate valid world events mutate expected state, precondition mismatch leaves state unchanged, and unsupported schema versions fail without mutation (`events/apply.rs:1503-1560`); unsupported epistemic payload schemas also fail (`events/apply.rs:1613-1627`).

This satisfies the present event-spine doctrine. It does not yet prove totality when future events are added. The hardening requirement is not to rewrite the event system; it is to make the following impossible to miss in review:

- adding an `EventKind` without metadata;
- adding a stream without an application rule;
- adding a schema version without a migrator/upcaster or loud replay failure test;
- accidentally permitting controller/debug/diagnostic events to mutate physical state;
- emitting a physical mutation as a non-world event.

### 5.3 Replay, projections, and checksum coverage

**Finding AF-04/AF-05: already-satisfied; coverage guard needed.**

Replay rebuild clones initial physical and agent state, validates schema compatibility, applies events through the event application path, and computes physical, epistemic, and agent checksums (`replay/rebuild.rs:45-150`). It groups unsupported schemas by stream (`:67-81`), applies stream events through the shared application path (`:91-110`), and records final checksums/report diffs (`:131-150`). Tests cover deterministic agent rebuild and canonical input stability (`:491-545`) and windowed need-delta batching producing the same final agent state/canonical input (`:589-642`). Physical checksums serialize canonical lines over actors, places, doors, containers, items, food supplies, and workplaces (`checksum.rs:47-152`); agent checksums cover needs, intentions, routines, decision traces, and stuck diagnostics (`checksum.rs:189-292`). Tests assert checksum stability and insertion-order independence (`checksum.rs:443-474`).

The weakness is total coverage: the checksum functions are hand-written over state fields. A future field can be added to `PhysicalState` or `AgentState` and omitted from the checksum without a compiler error. That would violate replay equality while all existing checks continue to pass if no fixture exercises the field. This spec requires an explicit coverage manifest/macro/test tied to the state field list.

### 5.4 Determinism, ordering, no wall-clock, and nondeterminism hazards

**Finding AF-06/AF-08: already-satisfied; needs CI gate.**

`PhysicalState` and `AgentState` use ordered `BTreeMap`/`BTreeSet` collections (`state.rs:1`, `:113-139`, `:197`, `:237`, `:316`). Simulation time is a discrete `SimTick(u64)` with deterministic construction/advance and tests for tick progression and passive need deltas (`time.rs:1-22`, `:45-66`). Scheduler phases and ordering keys are typed (`scheduler.rs:7-9`, `:52-90`), and no-human processing sorts windows and actors deterministically (`scheduler.rs:294-306`). No-human advance produces start/completion marker events and schedules ordinary proposals through ordering keys (`scheduler.rs:1770-1840`). Tests cover ordering independent of insertion order and deterministic proposal sequences (`scheduler.rs:2929-3020`) and scheduler tick/sequence progression (`scheduler.rs:3009-3020`).

The audited spine/TUI/content source subset contains no outcome-path use of `std::collections::HashMap`, `std::collections::HashSet`, `SystemTime`, `Instant`, `rand`, thread spawning, network, or filesystem access during resolution. That absence is a fact at this commit, not a structural rule. The workspace has a pinned Rust toolchain with clippy installed (`rust-toolchain.toml:1-7`) and CI runs fmt, clippy, build, and tests (`.github/workflows/ci.yml:20-62`), but no `clippy.toml`/`deny.toml` path appears in the manifest inventory. This spec requires a pinned-toolchain-compatible nondeterminism gate.

### 5.5 No-direct-dispatch and single mutation boundary

**Finding AF-07/AF-10/AF-16: needs hardening despite clean behavior.**

The intended path is clear: proposals carry origin/source context (`actions/proposal.rs:10-28`, `:54-87`), the registry is deterministic (`actions/registry.rs:1`, `:47-143`), diagnostics are typed (`actions/report.rs:4-10`, `:48-85`, `:91-169`, `:206-219`), the pipeline validates source context and routes accepted proposals through event construction, append, and application, and `EventApplicationContext` holds the mutable physical/agent references used by event application (`events/apply.rs:28-58`). Existing anti-regression tests source-scan scheduler and action surfaces (`anti_regression_guards.rs:1-63`) and verify typed trace/diagnostic posture (`:65-105`).

The structural hole is `state.rs:122-139`: authoritative state fields are public. That makes direct mutation by future code compile even if doctrine forbids it. Today the repository is disciplined; tomorrow a feature can write `state.actors.insert(...)` from a planner, debug panel, test harness, or scheduler. The correct remediation is compile-time sealing of authoritative mutation, backed by conformance source scans.

### 5.6 Truth firewall, holder-known / actor-known contexts, and debug quarantine

**Finding AF-11/AF-12: already-satisfied; keep sealed.**

Actor-known facts are private-field values with provenance (`agent/actor_known.rs:9-17`). Provenance distinguishes observed/remembered/routine/fixture facts from validation truth, debug omniscience, and unproven physical truth (`:142-162`). `ActorKnownPlanningContext` fields are private (`:184-195`), construction from observed parts is crate-private (`:197-223`), access is through getters (`:226-260`), facts are sorted and auditable by provenance (`:276-298`), and builders derive actor-local/projection-limited views (`:335-360`). Actor decision transactions require `&ActorKnownPlanningContext` (`agent/transaction.rs:13-21`) and construct proposals from that context, not from debug truth (`agent/transaction.rs:43-260`).

`DebugCapability` has a private field, compile-fail documentation for external construction, and a crate-private mint function (`debug_capability.rs:1-34`). TUI debug panel renderers repeat the `DEBUG NON-DIEGETIC` marker (`debug_panels.rs:10-14`, `:36-38`, `:62-64`, `:81-83`, `:102-104`). Hidden-truth gates prove actor-known-only audits and debug/unproven facts cannot masquerade as planning knowledge (`hidden_truth_gates.rs:99-309`).

The remediation is preservation and expansion: public constructors must not be added, source scans must stay adversarial, and debug truth must remain capability-guarded.

### 5.7 Content schema, deterministic load, validation, and no-script / no-prose-born facts

**Finding AF-13: already-satisfied; schema drift-prone.**

Content loading sorts input paths, deserializes, canonicalizes, validates, and computes a manifest from canonical bytes (`load.rs:40-69`); actor rosters and day windows are sorted (`load.rs:70-89`); shuffled input paths produce the same loaded world and fingerprint (`load.rs:147-185`). Validation uses typed reports, phases, statuses, and errors (`validate.rs:21-58`), sorts errors deterministically (`:82-96`), and validates both raw and parsed fixture bytes (`:100-118`). Validation phases cover IDs, references, topology, state, registry parity, no-player, no-script, shortcut rejection, epistemic seeds, determinism, contract, and serialization (`validate.rs:141-160`). Raw-field validation rejects forbidden keys and unknown fields (`:164-228`), reserved player/story/quest/objective/reward/culprit/director forms (`:263-432`), player-only action fields (`:1202-1216`), script markers and missing actor-known provenance for planner-intended seeds (`:1218-1285`), authored outcome chains (`:1316-1324`), shortcut truth-fields in prehistory (`:1426-1529`), determinism hazards (`:1532-1565`), and serialization drift (`:1582-1607`, `:1670-1727`).

Tests reject quest/reward/player/force-event fields (`forbidden_content.rs:23-35`), culprit/true culprit/stolen flag/NPC truth/quest state/player memory (`:39-60`), teleport/refill/hidden planner/final-event/scripted outcome shortcuts (`:63-95`), outcome-chain/debug acceptance/player-conditioned markers (`:130-198`), and missing provenance for planner-intended initial facts (`:144-160`). Fixture loading validates and loads the current fixture set deterministically (`fixtures_load.rs:130-235`).

The weakness is not the current validator. It is future field drift. Every new schema field needs a typed validation home, canonical serialization rule, provenance rule, and no-script/no-prose-born-fact review. This spec makes that a structural acceptance condition.

### 5.8 TUI-seam re-verification against completed TUI-proof gates

**Finding AF-14: re-verification passes at this commit; keep as CI evidence.**

This spec does not redo the TUI-proof work. It re-verifies that the merged seam still holds.

| Prior TUI-proof gate | Re-verification result | Evidence | Notes |
|---|---|---|---|
| `TUI-AC-001` sealed holder-known/current actor view | holds | `app.rs:151-175`; `embodied_flow.rs:7-53` | `current_view` builds embodied view from the bound actor and holder-known context metadata. |
| `TUI-AC-002` source-context proposal validation | holds | `app.rs:182-232`; `actions/proposal.rs:54-87`; `actions/report.rs:48-85` | Submitted semantic actions come from the current view and include source-context identity/hash/frontier/tick. |
| `TUI-AC-003` debug capability / debug truth quarantine | holds | `debug_capability.rs:1-34`; `debug_panels.rs:10-14`; `debug_panels.rs:36-104` | Debug is capability-gated and visibly non-diegetic. |
| `TUI-AC-004` typed availability and two-layer why-not | holds | `render.rs:4-27`; `render.rs:210-225`; `actions/report.rs:206-219` | Rejections render typed reason codes and actor-visible/debug summaries. |
| `TUI-AC-005` current-view command submission | holds | `input.rs:93-99`; `input.rs:148-159`; `app.rs:182-193` | Numeric/semantic commands resolve through the current embodied action list. |
| `TUI-AC-006` no-human quarantine | holds for current block | `scheduler.rs:1770-1840`; `transcript_snapshot.rs:29-40` | No-human sequencing remains scheduler/pipeline evidence, not TUI-authored mutation. |
| `TUI-AC-007` typed notebook/leads surface | holds at this scope | `transcript.rs:19-87`; `app.rs:151-175` | Notebook/transcript are derived surfaces, not authority. |
| `TUI-AC-008` deterministic transcript/replay | holds | `transcript.rs:90-121`; `transcript_snapshot.rs:9-40` | Transcript output is byte-stable in snapshot tests. |
| `TUI-AC-009` adversarial hidden/debug truth gates | holds | `hidden_truth_gates.rs:99-309`; `embodied_flow.rs:55-73` | Hidden food/route/workplace/debug facts do not become embodied affordances. |
| `TUI-AC-010` no direct event applier in TUI | holds by current source scan | `embodied_flow.rs:75-90`; `app.rs:196-232` | TUI uses `run_pipeline`; this spec expands the scan into a structural gate. |
| `TUI-AC-011` possession parity / no player privilege | holds at this seam | `controller.rs:40-100`; `app.rs:135-148`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:395-425` | Binding attaches controller through modeled events; no privileged player mutation path is evidenced. |
| `TUI-AC-012` debug panel non-diegetic labeling | holds | `debug_panels.rs:10-14`; `transcript_snapshot.rs:9-40` | Debug output is explicitly marked and snapshot-tested. |

No regression was found in the TUI seam. The required remediation is to keep this as a named re-verification suite and to prevent the spine hardening from weakening the TUI proof surface.

---

## 6. Is a New Spec Warranted?

**Verdict: yes, a new spec is warranted.**

The Phase 1 / Phase 1A spine is substantially aligned with doctrine at the analyzed commit, but its most important protections are not durable enough. The code already behaves like an event-sourced deterministic kernel, but future work can still compile direct state mutation because `PhysicalState` and `AgentState` expose public fields. Future event kinds can be added without a total stream/mutation/schema coverage gate. Future state fields can escape checksum/replay coverage. Future contributors can introduce `HashMap`, `HashSet`, wall-clock, threads, unseeded randomness, or filesystem/network reads in outcome paths unless CI forbids them. Content validation is strong today, but a new schema field can bypass no-script/provenance/canonicalization review without a field-registration gate. Existing anti-regression tests are useful but partial.

Therefore the correct intervention is not a rewrite. It is structural hardening: compile-time capabilities and private mutation surfaces where possible, conformance tests where Rust cannot express the architecture directly, and CI/lint checks where the boundary is API-level or source-pattern-level.

This spec is warranted because its value is converting present cleanliness into future failure modes: wrong changes should stop at compile time or CI, not be caught by a human remembering doctrine.

---

## 7. Structural Hardening Requirements and Anti-Contamination Gates

Each requirement declares the invariant(s) it enforces, the failure it prevents, and the required enforcement mode. Enforcement labels:

- **Structurally guaranteed:** Rust visibility, private fields, sealed constructors, type-state/newtype/capability patterns, or crate graph constraints make the invalid operation unreachable from the wrong layer.
- **Test-guaranteed:** ordinary, adversarial, compile-fail, snapshot, or source-scan tests fail if the operation appears.
- **Convention-only:** documentation/review only. Requirements below must not remain convention-only unless explicitly marked as an unavoidable fallback.

### SPINE-AC-001 — Seal authoritative state mutation behind the event-application capability

**Invariants enforced:** `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-099`, `INV-104`, `INV-107`.  
**Prevents:** direct mutation of physical/agent state from planners, scheduler, TUI, debug panels, tests, content loaders, or future convenience helpers.  
**Current evidence:** state collections are deterministic (`state.rs:1`, `:113-139`) but `PhysicalState` and `AgentState` expose public fields (`state.rs:122-139`). Event application owns the intended mutable context (`events/apply.rs:28-58`).  
**Required enforcement:** **structurally guaranteed**, backed by tests.

Required change:

1. Make authoritative mutable fields private or at most `pub(crate)` behind read-only public accessors. External crates must not be able to mutate `PhysicalState` or `AgentState` collections directly.
2. Introduce an unforgeable mutation capability, for example `WorldMutationCapability` / `AgentMutationCapability`, constructible only inside `crates/tracewake-core/src/events/apply.rs` or a private `events::mutation` module.
3. All state-changing methods must require that capability or be private to the application module.
4. Fixtures/loaders may construct initial states only through explicit seed/fixture builders that are not callable as runtime mutation paths and are audited by content validation.
5. Tests that need to build states must use builders marked as fixture/test construction, not runtime mutation.

Acceptance:

- Code outside the event-application module cannot compile if it tries to insert/remove/mutate authoritative world or agent fields directly.
- Existing replay, content loading, and TUI flows still work through builders and the event pipeline.
- A conformance source scan lists the only files allowed to call mutation-capability constructors.

### SPINE-AC-002 — Event schema versioning must have a total registry, migrator/upcaster gate, and loud unknown-version failure

**Invariants enforced:** `INV-009`, `INV-017`, `INV-018`, `INV-020`, `INV-092`.  
**Prevents:** silent replay of unsupported events, accidental acceptance of mixed schemas, and untested event evolution.  
**Current evidence:** `EVENT_SCHEMA_V1` exists (`events/envelope.rs:9`); envelope creation/checking exists (`events/envelope.rs:487-544`); apply/replay fail unsupported schema versions (`events/apply.rs:106-126`, `:1547-1560`; `replay/rebuild.rs:67-81`).  
**Required enforcement:** **test-guaranteed now**, **structurally guaranteed where practical** through a registry type.

Required change:

1. Replace ad hoc schema constants with an explicit `EventSchemaVersion` newtype/enum and registry.
2. Keep current `EVENT_SCHEMA_V1` compatibility as the only supported live version unless a new version is introduced deliberately.
3. Adding a new supported event schema version must require:
   - an upcaster/migrator function or an explicit no-migration proof;
   - replay tests for old-to-new, new-to-current, and unknown-future version failure;
   - a fixture or synthetic event stream proving the migration path.
4. Unknown versions must fail loudly before mutation, both in live apply and replay.

Acceptance:

- A test named like `unsupported_event_schema_version_replay_fails_loudly` exercises replay failure.
- A test named like `adding_event_schema_version_requires_migrator_registration` fails if the version registry changes without a migrator test.
- No requirement demands backward-compatible shims; only deliberate version handling is allowed.

### SPINE-AC-003 — Event kind stream/mutation metadata must be total and mechanically tested

**Invariants enforced:** `INV-008`, `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-068`, `INV-069`, `INV-099`, `INV-107`.  
**Prevents:** physical mutations on controller/debug/diagnostic streams; orphan event kinds; stream mismatch bypass.  
**Current evidence:** `EventKind::all` and metadata exist (`events/envelope.rs:21-148`); stream mapping and `physical_mutating` flags exist (`:151-226`); apply routes non-world streams away from physical mutation (`events/apply.rs:40-58`).  
**Required enforcement:** **test-guaranteed**, with structural metadata centralization.

Required change:

1. Keep a single authoritative event-kind metadata registry.
2. Every `EventKind` must have stream, schema, physical-mutation flag, and replay handling metadata.
3. Conformance tests must assert:
   - `EventKind::all()` covers every enum variant;
   - every physical-mutating kind maps to `EventStream::World` unless an explicit architecture exception exists;
   - controller, diagnostic, replay-debug, and debug-only streams cannot change `PhysicalState` checksum;
   - stream mismatch is rejected before mutation.

Acceptance:

- Tests fail if a new `EventKind` lacks metadata or replay/apply treatment.
- Tests include at least one adversarial non-world event with a physical-looking payload and prove state checksum is unchanged.

### SPINE-AC-004 — Replay and checksum coverage must be total over authoritative state

**Invariants enforced:** `INV-017`, `INV-018`, `INV-092`, `REPLAY`.  
**Prevents:** new state fields escaping canonical checksum/replay equality.  
**Current evidence:** replay rebuild computes physical/epistemic/agent checksums (`replay/rebuild.rs:45-150`); checksum code covers existing physical and agent fields (`checksum.rs:47-152`, `:189-292`); tests prove stability and insertion-order independence (`checksum.rs:443-474`).  
**Required enforcement:** **test-guaranteed**, optionally aided by declarative field registration.

Required change:

1. Add a checksum coverage declaration next to authoritative state definitions or next to checksum code.
2. A conformance test must fail if a `PhysicalState` or `AgentState` field exists without a corresponding checksum coverage entry.
3. New state-bearing structs that affect simulation outcome must either implement canonical checksum serialization or be explicitly marked non-authoritative with a reason.
4. Replay reports must include enough typed detail to identify the first divergent event and field family.

Acceptance:

- A synthetic new-field omission test pattern is documented in the conformance suite.
- Replay checksum tests include a field-level mutation that changes each major state family: actor location, door/container state, item/food state, workplace assignment, need, routine/intention, decision trace, and stuck diagnostic.

### SPINE-AC-005 — Outcome paths must ban nondeterministic APIs and unordered iteration

**Invariants enforced:** `INV-017`, `INV-018`, `INV-092`.  
**Prevents:** replay drift from randomized hash seeding, wall-clock reads, unseeded RNG, thread scheduling, filesystem/network timing, or process environment.  
**Current evidence:** audited spine state uses `BTreeMap`/`BTreeSet` (`state.rs:1`, `:113-139`); discrete ticks are deterministic (`time.rs:1-22`); CI already runs clippy/build/test with pinned toolchain (`rust-toolchain.toml:1-7`; `.github/workflows/ci.yml:20-62`); no `clippy.toml`/`deny.toml` path appears in the manifest inventory. A repo-wide scan at the analyzed commit finds zero `HashMap`/`HashSet`/`SystemTime`/`Instant`/`rand`/thread-spawn uses in `tracewake-core/src`, `tracewake-content/src`, and `tracewake-tui/src`, so the banned-API gate can land with an empty allowlist and pass clean on first run.  
**Required enforcement:** **test-guaranteed in the pinned toolchain**, with **lint-guaranteed** where clippy supports it.

Required change:

1. Add a workspace `clippy.toml` using `disallowed-types` / `disallowed-methods` where precise enough to ban:
   - `std::collections::HashMap` and `std::collections::HashSet` in outcome-affecting crates/modules;
   - `std::time::SystemTime` and `std::time::Instant` in outcome-affecting code;
   - known RNG/process/thread APIs in core outcome paths.
2. Where clippy cannot express module-scoped exceptions cleanly, add a no-dependency Rust source-scan conformance test under `cargo test`.
3. Allowed exceptions must be listed in a small allowlist with rationale and responsible layer. No blanket allowlist is permitted.

Acceptance:

- CI fails if any banned API appears outside an allowlisted file.
- The allowlist is empty for `tracewake-core` outcome mutation/replay/scheduler/action paths unless a future spec explicitly authorizes a deterministic wrapper.

### SPINE-AC-006 — Scheduler and no-human processing must only schedule proposals or transaction inputs, never primitive state changes

**Invariants enforced:** `INV-091`, `INV-103`, `INV-104`, `INV-108`, `NO-DIRECT`.  
**Prevents:** scheduler becoming a second actor brain or primitive action dispatcher.  
**Current evidence:** scheduler uses typed phases/order keys (`scheduler.rs:7-90`), deterministic sorted windows/actors (`scheduler.rs:294-306`), and no-human event markers/proposal sequencing (`scheduler.rs:1770-1840`); anti-regression tests scan scheduler shortcut/direct-dispatch hazards (`anti_regression_guards.rs:22-63`).  
**Required enforcement:** **test-guaranteed**, with source-level conformance.

Required change:

1. Scheduler functions may produce ordering keys, no-human windows, controller/system marker events, and actor transaction/proposal invocations.
2. Scheduler functions must not directly call primitive action builders in a way that bypasses proposal validation, must not mutate `PhysicalState`/`AgentState`, and must not use hidden truth as actor cognition.
3. Current source scans must be expanded into a named scheduler conformance test with a reviewed allowlist for marker-event constructors.

Acceptance:

- Tests fail if scheduler imports or calls action-definition builders except through allowed registry/pipeline surfaces.
- Tests fail if scheduler mutates authoritative state fields directly once `SPINE-AC-001` lands.

### SPINE-AC-007 — Proposal source-context and controller binding checks must be sealed and adversarially tested

**Invariants enforced:** `INV-008`, `INV-065`, `INV-066`, `INV-069`, `INV-099` through `INV-106`, `INV-108`, `DIAG`.  
**Prevents:** stale/forged/privileged semantic actions, holder-known context substitution, and player-only affordances.  
**Current evidence:** `ProposalSourceContext` carries holder-known context identity/hash/frontier/tick (`actions/proposal.rs:54-87`); reason codes include missing/mismatch/stale/forged source conditions (`actions/report.rs:48-85`); TUI submits actions from the current view (`app.rs:182-232`).  
**Required enforcement:** **structurally guaranteed for constructors where practical**, **test-guaranteed for adversarial cases**.

Required change:

1. Constructing a TUI semantic-action proposal must require a current embodied view/source-context value, not raw strings alone.
2. The public API must not expose a constructor that can forge a privileged proposal without a source context except for clearly marked test/fixture origins.
3. Negative tests must cover missing source context, actor mismatch, stale context hash/frontier/tick, and semantic action ID not present in current view.

Acceptance:

- Negative cases produce typed `ReasonCode` values, never substring-matched labels.
- TUI command submission cannot bypass current view resolution.

### SPINE-AC-008 — Pipeline append/apply order must be mandatory for accepted world actions

**Invariants enforced:** `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-104`, `NO-DIRECT`, `REPLAY`.  
**Prevents:** accepted mutation without event, event appended after mutation, dry-run mutation leakage, and alternate player/harness paths.  
**Current evidence:** pipeline stages include validation, mutation-plan construction, event envelope construction, append, application, projection update, and debug linkage; accepted proposals append to `EventLog` and then call `apply_event_stream`; `events/apply.rs:40-58` is the stream applier.  
**Required enforcement:** **test-guaranteed**, with compile-time support from `SPINE-AC-001`.

Required change:

1. Accepted world actions must follow: proposal -> validation -> event envelope construction -> append -> apply -> projection/update/report.
2. Validation/dry-run code may inspect state and perform hypothetical application only on clones or explicitly non-authoritative temporaries.
3. No TUI, scheduler, content loader, debug panel, or test harness may expose an accepted outcome without an appended event unless it is fixture initial-state construction outside runtime simulation.

Acceptance:

- A conformance test asserts event-log length changes before authoritative state checksum changes for accepted runtime proposals, where the observable API permits.
- A negative source-scan test fails on direct calls to `apply_event` outside event/replay/tests unless allowlisted.

### SPINE-AC-009 — Actor-known context and debug truth capabilities must remain unforgeable from outside their authority layer

**Invariants enforced:** `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-106`, `INV-107`, `INV-108`.  
**Prevents:** debug/validation/unproven truth entering actor planning, affordances, or embodied rendering.  
**Current evidence:** actor-known facts and planning context have private fields and crate-private construction (`agent/actor_known.rs:9-17`, `:184-223`); provenance gates actor-known status (`:142-162`, `:276-298`); debug capability is private and crate-minted (`debug_capability.rs:1-34`); hidden-truth tests reject debug/unproven facts (`hidden_truth_gates.rs:99-309`).  
**Required enforcement:** **structurally guaranteed**, with public API and compile-fail tests.

Required change:

1. Keep `ActorKnownPlanningContext` construction private/crate-private. External crates must receive it only from sanctioned projection/view/model builders.
2. Keep `DebugCapability::mint` crate-private or narrower.
3. Do not add `From<PhysicalState>` or similar privileged constructors for actor-known planning.
4. Add compile-fail/doc tests or public API tests proving external crates cannot construct debug capability or actor-known planning context from truth.

Acceptance:

- Hidden truth remains visible to validators only as validation truth, never as actor-known facts.
- A debug panel test proves debug output does not alter available actions, proposal source context, or actor-known checksums.

### SPINE-AC-010 — Content schema must reject prose-born facts, authored outcomes, player/quest fields, and determinism hazards by typed phase

**Invariants enforced:** `INV-020`, `INV-022`, `INV-058`, `INV-059`, `INV-060`, `INV-061`, `INV-062`, `INV-063`, `INV-097`, `DIAG`, `FIXTURE`.  
**Prevents:** content authors smuggling outcomes through prose, culprit fields, quest state, shortcut truth, hidden planner instructions, or noncanonical data ordering.  
**Current evidence:** typed validation phases and reports exist (`validate.rs:21-58`, `:141-160`); raw forbidden keys and unknown fields are rejected (`:164-228`, `:1670-1727`); no-player/no-script/outcome-chain/determinism validations exist (`:263-432`, `:1202-1324`, `:1532-1607`); forbidden-content tests cover quest/reward/player/force-event/culprit/hidden truth/scripted outcome cases (`forbidden_content.rs:23-198`).  
**Required enforcement:** **test-guaranteed**, with schema-field registration discipline.

Required change:

1. Every new authored schema field must be registered with:
   - typed domain field/parser;
   - validation phase;
   - canonical serialization rule;
   - provenance/no-script decision;
   - diagnostic reason code or validation error kind.
2. Raw prose fields may describe labels/flavor only after typed validation proves they do not create simulation facts.
3. New forbidden content fixtures must be added for each new no-script boundary.

Acceptance:

- Content validation tests include at least one adversarial fixture for each forbidden content family.
- Validation reports remain typed by phase/error, not display text.

### SPINE-AC-011 — Diagnostics and why-not reports must remain typed, provenance-bearing, and actor/debug separated

**Invariants enforced:** `INV-070`, `INV-105`, `INV-106`, `INV-107`, `DIAG`.  
**Prevents:** display strings becoming authority, substring-matched tests, debug reason leakage into actor-visible why-not, and untyped audit reports.  
**Current evidence:** `ReasonCode::stable_id`, typed `CheckedFact`, and `CheckedFactSource` exist (`actions/report.rs:48-169`); validation reports carry actor-visible and debug-only fields (`actions/report.rs:206-219`); execution doctrine forbids display-string test authority (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:135-144`).  
**Required enforcement:** **test-guaranteed**, with enum/newtype APIs.

Required change:

1. Tests must assert reason-code enum/stable IDs and validation phases, not actor-facing sentences.
2. Actor-visible summaries may be snapshot-tested for UI stability but must not be the sole semantic proof.
3. Debug-only facts must be structurally separated from actor-visible facts in report structs.

Acceptance:

- New adversarial tests fail if a diagnostic loses `ReasonCode`, `ValidationPhase`, `CheckedFact`, provenance/source, or actor/debug separation.

### SPINE-AC-012 — TUI proof seam must remain a required regression gate

**Invariants enforced:** `INV-008`, `INV-065`, `INV-066`, `INV-067`, `INV-068`, `INV-069`, `INV-070`, `INV-093`, `INV-094`, `INV-095`, `INV-107`, `INV-108`.  
**Prevents:** reintroduction of direct event application in TUI, debug parsed as embodied command, stale semantic actions, non-diegetic truth becoming affordance, and possession-specific privileges.  
**Current evidence:** current-view / submit / pipeline path (`app.rs:151-240`), debug panels (`app.rs:285-367`; `debug_panels.rs:10-104`), typed rendering (`render.rs:4-27`, `:210-225`), command parsing (`input.rs:93-159`), deterministic transcript (`transcript.rs:19-121`), and TUI tests (`embodied_flow.rs:7-90`; `transcript_snapshot.rs:9-40`).  
**Required enforcement:** **test-guaranteed** and **source-scan-guaranteed**.

Required change:

1. Keep the TUI-proof tests in the workspace gate.
2. Expand direct-applier source scans so TUI cannot import/call event application directly except through `run_pipeline` or explicit replay/debug display paths.
3. Add adversarial TUI tests for stale current-view context, debug command strings not being accepted as embodied commands, and debug panel rendering not altering semantic action lists.

Acceptance:

- `cargo test --workspace` runs the TUI re-verification suite.
- Snapshot tests remain deterministic and include debug non-diegetic markers.

### SPINE-AC-013 — Add a named spine conformance / anti-contamination suite

**Invariants enforced:** `INV-008`, `INV-009`, `INV-011`, `INV-017`, `INV-018`, `INV-020`, `INV-022`, `INV-058` through `INV-063`, `INV-091` through `INV-108`.  
**Prevents:** architectural rules living only in prose or one-off tests.  
**Current evidence:** source-scan tests exist (`anti_regression_guards.rs:1-165`), hidden-truth tests exist (`hidden_truth_gates.rs:99-309`), content forbidden tests exist (`forbidden_content.rs:23-198`), and TUI snapshot tests exist (`transcript_snapshot.rs:9-40`).  
**Required enforcement:** **test-guaranteed**.

Required change:

Add a named conformance surface, such as:

- `crates/tracewake-core/tests/spine_conformance.rs`
- `crates/tracewake-content/tests/schema_conformance.rs`
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`

or equivalent modules inside existing test files, provided the names are discoverable and acceptance maps one-to-one to the requirements in this spec.

The conformance suite must report failures by responsible layer: `core/events`, `core/replay`, `core/state`, `core/scheduler`, `core/actions`, `core/agent`, `content/schema`, `content/validation`, `tui/view-model`, `tui/debug`, `workspace/ci`.

Acceptance:

- Each `SPINE-AC-*` requirement has at least one named positive or negative test, unless the acceptance condition is compile-time structural and demonstrated by public API/compile-fail tests.

### SPINE-AC-014 — Add doc/invariant reference checks to CI through the pinned Rust toolchain

**Invariants enforced:** all cited `INV-###`; spec posture discipline; `DIAG` review-artifact discipline.  
**Prevents:** dangling invariant references, mistyped invariant numbers, stale spec references, and drift between docs and requirements.  
**Current evidence:** specs must cite invariant/gate posture (`docs/4-specs/README.md:30-38`), and this spec cites many invariants.  
**Required enforcement:** **test-guaranteed**.

Required change:

1. Add a no-dependency Rust test or small binary invoked by `cargo test` that:
   - parses `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` for defined `INV-###` headings/references;
   - scans `docs/4-specs`, `specs`, `tickets`, and selected test names/comments for `INV-###` references;
   - fails on dangling invariant references;
   - optionally reports unused invariants as warnings only.
2. Gate-code strings remain cross-references only; the linter must not attempt to define gate semantics.

Acceptance:

- CI fails on a dangling `INV-###` reference in this spec or future specs.
- The linter ignores archived historical ledgers except for syntactic sanity, so old baseline evidence is not promoted to live authority.

### SPINE-AC-015 — Acceptance artifacts must use scoped certification wording and file/line evidence

**Invariants enforced:** `INV-091`, `INV-092`, `INV-093`, `INV-095`, `INV-097`, `INV-098`, `P0-CERT` posture discipline.  
**Prevents:** overclaiming certification, hiding responsible layer, and relying on screenshots/display labels as proof.  
**Current evidence:** execution test/diagnostic doctrine requires responsible layer and typed diagnostic fields (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md:38-77`).  
**Required enforcement:** **test-guaranteed for artifacts where practical**, **review-required otherwise**.

Required change:

1. Implementation PRs derived from this spec must include a short acceptance artifact listing:
   - exact commit under test;
   - gates run;
   - changed files;
   - per-requirement acceptance evidence;
   - any residual convention-only item and why it could not be structural.
2. Certification wording must be scoped:
   - allowed: "Phase 1 / Phase 1A spine hardening remediation accepted for this commit."
   - forbidden: "Project is P0 certified" or "SPINE-CERT passed" unless the upstream certification process separately declares that outcome.

Acceptance:

- Spec-to-tickets decomposition can map each requirement to one or more tickets without re-litigating intent.

---

## 8. Required Fixtures and Tests

The table below names required positive and adversarial/negative tests. Existing tests may be hardened rather than duplicated. Names are illustrative but should remain recognizable.

| Test / fixture gate | Type | Existing evidence to harden | Required addition | Requirements covered |
|---|---|---|---|---|
| `state_mutation_requires_event_application_capability` | compile/public API + source scan | `events/apply.rs:28-58`; `state.rs:122-139` | After sealing, code outside apply cannot mutate authoritative fields; source scan allowlists only fixture builders and event application. | `SPINE-AC-001`, `SPINE-AC-008` |
| `unsupported_event_schema_version_replay_fails_loudly` | negative replay | `events/apply.rs:1547-1560`; `replay/rebuild.rs:67-81` | Synthetic unknown-version event stream fails before mutation and reports typed schema failure. | `SPINE-AC-002` |
| `event_kind_metadata_is_total` | conformance | `events/envelope.rs:21-226` | Every event kind has stream/mutation/schema metadata and replay/apply handling. | `SPINE-AC-003` |
| `non_world_stream_cannot_change_physical_checksum` | adversarial | `events/apply.rs:40-58`; `checksum.rs:47-152` | Controller/diagnostic/replay-debug events with physical-looking payload leave physical checksum unchanged. | `SPINE-AC-003`, `SPINE-AC-012` |
| `checksum_coverage_is_total_for_authoritative_state` | conformance | `checksum.rs:47-152`, `:189-292`; `state.rs:113-139` | Field registry/source parser fails if new state field lacks checksum coverage. | `SPINE-AC-004` |
| `nondeterminism_api_gate` | source scan/clippy | CI `.github/workflows/ci.yml:20-62`; `rust-toolchain.toml:1-7` | Ban `HashMap`, `HashSet`, `SystemTime`, `Instant`, unseeded RNG, thread, network, filesystem outcome reads in audited paths. | `SPINE-AC-005` |
| `scheduler_never_direct_dispatches_primitive_action` | source scan + runtime | `anti_regression_guards.rs:22-63`; `scheduler.rs:1770-1840` | Expand allowlist and assert scheduler only schedules proposals/transactions/marker events. | `SPINE-AC-006` |
| `forged_or_stale_source_context_rejected_by_reason_code` | adversarial pipeline | `actions/proposal.rs:54-87`; `actions/report.rs:48-85`; `app.rs:182-232` | Missing/stale/mismatched/forged current-view source contexts reject with typed reasons. | `SPINE-AC-007`, `SPINE-AC-011` |
| `accepted_action_appends_before_authoritative_apply` | runtime/conformance | `actions/pipeline.rs`; `events/log.rs:3-57`; `events/apply.rs:40-58` | Accepted proposal produces log entry and then state change; rejected proposal mutates nothing. | `SPINE-AC-008` |
| `actor_known_context_unforgeable_from_truth` | compile/public API + adversarial | `agent/actor_known.rs:142-223`; `debug_capability.rs:1-34`; `hidden_truth_gates.rs:99-309` | External crates/tests cannot construct actor-known planning from raw truth/debug facts. | `SPINE-AC-009` |
| `debug_panel_does_not_change_embodied_affordances` | adversarial TUI | `debug_panels.rs:10-104`; `embodied_flow.rs:55-90` | Rendering/opening debug panel leaves semantic action list and source context unchanged. | `SPINE-AC-009`, `SPINE-AC-012` |
| `content_prose_born_fact_rejected` | negative fixture | `validate.rs:164-228`, `:1202-1324`, `:1426-1607`; `forbidden_content.rs:23-198` | Add fixture with prose/notes implying culprit/outcome/hidden truth; reject by typed phase. | `SPINE-AC-010` |
| `content_new_field_requires_validator_and_canonical_serialization` | conformance | `validate.rs:141-160`; `serialization.rs`; `fixtures_load.rs:197-235` | Schema field registry test fails if a schema field lacks validation/canonical serialization/provenance decision. | `SPINE-AC-010` |
| `diagnostics_never_assert_display_label_as_authority` | source scan/test discipline | `actions/report.rs:48-169`; execution doc `10` `:135-144` | Scan tests for forbidden substring/display-label semantic assertions where typed reason exists. | `SPINE-AC-011` |
| `tui_current_view_submission_rejects_stale_selection` | adversarial TUI | `input.rs:148-159`; `app.rs:182-232` | Numeric/semantic action captured from previous view rejects after view hash/frontier change. | `SPINE-AC-012` |
| `tui_transcript_snapshot_remains_byte_stable` | positive snapshot | `transcript.rs:90-121`; `transcript_snapshot.rs:9-40` | Keep existing snapshot; update only through reviewed intentional transcript change. | `SPINE-AC-012` |
| `doc_invariant_references_are_live` | CI/doc linter | `docs/4-specs/README.md:30-38` | No dangling `INV-###` references in live specs/tickets/tests. | `SPINE-AC-014` |
| `scoped_acceptance_artifact_present` | review artifact | execution doc `10` `:38-77` | PR implementing this spec includes per-requirement evidence and scoped certification wording. | `SPINE-AC-015` |

---

## 9. Acceptance Checklist

The implementation of this spec is accepted only when all applicable items below pass for the implementation commit under review.

### 9.1 Workspace gates

- [x] `cargo fmt --all --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo build --workspace --all-targets --locked`
- [x] `cargo test --workspace`

The current CI already runs these shapes or close variants (`.github/workflows/ci.yml:20-62`) under a pinned toolchain (`rust-toolchain.toml:1-7`). Any new gate must remain expressible inside this pinned toolchain unless added as an explicitly optional suggestion outside this spec.

### 9.2 New structural gates

- [x] Authoritative state mutation is sealed behind event-application capability (`SPINE-AC-001`).
- [x] Event schema version registry/migrator/unknown-version failure tests exist (`SPINE-AC-002`).
- [x] Event kind stream/mutation metadata totality test exists (`SPINE-AC-003`).
- [x] Replay/checksum coverage test fails on uncovered authoritative fields (`SPINE-AC-004`).
- [x] Nondeterminism banned-API gate runs in CI through clippy config and/or no-dep source-scan tests (`SPINE-AC-005`).
- [x] Scheduler/no-human no-direct-dispatch conformance test exists (`SPINE-AC-006`).
- [x] Proposal source-context adversarial tests cover missing/stale/forged/mismatched contexts with typed reasons (`SPINE-AC-007`).
- [x] Pipeline append/apply order and no alternate accepted mutation path are tested (`SPINE-AC-008`).
- [x] Actor-known/debug capability public API tests or compile-fail tests prove unforgeability (`SPINE-AC-009`).
- [x] Content schema field registration, no-script, no-prose-born-fact, and canonical serialization tests exist (`SPINE-AC-010`).
- [x] Diagnostics tests use typed reason/provenance fields, not display text (`SPINE-AC-011`).
- [x] TUI re-verification suite remains in `cargo test --workspace` and includes stale-context/debug-no-affordance/adversarial checks (`SPINE-AC-012`).
- [x] Named spine/content/TUI conformance suite maps to each `SPINE-AC-*` requirement (`SPINE-AC-013`).
- [x] Doc/invariant linter fails dangling `INV-###` references in live specs/tickets/tests (`SPINE-AC-014`).
- [x] Implementation acceptance artifact uses scoped certification wording and file/line evidence (`SPINE-AC-015`).

### 9.3 Per-requirement enforcement status target

| Requirement | Minimum accepted enforcement | Target enforcement after implementation |
|---|---|---|
| `SPINE-AC-001` | compile-time/private fields + tests | structurally guaranteed |
| `SPINE-AC-002` | registry + tests | test-guaranteed, partially structural |
| `SPINE-AC-003` | total metadata tests | test-guaranteed |
| `SPINE-AC-004` | checksum coverage test | test-guaranteed |
| `SPINE-AC-005` | clippy/source-scan CI gate | test/lint-guaranteed |
| `SPINE-AC-006` | source-scan + runtime tests | test-guaranteed |
| `SPINE-AC-007` | typed context constructors + adversarial tests | partially structural, test-guaranteed |
| `SPINE-AC-008` | capability support + pipeline tests | structurally supported, test-guaranteed |
| `SPINE-AC-009` | private constructors/capabilities + compile/public API tests | structurally guaranteed |
| `SPINE-AC-010` | schema registry/validation tests | test-guaranteed |
| `SPINE-AC-011` | typed report API + tests | structurally supported, test-guaranteed |
| `SPINE-AC-012` | TUI re-verification tests/source scans | test-guaranteed |
| `SPINE-AC-013` | named conformance suite | test-guaranteed |
| `SPINE-AC-014` | no-dep doc/invariant test | test-guaranteed |
| `SPINE-AC-015` | review artifact | review-required with typed evidence |

### 9.4 Certification-result wording

Allowed implementation result wording:

> Phase 1 / Phase 1A spine anti-contamination hardening scoped remediation is accepted for commit `<sha>`; this contributes evidence toward the relevant upstream gate surfaces but does not independently certify current `main` or the full project.

Forbidden wording:

- "`P0-CERT` passed" unless the upstream certification process declares that result.
- "`SPINE-CERT` passed" unless the upstream certification process declares that result.
- "TUI proof re-certified" unless the upstream certification process declares that result. Use "TUI seam re-verification passed for this scope" instead.

---

## 10. Implementation Ordering Guidance

This section is guidance for a later spec-to-tickets pass. It is not a separate scope.

1. **Seal mutation first.** `SPINE-AC-001` is the highest-leverage change because it moves the no-direct-dispatch boundary from doctrine into the type system.
2. **Add conformance tests before refactors expand.** `SPINE-AC-013`, `SPINE-AC-005`, and `SPINE-AC-014` should land early so subsequent tickets fail fast.
3. **Lock schema/replay/checksum totality next.** `SPINE-AC-002`, `SPINE-AC-003`, and `SPINE-AC-004` protect the event/replay spine from future feature accretion.
4. **Harden source-context/debug/TUI after the spine gates compile.** `SPINE-AC-007`, `SPINE-AC-009`, and `SPINE-AC-012` should preserve the completed TUI-proof seam, not change gameplay.
5. **Harden content validation in parallel.** `SPINE-AC-010` and `SPINE-AC-011` can often land as tests/registry refactors without altering mechanics.

---

## 11. Additional Suggestions Appendix — Cross-Block / Repo-Wide Anti-Contamination Mechanisms

These are suggestions for the broader iterative campaign. They are not acceptance criteria for this spec unless promoted by a later spec.

### A. Workspace-wide conformance crate

Create `crates/tracewake-conformance` or a top-level `tests/conformance` harness that owns architecture-fitness tests for the whole workspace. It would centralize source scans, crate-boundary checks, invariant-reference checks, and no-contamination regression fixtures across later blocks (`0004` epistemics and `0005`–`0008` ordinary-life/no-human hardening). This prevents each block from reinventing source scanners.

### B. Shared lint profile

Add a shared `clippy.toml` and a documented `lint-allowlist.md` that names every exception to banned APIs. Keep the allowlist narrow and reviewed. Consider optional `cargo-deny` later for dependency/license/source bans, but do not make it a mandatory gate in this spec unless CI explicitly installs it and maintainers accept a non-core tool.

### C. Doc-as-code invariant index

Generate a checked invariant index from `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` and use it across specs, tickets, tests, and review artifacts. This would keep the doctrine spine queryable without promoting specs into authority.

### D. Macro-assisted authoritative state coverage

For future larger state surfaces, consider a local macro that declares authoritative fields once and generates read accessors, checksum coverage hooks, and conformance metadata. This would reduce manual drift. Keep it simple; do not introduce a procedural macro unless the maintenance burden is justified.

### E. Fixture manifest with threat tags

Extend fixture metadata with threat tags such as `no-script`, `hidden-truth`, `replay`, `scheduler`, `debug-quarantine`, `source-context`, and `checksum`. That would let CI assert that every anti-contamination family has at least one positive and one adversarial fixture.

### F. Public API compile-fail test policy

Adopt a small compile-fail test policy for authority boundaries: debug capability construction, actor-known planning context construction, state mutation capability construction, and content schema bypasses. If the project avoids extra test harness dependencies, keep these as doc tests or minimal UI tests inside existing crates.

---

## 12. Self-Check Against the Brief

- [x] The manifest was used only as path inventory.
- [x] All fetched repository files were exact-commit URLs under `joeloverbeck/tracewake` at `a1a31edb2659bff17fddd5882967d6f3b76381a7`.
- [x] No clone, code search, branch fetch, default-branch lookup, or repository metadata was used.
- [x] Archived specs were treated as historical evidence only; their embedded old baselines were not fetch targets.
- [x] Exactly one execution posture is declared: `P0-CERT scoped remediation`.
- [x] Gate codes appear only as cross-references.
- [x] Primary surface is the non-TUI kernel spine; TUI seam is re-verified, not re-specified.
- [x] Findings distinguish already-satisfied from needs-hardening.
- [x] A new-spec verdict is present.
- [x] Requirements prioritize structural enforcement over convention.
- [x] Positive and adversarial/negative test gates are specified.
- [x] No new mechanics, LLM, graphical client, institution, ordinary-life, or epistemic re-spec scope is introduced.
- [x] Acceptance includes the four workspace gates and the new conformance/lint/doc checks.
- [x] External prior-art claims are cited below and remain subordinate to Tracewake doctrine.

---

## 13. External References

[^fowler-event-sourcing]: Martin Fowler, "Event Sourcing", https://martinfowler.com/eaaDev/EventSourcing.html
[^akka-event-sourcing]: Akka Guide, "Event Sourcing", https://doc.akka.io/libraries/guide/concepts/event-sourcing.html
[^axon-event-versioning]: Axon Framework Reference, "Event Versioning", https://docs.axoniq.io/axon-framework-reference/5.1/events/event-versioning/
[^gaffer-lockstep]: Glenn Fiedler, "Deterministic Lockstep", https://gafferongames.com/post/deterministic_lockstep/
[^rust-hashmap]: Rust standard library, `std::collections::HashMap`, https://doc.rust-lang.org/std/collections/struct.HashMap.html
[^rust-btreemap]: Rust standard library, `std::collections::BTreeMap`, https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[^rust-privacy]: Rust Reference, "Visibility and Privacy", https://doc.rust-lang.org/reference/visibility-and-privacy.html
[^parse-dont-validate]: Alexis King, "Parse, don't validate", https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
[^clippy-config]: Rust Clippy documentation, "Lint configuration", https://doc.rust-lang.org/clippy/lint_configuration.html
[^thoughtworks-fitness]: Thoughtworks, "Fitness Function Driven Development", https://www.thoughtworks.com/insights/articles/fitness-function-driven-development
[^ocap-guide]: Terse Systems, "Introduction to Object Capabilities", https://tersesystems.github.io/ocaps/guide/introduction.html

## Outcome

Completed on 2026-06-08.

Changes:

- Implemented and archived all `0003PHA1SPIANT-001` through `0003PHA1SPIANT-014` tickets.
- Added structural state mutation sealing, event schema registry, event-kind metadata totality, replay/checksum coverage, nondeterminism gates, scheduler no-direct-dispatch guards, source-context binding, append-before-apply guards, actor-known/debug unforgeability, content schema registration/prose-born-fact rejection, typed diagnostic discipline, TUI proof-seam gates, doc/invariant reference linting, and named conformance indices.
- Added the scoped acceptance artifact template at `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.

Deviations:

- The capstone conformance suite indexes existing named behavioral tests instead of duplicating their assertions inline.
- No live `docs/4-specs/SPEC_LEDGER.md` active-spec status was flipped because this staged spec was not listed as an active spec entry.

Verification:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
