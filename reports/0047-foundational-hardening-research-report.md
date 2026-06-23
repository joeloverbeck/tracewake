# Spec-0047 foundational-conformance hardening and anti-regression report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `cb3102ef1993a88b2516cec77a145e97a2baae85` (`cb3102e`)  
**Survey mode:** static source and evidence review only; no build, test, replay, or mutation command was executed  
**Freshness scope:** user-supplied target commit only; this report does not independently verify current `main`  
**Historical 0047 implementation/evidence commit:** `4228e1e2e5efd759e7e7bddb939a599e344742e9`; provenance only, not the code baseline analyzed here

## 1. Verdict

**Divergences found; foundational remediation and substantial anti-regression hardening are warranted.**

At `cb3102e`, the 0047 surface does not satisfy the repository's already-ratified loaded-world, temporal-replay, and holder-known information-flow contracts. The most consequential failures are structural rather than cosmetic: the function named as the authoritative one-tick coordinator does not run ordinary actor transactions or world processes; the TUI commits a possessed actor action before asking the coordinator to validate and finish the tick; the scheduler frontier can be changed outside the coordinator; replay does not reconstruct that frontier from `TimeAdvanced`; and the actor-known interval surface is assembled by filtering raw global-log events through publicly forgeable strings and identifiers rather than by projecting a sealed, source-bearing holder-known delta. The real `advance_until` path also cannot currently reach its actor-known salient-observation stop branch.

Several adjacent 0047 properties are currently implemented correctly and should be preserved rather than redesigned: open durations are discovered from event-log start/terminal evidence; the `TimeAdvanced` envelope is cause-bearing, world-stream, and replay-significant; need accounting is centralized around counted `(actor, need, tick)` classifications; and the body-exclusive reservation predicate is general rather than sleep/wait-specific. Those holdings already have meaningful tests and mutation scope. The recommendations below therefore target the missing delta instead of re-commissioning existing work.

Because at least one foundational violation and multiple warranted hardening gaps exist, this is the required positive-mode downloadable report.

## 2. Disposition table

| Finding | Primary target | Severity | One-line basis |
|---|---|---:|---|
| The canonical one-tick coordinator omits ordinary loaded-world actor and process phases | `crates/tracewake-core/src/scheduler.rs` and the core transaction/pipeline collaborators | **violation (critical)** | `advance_world_one_tick` hard-codes zero actor transactions and zero world processes, contradicting `INV-005`, `INV-006`, `INV-091`, `INV-094`, `INV-108`, `INV-112`, foundation `08`, architecture `04`, and execution `05`/`06`. |
| TUI wait is committed before the authoritative tick, so the combined transition is not atomic | `crates/tracewake-tui/src/app.rs`; core world-step request/commit API | **violation (critical)** | `submit_entry_with_world_advance` mutates state/log through `run_pipeline` and only then calls the coordinator; later failure leaves the first commit in place and violates `INV-001`, `INV-009`, `INV-010`, `INV-018`, `INV-092`, `INV-103`, `INV-104`, and `INV-112`. |
| Temporal frontier authority is publicly writable and can advance from arbitrary appended-event timestamps | `crates/tracewake-core/src/scheduler.rs`; `crates/tracewake-tui/src/app.rs`; replay restoration boundary | **violation (critical)** | Public `current_tick`, TUI assignments, and `sync_scheduler_frontier_to_appended_events` create alternate temporal authorities and can bypass one-marker-per-tick ancestry, contrary to `INV-009`, `INV-010`, `INV-018`, `INV-092`, and `INV-112`. |
| Actor-known salient interruption is not reachable through the production world-step path | `scheduler.rs`, perception/epistemic projection, `TuiApp::advance_until` | **violation (high)** | The stop helper recognizes `ObservationRecorded`, but the canonical step emits no perception/epistemic events and the TUI records perception only after the loop has stopped; this conflicts with `INV-067`, `INV-099`, `INV-101`, `INV-102`, and `INV-112`. |
| Interval summaries are raw-log redaction with forgeable source and prose fields, not sealed holder-known deltas | `crates/tracewake-core/src/projections.rs`, `view_models.rs`, `crates/tracewake-tui/src/app.rs` | **violation (critical)** | Public `ActorKnownIntervalSource` fields and a TUI global-log scan violate positive construction and provenance requirements under `INV-067`, `INV-099`, `INV-101`, `INV-102`, and `INV-112`. |
| Replay does not derive or validate the temporal frontier from `TimeAdvanced` | `crates/tracewake-core/src/replay/rebuild.rs`, `replay/report.rs`, event application/temporal projection | **violation (critical)** | `TimeAdvanced` is a world no-op during rebuild, rebuild reports have no temporal frontier, and the named empty-tick test supplies the final tick externally; this violates `INV-018`, `INV-092`, and `INV-112`. |
| The human/no-human and parity evidence overstates what the executable witnesses prove | `world_step_coordinator.rs`; TUI parity scenario/runner and current live evidence mappings | **hardening-gap (high)** | The differential supplies no autonomous proposals and expects zero ordinary events; parity “typed witnesses” are partly declaration strings. This triggers the evidence-honesty doctrine and existing risks R-27 and R-29, and leaves `INV-091`, `INV-094`, and `INV-108` weakly protected. |

## 3. Method and provenance ledger

### 3.1 Authority and scope

The audit applied the repository's documented authority order:

1. `docs/0-foundation/*`;
2. `docs/1-architecture/*`;
3. `docs/2-execution/*`;
4. `docs/3-reference/*`;
5. `docs/4-specs/*`;
6. archived 0047/0046 artifacts as historical requirements and evidence claims;
7. implementation and tests at the exact target commit.

The entire live `0-foundation`, `1-architecture`, `2-execution`, `3-reference`, and `4-specs` tiers were acquired and read. The substantive code audit remained within the 0047-touched seams and their named conformance collaborators. Two extra manifest paths—`crates/tracewake-core/tests/generative_lock.rs` and `crates/tracewake-core/tests/support/generative.rs`—were acquired to evaluate the repository's existing deterministic generative-testing posture. No out-of-surface product finding is asserted.

The archived 0047 spec and acceptance report were used to identify promised properties and named witnesses. Their `4228e1e` pin was not treated as the code under review, and this audit does not infer what code existed at that historical commit. The implementation findings below are claims only about `cb3102e`.

### 3.2 Mandatory repository-acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: cb3102ef1993a88b2516cec77a145e97a2baae85
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact URL); container.download(full exact URL); ancillary full-URL curl retries
Requested unique file count: 101
Successfully verified unique file count: 101
Append-only exact-URL transport attempts: 117
Verified/usable transport attempts: 114
Redundant ancillary attempts not used as evidence: 3
Fetched repository files: complete requested/resolved URL history in companion ledger
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

The three non-evidence ancillary attempts were two DNS failures and one local MIME-save rejection after the affected paths had already been fetched successfully through verified exact URLs. They did not create an acquisition gap. The complete append-only URL history is in [`0047-exact-commit-final-ledger.txt`](0047-exact-commit-final-ledger.txt).

### 3.3 Existing anti-regression machinery accounted for

This report does **not** recommend merely adding infrastructure that already exists:

- `.cargo/mutants.toml` already includes `scheduler.rs`, `need_accounting.rs`, `projections.rs`, `actions/pipeline.rs`, `events/**`, `replay/**`, `view_models.rs`, `tracewake-tui/src/app.rs`, and `render.rs` in the standing mutation perimeter.
- `crates/tracewake-core/tests/anti_regression_guards.rs` already performs broad `include_str!`-based topology and source checks, including the scheduler, pipeline, event/replay, projection, view-model, TUI app/render, generative support, and mutation configuration.
- The repository already has external-crate negative fixtures for unforgeable/private boundaries; recommendations below extend that established pattern instead of adding a second compile-fail framework.
- `generative_lock.rs` already runs a deterministic recorded-seed corpus, checks replay/tamper properties, records reachability and omitted population, and checks duplicate need-charge keys. It explicitly does not claim exhaustive generation or shrinking.
- The 0046 parity registry and scenario runner execute real fixture/TUI operations and render checks. Their existence is valuable; the gap is that some evidence fields remain declaration metadata rather than measured typed outcomes.

Accordingly, the mutation recommendation is “add behavior that kills relevant mutants and run focused campaigns,” not “add cargo-mutants.” Source scans remain secondary topology alarms, never substitutes for behavior witnesses.

### 3.4 Static-survey limitation

The brief forbids execution. Every pass/fail statement about current tests is therefore a preliminary static reading of test intent and source shape, not an authoritative command result. The implementing session must run the ordinary workspace gates, replay/golden lanes, focused tests, and configured mutation campaigns and must retain their exact transcripts or deterministic reports. No finding below is premised on an assumed green or red command.

## 4. Findings and required hardening

### 4.1 The canonical one-tick coordinator is not a loaded-world step

#### Foundational driver

Foundation `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` states that waiting runs the simulation: one possessed actor supplies one input slot, while other loaded actors, world processes, and due consequences advance in one shared transition. Architecture `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` assigns deterministic due-work ordering, ordinary proposal routing for human and autonomous actors, and due consequences to one core world-step coordinator. Execution `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` makes ordinary actor/process transactions and due world processes explicit phases. Execution `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` rejects a human/no-human proof that exercises only a private possessed-actor tick.

This finding implicates `INV-005`, `INV-006`, `INV-091`, `INV-094`, `INV-108`, and `INV-112`; `INV-001` is also implicated because a control labeled as world advance is not running the modeled causes the doctrine assigns to that transition.

#### Current code state at `cb3102e`

`DeterministicScheduler::advance_world_one_tick`:

- validates `WorldAdvanceRequest.expected_tick`;
- computes `prior_tick` and `resulting_tick`;
- discovers due body-exclusive durations from the log;
- builds and appends `TimeAdvanced`;
- builds and appends duration lifecycle and need-accounting events;
- updates `current_tick`;
- returns `WorldStepDueWorkSummary` with `actor_transactions_attempted: 0` and `world_processes_applied: 0` unconditionally.

Its physical-state input is immutable. The function neither invokes the ordinary `ActorDecisionTransaction`/candidate/planner/proposal route nor runs a world-process registry or cadence. `DeterministicScheduler::advance_until` simply loops this same reduced operation. The names and result fields imply a broader semantic boundary than the implementation supplies.

#### Conformance verdict

**Foundational violation.** This is a duration/accounting tick coordinator, not the canonical loaded-world step ratified by the controlling documents. Human wait therefore cannot prove that an unpossessed actor receives its ordinary due opportunity or that a due loaded-world process advances through the same seam. The hard-coded zero counters make the omission explicit rather than merely untested.

#### Required remediation: substance and home

The core must own a single typed one-tick transaction that receives all inputs needed for that tick, including:

- the optional controlled-actor boundary input/proposal slot;
- deterministic autonomous-actor opportunities due at that tick;
- deterministic world-process work due at that tick;
- open-duration lifecycle work;
- one shared need-accounting reconciliation;
- perception/epistemic projection work needed for typed stop evidence;
- the temporal marker and all causal ancestry.

The implementation may stage an event batch against cloned/scratch state or use an explicit prepare/commit transaction, but it must expose one commit boundary. The scheduler may select cadence and invoke actor decision transactions under `INV-103`; it must not synthesize cognition from raw truth. Existing proposal validation and event application remain the owning seams.

Primary homes are `crates/tracewake-core/src/scheduler.rs`, the existing agent transaction/decision APIs, `actions/pipeline.rs`, and whatever existing process registry/cadence abstraction already owns loaded-world process work. `tracewake-tui` must not implement any of these phases.

#### Strongest practical guard

Add a deterministic **held-equal human/no-human differential** in `crates/tracewake-core/tests/world_step_coordinator.rs` with all of the following simultaneously live:

- a possessed actor whose human input is one-tick wait;
- at least one unpossessed actor with a due ordinary decision opportunity that commits a typed event;
- an open sleep or work duration due to terminate;
- passive need accounting for multiple actors;
- a due world process that commits a typed event;
- replay from the same initial state and event log.

Compare the typed event multiset/order (allowing only declared controller-origin differences), final physical state, final agent state, temporal frontier, projection frontiers, and checksums. Assert measured nonzero actor/process counts. A test whose autonomous proposal list is empty or whose expected ordinary event count is zero is not an acceptable witness.

Extend the existing deterministic generated-sequence harness with world-step schedules that vary actor opportunities, duration boundaries, and process due times. Record seeds and reachability as the repository already does. No new property-testing crate is justified yet: the current deterministic harness can express the state-machine property and already preserves reproducible seeds. Reconsider `proptest` only if shrinking failing schedules becomes a demonstrated maintenance problem.

Run focused `cargo-mutants` campaigns over the rewritten coordinator, cadence invocation, and result counters. The config already includes the files; the delta is tests that kill omission, zeroing, wrong-order, and skipped-phase mutants.

#### Evidence-honesty check

The differential must fail when the coordinator omits the unpossessed actor or process phase. A count asserted against another literal or a capability row containing a descriptive string would be decorative evidence and would violate execution `10`.

### 4.2 TUI wait and authoritative advancement are two commits, not one transaction

#### Foundational driver

Architecture `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` says the TUI asks core to perform a typed world step and never applies events, mutates state, owns duration terminals, or maintains a local clock. Execution `05` defines one phase-ordered core transaction. The event/replay doctrine requires immutable committed history, causal ordering, and deterministic rebuild.

This finding implicates `INV-001`, `INV-009`, `INV-010`, `INV-018`, `INV-092`, `INV-103`, `INV-104`, and `INV-112`.

#### Current code state at `cb3102e`

`TuiApp::submit_entry_with_world_advance` captures the current scheduler tick, constructs the possessed actor proposal, creates a mutable `PipelineContext`, and calls `run_pipeline`. That call may append events and mutate physical, agent, and epistemic state. Only after an accepted result does the TUI call `DeterministicScheduler::advance_world_one_tick` when the action is `wait.1_tick`.

If the subsequent coordinator call returns an error—such as malformed log-derived duration evidence or a later append/application failure—the `?` returns from the TUI method without rolling back the already-accepted action. The actor wait and any action-emitted need effects can therefore survive without the authoritative tick transition that was supposed to contain them.

The ordering is also inverted relative to execution `05`: the wait pipeline can append resulting-tick events before the tick-boundary marker is appended. The marker does not form the shared transaction boundary for those earlier events.

#### Conformance verdict

**Foundational violation.** A user-visible one-tick command is implemented as “commit actor action, then attempt world step.” That is neither one authoritative transition nor fail-closed atomic behavior.

#### Required remediation: substance and home

For world-advancing controls, the TUI must submit one typed request to core containing the controlled-actor input slot and expected frontier. Core must validate and prepare the complete step before committing any event or state mutation. On any validation, duration, accounting, process, append, application, projection, or replay-precondition failure, the pre-step physical state, agent state, epistemic projection, log, and temporal frontier must remain unchanged.

The ordinary actor proposal still travels through the existing proposal/validation/pipeline route; the change is that this route is invoked inside the core-owned one-tick transaction rather than committed by TUI choreography. Continuation/advance-until controls remain controller operations and must not be disguised as a second `wait` action.

Primary homes are the core world-step request/result API and `TuiApp::submit_entry_with_world_advance`. The TUI method should collapse to request construction plus result handling. No compatibility wrapper preserving the two-step choreography should remain.

#### Strongest practical guard

Add an all-or-nothing failure test that:

1. supplies a valid possessed wait input;
2. seeds malformed or contradictory due-duration evidence that makes the core step reject after it has inspected the controlled input;
3. snapshots physical state, agent state, epistemic projection, event log, scheduler frontier, and relevant checksums before the call;
4. asserts every snapshot is byte/structurally identical afterward.

Add a success-order witness asserting exactly one `TimeAdvanced` boundary for the increment and that every same-step resulting-tick effect has valid ordering ancestry through the prepared step. Add a mutation target that removes the prepare/commit barrier or commits the controlled proposal early.

A source guard may ban the old `run_pipeline`-then-`advance_world_one_tick` choreography as a secondary alarm, but the transactional failure test is the primary evidence because it exercises the forbidden behavior.

#### Evidence-honesty check

A test that checks only the returned error is insufficient. The protected property is absence of partial commit, so every mutable authority surface must be compared.

### 4.3 The temporal frontier has multiple writers and can advance without tick ancestry

#### Foundational driver

`INV-112` assigns authoritative ordering to scheduler/replay time without making it client authority. `INV-009`, `INV-010`, `INV-018`, and `INV-092` require meaningful temporal changes and their causes to be event/replay visible. Foundation `03` and architecture `02` require even an empty accepted step to carry ancestry sufficient to rebuild the temporal frontier.

#### Current code state at `cb3102e`

`DeterministicScheduler` exposes `pub current_tick`. Current writers include:

- `advance_world_one_tick`, the intended authority;
- `TuiApp::submit_entry_with_world_advance`, which assigns the tick from the last appended non-wait action event;
- `TuiApp::run_no_human_day`, which assigns `report.final_tick`;
- `scheduler::no_human::sync_scheduler_frontier_to_appended_events`, which sets the frontier to the maximum `sim_tick` among arbitrary newly appended events.

The no-human runner executes scheduled proposals separately from the coordinator and then calls the sync helper. An action such as wait can append an event at the next tick; the helper can move the frontier to that timestamp even though no `TimeAdvanced` marker was emitted for that increment. The post-acceptance mutation hardening around `max` protects the helper's comparison behavior, but it protects an alternate temporal-authority path rather than eliminating it.

#### Conformance verdict

**Foundational violation.** The code has a canonical temporal writer in name only. Event timestamps are being used as commands to move the scheduler frontier, and TUI/debug paths can assign it directly.

#### Required remediation: substance and home

Make the scheduler frontier private. Expose a read-only accessor. Runtime advancement must occur only through the canonical one-tick commit. Replay/load restoration may set an initial frontier only through a distinct typed restoration constructor that verifies temporal projection evidence; it must not share the ordinary advancement setter.

Delete `sync_scheduler_frontier_to_appended_events` as an advancement mechanism. Scheduled no-human proposals become inputs to the same one-tick transaction, so their event timestamps cannot move time independently. Delete TUI assignments. Debug no-human completion returns evidence; it does not grant the TUI a setter.

#### Strongest practical guard

Extend the existing external-crate negative-fixture pattern with a fixture proving downstream code cannot assign the scheduler frontier or construct a runtime frontier-restoration token. This makes the most dangerous regression fail at compile time.

Add runtime temporal-chain properties:

- from an initial frontier and accepted event log, every increment has exactly one `TimeAdvanced`;
- each marker payload has `prior_tick == reconstructed_frontier` and `resulting_tick == prior_tick + 1`;
- marker envelope tick, payload ticks, cause model, and stream ordering agree;
- no ordinary event timestamp alone changes the reconstructed frontier;
- a no-human scheduled wait cannot skip the marker;
- acceleration of `N` ticks yields `N` chained markers.

Target mutations at marker omission, duplicate markers, `+1` arithmetic, payload swaps, and bypass of the canonical setter.

#### Evidence-honesty check

A source assertion that the field is private is useful but not enough. The runtime marker-chain test must prove that the remaining legal API cannot create a markerless increment.

### 4.4 The actor-known salient-observation stop branch is not production-reachable

#### Foundational driver

Foundation `08` includes stop-on-salient-perceived-interruption among staged embodied time controls and requires actor-filtered summaries. Architecture `02` requires stop decisions to replay from typed step evidence and holder-known salience inputs. Architecture `03` and execution `04`/`07` require source-bearing holder-known construction, not raw truth events.

This finding implicates `INV-067`, `INV-099`, `INV-101`, `INV-102`, and `INV-112`.

#### Current code state at `cb3102e`

`DeterministicScheduler::advance_until` checks `step_appended_actor_known_salient_observation` after each call to `advance_world_one_tick`. The helper returns true only when one of that step's appended IDs resolves to `ObservationRecorded` for the possessed actor.

The current world step appends temporal, duration-terminal, and accounting events; it does not execute perception or epistemic projection. `TuiApp::advance_until` builds the interval summary immediately after the scheduler loop returns, then calls `record_current_place_perception_and_project`. That perception is therefore too late to stop the loop or appear in the just-built interval summary.

The scheduler unit test for the helper inserts an `ObservationRecorded` event directly and proves only the predicate's actor-ID comparison. It does not make the production route emit the event.

#### Conformance verdict

**Foundational violation.** The public stop reason exists, but the actual control cannot reach it through modeled perception. The named behavior is implemented as an isolated predicate, not a production capability.

#### Required remediation: substance and home

Perception and epistemic projection necessary for a time-control stop decision must occur inside the canonical step's typed phase contract, after authoritative effects and before stop evaluation. The step result should carry a typed holder-known delta or salience decision derived from the sealed actor-known context. `advance_until` consumes that typed evidence; it must not scan the global log for an `ObservationRecorded` row and infer actor knowledge from `actor_id` alone.

The salience policy must remain deterministic and actor-known. Exact hidden world events, raw due queues, or debug comparisons cannot stop embodied progression unless a modeled channel produces an allowed holder-known delta.

#### Strongest practical guard

Create a real fixture/path-under-test pair:

- positive world: at tick `N`, a modeled source produces an observation available to the possessed actor; `advance_until` stops exactly at `N`, returns the typed actor-known stop reason, and the summary cites the source;
- hidden-world variant: the same hidden event occurs for another actor or without an acquisition channel; the possessed actor's stop tick and summary remain unchanged;
- replay: rebuilding from the log reproduces both the temporal frontier and the stop evidence.

Mutate the actor/context match, source membership, and salience predicate. The positive and hidden variants must kill opposite classes of leakage/omission mutants.

#### Evidence-honesty check

Directly calling the private helper with a fabricated event is a unit negative, not acceptance evidence. The passing witness must run the same core/TUI path used by the actual control.

### 4.5 The interval summary violates the positive-construction boundary

#### Foundational driver

`INV-067` confines embodied mode to actor-known reality. `INV-099` forbids hidden truth from planning or embodied selection. `INV-101` requires a sealed actor-known context, and `INV-102` requires cognition/view-relevant provenance. `INV-112` applies the same rule to temporal facts.

Architecture `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` is unambiguous: actor-known interval summaries are positively constructed holder-known frontier deltas; they may not be generated by taking a raw world-event diff and redacting forbidden rows. Architecture `10` requires actor-known summaries and debug step reports to be separate products.

#### Current code state at `cb3102e`

The current API exposes:

- `ActorKnownIntervalSource` with public `actor_id`, `source_event_id`, and arbitrary `summary: String` fields;
- `build_actor_known_interval_summary` accepting any caller-provided stop-reason string and any iterable of those sources;
- `ActorKnownIntervalNotice` and `ActorKnownIntervalSummary` with public display strings.

The builder filters only on `source.actor_id == viewer_actor_id`, moves the arbitrary summary text into the embodied notice, stringifies the event ID, and labels an empty list as no new actor-known information. It does not accept or validate a `KnowledgeContext`, epistemic projection frontier, allowed source set, or provenance witness.

`TuiApp::actor_known_interval_sources` scans `EventLog` for the step's appended IDs, reads raw events, maps selected event kinds to prose, and supplies the public source objects. That is the raw-global-diff/redaction architecture the controlling document forbids. It also runs before resumption perception is projected.

The current hidden-other-actor adversarial test constructs a synthetic source directly and proves that actor-ID filtering hides it. It does not prove that an included source was holder-known, that its event ID witnesses the represented fact kind, or that prose could not be forged. No direct debug report is presently observed entering the embodied renderer; the failure is the positive side of the firewall and the public API shape.

#### Conformance verdict

**Foundational violation.** The output may often be intuitively correct for own sleep/work terminals, but the authority path is wrong and structurally permissive. A raw event plus matching actor ID is not a sealed holder-known delta, and a `String` is not provenance.

#### Required remediation: substance and home

Move interval-delta construction into the core epistemic/projection boundary. The builder must consume a sealed before/after holder-known frontier or a typed delta produced by the epistemic projection. Each output item must be derived from a closed fact/notice kind and retain a typed source reference whose holder visibility and fact-kind compatibility were verified.

Required API properties:

- external crates and the TUI cannot construct interval-source records directly;
- the viewer identity is bound by the sealed context, not repeated as a caller-controlled field;
- notice meaning and stop reason are closed enums/typed values, not arbitrary prose;
- rendering to strings happens only at the TUI boundary;
- “no new actor-known information” means the verified holder-known delta is empty, not that a raw global diff became empty after filtering;
- resumption perception is part of the interval projection before the final summary is returned;
- the exact debug step report remains a separate type with no conversion into the embodied summary.

Primary homes are `crates/tracewake-core/src/epistemics/projection.rs`, `crates/tracewake-core/src/projections.rs`, `view_models.rs`, and `TuiApp::advance_until`. Delete the TUI raw-log source builder rather than keeping it as a compatibility path.

#### Strongest practical guard

Use three complementary guards:

1. **Compile-time boundary:** extend external-crate negative fixtures to prove callers cannot construct source/notice internals, supply arbitrary summary prose, or convert a debug step report into the embodied interval type.
2. **Paired-world observational equivalence:** run two worlds with identical possessed-actor holder-known inputs and different hidden events. Their typed interval summaries and rendered embodied output must be identical. Add one modeled source-bearing observation to only one holder-known context and assert that exactly the permitted notice changes.
3. **Provenance closure:** for every emitted notice, resolve its source through the sealed context/projection and assert source existence, holder visibility, fact-kind compatibility, and interval-frontier membership. Dangling, wrong-kind, other-holder, and debug-only sources must fail closed.

The paired-world test is the strongest practical truth-firewall witness because it proves hidden-world variation cannot influence the low/embodied observation. The compile-time guard prevents future callers from bypassing the proof by fabricating inputs.

#### Evidence-honesty check

A renderer assertion that a known forbidden string is absent is useful but incomplete. The output must be invariant under hidden-state changes, and every positive output must have a verified source. Both directions are necessary.

### 4.6 Replay does not reconstruct the temporal frontier

#### Foundational driver

`INV-018` and `INV-092` require deterministic replay and automated replay proof. `INV-112` makes event/replay time authoritative for ordering. Foundation `03` and architecture `02` specifically require every accepted world step, including an empty step, to carry enough boundary ancestry to rebuild the temporal frontier.

#### Current code state at `cb3102e`

`ProjectionRebuildReport` contains physical, agent, epistemic, checksum, event-count, and stream-position outputs, but no reconstructed temporal frontier or temporal-divergence report. `rebuild_projection` receives a `ChecksumContext` whose `sim_tick` is supplied by the caller. In event application, `TimeAdvanced` is treated as a world no-op; no temporal projection consumes its prior/result payload or validates a chain.

The test named `empty_world_step_appends_time_advanced_and_rebuilds_frontier` calls `rebuild_projection` with `checksum_context(SimTick::new(42), 1)`. It verifies that one event was applied and that physical/agent state remained equal. It does not derive tick 42 from the event or compare a reconstructed frontier to the live frontier. The test name and archived acceptance claim are therefore stronger than the executable assertion.

The general generative replay helpers similarly construct checksum contexts with a known final tick from the live run. That is valid for physical/agent replay checks, but it is not temporal-frontier reconstruction.

#### Conformance verdict

**Foundational violation.** `TimeAdvanced` is emitted but not authoritative during replay. The current frontier remains an out-of-band caller input to rebuild/checksum rather than a replay projection.

#### Required remediation: substance and home

Introduce a replay-owned temporal projection that starts from an explicit initial frontier and consumes ordered `TimeAdvanced` events. For every marker it must validate:

- supported schema and event kind;
- required cause model and ordering ancestry;
- payload `prior_tick` equals the projection's current frontier;
- payload `resulting_tick` is exactly one greater;
- envelope `sim_tick` agrees with the resulting tick;
- no duplicate, gap, backward step, or multi-tick jump;
- marker ordering relative to the step's effects follows the canonical transaction contract.

`ProjectionRebuildReport` and `ReplayReport` must expose the reconstructed final frontier and typed temporal violations. The rebuild API should not accept the expected final tick as an unverified checksum fact. Split static checksum identity inputs from the final temporal context, derive the latter from replay, and then compute final checksums.

#### Strongest practical guard

Add replay tests for:

- a single otherwise-empty tick, supplied only an initial frontier;
- multiple chained ticks and advance-until;
- mid-duration save/rebuild/resume;
- missing marker;
- duplicate marker;
- prior/result mismatch;
- `+2` jump;
- backward marker;
- envelope/payload tick disagreement;
- missing or wrong cause ancestry;
- ordinary future-timestamp event without a marker.

Each malformed case must fail with a typed temporal divergence, not merely produce a different checksum. Add mutations around marker application and chain arithmetic. Include the temporal frontier in human/no-human differential comparisons.

#### Evidence-honesty check

A test that passes the desired final tick into the rebuild context cannot prove rebuild of that tick. The expected value must be withheld from the system under test and used only in the assertion.

### 4.7 Existing differential and parity evidence does not lock the claimed properties

#### Foundational driver

`INV-091`, `INV-094`, and `INV-108` require no-human and possession-parity proof. Execution `10` requires typed-before-rendered evidence, real path-under-test witnesses, live negatives, and explicit limits. Existing risk R-27 covers reachability overstatement; R-29 covers decorative locks.

#### Current code state at `cb3102e`

`differential_human_wait_and_no_human_wait_match_authoritative_outcome` does useful work: it compares duration completion, needs, physical state/checksum, and differing controller/process origins. But the no-human side receives `Vec::new()` scheduled proposals, the test asserts `ordinary_pipeline_events == 0`, and no due world process is present. It therefore proves duration/accounting parity for an unpossessed sleeping actor; it cannot detect omission of autonomous actor transactions or world processes from the canonical step.

The 0046/0047 parity scenario runner executes real fixture/TUI operations and renderer checks. However, `ScenarioWitnesses::ordered_witnesses` echoes the registry's declared assertion strings, and `CoverageRow::typed_witness` is true when the declaration text is non-empty. Replay/no-human statuses are registry labels. Those fields are useful inventory metadata, but they are not measured typed evidence by themselves. The real scenario assertions do not currently establish temporal-frontier reconstruction or holder-known provenance closure.

The archived 0047 acceptance report is immutable historical evidence and must not be edited. Its claims are not automatically current-state proof and do not override a stronger static finding at `cb3102e`.

#### Conformance verdict

**Warranted hardening gap and current evidence overclaim.** The tests prove narrower properties than the labels and acceptance mappings suggest. This does not invalidate the useful duration/accounting and rendering coverage; it requires precise scope and stronger witnesses.

#### Required remediation: substance and home

Replace declaration-as-proof fields for load-bearing 0047 properties with measured scenario outputs. The parity runner may retain human-readable witness descriptions, but pass/fail must derive from structured evidence returned by the real path, such as:

- reconstructed temporal frontier and marker count;
- nonzero autonomous actor/process work where required;
- exact duration terminal and one-charge classifications;
- sealed context/frontier identifiers;
- source IDs that resolve through allowed provenance;
- typed stop reason;
- replay match including temporal projection;
- explicit debug/embodied type disposition.

The stronger differential described in the first finding becomes the authoritative human/no-human witness. Parity scenarios should reference that behavior artifact or reproduce its measured checks; they must not convert an assertion sentence into a boolean “typed witness.”

#### Strongest practical guard

Add adversarial test variants that deliberately:

- omit autonomous actor invocation;
- omit world-process invocation;
- pass a hidden event with the same actor ID but no holder-known source;
- supply a desired final tick externally while removing a marker;
- leave a witness description populated while returning empty typed evidence.

Each variant must fail the actual conformance runner. This directly protects against R-27/R-29 rather than checking that rows or strings exist.

#### Evidence-honesty check

The live report must distinguish “duration/accounting differential,” “loaded-world actor/process differential,” “temporal replay,” and “holder-known noninterference.” One passing test must not be cited as all four unless it actually measures all four.

## 5. Comprehensive 0047 anti-regression layer

The table below covers every load-bearing property required by the brief, including properties that are currently correct.

| Property | Current `cb3102e` state | Already guarded | Delta to implement | Strongest practical mechanism and rationale |
|---|---|---|---|---|
| **Single semantic definition of a loaded-world tick** | **Incorrect/incomplete.** Coordinator handles marker, durations, and accounting but not ordinary actor/process phases. | Focused coordinator tests; duration tests; mutation perimeter includes scheduler. | Core atomic step with controlled input, autonomous opportunities, processes, perception/projection; nonzero differential and generated schedules. | **Typed API plus deterministic differential/state-machine tests.** The API removes alternate ownership; the tests prove real work is not silently omitted. |
| **Actor-known summary vs debug-report firewall** | Debug report types/rendering are structurally separate, but the positive actor-known builder is a raw-log filter with forgeable fields. | Debug-quarantine tests; renderer anti-leak checks; parity hidden-other-actor negative; source census. | Opaque sealed interval delta, typed notice/stop values, provenance closure, no debug conversion, paired hidden-world equivalence. | **Compile-time privacy plus observational-equivalence behavior.** Privacy prevents fabricated inputs; paired worlds prove hidden truth has no embodied influence. |
| **One accounting classification per `(actor, need, tick)`** | **Currently aligned.** Coordinator reconciles counted charges; action-emitted and passive/duration regimes have focused tests. | `need_accounting` authority; `AgentState::need_tick_charges`; wait/sleep/work boundary tests; generated duplicate-key check; mutation scope. | Integrate the same authority into the rebuilt full world-step; add generated mixed human/autonomous/duration schedules and assert exact one classification. | **Existing deterministic generative harness plus focused mutations.** No new library is needed; the relation spans sequences better than more example tests. |
| **Log-derived single source of truth for open durations** | **Currently aligned.** Starts/terminals are scanned from `EventLog`, duplicate/orphan evidence fails closed, and lifecycle resolution uses typed builders. | Prior-start completion tests; malformed/duplicate terminal tests; replay tamper/generative duration coverage; mutation scope. | Add mid-duration save/rebuild/resume and generated malformed lifecycle sequences; ensure no cached queue becomes authoritative. | **Replay/resume properties.** They directly distinguish log authority from process-local memory; source scans are only supplemental. |
| **General body-exclusive reservation enforcement** | **Currently aligned in predicate shape.** Reservation conflict is based on existing open body-exclusive duration rather than only a new sleep proposal. | Human sleep-then-wait rejection; pipeline tests; generated reservation-conflict reachability; mutation scope. | Registry-driven exhaustive test across every ordinary body-using action for human and autonomous origins while sleep/work is open; explicitly allow only typed continuation/lifecycle controls. | **Derived action-registry census plus behavior calls.** It prevents sibling-action omissions without hand-maintained one-off cases. |
| **Replay-visible `TimeAdvanced` ancestry and empty-tick rebuild** | Marker emission/cause classification are present; replay reconstruction is **not**. | Marker payload/cause assertions; empty-step physical/agent rebuild; event/replay mutation scope. | Temporal replay projection, final-frontier report, chain corruption negatives, marker count for acceleration, no external final tick input. | **Replay projection plus malformed-log suite.** This tests authority, not just marker presence. |
| **Human/no-human differential equivalence** | **Underpowered.** Existing test exercises duration/accounting with no autonomous proposals or world process. | Current held-equal initial-state comparison and origin distinction. | Add due unpossessed transaction, due process, perception/projection, temporal replay, and nonzero measured counters. | **Real core differential.** A synthetic or empty actor/process schedule cannot catch the actual relapse. |
| **No direct dispatch or mutation at the TUI boundary** | Ordinary proposals use the shared pipeline, but time-control choreography pre-commits and the TUI writes scheduler frontier. | TUI seam/source guards; command-loop real-pipeline witnesses; parity renderer paths. | One typed core call for world-advancing controls; private frontier; failure atomicity test; remove raw-log interval source construction. | **Unrepresentable API plus compile-fail fixture plus rollback test.** This is stronger than scanning source text for forbidden calls. |
| **Advance-until actor-known stopping** | **Not production-reachable** for salient observations. | Predicate unit test and duration-terminal stop witness. | Perception/projection inside step, typed salience delta, positive/hidden paired fixture, replayed stop result. | **Path-under-test paired fixture.** It proves both reachability and non-leakage. |

### 5.1 Mutation-testing disposition

No `.cargo/mutants.toml` perimeter expansion is currently required for the named 0047 files; they are already included. After remediation, run focused mutation campaigns on:

- canonical step phase invocation and ordering;
- prepare/commit atomicity;
- temporal chain arithmetic and validation;
- private-frontier restoration path;
- holder-known source membership and fact-kind compatibility;
- salience selection;
- one-charge reconciliation under mixed regimes;
- reservation checks across the derived action census.

Treat surviving mutants as signals that the proposed behavior witness is missing or the mutation is equivalent; classify them under the repository's existing mutation evidence process. Do not add text-only mutation expectations.

### 5.2 Property-based/generative-testing decision

**Do not add `proptest` or `quickcheck` in this hardening pass.** The repository already has a deterministic custom generated-sequence harness with recorded seeds, reachability checks, replay/tamper properties, and explicit omitted-population disclosure. Extending that harness minimizes dependency and integration cost and preserves the project's evidence conventions.

The useful properties are state-machine relations, not broad random value generation:

- one marker per accepted increment;
- one charge per `(actor, need, tick)`;
- live/replay equivalence including temporal frontier;
- hidden-world changes do not alter actor-known output;
- duration start/terminal legality across save/resume;
- full-step equivalence across human/no-human origins.

Add deterministic schedule generation and a simple domain-specific shrink/minimization routine only when a failing seed appears. Adopt `proptest` later if repeated failures demonstrate that maintaining shrinking manually is costlier than the dependency. This is a deliberate rejection-for-now, not a claim that property-based testing lacks value.

### 5.3 Compile-time and source-guard disposition

Use compile-time boundaries where Rust can make the regression unrepresentable:

- scheduler frontier private outside core temporal authority;
- interval-source constructors private to the sealed projection module;
- closed typed notice and stop-reason values;
- no conversion from debug step report to embodied summary;
- one core world-step entry point for world-advancing TUI controls.

Extend the repository's existing external-crate negative fixtures for these boundaries. Retain narrow `include_str!` source guards only as topology alarms—for example, banning direct TUI frontier assignment or reintroduction of the deleted raw-log interval builder. A source scan must not be cited as proof of atomicity, replay, or noninterference.

### 5.4 Conformance-row and live evidence disposition

After implementation, update the existing live conformance/evidence homes rather than creating a new certification:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`: keep the correct statement that human world advance consumes the 0017 accounting and open-duration authorities, and add/expand the existing time-control posture so its executable witness homes cover canonical actor/process phases, temporal replay, and sealed interval projection.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`: its current prompts are already correct; no doctrinal rewrite is required. Ensure implementation review artifacts answer them with live evidence.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: update the evidence/status of existing R-08, R-09, R-10, R-11, R-13, R-15, R-16, R-27, R-28, and R-29 as appropriate. Mint no risk ID.
- execution `10`: no doctrine change is needed; the new evidence package must obey its typed-before-rendered and anti-vacuity rules.

Do not amend the archived 0047 spec, its acceptance artifact, 0046 artifacts, or any passed certification. They remain historical, commit-pinned records.

## 6. Foundation and documentation corrections

### Determination

**No higher-tier doctrine correction is warranted.** The live foundation, architecture, execution, and reference documents already state the required behavior at the right authority altitude. In particular:

- foundation `08` says waiting advances the loaded world, not a private actor clock;
- architecture `04` assigns actor routing and due processes to the canonical step;
- architecture `02` requires empty-tick frontier rebuild;
- architecture `03` forbids raw-diff/redaction interval summaries;
- architecture `10` separates TUI request, actor-known summary, and debug report authority;
- execution `05` lists the full phase contract;
- execution `06` defines the required held-equal human/no-human proof;
- execution `07` already contains the corrected staging posture: embodied controls use the ordinary TUI/core boundary and must not reuse debug no-human gameplay/report paths;
- execution `10` already rejects artifact presence and declaration text without live behavior.

The implementation is below doctrine; changing foundation text would weaken a correct contract to fit incorrect code.

### Documentation work that remains in scope

The later implementation closeout should update **evidence references and risk status**, not doctrine:

- point live conformance rows at the new executable temporal/frontier, actor/process differential, and sealed-provenance witnesses;
- accurately scope the old differential as duration/accounting evidence until replaced;
- distinguish registry declaration metadata from measured typed evidence;
- record the static-survey findings and executed remediation evidence in a new implementation/review artifact, not by rewriting archived acceptance history.

No new invariant, gate code, risk ID, glossary term, or ratified paste-ready wording is proposed here.

## 7. Recommended closure order

1. **Make temporal authority singular:** private frontier; replay temporal projection; delete event-timestamp and TUI assignment paths.
2. **Create the atomic full-world step:** controlled input, autonomous actor opportunities, due processes, durations, accounting, perception/projection, one prepare/commit boundary.
3. **Refactor human and no-human callers:** both become inputs/loops over the same one-tick transaction; eliminate sync shortcuts.
4. **Replace interval-summary authority:** sealed holder-known delta, typed notices/stop reason, resumption perception, no TUI global-log scan.
5. **Make salient stopping real:** consume the typed holder-known step delta and add positive/hidden/replay fixtures.
6. **Strengthen evidence:** non-vacuous differential, temporal replay corruption suite, compile-fail boundaries, derived reservation census, generated mixed schedules, measured parity outputs.
7. **Execute and package:** formatting, lint, build, full tests, replay/goldens, focused and configured mutation campaigns, deterministic evidence report, and existing-risk/conformance-reference updates.

This order avoids writing tests around an API that must be replaced and ensures later evidence exercises one final authority path.

## 8. Open maintainer decisions

These are implementation choices inside settled doctrine, not reasons to defer the verdict:

- **Atomic staging strategy:** clone/scratch aggregate state and log, or a prepared event/application batch with explicit commit. Either is acceptable only if the all-or-nothing test proves every authority surface unchanged on failure.
- **Actor/process cadence source:** which existing scheduler/transaction structures enumerate due autonomous opportunities and world processes. The coordinator must invoke the owning abstractions without becoming cognition authority.
- **Typed salience policy:** which holder-known fact kinds stop acceleration and which remain summary-only. The policy must be deterministic, source-bearing, and replayable.
- **Temporal restoration input:** how initial frontier is represented in a save/snapshot without allowing ordinary runtime callers to set it. A verified restoration constructor or snapshot-temporal projection is preferable to a general setter.
- **Rendered temporal labels:** whether exact interval ticks are themselves actor-known in each control context. Debug may always show exact replay time; embodied rendering must follow holder-known temporal provenance.

## 9. References

### Repository authority and evidence

All repository references below were fetched from `joeloverbeck/tracewake` at exact commit `cb3102ef1993a88b2516cec77a145e97a2baae85`; see the companion exact-URL ledger.

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/4-specs/SPEC_LEDGER.md`
- `archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
- `archive/reports/0047_tui_authoritative_world_advance_acceptance.md`
- `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md`
- `archive/reports/0046-parity-acceptance-artifact.md`
- `reports/tui-human-wait-runs-simulation-research-report.md`
- `reports/tui-human-wait-runs-simulation-issue.md`
- `.cargo/mutants.toml`
- the code and test paths listed in the acquisition ledger.

### External research used to choose guard mechanisms

1. **cargo-mutants documentation**, “Welcome to cargo-mutants.” It characterizes mutation testing as finding locations where bugs can be inserted without causing tests to fail. This supports using focused mutants as a test-strength diagnostic, not as a substitute for specifying behavior. `https://mutants.rs/`
2. **The Rust rustdoc book**, “Documentation tests,” including `compile_fail` semantics. This supports compile-time boundary checks where a forbidden external API use must remain uncompilable. The repository's existing external-crate negative-fixture pattern is the preferred local implementation. `https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html`
3. **FoundationDB documentation**, “Simulation and Testing.” It emphasizes deterministic simulation's repeatability and synchronized simulated time. This supports recorded-seed, real-code state-machine schedules for the world-step seam. `https://apple.github.io/foundationdb/testing.html`
4. **TigerBeetle documentation**, “Safety.” Its VOPR description emphasizes running real code in a simulated environment under generated faults. This supports exercising the actual coordinator and replay path rather than a helper-only model. `https://docs.tigerbeetle.com/concepts/safety/`
5. **proptest documentation**, `Strategy`, and Claessen/Hughes's QuickCheck lineage. These establish generation/shrinking as available techniques, but do not justify adding a new dependency when the repository already has a deterministic custom corpus. `https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html`; QuickCheck bibliographic record: `https://doi.org/10.1145/351240.351266`

