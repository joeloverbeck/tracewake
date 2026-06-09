# 0014 Phase 3A Ordinary-Life Needs / Routines Alignment and Anti-Contamination Hardening Spec

**Staging path:** `specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `01e860b3f26fb087d0fa7437146dd9901e1a4019`  
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, latest `main`, full project, Phase 4, or later-proof readiness.  
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

## 0. Evidence ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 01e860b3f26fb087d0fa7437146dd9901e1a4019
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.run open/find using exact raw.githubusercontent.com and github.com/blob URLs
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/archive/specs/0013_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/need.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/methods.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/htn.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/candidate.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/generation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/trace.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/sleep.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/no_human_capstone.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/hidden_truth_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/acceptance_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/golden_scenarios.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/negative_fixture_runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/doc_invariant_references.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/event_schema_replay_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/no_human_day_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/planner_trace_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/ordinary_workday_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/no_human_advance_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-tui/src/debug_panels.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

### Commit-target resolution

The uploaded research brief names commit `461308af95940d59c2d56d32ffead35631c9db72`, but the explicit Exact-Commit Git Discipline Guardrail in the user message names `01e860b3f26fb087d0fa7437146dd9901e1a4019` as the target of record. This spec follows the guardrail. The uploaded manifest is used only as path inventory; no repository metadata, branch lookup, code search, clone, snippet, or connector namespace label is used as evidence.

## 1. Scope

### In scope

- Phase 3A needs, intentions, routines, HTN-like methods, bounded local planning, decision traces, and actor-known transaction usage.
- Phase 3A ordinary actions: `sleep`, `eat`, `work_block`, `continue_routine`, and extended `wait`, including their registration and use through the shared pipeline.
- Phase 3A surface inside shared files: no-human scheduler driver/context/proposal path, duration-completion events, passive need deltas, no-human metrics, and embodied/debug view surface insofar as it exposes Phase 3A ordinary-life state.
- Phase 3A fixtures and tests listed in the uploaded brief and manifest.

### Out of scope

- Re-auditing Phase 1 / Phase 1A spine internals already covered by the post-overhaul spine hardening series.
- Re-auditing Phase 2A epistemic internals already covered by `0013`.
- Designing Phase 4 institutions or later proof features.

Where this spec references a Phase 1 or Phase 2A seam, it does so only to constrain Phase 3A usage of that seam.


## Doctrine anchors used for the verdict

- **Authority order.** The docs map states that foundation outranks architecture, architecture outranks execution, execution outranks specs, and implementation convenience loses to accepted gates. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/README.md`.
- **Truth firewall and cognition authority.** Re-derived invariants: INV-099 truth may validate but not plan; INV-100 hidden-truth cognition is forbidden; INV-101 actor-known context must be sealed; INV-102 cognition inputs require provenance; INV-103 scheduler is not a cognition authority; INV-104 needs/routines do not dispatch primitives; INV-105 decision traces are authoritative typed structures; INV-106 validation failure must not leak hidden truth; INV-107 debug omniscience is quarantined; INV-108 possession is cognition-neutral. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`.
- **Phase-3A agent doctrine.** Needs are pressures rather than puppet strings, intentions are durable commitments, HTN methods are defeasible procedures rather than scripts, and bounded planning must operate from actor-known facts and modeled affordances. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`.
- **Ordinary action doctrine.** NPC and human actions share proposal/validation, affordances are conditional, sleep/food/work are causal, and hunger/fatigue pressure must not become direct action dispatch. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`.
- **Holder-known context architecture.** Every decision-facing context must be sealed with provenance; forbidden sources include physical truth, fixture truth, debug omniscience, validator internals, and raw route/workplace/food/sleep tables. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`.
- **Scheduler and validation limits.** The scheduler may order ticks, choose eligible actors, call transactions, enqueue proposals, and record metrics, but it may not emit primitives from raw state/need thresholds/routine labels, read true targets, or count markers as progress. Validation may accept/reject/resolve only and may not choose the next goal or reveal hidden targets. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`.
- **ORD-LIFE-CERT gate.** The gate requires event-sourced/replay-reconstructable needs/intentions/routines/stuck diagnostics, actor-known candidate generation, method/planning provenance, no scheduler primitive dispatch, stale/forged parameter rejection, real no-human metrics, debug quarantine, replay-rebuilt metrics/diagnostics, and responsible-layer fixture failures. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.
- **TFW gate.** Per-spec proof must name the holders doing cognition, the sealed-context builder, excluded truth sources, context-provenance citations for selected goals/methods, validator non-suggestion of hidden targets, debug separation, and replay rebuild; forbidden patterns include raw true food/workplace/sleep/route selection and scheduler primitive emission. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`.
- **Proof surface.** Acceptance may not rely on display strings or “looks right”; it must carry typed event, replay, holder-known, decision trace, stuck, validation, metric, TUI, debug, content-validation, and anti-regression artifacts. Evidence: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A ordinary-life substrate is not perfectly aligned and not maximally locked. This spec is therefore authored under the produce-only-if-positive rule.

This positive verdict rests on seven findings. The first five are direct alignment violations or high-severity proof-surface breaks. The last two are contamination/proof-surface gaps that can allow future regressions despite plausible current behavior. The presence of existing adversarial tests is valuable but insufficient: several current tests prove the transaction or planner in isolation while missing the no-human scheduler and embodied projection paths that manufacture or display actor-known facts from raw truth.


## 4. Findings and remediation requirements

### ORD-HARD-001 — No-human context construction leaks raw workplace truth and current-place sleep truth

**Severity:** blocker for scoped `ORD-LIFE-CERT` evidence.

**Responsible layers:** `scheduler`, `holder_known_context`, `candidate_generation`, `local_planning`.

**Doctrine breached:** INV-099, INV-100, INV-101, INV-102, INV-103, INV-105; `ORD-LIFE-CERT` actor-known candidate generation, method/planning provenance, scheduler-boundary, replay/diagnostic clauses; `TFW` context-exclusion and selected-goal/method provenance clauses.

**Evidence:** In the no-human path, `visible_local_planning_state` is built from authoritative `PhysicalState`. It reads `state.places`, `state.food_supplies`, and `state.workplaces`, filters workplaces by `assigned_actor_ids.contains(actor_id)`, and inserts the actor's current place as a known sleep place. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/scheduler.rs`. The scheduler then turns that surface into actor-known facts including `sleep_place_believed_accessible` and current-place workplace facts. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/scheduler.rs`.

**Why this violates doctrine:** The holder-known architecture explicitly forbids raw route/workplace/food/sleep tables as cognition sources and states that acceptance fails if an actor reaches true food, workplace, bed, route, clue, or other target without it existing in holder-known context with provenance. The no-human scheduler is allowed to call the actor decision transaction; it is not allowed to create the actor's workplace/sleep knowledge from raw assignment or location truth.

**Required correction:** Replace `visible_local_planning_state` with a provenance-bearing builder, tentatively named `NoHumanActorKnownSurfaceBuilder`, that consumes only modeled observations, memories, records, role-assignment notices, home/sleep-place knowledge, visible local affordances, and explicit unknowns. It may read authoritative truth only to validate visibility and generate modeled observations, not to generate cognitive facts directly. Remove the automatic `known_sleep_places = current_place` default. Sleep places must come from an explicit, provenance-carrying sleep/rest affordance fact.

**Structural lock:** Add a source guard in `anti_regression_guards.rs` that fails on `state.workplaces`, `state.sleep_places`, raw `state.food_supplies`, `state.places` adjacency traversal, or `BTreeSet::from([current_place_id.clone()])` inside the no-human context builder except through an allowlisted perception/observation function. Add negative fixtures:

- `no_human_unseen_workplace_assignment_does_not_plan_work_001`: raw assignment exists but actor lacks assignment notice/observation; no work candidate/proposal may be selected.
- `no_human_current_place_without_sleep_affordance_does_not_sleep_001`: fatigue is high and actor is at an arbitrary place; no sleep proposal may be selected without actor-known sleep surface.
- `no_human_known_workplace_requires_provenance_001`: a workplace is usable only after a modeled assignment notice/memory/observation is present in the actor-known packet.

### ORD-HARD-002 — Scheduler mutates actor-transaction proposals after cognition

**Severity:** high.

**Responsible layers:** `scheduler`, `proposal_construction`.

**Doctrine breached:** INV-103, INV-104, INV-105, INV-099; `ORD-LIFE-CERT` scheduler-boundary, proposal ancestry, and diagnostic clauses; `TFW` scheduler-forbidden-pattern clause.

**Evidence:** After `ActorDecisionTransaction::run`, `scheduler.rs` takes `proposed.proposal`; when the proposal action is `wait`, it inserts `reason = no_human_day:<window_id>` into `proposal.parameters`. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/scheduler.rs`.

**Why this violates doctrine:** The actor decision transaction owns proposal construction and trace ancestry. The scheduler may order/call/enqueue and record metrics; it may not author or mutate actor-visible primitive proposal parameters after the actor's transaction.

**Required correction:** Make transaction output proposals immutable to the scheduler. If no-human window identity is needed, record it as scheduler/debug metadata on the no-human report or ordering key, not as actor-visible proposal parameter. The actor-visible wait reason must come from the transaction/local plan.

**Structural lock:** Introduce `TransactionProposal` or `SealedProposal` with private fields and read-only accessors. Source guard: fail on `proposal.parameters.insert`, `proposal.target_ids.push`, `proposal.action_id =`, or equivalent mutation in `scheduler.rs` after transaction return. Add a regression test that compares the wait proposal reason in the decision trace to the committed wait event and asserts no no-human window id appears in actor-visible proposal parameters.

### ORD-HARD-003 — Silent method fallback can desynchronize trace, lifecycle, and proposal ancestry

**Severity:** high.

**Responsible layers:** `method_selection`, `intention_lifecycle`, `proposal_construction`.

**Doctrine breached:** INV-034, INV-036, INV-041, INV-105; `ORD-LIFE-CERT` event/replay/provenance clauses; `TFW` selected-goal/method provenance clause.

**Evidence:** `transaction.rs` selects a goal/decision trace, but if method selection fails it scans fallback candidates and selects the first method that works without rerunning the decision trace/lifecycle sequence. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/transaction.rs`. Later proposal construction inserts a `decision_trace_id` from the selected trace while inserting candidate/method fields from `method_goal`, which can be the fallback. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/transaction.rs`.

**Why this violates doctrine:** A decision trace is authoritative only if the selected candidate, lifecycle transition, method selection, local plan, and proposal ancestry are one coherent transaction bundle. Silent fallback after trace selection creates a path where behavior is plausible but proof is false.

**Required correction:** Choose one behavior and encode it structurally:

1. **Fail closed:** selected method failure produces a typed stuck diagnostic naming `method_selection`, with no proposal.
2. **Re-run:** fallback requires a new candidate-selection/lifecycle/method/planning transaction that emits a new decision trace for the fallback goal.

Do not keep silent fallback as a local patch.

**Structural lock:** Create a `SelectedGoalBundle` type carrying `candidate_goal_id`, `decision_trace_id`, `intention_transition_id`, `selected_method_id`, `local_plan_id`, and proposal ancestry. Proposal construction must accept only that bundle. Source guard: fail on fallback scans of `candidate_fallbacks.iter().find_map(...select_phase3a_method...)` in `transaction.rs`. Add a unit test where the highest-priority candidate has no method and the lower-priority candidate does; the test must prove either a stuck diagnostic or a new trace, never a mixed trace/proposal.

### ORD-HARD-004 — Hidden-truth audit is string scanning instead of structural provenance

**Severity:** high proof-surface defect.

**Responsible layers:** `candidate_generation`, `decision_trace`, `debug_quarantine`, `test_oracle`.

**Doctrine breached:** INV-102, INV-105, INV-107; `ORD-LIFE-CERT` provenance/diagnostic clauses; `TFW` forbidden-truth-audit clause.

**Evidence:** `DecisionInput.actor_known_inputs` is `Vec<String>`. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/decision.rs`. The hidden-truth audit checks whether those strings contain `unproven`, `debug_omniscience`, or `physical_truth`, then emits a note containing only counts and labels. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/agent/decision.rs`.

**Why this violates doctrine:** INV-105 requires decision traces to be typed structures, not display strings. A display string can omit the banned token while carrying forbidden source semantics, or include the token in a harmless explanation. This is not a firewall.

**Required correction:** Replace `Vec<String>` with a typed `Vec<ActorKnownInputRef>` that carries fact id, proposition id, provenance edge ids, source event ids, source class, confidence/staleness, and explicit-unknown marker. The hidden-truth audit must derive from provenance classes and forbidden-source enums, not text.

**Structural lock:** Source guard: fail on `contains("unproven")`, `contains("debug_omniscience")`, `contains("physical_truth")`, and `Vec<String>` used as authoritative actor-known input in `agent/decision.rs`. Add a negative test that constructs an unproven/debug-origin input whose display text omits banned words; the audit must still fail because provenance class is forbidden.

### ORD-HARD-005 — Sleep validation lacks modeled sleep/rest affordance validation

**Severity:** high.

**Responsible layers:** `action_validation`, `local_planning`, `holder_known_context`.

**Doctrine breached:** INV-043, INV-044, INV-045, INV-099, INV-101, INV-102; `ORD-LIFE-CERT` sleep-place and forged/stale parameter clauses; `TFW` selected-target provenance clause.

**Evidence:** `sleep.rs` validates that the actor exists and that optional `sleep_place_id` equals the actor's current place, then emits `SleepStarted`; it does not prove a modeled bed/rest/sleep affordance exists at that place. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/sleep.rs`. Existing tests cover duration and unreachable-place rejection, not the current-place-without-sleep-affordance case. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/actions/defs/sleep.rs`.

**Why this violates doctrine:** Sleep is an ordinary causal action with conditional affordances. Being physically present somewhere is not the same as the actor knowing and validating a legitimate sleep/rest surface.

**Required correction:** Model sleep/rest affordances explicitly in authoritative state and content schema. The sleep proposal must include a sleep affordance id or rest-surface id with actor-known provenance. Validation must reject absent/closed/forged/stale sleep surface parameters with a typed `NoSleepAffordance` or equivalent reason.

**Structural lock:** Add negative fixture `sleep_rejects_current_place_without_sleep_affordance_001`; add content validation requiring Phase 3A sleep-routine fixtures to provide a sleep surface or explicit no-sleep diagnostic expectation; add a source guard that fails if sleep start validation checks only `current_place_id` and does not reference the sleep-affordance state or typed validator.

### ORD-HARD-006 — Embodied workplace view uses raw workplace table

**Severity:** medium-high, because it can contaminate TUI affordances and human proposals.

**Responsible layers:** `view_model`, `holder_known_context`.

**Doctrine breached:** INV-067, INV-099, INV-100, INV-101, INV-102, INV-103, INV-105; `TFW` debug/embodied separation and context-provenance clauses; architecture TUI affordance formula.

**Evidence:** `EmbodiedProjectionSource::from_sealed_context` calls `actor_known_workplaces_for_context(state, viewer_actor_id)`. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/projections.rs`. That helper reads `state.workplaces` and returns every workplace whose `assigned_actor_ids` is empty or contains the viewer — exposing all unassigned workplaces to any viewer plus the viewer's own assignments, with no open-state filter and no actor-known-provenance check, as actor-known workplaces. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/projections.rs`.

**Why this violates doctrine:** Embodied views are holder-known surfaces. They must not infer affordances from raw workplace assignment truth. A human-bound actor should not see or select a work affordance solely because the fixture/state table assigns it.

**Required correction:** Move embodied workplace affordances into the sealed context packet. Projection may render only context-backed workplace facts and actor-visible action definitions. Debug projection may compare context-vs-truth, but must remain structurally separate and non-diegetic.

**Structural lock:** Source guard: fail on `state.workplaces` inside embodied actor-known projection helpers. Add TUI/view-model adversarial fixture where raw assignment exists but no actor-known workplace fact exists; embodied output must omit the work affordance while debug output may show the discrepancy.

### ORD-HARD-007 — No-human metrics use string scans for planner failure classification

**Severity:** medium proof-surface hardening gap.

**Responsible layers:** `projection`, `test_oracle`, `debug_quarantine`.

**Doctrine breached:** INV-105; `ORD-LIFE-CERT` metrics/replay/diagnostic clauses; architecture invalid-pass rule against display strings.

**Evidence:** `NoHumanDayMetrics` counts many event kinds structurally, but planner-failure classification scans event payload values for text like `planner_budget_exhausted`, or for strings containing `planner` and `failed`. Source: `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/src/projections.rs`.

**Why this violates doctrine:** Metrics and diagnostics are proof artifacts. They must count typed responsible-layer/blocker codes, not English payload fragments.

**Required correction:** Add typed fields on relevant decision/stuck events: `responsible_layer`, `blocker_code`, `input_source`, `actual_source`, and `hidden_truth_referenced`. Metrics must read those fields/enums. Replay must rebuild metrics from the same typed fields.

**Structural lock:** Source guard: fail on `.contains("planner")` / `.contains("failed")` in metrics projections. Schema/replay gate: every decision/stuck diagnostic event used by no-human metrics must carry typed layer and blocker code; replay must byte-match live metrics.

## 5. Anti-contamination lock layer

The implementation work must add a dedicated Phase 3A ordinary-life lock layer. Documentation-only locks are insufficient.

### 5.1 Source guards

Extend `crates/tracewake-core/tests/anti_regression_guards.rs` with named guards that fail the test suite if any of the following patterns appear outside explicit allowlists:

1. **Raw workplace/sleep/food/route reads in no-human context construction:** ban direct `state.workplaces`, raw sleep-place table reads, raw food supply scans, and direct route/adjacency traversal in scheduler cognition-surface builders unless the call goes through a named observation/perception/memory builder.
2. **Scheduler proposal mutation:** ban `proposal.parameters.insert`, `proposal.parameters.remove`, `proposal.target_ids.push`, action-id mutation, or actor-id mutation in `scheduler.rs` after transaction return.
3. **Silent fallback method selection:** ban fallback scans in `transaction.rs` that select a different method/candidate after decision trace selection without a new trace.
4. **String-based hidden-truth audit:** ban substring checks for forbidden provenance labels in authoritative audit paths.
5. **Raw workplace embodied projection:** ban `state.workplaces` in embodied actor-known projection helpers.
6. **Metrics string scans:** ban `.contains("planner")`, `.contains("failed")`, and equivalent text matching for no-human metrics classification.

### 5.2 Sealed types and private fields

Use Rust privacy to make invalid state unrepresentable:

- `SealedActorKnownSurface`: constructed only by provenance-bearing builders.
- `SelectedGoalBundle`: one object carrying selected candidate, decision trace, intention transition, selected method, local plan, and proposal ancestry.
- `SealedProposal` / `TransactionProposal`: transaction output cannot be mutated by scheduler except through explicitly non-cognitive metadata wrappers.
- `ActorKnownInputRef`: typed fact/proposition/provenance reference replacing display-string actor-known inputs.

The Rust Reference confirms that private items are accessible only within their module/submodules and that public structs may have private fields; use that language-level boundary as a compile-enforced lock rather than relying on review convention.

### 5.3 Negative / adversarial fixtures

Add or strengthen fixtures so each regression mode fails through normal `cargo test`:

- `no_human_unseen_workplace_assignment_does_not_plan_work_001`
- `no_human_current_place_without_sleep_affordance_does_not_sleep_001`
- `no_human_known_workplace_requires_provenance_001`
- `scheduler_cannot_rewrite_wait_reason_after_transaction_001`
- `method_fallback_requires_new_trace_or_stuck_001`
- `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001`
- `embodied_view_omits_raw_assignment_without_context_001`
- `no_human_metrics_require_typed_responsible_layer_001`

### 5.4 Replay/schema gates

- Decision/stuck events must carry typed `responsible_layer`, `blocker_code`, `input_source`, `actual_source`, `hidden_truth_referenced`, and `remediation_hint` fields where applicable.
- No-human metrics must rebuild from typed event fields and byte-match live metrics.
- Actor-known context hashes used by decisions must be replay-reconstructable from modeled knowledge, not from raw physical truth.

### 5.5 Conformance index

Add a Phase 3A ordinary-life conformance index mapping each cognitive input class to exactly one source path and provenance class:

| Cognitive input | Allowed source | Forbidden source | Lock |
|---|---|---|---|
| Need pressure | agent state / need delta events | scheduler threshold direct primitive | typed need-pressure candidate generator + guard |
| Workplace knowledge | observation, memory, assignment notice/record | `state.workplaces` raw assignment | context-builder guard + fixture |
| Sleep/rest knowledge | home/sleep-place observation, memory, household record, visible affordance | current place default | sleep-affordance validator + fixture |
| Food knowledge | observation/search/memory/testimony | raw food table / hidden pantry | actor-known planner guard + fixture |
| Route knowledge | observed adjacency/memory/map artifact | raw place adjacency hidden edge | route provenance test + guard |
| Wait reason | transaction/local plan/stuck diagnostic | scheduler window id injected into actor-visible proposal | sealed proposal + guard |

## 6. `TFW` checklist for this spec

| TFW requirement | Phase 3A answer | Required proof after implementation |
|---|---|---|
| Name each holder doing cognition | Ordinary actors in no-human driver, possessed actors via embodied TUI, view-model affordance selector as holder-known surface | Actor-known context ids in decision traces and embodied view evidence |
| Name sealed-context builder | Existing builder is insufficient where scheduler/projection manufactures facts; replace with provenance-bearing `NoHumanActorKnownSurfaceBuilder` and embodied context-backed affordance builder | Source guard and context-hash replay tests |
| Prove context excludes validation/debug/fixture truth | Current no-human/projection paths fail for raw workplace/sleep truth | Negative fixtures listed above |
| Prove selected goal/method cites only context inputs | Current fallback and string audit are insufficient | `SelectedGoalBundle` + typed provenance refs |
| Prove validator cannot suggest hidden targets | Existing doctrine/pipeline mostly aligns; this spec forbids scheduler/post-validation repair and requires typed failure feedback | Validator rejection tests with hidden food/workplace/sleep target absent from context |
| Prove debug cannot be parsed by embodied code | Embodied projection workplace helper violates the spirit by reading raw truth; debug comparison must be separate | View-model source guard and adversarial TUI fixture |
| Prove replay rebuilds context/metrics | Current capstone checks replay/metrics but metrics classification is partly string-based | Typed metrics schema and replay byte-match gate |

## 7. Adversarial fixture and test obligations

The execution gate requires integrated no-human day, food unavailable, routine no-teleport, possession-does-not-reset-intention, hidden-truth planning, planner trace, and routine blocker coverage. Current fixture files exist in the manifest/fetch set, including:

- `crates/tracewake-content/src/fixtures/no_human_day_001.rs`
- `crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs`
- `crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs`
- `crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs`
- `crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs`
- `crates/tracewake-content/src/fixtures/planner_trace_001.rs`
- `crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs`
- supporting ordinary-life fixtures `sleep_eat_work_001.rs`, `ordinary_workday_001.rs`, `no_human_advance_001.rs`, and `no_human_epistemic_check_001.rs`.

Existing tests also include useful anti-contamination coverage: hidden food and unknown routes are checked in planner/transaction tests; actor-known constructors are not freely public; no-human capstone asserts pipeline events, event counts, replay final state, replay traces, and replay metrics. Evidence: hidden-truth tests around actor-known construction and planner denial (`https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/hidden_truth_gates.rs`, `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/hidden_truth_gates.rs`, `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/hidden_truth_gates.rs`, `https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/hidden_truth_gates.rs`); no-human capstone replay/metrics checks (`https://github.com/joeloverbeck/tracewake/blob/01e860b3f26fb087d0fa7437146dd9901e1a4019/crates/tracewake-core/tests/no_human_capstone.rs`). These are necessary but not sufficient because they do not catch the scheduler/projection raw-truth paths and the string-based proof surfaces named above.

### Required strengthening map

| Required fixture family | Current status | Required strengthening |
|---|---|---|
| Integrated no-human day | Present | Add adversarial raw workplace/sleep truth absent from context and prove no plan/affordance uses it. |
| Food unavailable / replan | Present | Add typed validation feedback that does not reveal hidden replacement food. |
| Routine no-teleport | Present | Add provenance check that workplace route comes from actor-known route, not raw adjacency. |
| Possession does not reset intention | Present | Add explicit check that possessed embodied view does not gain raw workplace/sleep/food facts. |
| Hidden-truth planning | Present | Extend from planner/transaction unit seam into scheduler no-human context and embodied view seam. |
| Planner trace | Present | Require typed provenance refs, not display-string actor-known inputs. |
| Routine blocker | Present | Require typed responsible layer and replay-rebuilt stuck diagnostic. |

## 8. ORD-LIFE-CERT-scoped acceptance artifact

The acceptance artifact for this spec must be a new report under `reports/` after implementation, not a claim that the whole gate has passed. It must include:

1. Exact target commit and manifest evidence for the implementation under review.
2. Full list of context-builder source guards and their passing output.
3. Event log excerpt for each adversarial fixture showing proposal ancestry, validation report, stuck/feedback diagnostic, and replayed event ids.
4. Actor-known context packet ids, context hashes, and provenance edges for each selected goal/method/local plan.
5. Validation reports proving forged/stale/raw sleep/work/food parameters reject without revealing hidden targets in actor-visible summaries.
6. No-human metrics report built from typed event fields and byte-identical under replay.
7. TUI embodied/debug artifact showing embodied surfaces omit debug/raw workplace facts while debug can compare non-diegetically.
8. Responsible-layer matrix using the execution-layer vocabulary: `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, and `debug_quarantine`.
9. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not latest-main verification, not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 9. Implementation constraints

- No backwards-compatibility shim or alias path.
- No doctrine amendment.
- No crate-dependency inversion: `tracewake-core` remains dependency-free; `tracewake-content` depends on core; `tracewake-tui` depends on core/content.
- No branch-name fetches, code-search snippets, default-branch metadata, or manifest-text authority in acceptance evidence.
- External prior art may inform design wording only; Tracewake doctrine remains controlling.


## External prior art used only for design rationale

External work informed lock-mechanism choices, but never overrides Tracewake doctrine.

- Rao & Georgeff, *BDI Agents: From Theory to Practice* — used only to confirm the separation between beliefs/desires/intentions and practical implementation concerns for rational agents: https://aaai.org/papers/icmas95-042-bdi-agents-from-theory-to-practice/
- Georgievski & Aiello, *HTN planning: Overview, comparison, and beyond* — used only to frame HTN methods as domain/procedural knowledge whose search space must be explicit and inspectable: https://research.rug.nl/en/publications/htn-planning-overview-comparison-and-beyond/
- Orkin, *Three States and a Plan: The AI of F.E.A.R.* — used only as a design reminder that real-time GOAP planning should support replanning/adaptation without hand-authored scripts: https://gdcvault.com/play/1013394/Three-States-and-a-Plan
- Rust Reference, *Visibility and privacy* — used to justify sealed/private-field and accessor-only construction as build-enforced anti-contamination locks: https://doc.rust-lang.org/reference/visibility-and-privacy.html

## 10. Self-check

- [x] The determination is positive, and the spec is authored accordingly.
- [x] Every finding cites exact `INV-...` numbers re-derived from `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` at `01e860b3f26fb087d0fa7437146dd9901e1a4019`.
- [x] Every finding cites `ORD-LIFE-CERT` / `TFW` gate clauses and names responsible layers from the execution diagnostic vocabulary.
- [x] Every finding cites source files by named symbol at exact-commit URLs; line ranges are omitted because the symbol names are authoritative and grep-stable.
- [x] Every finding includes a structural lock designed to fail the build or test suite.
- [x] Scope remains within Phase 3A ordinary-life surface; Phase 1 spine, Phase 2A epistemics, and Phase 4 institutions are not re-audited.
- [x] No proposed change introduces a backwards-compatibility shim, alias path, or crate dependency inversion.
- [x] No doctrine is amended; potential gaps are handled as future-doctrine recommendations, not silent design changes.
- [x] The spec is framed as scoped evidence toward `ORD-LIFE-CERT` and does not claim the gate is passed.
- [x] Filename and staging path match the required `0014` spec.
- [x] External sources are cited and subordinated to Tracewake doctrine.
- [x] The fetch baseline contains every repository file relied on.
