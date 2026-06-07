# Phase 3A Implementation Audit — Needs, Routines, and No-Human Day

Repository: `joeloverbeck/tracewake`  
Target commit: `ec1e73f91b7330d87ce12ae93f8cf57ea3671306`  
Audit date: 2026-06-07

## 1. Evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: ec1e73f91b7330d87ce12ae93f8cf57ea3671306
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run.open / web.run.find / web.run.screenshot against exact raw or exact blob URLs; container curl was attempted but failed before returning content and was not used as evidence
Connector/tool namespace trusted as evidence: no
Stale connector namespace waiver applied: yes
```

The uploaded manifest was used only to identify path candidates. No branch name, default branch, connector label, repository-scoped search result, or prior-chat memory was used as source authority.

### Fetched repository files

The audit used exact commit URLs under this prefix:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/ec1e73f91b7330d87ce12ae93f8cf57ea3671306/
```

Material fetched paths:

```text
docs/README.md
docs/0-foundation/00_FOUNDATION_INDEX.md
docs/0-foundation/01_PROJECT_CHARTER.md
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md
docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md
docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md
docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md
docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md
docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md
docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md
docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md
docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md
docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md
docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md
docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md
docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md
docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md
docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md
docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md
docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md
docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md
docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md
docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
docs/3-reference/01_DESIGN_RISK_REGISTER.md
docs/3-reference/02_GLOSSARY.md
docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md
docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
docs/4-specs/0001_RESEARCH_NOTES.md
docs/4-specs/README.md
docs/4-specs/SPEC_LEDGER.md
archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md
crates/tracewake-core/src/agent/candidate.rs
crates/tracewake-core/src/agent/decision.rs
crates/tracewake-core/src/agent/generation.rs
crates/tracewake-core/src/agent/htn.rs
crates/tracewake-core/src/agent/intention.rs
crates/tracewake-core/src/agent/methods.rs
crates/tracewake-core/src/agent/mod.rs
crates/tracewake-core/src/agent/need.rs
crates/tracewake-core/src/agent/planner.rs
crates/tracewake-core/src/agent/routine.rs
crates/tracewake-core/src/agent/trace.rs
crates/tracewake-core/src/actions/defs/accuseprobe.rs
crates/tracewake-core/src/actions/defs/checkcontainer.rs
crates/tracewake-core/src/actions/defs/continue_routine.rs
crates/tracewake-core/src/actions/defs/eat.rs
crates/tracewake-core/src/actions/defs/inspect.rs
crates/tracewake-core/src/actions/defs/mod.rs
crates/tracewake-core/src/actions/defs/movement.rs
crates/tracewake-core/src/actions/defs/openclose.rs
crates/tracewake-core/src/actions/defs/sleep.rs
crates/tracewake-core/src/actions/defs/takeplace.rs
crates/tracewake-core/src/actions/defs/wait.rs
crates/tracewake-core/src/actions/defs/work.rs
crates/tracewake-core/src/actions/mod.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/proposal.rs
crates/tracewake-core/src/actions/registry.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/controller.rs
crates/tracewake-core/src/debug_reports.rs
crates/tracewake-core/src/events/apply.rs
crates/tracewake-core/src/events/envelope.rs
crates/tracewake-core/src/events/log.rs
crates/tracewake-core/src/events/mod.rs
crates/tracewake-core/src/replay/mod.rs
crates/tracewake-core/src/replay/rebuild.rs
crates/tracewake-core/src/replay/report.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/state.rs
crates/tracewake-core/src/time.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/tests/acceptance_gates.rs
crates/tracewake-core/tests/golden_scenarios.rs
crates/tracewake-content/src/fixtures/container_item_move_001.rs
crates/tracewake-content/src/fixtures/debug_attach_001.rs
crates/tracewake-content/src/fixtures/door_access_001.rs
crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs
crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs
crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs
crates/tracewake-content/src/fixtures/mod.rs
crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
crates/tracewake-content/src/fixtures/no_human_advance_001.rs
crates/tracewake-content/src/fixtures/no_human_day_001.rs
crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs
crates/tracewake-content/src/fixtures/ordinary_workday_001.rs
crates/tracewake-content/src/fixtures/planner_trace_001.rs
crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
crates/tracewake-content/src/fixtures/possession_parity_001.rs
crates/tracewake-content/src/fixtures/replay_item_location_001.rs
crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs
crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs
crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs
crates/tracewake-content/src/fixtures/strongbox_001.rs
crates/tracewake-content/src/fixtures/view_filtering_001.rs
crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
crates/tracewake-content/src/schema.rs
crates/tracewake-content/src/validate.rs
crates/tracewake-content/tests/fixtures_load.rs
crates/tracewake-content/tests/forbidden_content.rs
crates/tracewake-content/tests/golden_fixtures_run.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/src/debug_panels.rs
crates/tracewake-tui/src/input.rs
crates/tracewake-tui/src/lib.rs
crates/tracewake-tui/src/main.rs
crates/tracewake-tui/src/render.rs
crates/tracewake-tui/src/run.rs
crates/tracewake-tui/src/transcript.rs
crates/tracewake-tui/tests/command_loop_session.rs
crates/tracewake-tui/tests/embodied_flow.rs
crates/tracewake-tui/tests/readme_sample_session.rs
crates/tracewake-tui/tests/transcript_snapshot.rs
crates/tracewake-tui/tests/tui_acceptance.rs
.claude/skills/brainstorm/SKILL.md
.claude/skills/brainstorm/references/triage-and-deliverables.md
.claude/skills/reassess-spec/SKILL.md
.claude/skills/reassess-spec/references/codebase-validation.md
.claude/skills/reassess-spec/references/foundation-alignment.md
.claude/skills/reassess-spec/references/spec-writing-rules.md
.claude/skills/spec-to-tickets/SKILL.md
.claude/skills/spec-to-tickets/references/codebase-validation.md
```

Additional exact blob/raw attempts:

```text
https://github.com/joeloverbeck/tracewake/blob/ec1e73f91b7330d87ce12ae93f8cf57ea3671306/crates/tracewake-core/src/projections.rs
https://raw.githubusercontent.com/joeloverbeck/tracewake/ec1e73f91b7330d87ce12ae93f8cf57ea3671306/crates/tracewake-core/src/projections.rs
```

Failed exact fetch:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/ec1e73f91b7330d87ce12ae93f8cf57ea3671306/crates/tracewake-tui/src/launch.rs
https://github.com/joeloverbeck/tracewake/blob/ec1e73f91b7330d87ce12ae93f8cf57ea3671306/crates/tracewake-tui/src/launch.rs
```

Both exact URLs for `crates/tracewake-tui/src/launch.rs` returned 404. The file was present in the uploaded manifest, but no content from that path was used.

## 2. Source-scope limitations

I did not clone the repository and did not run `cargo test`. The audit is source/static plus exact-commit file inspection. Some Rust files are formatted by the raw viewer as single-line minified output; where enough content was visible, I used it. `crates/tracewake-core/src/projections.rs` and `crates/tracewake-content/src/validate.rs` were not fully inspectable through the exact raw/blob fetches. Findings below avoid relying on unverified internals of those files.

The exact GitHub blob page was opened for `projections.rs` only as an exact-commit file-content fallback. Repository page chrome such as stars, default branch, latest commit, or repository metadata was ignored and not used as evidence.

## 3. Executive verdict

**Verdict: `FAIL / DO NOT PROCEED TO NEXT PHASE`.**

Phase 3A has real planner-shaped and ordinary-life-shaped modules: needs, intention structs, routine templates, candidate goals, decision traces, local planner scaffolding, eat/sleep/work/wait actions, replay application for agent events, debug reports, content fixtures, and TUI debug panels. That is not enough.

The archival status is not behaviorally earned. The accepted no-human day is effectively a scheduler scaffold that submits `wait` actions, not an autonomous ordinary-life day. It passes empty needs, no active intention, no routine goal, and empty actor-known route/food inputs into the planner. The shared action pipeline mutates `PhysicalState`; agent-state mutation from `NeedDeltaApplied`, intention, routine, trace, and stuck events is handled by a separate `apply_agent_event` path used during replay/projection rebuild, not by the live action pipeline. As a result, Phase 3A’s central substrate does not yet exist as one live causal loop.

The current implementation is therefore **nominally complete as scaffolding, but materially failed as a foundation substrate**. Because the execution ladder says Phase 4 may start only when Phase 3 exits, later work should be blocked until a corrective hardening spec passes.

## 4. What archived Spec 0005 intended

The archived 0005 spec was not a mere file-creation spec. It intended the first AI/ordinary-life substrate:

- persistent needs that affect behavior and change by time/action events;
- durable intentions and defeasible routines;
- sleep/eat/work/wait/continue actions through the shared proposal/action/event pipeline;
- bounded HTN/GOAP-like planning from actor-known state;
- no-human day advancement with actors waking, eating, moving, working, resting, sleeping, failing, replanning, and emitting diagnostics;
- replay/debug visibility for needs, routines, plans, failures, hidden-truth audits, stuck conditions, and no-human metrics;
- TUI reachability for Phase 3A state and debug panels;
- fixtures proving food unavailable replan, blocked routines, no routine teleportation, no hidden truth planning, possession not resetting intention, and full no-human day behavior.

The archived outcome says completed and lists the phase as landed. That statement conflicts with the exact-commit implementation evidence.

## 5. What the code actually proves

The code proves these positive points:

- `NeedState`, `NeedBand`, `NeedChangeCause`, and `NeedPressure` exist.
- `Intention` has durable status/source/trace fields and lifecycle operations.
- `RoutineTemplate`, `RoutineExecution`, `RoutineStep`, and routine families exist.
- Candidate generation and decision-selection scaffolds exist and are deterministic.
- A local planner has an `ActorKnownPlanningState` input type and can plan small direct/local actions when supplied honest known inputs.
- Eat, sleep, work, wait, and continue-routine action definitions exist and can emit relevant events.
- Replay can rebuild an `AgentState` projection from agent-stream events.
- Debug reports and TUI debug panels can render needs, routines, planner, stuck, actor, and no-human metrics views.
- Content fixtures encode ambitious Phase 3A contracts.

The code does **not** prove the required integrated behavior:

- no-human day does not read live agent needs, active intentions, or routine windows;
- no-human day does not produce an ordinary sequence of wake/eat/move/work/rest/sleep through autonomous choice;
- normal action pipeline does not apply agent events to live `AgentState`;
- TUI action submission does not update TUI-owned `AgentState` from agent events;
- HTN method applicability accepts important conditions by string prefix or substring matching;
- hidden-truth safety is mostly a convention of supplied inputs, not impossible by construction;
- tests validate smoke behavior, synthetic logs, and rendered panels more than integrated adversarial behavior.

## 6. Alignment matrix

| Obligation | Authority | Exact-commit evidence | Audit result |
|---|---|---|---|
| Meaningful state changes must arise from events and replay must reproduce causal state. | Foundation event/replay contract; architecture event log contract. | Agent events exist and replay can apply them, but live pipeline applies only `PhysicalState` world events. `apply_agent_event` is separate. | **Fail** for live Phase 3A state. |
| Agents act from actor-known/believed state, not ground truth. | Foundation claims/beliefs/planning docs; Phase 2 and Phase 3 execution gates. | Planner input type is actor-known, but no-human runner constructs inputs from physical actor location and hard-coded strings; HTN strings auto-pass. | **Fail / high risk**. |
| Ordinary-life actions are real substrate, not cosmetic. | Foundation actions/ordinary-life docs; architecture ordinary-life docs. | Sleep/eat/work actions exist, but no-human day runs `wait`; tests do not prove daily action chain. | **Fail**. |
| Routines are embodied action chains, not teleports or schedule rails. | Foundation agent/routine docs; architecture scheduler docs. | Routine types and anti-teleport string tests exist; no-human runner ignores routines; `continue_routine` records a marker rather than executing next ordinary action. | **Fail**. |
| No-human day must demonstrate wake/eat/work/rest/sleep/move/failure/replan. | Execution Phase 3 gate and archived 0005. | `run_no_human_day` loops actors/windows and builds wait proposals. | **Blocker fail**. |
| Planner/HTN must have real applicability/precondition proof and bounded local action planning. | Architecture agent cognition docs; research lens. | `template_applicable` treats `actor_has_*`, `active_intention`, and `reason_available` as true by prefix/string; local planner is direct-scaffold rather than full action-sequence search. | **High defect**. |
| Debug/TUI must be hard gates. | Foundation TUI/debug docs; architecture validation docs. | Panels render synthetic/seeded data; TUI can debug no-human metrics but has no command to run a no-human day; live agent state can go stale. | **Fail / high debt**. |
| Tests must reject shortcuts. | Execution testing/metrics docs. | Core no-human acceptance accepts `ActorWaited`; no-human metrics golden test manually constructs log; fixture contracts are not enforced as behavioral proofs. | **Fail**. |
| Spec ledger/status must not overstate completion. | Spec ledger and archival workflow. | Ledger says 0005 landed with complete Phase 3A substrate; exact code does not support that. | **Documentation/status defect**. |

## 7. Research-informed audit lens

External research was used as pressure, not as feature expansion.

- STRIPS/GOAP-style planning is about operators/actions with preconditions and effects that can be sequenced to transform a state toward a goal. A file named `planner.rs` is not enough; the planner must reason through executable actions and validated preconditions. Source: Fikes & Nilsson, “STRIPS: A New Approach to the Application of Theorem Proving to Problem Solving,” `https://ai.stanford.edu/users/nilsson/OnlinePubs-Nils/PublishedPapers/strips.pdf`.
- HTN planning decomposes compound tasks into primitive/executable tasks using methods whose applicability is state-conditioned. String prefixes that auto-pass are not robust method applicability. Source: Bercher et al., “A Survey on Hierarchical Task Network Planning,” `https://www.ijcai.org/proceedings/2019/0632.pdf`.
- GOAP in game AI relies on actions whose preconditions must be met and whose effects can chain into multi-action plans. Source: Jeff Orkin, “Three States and a Plan: The A.I. of F.E.A.R.,” `https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf`.
- BDI systems give intentions commitment and lifecycle semantics; recreating or ignoring intentions per tick defeats that purpose. Source: Rao & Georgeff, “BDI Agents: From Theory to Practice,” `https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf`.
- Event sourcing’s core value is complete rebuild/replay from events. Phase 3A actions emitting events is insufficient if live state and replay state are not governed by one authoritative event-application contract. Source: Martin Fowler, “Event Sourcing,” `https://martinfowler.com/eaaDev/EventSourcing.html`.
- Needs-based simulation agents require current needs to drive action choice, and ordinary objects/actions must satisfy needs at costs. Wait-only no-human simulation does not pass this pressure test. Source: Robert Zubek, “Needs-Based AI,” `https://robert.zubek.net/publications/Needs-based-AI-draft.pdf`.

## 8. Findings table

| ID | Classification | Severity | Affected files | Finding | Destination |
|---|---:|---:|---|---|---|
| F-01 | Foundational violation; implementation defect | Blocker | `scheduler.rs`, core tests, Phase 3A fixtures | No-human day is a wait-only runner, not ordinary autonomous life. | Hardening spec |
| F-02 | Foundational violation; architectural debt; implementation defect | Blocker | `actions/pipeline.rs`, `events/apply.rs`, `state.rs`, TUI app, replay | Live action pipeline does not apply `AgentState`; agent events are replay-side/projection-side. | Hardening spec |
| F-03 | Implementation defect; foundational violation; research-informed design risk | High | `agent/htn.rs`, `agent/methods.rs` | HTN applicability auto-passes string prefixes/substrings. | Hardening spec |
| F-04 | Foundational violation; research-informed design risk | High | `planner.rs`, `generation.rs`, `scheduler.rs`, tests | Hidden-truth planning is not impossible by construction; it is mostly input-hygiene and self-attestation. | Hardening spec |
| F-05 | Implementation defect; architectural debt | High | `scheduler.rs`, `state.rs`, `intention.rs`, `routine.rs`, content schema | Intention/routine models exist but the integrated decision loop ignores them. | Hardening spec |
| F-06 | Implementation defect; architectural debt | High | `continue_routine.rs`, `methods.rs`, `routine.rs` | `continue_routine` records a continuation marker but does not actually execute the next ordinary action. | Hardening spec |
| F-07 | Architectural debt; likely correct but under-tested | High | `trace.rs`, `state.rs`, `debug_reports.rs`, TUI debug panels | Decision traces/stuck diagnostics exist but are stored/rendered as canonical strings and mostly seeded, not generated by real no-human choices. | Hardening spec |
| F-08 | TUI/debug gate failure | High | TUI app/input/run/render/tests | TUI can render panels but cannot run a no-human day; live Phase 3A state can go stale after actions. | Hardening spec |
| F-09 | Fixture/test defect | High | core tests, content fixtures, TUI tests | Tests assert smoke/synthetic/rendered behavior rather than adversarial integrated ordinary-life invariants. | Hardening spec |
| F-10 | Documentation/status defect | High | `docs/4-specs/SPEC_LEDGER.md`, `docs/4-specs/README.md`, archived 0005 | Status documents overstate Phase 3A completion. | Status errata |
| F-11 | Source-scope limitation | Low | `crates/tracewake-tui/src/launch.rs` | Manifest path exact URL returned 404. | Verification note |

## 9. Detailed findings

### F-01 — No-human day is a wait-only scheduler scaffold

Classification: **Foundational violation / Implementation defect**  
Severity: **Blocker**

Affected files:

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-content/src/fixtures/no_human_day_001.rs`
- `crates/tracewake-content/src/fixtures/ordinary_workday_001.rs`
- `crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs`
- `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs`
- `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs`

Evidence:

- `run_no_human_day` iterates day windows and actors, then calls `build_agent_wait_proposal` for each actor/window.
- `build_agent_wait_proposal` creates hard-coded `actor_known_inputs` such as `reason_available`, `reevaluation_scheduled`, and `day_window:*`.
- It passes `needs: Vec::new()`, `active_intention: None`, and `routine_window_goal: None` into candidate generation/method selection.
- It calls the local planner with `PlannerGoal::WaitWithReason`, empty known route edges, empty known containers, empty known food sources, and budget `1`.
- The core no-human acceptance test only requires ordinary pipeline events and at least one `ActorWaited`.
- The `no_human_day_001` fixture contract expects `FoodConsumed`/`EatFailed`, `SleepCompleted`, `WorkBlockCompleted`/`WorkBlockFailed`, movement before workplace access, missing-food failure, closed-workplace access failure, and no teleportation. The runner cannot produce that causal chain.

Violated obligation:

- Foundation: ordinary life before adventure; world ignores human; actions must be replayable causes.
- Foundation agent doc: routines are not teleports; needs and intentions must drive behavior.
- Architecture scheduler/action docs: no schedule rails or teleportation; all actors use shared proposal/action pipeline.
- Execution Phase 3 gate: full no-human day with wake/eat/work/rest/sleep/move/failure/replan and stuck diagnostics.
- Archived Spec 0005: no-human day capstone.

Why it matters:

This is the central Phase 3A proof. A wait-only no-human day means later Phase 4 institutions would be built atop actors who do not actually live ordinary lives. Wrong suspicion, reports, work absence, fatigue, hunger, theft opportunity, and routine interruption would all be occurring over a false substrate.

Recommended corrective action:

Replace `run_no_human_day` with an actual actor decision loop that reads live `AgentState`, actor-known planning state, routine assignments/windows, and current physical state through the same projection/action boundaries used by embodied play. Each actor/window/tick must select from candidate goals, resolve typed HTN methods, produce ordinary proposals, run the shared pipeline, apply world/agent/epistemic effects, and record decision/stuck traces. `wait` may remain, but only as a justified fallback with a typed reason and bounded retry policy.

Destination: **Hardening spec**.

### F-02 — Shared action pipeline does not apply live `AgentState`

Classification: **Foundational violation / Architectural debt / Implementation defect**  
Severity: **Blocker**

Affected files:

- `crates/tracewake-core/src/actions/pipeline.rs`
- `crates/tracewake-core/src/events/apply.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/actions/defs/eat.rs`
- `crates/tracewake-core/src/actions/defs/sleep.rs`
- `crates/tracewake-core/src/actions/defs/work.rs`
- `crates/tracewake-core/src/actions/defs/wait.rs`
- `crates/tracewake-core/src/replay/rebuild.rs`
- `crates/tracewake-tui/src/app.rs`

Evidence:

- `PipelineContext` carries `&mut PhysicalState`, event log, registry, optional controller bindings, optional epistemic projection, content manifest, and ordering key. It does not carry `&mut AgentState`.
- `run_pipeline` appends events and calls `apply_event(context.state, &event)`.
- `apply_event` mutates only `PhysicalState` for `EventStream::World`; non-world streams are no-ops there.
- `apply_agent_event` exists separately and can apply `NeedDeltaApplied`, intention events, routine events, decision traces, and stuck diagnostics to `AgentState`.
- Eat/sleep/work/wait emit `NeedDeltaApplied` and related events, but the normal live pipeline does not apply those events to live `AgentState`.
- The TUI app owns an `agent_state` seeded from the fixture, but action submission runs the pipeline against physical state and does not update `agent_state` by applying emitted agent events.

Violated obligation:

- Foundation event/replay contract: mutable simulation state not derived from event causes is a bug; replay and live state must preserve causal story.
- Foundation needs/planning doc: needs persist and change through causal time/action effects.
- Architecture event log/projections: projections are derived from authoritative events and must match live behavior.
- Execution Phase 3 gate: replay needs, scheduled actions, routines, traces.

Why it matters:

A hunger delta event emitted by `eat` or `wait` is not enough if the live AI loop never sees the changed hunger. Phase 3A needs/intention/routine state becomes a replay/debug artifact rather than authoritative live cognition. This also means TUI debug panels can show stale fixture-seeded state after Phase 3A actions.

Recommended corrective action:

Introduce a unified simulation application context or state bundle so the shared pipeline applies every accepted event to the appropriate live projections: physical, epistemic, and agent. Replay must use the same semantics and fail loudly on unsupported agent events. Tests must assert live-vs-replay equivalence for world + agent + epistemic state after actual Phase 3A action sequences.

Destination: **Hardening spec**.

### F-03 — HTN method applicability auto-passes string prefixes

Classification: **Implementation defect / Foundational violation / Research-informed design risk**  
Severity: **High**

Affected files:

- `crates/tracewake-core/src/agent/htn.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- content routine templates and routine validation tests

Evidence:

`template_applicable` accepts a condition if:

- the condition string starts with `actor_has_`;
- the condition is `active_intention`;
- the condition is `reason_available`;
- or any supplied actor-known input string contains the condition as a substring.

The method library uses string conditions such as `actor_knows_home`, `route_planner_available`, `actor_has_food_search_knowledge`, `reason_available`, and `reevaluation_scheduled`.

Violated obligation:

- Foundation: agents act from beliefs, not ground truth or convenient strings.
- Architecture agent cognition: HTN method validity must be explicit; methods are not plot scripts.
- Archived 0005: planner should reject methods with inspectable precondition reasons.
- Research pressure: HTN method applicability is state-conditioned; GOAP/STRIPS preconditions must be meaningful action/state constraints.

Why it matters:

This makes hidden-truth and fake-knowledge regressions easy. A content author or scaffold can type `actor_has_food_search_knowledge` and method applicability becomes true even if no actor belief, memory, observation, routine assignment, role, or accessible record supports it. That is not a safe substrate for later routines, institutions, or theft opportunity.

Recommended corrective action:

Replace string preconditions with typed method conditions resolved by a dedicated actor-known condition resolver. Each successful condition must carry proof source: current actor state, actor-known belief, observation/memory/record, routine assignment, current intention, physical locality visible to the actor, or explicit debug/test-only fixture seed. Prefix and substring auto-pass must be forbidden. Add tests that spoof `actor_has_*`/`reason_available` strings and require rejection.

Destination: **Hardening spec**.

### F-04 — Hidden-truth planning is not impossible by construction

Classification: **Foundational violation / Research-informed design risk / Likely correct but under-tested**  
Severity: **High**

Affected files:

- `crates/tracewake-core/src/agent/planner.rs`
- `crates/tracewake-core/src/agent/generation.rs`
- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs`
- tests around hidden truth planning

Evidence:

The local planner type accepts `ActorKnownPlanningState`, and its direct known-food planning path requires the food source to appear in `known_food_sources`. That is good. But the proof boundary is weak:

- integrated code must still construct `ActorKnownPlanningState` from safe actor-known projections;
- no-human runner builds planner input directly in scheduler code, using physical actor location and hard-coded strings;
- tests mostly hand-construct actor-known input and then assert hidden truth is absent from that input;
- decision traces set `hidden_truth_audit_result.actor_known_only = true` with notes asserting the boundary was used, rather than proving construction from beliefs by type.

Violated obligation:

- Foundation: psychics are forbidden; belief before truth; no hidden truth in planning.
- Architecture epistemics: ground truth must not cross into actor cognition except through modeled channels.
- Execution Phase 2/3 gates: actor-known view/planner checks and no hidden truth planning fixture.

Why it matters:

A planner that is safe only when callers are honest is not “impossible by construction.” Phase 4 institutions and suspicion scoring will need stronger guarantees than self-attested hidden-truth audit strings.

Recommended corrective action:

Create one canonical builder from `KnowledgeContext`/actor-known projection plus live `AgentState` into `ActorKnownPlanningState`. The planner should not accept raw global `PhysicalState` except through a small physical-locality interface already filtered by what the actor can observe or legitimately know. Tests must force hidden food/workplace/routes to exist physically while absent from beliefs and assert no method/action can select them.

Destination: **Hardening spec**.

### F-05 — Intention and routine state exists but is ignored by the integrated decision loop

Classification: **Implementation defect / Architectural debt**  
Severity: **High**

Affected files:

- `crates/tracewake-core/src/scheduler.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/agent/intention.rs`
- `crates/tracewake-core/src/agent/routine.rs`
- `crates/tracewake-content/src/schema.rs`

Evidence:

`AgentState` can store needs, intentions, active intentions, routine executions, decision traces, and stuck diagnostics. `Intention` has a lifecycle. `RoutineExecution` has step, status, ancestry, fallback attempts, and trace ID fields. But the no-human day passes `active_intention: None`, `routine_window_goal: None`, and empty needs into generation. Content schema can author routine templates/assignments, but the inspected `to_agent_state` path seeds initial needs and does not prove routine assignments become live active routine executions.

Violated obligation:

- Foundation agent doc: intentions persist, routines are defeasible, abandonment/adoption must be eventful.
- Architecture agent cognition: BDI → HTN → bounded planning → shared pipeline.
- Execution Phase 3: routines as defeasible intentions with durations, failures, interruptions, planner debug.

Why it matters:

Later work needs workers to remain workers, sleepers to stay asleep unless interrupted, hungry actors to abandon work for food, and possessed actors to resume plans after unpossession. The types alone do not establish that behavior.

Recommended corrective action:

Instantiate routine assignments into durable routine/intentional state at fixture load or first schedule window through events. The actor decision loop must read and update active intention/routine execution state. Possession attach/detach must not reset it. Tests must prove continuation, interruption, abandonment, and reactivation through actual events.

Destination: **Hardening spec**.

### F-06 — `continue_routine` is a marker, not actual routine continuation

Classification: **Implementation defect / Architectural debt**  
Severity: **High**

Affected files:

- `crates/tracewake-core/src/actions/defs/continue_routine.rs`
- `crates/tracewake-core/src/agent/methods.rs`
- `crates/tracewake-core/src/agent/routine.rs`

Evidence:

The `continue_routine` action validates proposal parameters such as current intention and next action ID, then emits a `ContinueRoutineProposed` event with payload fields like `routes_through_shared_pipeline=true`, `intention_mutated=false`, and `next_action_id`. It does not itself resolve and execute the next ordinary action through the pipeline, nor does it transition routine step state live.

Violated obligation:

- Archived 0005: continue-routine must be a shared-pipeline handoff for ordinary actions and must not become magic.
- Foundation: routines are embodied action chains, not bypasses.

Why it matters:

A `ContinueRoutineProposed` event can make debug output look alive while the actor has not actually moved, eaten, worked, slept, or failed. It is less dangerous than teleportation, but it is still not routine continuation.

Recommended corrective action:

Either make `continue_routine` an internal planning handoff that immediately proposes the next ordinary action through the same pipeline, or narrow it to a diagnostic/trace event that cannot satisfy behavioral acceptance. Completion tests must assert subsequent ordinary action ancestry, not just the continuation marker.

Destination: **Hardening spec**.

### F-07 — Debug traces and stuck diagnostics are shaped but not integrated enough

Classification: **Architectural debt / Likely correct but under-tested**  
Severity: **High**

Affected files:

- `crates/tracewake-core/src/agent/trace.rs`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/debug_reports.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-core/src/scheduler.rs`

Evidence:

`DecisionTrace` and `StuckDiagnostic` are richly typed. But `AgentState` stores `decision_traces` and `stuck_diagnostics` as `BTreeMap<..., String>`, and the debug reports mostly render canonical rows. TUI tests seed fixtures and assert rendered panels contain words like `candidate_goals`, `selected_method`, `rejected_reasons`, `blocked_preconditions`, and `hidden_truth_audit`; they do not prove that real no-human decisions generated those rows. The no-human runner records stuck diagnostics only if wait itself fails to append events, not when planning cannot produce eat/work/sleep/move.

Violated obligation:

- Foundation and architecture debug gates: failures must be loud, inspectable, replayable, and explain why choices were made/rejected.
- Archived 0005: debug reports explain rejected goals, rejected methods, failed preconditions, fallback choices, and action rejection.

Why it matters:

Later coding agents will need failure-loud traces to diagnose why a villager starved, missed work, failed to report, or accused the wrong person. String snapshots are fragile; a panel containing the right nouns is not proof of causal explanation.

Recommended corrective action:

Persist structured trace/diagnostic data or structured canonical fields sufficient to rebuild them. Every autonomous decision should emit trace events covering candidate generation, condition resolution, chosen method, action proposal, validation result, fallback/replan/abandonment, and hidden-truth audit proof. Debug panels should render these real events/projections.

Destination: **Hardening spec**.

### F-08 — TUI panels exist, but Phase 3A behavior is not TUI-reachable enough

Classification: **TUI/debug gate failure / Implementation defect**  
Severity: **High**

Affected files:

- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/src/render.rs`
- `crates/tracewake-tui/tests/tui_acceptance.rs`
- `crates/tracewake-tui/tests/transcript_snapshot.rs`

Evidence:

The TUI can bind an actor, render embodied Phase 3A status, submit semantic actions, and render debug panels for needs/routines/planner/stuck/no-human-day/actor. However:

- the command parser exposes `debug no-human-day`, not a command to run or advance a no-human day;
- the no-human-day panel summarizes current log metrics, not an executed ordinary-life day from the TUI;
- TUI action submission does not update `agent_state` by applying agent events emitted by actions;
- TUI tests render Phase 3A debug panels and transcripts, but do not exercise a full no-human ordinary-life run through TUI or view-model harness.

Violated obligation:

- Foundation TUI doc: every runnable phase has TUI/view-model gate; unreached mechanics are incomplete.
- Architecture validation doc: every feature needs no-human, TUI, replay, and debug gates.

Why it matters:

A substrate only visible in fixture snapshots is not a playable foundation. The user must be able to inspect why actors chose actions, why they failed, and how no-human runs advance without breaking actor-filtered boundaries.

Recommended corrective action:

Add a TUI or stable view-model harness path to run/advance the no-human day and inspect resulting actor-filtered and debug projections. After each submitted action/no-human step, the TUI must update live `AgentState` from events. Tests must assert post-run needs, intentions, routine steps, stuck diagnostics, no-hidden-truth boundaries, and replay/debug metrics.

Destination: **Hardening spec**.

### F-09 — Tests and fixtures assert contracts, not enough behavior

Classification: **Fixture/test defect**  
Severity: **High**

Affected files:

- `crates/tracewake-core/tests/acceptance_gates.rs`
- `crates/tracewake-core/tests/golden_scenarios.rs`
- `crates/tracewake-content/src/fixtures/*.rs`
- `crates/tracewake-content/tests/*.rs`
- `crates/tracewake-tui/tests/*.rs`

Evidence:

- Core no-human smoke test only asserts no controller, marker events, ordinary pipeline events, and `ActorWaited`.
- A no-human metrics golden test manually constructs a log with events such as routine step started, food consumed, need threshold, work failed, stuck diagnostic, and no-human completed; it does not run the no-human scheduler to produce those events.
- Replay tests manually append agent events rather than proving they came from a live ordinary day.
- TUI tests render initial fixture/debug state, deterministic panels, and command-loop smoke behavior.
- Content fixtures have excellent contracts, but fixture contracts are not enforced against actual integrated simulation runs.

Violated obligation:

- Execution testing/debug docs: tests must prove no-human day, replay, TUI/view, causality, failure cases, anti-shortcut fixtures.
- Archived 0005: explicit hard gates for food unavailable replan, routine blocked diagnostic, no teleport, no hidden truth, possession parity, replay equivalence.

Why it matters:

The current test suite can pass while the no-human day remains wait-only and while live `AgentState` is stale. That is exactly the failure the Phase 3A audit is meant to catch.

Recommended corrective action:

Add integrated adversarial tests that run actual code paths rather than synthetic logs:

1. full no-human day with event ancestry;
2. ordinary workday with movement before work;
3. food unavailable replan/failure with no hidden hunger fix;
4. routine blocked diagnostic;
5. routine no-teleport proof;
6. no hidden truth planning from physical-only food/workplace facts;
7. possession does not reset intention/needs/routine progress;
8. replay equivalence for actual no-human day;
9. planner trace explains candidate rejection/method rejection/action rejection/fallback;
10. TUI or view-model path can inspect Phase 3A state after actual run.

Destination: **Hardening spec**.

### F-10 — Status documents overclaim completion

Classification: **Documentation/status defect**  
Severity: **High**

Affected files:

- `docs/4-specs/SPEC_LEDGER.md`
- `docs/4-specs/README.md`
- `archive/specs/0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`

Evidence:

`SPEC_LEDGER.md` says Spec 0005 Phase 3A “landed” and describes the result as bounded needs, durable intentions, defeasible routines, sleep/eat/work/continue/wait through shared pipeline, no-human day runner/metrics, fixtures, replay/debug, embodied TUI panels, and acceptance evidence. That is materially stronger than what the implementation proves. `docs/4-specs/README.md` is also stale/incomplete relative to archived 0005 and needed corrective tracking.

Violated obligation:

- Reference review checklist: exact-codebase validation and no demo success in place of proof.
- Execution authority: when a test rewards a shortcut or implementation is convenient, the implementation/test/status is wrong.

Why it matters:

A coding agent reading the ledger could reasonably proceed to Phase 3B/4 on the false premise that ordinary no-human life is proven. That would compound the substrate defect.

Recommended corrective action:

Do not rewrite archived 0005 as if it never happened. Add a status errata and update ledger language to mark 0005 as historically archived but acceptance not earned at this commit. Record that a Phase 3A hardening spec blocks Phase 3B/4.

Destination: **Status errata**.

### F-11 — Manifest path `crates/tracewake-tui/src/launch.rs` could not be verified

Classification: **Source-scope limitation**  
Severity: **Low**

Affected file:

- `crates/tracewake-tui/src/launch.rs`

Evidence:

The manifest listed the file, but both exact raw and exact blob URLs for the target commit returned 404. No claim in this audit relies on that file.

Recommended corrective action:

Next repository-side verification should confirm whether the manifest was generated from the exact target tree or whether the path was stale/removed. This is not a Phase 3A behavioral blocker by itself.

Destination: **Verification note**.

## 10. Test and fixture coverage assessment

The fixtures are stronger than the implementation. They name the right adversarial scenarios: `food_unavailable_replan_001`, `no_hidden_truth_planning_001`, `no_human_day_001`, `ordinary_workday_001`, `planner_trace_001`, `possession_does_not_reset_intention_001`, `routine_blocked_diagnostic_001`, `routine_no_teleport_001`, and `sleep_eat_work_001`. Their contracts describe the right behavior.

The tests do not enforce those contracts deeply enough. Current coverage is concentrated around:

- unit-level type/serialization/determinism checks;
- action smoke tests;
- synthetic replay logs;
- fixture load/render behavior;
- debug panel snapshots;
- TUI command-loop smoke tests.

Missing or insufficient:

- actual full no-human day with sleep/eat/work/move/fail/replan ancestry;
- actual no-hidden-truth proof from the integrated planner boundary;
- actual food unavailable replan/failure through no-human decision loop;
- actual routine blocked diagnostic with failed action precondition and stuck trace;
- actual no-teleport proof around remote workplace access;
- live agent-state update and replay equivalence after ordinary actions;
- possession attach/detach/resume over live intentions;
- TUI run/advance no-human path.

## 11. No-human day behavioral assessment

Phase 3A’s no-human day is the failing capstone. The runner’s structure looks legitimate: it starts a no-human day event, iterates day windows and actors in deterministic order, runs proposals through the shared action pipeline, and emits a completion marker/metrics. But the proposal builder is hard-coded to wait.

A robust Phase 3A no-human day should show something boring but real:

```text
wake -> hunger/fatigue checked -> move/find food -> eat or fail to eat -> move to workplace -> work or fail -> return/rest/wait -> sleep -> replay/debug report explains all decisions
```

Current code instead does roughly:

```text
for each actor/window:
    create actor-known strings reason_available/reevaluation_scheduled/day_window
    ignore needs
    ignore active intention
    ignore routine window goal
    ask planner for WaitWithReason
    run wait through pipeline
complete day
```

That is a useful Phase 1-style scheduler proof. It is not Phase 3A.

## 12. Planner / HTN / GOAP-like assessment

There are three layers:

1. Candidate generation and decision selection.
2. HTN method selection.
3. Local planner action proposal generation.

All three are promising scaffolds, but not sufficiently robust.

Candidate generation can generate goals from needs and active intentions when supplied those inputs. The no-human runner supplies none. HTN method selection emits traces but accepts crucial conditions through string-prefix rules. The local planner has a safer shape because it receives `ActorKnownPlanningState`, but it is caller-trust-dependent and narrow. It is not yet a robust local action-sequence planner through real preconditions/effects; it can plan direct known food, route next step, known container check, and wait-like fallbacks.

Corrective direction: keep the symbolic substrate, but harden the boundaries. Remove auto-true string conditions. Tie methods to typed condition proof. Build actor-known planner input from epistemic projections by construction. Require selected planner actions to pass the shared pipeline, and record mismatch as a failure-loud diagnostic, not a silent wait.

## 13. Needs / routines / intentions assessment

Needs:

- Positive: persistent `NeedState`, bands, deltas, threshold-crossing events, wait/eat/sleep/work need events.
- Defect: live `AgentState` is not updated by the shared pipeline. Needs can exist and replay can rebuild them, but ordinary live AI is not reliably driven by changed needs.

Intentions:

- Positive: durable `Intention` struct with statuses and lifecycle operations.
- Defect: no-human day passes `None`; possession parity around intention is largely fixture/debug/snapshot-level, not integrated autonomy proof.

Routines:

- Positive: rich routine templates/steps/executions, content assignments, intended failure modes.
- Defect: routine assignments are not proven to become active routine execution; no-human ignores them; `continue_routine` is a marker; anti-teleport test is mostly structural/naming.

## 14. Replay / debug / TUI assessment

Replay:

- Positive: replay rebuild can apply physical, epistemic, and agent streams into projections.
- Defect: actual live no-human day does not generate the required Phase 3A causal story; replay tests use synthetic agent events.

Debug:

- Positive: debug report surfaces exist and are deterministic/read-only.
- Defect: meaningful trace/stuck information is not generated by real no-human choices; string storage weakens machine-checkable diagnostics.

TUI:

- Positive: Phase 3A status and debug panels are reachable as panels; action submission works for ordinary actions.
- Defect: no command/path runs the Phase 3A no-human day; TUI-owned agent state can be stale after actions; tests render panels rather than proving full behavior.

## 15. Status / ledger assessment

`docs/4-specs/SPEC_LEDGER.md` overstates completion. The least destructive correction is an errata document plus ledger wording changes. Archived 0005 should remain archived as historical evidence, but the project should record that exact-commit audit found acceptance not earned and that a corrective Phase 3A hardening spec blocks progression.

`docs/4-specs/README.md` is stale/incomplete relative to the archived spec history and the proposed corrective spec. It should either list archived historical specs accurately or point to a ledger/status file that does.

## 16. Risk assessment

Proceeding to Phase 3B or Phase 4 now would create a false tower:

- institutions would react to actors who do not actually live ordinary days;
- reports/wrong suspicion would depend on hidden or scripted triggers;
- no-human proof would become a demo transcript rather than a causal substrate;
- planner/HTN terminology would harden around unsafe string conditions;
- TUI/debug panels would incentivize snapshot assertions rather than real diagnosis;
- replay would prove synthetic logs, not actual simulation.

The highest-risk architectural drift is the separation between live physical mutation and replay-side agent-state projection. Fixing that now is cheaper than building institutions over it.

## 17. Recommendation

Do **not** proceed to Phase 3B or Phase 4. Implement a bounded Phase 3A hardening spec first.

The corrective work should not expand the product. It should make the first ordinary no-human day genuinely boring, replayable, inspectable, actor-known, and failure-loud:

- live needs/intention/routine state updated through events;
- actor decision loop uses same scheduler/action/proposal/event pipeline as possessed play;
- typed HTN/planner condition proof from actor-known state;
- no-hidden-truth planning impossible by construction;
- no-human day produces wake/eat/move/work/rest/sleep/fail/replan ancestry;
- debug/TUI can inspect real generated decisions and failures;
- replay proves equivalence for actual logs;
- status docs stop claiming Phase 3A is safely complete.

