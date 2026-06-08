# Spec 0008 — Phase 3A Anti-Contamination Hardening

Status: **REQUIRED / BLOCKING**  
Target repository: `joeloverbeck/tracewake`  
Target commit: `8e3cf3eccb94372b7873846ae952441fc1ca44d0`  
Spec type: requirements-first hardening specification  
Implementation posture: **do not implement here; do not decompose into tickets**

## 0. Evidence ledger

Requested repository: `joeloverbeck/tracewake`  
Target commit: `8e3cf3eccb94372b7873846ae952441fc1ca44d0`  
Freshness claim: user-supplied target commit only; not independently verified as latest `main`  
Manifest role: path inventory only  
Repository metadata used: no  
Default-branch lookup used: no  
Branch-name file fetch used: no  
Code search used: no  
Clone used: no  
URL fetch method: exact raw and exact blob URL fetches via `web.run.open` / `web.run.find`; no branch-name URLs  
Stale connector namespace observed: no  
Connector/tool namespace trusted as evidence: no  
Contamination observed in fetched content source: no  
Test suite run: no; the audit is static only

This workflow does **not** independently verify that the target commit is current `main`. All claims below are scoped to the exact user-supplied commit.

### 0.1 Fetched files

All repository evidence was fetched only from exact commit URLs of this form:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/<path>
```

or exact commit blob URLs of this form:

```text
https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/<path>
```

Material fetched files:

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/archive/specs/0007_PHASE_3A_SECOND_HARDENING_NO_HUMAN_ORDINARY_LIFE_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/archive/reports/PHASE_3A_IMPLEMENTATION_AUDIT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/archive/reports/PHASE_3A_STATUS_ERRATA.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/00_FOUNDATION_INDEX.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/01_PROJECT_CHARTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/4-specs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/docs/4-specs/SPEC_LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/scheduler.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/scheduler.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/need.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/intention.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/routine.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/htn.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/methods.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/planner.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/generation.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/agent/trace.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/pipeline.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/proposal.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/registry.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/defs/eat.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/defs/sleep.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/defs/work.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/defs/wait.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/events/apply.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/events/envelope.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/events/log.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/state.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/projections.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/debug_reports.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/view_models.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/replay/rebuild.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/src/replay/report.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/schema.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/validate.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/no_human_day_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/ordinary_workday_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/planner_trace_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/tests/acceptance_gates.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-core/tests/golden_scenarios.rs`
- `https://github.com/joeloverbeck/tracewake/blob/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/tests/golden_fixtures_run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/tests/fixtures_load.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-content/tests/forbidden_content.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/src/app.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/src/debug_panels.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/src/input.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/src/render.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/src/run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/tests/tui_acceptance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/tests/command_loop_session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/8e3cf3eccb94372b7873846ae952441fc1ca44d0/crates/tracewake-tui/tests/embodied_flow.rs`

## 1. Executive summary

Spec 0008 is required. Spec 0007 was directionally right and landed important scaffolding, but the target commit still permits exactly the kind of architectural contamination Phase 3A was supposed to prevent. The no-human path can still construct actor-known planning state from raw physical truth, route around the candidate/intention/HTN/local-planner chain with direct routine-family and need-threshold dispatch, attach decision/intention/routine explanation after the proposal instead of making it causally drive the proposal, and validate work constraints from caller-copied proposal parameters instead of authoritative live `AgentState`.

The current code contains useful parts: typed needs, typed intentions, typed routine structures, typed HTN conditions, shared pipeline routes for ordinary actions, Phase 3A event variants, replayable agent state, and capstone tests that run `run_no_human_day` once. Those parts are not enough. The architecture still has too many public seams where future Phase 3B / Phase 4 work can quietly build on physical-oracle planning, inert intentions, scripted routine labels, forged diagnostics, and string-only debug proof.

Spec 0008 therefore hardens Phase 3A around one canonical actor-decision transaction. The no-human scheduler must stop being a special-purpose ordinary-life brain. It may schedule actors and windows; it must not decide sleep/eat/work/movement by routine family, need thresholds, or raw `PhysicalState`. Every autonomous decision must pass through the same transaction shape:

```text
trigger
 -> actor-known planning context
 -> active intention lookup
 -> need/routine/project candidate generation
 -> intention continuation/adoption/interruption
 -> HTN/routine method selection
 -> bounded local planning
 -> ordinary proposal through shared pipeline
 -> eventful success/failure/replan/stuck trace
 -> live/replay projection agreement
```

Phase 3B and Phase 4 must remain blocked until this spec’s gates pass. This is not feature appetite. It is foundation protection.

## 2. What Spec 0005 originally intended

Spec 0005 intended the first executable ordinary-life autonomy substrate. Its intended causal chain was explicit:

```text
actor beliefs/needs/duties
 -> candidate goals
 -> selected durable intention
 -> HTN routine method
 -> concrete actions and bounded local planning from actor-known view
 -> shared proposal/action pipeline
 -> event log, replay, TUI, and debug explanation
```

The key commitments were:

1. **No hidden-truth planning.** Validators may inspect authoritative truth. Planners may not. An actor should not select a food source, route, workplace, sleep place, door, or container because the simulation knows the ground truth.
2. **Needs are pressures, not puppet strings.** Hunger, fatigue, safety, and duty should influence candidate/intention arbitration. They must not directly script action choices.
3. **Intentions are durable commitments.** An actor’s selected intention must persist and drive later proposals unless it completes, fails, is interrupted, or is explicitly replaced.
4. **Routines are causal machinery, not labels.** A workday routine should produce defeasible ordinary actions, not magic state changes or template-name dispatch.
5. **Ordinary actions go through the shared pipeline.** Sleep, eat, work, wait, move, and routine continuation should all use common validation/effects/event surfaces.
6. **Failures are eventful.** If the actor cannot eat, work, sleep, or continue a routine, the system must record a causally inspectable reason.
7. **Replay is authoritative.** Need changes, intention lifecycle, routine lifecycle, planning trace, stuck diagnostics, and no-human metrics must be reconstructible from the event log.
8. **Debug/TUI are non-authoritative.** They render event/projection state; they do not create truth.

That vision remains correct. Spec 0008 does not broaden it. Spec 0008 makes it hard to accidentally violate it.

## 3. What Specs 0006 and 0007 repaired

Spec 0006 correctly identified that the first Phase 3A implementation was too shape-matching: the no-human runner could wait instead of live ordinary life, the pipeline did not fully apply `AgentState`, HTN conditions were too string-heavy, hidden-truth protection was self-attested, intentions/routines were present but not sufficiently driving behavior, `continue_routine` could be mistaken for proof, diagnostics were too display-oriented, TUI proof was shallow, tests were smoke-heavy, and status docs overclaimed.

Spec 0007 tightened the standard again. It demanded no-human ordinary actions from live actor state, actor-known planning inputs, typed routine/need/intention decisions, HTN/local planning, shared validation, event ancestry, replay parity, and TUI/debug visibility. It explicitly forbade several shortcuts that the target commit still resembles:

- selecting food, workplace, sleep place, or movement destination from raw `PhysicalState`;
- constructing an empty `EpistemicProjection` and declaring actor-known safety;
- treating active intentions as absent when committed state exists;
- treating a successful wait or continuation marker as proof of routine progress;
- relying on string labels instead of typed decision and diagnostic records.

Specs 0006 and 0007 repaired some data structures and tests, but they did not force the final architecture into a single sealed path. The code still allows bypasses.

## 4. Doctrine and architecture alignment

The foundation docs are stricter than the implementation. The implementation must bend to the docs, not the other way around.

Relevant doctrine:

- `docs/README.md` says the foundation and architecture layers are authority docs; if implementation conflicts with a gate, the implementation is wrong.
- `docs/0-foundation/00_FOUNDATION_INDEX.md` centers explicit information flow, no hidden director, causality over drama, and review questions about what the actor could know.
- `docs/0-foundation/01_PROJECT_CHARTER.md` requires a world that can run coherently with no human, belief before truth, inspectable agents, durable intentions/projects, HTN routines, bounded planning, event-driven replanning, and debug traces.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` states that actors act from beliefs, records, memory, perception, relationships, plans, and bodily state, not hidden truth.
- `docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` defines the target chain: body/needs, beliefs/memories, candidate goals, durable intentions, HTN routines, local planning, and shared action pipeline. It explicitly denies ground-truth planning and schedule teleportation.
- `docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` says Phase 3 acceptance requires no-human ordinary life, replay of needs/routines/planner/stuck diagnostics, and no leakage; it blocks Phase 4 until Phase 3 passes.

External research supports the same shape:

- HTN planning is valuable because it encodes domain knowledge in structured task decomposition and method constraints; Tracewake should use HTN methods as typed decompositions, not as opaque strings or direct script dispatch.
- BDI planning literature stresses that recipe libraries alone are efficient but inflexible; stronger systems integrate intention management with runtime planning. Tracewake’s routines should be BDI commitments plus HTN/local planning, not fixed action scripts.
- GOAP-style game AI, including the `F.E.A.R.` architecture, is useful because proposed actions are planned under current state and still validated by world rules. Tracewake should preserve that separation: planning proposes; pipeline validates and applies.
- Explainable-agent literature emphasizes explanations over beliefs, desires, intentions, plans, and goals. Tracewake needs typed trace records, not post-hoc display strings.
- Provenance models distinguish entities, activities, agents, derivation, generation, usage, and attribution. Tracewake’s actor-known facts need comparable provenance instead of self-attesting tags.
- Property-based and mutation testing are appropriate because simple golden-label tests cannot prove the absence of hidden-oracle shortcuts.

## 5. Static code audit findings

### Finding 1 — the no-human path is still not one canonical cognition transaction

Severity: **Blocker**

In `crates/tracewake-core/src/scheduler.rs`, `run_no_human_day` calls `build_agent_proposal`. That function constructs planning state and first tries `build_routine_or_need_proposal`. The routine/need helper can select sleep, eat, work, or movement proposals directly from routine family or current need thresholds before the code falls back to candidate generation, intention selection, HTN method selection, and local planning.

This is the wrong dependency direction. Candidate/intention/routine/HTN planning should produce the proposal. In the target commit, the direct helper can produce the proposal first, and the scheduler then appends intention lifecycle, routine step, and decision trace events after pipeline execution.

This means the trace can become a narrative attached to an already-chosen action. It is not guaranteed to be the causal source of that action.

Required correction: replace no-human proposal construction with a single `ActorDecisionTransaction` or equivalent. The scheduler may select actor/time/window and trigger the transaction. It must not own ordinary-life decision policy.

### Finding 2 — `VisibleLocalPlanningState` is not a safe actor-known boundary in its current form

Severity: **Blocker**

The no-human builder constructs `EpistemicProjection::new(...)` and derives `visible_local_planning_state` directly from `PhysicalState`: current-place adjacency, local food supplies, sleep places, and actor-assigned workplaces. It then passes this into `build_actor_known_planning_state`, which records physical-derived facts as modeled actor-known facts.

This is not proof of actor knowledge. It is physical truth translated into actor-known-looking structures. The hidden-truth audit then trusts `Modeled` proof tags instead of a sealed provenance graph. That is self-attestation.

Some local perception from `PhysicalState` is legitimate, but only through a constrained perception/visibility interface that records why the actor could observe the fact. Raw `PhysicalState` cannot be a general planner input.

Required correction: introduce a sealed `ActorKnownPlanningContext` whose constructors distinguish observed-now facts, remembered/believed facts, routine/job-assignment facts, authored fixture possibility-space facts, validator-only physical truth, and debug-only omniscience. Planner facts must not be created from raw physical truth except through explicit visible-local observation functions that emit provenance.

### Finding 3 — no-human planning ignores Phase 2 epistemic projection where it matters most

Severity: **Blocker**

The TUI app holds an `epistemic_projection` and uses it for human pipeline submissions. The no-human run path, however, calls `run_no_human_day` without passing that projection. The scheduler then constructs an empty projection.

This contradicts the doctrine that agents act from belief, memory, observation, and records. It also creates a false sense of actor-known safety: the no-human planner has neither actual belief data nor an explicit admission that it is only local-perception-only.

Required correction: the autonomous transaction must receive the actor’s current epistemic projection or a narrower actor knowledge interface derived from it. If no projection is available, the transaction must record a typed limitation and avoid claims of belief-based actor-known proof.

### Finding 4 — needs and routines can still bypass BDI/HTN arbitration

Severity: **Blocker**

The scheduler can route `SleepNight` directly to `sleep_proposal`, `EatMeal` / `FindFood` directly to `eat_proposal`, and `GoToWork` / `WorkBlock` directly to `work_or_move_proposal`. It can also use hunger/fatigue thresholds to directly select eat/sleep proposals when no routine path is found.

This contaminates the BDI layer. Severe hunger or fatigue should interrupt routine duties through explicit candidate scoring, intention interruption/continuation/adoption, and event ancestry. Routines should contribute candidate duties and HTN methods. They should not be privileged script dispatch switches.

Required correction: routine windows, need pressures, current intentions, and projects all enter candidate generation and arbitration. The chosen candidate then drives intention lifecycle and method selection. Direct routine-family proposal constructors from scheduler are forbidden.

### Finding 5 — `work_block` validates forged/stale need echoes from proposal parameters

Severity: **Blocker**

`crates/tracewake-core/src/actions/defs/work.rs` validates fatigue/hunger constraints by reading `current_fatigue` and `current_hunger` from `proposal.parameters`. Missing or malformed values default to `0`. The scheduler also copies current need values into the work proposal.

This violates pipeline authority. Proposal parameters are caller-provided strings. They can be stale, forged, missing, or generated by a path that did not read the current `AgentState`. Validators must read authoritative current need values from the pipeline context.

Required correction: expose read-only `AgentState` through the validator/read context and make work validation use it. Proposals should carry actor choice inputs and target IDs, not live-state echoes. Tests must prove forged/missing/stale need parameters cannot bypass constraints.

### Finding 6 — typed traces exist, but authoritative state stores strings

Severity: **Blocker**

`agent/trace.rs` defines typed structures such as `DecisionTrace`, `HiddenTruthAudit`, and `StuckDiagnostic`, but `AgentState` stores `decision_traces` and `stuck_diagnostics` as `BTreeMap<..., String>`. `events/apply.rs` applies `DecisionTraceRecorded` and `StuckDiagnosticRecorded` by storing canonical strings.

The scheduler’s decision trace canonical string contains summary information and hard-coded actor-known assertions. This blocks robust future inspection. A string can be useful for checksum or display, but it cannot be the authoritative diagnostic state.

Required correction: make typed trace and stuck diagnostic records authoritative in live state and replay state. Events must carry enough structured data to rebuild them. Display strings must be derived at the edge.

### Finding 7 — `continue_routine` remains a marker, not behavioral progress

Severity: **Blocker**

The `continue_routine` action explicitly records `behavioral_progress=false`. That is correct for the marker itself. The problem is that integrated no-human continuation is not proven to resolve to a follow-on ordinary action in the same autonomous transaction. Existing tests can manually run a continuation and then a separate move/work/eat action; that does not prove the no-human runner itself cannot count a marker as progress.

Required correction: a routine continuation marker may be emitted only as part of a transaction that either commits a follow-on ordinary action through the shared pipeline or emits a typed blocked/replan/stuck diagnostic. No metric, acceptance gate, or debug panel may count `ContinueRoutineProposed` alone as ordinary progress.

### Finding 8 — action completions are not fully interleaved with decision transactions

Severity: **High / likely blocker for robust autonomy**

`run_no_human_day` collects scheduled sleep/work completions and appends completion events after window processing. That creates a temporal seam: later actor decisions in the same run may not observe the completed need/work effects at the correct point in the event sequence.

For Phase 3A, this may pass simple golden tests. For Phase 3B/4, it is unsafe. Durable routines and body-exclusive reservations require a single event-time model where scheduled completions due before the next decision are applied before that decision sees live state.

Required correction: the no-human runner must process due completions as eventful state transitions in chronological order relative to decision transactions. Replay and live state must agree after each transaction boundary, not merely after the final report.

### Finding 9 — stuck diagnostics are post-hoc, not transactional why-not records

Severity: **High / blocker if left as the only diagnostic path**

The no-human runner emits stuck diagnostics after it observes missing progress by actor/window. That can catch a stall, but it cannot reliably explain why an actor failed from actor-known uncertainty versus physical validation failure versus routine conflict versus reservation conflict.

Required correction: each failed or empty decision transaction must produce typed failure/replan/stuck records with structured reason, visible/known/provenance facts considered, candidate set, selected method if any, local planning failure if any, and pipeline rejection if any. Post-window aggregate diagnostics may remain as rollups, but they cannot be the only authoritative stuck explanation.

### Finding 10 — tests are better than smoke tests, but still too friendly

Severity: **Blocker**

The capstone test runs `run_no_human_day` once and checks many event labels. That is good progress. But it still checks labels and string absence more than causal ancestry. A test that says the log contains `DecisionTraceRecorded`, `RoutineStepStarted`, `FoodConsumed`, and no literal string `food_hidden` does not prove that the action was selected from actor-known facts.

Required correction: the tests must assert behavior-specific ancestry and typed provenance. They must fail if a developer reintroduces direct physical truth, direct routine-family dispatch, proposal need echoes, seeded debug strings, or marker-only routine continuation.

### Finding 11 — content validation does not yet make outcome-chain authoring impossible enough

Severity: **High**

The schema supports routine templates, assignments, day windows, homes, sleep places, food supplies, workplaces, initial beliefs, containers, doors, and affordances. That is appropriate. But Phase 3A validation needs stronger anti-script rules: fixtures must not smuggle intended outcomes into planner-visible inputs, routine labels, debug strings, or synthetic action chains.

Required correction: content validators must reject or flag fixture fields that encode authored outcome chains instead of starting conditions, especially if they preload route/food/work access facts without visible/belief provenance.

### Finding 12 — docs and ledger overclaim readiness

Severity: **Blocker**

The archived Spec 0007 and current ledger/status posture claim the second hardening closed Phase 3A ordinary-life proof. The target commit still violates several Spec 0007 forbidden shortcuts. The ledger must be corrected.

Required correction: replace `docs/4-specs/SPEC_LEDGER.md` and status errata to mark Phase 3A as still hardening. Phase 3B and Phase 4 remain blocked until Spec 0008 acceptance gates pass.

## 6. Defect list with severity

| ID | Severity | Defect | Required posture |
|---|---:|---|---|
| D-0008-01 | Blocker | No canonical actor-decision transaction; scheduler still owns direct proposal policy. | Replace with single transaction path. |
| D-0008-02 | Blocker | Actor-known state can be built from raw `PhysicalState` and self-attested `Modeled` facts. | Seal actor-known context and require provenance-rich construction. |
| D-0008-03 | Blocker | No-human path creates empty `EpistemicProjection` rather than using actor beliefs/memories/observations. | Pass actual epistemic/knowledge interface or explicitly limited perception-only context. |
| D-0008-04 | Blocker | Routine family and need thresholds can bypass candidate/intention/HTN arbitration. | Make needs/routines candidate inputs only; forbid direct scheduler dispatch. |
| D-0008-05 | Blocker | Work validation uses caller-copied string need parameters and defaults missing/malformed values to zero. | Validators read authoritative `AgentState`; proposal need echoes forbidden. |
| D-0008-06 | Blocker | Authoritative decision traces/stuck diagnostics are strings. | Store typed records; strings are derived display/checksum only. |
| D-0008-07 | Blocker | Continue-routine marker can be confused with progress. | Marker must be paired with ordinary action or typed blockage in same transaction. |
| D-0008-08 | High | Sleep/work completions are not interleaved in chronological transaction order. | Process due completions before later decisions observe state. |
| D-0008-09 | High | Stuck diagnostics are post-hoc rollups rather than transactional why-not records. | Emit structured failure/replan/stuck records from transaction. |
| D-0008-10 | Blocker | Tests assert labels/string absence more than typed causal ancestry and provenance. | Add adversarial fixtures, static guards, property/mutation tests, replay trace checks. |
| D-0008-11 | High | Fixture/content validation can still permit authored outcome chains. | Add anti-script validation tied to provenance and routine semantics. |
| D-0008-12 | Blocker | Ledger/status overclaim Phase 3A readiness. | Replace status docs; block Phase 3B/4. |

## 7. Blocking versus deferred scope

### 7.1 Blocking for Phase 3A exit

The following must be complete before Phase 3A can be called safe:

1. Canonical actor-decision transaction used by no-human ordinary life.
2. Sealed/provenance-rich actor-known planning context.
3. No direct scheduler proposal construction by routine family, need threshold, or raw physical state.
4. Actual epistemic/belief/observation input for autonomous planning, or explicit typed absence/limitation.
5. Needs/routines/intention lifecycle as causal drivers, not after-the-fact labels.
6. Work/eat/sleep validators reading authoritative state from pipeline context where relevant.
7. No proposal parameters for current live need values.
8. Typed decision traces and typed stuck diagnostics in live state, events, replay, debug reports, and TUI projections.
9. Continue-routine marker prohibited from counting as progress without follow-on ordinary action or typed blockage.
10. Chronological completion processing robust enough for live/replay parity.
11. Adversarial hidden-truth, stale-param, marker-only, replay, possession, and anti-script tests.
12. Replacement status/ledger docs.

### 7.2 Tolerable deferred scope

The implementation session must not expand into these areas:

1. Full Phase 4 institutions, records, courts, clerks, gossip, sanctions, or wrong-suspicion workflows.
2. Full Phase 3B speech/testimony. Minimal terms may be clarified only where needed to protect Phase 3A actor-known boundaries.
3. Wages, prices, debt, markets, ownership law, household economies, or mature work accounting.
4. Regional travel, roads, notices, bounties, companions, combat, injury, weather, disease, animals, graphical clients, or LLM brains.
5. A generic AI framework rewrite beyond what Phase 3A needs.
6. Perfect long-term memory modeling for every future fact type. Phase 3A only needs a sealed provenance channel sufficient for local perception, authored starting beliefs, routine/job facts, and ordinary-life choices.

## 8. Required architectural target state

### 8.1 Canonical actor-decision transaction

Introduce one canonical transaction, name flexible, shape mandatory:

```text
ActorDecisionTransaction
  input:
    - trigger: NoHumanWindow | PossessedActorCommand | ScheduledContinuation | ReplanAfterFailure | DueCompletionWakeup
    - actor_id
    - event_time / decision_window
    - physical read view available only to perception/validation subsystems
    - agent state
    - epistemic projection / actor knowledge interface
    - content manifest / routine templates
    - shared action registry/pipeline

  stages:
    1. apply due completions before this decision observes state
    2. build ActorKnownPlanningContext from explicit perception and epistemic facts
    3. load current needs, active intention, routine execution, reservations
    4. generate candidate goals from needs/routines/projects/current intention
    5. score/select continue/adopt/interrupt/drop intention
    6. select typed HTN/routine method
    7. run bounded local planner from actor-known context
    8. create ordinary proposal with typed origin and ancestry
    9. validate/apply through shared pipeline
    10. emit typed decision/routine/intention/failure/stuck records
    11. verify live/replay/projection invariants at transaction boundary
```

No-human scheduling becomes a caller of this transaction. It must not know that `SleepNight` maps to `sleep`, `EatMeal` maps to `eat`, or `WorkBlock` maps to `work_block`. It may define a trigger and time window. The transaction owns cognition.

### 8.2 Actor-known planning context

Replace or harden `ActorKnownPlanningState` / `VisibleLocalPlanningState` into a sealed `ActorKnownPlanningContext` or equivalent.

Mandatory properties:

1. Public fields are removed or made read-only through narrow accessors.
2. Constructors are restricted to module-private builders or explicit perception/epistemic assembly functions.
3. Each fact has stable identity, semantic kind, value, time, actor, and provenance.
4. Provenance distinguishes at minimum:
   - `ObservedNow`: visible local perception with observation event/projection reference;
   - `RememberedBelief`: belief/memory/proposition reference with confidence/stance/source;
   - `RoutineAssignment`: duty/job/home/sleep/work assignment known because of authored initial state or prior event;
   - `FixturePossibility`: authored possibility-space seed allowed only for fixture validation/local planning surfaces, not outcome selection;
   - `PipelineValidationTruth`: explicitly not constructible inside planner context;
   - `DebugOmniscience`: explicitly not constructible inside planner context.
5. Hidden-truth audit computes from the provenance graph. It does not trust booleans or free-form strings.
6. Planner APIs accept this context, not raw `PhysicalState` plus public mutable known lists.
7. Tests must fail to compile or fail at runtime if code attempts to construct actor-known facts directly from physical truth outside approved builders.

### 8.3 Candidate/BDI/HTN/local planning integration

Needs, routine windows, current intentions, and project hooks are inputs to candidate generation. They cannot directly create ordinary proposals.

Required behavior:

1. Current active intention is considered first for continuation.
2. Severe need pressures can interrupt routines, but only through explicit arbitration and emitted intention/routine events.
3. Routine duties create candidate goals and HTN methods.
4. HTN methods produce typed local-planner goals or primitive action intents.
5. Local planning proposes concrete ordinary actions using actor-known context.
6. The shared pipeline validates the proposal against authoritative world truth.
7. Pipeline failure returns structured reason to the transaction for replan/blockage/stuck diagnostics.

### 8.4 Proposal and pipeline authority

The pipeline remains the only path for world-affecting ordinary actions.

Required changes:

1. `PipelineReadContext` exposes read-only `AgentState` to validators.
2. Validators that need live agent state read it from the context, not proposal parameters.
3. Proposal parameters for live state echoes such as `current_hunger` and `current_fatigue` are forbidden.
4. Proposal parameters remain acceptable for actor choice inputs: target food source, target place, worksite ID, intended duration, selected method ID, trace/ancestry IDs.
5. All proposal origins must include enough ancestry to link decision trace, candidate, intention, method, local plan, and eventual event IDs.
6. No action validator may default missing critical state to safe success. Missing authoritative state is a failure, not zero hunger.

### 8.5 Typed diagnostics and replay

Typed diagnostic state is mandatory.

Required record classes, names flexible:

```text
DecisionTraceRecord
  trace_id
  actor_id
  trigger
  decision_window
  actor_known_context_id
  candidate_ids
  selected_candidate_id
  active_intention_before
  active_intention_after
  selected_method_id
  local_plan_id
  proposal_id
  pipeline_result_event_ids
  hidden_truth_audit_id
  outcome

StuckDiagnosticRecord
  diagnostic_id
  actor_id
  trigger
  decision_window
  transaction_stage
  failure_kind
  actor_known_uncertainties
  pipeline_validation_failures
  missing_affordances
  reservation_conflicts
  routine/intention references
  recommended next diagnostic action, if any
```

Canonical strings may remain, but only as derived serialization for checksum/display. Replay must rebuild typed records and compare them structurally.

### 8.6 Debug and TUI surfaces

Debug panels must be strictly read-only projections from typed event/projection state. They must not inspect hidden physical truth to explain actor decisions except in explicitly labeled omniscient debug comparison panels.

The default player-facing and actor-debug panels must answer:

1. What did the actor know or believe?
2. Which need/routine/intention candidate won?
3. Which HTN method and local plan were selected?
4. Which pipeline validator accepted or rejected the proposal?
5. Was failure caused by actor-known uncertainty, hidden world truth, access rules, reservation/body constraints, or authored content invalidity?

## 9. Required code-surface changes by subsystem

This section names surfaces and required outcomes. It is not a patch and not a ticket decomposition.

### 9.1 `crates/tracewake-core/src/scheduler.rs`

Required:

- Remove direct ordinary proposal construction from no-human scheduling.
- Remove routine-family dispatch from the scheduler.
- Remove need-threshold direct eat/sleep proposal paths from the scheduler.
- Remove construction of empty `EpistemicProjection` as no-human proof.
- Stop building `VisibleLocalPlanningState` directly from raw `PhysicalState` in scheduler policy code.
- Call the canonical actor-decision transaction for each actor/window.
- Process due completions before later decisions observe live state.
- Emit no-human metrics from transaction outcomes, not label counts.
- Treat no proposal as a typed transaction failure, not silent continue.

Forbidden in this subsystem after Spec 0008:

```text
EpistemicProjection::new(...) inside no-human cognition path
build_routine_or_need_proposal as direct policy
SleepNight -> sleep_proposal
EatMeal / FindFood -> eat_proposal
GoToWork / WorkBlock -> work_or_move_proposal
live hunger/fatigue threshold -> proposal without candidate/intention ancestry
hard-coded actor_known_only=true
post-hoc trace as only explanation
```

### 9.2 `crates/tracewake-core/src/agent/planner.rs`

Required:

- Replace public mutable actor-known lists with sealed/provenance-bearing context.
- Split visible-local perception from remembered/believed knowledge.
- Ensure local physical facts can be admitted only through explicit perception functions.
- Make hidden-truth audit inspect provenance graph.
- Add compile-time and test-time red flags for direct construction.

### 9.3 `agent/generation.rs`, `agent/intention.rs`, `agent/routine.rs`, `agent/htn.rs`, `agent/methods.rs`

Required:

- Make candidate generation the only entry point for need/routine/project pressures.
- Ensure active intentions are durable commitments and are not bypassed by routine family.
- Emit explicit continuation, interruption, completion, failure, and replacement records.
- Keep HTN method preconditions typed. No return to free-form substring conditions.
- Ensure routine methods produce local-planner goals or primitive intents with actor-known prerequisites.
- Prevent `routine_continue_current_intention` from becoming a no-op progress marker.

### 9.4 `actions/pipeline.rs`, `actions/proposal.rs`, `actions/defs/work.rs`, `actions/defs/eat.rs`, `actions/defs/sleep.rs`, `actions/defs/wait.rs`, `actions/defs/continue_routine.rs`

Required:

- Add read-only `AgentState` to validation context.
- Remove work validation dependence on `proposal.parameters["current_hunger"]` and `proposal.parameters["current_fatigue"]`.
- Reject or ignore proposal-provided live-state echoes.
- Preserve proposals for actor choices and target IDs only.
- Make `ContinueRoutineProposed` explicitly non-progress in all metrics.
- Require follow-on ordinary action ancestry for continuation success.
- Ensure sleep/eat/work completion effects apply live and replay state equivalently at event-time boundaries.
- Make body-exclusive reservations visible to transaction failure diagnostics.

### 9.5 `events/envelope.rs`, `events/apply.rs`, `events/log.rs`, `state.rs`, `replay/rebuild.rs`, `replay/report.rs`

Required:

- Add structured payloads or typed event variants sufficient to rebuild decision traces and stuck diagnostics.
- Change `AgentState` trace/diagnostic maps from strings to typed records.
- Preserve deterministic canonical serialization as a derived representation.
- Replay must rebuild typed records and no-human metrics, then compare live/replay state structurally.
- Event application must maintain causal ancestry IDs linking transaction, candidates, intentions, methods, local plans, proposals, and pipeline events.

### 9.6 `debug_reports.rs`, `view_models.rs`, TUI debug panels

Required:

- Render from typed records.
- Remove substring/string-canonical assertions as primary proof.
- Add panels or report fields for actor-known uncertainty versus ground-truth validation failure.
- Keep omniscient debug comparison explicitly labeled and non-authoritative.

### 9.7 `tracewake-content` schema, validation, fixtures

Required:

- Prevent fixture-authored outcome chains.
- Require actor-known provenance for any initial belief/workplace/sleep/food/route facts intended for autonomous planning.
- Add adversarial fixtures where hidden physical truth exists but cannot be used.
- Ensure routine assignments are duties/starting commitments, not scripts that imply guaranteed outcomes.
- Validate that ordinary-life fixtures contain starting conditions, not pre-written no-human event sequences.

## 10. Required fixture and test gates

### 10.1 Integrated no-human capstone gate

The capstone must still run `run_no_human_day` once. It must now prove typed causal ancestry:

```text
NoHumanWindow trigger
 -> DecisionTraceRecord
 -> ActorKnownPlanningContext proof graph
 -> candidate ids
 -> selected intention lifecycle event
 -> HTN method id
 -> local plan id
 -> proposal id
 -> shared pipeline accepted/rejected event ids
 -> routine/need effects
 -> replay rebuilt typed records
```

The test must fail if decision trace records are inserted after an unrelated proposal, if proposal origin lacks ancestry, or if typed trace fields do not match event IDs.

### 10.2 Hidden-truth adversarial gates

Add fixtures/tests that fail if planners can use hidden truth:

1. Hidden food exists in a closed/private container at the actor’s current place. The actor has not observed it and has no belief. The planner must not select it for `eat`.
2. Hidden food exists at a nearby place reachable through a route unknown to the actor. The actor must not move toward it unless the route/food is observed or believed.
3. A workplace exists in `PhysicalState` but is not known to the actor except as an assignment fact with explicit provenance. The planner must not select it without that provenance.
4. A route edge exists physically but is not visible/known under the actor’s context. The local planner must not use it.
5. Debug-only omniscient facts exist for a report. They must not appear in `ActorKnownPlanningContext`.

The assertion must inspect proof provenance, not display strings.

### 10.3 Need and routine arbitration gates

Add tests where:

1. A routine duty is active but hunger becomes severe. The actor interrupts or suspends the routine through explicit candidate/intention/routine events before eating or searching for food.
2. A routine duty is active but fatigue becomes severe. The actor interrupts through explicit events before sleeping.
3. A mild need does not automatically override a committed routine.
4. The active intention survives possession/unpossession and resumes afterward unless interrupted by an eventful reason.
5. Routine steps fail and replan through typed failure causes, not labels.

### 10.4 Work validator stale/forged parameter gates

Add tests where:

1. The proposal claims `current_fatigue=0`, but authoritative `AgentState` fatigue is above the work maximum. The pipeline must fail work.
2. The proposal omits `current_hunger/current_fatigue`. The validator must not default to zero and accept.
3. The proposal contains malformed need parameters. The validator must ignore or reject them; it must not turn them into safe values.
4. A stale proposal created before a need delta must be rejected if current state now violates thresholds.

### 10.5 Continue-routine gates

Add tests where:

1. A transaction emits `ContinueRoutineProposed` but cannot produce follow-on ordinary action. Metrics must count no behavioral progress and must emit a typed blockage.
2. A transaction emits `ContinueRoutineProposed` and then a move/work/eat/sleep action with matching ancestry. Only the ordinary action counts as progress.
3. A run containing only continuation markers fails the ordinary-life gate.

### 10.6 Typed diagnostics and replay gates

Add tests where:

1. Live and replay typed `DecisionTraceRecord` maps are equal.
2. Live and replay typed `StuckDiagnosticRecord` maps are equal.
3. Display strings are regenerated from typed records and are not stored as the only source.
4. Tampering with canonical string text does not change typed facts; tampering with typed facts changes checksum/replay comparison.
5. Why-not output distinguishes actor-known uncertainty from validator-known physical failure.

### 10.7 Static/anti-regression guard gates

Add tests or lints that fail on forbidden shortcut reintroduction:

```text
scheduler/no-human path constructs EpistemicProjection::new as actor-known proof
scheduler/no-human path directly calls sleep/eat/work proposal builders
scheduler/no-human path dispatches on RoutineFamily to primitive action
work/eat/sleep validators read current need values from Proposal.parameters
AgentState stores decision_traces or stuck_diagnostics as String
actor_known_only is hard-coded true in scheduler
ActorKnownPlanningState/Context fields allow arbitrary public construction
hidden-truth audit trusts tags without provenance graph
ContinueRoutineProposed contributes to ordinary progress count by itself
```

Mutation tests should deliberately introduce each shortcut and confirm the suite fails.

### 10.8 Content validation gates

Add fixtures/tests that reject:

1. Routine templates that encode guaranteed outcome chains rather than defeasible methods.
2. Fixture fields that preload hidden food/workplace/route facts as actor-known without provenance.
3. Debug labels intended to satisfy acceptance tests.
4. Player/protagonist-conditioned ordinary-life content.
5. Routine step names or strings that imply action success without shared pipeline events.

## 11. Required documentation/status changes

The implementation session must produce complete replacement docs, not diffs.

Required replacements included in this package:

1. `docs/4-specs/SPEC_LEDGER.md` replacement.
2. `archive/reports/PHASE_3A_STATUS_ERRATA.md` replacement.
3. New architecture clarification doc: `docs/1-architecture/14_ACTOR_KNOWN_AUTONOMY_TRANSACTION.md`.

Required status claims:

- Spec 0005 remains the original Phase 3A implementation intent.
- Spec 0006 remains a first hardening pass, now superseded by stronger gates.
- Spec 0007 remains a second hardening pass, but its readiness claim is corrected at this target commit.
- Spec 0008 is blocking for Phase 3A exit.
- Phase 3B and Phase 4 must not build on the current target commit as if Phase 3A is safe.

## 12. Explicit non-goals

Spec 0008 must not implement or require:

- Phase 4 institutions, reports, records, courts, gossip, sanctions, clerks, or wrong-suspicion workflows.
- Full Phase 3B speech/testimony.
- Full economy, wages, markets, debt collection, payment ledgers, prices, or ownership law.
- Mature households beyond minimal home/food/sleep/work access already needed by Phase 3A.
- Notices, bounties, regional travel, roads, companions, combat, injury, weather, disease, animals, or graphical clients.
- LLM brains, LLM dialogue, or LLM-authored world state.
- Generic AI-framework rewrite.
- Ticket decomposition.

## 13. Acceptance checklist

Phase 3A may be called safe only when all of the following are true:

- [ ] No-human ordinary actions are produced only through the canonical actor-decision transaction.
- [ ] No no-human planner path constructs actor-known state from raw `PhysicalState` without provenance-producing perception/knowledge interfaces.
- [ ] No no-human planner path uses an empty `EpistemicProjection` as actor-known proof.
- [ ] Needs, routines, and projects enter candidate generation and intention arbitration; they do not directly dispatch primitive actions.
- [ ] Active intentions are durable, replayable commitments that drive later proposals.
- [ ] Routine executions can resume, fail, interrupt, complete, and explain progress through typed events.
- [ ] Severe needs interrupt routine duties only through explicit candidate/intention/routine events.
- [ ] `continue_routine` alone is never counted as behavioral progress.
- [ ] All world-affecting actions use the shared pipeline.
- [ ] Validators read authoritative state from validation context, not caller-provided live-state echoes.
- [ ] `work_block` cannot be tricked by forged, stale, missing, or malformed need parameters.
- [ ] Sleep/eat/work completions update live and replay state in chronological transaction order.
- [ ] Decision traces and stuck diagnostics are typed authoritative state.
- [ ] Debug/TUI surfaces derive from typed records.
- [ ] Why-not output distinguishes actor-known uncertainty from ground-truth validation failure.
- [ ] Replay rebuilds physical, epistemic, agent, intention, routine, decision-trace, stuck-diagnostic, and no-human metric state.
- [ ] Capstone no-human test proves integrated causal ancestry, not labels.
- [ ] Hidden-truth adversarial fixtures fail if physical truth leaks into planning.
- [ ] Static/anti-regression tests fail on forbidden shortcuts.
- [ ] Content validation rejects authored outcome chains.
- [ ] Ledger/status docs mark Phase 3A as hardened only after these gates pass.

## 14. Implementation constraints

The future implementation session must obey these constraints:

1. Prefer correct architecture over preserving partial convenience layers.
2. Do not add more tests around the current bypassing shape and call that hardening.
3. Do not keep direct scheduler dispatch behind a wrapper; remove it from the autonomous decision path.
4. Do not make actor-known proof a boolean.
5. Do not store authoritative diagnostic records as display strings.
6. Do not make proposal parameters a side channel for live state.
7. Do not count no-human progress by labels alone.
8. Do not mark Phase 3A complete until adversarial and mutation/anti-regression gates pass.
9. Keep replacement docs complete and downloadable.
10. Preserve deterministic replay and checksum discipline.

## 15. Risks if not fixed

If Spec 0008 is skipped, later work will inherit bad foundations:

- Phase 3B speech/testimony will accidentally describe omniscient choices as actor beliefs.
- Phase 4 institutions and records will evaluate conduct generated by hidden world truth.
- Debug panels will show convincing strings that are not causal explanations.
- Routines will become scripts by another name.
- Needs will become puppet strings instead of pressures.
- Tests will continue to pass because labels exist, even when the architecture is wrong.
- Replay will reproduce strings rather than inspectable cognition.
- Future contributors will treat the no-human runner as the correct pattern and spread the shortcut.

That is the contamination this spec is meant to stop.

## 16. Required audit-question answers

### Actor-known / hidden-truth boundary

1. **Can the autonomous proposal generator select food, workplace, route, door, container, sleep place, or movement destination from raw `PhysicalState` without a modeled actor-known/visible-local channel?**  
   Yes. The current no-human builder derives adjacent places, local food supplies, sleep places, and assigned workplaces from raw `PhysicalState`, then direct proposal helpers use those lists. This is a blocker.

2. **Is `VisibleLocalPlanningState` a valid actor-known boundary, or does it smuggle too much physical truth?**  
   In its current form it smuggles too much physical truth. It may become valid only if produced by a constrained perception interface with explicit provenance.

3. **Does actor-known planning use actual epistemic beliefs/memories/observations where Phase 2 already provides them?**  
   Not sufficiently. The no-human path creates an empty `EpistemicProjection`; the TUI owns a projection but does not pass it into `run_no_human_day`.

4. **Can hidden food, hidden routes, hidden container contents, closed/private containers, or non-visible workplaces affect no-human decisions?**  
   The current code limits some surfaces by current-place lists, but the boundary is not safe. Physical local food, adjacency, and assigned workplaces can affect decisions without enough modeled observation/belief provenance. Closed/private container semantics are not strongly represented in the visible-local builder. Blocker.

5. **Is hidden-truth audit derived from proof provenance or merely from tags/booleans/strings?**  
   It is effectively tag/self-attestation based. It trusts `Modeled` proof sources that can be created from physical-derived facts. Blocker.

### Intentions and routines

6. **Are intentions durable live commitments that drive later proposals?**  
   The data model supports durability, but the no-human proposal path can bypass active intention logic and append lifecycle events after proposal execution. Not acceptable.

7. **Are routine executions eventful and stateful enough to resume, fail, interrupt, complete, and explain progress?**  
   The type/event scaffolding exists, but runtime behavior is still too after-the-fact and direct-dispatch heavy. It must be made causally authoritative.

8. **Does no-human behavior actually flow through candidate generation, intention selection/continuation, HTN method selection, and local planning?**  
   Only on fallback paths. The direct routine/need helper can bypass that chain. Blocker.

9. **Can severe needs interrupt routine duties through explicit intention/routine events?**  
   Not reliably. Direct thresholds can select eat/sleep without explicit interruption ancestry. Blocker.

10. **Can `continue_routine` be mistaken for progress?**  
   Yes, unless tests and metrics explicitly prohibit it. The action itself says it is non-progress; the integrated no-human path must enforce that.

### Actions and pipeline

11. **Do all world-affecting actions pass through the shared pipeline?**  
   Mostly yes for ordinary proposals, which is good. But the selection architecture before the pipeline remains contaminated, and completion timing needs hardening.

12. **Do action validators read authoritative state from pipeline context rather than caller-provided state echoes?**  
   Not for work need constraints. Pipeline validation context lacks read-only `AgentState`, and `work_block` reads proposal parameters. Blocker.

13. **Does `work_block` validate need constraints safely?**  
   No. It reads `current_fatigue/current_hunger` from string parameters and defaults missing/malformed values to zero. Blocker.

14. **Do sleep/eat/work completion effects apply to live agent state and replay state equivalently?**  
   Some tests cover equivalence, but completion scheduling is not interleaved at transaction boundaries. It is not robust enough for the intended architecture.

15. **Are body-exclusive reservations and duration completions robust?**  
   They are partially represented. Spec 0008 requires chronological completion processing and typed failure diagnostics to make them robust.

### Events, replay, and diagnostics

16. **Is every meaningful need/intention/routine/decision/failure state change eventful or replay-reconstructible?**  
   Partially. Needs/intentions/routines have events, but decision/stuck records are string-only and some traces are post-hoc.

17. **Does replay rebuild physical, epistemic, agent, intention, routine, decision-trace, stuck-diagnostic, and no-human metric state?**  
   It rebuilds `AgentState` including strings, and tests compare final checksums/state. It does not yet prove typed diagnostic/provenance reconstruction.

18. **Are traces/stuck diagnostics typed enough to support future TUI/debug projections and tests?**  
   No. Typed structs exist, but authoritative storage is strings. Blocker.

19. **Are debug panels non-authoritative and derived from event/projection state?**  
   Largely yes in posture, but they derive from string-heavy state. They need typed diagnostic backing.

20. **Can why-not/debug output distinguish actor-known uncertainty from ground truth failure?**  
   Not sufficiently. Spec 0008 requires structured failure kinds and provenance.

### Tests and fixtures

21. **Does the capstone run `run_no_human_day` once and prove integrated behavior without synthetic post-run proposals?**  
   It does run once, which is good. It does not yet prove the integrated causal architecture strongly enough.

22. **Are the fixtures adversarial enough?**  
   No. They must specifically attack hidden food, hidden routes, hidden workplaces, private containers, marker-only continuation, stale proposal need params, and seeded debug strings.

23. **Are tests proving behavior-specific ancestry rather than labels?**  
   Insufficiently. They rely heavily on event-kind presence and substring checks.

24. **Are forbidden shortcut tests present?**  
   Not enough. Add static/anti-regression guards and mutation-style tests.

25. **Are content validation tests strong enough to prevent authored outcome chains?**  
   Not yet. They must reject fixtures that encode outcomes as routine/debug/planner facts rather than starting conditions.

### Documentation/status

26. **Does `SPEC_LEDGER.md` overclaim Spec 0007 readiness?**  
   Yes. The target commit still violates Spec 0007’s own architectural prohibitions.

27. **Do docs need replacement to mark Phase 3A as still hardening rather than safe?**  
   Yes. Replace ledger and status errata.

28. **Does architecture documentation need clarification to prevent future misuse?**  
   Yes. Add an actor-known autonomy transaction clarification doc.

29. **Is the execution ladder still accurate after the hardening?**  
   The ladder is accurate; the status is not. Phase 3 has not exited.

30. **Should Phase 3B or Phase 4 be blocked until Spec 0008 gates pass?**  
   Yes. Absolutely.

## 17. Deliverables required from the implementation session

The future implementation session must produce:

1. Canonical actor-decision transaction or equivalent.
2. Sealed/provenance-rich actor-known planning context.
3. Updated scheduler/no-human runner that delegates decisions to the transaction.
4. Updated pipeline validation context with authoritative read-only `AgentState`.
5. Updated work/eat/sleep/continue validation and effect paths where required.
6. Typed decision trace and stuck diagnostic state, events, replay, debug reports, and TUI projections.
7. Chronological completion handling at transaction boundaries.
8. Adversarial fixtures and tests described in this spec.
9. Static/anti-regression tests for forbidden shortcuts.
10. Complete replacement docs: ledger, status errata, and actor-known autonomy architecture clarification.

No implementation ticket breakdown is part of this specification.
