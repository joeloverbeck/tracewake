# Tracewake Phase 3A Second Hardening Specification

## Specification metadata

- **Spec ID:** 0007
- **Title:** Phase 3A Second Hardening: Integrated No-Human Ordinary-Life Proof
- **Status:** Landed (2026-06-07).
- **Phase:** Phase 3A second hardening — a narrow ordinary-life substrate slice of execution Phase 3 (`docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md`), not full Phase 3.
- **Predecessor:** Spec 0006 (`archive/specs/0006_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_HARDENING_SPEC.md`); final result recorded in `docs/4-specs/SPEC_LEDGER.md` under "Spec 0007".

Source commit and freshness are recorded under "Static audit target" below.

## Static audit target

Repository: `joeloverbeck/tracewake`

Target commit: `93759ded3967936431c359a4ec8eefa1619b518b`

Freshness claim: user-supplied target commit only; not independently verified as latest `main`.

Audit mode: static exact-commit source audit. The test suite was not run.

## Purpose

This specification repairs the remaining Phase 3A ordinary-life substrate gaps after the first hardening slice. The goal is not to add Phase 3B speech, Phase 4 institutions, personality modeling, social systems, or richer narrative machinery. The goal is narrower and stricter: Tracewake must be able to run a boring ordinary no-human day through real needs, durable intentions, defeasible routines, actor-known planning, shared action validation, event ancestry, TUI reachability, debug explanation, and deterministic replay strongly enough that later Phase 3 and Phase 4 work do not build on shortcut debt.

## Authority and scope

Authority order remains:

1. `docs/README.md`
2. `docs/0-foundation/**`
3. `docs/1-architecture/**`
4. `docs/2-execution/**`
5. `docs/3-reference/**`
6. `docs/4-specs/**`
7. archived implementation and hardening specs
8. exact-commit code, fixtures, and tests

This hardening spec applies to the Phase 3A substrate only:

- no-human day scheduling and actor decision generation;
- needs, thresholds, and action-derived need deltas;
- intentions and routine execution ancestry;
- HTN / local planning boundaries;
- hidden-truth safety for autonomous decisions;
- sleep, eat, work, wait, continue-routine, and movement ancestry;
- TUI, debug, replay, and provenance surfaces needed to inspect Phase 3A behavior;
- fixtures and tests proving integrated behavior rather than smoke, marker, or synthetic behavior.

## Non-goals

This spec does not require:

- Phase 3B speech or testimony;
- Phase 4 institutions, records, suspicion, courts, gossip, reports, or wrong-suspicion workflows;
- multi-day economic simulation beyond what is needed to prove one ordinary no-human day;
- LLM dialogue;
- MCTS, advanced GOAP, neural planning, personality modeling, or director systems;
- graphical client work;
- destructive rewrites of otherwise-correct Phase 1, Phase 2, or stable doctrine docs.

## Defects being corrected

The following defects are binding correction targets.

### D-01 — No-human day remains only partially integrated

The no-human runner considers multiple actors and can emit ordinary pipeline events, but the decision loop still contains fallback wait planning, direct routine template-name dispatch, and physical-state oracle proposal selection. The no-human day must instead derive every ordinary action from live actor state, actor-known planning inputs, and typed routine/need/intention decisions.

### D-02 — Live `AgentState` integration remains incomplete

The shared pipeline can mutate `AgentState`, and replay can rebuild agent state, but the no-human decision generator must consistently consume and advance live needs, active intentions, routine executions, decision traces, and blocker diagnostics through the same event ancestry it emits.

### D-03 — String-prefix / substring decision shortcuts remain

HTN method conditions are improved, but Phase 3A still contains unacceptable string shortcuts such as routine template `contains("sleep"/"food"/"work")`, candidate generation based on `starts_with("known_food:")`, and magic exact inputs such as `reason_available`. Replace these with typed conditions or typed actor-known facts.

### D-04 — Hidden-truth planning is not impossible by construction

A canonical actor-known planning type exists, and planner functions can reject unknown food/routes. That is not sufficient while no-human proposal builders choose food, workplaces, and movement destinations from authoritative physical state before planning. Autonomous planning must be structurally unable to inspect hidden physical truth as a decision oracle.

### D-05 — Intentions/routines are not yet durable live commitments in the no-human loop

Intention and routine structs/events exist, but the no-human loop must adopt, continue, progress, interrupt, complete, fail, abandon, and resume intentions/routines through events that drive later proposals. These events must not merely be available to tests; they must arise from the integrated scheduler path.

### D-06 — `continue_routine` remains a marker

A `ContinueRoutineProposed` event with `behavioral_progress=false` is not behavioral continuation. Continuing a routine must either produce an ordinary follow-on action in the same scheduler transaction or be explicitly treated as a non-progress diagnostic marker that never satisfies Phase 3A progress gates.

### D-07 — Diagnostics remain too label/string-heavy

Debug panels and traces exist, but Phase 3A proof must not depend on seeded labels, static hidden-truth audit strings, or pipe-delimited stuck summaries alone. Debug output must be derived from event ancestry and typed decision/planner/routine records.

### D-08 — TUI/debug reachability needs stronger behavioral proof

The TUI can run a no-human day and render panels. This spec requires tests that prove the TUI exposes actual no-human decisions, blockers, needs, intentions, routine progress, replay metrics, and actor-filtered views without hidden truth.

### D-09 — Tests remain partly smoke/synthetic

Tests must stop treating manually forced post-run proposals as proof of the autonomous no-human day. Synthetic action tests are still useful as unit tests, but the Phase 3A capstone must assert that the integrated no-human run itself produces the required ancestry.

### D-10 — Status and ledger language overclaims readiness

Status docs and ledgers must not say Phase 3A has landed far enough to unblock Phase 3B or ordinary-life-dependent expansion unless the acceptance gates in this spec pass.

## Binding implementation constraints

1. Every world-affecting proposal from human, scheduler, autonomous actor, or test must still pass through the shared action/proposal/event pipeline.
2. Autonomous decision generation may not read hidden physical truth except through typed actor-known / visible-local inputs.
3. The action validator may read authoritative physical state to accept or reject proposals. The planner and autonomous proposal generator may not use validator-only truth to choose goals, food, routes, workplaces, doors, or containers.
4. No routine execution may dispatch by substring or template-name convention.
5. No candidate goal may become applicable through raw caller self-attestation such as `reason_available`, `route_planner_available`, or unchecked string facts.
6. Debug panels are read-only and non-diegetic. They must not mutate world, agent, or epistemic state.
7. Embodied TUI views must remain actor-filtered and must not reveal hidden food, hidden routes, hidden containers, invisible blockers, or omniscient planner facts.
8. A no-human run that emits only markers, waits, or marker-plus-debug events does not satisfy Phase 3A.
9. Replay equivalence must compare live state and replay-rebuilt projections for the Phase 3A state that matters: needs, routine executions, intention state, no-human metrics, and decision/stuck traces.
10. Tests may use synthetic proposals for action unit coverage, but the capstone acceptance test must not rely on manually injected ordinary actions after `run_no_human_day`.

### Binding invariants from the foundation docs

The constraints above are not new policy; they enforce existing constitutional invariants (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`). Each deliverable must satisfy at least as strictly as:

- **INV-004 — The authoritative world ignores human existence.** The no-human day must run coherently from live actor state, not from a sacred player path; constraints 1 and 8.
- **INV-006 — Possession transfers no world knowledge** and **INV-024 — No telepathy.** Autonomous planning and actor-filtered views may use only actor-known / visible-local inputs; constraints 2, 7, and D-04. Leakage is a high-severity defect (**INV-093**).
- **INV-008 — UI assistance is not authority** and **INV-043 — Action validation is ordinary-agent validation.** Every world-affecting proposal flows through the shared pipeline; the validator may read authoritative truth but the planner/proposal generator may not use validator-only truth as a decision oracle; constraints 1 and 3.
- **INV-017 / INV-018 / INV-019 — seedable/auditable randomness, deterministic replay, ancestry-preserving snapshots.** Live-vs-replay equivalence over Phase 3A needs, intentions, routines, metrics, and traces; constraint 9.
- **INV-032 / INV-034 / INV-035 / INV-036 / INV-037 — symbolic inspectable agents, durable intentions, defeasible routines, HTN methods as procedures, bounded local planning.** Intentions/routines must be live commitments over typed conditions, not substring dispatch or inert structs; D-03, D-05, and the HTN/routine sections.
- **INV-031 / INV-068 — human/debug notes are non-diegetic; debug mode is visibly non-diegetic** and **INV-041 — agent decisions need debug traces.** Debug panels are read-only and derived from typed decision/planner/routine records; constraint 6 and D-07.
- **INV-070 — Why-not explanations are mandatory.** Typed wait/blocker/replan reasons and the debug planner panel expose missing preconditions in actor-known terms.
- **INV-091 / INV-092 / INV-095 — no-human, deterministic-replay, and TUI/view-model tests are acceptance tests.** The capstone, replay/provenance, and TUI/view-model test sections below carry these gates.

## Required code-surface changes by subsystem

### Scheduler and no-human runner

Replace the current no-human proposal builder with a typed actor decision driver.

Required behavior:

- enumerate day windows and actors deterministically;
- before each actor decision, apply passive need deltas through agent events with ancestry from elapsed time or prior action;
- build a canonical actor-known planning context for that actor and tick;
- derive active intention and routine window goal from `AgentState`, routine assignments, current tick, and prior routine events;
- generate candidate goals from live needs, active intention, routine duties, and typed actor-known facts;
- select a candidate deterministically and record the decision trace;
- select an HTN/routine method using typed conditions;
- plan ordinary action proposals using only actor-known planning state;
- submit the selected ordinary proposal through the shared pipeline;
- emit typed failure, replan, wait-with-reason, stuck, or routine-interruption events when no ordinary action can be produced;
- count progress only when the accepted event ancestry includes an ordinary action or a typed failure/replan/stuck event that proves the actor was considered and did not silently freeze.

Forbidden in the no-human decision path:

- selecting sleep/eat/work by `template_id.contains(...)`;
- selecting food directly from `PhysicalState.food_supplies` unless the same food appears in actor-known/visible-local planning inputs;
- selecting workplaces directly from `PhysicalState.workplaces` unless the actor has typed work assignment knowledge appropriate for Phase 3A;
- selecting movement destinations from raw `PlaceState.adjacent_place_ids` unless the edge is actor-known or currently visible;
- constructing an empty `EpistemicProjection` and then declaring hidden-truth safety by convention;
- passing `active_intention: None` when the actor has an active intention in `AgentState`;
- treating a successful `wait` as ordinary-life proof when it masks missing routine/need behavior.

### Needs

Needs must remain bounded, deterministic, replayable, and causally explained.

Required behavior:

- hunger, fatigue, and safety live in `AgentState` for each modeled actor;
- passive need deltas are emitted as agent events with tick/window ancestry;
- action effects emit `NeedDeltaApplied` with action-event ancestry;
- threshold crossings can trigger candidate re-evaluation;
- severe needs can interrupt lower-priority routine intentions through typed intention/routine events;
- debug/TUI can display current value, band, last cause, and last threshold crossing;
- replay rebuild reproduces the same values and last-cause summaries.

Do not encode need state as proposal parameters where the pipeline can instead read live `AgentState` from `PipelineContext`. Proposal parameters may carry stable user input, not hidden live state copied from the scheduler.

### Intentions

Intentions must be live commitments, not inert structs.

Required behavior:

- adopting a routine/need goal emits `IntentionStarted` or equivalent event;
- continuing emits an event linked to the chosen ordinary follow-on action;
- completing, failing, suspending, resuming, abandoning, or interrupting emits typed events with causes;
- possession attach/detach/switch never clears an ordinary actor’s active intention;
- active intention stabilizes selection unless a severe need, blocker, contradiction, or scheduled routine boundary justifies switching;
- debug trace shows selected intention, reason for continuation/switch/interruption, and event ancestry.

### Routines

Routines must be defeasible procedures over ordinary actions.

Required behavior:

- routine templates use typed families, steps, and conditions;
- routine assignments instantiate windows without teleportation;
- routine step start/completion/failure events are emitted by the integrated no-human and TUI paths;
- routine execution reads actor-known locality and typed work/home/food conditions;
- blockers produce typed routine failure or replan diagnostics;
- routine progress is tied to ordinary action ancestry such as movement, sleep, eat, work, search, wait-with-reason, or typed failure;
- routines can be interrupted by severe needs, blocked access, action failure, work-hour boundaries, safety concerns, or relevant epistemic contradictions.

### HTN and local planner

The HTN/planner boundary must become typed enough to support future phases without shortcut debt.

Required behavior:

- replace stringly `actor_known_inputs` with typed actor-known facts or a sealed value object built from actor-visible/provenance-safe sources;
- if stable string IDs are retained for debug/serialization, generate them from typed facts and never parse them as authority inside decision logic;
- remove condition success based on magic caller strings such as `reason_available`, `route_planner_available`, and ad hoc `day_window:*` unless they are generated from typed, validated planner context;
- every method applicability decision records structured proof sources or structured missing-proof blockers;
- planner traces must distinguish validation truth from planning knowledge;
- hidden-truth audit is computed from input provenance and typed proof sources, not hard-coded to `true`.

### Action definitions

Sleep, eat, work, wait, movement, and continue-routine actions remain ordinary actions through the shared pipeline.

Required changes:

- `eat` must be reachable from actor-known/visible food planning, and forced hidden-food proposals must still fail at validation if not physically/evidentially valid;
- `work_block` must validate physical workplace constraints, but autonomous selection must use typed work assignment knowledge and local route knowledge;
- `sleep` must use a typed known/visible/rest-place target where needed, not just actor current place by scheduler convenience;
- `wait` must carry a typed reason, blocker, replan, or routine wait cause;
- `continue_routine` must not count as behavioral progress unless linked in the same decision transaction to an ordinary action or typed failure/replan event.

### TUI, view models, and debug panels

Required behavior:

- embodied views show Phase 3A needs and active intention for the possessed actor;
- actor-filtered views never reveal hidden food, hidden route, hidden workplace access, or other omniscient facts;
- TUI command loop can run/advance no-human segments and inspect resulting no-human metrics;
- debug panels expose no-human metrics, planner trace, active needs, active/suspended intentions, routine execution state, stuck diagnostics, and replay comparison;
- debug panels are read-only and covered by checksum/event-count tests;
- tests must assert behavior-specific rows, not only labels such as `candidate_goals` or `no_human_day_metrics_v1`.

### Replay and provenance

Required behavior:

- replay rebuilds physical state, agent state, epistemic projection where Phase 3A touches it, no-human metrics, routine state, intention state, decision traces, and stuck diagnostics from the event log;
- live-vs-replay checksums must include Phase 3A agent state and no-human metrics;
- no-human day reports must be reproducible from the event log, not from side-channel scheduler state;
- provenance for ordinary actions must link decision trace -> method/routine step -> planned proposal -> validation report -> emitted event -> state change.

### Content and validation

Required behavior:

- routine templates, routine assignments, work windows, known food/rest/work facts, and fixture contracts use typed schema fields;
- validation forbids routine template names whose semantic meaning is only discoverable by substring;
- validation rejects fixture contracts that assert capstone behavior by expected text only;
- fixture contracts must distinguish autonomous no-human events from manually forced action-unit events.

## Required fixtures and tests

### Integrated no-human day capstone

Create or replace a capstone fixture/test that runs `run_no_human_day` exactly once and asserts, without manually injecting post-run ordinary proposals, that the resulting log includes:

- deterministic stable actor order for at least four actors;
- passive need deltas for relevant actors before decisions;
- at least one fatigue-driven sleep action and sleep completion/recovery ancestry;
- at least one hunger-driven eat action from actor-known/visible food;
- at least one hunger-driven food failure or search/replan/wait-with-typed-reason when food is not actor-known or not available;
- movement before remote work;
- a work block started/completed only after locality/access/work assignment validation;
- a work failure or typed blocker when access/workplace conditions fail;
- at least one active intention adopted and later continued, completed, failed, or interrupted;
- routine step start/progress/failure/completion ancestry;
- no `Player`, controller, or protagonist-conditioned event;
- no actor silently freezing merely because unpossessed;
- no hidden food or hidden route used by autonomous planning;
- replay rebuild of physical state, agent state, no-human metrics, intention/routine state, and decision/stuck traces.

### Hidden-truth construction tests

Add tests where hidden physical truth exists but actor-known proof does not:

- hidden food at the actor’s current physical location;
- hidden route edge to a workplace;
- hidden workplace assignment or blocked workplace condition;
- hidden container/search surface.

The autonomous decision generator must not select those facts. The validator may reject forced human/test proposals that target them.

### Continue-routine ancestry tests

Add tests proving:

- `ContinueRoutineProposed` alone has zero behavioral progress and cannot satisfy no-human progress metrics;
- a successful routine continuation transaction includes the follow-on ordinary action or typed failure/replan event;
- replay reconstructs the same intention/routine progress from the event chain.

### TUI/view-model tests

Add tests proving:

- `run no-human-day` through the command loop produces the integrated capstone ancestry;
- embodied view shows actor needs and active intention but not hidden facts;
- debug planner panel exposes the selected goal, rejected goals, method condition proofs, hidden-truth proof sources, and blocker categories for a real no-human decision;
- debug no-human panel is derived from log metrics and remains read-only;
- debug stuck panel shows typed no-progress diagnostics from the integrated run.

### Replay/provenance tests

Add tests proving:

- live-vs-replay physical checksum equality;
- live-vs-replay agent-state checksum equality;
- log-derived no-human metrics byte identity;
- decision trace ancestry survives replay;
- routine/intention state survives replay;
- no-human report can be regenerated from the event log.

## Forbidden shortcuts

The following are not acceptable as Phase 3A completion evidence:

- no-human day emits start/end markers plus waits only;
- `ordinary_pipeline_events > 0` without proving specific ordinary-life ancestry;
- manually forced eat/sleep/move/work proposals after `run_no_human_day` counted as no-human behavior;
- `ContinueRoutineProposed` counted as routine progress without a follow-on ordinary action or typed failure/replan;
- hidden-truth audit hard-coded to true;
- planner safety proven only by a boolean or by caller promises;
- HTN applicability decided by substring or prefix over raw strings;
- fixture `expected_events_or_reports` strings accepted as behavioral proof;
- debug panels that only render labels or seeded rows;
- replay metrics that ignore agent needs, intentions, routines, and stuck/decision traces;
- Phase 4/3B status claims that rely on capstone smoke tests.

## Acceptance criteria

This hardening spec is complete only when all of the following are true in source and tests:

1. The no-human day capstone produces the required ordinary-life chain without manually injected post-run ordinary actions.
2. Needs affect live decisions and are replay-reconstructible with last-cause ancestry.
3. Intentions are adopted, continued, interrupted/completed/failed, and replay-reconstructible.
4. Routines execute as typed defeasible procedures over ordinary actions.
5. HTN and candidate generation no longer rely on substring/prefix/magic caller facts for Phase 3A applicability.
6. Autonomous planning receives a canonical actor-known planning context built from actor-known or visible-local sources.
7. Hidden physical truth can exist in fixtures without autonomous actors using it unless actor-known proof exists.
8. `continue_routine` cannot satisfy progress gates by itself.
9. TUI can run and inspect Phase 3A behavior through actor-filtered views and non-mutating debug panels.
10. Replay rebuilds the meaningful Phase 3A state and metrics from the event log.
11. Tests distinguish unit action coverage, fixture validation, debug rendering, and integrated no-human behavioral proof.
12. Status docs no longer overclaim Phase 3A readiness beyond proven evidence.

### Mapping to Phase 3 execution acceptance gates

These acceptance criteria mirror the named gates in `docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` (execution doctrine labels this "Phase 3"; "Phase 3A" is the spec-package substrate slice). An implementer must prove the doc-07 phase-exit checklist, not only the criteria above:

- **TUI/view-model gate** (doc-07) ← criterion 9 and the TUI/view-model tests.
- **No-human simulation gate** (doc-07) ← criteria 1, 8 and the integrated capstone.
- **Deterministic replay gate** (doc-07) ← criterion 10 and the replay/provenance tests.
- **Test gate** (doc-07) ← criterion 11 (unit / fixture / debug / integrated proof distinguished).
- **Data/fixture gate** (doc-07) ← the content-and-validation section and the required fixtures.
- **Debug/inspection gate** (doc-07) ← criteria 2–8 and the debug planner / no-human / stuck panels.
- **Phase exit checklist** (doc-07) ← the full acceptance-criteria set plus the forbidden-shortcuts list.

## Explicit phase-exit impact

This spec has passed. Phase 3A's no-human ordinary-life substrate is now evidenced by the Spec 0007 gates recorded in `docs/4-specs/SPEC_LEDGER.md`.

This does not close full Phase 3. Minimal social speech/testimony and broader autonomous missing-property story setup remain Phase 3B work, and Phase 4 entry remains governed by the execution ladder and higher-authority doctrine.

Later work that depends on Phase 3A ordinary-life readiness should cite Spec 0007 rather than Spec 0005 or Spec 0006 alone.

## Documentation/status updates required

After implementation and acceptance evidence:

- update `docs/4-specs/SPEC_LEDGER.md` to record this second hardening spec and its result;
- update `docs/4-specs/README.md` only if its 0006 wording still implies readiness beyond evidence;
- add a narrow status errata/report if any archived completion claim remains misleading;
- do not rewrite foundation, architecture, execution, or reference doctrine unless a separate doctrine contradiction is discovered.

Ledger/status language records that Spec 0006 landed useful hardening but did not fully prove Phase 3A integrated ordinary-life readiness at its audit point, and that Spec 0007 is the follow-up evidence.

## Static-audit limitations

This spec is based on exact-commit static source inspection. The test suite was not run. It does not prove runtime failures; it identifies source-level architectural and evidentiary gaps that must be closed before the Phase 3A substrate is safe to build on.

## Outcome

Completed: 2026-06-07

Changed behavior:
- Tickets `0007PHA3ASECHAR-001` through `0007PHA3ASECHAR-012` closed the typed actor-known planning, routine dispatch, candidate generation, no-human proposal routing, live need, intention, routine ancestry, continue-progress, TUI/debug, replay checksum, content validation, and integrated capstone gaps described here.
- Ticket `0007PHA3ASECHAR-013` recorded the final status in `docs/4-specs/SPEC_LEDGER.md` and `docs/4-specs/README.md`.

Deviations:
- The integrated capstone exposed missing in-run duration completion and decision-trace emission; those were completed in the final capstone ticket so the no-human run itself produces the proof instead of relying on post-run manual proposals.

Verification:
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
