# Spec-0047 foundational-conformance hardening — second-pass research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `8d7c119d94e5438244b6cb1e8e57428c26cb233e`  
**Assessment type:** cross-cutting foundational-conformance and anti-regression re-audit  
**Freshness scope:** user-supplied target commit only; this report does **not** independently verify that the commit is the current `main`  
**Execution scope:** static source survey only; no command in this report was executed by the research session

## 1. Verdict

**Not conformant.** The current spec-0047 surface at `8d7c119d94e5438244b6cb1e8e57428c26cb233e` is materially better than the pre-remediation `cb3102e` baseline, but it does not yet satisfy the governing foundation, architecture, execution, and reference contracts.

The 0048 line appears, by static reading, to have corrected several serious predecessor defects: the one-tick coordinator stages work on scratch state and commits at the end; the scheduler frontier is no longer a general public write surface; a temporal replay projector validates `TimeAdvanced` ancestry and rebuilds the frontier; actor-known interval products are typed and positively constructed; duration terminals are derived from the log; passive need charging and body-exclusive reservation have substantive guards. The narrow 0049 mutation witnesses also appear structurally capable of killing the exact predicate and accessor mutants they name.

Those correct holdings are not enough. Four live foundational violations remain:

1. production world-step callers do not discover loaded-world due work; they pass empty actor and process collections while tests manually inject them;
2. the coordinator accepts caller-authored raw process events instead of invoking declared processes through an owned registry/cadence and ordinary causal boundary;
3. the coordinator invokes the actor decision transaction but discards its trace, lifecycle, plan, stuck, and pipeline-result products, while a separate no-human choreography still records a subset afterward;
4. the TUI still appends and projects perception events and reconstructs interval products after core completion, including a path that can append duplicate causal identities.

A fifth violation is in replay evidence: `ReplayReport::matches_expected` omits temporal violations, so the report can claim success while its temporal projector reports divergence. Two additional hardening gaps remain: the salient-stop policy and witness are too broad to prove useful novelty-sensitive stopping, and the 0048 behavioral-lock evidence overstates production reachability even though the narrow 0049 mutant witnesses are structurally non-vacuous. Finally, the `0049MUTWIT` implementation line has the required source-discipline defect: it exists only as archived tickets and code/tests, with no spec package and no `SPEC_LEDGER.md` record.

No higher-tier doctrinal amendment is warranted. The governing doctrine is coherent and already forbids the remaining implementation and evidence shapes. The required work is code, witness, conformance-status, risk-evidence, and source-record repair—not foundation text weakened to fit current code.

All test, build, lint, replay, parity, golden, and mutation outcomes in existing repository artifacts are historical claims made by those artifacts. This session did not run `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, replay/golden lanes, CI, or `cargo-mutants`. Every current pass/fail statement below is therefore a **preliminary static judgment about source shape and witness intent**, not an authoritative command result.

## 2. Disposition table

| ID | Finding | Primary target | Classification | Basis |
|---|---|---|---|---|
| F-01 | Production world steps do not derive loaded actors or due world processes | `crates/tracewake-core/src/scheduler.rs`; `crates/tracewake-tui/src/app.rs` | **Violation — critical** | INV-004, INV-005, INV-087, INV-088, INV-091, INV-094, INV-098, INV-108, plus the shared loaded-world transition doctrine require possession-neutral, no-human-capable advancement; production callers submit empty work while tests inject it. |
| F-02 | Public request API admits raw caller-authored process events | `crates/tracewake-core/src/scheduler.rs` | **Violation — critical** | INV-001, INV-009, INV-010, INV-088 and architecture `04` require declared causal processes and ordinary proposal/event commitment; an arbitrary `Vec<EventEnvelope>` is appended and applied directly. |
| F-03 | Actor decision transaction results are only partially consumed and two choreographies remain | `scheduler.rs`; `agent/transaction.rs` | **Violation — critical** | INV-041, INV-103, INV-104, INV-105 and the canonical cognition transaction require typed traces, lifecycle, local plan, stuck diagnostics, shared validation, and resulting events to survive one authoritative transaction. |
| F-04 | The TUI still mutates epistemic/log state and rebuilds interval output after core completion | `crates/tracewake-tui/src/app.rs`; `agent/perception.rs`; `events/log.rs` | **Violation — critical** | INV-009, INV-018, INV-067, INV-069, INV-102, INV-112 and architecture `10` deny simulation/event/projection authority to the client. The final-tick path can append the same deterministic perception event ID twice. |
| F-05 | Replay aggregate success is fail-open with respect to temporal divergence | `crates/tracewake-core/src/replay/report.rs` | **Violation — high** | INV-018, INV-092, INV-112 and replay/evidence doctrine require temporal ancestry failure to make replay fail, not coexist with `matches_expected = true`. |
| F-06 | Salient-stop semantics and the prominent production-path witness do not establish novelty-sensitive stopping | `scheduler.rs`; `tests/salient_stop_actor_known.rs` | **Hardening gap — high** | Holder-known stopping is typed, but every Observation/Record/Belief notice is treated as salient; routine repeated local-actor perception can satisfy the branch. R-27/R-29 and execution `10` require a real, discriminating path-under-test witness. |
| F-07 | Narrow 0049 witnesses are structurally non-vacuous, but broader 0048 behavioral evidence overstates production reachability | core/TUI test seams; architecture conformance and risk evidence rows | **Hardening gap — high** | The mutation witnesses assert mutant-sensitive boundaries in real functions, but loaded-world tests manufacture due work and the salient witness is semantically weak. Historical green claims do not prove the present exact commit. |
| F-08 | `0049MUTWIT` has no spec package and no `SPEC_LEDGER.md` entry | `docs/4-specs/SPEC_LEDGER.md`; owner/reassess source record | **Provenance gap — high** | The ledger is the repository’s source-discipline/navigation authority. Mutation-witness code and tickets landed without the record that explains their authority, scope, and survivor floor. |

## 3. Method and provenance ledger

### 3.1 Authority order

The audit applied the repository’s stated authority order without inversion:

```text
0-foundation
  -> 1-architecture
  -> 2-execution
  -> 3-reference
  -> 4-specs
  -> archived specs/tickets/acceptance as historical promises and evidence
  -> live code and tests as the implementation under review
```

`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` remained the controlling product contract. In particular, this report measured the surface against causality/event sourcing (INV-001, INV-009, INV-010, INV-018), human/no-human and possession neutrality (INV-004, INV-005, INV-006, INV-087, INV-091, INV-094, INV-108), TUI boundaries (INV-067, INV-069), declared processes (INV-088), harsh feature acceptance (INV-098), the truth firewall and cognition transaction (INV-101 through INV-105), and temporal authority (INV-112).

The first-pass report at `cb3102e` was used only as the **pre-remediation baseline**. None of its seven findings was carried forward merely because it had once been true. Each relevant seam was re-read at `8d7c119`; corrected properties are recorded as corrected, remaining defects are independently grounded in live code, and newly exposed defects are reported separately.

### 3.2 Exact-commit acquisition provenance

Repository files were located only through the user-supplied manifest and fetched only through full raw URLs containing the exact owner, repository, full commit SHA, and exact manifest path. No clone, branch fetch, default-branch lookup, repository metadata lookup, target-repository code search, or snippet reconstruction was used. References to other repositories inside validly fetched content were treated as ordinary content, not contamination.

```text
Requested repository: joeloverbeck/tracewake
Target commit: 8d7c119d94e5438244b6cb1e8e57428c26cb233e
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw URL)
Intermediate pre-analysis acquisition: 77 requested / 77 successfully verified
Final consolidated unique exact URLs: 91
Successfully verified unique file count: 91
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

The complete URL inventory is supplied beside this report as [`0047-foundational-hardening-second-pass-exact-commit-ledger.txt`](../0047-foundational-hardening-second-pass-exact-commit-ledger.txt).

### 3.3 Evidence posture

The survey accounted for existing machinery instead of proposing duplicates:

- `.cargo/mutants.toml` already includes the relevant scheduler, replay, projection, pipeline, and TUI/core seams;
- `anti_regression_guards.rs` already provides source/topology alarm infrastructure;
- `negative_fixture_runner.rs` and the external-crate negative-fixture corpus already provide compile-fail boundary enforcement;
- `generative_lock.rs` already supplies a deterministic generated corpus and ratcheted witness style;
- the 0049 tickets already add narrow mutation witnesses for the eight 0048 survivors and adjacent same-function mutants.

Accordingly, the report prefers compile-time unrepresentability first, production-path behavior witnesses second, focused mutation kills third, and `include_str!`/source scans only as topology alarms. It does not recommend `proptest` or `quickcheck`: the existing deterministic generative harness can express the required schedule, paired-world, prefix-replay, and boundary properties.

### 3.4 Static-survey limitation

No executable verification was performed. Static reading can establish that a witness calls a production function and makes a mutant-sensitive assertion; it cannot establish that the current workspace compiles, that the test runs in CI, that a filtered mutation denominator is unchanged, or that commands currently pass. Every remediation section therefore names the authoritative executable closure required from the implementing session.

## 4. Findings

## F-01 — Production world steps do not derive loaded-world due work

### Foundational driver

The authoritative world must run coherently without a human (INV-004, INV-091); possession changes input only (INV-005, INV-094, INV-108); human focus cannot make the world wait (INV-087); declared regional/world processes must have cadence, trigger, ancestry, and replay visibility (INV-088); and a runnable feature must be no-human runnable, replay-safe, TUI-reachable, and regression-tested (INV-098). Foundation `08` states that waiting runs the simulation in one shared transition: the human consumes one input slot while other loaded actors, world processes, and due consequences also advance. Architecture `04` assigns eligible actor/process selection to the scheduler and architecture `10` requires the TUI to ask core for a typed world step rather than assemble the step itself.

### Current `8d7c119` code state

`WorldStepTransactionRequest` publicly contains both `due_actor_ids: Vec<ActorId>` and `world_process_events: Vec<EventEnvelope>` ([`scheduler.rs`, request definition](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L224-L231)). The coordinator sorts and iterates the supplied actor IDs; it does not derive them from the loaded world ([actor phase](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L663-L760)).

Every production call path inspected supplies empty collections:

- TUI authoritative action/wait path: `due_actor_ids: Vec::new()` and `world_process_events: Vec::new()` ([`app.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-tui/src/app.rs#L283-L305));
- core `advance_until`: the same two empty collections on every tick ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L850-L870));
- the no-human tick path used to advance the frontier: again both empty ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L2243-L2256)).

By contrast, the prominent “loaded-world differential” test constructs `due_actor_ids` and `world_process_events` in the harness and passes them into the request ([`world_step_coordinator.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/tests/world_step_coordinator.rs#L1036-L1050)). Its non-vacuity assertions prove that the coordinator processes supplied work and that deleting either supplied element changes the test result. They do **not** prove that a production entry point discovers or schedules that work.

This is not the predecessor’s old “zero actor/process phases” finding carried forward. The phases now exist. The live defect is narrower and more fundamental: **the caller remains the source of the phase population, and production callers choose an empty population.**

### Conformance verdict

**Violation.** The current canonical step is a transaction over caller-enumerated work, not a loaded-world step. Human wait, core interval continuation, and the no-human frontier path cannot, through this API usage, advance an unpossessed actor or a declared world process. The differential evidence is therefore a harness-level composition proof, not production reachability proof.

### Required remediation

**Code home:** `crates/tracewake-core/src/scheduler.rs`, the loaded-world state/registry owner, and the narrow call sites in `tracewake-tui/src/app.rs` and the no-human scheduler module.

Replace the public request shape so the caller supplies only controlled input and authority information—expected frontier, origin/controller binding, content/ruleset identity, and an optional ordinary controlled proposal. Core must derive, in deterministic order:

1. due duration completions from the event log;
2. due declared world-process invocations from an owned registry/cadence/trigger surface;
3. eligible loaded actors from authoritative scheduling state, then invoke their sealed actor-known decision transactions;
4. the possessed actor’s controlled proposal in the agreed phase;
5. accounting, observations, projections, interval delta, and one commit/rollback decision.

Human wait, `advance_until`, and no-human advancement must delegate to that same one-tick operation. Delete the caller-injected actor/process collections rather than retaining aliases or compatibility wrappers.

**Documentation home:** truth the loaded-world/time-control rows in `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`; align implementation-specific choreography in `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` and the human/no-human evidence claim in execution `06`. The higher-tier rule itself needs no amendment.

### Strongest practical anti-regression guard

1. **Compile-time:** make due-actor and due-process derivation private to core. Add an external-crate negative fixture proving callers cannot inject or omit those populations.
2. **Behavior:** run the real production TUI wait path and the real no-human path from identical loaded state containing an unpossessed due actor and a due declared process; compare final physical/agent/epistemic state, ordered event ancestry, reconstructed frontier, and due-work metrics. The harness must configure world state/registry only—not pass the due work directly.
3. **Mutation:** add mutants or focused kills around actor/process eligibility and cadence boundaries, especially “skip actor”, “skip process”, “wrong due comparison”, and “possessed-origin changes population.”
4. **Topology alarm:** retain a source guard only to ban reintroduction of public `due_actor_ids`/raw `world_process_events`; never cite the scan as behavioral proof.

### Evidence-honesty check

The implementing session must demonstrate that the witness fails when production discovery is disabled while the test fixture remains unchanged. It must report nonzero measured actor and process work from the production entry points and must not obtain those counts from request fields authored by the test.

## F-02 — Raw process event injection bypasses declared process authority

### Foundational driver

INV-001 and INV-010 require modeled causes; INV-009 requires meaningful mutation to be eventful; INV-088 requires regional/world processes to have a declared source, cadence or trigger, inputs, random model, scope, delivery channel, traces, ancestry, and replay/debug visibility. Architecture `04` requires world-affecting work to enter through ordinary proposal/validation or an equivalent owned process transaction; execution `05` forbids direct dispatch and makes the core coordinator the owner of due consequences.

### Current `8d7c119` code state

The request exposes `world_process_events: Vec<EventEnvelope>`. Inside the transaction, the coordinator iterates those arbitrary envelopes, appends each to the scratch log, and applies the event stream directly ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L564-L580)). The API does not identify a process registry entry, due cadence, trigger evaluation, process-local input packet, typed invocation, or process-owned proposal. It trusts the caller to provide the already-authored authoritative event.

The current production callers pass an empty vector, so this direct-injection path is mostly visible in tests. That does not make it harmless. Once F-01 is corrected, this shape would still let the caller decide not merely **which process is due** but **which authoritative event already happened**.

### Conformance verdict

**Violation.** An `EventEnvelope` is the committed causal record, not a public scheduling request. Supplying final envelopes through the world-step request collapses process trigger, decision, validation, event construction, and ancestry into caller authority.

### Required remediation

**Code home:** `scheduler.rs`, a process registry/cadence module owned by core, and process-specific action/event builders.

Replace raw envelopes with a private typed `DueProcessInvocation` or equivalent, created only by core after evaluating a declared process registry. The invocation must name the process, trigger/cadence witness, effective tick, source event IDs, ruleset/content version, and deterministic random provenance if any. It should then either:

- produce an ordinary process-origin proposal routed through the shared pipeline; or
- call a narrowly typed process transition whose event builder is owned by the declared process and whose result is committed within the same scratch transaction.

Do not preserve a public escape hatch for arbitrary envelopes. Test fixtures that need synthetic events should use explicit test-only constructors or initial authored history, not the production world-step request.

**Documentation home:** implementation details in architecture `04`, relevant process ownership in architecture `12`, and execution `05`; conformance status in architecture `00`. No new invariant or glossary term is needed.

### Strongest practical anti-regression guard

- **Compile-time:** private invocation constructors and no public request field accepting `EventEnvelope`; external-crate negative fixture that attempts raw process injection and fails to compile.
- **Behavior:** a declared process with a known cadence becomes due from state/log, emits one causally linked event through the real coordinator, is absent one tick earlier, and replays identically.
- **Mutation:** kill cadence boundary inversions, skipped trigger witness, wrong process ID/source ancestry, and direct-apply bypasses.

### Evidence-honesty check

The witness must create only registry/state inputs. If it calls an event builder in the test or inserts the final process event into the request, it is reproducing the current defect and does not count.

## F-03 — The actor decision transaction is not consumed as one authoritative transaction

### Foundational driver

INV-099 establishes that truth may validate actions but may not plan them. Foundation `14` requires a canonical actor decision transaction that selects an actor and trigger, builds a sealed actor-known context, gathers pressures and commitments, generates candidates, selects method/intention, performs bounded local planning, constructs a proposal with ancestry, validates through the shared pipeline, and records typed trace/stuck/lifecycle outputs. INV-041 requires decision debug traces; INV-103 denies cognition authority to the scheduler; INV-104 forbids needs/routines from bypassing candidate generation and planning; INV-105 makes decision traces, stuck diagnostics, hidden-truth audits, lifecycle changes, and planner outputs authoritative diagnostic data.

Architecture `05` and execution `05`/`06` require the scheduler to invoke—not partially imitate—the actor transaction, and execution `10` requires typed artifacts before rendered or declarative evidence.

### Current `8d7c119` code state

`ActorDecisionTransactionOutcome::Proposed` carries the sealed proposal, decision trace, `DecisionTraceRecord`, lifecycle effects, and local plan; `Stuck` carries a typed diagnostic ([`agent/transaction.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/transaction.rs#L27-L43), [`Proposed` construction](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/transaction.rs#L369-L375)).

The world-step coordinator destructures only the `Proposed` variant, uses only `proposed.proposal`, silently `continue`s on `Stuck`, and ignores the returned `PipelineResult` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L725-L760)). The trace, trace record, lifecycle effects, and local plan are dropped before commit; rejection/failure feedback from `run_pipeline` is not turned into the transaction’s modeled lifecycle or diagnostic outcome.

A separate no-human choreography remains. `build_agent_proposal` extracts only the proposal and `DecisionTraceRecord`, returns `None` on `Stuck`, and discards the other products ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L2040-L2105)). After pipeline execution, the no-human loop separately appends intention lifecycle, routine-step, decision-trace, and stuck-diagnostic events ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L1895-L1945)). That path is not the same transaction as the loaded-world actor phase, and its stuck handling is assembled from other scheduler state after the actor transaction’s own typed `Stuck` value was discarded.

### Conformance verdict

**Violation.** The code invokes the correct cognition boundary but does not commit its full typed result. This makes the transaction partly decorative and preserves two competing actor-transition choreographies. It also weakens atomicity: the scratch world step is atomic for the operations it owns, while no-human trace/lifecycle recording remains a later, separate sequence.

### Required remediation

**Code home:** `agent/transaction.rs`, `scheduler.rs`, and the no-human module inside `scheduler.rs`.

Define a closed internal actor-step outcome that the coordinator must exhaustively handle. At minimum it must preserve:

- selected proposal and all proposal ancestry;
- typed decision trace and trace record;
- local plan identity/content needed for replay/debug;
- intention/routine lifecycle effects;
- `StuckDiagnosticRecord` when no proposal is available;
- pipeline accepted/rejected/failed result and actor-legible modeled feedback;
- resulting event IDs linking decision, proposal, validation, ordinary events, lifecycle, and diagnostics.

Commit those products inside the same scratch transaction and expose a typed summary in `WorldAdvanceResult`. Make the no-human day/window machinery schedule actor opportunities through the same coordinator and delete the parallel post-proposal append choreography. Do not keep wrappers that perpetuate both paths.

**Documentation home:** architecture `05`, execution `05`, execution `06`, execution `10`, architecture conformance `00`, and evidence/status notes under existing R-27/R-29. The foundation already states the correct transaction and should not be amended.

### Strongest practical anti-regression guard

1. **Compile-time/exhaustiveness:** a non-exhaustive-to-callers, closed internal outcome enum whose variants cannot be reduced to `Option<Proposal>`; required artifact fields should be private and only constructible through the transaction.
2. **Behavior:** a real one-tick actor run that produces an ordinary action and proves trace → proposal → validation → event ancestry; a second run that produces `Stuck` and proves the typed diagnostic is committed with no primitive action; a rejection run that proves actor-legible feedback/lifecycle without validator-truth leakage.
3. **Differential:** the same actor opportunity reached through possessed wait and no-human scheduling yields the same actor transaction artifacts apart from controller-origin metadata.
4. **Mutation:** kill omission of each artifact class, `Stuck => continue`, ignored pipeline status, and post-commit lifecycle append.

### Evidence-honesty check

A test that calls `ActorDecisionTransaction::run` directly proves the transaction in isolation but not coordinator integration. Closure requires assertions over the event log and returned world-step result produced by the real scheduler entry point, plus a negative proving the test fails if any artifact append is removed.

## F-04 — The TUI remains a simulation/projection writer after core completion

### Foundational driver

INV-069 is explicit: the TUI consumes actor-filtered view models and submits typed attempts; it must not implement simulation rules or mutate the world through player-only paths. INV-009 and INV-018 require meaningful epistemic/log changes to be eventful and replay-safe; INV-006 forbids possession from granting world knowledge; INV-067 requires embodied output to remain actor-known; INV-101 seals actor-known context; INV-102 requires cognition inputs to retain source provenance; INV-112 keeps authoritative time and holder-known temporal interpretation separate. Architecture `03` requires interval summaries to be positively constructed holder-known frontier deltas, and architecture `10` makes actor-known summary and debug report separate core products while denying event application, local clocks, and duration ownership to the TUI.

### Current `8d7c119` code state

The core one-tick transaction already records current-place perception for the interval actor at the resulting tick, builds the post-step context, and derives an `ActorKnownIntervalDelta` before committing scratch state ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L762-L790)). `advance_until` uses that delta to decide whether to stop ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L850-L897)).

After core returns, the TUI nevertheless calls `record_current_place_perception_and_project` again at the scheduler’s current tick, rebuilds an “after” knowledge context, and calls `actor_known_interval_delta` itself to create the displayed interval summary ([`app.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-tui/src/app.rs#L356-L395)). It also appends perception after any accepted command ([`app.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-tui/src/app.rs#L333-L347)) and at other bind/debug/no-human restoration seams.

The perception helper is a writer: it creates events, appends them to `EventLog`, and applies them to `EpistemicProjection` ([`agent/perception.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/perception.rs#L20-L50)). Perception event identity is deterministic over actor, tick, perceived kind, and target; the function’s `index` contributes to the ordering key but not to the `EventId` ([`agent/perception.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/perception.rs#L640-L664)). `EventLog::append` assigns positions and pushes the event without rejecting an existing `EventId` ([`events/log.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/events/log.rs)).

For `advance_until`, the final core tick and the immediate TUI call use the same actor, tick, visible kind, and target. When the visible set is unchanged, the TUI can append an event with the same causal ID that core just appended. Even without that duplicate, the architectural defect is already complete: the client owns event creation, projection mutation, and final interval reconstruction.

### Conformance verdict

**Violation.** The 0048 move toward a core-owned holder-known delta is incomplete at the production client boundary. The authoritative interval product is split between core and TUI, and the TUI remains an epistemic event writer. The duplicate-ID possibility additionally breaks the expectation that causal identities uniquely name events and makes ordered replay/history ambiguous even though global positions remain monotonic.

### Required remediation

**Code home:** `scheduler.rs`, `agent/perception.rs`, `events/log.rs`, `projections.rs`/`view_models.rs`, and `tracewake-tui/src/app.rs`.

- Make the core step/interval result contain the final typed actor-known interval summary or all closed typed data needed to render it without another projection query or event append.
- Generate final perception, interval stop, and resume-boundary evidence inside the scratch transaction. The TUI should store/render the returned typed product only.
- Route ordinary accepted-action perception through a modeled core transition or explicit core-owned perception operation; remove direct TUI calls to append/project helpers.
- Make duplicate `EventId` rejection a fail-closed `EventLogError` for both live append and deserialization. If repeated observations are semantically distinct events, include a deterministic causal sequence/parent identity in the ID rather than permitting duplicates.
- Keep debug inspection read-only and prevent any debug-to-embodied interval conversion.

**Documentation home:** architecture `03` and `10`, execution `04`/`07`/`10`, and the conformance index. The existing foundation already denies the TUI this authority.

### Strongest practical anti-regression guard

1. **Compile-time:** do not export event-appending perception helpers to the TUI crate; expose read-only typed view/summary products. Extend the external-crate negative-fixture pattern so a client cannot construct or convert a debug report into an embodied interval summary.
2. **Behavior:** record log length and projection checksum before consuming the returned world-step/interval result in the TUI; rendering and summary storage must not change either. Assert exactly one event per `EventId` after multi-tick intervals.
3. **Replay:** rebuild from the resulting log and compare physical, agent, epistemic, and temporal frontiers; no client-side repair step may be needed.
4. **Mutation:** kill removal of duplicate-ID checks, reintroduction of post-step append, and substitution of debug/raw context for the sealed holder-known result.

### Evidence-honesty check

A source scan banning the helper name in `app.rs` is only a topology alarm. The decisive witness must execute the ordinary TUI command/advance path and prove that consuming/rendering the core result is read-only. It must also fail when the core’s final perception append is removed or when the client appends a second copy.

## F-05 — Replay aggregate success ignores temporal divergence

### Foundational driver

INV-018 makes deterministic replay foundational; INV-092 requires automated replay/seed verification; INV-112 requires every accepted step, including empty steps, to carry enough ancestry to rebuild the temporal frontier. Foundation `03`, architecture `02`, and execution `10` all require replay and evidence to fail closed when causal or temporal reconstruction diverges.

### Current `8d7c119` code state

`replay/temporal.rs` now contains a dedicated temporal projector and validator, and `rebuild_projection` reports `temporal_violations` plus a reconstructed final frontier. That is a substantive correction to the predecessor baseline.

However, `run_replay` computes `matches_expected` from checksum matches, state diff, unsupported versions, invariant/application errors, and decision-context hash failures while omitting `rebuild.temporal_violations.is_empty()`. The same report then exposes the nonempty temporal violations in a separate field ([`replay/report.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/replay/report.rs)). A consumer that trusts the aggregate boolean can therefore receive `matches_expected == true` alongside a temporal divergence.

The existing 0049 replay witnesses focus on boundary predicates in `validate_time_advanced`, marker-branch selection in `rebuild_projection`, and duplicate need-tick detection. Those are valuable, but none closes this report-level aggregation omission.

### Conformance verdict

**Violation.** The temporal projector can detect the right error while the repository’s high-level replay verdict still reports success. That is fail-open evidence and makes INV-112 enforcement optional for report consumers.

### Required remediation

**Code home:** `crates/tracewake-core/src/replay/report.rs`, with integration witnesses in `tests/replay_temporal_frontier.rs` or a report-focused sibling.

Include `rebuild.temporal_violations.is_empty()` in `matches_expected`. Extend first-divergence reporting with a typed temporal family/detail so a temporal failure does not appear only as an auxiliary list. Any acceptance/golden/replay consumer that gates on `matches_expected` should thereby fail without needing bespoke secondary checks.

**Documentation home:** architecture `02`, execution `09`/`10`, architecture conformance `00`, and existing R-08/R-10/R-29 evidence status as applicable. Do not revise the temporal doctrine.

### Strongest practical anti-regression guard

- **Behavior integration:** construct a log whose physical/agent/checksum result otherwise matches the expected result but whose `TimeAdvanced` ancestry is invalid; assert `temporal_violations` is nonempty and `matches_expected` is false.
- **Mutation:** kill omission/inversion of the temporal-empty conjunct and the typed temporal first-divergence route.
- **Consumer check:** at least one normal replay/golden lane must rely on the aggregate verdict, proving the correction reaches the product evidence boundary.

### Evidence-honesty check

A unit test of `validate_time_advanced` is not enough. The witness must call `run_replay`, preserve all non-temporal success conditions, and fail solely because temporal reconstruction is invalid.

## F-06 — Salient-stop semantics are too broad for the claimed embodied behavior

### Foundational driver

The staged time-control doctrine permits stop-on-salient behavior only through actor-known, provenance-bearing change. INV-067 protects embodied actor-known reality; INV-102 requires source-bearing cognition; INV-112 separates authoritative interval order from holder-known temporal meaning. Architecture `03` requires positively constructed holder-known frontier deltas. Execution `10`, R-27, and R-29 require witnesses to prove the real branch under meaningful discriminating conditions rather than merely execute a symbol.

### Current `8d7c119` code state

The stop branch is now production-reachable: `advance_until` inspects the core-produced actor-known delta. That repairs the predecessor’s unreachable-branch defect.

The policy itself is nevertheless extremely broad. `actor_known_interval_delta_is_salient` returns true when **any** notice has kind `Observation`, `Record`, or `Belief` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs#L932-L940)). Current-place perception emits a `visible_actor` observation for every other actor co-located at the tick ([`agent/perception.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/perception.rs#L168-L183)); that projection record is classified as an `Observation` source ([`agent/perception.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/perception.rs#L517-L535)). Re-observing an unchanged co-located actor can therefore satisfy the salient-kind predicate.

The prominent salient-stop test uses co-location and additional injected process activity. By static shape, it proves that a typed actor-known notice can stop `advance_until`; it does not cleanly prove that quiet unchanged ticks continue and that a **novel, modeled, holder-visible** event at a later tick is what stops the interval. The current rule may be an intentional “stop on any observation” product policy, but the docs and test names imply salience/novelty rather than routine re-observation.

### Conformance verdict

**Hardening gap, not a settled constitutional violation.** Typed holder filtering and source-bearing construction are correct. The unresolved issue is product semantics and evidence precision: the current rule can make continuation stop on routine observational churn, and the witness does not distinguish that behavior from meaningful salience.

### Required remediation

**Code home:** the closed interval notice/stop policy in `scheduler.rs` and actor-known delta construction in `epistemics/projection.rs`/`projections.rs`.

Owner/reassess should choose, within existing doctrine, a closed typed salience policy. Strong options include:

- notice novelty relative to the before context;
- contradiction, material confidence/stance change, newly accessible record, newly perceived entity/state change, or possessed duration terminal;
- an explicit typed `IntervalSalience` classification produced during holder-known delta construction, not inferred from rendered prose or broad notice category.

Do not introduce a debug/raw-world salience oracle. Keep stop reasons closed and typed.

**Documentation home:** architecture `03`/`10` and execution `07`/`10` should state the selected implementation semantics; architecture conformance and existing R-27/R-29 evidence rows should name the discriminating witnesses. No new invariant, gate, risk ID, or glossary term is needed.

### Strongest practical anti-regression guard

Use production `advance_until` with four paired cases:

1. unchanged actor-known state for `N` ticks reaches `ControllerSafetyBound`;
2. a novel modeled holder-visible source becomes due at tick `k > 1` and stops exactly at `k`;
3. an otherwise identical hidden/non-holder-visible source does not stop;
4. replay of the same log reconstructs the same typed stop reason and interval summary.

Then run focused mutation over the holder, source, novelty/frontier, notice-kind, and stop-branch predicates.

### Evidence-honesty check

The novel event must arise from the production process/actor discovery fixed in F-01/F-02. Manually inserting a notice, raw event, or due-work vector would only prove the lower-level delta classifier. The unchanged case must contain at least one routine perception opportunity so that “no notices at all” cannot make the test vacuously pass.

## F-07 — The 0049 witnesses are credible, but the broader 0048 evidence is not production-complete

### Foundational driver

Execution `10` requires typed-before-rendered evidence, real path-under-test witnesses, and anti-vacuity. R-27 warns against reachability overstatement; R-28 requires defect-family completeness; R-29 rejects decorative locks. INV-092/INV-098 require regression evidence to measure the significant causal scenario, not a declaration or test-only substitute.

### Current `8d7c119` code and evidence state

The 0048 acceptance records a focused mutation denominator of 61: 40 caught, eight missed, and 13 unviable. It explicitly says the temporal survivors were not classified equivalent and that a future pass should run a broader seam or add a narrower witness. The three 0049 tickets then add focused scheduler, interval, replay/rebuild, and pipeline witnesses and record zero misses in their respective filtered campaigns.

Static re-verification supports a split verdict:

**Narrow 0049 witnesses — structurally non-vacuous.** The named tests assert exact values or branch boundaries in the real protected functions:

- exact event selection, strict body-exclusive completion boundary, decision-trace append precondition, and human rejection rollback for scheduler survivors;
- one-axis holder mismatch, debug-context rejection, equal/regressing frontiers, and exact `source_key()` identity for interval survivors;
- split duplicate temporal conditions, equal-tick classification, marker/no-marker rebuild selection, and same-tick need-candidate matching for replay/pipeline survivors.

These assertions are shaped to distinguish the named mutants rather than merely assert symbol presence. The ticket outcomes report 58 scheduler mutants (53 caught, five unviable), 15 interval mutants (14 caught, one unviable), and 36 replay/pipeline mutants (35 caught, one unviable), all with zero misses. This session did not rerun those commands, so the counts are historical records, not fresh command evidence.

**Broader 0048 behavioral locks — production reachability overstated.** The loaded-world differential manually populates the exact collections production leaves empty (F-01). It is non-vacuous for coordinator consumption but vacuous for discovery/reachability. The salient-stop witness reaches the branch but does not discriminate routine re-observation from novel salience (F-06). The replay temporal tests validate the projector but do not make the aggregate report fail on temporal violations (F-05). Source guards and parity metadata remain useful topology/census alarms but cannot repair those missing behavior paths.

The architecture conformance index and risk-register 0048 evidence notes currently cite these witness classes as if the full production posture were closed. Because the live code contradicts that breadth, those rows must be treated as stale/overbroad status evidence rather than current conformance proof.

### Conformance verdict

**Hardening gap.** The exact eight-survivor follow-up is not decorative by source shape, but the 0048 claim bundle is broader than what its tests establish. The repository should preserve the narrow 0049 witnesses and replace—not merely supplement—the reachability-overstating locks.

### Required remediation

**Code/test home:** extend the existing `world_step_coordinator.rs`, `salient_stop_actor_known.rs`, `replay_temporal_frontier.rs`, TUI seam/parity tests, negative-fixture corpus, and deterministic `generative_lock.rs`. Do not create a parallel test framework.

**Documentation home:** after implementation and fresh execution, truth the affected rows in:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`;
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`, using existing R-27, R-28, and R-29 only;
- relevant execution `10` evidence maps.

Do not edit the historical 0048 acceptance or archived 0049 tickets. They accurately record what their sessions claimed at the time and are not current-state proof.

### Strongest practical anti-regression guard

After F-01 through F-06 are corrected, the implementing session should run at least:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked

cargo test -p tracewake-core \
  --test world_step_coordinator \
  --test replay_temporal_frontier \
  --test holder_known_interval_projection \
  --test salient_stop_actor_known \
  --test reservation_body_exclusive_census \
  --test generative_lock

cargo test -p tracewake-tui \
  --test playable_capability_parity \
  --test parity_adversarial \
  --test tui_seam_conformance \
  --test command_loop_session \
  --test embodied_flow
```

Then rerun the three 0049 focused mutation commands exactly as recorded by their tickets, plus a focused campaign over the newly changed discovery/process/actor-integration/TUI-boundary/replay-report/salience functions. Finally run the repository’s configured standing mutation/CI lane rather than treating the narrow campaigns as a replacement.

### Evidence-honesty check

For each load-bearing claim, deliberately break the production behavior—not the test fixture—and confirm the witness fails. At minimum: disable actor discovery, disable process discovery, drop decision trace append, swallow `Stuck`, re-enable TUI post-step perception, omit temporal failures from replay success, and collapse salience novelty. Record the exact command, selected mutation denominator, caught/missed/unviable sets, commit SHA, and whether a test exercised a TUI/core production entry point or a lower-level helper.

## F-08 — The `0049MUTWIT` line lacks a source-discipline record

### Foundational driver

`docs/4-specs/SPEC_LEDGER.md` is the repository’s active/archived spec navigation and source-discipline authority. The reference and execution layers repeatedly distinguish historical tickets, accepted specs, evidence artifacts, current code, and certification scope. R-27/R-28/R-29 make provenance, completeness, and non-decorative evidence especially important for mutation locks.

### Current `8d7c119` repository state

The manifest contains three completed archived tickets:

- `archive/tickets/0049MUTWIT-001.md`;
- `archive/tickets/0049MUTWIT-002.md`;
- `archive/tickets/0049MUTWIT-003.md`.

They explain why test-only mutation witnesses were added, name the 0048 survivors and adjacent selected mutants, and record focused command outcomes. There is no corresponding file in `docs/4-specs/` or `archive/specs/`, and `SPEC_LEDGER.md` ends its relevant archived rows at 0048 before moving to “Next known execution move” ([ledger tail](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/4-specs/SPEC_LEDGER.md#L87-L97)).

This report does not assume that every test-only ticket must become a numbered full feature spec. The defect is that a cross-file mutation-remediation line which closes a recorded accepted-spec survivor floor has no authoritative source/navigation record at all. Future readers can find the tests only by archaeology and cannot tell from the ledger whether 0049 is an implementation correction, an accepted follow-up package, an unratified ticket series, or superseding evidence.

### Conformance verdict

**Provenance gap.** The code/tests may be sound, but the repository’s own source-discipline graph is incomplete. That weakens the long-term anti-regression posture and makes the 0048 survivor closure easy to lose or miscite.

### Required remediation

**Home:** owner/reassess and the repository’s established spec/ledger process, with the resulting navigation/status update in `docs/4-specs/SPEC_LEDGER.md` and any required current source package in the home chosen by that process.

The record should, in substance:

- identify the 0049 line as the follow-up to the eight 0048 survivors and adjacent same-function in-diff mutants;
- name its three archived tickets and affected surfaces;
- state that it is test-only mutation-witness remediation, not a new product feature or latest-main certification;
- preserve the distinction between historical ticket command records and fresh current-commit evidence;
- state whether and how it supplements or supersedes the mutation-evidence scope of the 0048 acceptance;
- route future re-verification to the standing configured perimeter and the named focused witnesses.

Do not invent a new identifier, write paste-ready ratified ledger text, or edit the archived tickets. The repository’s own reassess/ledger workflow must determine the proper package and wording.

### Strongest practical anti-regression guard

Extend the existing spec/ledger integrity guard so a completed cross-cutting remediation ticket series that is cited as closing an accepted spec’s mutation survivor floor cannot be orphaned from the authoritative navigation record. This is a source-graph check, not proof that the tests work; it must remain separate from behavior and mutation evidence.

### Evidence-honesty check

The ledger record must link to independently executable witnesses and exact historical artifacts. The mere presence of a ledger row must never be counted as mutation closure, current command success, or foundational conformance.

## 5. Comprehensive anti-regression layer

The table below covers the load-bearing spec-0047 properties whether currently correct or defective. “Current state” is a static judgment at the target commit. “Existing guard” names repository machinery already present; it is not a fresh command result.

| Property | Current state at `8d7c119` | Existing guard/evidence | Required delta | Strongest practical mechanism |
|---|---|---|---|---|
| One canonical loaded-world tick | **Not satisfied in production.** One coordinator exists, but callers enumerate due work and production submits empty actor/process populations. | `world_step_coordinator.rs`; 0048 differential; TUI seam tests. | Core derives all due actors/processes and every human/no-human interval path delegates to it. | Private core entry point plus production-path human/no-human differential and compile-fail caller boundary. |
| Atomic human action plus world advance | **Substantially correct for operations the coordinator owns.** Scratch physical/agent/log/projection state is committed only at the end, and human rejection can return a rolled-back result. | 0049 pre/post-marker rollback witnesses; world-step tests. | Extend atomic ownership to complete actor transaction products, process invocation, and final perception/interval output. | One scratch transaction with closed phase outcomes; mutations that fail each phase and assert no partial commit. |
| Due-duration discovery from log | **Appears correct.** Open duration candidates are derived from logged starts and completed at the resulting tick. | duration lifecycle tests; generative lock; 0049 boundary witnesses. | Preserve while replacing the request API; make no caller-supplied terminal path. | Private discovery + generated schedules + replay/prefix replay + mutation of due boundaries. |
| Log-derived duration completion | **Appears correct.** Completion events are built from discovered starts and ancestry. | reservation/body-exclusive census; world-step and golden scenarios. | Add production TUI interval witness after F-01/F-04 so the real client path proves it. | Real sleep/work continuation plus exact ancestry and replay assertion. |
| Single-charge passive need accounting | **Appears substantially correct.** Duplicate need-tick application is suppressed and boundary mutants have witnesses. | `need_accounting.rs`; pipeline duplicate-candidate test; deterministic generative lock. | Ensure actor/process discovery changes do not introduce second accounting passes. | One charge per actor/need/tick invariant over generated mixed schedules, prefix replay, and mutation. |
| Body-exclusive reservation | **Appears substantially correct.** Strict `completion > tick` boundary and non-body-exclusive negatives are witnessed. | `reservation_body_exclusive_census.rs`; 0049 scheduler witness. | Re-run through the unified production actor step; preserve strict boundary. | Private reservation query + census + exact-boundary mutation kill. |
| Loaded actor eligibility/discovery | **Missing from production step.** | Test-injected actor IDs and due-work counter. | Core-owned deterministic eligibility from loaded state/scheduling registry. | Compile-time no-injection API; production-path nonzero witness; generated actor-set permutation invariance. |
| Declared process registry/cadence | **Missing from the world-step contract.** Raw final events are accepted instead. | Process counter in world-step test. | Introduce owned declared process invocation/cadence and remove raw envelope input. | Private typed invocation, external-crate negative, boundary/mutation tests. |
| Ordinary proposal routing for autonomous actors | **Partially satisfied.** Coordinator calls the real actor transaction and shared pipeline, but drops artifacts and result handling. | Actor transaction unit tests; world-step actor counter. | Consume the full transaction outcome and commit trace/lifecycle/stuck/feedback atomically. | Closed exhaustive outcome + real-step Proposed/Stuck/Rejected witnesses. |
| No direct dispatch | **Violated for world processes; structurally better for actors.** | pipeline guards; anti-regression source scans. | Remove raw process event append/apply and route declared process work through owned transition/proposal boundaries. | Type/API denial first; behavioral process test second; source scan only as alarm. |
| Decision trace and planner output authority | **Not satisfied in coordinator.** Products exist but are discarded. | Actor transaction unit tests; legacy no-human append helpers. | Commit typed trace/local plan/lifecycle inside the canonical step; delete parallel choreography. | Closed artifact bundle + event-ancestry witness + mutation of every append. |
| Stuck diagnostics | **Not satisfied in coordinator.** `Stuck` becomes `continue`; legacy path reconstructs other diagnostics later. | Actor transaction stuck unit tests; no-human diagnostic tests. | Preserve the transaction’s typed diagnostic and commit it atomically. | Exhaustive match plus real coordinator stuck witness and mutation. |
| One temporal-frontier writer | **Appears corrected.** Scheduler frontier is private and moves through accepted world steps/validated restore. | replay frontier tests; scheduler privacy/source guards. | Preserve while changing APIs; no direct test/state assignment except sanctioned test-only constructors. | Rust privacy + external-crate compile-fail + restore replay witness. |
| Every accepted/empty step has tick ancestry | **Appears corrected at event construction/projector level.** | `build_time_advanced_event`; `validate_time_advanced`; 0049 replay witnesses. | Ensure every new actor/process path remains parented to the same marker and report aggregate fails on violation. | Typed marker parent required by builders; replay integration mutation. |
| Replay reconstructs temporal frontier | **Appears corrected in rebuild.** | `replay/temporal.rs`, `replay/rebuild.rs`, 0049 marker-branch witness. | Add aggregate fail-closed handling and product-level consumer test. | `run_replay` integration with isolated temporal corruption. |
| Replay report success includes temporal validity | **Not satisfied.** | Temporal violations are exposed as a field only. | Add temporal-empty conjunct and typed temporal first divergence. | Integration witness plus focused mutation. |
| Positive holder-known interval delta | **Appears corrected.** Context-holder, debug-context, frontier, source, and exact accessor boundaries are typed and witnessed. | `holder_known_interval_projection.rs`; 0049 interval tests; private notice construction. | Preserve; make core’s returned delta/summary the only embodied product. | Private constructors + one-axis mismatch tests + paired hidden-world noninterference. |
| Debug/embodied separation | **Mostly correct in types, violated by client-side reconstruction authority.** | negative fixtures; debug quarantine tests; view-model tests. | Remove TUI event/projection writes and forbid debug-to-embodied conversion. | Compile-fail boundary and production TUI read-only checksum/log witness. |
| Actor-known salient stop | **Reachable but semantically under-specified/overbroad.** | `salient_stop_actor_known.rs`; typed stop enum. | Select closed novelty/materiality policy and prove quiet/novel/hidden/replay cases. | Production-path four-case behavior witness plus focused mutation. |
| TUI owns no clock/duration/simulation rule | **Partly satisfied.** Core advances time/durations; TUI still appends perception and creates interval summary. | TUI seam/source guards; parity tests. | Return complete typed core result and make rendering read-only. | Crate privacy + negative fixture + log/projection immutability behavior test. |
| Event ID uniqueness | **Not enforced.** Deterministic perception IDs can be appended more than once. | Monotonic global/stream position tests only. | Reject duplicate IDs or deterministically distinguish repeated causal occurrences. | `EventLog` uniqueness invariant for live append and deserialize; mutation and replay tests. |
| Human/no-human possession parity | **Not established for a genuinely loaded step.** | 0048 differential with manually injected work; parity registry/runner. | Drive both paths from identical loaded state/registry without request-side due-work injection. | Paired production runs comparing state, log ancestry, traces, projection, frontier, and due-work metrics. |
| No-human autonomous continuity | **Legacy path performs ordinary work, but canonical tick integration is split.** | no-human capstone and ordinary-life fixtures. | Route no-human actor opportunities and world processes through the single canonical coordinator; delete dual append path. | Long deterministic run + prefix replay + possessed/unpossessed switch differential. |
| Measured TUI parity output | **Useful but cannot compensate for kernel reachability gaps.** | `playable_capability_parity.rs`, parity runner/census/adversarial tests. | Make measured rows consume real nonzero production discovery, core-owned interval summary, and temporal-fail report. | Measured typed result fields from production path; adversarial missing-work and hidden-source negatives. |
| Deterministic generated schedules | **Existing and valuable.** | `generative_lock.rs` and support corpus. | Extend its existing model to mixed controlled input + discovered actors + declared processes + interval stop, without test-built final events. | Deterministic seed corpus, two-sided floors, prefix replay, metamorphic actor/process order invariance. |
| Compile-time authority boundaries | **Strong in several epistemic/debug areas; absent for due-work/raw process injection and TUI perception writes.** | external-crate negative fixtures and runner. | Add negative fixtures for public due-work injection, raw process events, duplicate/direct frontier writes, and client event append/conversion. | Rust module/crate privacy and unforgeable tokens/constructors. |
| Mutation perimeter | **Already covers the seam.** | `.cargo/mutants.toml`, in-diff CI, empty/controlled baseline posture, 0049 focused witnesses. | Add witnesses for new behavior and rerun configured perimeter; do not re-add files or create a second config. | Focused kills during implementation, configured standing lane at closeout, exact denominator/artifact record. |
| Source/topology guards | **Useful but insufficient.** | `anti_regression_guards.rs`, CI scans, meta-lock conventions. | Add only narrow alarms for forbidden API shapes after replacement. | Synthetic-negative source guard explicitly labeled topology-only. |
| 0049 source record | **Absent.** | Three archived tickets only. | Owner/reassess creates proper source package/navigation record and ledger status. | Spec/ledger graph integrity check, kept separate from behavior proof. |

### 5.1 Mutation-testing disposition

The eight survivors recorded by the 0048 acceptance should be treated as follows:

| 0048 survivor group | Live witness posture | Second-pass disposition |
|---|---|---|
| Two `VerifiedActorKnownIntervalNotice::source_key` replacements | Exact accessor witness exists and is directly mutant-sensitive. | Preserve; rerun the 0049 interval command and configured lane. No further design change is indicated. |
| Two `transact_world_one_tick` rejection-predicate inversions | Pre- and post-marker human rejection rollback witnesses exercise real transaction branches and compare rolled-back state. | Preserve; structurally non-vacuous. Extend atomic rollback coverage after actor/process/perception ownership is unified. |
| Four temporal projector/validator boundary survivors | Split-condition/equal-boundary tests exist in the real functions; rebuild marker branch is also witnessed. | Preserve; structurally non-vacuous. Add the missing `ReplayReport::matches_expected` integration witness, which is a distinct defect. |

The 0049 tickets also selected adjacent same-function mutants beyond the original eight, which is healthy: a filter denominator is a property of selected functions, not a promise that only the originally listed mutations exist. Closure must record the fresh selected denominator and full caught/missed/unviable sets. Zero misses in a filtered campaign does not replace the configured standing perimeter.

### 5.2 Compile-time and source-guard disposition

Rust privacy is the right first defense for authority boundaries: a caller cannot misuse a field or constructor that is not visible. The remediation should therefore remove public representability of caller-supplied due work, raw process envelopes, direct frontier mutation, and client-authored embodied interval products. Compile-fail fixtures should prove those boundaries from outside `tracewake-core`.

Source scans remain appropriate for topology questions such as “did a forbidden field name or helper call reappear in the TUI?” They are not acceptable evidence for atomicity, process cadence, replay reconstruction, noninterference, or actor decision integration. Every such scan needs a synthetic negative routed through the real detector, consistent with the repository’s existing meta-lock posture.

### 5.3 Property and noninterference disposition

The existing deterministic generative harness is sufficient. Extend it with paired cases rather than adding a new property-testing dependency:

- same modeled world and seed, human vs no-human origin;
- same holder-known history, hidden-world counterpart changed;
- same due set inserted in different storage order;
- full run vs every replay prefix;
- one relevant causal change at tick `k`, with quiet ticks before it;
- duplicate or missing causal marker/event identity as a negative.

The paired hidden-world case is especially important for interval summaries and stop reasons: a fact unavailable to the possessed actor must not change embodied output. This is a practical noninterference test over the repository’s own sealed holder-known boundary, not an invitation to add a theorem prover.

## 6. Foundation and documentation determination

### 6.1 Higher-tier amendment verdict

**No amendment to `docs/0-foundation`, `docs/1-architecture`, `docs/2-execution`, or `docs/3-reference` is warranted to authorize the current code.**

The doctrine is internally aligned on every disputed point:

- foundation says waiting advances the shared world, the TUI is not simulation authority, ordinary actors use a canonical sealed cognition transaction, declared processes have causal cadence/ancestry, and replay must reconstruct temporal authority;
- architecture assigns due actor/process selection and consequences to one core coordinator, requires ordinary proposal routing, positively constructed holder-known deltas, a read-only client boundary, and fail-closed replay/evidence;
- execution requires the phase-ordered transaction, human/no-human parity on the real path, typed-before-rendered evidence, and non-vacuous witnesses;
- reference already names reachability overstatement, defect-family incompleteness, and decorative locks as R-27, R-28, and R-29.

The implementation is below the doctrine. Amending doctrine to permit empty production due-work populations, raw event injection, discarded transaction products, client-side event writes, or replay success with temporal violations would erase Tracewake’s accepted causal and possession boundaries. That is forbidden.

F-06 requires an implementation-level owner choice about what counts as salient; it does not expose a foundation contradiction. F-08 requires the source/ledger process to classify and record the 0049 line; it does not justify a new invariant or risk ID.

### 6.2 Required documentation work

After code and executable evidence are complete:

1. **`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`** — truth the 0048 loaded-world, time-control, holder-known stop, parity, and replay-report rows so each cites the repaired production path and exact witness scope. Until then, the current rows should not be used as current conformance proof.
2. **`docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`** — record the final phase choreography and owned process/actor discovery boundary, without weakening higher tiers.
3. **`docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`** — make the human/no-human proof explicitly use core-derived loaded work and the same actor transaction artifacts.
4. **`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`** — identify the complete core-owned interval result and read-only TUI consumption path.
5. **`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`** — map each claim to production-path behavior, mutation, compile-fail, replay, and topology evidence without conflating them.
6. **`docs/3-reference/01_DESIGN_RISK_REGISTER.md`** — update evidence/status under existing R-27, R-28, and R-29; do not mint a new risk ID.
7. **`docs/4-specs/SPEC_LEDGER.md` plus owner/reassess-selected source home** — record the 0049 mutation-witness line’s authority and relation to 0048. Do not author the ratified text in this report.

Archived specs, tickets, acceptance artifacts, and passed certifications remain immutable historical records. None should be edited to make current code look compliant.

### 6.3 Forward routing

This is a **cross-cutting** hardening pass: the defects span foundation-governed authority, architecture transaction ownership, execution evidence, reference risks, source discipline, core code, and the TUI boundary. Out-of-scope or deferred choices therefore route only to owner/reassess decisions and future implementation specs. They do **not** route “down” to a lower documentation tier, because no lower tier can resolve a cross-tier authority conflict.

## 7. Recommended closure order

The order matters. Tests should not harden an API that must be deleted.

1. **Replace the world-step request and establish core-owned discovery.** Remove public due-actor and raw-process-event fields. Introduce deterministic actor eligibility and declared process registry/cadence. Add compile-fail boundaries first.
2. **Unify the actor transition.** Make the coordinator consume and commit the full `ActorDecisionTransactionOutcome`; route no-human actor opportunities through it; delete legacy post-proposal trace/lifecycle/stuck choreography.
3. **Complete core ownership of perception and interval output.** Return the final typed holder-known summary/result from core, remove all TUI event/projection writes, and add fail-closed event-ID uniqueness.
4. **Close replay reporting.** Make temporal violations fail `matches_expected` and add typed temporal first divergence.
5. **Choose and encode salient-stop semantics.** Implement the closed novelty/materiality rule after the process and interval boundaries are authoritative.
6. **Rebuild behavior evidence on production entry points.** Replace harness-injected loaded-world tests with real discovery; add quiet/novel/hidden/replay salient cases; prove TUI read-only consumption and human/no-human parity.
7. **Extend existing deterministic and compile-fail harnesses.** Add generated mixed schedules, order invariance, prefix replay, hidden-world pairs, and external-crate authority negatives. Do not duplicate infrastructure.
8. **Run focused mutation during each change, then the configured standing perimeter.** Preserve the 0049 witnesses, add kills for the new branches, record exact denominators and artifacts, and reject unexplained misses.
9. **Truth documentation and source records only after executable evidence.** Update conformance/risk rows and complete the owner/reassess 0049 ledger/source record. Keep historical artifacts unchanged.

## 8. Open maintainer decisions

These are implementation choices inside settled doctrine, not invitations to reopen the constitutional boundary.

1. **Loaded-actor eligibility representation.** Choose the authoritative source of “due”: per-actor next-decision tick, scheduled opportunity queue, deterministic cadence derived from actor state, or another replayable structure. It must be core-owned, deterministic, and possession-neutral.
2. **World-process registry shape.** Choose whether declared processes produce ordinary proposals or typed process transition results. Either way, cadence/trigger/source ancestry must be explicit and raw final envelopes must not cross the public request boundary.
3. **Actor-step outcome packaging.** Decide the exact internal enum/struct that joins transaction trace, local plan, lifecycle, stuck, pipeline result, and resulting event IDs. The compiler should force every variant to be handled.
4. **Perception timing within the tick.** Decide the phase(s) at which actors perceive before decision and after world change, while preventing duplicate same-causal-identity events and preserving actor-known causality.
5. **Repeated observation identity.** Decide whether identical observations at different causal opportunities receive a deterministic occurrence/parent component or are deduplicated as the same fact. Duplicate event IDs in the log should not remain representable.
6. **Salience policy.** Decide whether novelty, contradiction, confidence/stance change, newly available record, local state change, or an explicit materiality class stops an interval. The policy must remain holder-known and typed.
7. **World-step versus interval result layering.** Decide whether every tick returns an actor-known delta and `advance_until` aggregates them, or whether the interval driver owns aggregation. The TUI must receive the final typed product and remain read-only either way.
8. **0049 source-package classification.** Owner/reassess must decide the correct repository-native home and status for a test-only mutation follow-up that closes an accepted spec’s survivor floor. This report deliberately does not invent the ratified record.

## 9. Self-check

- [x] The verdict is explicit and the report-file branch is correct: violations and warranted gaps exist.
- [x] Every repository file used as target evidence was fetched from the full exact-commit URL; all selected paths occur in the uploaded manifest.
- [x] No `cb3102e` predecessor finding was asserted as current without re-verification at `8d7c119`.
- [x] Every finding identifies governing invariants/doctrine and names code plus documentation/remediation homes.
- [x] The `0049MUTWIT` absence from both the spec package and `SPEC_LEDGER.md` is a dedicated graded finding.
- [x] The eight 0048 survivors and the three 0049 witness packages were re-assessed for structural vacuity; authoritative rerun commands are named.
- [x] Recommendations extend `.cargo/mutants.toml`, `anti_regression_guards.rs`, the negative-fixture corpus, and `generative_lock.rs`; no duplicate framework or unjustified property-testing dependency is proposed.
- [x] No archived spec/acceptance/certification edit is recommended; no new invariant, gate, risk ID, or glossary term is minted; no paste-ready ratified doctrine wording is authored.
- [x] The higher-tier amendment determination is explicit and rejects changing doctrine to fit incorrect code.
- [x] Repository evidence and external research are separated.
- [x] The static-survey limitation is explicit; no current command result is assumed.
- [x] Forward routing is owner/reassess plus future implementation specs because the pass is cross-cutting.

## 10. References

### 10.1 Target-repository evidence — exact commit

The complete exact-URL set is in the accompanying acquisition ledger. The most load-bearing sources are:

- [`docs/README.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/README.md)
- [`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md)
- [`docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md)
- [`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md)
- [`docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md)
- [`docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md)
- [`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md)
- [`docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md)
- [`docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md)
- [`docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md)
- [`docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md)
- [`docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md)
- [`docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md)
- [`docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md)
- [`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md)
- [`docs/3-reference/01_DESIGN_RISK_REGISTER.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/3-reference/01_DESIGN_RISK_REGISTER.md)
- [`docs/4-specs/SPEC_LEDGER.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/docs/4-specs/SPEC_LEDGER.md)
- [`reports/0047-foundational-hardening-research-report.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/reports/0047-foundational-hardening-research-report.md)
- [`reports/0048_foundational_conformance_hardening_acceptance.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/reports/0048_foundational_conformance_hardening_acceptance.md)
- [`archive/tickets/0049MUTWIT-001.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/archive/tickets/0049MUTWIT-001.md)
- [`archive/tickets/0049MUTWIT-002.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/archive/tickets/0049MUTWIT-002.md)
- [`archive/tickets/0049MUTWIT-003.md`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/archive/tickets/0049MUTWIT-003.md)
- [`.cargo/mutants.toml`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/.cargo/mutants.toml)
- [`crates/tracewake-core/src/scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/scheduler.rs)
- [`crates/tracewake-core/src/agent/transaction.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/transaction.rs)
- [`crates/tracewake-core/src/agent/perception.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/agent/perception.rs)
- [`crates/tracewake-core/src/events/log.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/events/log.rs)
- [`crates/tracewake-core/src/replay/temporal.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/replay/temporal.rs)
- [`crates/tracewake-core/src/replay/report.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/src/replay/report.rs)
- [`crates/tracewake-core/tests/world_step_coordinator.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/tests/world_step_coordinator.rs)
- [`crates/tracewake-core/tests/salient_stop_actor_known.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-core/tests/salient_stop_actor_known.rs)
- [`crates/tracewake-tui/src/app.rs`](https://github.com/joeloverbeck/tracewake/blob/8d7c119d94e5438244b6cb1e8e57428c26cb233e/crates/tracewake-tui/src/app.rs)

### 10.2 External research lane

External sources were used only to sharpen guard design; they were not used to assert target-repository state.

- **Rust Reference, “Visibility and privacy.”** Rust’s module/crate privacy rules support making invalid authority crossings unrepresentable rather than relying only on tests. <https://doc.rust-lang.org/reference/visibility-and-privacy.html>
- **cargo-mutants documentation.** Mutation testing asks whether inserted faults cause tests to fail; this supports treating survivor-killing behavior witnesses as stronger than source presence checks. <https://mutants.rs/>
- **Leslie Lamport, “Time, Clocks, and the Ordering of Events in a Distributed System.”** Causal order and an order consistent with it motivate explicit event ancestry/frontier validation rather than a client-maintained clock. <https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/>
- **Fred B. Schneider, “Implementing Fault-Tolerant Services Using the State Machine Approach: A Tutorial.”** Deterministic ordered requests applied to a state machine reinforce the single authoritative transition and replay-derived state posture. <https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf>
- **Joseph A. Goguen and José Meseguer, “Security Policies and Security Models.”** The noninterference framing—one class of hidden actions must not alter what another observer sees—sharpens paired hidden-world tests for actor-known interval output. <https://www.cs.purdue.edu/homes/ninghui/readings/AccessControl/goguen_meseguer_82.pdf>
- **Richard A. DeMillo, Richard J. Lipton, and Frederick G. Sayward, “Hints on Test Data Selection: Help for the Practicing Programmer.”** The live-mutant distinction supports the report’s requirement that witnesses distinguish concrete faults or explicitly classify equivalence rather than accepting unexplained survivors. <https://www.inf.pucrs.br/~zorzo/cs/demillo-mutants.pdf>

## 11. Final determination

At the user-supplied target commit, the spec-0047 surface is **not foundationally conformant**. The 0048/0049 work should be retained where it genuinely improved atomic staging, frontier privacy, temporal projection, holder-known types, duration/accounting/reservation behavior, and exact mutant sensitivity. Closure now requires replacing caller-owned due work and raw process events, unifying the full actor decision transaction, removing TUI event/projection authority, making replay success temporally fail-closed, strengthening salient-stop and production-reachability evidence, and repairing the 0049 source-discipline record. No foundation amendment is justified.
