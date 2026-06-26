# Spec-0047 foundational-conformance hardening — fourth-pass research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `6495d7dfe7d2d8887d4bb2ce583074c87fb273e8`  
**Audit posture:** independent post-0051 static re-audit of the loaded-world / time-control / TUI-authority seam  
**Certification posture:** preliminary static judgment only; no build, test, replay, golden, CI, or mutation command was executed in this session

---

## 1. Verdict

### 1.1 Foundational-conformance verdict

**Not conformant.** At exact commit `6495d7dfe7d2d8887d4bb2ce583074c87fb273e8`, spec 0051 has produced a real core-owned `LoadedWorldRuntime` and has repaired several important internal coordinator defects, but the production spec-0047 surface is still below the governing foundation and architecture in four decisive ways:

1. **The production TUI does not use the loaded-content handoff that derives runtime due-work authority.** `LoadedFixture::into_runtime_initial_state` constructs a scheduler with `DeterministicScheduler::from_loaded_world`, but `TuiApp::from_golden` manually rebuilds `RuntimeInitialState` and injects `DeterministicScheduler::new(SimTick::ZERO)`. The production TUI therefore starts with empty loaded-actor and declared-process registries even though a separate loader test exercises the correct handoff. This is a production-reachability and witness-vacuity failure, not a minor wiring omission.

2. **The runtime is not the sole command and mutation authority.** Its closed `RuntimeCommand` currently represents only one-tick wait. Public methods expose raw authoritative aggregates, proposal-sequence allocation, a caller-selected `advance_world_after_acceptance` boolean, direct non-wait pipeline dispatch, raw `advance_until`, caller-supplied no-human actor lists, replay rebuild, and perception refresh. The TUI composes those methods itself. A core-owned struct exists, but the client still owns meaningful transaction choreography.

3. **Replay restoration reconstructs a fresh default scheduler rather than replaying scheduler authority.** Both restore paths call `from_loaded_world` at the reconstructed frontier. That resets proposal sequencing, actor next-opportunity state, and process declarations to the default every-tick synthetic topology. The current continuation witness begins from that same default topology and compares a shallow next-step projection, so it cannot prove preservation of non-default runtime authority.

4. **The declared loaded-world process remains a generic marker that applies as `WorldNoOp` and is nevertheless counted as an applied world process.** Its default declaration has no source-event IDs or random provenance, and it has no process-specific state or projection transition. This does not satisfy the declared-causal-process burden of INV-088.

Additional live defects reinforce that verdict: the actor census is not exhaustive despite the 0051 closure claim; exact hidden temporal/control metadata is printed in a normal command path and remains exposed by the embodied product API; public `test-support` feature shapes make supposedly sealed products forgeable; the no-human/debug path is a TUI-authored sequence over raw aggregates; the standing mutation lane is not a merge-required full-surface green barrier; and live conformance/risk rows still state production reachability and family closure that the current code does not establish.

The result is not “0051 accomplished nothing.” It accomplished substantial and necessary work. The result is that **0051 closed important implementation instances without closing the authority class**. The seam has therefore reopened for a fourth pass.

### 1.2 Higher-tier amendment verdict

**No amendment to `docs/0-foundation` through `docs/3-reference` is warranted.** The governing doctrine already requires:

- one ordinary actor/scheduler/action path without human privilege;
- eventful, causal, replayable world progression;
- declared processes with source, cadence, inputs, randomness, traces, and replay ancestry;
- a TUI that consumes actor-filtered products rather than implementing simulation rules;
- sealed actor-known cognition and temporal information-flow boundaries;
- harsh, path-under-test, non-vacuous acceptance evidence;
- permanent anti-contamination and mutation sensitivity rather than a decorative or perpetually non-green lock.

The code and evidence fall below those rules. Weakening doctrine to bless an empty production scheduler, caller-selected progression, replay reseeding, a diagnostic process marker, client-owned aggregate choreography, exact hidden time in normal output, or a permanent mutation-survivor floor would be a constitutional inversion. Live documentation needs post-implementation truthing, but not doctrinal relaxation or a newly minted invariant, gate code, risk ID, or glossary term.

### 1.3 Decisive evidence summary

| Decisive property | Static verdict at `6495d7d` | Basis |
|---|---|---|
| Production loaded-content handoff derives actor/process due work | **Violation / vacuity gap** | Loader has the correct handoff; `TuiApp::from_golden` bypasses it and injects `DeterministicScheduler::new`. |
| Runtime aggregates are physically owned by core | **Present** | `LoadedWorldRuntime` fields are private and the direct-field mutation negative fixture targets the real type. |
| Runtime is the sole public command/mutation boundary | **Violation** | Only wait is a closed command; raw getters and parallel public mutation methods remain. |
| Controlled actor is excluded from autonomous same-tick opportunity | **Present** | The world-step coordinator builds a controlled-actor set and filters due autonomous actors. |
| Due actors consume `ActorDecisionTransactionOutcome` | **Present** | Proposed and stuck outcomes are consumed into typed summaries/events/diagnostics. |
| Exhaustive one-disposition-per-loaded-actor census | **Not established** | Only attempted due actors receive `ActorStepSummary`; status is only `Proposed` or `Stuck`. |
| Replay restores runtime scheduling authority | **Violation** | Restore calls `from_loaded_world`, recreating defaults instead of reconstructing proposal/process/actor schedules. |
| Declared process performs a causal process transition | **Violation** | Generic `DeclaredWorldProcessApplied` applies as `WorldNoOp` and is counted as applied. |
| Embodied renderer omits exact interval metadata | **Present in `render_embodied_view`** | It renders qualitative notices only. |
| Normal TUI surface is temporally non-leaking | **Violation** | `continue` prints exact stop reason, tick count, and stop tick; APIs expose exact ticks/frontiers. |
| Standing mutation evidence is a durable green merge barrier | **Violation / evidence-honesty gap** | Historical standing run had 23 misses; full sharded lane is scheduled/manual, while PR in-diff mutation cannot detect all witness-only regressions. |
| Live conformance rows truthfully describe production path | **Violation** | Architecture/reference rows name `from_loaded_world` as production entry and cite closure that the TUI path contradicts. |

---

## 2. Disposition table

| ID | Finding | Primary code target | Primary live-doc target after executable closure | Classification | One-line constitutional basis |
|---|---|---|---|---|---|
| F4-01 | Production TUI bypasses the derived loaded-world runtime handoff | `tracewake-content/src/load.rs`; `tracewake-tui/src/app.rs`; `runtime/session.rs` | Architecture `00`, `04`, `10`, `13`; execution `05`, `10` | **Violation; vacuity gap; evidence-honesty gap** | INV-004/005/087/091/094 require the same autonomous world under possession; INV-098 rejects a witness that bypasses production reachability. |
| F4-02 | The public runtime command boundary is incomplete and retains client-selected transaction choreography | `runtime/session.rs`; `tracewake-tui/src/app.rs`; `actions/pipeline.rs` | Architecture `04`, `05`, `10`; execution `05`, `07` | **Violation** | INV-009/018/043/069/098 require shared eventful validation and forbid TUI simulation authority. |
| F4-03 | Replay restore reseeds scheduler authority instead of reconstructing it | `scheduler.rs`; `replay/{temporal,rebuild,report}.rs`; `runtime/session.rs` | Foundation replay contract; architecture `02`, `04`; execution `05`, `09`, `10` | **Violation; mutation-survivor disposition** | INV-018/092 require deterministic continuation from replay-critical history, not a fresh default scheduler. |
| F4-04 | Declared loaded-world process is a generic no-op marker counted as applied | `scheduler.rs`; `events/{envelope,apply,log}.rs` | Architecture `02`, `04`, `12`; execution `05`, `10` | **Violation** | INV-001/009/010/088 require a declared causal process with source, transition, trace, and replay meaning. |
| F4-05 | The claimed exhaustive actor disposition census is incomplete | `scheduler.rs`; actor transaction/trace; coordinator and reservation tests | Architecture `04`, `05`, `13`; execution `05`, `06`, `10` | **Hardening gap; evidence-honesty gap** | INV-004/041/043/094/098 require ordinary-agent parity and inspectable, non-vacuous per-opportunity evidence. |
| F4-06 | Embodied temporal products and normal command output remain authority-bearing and forgeable | `view_models.rs`; `projections.rs`; `tracewake-tui/src/{app,run,render}.rs`; crate feature configuration | Architecture `03`, `10`, `13`; execution `07`, `10` | **Violation; mutation-survivor disposition** | INV-067/068/093/101/102/112 require actor-known temporal information flow and debug quarantine, not exact clock/control leakage. |
| F4-07 | No-human, replay, perception, view construction, checksum, and debug flows remain TUI-orchestrated over raw aggregates | `runtime/session.rs`; `tracewake-tui/src/app.rs`; core view/debug facade | Architecture `02`, `04`, `10`; execution `05`, `06`, `07` | **Violation; mutation-survivor disposition** | INV-069 makes the TUI a client; INV-009/018/101/108 require core-owned causal and cognition transactions. |
| F4-08 | The standing mutation floor is not yet a required, green, full-surface barrier | `.cargo/mutants.toml`; CI workflow; existing lock/mutation suites | Architecture `13`; execution `03`, `10`; reference R-27/R-28/R-29 status | **Mutation-survivor disposition; evidence-honesty gap** | INV-092/093/094/098 require regression tests that detect meaningful replay, leakage, parity, and authority changes. |
| F4-09 | Live conformance and risk documentation still overclaims closure and production reachability | Architecture index; architecture `02/04/05/10/13`; execution `05/06/07/10`; reference `00/01`; spec ledger navigation | Same files | **Evidence-honesty gap** | INV-098 and the validation doctrine reject documentation that treats a non-production or mutation-insensitive path as closure. |

The `food_source_fact_supersedes` mutation family is deliberately **not** reassigned to a lower tier. It remains a cross-cutting, routed-forward mutation concern, exactly as the 0051 acceptance classified it. It still has to be resolved or semantically justified before the canonical standing perimeter can honestly be called green.

---

## 3. Method and provenance ledger

### 3.1 Authority order and non-carry-forward posture

The audit applied the repository authority order exactly as documented:

1. `docs/0-foundation` governs all behavior and acceptance decisions.
2. `docs/1-architecture` translates foundation into subsystem ownership and boundaries.
3. `docs/2-execution` owns implementation proof order and executable acceptance procedure.
4. `docs/3-reference` provides reviewer navigation and risk memory, not competing doctrine.
5. `docs/4-specs`, reports, archived specs, tickets, and acceptance artifacts are lineage and historical evidence only.

The third-pass report’s F-01…F-09 findings were treated as a **pre-remediation specification and seed**, not as live facts. Each property was re-derived from post-0051 code. Where the code now establishes a property, this report records it as **present**. Where a closure claim is contradicted or not made non-vacuous by the current source, this report records the live defect. The 0051 acceptance is cited only for what it historically claimed and for its historical command/mutation transcript; its “pass” rows are not used as proof of current conformance.

### 3.2 Exact-commit acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 6495d7dfe7d2d8887d4bb2ce583074c87fb273e8
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw URL) and container.download(full exact raw URL) for local review copies
Requested file count: 158
Successfully verified file count: 158
Fetched repository files:
- Complete append-only exact-URL list: Appendix A of this report
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Every selected path appears exactly in the uploaded manifest. Every target-repository request used the mechanical base:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/
```

plus the exact manifest path. Representative preflight and bulk acquisition returned the requested file content rather than repository metadata, a branch page, search result, snippet, or error-page substitute. No requested or resolved source changed owner, repository, commit, or path. References to other repositories inside validly fetched content were treated as ordinary file content.

The pre-analysis acquisition report contained 151 required paths. During substantive review, seven additional manifest-listed paths were appended to verify workspace/tooling configuration, feature topology, and the relevant negative-fixture feature shapes. The final authoritative acquisition count is therefore 158, and Appendix A is the complete append-only ledger.

### 3.3 Static-survey limitation

This session did **not** run `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, replay/golden commands, GitHub Actions, or `cargo mutants`. Therefore:

- “present,” “violation,” and “gap” are static judgments about source shape, visibility, authority, data flow, test intent, and witness topology;
- no current gate is certified green or red by execution here;
- command outcomes in the 0051 acceptance are historical claims by that artifact at implementation commit `a3b46c6…`, not current results at `6495d7d…`;
- the implementing session must execute the complete closure gate from a clean baseline and may discover additional runtime defects;
- no static finding should be “closed” by source-text assertions alone.

### 3.4 Re-verification matrix against third-pass F-01…F-09

| Third-pass property | Fourth-pass disposition from post-0051 source |
|---|---|
| F-01 production loaded-world discovery | **Still violated at the production TUI bootstrap.** The loader handoff exists, but the TUI bypasses it. |
| F-02 replay/save reconstruction of due-work authority | **Still violated.** Restore recreates default authority with `from_loaded_world`; it does not reconstruct scheduler state. |
| F-03 declared-process causal transaction | **Still violated.** The default process event is a generic `WorldNoOp` marker counted as applied. |
| F-04 controlled/autonomous mutual exclusion and census | **Mutual exclusion present; exhaustive census not established.** This is a genuine partial repair. |
| F-05 full actor decision transaction consumption | **Present in the core world-step path.** Proposed/stuck outcomes are consumed with typed ancestry/diagnostics. |
| F-06 TUI de-authority | **Still violated at the public API and orchestration layer.** Ownership moved into core, but the TUI retains raw handles and transaction sequencing. |
| F-07 sealed split temporal products | **Renderer repair present; product and normal-command boundary still violated.** Qualitative view rendering exists, but exact metadata remains public and is printed by `continue`. |
| F-08 mutation closure | **Not closed.** The historical standing campaign was non-green; current CI is not a required full-surface green merge barrier. |
| F-09 live evidence truthing | **Still violated.** Live rows describe a production constructor/path the TUI does not use and cite closure beyond the static code’s support. |

### 3.5 Properties genuinely present after 0051

These repairs should be preserved, not re-commissioned:

- `LoadedWorldRuntime` physically owns the registry, physical state, agent state, event log, epistemic projection, controller bindings, scheduler, and manifest identity behind private fields.
- The one-tick world step uses scratch aggregates before commit, so its intended atomic cutover is materially stronger than the pre-0051 path.
- The world-step request no longer accepts caller-injected due actor IDs or prebuilt process events.
- Controlled actors are excluded from the autonomous due-actor set for the same tick.
- Due autonomous actors enter `ActorDecisionTransaction`, and both `Proposed` and `Stuck` outcomes are consumed into typed summaries and committed ancestry/diagnostics.
- Duplicate `EventId` rejection remains a fail-closed event-log boundary.
- `render_embodied_view` renders qualitative interval notices and omits exact interval ticks/frontiers/stop reason.
- The debug overlay is explicitly labeled non-diegetic.
- Existing negative-fixture, generative-lock, coordinator, replay, interval, salience, reservation, parity, and mutation machinery provides the right framework to extend; no new testing dependency is necessary.

Those are meaningful improvements. They are not enough to offset the production-bootstrap and public-boundary defects.

---

## 4. Findings

## F4-01 — Production TUI bootstrap bypasses the derived loaded-world runtime handoff

### Foundational driver

The controlling foundation is INV-004, INV-005, INV-087, INV-091, INV-094, and INV-098. Human possession must change input/viewpoint only; the same autonomous actors and processes must continue; and acceptance must prove the real playable path. Architecture `04` assigns world-step authority to core, architecture `10` makes the TUI a read-only client, architecture `13` requires production-path observability, and execution `05` forbids a parallel possessed-actor route.

### Current `6495d7d` code state

The content-side handoff is structurally correct:

- [`LoadedFixture::into_runtime_initial_state`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/src/load.rs#L56-L77) calls `DeterministicScheduler::from_loaded_world` with loaded physical state, agent state, and manifest ID.
- The loader unit witness, [`loaded_fixture_hands_off_derived_runtime_due_work`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/src/load.rs#L584-L612), constructs a runtime through that handoff and observes two actor transactions plus one process invocation.

The production TUI takes a different path:

- [`TuiApp::from_golden`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/app.rs#L82-L124) loads content, then manually reassembles `RuntimeInitialState`.
- At lines 103–112 it calls public `LoadedWorldRuntime::from_initial_state` and injects [`DeterministicScheduler::new(SimTick::ZERO)`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/app.rs#L103-L112), not the scheduler derived by the loader.
- `DeterministicScheduler::new` initializes empty `loaded_actor_next_decision_tick` and `declared_world_processes` maps. Consequently the production TUI wait path does not start with the loaded actor/process census that the loader handoff witness proves.

This is the exact shape R-27 and R-29 warn against: a correct implementation seam and a green-looking witness can coexist while the production entry bypasses both.

### Conformance verdict

**Violation; vacuity gap; evidence-honesty gap.** The user-facing production path does not establish the no-human/possession-parity world progression claimed by 0051. The separate loader and generated-runtime witnesses prove a reachable correct path, not the path `TuiApp` uses.

### Required remediation

**Code home**

1. Make content loading return an opaque core-owned bootstrap/export that can be consumed only by the production runtime constructor. Do not expose a public aggregate bag containing an injectable scheduler.
2. Remove the externally usable `LoadedWorldRuntime::from_initial_state(RuntimeInitialState)` authority path, or make it crate-private behind a core/content integration boundary that the TUI cannot call.
3. Make `TuiApp::from_golden` call the same loader-to-runtime production constructor as all other loaded worlds. It should receive an already-authoritative runtime/session, not physical state plus a scheduler choice.
4. Preserve initial replay seed state through an opaque replay seed/snapshot product owned by core. The TUI should not retain mutable-authority initial aggregates merely to orchestrate rebuild later.
5. Do not add a compatibility alias that leaves the injectable path available. This must be an atomic cutover.

**Live-doc home, after executable closure**

- Truth the loaded-world row in architecture `00`.
- Clarify the single bootstrap and client boundary in architecture `04` and `10`.
- Record path-under-test evidence requirements in architecture `13` and execution `05`/`10`.
- Update reference `00` and R-27/R-29 status evidence without minting risk IDs.

### Strongest practical anti-regression guard

The load-bearing guard is compile-time unrepresentability plus a public-boundary behavior test:

- External negative fixture: a client crate cannot construct `RuntimeInitialState`, inject `DeterministicScheduler`, or call an unvalidated constructor.
- Production-boundary integration test: invoke the real content loader and the same public TUI/application bootstrap, bind one actor, submit the typed wait command, and assert committed effects from at least one *other* loaded actor and one declared process.
- The witness must assert event IDs/types, actor/process identities, resulting state/projection changes, and replay agreement—not just due-work counters.
- A deliberate local mutation that swaps the production constructor for `DeterministicScheduler::new` must make this test fail.

Extend `negative_fixture_runner.rs`, `generative_lock.rs`, the loader test, `world_step_coordinator.rs`, and the TUI seam tests. Do not add a new framework.

### Evidence-honesty check

A non-vacuous closure witness must begin at the production bootstrap and never construct a scheduler, seed actor eligibility, register a process, or call `from_loaded_world` from test-only setup. It becomes vacuous if the harness creates a correct runtime independently of `TuiApp`, asserts only that `from_loaded_world` can work, or observes only a `TimeAdvanced` marker while loaded actors/processes remain absent.

---

## F4-02 — The runtime command boundary is incomplete and preserves client-selected transaction choreography

### Foundational driver

INV-009, INV-018, INV-043, INV-069, INV-087, INV-098, and INV-108 require one eventful, ordinary-agent path with core-owned ordering. Architecture `04` requires one core world-step coordinator; architecture `05` requires the actor decision transaction; architecture `10` forbids client ownership of simulation aggregates; execution `05` forbids direct dispatch and parallel possessed-actor paths.

### Current `6495d7d` code state

The type names promise a closed command boundary, but the API does not yet enforce one:

- [`RuntimeCommandKind`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/session.rs#L44-L64) contains only `OneTickWait`.
- Even that closed command is not the production TUI wait boundary: `TuiApp::submit_entry` converts `wait.1_tick` into a proposal and routes it through `submit_controlled_proposal`, rather than calling `RuntimeCommand::one_tick_wait` / `LoadedWorldRuntime::wait_one_tick`. The nominal command API is therefore demonstrated by isolated runtime/loader witnesses, not by the real interactive path.
- [`RuntimeInitialState`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/session.rs#L18-L29) is public with public fields, including the scheduler.
- The runtime publicly exposes registry, physical state, agent state, event log, epistemic projection, controller bindings, and manifest ID by reference.
- [`assign_proposal_sequence`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/session.rs#L167-L169) lets the client allocate ordering authority.
- [`submit_controlled_proposal`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/session.rs#L171-L230) accepts a caller-selected `advance_world_after_acceptance` boolean. The `true` branch uses `transact_world_one_tick`; the `false` branch constructs `PipelineContext` and calls `run_pipeline` directly without a world step.
- [`TuiApp::submit_entry`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/app.rs#L238-L280) decides that only semantic action `wait.1_tick` advances the world, allocates the proposal sequence, constructs the proposal, and forwards the boolean.

Thus non-wait human actions remain a parallel client-selected transaction class. The TUI decides whether the rest of the loaded world advances, and it participates in proposal identity/ordering authority.

### Conformance verdict

**Violation.** Moving fields into a core struct is not equivalent to moving authority into a core command protocol. The public API still allows clients to choose transaction topology.

### Required remediation

**Code home**

1. Replace the boolean and raw proposal route with a closed typed command family owned by core. Commands should represent semantic player intent, one-tick wait, continue-until, binding, no-human/debug operations where permitted, and replay/debug requests as distinct variants or opaque constructors.
2. Allocate proposal IDs/sequences and ordering keys inside the runtime. The client supplies semantic intent and actor-filtered target selection, not scheduler sequence authority.
3. Route every accepted world-affecting command through one core transaction coordinator. A non-wait action may or may not consume time according to doctrine and action semantics, but that policy must be a core rule—not a TUI boolean.
4. Return sealed receipts that contain only the correct product class: embodied result, debug result under capability, or typed rejection. Do not return raw `PipelineResult`/`AdvanceUntilResult` where they expose authoritative internals.
5. Keep rejected-action atomicity explicit: the core decides whether a rejected proposal advances nothing, commits a consequential failure event, or produces actor-visible feedback, in accordance with event doctrine.

**Live-doc home, after executable closure**

- Architecture `04` and execution `05` should name the closed command/receipt boundary and state that no client chooses world-step choreography.
- Architecture `05` should point from human semantic intent to the same decision/proposal/validation infrastructure without implying player-only proposal authority.
- Architecture `10`/execution `07` should describe the receipt/view products, not raw aggregate access.

### Strongest practical anti-regression guard

- Compile-fail fixtures proving an external crate cannot call `run_pipeline` with runtime aggregates, assign proposal sequences, pass a world-advance policy boolean, or obtain authoritative aggregate references from the session.
- A table-driven public-boundary suite that sends wait, non-wait accepted, non-wait rejected, duration start/continue, and pause/continue commands through one dispatcher and asserts the correct world-step/event behavior.
- A differential witness in which an equivalent controlled and autonomous actor action reaches the same validation/event rules, with controller binding changing input only.
- Mutation coverage over command dispatch, action-to-time policy, rollback/commit, and sequence allocation.

### Evidence-honesty check

A closure test is vacuous if it calls `transact_world_one_tick` directly, builds a proposal with a test sequence, or tests only wait. It must use the public production command accepted by the TUI/application and observe the sealed receipt plus committed log/projection consequences.

---

## F4-03 — Replay restoration recreates default scheduler authority instead of reconstructing it

### Foundational driver

INV-018 and INV-092 make deterministic replay foundational and tested. INV-009/010 require eventful causal ancestry. INV-088 requires declared process identity/cadence/source/random ancestry. Architecture `02` owns replay/save/restore; architecture `04` owns scheduler authority; execution `05`, `09`, and `10` require continuation evidence and fail-closed reconstruction.

### Current `6495d7d` code state

`DeterministicScheduler` owns replay-critical state:

- current tick;
- proposal sequence assigner;
- loaded actor next-decision ticks;
- declared process registry, including first due tick, cadence, source event IDs, manifest, and random provenance.

Yet both restore functions perform the same reset:

- `restore_from_temporal_projection` returns `Some(Self::from_loaded_world(projection.reconstructed_final_frontier, ...))` when temporal violations are empty.
- `restore_from_rebuild_report` returns `Some(Self::from_loaded_world(report.reconstructed_final_frontier, ...))` when temporal violations are empty.

`from_loaded_world` then:

- resets proposal sequence to zero;
- schedules every actor with needs at `current_tick + 1`;
- declares one synthetic `process_loaded_world_tick` process at `current_tick + 1`, cadence one;
- supplies no source event IDs and no random provenance.

The current replay continuation test starts both branches from that same default `from_loaded_world` topology, advances a short prefix, restores, and compares the next step mainly by counts/status/event-kind identity. It does not exercise irregular actor opportunity intervals, a nonzero proposal sequence, multiple process definitions, non-unit cadence, process source ancestry, random provenance, or mid-duration/per-actor schedule distinctions. The historical survivor `restore_from_temporal_projection -> None` is therefore a meaningful warning, not an equivalence candidate.

### Conformance verdict

**Violation; mutation-survivor disposition.** Reconstructing the clock frontier is not reconstructing runtime authority. Continuation can diverge while the current shallow default-topology witness still passes.

### Required remediation

**Code home**

1. Define replay-critical scheduler authority as an explicit projection or ancestry-preserving snapshot derived from the event log and accepted initial configuration.
2. Reconstruct at least:
   - next proposal sequence or a deterministic event-derived equivalent;
   - per-loaded-actor next opportunity and any deferral/reservation disposition;
   - every declared process identity, trigger/cadence, source-event ancestry, scope, content identity, deterministic random provenance, and next due state;
   - open duration/reservation state relevant to the next world step.
3. Make restore fail closed when required authority is absent, ambiguous, unsupported, or inconsistent with physical/agent/projection state.
4. Do not silently synthesize the default process topology during replay restore. Initial loading may derive initial declarations from validated content, but continuation must preserve the authority that actually existed at the replay frontier.
5. Make `LoadedWorldRuntime::rebuild_from_owned_log` an internal atomic command that replaces all aggregates and scheduler authority together or changes nothing.

**Live-doc home, after executable closure**

- Foundation replay contract `03` needs no rule change, but implementation pointers may be refreshed.
- Architecture `02` should identify scheduler authority as replay-critical state and name the fail-closed restoration product.
- Architecture `04`/execution `05` should state how actor/process opportunity state is restored.
- Execution `09`/`10` should specify the continuation witness dimensions and diagnostics.

### Strongest practical anti-regression guard

Use an uninterrupted-versus-prefix-rebuild differential over the real runtime boundary:

- create a loaded world with at least three actors on nonuniform next-opportunity schedules;
- consume multiple proposal sequences before the save point;
- declare at least two processes with different cadence, source events, and deterministic random provenance;
- include an open duration/body reservation and an actor deferred past the immediate next tick;
- run a prefix, rebuild from owned history, then execute the same next N typed commands;
- compare exact event envelopes/order/IDs, proposal ancestry, process triggers, actor dispositions, receipts, physical/agent/epistemic projections, and checksum/frontier.

Mutants that return `None`, reset sequences, change cadence, drop a process/source, or move an actor’s next tick must be killed. A test that compares only final physical state or event kinds is insufficient.

### Evidence-honesty check

The witness is vacuous if the restored topology equals the constructor default by construction. It must deliberately create replay-critical state that `from_loaded_world` cannot infer from current physical/agent maps alone.

---

## F4-04 — The declared loaded-world process is a generic no-op marker counted as applied

### Foundational driver

INV-001, INV-009, and INV-010 require modeled causes and meaningful eventful transitions. INV-088 is explicit: a regional/world process requires declared source, cadence/trigger, inputs, random model, scope, delivery channel, traces, affected beliefs/records, local entry events, ancestry, and replay/debug visibility. Architecture `04` and `12` assign process scheduling/scale semantics; execution `05` requires due work inside the atomic world-step boundary.

### Current `6495d7d` code state

The scheduler’s private process registry and due-invocation type are useful structural repairs. However, the default loaded-world declaration remains synthetic and semantically empty:

- process ID `process_loaded_world_tick`;
- first due tick is the next tick;
- cadence is one;
- source event IDs are empty;
- random provenance is `None`.

At each due tick the scheduler builds a `DeclaredWorldProcessApplied` event. `events/apply.rs` classifies that event as `WorldNoOp`. The world-step loop increments `world_processes_applied` for either `ApplyOutcome::Applied` **or** `ApplyOutcome::WorldNoOp`. The coordinator witness asserts the marker/counter, not a process-specific physical, agent, epistemic, institutional, or regional transition.

The event is therefore simultaneously treated as:

- proof that a process transaction occurred; and
- a no-op at application time.

That is not a declared causal process. It is an observability marker with an authoritative-sounding name and counter.

### Conformance verdict

**Violation.** A no-op marker may be useful diagnostics, but it cannot satisfy INV-088 or be counted as an applied world process.

### Required remediation

**Code home**

Choose one honest model:

1. **Real process transaction:** validated content declares a concrete process kind and causal inputs; invocation runs a process-specific transition that emits one or more meaningful events/effects with source/cadence/random ancestry; only committed, successfully applied effects increment an applied-process count.
2. **Diagnostic marker:** retain a due-process observation/attempt marker, but classify it as diagnostic/non-world-no-op, rename counters accordingly, and do not present it as process application or progression.

For the spec-0047 loaded-world surface, the stronger recommendation is a minimal real process fixture whose cadence produces an observable but bounded world/projection effect. This makes no-human progression and replay continuation non-vacuous.

**Live-doc home, after executable closure**

- Architecture `04` owns the process transaction boundary.
- Architecture `12` owns declared process shape at regional/LOD altitude.
- Architecture `02` owns replay ancestry.
- Execution `05`/`10` own the behavior witness and diagnostics.

### Strongest practical anti-regression guard

- A process fixture with a pre-due, due, and post-due assertion.
- Assert process-specific state/projection/event effects, declaration/source/cadence/random ancestry, and exact replay continuation.
- Assert a `WorldNoOp` outcome does **not** increment “applied” count.
- Mutate event application to `WorldNoOp`, remove the concrete effect, alter cadence/source ancestry, or count no-op as applied; each must fail.

### Evidence-honesty check

A marker’s presence, a counter of one, or an event name containing “Applied” is not evidence of a process transition. The witness must prove an effect that could not exist without the declared process and must replay from its ancestry.

---

## F4-05 — The actor opportunity census is not exhaustive despite the closure claim

### Foundational driver

INV-004/005/087/094 require possession-neutral ordinary actors. INV-041 and INV-105 require inspectable decision/stuck diagnostics. INV-043 requires ordinary-agent validation. INV-098 requires a harsh, non-vacuous acceptance surface. Architecture `04`/`05` and execution `05`/`06` require one scheduler-owned actor opportunity path.

### Current `6495d7d` code state

The strongest part is now present:

- controlled actor IDs are collected from controlled proposals;
- the autonomous due-actor list filters those IDs;
- each remaining due actor enters `ActorDecisionTransaction`;
- `Proposed` and `Stuck` outcomes are consumed into `ActorStepSummary`, proposal/local-plan ancestry, pipeline status, committed events, or diagnostic event.

The missing part is the “closed exhaustive per-tick actor disposition census” claimed by the 0051 acceptance:

- `ActorStepStatus` has only `Proposed` and `Stuck`.
- `actor_step_summaries` is populated for actors whose transaction was attempted.
- There is no closed disposition for controlled this tick, not due, deferred, body-exclusive/reserved, missing required agent substrate, budget-exhausted, invalidated, or otherwise skipped.
- `WorldStepDueWorkSummary` provides counts but no one-row-per-loaded-actor identity proof.

Controlled/autonomous double opportunity appears fixed. Exhaustive census closure is not represented.

### Conformance verdict

**Hardening gap and evidence-honesty gap.** The live behavior is materially better than the third-pass baseline, but the named closure claim exceeds the product and witness shape.

### Required remediation

**Code home**

1. Introduce a closed core-owned actor disposition for every loaded actor at each world step. Reuse existing concepts; do not mint a doctrine identifier.
2. Include at least: controlled proposal path, autonomous proposed, autonomous stuck, not due, deferred/reserved/body-exclusive, and invalid/missing-substrate fail-closed states as the actual implementation requires.
3. Derive the census from the runtime-owned loaded actor set. The caller supplies no actor list.
4. Enforce exactly one disposition per loaded actor, stable ordering, and no duplicate opportunity.
5. Make diagnostics include responsible layer and causal/temporal basis without leaking hidden truth into embodied products.

**Live-doc home, after executable closure**

- Architecture `04`/`05` describe the disposition product and transaction ownership.
- Execution `05`/`06` describe census proof and possession/no-human differential.
- Architecture `13`/execution `10` state that counts alone do not prove exhaustiveness.

### Strongest practical anti-regression guard

- Deterministic generated corpus over actor order, possession placement, due ticks, reservations, active durations, and stuck/proposed outcomes.
- For every step: `census.len() == loaded_actor_set.len()`, actor IDs equal as sets, each actor appears once, controlled actor is never autonomous in the same step, and every closed disposition is reached by at least one fixture or explicitly documented as staged.
- Differential human/no-human run holding world state and due-work equal.
- Mutation coverage over filters, disposition arms, and census cardinality.

### Evidence-honesty check

The witness is vacuous if it compares only `actor_transactions_attempted`, tests a one-actor world, or preselects the actor IDs. It must derive the full loaded actor set independently from the public runtime receipt or accepted debug product and prove one disposition for each.

---

## F4-06 — Embodied temporal products and normal command output remain authority-bearing and forgeable

### Foundational driver

INV-067 requires actor-known embodied reality. INV-068 requires visibly non-diegetic debug. INV-093 treats leakage as high severity. INV-101/102 require sealed actor-known context and provenance. INV-112 permits authoritative time for validation/order while forbidding it as unmodeled cognition or embodied-view authority. Architecture `03` and `10`, plus execution `07`, own this boundary.

### Current `6495d7d` code state

A real rendering repair is present:

- [`render_embodied_view`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/render.rs#L12-L157) renders “actor-known update,” qualitative notice kinds, and a no-new-information message. It does not print exact interval ticks/frontiers/stop reason.
- [`render_debug_overlay`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/render.rs#L159-L191) labels exact context/tick/frontier material `DEBUG NON-DIEGETIC`.

The overall product/client boundary still leaks and remains forgeable:

- [`TuiApp::advance_until`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/app.rs#L293-L305) receives raw `AdvanceUntilResult` and converts its delta into `TypedActorKnownIntervalSummary` in the TUI.
- The normal `continue` command prints `reason`, `ticks`, and exact `stop_tick` outside any debug label in [`run.rs` lines 80–90](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/run.rs#L80-L90). `controller_safety_bound` is controller/debug policy metadata, not actor-known temporal information.
- `TypedActorKnownIntervalSummary` exposes exact start/stop ticks, frontiers, stop reason, notices, and no-new-information through public accessors.
- `EmbodiedViewModel` exposes exact `sim_tick` and holder-known frontier/context accessors.
- Under the public `tracewake-core` feature `test-support`, temporal fields and test constructors become public. Cargo features are additive and selectable by dependants; the TUI itself enables that feature in its dev dependency. The default-feature negative fixtures prove only one compilation shape, not unforgeability across supported feature combinations.
- The TUI mutates the assembled embodied product with public setters for notebook, debug availability, and interval summary after core construction.

This demonstrates the information-flow distinction: access restrictions on fields do not stop exact authoritative time/control facts from flowing through public values and normal rendering.

### Conformance verdict

**Violation; mutation-survivor disposition.** The renderer-specific repair is present, but the normal command surface and public product API still violate the sealed embodied/debug split. The eight view-model accessor survivors are directly relevant because the suite historically did not distinguish meaningful exact metadata from defaults.

### Required remediation

**Code home**

1. Return two distinct core-created products:
   - an **embodied receipt/view** containing only actor-known qualitative interval consequences and actor-legible stop information with provenance;
   - a **debug receipt/view** containing exact ticks, frontiers, event IDs, scheduler stop reasons, and control bounds, constructible only through the existing debug capability boundary.
2. Remove exact tick/frontier/control reason from the embodied public type. Do not merely hide fields while keeping public getters.
3. Make all production fields and constructors private in every feature combination. Replace public `test-support` constructors/fields with internal `#[cfg(test)]` builders, fixture modules, or crate-private test helpers.
4. Move interval-summary attachment and notebook/debug composition into core view-model construction. The TUI receives an immutable product.
5. Render normal `continue` output from the embodied receipt. Exact `stop_tick`, raw tick count, and `controller_safety_bound` belong only in a visibly non-diegetic debug product unless a modeled actor-known clock/procedure supplies them.

**Live-doc home, after executable closure**

- Architecture `03` and `10` name the split product and provenance boundary.
- Architecture `13` names the noninterference witness.
- Execution `07`/`10` specify normal-versus-debug transcript assertions.

### Strongest practical anti-regression guard

- External compile-fail fixtures under default and all supported feature combinations: cannot construct/mutate embodied temporal products, cannot call debug constructors without capability, cannot convert debug receipt into embodied receipt.
- Public command-loop hidden-world pair: two worlds differing only in hidden exact time/frontier/control metadata produce identical normal output and semantic action surface, while debug output differs and is labeled.
- Normal transcript guard bans exact tick/frontier/event IDs and internal stop-reason tokens; debug transcript positively requires them.
- Mutation tests for all interval accessors/stop classification and transcript-section selection.

### Evidence-honesty check

A source scan for the word `tick` is only a topology alarm. Non-vacuous proof compares semantically equivalent hidden-world pairs through the public command boundary and exercises both normal and debug products. A compile-fail test is vacuous if a public feature reopens the fields or if a public getter exposes the same authority.

---

## F4-07 — No-human, replay, perception, view construction, checksum, and debug flows remain TUI-orchestrated over raw aggregates

### Foundational driver

INV-009/018 require eventful/replay-safe mutation. INV-069 says the TUI must not implement simulation rules. INV-101/102 require core-sealed cognition products. INV-108 requires possession neutrality. Architecture `02`, `04`, and `10` assign replay, transaction, and client ownership; execution `05`/`06`/`07` require one core path.

### Current `6495d7d` code state

The runtime owns aggregate fields, but exports enough references and commands for the TUI to remain an authoritative orchestrator:

- `current_view` reads `physical_state`, `agent_state`, action registry, epistemic projection, exact tick, manifest ID, and event-log length; it constructs `KnowledgeContext`, `EmbodiedTruthSnapshot`, projection/preflight sources, and then mutates the view with notebook/debug/interval products.
- `run_no_human_day` in the runtime accepts `Vec<ActorId>` from the caller and invokes the legacy no-human path directly rather than the runtime command dispatcher.
- `TuiApp::run_no_human_day` calls that method with an empty vector, then separately computes a checksum context, calls `rebuild_from_owned_log`, and separately refreshes possessed-actor perception. The client authors the transaction order and decides which postconditions belong to “run no-human day.”
- `rebuild_from_owned_log` and `refresh_actor_current_place_perception` are public mutable methods; both were historical standing survivors.
- Debug panels and checksum methods read raw state/log/projection/controller bindings from the runtime. Debug access is checked in the TUI, not enforced by a core capability-bearing receipt for each raw product.

The public API therefore allows other clients to compose a different order, omit rebuild or perception refresh, supply a different actor list, or read authoritative aggregates to construct their own view/debug surface.

### Conformance verdict

**Violation; mutation-survivor disposition.** Core field ownership is present, but client de-authority is incomplete. The authority boundary must be judged by what the client can cause and observe, not by which struct stores the fields.

### Required remediation

**Code home**

1. Expose one opaque session handle with typed commands and sealed view/debug receipts. Remove public raw aggregate getters from the client-facing API.
2. Make no-human advancement a single runtime command that derives its actor census internally, performs advancement/rebuild/projection/perception work atomically, and returns a typed debug/observer receipt.
3. Make rebuild an internal recovery operation, not a client command. The runtime owns its initial seed/snapshot and checksum context.
4. Make possession binding/perception update one core transaction with explicit event/projection effects and receipt.
5. Route debug queries through the existing non-forgeable debug capability. A debug receipt can expose exact state safely; the TUI should not receive general-purpose authoritative references.
6. Keep the core’s internal modules free to use references; the restriction is the public production client surface.

**Live-doc home, after executable closure**

- Architecture `02` documents atomic rebuild ownership.
- Architecture `04` documents the command protocol.
- Architecture `10` and execution `07` document opaque session/sealed receipts.
- Execution `05`/`06` document runtime-owned no-human census and possession parity.

### Strongest practical anti-regression guard

- External negative fixtures cannot obtain `&PhysicalState`, `&AgentState`, `&EventLog`, `&EpistemicProjection`, `&ControllerBindings`, or `&ActionRegistry` from the production session; cannot call rebuild, perception refresh, sequence allocation, or no-human with an actor list.
- Public-boundary TUI tests bootstrap production, invoke no-human through a typed debug command, and assert atomic committed receipt/replay equivalence.
- A fault-injection/duplicate-event fixture forces an internal post-step failure and proves no partial state/log/projection cutover.
- Mutation kills for `rebuild_from_owned_log -> Ok(())` and perception refresh deletion.

### Evidence-honesty check

A negative fixture that merely fails to mutate a private field is not enough. It must attempt the actual surviving authority routes: public constructor, raw getters, sequencing, rebuild, no-human actor injection, and perception refresh. A behavior witness must use the same public TUI command and prove all constituent effects, not call each runtime helper separately.

---

## F4-08 — The standing mutation perimeter is not a required, green, full-surface regression barrier

### Foundational driver

INV-092, INV-093, INV-094, and INV-098 require replay, leakage, possession parity, and acceptance properties to be regression-tested. Architecture `13` and execution `10` reject decorative locks and require typed path-under-test sensitivity. R-27/R-28/R-29 explicitly warn against overstatement, incomplete correction families, and vacuous guards.

### Current `6495d7d` evidence state

The existing machinery is strong and should be extended:

- [`.cargo/mutants.toml`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/.cargo/mutants.toml) includes the runtime, scheduler, events, replay, view models, epistemics, content loader, and TUI surface.
- The checked baseline-miss file is empty, creating a zero-survivor ratchet for the in-diff lane.
- CI runs lock-layer negative fixtures, invariants, spine/acceptance/anti-regression/replay/hidden-truth/generative/loader/TUI tests on normal pull requests.
- The in-diff mutation job runs on pull request/push and fails new changed-line misses/timeouts not in the baseline.
- The full canonical census and eight mutation shards are supervised and reconciled with commit/config/toolchain fingerprints.

The durable-gap remains:

1. The 0051 standing campaign historically completed with **23 missed mutants**, and 0051 accepted the remediation line with the standing perimeter explicitly non-green.
2. The full standing baseline/shard/reconciliation jobs run only on `workflow_dispatch` or weekly schedule, not as a merge-required pull-request job.
3. In-diff mutation considers mutable production lines in the diff. A pull request that weakens a test, negative fixture, generated corpus, transcript selection, CI trigger, or public-boundary witness without changing the protected production line can pass the in-diff mutation step while making an old mutant survive.
4. A scheduled red run does not itself prevent merging unless repository governance makes its status/recovery mandatory and maintainers treat it as release-blocking.
5. Source guards in `anti_regression_guards.rs` remain topology alarms; they cannot prove production bootstrap, transaction atomicity, continuation, process semantics, or information-flow noninterference.

### Conformance verdict

**Mutation-survivor disposition and evidence-honesty gap.** The repository has the right components, but not yet the required standing barrier. A recurring non-green floor is itself a regression vector.

### Required remediation

This finding is primarily implemented by the standing-gate design in §6. At minimum:

- drive every in-surface survivor to caught or unviable through a defensible compiler/tool reason;
- resolve the routed-forward `food_source` family before calling the canonical perimeter green, or split ownership only in a way that preserves one canonical reconciliation and does not launder survivors;
- require the public-boundary conformance lane on every pull request touching production **or its witnesses/configuration**;
- require a full canonical surface campaign before accepting this remediation and on any change to the perimeter, command boundary, scheduler/replay/process/view products, mutation tooling, or witness suites;
- make the named conformance job and mutation reconciliation required branch-protection checks;
- fail on misses, timeouts, missing shards, missing canonical members, tool error, stale baseline, or fingerprint mismatch;
- keep the accepted baseline at zero for this surface.

### Strongest practical anti-regression guard

The guard is the three-layer enforced barrier in §6: compile-time unrepresentability, a production-bootstrap-to-sealed-receipt CI lane, and a green canonical mutation perimeter. None is sufficient alone.

### Evidence-honesty check

“Mutation completed” is not “mutation passed.” Focused zero-miss campaigns do not certify the standing perimeter. A current green claim requires the exact canonical campaign/reconciliation at the implementation commit, with zero missed/timeouts and complete shard/census fingerprints, plus path-under-test public-boundary tests.

---

## F4-09 — Live conformance and risk evidence still overstates production reachability and closure

### Foundational driver

INV-098’s harsh acceptance rule is translated by architecture `13`, execution `10`, and R-27/R-28/R-29. Live status/navigation documents must not convert a historical acceptance row or a reachable non-production path into present conformance.

### Current `6495d7d` documentation state

The loaded-world row in [architecture `00`](https://github.com/joeloverbeck/tracewake/blob/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md#L98-L108) says:

- the current production entry point is `LoadedWorldRuntime::from_loaded_world`;
- it derives loaded actors and declared processes;
- the TUI crosses through `submit_entry_with_world_advance` rather than client-authored mutation;
- named witnesses prove production construction, process transactions, actor census, and public command crossing.

The code contradicts the first and third claims: `TuiApp::from_golden` uses `from_initial_state` with `DeterministicScheduler::new`, and the TUI still chooses proposal sequencing/world-advance policy and orchestrates no-human/rebuild/perception/view/debug work.

R-27’s 0051 evidence row repeats the production constructor claim. R-28 says the 0051 rows should be cited for current defect-family closure. R-29 describes the seam as behavior-first and production-shaped. Architecture/execution sections also name `from_loaded_world` as the current production constructor.

The spec ledger is more honest: it records the archived line and non-green mutation disposition. Its historical rows should remain immutable. The correction belongs in live conformance/status/navigation only after executable closure.

### Conformance verdict

**Evidence-honesty gap.** The doctrine is correct; the status evidence is ahead of the code.

### Required remediation

Do not edit docs first. After F4-01…F4-08 are implemented and executed:

- architecture `00`: replace the overclaim with the actual single production bootstrap and current named public-boundary/green-mutation witnesses;
- architecture `02`, `04`, `05`, `10`, `13`: update implementation pointers and evidence shape, not doctrine altitude;
- execution `05`, `06`, `07`, `10`: update current commands, products, census, replay, and gate procedure;
- reference `00`: update reviewer pointers;
- reference `01`: update R-27/R-28/R-29 status/evidence only; mint no risk ID;
- `SPEC_LEDGER.md`: route the future numbered remediation spec through the normal process and update “Next known execution move” only when the repository’s acceptance process says so;
- edit no archived spec, ticket, report, or acceptance artifact.

### Strongest practical anti-regression guard

Extend the existing doc-invariant and anti-regression guards so live status rows name symbols/tests that exist, but treat those checks as topology alarms. Pair them with the required public-boundary and mutation jobs. A synthetic negative should deliberately switch the production bootstrap or remove a receipt effect and prove the behavior job fails even if the docs remain unchanged.

### Evidence-honesty check

A grep proving the words `LoadedWorldRuntime::from_loaded_world` appear in a live row is not conformance evidence. The row may become current only after the production TUI test and green canonical mutation result at the same implementation commit are attached.

---

## 5. Standing-mutation survivor disposition

### 5.1 Governing posture

The historical 0051 acceptance states that at implementation commit `a3b46c6…`, `cargo mutants --timeout 183` selected 3,275 mutants and completed with 2,549 caught, 703 unviable, **23 missed**, zero timeout, exit status 2. This report treats those figures as a historical artifact claim, not a current run. No survivor is called equivalent merely because the suite did not kill it. Equivalence requires a semantic argument; in general, deciding mutant equivalence is not mechanically guaranteed.

### 5.2 Survivor-by-survivor disposition

| # | Historical survivor | Surface | Static fourth-pass disposition | Required closure witness |
|---:|---|---|---|---|
| 1 | `food_source_fact_supersedes -> true` | Cross-cutting projection | **Routed forward; unresolved.** Would collapse all replacement decisions to true. No equivalence claim. | Explicit truth table over `(old source, new source, recency/provenance)` plus actor-known projection behavior and mutation rerun. |
| 2 | `food_source_fact_supersedes -> false` | Cross-cutting projection | **Routed forward; unresolved.** Would disable replacement. | Same family truth table and stale/new notice fixtures. |
| 3 | Delete `(Some(_), None)` arm | Cross-cutting projection | **Routed forward; unresolved.** | Case-specific test proving whether sourced old fact supersedes unsourced new fact under doctrine. |
| 4 | Delete `(None, Some(_))` arm | Cross-cutting projection | **Routed forward; unresolved.** | Case-specific test proving sourced new fact behavior. |
| 5 | Replace `<` with `==` | Cross-cutting projection | **Routed forward; unresolved.** | Three-point ordering test: older/equal/newer source frontier or tick. |
| 6 | Replace `<` with `>` | Cross-cutting projection | **Routed forward; unresolved.** | Same ordering test; actor-known stale-workplace/food evidence should expose reversal. |
| 7 | Replace `<` with `<=` | Cross-cutting projection | **Routed forward; unresolved.** | Equal-source semantics test; no laundering as equivalent without proof. |
| 8 | `restore_from_temporal_projection -> None` | **In surface** | **Concrete F4-03 target.** Existing restore reachability/sensitivity is insufficient. | Non-default uninterrupted-versus-restored continuation through public runtime; exact next-N event/receipt equality. |
| 9 | `EmbodiedViewModel::sim_tick -> default` | **In surface** | **Concrete F4-06 target; preferably eliminate accessor from embodied API.** | Debug-only exact-time test plus compile-time absence from embodied product; rerun mutation if function remains. |
| 10 | Interval `start_tick -> default` | **In surface** | **Concrete F4-06 target.** | Remove from embodied public product or assert distinct nonzero start in debug/internal projection witness. |
| 11 | Interval `stop_tick -> default` | **In surface** | **Concrete F4-06 target.** | Nontrivial interval with distinct stop; public-boundary debug/internal witness. |
| 12 | `start_frontier -> 0` | **In surface** | **Concrete F4-06 target.** | Nonzero, non-one start frontier and exact ancestry check in debug/internal product. |
| 13 | `start_frontier -> 1` | **In surface** | **Concrete F4-06 target.** | Same witness with value neither 0 nor 1. |
| 14 | `stop_frontier -> 0` | **In surface** | **Concrete F4-06 target.** | Interval that appends multiple events and asserts exact stop frontier internally/debug. |
| 15 | `stop_frontier -> 1` | **In surface** | **Concrete F4-06 target.** | Same nontrivial frontier witness. |
| 16 | `no_new_actor_known_information -> true` | **In surface** | **Concrete F4-06 target.** | Positive-notice interval must render notices and false no-new flag; empty interval must render true. |
| 17 | Delete match arm 24 in `StuckDiagnostic::deserialize_canonical` | **In surface** | **Concrete target.** Current source contains targeted deserialization tests, but current execution is unverified. | Full-field non-default round trip plus malformed/missing/unknown-field fail-closed cases; focused and standing rerun. |
| 18 | Delete `!` in `StuckDiagnostic::deserialize_canonical` | **In surface** | **Concrete target.** Likely flips a validity predicate. | Construct both valid and invalid canonical records that differ only in the affected predicate; assert typed result/error. |
| 19 | `assign_proposal_sequence -> default` | **In surface** | **Concrete F4-02 target.** A targeted unit test now exists, but the public method itself is authority leakage. | Remove client method; runtime command test submits multiple commands and proves unique monotonic/event-derived ordering. |
| 20 | `rebuild_from_owned_log -> Ok(())` | **In surface** | **Concrete F4-03/F4-07 target.** | Public command creates state divergence that only actual rebuild repairs, then compares all aggregates/scheduler; no direct helper call from TUI. |
| 21 | Delete `refresh_actor_current_place_perception` | **In surface** | **Concrete F4-07 target.** | Binding/post-rebuild command must cause a provenance-bearing perception/epistemic delta observable in sealed view and replay. |
| 22 | Delete `!` in `submit_entry_with_world_advance` (historical line) | **In surface** | **Concrete F4-01/F4-02 target.** Current source expresses the branch as `semantic_action_id == "wait.1_tick"`; the semantic risk remains. | Public TUI differential proving wait and non-wait use the correct single core transaction policy; eliminate client boolean. |
| 23 | Transcript section selector `==` to `!=` | **In surface** | **Concrete evidence target.** Current source has a targeted test, but current mutation result is unknown. | Representative transcript must assert exact required section membership/exclusion and feed the real acceptance artifact path. |

### 5.3 Family-level disposition

- **`food_source_fact_supersedes` (7):** routed-forward cross-cutting. It is not moved to a lower tier and cannot disappear from canonical mutation governance. The implementing line must either resolve it in the same remediation or leave the overall canonical perimeter explicitly non-green and therefore not accept the standing gate.
- **Replay restore (1):** critical live semantic defect, not only a test gap.
- **View/interval accessors (8):** the preferable closure is to remove exact authority from embodied public products, then retain/test exact fields only in internal/debug products. A test that merely calls each getter is weaker than making the forbidden product unrepresentable.
- **Stuck deserialization (2):** likely test-sensitivity gaps in a replay/debug diagnostic codec. They require full non-default round trips and malformed-input cases.
- **Runtime/TUI orchestration (4):** sequence, rebuild, perception refresh, and wait/non-wait branch survivors align directly with the live public-authority defects.
- **Transcript selection (1):** evidence machinery must be mutation-sensitive because acceptance relies on representative transcript sections.

---

## 6. Structural anti-regression and recurrence analysis

### 6.1 Why the seam reopened after a core-owned runtime landed

The third pass correctly diagnosed the need for a single core-owned runtime, but 0051 implemented **aggregate ownership without complete authority closure**. Four recurrence mechanisms remain:

1. **A correct constructor exists beside an injectable constructor.** Tests used the correct one; production used the injectable one. The type did not make the wrong bootstrap impossible.
2. **A closed command token exists beside open public methods.** Wait uses the command; other behavior uses raw methods, booleans, getters, and helper choreography. Reviewers could point to the token while clients bypassed it.
3. **Private fields were treated as client de-authority.** Private storage blocks direct assignment but not public aggregate getters, raw result types, sequence allocation, or multi-call transaction composition.
4. **Focused evidence was allowed to close a family while the standing mutation floor remained non-green.** Focused campaigns tested named repairs; they did not force the entire surface and its public production bootstrap to remain sensitive.

This is why another list of per-finding tests is insufficient. The repository needs a standing barrier that binds production reachability, API authority, and mutation sensitivity together.

### 6.2 Durable enforced standing gate

The barrier should extend existing machinery and have three mutually reinforcing layers.

#### Layer A — Compile-time unrepresentability is load-bearing

The production public API should make the following impossible:

- construct a loaded runtime with a caller-selected scheduler;
- obtain or mutate authoritative aggregate handles from the session;
- allocate proposal sequence/order authority;
- choose whether a command advances the world via a boolean or callback;
- supply the no-human actor/process census;
- call replay rebuild or perception writers as independent client operations;
- construct or mutate embodied temporal/context/debug fields;
- obtain exact debug/time products without the existing non-forgeable debug capability;
- compile a feature combination that reopens those authorities.

Implement with private fields, crate-private constructors, opaque loader exports, unexported authority tokens, closed command constructors, sealed receipts, and separate embodied/debug types. `#[non_exhaustive]` may help evolution, but it is not an authority boundary by itself. Public getters and public feature-gated fields remain flows.

Extend the existing external-crate negative-fixture pattern. Add both default-feature and all-supported-feature compile checks. Keep `anti_regression_guards.rs` only as a secondary topology alarm that verifies the fixtures/perimeter remain registered.

#### Layer B — Required public-boundary conformance CI lane

Create one named CI job by composing existing tests; do not mint a new doctrine gate code. Its path must be:

```text
validated content package
    -> production loader/bootstrap
    -> opaque LoadedWorldRuntime/session
    -> typed TUI/application commands
    -> core transaction(s)
    -> sealed embodied/debug receipts
    -> rendered normal/debug output
    -> replay/rebuild continuation
```

The job must not construct `DeterministicScheduler`, `RuntimeInitialState`, `PipelineContext`, due actor/process lists, raw proposals with assigned sequence, or raw view models in the proving tests.

Minimum behavior matrix:

1. **Production bootstrap + one-tick wait:** other loaded actors and a real declared process advance.
2. **Non-wait accepted action:** uses the same command coordinator; exact time-consumption policy is core-owned.
3. **Rejected action:** proves atomicity and actor-visible feedback without hidden-truth leakage.
4. **Continue/duration:** sealed embodied stop product; exact debug product separately.
5. **No-human:** runtime-derived census and ordinary actor transactions through the same coordinator.
6. **Possession differential:** held-equal controlled/uncontrolled worlds differ only by input route.
7. **Replay restore:** non-default authority continuation, not constructor-default continuation.
8. **Declared process:** concrete state/projection/event effect with source/cadence/random ancestry.
9. **Actor census:** exactly one disposition per loaded actor.
10. **Information-flow pair:** hidden exact time/truth changes do not alter normal output; debug output may differ and is labeled.
11. **Failure injection:** duplicate/event/application/rebuild failure leaves no partial aggregate cutover.

Use existing `world_step_coordinator.rs`, `generative_lock.rs`, replay/interval/salience/reservation/no-human suites, TUI command/seam/adversarial/parity tests, and negative fixtures. Add a dedicated production-bootstrap integration entry if needed, but no new test framework.

Make this job a required branch-protection check. A repository workflow file cannot by itself prove branch protection is configured; the acceptance artifact must record the required-check name and repository-governance confirmation.

#### Layer C — Green standing mutation perimeter with zero-floor governance

The canonical perimeter remains `.cargo/mutants.toml`; do not create a parallel spec-0047 config that can drift.

Required governance:

- **Zero accepted in-surface misses/timeouts.** The baseline for this surface stays empty.
- **Full canonical reconciliation before remediation acceptance.** Every shard must match commit, config, toolchain, and canonical census; missing shards or mutants fail.
- **PR trigger breadth:** run the public-boundary conformance lane whenever production files, tests, fixtures, negative fixtures, mutation config/baseline, CI workflow, merge/supervisor tooling, or live conformance evidence for this surface changes.
- **Mutation trigger:** retain in-diff mutation for fast feedback, but require a full surface campaign when a change can weaken old witnesses without touching old production lines. At minimum: command/runtime/scheduler/replay/process/view model files, any proving tests/fixtures, mutation config/tooling, or CI topology.
- **Scheduled campaign:** keep weekly full mutation as drift detection, but treat a red scheduled result as release/merge-blocking until repaired. Record ownership and an explicit maximum unresolved interval in repository governance, not as a doctrinal threshold invented here.
- **No laundering:** exit 2 (“missed”) and exit 3 (“timeout”) are failures for the standing green claim. Tool errors, incomplete census, stale artifacts, or absent output are failures, not passes.
- **Out-of-surface routing:** the food-source family can be owned by cross-cutting remediation, but the canonical reconciliation must still report it. The project may not call the canonical standing perimeter green while it survives.

The merge script, supervised shards, fingerprinting, empty baseline, in-diff job, and existing mutation tests already provide most mechanics. The necessary change is to make green completion an enforced acceptance and merge property rather than a recorded optional/scheduled condition.

### 6.3 Correct placement in the existing certification ladder

No new gate code is needed or permitted.

`docs/2-execution/03` already says temporal evidence and playable-capability parity are standing acceptance dimensions within the existing ladder. The conformance barrier should be an artifact dependency distributed as follows:

- **SPINE-CERT:** sole command boundary, event/process causality, atomic world step, replay restoration, duplicate-ID fail-closed behavior.
- **EPI-CERT:** sealed actor-known interval/view products, temporal noninterference, debug capability/quarantine.
- **ORD-LIFE-CERT:** no-human actor census, full actor decision transaction, possession-neutral opportunity handling, stuck diagnostics.
- **FIRST-PROOF-CERT:** integrated production content bootstrap through TUI commands to sealed receipts/rendering and replay, with the canonical mutation perimeter green for the depended-on surface.

Each rung’s artifact should cite the same standing job/results appropriate to its burden. This is not a new rung and must not retroactively rewrite already archived certification artifacts. Future acceptance that depends on this surface cannot treat the relevant rung complete while the standing barrier is red or bypassed.

### 6.4 Source-text guard demotion

Source scans remain useful to catch topology drift such as:

- forbidden constructor/method names reappearing;
- raw aggregate imports entering the TUI;
- a mutation perimeter dropping a file;
- a negative fixture disappearing;
- normal output containing known debug token literals.

They are not proof of:

- atomicity;
- replay continuation;
- process semantics;
- exactly one actor opportunity;
- actor-known information-flow noninterference;
- production bootstrap reachability;
- mutation sensitivity.

Every source guard for this surface must point to the compile-time or executable witness it protects and include a synthetic negative that proves the guard itself can fire.

### 6.5 External-research sharpening — separate evidence lane

The recommended barrier is consistent with established external principles:

- Rust visibility can make fields/functions inaccessible outside modules/crates, which supports unforgeable authority boundaries; however, public methods and feature-enabled public fields remain public flows. See the [Rust Reference: visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html) and [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
- Rust’s [`non_exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) is primarily an evolution/construction-matching control, not an information-flow or authority solution.
- State-machine replication requires replicas to begin from equivalent state and process the same ordered requests deterministically. Scheduler/process opportunity state is part of that effective state, not disposable metadata. See Fred Schneider, [Implementing Fault-Tolerant Services Using the State Machine Approach](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf).
- Temporal’s documented workflow model reconstructs workflow state by replaying durable history, a useful prior-art analogy for keeping authoritative orchestration inside the durable runtime rather than in a client. See [Temporal workflows](https://docs.temporal.io/workflows) and [workflow tasks](https://docs.temporal.io/tasks).
- Language-based information-flow work distinguishes access control from controlling what information can flow to outputs. Private fields alone therefore do not prove actor-known noninterference. See Sabelfeld and Myers, [Language-Based Information-Flow Security](https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf).
- `cargo-mutants` documents missed mutants as an unsuccessful mutation-testing outcome and provides CI, baseline, configuration, and in-diff mechanisms; in-diff is fast feedback, not a substitute for a complete standing campaign. See [exit codes](https://mutants.rs/exit-codes.html), [CI](https://mutants.rs/ci.html), [in-diff](https://mutants.rs/in-diff.html), [baseline](https://mutants.rs/baseline.html), and [configuration](https://mutants.rs/config-file.html).
- Mutation-testing research treats equivalent mutants as a difficult semantic problem; survival alone is not proof of equivalence. See Jia and Harman, [An Analysis and Survey of the Development of Mutation Testing](https://web.eecs.umich.edu/~weimerw/2022-481F/readings/mutation-testing.pdf).

These sources shape implementation strategy only. They are not evidence of what exists in `joeloverbeck/tracewake`.

---

## 7. Foundation and documentation determination

### 7.1 Amendment determination by tier

| Tier | Amendment warranted? | Determination |
|---|---:|---|
| `0-foundation` | **No** | INV-001/004/005/009/010/018/041/043/067/068/069/087/088/091/092/093/094/098/101/102/105/108/112 already govern every live defect. |
| `1-architecture` | **No doctrinal amendment** | Ownership and evidence contracts already require one core coordinator, replay authority, read-only clients, sealed products, and behavior-first evidence. Status/implementation pointers need later truthing. |
| `2-execution` | **No doctrinal amendment** | Existing execution docs already prohibit direct dispatch/caller census and require public-path, replay, no-human, parity, anti-leak, and mutation evidence. The standing barrier fits the existing ladder as an artifact dependency. |
| `3-reference` | **No doctrinal amendment** | R-27/R-28/R-29 already name the exact recurrence class. Their current evidence/status rows need correction after code closure. |

No contradiction blocks remediation. The “conditional amendment” branch therefore terminates with **no amendment**.

### 7.2 Post-implementation live-doc work

All documentation work below occurs **after** code and executable witnesses exist at one exact commit.

| File/home | Post-implementation work | Prohibited overreach |
|---|---|---|
| `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Truth the loaded-world row: actual production constructor, closed command boundary, replay/process/census/view products, required job and green mutation evidence. | Do not claim current/latest main or broad certification. |
| Architecture `02` | Point to replay-critical scheduler/process authority projection and fail-closed restore. | Do not weaken replay to current-state reseeding. |
| Architecture `04` | Point to the sole runtime command/world-step coordinator and real process transition. | Do not permit client-selected booleans or caller due lists. |
| Architecture `05` | Point to full actor transaction and exhaustive disposition census. | Do not create a possessed-actor cognition path. |
| Architecture `10` | Point to opaque session and split sealed embodied/debug receipts. | Do not bless raw aggregate getters or public test-feature authority. |
| Architecture `13` | Record path-under-test production bootstrap and standing-green evidence requirements. | Do not treat source scans/focused mutation as exhaustive proof. |
| Execution `05` | Update exact commands, transaction choreography, replay restore, process and census witnesses. | Do not restate a parallel action path. |
| Execution `06` | Update no-human/possession differential through the public runtime. | Do not accept caller-selected actor lists. |
| Execution `07` | Update sealed interval/view/debug products and normal transcript noninterference. | Do not expose exact hidden time as embodied convenience. |
| Execution `10` | Record required conformance job, complete mutation reconciliation, zero-survivor rule, and evidence diagnostics. | Do not make “completed non-green” a pass. |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Update reviewer pointers to the actual public boundary and standing barrier. | Do not create a new checklist identifier. |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Update R-27/R-28/R-29 evidence/status only. | Mint no risk ID and do not retire the general risks. |
| `docs/4-specs/SPEC_LEDGER.md` | Route the future remediation spec and closeout through normal ledger/archive process; update next move only through that process. | Do not assign a spec number in this report; do not edit archived rows as if live. |

Archived specs, tickets, reports, and acceptance artifacts remain immutable historical evidence.

---

## 8. Recommended closure order

1. **Make the wrong production bootstrap unrepresentable.** Cut over content → runtime → TUI and remove scheduler injection/public aggregate initial state.
2. **Close the command boundary.** Replace raw methods/boolean policy/sequence allocation with typed commands and sealed receipts for wait, non-wait, continue, binding, no-human, rebuild/recovery, view, and debug.
3. **Internalize all client orchestration.** Remove raw aggregate getters and TUI-composed view/no-human/rebuild/perception/checksum transactions.
4. **Implement replay-critical runtime authority reconstruction.** Preserve proposal, actor opportunity, process declaration/cadence/source/random state and fail closed.
5. **Replace the diagnostic process marker with a real process transaction or honestly demote it.** Do not count `WorldNoOp` as applied.
6. **Complete the actor disposition census.** One runtime-derived disposition per loaded actor per step.
7. **Split embodied and debug temporal products.** Eliminate exact time/control metadata and public feature-gated forgeability from embodied APIs; repair normal `continue` output.
8. **Strengthen the production-boundary behavior corpus and external negative fixtures.** Make every closure witness non-vacuous and mutation-sensitive.
9. **Dispose all 23 historical survivors and run focused campaigns.** Treat food-source as routed-forward but still canonical.
10. **Run the complete canonical standing campaign/reconciliation to green.** Zero misses/timeouts; exact fingerprints; no missing shards/census.
11. **Install/verify required CI and repository-governance checks.** Public-boundary conformance and required full-surface mutation triggers.
12. **Only then truth live docs and produce acceptance evidence.** Preserve all archived artifacts and route the numbered spec through the normal ledger process.

This order matters. Documentation and source alarms cannot compensate for an open public API, and focused mutation cannot certify a production path that bypasses the code under test.

---

## 9. Open maintainer decisions

These are implementation choices inside settled doctrine, not questions that block this report:

1. **Opaque bootstrap shape:** whether content returns a `LoadedWorldPackage`, a core constructor token, or a crate-private conversion module. The settled property is that the TUI cannot inject scheduler authority.
2. **Command granularity:** one command enum versus opaque command constructors/facade methods. The settled property is one core dispatcher with no client transaction-policy boolean.
3. **Time consumption for non-wait actions:** action-specific semantics may remain; the settled property is that core owns the rule and event choreography.
4. **Replay authority representation:** event-derived scheduler projection, ancestry-preserving snapshot, or a combination. The settled property is exact continuation and fail-closed missing authority.
5. **Minimal real declared process:** which existing content/process effect best supplies a bounded causal witness. The settled property is process-specific effect and ancestry, not a generic marker.
6. **Actor disposition taxonomy:** exact variant names and whether some are debug-only. The settled property is one closed disposition per loaded actor without hidden-truth leakage.
7. **Embodied stop vocabulary:** qualitative actor-legible labels and provenance. The settled property is no exact hidden tick/frontier/control reason unless modeled as holder-known.
8. **Full mutation trigger policy:** every PR versus path-sensitive full runs plus merge queue. The settled property is that witness-only/config/CI changes cannot weaken old mutation sensitivity and still merge.
9. **Repository governance recording:** where branch-protection required-check confirmation is captured. It must be operational evidence, not a new gate identifier.

---

## 10. Self-check

- [x] The verdict is explicit and evidence-based for both foundational conformance and higher-tier amendment.
- [x] Every target-state claim uses a manifest-listed file fetched by full exact-commit URL; Appendix A contains the complete ledger.
- [x] Third-pass F-01…F-09 were treated as pre-remediation baseline; genuine repairs are marked **present**; live and newly exposed defects were re-derived.
- [x] Every finding names controlling invariants, code home, live-doc home, strongest practical guard, and evidence-honesty failure mode.
- [x] All 23 historical standing misses are explicitly disposed; the seven `food_source` misses remain routed-forward cross-cutting; no survivor is called equivalent without a semantic argument.
- [x] The report designs an enforced standing barrier: compile-time unrepresentability, public production-boundary CI, and a required green canonical mutation perimeter.
- [x] Recommendations extend `.cargo/mutants.toml`, CI, `anti_regression_guards.rs`, external negative fixtures, `negative_fixture_runner.rs`, `generative_lock.rs`, coordinator/replay/interval/salience/reservation/parity suites; no new property-testing dependency or compatibility shim is recommended.
- [x] No archived artifact is proposed for editing; no invariant, gate, risk, glossary, or spec identifier is minted.
- [x] Static-survey limits are explicit; historical command outcomes remain attributed to the 0051 acceptance.
- [x] Repository evidence and external research are separate lanes.
- [x] This is one new analysis/recommendation report, not a numbered spec.

---

## 11. References

### 11.1 User-supplied control material

- `0047-foundational-hardening-research-brief-fourth-pass.md` — assignment, scope, authority order, static-survey constraint, survivor disposition requirements, and deliverable shape.
- `manifest_2026-06-25_6495d7d.txt` — user-supplied path inventory for the stated commit; not file-content evidence.
- Exact-commit provenance guardrail in the invoking prompt — acquisition and evidence-lane discipline.

### 11.2 Target-repository evidence

All repository references below were fetched from the exact commit and appear in Appendix A. Load-bearing evidence includes:

- foundation: constitutional invariants; causal replay contract; TUI possession/view/debug doctrine; actor-known cognition transaction/truth firewall;
- architecture: conformance index; replay; holder-known/provenance; action/scheduler pipeline; actor transaction; TUI/client boundary; validation/evidence;
- execution: phase ladder; transaction scheduler/no-direct-dispatch; no-human proof; view/debug proof; testing/evidence doctrine;
- reference: reviewer index and R-27/R-28/R-29;
- lineage: third-pass report, specs 0047/0050/0051, original/0050/0051 acceptance, spec ledger;
- production code: content loader, runtime session, scheduler, events, actor transaction/trace/perception, projections/view models, replay, pipeline, TUI app/run/render/transcript;
- enforcement: mutation configuration/baseline, CI workflow, shard merge tooling, negative fixtures/runner, generative/coordinator/replay/interval/salience/reservation/no-human/parity/TUI suites.

### 11.3 External research — not target-repository evidence

- Rust Reference, [Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html).
- Cargo Reference, [Features](https://doc.rust-lang.org/cargo/reference/features.html).
- Rust Reference, [`non_exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
- Fred B. Schneider, [Implementing Fault-Tolerant Services Using the State Machine Approach](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf).
- Temporal, [Workflows](https://docs.temporal.io/workflows) and [Workflow tasks](https://docs.temporal.io/tasks).
- Andrei Sabelfeld and Andrew C. Myers, [Language-Based Information-Flow Security](https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf).
- cargo-mutants, [Exit codes](https://mutants.rs/exit-codes.html), [Continuous integration](https://mutants.rs/ci.html), [Mutants in changed code](https://mutants.rs/in-diff.html), [Baseline](https://mutants.rs/baseline.html), and [Configuration](https://mutants.rs/config-file.html).
- Yue Jia and Mark Harman, [An Analysis and Survey of the Development of Mutation Testing](https://web.eecs.umich.edu/~weimerw/2022-481F/readings/mutation-testing.pdf).

---

## Appendix A — Complete append-only exact-commit acquisition ledger

Every URL below was constructed mechanically as `base + manifest path` and verified as target-repository acquisition evidence.

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/00_FOUNDATION_INDEX.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/01_PROJECT_CHARTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/docs/4-specs/SPEC_LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/reports/0047-foundational-hardening-research-report-third-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/reports/0047-foundational-hardening-research-brief-third-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/reports/0047_tui_authoritative_world_advance_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/reports/0048_foundational_conformance_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_LOADED_WORLD_DISCOVERY_ACTOR_TRANSACTION_UNIFICATION_TUI_DEAUTHORITY_AND_REPLAY_FAIL_CLOSED_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_PRODUCTION_REACHABILITY_PROCESS_TRANSACTIONS_ACTOR_CENSUS_AND_TUI_DEAUTHORITY_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/.cargo/mutants.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/.cargo/mutants-baseline-misses.txt`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/.github/workflows/ci.yml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/runtime/session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/scheduler.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/events/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/events/envelope.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/events/apply.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/events/log.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/events/mutation.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/agent/transaction.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/agent/trace.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/agent/perception.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/agent/decision.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/agent/no_human_surface.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/epistemics/projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/epistemics/observation.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/epistemics/knowledge_context.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/projections.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/view_models.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/replay/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/replay/temporal.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/replay/rebuild.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/replay/report.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/actions/pipeline.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/actions/proposal.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/actions/report.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/actions/registry.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/controller.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/time.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/src/need_accounting.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/src/load.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/app.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/render.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/transcript.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/launch.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/input.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/src/debug_panels.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/world_step_coordinator.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/negative_fixture_runner.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/anti_regression_guards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/generative_lock.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/holder_known_interval_projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/replay_temporal_frontier.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/salient_stop_actor_known.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/reservation_body_exclusive_census.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/no_human_capstone.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/spine_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/acceptance_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/mutation_completion_merge.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/ci_workflow_guards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/hidden_truth_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/golden_scenarios.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/support/generative.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/tests/support/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/command_loop_session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/tui_seam_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/transcript_snapshot.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/embodied_flow.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/tui_acceptance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/adversarial_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity_adversarial.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/playable_capability_parity.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity/runner.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity/scenario.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity/census_actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/tests/parity/census_families.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/tests/fixtures_load.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-content/tests/golden_fixtures_run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_assign_scheduler_frontier/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_call_core_perception_append_helper/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_call_scheduler_perception_writer/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_construct_pipeline_context_with_runtime_aggregates/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_forge_interval_notice/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_mutate_embodied_temporal_fields/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_mutate_loaded_runtime_fields/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_name_due_process_invocation/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_reduce_actor_step_outcome_to_option/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_set_world_step_due_actor_ids/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_set_world_step_process_events/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-core/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/crates/tracewake-tui/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tools/merge-mutation-shards.py`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_mutate_embodied_temporal_fields/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/6495d7dfe7d2d8887d4bb2ce583074c87fb273e8/tests/negative-fixtures/external_crate_cannot_mutate_loaded_runtime_fields/Cargo.toml`
