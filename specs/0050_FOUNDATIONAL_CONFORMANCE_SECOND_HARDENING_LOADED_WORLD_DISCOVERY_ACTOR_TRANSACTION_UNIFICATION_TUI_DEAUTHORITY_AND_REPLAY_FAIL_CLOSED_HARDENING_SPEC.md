# 0050 Foundational Conformance Second Hardening: Loaded-World Discovery, Actor-Transaction Unification, TUI De-Authority, and Replay Fail-Closed Hardening Spec

**Status**: PROPOSED

This is a staged hardening spec in the parallel `specs/NNNN` series. It is staged in
`specs/` and is promoted to `archive/specs/` on acceptance; it is never promoted to the
live `docs/4-specs/` tier, and it does not amend constitutional invariants, define gate
semantics, or weaken execution gates. It uses the canonical hardening-spec house structure
of its sibling specs (e.g. `0048`/`0046`/`0025`), not the `docs/NN_*` narrative-document
style. The default canonical section set is not used; this sibling-derived structure is.

It is the **second-pass** foundational-conformance hardening of the `0047` surface (TUI
authoritative world-advance, duration completion, actor-known interval summaries). The
first pass was `0048`; this pass closes the residual violations the second-pass re-audit
found surviving `0048`/`0049`.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-second-pass.md`, a
  recommendation-altitude deep-research cross-cutting conformance re-audit of the `0047`
  surface as hardened by `0048` and the `0049MUTWIT` mutation-witness line. The report is the
  originating analysis; it is not itself doctrine and minted no spec, invariant, risk
  identifier, gate code, or glossary term. Its predecessor is
  `reports/0047-foundational-hardening-research-report.md` (the first pass that drove `0048`),
  used by the driver only as a pre-remediation baseline reference.
- **Report target commit.** The report was conducted against
  `8d7c119d94e5438244b6cb1e8e57428c26cb233e` (`8d7c119`), which is the current repository
  `HEAD` at authoring time. Every load-bearing code claim cited below was independently
  re-verified against that working tree (see §3). The named symbols are authoritative; the
  report relies on named symbols, not line numbers (the report's line numbers are not relied
  upon and several were observed to be off by tens of lines against the live tree).
- **Static-survey limitation inherited from the driver.** The report forbade execution, so its
  pass/fail statements are static readings of source/test intent, not command results. This
  spec therefore states code *structure* facts as verified and defers all green/red command
  claims to the implementing session (§7).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-`main` certification or any phase
  entry. When executed, the implementation must name its own exact implementation commit, not
  assume this baseline commit is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` archived-row entry at acceptance/closeout, not at proposal.
  This spec authors no ledger row now and makes no change to live `0001` or the ledger.
- **Archived-history discipline.** The archived `0047` and `0048` specs, their acceptance
  artifacts, the `0049MUTWIT-001..003` tickets, the `0046` artifacts, and any passed
  certification are immutable, commit-pinned historical records. This spec does not edit them
  and does not treat their claims as automatic current-state proof. The closeout for this spec
  records the `0049MUTWIT` source record (§4.8) **without** editing those archived tickets.

## 1. Scope

### 1.1 In scope

The eight divergences and warranted gaps the driver found at `8d7c119`, on the `0047`/`0048`
seams and their named conformance collaborators:

1. **F-01 — Production world steps do not derive loaded-world due work (critical).** The
   public `WorldStepTransactionRequest` carries `due_actor_ids` and `world_process_events`;
   the coordinator iterates the supplied actor IDs rather than deriving them from loaded state,
   and every production caller (TUI authoritative path, core `advance_until`, the no-human
   paths) submits empty collections while only tests inject due work.
2. **F-02 — The public request admits raw caller-authored process events (critical).** The
   transaction iterates arbitrary `world_process_events` envelopes, appends each to the scratch
   log, and applies them directly — bypassing any declared process registry, cadence, trigger,
   typed invocation, or process-owned proposal.
3. **F-03 — The actor decision transaction is not consumed as one authoritative transaction
   (critical).** The coordinator destructures only `Proposed`, uses only `proposed.proposal`,
   `continue`s on `Stuck`, and discards the returned `PipelineResult`, decision trace, trace
   record, lifecycle effects, and local plan; a separate no-human choreography
   (`build_agent_proposal` + post-pipeline append helpers) remains and reconstructs a subset of
   diagnostics afterward.
4. **F-04 — The TUI remains a simulation/projection writer after core completion (critical).**
   The TUI re-calls `record_current_place_perception_and_project` at the scheduler's current
   tick, rebuilds an "after" context, and re-derives the interval delta itself; the perception
   helper creates/appends/projects events; perception `EventId` is deterministic over
   `(actor, tick, kind, target)` and excludes the `index`, while `EventLog::append` does not
   reject duplicate `EventId`s — so the final-tick TUI path can append a second event with the
   same causal identity core just appended.
5. **F-05 — Replay aggregate success is fail-open with respect to temporal divergence (high).**
   `run_replay` computes `matches_expected` without the `rebuild.temporal_violations.is_empty()`
   conjunct; temporal violations are exposed only as a separate field, so `matches_expected ==
   true` can coexist with a temporal divergence.
6. **F-06 — Salient-stop semantics are too broad for the claimed embodied behavior (hardening
   gap, high).** `actor_known_interval_delta_is_salient` returns true when *any* notice has kind
   `Observation`/`Record`/`Belief`; routine re-observation of an unchanged co-located actor can
   satisfy the branch, and the prominent witness does not discriminate routine churn from novel,
   modeled, holder-visible salience.
7. **F-07 — The `0048` behavioral evidence overstates production reachability (hardening gap,
   high).** The loaded-world differential manufactures the exact populations production leaves
   empty (F-01), the salient witness is semantically weak (F-06), and the temporal-replay tests
   do not make the aggregate report fail (F-05); the narrow `0049` mutation witnesses are
   structurally non-vacuous and are preserved.
8. **F-08 — The `0049MUTWIT` line lacks a source-discipline record (provenance gap, high).** The
   mutation-witness line exists only as three archived tickets with no spec package and no
   `SPEC_LEDGER.md` entry; the source-discipline/navigation graph is incomplete.

The corresponding anti-regression layer: production-path discovery witnesses, a declared-process
behavior witness, a closed exhaustively-handled actor-step outcome, fail-closed `EventId`
uniqueness, a replay-report integration witness, a four-case salient-stop witness, extended
compile-fail boundary fixtures, extended deterministic generated schedules with hidden-world
pairs, and focused post-remediation mutation campaigns over the new branches.

### 1.2 Out of scope (non-goals)

- Any higher-tier doctrine change (foundation/architecture/execution/reference). The driver's
  determination — confirmed in §5 — is that the live docs already state the correct contract
  and already forbid the remaining code shapes; the code is below doctrine. Changing doctrine to
  fit incorrect code is prohibited.
- Minting any new invariant, risk ID, gate, glossary term, or ratified paste-ready wording.
- Authoring the ratified `0049MUTWIT` ledger wording or a new identifier for that line inside
  this report-stage document; §4.8 scopes the record's *substance and home*, and the
  acceptance/closeout commit authors the row per the existing ledger process.
- Adding a property-testing crate (`proptest`/`quickcheck`); see §6.4 and §9.
- Expanding the `.cargo/mutants.toml` perimeter — the named seams are already included; the
  delta is behavior that kills the relevant mutants.
- Re-litigating the `0048`/`0049` properties the driver verified as currently correct (§3.1);
  these are preserved, integrated, and must not regress.
- Editing archived/certified artifacts, including the `0049MUTWIT` tickets and the `0048`
  acceptance report (§0).

## 2. Doctrine anchors

Authority order applied (foundation → architecture → execution → reference). Load-bearing anchors:

- **Foundation `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`** — waiting runs the simulation in
  one shared transition: the possessed actor consumes one input slot while other loaded actors,
  world processes, and due consequences advance; embodied summaries are actor-filtered; the TUI
  is not simulation authority.
- **Foundation `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`** — every accepted step, including
  an empty one, carries ancestry sufficient to rebuild the temporal frontier; replay fails closed
  when causal/temporal reconstruction diverges.
- **Foundation `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`** — the canonical actor
  decision transaction selects actor/trigger, builds a sealed actor-known context, generates
  candidates, plans locally, constructs an ancestry-bearing proposal, validates through the shared
  pipeline, and records typed trace/stuck/lifecycle/planner outputs; truth may validate but never
  plan.
- **Architecture `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`** — replay reconstructs
  the temporal frontier and the aggregate verdict fails closed on temporal divergence.
- **Architecture `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`** — actor-known
  interval summaries are positively constructed holder-known frontier deltas; one core-owned
  embodied product, not a client reconstruction.
- **Architecture `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`** —
  world-affecting work enters through ordinary proposal/validation or an owned process transaction;
  due actor/process selection and due consequences belong to one core world-step coordinator; no
  direct dispatch.
- **Architecture `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`** — the
  scheduler invokes, and does not partially imitate, the actor decision transaction; its full typed
  result is authoritative.
- **Architecture `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`** — the TUI asks
  core for a typed world step and never applies events, mutates projections, owns duration terminals,
  maintains a local clock, or reconstructs the embodied interval; actor-known summary and debug
  report are separate core products.
- **Execution `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`** — the full
  phase-ordered one-tick core transaction owns due consequences; ordinary actor transactions and
  declared world processes are explicit phases; no caller-authored authoritative events.
- **Execution `06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`** — the human/no-human proof
  uses core-derived loaded work and the same actor-transaction artifacts on the real path.
- **Execution `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`** — embodied controls use the
  ordinary TUI/core boundary; the complete core-owned interval result is read-only at the client.
- **Execution `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`** —
  typed-before-rendered evidence, real path-under-test witnesses, anti-vacuity, explicit limits; no
  declaration text or artifact presence as proof.
- **Reference `01_DESIGN_RISK_REGISTER.md`** — existing R-27 (reachability overstatement), R-28
  (defect-family completeness), and R-29 (decorative locks) govern the evidence-honesty findings;
  closeout updates the evidence/status of existing risk rows and mints no new risk ID.

Implicated invariants (all verified present in `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`):
`INV-001`, `INV-004`, `INV-005`, `INV-006`, `INV-009`, `INV-010`, `INV-018`, `INV-041`, `INV-067`,
`INV-069`, `INV-087`, `INV-088`, `INV-091`, `INV-092`, `INV-094`, `INV-098`, `INV-099`, `INV-101`,
`INV-102`, `INV-103`, `INV-104`, `INV-105`, `INV-108`, `INV-112`. See §10.

> Verification note: the §4 verdicts cite these invariants by the driver's attribution. The
> implementing session must spot-confirm each cited invariant's wording against the constitution
> before pasting it into a ticket; this spec relies on the named symbols re-verified in code (§3),
> not on second-hand paraphrase of invariant text.

## 3. Determination

**Divergences found; foundational remediation and substantial anti-regression hardening are
warranted.** Four foundational violations (critical), one fail-open replay violation (high), two
evidence/semantics hardening gaps (high), and one provenance gap (high) exist on the `0047`/`0048`
surface at `8d7c119`. All eight were independently re-verified in the working tree at the baseline
commit (see each §4 item; named symbols confirmed, report line numbers not relied upon).

### 3.1 Verified holdings (preserve; do not re-litigate)

The driver found these `0048`/`0049` properties currently correct, with meaningful tests and
mutation scope. They are preserved, integrated into the rebuilt step, and must not regress:

- **Scratch-staged atomic commit.** The one-tick coordinator stages on scratch state and commits at
  the end; human rejection can return a rolled-back result. *Delta:* extend atomic ownership to the
  full actor-transaction products (§4.3), declared-process invocation (§4.2), and final
  perception/interval output (§4.4).
- **Private temporal frontier.** The scheduler frontier is private and moves through accepted world
  steps / validated restore. *Delta:* preserve through the API changes; no new direct writer.
- **Replay temporal projector.** `replay/temporal.rs` validates `TimeAdvanced` ancestry and rebuilds
  the frontier; `rebuild_projection` reports `temporal_violations` and a reconstructed final frontier.
  *Delta:* the aggregate verdict must consume that signal (§4.5).
- **Typed, positively-constructed actor-known interval products.** Holder/debug-context/frontier/source
  boundaries are typed and witnessed. *Delta:* make the core-returned delta the *only* embodied product
  and remove client reconstruction (§4.4).
- **Log-derived duration terminals.** Open durations and terminals are derived from logged starts.
  *Delta:* preserve while replacing the request API; no caller-supplied terminal path.
- **Single-charge passive need accounting and general body-exclusive reservation.** Duplicate need-tick
  application is suppressed; the strict `completion > tick` reservation boundary has witnesses. *Delta:*
  ensure actor/process discovery does not introduce a second accounting pass; re-run reservation through
  the unified actor step.
- **The narrow `0049MUTWIT` mutation witnesses are structurally non-vacuous.** The scheduler, interval,
  and replay/pipeline witnesses assert mutant-sensitive boundaries in real functions and recorded zero
  misses in their filtered campaigns. They are **preserved, not replaced**; the broader `0048`
  behavioral locks (F-07) are what get replaced.

### 3.2 Validated — no action

- **Mutation perimeter.** `.cargo/mutants.toml` already includes the scheduler, replay, projection,
  pipeline, and TUI/core seams. No perimeter expansion required (driver §3.3 / §5; verified present).
- **`anti_regression_guards.rs`** already provides `include_str!`-based source/topology alarms across
  the named surfaces. This spec extends those alarms only as narrow forbidden-shape bans, never as
  behavioral proof.
- **`negative_fixture_runner.rs`** and the external-crate negative-fixture corpus already enforce
  compile-fail boundaries; this spec extends that pattern rather than adding a second compile-fail
  framework.
- **`generative_lock.rs`** already supplies a deterministic recorded-seed corpus and ratcheted witness
  style; this spec extends that harness; no new property-testing crate (§6.4).
- **Higher-tier doctrine.** No correction warranted — the live foundation/architecture/execution/
  reference tiers already state the required behavior and already forbid the remaining implementation
  and evidence shapes (§5).

## 4. Findings and remediation requirements

Each item names the verified current state, the conformance verdict, the required remediation and its
core home, and the strongest practical guard. The TUI must implement none of these phases.

### 4.1 Production world steps do not derive loaded-world due work — **violation (critical)**

- **Verified state (`8d7c119`).** `WorldStepTransactionRequest` publicly contains
  `pub due_actor_ids: Vec<ActorId>` and `pub world_process_events: Vec<EventEnvelope>`. The coordinator
  sorts/dedups and iterates the *supplied* `due_actor_ids`; it does not derive them from loaded state.
  Every production constructor passes `Vec::new()` for both fields — the TUI authoritative action/wait
  path, core `advance_until`, and the no-human paths (`run_no_human_day`,
  `advance_no_human_scheduler_to_tick`, `advance_no_human`). Only `#[cfg(test)]` harnesses populate the
  collections; the prominent loaded-world differential constructs both in-harness and passes them in.
- **Verdict.** Foundational violation of `INV-004`, `INV-005`, `INV-087`, `INV-088`, `INV-091`,
  `INV-094`, `INV-098`, `INV-108` and the shared loaded-world transition doctrine. The canonical step is
  a transaction over caller-enumerated work; human wait, interval continuation, and the no-human paths
  cannot, through this API usage, advance an unpossessed actor or a declared world process. The
  differential is a harness-level composition proof, not a production-reachability proof.
- **Required remediation + home.** Replace the public request shape so the caller supplies only
  controlled input and authority information (expected frontier, origin/controller binding,
  content/ruleset identity, optional ordinary controlled proposal). Core derives, in deterministic
  order: due duration completions from the log; due declared world-process invocations from an owned
  registry/cadence/trigger surface (§4.2); eligible loaded actors from authoritative scheduling state,
  invoking their sealed actor-known decision transactions (§4.3); the possessed actor's controlled
  proposal in the agreed phase; then accounting, observations, projections, interval delta, and one
  commit/rollback decision. Human wait, `advance_until`, and no-human advancement all delegate to that
  same one-tick operation. Delete the caller-injected actor/process collections — no aliases or
  compatibility wrappers. Homes: `crates/tracewake-core/src/scheduler.rs`, the loaded-world
  state/registry owner, the no-human scheduler module, and the narrow `tracewake-tui/src/app.rs` call
  sites.
- **Strongest guard.** Run the real production TUI wait path and the real no-human path from identical
  loaded state containing an unpossessed due actor and a due declared process; the harness configures
  world state/registry only — it must not pass the due work directly. Compare final physical/agent/
  epistemic state, ordered event ancestry, reconstructed frontier, and due-work metrics; assert measured
  nonzero actor/process counts sourced from production discovery, not from request fields. Add a compile-
  fail external-crate fixture proving callers cannot inject or omit those populations. Add focused
  mutation around actor/process eligibility and cadence boundaries ("skip actor", "skip process", "wrong
  due comparison", "possessed-origin changes population"). A source guard banning re-introduction of
  public `due_actor_ids`/raw `world_process_events` is a secondary topology alarm only. **Evidence
  honesty:** the witness must fail when production discovery is disabled while the fixture is unchanged.

### 4.2 The public request admits raw caller-authored process events — **violation (critical)**

- **Verified state.** Inside the transaction the coordinator iterates `request.world_process_events`,
  appends each `EventEnvelope` to the scratch log, and applies the stream directly via
  `apply_event_stream`. There is no process-registry entry, cadence, trigger evaluation, process-local
  input, typed invocation, or process-owned proposal — the caller supplies the already-authored
  authoritative event. Production callers pass an empty vector, so the path is currently exercised mostly
  by tests; once F-01 is corrected it would still let the caller decide *which authoritative event
  already happened*.
- **Verdict.** Foundational violation of `INV-001`, `INV-009`, `INV-010`, `INV-088` and architecture
  `04` / execution `05` (no direct dispatch). An `EventEnvelope` is the committed causal record, not a
  public scheduling request; supplying final envelopes collapses process trigger, decision, validation,
  event construction, and ancestry into caller authority.
- **Required remediation + home.** Replace raw envelopes with a private typed `DueProcessInvocation` (or
  equivalent) created only by core after evaluating a declared process registry. The invocation names the
  process, trigger/cadence witness, effective tick, source event IDs, ruleset/content version, and
  deterministic random provenance if any. It then either produces an ordinary process-origin proposal
  routed through the shared pipeline, or calls a narrowly typed process transition whose event builder is
  owned by the declared process and whose result commits inside the same scratch transaction. No public
  escape hatch for arbitrary envelopes remains; test fixtures needing synthetic events use explicit
  test-only constructors or initial authored history, not the production world-step request. Homes:
  `scheduler.rs`, a core-owned process registry/cadence module, and process-specific action/event
  builders.
- **Strongest guard.** A declared process with a known cadence becomes due from state/log, emits one
  causally linked event through the real coordinator, is absent one tick earlier, and replays identically.
  Compile-time: private invocation constructors; no public request field accepting `EventEnvelope`;
  external-crate negative fixture attempting raw process injection fails to compile. Mutation: kill cadence
  boundary inversions, skipped trigger witness, wrong process ID/source ancestry, and direct-apply
  bypasses. **Evidence honesty:** the witness creates only registry/state inputs; calling an event builder
  in the test or inserting the final event into the request reproduces the defect and does not count.

### 4.3 The actor decision transaction is not consumed as one authoritative transaction — **violation (critical)**

- **Verified state.** `ActorDecisionTransactionOutcome` has `Proposed(Box<ActorDecisionProposalOutcome>)`
  (carrying `proposal`, `decision_trace`, `decision_trace_record`, `lifecycle_effects`, `local_plan`) and
  `Stuck { diagnostic: Box<StuckDiagnosticRecord> }`. The world-step coordinator destructures only
  `Proposed`, takes `proposed.proposal.into_proposal()`, `continue`s on `Stuck`, and discards the returned
  `PipelineResult` — the trace, trace record, lifecycle effects, and local plan are dropped before commit.
  A separate no-human choreography remains: `build_agent_proposal` extracts only proposal +
  `decision_trace_record`, returns `None` on `Stuck`, and after pipeline execution the no-human loop
  separately appends intention-lifecycle, routine-step, decision-trace, and stuck-diagnostic events
  (`append_intention_lifecycle_after_proposal`, `append_routine_step_events_after_proposal`,
  `append_decision_trace_after_proposal`). That path is not the same transaction as the loaded-world actor
  phase, and its stuck handling is assembled from other scheduler state after the actor transaction's own
  typed `Stuck` was discarded.
- **Verdict.** Foundational violation of `INV-041`, `INV-099`, `INV-103`, `INV-104`, `INV-105` and the
  canonical cognition transaction. The code invokes the correct boundary but does not commit its full
  typed result, leaving the transaction partly decorative, preserving two competing actor-transition
  choreographies, and weakening atomicity (no-human trace/lifecycle recording is a later, separate
  sequence).
- **Required remediation + home.** Define a closed internal actor-step outcome the coordinator must
  exhaustively handle. At minimum it preserves: the selected proposal and all proposal ancestry; the typed
  decision trace and trace record; local-plan identity/content needed for replay/debug;
  intention/routine lifecycle effects; the `StuckDiagnosticRecord` when no proposal is available; the
  pipeline accepted/rejected/failed result with actor-legible modeled feedback (no validator-truth leak);
  and the resulting event IDs linking decision, proposal, validation, ordinary events, lifecycle, and
  diagnostics. Commit those products inside the same scratch transaction and expose a typed summary in
  `WorldAdvanceResult`. Route no-human day/window actor opportunities through the same coordinator and
  delete the parallel post-proposal append choreography (`build_agent_proposal` and the
  `*_after_proposal` helpers). No wrappers perpetuating both paths. Homes: `agent/transaction.rs`,
  `scheduler.rs`, and the no-human module inside `scheduler.rs`.
- **Strongest guard.** A real one-tick actor run producing an ordinary action proves trace → proposal →
  validation → event ancestry; a second producing `Stuck` proves the typed diagnostic is committed with no
  primitive action; a rejection run proves actor-legible feedback/lifecycle without validator-truth leak. A
  differential proves the same actor opportunity reached through possessed wait and no-human scheduling
  yields the same artifacts apart from controller-origin metadata. Compile-time: a closed outcome enum that
  cannot be reduced to `Option<Proposal>`; required artifact fields private and only constructible through
  the transaction. Mutation: kill omission of each artifact class, `Stuck => continue`, ignored pipeline
  status, and post-commit lifecycle append. **Evidence honesty:** assertions must run over the event log
  and returned world-step result from the real scheduler entry point (not `ActorDecisionTransaction::run`
  in isolation), with a negative proving failure if any artifact append is removed.

### 4.4 The TUI remains a simulation/projection writer after core completion — **violation (critical)**

- **Verified state.** After `advance_until` returns, the TUI calls
  `record_current_place_perception_and_project` again at `scheduler.current_tick()`, rebuilds an "after"
  knowledge context, and calls `actor_known_interval_delta` itself to build the displayed summary; it also
  appends perception after any accepted command. The perception helper creates perception events, appends
  them via `log.append()`, and applies them via `project_perception_event`. Perception `EventId` is
  deterministic over `(actor_id, decision_tick, perceived.kind, perceived.target_id)`; the `index`
  parameter contributes to the ordering tie-break but **not** to the `EventId`. `EventLog::append` checks
  only schema version and assigns positions — it does **not** reject an already-present `EventId`. For
  `advance_until`, the final core tick and the immediate TUI call share actor/tick/kind/target, so when the
  visible set is unchanged the TUI can append an event with the same causal ID core just appended.
- **Verdict.** Foundational violation of `INV-009`, `INV-018`, `INV-067`, `INV-069`, `INV-101`,
  `INV-102`, `INV-112` and architecture `03`/`10`. The authoritative interval product is split between core
  and TUI, the client owns event creation and projection mutation, and the duplicate-`EventId` possibility
  breaks the expectation that causal identities uniquely name events (ambiguous ordered replay/history even
  though global positions stay monotonic).
- **Required remediation + home.** Make the core step/interval result contain the final typed actor-known
  interval summary (or all closed typed data needed to render it without another projection query or event
  append). Generate final perception, interval-stop, and resume-boundary evidence inside the scratch
  transaction; the TUI stores/renders the returned typed product only. Route ordinary accepted-action
  perception through a modeled core transition or explicit core-owned perception operation; remove direct
  TUI calls to append/project helpers. Make duplicate `EventId` a fail-closed `EventLogError` for both live
  append and deserialization; if repeated observations are semantically distinct, include a deterministic
  causal sequence/parent component in the ID rather than permitting duplicates (see §9). Keep debug
  inspection read-only; forbid any debug-to-embodied interval conversion. Homes: `scheduler.rs`,
  `agent/perception.rs`, `events/log.rs`, `projections.rs`/`view_models.rs`, and `tracewake-tui/src/app.rs`.
- **Strongest guard.** Record log length and projection checksum before the TUI consumes the returned
  world-step/interval result; rendering and summary storage must not change either, and there must be
  exactly one event per `EventId` after multi-tick intervals. Replay from the resulting log and compare
  physical/agent/epistemic/temporal frontiers with no client-side repair. Compile-time: do not export
  event-appending perception helpers to the TUI crate; extend the negative-fixture pattern so a client
  cannot construct or convert a debug report into an embodied interval summary. Mutation: kill removal of
  the duplicate-ID check, re-introduction of post-step append, and substitution of debug/raw context for
  the sealed result. **Evidence honesty:** a source scan banning the helper name in `app.rs` is only a
  topology alarm; the decisive witness executes the ordinary TUI command/advance path and must fail when
  the core's final perception append is removed or when the client appends a second copy.

### 4.5 Replay aggregate success ignores temporal divergence — **violation (high)**

- **Verified state.** `run_replay` computes `matches_expected` from checksum matches, state diff,
  unsupported versions, invariant/application errors, and decision-context hash failures, and **omits**
  `rebuild.temporal_violations.is_empty()`. The same report exposes the nonempty temporal violations only
  as a separate field (`ReplayReport::temporal_violations`, populated from `rebuild.temporal_violations`),
  so a consumer trusting the aggregate boolean can receive `matches_expected == true` alongside a temporal
  divergence. The `0049` replay witnesses cover boundary predicates in `validate_time_advanced`, the
  marker branch in `rebuild_projection`, and duplicate need-tick detection — none closes this report-level
  aggregation omission.
- **Verdict.** Foundational violation of `INV-018`, `INV-092`, `INV-112`. The temporal projector can detect
  the right error while the high-level verdict still reports success — fail-open evidence that makes
  `INV-112` enforcement optional for report consumers. (Severity note: rated *high* rather than *critical*
  despite sharing replay/temporal invariants with the *critical* predecessor sibling `0048` §4.6 "Replay
  does not reconstruct the temporal frontier", because here the temporal projector already works and only
  the one-line aggregate conjunct is missing — an evidence fail-open on a working detector, not an absent
  detector; this follows the driver's calibration.)
- **Required remediation + home.** Include `rebuild.temporal_violations.is_empty()` in `matches_expected`.
  Extend first-divergence reporting with a typed temporal family/detail so a temporal failure does not
  appear only as an auxiliary list. Any acceptance/golden/replay consumer that gates on `matches_expected`
  thereby fails without bespoke secondary checks. Homes: `crates/tracewake-core/src/replay/report.rs`, with
  integration witnesses in `tests/replay_temporal_frontier.rs` or a report-focused sibling.
- **Strongest guard.** Construct a log whose physical/agent/checksum result otherwise matches the expected
  result but whose `TimeAdvanced` ancestry is invalid; assert `temporal_violations` is nonempty and
  `matches_expected` is false. Ensure at least one normal replay/golden lane relies on the aggregate
  verdict so the correction reaches the product evidence boundary. Mutation: kill omission/inversion of the
  temporal-empty conjunct and the typed temporal first-divergence route. **Evidence honesty:** a unit test
  of `validate_time_advanced` is insufficient; the witness must call `run_replay`, preserve all non-temporal
  success conditions, and fail solely because temporal reconstruction is invalid.

### 4.6 Salient-stop semantics are too broad for the claimed embodied behavior — **hardening gap (high)**

- **Verified state.** The stop branch is production-reachable (`advance_until` inspects the core-produced
  actor-known delta — a correction over the predecessor's unreachable branch). But
  `actor_known_interval_delta_is_salient` returns true when **any** notice has kind `Observation`,
  `Record`, or `Belief`. Current-place perception emits a `visible_actor` observation for every co-located
  actor at the tick, classified as an `Observation` source; re-observing an unchanged co-located actor can
  therefore satisfy the salient-kind predicate. The prominent salient-stop test uses co-location plus
  injected process activity and proves a typed notice can stop `advance_until`; it does not prove that quiet
  unchanged ticks continue while a novel, modeled, holder-visible event stops the interval.
- **Verdict.** Hardening gap, not a settled constitutional violation. Typed holder filtering and
  source-bearing construction are correct; the unresolved issue is product semantics and evidence
  precision under `INV-067`, `INV-102`, `INV-112` and architecture `03` / execution `10` (R-27/R-29). The
  current rule can stop on routine observational churn and the witness does not distinguish that from
  meaningful salience.
- **Required remediation + home.** Owner/reassess chooses, within existing doctrine, a closed typed
  salience policy (see §9.6). Strong options: notice novelty relative to the before context;
  contradiction, material confidence/stance change, newly accessible record, newly perceived entity/state
  change, or possessed duration terminal; an explicit typed `IntervalSalience` classification produced
  during holder-known delta construction — not inferred from rendered prose or broad notice category. Do
  not introduce a debug/raw-world salience oracle; keep stop reasons closed and typed. Homes: the closed
  interval notice/stop policy in `scheduler.rs` and actor-known delta construction in
  `epistemics/projection.rs`/`projections.rs`.
- **Strongest guard.** Production `advance_until` with four paired cases: (1) unchanged actor-known state
  for `N` ticks reaches the controller safety bound; (2) a novel modeled holder-visible source becomes due
  at tick `k > 1` and stops exactly at `k`; (3) an otherwise identical hidden/non-holder-visible source
  does not stop; (4) replay reconstructs the same typed stop reason and interval summary. Then focused
  mutation over the holder, source, novelty/frontier, notice-kind, and stop-branch predicates. **Evidence
  honesty:** the novel event must arise from the production process/actor discovery fixed in F-01/F-02
  (manual insertion of a notice/raw event/due-work vector only proves the lower-level classifier), and the
  unchanged case must contain at least one routine perception opportunity so "no notices at all" cannot
  make the test vacuously pass.

### 4.7 The `0048` behavioral evidence overstates production reachability — **hardening gap (high)**

- **Verified state.** The `0048` loaded-world differential manually populates the exact collections
  production leaves empty (F-01) — non-vacuous for coordinator consumption, vacuous for
  discovery/reachability. The salient-stop witness reaches the branch but does not discriminate routine
  re-observation from novel salience (F-06). The replay temporal tests validate the projector but do not
  make the aggregate report fail (F-05). The architecture conformance index and the risk-register `0048`
  evidence notes cite these witness classes as if the full production posture were closed; the live code
  contradicts that breadth, so those rows are stale/overbroad status evidence. The narrow `0049` witnesses
  are structurally non-vacuous and are **preserved**.
- **Verdict.** Hardening gap and current evidence overclaim (R-27/R-28/R-29). Preserve the narrow `0049`
  witnesses; **replace — not merely supplement —** the reachability-overstating `0048` locks. Implicates
  weakly-protected `INV-091`, `INV-094`, `INV-098`, `INV-108`.
- **Required remediation + home.** Extend the existing `world_step_coordinator.rs`,
  `salient_stop_actor_known.rs`, `replay_temporal_frontier.rs`, TUI seam/parity tests, the negative-fixture
  corpus, and the deterministic `generative_lock.rs` — do not create a parallel test framework. The
  rebuilt §4.1 production-path differential becomes the authoritative human/no-human witness; parity
  scenarios must consume real nonzero production discovery, the core-owned interval summary, and the
  temporal-fail report, and must distinguish "duration/accounting differential", "loaded-world
  actor/process differential", "temporal replay", and "holder-known noninterference" rather than citing one
  passing test as all four. Homes: the named core/TUI test seams and the deterministic harness. Do **not**
  edit the historical `0048` acceptance or the archived `0049` tickets (§0); the documentation truthing of
  the affected conformance/risk rows is §7 work, performed only after fresh execution.
- **Strongest guard.** Adversarial variants that deliberately disable actor discovery, disable process
  discovery, drop the decision-trace append, swallow `Stuck`, re-enable TUI post-step perception, omit
  temporal failures from replay success, and collapse salience novelty — each must fail the real
  conformance runner. After F-01..F-06, run the workspace gates, the named focused core/TUI tests, the
  three `0049` focused mutation commands as recorded, a focused campaign over the newly changed
  discovery/process/actor-integration/TUI-boundary/replay-report/salience functions, and the configured
  standing mutation/CI lane (the narrow campaigns are not a replacement). Record exact command, selected
  denominator, caught/missed/unviable sets, commit SHA, and whether a TUI/core production entry point or a
  lower-level helper was exercised.

### 4.8 The `0049MUTWIT` line lacks a source-discipline record — **provenance gap (high)**

- **Verified state.** The repository contains three completed archived tickets
  `archive/tickets/0049MUTWIT-001..003.md` that explain why test-only mutation witnesses were added, name
  the `0048` survivors and adjacent selected mutants, and record focused command outcomes. There is **no**
  corresponding file in `docs/4-specs/` or `archive/specs/`, and `SPEC_LEDGER.md` ends its relevant
  archived rows at `0048` before "Next known execution move"; `0049MUTWIT` appears nowhere in the ledger
  (confirmed). Future readers can find the tests only by archaeology and cannot tell from the ledger whether
  `0049` is an implementation correction, an accepted follow-up package, an unratified ticket series, or
  superseding evidence.
- **Verdict.** Provenance gap. The code/tests may be sound, but the repository's source-discipline graph is
  incomplete, weakening the long-term anti-regression posture and making the `0048` survivor closure easy to
  lose or miscite. R-27/R-28/R-29 make provenance/completeness especially important for mutation locks.
- **Required remediation + home.** At this spec's acceptance/closeout, create the authoritative
  source/navigation record for the `0049MUTWIT` line through the repository's established reassess/ledger
  process, with the resulting status update in `docs/4-specs/SPEC_LEDGER.md` and any required source package
  in the home that process selects. The record should, in substance: identify the `0049` line as the
  follow-up to the eight `0048` survivors and adjacent same-function in-diff mutants; name its three archived
  tickets and affected surfaces; state that it is test-only mutation-witness remediation, not a new product
  feature or latest-`main` certification; preserve the distinction between historical ticket command records
  and fresh current-commit evidence; state whether/how it supplements or supersedes the mutation-evidence
  scope of the `0048` acceptance; and route future re-verification to the standing configured perimeter and
  the named focused witnesses. Do **not** invent a new identifier, author paste-ready ratified ledger text in
  this report-stage spec, or edit the archived tickets. Home: owner/reassess + `docs/4-specs/SPEC_LEDGER.md`.
- **Strongest guard.** Extend the existing spec/ledger integrity guard so a completed cross-cutting
  remediation ticket series cited as closing an accepted spec's mutation survivor floor cannot be orphaned
  from the authoritative navigation record. This is a source-graph check, kept separate from behavior and
  mutation evidence. **Evidence honesty:** the ledger record must link to independently executable witnesses
  and exact historical artifacts; the mere presence of a row is never counted as mutation closure, command
  success, or foundational conformance.

## 5. Doctrine amendments

**None.** The driver's determination, independently confirmed here, is that no higher-tier doctrine
correction is warranted: foundation `08`/`03`/`14`, architecture `02`/`03`/`04`/`05`/`10`, and execution
`05`/`06`/`07`/`10` already require no-human-capable loaded-world advancement, declared-process authority,
the full canonical actor decision transaction, a read-only TUI boundary with a single core-owned embodied
product, and fail-closed temporal replay; reference `01` already names reachability overstatement,
defect-family incompleteness, and decorative locks as R-27/R-28/R-29. The implementation is below doctrine;
amending doctrine to permit empty production due-work populations, raw event injection, discarded
transaction products, client-side event writes, or replay success with temporal violations would erase
Tracewake's accepted causal and possession boundaries. This spec mints no invariant, risk ID, gate,
glossary term, or ratified wording.

The **documentation work that the driver does call for** (§6.2 of the report) is *status/evidence truthing*
of existing conformance and risk rows plus the `0049MUTWIT` ledger record — not doctrinal amendment. It is
scoped here as §7 acceptance work performed only after fresh executable evidence, and as §4.8. The user's
conditional ("include doc amendments if necessary") is therefore satisfied by recording that no doctrine
amendment is warranted and routing the required *truthing* edits to §7.

## 6. Required fixtures and tests

Consolidated from §4 guards plus the cross-cutting anti-regression layer (driver §5).

### 6.1 Behavior witnesses (primary)

- The production-path human/no-human loaded-world differential (§4.1) driven from configured state/registry
  only, with measured nonzero actor/process counts, full state/frontier/projection/checksum comparison, and
  replay; failing when production discovery is disabled.
- The declared-process behavior witness: a known-cadence process becomes due from state/log, emits one
  causally linked event through the real coordinator, is absent one tick earlier, and replays identically
  (§4.2).
- Real one-tick actor runs proving `Proposed` (trace → proposal → validation → event ancestry), `Stuck`
  (typed diagnostic committed, no primitive action), and `Rejected` (actor-legible feedback/lifecycle, no
  truth leak), plus the possessed/no-human artifact-equality differential (§4.3).
- TUI read-only consumption witnesses: log length + projection checksum unchanged by render/store; exactly
  one event per `EventId` after multi-tick intervals; replay from the resulting log with no client repair
  (§4.4).
- The replay-report integration witness: otherwise-matching result with invalid `TimeAdvanced` ancestry
  yields nonempty `temporal_violations` and `matches_expected == false`, surfaced via `run_replay` (§4.5).
- The four-case salient-stop fixture (quiet/novel/hidden/replay) over production `advance_until` (§4.6).
- Measured parity scenario outputs and adversarial parity variants distinguishing the four evidence classes
  (§4.7).
- Preserved holdings (§3.1): mid-duration save/rebuild/resume coverage; the body-exclusive reservation
  census re-run through the unified production actor step; the one-charge-per-`(actor, need, tick)` check
  under the new discovery path.

### 6.2 Compile-time boundaries (extend existing external-crate negative fixtures)

- Due-actor and due-process derivation private to core; no public request field accepting injected
  populations or raw `EventEnvelope`s; `DueProcessInvocation` constructors private.
- A closed actor-step outcome enum that callers cannot reduce to `Option<Proposal>`; required artifact
  fields private and only constructible through the transaction.
- Event-appending perception helpers not exported to the TUI crate; no debug-step-report → embodied-interval
  conversion; the core-owned interval summary the only embodied product.
- Fail-closed duplicate-`EventId` rejection at `EventLog::append` and deserialization.

Retain narrow `include_str!` source guards only as secondary topology alarms (e.g. banning re-introduction
of public `due_actor_ids`/raw process injection, the TUI post-step append, and the spec/ledger orphan check
of §4.8). A source scan is never cited as proof of discovery, atomicity, replay, or noninterference.

### 6.3 Mutation campaigns (post-remediation, existing perimeter)

No `.cargo/mutants.toml` change. After remediation, run focused campaigns over: actor/process eligibility
and cadence boundaries; declared-process invocation; the closed actor-step outcome (each artifact-class
omission, `Stuck => continue`, ignored pipeline status, post-commit append); duplicate-`EventId` rejection
and post-step append re-introduction; the `matches_expected` temporal conjunct and typed temporal
first-divergence; salience holder/source/novelty/notice-kind/stop predicates. Preserve and re-run the three
`0049` focused commands exactly as recorded by their tickets, then run the configured standing perimeter —
the narrow campaigns are not a replacement. Record exact denominators and caught/missed/unviable sets; no
text-only mutation expectations.

### 6.4 Generative / property-testing decision

**Do not add `proptest` or `quickcheck` in this pass.** Extend the existing deterministic
generated-sequence harness (`generative_lock.rs` / `tests/support/generative.rs`) with mixed controlled
input + discovered actors + declared processes + interval stop, without test-built final events. Add the
paired cases the driver names: same world/seed human-vs-no-human origin; same holder-known history with a
changed hidden-world counterpart (noninterference); same due set in different storage order (metamorphic
order invariance); full run vs every replay prefix; one relevant causal change at tick `k` with quiet ticks
before it; duplicate/missing causal marker or event identity as a negative. The paired hidden-world case is
the strongest truth-firewall witness for interval summaries and stop reasons. Add domain-specific
shrink/minimization only when a failing seed appears; reconsider `proptest` only if manual shrinking becomes
a demonstrated maintenance cost (deliberate rejection-for-now, not a value claim).

## 7. Acceptance artifact and evidence

The implementing session must:

- Run the ordinary workspace gates and retain exact transcripts: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace --locked`.
- Run the focused new tests of §6 (at least the core and TUI suites the driver names —
  `world_step_coordinator`, `replay_temporal_frontier`, `holder_known_interval_projection`,
  `salient_stop_actor_known`, `reservation_body_exclusive_census`, `generative_lock`,
  `playable_capability_parity`, `parity_adversarial`, `tui_seam_conformance`, `command_loop_session`,
  `embodied_flow`), the replay/golden lanes, and the mutation campaigns of §6.3; retain exact transcripts
  or deterministic reports. No claim in this spec is premised on an assumed green or red command; every
  pass/fail must be a recorded command result.
- For each load-bearing claim, deliberately break the **production behavior** (not the test fixture) per the
  §4 evidence-honesty checks and confirm the witness fails.
- Produce a new implementation/review artifact recording the static-survey findings and the executed
  remediation evidence (per execution `10` typed-before-rendered and anti-vacuity rules). Do not rewrite
  archived acceptance history.
- **Update live conformance/evidence homes (status/evidence truthing only, not doctrine):**
  - `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — truth the loaded-world, time-control,
    holder-known stop, parity, and replay-report rows so each cites the repaired production path and exact
    witness scope; until then the current rows are not current conformance proof.
  - `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — record the final
    phase choreography and the owned process/actor discovery boundary, without weakening higher tiers.
  - `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — make the human/no-human proof
    explicitly use core-derived loaded work and the same actor-transaction artifacts.
  - `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — identify the complete
    core-owned interval result and the read-only TUI consumption path.
  - `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — map each claim to
    production-path behavior, mutation, compile-fail, replay, and topology evidence without conflating them.
  - `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — update evidence/status under existing R-27, R-28, R-29
    (mint no risk ID).
  - `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — answer its review-checklist prompts with
    the live executable evidence produced this pass (no doctrinal rewrite; mint no new checklist item).
- Create the `0049MUTWIT` source/navigation record and its `docs/4-specs/SPEC_LEDGER.md` status (§4.8)
  through the established reassess/ledger process.
- Add the `docs/4-specs/SPEC_LEDGER.md` archived-row entry for **this** package at acceptance/closeout, and
  move this spec to `archive/specs/` per the hardening-series convention.

## 8. Implementation constraints

- **Recommended closure order** (driver §7; avoids hardening an API that must be deleted):
  1. Replace the world-step request and establish core-owned discovery — remove public due-actor and
     raw-process-event fields; introduce deterministic actor eligibility and a declared process
     registry/cadence; add compile-fail boundaries first.
  2. Unify the actor transition — the coordinator consumes and commits the full
     `ActorDecisionTransactionOutcome`; route no-human actor opportunities through it; delete the legacy
     post-proposal trace/lifecycle/stuck choreography.
  3. Complete core ownership of perception and interval output — return the final typed holder-known
     summary/result; remove all TUI event/projection writes; add fail-closed `EventId` uniqueness.
  4. Close replay reporting — temporal violations fail `matches_expected`; add typed temporal first
     divergence.
  5. Choose and encode salient-stop semantics — the closed novelty/materiality rule, after the process and
     interval boundaries are authoritative.
  6. Rebuild behavior evidence on production entry points — replace harness-injected loaded-world tests with
     real discovery; add quiet/novel/hidden/replay salient cases; prove TUI read-only consumption and
     human/no-human parity.
  7. Extend the existing deterministic and compile-fail harnesses — generated mixed schedules, order
     invariance, prefix replay, hidden-world pairs, external-crate authority negatives. Do not duplicate
     infrastructure.
  8. Run focused mutation during each change, then the configured standing perimeter — preserve the `0049`
     witnesses, add kills for the new branches, record exact denominators/artifacts, reject unexplained
     misses.
  9. Truth documentation and source records only after executable evidence — update conformance/risk rows
     and complete the `0049MUTWIT` ledger/source record; keep historical artifacts unchanged.
- `tracewake-core` keeps zero dependencies; the one-way crate dependency direction
  (core ← content ← tui) is preserved. The TUI implements none of the world-step phases and remains a
  read-only consumer of the typed core result.
- No backwards-compatibility shims or alias paths: delete the caller-injected actor/process collections,
  the raw-envelope path, the parallel no-human append choreography, and the TUI perception/interval
  reconstruction rather than retaining them as compatibility paths.
- Source scans remain secondary topology alarms, never substitutes for behavior witnesses.

## 9. Risks and open questions

These are implementation choices inside settled doctrine (driver §8), not reasons to defer the verdict:

1. **Loaded-actor eligibility representation.** Per-actor next-decision tick, a scheduled opportunity queue,
   deterministic cadence derived from actor state, or another replayable structure. It must be core-owned,
   deterministic, and possession-neutral.
2. **World-process registry shape.** Whether declared processes produce ordinary proposals or typed process
   transition results. Either way cadence/trigger/source ancestry is explicit and raw final envelopes must
   not cross the public request boundary.
3. **Actor-step outcome packaging.** The exact internal enum/struct joining trace, local plan, lifecycle,
   stuck, pipeline result, and resulting event IDs; the compiler must force every variant to be handled.
4. **Perception timing within the tick.** The phase(s) at which actors perceive before decision and after
   world change, while preventing duplicate same-causal-identity events and preserving actor-known
   causality.
5. **Repeated observation identity.** Whether identical observations at different causal opportunities
   receive a deterministic occurrence/parent component or are deduplicated as the same fact. Duplicate event
   IDs must not remain representable (§4.4).
6. **Salience policy.** Whether novelty, contradiction, confidence/stance change, newly available record,
   local state change, or an explicit materiality class stops an interval. The policy must remain
   holder-known and typed. (Severity note: §4.6 is rated *high* rather than *critical* despite sharing
   truth-firewall invariants with the *critical* §4.4, because it is an over-broad-but-typed policy/evidence
   gap rather than an active client-authority breach; this follows the driver's own calibration.)
7. **World-step versus interval result layering.** Whether every tick returns an actor-known delta and
   `advance_until` aggregates them, or the interval driver owns aggregation. The TUI must receive the final
   typed product and remain read-only either way.
8. **`0049MUTWIT` source-package classification.** The correct repository-native home and status for a
   test-only mutation follow-up that closes an accepted spec's survivor floor (§4.8). This spec deliberately
   does not invent the ratified record.
9. **Static-survey carryover.** The driver executed no commands. If the implementing session's gate run
   surfaces a current-state fact that contradicts a §4 structural claim, correct this spec first (per the
   repo ticket/spec-divergence convention), then implement.
10. **Spec number.** This package is numbered `0050` because `0049` is already claimed by the `0049MUTWIT`
    ticket prefix whose source record §4.8 creates; if owner/reassess prefers a different number or absorbs
    the `0049` line differently, renumber before acceptance.
11. **Fail-closed `EventId` uniqueness vs. pre-existing artifacts.** Once §4.4 makes a duplicate `EventId` a
    fail-closed `EventLogError` on both live append **and deserialization**, any already-recorded log,
    golden-replay fixture, or save that contains a duplicate perception identity (exactly what the current
    buggy TUI final-tick path can emit) will fail to deserialize/replay. This is intentional and acceptable
    under §8's no-compat-shims stance (pre-release): affected golden fixtures/saves are regenerated from the
    corrected production path rather than migrated by a shim. The implementing session must expect to
    regenerate such artifacts when the §7 replay/golden lanes first run, and must not weaken the uniqueness
    rule to accommodate a stale fixture.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-004` / `INV-005` / `INV-006` / `INV-087` / `INV-091` / `INV-094` / `INV-108` | aligns | Core-owned discovery of due actors/processes (§4.1) gives unpossessed actors and world processes their ordinary opportunity on every human/no-human path; possession changes only input/viewpoint and human focus cannot make the world wait @ scheduler/world-step. |
| `INV-001` / `INV-009` / `INV-010` / `INV-088` | aligns | Replacing raw `EventEnvelope` injection with a declared `DueProcessInvocation` routed through proposal/validation (§4.2) makes every world-affecting change a modeled, cause-bearing committed event with declared cadence/ancestry @ world kernel/event log/process registry. |
| `INV-041` / `INV-099` / `INV-103` / `INV-104` / `INV-105` | aligns | The closed, exhaustively-handled actor-step outcome committed atomically (§4.3) preserves trace/lifecycle/plan/stuck/feedback as authoritative diagnostics, keeps cognition in the transaction not the scheduler, and routes needs/routines through candidate generation/planning @ actor transaction/scheduler. |
| `INV-067` / `INV-069` / `INV-101` / `INV-102` | aligns | A single core-owned holder-known interval product plus removal of TUI event/projection writes (§4.4/§4.6) restores the sealed, source-bearing, read-only embodied boundary @ epistemic projection/view models/TUI. |
| `INV-018` / `INV-092` / `INV-112` | aligns | Folding `temporal_violations` into `matches_expected` and fail-closed `EventId` uniqueness (§4.5/§4.4) make replay reconstruct authoritative time and fail closed on divergence, with causal identities uniquely naming events @ replay/event log. |
| `INV-098` | aligns | Production-path discovery, four-case salient evidence, and replaced reachability locks (§4.1/§4.6/§4.7) make the hardened features no-human runnable, replay-safe, TUI-reachable, and regression-tested @ core/TUI test seams. |

No invariant is tensioned; this spec enforces existing invariants against below-doctrine code and amends
none (§5).

## Outcome

_Pending implementation and acceptance._ On acceptance the closeout commit records the exact implementation
baseline commit(s) and evidence/report commit, archives the implementation tickets, adds the
`docs/4-specs/SPEC_LEDGER.md` archived row for this package, creates the `0049MUTWIT` source/ledger record
(§4.8), updates the existing architecture-conformance and risk rows (R-27/R-28/R-29) and the named
execution evidence maps, moves this spec to `archive/specs/`, and runs the required post-closeout gates.
This spec mints no invariant, doctrine change, gate, glossary term, or risk ID.
