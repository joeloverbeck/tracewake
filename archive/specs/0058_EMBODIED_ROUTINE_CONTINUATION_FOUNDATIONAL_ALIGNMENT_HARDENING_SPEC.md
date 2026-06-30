# 0058 Embodied Routine Continuation Foundational Alignment Hardening Spec

**Status**: COMPLETED
**Intended repository path:** `archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md`
**Series:** parallel `specs/NNNN` feature/hardening series, staged in `specs/` and archived to `archive/specs/` only after acceptance.
**Posture:** scoped feature-hardening implementation spec, not a project-wide certification audit.
**P0-CERT:** not applicable.
**First-proof / Phase-4 / second-proof:** not claimed.
**Authority posture:** subordinate to live doctrine tiers. This spec mints no invariant, gate, glossary term, or design-risk ID. Any amendment language below is routed substance + home only, not ratified doctrine.

---

## 0. Status / posture header

This spec is the next contiguous staging item after archived spec 0057. It hardens the already-merged embodied routine-continuation feature by auditing the baseline implementation against the doctrine pack, identifying foundational-alignment defects, and requiring anti-regression locks strong enough that future code cannot quietly reintroduce them.

This spec is intentionally narrow. It does **not** certify current `main`, the whole repository, no-human ordinary life, first proof, P0, Phase 4, second proof, institutions, LOD, speech/LLM, economy, story sifting, or any unrelated subsystem. It does not promote archived history into live doctrine. It does not weaken any upstream tier.

---

## 1. Baseline statement & source discipline

**Audited repository:** `joeloverbeck/tracewake`  
**Audited commit:** `4382f6db10b1cad247ea2793c94a6cda81f36d6f` (`4382f6d`)  
**Freshness claim:** user-supplied target commit only. This spec does not independently verify that the commit is current `main`.  
**Manifest role:** the uploaded `manifest_2026-06-30_4382f6d.txt` is treated only as a path inventory for the stated commit.  
**Repository acquisition:** every target-repository file used for repository-state claims was fetched from an exact URL containing owner `joeloverbeck`, repository `tracewake`, full commit `4382f6db10b1cad247ea2793c94a6cda81f36d6f`, and the exact manifest path. No clone, branch fetch, repository metadata lookup, default-branch lookup, GitHub code search, search snippet, connector repository selection, or prior chat memory is used as target-repository evidence.

Named symbols and paths are authoritative. Any rendered line numbers in the audit notes are provenance aids only; the implementing session must re-check the named symbols at its own implementation commit and must name that exact commit in the acceptance artifact.

The acquisition ledger appears in Appendix A. The ledger records 96 unique target-repository files. Two exact-commit GitHub blob URLs were additionally opened only to render already-acquired files with better line context; they are listed separately and do not add new file evidence.

---

## 2. Scope

### 2.1 In scope

This spec covers the 0057 embodied `Continue routine` implementation and the minimum adjacent machinery needed to harden it:

- `crates/tracewake-core/src/agent/routine_continuation.rs` — shared actor-known follow-on resolver.
- `crates/tracewake-core/src/runtime/session.rs` — embodied command surface, marker + follow-on composition, recursive follow-on guard, stuck outcome, and receipt composition.
- `crates/tracewake-core/src/actions/defs/continue_routine.rs` — marker event builder and authoritative active-intention validation.
- `crates/tracewake-core/src/scheduler.rs` — no-human/world-step transaction semantics, stuck diagnostics, temporal authority, and `advance_until` stop reasons. Its `routine_window_family` / `eligible_routine_execution_for_actor` helpers are the autonomous routine-family derivation baseline the embodied path must match for R2; 0058 names them as the equivalence baseline but does not rewrite them (see §2.2).
- `crates/tracewake-core/src/actions/pipeline.rs` — shared action pipeline dispatch.
- `crates/tracewake-tui/src/app.rs` — TUI forwarding boundary.
- `crates/tracewake-tui/tests/parity/` and `crates/tracewake-tui/tests/embodied_flow.rs` — playable parity and embodied regression witnesses.
- `crates/tracewake-core/tests/anti_regression_guards.rs`, `crates/tracewake-core/tests/ci_workflow_guards.rs`, and `.cargo/mutants.toml` — standing guard/mutation machinery.
- The 0057 spec and acceptance report, sibling spec precedent 0046/0047, and 0056 focused-mutation precedent.

### 2.2 Out of scope

The autonomous no-human planner, scheduler cadence, and the 0047 authoritative world-tick coordinator are not rewritten here. 0058 consumes them and hardens the 0057 seam around them.

The marker event stays a marker. This spec does not make `continue_routine` itself count as behavioral progress.

This spec does not add auto-running a whole routine on one keystroke, multi-step fast-forward, or `advance_until` stop-reason changes.

This spec does not introduce new ontology, new affordance kinds, food/sleep redesign, institutional behavior, notices, travel, LOD, LLM/speech, or story-sifting.

This spec does not make any whole-project or latest-`main` certification claim, and does not mint a new invariant, gate, glossary term, or risk ID.

---

## 3. Determination / verdict

### 3.1 Verdict

**The 0057 baseline at `4382f6d` is not clean under harsh foundational review.**

The baseline correctly establishes several major alignment properties: the `continue_routine` event is an explicit non-progress marker, the shared resolver consumes sealed actor-known context, the TUI forwards rather than simulating, and the embodied path reuses the core pipeline for ordinary follow-ons.

However, two foundational-alignment failures require remediation:

1. **OQ1 / active-intention authority violation.** `LoadedWorldRuntime::embodied_routine_window_family` can choose `RoutineFamily::WorkBlock` from the actor-known current workplace before it consults the single authoritative active intention. That is a possessed-path fork in routine-step authority. It can select the routine family from an actor-known environmental fact rather than from the actor's active intention and current unresolved step. This violates the spirit of INV-035, INV-094, INV-098, INV-101/102, INV-104, and the 0057 OQ1 settlement requirement. It is actor-known rather than hidden truth, so it is **not** a hidden-truth leak; the defect is that actor-known context is being used as a routine-family selector ahead of the active intention.

2. **R1 / temporal authority acceptance failure.** `run_embodied_continue_routine_follow_on` direct-runs the follow-on proposal through `run_pipeline` at the current scheduler tick. A `wait` proposal is an ordinary action whose builder emits `ActorWaited` and need-delta events at `requested_tick.advance_by(ticks)`. The world-step coordinator already has special handling for controlled `wait` proposals so time advances under scheduler authority. The embodied follow-on path lacks a visible guard that either routes a time-advancing follow-on through the world-step transaction or rejects it as typed stuck. Even if the current golden workday does not trigger that path, harsh feature acceptance does not allow an unguarded tick/need-accounting escape hatch. This violates INV-098 and INV-112 until the seam is explicitly closed.

No strict hidden-truth selection violation was proven in `resolve_routine_step_follow_on()` itself. No TUI simulation-authority leak was proven in `tracewake-tui/src/app.rs::submit_entry()`. No `advance_until` stop-reason change was proven, but R3 still needs an explicit anti-regression witness because the follow-on receipt path now composes marker and follow-on results.

### 3.2 Evidence table

| Holding | Verdict | Doctrine / contract | Baseline evidence | Required action |
|---|---:|---|---|---|
| H-0058-01 — marker-is-not-progress | Aligned, keep locked | INV-035, INV-094, INV-098; `docs/2-execution/06`; 0057 marker rule | `crates/tracewake-core/src/actions/defs/continue_routine.rs::build_continue_routine_event` validates the active intention and emits `intention_mutated=false`, `behavioral_progress=false`, and a marker-only effects summary. Existing guard `guard_0057_continue_routine_progress_of_record_is_follow_on` scans for this. | Preserve. Add one behavioral replay test that proves a marker-only receipt cannot advance the routine step or count as progress. |
| H-0058-02 — shared actor-known resolver | Mostly aligned, harden | INV-099, INV-100, INV-101, INV-102, architecture `03`, execution `04` | `crates/tracewake-core/src/agent/routine_continuation.rs::resolve_routine_step_follow_on` builds from `ActorKnownPlanningContext`, calls the same local planner, seals proposal ancestry, and has hidden-truth audit tests. `ActorDecisionTransaction::run` also constructs actor-known facts and calls this resolver. | Preserve. Add metamorphic output-equivalence tests proving embodied and autonomous resolution produce the same proposal from equivalent actor-known state. |
| H-0058-03 — OQ1 active-intention current-step authority | **Violation** | INV-035, INV-094, INV-098, INV-101/102, INV-104; 0057 OQ1 | `crates/tracewake-core/src/runtime/session.rs::embodied_routine_window_family` first checks `actor_known_context.known_workplaces()` and returns `RoutineFamily::WorkBlock` when the current place is a known workplace. Only after that does it read `active_intention_by_actor`, the active intention, and selected routine method. Its test explicitly requires the early work-block branch to win even with empty agent state. | Remove the pre-active-intention workplace branch. Derive routine family/step only from the single authoritative active intention's selected routine method and unresolved current step. Actor-known context may parameterize or validate, never override that authority. |
| H-0058-04 — R1 tick/need accounting | **Violation / harsh-acceptance failure** | INV-018, INV-092, INV-098, INV-112; architecture `02`; execution `05`/`06`; 0057 R1 | `run_embodied_continue_routine_follow_on` commits the follow-on with direct `run_pipeline` under `SchedulePhase::HumanCommand` at `decision_tick`. `wait` emits `ActorWaited` and need deltas at `proposal.requested_tick.advance_by(tick_count)`. `DeterministicScheduler::transact_world_one_tick` has a scheduler-owned path for controlled `wait` proposals after the world-step marker; the embodied direct path does not use it. | Add a follow-on temporal gateway. Same-tick ordinary follow-ons may use direct pipeline; time-advancing follow-ons must be routed through the world-step transaction as controlled proposals or rejected as typed stuck until that route is implemented. Add single-charge tests. |
| H-0058-05 — R2 no fork | Partially aligned, under-proved because H-0058-03 forks the embodied family selector | INV-094, INV-098, INV-099, INV-104; 0057 R2 | Shared resolver exists, but `run_embodied_continue_routine_follow_on` (`session.rs`) passes `routine_window_family: embodied_routine_window_family(...)`. The autonomous baseline is a *different* helper, `scheduler.rs::routine_window_family`, which derives family from `eligible_routine_execution_for_actor(window)` (start-tick/deadline/min-by-start keyed) and applies a workplace check in the opposite, correct direction (downgrade `WorkBlock`→`GoToWork` when the actor is *not* at a known workplace). Because the embodied helper instead *upgrades* to `WorkBlock` from the workplace branch and bypasses the active intention, embodied command resolution can diverge from autonomous transaction resolution. | After H-0058-03, add exact equivalence tests over fixed actor-known state comparing embodied follow-on output against the autonomous baseline (`scheduler.rs::routine_window_family`): action id, target ids, parameters, local plan id, proposal ancestry class, and hidden-truth audit must match. A proven divergence that would require changing the autonomous derivation itself is a follow-up scheduler spec, not 0058 (§2.2, §9.3). |
| H-0058-06 — R3 stop-reason semantics | No violation proven, needs lock | INV-095, INV-098; 0047 world advance contract; 0057 R3 | `LoadedWorldRuntime::advance_until` still calls `scheduler.advance_until(...)` and returns scheduler `AdvanceUntilResult`; the embodied follow-on path is separate. No code path was found that mutates `AdvanceUntilStopReason`. | Add regression test proving a continue-routine submission and stuck receipt do not change `advance_until` stop reasons or `ticks_advanced` arithmetic. |
| H-0058-07 — stuck diagnostics and `stuck_actors` accounting | No strict violation proven, but under-locked | INV-015, INV-098, INV-105; execution `10`; 0057 §1.2 observation | The embodied path can append scheduler-owned stuck diagnostics and prefix them into receipts; stuck outcomes build typed `StuckDiagnosticRecorded` events. Existing tests check source reuse/de-duplication, but there is no focused property that a successful follow-on cannot create or report a stuck actor solely from the success path. | Add success-path and stuck-path behavioral tests: success creates no new stuck diagnostic for the actor; stuck creates exactly one typed current diagnostic plus any pre-existing scheduler-owned diagnostics, with no duplicate scan effects. |
| H-0058-08 — TUI de-authority | Aligned, keep locked | INV-069, INV-095, INV-108; architecture `10` | `crates/tracewake-tui/src/app.rs::submit_entry` forwards the selected semantic action to the runtime command and retains only receipt/render handling and parity debug assertions. | Preserve with a source-scan guard and a playable parity row that asserts outcome, not just menu presence. |

---

## 4. Findings & remediation requirements

### 4.1 F-0058-01 — Embodied continuation must resolve from the active intention's current step

**Finding.** The baseline embodied helper treats “the actor knows this current place is a workplace” as sufficient to return `RoutineFamily::WorkBlock` before reading the actor's active intention. That is not a hidden-truth leak, but it is a routine-authority leak. The actor's knowledge can inform the local planner and validators; it cannot choose a different routine family before the active intention/current step is consulted.

**Doctrine anchor.** INV-035 requires routines to be defeasible intentions, not puppet strings. INV-094 requires possession parity. INV-098 requires harsh feature acceptance. INV-101/102 require sealed/provenanced actor-known context. INV-104 forbids routine labels from bypassing candidate generation, actor-known context, local planning, proposal construction, shared validation, and event commitment.

**Required remediation.**

- Replace `embodied_routine_window_family(...)` with a helper whose authority order is: actor id → `active_intention_by_actor` → active intention record → selected routine method / bound routine execution → unresolved current step. The helper may return `None` or a typed stuck diagnostic when that chain is absent or ambiguous.
- Remove the early `known_workplaces()` → `WorkBlock` branch. That branch *upgrades* to `WorkBlock` from an environmental fact ahead of the active intention; it is the inverted mirror of the autonomous baseline `scheduler.rs::routine_window_family`, which uses actor-known workplace knowledge only to *downgrade* `WorkBlock`→`GoToWork` when the actor is not at a known workplace. Preserve that legitimate `WorkBlock↔GoToWork` distinction: actor-known workplace knowledge may refine an already-selected work family (am I at the workplace yet, or must I still travel?), but it may not select a different routine family before the active intention is consulted.
- Do not scan all unresolved routine executions for a family unless the execution is bound to the active intention's selected method/execution and current step. Scanning all windows is exactly the assigned-but-inactive routine failure OQ1 warned about.
- When active-intention data is absent, return typed stuck rather than falling back to window/routine guesses.
- Update the existing unit witnesses for the removed branch: `embodied_routine_window_family_returns_work_block_at_known_workplace` (in `crates/tracewake-core/src/runtime/session.rs`) asserts the to-be-removed behavior on an empty `AgentState` and bakes its mutant-kill rationale into the workplace branch — invert or replace it with an active-intention-authority witness rather than adapting it to keep the old branch green. Preserve the sibling witnesses `embodied_routine_window_family_filters_executions_by_actor` and `embodied_routine_window_family_ignores_resolved_executions`, which exercise the post-branch active-execution scan and remain load-bearing.

### 4.2 F-0058-02 — Embodied follow-ons must not own time advancement outside the scheduler

**Finding.** The baseline direct follow-on path is acceptable for instantaneous ordinary actions such as move/eat/work-start validation at the current tick, but unsafe for any follow-on whose event semantics advance time or charge passive needs. `wait` is the visible seam: its builder emits an `ActorWaited` event and need deltas at `requested_tick.advance_by(tick_count)`. The world-step transaction already has a controlled-proposal branch for wait because time advancement is scheduler-owned. `run_embodied_continue_routine_follow_on` bypasses that branch.

**Doctrine anchor.** INV-018/092 require deterministic replay. INV-098 requires replay-safe and regression-tested feature acceptance. INV-112 says time may validate, but holder-known time must plan; the scheduler and replay clock may order and validate but must not become cognition authority. `docs/1-architecture/02` and `docs/2-execution/05` make the event log / scheduler pipeline the temporal authority.

**Required remediation.**

- Add a typed gateway near `run_embodied_continue_routine_follow_on` before `run_pipeline`:
  - `continue_routine` follow-ons remain recursive typed stuck.
  - Same-tick non-duration ordinary follow-ons may be committed through the shared pipeline at the marker tick.
  - Tick-advancing follow-ons (`wait`, any future action with duration/passive-need semantics) must not be direct-pipelined. They must either be routed through `DeterministicScheduler::transact_world_one_tick` as a controlled proposal with the marker in receipt ancestry, or rejected as a typed stuck diagnostic whose actor-visible reason is that continuation cannot safely commit a time-advancing follow-on yet.
- Resolve R1 explicitly in code comments and tests: the marker shares the current tick and is never progress; a non-time-advancing follow-on shares that marker tick; a time-advancing follow-on, when supported, advances on the next scheduler-owned world tick and charges needs exactly once.
- Add a replay/projection assertion that marker + follow-on reconstruction yields the same scheduler frontier and no duplicate need charge.

### 4.3 F-0058-03 — Stuck diagnostics must be honest and non-polluting

**Finding.** The baseline has typed stuck outcome machinery, but the success and stuck paths need stronger behavioral locks. A successful embodied continuation must not create a current stuck diagnostic for the actor merely because the stuck-diagnostic scan ran. Conversely, a genuine stuck continuation must create a typed diagnostic event that is eventful, replayable, and actor-known in its explanation.

**Doctrine anchor.** INV-015 makes consequential failure eventful. INV-098 requires debug-inspectable and regression-tested acceptance. INV-105 requires decision traces and stuck diagnostics to be typed/structurally inspectable.

**Required remediation.**

- Split “scheduler-owned prior diagnostic scan” from “current continuation stuck outcome” in naming and receipt assembly. A reader of the receipt must be able to tell whether a diagnostic predates the successful follow-on, arose from the current follow-on's rejection/stuck outcome, or is absent.
- Add success-path test: marker + accepted follow-on must not append `StuckDiagnosticRecorded` for the actor unless a prior scheduler-owned diagnostic was already due and de-duplicated.
- Add stuck-path test: a stuck transaction emits exactly one current typed stuck diagnostic and a rejected validation report with `RoutineStepBlocked`; duplicate scans do not emit duplicates.
- Add a post-work observation test: after an embodied workday continuation succeeds through work completion, `stuck_actors`/stuck-diagnostic accounting remains empty for that actor unless a real unresolved stale execution exists.

### 4.4 F-0058-04 — R2 must become a behavioral equivalence property, not just a shared-symbol assertion

**Finding.** The shared resolver exists, and that is the right architecture, but source sharing alone is too weak. The possessed path can still fork before it calls the shared resolver by passing a different `routine_window_family` than the autonomous transaction would derive.

**Doctrine anchor.** INV-094 and INV-108 make possession input-only. INV-099/101/102 define the actor-known firewall. INV-104 forbids bypass of the transaction pipeline.

**Required remediation.**

- Create a core test helper that builds one fixed actor-known state and active intention, then invokes both:
  - the autonomous baseline `ActorDecisionTransaction::run(...)` as the no-human path drives it — i.e. with `routine_window_family` derived by `scheduler.rs::routine_window_family` over the equivalent scheduled window, since that helper (not the active intention directly) is the autonomous family-derivation surface; and
  - the embodied follow-on resolution path after a valid `ContinueRoutineProposed` marker.
- Compare the selected proposal at semantic shape level: action id, actor id, target ids, relevant parameters, selected routine execution/current step, local plan id presence, proposal ancestry class, decision-trace outcome, and hidden-truth audit.
- Add adversarial variants: actor at a known workplace while active intention is eat/sleep; hidden workplace known only in truth; assigned but inactive routine execution exists; resolved routine execution exists; another actor's execution exists.
- Doctrine reconciliation: under INV-035/INV-103/INV-104 the single active intention is the planning authority and the scheduler is not a cognition authority, so the autonomous window-eligible execution must, in a correct model, be the execution bound to the active intention — making the active-intention-derived embodied family and the autonomous window-derived family coincide. The equivalence test is the binding proof of that coincidence. If an adversarial case proves the two derivations genuinely diverge in a way that can only be reconciled by changing the autonomous derivation, that is a new scheduler spec, not 0058 ticket creep (§2.2, §9.3).

### 4.5 F-0058-05 — TUI reachability must prove consequence, not only affordance presence

**Finding.** Existing parity row `spec0057.routine.embodied_continue_workday` correctly demands typed causal consequences, actor-knowledge witness, rendered witness, replay evidence, no-human evidence, and an anti-leak fixture. 0058 must extend that row class with active-intention and temporal-authority adversaries.

**Doctrine anchor.** INV-065/066/069/095/108 and architecture `10`.

**Required remediation.**

- Add one or two `spec0058.*` parity rows under `crates/tracewake-tui/tests/parity/census_actions.rs`:
  - `spec0058.routine.embodied_continue_active_intention_current_step`
  - `spec0058.routine.embodied_continue_temporal_authority` if the time-advancing route is implemented; otherwise encode the typed-stuck branch.
- Each row must measure typed causal evidence, actor-known evidence, rendered evidence, replay evidence, and anti-leak fixture evidence.
- Keep `tracewake-tui/src/app.rs` at forwarding altitude. Do not add simulation rules, target repair, routine-family selection, or hidden truth checks to TUI code.

---

## 5. Doctrine amendments routed as substance + home

No constitutional amendment is required to permit the capability. The current constitution already says what matters: possession is input-only, actor-known context is sealed, routines are defeasible intentions, the TUI is not simulation authority, and time/replay are scheduler-owned.

The following clarifications are routed as lower-tier substance. They are **not** ratified by this spec and do not mint new invariant, gate, glossary, or risk identifiers.

### 5.1 Active-intention authority for possessed routine continuation

**Home:** `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` and `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.

**Proposed substance:** A possessed actor's `Continue routine` follow-on is resolved from the same actor decision transaction authority as autonomous routine continuation. The selected routine family/current step must come from the actor's single authoritative active intention and its bound unresolved routine step. Actor-known context may supply perceived routes, workplaces, food sources, blockers, and validation context, but it may not select a different routine family before the active intention is consulted.

### 5.2 Temporal authority for marker + embodied follow-on

**Home:** `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`, `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`, and `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.

**Proposed substance:** `Continue routine` marker events are same-tick, non-progress evidence. A same-tick ordinary follow-on may be committed through the shared action pipeline. A tick-advancing follow-on, including wait/passive-need semantics, must be owned by the world-step transaction or fail closed as typed stuck. Need deltas for a continuation must be charged once and be replay-reconstructable from event ancestry.

### 5.3 Focused mutation evidence for scoped feature hardening

**Home:** `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` and `docs/4-specs/README.md`.

**Proposed substance:** For large standing mutation perimeters, a scoped hardening spec may require focused mutation campaigns over touched seam files plus `--iterate` convergence over `mutants.out`, reserving a full standing campaign for the end only when needed. This is an evidence discipline, not a weakening of the four local acceptance gates.

---

## 6. Required fixtures, tests, and anti-regression measures

### 6.1 Behavioral tests

Add or extend tests at the highest practical behavioral level before adding source scans.

Required core tests:

1. `embodied_continue_uses_active_intention_current_step_not_known_workplace`  
   Builds an actor at a known workplace with an active non-work routine step. The follow-on must match the active step, not `WorkBlock`.

2. `embodied_continue_assigned_inactive_window_does_not_drive_follow_on`  
   Adds an assigned-but-inactive routine execution with a tempting family. The follow-on must ignore it and either use the active intention's current step or produce typed stuck.

3. `embodied_continue_wait_follow_on_is_not_direct_pipelined`  
   Forces the resolver to propose `wait`. The baseline failure mode is direct `run_pipeline`; the fixed behavior must be either scheduler-owned world-step routing or a typed stuck diagnostic. The test must fail if `ActorWaited` appears at `decision_tick + 1` without scheduler frontier advancement.

4. `embodied_continue_time_advancing_follow_on_charges_needs_once`  
   Required only if scheduler-owned wait routing is implemented in 0058. It must assert exactly one hunger and one fatigue delta for the elapsed tick, no duplicate `NeedDeltaApplied`, and replay reconstruction of the final need state.

5. `embodied_continue_success_does_not_emit_current_stuck_diagnostic`  
   A successful move/work/eat follow-on must not append a current `StuckDiagnosticRecorded` for the actor.

6. `embodied_continue_stuck_emits_one_current_typed_diagnostic`  
   A genuine stuck resolution emits one typed current diagnostic with actor-known explanation and no duplicate diagnostic on re-scan.

7. `embodied_continue_and_autonomous_transaction_match_from_equivalent_actor_known_state`  
   Metamorphic equivalence test: autonomous and embodied outputs from equivalent actor-known state must match semantic proposal shape and hidden-truth audit.

8. `continue_routine_marker_alone_does_not_advance_routine_or_progress`  
   Event-log-level test proving the marker alone cannot increment routine progress, mutate the intention, move the actor, start work, or satisfy scheduler progress accounting.

9. `embodied_continue_receipt_does_not_change_advance_until_stop_reason`  
   Regression around R3: stop reasons and `ticks_advanced` remain owned by `advance_until`, not by marker/follow-on receipt composition.

### 6.2 Playable-capability parity rows

Extend `crates/tracewake-tui/tests/parity/census_actions.rs` with staged rows under `OwnershipScope::FuturePack { namespace: "spec0058_embodied_routine_continuation_foundational_alignment" }`.

Each row must have:

- `CapabilityClass::ActorObservableConsequence`.
- `SurfaceDisposition::Embodied`.
- typed causal witness for marker + follow-on or marker + typed stuck.
- actor-knowledge witness proving the target/family came from active intention and actor-known context, not hidden truth or assigned inactive windows.
- rendered witness proving the TUI shows the resulting actor-known state or actor-visible stuck explanation.
- replay evidence required.
- anti-leak fixture when hidden truth/workplace/adversarial inactive routine is present.

### 6.3 Source-scan / meta-lock guards

Add guards to `crates/tracewake-core/tests/anti_regression_guards.rs` only after the behavioral tests exist. Each new guard needs a lock id, negative id, and a non-vacuous witness minimum consistent with the house meta-lock registry. Use the house `negative_id` convention — snake_case `synthetic_<description>` identifiers (as in the existing registry, e.g. `synthetic_meta_lock_without_negative`), not a dotted `negNNNN.` form.

Required guards:

1. `guard_0058_embodied_routine_family_has_no_pre_intention_workplace_selector`  
   **Negative id:** `synthetic_workplace_before_active_intention`  
   **Witness minimum:** fails on a synthetic source that returns `RoutineFamily::WorkBlock` from `known_workplaces()` before `active_intention_by_actor()`.

2. `guard_0058_embodied_continue_time_advancing_follow_on_is_gated`  
   **Negative id:** `synthetic_direct_wait_follow_on`  
   **Witness minimum:** fails on a synthetic source that runs `run_pipeline` for a `wait` follow-on without checking the temporal gateway.

3. `guard_0058_embodied_continue_success_path_not_current_stuck`  
   **Negative id:** `synthetic_success_prefixed_current_stuck`  
   **Witness minimum:** fails on a synthetic source that builds current `StuckDiagnosticRecorded` before matching the transaction outcome and prefixes it into success as if it were ordinary receipt ancestry.

4. `guard_0058_tui_continue_routine_forwards_only`  
   **Negative id:** `synthetic_tui_routine_selection`  
   **Witness minimum:** fails if `tracewake-tui/src/app.rs` imports or calls routine/planner/session internals beyond the runtime command surface or constructs follow-on targets.

### 6.4 Focused mutation discipline

Use focused mutation. Do **not** design the work around repeated full standing campaigns across the ~3.5K-mutant perimeter.

Required sequence:

1. Run the four local gates before mutation:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

2. Run focused mutation over the touched 0057/0058 seam files. Use `--in-diff` when a diff file is available, and/or per-file `--file` invocations for:

```text
crates/tracewake-core/src/runtime/session.rs
crates/tracewake-core/src/agent/routine_continuation.rs
crates/tracewake-core/src/actions/defs/continue_routine.rs
crates/tracewake-core/src/actions/defs/wait.rs
crates/tracewake-core/src/actions/defs/need_events.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-tui/src/app.rs
crates/tracewake-tui/tests/parity/census_actions.rs
crates/tracewake-tui/tests/parity/runner.rs
crates/tracewake-core/tests/anti_regression_guards.rs
```

3. Converge missed mutants with `cargo mutants --iterate` over the existing `mutants.out` results. Add behavioral tests for meaningful misses; do not adapt tests to bugs. Treat unviable/equivalent mutants honestly in the acceptance artifact.

4. Reserve at most one final full standing campaign for the very end, only if the implementation materially changes cross-seam behavior or if focused results suggest broad coverage erosion.

This discipline follows cargo-mutants' own guidance: `--in-diff` targets changed code, `--file` can retest changed files, `--iterate` skips previously caught/unviable mutants while accumulating results, and mutation misses are evidence of likely coverage gaps that should usually be addressed by higher-level public behavior tests rather than mutant-shaped private tests. External source references appear in Appendix B.

---

## 7. Acceptance artifact & evidence

The closeout artifact must use `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and must be staged under `archive/reports/0058_..._acceptance.md` at acceptance.

The acceptance artifact must include:

- implementation commit SHA, full length, produced by the implementing session;
- explicit statement that the audited baseline was `4382f6d` but the accepted implementation commit is the new implementation commit;
- a per-ticket evidence table;
- four-gate command transcripts or exact command results:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace`
- focused mutation commands, tool version, filter/per-file scope, `mutants.out` summary, missed/unviable/timeout disposition, and whether a final full campaign was run;
- replay evidence for marker + follow-on reconstruction;
- parity report evidence for all `spec0058.*` rows;
- explicit non-certification statement: no P0-CERT, first-proof, Phase-4, second-proof, latest-main, or whole-project claim;
- doctrine amendment routing section, if any docs are changed, clearly saying the spec proposed substance but did not ratify new invariants/gates/glossary/risk IDs.

Acceptance fails if any target-repository claim in the artifact depends on an unfetched file, branch name, repository snippet, memory, or another repository.

---

## 8. Implementation decomposition

### 0058EMBROUFOU-001 — Active-intention current-step authority

**Reviewable diff:** `crates/tracewake-core/src/runtime/session.rs`; supporting unit tests in the same module or core integration tests.

**Work:** Remove the early known-workplace `WorkBlock` selector; derive routine family/current step solely from active intention and bound unresolved execution. Preserve the legitimate `WorkBlock↔GoToWork` workplace-known distinction (mirror of the autonomous `scheduler.rs::routine_window_family` downgrade), not the inverted upgrade-to-`WorkBlock` branch. Return typed stuck for missing/ambiguous active-intention data. Invert or replace the existing `embodied_routine_window_family_returns_work_block_at_known_workplace` witness rather than adapting it to keep the removed branch green; preserve the `_filters_executions_by_actor` and `_ignores_resolved_executions` siblings.

**Acceptance:** OQ1 adversarial tests pass. Mutants that invert active-intention/execution filters are caught.

### 0058EMBROUFOU-002 — Temporal gateway for embodied follow-ons

**Reviewable diff:** `crates/tracewake-core/src/runtime/session.rs`; possibly `crates/tracewake-core/src/scheduler.rs` only if scheduler-owned controlled follow-on routing is implemented.

**Work:** Add a typed gateway for time-advancing follow-ons. Either route `wait` through `transact_world_one_tick` as a controlled proposal or reject it as typed stuck. Preserve same-tick direct pipeline for non-time-advancing ordinary actions.

**Acceptance:** Direct-pipeline wait mutant is caught. Single-charge/replay test passes if wait routing is implemented; typed-stuck test passes if wait routing is deferred.

### 0058EMBROUFOU-003 — Stuck diagnostics and receipt honesty

**Reviewable diff:** `crates/tracewake-core/src/runtime/session.rs`; possible test support in `crates/tracewake-core/src/scheduler.rs` tests.

**Work:** Clarify prior scheduler diagnostics vs current continuation stuck outcome. Ensure success path does not create/prefix a current stuck diagnostic. Ensure stuck path emits exactly one current typed diagnostic.

**Acceptance:** Success/stuck diagnostic tests pass; duplicate scan test remains non-vacuous.

### 0058EMBROUFOU-004 — Resolver/no-fork metamorphic proof

**Reviewable diff:** `crates/tracewake-core/src/agent/routine_continuation.rs`, `crates/tracewake-core/src/agent/transaction.rs` tests, and/or runtime integration tests.

**Work:** Add equivalence helpers comparing autonomous transaction output to embodied follow-on output from identical actor-known state. Cover active-intention/current-step adversaries and hidden-workplace anti-leak cases.

**Acceptance:** R2 property fails on synthetic resolver fork and passes on fixed code.

### 0058EMBROUFOU-005 — TUI parity and de-authority locks

**Reviewable diff:** `crates/tracewake-tui/tests/parity/*`, `crates/tracewake-tui/tests/embodied_flow.rs`, and `crates/tracewake-tui/src/app.rs` only if necessary to keep forwarding clean.

**Work:** Add `spec0058.*` parity rows and scenario measurement for active-intention and temporal authority. Keep TUI as runtime-command forwarder only.

**Acceptance:** `cargo test --workspace` includes parity rows; source-scan guard fails on synthetic TUI simulation authority.

### 0058EMBROUFOU-006 — Meta-lock guards and focused mutation

**Reviewable diff:** `crates/tracewake-core/tests/anti_regression_guards.rs`, `crates/tracewake-core/tests/ci_workflow_guards.rs` only if new touched files are outside existing trigger coverage, and acceptance artifact mutation evidence.

**Work:** Add source-scan guards only after behavioral tests. Run focused mutation. Disposition misses honestly.

**Acceptance:** New guards include negative ids and witness minima. Focused mutation catches the meaningful mutants named by this spec, or documents equivalent/unviable disposition.

### 0058EMBROUFOU-007 — Acceptance closeout and archive routing

**Reviewable diff:** acceptance report only at closeout; archive move only after acceptance.

**Work:** Produce `archive/reports/0058_..._acceptance.md`, then archive the accepted spec to `archive/specs/`. Add `SPEC_LEDGER.md` archived-row entry only at acceptance/closeout.

**Acceptance:** Closeout artifact proves all tickets, commands, mutation scope, and non-certification posture.

---

## 9. Risks / open questions and invariants alignment

### 9.1 0057 risk closure posture

- **R1 — marker + follow-on tick/need accounting:** closed by this spec's required gateway. Marker is same-tick non-progress. Same-tick ordinary follow-on is allowed. Tick-advancing follow-on must be scheduler-owned or typed stuck. Need deltas must be single-charged and replayable.
- **R2 — no resolver fork:** not fully closed at baseline because `embodied_routine_window_family` forks before shared resolution, deriving family by an inverted workplace upgrade while the autonomous baseline `scheduler.rs::routine_window_family` derives from the window-eligible execution. Closed only after active-intention authority is fixed and metamorphic tests prove the embodied derivation coincides with that autonomous baseline. Any adversarial divergence that can only be reconciled by changing the autonomous derivation is deferred to a follow-up scheduler spec (§2.2).
- **R3 — stop reason semantics:** no baseline violation proven. Closed by an explicit regression test that `advance_until` stop reasons remain scheduler-owned.
- **OQ1 — active-intention current step:** baseline violation. Closed by removing the known-workplace/window selector and deriving only from the authoritative active intention's current unresolved step.
- **Post-work `stuck_actors` observation:** no strict baseline violation proven, but the success/stuck receipt paths need stronger locks. Closed by current-vs-prior diagnostic tests and post-work no-stuck accounting assertion.

### 9.2 Invariant alignment matrix

| Invariant / contract | 0058 alignment requirement |
|---|---|
| INV-035 routines are defeasible intentions | Continue follows active intention/current step, not environment shortcuts. |
| INV-094 possession parity is tested | Embodied and autonomous output equivalence is behavioral, not merely source-shared. |
| INV-095 TUI/view-model tests are acceptance tests | New parity rows prove actor-visible consequence through TUI surface. |
| INV-098 feature acceptance is harsh | No unguarded time/need-accounting seam, no under-tested fork. |
| INV-099 truth may validate but not plan | Resolver consumes actor-known context; validators may reject against truth. |
| INV-101/102 sealed/provenanced actor-known context | Actor-known facts can parameterize planning but not bypass active intention. |
| INV-104 routines/needs do not dispatch primitive actions directly | Routine family/current step routes through transaction, resolver, planner, proposal, pipeline. |
| INV-105 typed diagnostics | Stuck outcomes are typed events, not display-string authority. |
| INV-108 human possession is cognition-neutral | Binding a controller changes input/viewpoint only; it does not improve or redirect routine competence. |
| INV-112 temporal authority | Time-advancing follow-ons are scheduler-owned or rejected. |

### 9.3 Remaining risks

The focused mutation strategy deliberately does not prove the full standing mutation perimeter. That is acceptable for this scoped feature-hardening spec only because the seam files are inside the existing perimeter and because at most one final full campaign remains available if the focused campaign exposes cross-seam erosion.

The spec intentionally avoids redesigning planner cadence, no-human routine selection, and the 0047 coordinator. If implementation pressure suggests those must change, that is a new spec, not ticket creep in 0058.

---

## Appendix A — Exact-commit acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 4382f6db10b1cad247ea2793c94a6cda81f36d6f
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run.open with full exact raw.githubusercontent.com URLs; supplementary exact GitHub blob URLs used only to render already-acquired files with better line context
Requested unique repository file count: 96
Successfully verified unique repository file count: 96
Fetched repository files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/routine_continuation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/runtime/session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/continue_routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/pipeline.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/tests/anti_regression_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/tests/ci_workflow_guards.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/.cargo/mutants.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/parity/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/parity/census_actions.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/parity/runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/parity/scenario.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/archive/reports/0057_embodied_routine_continuation_acceptance.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/archive/specs/0056_FOUNDATIONAL_CONFORMANCE_SEVENTH_HARDENING_SEALED_VALIDATED_CONTENT_BOOTSTRAP_OPERATOR_GATED_DEBUG_AUTHORITY_FAIL_CLOSED_TAXONOMY_AND_CURRENT_SYMBOL_NEGATIVE_FIXTURES_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/transaction.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/decision.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/planner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/routine.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/registry.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/runtime/command.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/runtime/receipt.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/need_accounting.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/events/apply.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/replay/rebuild.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-tui/tests/playable_capability_parity.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/tests/world_step_coordinator.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-content/src/fixtures/embodied_continue_hidden_workplace_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/wait.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/need_events.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/work.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/eat.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/actions/defs/sleep.rs

Supplementary exact-commit render URLs for already-acquired files, not counted as additional unique files:
- https://github.com/joeloverbeck/tracewake/blob/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/runtime/session.rs
- https://github.com/joeloverbeck/tracewake/blob/4382f6db10b1cad247ea2793c94a6cda81f36d6f/crates/tracewake-core/src/scheduler.rs

Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

---

## Appendix B — External research lane

External sources shaped the testing and evidence discipline only. They are not used to assert target-repository state.

1. Microsoft Azure Architecture Center, “Event Sourcing Pattern.” The event-sourcing model records logical changes in an append-only store, treats event streams as authoritative history, and derives current state by replay/rehydration. This supports the 0058 requirement that marker + follow-on behavior remain replay-reconstructable and event-ancestry-preserving.  
   Source: `https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing`

2. Gabriel Gambetta, “Client-Side Prediction and Server Reconciliation.” The source describes the split between client responsiveness and authoritative server processing of input. This supports treating the TUI as an input/rendering boundary rather than a simulation authority.  
   Source: `https://www.gabrielgambetta.com/client-side-prediction-server-reconciliation.html`

3. Alzahrani, Spichkova, and Harland, “Application of Property-based Testing Tools for Metamorphic Testing,” ENASE 2022. The paper frames metamorphic relations as a way to specify expected relations across executions and property-based tools as automation for those relations. This supports the R2 equivalence lock: embodied output must match autonomous output from equivalent actor-known state.  
   Source: `https://www.scitepress.org/Papers/2022/111017/111017.pdf`

4. cargo-mutants documentation, “Testing code changed in a diff.” The `--in-diff` option tests mutants overlapping changed regions and is useful for large projects, while the docs caution that incremental mutation is not a full substitute. This supports 0058's focused mutation discipline with an optional final full campaign.  
   Source: `https://mutants.rs/in-diff.html`

5. cargo-mutants documentation, “Iterating on missed mutants.” The `--iterate` option skips previously caught/unviable mutants and accumulates results while iterating on missed mutants. This supports the required `mutants.out` convergence loop.  
   Source: `https://mutants.rs/iterate.html`

6. cargo-mutants documentation, “Using the results.” The docs classify missed mutants as likely coverage gaps and recommend adding tests at the right abstraction level rather than narrowly targeting private implementation details. This supports the spec's “behavioral first, source-scan second” lock order.  
   Source: `https://mutants.rs/using-results.html`

7. cargo-mutants documentation, “The mutants.out directory.” This describes `mutants.out`, result files, logs, diffs, and `previously_caught.txt`. This supports the acceptance artifact's mutation evidence requirements.  
   Source: `https://mutants.rs/mutants-out.html`

## Outcome

Completed: 2026-06-30

Implemented the `0058EMBROUCON` ticket series in dependency order and archived tickets `0058EMBROUCON-001` through `0058EMBROUCON-007`.

Implementation summary:

- Removed the embodied pre-active-intention workplace selector and made active intention/current unresolved routine execution the authority for `continue_routine` family selection.
- Added a temporal gateway that rejects time-advancing embodied follow-ons as typed stuck instead of direct-pipelining `wait`.
- Split prior scheduler stuck diagnostics from current stuck outcomes and added success/stuck receipt honesty tests.
- Added embodied/autonomous parity tests over equivalent actor-known state, including hidden-workplace and inactive/other-actor adversaries.
- Added two real-pipeline `spec0058.*` TUI parity rows and runner checks for active-intention and temporal-authority consequences.
- Added four 0058 meta-lock source guards with non-vacuous `synthetic_*` negative witnesses.
- Produced `archive/reports/0058_embodied_routine_continuation_foundational_alignment_acceptance.md`.

Verification:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.
- Focused mutation evidence was recorded with `cargo-mutants 27.1.0`; the final iterate run remained survivorful (15 tested, 8 caught, 7 missed, 0 unviable, 0 timeout), so this spec makes no mutation-perfect or certification-style claim.

Acceptance posture:

- This closeout accepts scoped `0058` feature-hardening evidence only.
- The acceptance report records a computed `non-pass` status for certification-style use because focused mutation remains survivorful.
- This spec does not certify latest main, Phase-4 entry, second-proof entry, `FIRST-PROOF-CERT`, `P0-CERT`, or any whole-project status, and it mints no invariant, gate, glossary term, or risk ID.
